use rustyline::completion::{Completer, Pair};
use rustyline::{Context, Result};

pub struct IgnisCompleter {
    commands: Vec<String>,
}

impl IgnisCompleter {
    pub fn new() -> Self {
        Self {
            commands: vec![
                "cd".to_string(),
                "ls".to_string(),
                "git".to_string(),
                "cargo".to_string(),
                "echo".to_string(),
                "cat".to_string(),
                "clear".to_string(),
                "exit".to_string(),
            ],
        }
    }

    pub fn complete_command(&self, input: &str) -> Vec<String> {
        self.commands
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .cloned()
            .collect()
    }
}

impl Completer for IgnisCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let mut completions = Vec::new();
        let prefix = &line[..pos];

        for cmd in &self.commands {
            if cmd.starts_with(prefix) {
                completions.push(Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                });
            }
        }

        Ok((0, completions))
    }
}