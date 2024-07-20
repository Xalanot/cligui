use crate::parsing::CLIParameters;

#[derive(Debug, PartialEq)]
pub enum Section {
    Arguments,
    Flags,
    Options,
}

#[derive(Debug)]
pub struct Model {
    pub title: String,
    pub parameters: CLIParameters,
    pub current_section: Section,
    pub current_key_index: usize, // Use an index instead of a direct reference
    pub exit: bool,
}

use crate::ui::GUIDisplay;

impl Model {
    pub fn new(title: impl Into<String>, parameters: CLIParameters) -> Self {
        Self {
            title: title.into(),
            parameters,
            current_section: Section::Arguments,
            current_key_index: 0,
            exit: false,
        }
    }

    pub fn get_selected_description(&self) -> Option<String> {
        match self.current_section {
            Section::Arguments => return self.parameters.arguments[self.current_key_index].display_description(),
            Section::Flags => return self.parameters.flags[self.current_key_index].display_description(),
            Section::Options => return self.parameters.flags[self.current_key_index].display_description(),
        }
    }

    pub fn get_selected_index(&self, section: Section) -> Option<usize> {
        if section == self.current_section {
            return Some(self.current_key_index);
        } else {
            return None;
        }
    }
}