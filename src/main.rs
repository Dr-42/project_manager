use project_manager::{init::initial_setup, proj_handler::get_projects_from_root_folder};

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
    let conf_file = std::fs::read_to_string(&config_file)?;
    let conf_file: toml::Value = toml::from_str(&conf_file)?;
    let conf_file = conf_file.as_table().unwrap();

    let fast_working_dir = conf_file.get("fast_working_dir").unwrap().as_str().unwrap();
    let slow_working_dir = conf_file.get("slow_working_dir").unwrap().as_str().unwrap();

    let immediate_projects_root = PathBuf::from(fast_working_dir);
    let archived_projects_root = PathBuf::from(slow_working_dir);

    let immediate_projects = get_projects_from_root_folder(immediate_projects_root)?;
    let archived_projects = get_projects_from_root_folder(archived_projects_root)?;

    //display_interface(&config_file, proj_dirs.config_dir())?;
    println!("Immediate projects:");
    for project in immediate_projects {
        println!("{:#?}", project);
    }

    println!("Archived projects:");
    for project in archived_projects {
        println!("{:#?}", project);
    }
    Ok(())
}
