// Copyright (C) 2025  Suyog Tandel(RedddFoxxyy)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::data::{
	stores::{
		ACTIVE_DOCUMENT_TITLE, CLIPBOARD, CURRENT_TAB, FILES_ARENA, PLATFORM, RECENT_FILES, TABS, THEME_STORE, new_tab, push_tab,
		switch_tab,
	},
	types::{APP_DATA_DIR, DEFAULT_TROVE_DIR, MarkdownFile, USER_DATA_FILE, UserData},
};
use freya::prelude::*;
use log::LevelFilter;
use log4rs::{
	append::{
		console::{ConsoleAppender, Target},
		rolling_file::{
			RollingFileAppender,
			policy::compound::{CompoundPolicy, roll::delete::DeleteRoller, trigger::size::SizeTrigger},
		},
	},
	config::{Appender, Logger, Root},
	encode::pattern::PatternEncoder,
};
use std::{fs, io::Write, path::PathBuf};
use tokio::{
	fs::{File, rename},
	io::AsyncWriteExt,
	runtime::Runtime,
};

/// Initializes log4rs with custom configuration for stdout and file logging.
pub fn logger_init() {
	let log_file_path = {
		let Some(state) = dirs::state_dir() else {
			log::error!("No App State directory could be found/accessed!");
			panic!("Failed to find App State directory.")
		};
		let log_dir = state.join(APP_DATA_DIR);

		if let Err(e) = fs::create_dir_all(&log_dir) {
			log::error!("Failed to create log directory!: {e}");
		};

		log_dir.join("rhyolite.log")
	};

	// TODO: Add session based log files or rolling log files with archiving of old files, to prevent a single log file from growing too large.
	let size_trigger = SizeTrigger::new(10 * 1024 * 1024); // 10 MB
	let roller = DeleteRoller::new();
	let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(roller));

	let logfile = RollingFileAppender::builder()
		.encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S %Z)} {l} {t}] {m}{n}")))
		.build(log_file_path, Box::new(policy))
		.unwrap();

	let stdout = ConsoleAppender::builder()
		.target(Target::Stdout)
		.encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S %Z)} {h({l})} {t}] {m}{n}")))
		.build();

	let (app_level, root_level) = if cfg!(debug_assertions) {
		(LevelFilter::Trace, LevelFilter::Debug)
	} else {
		(LevelFilter::Info, LevelFilter::Error)
	};
	let config = log4rs::Config::builder()
		.appender(Appender::builder().build("stdout", Box::new(stdout)))
		.appender(Appender::builder().build("logfile", Box::new(logfile)))
		.logger(Logger::builder().build("Rhyolite", app_level))
		.build(Root::builder().appenders(vec!["logfile", "stdout"]).build(root_level))
		.unwrap();

	log4rs::init_config(config).unwrap();
}

/// Returns the path to the default trove directory.
pub fn get_default_trove_dir() -> PathBuf {
	// TODO: Handle path resolution/creation without panicking
	let Some(documents_path) = dirs::document_dir() else {
		log::error!("No document directory could be found/accessed!");
		panic!("Failed to find Documents directory.")
	};
	let default_trove_path = documents_path.join(DEFAULT_TROVE_DIR);
	fs::create_dir_all(&default_trove_path).expect("Could not create default Trove.");
	default_trove_path
}

/// Returns the path to the config directory for the app data.
pub fn get_config_dir() -> PathBuf {
	// TODO: Handle path resolution/creation without panicking
	let Some(mut path) = dirs::config_dir() else {
		log::error!("No Config directory could be found/accessed!");
		panic!("Failed to find Config directory.")
	};
	path.push(APP_DATA_DIR);
	fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
	path
}

pub fn get_userdata_path() -> PathBuf {
	let userdata_dir = get_config_dir();
	fs::create_dir_all(&userdata_dir).expect("Could not create Rhyolite config directory");
	userdata_dir.join(USER_DATA_FILE)
}

/// Generate a path that is not conflicting by incrementing a counter at the file end
pub fn generate_available_path(path: PathBuf) -> PathBuf {
	if !path.exists() {
		return path;
	}
	let suffix = path
		.extension()
		.map(|ext| format!(".{}", ext.to_string_lossy()))
		.unwrap_or("".to_string());
	let prefix = path
		.file_stem()
		.unwrap_or_else(|| panic!("Unable to read path: {}", path.display()));
	let mut prefix_without_num = prefix
		.to_string_lossy()
		.to_string()
		.trim_end_matches(|c: char| c.is_ascii_digit())
		.to_string();
	if prefix.len() == prefix_without_num.len() && !prefix_without_num.ends_with(' ') {
		prefix_without_num.push(' ');
	}
	let mut num = 1;
	loop {
		let new_path = path.with_file_name(format!("{prefix_without_num} {num}{suffix}"));
		if !new_path.exists() {
			return new_path;
		}
		num += 1;
	}
}

