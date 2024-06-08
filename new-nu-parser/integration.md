
11/22/2023

I asked Sophia today how the Compiler would integrate in and she said that
the Compiler would act similar to the current Delta.  

So Delta would merge into the EngineState see

#### nu-protocol/src/engine/engine_state.rs

```rust
engine_state.merge_delta(delta)
pub fn merge_delta(&mut self, mut delta: StateDelta) -> Result<(), ShellError>
```
