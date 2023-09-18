use std::{error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub repo: Option<String>,
}

pub fn get_projects_from_root_folder(root_path: PathBuf) -> Result<Vec<Project>, Box<dyn Error>> {
    let mut res = Vec::new();
    for entry in std::fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().unwrap().to_str().unwrap() == "third_party"
                || path.file_name().unwrap().to_str().unwrap() == "probe"
            {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let repo = if path.join(".git").exists() {
                // Get remote url
                let output = std::process::Command::new("git")
                    .arg("remote")
                    .arg("get-url")
                    .arg("origin")
                    .current_dir(&path)
                    .output()?;
                if output.stdout.len() == 0 {
                    None
                } else {
                    Some(String::from_utf8(output.stdout)?.trim().to_string())
                }
            } else {
                None
            };
            let path = PathBuf::from(path.to_str().unwrap().replace("\\", "/"));
            res.push(Project { name, path, repo });
        }
    }
    Ok(res)
}
