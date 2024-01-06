

Inside the *core_editor* module there are two key files...

* editor.rs
* line_buffer.rs

If you look inside *editor.rs* and search for *line_buffer* you will see
that the editor code is continually referencing and manipulating the underlying
line_buffer...

As noted in the code comments of *editor.rs*

```rust
/// Stateful editor executing changes to the underlying [`LineBuffer`]
///
/// In comparison to the state-less [`LineBuffer`] the [`Editor`] keeps track of
/// the undo/redo history and has facilities for cut/copy/yank/paste
```

### Implementation Code Details

Inside the engine the open_editor code spawns off the command that was set
with the *with_buffer_editor* code inside the reedline/examples/demo.rs or
inside the nushell cli.  In this case it was emacs.  But it can
be whatever editor you have on your system including {vi, helix, etc...}

```rust
// Adding emacs as text editor
let temp_file = temp_dir().join("temp_file.nu");
let mut command = Command::new("emacs");
command.arg(&temp_file);
line_editor = line_editor.with_buffer_editor(command, temp_file);
```

Then the *open_editor* function spawned process waits until the editor was
closed at which time it does a *set_buffer* on the returned string.

```rust
fn open_editor(&mut self) -> Result<()> {
    match &mut self.buffer_editor {
        Some(BufferEditor {
            ref mut command,
            temp_file,
        }) => {
            {
                let mut file = File::create(&temp_file)?;
                write!(file, "{}", self.editor.get_buffer())?;
            }
            {
                let mut child = command.spawn()?;
                child.wait()?;
            }

            let res = std::fs::read_to_string(temp_file)?;
            let res = res.trim_end().to_string();

            self.editor.set_buffer(res, UndoBehavior::CreateUndoPoint);

            Ok(())
        }
        _ => Ok(()),
    }
}
```
