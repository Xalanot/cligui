use std::process::Command;

pub fn build_help_command(args: Vec<String>) -> Command {
    let command_to_run = &args[0];
    let mut command_args: Vec<&str> = args.iter().skip(1).map(|arg| arg.as_str()).collect();
    command_args.push("--help");

        // Example: Running a command with the collected arguments
    let mut output = Command::new(command_to_run);
    output.args(command_args);
    output
}

pub fn run_help_command(mut command: Command) -> std::io::Result<String> {
    let command = command.output()?;
    if command.status.success() {
        let output = String::from_utf8_lossy(&command.stdout).to_string();
        return Ok(output);
    } else {
        let error = String::from_utf8_lossy(&command.stderr);
        panic!("Failed to retrieve help description: {error}");
    }
}

pub fn run_external_command(mut command: Command) -> std::io::Result<()> {
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

#[test]
fn test_build_help_command() {
    let args = vec![String::from("greeter.exe")];

    let help_command = build_help_command(args);

    let mut expected_help_command = Command::new("greeter.exe");
    expected_help_command.arg("--help");
    assert_eq!(
        format!("{:?}", help_command),
        format!("{:?}", expected_help_command),
    )
}

#[test]
fn test_build_help_command_from_multiple_args() {
    let args = vec![String::from("python"), String::from("greeter.py")];

    let help_command = build_help_command(args);

    let mut expected_help_command = Command::new("python");
    expected_help_command.args(vec![String::from("greeter.py"), String::from("--help")]);
    assert_eq!(
        format!("{:?}", help_command),
        format!("{:?}", expected_help_command),
    )
}