
## Notes about the engine

#### repaint

```rust
fn read_line_helper(&mut self, prompt: &dyn Prompt) -> Result<Signal> {
     self.painter.initialize_prompt_position()?;
     self.hide_hints = false;
     self.repaint(prompt)?;
```

The first repaint in the engine code doesn't really do anything except draw
the prompt initially.  The code does not really have to be there.  If its
not there you just don't see the prompt when you hit ENTER.  However, you
will see the prompt once you type your first key.
