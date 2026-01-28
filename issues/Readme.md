

I am getting this issue on startup...

Error: nu::shell::error

  × Can't read old plugin file to migrate
  help: No such file or directory (os error 2)

----

Do I see the issue here ?

43dff0c94b2c635e558e919186da60f7e138140b

---

tmp20 is the release version of nushell from yash last week...

---

Without XDG this happens in the released version 0.110.0

Environment config file created at: /Users/ma/Library/Application Support/nushell/env.nu
Config file created at: /Users/ma/Library/Application Support/nushell/config.nu

---

With XDG

WARNING: XDG_CONFIG_HOME has been set but /Users/ma/.config/nushell is empty.

Nushell will not move your configuration files from /Users/ma/Library/Application Support/nushell

Environment config file created at: /Users/ma/.config/nushell/env.nu
Config file created at: /Users/ma/.config/nushell/config.nu

---

-rw-r--r--  1 ma  staff  598 Jan 27 19:33 config.nu
-rw-r--r--  1 ma  staff  536 Jan 27 19:33 env.nu

---
