use colored::Colorize;
use std::fmt::Display;

pub struct Logger {
    depth: i32,
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        let name = std::any::type_name_of_val(&f);
        name.strip_suffix("::f").unwrap_or(name);
        name.split("::{{closure}}").next().unwrap_or(name)
    }};
}

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

impl Logger {
    pub fn new(depth: i32) -> Logger {
        return Logger { depth };
    }

    pub fn log(&self, function: &str, t: Type, content: impl std::fmt::Display) {
        let formatted_function: String;
        let function_arr = function.split("::").collect::<Vec<&str>>();
        if function_arr.len() > 1 && function.len() > (self.depth as usize) {
            formatted_function =
                function_arr[((function_arr.len() - 1) - (self.depth as usize))..].join("::");
        } else {
            formatted_function = function.to_string();
        }

        let output = match t {
            Type::Error => format!("[{}]: {}", formatted_function, content).red(),
            Type::Warning => format!("[{}]: {}", formatted_function, content).yellow(),
            Type::Log => format!("[{}]: {}", formatted_function, content).white(),
        };

        println!("{}", output);
    }
}

pub fn log(function: &str, t: Type, content: impl std::fmt::Display) {
    let formatted_function: String;
    let function_arr = function.split("::").collect::<Vec<&str>>();
    if function_arr.len() > 2 {
        formatted_function =
            function_arr[((function_arr.len() - 1) - 1)..].join("::");
    } else {
        formatted_function = function.to_string();
    }

    let output = match t {
        Type::Error => format!("[{}]: {}", formatted_function, content).red(),
        Type::Warning => format!("[{}]: {}", formatted_function, content).yellow(),
        Type::Log => format!("[{}]: {}", formatted_function, content).white(),
    };

    println!("{}", output);
}
