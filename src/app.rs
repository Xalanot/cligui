use std::process::Command;
use std::{
    io,
    time::Duration,
};

use ratatui::crossterm::event::{self, Event};

use crate::ui::{Tui, render_frame};
use crate::model::Model;
use crate::controller::{update, messages::{Message, handle_key_event}};
use crate::parsing::convert_to_cli;

fn handle_event(model: &Model) -> Option<Message>{
    if event::poll(Duration::from_millis(250)).unwrap() {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Press {
                return handle_key_event(key, model);
            }
        }
    }
    None
}

pub fn run(terminal: &mut Tui, model: &mut Model) -> io::Result<Option<Command>> {
    while !model.exit {
        terminal.draw(|frame| render_frame(frame, model))?;
        
        let message = handle_event(model);

        if let Some(message) = message {
            update(model, message);
        }

        if model.run {
            return Ok(Some(convert_to_cli(&model.parameters)));
        }
    }
    Ok(None)
}