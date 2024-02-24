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
    let program = String::from("poc");
    let description = String::from("Run http server locally");
    let version = String::from("0.1.0");

    let help_identifier = String::from("h");
    let version_identifier = String::from("v");
    let listen_identifier = String::from("l");

    let commands: Vec<(String, String, String, String)> = vec![
        (
            help_identifier.clone(),
            String::from("help"),
            String::from("Shows this help message"),
            String::from(""),
        ),
        (
            version_identifier.clone(),
            String::from("version"),
            String::from("Displays the current version of the application"),
            String::from(""),
        ),
        (
            listen_identifier.clone(),
            String::from("listen"),
            String::from("Specify a URI endpoint on which to listen"),
            String::from("0.0.0.0:3000"),
        ),
    ];

    let show_current_help = show_help(program, description, commands.clone());
    let show_current_version = show_version(version);

    let closures: Vec<(String, &dyn Fn())> = vec![
        (help_identifier.clone(), &show_current_help),
        (version_identifier.clone(), &show_current_version),
        (listen_identifier.clone(), &show_current_help),
    ];

    let available_commands = commands
        .iter()
        .map(|(short, long, desc, default)| {
            return (
                String::from(short),
                String::from(long),
                String::from(desc),
                String::from(default),
                closures.iter().find(|(s, ..)| s == short).unwrap().1,
            );
        })
        .into_iter()
        .collect();

    let command = poc::cli::parse_args(available_commands);
    let handler = command.4;

    handler();
}
