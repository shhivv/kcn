use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "deep_space",
        Theme {
            name: "deep_space".to_string(),
            root_content: r#"--background: oklch(0.95 0.02 280);
  --foreground: oklch(0.1 0.05 280);
  --card: oklch(0.9 0.03 280);
  --card-foreground: oklch(0.15 0.04 280);
  --popover: oklch(0.85 0.04 280);
  --popover-foreground: oklch(0.2 0.03 280);
  --primary: oklch(0.6 0.3 280);
  --primary-foreground: oklch(0.95 0.02 280);
  --secondary: oklch(0.8 0.1 280);
  --secondary-foreground: oklch(0.1 0.05 280);
  --muted: oklch(0.85 0.05 280);
  --muted-foreground: oklch(0.4 0.08 280);
  --accent: oklch(0.7 0.2 280);
  --accent-foreground: oklch(0.95 0.02 280);
  --destructive: oklch(0.7 0.25 25);
  --destructive-foreground: oklch(0.95 0.02 280);
  --border: oklch(0.8 0.06 280);
  --input: oklch(0.8 0.06 280);
  --ring: oklch(0.6 0.3 280);
  --chart-1: oklch(0.7 0.2 20);
  --chart-2: oklch(0.6 0.2 280);
  --chart-3: oklch(0.5 0.2 200);
  --chart-4: oklch(0.8 0.1 100);
  --chart-5: oklch(0.6 0.1 300);
  --sidebar: oklch(0.9 0.03 280);
  --sidebar-foreground: oklch(0.15 0.04 280);
  --sidebar-primary: oklch(0.6 0.3 280);
  --sidebar-primary-foreground: oklch(0.95 0.02 280);
  --sidebar-accent: oklch(0.7 0.2 280);
  --sidebar-accent-foreground: oklch(0.95 0.02 280);
  --sidebar-border: oklch(0.8 0.06 280);
  --sidebar-ring: oklch(0.6 0.3 280);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.1 0.05 280);
  --foreground: oklch(0.9 0.03 280);
  --card: oklch(0.15 0.04 280);
  --card-foreground: oklch(0.85 0.03 280);
  --popover: oklch(0.2 0.05 280);
  --popover-foreground: oklch(0.8 0.04 280);
  --primary: oklch(0.7 0.35 280);
  --primary-foreground: oklch(0.95 0.02 280);
  --secondary: oklch(0.3 0.15 280);
  --secondary-foreground: oklch(0.85 0.03 280);
  --muted: oklch(0.25 0.08 280);
  --muted-foreground: oklch(0.6 0.1 280);
  --accent: oklch(0.4 0.2 280);
  --accent-foreground: oklch(0.9 0.03 280);
  --destructive: oklch(0.75 0.3 25);
  --destructive-foreground: oklch(0.95 0.02 280);
  --border: oklch(0.2 0.08 280 / 30%);
  --input: oklch(0.2 0.08 280 / 40%);
  --ring: oklch(0.7 0.35 280);
  --chart-1: oklch(0.75 0.25 20);
  --chart-2: oklch(0.65 0.25 280);
  --chart-3: oklch(0.55 0.25 200);
  --chart-4: oklch(0.85 0.15 100);
  --chart-5: oklch(0.65 0.15 300);
  --sidebar: oklch(0.15 0.04 280);
  --sidebar-foreground: oklch(0.85 0.03 280);
  --sidebar-primary: oklch(0.7 0.35 280);
  --sidebar-primary-foreground: oklch(0.95 0.02 280);
  --sidebar-accent: oklch(0.4 0.2 280);
  --sidebar-accent-foreground: oklch(0.9 0.03 280);
  --sidebar-border: oklch(0.2 0.08 280 / 30%);
  --sidebar-ring: oklch(0.7 0.35 280);"#
                .to_string(),
        },
    )
}