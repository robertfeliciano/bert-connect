use anyhow::Context;
use exec;
use nix::sys::wait::wait;
use nix::unistd::{
    fork,
    ForkResult::{Child, Parent},
};

use bdrop::errors::BDError::{ConnectionError, SystemError};
use bdrop::get_server;

pub fn ping() -> Result<(), anyhow::Error> {
    let (configs, index) = get_server()?;
    let server = &configs[index as usize];
    let addr = format!("{}", server.addr);
    
    unsafe {
        let pid = fork().context(SystemError("Fork failed"))?;

        match pid {
            Child => {
                let _ = exec::Command::new("ping")
                    .args(&["-c", "3", addr.as_str()])
                    .exec();
                return Err(ConnectionError("Ping failed").into());
            }
            Parent { child: _ } => {
                wait().context("What could go wrong here?")?;
            }
        }

        Ok(())
    }
}

pub fn ssh() -> Result<(), anyhow::Error> {
    let (configs, index) = get_server()?;
    let server = &configs[index as usize];
    let connection = format!("{}@{}:{}", server.user, server.addr, server.port_no);

    unsafe {
        let pid = fork().context(SystemError("Fork failed"))?;

        match pid {
            Child => {
                let _ = exec::Command::new("ssh")
                    .args(&[connection.as_str()])
                    .exec();
                return Err(ConnectionError("SSH failed").into());
            }
            Parent { child: _ } => {
                wait().context("What could go wrong here?")?;
            }
        }

        Ok(())
    }
}
