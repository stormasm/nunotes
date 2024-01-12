
### Validator and InsertNewline

The reason unbalanced brackets and braces causes a *InsertNewline*.

The following code is in *engine.rs*

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

### How it works in Nushell

nu-parser/src/lex.rs

```rust
// If there is still unclosed opening delimiters, remember they were missing
 if let Some(block) = block_level.last() {
     let delim = block.closing();
     let cause =
         ParseError::UnexpectedEof((delim as char).to_string(), Span::new(span.end, span.end));

     return (
         Token {
             contents: TokenContents::Item,
             span,
         },
         Some(cause),
     );
 }
```

An *UnexpectedEof* gets returned which matches up with the
[NuValidator](https://github.com/nushell/nushell/blob/main/crates/nu-cli/src/validation.rs)

```rust
impl Validator for NuValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        let mut working_set = StateWorkingSet::new(&self.engine_state);
        parse(&mut working_set, None, line.as_bytes(), false);

        if matches!(
            working_set.parse_errors.first(),
            Some(ParseError::UnexpectedEof(..))
        ) {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}
```

which returns a *ValidationResult::Incomplete* which triggers the
*ReedlineEvent::Enter* above.
