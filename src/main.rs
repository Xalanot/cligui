use std::io;

use model::Model;
use parsing::{CLIArgument, CLIFlag, CLIParameters};

mod parsing;
mod ui;
mod app;
mod model;
mod controller;

fn main() -> io::Result<()> {
    let mut terminal = ui::init()?;

    let arguments = vec![
        CLIArgument {
            key: String::from("--name"),
            name: String::from("NAME"),
            description: Some(String::from("Name to greet")),
            default_value: None,
        },
        CLIArgument {
            key: String::from("--count"),
            name: String::from("COUNT"),
            description: Some(String::from("Numeber of times to greet.")),
            default_value: Some(String::from("1")),
        }
    ];
    let flags = vec![
        CLIFlag {
            key: String::from("--help"),
            description: Some(String::from("Print help")),
            set: false
        }
    ];
    let parameters = CLIParameters {
        arguments,
        flags,
        options: Vec::new()
    };

    let model = Model::new("Demo App", parameters);
    app::run(&mut terminal, &model)?;
    ui::restore()?;
    Ok(())
}
