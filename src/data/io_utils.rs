use crate::data::{
	stores::{
		docspace::{FILES_BUFFER, USER_DATA},
		tabs::{new_tab, push_tab},
	},
	types::{DEFAULT_TROVE_DIR, MarkdownFile, USER_DATA_DIR, USER_DATA_FILE, UserData},
};
use std::fs;
use std::path::PathBuf;

use super::{stores::tabs::CURRENT_TAB, types::APP_DATA_DIR};

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
pub(crate) fn get_trove_dir(trove_name: &str) -> PathBuf {
	//Get the path to documents/Rhyolite.
	let documents_dir = get_rhyolite_dir();

	//Append the default trove name to the 'documents/Rhyolite path'.
	let trove_dir = documents_dir.join(trove_name);

	//Then create the path 'documents/Rhyolite/trove_name' if it does not
	fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

	//retrun the path of the default trove directory.
	trove_dir
}

pub fn get_userdata_path() -> PathBuf {
	get_rhyolite_dir().join(USER_DATA_DIR).join(USER_DATA_FILE)
}

/// Generate a path that is not conflicting by incrementing a counter at file end
pub(crate) fn generate_available_path(path: PathBuf) -> PathBuf {
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
pub fn open_file_from_path(path: PathBuf) -> Option<MarkdownFile> {
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
			content,
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
				content: String::new(),
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
							content,
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
								content,
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

/// Loads last saved State of the App.
pub fn initialise_app() {
	let userdata_path = get_rhyolite_dir().join(USER_DATA_DIR).join(USER_DATA_FILE);

	let markdownfiles = if !userdata_path.exists() {
		load_default_trove()
	} else {
		load_from_userdata()
	};

	if markdownfiles.is_empty() {
		new_tab();
	} else {
		for file in markdownfiles {
			let insertion_index = FILES_BUFFER().len();
			push_tab(file.title.clone(), insertion_index);
			FILES_BUFFER.write().push(file);
		}
		*CURRENT_TAB.write() = Some(0);
	}
}
