
## Darren config

### ide_completion_menu

```rust
{
  name: ide_completion_menu
  only_buffer_difference: false
  marker: $" \n❯ (char -u '1f4ce') "
  type: {
    layout: ide
    min_completion_width: 0,
    max_completion_width: 50,
    # max_completion_height: 10, # will be limited by the available lines in the terminal
    padding: 0,
    border: true,
    cursor_offset: 0,
    description_mode: "prefer_right"
    min_description_width: 0
    max_description_width: 50
    max_description_height: 10
    description_offset: 1
    # If true, the cursor pos will be corrected, so the suggestions match up with the typed text
    #
    # C:\> str
    #      str join
    #      str trim
    #      str split
    correct_cursor_pos: true
  }
  style: {
    # text: { fg: "#33ff00" }
    text: green
    # selected_text: { fg: "#0c0c0c" bg: "#33ff00" attr: b}
    selected_text: { attr: r }
    description_text: yellow
    # match_text: { attr: u }
    match_text: { fg: "#33ff00" }
    # selected_match_text: { attr: ur }
    selected_match_text: { fg: "#33ff00" attr: r }
  }
}
```

### completion_menu

```rust
{
  name: completion_menu
  only_buffer_difference: false # Search is done on the text written after activating the menu
  marker: $" \n❯ (char -u '1f4ce') "
  # marker: "| "
  type: {
    layout: columnar
    columns: 4
    col_width: 20   # Optional value. If missing all the screen width is used to calculate column width
    col_padding: 2
  }
  style: {
    # text: { fg: "#33ff00" }
    text: green
    # selected_text: { fg: "#0c0c0c" bg: "#33ff00" attr: b}
    selected_text: {attr: r}
    description_text: yellow
    # match_text: { attr: u }
    match_text: { fg: "#33ff00" }
    # selected_match_text: { attr: ur }
    selected_match_text: { fg: "#33ff00" attr: r }
  }
}
```

[discord ref](https://discord.com/channels/601130461678272522/1202725430701072405/1202725933627220108)
