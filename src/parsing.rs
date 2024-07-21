use std::process::Command;

use regex::Regex;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum CLILib {
    #[default]
    Clap,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CLIArgument {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub value: String
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CLIFlag {
    pub key: String,
    pub description: Option<String>,
    pub set: bool,
}

impl CLIFlag {
    pub fn name(&self) -> String {
        self.key.trim_start_matches('-').to_uppercase()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CLIParameter {
    Argument(CLIArgument),
    Flag(CLIFlag),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CLIParameters {
    pub cli_name: String,
    pub arguments: Vec<CLIArgument>,
    pub options: Vec<CLIArgument>,
    pub flags: Vec<CLIFlag>,
    pub cli_lib: CLILib,
}

/// Parses a help string from a CLI to determine the arguments and the options
pub fn parse_help_string(help_string: &str) -> Option<CLIParameters> {
    let parses_to_try = vec![
        parse_clap_help_string
    ];
    parses_to_try.iter().find_map(|parse| parse(help_string))
}

/// Parses a clap help string
fn parse_clap_help_string(help_string: &str) -> Option<CLIParameters> {
    let option_explanation = retrieve_clap_option_explanation(help_string)?;
    let parameters = parse_clap_option_explanation(option_explanation)?;
    let usage_explanation = retrieve_clap_usage_explanation(help_string)?;
    let (cli_name, argument_keys) = parse_clap_usage_explanation(usage_explanation);
    let mut result = CLIParameters::default();
    result.cli_name = cli_name;
    result.cli_lib = CLILib::Clap;

    for parameter in parameters {
        match parameter {
            CLIParameter::Argument(argument) => {
                if argument_keys.contains(&argument.key) {
                    result.arguments.push(argument);
                } else {
                    result.options.push(argument);
                }
            },
            CLIParameter::Flag(flag) => result.flags.push(flag)
        }
    }
    Some(result)
}

/// Retrieve the option explanation of a clap help string, e.g.
/// Options:
///     -n, --name <NAME> Name of the person to greet
fn retrieve_clap_option_explanation<'a>(help_string: &'a str) -> Option<&'a str> {
    let option_index = help_string.find("Options:")?;
    Some(&help_string[option_index..])
}

/// Retrieve the usage explanation of a clap help string, e.g.
/// Usage: clap_example.exe [OPTIONS] --name <NAME>
fn retrieve_clap_usage_explanation<'a>(help_string: &'a str) -> Option<&'a str> {
    help_string
        .lines()
        .find(|line| line.starts_with("Usage:"))
        .clone()
}

/// Parse a single clap option line for cli parameters
/// There exists two version of option line
/// 1. Arguments: -n, --name <NAME> Name of the person to greet [default: Me]
/// 2. Flags: -h, --help Print help
fn parse_clap_option_line(option_line: &str) -> Option<CLIParameter> {
    let re = Regex::new(r"[ ]*(?P<short_key>-\w)?[ ,]*(?P<long_key>--\w+(?:-\w+)?)?\s*(<(?P<name>\w+)>)?(?P<description>[ \w]+)?(\[default: (?P<value>.+)\])?").ok()?;
    let caps = re.captures(option_line)?;
    let key = caps.name("long_key")
        .or_else(|| caps.name("short_key"))
        .map(|k| k.as_str().to_string())?;
    let name = caps.name("name").map(|name| name.as_str().to_string());
    let description = caps.name("description").map(|description| description.as_str().trim().to_string());
    let value = caps.name("value").map(|value| value.as_str().to_string()).unwrap_or(String::new());
    if let Some(name) = name {
        Some(CLIParameter::Argument(CLIArgument {
            key,
            name,
            description,
            value,
        }))
    } else {
        Some(CLIParameter::Flag(CLIFlag {
            key,
            description,
            set: false,
        }))
    }
}

/// Parse the option explanation of a clap help string
fn parse_clap_option_explanation(option_string: &str) -> Option<Vec<CLIParameter>> {
    let parsed_options = option_string
        .lines()
        .skip(1) // Skip the "Options:" line
        .filter_map(|line| parse_clap_option_line(line))
        .collect::<Vec<CLIParameter>>();

    if parsed_options.is_empty() {
        None
    } else {
        Some(parsed_options)
    }
}

/// Parse the usage explanation of a clap help string
/// Used to distinguish between arguments and options
fn parse_clap_usage_explanation(usage_string: &str) -> (String, Vec<String>) {
    let cli_name_pattern = Regex::new(r"Usage: (?P<cli_name>[\w\.]+)").unwrap();
    let caps = cli_name_pattern.captures(usage_string).unwrap();
    let cli_name: String = caps.name("cli_name").map(|m| m.as_str().to_string()).unwrap();
    let key_pattern = Regex::new(r"--?\w+(?:-\w+)?").unwrap();
    let keys = key_pattern.find_iter(usage_string)
        .map(|mat| mat.as_str().to_string())
        .collect();
    (cli_name, keys)
}

/// Convert the parameters to an actual cli command
pub fn convert_to_cli(parameters: &CLIParameters) -> Command {
    match  parameters.cli_lib {
        CLILib::Clap => return convert_to_clap_cli(parameters),
    }
}

/// Convert the parameters to clap cli command
fn convert_to_clap_cli(parameters: &CLIParameters) -> Command {
    let mut cli_command = Command::new(parameters.cli_name.clone());
    for option in &parameters.options {
        if !option.value.is_empty() {
            cli_command.args([&option.key, &option.value]);
        }
    }
    for flag in &parameters.flags {
        if flag.set {
            cli_command.arg(&flag.key);
        }
    }
    for argument in &parameters.arguments {
        cli_command.args([&argument.key, &argument.value]);
    }
    cli_command
}

// Unit tests

#[allow(dead_code)]
fn get_test_clap_help_string() -> String {
    String::from("Simple program to greet a person

Usage: greeter.exe [OPTIONS] --first-name <FIRST_NAME> --last-name <LAST_NAME>

Options:
    -f, --first-name <FIRST_NAME>  First name of the person to greet
    -l, --last-name <LAST_NAME>    Last name of the person to greet
        --caps                     Greet in caps
        --german                   Greet in german
    -c, --count <COUNT>            Number of times to greet [default: 1]
    -h, --help                     Print help
    -V, --version                  Print version")
}

#[allow(dead_code)]
fn get_test_clap_option_explanation() -> String {
    String::from("Options:
    -f, --first-name <FIRST_NAME>  First name of the person to greet
    -l, --last-name <LAST_NAME>    Last name of the person to greet
        --caps                     Greet in caps
        --german                   Greet in german
    -c, --count <COUNT>            Number of times to greet [default: 1]
    -h, --help                     Print help
    -V, --version                  Print version")
}

#[allow(dead_code)]
fn get_test_clap_usage_explanation() -> String {
    String::from("Usage: greeter.exe [OPTIONS] --first-name <FIRST_NAME> --last-name <LAST_NAME>")
}

#[test]
fn test_retrieve_clap_option_explanation() {
    let help_string = get_test_clap_help_string();
    let usage_explanation = retrieve_clap_usage_explanation(&help_string).unwrap();

    assert_eq!(
        usage_explanation,
        get_test_clap_usage_explanation()
    )
}

#[test]
fn test_retrieve_clap_usage_explanation() {
    let help_string = get_test_clap_help_string();
    let option_explanation = retrieve_clap_option_explanation(&help_string).unwrap();

    assert_eq!(
        option_explanation,
        get_test_clap_option_explanation()
    )
}

#[test]
fn test_parse_clap_option_line() {
    let option_line = "-n, --name <NAME>    Name of the person to greet";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("NAME"),
            key: String::from("--name"),
            description: Some(String::from("Name of the person to greet")),
            value: String::new(),
        }),
    )
}

