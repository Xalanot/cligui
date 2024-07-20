use ratatui::layout::{
        Constraint, Direction, Layout, Margin, Rect
    };

use crate::model::Model;

pub struct UILayout {
    pub left_third: Rect,
    pub middle_third: Rect, 
    pub argument_section: Rect,
    pub flag_section: Rect,
    pub option_section: Rect,
    pub description_section: Rect,
}

impl UILayout {
    pub fn build(area: Rect, _model: &Model) -> UILayout {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3)
            ])
            .split(area);

        let margin = Margin {
            vertical: 3,
            horizontal: 5,
        };

        let argument_section = chunks[0].inner(margin);
        let flag_section = chunks[1].inner(margin);
        let option_section = chunks[2].inner(margin);

        // Use bottom for description section
        let description_section = Rect::new(area.x, area.height - 2, area.width, 2).inner(Margin {horizontal: 2, vertical: 0});
        
        UILayout {
            left_third: chunks[0],
            middle_third: chunks[1],
            argument_section,
            flag_section,
            option_section,
            description_section,
        }
    }
}