use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "arctic_ice",
        Theme {
            name: "arctic_ice".to_string(),
            root_content: r#"--background: oklch(0.95 0.01 240);
  --foreground: oklch(0.1 0.01 240);
  --card: oklch(0.9 0.02 240);
  --card-foreground: oklch(0.15 0.01 240);
  --popover: oklch(0.85 0.03 240);
  --popover-foreground: oklch(0.2 0.01 240);
  --primary: oklch(0.85 0.15 240);
  --primary-foreground: oklch(0.95 0.01 240);
  --secondary: oklch(0.8 0.08 240);
  --secondary-foreground: oklch(0.1 0.01 240);
  --muted: oklch(0.85 0.04 240);
  --muted-foreground: oklch(0.4 0.03 240);
  --accent: oklch(0.8 0.12 240);
  --accent-foreground: oklch(0.95 0.01 240);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.01 240);
  --border: oklch(0.8 0.05 240);
  --input: oklch(0.8 0.05 240);
  --ring: oklch(0.85 0.15 240);
  --chart-1: oklch(0.85 0.1 210);
  --chart-2: oklch(0.75 0.1 240);
  --chart-3: oklch(0.65 0.1 270);
  --chart-4: oklch(0.9 0.08 180);
  --chart-5: oklch(0.7 0.08 300);
  --sidebar: oklch(0.9 0.02 240);
  --sidebar-foreground: oklch(0.15 0.01 240);
  --sidebar-primary: oklch(0.85 0.15 240);
  --sidebar-primary-foreground: oklch(0.95 0.01 240);
  --sidebar-accent: oklch(0.8 0.12 240);
  --sidebar-accent-foreground: oklch(0.95 0.01 240);
  --sidebar-border: oklch(0.8 0.05 240);
  --sidebar-ring: oklch(0.85 0.15 240);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.1 0.02 240);
  --foreground: oklch(0.9 0.01 240);
  --card: oklch(0.15 0.03 240);
  --card-foreground: oklch(0.85 0.01 240);
  --popover: oklch(0.2 0.04 240);
  --popover-foreground: oklch(0.8 0.02 240);
  --primary: oklch(0.9 0.2 240);
  --primary-foreground: oklch(0.95 0.01 240);
  --secondary: oklch(0.25 0.1 240);
  --secondary-foreground: oklch(0.85 0.01 240);
  --muted: oklch(0.2 0.06 240);
  --muted-foreground: oklch(0.6 0.08 240);
  --accent: oklch(0.3 0.15 240);
  --accent-foreground: oklch(0.9 0.01 240);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.01 240);
  --border: oklch(0.2 0.06 240 / 30%);
  --input: oklch(0.2 0.06 240 / 40%);
  --ring: oklch(0.9 0.2 240);
  --chart-1: oklch(0.9 0.15 210);
  --chart-2: oklch(0.8 0.15 240);
  --chart-3: oklch(0.7 0.15 270);
  --chart-4: oklch(0.95 0.12 180);
  --chart-5: oklch(0.75 0.12 300);
  --sidebar: oklch(0.15 0.03 240);
  --sidebar-foreground: oklch(0.85 0.01 240);
  --sidebar-primary: oklch(0.9 0.2 240);
  --sidebar-primary-foreground: oklch(0.95 0.01 240);
  --sidebar-accent: oklch(0.3 0.15 240);
  --sidebar-accent-foreground: oklch(0.9 0.01 240);
  --sidebar-border: oklch(0.2 0.06 240 / 30%);
  --sidebar-ring: oklch(0.9 0.2 240);"#
                .to_string(),
        },
    )
}