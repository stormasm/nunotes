
- LineBuffer is the core text model,
- Editor is the stateful editing controller, and
- Reedline is the top-level interactive engine.

### LineBuffer

- the LineBuffer owns the actual text plus cursor/selection state:

Its responsibility is:

- store the current input text
- track cursor position
- track selection anchor
- implement primitive text operations:
  - insert/delete
  - move by grapheme/word/line
  - clear/replace ranges
  - find surrounding pairs
  - operate safely on Unicode grapheme boundaries

So LineBuffer is basically the document + caret.

### selection anchor is an Optional field in the LineBuffer

In a code editor, a selection anchor is a fixed starting point for highlighting text. When you make a selection, the anchor remains at the exact spot you began, while the active cursor (often called the focus) moves as you extend the highlight
