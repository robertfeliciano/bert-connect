use anyhow::{anyhow, Context};
use inquire::ui::{IndexPrefix, RenderConfig};
use inquire::{InquireError, Select, Text};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, net::IpAddr, str::FromStr};

use bdrop::errors::BDError::ConfigError;
use bdrop::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: String,
    pub host: String,
    pub addr: IpAddr,
    pub port_no: u16,
}

fn menu(items: &[String]) -> String {
    Select::new("What would you like to do?", items.to_vec())
        .with_vim_mode(true)
        .prompt()
        .unwrap_or_else(|e: InquireError| e.to_string())
}

fn get_input() -> Result<Config, anyhow::Error> {
    let user = Text::new("Username:")
        .prompt()
        .context(ConfigError("Invalid username.".to_owned()))?;

    let host = Text::new("Hostname:")
        .prompt()
        .context(ConfigError("Invalid hostname.".to_owned()))?;

    let addr = IpAddr::from_str(Text::new("IP Address of host:").prompt()?.as_str())
        .context(ConfigError("Invalid IP Address.".to_owned()))?;

    let port_no: u16 = Text::new("Port number to connect to:")
        .prompt()?
        .parse::<u16>()
        .context(ConfigError("Invalid port number.".to_owned()))?;

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
    fs::write(CONFIG, serialized)?;
    Ok(())
}

fn add_server() -> Result<(), anyhow::Error> {
    let server = match get_input() {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    // file should be in ~/.config/bdrop/config.json
    let data = if let Ok(f) = fs::OpenOptions::new().read(true).open(CONFIG) {
        // if the file exists we read the data from it and add the new server to it
        let mut data: Vec<Config> = serde_json::from_reader(f)?;
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

fn server_choices(raw: &[Config]) -> (Vec<String>, HashMap<String, u16>) {
    let mut choices: Vec<String> = Vec::new();
    let mut index: u16 = 0;
    let mut indices = HashMap::new();
    raw.iter().for_each(|server| {
        let key = format!(
            "{}@{} | {}:{}",
            server.user, server.host, server.addr, server.port_no
        );
        choices.push(key);
        // need to redo the format! because it was giving me  a "use of moved value"... didn't feel like fixing it
        indices.insert(
            format!(
                "{}@{} | {}:{}",
                server.user, server.host, server.addr, server.port_no
            ),
            index,
        );
        index += 1;
    });
    (choices, indices)
}

fn get_servers() -> Result<(Vec<String>, HashMap<String, u16>, Vec<Config>), anyhow::Error> {
    let f = fs::OpenOptions::new()
        .read(true)
        .open(CONFIG)
        .context("No servers to edit.")?;
    
    let raw: Vec<Config> = serde_json::from_reader(f)?;
    let (choices, map) = server_choices(&raw);
    Ok((choices, map, raw))
}

fn edit_config() -> Result<(), anyhow::Error> {
    let (items, indices, mut configs) = match get_servers() {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let select_renderer = RenderConfig::default().with_option_index_prefix(IndexPrefix::SpacePadded);

    let choice = Select::new("Which configuration would you like to edit?", items)
        .with_render_config(select_renderer)
        .with_vim_mode(true)
        .prompt()?;

    let index = if let Some(i) = indices.get(&choice) {
        *i
    } else {
        return Err(anyhow!("Choice issue"));
    };

    let edited = get_input()?;
    configs[index as usize] = edited;

    write_to_config(configs)?;
    Ok(())
}

pub fn configure() -> Result<(), anyhow::Error> {
    match menu(&["Add new server".into(), "Change server endpoint".into()]).as_str() {
        "Add new server" => match add_server() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },

        "Change server endpoint" => match edit_config() {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },

        _ => Err(anyhow!("Menu Error.")),
    }
}
