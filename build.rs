use std::path::Path;
use std::{
	fs::{self, File},
	io::Write,
};

const SRC_THEMES_DIR: &str = "./app_themes";
const OUT_FILE: &str = "./src/build/themes_build.rs";
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

	let mut out = File::create(OUT_FILE).unwrap();

	writeln!(&mut out, "pub const THEMES: &[(&str, &str)] = &[").unwrap();

	for entry in fs::read_dir(SRC_THEMES_DIR).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) == Some("toml") {
			let fname = path.file_name().unwrap().to_str().unwrap();
			writeln!(&mut out, "(\"{fname}\", include_str!(\"../../{SRC_THEMES_DIR}/{fname}\")),").unwrap();
		}
	}

	writeln!(&mut out, "];").unwrap();
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
