use nix::sys::wait::wait;
use nix::unistd::fork;
use nix::unistd::ForkResult::{Child, Parent};

use exec;

use std::process;

// use bdrop::errors::BDError::ConnectionError;

pub fn ping() {
    unsafe {
        let pid = fork();

        match pid.expect("Fork Failed.") {
            Child => {
                // this should never return something to err unless an issue occurred
                let err = exec::Command::new("ping")
                    .args(&["-c", "3", "192.168.1.201"])
                    .exec();
                println!("Error: {}", err);
                process::exit(1);
            }
            Parent { child: _ } => {
                wait().unwrap();
                println!("Ping completed.");
            }
        }
    }
}

pub fn ssh_into() {
    unsafe {
        let pid = fork();

        match pid.expect("Fork Failed.") {
            Child => {
                // this should never return something to err unless an issue occurred
                let err = exec::Command::new("ssh")
                    .args(&["student@192.168.1.201"])
                    .exec();
                println!("Error: {}", err);
                process::exit(1);
            }
            Parent { child: _ } => {
                wait().unwrap();
                println!("Exited SSH...");
            }
        }
    }
}
