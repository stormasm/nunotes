
## Cratification Plans

I am happy and excited to do the work to make this happen and you all
can help guide me along... Will continue to seek guidance in our core
team meetings and give status updates...

I tend to work a bit slower than most so as long as you all our patient
with me this work will get done.  If others are excited to expedite the
process then by all means jump in and help out but I don't mind doing
the work to see this through as long as you all guide me along as we
have been doing in the core team meetings the past few weeks.

So I appreciate Stefan's question and interest in asking for the next
steps so here they are :)

## Background Concepts

This is where we are going:

* nu-cmd-lang
* nu-cmd-shell
* nu-cmd-extra

What will be in these 3 crates is a work in progress.  I think these
crates will continue to evolve over time and lay the ground work for
other developers to come along and build on top of these crates their
own versions of what they would like nushell to do (for them and their
organizations longer term.)
g
## Step One: Example Code

Now that we have two crates in the repo with commands --- if you review
the conversation in [PR 8181](https://github.com/nushell/nushell/pull/8181)
you will note that there are some outstanding issues that mainly Stefan
and I discussed as the PR was unfolding and they are...

#### Busting up the example code into nu-test-support

The first PR landed with (2) copies of
[example_test.rs](https://github.com/nushell/nushell/blob/main/crates/nu-cmd-lang/src/example_test.rs) so
[these 4 methods](https://github.com/stormasm/nutmp/blob/main/code/example_test_support.rs) have to be moved over to **nu-test-support**.  I was planning on making this happen this week.

## Step Two: Test code infrastructure

In PR 8181 Stefan  has some other ideas about how he would like the tests to look.  My plan is to work with him to get this all sorted out post Step One
getting completed...  Since I don't quite understand the scope of this work
I will update more on this point once Step 1 is done.

## Step Three: nu-cmd-shell

Once the test support work is done and we are happy with the concept of how
examples and tests work in the context of multiple command crates we will move
on to creating nu-cmd-shell.  I would prefer to wait on creating nu-cmd-shell
until the ideas for examples and tests are flushed out and we have a working
version living in the code base.

My idea is to start out with nu-cmd-shell have the following directories

* env
* shells
* system
* viewers

The ideas behind why these are bundled together are in the reference documents
below but for the most part they are tightly interconnected in the code base and
natually fall out into one piece of work.

Practially though the shell has to be able to display data so viewers are clearly required..

The config system which also needs to be revamped and refactored is a different
subject and a different document but I could see the config refactor work happening in parallel to this work as time unfolds.  

Bottom line is this...

Currently to bring up nushell the config system requires you need the env system in place so you need this directory.

System has the infrastructure code to run external commands which is interdependent with above mentioned directories.

Other candidates for nu-cmd-shell include

* filesystem
* strings
* filters

## Step Four: Running nu-cmd-lang standalone

The ability to run nu-cmd-lang standalone in the code base requires
you swap out these two methods in [main.rs](https://github.com/nushell/nushell/blob/main/src/main.rs)

```rust
use nu_command::create_default_context;
with
use nu_cmd_lang::create_default_context;
```

It would be nice to be able to flip a switch in the build process or send
a flag into nushell making this happen automagically.

### References

* [Output of version command in nu-cmd-lang](https://github.com/stormasm/nunotes/blob/main/nu-cmd-lang-view.md)
* [Core Team Meeting Notes 2023-02-22 on this subject](https://github.com/stormasm/nunotes/blob/main/nu-cmd-lang.md)
* [Core Team Meeting Notes 2023-02-22](https://hackmd.io/inSnmcirTfO7oTg8hTljwQ)
* [Core Team Meeting Notes 2023-02-15](https://hackmd.io/YeL2nzIUS1ChdcsxUx-T1A)