#[test]
fn test_parse_clap_option_line_multiple_words_in_key() {
    let option_line = "-n, --first-name <FIRST_NAME>    Name of the person to greet";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("FIRST_NAME"),
            key: String::from("--first-name"),
            description: Some(String::from("Name of the person to greet")),
            value: String::new(),
        }),
    )
}

#[test]
fn test_parse_clap_option_line_default_value() {
    let option_line = "-c, --count <COUNT>  Number of times to greet [default: 10]";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("COUNT"),
            key: String::from("--count"),
            description: Some(String::from("Number of times to greet")),
            value: String::from("10"),
        }),
    )
}

#[test]
fn test_parse_clap_option_line_only_short_key() {
    let option_line = "-n <NAME>    Name of the person to greet";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("NAME"),
            key: String::from("-n"),
            description: Some(String::from("Name of the person to greet")),
            value: String::new(),
        }),
    )
}

#[test]
fn test_parse_clap_option_line_only_long_key() {
    let option_line = "--name <NAME>    Name of the person to greet";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("NAME"),
            key: String::from("--name"),
            description: Some(String::from("Name of the person to greet")),
            value: String::new(),
        }),
    )
}

#[test]
fn test_parse_clap_option_missing_keys() {
    let option_line = "<NAME>    Name of the person to greet";

    let argument = parse_clap_option_line(&option_line);

    assert_eq!(
        argument,
        None,
    )
}

