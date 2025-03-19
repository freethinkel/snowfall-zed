#![recursion_limit = "2000"]

use builder::Builder;
use color::Color;
use theme::{
    Brightness, Theme, ThemeCreatePayload, ThemeDiagnostic, ThemeGit, ThemePackage, ThemeTerminal,
    ThemeTokens,
};

mod builder;
mod color;
mod theme;

fn main() {
    let dark_theme: Theme = Theme::new(ThemeCreatePayload {
        name: String::from("Snowfall dark"),
        brightness: Brightness::Dark,
        accent: Color::from_hex("#84c4df"),
        background: Color::from_hex("#24272d"),
        foreground: Color::from_hex("#91a2a6"),
        git: ThemeGit {
            added: Color::from_hex("#BDB969"),
            modified: Color::from_hex("#84c4df"),
            removed: Color::from_hex("#FA7583"),
        },
        terminal: ThemeTerminal {
            black: Color::from_hex("#1c1c1c"),
            red: Color::from_hex("#cc6666"),
            green: Color::from_hex("#bdb968"),
            yellow: Color::from_hex("#f0c674"),
            blue: Color::from_hex("#81a2be"),
            magenta: Color::from_hex("#b193ba"),
            cyan: Color::from_hex("#7fb2c8"),
            white: Color::from_hex("#c8ccd4"),
            bright_black: Color::from_hex("#636363"),
            bright_red: Color::from_hex("#a04041"),
            bright_green: Color::from_hex("#8b9440"),
            bright_yellow: Color::from_hex("#EBD2A7"),
            bright_blue: Color::from_hex("#5d7f9a"),
            bright_magenta: Color::from_hex("#82658c"),
            bright_cyan: Color::from_hex("#5e8d87"),
            bright_white: Color::from_hex("#ffffff"),
        },
        tokens: ThemeTokens {
            brackets: Color::from_hex("#7FB2C7"),
            comment: Color::from_hex("#474c54"),
            strings: Color::from_hex("#BDB969"),
            types: Color::from_hex("#ACBDC3"),
            functions: Color::from_hex("#7FB2C7"),
            properties: Color::from_hex("#EBD2A7"),
            keywords: Color::from_hex("#B08CBA"),
            constants: Color::from_hex("#EBD2A7"),
            operators: Color::from_hex("#B08CBA"),
        },
        diagnostics: ThemeDiagnostic {
            error: Color::from_hex("#FA7583"),
            warning: Color::from_hex("#EBD2A7"),
            info: Color::from_hex("#9BCAFF"),
        },
    });

    let light_theme = Theme::new(ThemeCreatePayload {
        name: String::from("Snowfall light"),
        brightness: Brightness::Light,
        background: Color::from_hex("#ffffff"),
        foreground: Color::from_hex("#5C6165"),
        accent: Color::from_hex("#88C0D0"),
        tokens: ThemeTokens {
            brackets: Color::from_hex("#50afce"),
            comment: Color::from_hex("#bdc1c8"),    // base03
            strings: Color::from_hex("#85B300"),    // base0B
            constants: Color::from_hex("#d59225"),  // base09
            keywords: Color::from_hex("#a25cb5"),   // base0E
            operators: Color::from_hex("#ACBDC3"),  // base05,
            properties: Color::from_hex("#d59225"), // base0D
            types: Color::from_hex("#5C6165"),      // base0C
            functions: Color::from_hex("#50afce"),
        },
        git: ThemeGit {
            added: Color::from_hex("#8ACB6B"),
            modified: Color::from_hex("#88C0D0"),
            removed: Color::from_hex("#FF8F9B"),
        },
        diagnostics: ThemeDiagnostic {
            error: Color::from_hex("#FA7583"),
            warning: Color::from_hex("#ec9c62"),
            info: Color::from_hex("#9BCAFF"),
        },
        terminal: ThemeTerminal {
            black: Color::from_hex("#1c1c1c"),
            red: Color::from_hex("#cc6666"),
            green: Color::from_hex("#bdb968"),
            yellow: Color::from_hex("#f0c674"),
            blue: Color::from_hex("#81a2be"),
            magenta: Color::from_hex("#b193ba"),
            cyan: Color::from_hex("#7fb2c8"),
            white: Color::from_hex("#c8ccd4"),
            bright_black: Color::from_hex("#636363"),
            bright_red: Color::from_hex("#a04041"),
            bright_green: Color::from_hex("#8b9440"),
            bright_yellow: Color::from_hex("#ec9c62"),
            bright_blue: Color::from_hex("#5d7f9a"),
            bright_magenta: Color::from_hex("#82658c"),
            bright_cyan: Color::from_hex("#5e8d87"),
            bright_white: Color::from_hex("#ffffff"),
        },
    });

    Builder::new(&ThemePackage {
        name: "Snowfall".into(),
        author: "freethinkel".into(),
        themes: vec![
            dark_theme.clone(),
            dark_theme.clone().to_bg_syntax(),
            light_theme.clone(),
            light_theme.clone().to_bg_syntax(),
        ],
    })
    .build();
}