/// Opens the file from the given path.
pub fn _open_file_from_path(path: PathBuf) -> Option<MarkdownFile> {
	let markdown_file = fs::read_to_string(path.clone());

	// TODO: Handle this gracefully
	let file_name = path
		.clone()
		.file_stem()
		.unwrap_or_else(|| panic!("Unable to read path: {}", path.display()))
		.to_str()
		.unwrap()
		.to_string();

	if let Ok(content) = markdown_file {
		Some(MarkdownFile {
			path,
			title: file_name,
			editable: UseEditable::new_in_hook(
				CLIPBOARD(),
				PLATFORM(),
				EditableConfig::new(content).with_allow_tabs(true),
				EditableMode::SingleLineMultipleEditors,
			),
		})
	} else {
		None
	}
}

/// Generates a new Markdown file from the given path (does not save it)
pub fn new_file_from_path(path: PathBuf) -> Option<MarkdownFile> {
	let cloned_path = path.clone();

	let Some(file_name) = cloned_path.file_stem() else {
		// TODO: Improve the error message.
		log::error!("Unable to read path: {}", path.display());
		return None;
	};
	Some(MarkdownFile {
		path,
		title: file_name.to_string_lossy().into_owned(),
		editable: UseEditable::new_in_hook(
			CLIPBOARD(),
			PLATFORM(),
			EditableConfig::new(String::new()).with_allow_tabs(true),
			EditableMode::SingleLineMultipleEditors,
		),
	})
}

pub async fn save_userdata() {
	let last_open_tab = CURRENT_TAB().unwrap_or_default();

	let current_editor_state = UserData {
		active_tabs: TABS(),
		last_open_tab,
		recent_files: RECENT_FILES(),
		current_theme: THEME_STORE().current_theme.clone(),
	};

	if let Ok(toml_serialised_state) = toml::to_string::<UserData>(&current_editor_state)
		&& let Ok(mut userdata_file) = fs::File::create(get_userdata_path())
	{
		// TODO: Handle Error for this operation and the parent operations.
		let io_result = userdata_file.write(toml_serialised_state.as_bytes());
		if io_result.is_err() {
			log::error!("Unable to write userdata file: {}", io_result.unwrap_err());
		}
	}
}

pub fn load_files_from_trove(trove_path: PathBuf) {
	let mut markdownfiles: Vec<MarkdownFile> = Vec::new();

	// NOTE: Wrote this half asleep, do not judge :(
	if let Ok(entries) = fs::read_dir(&trove_path) {
		for entry in entries {
			let Ok(entry) = entry else { continue };
			let path = entry.path();

			if !path.is_file() {
				continue;
			}

			let Some(extension) = path.extension() else {
				log::error!("Failed to get the file extension, did not load {path:?}");
				continue;
			};

			if extension != "md" {
				log::error!("{path:?} is not a markdown file, skipped loading it.");
				continue;
			}

			let content = match fs::read_to_string(&path) {
				Ok(c) => c,
				Err(e) => {
					log::error!("Error reading file {path:?}: {e}");
					// TODO: Handle the error
					continue;
				}
			};

			let title = path.file_stem().and_then(|name| name.to_str()).unwrap().to_string();

			let file_data = MarkdownFile {
				path: path.clone(),
				title,
				editable: UseEditable::new_in_hook(
					CLIPBOARD(),
					PLATFORM(),
					EditableConfig::new(content).with_allow_tabs(true),
					EditableMode::SingleLineMultipleEditors,
				),
			};

			markdownfiles.push(file_data);
		}
	} else {
		log::error!("Error reading directory: {trove_path:?}");
	}

	let tokio = Runtime::new().unwrap();

	if markdownfiles.is_empty() {
		tokio.block_on(new_tab());
	} else {
		for file in markdownfiles {
			let title = file.title.clone();
			let file_key = FILES_ARENA.write().insert(file);
			tokio.block_on(push_tab(title, file_key));
		}
		*CURRENT_TAB.write() = Some(0);
		tokio.block_on(switch_tab(CURRENT_TAB().unwrap_or_default()));
	}
}

pub fn load_default_trove() {
	load_files_from_trove(get_default_trove_dir())
}

