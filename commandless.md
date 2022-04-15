
Move BufferedReader somewhere else possibly
to filesystem/util.rs

It is referenced in these 3 spots.

* filesystem/open.rs
* network/fetch.rs
* network/post.rs

We do not want it in open.rs because of the rusqlite dependency...
