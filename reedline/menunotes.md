
* repaint
* buffer_paint
* repaint_buffer

### repaint_buffer is how the prompt and buffer get painted

* print_small_buffer
* print_large_buffer

```rust
fn print_small_buffer(
     &mut self,
     prompt: &dyn Prompt,
     lines: &PromptLines,
     menu: Option<&ReedlineMenu>,
     use_ansi_coloring: bool,
 ) -> Result<()> {

fn print_large_buffer(
     &mut self,
     prompt: &dyn Prompt,
     lines: &PromptLines,
     menu: Option<&ReedlineMenu>,
     use_ansi_coloring: bool,
 ) -> Result<()> {
```
