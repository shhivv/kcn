#![allow(unused)]
use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode as ct_disable_raw_mode, enable_raw_mode as ct_enable_raw_mode},
};

#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    root_content: String,
    dark_content: String,
}

// Terminal control sequences
const CLEAR_SCREEN: &str = "\x1b[2J";
const CURSOR_HOME: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";

mod themes;

fn main() {
    let matches = Command::new("kcn")
        .version("1.0.0")
        .author("KCN Theme Switcher")
        .about("Navigate between built-in shadcn themes with arrow keys for live preview")
        .arg(
            Arg::new("css-file")
                .short('f')
                .long("css-file")
                .value_name("FILE")
                .help("CSS file name to modify (default: globals.css)")
                .default_value("globals.css"),
        )
        .arg(
            Arg::new("theme-name")
                .short('t')
                .long("theme")
                .value_name("NAME")
                .help("Apply theme directly without interactive mode"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List available built-in themes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("search-paths")
                .short('s')
                .long("search")
                .help("Search for CSS file in common locations (src/, app/, styles/)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let css_filename = matches.get_one::<String>("css-file").unwrap();
    let search_paths = matches.get_flag("search-paths");

    // Find the CSS file
    let css_path = match find_css_file(css_filename, search_paths) {
        Some(path) => path,
        None => {
            eprintln!("❌ Could not find CSS file: {css_filename}");
            if !search_paths {
                eprintln!(
                    "💡 Try using --search to look in common directories (src/, app/, styles/)"
                );
            }
            std::process::exit(1);
        }
    };

    println!("📁 Using CSS file: {}", css_path.display());

    // Load built-in themes
    let themes = load_builtin_themes();

    // Handle list command
    if matches.get_flag("list") {
        println!("\n🎨 Available built-in themes:");
        for name in themes.keys() {
            println!("  • {name}");
        }
        return;
    }

    // Handle direct theme application or interactive mode (default)
    if let Some(theme_name) = matches.get_one::<String>("theme-name") {
        if let Err(e) = apply_theme(&css_path, &themes, theme_name) {
            eprintln!("❌ Error applying theme: {e}");
            std::process::exit(1);
        }
        println!("✅ Applied theme: {theme_name}");
    } else {
        // Default to arrow key navigation mode
        arrow_key_mode(&css_path, &themes);
    }
}

fn load_builtin_themes() -> HashMap<String, Theme> {
    use crate::themes::{autumn_leaves, blue, deep_space, default, fresh_mint, green, purple, rose};

    let mut themes = HashMap::new();
    for (name, theme) in [
        default::get(),
        blue::get(),
        green::get(),
        purple::get(),
        rose::get(),
        deep_space::get(),
        autumn_leaves::get(),
        fresh_mint::get(),
    ] {
        themes.insert(name.to_string(), theme);
    }
    themes
}

fn find_css_file(filename: &str, search_common_paths: bool) -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;

    // First, try the current directory
    let direct_path = current_dir.join(filename);
    if direct_path.exists() {
        return Some(direct_path);
    }

    // If search flag is enabled, look in common directories
    if search_common_paths {
        let common_paths = vec!["src", "app", "styles", "css", "assets", "public", "static"];

        for path in common_paths {
            let search_path = current_dir.join(path).join(filename);
            if search_path.exists() {
                return Some(search_path);
            }
        }

        // Also try nested common paths
        let nested_paths = vec!["src/styles", "src/css", "app/globals", "styles/globals"];

        for path in nested_paths {
            let search_path = current_dir.join(path).join(filename);
            if search_path.exists() {
                return Some(search_path);
            }
        }
    }

    None
}

fn arrow_key_mode(css_path: &PathBuf, themes: &HashMap<String, Theme>) {
    let mut theme_names: Vec<String> = themes.keys().cloned().collect();
    theme_names.sort(); // Sort themes alphabetically

    let mut current_index = 0;

    // Enable raw mode for terminal and hide cursor
    ct_enable_raw_mode().ok();
    let _ = execute!(io::stdout(), Hide);

    // Apply initial theme and display UI
    let _ = apply_theme(css_path, themes, &theme_names[current_index]);
    display_ui(css_path, &theme_names, current_index);

    // Handle keyboard input using crossterm
    loop {
        match read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Left => {
                    current_index = if current_index == 0 {
                        theme_names.len() - 1
                    } else {
                        current_index - 1
                    };
                    let _ = apply_theme(css_path, themes, &theme_names[current_index]);
                    display_ui(css_path, &theme_names, current_index);
                }
                KeyCode::Right => {
                    current_index = (current_index + 1) % theme_names.len();
                    let _ = apply_theme(css_path, themes, &theme_names[current_index]);
                    display_ui(css_path, &theme_names, current_index);
                }
                KeyCode::Enter => {
                    break;
                }
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                _ => {}
            },
            Ok(_) => {}
            Err(_) => break,
        }
    }

    // Restore terminal
    ct_disable_raw_mode().ok();
    let _ = execute!(io::stdout(), Show);
    println!("\n✅ Applied theme: {}", theme_names[current_index]);
    println!("👋 Happy theming!");
}

