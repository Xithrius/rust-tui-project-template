use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::Path,
};

use color_eyre::eyre::{bail, Error, Result};
use serde::Deserialize;

use crate::utils::pathing::config_path;

const CONFIG_URL: &str =
    "https://raw.githubusercontent.com/Xithrius/rust-tui-project-template/main/default-config.toml";

#[derive(Deserialize, Debug, Clone)]
pub struct CompleteConfig {
    pub terminal: TerminalConfig,
    pub frontend: FrontendConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TerminalConfig {
    pub tick_delay: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FrontendConfig {
    pub margin: u16,
}

fn download_file(url_source: &str, destination: &str) -> Result<(), ureq::Error> {
    let mut file = File::create(destination).unwrap();

    let body = ureq::get(url_source).call().unwrap().into_string().unwrap();

    file.write_all(body.as_bytes()).unwrap();

    Ok(())
}

impl CompleteConfig {
    pub fn new() -> Result<Self, Error> {
        let path_str = config_path("config.toml");

        let p = Path::new(&path_str);

        if !p.exists() {
            create_dir_all(p.parent().unwrap()).unwrap();

            download_file(CONFIG_URL, &path_str).unwrap();

            bail!("Configuration was generated at {path_str}, please fill it out with necessary information.")
        } else if let Ok(config_contents) = read_to_string(&p) {
            let config: CompleteConfig = toml::from_str(config_contents.as_str()).unwrap();

            Ok(config)
        } else {
            bail!(
                "Configuration could not be read correctly. See the following link for the example config: {}",
                format!("{}/blob/main/default-config.toml", env!("CARGO_PKG_REPOSITORY"))
            )
        }
    }
}
