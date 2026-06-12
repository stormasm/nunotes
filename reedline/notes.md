
- LineBuffer is the core text model,
- Editor is the stateful editing controller, and
- Reedline is the top-level interactive engine.

### selection anchor is an Optional field in the LineBuffer

In a code editor, a selection anchor is a fixed starting point for highlighting text. When you make a selection, the anchor remains at the exact spot you began, while the active cursor (often called the focus) moves as you extend the highlight
