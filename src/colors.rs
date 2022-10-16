use crate::args::{parse_args, Args};

use lscolors::LsColors;
use ansi_term::Style;

use std::path::Path;

// color for entry
pub struct Color {
    // ansi_term style
    style: Style,
}

impl Color {
    // create color style
    pub fn new(path: &Path) -> Self {
        let Args { colored, .. } = parse_args();

        if !colored {
            // return no color
            return Self { style: Style::new() };
        }

        let lscolors = LsColors::from_env().unwrap_or_default();

        // style with lscolors
        let style = lscolors
            .style_for_path(path)
            .map(lscolors::Style::to_ansi_term_style)
            .unwrap_or_default();

        Self { style }
    }

    // paint text using style
    pub fn paint(&self, text: String) -> String {
        self.style.paint(text).to_string()
    }
}
