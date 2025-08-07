

| clr                                                 | name                             | description                                                                                        |
| --------------------------------------------------- | -------------------------------- | -------------------------------------------------------------------------------------------------- |
| <div style="background-color: #d73a4a">&nbsp;</div> | :bug:  bug                       | Something isn't working                                                                            |
| <div style="background-color: #9B1CA0">&nbsp;</div> | LSP                              | Language Server Protocol (nu-lsp)                                                                  |
| <div style="background-color: #006b75">&nbsp;</div> | POSIX-expectations               | Touches on behavior found in other (POSIX-compliant) shells. We need to consider if we follow that |
| <div style="background-color: #852570">&nbsp;</div> | accessibility                    | An issue to make Nushell more accessible to anyone                                                 |
| <div style="background-color: #CB60F1">&nbsp;</div> | alias                            | Issues around support for command aliases, touches parser and name resolution                      |
| <div style="background-color: #566C59">&nbsp;</div> | android                          | Issues observed specifically on Android/Termux etc.                                                |
| <div style="background-color: #fef2c0">&nbsp;</div> | build-package                    | Everything concerning the CI build process and packaging of nushell                                |
| <div style="background-color: #f9d0c4">&nbsp;</div> | cell-path-semantics              | All around the cell path type and its semantics for access                                         |
| <div style="background-color: #BE46A2">&nbsp;</div> | ci                               | Related to one or more of the CI jobs                                                              |
| <div style="background-color: #F33287">&nbsp;</div> | command-overloading              | Problems related to the overloading of command verbs based on different input (pipeline) types     |
| <div style="background-color: #51c916">&nbsp;</div> | completions                      | Issues related to tab completion                                                                   |
| <div style="background-color: #fef2c0">&nbsp;</div> | configuration                    | Issue related to nu's configuration                                                                |
| <div style="background-color: #c2e0c6">&nbsp;</div> | const-eval                       | Evaluation of const expressions at parse time if possible                                          |
| <div style="background-color: #659F2E">&nbsp;</div> | coreutils-uutils                 | Changes relating to coreutils/uutils                                                               |
| <div style="background-color: #C42BA3">&nbsp;</div> | dataframe                        | Work related to the polars dataframe implementation                                                |
| <div style="background-color: #c2e0c6">&nbsp;</div> | datetime-handling                | Semantics and implementation of the datetime/duration types and commands                           |
| <div style="background-color: #76ce40">&nbsp;</div> | delight                          | this feature would delight users                                                                   |
| <div style="background-color: #0366d6">&nbsp;</div> | dependencies                     | Pull requests that update a dependency file                                                        |
| <div style="background-color: #601B01">&nbsp;</div> | deprecation                      | Related to the deprecation of commands/features/options                                            |
| <div style="background-color: #cbea88">&nbsp;</div> | documentation                    | issues relating to documentation                                                                   |
| <div style="background-color: #D54305">&nbsp;</div> | duplicate                        | This issue is a duplicate of another issue and will be consolidated for easier handling            |
| <div style="background-color: #a2eeef">&nbsp;</div> | enhancement                      | New feature or request                                                                             |
| <div style="background-color: #D83D03">&nbsp;</div> | environment                      | Related to the management of environment variables/process state                                   |
| <div style="background-color: #bfdadc">&nbsp;</div> | error-handling                   | How errors in externals/nu code are caught or handled programmatically (see also unhelpful-error)  |
| <div style="background-color: #c65d01">&nbsp;</div> | external-commands                | Issues related to external commands                                                                |
| <div style="background-color: #c2e0c6">&nbsp;</div> | file-format                      | Parsing/Writing of file formats/protocols                                                          |
| <div style="background-color: #F0A6DC">&nbsp;</div> | file-system                      | Related to commands and core nushell behavior around the file system                               |
| <div style="background-color: #D4394F">&nbsp;</div> | freebsd                          | Issues observed specifically on FreeBSD                                                            |
| <div style="background-color: #000000">&nbsp;</div> | github_actions                   | Pull requests that update GitHub Actions code                                                      |
| <div style="background-color: #F2F68B">&nbsp;</div> | glob-expansion                   | Specific behavior around file-system globbing with regular commands or `glob`                      |
| <div style="background-color: #7057ff">&nbsp;</div> | good first issue                 | Good for newcomers                                                                                 |
| <div style="background-color: #008672">&nbsp;</div> | help wanted                      | Extra attention is needed                                                                          |
| <div style="background-color: #CBDC9E">&nbsp;</div> | help-system                      | Related to help commands and our documentation system (not docs itself)                            |
| <div style="background-color: #E6E31D">&nbsp;</div> | history                          | Related to the history                                                                             |
| <div style="background-color: #79431A">&nbsp;</div> | hooks                            | Hooks are used to react to changes during interactive execution                                    |
| <div style="background-color: #F053B0">&nbsp;</div> | inconsistent-behavior            | Behavior between different commands or types inconsistent/unexpected                               |
| <div style="background-color: #fef2c0">&nbsp;</div> | investigate                      | this requires investigation                                                                        |
| <div style="background-color: #19D0DA">&nbsp;</div> | ir-compiler                      | Issues/pull requests related to compiling Nushell parser output to internal representation (IR)    |
| <div style="background-color: #fbca04">&nbsp;</div> | job-control                      | The system to manage background jobs and concurrent tasks                                          |
| <div style="background-color: #d4c5f9">&nbsp;</div> | keybindings                      |                                                                                                    |
| <div style="background-color: #2E5890">&nbsp;</div> | language                         |                                                                                                    |
| <div style="background-color: #56510A">&nbsp;</div> | line editor                      | Issues related to reedline                                                                         |
| <div style="background-color: #0E8A16">&nbsp;</div> | linux                            | A Linux specific issue                                                                             |
| <div style="background-color: #bfd4f2">&nbsp;</div> | localization                     | Issues relating to i18n and locale for display formats                                             |
| <div style="background-color: #0E8A16">&nbsp;</div> | macos                            | A MacOS specific issue                                                                             |
| <div style="background-color: #19567f">&nbsp;</div> | meta-issue                       | An issue that tracks other issues                                                                  |
| <div style="background-color: #44618E">&nbsp;</div> | modules                          |                                                                                                    |
| <div style="background-color: #f9d0c4">&nbsp;</div> | more information needed          | we need more information from you                                                                  |
| <div style="background-color: #0e8a16">&nbsp;</div> | naming-things :bike: :hut:       | Working towards consistent naming. No bikeshedding brigade please!                                 |
| <div style="background-color: #D93F0B">&nbsp;</div> | needs-core-team-attention        | An issue than needs the attention of other core-team members                                       |
| <div style="background-color: #541ebf">&nbsp;</div> | needs-design                     | this feature requires design                                                                       |
| <div style="background-color: #B60205">&nbsp;</div> | needs-triage                     | An issue that hasn't had any proper look                                                           |
| <div style="background-color: #41225B">&nbsp;</div> | networking                       | All about our `http` and `url` commands and everything going accross the network.                  |
| <div style="background-color: #1A43A5">&nbsp;</div> | new-command                      |                                                                                                    |
| <div style="background-color: #4ABBDE">&nbsp;</div> | nuon-format                      | I/O and spec of the nuon data format                                                               |
| <div style="background-color: #c2e0c6">&nbsp;</div> | optimization                     |                                                                                                    |
| <div style="background-color: #ff0000">&nbsp;</div> | panic                            |                                                                                                    |
| <div style="background-color: #f4825d">&nbsp;</div> | parser                           | Issues related to parsing                                                                          |
| <div style="background-color: #00F96C">&nbsp;</div> | paths                            |                                                                                                    |
| <div style="background-color: #3C6630">&nbsp;</div> | pattern-matching                 |                                                                                                    |
| <div style="background-color: #64FCAB">&nbsp;</div> | performance                      | Work to make nushell quicker and use less resources                                                |
| <div style="background-color: #7E098B">&nbsp;</div> | pipeline                         |                                                                                                    |
| <div style="background-color: #7DE835">&nbsp;</div> | pipeline-metadata                | Additional information passed along that is not directly attached to the values (e.g. LSCOLORS)    |
| <div style="background-color: #ea5254">&nbsp;</div> | platform-specific                | platform-specific issues                                                                           |
| <div style="background-color: #1d76db">&nbsp;</div> | plugins                          | This issue is about plugins                                                                        |
| <div style="background-color: #e0c518">&nbsp;</div> | polish                           | this problem makes nu feel unpolished                                                              |
| <div style="background-color: #bfdadc">&nbsp;</div> | post-1.0                         | This cool feature may be in the further future but doesn't have to land before 1.0                 |
| <div style="background-color: #bfd4f2">&nbsp;</div> | pr:api-change                    | This PR should be mentioned in #api-updates channel on Discord                                     |
| <div style="background-color: #51DEE3">&nbsp;</div> | pr:breaking-change               | This PR implies a change affecting users and has to be noted in the release notes                  |
| <div style="background-color: #AE3D37">&nbsp;</div> | pr:bugfix                        | This PR fixes some bug                                                                             |
| <div style="background-color: #c2e0c6">&nbsp;</div> | pr:commands                      | This PR changes our commands in some way                                                           |
| <div style="background-color: #006b75">&nbsp;</div> | pr:errors                        | This PR improves our error messages                                                                |
| <div style="background-color: #bfd4f2">&nbsp;</div> | pr:language                      | This PR makes some language changes                                                                |
| <div style="background-color: #0052cc">&nbsp;</div> | pr:plugin-protocol-change        | Pull requests that will need documentation of the plugin protocol reference to be updated          |
| <div style="background-color: #d4c5f9">&nbsp;</div> | pr:plugins                       | This PR is related to plugins                                                                      |
| <div style="background-color: #19CE1C">&nbsp;</div> | pr:release-note-mention          | Addition/Improvement to be mentioned in the release notes                                          |
| <div style="background-color: #fbca04">&nbsp;</div> | pr:screenshot                    | This PR has a screenshot that could go to release notes                                            |
| <div style="background-color: #d93f0b">&nbsp;</div> | priority                         | these issues are important                                                                         |
| <div style="background-color: #D6EC00">&nbsp;</div> | quality-of-life                  |                                                                                                    |
| <div style="background-color: #d876e3">&nbsp;</div> | question                         | the issue author asks something                                                                    |
| <div style="background-color: #bfdadc">&nbsp;</div> | quoting/expansion                | Issues related to string quoting and expansion of variable or glob patterns                        |
| <div style="background-color: #2F1379">&nbsp;</div> | recursion                        |                                                                                                    |
| <div style="background-color: #d4c5f9">&nbsp;</div> | redirection-pipe                 | All related to redirection to files or more complex pipelines with STDERR                          |
| <div style="background-color: #245C5E">&nbsp;</div> | refactor                         |                                                                                                    |
| <div style="background-color: #78C8E2">&nbsp;</div> | regex                            |                                                                                                    |
| <div style="background-color: #782E32">&nbsp;</div> | regression                       | Something that worked does not work anymore.                                                       |
| <div style="background-color: #A44376">&nbsp;</div> | removal-after-deprecation        | The component has already been sunset with `deprecation` and is now up for final removal           |
| <div style="background-color: #99BC9E">&nbsp;</div> | repl                             |                                                                                                    |
| <div style="background-color: #000000">&nbsp;</div> | rust                             | Pull requests that update Rust code                                                                |
| <div style="background-color: #8F9338">&nbsp;</div> | scoping/name-resolution          | How Nu finds which variables/functions are in scope and to what they are bound                     |
| <div style="background-color: #F0D7CB">&nbsp;</div> | scripting                        | Related to writing scripts                                                                         |
| <div style="background-color: #F84C74">&nbsp;</div> | semantics                        | Places where we should define/clarify nushell's semantics                                          |
| <div style="background-color: #f9d0c4">&nbsp;</div> | should-be-plugin                 | The proposed feature is either experimental or has a niche audience and should be a plugin first   |
| <div style="background-color: #E653F1">&nbsp;</div> | signal-cancel                    | Problems with how executions react to signals or attempts at cancellation (CTRL-C)                 |
| <div style="background-color: #E4185B">&nbsp;</div> | silent-fail                      | Cases where nothing or something bad happens and an error would be helpful and expected            |
| <div style="background-color: #AB007B">&nbsp;</div> | special-characters               |                                                                                                    |
| <div style="background-color: #81241A">&nbsp;</div> | stability-concern                | Design so we can evolve Nu w/o breaking code post 1.0-stability.                                   |
| <div style="background-color: #489CA3">&nbsp;</div> | std-library                      | Defining and improving the standard library written in nu and the core Rust ccommands              |
| <div style="background-color: #0e8a16">&nbsp;</div> | streaming                        | Issues related to streaming data (or collecting data when it should be streamed)                   |
| <div style="background-color: #C19A74">&nbsp;</div> | syntax                           | Changes to the grammar or syntax beyond parser bugfixes                                            |
| <div style="background-color: #8C5FCF">&nbsp;</div> | syntax-highlighting              | Bugs or performance issues with the syntax highlighting logic                                      |
| <div style="background-color: #628E4F">&nbsp;</div> | system-shell-use                 | Issues specific to using Nushell as the only/login shell or setting it as `$SHELL`                 |
| <div style="background-color: #bfd4f2">&nbsp;</div> | tabled                           | Issues about our new table renderer (tabled)                                                       |
| <div style="background-color: #e99695">&nbsp;</div> | tests                            | issues to add tests or fix tests                                                                   |
| <div style="background-color: #d93f0b">&nbsp;</div> | third-party-integration          | Issues at the interface of Nushell with third-party tools and ensuring tools can run on Nu         |
| <div style="background-color: #9B16E2">&nbsp;</div> | toolkit                          |                                                                                                    |
| <div style="background-color: #E3957B">&nbsp;</div> | type-system                      | Problems or features related to nushell's type system                                              |
| <div style="background-color: #d4c5f9">&nbsp;</div> | unhelpful-error                  | The error message you observe is not helpful to identify the problem                               |
| <div style="background-color: #dbf977">&nbsp;</div> | upstream                         | problem with upstream dependency                                                                   |
| <div style="background-color: #BA5178">&nbsp;</div> | wait-for-upstream                | This PR is blocked waiting for change/release of a dependency                                      |
| <div style="background-color: #600EA5">&nbsp;</div> | wait-until-after-nushell-release |                                                                                                    |
| <div style="background-color: #1C6604">&nbsp;</div> | wasm                             |                                                                                                    |
| <div style="background-color: #0E8A16">&nbsp;</div> | windows                          | A Windows specific issue                                                                           |
| <div style="background-color: #ffffff">&nbsp;</div> | wontfix                          | This will not be worked on                                                                         |

