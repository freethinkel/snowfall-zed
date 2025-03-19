use std::fmt::Display;

use crate::color::Color;

#[derive(Debug, Clone)]
pub enum Brightness {
    Light,
    Dark,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ThemeDiagnostic {
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

#[derive(Clone)]
pub struct ThemeGit {
    pub added: Color,
    pub modified: Color,
    pub removed: Color,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Theme {
    pub is_background_syntax: bool,
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

pub struct ThemeCreatePayload {
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

impl Theme {
    pub fn new(payload: ThemeCreatePayload) -> Self {
        Theme {
            is_background_syntax: false,
            name: payload.name,
            brightness: payload.brightness,
            accent: payload.accent,
            background: payload.background,
            foreground: payload.foreground,
            tokens: payload.tokens,
            terminal: payload.terminal,
            diagnostics: payload.diagnostics,
            git: payload.git,
        }
    }
    pub fn to_bg_syntax(&self) -> Self {
        Theme {
            is_background_syntax: true,
            name: format!("{} (background)", self.name),
            brightness: self.brightness.clone(),
            accent: self.accent,
            background: self.background,
            foreground: self.foreground,
            tokens: self.tokens.clone(),
            terminal: self.terminal.clone(),
            diagnostics: self.diagnostics.clone(),
            git: self.git.clone(),
        }
    }
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
