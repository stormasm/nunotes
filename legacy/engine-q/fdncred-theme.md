```rust
let base00 = "#181818" # Default Background
let base01 = "#282828" # Lighter Background (Used for status bars, line number and folding marks)
let base02 = "#383838" # Selection Background
let base03 = "#585858" # Comments, Inv

let base16 = {
  separator: $base03
  leading_trailing_space_bg: $base04
  header: $base0b
  date: $base0e
  filesize: $base0d

let config = {
  filesize_metric: $true
  table_mode: rounded # basic, compact, compact_double, light, thin, with_love, rounded, reinforced, heavy, none, other
  use_ls_colors: $true
  color_config: $base16
  use_grid_icons: $true
  footer_mode: always #always, never, number_of_rows, auto
  animate_prompt: $false
  float_precision: 2
  without_color: $false
  filesize_format: "b" # b, kb, kib, mb, mib, gb, gib, tb, tib, pb, pib, eb, eib, zb, zib, auto
}
```

```rust

let-env LS_COLORS
#rrggb
red

{ fg: #rrggbb bg: #rrggbb attr: bli }

"#rrggbb"

let base03 = red
let config = { }

let config = ($config | update foo bar)

$config.color_config.separator = blue
```
