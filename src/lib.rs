pub mod errors;
use anyhow::{anyhow, Context};
use inquire::ui::{IndexPrefix, RenderConfig};
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, net::IpAddr};

pub static CONFIG: &str = "test.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: String,
    pub host: String,
    pub addr: IpAddr,
    pub port_no: u16,
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
pub fn get_server() -> Result<(Vec<Config>, u16), anyhow::Error> {
    let f = fs::OpenOptions::new()
        .read(true)
        .open(CONFIG)
        .context(errors::BDError::ConfigError("No servers to edit."))?;

    let raw: Vec<Config> = serde_json::from_reader(f).context(errors::BDError::ConfigError(
        "Problem reading from config file",
    ))?;
    let (choices, map) = server_choices(&raw);

    let select_renderer =
        RenderConfig::default().with_option_index_prefix(IndexPrefix::SpacePadded);

    let choice = Select::new("Which configuration would you like to edit?", choices)
        .with_render_config(select_renderer)
        .with_vim_mode(true)
        .prompt()
        .context("Problem reading choice.")?;

    let index = if let Some(i) = map.get(&choice) {
        *i
    } else {
        return Err(anyhow!("Choice issue"));
    };
    Ok((raw, index))
}
