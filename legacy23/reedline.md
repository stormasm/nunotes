
### Code Organization

##### completer

The DefaultCompleter is only called from

* engine
* main
* lib


### The Painter Notes

Being part of Reedline makes sense to ensure that Reedline can manage event dispatch (for all menus were buffer editing could take place as well) and to have control over the painting/space requirements.
State fulness is some part of Reedline as we previously handed back to the hosting application on ctrl L while maintaining buffer state
In general I find the menus to really expand the state machine when they are active to an extent that is hard to follow (they capture events, update their state on buffer changes, moves through pages by the user, update the buffers them self) on top of that they contain most of their painting (to manage their size awareness}

[discord](https://discord.com/channels/601130461678272522/855886335980994600/971978131487473775)

in reviewing the painter / prior to menus all painting went through the engine / but it looks like menus break out of that / why don't the menus use the engine for their painting ?  Is it because the menus take over / and leave the engine behind ?  Or in other words / the menus don't need the engine / all the menus need is the painter ?

the menus and the engine are orthogonal to each other :). is their any relationship between the engine and the menus / if so what is it... thanks / I am new to understanding this...

Or to state it another way...

The only user of the Painter is the
1) the engine
2) the menus

So what are the jobs of the engine ?
1) to handle event dispatch
2) to kick off all of the main functionalities including Painting
starting to make sense that the menus are the most complex part of painting whereas all of the line editing tasks are much more mundane


### How to clear the terminal buffer on the Mac default terminal

I was looking for a method of clearing the buffer of the vertical scroll back slider and came across this little Terminal escape snippet I had never seen before:   

Code:   
```rust
printf "%b" "\033[3J"   
```

SO...   
Code:   
```rust
printf "%b" "\033c\033[3J\033[2J\033[0m\033[H"
```

Performs a terminal reset, buffer clearance, clear the window, set back to default colours etc... and places prompt at the top.
Neat eh!
Even the builtin Terminal  reset  command doesn't do that.
(This also works under 'xterm' too.)

[ref](https://www.unix.com/os-x-apple-/279401-means-clearing-scroll-buffer-osx-terminal.html)
