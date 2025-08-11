use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "ocean_breeze",
        Theme {
            name: "ocean_breeze".to_string(),
            root_content: r#"--background: oklch(0.92 0.05 220);
  --foreground: oklch(0.1 0.06 220);
  --card: oklch(0.88 0.06 220);
  --card-foreground: oklch(0.15 0.05 220);
  --popover: oklch(0.83 0.07 220);
  --popover-foreground: oklch(0.2 0.05 220);
  --primary: oklch(0.65 0.25 200);
  --primary-foreground: oklch(0.95 0.02 220);
  --secondary: oklch(0.75 0.15 220);
  --secondary-foreground: oklch(0.1 0.06 220);
  --muted: oklch(0.8 0.08 220);
  --muted-foreground: oklch(0.35 0.09 220);
  --accent: oklch(0.7 0.2 200);
  --accent-foreground: oklch(0.95 0.02 220);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 220);
  --border: oklch(0.75 0.09 220);
  --input: oklch(0.75 0.09 220);
  --ring: oklch(0.65 0.25 200);
  --chart-1: oklch(0.7 0.2 180);
  --chart-2: oklch(0.6 0.2 220);
  --chart-3: oklch(0.5 0.2 260);
  --chart-4: oklch(0.8 0.1 140);
  --chart-5: oklch(0.6 0.1 300);
  --sidebar: oklch(0.88 0.06 220);
  --sidebar-foreground: oklch(0.15 0.05 220);
  --sidebar-primary: oklch(0.65 0.25 200);
  --sidebar-primary-foreground: oklch(0.95 0.02 220);
  --sidebar-accent: oklch(0.7 0.2 200);
  --sidebar-accent-foreground: oklch(0.95 0.02 220);
  --sidebar-border: oklch(0.75 0.09 220);
  --sidebar-ring: oklch(0.65 0.25 200);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.1 0.08 220);
  --foreground: oklch(0.9 0.03 220);
  --card: oklch(0.15 0.09 220);
  --card-foreground: oklch(0.85 0.03 220);
  --popover: oklch(0.2 0.1 220);
  --popover-foreground: oklch(0.8 0.04 220);
  --primary: oklch(0.7 0.3 200);
  --primary-foreground: oklch(0.95 0.02 220);
  --secondary: oklch(0.3 0.2 220);
  --secondary-foreground: oklch(0.85 0.03 220);
  --muted: oklch(0.25 0.12 220);
  --muted-foreground: oklch(0.6 0.15 220);
  --accent: oklch(0.4 0.22 220);
  --accent-foreground: oklch(0.9 0.03 220);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 220);
  --border: oklch(0.2 0.12 220 / 30%);
  --input: oklch(0.2 0.12 220 / 40%);
  --ring: oklch(0.7 0.3 200);
  --chart-1: oklch(0.75 0.25 180);
  --chart-2: oklch(0.65 0.25 220);
  --chart-3: oklch(0.55 0.25 260);
  --chart-4: oklch(0.85 0.15 140);
  --chart-5: oklch(0.65 0.15 300);
  --sidebar: oklch(0.15 0.09 220);
  --sidebar-foreground: oklch(0.85 0.03 220);
  --sidebar-primary: oklch(0.7 0.3 200);
  --sidebar-primary-foreground: oklch(0.95 0.02 220);
  --sidebar-accent: oklch(0.4 0.22 220);
  --sidebar-accent-foreground: oklch(0.9 0.03 220);
  --sidebar-border: oklch(0.2 0.12 220 / 30%);
  --sidebar-ring: oklch(0.7 0.3 200);"#
                .to_string(),
        },
    )
}