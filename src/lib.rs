use colored::Colorize;
use std::fmt::Display;

pub enum Type {
    Log,
    Warning,
    Error,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Error => write!(f, "Error"),
            Type::Warning => write!(f, "Warning"),
            Type::Log => write!(f, "Log"),
        }
    }
}

pub fn log(function: &str, t: Type, content: impl std::fmt::Display) {
    let output = match t {
        Type::Error => format!("[{}]: {}", function, content).red(),
        Type::Warning => format!("[{}]: {}", function, content).yellow(),
        Type::Log =>  format!("[{}]: {}", function, content).white(),
    };

    println!("{}", output);
}
