use std::env;

// Duplicate from main.rs
type Handler = dyn Fn(Vec<String>);

pub fn parse_args(
    commands: Vec<(String, String, String, Vec<String>, &Box<Handler>)>,
) -> (
    (String, String, String, Vec<String>, &Box<Handler>),
    Vec<String>,
) {
    let default_command = commands.first().unwrap().clone();

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    let default_command_value = String::from("");
    let mut command_index = 0;
    let mut command_value = String::from("");

    let command = commands
        .iter()
        .find(|(short, long, _desc, def, ..)| {
            let value = match def.len() {
                0 => String::from(default_command_value.clone()),
                _ => String::from(def[1].as_str()),
            };

            command_index = 0;
            command_value = String::from(value);
            args.iter().any(|arg| {
                command_index += 1;

                if arg.to_string() == format!("-{}", short)
                    || arg.to_string() == format!("--{}", long)
                {
                    command_value = args
                        .iter()
                        .nth(command_index)
                        .unwrap_or(&default_command_value)
                        .clone();
                    return true;
                }

                return false;
            })
        })
        .unwrap_or(&default_command)
        .clone();

    dbg!(&command_value);

    (command, vec![command_value])
}
