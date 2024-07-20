use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

use crate::{
    model::{Model, Section},
    parsing::{
        CLIArgument, CLIFlag, CLILib, CLIParameters
    }
};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Move(Direction),
    TextEdit(char),
    RemoveText,
    Toggle,
    Run,
    Quit,
}

pub fn handle_key_event(key: KeyEvent, model: &Model) -> Option<Message>{
    if key.kind != KeyEventKind::Press {
        return None;
    }

    match key.code {
        KeyCode::Up => return Some(Message::Move(Direction::Up)),
        KeyCode::Down => return Some(Message::Move(Direction::Down)),
        KeyCode::Left => return Some(Message::Move(Direction::Left)),
        KeyCode::Right => return Some(Message::Move(Direction::Right)),
        KeyCode::Enter => return Some(Message::Run),
        KeyCode::Char('q') | KeyCode::Char('Q') if key.modifiers == KeyModifiers::CONTROL => return Some(Message::Quit),
        KeyCode::Char(' ') if model.current_section == Section::Flags => return Some(Message::Toggle),
        KeyCode::Char(c) if model.current_section == Section::Arguments || model.current_section == Section::Options => return Some(Message::TextEdit(c)),
        KeyCode::Backspace if model.current_section == Section::Arguments || model.current_section == Section::Options => return Some(Message::RemoveText),
        _ => return None,
    }
}

#[allow(dead_code)]
fn create_test_model() -> Model {
    let arguments = vec![
        CLIArgument {
            key: String::from("--name"),
            name: String::from("NAME"),
            description: Some(String::from("Name to greet")),
            value: String::new(),
        },
        CLIArgument {
            key: String::from("--count"),
            name: String::from("COUNT"),
            description: Some(String::from("Numeber of times to greet.")),
            value: String::from("1"),
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
        cli_name: String::from("greeter.exe"),
        arguments,
        flags,
        options: Vec::new(),
        cli_lib: CLILib::Clap,
    };

    Model::new(parameters)
}

#[test]
fn test_arrow_up_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::empty(), KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::Move(Direction::Up))
    );
}

#[test]
fn test_enter_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Enter, KeyModifiers::empty(), KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::Run)
    );
}

#[test]
fn test_char_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('a'), KeyModifiers::empty(), KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::TextEdit('a'))
    );
}

#[test]
fn test_number_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('1'), KeyModifiers::empty(), KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::TextEdit('1'))
    );
}

#[test]
fn test_char_pressed_during_flag_section() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('a'), KeyModifiers::empty(), KeyEventKind::Press);
    let mut model = create_test_model();
    model.current_section = Section::Flags;

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        None
    );
}

#[test]
fn test_space_pressed_during_flag_section() {
    let key = KeyEvent::new_with_kind(KeyCode::Char(' '), KeyModifiers::empty(), KeyEventKind::Press);
    let mut model = create_test_model();
    model.current_section = Section::Flags;

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::Toggle),
    );
}

#[test]
fn test_ctrl_and_q_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('q'), KeyModifiers::CONTROL, KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::Quit)
    );
}

#[test]
fn test_ctrl_and_upper_q_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('Q'), KeyModifiers::CONTROL, KeyEventKind::Press);
    let model = create_test_model();

    let message = handle_key_event(key, &model);

    assert_eq!(
        message,
        Some(Message::Quit)
    );
}
