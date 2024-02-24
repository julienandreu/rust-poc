use colored::*;
use std::net::TcpListener;

fn show_single_usage(program: String, long: String) {
    println!("\n  $ {} --{}", program.blue(), long);
}

fn show_single_option((short, long, description, default): (String, String, String, Vec<String>)) {
    let default_value = match default.len() {
        0 => "",
        _ => default[0].as_str(),
    };

    let spaces = match format!("--{} [{}]", long, default_value).len() >= 12 {
        true => "\t\t",
        false => "\t\t\t",
    };

    match default.len() {
        0 => {
            println!("\n    -{}, --{}{}{}", short, long, spaces, description)
        }
        _ => {
            println!(
                "\n    -{}, --{} {}{}{} (default: {})",
                short,
                long,
                format!("[{}]", default[0].underline()),
                spaces,
                description,
                default[1]
            )
        }
    }
}

type Handler = dyn Fn(Vec<String>);

fn show_help(
    program: String,
    description: String,
    commands: Vec<(String, String, String, Vec<String>)>,
) -> Box<Handler> {
    Box::new(move |_| {
        println!("\n{} - {}", program.bright_blue(), description);
        println!("\nUSAGE");
        commands.iter().for_each(|(_short, long, ..)| {
            show_single_usage(program.clone(), long.clone());
        });
        println!("\nOPTIONS");
        commands.iter().for_each(|command| {
            show_single_option(command.clone());
        });
        println!("");

        std::process::exit(0);
    })
}

fn show_version(version: String) -> Box<Handler> {
    Box::new(move |_| {
        println!("{}", version);

        std::process::exit(0);
    })
}

fn start_server() -> Box<Handler> {
    Box::new(|args| {
        let addr = args.first().unwrap();
        let listener = TcpListener::bind(addr).unwrap();
        println!("Listening on: {}", addr);

        for stream in listener.incoming() {
            let _stream = stream.unwrap();

            println!("Connection established!");
        }
    })
}

fn main() {
    let program = String::from("poc");
    let description = String::from("Run http server locally");
    let version = String::from("0.1.0");

    let help_identifier = String::from("h");
    let version_identifier = String::from("v");
    let listen_identifier = String::from("l");

    let commands: Vec<(String, String, String, Vec<String>)> = vec![
        (
            help_identifier.clone(),
            String::from("help"),
            String::from("Shows this help message"),
            vec![],
        ),
        (
            version_identifier.clone(),
            String::from("version"),
            String::from("Displays the current version of the application"),
            vec![],
        ),
        (
            listen_identifier.clone(),
            String::from("listen"),
            String::from("Specify a URI endpoint on which to listen"),
            vec![String::from("uri"), String::from("0.0.0.0:3000")],
        ),
    ];

    let show_current_help = show_help(program, description, commands.clone());
    let show_current_version = show_version(version);
    let start_listening = start_server();

    let closures: Vec<(String, &Box<Handler>)> = vec![
        (help_identifier.clone(), &show_current_help),
        (version_identifier.clone(), &show_current_version),
        (listen_identifier.clone(), &start_listening),
    ];

    let available_commands = commands
        .iter()
        .map(|(short, long, desc, default)| {
            return (
                String::from(short),
                String::from(long),
                String::from(desc),
                default.clone(),
                closures.iter().find(|(s, ..)| s == short).unwrap().1,
            );
        })
        .into_iter()
        .collect();

    let (command, param) = poc::cli::parse_args(available_commands);
    let handler = command.4;

    handler(param);
}
