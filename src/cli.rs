use std::env;

#[derive(Debug)]
pub struct Cli {
    pub command: String,
    pub args: Vec<String>,
}

pub fn parse_args(
    commands: Vec<(String, String, String, String, &dyn Fn() -> ())>,
) -> (String, String, String, String, &dyn Fn() -> ()) {
    let default_command = commands.first().unwrap().clone();

    let args = env::args().collect::<Vec<String>>();

    let command = commands
        .iter()
        .find(|(short, long, ..)| {
            args.iter().any(|arg| {
                arg.to_string() == format!("-{}", short) || arg.to_string() == format!("--{}", long)
            })
        })
        .unwrap_or(&default_command)
        .clone();

    command
}
