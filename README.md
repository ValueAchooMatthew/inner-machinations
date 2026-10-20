# Inner Machinations

A desktop app for creating, visualizing, and testing deterministic and non-deterministic finite automata, built with Rust, Tauri, SvelteKit, and SQLite.

![Inner Machinations workspace](docs/screenshot.png)

I was inspired to make this project by my studies in discrete math at university, with the goal of letting anybody, anywhere create and visualize DFAs and NFAs, which have applications ranging from pattern matching to compiler theory.

## Features

- Draw DFAs and NFAs (including epsilon transitions) on a canvas
- Check whether an automaton accepts a given string
- Test strings against a regular expression, and see its parse tree
- Convert an NFA to an equivalent DFA
- Minimize a DFA
- Determine the language an automaton accepts, written as a regular expression
- Save automata to a local SQLite database

The regex parser and NFA construction, NFA-to-DFA conversion, DFA minimization, and language determination are implemented from scratch in Rust (`src-tauri/src`).

## Running it locally

You need Rust, Node.js, and the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your OS.

```sh
npm install
npx tauri dev
```

Workspaces are saved to `mydb.sqlite3` in the app's data folder: `~/.local/share/com.mach.build` on Linux, `~/Library/Application Support/com.mach.build` on macOS, and `%APPDATA%\com.mach.build` on Windows.

Regular expressions use `+` (or `|`) for union, `*` for the Kleene star, parentheses for grouping, and `ε` for the empty string.
