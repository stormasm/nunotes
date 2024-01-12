
### Validator and InsertNewline

The reason unbalanced brackets and braces causes a *InsertNewline*.

```rust
ReedlineEvent::Enter => {
     let buffer = self.editor.get_buffer().to_string();
     match self.validator.as_mut().map(|v| v.validate(&buffer)) {
         None | Some(ValidationResult::Complete) => Ok(self.submit_buffer(prompt)?),
         Some(ValidationResult::Incomplete) => {
             self.run_edit_commands(&[EditCommand::InsertNewline]);
             Ok(EventStatus::Handled)
         }
     }
 }
```

* [DefaultValidator checks for mismatched quotes and brackets](https://github.com/nushell/reedline/blob/main/src/validator/default.rs)

and defines the validate method above

Also in the Reedline example demo there is the Keybinding

```rust
fn add_newline_keybinding(keybindings: &mut Keybindings) {
    // This does work for macOS where the ALT key is the Option key
    keybindings.add_binding(
        KeyModifiers::ALT,
        KeyCode::Enter,
        ReedlineEvent::Edit(vec![EditCommand::InsertNewline]),
    );
}
```

#### More details from when I first started learning Reedline

In the very beginning of my *Reedline journey* I did not understand the above concept
and so I wrote up this piece below.  Since that time clearly I have learned
more about the Validator's functionality as witnessed by the notes above...

So I reviewed the Validator in Reedline and its not an important feature I
think I would ever use at this time...

If you do a *rg* in the nushell code it is not really being used except possibly
to report some error messages.

I will explore this idea later on but for now we are good with it.
