use messages::{
    Message,
    Direction,
};

use crate::{
    model::{Model, Section},
    parsing::{
        CLIArgument, CLIFlag, CLILib, CLIParameters
    }
};

pub mod messages;

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Move(direction) => move_selected_index(model, direction),
        Message::TextEdit(ch) => edit_text(model, ch),
        Message::RemoveText => remove_text(model),
        Message::Toggle => toggle_flag(model),
        Message::Run => run(model),
        Message::Quit => quit(model),
    }
}

fn get_next_section(section: Section) -> Section {
    match section {
        Section::Arguments => return Section::Flags,
        Section::Flags => return Section::Options,
        Section::Options => return Section::Arguments,
    }
}

fn get_previous_section(section: Section) -> Section {
    match section {
        Section::Arguments => return Section::Options,
        Section::Flags => return Section::Arguments,
        Section::Options => return Section::Flags,
    }
}

fn set_next_section(model: &mut Model) {
    let mut possible_next_section = get_next_section(model.current_section);
    loop {
        if model.section_is_available(possible_next_section) {
            model.current_section = possible_next_section;
            model.current_key_index = 0;
            return;
        } else {
            possible_next_section = get_next_section(possible_next_section);
        }
    }
}

fn set_previous_section(model: &mut Model) {
    let mut possible_previous_section = get_previous_section(model.current_section);
    loop {
        if model.section_is_available(possible_previous_section) {
            model.current_section = possible_previous_section;
            model.current_key_index = 0;
            return;
        } else {
            possible_previous_section = get_previous_section(possible_previous_section);
        }
    }
}

fn move_selected_index(model: &mut Model, direction: Direction) {
    match direction {
        Direction::Down => {
            if model.current_key_index >= model.get_selected_parameter_len() - 1 {
                model.current_key_index = 0;
            } else {
                model.current_key_index += 1;
            }
        },
        Direction::Up => {
            if model.current_key_index <= 0 {
                model.current_key_index = model.get_selected_parameter_len() - 1;
            } else {
                model.current_key_index -= 1;
            }
        },
        Direction::Right => set_next_section(model),
        Direction::Left => set_previous_section(model),
    }
}

fn edit_text(model: &mut Model, ch: char) {
    match model.current_section {
        Section::Arguments => model.parameters.arguments[model.current_key_index].value.push(ch),
        Section::Options => model.parameters.options[model.current_key_index].value.push(ch),
        Section::Flags => (),
    };
}

fn remove_text(model: &mut Model) {
    match model.current_section {
        Section::Arguments => model.parameters.arguments[model.current_key_index].value.pop(),
        Section::Options => model.parameters.options[model.current_key_index].value.pop(),
        Section::Flags => None,
    };
}

fn toggle_flag(model: &mut Model) {
    match model.current_section {
        Section::Flags => model.parameters.flags[model.current_key_index].set = !model.parameters.flags[model.current_key_index].set,
        _ => ()
    };
}

fn run(model: &mut Model) {
    model.run = true;
} 

fn quit(model: &mut Model) {
    model.exit = true;
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
fn test_down_advances_selected_index() {
    let mut model = create_test_model();
    let message = Message::Move(Direction::Down);

    update(&mut model, message);

    assert_eq!(
        model.current_key_index,
        1
    );
}

#[test]
fn test_down_and_up_same_index() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Down));
    update(&mut model, Message::Move(Direction::Up));

    assert_eq!(
        model.current_key_index,
        0
    );
}

#[test]
fn test_down_wraps() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Down));
    update(&mut model, Message::Move(Direction::Down));

    assert_eq!(
        model.current_key_index,
        0
    );
}

#[test]
fn test_up_wraps() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Up));

    assert_eq!(
        model.current_key_index,
        1
    );
}

#[test]
fn test_right() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Right));

    assert_eq!(
        model.current_section,
        Section::Flags,
    );
}

#[test]
fn test_right_and_left_same_section() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Right));
    update(&mut model, Message::Move(Direction::Left));

    assert_eq!(
        model.current_section,
        Section::Arguments,
    );
}

#[test]
fn test_right_wraps() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Right));
    update(&mut model, Message::Move(Direction::Right));

    assert_eq!(
        model.current_section,
        Section::Arguments,
    );
}

#[test]
fn test_left_wraps() {
    let mut model = create_test_model();
    
    update(&mut model, Message::Move(Direction::Left));

    assert_eq!(
        model.current_section,
        Section::Flags,
    );
}

#[test]
fn test_text_edit() {
    let mut model = create_test_model();
    
    update(&mut model, Message::TextEdit('a'));

    assert_eq!(
        model.parameters.arguments[0].value,
        "a",
    );
}

#[test]
fn test_remove_text() {
    let mut model = create_test_model();
    model.parameters.arguments[0].value.push('a');
    
    update(&mut model, Message::RemoveText);

    assert_eq!(
        model.parameters.arguments[0].value,
        "",
    );
}

#[test]
fn test_toggle_flag() {
    let mut model = create_test_model();
    model.current_section = Section::Flags;
    
    update(&mut model, Message::Toggle);

    assert_eq!(
        model.parameters.flags[0].set,
        true,
    );
}

#[test]
fn test_run() {
    let mut model = create_test_model();
    let message = Message::Run;

    update(&mut model, message);

    assert!(model.run);
}

#[test]
fn test_quit() {
    let mut model = create_test_model();
    let message = Message::Quit;

    update(&mut model, message);

    assert!(model.exit);
}
