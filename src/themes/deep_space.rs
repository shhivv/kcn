use crate::Theme;

pub fn get() -> (&'static str, Theme) {
    (
        "deep_space",
        Theme {
            name: "deep_space".to_string(),
            root_content: r#"--background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.141 0.005 285.823);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.5 0.2 280);
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
  --ring: oklch(0.5 0.2 280);
  --chart-1: oklch(0.646 0.222 41.116);
  --chart-2: oklch(0.6 0.118 184.704);
  --chart-3: oklch(0.398 0.07 227.392);
  --chart-4: oklch(0.828 0.189 84.429);
  --chart-5: oklch(0.769 0.188 70.08);
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.141 0.005 285.823);
  --sidebar-primary: oklch(0.5 0.2 280);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.967 0.001 286.375);
  --sidebar-accent-foreground: oklch(0.21 0.006 285.885);
  --sidebar-border: oklch(0.92 0.004 286.32);
  --sidebar-ring: oklch(0.5 0.2 280);"#
                .to_string(),
            dark_content: r#"--background: oklch(0.1 0.02 280);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.15 0.015 280);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.15 0.015 280);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.6 0.25 280);
  --primary-foreground: oklch(0.969 0.015 12.422);
  --secondary: oklch(0.2 0.02 280);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.2 0.02 280);
  --muted-foreground: oklch(0.7 0.02 280);
  --accent: oklch(0.2 0.02 280);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --destructive-foreground: oklch(0.969 0.015 12.422);
  --border: oklch(0.2 0.01 280 / 20%);
  --input: oklch(0.2 0.01 280 / 25%);
  --ring: oklch(0.6 0.25 280);
  --chart-1: oklch(0.488 0.243 264.376);
  --chart-2: oklch(0.696 0.17 162.48);
  --chart-3: oklch(0.769 0.188 70.08);
  --chart-4: oklch(0.627 0.265 303.9);
  --chart-5: oklch(0.645 0.246 16.439);
  --sidebar: oklch(0.15 0.015 280);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.6 0.25 280);
  --sidebar-primary-foreground: oklch(0.969 0.015 12.422);
  --sidebar-accent: oklch(0.2 0.02 280);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(0.2 0.01 280 / 20%);
  --sidebar-ring: oklch(0.6 0.25 280);"#
                .to_string(),
        },
    )
}