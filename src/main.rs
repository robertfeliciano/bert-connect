use inquire::{
    ui::{Attributes, Color, RenderConfig, StyleSheet},
    InquireError, Select,
};

pub mod config;

pub mod network;

macro_rules! reset {
    ($s:expr) => {
        println!("\x1b[2J\x1b[1;1H{}", $s)
    };
}

fn get_render_config() -> RenderConfig {
    let mut render_config = RenderConfig::default();

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightCyan);

    render_config.help_message = StyleSheet::new().with_fg(Color::LightCyan);

    render_config
}

fn menu(items: &[String]) -> String {
    Select::new("What would you like to do?", items.to_vec())
        .with_vim_mode(true)
        .with_help_message("Vim Mode enabled, enter to select, type to filter")
        .prompt()
        .unwrap_or_else(|e: InquireError| e.to_string())
}

fn main() {
    inquire::set_global_render_config(get_render_config());
    loop {
        match menu(&[
            "Send Something".into(),
            "Query Server".into(),
            "Ping Server".into(),
            "SSH".into(),
            "Configure".into(),
            "Quit".into(),
        ])
        .as_str()
        {
            "Send Something" => println!("send..."),

            "Query Server" => reset!("querying"),

            "Ping Server" => match network::ping() {
                Ok(_) => continue,
                Err(e) => eprintln!("{e}"),
            },

            "SSH" => match network::ssh() {
                Ok(_) => continue,
                Err(e) => eprintln!("{e}"),
            },

            "Configure" => match config::configure() {
                Ok(_) => continue,
                Err(e) => eprintln!("{e}"),
            },

            "Quit" => break,

            err => {
                println!("{err}");
                break;
            }
        }
    }
}
