use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "golden_sand",
        Theme {
            name: "golden_sand".to_string(),
            root_content: r#"--background: oklch(0.95 0.04 80);
  --foreground: oklch(0.15 0.04 80);
  --card: oklch(0.9 0.05 80);
  --card-foreground: oklch(0.2 0.04 80);
  --popover: oklch(0.85 0.06 80);
  --popover-foreground: oklch(0.25 0.04 80);
  --primary: oklch(0.8 0.25 80);
  --primary-foreground: oklch(0.95 0.02 80);
  --secondary: oklch(0.85 0.15 80);
  --secondary-foreground: oklch(0.15 0.04 80);
  --muted: oklch(0.85 0.07 80);
  --muted-foreground: oklch(0.4 0.08 80);
  --accent: oklch(0.75 0.2 80);
  --accent-foreground: oklch(0.95 0.02 80);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 80);
  --border: oklch(0.8 0.08 80);
  --input: oklch(0.8 0.08 80);
  --ring: oklch(0.8 0.25 80);
  --chart-1: oklch(0.8 0.2 60);
  --chart-2: oklch(0.7 0.2 80);
  --chart-3: oklch(0.6 0.2 100);
  --chart-4: oklch(0.85 0.15 120);
  --chart-5: oklch(0.65 0.15 40);
  --sidebar: oklch(0.9 0.05 80);
  --sidebar-foreground: oklch(0.2 0.04 80);
  --sidebar-primary: oklch(0.8 0.25 80);
  --sidebar-primary-foreground: oklch(0.95 0.02 80);
  --sidebar-accent: oklch(0.75 0.2 80);
  --sidebar-accent-foreground: oklch(0.95 0.02 80);
  --sidebar-border: oklch(0.8 0.08 80);
  --sidebar-ring: oklch(0.8 0.25 80);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.2 0.06 80);
  --foreground: oklch(0.9 0.03 80);
  --card: oklch(0.25 0.07 80);
  --card-foreground: oklch(0.85 0.03 80);
  --popover: oklch(0.3 0.08 80);
  --popover-foreground: oklch(0.8 0.04 80);
  --primary: oklch(0.85 0.3 80);
  --primary-foreground: oklch(0.95 0.02 80);
  --secondary: oklch(0.35 0.18 80);
  --secondary-foreground: oklch(0.85 0.03 80);
  --muted: oklch(0.3 0.1 80);
  --muted-foreground: oklch(0.6 0.12 80);
  --accent: oklch(0.4 0.22 80);
  --accent-foreground: oklch(0.9 0.03 80);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 80);
  --border: oklch(0.3 0.09 80 / 30%);
  --input: oklch(0.3 0.09 80 / 40%);
  --ring: oklch(0.85 0.3 80);
  --chart-1: oklch(0.85 0.25 60);
  --chart-2: oklch(0.75 0.25 80);
  --chart-3: oklch(0.65 0.25 100);
  --chart-4: oklch(0.9 0.2 120);
  --chart-5: oklch(0.7 0.2 40);
  --sidebar: oklch(0.25 0.07 80);
  --sidebar-foreground: oklch(0.85 0.03 80);
  --sidebar-primary: oklch(0.85 0.3 80);
  --sidebar-primary-foreground: oklch(0.95 0.02 80);
  --sidebar-accent: oklch(0.4 0.22 80);
  --sidebar-accent-foreground: oklch(0.9 0.03 80);
  --sidebar-border: oklch(0.3 0.09 80 / 30%);
  --sidebar-ring: oklch(0.85 0.3 80);"#
                .to_string(),
        },
    )
}