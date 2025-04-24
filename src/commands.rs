use std::process::{Command, Output};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Command execution failed")]
    ExecutionError(#[from] std::io::Error),
    #[error("Command not found")]
    NotFound,
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, command: &str) -> String {
        if command.is_empty() {
            return String::new();
        }

        match command {
            "clear" => return String::new(),
            "exit" => std::process::exit(0),
            _ => {}
        }

        #[cfg(unix)] {
            use std::os::unix::process::CommandExt;
            match Command::new("sh").arg("-c").arg(command).output() {
                Ok(output) => self.format_output(&output),
                Err(e) => format!("Error: {}", e),
            }
        }
        
        #[cfg(windows)] {
            match Command::new("cmd").arg("/C").arg(command).output() {
                Ok(output) => self.format_output(&output),
                Err(e) => format!("Error: {}", e),
            }
        }
    }

    fn format_output(&self, output: &Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        if !stderr.is_empty() {
            format!("Error:\n{}", stderr)
        } else {
            stdout.to_string()
        }
    }
}