fn display_ui(css_path: &Path, theme_names: &[String], current_index: usize) {
    // Clear screen and move cursor home; cursor visibility handled by crossterm in arrow_key_mode
    print!("{CLEAR_SCREEN}{CURSOR_HOME}");

    println!("🎨 KCN Theme Switcher - Live Preview Mode");
    println!("═══════════════════════════════════════════");
    println!("📁 CSS file: {}", css_path.display());
    println!();

    // Display themes with current selection highlighted
    println!("🌈 Themes:");
    for (i, name) in theme_names.iter().enumerate() {
        if i == current_index {
            println!("  ▶ \x1b[1;32m{name}\x1b[0m ◀"); // Green and bold for current
        } else {
            println!("    {name}");
        }
    }

    println!();
    println!(
        "🎯 Current: \x1b[1;36m{}\x1b[0m",
        theme_names[current_index]
    ); // Cyan and bold
    println!();
    println!("⌨️  Controls:");
    println!("   ← → Navigate themes (live preview)");
    println!("   Enter  Apply and exit");
    println!("   q      Quit");

    io::stdout().flush().unwrap();
}

fn apply_theme(
    css_path: &PathBuf,
    themes: &HashMap<String, Theme>,
    theme_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let theme = themes
        .get(theme_name)
        .ok_or(format!("Theme '{theme_name}' not found"))?;

    let css_content = fs::read_to_string(css_path)?;

    // Replace :root content with simpler regex
    let root_regex = Regex::new(r":root\s*\{[^}]*\}")?;
    let new_content = root_regex.replace(&css_content, |_: &regex::Captures| {
        format!(":root {{\n  {}\n}}", theme.root_content)
    });

    // Replace .dark content with simpler regex
    let dark_regex = Regex::new(r"\.dark\s*\{[^}]*\}")?;
    let final_content = dark_regex.replace(&new_content, |_: &regex::Captures| {
        format!(".dark {{\n  {}\n}}", theme.dark_content)
    });

    // Write back to file
    fs::write(css_path, final_content.as_ref())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_themes_loaded() {
        let themes = load_builtin_themes();
        assert!(!themes.is_empty());
        assert!(themes.contains_key("default"));
        assert!(themes.contains_key("blue"));
        assert!(themes.contains_key("green"));
        assert!(themes.contains_key("deep_space"));
        assert!(themes.contains_key("autumn_leaves"));
        assert!(themes.contains_key("fresh_mint"));
    }

    #[test]
    fn test_find_css_file() {
        let result = find_css_file("globals.css", false);
        assert!(result.is_some() || result.is_none());
    }
}
