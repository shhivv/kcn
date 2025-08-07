use clap::{Arg, Command};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode as ct_disable_raw_mode, enable_raw_mode as ct_enable_raw_mode},
};

#[derive(Debug, Clone)]
struct Theme {
    name: String,
    root_content: String,
    dark_content: String,
}

// Terminal control sequences
const CLEAR_SCREEN: &str = "\x1b[2J";
const CURSOR_HOME: &str = "\x1b[H";
const HIDE_CURSOR: &str = "\x1b[?25l";
const SHOW_CURSOR: &str = "\x1b[?25h";

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
            eprintln!("❌ Could not find CSS file: {}", css_filename);
            if !search_paths {
                eprintln!("💡 Try using --search to look in common directories (src/, app/, styles/)");
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
        for (name, _) in &themes {
            println!("  • {}", name);
        }
        return;
    }

    // Handle direct theme application or interactive mode (default)
    if let Some(theme_name) = matches.get_one::<String>("theme-name") {
        if let Err(e) = apply_theme(&css_path, &themes, theme_name) {
            eprintln!("❌ Error applying theme: {}", e);
            std::process::exit(1);
        }
        println!("✅ Applied theme: {}", theme_name);
    } else {
        // Default to arrow key navigation mode
        arrow_key_mode(&css_path, &themes);
    }
}

fn load_builtin_themes() -> HashMap<String, Theme> {
    let mut themes = HashMap::new();

    // Default theme (OKLCH format)
    themes.insert("default".to_string(), Theme {
        name: "default".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.645 0.246 16.439);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.645 0.246 16.439);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.645 0.246 16.439);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.645 0.246 16.439);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.645 0.246 16.439);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.645 0.246 16.439);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.645 0.246 16.439);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.645 0.246 16.439);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

    // Blue theme (OKLCH format)
    themes.insert("blue".to_string(), Theme {
        name: "blue".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.6 0.25 250);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.6 0.25 250);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.6 0.25 250);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.6 0.25 250);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.7 0.25 250);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.7 0.25 250);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.7 0.25 250);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.7 0.25 250);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

    // Green theme (OKLCH format)
    themes.insert("green".to_string(), Theme {
        name: "green".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.6 0.25 140);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.6 0.25 140);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.6 0.25 140);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.6 0.25 140);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.7 0.25 140);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.7 0.25 140);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.7 0.25 140);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.7 0.25 140);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

    // Purple theme (OKLCH format)
    themes.insert("purple".to_string(), Theme {
        name: "purple".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.6 0.25 280);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.6 0.25 280);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.6 0.25 280);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.6 0.25 280);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.7 0.25 280);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.7 0.25 280);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.7 0.25 280);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.7 0.25 280);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

    // Rose theme (OKLCH format)
    themes.insert("rose".to_string(), Theme {
        name: "rose".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.6 0.25 350);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.6 0.25 350);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.6 0.25 350);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.6 0.25 350);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.7 0.25 350);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.7 0.25 350);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.7 0.25 350);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.7 0.25 350);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

    // Orange theme (OKLCH format)
    themes.insert("orange".to_string(), Theme {
        name: "orange".to_string(),
        root_content: r#"--radius: 0.5rem;
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.6 0.25 60);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.967 0.001 286.375);
  --secondary-foreground: oklch(0.21 0.006 285.885);
  --muted: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.552 0.016 285.938);
  --accent: oklch(0.967 0.001 286.375);
  --accent-foreground: oklch(0.21 0.006 285.885);
  --destructive: oklch(0.577 0.245 27.325);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.92 0.004 286.32);
  --input: oklch(0.92 0.004 286.32);
  --ring: oklch(0.6 0.25 60);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.6 0.25 60);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.6 0.25 60);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
        dark_content: r#"--background: oklch(0.141 0.005 285.823);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.21 0.006 285.885);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.21 0.006 285.885);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.7 0.25 60);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.274 0.006 286.033);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.274 0.006 286.033);
  --muted-foreground: oklch(0.705 0.015 286.067);
  --accent: oklch(0.274 0.006 286.033);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.7 0.25 60);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.21 0.006 285.885);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.7 0.25 60);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.274 0.006 286.033);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.7 0.25 60);
  --font-sans: Geist Mono, monospace;
  --font-serif: Geist Mono, monospace;
  --font-mono: Geist Mono, monospace;"#.to_string(),
    });

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
        let common_paths = vec![
            "src",
            "app", 
            "styles",
            "css",
            "assets",
            "public",
            "static",
        ];

        for path in common_paths {
            let search_path = current_dir.join(path).join(filename);
            if search_path.exists() {
                return Some(search_path);
            }
        }

        // Also try nested common paths
        let nested_paths = vec![
            "src/styles",
            "src/css", 
            "app/globals",
            "styles/globals",
        ];

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
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                match key.code {
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
                }
            }
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

fn display_ui(css_path: &PathBuf, theme_names: &[String], current_index: usize) {
    // Clear screen and move cursor home; cursor visibility handled by crossterm in arrow_key_mode
    print!("{}{}", CLEAR_SCREEN, CURSOR_HOME);
    
    println!("🎨 KCN Theme Switcher - Live Preview Mode");
    println!("═══════════════════════════════════════════");
    println!("📁 CSS file: {}", css_path.display());
    println!();
    
    // Display themes with current selection highlighted
    println!("🌈 Themes:");
    for (i, name) in theme_names.iter().enumerate() {
        if i == current_index {
            println!("  ▶ \x1b[1;32m{}\x1b[0m ◀", name); // Green and bold for current
        } else {
            println!("    {}", name);
        }
    }
    
    println!();
    println!("🎯 Current: \x1b[1;36m{}\x1b[0m", theme_names[current_index]); // Cyan and bold
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
        .ok_or(format!("Theme '{}' not found", theme_name))?;

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
    }

    #[test]
    fn test_find_css_file() {
        let result = find_css_file("globals.css", false);
        assert!(result.is_some() || result.is_none());
    }
}