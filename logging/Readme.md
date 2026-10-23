
For logging notes in general...

Does Nushell use simplelog as its main log crate

Answer: No, Nushell does not use simplelog as its main logging crate; instead, it relies on Rust's lightweight log facade (log crate) paired with custom internal logging/diagnostic handling across its modules.

Details

• Logging Facade: Nushell and its internal crates (such as nu-protocol, nu-utils, and nu-plugin) depend on the standard Rust log crate rather than binding directly to a concrete configuration/logger backend like simplelog.

• Dependencies: Checking Nushell's workspace and crate manifests reveals standard usage of log for diagnostic macro calls (info!, warn!, error!, etc.) without simplelog serving as the application's primary logging engine.


