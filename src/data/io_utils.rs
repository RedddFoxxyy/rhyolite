use crate::data::{
	stores::{
		doc_store::{ACTIVE_DOCUMENT_TITLE, CLIPBOARD, FILES_ARENA, PLATFORM},
		tabs_store::{CURRENT_TAB, TABS, new_tab, push_tab},
	},
	types::{APP_DATA_DIR, DEFAULT_TROVE_DIR, MarkdownFile, USER_DATA_DIR, USER_DATA_FILE, UserData},
};
use freya::prelude::*;
use log::LevelFilter;
use std::{fs, io::Write, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt, runtime::Runtime};

use super::stores::{doc_store::RECENT_FILES, ui_store::THEME_STORE};

pub fn env_logger_init() {
	let mut builder = env_logger::Builder::new();
	builder.filter(None, LevelFilter::Warn);
	builder.filter_module("Rhyolite", LevelFilter::Trace);
	builder.init();
}

/// This function returns the path to the documents' directory.
pub fn get_rhyolite_dir() -> PathBuf {
	#[cfg(target_os = "android")]
	{
		// On Android, use the app's private storage directory
		let path = PathBuf::from("/data/user/0/com.rhyolite.dev/Rhyolite");
		// Create the directory if it doesn't exist
		fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
		path
	}

	#[cfg(not(target_os = "android"))]
	{
		let mut path = dirs::document_dir().expect("Could not find Documents directory");
		// TODO: Use a const for this name.
		path.push(APP_DATA_DIR);
		fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
		path
	}
}

/// This function returns the path to the default trove directory.
pub fn get_trove_dir(trove_name: &str) -> PathBuf {
	//Get the path to documents/Rhyolite.
	let rhyolite_dir = get_rhyolite_dir();

	//Append the default trove name to the 'documents/Rhyolite path'.
	let trove_dir = rhyolite_dir.join(trove_name);

	//Then create the path 'documents/Rhyolite/trove_name' if it does not
	fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

	//retrun the path of the default trove directory.
	trove_dir
}

pub fn get_userdata_path() -> PathBuf {
	let userdata_dir = get_rhyolite_dir().join(USER_DATA_DIR);
	fs::create_dir_all(&userdata_dir).expect("Could not create Rhyolite appdata directory");
	get_rhyolite_dir().join(USER_DATA_DIR).join(USER_DATA_FILE)
}

/// Generate a path that is not conflicting by incrementing a counter at file end
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

/// Generates a new markdown file from the given path (does not save it)
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

pub fn save_userdata() {
	let current_editor_state = UserData {
		active_tabs: TABS(),
		last_open_tab: CURRENT_TAB().unwrap(),
		recent_files: RECENT_FILES(),
		current_theme: THEME_STORE().current_theme.clone(),
	};

	if let Ok(toml_serialised_state) = toml::to_string::<UserData>(&current_editor_state) {
		if let Ok(mut userdata_file) = fs::File::create(get_userdata_path()) {
			// TODO: Handle Error for this operation and the parent operations.
			let _op_result = userdata_file.write(toml_serialised_state.as_bytes());
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
				log::error!("Failed to get the file extention, did not load {path:?}");
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
	}
}

pub fn load_default_trove() {
	load_files_from_trove(get_trove_dir(DEFAULT_TROVE_DIR))
}

pub fn load_from_userdata() {
	let userdata_string = fs::read_to_string(get_userdata_path()).expect("Could not read user data file");

	let Ok(userdata) = toml::from_str::<UserData>(userdata_string.as_str()) else {
		log::warn!("Failed to load the userdata, corrupted userdata file.");
		// TODO: Handle Error
		let _ = fs::remove_file(get_userdata_path());
		log::info!("Loading all files from the default trove.");
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
			log::error!("Failed to get the file extention, did not load {path:?}");
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
	}
}

pub async fn save_file(markdownfile: MarkdownFile) {
	if let Ok(mut file) = File::create(markdownfile.path.clone()).await {
		if let Ok(_result) = file.write_all(markdownfile.editable.editor().to_string().as_bytes()).await {
			log::info!("Succesfully saved {} at {:#?}", markdownfile.title, markdownfile.path)
		} else {
			log::error!("Failed to save {} at {:#?}!", markdownfile.title, markdownfile.path)
		}
	}
}

pub async fn delete_file(markdownfile: MarkdownFile) {
	if let Ok(_result) = tokio::fs::remove_file(markdownfile.path.clone()).await {
		// log::info!("Succesfully deleted {}.", markdownfile.title)
	} else {
		log::error!("Failed to save the file!")
	}
}

/// Loads last saved State of the App.
pub fn initialise_app() {
	let userdata_path = get_rhyolite_dir().join(USER_DATA_DIR).join(USER_DATA_FILE);

	if !userdata_path.exists() {
		log::warn!("No UserData file found!!! Proceeding to load files from default trove!");
		load_default_trove()
	} else {
		log::info!("Loading last app state.");
		load_from_userdata()
	};

	// TODO: yeah um handle the unwraps lol
	let Some(title) = TABS().get(CURRENT_TAB().unwrap()).map(|tab| tab.title.clone()) else {
		log::error!("Failed to set the document title of the current file.");
		return;
	};

	*ACTIVE_DOCUMENT_TITLE.write() = title;
}

pub fn deinitialise_app() {
	let tokio = Runtime::new().unwrap();
	for tab in TABS().iter() {
		if let Some(markdownfile) = FILES_ARENA().get(tab.file_key) {
			tokio.block_on(save_file(markdownfile.clone()));
		}
	}
	save_userdata();
}
