use std::{
    io::{self, Read, Write},
    process::{Child, ExitStatus},
};

use semver_common::Alert;

pub struct Terminal {}

impl Terminal {
    pub fn read_stdout_from(child: &mut Child) -> Result<ExitStatus, Alert> {
        let mut child_stdout = child
            .stdout
            .take()
            .ok_or("There was a problem acquiring stdout from child process")?;
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = child_stdout.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            io::stdout().write_all(&buffer[..bytes_read])?;
        }
        Ok(child.wait()?)
    }
}
