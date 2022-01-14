
I am working on the command sort-by and slowly figuring out what its
going to take to get this going...

One thing I came across in the nushell sort-by was this function

[coerce_compare](https://github.com/nushell/nushell/blob/main/crates/nu-data/src/base.rs#L113)

As you can see it returns a
[CompareValues](https://github.com/nushell/nushell/blob/main/crates/nu-data/src/base.rs#L73)

In nushell the only place the function **coerce_compare** is used is inside sort-by.

I am thinking I am going to need this for my **sort-by** implementation ?

Do we already have similar functionality in engine-q, or do you believe I should move this code over to engine-q ?
