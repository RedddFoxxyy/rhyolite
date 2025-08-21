use std::fs;
use std::path::Path;

const SRC_THEMES_DIR: &str = "./app_themes";
const APP_DATA_DIR: &str = "rhyolite";

fn main() {
	// Re-run this script whenever any file or folder changes in app_themes
	println!("cargo:rerun-if-changed={SRC_THEMES_DIR}");

	let themes_dir = {
		let Some(data) = dirs::state_dir() else {
			eprintln!("No Data directory could be found/accessed!");
			panic!("Failed to find Data directory.")
		};
		let app_data_dir = data.join(APP_DATA_DIR);

		let themes_dir = app_data_dir.join("Themes");

		if let Err(e) = fs::create_dir_all(&themes_dir) {
			eprintln!("Failed to create App Themes directory: {}", e);
		};

		themes_dir
	};

	copy_toml_files(SRC_THEMES_DIR, &themes_dir).unwrap_or_else(|e| panic!("Copy failed: {}", e));

	fs::create_dir_all("./src/build").expect("Could not create src/build directory");
}

fn copy_toml_files(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
	fs::create_dir_all(&dst)?;
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) == Some("toml") {
			let dest = dst.as_ref().join(path.file_name().unwrap());
			fs::copy(&path, dest)?;
		}
	}
	Ok(())
}
