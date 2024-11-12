
- [Discord Ref 11/12/2024](https://discord.com/channels/601130461678272522/683070703716925568/1305868667569049671)
- https://github.com/nushell/nu_scripts/blob/main/make_release/Readme.md

To push tags to nushell, you need to set your git remote to SSH, for example

```rust
git remote add publish git@github.com:nushell/nushell.git
```

Then, you'd do git push publish main --tags at the end of step 2

* [Reference](https://hackmd.io/LIKj-derSki6dmtVDwopFA?view)

# The release process of Nushell 0.90.0
## 0. Release direct dependencies
> **Note**  
> the following procedure is the same for `nu-ansi-term` and `reedline` and needs to be repeated

> **Warning**  
> release `nu-ansi-term` **before** `reedline` and `reedline` **before** Nushell

> **Note**  
> `nu-ansi-term` is typically released only when there are changes to publish.
> `reedline` is typically released on the same schedule as Nushell.

> **Note**  
> in the following, `dep` denotes either the `reedline` or the `nu-ansi-term` remote
> e.g. `https://github.com/nushell/reedline` or `git@github.com:nushell/nu-ansi-term`,
> depending on the dependency being installed

- [x] bump the version (example with [`reedline`][reedline bump example] and [`nu-ansi-term`][nu-ansi-term bump example])
- [x] get the latest revision with `git pull dep main`
- [x] publish the crate with `cargo publish` (*need to be a member of the publishing team*)
- [x] tag the project with `git tag v0.xx.0`
- [x] push the release tag with `git push dep main --tags`
- [x] publish the release (include the (breaking) changes and take inspiration from the [other releases](https://github.com/nushell/reedline/releases))
- [x] bump the version on the Nushell side ([example with `reedline`][reedline pin example]) (reference the release notes for courtesy)

## 1. Minor bump of the version ([example][nushell bump example])
- [x] in the repo of Nushell, run `/path/to/nu_scripts/make_release/bump-version.nu`
- [x] Also commit `Cargo.lock` AFTER running a Cargo command like `cargo check --workspace`
    - [x] merge https://github.com/nushell/nushell/pull/11786
    - [x] merge 0.90.1 bump PR: https://github.com/nushell/nushell/pull/11787
    - [ ] test that you can do `cargo build` in all `nu-*` crates, ESPECIALLY `nu-protocol`

## 2. Tag the [`nushell`] repo
> **Warning**  
> this is maybe the most critical step of the whole release process!!
> this step, once pushed to *GitHub* will trigger the release workflows.

> **Note**  
> in the following, `nushell` will be used to pull and push to the [`nushell`] repo,
> e.g. the `nushell` remote would be `https://github.com/nushell/nushell` or `git@github.com:nushell/nushell`

- [x] get the latest version bump commit with `git pull nushell main`
- [x] run `cargo build` to check if it's ok and check last features
- [x] tag the project with `git tag 0.xx.0`
- [x] :warning: push the release tag to *GitHub* `git push nushell main --tags` :warning:

:point_right: check the [CI jobs](https://github.com/nushell/nushell/actions)  
:point_right: check that there is the same number of targets compared to [last release](https://github.com/nushell/nushell/releases/latest)

## 3. Publish `nu` to *crates.io*
- [ ] Wait for the CI jobs to finish
- [x] check the order of dependencies with `nushell/nu_scripts/make_release/nu_deps.nu` from the `nushell` repo
    - NOTE! `nu_deps.nu` reported slightly changed order. If the ordering in the next step is an issue, try https://github.com/nushell/nu_scripts/pull/754
- [ ] release the Nushell crates `nushell/nu_scripts/make_release/nu_release.nu` from the `nushell` repo
    - **ERROR HERE: Got stuck with serde errors when publishing nu-protocol. Crates up to and including nu-test-support are published. nu-protocol fails to compile. Fixed by https://github.com/nushell/nushell/pull/11786**

> **Note**  
> if there is a new crate, you must add it to the `github:nushell:publishing` group (`cargo owner --list`)

> **Note**  
> if a step fails
> - ask the owner to `cargo owner --add github:nushell:publishing`
> - edit the `nu_release.nu` script to start again where it failed
> - re-run the script

- [ ] Yank the already published 0.90.0 crates (`nu-test-support` and above in `nu_release.nu` script)

## 4. Publish the release note on the website
> **Note**  
> the scripts have been written in such a way they can be run from anywhere

- [ ] Check that the notes do not mention 0.90.0 since we're releasing 0.90.1
- [ ] inspect the merged PRs to write changelogs with `./make_release/release-note/list-merged-prs nushell/nushell`
- [ ] reorder sections by priority, what makes the most sense to the user?
- [ ] paste the output of  `./make_release/release-note/list-merged-prs nushell/nushell --label breaking-change --pretty --no-author` to the "*Breaking changes*" section
- [ ] make sure breaking changes titles are clear enough
- [ ] paste the output of `./make_release/release-note/get-full-changelog` to the "*Full changelog*" section
- [ ] mark as *ready for review* when uploading to *crates.io*
- [ ] land when
    - **fully uploaded** to *crates.io*
    - **before** the *GitHub* release

## 5. Publish the release on *GitHub*
- [ ] go to the draft release on the [release page](https://github.com/nushell/nushell/releases)
- [ ] grab the message of [last one](https://github.com/nushell/nushell/releases/latest)
- [ ] wait for the website to publish the release (in the [actions](https://github.com/nushell/nushell.github.io/actions) tab and on the [website](https://www.nushell.sh/blog/))
- [ ] publish the release on *GitHub*

## 6. social media
- [ ] post a status update on Discord
- [ ] tweet about the new release

## 7. Create the next release note PR on the website
- [ ] run `./make_release/release-note/create-pr 0.xx.0 ((date now) + 4wk | format date "%Y-%m-%d" | into datetime)`

## 8. Bump the version as development
- [ ] bump the patch version on [`nushell`] ([example][nushell dev example]) by running
```nushell
/path/to/nu_scripts/make_release/bump-version.nu --patch
```

## After the release
The main things to do once a release has been published are
- landing PRs marked with the [`wait-until-after-nushell-release`](https://github.com/nushell/nushell/labels/wait-until-after-nushell-release) label which have been already approved
- listening to the community for feedback


[reedline bump example]: https://github.com/nushell/reedline/pull/596/files
[nu-ansi-term bump example]: https://github.com/nushell/nu-ansi-term/pull/45/files
[reedline pin example]: https://github.com/nushell/nushell/pull/9532
[nushell bump example]: https://github.com/nushell/nushell/pull/9530/files
[nushell dev example]: https://github.com/nushell/nushell/pull/9543

[`nushell`]: https://github.com/nushell/nushell
[`reedline`]: https://github.com/nushell/reedline
[`nu-ansi-term`]: https://github.com/nushell/nu-ansi-term
