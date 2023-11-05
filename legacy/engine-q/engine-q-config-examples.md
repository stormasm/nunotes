
[kobouch example config](https://github.com/nushell/nu_scripts/tree/main/engine-q/example-config)

### next release of config

```rust
let $config = {
  filesize_metric: $true
  table_mode: rounded # basic, compact, compact_double, light, thin, with_love, rounded, reinforced, heavy, none, other
  use_ls_colors: $true
  color_config: {
    separator: yd
    leading_trailing_space_bg: white
    header: cb
    date: pu
    filesize: ub
    row_index: yb

    bool: red
    int: green
    duration: red
    range: red
    float: red
    #string: red
    nothing: red
    binary: red
    cellpath: red
  }
  use_grid_icons: $true
  footer_mode: always #always, never, number_of_rows, auto
  animate_prompt: $false
  float_precision: 2
  without_color: $false
  filesize_format: "b" # b, kb, kib, mb, mib, gb, gib, tb, tib, pb, pib, eb, eib, zb, zib, auto
}
```

* [discord](https://discord.com/channels/601130461678272522/683070703716925568/918624930793422858)
* [issue 459](https://github.com/nushell/engine-q/issues/459)

### fdncred v1

```rust
let-env HOME = "/Users/fdncred"
let-env LANG = "en_US.UTF-8"
let-env LESS = "-FRX"
let-env PAGER = "less"

# let-env PATH = [
#     "/usr/local/sbin",
#     "/Users/fdncred/.cargo/bin",
#     "/usr/local/bin",
#     "/usr/bin",
#     "/bin",
#     "/usr/sbin",
#     "/sbin",
#     "/usr/local/share/dotnet",
#     "/opt/X11/bin",
#     "~/.dotnet/tools",
#     "/Library/Frameworks/Mono.framework/Versions/Current/Commands",
#     "/usr/local/opt/coreutils/libexec/gnubin",
#     "/Users/fdncred/.local/bin"
# ]

# let name = "darren"

let $config = {
  filesize_metric: $true
  table_mode: rounded
  use_ls_colors: $true
  color_config: {
    separator: yd
    leading_trailing_space_bg: white
    header: cb
    date: pu
    filesize: ub
    row_index: yb

    bool: red
    int: green
    duration: red
    range: red
    float: red
    #string: red
    nothing: red
    binary: red
    cellpath: red
  }
  use_grid_icons: $true
  footer_mode: always # always, never, auto, "5"
  animate_prompt: $false
}

# note - the string color will affect ls_colors
# if you make it wd, all colors for strings will
# be dimmed


# export LS_COLORS="$(vivid generate molokai)"
let-env LS_COLORS = (vivid generate molokai)
# other vivid themes
# ayu, iceberg-dark, jellybeans, lava, molokai
# nord, one-dark, one-light, snazzy, solarized-dark
# solarized-light
```

### fdncred with a different color_config

```rust
color_config: {
    separator: purple
    leading_trailing_space_bg: white
    header: gb
    date: wd
    filesize: c
    row_index: cb

    bool: red
    int: green
    duration: blue_bold
    range: purple
    float: red
    string: white
    nothing: red
    binary: red
    cellpath: cyan
  }
```
