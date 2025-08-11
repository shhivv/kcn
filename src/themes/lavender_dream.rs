use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "lavender_dream",
        Theme {
            name: "lavender_dream".to_string(),
            root_content: r#"--background: oklch(0.95 0.05 270);
  --foreground: oklch(0.1 0.05 270);
  --card: oklch(0.9 0.06 270);
  --card-foreground: oklch(0.15 0.05 270);
  --popover: oklch(0.85 0.07 270);
  --popover-foreground: oklch(0.2 0.05 270);
  --primary: oklch(0.75 0.25 270);
  --primary-foreground: oklch(0.95 0.02 270);
  --secondary: oklch(0.8 0.15 270);
  --secondary-foreground: oklch(0.1 0.05 270);
  --muted: oklch(0.85 0.08 270);
  --muted-foreground: oklch(0.4 0.09 270);
  --accent: oklch(0.7 0.2 270);
  --accent-foreground: oklch(0.95 0.02 270);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 270);
  --border: oklch(0.8 0.09 270);
  --input: oklch(0.8 0.09 270);
  --ring: oklch(0.75 0.25 270);
  --chart-1: oklch(0.75 0.2 240);
  --chart-2: oklch(0.65 0.2 270);
  --chart-3: oklch(0.55 0.2 300);
  --chart-4: oklch(0.85 0.15 210);
  --chart-5: oklch(0.65 0.15 330);
  --sidebar: oklch(0.9 0.06 270);
  --sidebar-foreground: oklch(0.15 0.05 270);
  --sidebar-primary: oklch(0.75 0.25 270);
  --sidebar-primary-foreground: oklch(0.95 0.02 270);
  --sidebar-accent: oklch(0.7 0.2 270);
  --sidebar-accent-foreground: oklch(0.95 0.02 270);
  --sidebar-border: oklch(0.8 0.09 270);
  --sidebar-ring: oklch(0.75 0.25 270);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.15 0.07 270);
  --foreground: oklch(0.9 0.04 270);
  --card: oklch(0.2 0.08 270);
  --card-foreground: oklch(0.85 0.04 270);
  --popover: oklch(0.25 0.09 270);
  --popover-foreground: oklch(0.8 0.05 270);
  --primary: oklch(0.8 0.3 270);
  --primary-foreground: oklch(0.95 0.02 270);
  --secondary: oklch(0.3 0.18 270);
  --secondary-foreground: oklch(0.85 0.04 270);
  --muted: oklch(0.25 0.12 270);
  --muted-foreground: oklch(0.6 0.15 270);
  --accent: oklch(0.4 0.22 270);
  --accent-foreground: oklch(0.9 0.04 270);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 270);
  --border: oklch(0.25 0.1 270 / 30%);
  --input: oklch(0.25 0.1 270 / 40%);
  --ring: oklch(0.8 0.3 270);
  --chart-1: oklch(0.8 0.25 240);
  --chart-2: oklch(0.7 0.25 270);
  --chart-3: oklch(0.6 0.25 300);
  --chart-4: oklch(0.85 0.2 210);
  --chart-5: oklch(0.7 0.2 330);
  --sidebar: oklch(0.2 0.08 270);
  --sidebar-foreground: oklch(0.85 0.04 270);
  --sidebar-primary: oklch(0.8 0.3 270);
  --sidebar-primary-foreground: oklch(0.95 0.02 270);
  --sidebar-accent: oklch(0.4 0.22 270);
  --sidebar-accent-foreground: oklch(0.9 0.04 270);
  --sidebar-border: oklch(0.25 0.1 270 / 30%);
  --sidebar-ring: oklch(0.8 0.3 270);"#
                .to_string(),
        },
    )
}