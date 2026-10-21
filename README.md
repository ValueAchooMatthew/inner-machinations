# Inner Machinations

A desktop app for creating, visualizing, and testing deterministic and non-deterministic finite automata, built with Rust, Tauri, SvelteKit, and SQLite.

![A DFA for strings ending in "ab", with the language it accepts written below it](docs/screenshot.png)

I was inspired to make this project by my studies in discrete math at university, with the goal of letting anybody, anywhere create and visualize DFAs and NFAs, which have applications ranging from pattern matching to compiler theory.

## Features

- Draw DFAs and NFAs, including epsilon transitions, and drag states and curves into place
- Undo and redo any change
- Check whether an automaton accepts a string, optionally stepping through the states it passes
- Convert an NFA to an equivalent DFA, and minimize a DFA
- See the language an automaton accepts, written as a regular expression
- Test strings against a regular expression, and see its parse tree
- Workspaces are saved as you go, to a SQLite database on your computer

The regex parser and NFA construction, NFA-to-DFA conversion, DFA minimization, and language determination are implemented from scratch in Rust (`src-tauri/src`).

## Installing

Installers for Windows, macOS and Linux are attached to each [release](https://github.com/ValueAchooMatthew/inner-machinations/releases).

The installers aren't signed, so your system will warn you the first time:

- **Windows:** in the "Windows protected your PC" box, choose **More info**, then **Run anyway**.
- **macOS:** after the first attempt to open the app, go to **System Settings > Privacy & Security** and choose **Open Anyway**. Or, in a terminal: `xattr -dr com.apple.quarantine /Applications/inner-machinations.app`
- **Linux:** the `.deb`, `.rpm` and AppImage packages are built on Ubuntu 22.04, so they need glibc 2.35 or later and WebKitGTK 4.1, as Ubuntu 22.04 and Debian 12 have.

## Using it

Pick a tool from the sidebar and click the canvas to place states and draw connections. A connection uses the default character, which the options menu (Esc) sets; to change one, select it with the select tool and type the new character. The select tool also moves states and reshapes curves. Delete removes what is selected, and Ctrl+Z undoes.

Regular expressions write union as `+` (or `|`), "any number of times" as `*`, and the empty string as `ε`, with parentheses for grouping. Characters written one after another are concatenated.

Workspaces are saved to `mydb.sqlite3` in the app's data folder: `~/.local/share/com.mach.build` on Linux, `~/Library/Application Support/com.mach.build` on macOS, and `%APPDATA%\com.mach.build` on Windows.

## Running it from source

You need:

- Rust 1.90 or later
- Node.js 22.12 or later, on an even-numbered release (22, 24, 26 and so on)
- The [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your system. On Linux that includes WebKitGTK 4.1, which Ubuntu 22.04, Debian 12 and later have

```sh
npm install
npx tauri dev
```

## Testing

```sh
npm test                    # the frontend's tests
npm run check               # type checking
cd src-tauri && cargo test  # the backend's tests
```

`npm run test:e2e` drives a built copy of the app through its interface; it runs on Linux only, and [e2e/README.md](e2e/README.md) explains what it needs. GitHub Actions runs the first three on every push and pull request.

## Releasing

Pushing a tag such as `v2.0.0` builds the installers on GitHub Actions and attaches them to a draft release for that tag, to check and then publish. Running the Release workflow by hand builds them without making a release.

## License

[GNU General Public License, version 3 or later](LICENSE). You may use, change and share it, as long as anything you share that is built from it is shared under the same license, with its source.
