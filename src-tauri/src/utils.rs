use std::path::PathBuf;

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
        .expect(format!("Unable to read path: {}", path.display()).as_str());
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
