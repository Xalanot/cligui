use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

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
    Run,
    Quit,
}

fn handle_key_event(key: KeyEvent) -> Option<Message>{
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
        _ => return None,
    }
}

#[test]
fn test_arrow_up_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Up, KeyModifiers::empty(), KeyEventKind::Press);

    let message = handle_key_event(key);

    assert_eq!(
        message,
        Some(Message::Move(Direction::Up))
    );
}

#[test]
fn test_enter_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Enter, KeyModifiers::empty(), KeyEventKind::Press);

    let message = handle_key_event(key);

    assert_eq!(
        message,
        Some(Message::Run)
    );
}

#[test]
fn test_ctrl_and_q_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('q'), KeyModifiers::CONTROL, KeyEventKind::Press);

    let message = handle_key_event(key);

    assert_eq!(
        message,
        Some(Message::Quit)
    );
}

#[test]
fn test_ctrl_and_upper_q_pressed() {
    let key = KeyEvent::new_with_kind(KeyCode::Char('Q'), KeyModifiers::CONTROL, KeyEventKind::Press);

    let message = handle_key_event(key);

    assert_eq!(
        message,
        Some(Message::Quit)
    );
}
