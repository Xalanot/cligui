use std::io;

use model::Model;
use parsing::{CLIArgument, CLIFlag, CLIParameters, CLILib};

mod parsing;
mod ui;
mod app;
mod model;
mod controller;
mod cli;

fn main() -> io::Result<()> {
    let mut terminal = ui::init()?;

    let arguments = vec![
        CLIArgument {
            key: String::from("--first-name"),
            name: String::from("FIRST NAME"),
            description: Some(String::from("First name of the person to greet")),
            value: String::new(),
        },
        CLIArgument {
            key: String::from("--last-name"),
            name: String::from("LAST NAME"),
            description: Some(String::from("Last name of the person to greet")),
            value: String::new(),
        }
    ];
    let flags = vec![
        CLIFlag {
            key: String::from("--caps"),
            description: Some(String::from("Greet in caps")),
            set: false
        },
        CLIFlag {
            key: String::from("--german"),
            description: Some(String::from("Greet in german")),
            set: false
        },
    ];
    let options = vec![
        CLIArgument {
            key: String::from("--count"),
            name: String::from("COUNT"),
            description: Some(String::from("Number of times to greet")),
            value: String::from("1"),
        }
    ];
    let parameters = CLIParameters {
        cli_name: String::from("greeter.exe"),
        arguments,
        flags,
        options: options,
        cli_lib: CLILib::Clap,
    };

    let mut model = Model::new(parameters);
    let cli_command = app::run(&mut terminal, &mut model)?;
    ui::restore()?;
    if let Some(cli_command) = cli_command {
        cli::run_external_command(cli_command)?;
    }
    Ok(())
}
