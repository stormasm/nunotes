
### Core of Nushell

* For me its all about the community of people working together on nushell
* The actual underlying product is number two on the list
* I like the ability to program nushell using 3 different core methodologies
    * internal commands in rust
    * custom commands in nu
    * plugins
* I like the abstraction layer of being able to build nu from the ground up
using a set of crates starting with **nu-protocol** at the base of the stack.
This will become even more interesting as we attempt to break out the language from the
piped execution platform / engine.
* Having a set of crates that will enable me to mix and match what commands I want in the platform is a very powerful concept. Today we only have nu-command;
but into the future I look forward to being able to build custom nushell engines
by pulling in the things I need and want for the particular application I am working on at the time.  This will give me the ability to control the binary size for smaller memory footprint embedded systems or larger scale enterprise level applications.