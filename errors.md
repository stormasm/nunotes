
### Errors in Nushell, what are they ?

something i can't figure out... are Nushell errors more like exceptions that are raised and caught with try or values that can be manipulated?

sometimes a peace of code will panic and sometimes the errors will be silently captured in a variable for example and as soon as you use that variable, the code throws the errors
I'm just confused a bit

##### Devyn response

they are values that can't be manipulated 😉

no but really we have a lot of trouble in the internals trying to preserve them as values, because then errors don't get propagated and people get weird type mismatch errors rather than the actual error that caused it

so we often try to bubble an error up from a value to an error if it was present somewhere we were going to look at the value - e.g. not within a list/record, but then if we end up looking at list items or record values then we probably will try to propagate the error.

I guess they're kind of like errors (not Either) in Haskell, where because of lazy evaluation you might just not even get an error until you end up evaluating that thunk

if you use catch then it captures the error value for you and gives you a record that contains the original error value under .raw, which you can kind of use to re-throw the error
tbh I do think it is kind of a confusing compromise between functional-style (e.g. Result, Either) errors and exceptions
I think I'd rather that errors very consistently bubble up, just because the original error is usually the most helpful

- [discord](https://discord.com/channels/601130461678272522/683070703716925568/1251840644192931902)