#[test]
fn test_parse_clap_option_without_description() {
    let option_line = "  --name <NAME>";

    let argument = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        argument,
        CLIParameter::Argument(CLIArgument {
            name: String::from("NAME"),
            key: String::from("--name"),
            description: None,
            value: String::new(),
        }),
    )
}

#[test]
fn test_parse_clap_option_flag() {
    let option_line = "  -h, --help           Print help";

    let parameter = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        parameter,
        CLIParameter::Flag(CLIFlag {
            key: String::from("--help"),
            description: Some(String::from("Print help")),
            set: false,
        })
    )
}

#[test]
fn test_parse_clap_option_flag_without_description() {
    let option_line = "  -h, --help";

    let parameter = parse_clap_option_line(&option_line).unwrap();

    assert_eq!(
        parameter,
        CLIParameter::Flag(CLIFlag {
            key: String::from("--help"),
            description: None,
            set: false,
        })
    )
}

#[test]
fn test_parse_clap_option_explanation() {
    let arguments = parse_clap_option_explanation(&get_test_clap_option_explanation()).unwrap();

    assert_eq!(
        arguments,
        vec![
            CLIParameter::Argument(CLIArgument {
                name: String::from("FIRST_NAME"),
                key: String::from("--first-name"),
                description: Some(String::from("First name of the person to greet")),
                value: String::new(),
            }),
            CLIParameter::Argument(CLIArgument {
                name: String::from("LAST_NAME"),
                key: String::from("--last-name"),
                description: Some(String::from("Last name of the person to greet")),
                value: String::new(),
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--caps"),
                description: Some(String::from("Greet in caps")),
                set: false,
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--german"),
                description: Some(String::from("Greet in german")),
                set: false,
            }),
            CLIParameter::Argument(CLIArgument {
                name: String::from("COUNT"),
                key: String::from("--count"),
                description: Some(String::from("Number of times to greet")),
                value: String::from("1"),
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--help"),
                description: Some(String::from("Print help")),
                set: false,
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--version"),
                description: Some(String::from("Print version")),
                set: false,
            }),
        ]
    )
}

#[test]
fn test_parse_usage_explanation() {
    let usage_string = String::from("Usage: greeter.exe [OPTIONS] --name <NAME>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        (String::from("greeter.exe"), vec![String::from("--name")]),
    )
}

