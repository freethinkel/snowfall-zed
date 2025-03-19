use std::{env::current_dir, fs::File, io::Write};

use crate::{
    color::Color,
    theme::{Brightness, Theme, ThemePackage},
};
use serde::Serialize;
use serde_json::{to_string_pretty, Map, Value};

pub struct Builder<'a> {
    theme: &'a ThemePackage,
}

impl<'a> Builder<'a> {
    pub fn new(theme: &'a ThemePackage) -> Self {
        return Self { theme };
    }

    pub fn build(&self) {
        let raw_theme = serde_json::json!({
          "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
          "name": self.theme.name,
          "author": self.theme.author,
          "themes": self.theme.themes,
        });

        let dir = current_dir().unwrap();
        let target_path = dir.join("../themes/theme.json");

        let mut file = File::create(target_path).unwrap();
        let _ = file
            .write(to_string_pretty(&raw_theme).unwrap().as_bytes())
            .expect("Error writing theme");
    }
}

trait SyntaxTheme {
    fn syntax(&self, background_color: bool) -> Value;
}

impl SyntaxTheme for Theme {
    fn syntax(&self, background_color: bool) -> Value {
        let bg_fields = vec![
            "string",
            "function",
            "comment",
            "comment.doc",
            "constant",
            "character",
        ];

        serde_json::json!({
            "attribute": {
              "color": self.tokens.properties,
              "font_style": null,
              "font_weight": null
            },
            "boolean": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "character": {
              "color": "#749689",
              "font_style": null,
              "font_weight": null
            },
            "comment": {
              "color": self.tokens.comment,
              "font_style": null,
              "font_weight": null
            },
            "comment.doc": {
              "color": self.tokens.comment.mix(self.foreground, 0.5),
              "font_style": null,
              "font_weight": null
            },
            "constant": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "constructor": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "embedded": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            "emphasis": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            // "emphasis.strong": {
            //   "color": "#0000ff",
            //   "font_style": null,
            //   "font_weight": null
            // },
            "function": {
              "color": self.tokens.functions,
              "font_style": null,
              "font_weight": null
            },
            "hint": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            "keyword": {
              "color": self.tokens.keywords,
              "font_style": null,
              "font_weight": null
            },
            "label": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            "link_text": {
              "color": self.tokens.functions,
              "font_style": null,
              "font_weight": null
            },
            "link_uri": {
              "color": self.tokens.strings,
              "font_style": null,
              "font_weight": null
            },
            "number": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "predictive": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            "primary": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
            },
            "property": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "punctuation": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "punctuation.bracket": {
              "color": self.tokens.brackets,
              "font_style": null,
              "font_weight": null
            },
            "punctuation.delimiter": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "punctuation.list_marker": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "punctuation.special": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "string": {
              "color": self.tokens.strings,
              "font_style": null,
              "font_weight": null
            },
            "string.escape": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "string.regex": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "string.special": {
              "color": self.tokens.strings,
              "font_style": null,
              "font_weight": null
            },
            "string.special.symbol": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "tag": {
              "color": self.tokens.functions,
              "font_style": null,
              "font_weight": null
            },
            "text.literal": {
              "color": self.foreground,
              "font_style": null,
              "font_weight": null
            },
            "title": {
              "color": self.tokens.keywords,
              "font_style": null,
              "font_weight": null
            },
            "type": {
              "color": self.tokens.types,
              "font_style": null,
              "font_weight": 600
            },
            "variable": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "variable.special": {
              "color": self.tokens.constants,
              "font_style": null,
              "font_weight": null
            },
            "variant": {
              "color": "#0000ff",
              "font_style": null,
              "font_weight": null
          }
        })
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            let value = if background_color && bg_fields.contains(&key.as_str()) {
                serde_json::json!({
                  "background_color": Color::from_hex(
                    value["color"].as_str().unwrap()
                  ).with_opacity(0.2),
                  "color": value["color"],
                  "font_style": value["font_style"],
                  "font_weight": value["font_weight"]
                })
            } else {
                serde_json::json!({
                  "color": value["color"],
                  "font_style": value["font_style"],
                  "font_weight": value["font_weight"]
                })
            };

