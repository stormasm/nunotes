
### Headless Shell Protocol

- https://git.ikl.sh/132ikl/hsp-rs

In the context of Nushell, HSP is not a formal part of the core language, but rather refers to a specialized personal project known as the Headless Shell Protocol (HSP) on GitHub.

Because Nushell is designed around structured data pipelines instead of plain text streams, traditional text-parsing utilities like grep or jq are not always efficient or necessary. To overcome this, the Headless Shell Protocol project provides a JSON-RPC-based system that allows external programs and editors to interface directly with a "headless" instance of the shell.

### Typical Use Cases for HSP

Remote command evaluation: Allowing external clients or language environments to programmatically send blocks of Nushell code and retrieve structured data (like tables or records) directly in JSON format.

No jq dependency: Bypassing the need to pipe Nushell's data to external jq tools, allowing raw programmatic communication with the shell.