pub fn load_from_userdata() {
	let userdata_string = fs::read_to_string(get_userdata_path()).expect("Could not read user data file");

	let Ok(userdata) = toml::from_str::<UserData>(userdata_string.as_str()) else {
		log::error!("Failed to load the userdata, corrupted userdata file.");
		// TODO: Handle Error
		let _ = fs::remove_file(get_userdata_path());
		log::warn!("Loading all files from the default trove.");
		return load_default_trove();
	};

	let mut markdownfiles: Vec<MarkdownFile> = Vec::new();

	// NOTE: Wrote this half asleep, do not judge :(
	for tab in userdata.active_tabs {
		let path = tab.file_path;

		if !path.is_file() {
			log::error!("{path:?} is not a valid file!!!");
			continue;
		}

		let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
			log::error!("Failed to get the name of the file {path:?}");
			continue;
		};

		if stem != tab.title {
			log::error!("File name does not match the title of the tab in userdata, did not load {path:?}");
			continue;
		}

		let Some(extension) = path.extension() else {
			log::error!("Failed to get the file extension, did not load {path:?}");
			continue;
		};

		if extension != "md" {
			log::error!("{path:?} is not a markdown file, skipped loading it.");
			continue;
		}

		let Ok(content) = fs::read_to_string(&path) else {
			log::error!("Error reading file {path:?}");
			// TODO: Handle the error
			continue;
		};

		let file_data = MarkdownFile {
			path,
			title: tab.title,
			editable: UseEditable::new_in_hook(
				CLIPBOARD(),
				PLATFORM(),
				EditableConfig::new(content).with_allow_tabs(true),
				EditableMode::SingleLineMultipleEditors,
			),
		};

		markdownfiles.push(file_data);
	}

	let tokio = Runtime::new().unwrap();

	if markdownfiles.is_empty() {
		tokio.block_on(new_tab());
	} else {
		for file in markdownfiles {
			let title = file.title.clone();
			let file_key = FILES_ARENA.write().insert(file);
			tokio.block_on(push_tab(title, file_key));
		}
		*CURRENT_TAB.write() = Some(userdata.last_open_tab);
		THEME_STORE.write().current_theme = userdata.current_theme;
		*RECENT_FILES.write() = userdata.recent_files;
		tokio.block_on(switch_tab(CURRENT_TAB().unwrap_or_default()));
	}
}

pub async fn save_file(markdownfile: MarkdownFile) {
	if let Ok(mut file) = File::create(markdownfile.path.clone()).await {
		if let Ok(_result) = file.write_all(markdownfile.editable.editor().to_string().as_bytes()).await {
			log::debug!("Successfully saved {} at {:#?}", markdownfile.title, markdownfile.path)
		} else {
			log::error!("Failed to save {} at {:#?}!", markdownfile.title, markdownfile.path)
		}
	}
}

pub async fn delete_file(markdownfile: MarkdownFile) {
	if let Ok(_result) = tokio::fs::remove_file(markdownfile.path.clone()).await {
		log::debug!("Successfully removed {}.", markdownfile.title)
	} else {
		log::error!("Failed to save the file!")
	}
}

pub async fn update_document_title(new_title: String) {
	let current_tab_index = CURRENT_TAB().unwrap();
	let tabs = TABS.read();

	let Some(tab) = tabs.get(current_tab_index) else {
		// TODO: Handle This error
		return;
	};
	let file_key = tab.file_key;

	if let Some(markdown_file) = FILES_ARENA.write().get_mut(file_key) {
		let old_path = markdown_file.path.clone();
		let new_path = old_path.with_file_name(format!("{}.md", new_title));

		if let Err(e) = rename(&old_path, &new_path).await {
			log::error!("Failed to rename file: {}", e);
			return;
		}

		markdown_file.title = new_title.clone();
		markdown_file.path = new_path.clone();

		drop(tabs);

		let mut tabs_mut = TABS.write();
		if let Some(tab_mut) = tabs_mut.get_mut(current_tab_index) {
			tab_mut.title = new_title.clone();
			tab_mut.file_path = new_path;
		}

		*ACTIVE_DOCUMENT_TITLE.write() = new_title;
	}
	save_userdata().await;
}

/// Loads last saved State of the App.
pub fn initialise_app() {
	let userdata_path = get_userdata_path();

	if !userdata_path.exists() {
		log::error!("No UserData file found!!! Proceeding to load files from default trove!");
		load_default_trove()
	} else {
		log::debug!("Loading last app state.");
		load_from_userdata()
	};
}

// TODO: Mark saved files and only save the unsaved files.
pub fn deinitialise_app() {
	let tokio = Runtime::new().unwrap();
	for tab in TABS().iter() {
		if let Some(markdownfile) = FILES_ARENA().get(tab.file_key) {
			tokio.block_on(save_file(markdownfile.clone()));
		}
	}
	tokio.block_on(save_userdata());
}