            (key.clone(), value.clone())
        })
        .collect::<Map<String, Value>>()
        .into()
    }
}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = serde_json::json!({
          "name": self.name,
          "appearance": match self.brightness {
            Brightness::Dark => "dark",
            Brightness::Light => "light",
          },
          "style": {
            "background": self.background,
            "editor.background": self.background,
            "editor.gutter.background": self.background,
            "panel.background": self.secondary_bg(),
            "editor.foreground": self.foreground,
            "toolbar.background": self.secondary_bg(),
            "editor.subheader.background": self.background.darken(0.1),

            "surface.background": self.secondary_bg(),
            "elevated_surface.background": self.secondary_bg().mix(self.foreground, 0.05),
            "link_text.hover": self.diagnostics.info,

            "editor.document_highlight.read_background": self.accent.with_opacity(0.2),

            "drop_target.background": self.accent.with_opacity(0.15),
            "panel.focused_border": self.accent,
            "editor.line_number": self.foreground.with_opacity(0.4),
            "editor.active_line_number": self.foreground,


            "text.accent": self.accent,
            "text.muted": self.foreground.with_opacity(0.8),
            "text": self.foreground,

            "text.disabled": self.foreground.with_opacity(0.6),

            // GIT
            "created": self.git.added,
            "created.background": self.git.added.with_opacity(0.2),
            "modified": self.git.modified,
            "modified.background": self.git.modified.with_opacity(0.2),
            "deleted": self.git.removed,
            "deleted.background": self.git.removed.with_opacity(0.2),
            "conflict": self.git.removed,
            "conflict.background": self.git.removed.with_opacity(0.2),
            // VSC
            "version_control.ignored": self.foreground.with_opacity(0.5),
            "version_control.ignored_background": self.foreground.with_opacity(0.2),
            "version_control.added": self.git.added,
            "version_control.added_background": self.git.added.with_opacity(0.2),
            "version_control.modified": self.git.modified,
            "version_control.modified_background": self.git.modified.with_opacity(0.2),
            "version_control.deleted": self.git.removed,
            "version_control.deleted_background": self.git.removed.with_opacity(0.2),

            "hint": self.foreground.with_opacity(0.5),
            "hint.background": self.diagnostics.info.with_opacity(0.1),
            "hint.border": self.diagnostics.info.with_opacity(0.2),
            "predictive": self.foreground.mix(self.background, 0.4),

            "selected": self.accent,

            // DIAGNOSTICS
            "info": self.diagnostics.info,
            "info.background": self.diagnostics.info.with_opacity(0.1),
            "info.border": "#ff00000",
            "error": self.diagnostics.error,
            "accent": self.accent,
            "accent.background": self.accent.with_opacity(0.2),
            "error.background": self.diagnostics.error.with_opacity(0.2),
            "warning": self.diagnostics.warning,
            "warning.background": self.diagnostics.warning.with_opacity(0.2),
            "warning.border": self.diagnostics.warning.with_opacity(0.3),
            "success": self.git.added,
            "success.background":  self.git.added.with_opacity(0.2),
            "success.border": self.git.added.with_opacity(0.3),

            // borders
            "border": self.border(),
            "border.variant": self.border(),
            "border.focused": self.border().mix(self.accent, 0.5),
            "border.selected": self.accent,
            "border.transparent": self.background.with_opacity(0.0),
            "border.disabled": self.secondary_bg(),

            // elements
            "element.background": self.accent.with_opacity(0.2),
            "element.hover": self.accent.with_opacity(0.1),
            "element.active": self.accent.with_opacity(0.1),
            "element.selected": self.accent.with_opacity(0.1),
            "element.disabled": "#ff0000",

            "icon": "#0000ff",
            "icon.muted": "#0000ff",
            "icon.disabled": "#0000ff",
            "icon.placeholder": "#0000ff",
            "icon.accent": "#0000ff",

            // bars
            "status_bar.background": self.secondary_bg(),
            "title_bar.background": self.secondary_bg(),
            "title_bar.inactive_background": self.secondary_bg(),
            "tab_bar.background": self.secondary_bg(),
            "toolbar.background": self.background,

            // tabs
            "tab.active_background": self.background,
            "tab.inactive_background": self.secondary_bg(),


            "players": [
              {
                "background": self.accent,
                "cursor": self.accent,
                "selection": self.accent.with_opacity(0.2)
              }
            ],

            // TERMINAL
            "terminal.background": self.background,
            "terminal.foreground": self.foreground,
            "terminal.ansi.black": self.terminal.black,
            "terminal.ansi.bright_black": self.terminal.bright_black,
            "terminal.ansi.red": self.terminal.red,
            "terminal.ansi.bright_red": self.terminal.bright_red,
            "terminal.ansi.green": self.terminal.green,
            "terminal.ansi.bright_green": self.terminal.bright_green,
            "terminal.ansi.yellow": self.terminal.yellow,
            "terminal.ansi.bright_yellow": self.terminal.bright_yellow,
            "terminal.ansi.blue": self.terminal.blue,
            "terminal.ansi.bright_blue": self.terminal.bright_blue,
            "terminal.ansi.magenta": self.terminal.magenta,
            "terminal.ansi.bright_magenta": self.terminal.bright_magenta,
            "terminal.ansi.cyan": self.terminal.cyan,
            "terminal.ansi.bright_cyan": self.terminal.bright_cyan,
            "terminal.ansi.white": self.terminal.white,
            "terminal.ansi.bright_white": self.terminal.bright_white,
            "syntax": self.syntax(self.is_background_syntax)
          }
        });
        value.serialize(serializer)
    }
}
