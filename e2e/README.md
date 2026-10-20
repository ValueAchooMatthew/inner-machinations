# End-to-end test

`run.mjs` drives a built copy of the app through its real UI: importing a database from an older build, building and testing a DFA, minimizing, saving and reopening, strict checking, the regex builder, and converting an NFA with an epsilon cycle. It runs on Linux only, inside an invisible KWin session, so it never touches your desktop or your saved workspaces.

## Requirements

- `kwin_wayland`, `dbus-run-session` and `sqlite3`
- `WebKitWebDriver` at `/usr/bin/WebKitWebDriver` (Arch: `webkit2gtk-4.1`; Debian and Ubuntu: `webkit2gtk-driver`)
- `tauri-driver`: `cargo install tauri-driver --locked`

## Running

```sh
npx tauri build --debug --no-bundle
npm run test:e2e
```

Each step writes a screenshot and a line of JSON to `e2e/results`. The command exits with status 1 if any step fails.

Options, as environment variables:

- `TAURI_DRIVER`: path to `tauri-driver`, if it isn't in `~/.cargo/bin`
- `E2E_WORK_DIR`: where run folders go (default `~/.cache/inner-machinations-e2e`). A run folder holds the app's data and the driver and compositor logs. It is deleted after a passing run and kept after a failing one
- `E2E_KEEP_RUN=1`: keep the run folder even when every step passes
- `REGEX_PROBES='[["ab+c","c"]]'`: extra regex and string pairs to try on the regex page; their results are recorded but never fail the run

A debug build plus the test uses a lot of memory. On a machine with little to spare, cap it:

```sh
systemd-run --user --scope -p MemoryMax=3G -p MemorySwapMax=0 npm run test:e2e
```
