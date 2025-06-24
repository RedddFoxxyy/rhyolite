use std::path::PathBuf;

use std::fs;

use super::types::APP_DATA_DIR;

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
	let documents_dir = get_rhyolite_dir();

	//Append the default trove name to the 'documents/Rhyolite path'.
	let trove_dir = documents_dir.join(trove_name);

	//Then create the path 'documents/Rhyolite/trove_name' if it does not
	fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

	//retrun the path of the default trove directory.
	trove_dir
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
