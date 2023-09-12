use project_manager::init::initial_setup;

use directories::ProjectDirs;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let proj_dirs = ProjectDirs::from("com", "Dr42Apps", "project_manager");
    let proj_dirs = match proj_dirs {
        Some(proj_dirs) => proj_dirs,
        None => {
            println!("Could not get a user config directory. Exiting...\n Please report this issue at https://github.com/Dr-42/project_manager/issues/new");
            std::process::exit(1);
        }
    };
    let config_file = PathBuf::from(proj_dirs.config_dir().join("config.toml"));
    if !config_file.exists() {
        initial_setup(&proj_dirs, &config_file)?;
    }
    Ok(())
}