<details>
    <summary>Raw JSON</summary>

```json
[
  {
    "color": "d73a4a",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "Something isn't working",
    "id": "MDU6TGFiZWwxMzU2MjY5ODY4",
    "isDefault": false,
    "name": ":bug:  bug",
    "updatedAt": "2022-08-23T18:37:24Z",
    "url": "https://github.com/nushell/nushell/labels/%3Abug%3A%20%20bug"
  },
  {
    "color": "9B1CA0",
    "createdAt": "2024-01-11T19:37:08Z",
    "description": "Language Server Protocol (nu-lsp)",
    "id": "LA_kwDOCxaBas8AAAABfqz4zQ",
    "isDefault": false,
    "name": "LSP",
    "updatedAt": "2024-01-11T19:37:08Z",
    "url": "https://github.com/nushell/nushell/labels/LSP"
  },
  {
    "color": "006b75",
    "createdAt": "2023-08-09T19:42:07Z",
    "description": "Touches on behavior found in other (POSIX-compliant) shells. We need to consider if we follow that",
    "id": "LA_kwDOCxaBas8AAAABW2em6Q",
    "isDefault": false,
    "name": "POSIX-expectations",
    "updatedAt": "2023-08-09T19:42:07Z",
    "url": "https://github.com/nushell/nushell/labels/POSIX-expectations"
  },
  {
    "color": "852570",
    "createdAt": "2023-05-05T16:49:19Z",
    "description": "An issue to make Nushell more accessible to anyone",
    "id": "LA_kwDOCxaBas8AAAABRgd8eA",
    "isDefault": false,
    "name": "accessibility",
    "updatedAt": "2023-05-05T16:49:19Z",
    "url": "https://github.com/nushell/nushell/labels/accessibility"
  },
  {
    "color": "CB60F1",
    "createdAt": "2023-02-27T20:28:23Z",
    "description": "Issues around support for command aliases, touches parser and name resolution",
    "id": "LA_kwDOCxaBas8AAAABNkeJgw",
    "isDefault": false,
    "name": "alias",
    "updatedAt": "2023-02-27T20:28:23Z",
    "url": "https://github.com/nushell/nushell/labels/alias"
  },
  {
    "color": "566C59",
    "createdAt": "2024-04-15T11:35:01Z",
    "description": "Issues observed specifically on Android/Termux etc.",
    "id": "LA_kwDOCxaBas8AAAABls9i4Q",
    "isDefault": false,
    "name": "android",
    "updatedAt": "2024-04-15T11:35:01Z",
    "url": "https://github.com/nushell/nushell/labels/android"
  },
  {
    "color": "fef2c0",
    "createdAt": "2022-06-11T02:35:54Z",
    "description": "Everything concerning the CI build process and packaging of nushell",
    "id": "LA_kwDOCxaBas77np1t",
    "isDefault": false,
    "name": "build-package",
    "updatedAt": "2022-10-10T12:36:50Z",
    "url": "https://github.com/nushell/nushell/labels/build-package"
  },
  {
    "color": "f9d0c4",
    "createdAt": "2024-08-16T17:35:04Z",
    "description": "All around the cell path type and its semantics for access",
    "id": "LA_kwDOCxaBas8AAAABtW5E3A",
    "isDefault": false,
    "name": "cell-path-semantics",
    "updatedAt": "2024-08-16T17:35:04Z",
    "url": "https://github.com/nushell/nushell/labels/cell-path-semantics"
  },
  {
    "color": "BE46A2",
    "createdAt": "2023-05-14T17:04:27Z",
    "description": "Related to one or more of the CI jobs",
    "id": "LA_kwDOCxaBas8AAAABR_IfCw",
    "isDefault": false,
    "name": "ci",
    "updatedAt": "2023-05-14T17:04:27Z",
    "url": "https://github.com/nushell/nushell/labels/ci"
  },
  {
    "color": "F33287",
    "createdAt": "2022-08-23T08:57:22Z",
    "description": "Problems related to the overloading of command verbs based on different input (pipeline) types",
    "id": "LA_kwDOCxaBas8AAAABCYChxg",
    "isDefault": false,
    "name": "command-overloading",
    "updatedAt": "2022-08-23T08:57:22Z",
    "url": "https://github.com/nushell/nushell/labels/command-overloading"
  },
  {
    "color": "51c916",
    "createdAt": "2020-04-28T13:56:03Z",
    "description": "Issues related to tab completion",
    "id": "MDU6TGFiZWwyMDIxODA0NTk1",
    "isDefault": false,
    "name": "completions",
    "updatedAt": "2020-04-28T13:56:03Z",
    "url": "https://github.com/nushell/nushell/labels/completions"
  },
  {
    "color": "fef2c0",
    "createdAt": "2022-05-10T20:46:19Z",
    "description": "Issue related to nu's configuration",
    "id": "LA_kwDOCxaBas71emEk",
    "isDefault": false,
    "name": "configuration",
    "updatedAt": "2022-05-10T20:46:19Z",
    "url": "https://github.com/nushell/nushell/labels/configuration"
  },
  {
    "color": "c2e0c6",
    "createdAt": "2024-08-24T10:59:12Z",
    "description": "Evaluation of const expressions at parse time if possible",
    "id": "LA_kwDOCxaBas8AAAABt0XlKg",
    "isDefault": false,
    "name": "const-eval",
    "updatedAt": "2024-08-24T10:59:12Z",
    "url": "https://github.com/nushell/nushell/labels/const-eval"
  },
  {
    "color": "659F2E",
    "createdAt": "2023-08-22T22:37:37Z",
    "description": "Changes relating to coreutils/uutils",
    "id": "LA_kwDOCxaBas8AAAABXi7heQ",
    "isDefault": false,
    "name": "coreutils-uutils",
    "updatedAt": "2023-08-22T22:37:37Z",
    "url": "https://github.com/nushell/nushell/labels/coreutils-uutils"
  },
  {
    "color": "C42BA3",
    "createdAt": "2021-06-24T18:20:56Z",
    "description": "Work related to the polars dataframe implementation",
    "id": "MDU6TGFiZWwzMTE0NDE3MTMz",
    "isDefault": false,
    "name": "dataframe",
    "updatedAt": "2025-02-23T19:23:28Z",
    "url": "https://github.com/nushell/nushell/labels/dataframe"
  },
  {
    "color": "c2e0c6",
    "createdAt": "2024-09-02T14:00:50Z",
    "description": "Semantics and implementation of the datetime/duration types and commands",
    "id": "LA_kwDOCxaBas8AAAABuY27Bg",
    "isDefault": false,
    "name": "datetime-handling",
    "updatedAt": "2024-09-02T14:00:50Z",
    "url": "https://github.com/nushell/nushell/labels/datetime-handling"
  },
  {
    "color": "76ce40",
    "createdAt": "2019-08-26T02:13:35Z",
    "description": "this feature would delight users",
    "id": "MDU6TGFiZWwxNTE4OTE3MDE5",
    "isDefault": false,
    "name": "delight",
    "updatedAt": "2019-08-26T02:13:35Z",
    "url": "https://github.com/nushell/nushell/labels/delight"
  },
  {
    "color": "0366d6",
    "createdAt": "2022-07-06T20:15:42Z",
    "description": "Pull requests that update a dependency file",
    "id": "LA_kwDOCxaBas8AAAABAIPHVw",
    "isDefault": false,
    "name": "dependencies",
    "updatedAt": "2022-07-06T20:15:42Z",
    "url": "https://github.com/nushell/nushell/labels/dependencies"
  },
  {
    "color": "601B01",
    "createdAt": "2023-08-11T08:00:29Z",
    "description": "Related to the deprecation of commands/features/options",
    "id": "LA_kwDOCxaBas8AAAABW7qcnQ",
    "isDefault": false,
    "name": "deprecation",
    "updatedAt": "2025-02-23T19:23:55Z",
    "url": "https://github.com/nushell/nushell/labels/deprecation"
  },
  {
    "color": "cbea88",
    "createdAt": "2019-08-26T01:51:22Z",
    "description": "issues relating to documentation",
    "id": "MDU6TGFiZWwxNTE4ODkwNDMz",
    "isDefault": true,
    "name": "documentation",
    "updatedAt": "2021-07-15T12:37:12Z",
    "url": "https://github.com/nushell/nushell/labels/documentation"
  },
  {
    "color": "D54305",
    "createdAt": "2023-05-10T13:11:12Z",
    "description": "This issue is a duplicate of another issue and will be consolidated for easier handling",
    "id": "LA_kwDOCxaBas8AAAABRxLo3A",
    "isDefault": true,
    "name": "duplicate",
    "updatedAt": "2023-05-10T13:11:12Z",
    "url": "https://github.com/nushell/nushell/labels/duplicate"
  },
  {
    "color": "a2eeef",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "New feature or request",
    "id": "MDU6TGFiZWwxMzU2MjY5ODc0",
    "isDefault": true,
    "name": "enhancement",
    "updatedAt": "2019-05-10T16:59:42Z",
    "url": "https://github.com/nushell/nushell/labels/enhancement"
  },
  {
    "color": "D83D03",
    "createdAt": "2023-06-30T16:40:40Z",
    "description": "Related to the management of environment variables/process state",
    "id": "LA_kwDOCxaBas8AAAABUsRLcQ",
    "isDefault": false,
    "name": "environment",
    "updatedAt": "2025-02-23T19:24:52Z",
    "url": "https://github.com/nushell/nushell/labels/environment"
  },
  {
    "color": "bfdadc",
    "createdAt": "2022-12-01T10:51:38Z",
    "description": "How errors in externals/nu code are caught or handled programmatically (see also unhelpful-error)",
    "id": "LA_kwDOCxaBas8AAAABIcqcEg",
    "isDefault": false,
    "name": "error-handling",
    "updatedAt": "2022-12-01T10:51:38Z",
    "url": "https://github.com/nushell/nushell/labels/error-handling"
  },
  {
    "color": "c65d01",
    "createdAt": "2019-08-27T17:35:24Z",
    "description": "Issues related to external commands",
    "id": "MDU6TGFiZWwxNTIyMzQzODg2",
    "isDefault": false,
    "name": "external-commands",
    "updatedAt": "2019-08-27T17:35:24Z",
    "url": "https://github.com/nushell/nushell/labels/external-commands"
  },
  {
    "color": "c2e0c6",
    "createdAt": "2022-10-12T12:59:36Z",
    "description": "Parsing/Writing of file formats/protocols",
    "id": "LA_kwDOCxaBas8AAAABFZDSBA",
    "isDefault": false,
    "name": "file-format",
    "updatedAt": "2022-10-12T12:59:36Z",
    "url": "https://github.com/nushell/nushell/labels/file-format"
  },
  {
    "color": "F0A6DC",
    "createdAt": "2022-10-12T12:58:31Z",
    "description": "Related to commands and core nushell behavior around the file system",
    "id": "LA_kwDOCxaBas8AAAABFZC9zQ",
    "isDefault": false,
    "name": "file-system",
    "updatedAt": "2022-10-12T12:58:31Z",
    "url": "https://github.com/nushell/nushell/labels/file-system"
  },
  {
    "color": "D4394F",
    "createdAt": "2024-05-02T00:47:57Z",
    "description": "Issues observed specifically on FreeBSD",
    "id": "LA_kwDOCxaBas8AAAABmx1ZMA",
    "isDefault": false,
    "name": "freebsd",
    "updatedAt": "2024-05-02T00:47:57Z",
    "url": "https://github.com/nushell/nushell/labels/freebsd"
  },
  {
    "color": "000000",
    "createdAt": "2023-01-11T22:31:00Z",
    "description": "Pull requests that update GitHub Actions code",
    "id": "LA_kwDOCxaBas8AAAABKdqK2w",
    "isDefault": false,
    "name": "github_actions",
    "updatedAt": "2023-01-11T22:31:00Z",
    "url": "https://github.com/nushell/nushell/labels/github_actions"
  },
  {
    "color": "F2F68B",
    "createdAt": "2023-07-09T19:53:41Z",
    "description": "Specific behavior around file-system globbing with regular commands or `glob`",
    "id": "LA_kwDOCxaBas8AAAABVL71UQ",
    "isDefault": false,
    "name": "glob-expansion",
    "updatedAt": "2023-07-09T19:53:41Z",
    "url": "https://github.com/nushell/nushell/labels/glob-expansion"
  },
  {
    "color": "7057ff",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "Good for newcomers",
    "id": "MDU6TGFiZWwxMzU2MjY5ODc5",
    "isDefault": true,
    "name": "good first issue",
    "updatedAt": "2019-05-10T16:59:42Z",
    "url": "https://github.com/nushell/nushell/labels/good%20first%20issue"
  },
  {
    "color": "008672",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "Extra attention is needed",
    "id": "MDU6TGFiZWwxMzU2MjY5ODc2",
    "isDefault": true,
    "name": "help wanted",
    "updatedAt": "2019-05-10T16:59:42Z",
    "url": "https://github.com/nushell/nushell/labels/help%20wanted"
  },
  {
    "color": "CBDC9E",
    "createdAt": "2023-05-06T07:17:56Z",
    "description": "Related to help commands and our documentation system (not docs itself)",
    "id": "LA_kwDOCxaBas8AAAABRiM2WA",
    "isDefault": false,
    "name": "help-system",
    "updatedAt": "2024-08-02T19:53:38Z",
    "url": "https://github.com/nushell/nushell/labels/help-system"
  },
  {
    "color": "E6E31D",
    "createdAt": "2023-05-13T07:07:06Z",
    "description": "Related to the history",
    "id": "LA_kwDOCxaBas8AAAABR7Q_sA",
    "isDefault": false,
    "name": "history",
    "updatedAt": "2023-05-13T07:07:06Z",
    "url": "https://github.com/nushell/nushell/labels/history"
  },
  {
    "color": "79431A",
    "createdAt": "2022-09-05T11:37:13Z",
    "description": "Hooks are used to react to changes during interactive execution",
    "id": "LA_kwDOCxaBas8AAAABDC7v5A",
    "isDefault": false,
    "name": "hooks",
    "updatedAt": "2022-09-05T11:37:13Z",
    "url": "https://github.com/nushell/nushell/labels/hooks"
  },
  {
    "color": "F053B0",
    "createdAt": "2022-09-06T13:06:44Z",
    "description": "Behavior between different commands or types inconsistent/unexpected",
    "id": "LA_kwDOCxaBas8AAAABDG3XhQ",
    "isDefault": false,
    "name": "inconsistent-behavior",
    "updatedAt": "2022-09-06T13:06:44Z",
    "url": "https://github.com/nushell/nushell/labels/inconsistent-behavior"
  },
  {
    "color": "fef2c0",
    "createdAt": "2019-08-27T17:34:23Z",
    "description": "this requires investigation",
    "id": "MDU6TGFiZWwxNTIyMzQyMzg2",
    "isDefault": false,
    "name": "investigate",
    "updatedAt": "2019-08-27T17:34:23Z",
    "url": "https://github.com/nushell/nushell/labels/investigate"
  },
  {
    "color": "19D0DA",
    "createdAt": "2024-07-27T07:07:20Z",
    "description": "Issues/pull requests related to compiling Nushell parser output to internal representation (IR)",
    "id": "LA_kwDOCxaBas8AAAABsIJBdQ",
    "isDefault": false,
    "name": "ir-compiler",
    "updatedAt": "2024-07-27T07:07:20Z",
    "url": "https://github.com/nushell/nushell/labels/ir-compiler"
  },
  {
    "color": "fbca04",
    "createdAt": "2025-03-19T16:18:46Z",
    "description": "The system to manage background jobs and concurrent tasks",
    "id": "LA_kwDOCxaBas8AAAAB7-ur4g",
    "isDefault": false,
    "name": "job-control",
    "updatedAt": "2025-03-19T16:18:46Z",
    "url": "https://github.com/nushell/nushell/labels/job-control"
  },
  {
    "color": "d4c5f9",
    "createdAt": "2021-11-25T22:19:05Z",
    "description": "",
    "id": "LA_kwDOCxaBas7Vqv_5",
    "isDefault": false,
    "name": "keybindings",
    "updatedAt": "2021-11-25T22:19:05Z",
    "url": "https://github.com/nushell/nushell/labels/keybindings"
  },
  {
    "color": "2E5890",
    "createdAt": "2023-06-30T16:40:54Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABUsRN3Q",
    "isDefault": false,
    "name": "language",
    "updatedAt": "2023-06-30T16:40:54Z",
    "url": "https://github.com/nushell/nushell/labels/language"
  },
  {
    "color": "56510A",
    "createdAt": "2021-08-24T14:07:21Z",
    "description": "Issues related to reedline",
    "id": "MDU6TGFiZWwzMjkwNjMwMjMy",
    "isDefault": false,
    "name": "line editor",
    "updatedAt": "2022-03-24T16:41:41Z",
    "url": "https://github.com/nushell/nushell/labels/line%20editor"
  },
  {
    "color": "0E8A16",
    "createdAt": "2023-05-05T16:54:40Z",
    "description": "A Linux specific issue",
    "id": "LA_kwDOCxaBas8AAAABRge43g",
    "isDefault": false,
    "name": "linux",
    "updatedAt": "2023-05-05T16:54:40Z",
    "url": "https://github.com/nushell/nushell/labels/linux"
  },
  {
    "color": "bfd4f2",
    "createdAt": "2023-09-28T13:36:14Z",
    "description": "Issues relating to i18n and locale for display formats",
    "id": "LA_kwDOCxaBas8AAAABZsJeAQ",
    "isDefault": false,
    "name": "localization",
    "updatedAt": "2023-09-28T13:36:14Z",
    "url": "https://github.com/nushell/nushell/labels/localization"
  },
  {
    "color": "0E8A16",
    "createdAt": "2023-05-05T16:55:19Z",
    "description": "A MacOS specific issue",
    "id": "LA_kwDOCxaBas8AAAABRge_5Q",
    "isDefault": false,
    "name": "macos",
    "updatedAt": "2023-05-05T16:55:19Z",
    "url": "https://github.com/nushell/nushell/labels/macos"
  },
  {
    "color": "19567f",
    "createdAt": "2019-08-29T17:01:10Z",
    "description": "An issue that tracks other issues",
    "id": "MDU6TGFiZWwxNTI2NDcwOTIy",
    "isDefault": false,
    "name": "meta-issue",
    "updatedAt": "2019-08-29T17:01:44Z",
    "url": "https://github.com/nushell/nushell/labels/meta-issue"
  },
  {
    "color": "44618E",
    "createdAt": "2023-05-19T09:38:25Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABSRdm5g",
    "isDefault": false,
    "name": "modules",
    "updatedAt": "2023-05-19T09:38:25Z",
    "url": "https://github.com/nushell/nushell/labels/modules"
  },
  {
    "color": "f9d0c4",
    "createdAt": "2021-07-14T13:27:34Z",
    "description": "we need more information from you",
    "id": "MDU6TGFiZWwzMTY4MTEyNTM5",
    "isDefault": false,
    "name": "more information needed",
    "updatedAt": "2021-07-14T13:27:34Z",
    "url": "https://github.com/nushell/nushell/labels/more%20information%20needed"
  },
  {
    "color": "0e8a16",
    "createdAt": "2023-09-05T13:34:50Z",
    "description": "Working towards consistent naming. No bikeshedding brigade please!",
    "id": "LA_kwDOCxaBas8AAAABYTqdvw",
    "isDefault": false,
    "name": "naming-things :bike: :hut:",
    "updatedAt": "2023-09-05T13:34:50Z",
    "url": "https://github.com/nushell/nushell/labels/naming-things%20%3Abike%3A%20%3Ahut%3A"
  },
  {
    "color": "D93F0B",
    "createdAt": "2023-04-27T13:24:40Z",
    "description": "An issue than needs the attention of other core-team members",
    "id": "LA_kwDOCxaBas8AAAABRDeXwA",
    "isDefault": false,
    "name": "needs-core-team-attention",
    "updatedAt": "2023-04-27T13:24:40Z",
    "url": "https://github.com/nushell/nushell/labels/needs-core-team-attention"
  },
  {
    "color": "541ebf",
    "createdAt": "2019-08-26T02:09:36Z",
    "description": "this feature requires design",
    "id": "MDU6TGFiZWwxNTE4OTEyMjEx",
    "isDefault": false,
    "name": "needs-design",
    "updatedAt": "2019-08-26T02:09:36Z",
    "url": "https://github.com/nushell/nushell/labels/needs-design"
  },
  {
    "color": "B60205",
    "createdAt": "2023-04-27T13:23:15Z",
    "description": "An issue that hasn't had any proper look",
    "id": "LA_kwDOCxaBas8AAAABRDeElg",
    "isDefault": false,
    "name": "needs-triage",
    "updatedAt": "2023-04-27T13:23:15Z",
    "url": "https://github.com/nushell/nushell/labels/needs-triage"
  },
  {
    "color": "41225B",
    "createdAt": "2023-04-17T12:53:23Z",
    "description": "All about our `http` and `url` commands and everything going accross the network.",
    "id": "LA_kwDOCxaBas8AAAABQdae8w",
    "isDefault": false,
    "name": "networking",
    "updatedAt": "2023-04-17T12:53:23Z",
    "url": "https://github.com/nushell/nushell/labels/networking"
  },
  {
    "color": "1A43A5",
    "createdAt": "2023-06-16T08:29:08Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABT2k15Q",
    "isDefault": false,
    "name": "new-command",
    "updatedAt": "2023-06-16T08:29:08Z",
    "url": "https://github.com/nushell/nushell/labels/new-command"
  },
  {
    "color": "4ABBDE",
    "createdAt": "2022-10-16T11:15:51Z",
    "description": "I/O and spec of the nuon data format",
    "id": "LA_kwDOCxaBas8AAAABFrA60w",
    "isDefault": false,
    "name": "nuon-format",
    "updatedAt": "2022-10-16T11:15:51Z",
    "url": "https://github.com/nushell/nushell/labels/nuon-format"
  },
  {
    "color": "c2e0c6",
    "createdAt": "2019-08-26T00:12:59Z",
    "description": "",
    "id": "MDU6TGFiZWwxNTE4Nzk2NDU0",
    "isDefault": false,
    "name": "optimization",
    "updatedAt": "2019-08-26T00:12:59Z",
    "url": "https://github.com/nushell/nushell/labels/optimization"
  },
  {
    "color": "ff0000",
    "createdAt": "2023-05-21T11:13:57Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABSXxO_g",
    "isDefault": false,
    "name": "panic",
    "updatedAt": "2023-05-21T11:13:57Z",
    "url": "https://github.com/nushell/nushell/labels/panic"
  },
  {
    "color": "f4825d",
    "createdAt": "2019-08-27T18:13:17Z",
    "description": "Issues related to parsing",
    "id": "MDU6TGFiZWwxNTIyNDA0MTI3",
    "isDefault": false,
    "name": "parser",
    "updatedAt": "2019-08-27T18:13:17Z",
    "url": "https://github.com/nushell/nushell/labels/parser"
  },
  {
    "color": "00F96C",
    "createdAt": "2023-06-25T12:04:10Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABUXlD9w",
    "isDefault": false,
    "name": "paths",
    "updatedAt": "2023-06-25T12:04:10Z",
    "url": "https://github.com/nushell/nushell/labels/paths"
  },
  {
    "color": "3C6630",
    "createdAt": "2023-05-17T15:57:47Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABSK7Z0A",
    "isDefault": false,
    "name": "pattern-matching",
    "updatedAt": "2023-05-17T15:57:47Z",
    "url": "https://github.com/nushell/nushell/labels/pattern-matching"
  },
  {
    "color": "64FCAB",
    "createdAt": "2022-08-31T17:55:56Z",
    "description": "Work to make nushell quicker and use less resources",
    "id": "LA_kwDOCxaBas8AAAABC0dp6A",
    "isDefault": false,
    "name": "performance",
    "updatedAt": "2022-08-31T17:55:56Z",
    "url": "https://github.com/nushell/nushell/labels/performance"
  },
  {
    "color": "7E098B",
    "createdAt": "2023-06-14T16:33:17Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABTwJLzw",
    "isDefault": false,
    "name": "pipeline",
    "updatedAt": "2023-06-14T16:33:17Z",
    "url": "https://github.com/nushell/nushell/labels/pipeline"
  },
  {
    "color": "7DE835",
    "createdAt": "2022-12-30T13:15:43Z",
    "description": "Additional information passed along that is not directly attached to the values (e.g. LSCOLORS)",
    "id": "LA_kwDOCxaBas8AAAABJ46UzQ",
    "isDefault": false,
    "name": "pipeline-metadata",
    "updatedAt": "2022-12-30T13:15:43Z",
    "url": "https://github.com/nushell/nushell/labels/pipeline-metadata"
  },
  {
    "color": "ea5254",
    "createdAt": "2019-08-26T03:45:21Z",
    "description": "platform-specific issues",
    "id": "MDU6TGFiZWwxNTE5MDI5Nzk1",
    "isDefault": false,
    "name": "platform-specific",
    "updatedAt": "2021-12-08T10:55:05Z",
    "url": "https://github.com/nushell/nushell/labels/platform-specific"
  },
  {
    "color": "1d76db",
    "createdAt": "2024-04-24T14:35:48Z",
    "description": "This issue is about plugins",
    "id": "LA_kwDOCxaBas8AAAABmUBePQ",
    "isDefault": false,
    "name": "plugins",
    "updatedAt": "2024-04-24T14:35:48Z",
    "url": "https://github.com/nushell/nushell/labels/plugins"
  },
  {
    "color": "e0c518",
    "createdAt": "2019-08-26T03:46:26Z",
    "description": "this problem makes nu feel unpolished",
    "id": "MDU6TGFiZWwxNTE5MDMxMTI2",
    "isDefault": false,
    "name": "polish",
    "updatedAt": "2019-08-26T03:46:26Z",
    "url": "https://github.com/nushell/nushell/labels/polish"
  },
  {
    "color": "bfdadc",
    "createdAt": "2023-08-09T19:17:52Z",
    "description": "This cool feature may be in the further future but doesn't have to land before 1.0",
    "id": "LA_kwDOCxaBas8AAAABW2atVQ",
    "isDefault": false,
    "name": "post-1.0",
    "updatedAt": "2023-08-09T19:17:52Z",
    "url": "https://github.com/nushell/nushell/labels/post-1.0"
  },
  {
    "color": "bfd4f2",
    "createdAt": "2024-03-06T18:02:42Z",
    "description": "This PR should be mentioned in #api-updates channel on Discord",
    "id": "LA_kwDOCxaBas8AAAABjMPNpQ",
    "isDefault": false,
    "name": "pr:api-change",
    "updatedAt": "2024-03-06T18:02:42Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Aapi-change"
  },
  {
    "color": "51DEE3",
    "createdAt": "2022-10-27T19:38:11Z",
    "description": "This PR implies a change affecting users and has to be noted in the release notes",
    "id": "LA_kwDOCxaBas8AAAABGgrJyw",
    "isDefault": false,
    "name": "pr:breaking-change",
    "updatedAt": "2024-01-11T15:24:11Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Abreaking-change"
  },
  {
    "color": "AE3D37",
    "createdAt": "2024-01-11T15:20:31Z",
    "description": "This PR fixes some bug",
    "id": "LA_kwDOCxaBas8AAAABfp7Ewg",
    "isDefault": false,
    "name": "pr:bugfix",
    "updatedAt": "2024-01-11T15:20:48Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Abugfix"
  },
  {
    "color": "c2e0c6",
    "createdAt": "2024-01-17T18:00:27Z",
    "description": "This PR changes our commands in some way",
    "id": "LA_kwDOCxaBas8AAAABgAr4SA",
    "isDefault": false,
    "name": "pr:commands",
    "updatedAt": "2024-01-17T18:07:06Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Acommands"
  },
  {
    "color": "006b75",
    "createdAt": "2024-01-17T18:03:42Z",
    "description": "This PR improves our error messages",
    "id": "LA_kwDOCxaBas8AAAABgAsmHQ",
    "isDefault": false,
    "name": "pr:errors",
    "updatedAt": "2024-01-17T18:03:42Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Aerrors"
  },
  {
    "color": "bfd4f2",
    "createdAt": "2024-01-17T18:09:56Z",
    "description": "This PR makes some language changes",
    "id": "LA_kwDOCxaBas8AAAABgAt5zQ",
    "isDefault": false,
    "name": "pr:language",
    "updatedAt": "2024-01-17T18:09:56Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Alanguage"
  },
  {
    "color": "0052cc",
    "createdAt": "2024-08-06T05:14:54Z",
    "description": "Pull requests that will need documentation of the plugin protocol reference to be updated",
    "id": "LA_kwDOCxaBas8AAAABss2_hQ",
    "isDefault": false,
    "name": "pr:plugin-protocol-change",
    "updatedAt": "2024-08-06T05:14:54Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Aplugin-protocol-change"
  },
  {
    "color": "d4c5f9",
    "createdAt": "2022-07-04T15:33:30Z",
    "description": "This PR is related to plugins",
    "id": "LA_kwDOCxaBas8AAAABAAvhuQ",
    "isDefault": false,
    "name": "pr:plugins",
    "updatedAt": "2024-04-24T14:34:54Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Aplugins"
  },
  {
    "color": "19CE1C",
    "createdAt": "2023-07-04T18:57:16Z",
    "description": "Addition/Improvement to be mentioned in the release notes",
    "id": "LA_kwDOCxaBas8AAAABU5xICA",
    "isDefault": false,
    "name": "pr:release-note-mention",
    "updatedAt": "2024-02-08T20:57:56Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Arelease-note-mention"
  },
  {
    "color": "fbca04",
    "createdAt": "2024-02-08T20:58:18Z",
    "description": "This PR has a screenshot that could go to release notes",
    "id": "LA_kwDOCxaBas8AAAABhYsIPA",
    "isDefault": false,
    "name": "pr:screenshot",
    "updatedAt": "2024-02-08T20:58:18Z",
    "url": "https://github.com/nushell/nushell/labels/pr%3Ascreenshot"
  },
  {
    "color": "d93f0b",
    "createdAt": "2019-08-26T00:21:40Z",
    "description": "these issues are important",
    "id": "MDU6TGFiZWwxNTE4ODAzMTk0",
    "isDefault": false,
    "name": "priority",
    "updatedAt": "2021-12-08T10:46:23Z",
    "url": "https://github.com/nushell/nushell/labels/priority"
  },
  {
    "color": "D6EC00",
    "createdAt": "2024-12-18T12:49:24Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAAB1uKokA",
    "isDefault": false,
    "name": "quality-of-life",
    "updatedAt": "2024-12-18T12:49:24Z",
    "url": "https://github.com/nushell/nushell/labels/quality-of-life"
  },
  {
    "color": "d876e3",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "the issue author asks something",
    "id": "MDU6TGFiZWwxMzU2MjY5ODg0",
    "isDefault": true,
    "name": "question",
    "updatedAt": "2021-12-08T10:59:59Z",
    "url": "https://github.com/nushell/nushell/labels/question"
  },
  {
    "color": "bfdadc",
    "createdAt": "2022-08-25T10:59:55Z",
    "description": "Issues related to string quoting and expansion of variable or glob patterns",
    "id": "LA_kwDOCxaBas8AAAABCf2Hzw",
    "isDefault": false,
    "name": "quoting/expansion",
    "updatedAt": "2022-08-25T11:19:11Z",
    "url": "https://github.com/nushell/nushell/labels/quoting%2Fexpansion"
  },
  {
    "color": "2F1379",
    "createdAt": "2023-06-22T16:46:07Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABUOaKlw",
    "isDefault": false,
    "name": "recursion",
    "updatedAt": "2023-06-22T16:46:07Z",
    "url": "https://github.com/nushell/nushell/labels/recursion"
  },
  {
    "color": "d4c5f9",
    "createdAt": "2023-01-18T21:30:48Z",
    "description": "All related to redirection to files or more complex pipelines with STDERR",
    "id": "LA_kwDOCxaBas8AAAABLUKcyA",
    "isDefault": false,
    "name": "redirection-pipe",
    "updatedAt": "2023-01-18T21:30:48Z",
    "url": "https://github.com/nushell/nushell/labels/redirection-pipe"
  },
  {
    "color": "245C5E",
    "createdAt": "2023-06-13T16:52:04Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABTru1AA",
    "isDefault": false,
    "name": "refactor",
    "updatedAt": "2023-06-13T16:52:04Z",
    "url": "https://github.com/nushell/nushell/labels/refactor"
  },
  {
    "color": "78C8E2",
    "createdAt": "2023-07-01T09:03:30Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABUuMNSw",
    "isDefault": false,
    "name": "regex",
    "updatedAt": "2023-07-01T09:03:30Z",
    "url": "https://github.com/nushell/nushell/labels/regex"
  },
  {
    "color": "782E32",
    "createdAt": "2022-10-13T07:02:47Z",
    "description": "Something that worked does not work anymore.",
    "id": "LA_kwDOCxaBas8AAAABFczQ6g",
    "isDefault": false,
    "name": "regression",
    "updatedAt": "2022-10-13T07:02:47Z",
    "url": "https://github.com/nushell/nushell/labels/regression"
  },
  {
    "color": "A44376",
    "createdAt": "2023-11-08T18:20:05Z",
    "description": "The component has already been sunset with `deprecation` and is now up for final removal",
    "id": "LA_kwDOCxaBas8AAAABcLlfIw",
    "isDefault": false,
    "name": "removal-after-deprecation",
    "updatedAt": "2023-11-08T18:20:05Z",
    "url": "https://github.com/nushell/nushell/labels/removal-after-deprecation"
  },
  {
    "color": "99BC9E",
    "createdAt": "2023-06-28T16:23:34Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABUkk82Q",
    "isDefault": false,
    "name": "repl",
    "updatedAt": "2023-06-28T16:23:34Z",
    "url": "https://github.com/nushell/nushell/labels/repl"
  },
  {
    "color": "000000",
    "createdAt": "2022-07-06T20:15:43Z",
    "description": "Pull requests that update Rust code",
    "id": "LA_kwDOCxaBas8AAAABAIPHWA",
    "isDefault": false,
    "name": "rust",
    "updatedAt": "2022-07-06T20:15:43Z",
    "url": "https://github.com/nushell/nushell/labels/rust"
  },
  {
    "color": "8F9338",
    "createdAt": "2023-07-18T15:56:16Z",
    "description": "How Nu finds which variables/functions are in scope and to what they are bound",
    "id": "LA_kwDOCxaBas8AAAABVqsWhQ",
    "isDefault": false,
    "name": "scoping/name-resolution",
    "updatedAt": "2023-07-18T15:56:16Z",
    "url": "https://github.com/nushell/nushell/labels/scoping%2Fname-resolution"
  },
  {
    "color": "F0D7CB",
    "createdAt": "2023-05-06T07:20:00Z",
    "description": "Related to writing scripts",
    "id": "LA_kwDOCxaBas8AAAABRiNIYQ",
    "isDefault": false,
    "name": "scripting",
    "updatedAt": "2023-05-06T07:20:00Z",
    "url": "https://github.com/nushell/nushell/labels/scripting"
  },
  {
    "color": "F84C74",
    "createdAt": "2022-10-19T18:07:37Z",
    "description": "Places where we should define/clarify nushell's semantics",
    "id": "LA_kwDOCxaBas8AAAABF8pCvg",
    "isDefault": false,
    "name": "semantics",
    "updatedAt": "2022-10-19T18:07:37Z",
    "url": "https://github.com/nushell/nushell/labels/semantics"
  },
  {
    "color": "f9d0c4",
    "createdAt": "2023-08-09T19:16:16Z",
    "description": "The proposed feature is either experimental or has a niche audience and should be a plugin first",
    "id": "LA_kwDOCxaBas8AAAABW2acuA",
    "isDefault": false,
    "name": "should-be-plugin",
    "updatedAt": "2023-08-09T19:16:16Z",
    "url": "https://github.com/nushell/nushell/labels/should-be-plugin"
  },
  {
    "color": "E653F1",
    "createdAt": "2022-12-01T13:49:36Z",
    "description": "Problems with how executions react to signals or attempts at cancellation (CTRL-C)",
    "id": "LA_kwDOCxaBas8AAAABIdP5nw",
    "isDefault": false,
    "name": "signal-cancel",
    "updatedAt": "2022-12-01T13:49:36Z",
    "url": "https://github.com/nushell/nushell/labels/signal-cancel"
  },
  {
    "color": "E4185B",
    "createdAt": "2024-08-16T18:16:44Z",
    "description": "Cases where nothing or something bad happens and an error would be helpful and expected",
    "id": "LA_kwDOCxaBas8AAAABtXBnCw",
    "isDefault": false,
    "name": "silent-fail",
    "updatedAt": "2024-08-16T18:16:44Z",
    "url": "https://github.com/nushell/nushell/labels/silent-fail"
  },
  {
    "color": "AB007B",
    "createdAt": "2023-05-30T16:36:57Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABS5JL4g",
    "isDefault": false,
    "name": "special-characters",
    "updatedAt": "2023-05-30T16:36:57Z",
    "url": "https://github.com/nushell/nushell/labels/special-characters"
  },
  {
    "color": "81241A",
    "createdAt": "2024-05-04T20:48:08Z",
    "description": "Design so we can evolve Nu w/o breaking code post 1.0-stability.",
    "id": "LA_kwDOCxaBas8AAAABm82BAA",
    "isDefault": false,
    "name": "stability-concern",
    "updatedAt": "2024-05-04T20:48:08Z",
    "url": "https://github.com/nushell/nushell/labels/stability-concern"
  },
  {
    "color": "489CA3",
    "createdAt": "2023-03-04T09:34:02Z",
    "description": "Defining and improving the standard library written in nu and the core Rust ccommands",
    "id": "LA_kwDOCxaBas8AAAABN2yU5Q",
    "isDefault": false,
    "name": "std-library",
    "updatedAt": "2023-03-04T09:34:02Z",
    "url": "https://github.com/nushell/nushell/labels/std-library"
  },
  {
    "color": "0e8a16",
    "createdAt": "2022-12-23T00:03:17Z",
    "description": "Issues related to streaming data (or collecting data when it should be streamed)",
    "id": "LA_kwDOCxaBas8AAAABJlCrsg",
    "isDefault": false,
    "name": "streaming",
    "updatedAt": "2022-12-23T00:03:34Z",
    "url": "https://github.com/nushell/nushell/labels/streaming"
  },
  {
    "color": "C19A74",
    "createdAt": "2023-09-05T13:08:45Z",
    "description": "Changes to the grammar or syntax beyond parser bugfixes",
    "id": "LA_kwDOCxaBas8AAAABYTkS8g",
    "isDefault": false,
    "name": "syntax",
    "updatedAt": "2023-09-05T13:08:45Z",
    "url": "https://github.com/nushell/nushell/labels/syntax"
  },
  {
    "color": "8C5FCF",
    "createdAt": "2022-08-22T11:04:27Z",
    "description": "Bugs or performance issues with the syntax highlighting logic",
    "id": "LA_kwDOCxaBas8AAAABCUuV5A",
    "isDefault": false,
    "name": "syntax-highlighting",
    "updatedAt": "2022-08-22T11:04:27Z",
    "url": "https://github.com/nushell/nushell/labels/syntax-highlighting"
  },
  {
    "color": "628E4F",
    "createdAt": "2025-02-05T17:29:50Z",
    "description": "Issues specific to using Nushell as the only/login shell or setting it as `$SHELL`",
    "id": "LA_kwDOCxaBas8AAAAB4yTxYA",
    "isDefault": false,
    "name": "system-shell-use",
    "updatedAt": "2025-02-05T17:54:33Z",
    "url": "https://github.com/nushell/nushell/labels/system-shell-use"
  },
  {
    "color": "bfd4f2",
    "createdAt": "2022-07-06T21:16:25Z",
    "description": "Issues about our new table renderer (tabled)",
    "id": "LA_kwDOCxaBas8AAAABAIXRLw",
    "isDefault": false,
    "name": "tabled",
    "updatedAt": "2022-07-06T21:16:25Z",
    "url": "https://github.com/nushell/nushell/labels/tabled"
  },
  {
    "color": "e99695",
    "createdAt": "2022-06-08T19:34:28Z",
    "description": "issues to add tests or fix tests",
    "id": "LA_kwDOCxaBas77KdS5",
    "isDefault": false,
    "name": "tests",
    "updatedAt": "2022-09-20T14:54:22Z",
    "url": "https://github.com/nushell/nushell/labels/tests"
  },
  {
    "color": "d93f0b",
    "createdAt": "2025-02-10T13:24:06Z",
    "description": "Issues at the interface of Nushell with third-party tools and ensuring tools can run on Nu",
    "id": "LA_kwDOCxaBas8AAAAB5Hoq0A",
    "isDefault": false,
    "name": "third-party-integration",
    "updatedAt": "2025-02-10T13:24:06Z",
    "url": "https://github.com/nushell/nushell/labels/third-party-integration"
  },
  {
    "color": "9B16E2",
    "createdAt": "2023-06-02T16:41:58Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABTE2GeQ",
    "isDefault": false,
    "name": "toolkit",
    "updatedAt": "2023-06-02T16:41:58Z",
    "url": "https://github.com/nushell/nushell/labels/toolkit"
  },
  {
    "color": "E3957B",
    "createdAt": "2022-08-23T08:56:32Z",
    "description": "Problems or features related to nushell's type system",
    "id": "LA_kwDOCxaBas8AAAABCYCXeA",
    "isDefault": false,
    "name": "type-system",
    "updatedAt": "2022-08-23T08:56:32Z",
    "url": "https://github.com/nushell/nushell/labels/type-system"
  },
  {
    "color": "d4c5f9",
    "createdAt": "2022-02-11T18:57:14Z",
    "description": "The error message you observe is not helpful to identify the problem",
    "id": "LA_kwDOCxaBas7j4mbd",
    "isDefault": false,
    "name": "unhelpful-error",
    "updatedAt": "2022-02-11T18:57:14Z",
    "url": "https://github.com/nushell/nushell/labels/unhelpful-error"
  },
  {
    "color": "dbf977",
    "createdAt": "2019-08-26T00:23:45Z",
    "description": "problem with upstream dependency",
    "id": "MDU6TGFiZWwxNTE4ODA0Nzg1",
    "isDefault": false,
    "name": "upstream",
    "updatedAt": "2019-08-26T00:23:45Z",
    "url": "https://github.com/nushell/nushell/labels/upstream"
  },
  {
    "color": "BA5178",
    "createdAt": "2023-07-07T09:32:34Z",
    "description": "This PR is blocked waiting for change/release of a dependency",
    "id": "LA_kwDOCxaBas8AAAABVDzI8w",
    "isDefault": false,
    "name": "wait-for-upstream",
    "updatedAt": "2023-07-07T09:32:34Z",
    "url": "https://github.com/nushell/nushell/labels/wait-for-upstream"
  },
  {
    "color": "600EA5",
    "createdAt": "2022-09-27T13:46:07Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAABESvkeg",
    "isDefault": false,
    "name": "wait-until-after-nushell-release",
    "updatedAt": "2022-09-27T13:46:07Z",
    "url": "https://github.com/nushell/nushell/labels/wait-until-after-nushell-release"
  },
  {
    "color": "1C6604",
    "createdAt": "2025-01-02T21:06:34Z",
    "description": "",
    "id": "LA_kwDOCxaBas8AAAAB2jF43Q",
    "isDefault": false,
    "name": "wasm",
    "updatedAt": "2025-01-02T21:06:34Z",
    "url": "https://github.com/nushell/nushell/labels/wasm"
  },
  {
    "color": "0E8A16",
    "createdAt": "2023-05-05T16:54:23Z",
    "description": "A Windows specific issue",
    "id": "LA_kwDOCxaBas8AAAABRge2CQ",
    "isDefault": false,
    "name": "windows",
    "updatedAt": "2023-05-05T16:54:23Z",
    "url": "https://github.com/nushell/nushell/labels/windows"
  },
  {
    "color": "ffffff",
    "createdAt": "2019-05-10T16:59:42Z",
    "description": "This will not be worked on",
    "id": "MDU6TGFiZWwxMzU2MjY5ODg3",
    "isDefault": true,
    "name": "wontfix",
    "updatedAt": "2019-05-10T16:59:42Z",
    "url": "https://github.com/nushell/nushell/labels/wontfix"
  }
]
```

</details>
