use std::{
	fs::{self, File},
	io::Write,
};

const THEMES_DIR: &str = "./app_themes";
const OUT_FILE: &str = "./src/build/themes_build.rs";

fn main() {
	// Re-run this script whenever any file or folder changes in app_themes
	println!("cargo:rerun-if-changed={}", THEMES_DIR);

	std::fs::create_dir_all("./src/build").expect("Could not create src/build directory");

	let mut out = File::create(OUT_FILE).unwrap();

	writeln!(&mut out, "pub const THEMES: &[(&str, &str)] = &[").unwrap();

	for entry in fs::read_dir(THEMES_DIR).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		if path.extension().and_then(|e| e.to_str()) == Some("toml") {
			let fname = path.file_name().unwrap().to_str().unwrap();
			writeln!(
				&mut out,
				"(\"{name}\", include_str!(\"../../{dir}/{name}\")),",
				name = fname,
				dir = THEMES_DIR
			)
			.unwrap();
		}
	}

	writeln!(&mut out, "];").unwrap();
}
