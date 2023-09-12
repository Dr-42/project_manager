use directories::ProjectDirs;
use std::{
    error::Error,
    io::{self, Write},
    path::PathBuf,
};

fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn initial_setup(proj_dirs: &ProjectDirs, config_file: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Hello new user! Let's get you set up.");
    let user_name = get_input("What is your name? : ")?;
    let fast_working_dir = get_input("Where do you want to store your projects? : ")?;
    let slow_working_dir = get_input("Where do you want to store your backups? : ")?;
    if !PathBuf::from(&fast_working_dir).exists() {
        std::fs::create_dir(fast_working_dir.clone())?;
    }

    if !PathBuf::from(&slow_working_dir).exists() {
        std::fs::create_dir(slow_working_dir.clone())?;
    }
    let config = format!(
        r#"
            user_name = "{}"
            fast_working_dir = "{}"
            slow_working_dir = "{}"
            "#,
        user_name, &fast_working_dir, &slow_working_dir
    );
    let config = toml::from_str::<toml::Value>(&config)?;
    if proj_dirs.config_dir().exists() {
        std::fs::write(&config_file, toml::to_string(&config)?)?;
    } else {
        std::fs::create_dir_all(proj_dirs.config_dir())?;
        std::fs::write(&config_file, toml::to_string(&config)?)?;
    }
    Ok(())
}
