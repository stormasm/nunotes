
* [repaint](./codenotes.md#notes-about-the-engines-repaint)
* buffer_paint
* repaint_buffer

##### repaint_buffer is how the prompt and buffer get painted

and where the following two methods get called

```rust
if self.large_buffer {
       self.print_large_buffer(prompt, lines, menu, use_ansi_coloring)?;
   } else {
       self.print_small_buffer(prompt, lines, menu, use_ansi_coloring)?;
   }
```

* print_large_buffer
* print_small_buffer

```rust
fn print_large_buffer(
     &mut self,
     prompt: &dyn Prompt,
     lines: &PromptLines,
     menu: Option<&ReedlineMenu>,
     use_ansi_coloring: bool,
 ) -> Result<()> {

fn print_small_buffer(
     &mut self,
     prompt: &dyn Prompt,
     lines: &PromptLines,
     menu: Option<&ReedlineMenu>,
     use_ansi_coloring: bool,
 ) -> Result<()> {

```
