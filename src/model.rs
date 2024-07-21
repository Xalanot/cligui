use crate::parsing::CLIParameters;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Section {
    Arguments,
    Flags,
    Options,
}

#[derive(Debug)]
pub struct Model {
    pub parameters: CLIParameters,
    pub current_section: Section,
    pub current_key_index: usize,
    pub run: bool,
    pub exit: bool,
}

use crate::ui::GUIDisplay;

impl Model {
    pub fn new(parameters: CLIParameters) -> Self {
        Self {
            parameters,
            current_section: Section::Arguments,
            current_key_index: 0,
            run: false,
            exit: false,
        }
    }

    pub fn get_selected_description(&self) -> Option<String> {
        match self.current_section {
            Section::Arguments => return self.parameters.arguments[self.current_key_index].display_description(),
            Section::Flags => return self.parameters.flags[self.current_key_index].display_description(),
            Section::Options => return self.parameters.options[self.current_key_index].display_description(),
        }
    }

    pub fn get_selected_index(&self, section: Section) -> Option<usize> {
        if section == self.current_section {
            return Some(self.current_key_index);
        } else {
            return None;
        }
    }

    pub fn get_selected_parameter_len(&self) -> usize {
        match self.current_section {
            Section::Arguments => return self.parameters.arguments.len(),
            Section::Flags => return self.parameters.flags.len(),
            Section::Options => return self.parameters.options.len(),
        }
    }

    pub fn section_is_available(&self, section: Section) -> bool {
        match section {
            Section::Arguments => return !self.parameters.arguments.is_empty(),
            Section::Flags => return !self.parameters.flags.is_empty(),
            Section::Options => return !self.parameters.options.is_empty(),
        }
    } 
}