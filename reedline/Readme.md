
### Validator

So I reviewed the Validator in Reedline and its not an important feature I
think I would ever use at this time...

If you do a *rg* in the nushell code it is not really being used except possibly
to report some error messages.

I will explore this idea later on but for now we are good with it.

### The whole issue with clearing the screen

Bottom line is do a Ctrl-L in nushell

* [event ClearScrollback is working in reedline](https://github.com/nushell/nushell/pull/5405)
* [move the clear screen/Ctrl-L logic inside the line editor engine](https://github.com/nushell/reedline/issues/120)
* [it works in the reedline example demo.rs](https://github.com/nushell/reedline/blame/main/examples/demo.rs#L165)
* [painter.rs clear_scrollback](https://github.com/nushell/reedline/commit/2e2bdc54621643e7bee5ba2e2d9bf28e757074e0#diff-efa89641da2318bb971073816774b04618929e3c9d42fe3d99919554fe33ccfd)
