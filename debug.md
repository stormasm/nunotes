
### How does debugging work ?

- [DebugContext](https://github.com/nushell/nushell/blob/main/crates/nu-protocol/src/debugger/debugger_trait.rs)
- [discord](https://discord.com/channels/601130461678272522/683070703716925568/1253732915209048235)

The *debug profile* command uses it.   
It implements the Debugger trait whose methods are called by the evaluator.
