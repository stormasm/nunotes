
There have been several page one nushell posts on hacker news over the years.  
Here are the highlights from some of them...

The most recent post currently has 308 points, and was at the number one slot for a short time and the number three slot more than once as it attained that spot and then [climbed back up there](https://discord.com/channels/601130461678272522/683070703716925568/1037063155131617363)...

## [November 1, 2022: Introduction to a new kind of shell](https://news.ycombinator.com/item?id=33419944)

##### [LLVM for shell scripts](https://news.ycombinator.com/item?id=33433840)

Nushell looks pretty good!
But, how to automatically convert all existing shell scripts -- to Nushell?

I think it would be highly interesting if someone created "an LLVM for shell scripts"...

In other words -- given a shell script that runs on one shell, turn that into a parse tree -- then transpile that parse tree back into a script such that the transpiled script runs on a different shell -- with guaranteed full compatibility...

Maybe some work in this area is being done, or already has been done...

I'd be interested if anyone knew of anything like this...

If so -- then links would be appreciated!

##### [additional thoughts](https://news.ycombinator.com/item?id=33427070)

Traditional shell scripting languages are great at exactly three things:

* typing commands interactively,
* running other programs,
* and sourcing other shell scripts.

They are also is distinct in that they are "strongly-typed" (i.e. everything is a string), and moreover that syntactically bare symbols are also strings.

##### [does not support suspended jobs](https://news.ycombinator.com/item?id=33425689)

##### [macos issue: Library/Application Support](https://news.ycombinator.com/item?id=33421658)

One issue I have with nushell is that on macOS they use the `Library/Application Support` path for the config instead of the `.config` making it awkward, their reason is that the former is the correct path of application data, while none of the other CLI apps I use do this, every other app just points to the `$HOME` dir or `.config` dir.

##### [external programs are 2nd class citizens](https://news.ycombinator.com/item?id=33425990)

Please correct me if I'm wrong, but doesn't nushell suffer from the same problem as powershell that all the nice fancy stuff works only for in-process commands and external programs are bit of a second-class citizens?

##### [And finally... More than one post says our name should have been nutshell](https://news.ycombinator.com/item?id=33421581)

> I actually kind of like nutshell...  At least gmail thinks it should be nutshell too as that is what it auto corrects to :)

## [Previous Discussions about Nushell](https://news.ycombinator.com/item?id=33420520)
