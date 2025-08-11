use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "sunset_glow",
        Theme {
            name: "sunset_glow".to_string(),
            root_content: r#"--background: oklch(0.95 0.07 40);
  --foreground: oklch(0.15 0.06 40);
  --card: oklch(0.9 0.08 40);
  --card-foreground: oklch(0.2 0.06 40);
  --popover: oklch(0.85 0.09 40);
  --popover-foreground: oklch(0.25 0.06 40);
  --primary: oklch(0.8 0.35 40);
  --primary-foreground: oklch(0.95 0.02 40);
  --secondary: oklch(0.85 0.25 40);
  --secondary-foreground: oklch(0.15 0.06 40);
  --muted: oklch(0.85 0.1 40);
  --muted-foreground: oklch(0.4 0.12 40);
  --accent: oklch(0.75 0.3 40);
  --accent-foreground: oklch(0.95 0.02 40);
  --destructive: oklch(0.7 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 40);
  --border: oklch(0.8 0.12 40);
  --input: oklch(0.8 0.12 40);
  --ring: oklch(0.8 0.35 40);
  --chart-1: oklch(0.8 0.3 20);
  --chart-2: oklch(0.7 0.3 40);
  --chart-3: oklch(0.6 0.3 60);
  --chart-4: oklch(0.85 0.25 80);
  --chart-5: oklch(0.7 0.25 10);
  --sidebar: oklch(0.9 0.08 40);
  --sidebar-foreground: oklch(0.2 0.06 40);
  --sidebar-primary: oklch(0.8 0.35 40);
  --sidebar-primary-foreground: oklch(0.95 0.02 40);
  --sidebar-accent: oklch(0.75 0.3 40);
  --sidebar-accent-foreground: oklch(0.95 0.02 40);
  --sidebar-border: oklch(0.8 0.12 40);
  --sidebar-ring: oklch(0.8 0.35 40);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.15 0.08 40);
  --foreground: oklch(0.9 0.04 40);
  --card: oklch(0.2 0.09 40);
  --card-foreground: oklch(0.85 0.04 40);
  --popover: oklch(0.25 0.1 40);
  --popover-foreground: oklch(0.8 0.05 40);
  --primary: oklch(0.85 0.4 40);
  --primary-foreground: oklch(0.95 0.02 40);
  --secondary: oklch(0.3 0.25 40);
  --secondary-foreground: oklch(0.85 0.04 40);
  --muted: oklch(0.25 0.15 40);
  --muted-foreground: oklch(0.6 0.18 40);
  --accent: oklch(0.4 0.3 40);
  --accent-foreground: oklch(0.9 0.04 40);
  --destructive: oklch(0.8 0.35 25);
  --destructive-foreground: oklch(0.95 0.02 40);
  --border: oklch(0.25 0.12 40 / 30%);
  --input: oklch(0.25 0.12 40 / 40%);
  --ring: oklch(0.85 0.4 40);
  --chart-1: oklch(0.85 0.35 20);
  --chart-2: oklch(0.75 0.35 40);
  --chart-3: oklch(0.65 0.35 60);
  --chart-4: oklch(0.9 0.3 80);
  --chart-5: oklch(0.75 0.3 10);
  --sidebar: oklch(0.2 0.09 40);
  --sidebar-foreground: oklch(0.85 0.04 40);
  --sidebar-primary: oklch(0.85 0.4 40);
  --sidebar-primary-foreground: oklch(0.95 0.02 40);
  --sidebar-accent: oklch(0.4 0.3 40);
  --sidebar-accent-foreground: oklch(0.9 0.04 40);
  --sidebar-border: oklch(0.25 0.12 40 / 30%);
  --sidebar-ring: oklch(0.85 0.4 40);"#
                .to_string(),
        },
    )
}