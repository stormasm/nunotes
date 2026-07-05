
Stack of how the repl gets called...

- at the bottom of main.rs is a method called run_repl
- the method *run_repl* lives inside run.rs
- inside run.rs is a call to the method *evaluate_repl*

### the file repl.rs in the nu-cli crate

- which lives inside the file repl.rs in the crate nu-cli
- the *evaluate_repl* function has a loop construct inside of it
- inside the loop construct is a call to loop_iteration
-
- every time you hit return [inside the repl] loop_iteration gets called
- from the relative top of the loop construct which lives at line 189 in repl.rs
- at line 747 in the file repl.rs finally we get our input from Reedline and then we deal with it
