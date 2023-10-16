use ignore::Walk;
use std::{collections::HashMap, error::Error, path::PathBuf};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub repo: Option<String>,
    pub lang: Option<HashMap<String, f64>>,
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

            println!("Getting language for {}", path.to_str().unwrap());
            let lang = get_lang_count(path.clone());
            let lang = lang.into_iter().collect::<Vec<_>>();

            let total_files = lang.iter().map(|(_, count)| count).sum::<i64>();

            let lang = lang
                .into_iter()
                .map(|(lang, count)| (lang, 100.0 * count as f64 / total_files as f64))
                .collect::<HashMap<_, _>>();

            res.push(Project {
                name,
                path,
                repo,
                lang: Some(lang),
            });
        }
    }
    Ok(res)
}

fn get_language(file: PathBuf) -> Option<String> {
    let ext = file.extension();
    let ext = match ext {
        Some(ext) => ext.to_str().unwrap(),
        None => return None,
    };

    let extr = match ext {
        "rs" => "Rust".to_string(),
        "py" => "Python".to_string(),
        "c" => "C".to_string(),
        "cpp" => "C++".to_string(),
        "h" => "C".to_string(),
        "hpp" => "C++".to_string(),
        "js" => "JavaScript".to_string(),
        "ts" => "TypeScript".to_string(),
        "html" => "HTML".to_string(),
        "css" => "CSS".to_string(),
        "toml" => "TOML".to_string(),
        "json" => "JSON".to_string(),
        "md" => "Markdown".to_string(),
        "tex" => "LaTeX".to_string(),
        "sh" => "Shell".to_string(),
        "bat" => "Batch".to_string(),
        "ps1" => "PowerShell".to_string(),
        "cmd" => "Batch".to_string(),
        "yaml" => "YAML".to_string(),
        "yml" => "YAML".to_string(),
        "xml" => "XML".to_string(),
        "java" => "Java".to_string(),
        "kt" => "Kotlin".to_string(),
        "clj" => "Clojure".to_string(),
        "go" => "Go".to_string(),
        "rb" => "Ruby".to_string(),
        "php" => "PHP".to_string(),
        "lua" => "Lua".to_string(),
        "swift" => "Swift".to_string(),
        "scala" => "Scala".to_string(),
        "sql" => "SQL".to_string(),
        "asm" => "Assembly".to_string(),
        "asmx" => "Assembly".to_string(),
        "s" => "Assembly".to_string(),
        "dart" => "Dart".to_string(),
        _ => "".to_string(),
    };
    if extr == "" {
        None
    } else {
        Some(extr)
    }
}

fn get_lang_count(folder: PathBuf) -> HashMap<String, i64> {
    let mut res = HashMap::new();
    for entry in Walk::new(folder) {
        let entry = entry.unwrap();
        let path = PathBuf::from(entry.path().to_str().unwrap().replace("\\", "/"));
        if path.is_dir() {
            continue;
        } else {
            let lang = get_language(path);
            if let Some(lag) = lang {
                res.entry(lag).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }
    res
}
