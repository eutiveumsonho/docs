use std::fs;
use std::path::Path;

pub fn update_links(folder_path: &str, base_path: &str) -> Result<(), std::io::Error> {
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

    // If file is an HTML and contains the line starting with a React import, skip it
    if file_path.extension().and_then(|ext| ext.to_str()) == Some("html") {
        if file_content.contains("import") {
            return Ok(());
        }
    }

    // If import is not from the root directory, skip it
    if !file_content.contains("href=\"/") && !file_content.contains("src=\"/") {
        return Ok(());
    }

    // Add the base path in any href link terminating in .js.html
    if (file_content.contains("href=\""))
        && file_content.contains(".js.html")
    {
        let updated_content = file_content.replace("href=\"", &format!("href=\"{}", &base_path.to_string_lossy()));

        fs::write(file_path, updated_content)?;

        return Ok(());
    }

    let mut updated_content = file_content.replace("styles/", &base_path.to_string_lossy());
    updated_content = file_content.replace("scripts/", &base_path.to_string_lossy());

    fs::write(file_path, updated_content)?;

    Ok(())
}
