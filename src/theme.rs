use std::fmt::Display;

use crate::color::Color;

#[derive(Debug)]
pub enum Brightness {
    Light,
    Dark,
}

pub struct ThemeTokens {
    pub brackets: Color,
    pub comment: Color,
    pub strings: Color,
    pub types: Color,
    pub functions: Color,
    pub properties: Color,
    pub keywords: Color,
    pub constants: Color,
    pub operators: Color,
}

pub struct ThemeDiagnostic {
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

pub struct ThemeGit {
    pub added: Color,
    pub modified: Color,
    pub removed: Color,
}

pub struct ThemeTerminal {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
    pub bright_black: Color,
    pub bright_red: Color,
    pub bright_green: Color,
    pub bright_yellow: Color,
    pub bright_blue: Color,
    pub bright_magenta: Color,
    pub bright_cyan: Color,
    pub bright_white: Color,
}

pub struct Theme {
    pub name: String,
    pub brightness: Brightness,
    pub accent: Color,
    pub background: Color,
    pub foreground: Color,
    pub tokens: ThemeTokens,
    pub terminal: ThemeTerminal,
    pub diagnostics: ThemeDiagnostic,
    pub git: ThemeGit,
}

pub struct ThemePackage {
    pub name: String,
    pub author: String,
    pub themes: Vec<Theme>,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Theme {{\
              \n\tname: {},\
              \n\tbrightness: {:#?},\
            \n}}",
            self.name, self.brightness,
        )
    }
}

impl Theme {
    pub fn secondary_bg(&self) -> Color {
        let bg = self.foreground;
        self.background.mix(bg, 0.025)
    }
    pub fn border(&self) -> Color {
        self.accent.mix(self.background, 0.9)
    }
}
