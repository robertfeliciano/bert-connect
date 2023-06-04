use anyhow::{anyhow, Context};
use inquire::{InquireError, Select, Text};
use std::{fs, net::IpAddr, str::FromStr};

use bdrop::errors::BDError::ConfigError;
use bdrop::{get_server, Config, CONFIG};

fn menu(items: &[String]) -> String {
    Select::new("What would you like to do?", items.to_vec())
        .with_vim_mode(true)
        .prompt()
        .unwrap_or_else(|e: InquireError| e.to_string())
}

fn get_input() -> Result<Config, anyhow::Error> {
    let user = Text::new("Username:")
        .prompt()
        .context(ConfigError("Invalid username."))?;

    let host = Text::new("Hostname:")
        .prompt()
        .context(ConfigError("Invalid hostname."))?;

    let addr = IpAddr::from_str(Text::new("IP Address of host:").prompt()?.as_str())
        .context(ConfigError("Invalid IP Address."))?;

    let port_no: u16 = Text::new("Port number to connect to:")
        .prompt()?
        .parse::<u16>()
        .context(ConfigError("Invalid port number."))?;

    let new_server = Config {
        user,
        host,
        addr,
        port_no,
    };

    Ok(new_server)
}

fn write_to_config(configs: Vec<Config>) -> Result<(), anyhow::Error> {
    let serialized = serde_json::to_string_pretty(&configs)?;
    fs::write(CONFIG, serialized).context(ConfigError("Problem writing to config file"))?;
    Ok(())
}

fn add_server() -> Result<(), anyhow::Error> {
    let server = get_input()?;
    // file should be in ~/.config/bdrop/config.json
    let data = if let Ok(f) = fs::OpenOptions::new().read(true).open(CONFIG) {
        // if the file exists we read the data from it and add the new server to it
        let mut data: Vec<Config> =
            serde_json::from_reader(f).context(ConfigError("Problem reading from config file"))?;
        data.push(server);
        data
    } else {
        // otherwise we just create the file and a new vector with only the new server
        let _ = fs::File::create(CONFIG);
        vec![server]
    };
    write_to_config(data)?;
    Ok(())
}

fn edit_config() -> Result<(), anyhow::Error> {
    let (mut configs, index) = get_server()?;

    let edited = get_input()?;
    configs[index as usize] = edited;

    write_to_config(configs)?;
    Ok(())
}

pub fn configure() -> Result<(), anyhow::Error> {
    match menu(&["Add new server".into(), "Edit a configuration".into()]).as_str() {
        "Add new server" => match add_server() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },

        "Edit a configuration" => match edit_config() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },

        _ => Err(anyhow!("Menu Error.")),
    }
}
