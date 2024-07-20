use std::process::Command;

pub fn run_external_command(mut command: Command) -> std::io::Result<()>{
    let command = command.output()?;
    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout);
        println!("{output}");
    } else {
        let error = String::from_utf8_lossy(&command.stderr);
        eprintln!("Command failed: {}", error);
    }

    Ok(())
}