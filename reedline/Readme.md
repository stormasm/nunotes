

### The whole issue with clearing the screen

Bottom line is do a Ctrl-L in nushell

* [event ClearScrollback is working in reedline](https://github.com/nushell/nushell/pull/5405)\
* [Move the clear screen/Ctrl-L logic inside the line editor engine](https://github.com/nushell/reedline/issues/120)
* [It works in the reedline example demo.rs](https://github.com/nushell/reedline/blame/main/examples/demo.rs#L165)
* [painter.rs clear_scrollback](https://github.com/nushell/reedline/commit/2e2bdc54621643e7bee5ba2e2d9bf28e757074e0#diff-efa89641da2318bb971073816774b04618929e3c9d42fe3d99919554fe33ccfd)
