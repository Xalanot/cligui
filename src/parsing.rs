use regex::Regex;

#[derive(Debug, Default, PartialEq, Eq)]
struct CLIArgument {
    key: String,
    name: String,
    description: Option<String>,
    default_value: Option<String>
}

#[derive(Debug, Default, PartialEq, Eq)]
struct CLIFlag {
    key: String,
    description: Option<String>
}

#[derive(Debug, PartialEq, Eq)]
enum CLIParameter {
    Argument(CLIArgument),
    Flag(CLIFlag),
}

#[derive(Debug, Default, PartialEq, Eq)]
struct CLIParameters {
    arguments: Vec<CLIArgument>,
    options: Vec<CLIArgument>,
    flags: Vec<CLIFlag>
}

/// Parses a help string from a CLI to determine the arguments and the options
fn parse_help_string(help_string: &str) -> Option<CLIParameters> {
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
    let argument_keys = parse_clap_usage_explanation(usage_explanation);
    let mut result = CLIParameters::default();

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
    let re = Regex::new(r"[ ]*(?P<short_key>-\w)?[ ,]*(?P<long_key>--\w+)?\s*(<(?P<name>\w+)>)?(?P<description>[ \w]+)?(\[default: (?P<default_value>.+)\])?").ok()?;
    let caps = re.captures(option_line)?;
    let key = caps.name("long_key")
        .or_else(|| caps.name("short_key"))
        .map(|k| k.as_str().to_string())?;
    let name = caps.name("name").map(|name| name.as_str().to_string());
    let description = caps.name("description").map(|description| description.as_str().trim().to_string());
    let default_value = caps.name("default_value").map(|default_value| default_value.as_str().to_string());
    if let Some(name) = name {
        Some(CLIParameter::Argument(CLIArgument {
            key,
            name,
            description,
            default_value
        }))
    } else {
        Some(CLIParameter::Flag(CLIFlag {
            key,
            description
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
fn parse_clap_usage_explanation(usage_string: &str) -> Vec<String> {
    let key_pattern = Regex::new(r"--?\w+").unwrap();
    key_pattern.find_iter(usage_string)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

// Unit tests

fn get_test_clap_help_string() -> String {
    String::from("Simple program to greet a person

Usage: clap_example.exe [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version")
}

fn get_test_clap_option_explanation() -> String {
    String::from("Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet [default: 1]
  -h, --help           Print help
  -V, --version        Print version")
}

fn get_test_clap_usage_explanation() -> String {
    String::from("Usage: clap_example.exe [OPTIONS] --name <NAME>")
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
            default_value: None
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
            default_value: Some(String::from("10")),
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
            default_value: None
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
            default_value: None
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
            default_value: None
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
            description: Some(String::from("Print help"))
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
            description: None
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
                name: String::from("NAME"),
                key: String::from("--name"),
                description: Some(String::from("Name of the person to greet")),
                default_value: None
            }),
            CLIParameter::Argument(CLIArgument {
                name: String::from("COUNT"),
                key: String::from("--count"),
                description: Some(String::from("Number of times to greet")),
                default_value: Some(String::from("1"))
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--help"),
                description: Some(String::from("Print help")),
            }),
            CLIParameter::Flag(CLIFlag {
                key: String::from("--version"),
                description: Some(String::from("Print version")),
            })
        ]
    )
}

#[test]
fn test_parse_usage_explanation() {
    let usage_string = String::from("Usage: clap_example.exe [OPTIONS] --name <NAME>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        vec!["--name"]
    )
}

#[test]
fn test_parse_usage_explanation_short_key() {
    let usage_string = String::from("Usage: clap_example.exe [OPTIONS] -n <NAME>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        vec!["-n"]
    )
}

#[test]
fn test_parse_clap_option_explanation_multiple_keys() {
    let usage_string = String::from("Usage: clap_example.exe [OPTIONS] -n <NAME> --count <COUNT>");
    
    let argument_keys = parse_clap_usage_explanation(&usage_string);

    assert_eq!(
        argument_keys,
        vec!["-n", "--count"]
    )
}

#[test]
fn parse_clap() {
    let help_string = get_test_clap_help_string();
    let cli_arguments = parse_help_string(&help_string);

    let expected_cli_arguments = Some(CLIParameters {
        arguments: vec![
            CLIArgument {
                name: String::from("NAME"),
                key: String::from("--name"),
                description: Some(String::from("Name of the person to greet")),
                default_value: None
            },
        ],
        options: vec![
            CLIArgument {
                name: String::from("COUNT"),
                key: String::from("--count"),
                description: Some(String::from("Number of times to greet")),
                default_value: Some(String::from("1"))
            }
        ],
        flags: vec![
            CLIFlag {
                key: String::from("--help"),
                description: Some(String::from("Print help")),
            },
            CLIFlag {
                key: String::from("--version"),
                description: Some(String::from("Print version"))
            }
        ]
    });
    assert_eq!(cli_arguments, expected_cli_arguments);
}
