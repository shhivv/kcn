use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "fresh_mint",
        Theme {
            name: "fresh_mint".to_string(),
            root_content: r#"--background: oklch(0.95 0.04 150);
  --foreground: oklch(0.1 0.04 150);
  --card: oklch(0.9 0.05 150);
  --card-foreground: oklch(0.15 0.04 150);
  --popover: oklch(0.85 0.06 150);
  --popover-foreground: oklch(0.2 0.04 150);
  --primary: oklch(0.75 0.2 150);
  --primary-foreground: oklch(0.95 0.02 150);
  --secondary: oklch(0.8 0.1 150);
  --secondary-foreground: oklch(0.1 0.04 150);
  --muted: oklch(0.85 0.07 150);
  --muted-foreground: oklch(0.4 0.08 150);
  --accent: oklch(0.7 0.15 150);
  --accent-foreground: oklch(0.95 0.02 150);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 150);
  --border: oklch(0.8 0.08 150);
  --input: oklch(0.8 0.08 150);
  --ring: oklch(0.75 0.2 150);
  --chart-1: oklch(0.7 0.2 120);
  --chart-2: oklch(0.6 0.2 150);
  --chart-3: oklch(0.5 0.2 180);
  --chart-4: oklch(0.8 0.1 90);
  --chart-5: oklch(0.6 0.1 210);
  --sidebar: oklch(0.9 0.05 150);
  --sidebar-foreground: oklch(0.15 0.04 150);
  --sidebar-primary: oklch(0.75 0.2 150);
  --sidebar-primary-foreground: oklch(0.95 0.02 150);
  --sidebar-accent: oklch(0.7 0.15 150);
  --sidebar-accent-foreground: oklch(0.95 0.02 150);
  --sidebar-border: oklch(0.8 0.08 150);
  --sidebar-ring: oklch(0.75 0.2 150);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.1 0.06 150);
  --foreground: oklch(0.9 0.03 150);
  --card: oklch(0.15 0.07 150);
  --card-foreground: oklch(0.85 0.03 150);
  --popover: oklch(0.2 0.08 150);
  --popover-foreground: oklch(0.8 0.04 150);
  --primary: oklch(0.8 0.25 150);
  --primary-foreground: oklch(0.95 0.02 150);
  --secondary: oklch(0.3 0.15 150);
  --secondary-foreground: oklch(0.85 0.03 150);
  --muted: oklch(0.25 0.1 150);
  --muted-foreground: oklch(0.6 0.12 150);
  --accent: oklch(0.4 0.18 150);
  --accent-foreground: oklch(0.9 0.03 150);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 150);
  --border: oklch(0.2 0.1 150 / 30%);
  --input: oklch(0.2 0.1 150 / 40%);
  --ring: oklch(0.8 0.25 150);
  --chart-1: oklch(0.75 0.25 120);
  --chart-2: oklch(0.65 0.25 150);
  --chart-3: oklch(0.55 0.25 180);
  --chart-4: oklch(0.85 0.15 90);
  --chart-5: oklch(0.65 0.15 210);
  --sidebar: oklch(0.15 0.07 150);
  --sidebar-foreground: oklch(0.85 0.03 150);
  --sidebar-primary: oklch(0.8 0.25 150);
  --sidebar-primary-foreground: oklch(0.95 0.02 150);
  --sidebar-accent: oklch(0.4 0.18 150);
  --sidebar-accent-foreground: oklch(0.9 0.03 150);
  --sidebar-border: oklch(0.2 0.1 150 / 30%);
  --sidebar-ring: oklch(0.8 0.25 150);"#
                .to_string(),
        },
    )
}