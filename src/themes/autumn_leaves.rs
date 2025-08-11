use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "autumn_leaves",
        Theme {
            name: "autumn_leaves".to_string(),
            root_content: r#"--background: oklch(0.95 0.03 60);
  --foreground: oklch(0.15 0.03 60);
  --card: oklch(0.9 0.04 60);
  --card-foreground: oklch(0.2 0.03 60);
  --popover: oklch(0.85 0.05 60);
  --popover-foreground: oklch(0.25 0.03 60);
  --primary: oklch(0.7 0.25 50);
  --primary-foreground: oklch(0.95 0.02 60);
  --secondary: oklch(0.8 0.15 60);
  --secondary-foreground: oklch(0.15 0.03 60);
  --muted: oklch(0.85 0.08 60);
  --muted-foreground: oklch(0.4 0.06 60);
  --accent: oklch(0.75 0.2 50);
  --accent-foreground: oklch(0.95 0.02 60);
  --destructive: oklch(0.6 0.2 25);
  --destructive-foreground: oklch(0.95 0.02 60);
  --border: oklch(0.8 0.07 60);
  --input: oklch(0.8 0.07 60);
  --ring: oklch(0.7 0.25 50);
  --chart-1: oklch(0.7 0.2 30);
  --chart-2: oklch(0.6 0.2 60);
  --chart-3: oklch(0.5 0.2 90);
  --chart-4: oklch(0.8 0.1 120);
  --chart-5: oklch(0.6 0.1 30);
  --sidebar: oklch(0.9 0.04 60);
  --sidebar-foreground: oklch(0.2 0.03 60);
  --sidebar-primary: oklch(0.7 0.25 50);
  --sidebar-primary-foreground: oklch(0.95 0.02 60);
  --sidebar-accent: oklch(0.75 0.2 50);
  --sidebar-accent-foreground: oklch(0.95 0.02 60);
  --sidebar-border: oklch(0.8 0.07 60);
  --sidebar-ring: oklch(0.7 0.25 50);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.15 0.05 60);
  --foreground: oklch(0.9 0.03 60);
  --card: oklch(0.2 0.06 60);
  --card-foreground: oklch(0.85 0.03 60);
  --popover: oklch(0.25 0.07 60);
  --popover-foreground: oklch(0.8 0.04 60);
  --primary: oklch(0.75 0.3 50);
  --primary-foreground: oklch(0.95 0.02 60);
  --secondary: oklch(0.3 0.18 60);
  --secondary-foreground: oklch(0.85 0.03 60);
  --muted: oklch(0.25 0.1 60);
  --muted-foreground: oklch(0.6 0.12 60);
  --accent: oklch(0.4 0.22 60);
  --accent-foreground: oklch(0.9 0.03 60);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 60);
  --border: oklch(0.25 0.09 60 / 30%);
  --input: oklch(0.25 0.09 60 / 40%);
  --ring: oklch(0.75 0.3 50);
  --chart-1: oklch(0.75 0.25 30);
  --chart-2: oklch(0.65 0.25 60);
  --chart-3: oklch(0.55 0.25 90);
  --chart-4: oklch(0.85 0.15 120);
  --chart-5: oklch(0.65 0.15 30);
  --sidebar: oklch(0.2 0.06 60);
  --sidebar-foreground: oklch(0.85 0.03 60);
  --sidebar-primary: oklch(0.75 0.3 50);
  --sidebar-primary-foreground: oklch(0.95 0.02 60);
  --sidebar-accent: oklch(0.4 0.22 60);
  --sidebar-accent-foreground: oklch(0.9 0.03 60);
  --sidebar-border: oklch(0.25 0.09 60 / 30%);
  --sidebar-ring: oklch(0.75 0.3 50);"#
                .to_string(),
        },
    )
}