#[test]
fn test_parse_usage_explanation_multiple_words_in_key() {
    let usage_string = String::from("Usage: greeter.exe [OPTIONS] --first-name <FIRST_NAME>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        (String::from("greeter.exe"), vec![String::from("--first-name")]),
    )
}

#[test]
fn test_parse_usage_explanation_short_key() {
    let usage_string = String::from("Usage: greeter.exe [OPTIONS] -n <FIRST_NAME>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        (String::from("greeter.exe"), vec![String::from("-n")]),
    )
}

#[test]
fn test_parse_clap_option_explanation_multiple_keys() {
    let usage_string = String::from("Usage: greeter.exe [OPTIONS] --first-name <NAME> --count <COUNT>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        (String::from("greeter.exe"), vec![String::from("--first-name"), String::from("--count")]),
    )
}

#[test]
fn parse_clap() {
    let help_string = get_test_clap_help_string();
    let cli_arguments = parse_help_string(&help_string);

    let expected_cli_arguments = Some(CLIParameters {
        cli_name: String::from("greeter.exe"),
        arguments: vec![
            CLIArgument {
                name: String::from("FIRST_NAME"),
                key: String::from("--first-name"),
                description: Some(String::from("First name of the person to greet")),
                value: String::new(),
            },
            CLIArgument {
                name: String::from("LAST_NAME"),
                key: String::from("--last-name"),
                description: Some(String::from("Last name of the person to greet")),
                value: String::new(),
            },
        ],
        options: vec![
            CLIArgument {
                name: String::from("COUNT"),
                key: String::from("--count"),
                description: Some(String::from("Number of times to greet")),
                value: String::from("1")
            }
        ],
        flags: vec![
            CLIFlag {
                key: String::from("--caps"),
                description: Some(String::from("Greet in caps")),
                set: false,
            },
            CLIFlag {
                key: String::from("--german"),
                description: Some(String::from("Greet in german")),
                set: false,
            },
            CLIFlag {
                key: String::from("--help"),
                description: Some(String::from("Print help")),
                set: false,
            },
            CLIFlag {
                key: String::from("--version"),
                description: Some(String::from("Print version")),
                set: false,
            },
        ],
        cli_lib: CLILib::Clap,
    });
    assert_eq!(cli_arguments, expected_cli_arguments);
}

#[test]
fn test_convert_to_cli() {
    let parameters = CLIParameters {
        cli_name: String::from("greeter.exe"),
        arguments: vec![
            CLIArgument {
                name: String::from("FIRST NAME"),
                key: String::from("--first-name"),
                description: Some(String::from("First name of the person to greet")),
                value: String::from("Ferris"),
            },
            CLIArgument {
                name: String::from("LAST NAME"),
                key: String::from("--last-name"),
                description: Some(String::from("Last name of the person to greet")),
                value: String::from("the Crab"),
            },
        ],
        options: vec![
            CLIArgument {
                name: String::from("COUNT"),
                key: String::from("--count"),
                description: Some(String::from("Number of times to greet")),
                value: String::from("5")
            }
        ],
        flags: vec![
            CLIFlag {
                key: String::from("--caps"),
                description: Some(String::from("Greet in caps")),
                set: true,
            },
            CLIFlag {
                key: String::from("--german"),
                description: Some(String::from("Greet in german")),
                set: false,
            },
            CLIFlag {
                key: String::from("--help"),
                description: Some(String::from("Print help")),
                set: false,
            },
            CLIFlag {
                key: String::from("--version"),
                description: Some(String::from("Print version")),
                set: false,
            },
        ],
        cli_lib: CLILib::Clap,
    };

    let cli_command = convert_to_cli(&parameters);

    let mut expected_cli_command = Command::new("greeter.exe");
    expected_cli_command.args(["--count", "5", "--caps", "--first-name", "Ferris", "--last-name", "the Crab"]);
    assert_eq!(
        format!("{:?}", cli_command),
        format!("{:?}", expected_cli_command),
    )
}
