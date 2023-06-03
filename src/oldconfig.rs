use std::fs::{OpenOptions, self, File};
use std::net::IpAddr;
use std::str::FromStr;
use std::{env, path};
use std::io::Write;

use inquire::{Text, Select, InquireError, required, ui::RenderConfig};

#[derive(Debug)]
pub struct ConfigFile {
    pub user: String,
    pub host: String,
    pub addr: IpAddr,
    pub port_no: u16,
}

#[inline]
fn get_host() -> String {
    loop {
        let input = Text::new("Name of the host:")
            .with_validator(required!())
            .prompt();
        match input {
            Ok(host) => {
                return host;
            },
            Err(_) => {
                println!("An error happened when asking for the host's name, please try again.")
            },
        }
    }
}

#[inline]
fn get_addr() -> IpAddr {
    loop {
        let input = Text::new("Address of the host:")
            .with_validator(required!())
            .prompt();
        match input {
            Ok(addr) => {
                let ip_addr = IpAddr::from_str(&addr);
                match ip_addr {
                    Ok(ip) => return ip,
                    Err(_) => println!("Not a valid IP address. Try again.")
                }
            },
            Err(_) => {
                println!("An error happened when asking for the host's address, please try again.")
            },
        }
    }
}

#[inline]
fn get_port() -> u16 {
    loop {
        let input = Text{
            message: "Port you want to connect to (empty for port 22):",
            initial_value: None,
            default: None,
            placeholder: Some("22"),
            help_message: None,
            formatter: Text::DEFAULT_FORMATTER,
            validators: Vec::new(),
            page_size: Text::DEFAULT_PAGE_SIZE,
            autocompleter: None,
            render_config: RenderConfig::default(),
        }
            .prompt();
        match input {
            Ok(port) => {
                if port.is_empty() { return 22 };
                let port_no = u16::from_str(&port);
                match port_no {
                    Ok(p) => return p,
                    Err(_) => println!("Not a valid IP address. Try again.")
                }
            },
            Err(_) => {
                println!("An error happened when asking for the port number, please try again.")
            },
        }
    }
}

#[inline]
fn get_user() -> String {
    loop {
        let input = Text::new("Username:")
            .with_validator(required!())
            .prompt();
        match input {
            Ok(name) => {
                return name;
            },
            Err(_) => {
                println!("An error happened when asking for the host's name, please try again.")
            },
        }
    }
}

fn menu(items: &[String]) -> String {
    Select::new("Setup new host or change existing one?", items.to_vec())
        .with_vim_mode(true)
        .with_help_message("Vim Mode enabled, enter to select, type to filter")
        .prompt()
        .unwrap_or_else(|e: InquireError| e.to_string())
}

fn change_entry(conf: &ConfigFile, _2file: &File, _content: &str) {
    println!("Changing entry for {}@{}", conf.user, conf.addr);
}

    /**
     * check if file ~/.config/bertconnect/config.bert exists
     * create it if it doesnt
     * ask to setup new connection or change existing one
     * setup new: append this stuff to the end of the file
     * change existing: rewrite file at location of "Host" they input
     */
pub fn configure(){
    let bertconnect = format!("{}/.config/bertconnect", env::var("HOME").unwrap());
    let path_to_conf = format!("{}/.config/bertconnect/config.bert", env::var("HOME").unwrap());

    if !path::Path::new(&bertconnect).exists(){
        fs::create_dir(format!("{}/.config/bertconnect/", env::var("HOME").expect("hmm"))).expect("aint no way");
        let _conf = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path_to_conf);
    }

    let mut setup = false;
    match menu(&["Setup new".into(), "Change existing".into()]).as_str() {
        "Setup new" => setup = true,
        "Change existing" => (),
        err => println!("{err}"),
    }

    let user = get_user();
    let host = get_host();
    let addr = get_addr();
    let port_no = get_port();
    let conf = ConfigFile { user, host, addr, port_no };
    // println!("{:?}", conf);
    let conf_content = format!("===\nHost {}\n  Address {}\n  User {}\n  Port {}\n", conf.host, conf.addr, conf.user, conf.port_no);
    println!("{conf_content}");

    if setup {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&path_to_conf)
            .expect("Couldn't open file");
        if let Err(_) = writeln!(file, "{}", conf_content) { panic!("I'm tired of error checking") };
    }
    else {
        let mut file = OpenOptions::new()
            .write(true)
            .open(&path_to_conf)
            .expect("Couldn't open file");
        change_entry(&conf, &file, &conf_content);
    };
    
}