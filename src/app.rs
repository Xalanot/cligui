use std::io;

use crate::ui::{Tui, render_frame};
use crate::model::Model;

pub fn run(terminal: &mut Tui, model: &Model) -> io::Result<()> {
    while !model.exit {
        terminal.draw(|frame| render_frame(frame, model))?;
    }
    Ok(())
}