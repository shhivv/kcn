# KCN Theme Switcher

A CLI tool to switch between built-in shadcn themes with live preview.

## Features

- 13 built-in themes including default shadcn themes and custom themes
- Live preview of themes with arrow key navigation
- Apply themes directly or interactively
- Search for CSS files in common locations

## Installation


## Usage

### Interactive Mode (Default)

```bash
kcn [globals.css]
```

Navigate through themes with ↑/↓ arrow keys, press Enter to apply the selected theme, or q/Esc to quit without applying.

### Direct Theme Application

```bash
kcn --theme <theme-name> [globals.css]
```

Apply a specific theme directly without interactive mode.

### List Available Themes

```bash
kcn --list
```

Show all available built-in themes.

### Search for CSS File

```bash
kcn --search
```

Look for CSS file in common directories (src/, app/, styles/, etc.).

## Available Themes

- default
- blue
- green
- purple
- rose
- deep_space
- autumn_leaves
- fresh_mint
- ocean_breeze
- sunset_glow
- lavender_dream
- golden_sand
- arctic_ice

## License

MIT License - See LICENSE file for details.