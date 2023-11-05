This is a cut and paste of
[this article](https://github.com/skelly37/Reject-POSUCKS-embrace-Nushell)
for archival purposes...

* [version history](https://github.com/skelly37/Reject-POSUCKS-embrace-Nushell/commit/540a6f9d948a2e65dd531a2e2fc4168386b18535)

# Reject POSUCKS, embrace nushell
Why do we still stick to archaic tech just to be compatible with some niche OSes? Windows compatibility, written in Rust, bytestream replaced by tables are here to help us make the technological progress! 

*Don't be scared by the edgy title, the article is more substantive, I promise.*

## First words
Remember coming to conclusion *I should've done it in C or Python* after debugging your {insert your favorite POSIX shell's name} code for a few hours? Remember struggling to parse some input or to store your data efficiently? Maybe you needed some quick maths without messing with bc or Python shell? If you've anwsered "yes" at least once, you surely should read it! (100% negatives are also welcome, of course)

## TL;DR
Nushell: 
- is compatible with Windows, MacOS and Linux
- prioritizes development of the data structures and their parsers
- automates the boring stuff like `--help`
- provides very friendly UI and UX for the interactive mode
which makes it the best shell solution to develop and promote. 

**It's a shell ready to face the 2020s reality.**

## Table of contents
- [Windows compatibility vs. BSD, Solaris and exotic Unix compatibility](#windows-compatibility-vs-bsd-solaris-and-exotic-unix-compatibility)
- [Data structures](#data-structures)
- [Aesthetic output](#aesthetic-output)
- [Auto-generated help for functions](#auto-generated-help-for-functions)
- [Built-in parsers](#built-in-parsers) 
- [Built-in calculator and other useful functions](#built-in-calculator-and-other-useful-functions)
- [Community repository of user scripts](#community-repository-of-user-scripts)
- [Self-criticism: what I lack in nushell, what annoys me (in comparison to my bash experience)](#self-criticism-what-i-lack-in-nushell-what-annoys-me-in-comparison-to-my-bash-experience)
- [Bonus: Nushell vs current Windows state](#bonus-nushell-vs-current-windows-state)
- [Bonus 2: Bash vs Nushell script](#bonus-2-bash-vs-nushell-script)
- [Final thoughts](#final-thoughts)
- [Useful links](#useful-links)



## Windows compatibility vs. BSD, Solaris and exotic Unix compatibility
Works on POSIX started in 1985, as an attempt to unify Unix experience. The latest significant change was made in 2008 (POSIX.1-2008), the development is rather slow. Quick look at the simple Unix history (from [Wikipedia](https://pl.wikipedia.org/wiki/Unix)) is self-explanatory, if one wonders why the standard was that needed.  

![UNIX history graph](https://upload.wikimedia.org/wikipedia/commons/thumb/7/77/Unix_history-simple.svg/1280px-Unix_history-simple.svg.png)

Nowadays it's way different — so called *YEAR OF LINUX!!* is nothing more but a joke and wishful-thinking, Windows dominated the market [[source]](https://www.t4.ai/industry/desktop-operating-system-market-share): 

![OS market share](https://global-uploads.webflow.com/5d71c5b5ed21579fe7c3535a/5f3931dea570651394e36a57_desktop-operating-system-market-share-p-800.png)

Windows support is more important than previously mentioned Unix systems other than MacOS and Linux. Saving people from cmd hell is the best we can do for them. Crossplatform solution in 2021 is not cross-Unix, it's Windows/Mac/Linux. There are still lots of people who simply cannot leave Windows because of their work and they'd rather use PowerShell than switch to Linux because **opensource**. *Actually I'm a huge FOSS enthusiast but orthodoxy goes off if you want to earn for a living.* Ignoring them is like digging a grave for your still-alive tech.

Nushell makes you also independent of stuff like `curl`, `sed`, `bc`, etc. (`fetch`, `str find-replace` and nushell itself, respectively). So even though you'd have to rewrite the legacy code into nushell, it could pay off with its portability (just consider it, it might not be a right thing to actually do in your case). POSIX standard is a nice idea, but NT+UNIX shell is way better and more beneficial to the community in the long run.

Don't get me wrong. I'm not saying that we shouldn't support e.g. BSD. Actually, I'd love to play with OpenBSD + nushell. I'm just trying to say we need to leave the Unix bubble and face the NT reality, if we want our shell to become a standard (or at least a decent competitor/alternative).


# Data structures
There are integers, strings and booleans but I'll skip them from obvious reason — you can use them in POSIX as well. Working with data is actually a nushell's priority and the reason for its very existence. I won't cover everything, I'll cust cut to the chase with the most interesting ones. Let me focus on the improvements then:

1) decimal (float) — Yes, you can perform floating-point arithmetics within your shell! No need for bc, python or other tools. 
2) Date (date and time) — As you can guess, it may save you lots of useless effort. Docs describe it pretty well: Dates and times are held together in the Date value type. Date values used by the system are timezone-aware, and by default use the UTC timezone.
3) Duration — Not only you can store Data conveniently but also periods of time. Want to add 1hr to your Date variable? Fine, go on!
4) Range — `1..3`, `..3`, `3..`, `1..<3`. Open ranges, inclusive, non-inclusive, everything. `1..3` is basically `1, 2, 3`.
5) File size – Specific integer values for file sizes, as the name says. Useful in some scripts but also in trivial tasks, like sorting the `ls`. 
6) **Structures** — the soul and last but not least of the nushell's pros. 
    - Table — list of rows, intuitive for users familiar with **SQL** or **LINQ** — `get`, `select`, `sort-by` in one's fingertips. I'm sure you're aware how it's helpful during, e.g. parsing output from some API call. You need to store the received, let's say, json (parsed by `from json` btw) in some kind of structure. I can't think of better shell to do so.
    - List — acts like Pythonic list, i.e. you can easily mix types in there. You can also do things like that: `"blue" in $colors`. 
    - Dataframe — the most advanced and powerful one. Comparable to the Python Pandas ([actually benchmarked](https://www.nushell.sh/book/dataframes.html#benchmark-comparisons)). They're quite a long topic, so for the deeper technicals check out the link. But the benchmark proves it's a really decent solution (actually halved the execution time, when compared to Pandas), so it's good to at least aknowledge their existence. Gets, selects, aggregations are here to help you — and they're still being improved! 

Moreover, the speficic types like Date, file size etc. support arithmetics and comparison. Let's say, you want to `ls` but only the files modified 1 week ago or earlier, that are bigger than 1MB. Here ya go: `ls|where modified > (date now) - 1wk && size > 1mb`. Yes, that's it!

To sum up — Nushell provides fantastic date and time support, is a decent calculator and allows you to organize your data conveniently in tables.


## Aesthetic output
Since data is pretty well organized into tables, it can also be displayed in a decent way. Let's take for example default `ls` function: 

![ls output](https://github.com/skelly37/literally-everything/raw/main/screenshots/nushell/ls.png)

Or maybe `sys` (data about your setup, as the name suggests) output?

![sys output](https://github.com/skelly37/literally-everything/raw/main/screenshots/nushell/sys.png)

Visual preferences are fully subjective but I think most of us agree on that it's better than the `ls` known from bash or sh. The output is more clear and readable by default. One could argue about its true usefulness but the more time you spend in terminal, the more you appreciate TUI aesthetics. 

Also, nu became even more aesthetic after the engine change.


## Easily customisable left and right prompts, also the indicators
When it comes to the aesthetics, there's also something for ricers: Easy and intuitive prompt customisations, well-documented in the default `config.nu` file. I personally removed the right prompt and added scanning for the superuser in the left one:

```nushell
def create_left_prompt [] {
    let path_segment = (if ($env.USER != "root") {($env.PWD)} else {echo $"[ROOT] ($env.PWD)"})

    $path_segment
}

def create_right_prompt [] {}

# Use nushell functions to define your right and left prompt
let-env PROMPT_COMMAND = { create_left_prompt }
let-env PROMPT_COMMAND_RIGHT = { create_right_prompt }


# The prompt indicators are environmental variables that represent
# the state of the prompt
let-env PROMPT_INDICATOR = { "〉" }
let-env PROMPT_INDICATOR_VI_INSERT = { ": " }
let-env PROMPT_INDICATOR_VI_NORMAL = { "〉" }
let-env PROMPT_MULTILINE_INDICATOR = { "::: " }
```

## Auto-generated help for functions
Honestly, I hadn't known about the feature until my fellow told me about it today. First I'll say a bit about theory, then practical example. 

When you define a function, nu automatically adds flags for help and generates the help page. Usage, parameters and flags. And, as I'll explain below, editing the help menu is intuitive and easy. Decent help pages for shell scripts should be a standard, so it's nice that nushell takes care of it. 

How does it work? Let's assume we have a `foo` function with code like this:
```
def foo [ a, b ] {
  echo a + b
}
```
When you'll type `help foo`, `foo -h` or `foo --help`, you'll receive the following output:
```
Usage:
  > foo <$a> <$b> {flags}

Parameters:
  <$a>
  <$b>

Flags:
  -h, --help: Display this help message
```

You can also add comments with # (like in Python) line above the def to print some description above the Usage paragraph. Or you can also comment next to the arguments to display info about them, e.g.:
```
#Foo function that adds two given numbers 
def foo [ a, #first number
b #second number 
] {
  echo a + b
}
```
Output:
```
Foo function that adds two given numbers 

Usage:
  > foo <$a> <$b> {flags}

Parameters:
  <$a> first number
  <$b> second number

Flags:
  -h, --help: Display this help message
```

## Nu completions & custom ones
![completion 1](https://github.com/skelly37/literally-everything/raw/main/screenshots/nushell/completion1.png)
![completion 2](https://github.com/skelly37/literally-everything/raw/main/screenshots/nushell/completion2.png)
*TODO*

## Custom menus
*TODO*

# Built-in parsers
Since nushell prioritizes working on data, it provides functions to convert different types of data into nushell table (and reverse). `help to; help from` will give you the full list, so I'll just mention a few *tos* and *froms* to provide some kind of a preview.

1) **from** — in most cases it parses given string as a file with given extension
    - csv
    - json
    - ssv
    - toml
    - xml
    - yaml
    - mp4 — gets metadata of mp4 file
    - ods/xlsx — binary Excel/.ods data into table

2) **to** – convert your nushell table into desired format
    - csv
    - html
    - json
    - md
    - toml
    - xml
    - yaml

I'm sure it's pretty convincing if you're frequently using some kind of APIs. Just `fetch`, parsing function and getting desired data/selecting columns from the table. Nushell's operations on data are simply a killer-feature.


## Built-in calculator and other useful functions
Surely, you can use bc (*Runtime warning (func=(main), adr=8): non-zero scale in exponent*), you can [make Python shell more friendly](https://github.com/skelly37/funny-experiments#lowkey-aliases-in-python-shell-mode) but it's nice to simply do Super+T, `2 + 2` and get the `4`. I'd also say that Nushell's calculator mode is more advanced than bc, which is another advantage. For example, you can do `2.1 ** 3.704` without errors other than issues with floating-points. 

There's also a whole math library (`help math`) with bunch of useful functions, like sqrt, median, variance, and so on..., e.g. `9 | math sqrt` obviously gives you `3`

There's an advanced random library or even a calendar (customizable start of the week included)! All you can imagine.

One can say *it's bloat* or *it's useless* — sure, but the features are useful for me, so I enjoy the abilities of my shell itself. If you like to reinvent the wheel or mess with the archaic bc, that's your life and your PC. Though, if you care about the minimalistic and somehow poor shell, you wouldn't last until there, so I'll assume that you also like nu as a calculator

## Community repository of user scripts
I enjoy the fact of having such repository. You can share your ideas with other users, help with the development of the shell (stdlib-candidates), you can find something useful instead of reinventing the wheel. PR merge time is pretty fast, people up there are very kind — don't forget to check it out! [link](https://github.com/nushell/nu_scripts/).

Sadly, I haven't seen such thing for any POSIX shell. Categorized GH repository is, at least for me, way more comfortable than going through some stack exchanges and messing up with fragments of code snippets. 

## Self-criticism: what I lack in nushell, what annoys me (in comparison to my bash experience)
Nushell is still under development, so it isn't surprising that nu lacks some features. But how short is that list (when we consider the age of POSIX) surely is. Also keep in mind how easy is to handle most of the issues yourself. The issues are present, though, and I'd like to be fully honest with everyone.

  
### 1) Lack of $CDPATH out-of-the-box
  That's the first issue I've encountered after switching from bash to nu. It's one of the most important functionalities of the shell for me, so I couldn't sleep without solving it. [Here's the result](https://github.com/nushell/nu_scripts/blob/main/filesystem/cdpath.nu). Again — a bit annoying, easily solvable. 
  
Note that this is the only element that's left after just 7 months of this text's **very existence**! Great job, nu devs :)
    

## Bonus: Nushell vs current Windows state
Currently, Windows users have 2 shells preinstalled to everyday use: cmd and PowerShell.

1) cmd / command prompt — it's very poor and unconvenient, I'd say it's even worse than POSIX shells (and I think even Windows users could agree on that), so we'll just leave it be. It's almost impossible for a modern shell to be worse and less-featured than cmd.
 
2) PowerShell — pwsh is a very powerful scripting language, though. It's a very decent competitor for nu (I must admit to envying Windows users having pwsh before finding out about nushell). Leaving scripting behind (pwsh still has better capabilities and is the natural choice in some cases), Nushell is worth trying for Windows command-liners because of one thing: shell-mode-friendliness. Length of the PowerShell commands is a joke, a legend and a horror (depends on whether you're writing the code or reading it). I also tend to write long and descriptive code but it's really unconvenient to use it in casual terminal session. When we combine previously mentioned advantages like: aesthetic output, decent data structures and parsers with the length of nu's commands, we receive something great. 

Windows users, don't be afraid to give it a try! Don't be afraid to ditch cmd and to focus on the true PowerShell advantages and use-cases (messing with Windows API)!
  
## Bonus 2: Bash vs Nushell script  
Here's a simple example (contributed via fellow mentioned before) of a script stopping all running containers via their compose files. Bash vs Nushell, elders first

1) **Bash** 
```
docker ps \
|awk '{print $1}' \ # retrieving container id requires familiarity with awk syntax, alternatively we could use cut here
|tail +2 \ #previous command will return the container ids together with the column header, we use tail here to remove the header
|xargs -I{} docker inspect {} \ | xargs is bash's extremely unintuitively named equivalent of forEach loop
|jq -r '.[].Config.Labels | ."com.docker.compose.project.working_dir" + "/" + ."com.docker.compose.project.config_files"' | # parsing the fields we need requires knowledge of jq parsing syntax. At least we get field concatenation for free
|uniq \ 
 # filter out duplicated values
|xargs -I{} docker-compose -f {} down # another xargs
```

2) **Nushell**
```
docker ps -a|
detect columns|
each {|x| 
  docker inspect $x.ID|
  from json|
  get Config.Labels
}|
flatten|
select 'com.docker.compose.project.working_dir' 'com.docker.compose.project.config_files'|
rename dir file|
uniq|
update file {
  get file|
  split row '/'|
  last|
  get 0
}|
each {
  |x| docker-compose -f $"($x.dir)/($x.file)" down
}
```

Note that Bash solution requires you to also know `awk`, `tail`, `xargs` and `jq`, while Nushell handles everything itself and seems to be more clear and readable (at least for me).

  
## Final thoughts
If you're POSIX or PowerShell user, then I hope you're convinced to try out nushell or at least that you've enjoyed the possibility of finding out about some novelty and understanding it (at least a bit). There are different ways of handling the same problems, we should be more empathetic and open. 

If you're already familiar with nushell — it'd be nice if you found out about something new and that it wasn't a waste of your time. 

I know I was biased but even the title suggests my personal views. Nevertheless, I was trying as much as possible not to ignore the problems and to tell the bare truth. 

Let's treat the issues like comments & discussion section, in case you have anything to say.

**But in the end, used tech is as personal as your panties' color — it's fine as long as it does the job!**
  
## Useful links
- [Main website with docs](https://www.nushell.sh/book/)
- [Github page of the shell](https://github.com/nushell/nushell)
- [Github scripts repository](https://github.com/nushell/nu_scripts/) (previously mentioned)
- [Web demo mode](https://www.nushell.sh/demo/)
- [Discord server](https://discord.com/invite/NtAbbGn) (I can assure you they're all really helpful)
- [Twitter account with updates etc.](https://twitter.com/nu_shell)
- [Bash-Nushell cheatsheet](https://www.nushell.sh/book/coming_from_bash.html)
