use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend, crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, layout::{Alignment, Rect}, style::Stylize, text::Line, widgets::{block::{Position, Title}, Block, Borders, Paragraph, List, ListState}, Frame, Terminal,
    style::{Style, Modifier}
};

use crate::{
    model::{Model, Section},
    parsing::{
        CLIArgument,
        CLIFlag,
    }
};

mod layout;

use layout::UILayout;

/// Trait for items being displayed in the gui
pub trait GUIDisplay {
    fn display_list(&self) -> String;
    fn display_description(&self) -> Option<String>;
}

impl GUIDisplay for CLIArgument {
    fn display_list(&self) -> String {
        format!("{}: {}", self.name, self.default_value.as_deref().unwrap_or(""))
    }

    fn display_description(&self) -> Option<String> {
        Some(format!("{}: {}", self.name, self.description.as_deref()?))
    }
}

impl GUIDisplay for CLIFlag {
    fn display_list(&self) -> String {
        let checkbox = if self.set {String::from("[x]")} else {String::from("[ ]")};
        format!("{checkbox} {}", self.name())
    }

    fn display_description(&self) -> Option<String> {
        Some(format!("{}: {}", self.name(), self.description.as_deref()?))
    }
}

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Render main border
fn render_main_border(frame: &mut Frame, title: &str) {
    let title = Title::from(title.bold());
    let instructions = Title::from(Line::from(vec![
        " Run ".into(),
        "<Enter>".blue().into(),
        " Quit ".into(),
        "<Ctrl + Q> ".blue().into(),
    ]));
    let block = Block::bordered()
        .title(title)
        .title(instructions.alignment(Alignment::Center).position(Position::Bottom));
    frame.render_widget(block, frame.size());
}

/// Render additional layout lines
fn render_layout(frame: &mut Frame, layout: &UILayout) {
    let vertical_line = Block::default()
        .borders(Borders::RIGHT);
    frame.render_widget(vertical_line.clone(), layout.left_third);
    frame.render_widget(vertical_line, layout.middle_third);
}

fn render_parameters_section<T: GUIDisplay>(frame: &mut Frame, parameters: &Vec<T>, selected_index: Option<usize>, title: &str, area: Rect) {
    let mut state = ListState::default().with_selected(selected_index);
    let items: Vec<String> = parameters
        .iter()
        .map(|argument| argument.display_list())
        .collect();
    let list = List::new(items)
        .block(Block::default().title(title).title_alignment(Alignment::Center))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    frame.render_stateful_widget(list, area, &mut state);
}

fn render_description(frame: &mut Frame, model: &Model, area: Rect) {
    let description = model.get_selected_description();
    if let Some(description) = description {
        frame.render_widget(Paragraph::new(description), area);
    }
}

/// Render a frame on the terminal
pub fn render_frame(frame: &mut Frame, model: &Model) {
    let layout = layout::UILayout::build(frame.size(), model);
    render_layout(frame, &layout);
    render_parameters_section(frame, &model.parameters.arguments, model.get_selected_index(Section::Arguments), "Arguments", layout.argument_section);
    render_parameters_section(frame, &model.parameters.flags, model.get_selected_index(Section::Flags), "Flags", layout.flag_section);
    render_description(frame, model, layout.description_section);
    render_main_border(frame, &model.title);
}