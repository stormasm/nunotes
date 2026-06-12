
- LineBuffer is the core text model,
- Editor is the stateful editing controller, and
- Reedline is the top-level interactive engine.

### Editor

- Editor contains a LineBuffer and adds higher-level editing concerns:

So compared to LineBuffer, Editor adds:

- undo/redo history
- clipboard / cut buffer
- edit-mode-aware behavior
- selection semantics
- execution of higher-level edit commands

This is the main architectural boundary:

- LineBuffer = what the text is and how it can be mutated
- Editor = how user editing actions are applied to that text safely and with history

---

Editor delegates actual text changes to LineBuffer

This shows the pattern:

- Editor manages selection/undo policy
- then calls into LineBuffer
- then records the appropriate undo behavior
- So Editor is effectively the policy layer around the LineBuffer.

---

### Editor exposes the current LineBuffer

Editor also provides direct accessors:

That means the rest of reedline can inspect the current text/cursor state through Editor, without owning the raw buffer itself.

---

### Reedline: the outer engine that runs the interactive session

At the top level, Reedline is the full line editor engine used by applications:

And more broadly it owns things like completers, highlighters, menus, hints, and terminal interaction:

So Reedline is the orchestration layer that combines:

- terminal I/O
- prompt rendering
- events/keypress handling
- completion
- hinting
- highlighting
- menus
- external editor integration
- and the actual editable state via Editor

### How LineBuffer reaches the screen

Inside the engine loop, Reedline passes the current LineBuffer to the painter/rendering side:

That’s an important architectural clue:

LineBuffer is the canonical source of current text/cursor state

- Editor owns it
- Reedline asks Editor for it
- the painter uses it to render the current line(s)
- So rendering depends on LineBuffer, but does not mutate it directly.

### Practical mental model

A good mental stack is:

- LineBuffer = text storage + cursor/selection math
- Editor = command execution + undo/redo + clipboard + mode-aware edits
- Reedline = interactive shell/editor engine around Editor

In flow form:

keypress/event → Reedline → Editor → LineBuffer → painter/rendering

### Example of the layering

Suppose the user presses a key that moves up a line.

- Reedline handles the input event
- it turns that into an editing action
- Editor applies the action, preserving selection/undo semantics
- LineBuffer computes the new insertion point using its multiline/grapheme-aware logic
- Reedline repaints using the updated buffer

---

Why this split is useful

### This architecture keeps responsibilities separated:

- LineBuffer can focus on correct text manipulation, especially Unicode and cursor math.
- Editor can focus on editing semantics like undo, selection, cut/yank, and mode behavior.
- Reedline can focus on user interaction and terminal integration.

That makes the codebase easier to evolve:

- you can improve cursor/word logic in LineBuffer
- change undo behavior in Editor
- add UI features in Reedline
- without collapsing everything into one giant type.

---

### Short version

- LineBuffer is the text engine,
- Editor is the editing state machine, and
- Reedline is the interactive shell-facing frontend.

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
