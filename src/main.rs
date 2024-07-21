use std::{
    env,
    io,
};

use model::Model;

mod parsing;
mod ui;
mod app;
mod model;
mod controller;
mod cli;

fn main() -> io::Result<()> {
    // setup
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        panic!("No arguments provided")
    }
    let help_command = cli::build_help_command(args);
    let help_string = cli::run_help_command(help_command)?;
    let parameters = parsing::parse_help_string(&help_string);
    let mut model = Model::new(parameters.expect("Cannot parse the help string"));
    let mut terminal = ui::init()?;

    // main loop
    let cli_command = app::run(&mut terminal, &mut model)?;

    // run actual cli
    ui::restore()?;
    if let Some(cli_command) = cli_command {
        println!("Call command: {:?}", cli_command);
        cli::run_external_command(cli_command)?;
    }
    Ok(())
}
