
### How to clear the terminal buffer on the Mac default terminal

I was looking for a method of clearing the buffer of the vertical scroll back slider and came across this little Terminal escape snippet I had never seen before:
Code:
printf "%b" "\033[3J"

SO...
Code:
printf "%b" "\033c\033[3J\033[2J\033[0m\033[H"

Performs a terminal reset, buffer clearance, clear the window, set back to default colours etc... and places prompt at the top.
Neat eh!
Even the builtin Terminal  reset  command doesn't do that.
(This also works under 'xterm' too.)

[ref](https://www.unix.com/os-x-apple-/279401-means-clearing-scroll-buffer-osx-terminal.html)
