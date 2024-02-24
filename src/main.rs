fn show_single_usage(program: String, long: String) {
    println!("\n  $ {} --{}", program, long);
}

fn show_single_option((short, long, description, ..): (String, String, String, String)) {
    println!("\n  -{}, --{}\t{}", short, long, description);
}

fn show_help(
    program: String,
    description: String,
    commands: Vec<(String, String, String, String)>,
) -> impl Fn() {
    move || {
        println!("\n{} - {}", program, description);
        println!("\nUSAGE");
        commands.iter().for_each(|(_short, long, ..)| {
            show_single_usage(program.clone(), long.clone());
        });
        println!("\nOPTIONS");
        commands.iter().for_each(|command| {
            show_single_option(command.clone());
        });
        println!("");
    }
}

fn show_version(version: String) -> impl Fn() {
    move || {
        println!("{}", version);
    }
}

fn main() {
    let program = "poc".to_string();
    let description = "Run http server locally".to_string();
    let version = "0.1.0".to_string();

    let commands: Vec<(String, String, String, String)> = vec![
        (
            "h".to_string(),
            "help".to_string(),
            "Shows this help message".to_string(),
            "".to_string(),
        ),
        (
            "v".to_string(),
            "version".to_string(),
            "Displays the current version of the application".to_string(),
            "".to_string(),
        ),
        (
            "l".to_string(),
            "listen".to_string(),
            "Specify a URI endpoint on which to listen".to_string(),
            "0.0.0.0:3000".to_string(),
        ),
    ];

    let show_current_help = show_help(
        program.to_string(),
        description.to_string(),
        commands.clone(),
    );
    let show_current_version = show_version(version.to_string());

    let closures: Vec<(String, &dyn Fn())> = vec![
        ("h".to_string(), &show_current_help),
        ("v".to_string(), &show_current_version),
        ("l".to_string(), &show_current_help),
    ];

    let available_commands = commands
        .iter()
        .map(|(short, long, desc, default)| {
            return (
                short.clone(),
                long.clone(),
                desc.clone(),
                default.clone(),
                closures.iter().find(|(s, ..)| s == short).unwrap().1,
            );
        })
        .into_iter()
        .collect();

    let command = poc::cli::parse_args(available_commands);
    let handler = command.4;

    handler();
}
