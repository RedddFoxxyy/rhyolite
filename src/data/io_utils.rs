use crate::data::{
	stores::{
		doc_store::{ACTIVE_DOCUMENT_TITLE, CLIPBOARD, FILES_ARENA, PLATFORM, USER_DATA},
		tabs_store::{CURRENT_TAB, TABS, new_tab, push_tab},
	},
	types::{
		APP_DATA_DIR, DEFAULT_TROVE_DIR, MarkdownFile, USER_DATA_DIR, USER_DATA_FILE, UserData,
	},
};
use freya::prelude::*;
use log::LevelFilter;
use std::{fs, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt, runtime::Runtime};

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
		// Original desktop behavior
		let mut path = dirs::document_dir().expect("Could not find Documents directory");
		// TODO: Use a const for this name.
		path.push(APP_DATA_DIR);
		// Create the directory if it doesn't exist
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
		let new_path = path.with_file_name(format!("{} {}{}", prefix_without_num, num, suffix));
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
	let file_name = cloned_path.file_stem();
	match file_name {
		Some(name) => {
			return Some(MarkdownFile {
				path,
				title: name.to_string_lossy().into_owned(),
				editable: UseEditable::new_in_hook(
					CLIPBOARD(),
					PLATFORM(),
					EditableConfig::new(String::new()).with_allow_tabs(true),
					EditableMode::SingleLineMultipleEditors,
				),
			});
		}
		None => {
			// TODO: Improve the error message.
			log::error!("Unable to read path: {}", path.display());
			return None;
		}
	};
}

pub fn load_files_from_trove(trove_path: PathBuf) -> Vec<MarkdownFile> {
	let mut markdown_files_data: Vec<MarkdownFile> = Vec::new();

	if let Ok(entries) = fs::read_dir(&trove_path) {
		for entry in entries {
			let Ok(entry) = entry else { continue };
			let path = entry.path();

			if path.is_file() {
				if let Some(extension) = path.extension() {
					if extension == "md" {
						let content = match fs::read_to_string(&path) {
							Ok(c) => c,
							Err(e) => {
								log::error!("Error reading file {:?}: {}", path, e);
								// TODO: Handle the error
								continue;
							}
						};

						let title = path
							.file_stem()
							.and_then(|name| name.to_str())
							.unwrap()
							.to_string();

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

						markdown_files_data.push(file_data);
					}
				}
			}
		}
	} else {
		log::error!("Error reading directory: {:?}", trove_path);
	}

	markdown_files_data
}

pub fn load_default_trove() -> Vec<MarkdownFile> {
	load_files_from_trove(get_trove_dir(DEFAULT_TROVE_DIR))
}

pub fn load_from_userdata() -> Vec<MarkdownFile> {
	let userdata_string =
		fs::read_to_string(get_userdata_path()).expect("Could not read user data file");

	let Ok(userdata) = toml::from_str::<UserData>(userdata_string.as_str()) else {
		// TODO: Handle Error
		let _ = fs::remove_file(get_userdata_path());
		return load_default_trove();
	};

	let mut markdown_files_data: Vec<MarkdownFile> = Vec::new();

	*USER_DATA.write() = userdata;
	for tab in USER_DATA().active_tabs {
		if let Ok(entries) = fs::read_dir(get_trove_dir(DEFAULT_TROVE_DIR)) {
			for entry in entries {
				let Ok(entry) = entry else { continue };
				let path = entry.path();

				// NOTE: Wrote this half asleep, do not judge :(
				if path.is_file() && path.file_name().unwrap().to_str().unwrap() == tab.title {
					if let Some(extension) = path.extension() {
						if extension == "md" {
							let content = match fs::read_to_string(&path) {
								Ok(c) => c,
								Err(e) => {
									log::error!("Error reading file {:?}: {}", path, e);
									// TODO: Handle the error
									continue;
								}
							};

							let title = path
								.file_stem()
								.and_then(|name| name.to_str())
								.unwrap()
								.to_string();

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

							markdown_files_data.push(file_data);
						}
					}
				}
			}
		} else {
			log::error!("Error reading directory.");
		}
	}
	markdown_files_data
}

pub async fn save_file(markdownfile: MarkdownFile) {
	if let Ok(mut file) = File::create(markdownfile.path.clone()).await {
		if let Ok(_result) = file
			.write_all(markdownfile.editable.editor().to_string().as_bytes())
			.await
		{
			log::info!("Succesfully saved current file")
		} else {
			log::error!("Failed to save the file!")
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
	let tokio = Runtime::new().unwrap();
	let userdata_path = get_rhyolite_dir().join(USER_DATA_DIR).join(USER_DATA_FILE);

	let markdownfiles = if !userdata_path.exists() {
		load_default_trove()
	} else {
		load_from_userdata()
	};

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

	// TODO: yeah um handle the unwraps lol
	*ACTIVE_DOCUMENT_TITLE.write() = TABS().get(CURRENT_TAB().unwrap()).unwrap().title.clone();
}

pub fn deinitialise_app() {
	let tokio = Runtime::new().unwrap();
	for tab in TABS().iter() {
		if let Some(markdownfile) = FILES_ARENA().get(tab.file_key) {
			// NOTE: I think this can be handled better here? not sure if this will cause any performance issues as such.
			tokio.block_on(save_file(markdownfile.clone()));
		}
	}
}
