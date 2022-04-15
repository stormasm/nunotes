
nu-command cleanup

* move duplicated code to central location
* remove rusqlite dependency tied to src/main.rs

Move BufferedReader to nu-command/src/util.rs

It is referenced in these 3 spots.

* filesystem/open.rs
* network/fetch.rs
* network/post.rs

We do not want it in open.rs because of the rusqlite dependency...

recently the rusqlite crate was added to filesystem/open
to enable the ability to open sqlite databases.

in doing that it tied a rusqlite crate dependency to main.rs

this is not prudent long term as custom nu-command crates
will be inhibited due to main.rs needing this particular
open command which is not related in any way to main.rs
