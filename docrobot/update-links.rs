use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let folder_path = "/path/to/folder";
    let base_path = "/base/path";

    update_links(folder_path, base_path).unwrap();
}

fn update_links(folder_path: &str, base_path: &str) -> Result<(), std::io::Error> {
    let folder = Path::new(folder_path);
    let base = Path::new(base_path);

    for entry in fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let extension = path.extension().and_then(|ext| ext.to_str());

            if let Some(extension) = extension {
                if extension == "html" || extension == "css" || extension == "js" {
                    update_file_links(&path, base)?;
                }
            }
        } else if path.is_dir() {
            update_links(&path.to_string_lossy(), base_path)?;
        }
    }

    Ok(())
}

fn update_file_links(file_path: &Path, base_path: &Path) -> Result<(), std::io::Error> {
    let file_content = fs::read_to_string(file_path)?;
    let updated_content = file_content.replace("./", &base_path.to_string_lossy());

    fs::write(file_path, updated_content)?;

    Ok(())
}
