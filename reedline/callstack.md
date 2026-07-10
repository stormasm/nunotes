
### Call stack take it from the top

Nushell calls into Reedline and the basic demo calls into Reedline
in the same way which is outlined below.

In reality this is the way everyone fires up and uses Reedline

- read_line
- read_line_helper  [lines 830 - 968]
- process_input_batch
- [repaint](./code.md)
- update_working_details
