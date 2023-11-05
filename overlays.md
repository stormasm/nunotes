
### OverlayFrame is defined in nu-protocol/src/engine/overlay.rs

```rust
crates/nu-protocol/src/engine/state_delta.rs
78:    pub fn last_overlay_mut(&mut self) -> Option<&mut OverlayFrame> {
97:    pub fn last_overlay(&self) -> Option<&OverlayFrame> {

crates/nu-protocol/src/engine/state_working_set.rs
754:    pub fn find_overlay(&self, name: &[u8]) -> Option<&OverlayFrame> {
783:    pub fn last_overlay(&self) -> &OverlayFrame {
799:    pub fn last_overlay_mut(&mut self) -> &mut OverlayFrame {

crates/nu-protocol/src/engine/engine_state.rs
365:    pub fn last_overlay(&self, removed_overlays: &[Vec<u8>]) -> &OverlayFrame {
381:    pub fn get_overlay(&self, overlay_id: OverlayId) -> &OverlayFrame {
```

* [Why does Python use virtual environments, but other languages do not?](https://www.quora.com/Why-does-Python-use-virtual-environments-but-other-languages-do-not)

Because many other languages don’t use a global package control system:

When installing a module via pip/conda you install these packages globally, making them available for every future project.

Languages like Java don’t do that. Instead you manually add all libraries you need to your project. Because different projects can be bound to different versions of the same library you don’t need virtual environments: Every project is his own environment.

There are of course other languages that have a global package control, but for these there are venv-equivalencies:

Node.js has the “bundle” command that reads dependencies from a package.json:

{ "name": "APP_NAME", "version": "0.0.1", "dependencies": {"MODULE_NAME": "MODULE_VERSION"}}
There are also Node version mangers like Nave or NVM.

Haskell’s Cabal package manager has sandboxes and The Haskell Tool Stack has “Snapshots”.

Either they exist (if packages are installed globally), or the packages are bound to each project individually, eliminating the need for Environments.
