# Theme Creation Plan

## New Themes to Create

1. **Deep Space** - Indigo/purple tones
2. **Autumn Leaves** - Amber/orange tones
3. **Fresh Mint** - Light green tones

## Implementation Steps

1. Switch to Code mode to create the new theme files
2. Create each theme file with appropriate OKLCH color values
3. Update mod.rs to include the new themes
4. Update load_builtin_themes function in main.rs to include new themes
5. Update tests to include new themes

## Color Palette Details

### Deep Space (Indigo/Purple)
- Primary: Deep indigo/purple tones
- Background: Dark space-like colors
- Accents: Star-like highlights

### Autumn Leaves (Amber/Orange)
- Primary: Warm amber/orange tones
- Background: Earthy fall colors
- Accents: Golden highlights

### Fresh Mint (Light Green)
- Primary: Light mint green tones
- Background: Clean, fresh colors
- Accents: Cool highlights

## File Structure
- src/themes/deep_space.rs
- src/themes/autumn_leaves.rs
- src/themes/fresh_mint.rs
- Update src/themes/mod.rs
- Update src/main.rs (load_builtin_themes function and tests)