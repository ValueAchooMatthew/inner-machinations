// Headless end-to-end test for Inner Machinations. Linux only; see e2e/README.md.
//
// usage: node e2e/run.mjs [binaryPath] [outDir]
//   binaryPath defaults to src-tauri/target/debug/inner-machinations
//   outDir defaults to e2e/results
//
// Runs the app inside a private, invisible KWin compositor (kwin_wayland --virtual) with its own
// D-Bus session, drives it through tauri-driver and WebKitWebDriver, and writes a screenshot plus
// one JSON line per step to outDir. Exits with status 1 if any step fails.
// The app keeps its database in its data folder ($XDG_DATA_HOME/com.mach.build), and the run points
// XDG_* at private folders, so no real data is read or written. The binary is linked into a throwaway
// run folder next to a database in the old format (with an account), which the app should import and
// migrate on first launch.
import { spawn, execFileSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const REPO = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const BINARY = path.resolve(process.argv[2] ?? `${REPO}/src-tauri/target/debug/inner-machinations`);
const OUT = path.resolve(process.argv[3] ?? `${REPO}/e2e/results`);
// Run folders go here rather than in /tmp, which is often held in memory
const WORK = process.env.E2E_WORK_DIR ?? path.join(os.homedir(), '.cache/inner-machinations-e2e');
const TAURI_DRIVER = process.env.TAURI_DRIVER ?? path.join(os.homedir(), '.cargo/bin/tauri-driver');
const STAMP = `${Date.now()}-${process.pid}`;
const RUN_DIR = `${WORK}/run-${STAMP}`;
const APP = `${RUN_DIR}/app-under-test`;
const SOCKET = `im-e2e-${STAMP}`;
const PORT = 4600 + Math.floor(Math.random() * 300);
const NATIVE_PORT = PORT + 1000;
const BASE = `http://127.0.0.1:${PORT}`;
const DB = `${RUN_DIR}/xdg/data/com.mach.build/mydb.sqlite3`;
const LEGACY_DB = `${RUN_DIR}/mydb.sqlite3`;
const sql = (db, query) => execFileSync('sqlite3', [db, query]).toString().trim();
const SCREEN = { width: 1600, height: 1000 };

for (const [what, file] of [['the app binary (build it with `npx tauri build --debug --no-bundle`)', BINARY], ['tauri-driver (set TAURI_DRIVER)', TAURI_DRIVER]]) {
  if (!fs.existsSync(file)) {
    console.error(`Missing ${what}: ${file}`);
    process.exit(2);
  }
}

fs.rmSync(OUT, { recursive: true, force: true });
fs.mkdirSync(OUT, { recursive: true });
for (const d of ['xdg/data', 'xdg/config', 'xdg/cache', 'xdg/state']) fs.mkdirSync(`${RUN_DIR}/${d}`, { recursive: true });
// A hard link is instant and takes no space; the app finds its own path through it, so the old database
// below sits beside "its" executable. Copy instead when the run folder is on another file system
try { fs.linkSync(BINARY, APP); } catch { fs.copyFileSync(BINARY, APP); }
// A database as builds before the data-folder move left it beside the executable: one account, one workspace
execFileSync('sqlite3', [LEGACY_DB], { input: fs.readFileSync(`${REPO}/migrations/2024-02-15-050449_users/up.sql`, 'utf8') + `;
  CREATE TABLE __diesel_schema_migrations (version VARCHAR(50) PRIMARY KEY NOT NULL, run_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);
  INSERT INTO __diesel_schema_migrations (version) VALUES ('20240215050449');
  INSERT INTO users (id, email, password, verified) VALUES (1, 'encrypted-email', 'encrypted-password', 1);
  INSERT INTO saved_workspaces (id, user_id, workspace_name, type_of_automata) VALUES (1, 1, 'legacy-ws', 'DFA');
  INSERT INTO saved_states (workspace_id, position, is_start, is_final) VALUES (1, '200,200', 1, 0), (1, '400,200', 0, 1);
  INSERT INTO saved_connections (workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character)
    VALUES (1, '200,200', '200,200', '400,200', '400,200', 'b');` });
const resultsFile = `${OUT}/results.jsonl`;
fs.writeFileSync(resultsFile, '');

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const log = (...a) => console.log(new Date().toISOString().slice(11, 19), ...a);

// ---------------------------------------------------------------- processes
const env = { ...process.env };
for (const k of ['DISPLAY', 'WAYLAND_DISPLAY', 'SESSION_MANAGER', 'DBUS_SESSION_BUS_ADDRESS', 'XDG_SESSION_ID',
  'XDG_SESSION_TYPE', 'XDG_SESSION_DESKTOP', 'XDG_CURRENT_DESKTOP', 'KDE_FULL_SESSION', 'KDE_SESSION_UID',
  'DESKTOP_SESSION', 'XAUTHORITY']) delete env[k];
Object.assign(env, {
  XDG_DATA_HOME: `${RUN_DIR}/xdg/data`, XDG_CONFIG_HOME: `${RUN_DIR}/xdg/config`,
  XDG_CACHE_HOME: `${RUN_DIR}/xdg/cache`, XDG_STATE_HOME: `${RUN_DIR}/xdg/state`,
  NO_AT_BRIDGE: '1', GTK_USE_PORTAL: '0', WEBKIT_DISABLE_COMPOSITING_MODE: '1',
  // The virtual compositor has no GPU; WebKit's DMA-BUF renderer then dies with
  // "Error 71 (Protocol error) dispatching to Wayland display"
  WEBKIT_DISABLE_DMABUF_RENDERER: '1',
});

// One private D-Bus session holds both the virtual compositor and tauri-driver (which
// launches WebKitWebDriver, which launches the app). Everything shares one process group.
const inner = `
set -e
kwin_wayland --virtual --socket=${SOCKET} --width=${SCREEN.width} --height=${SCREEN.height} \\
  --no-lockscreen --no-global-shortcuts --no-kactivities > "${RUN_DIR}/kwin.log" 2>&1 &
for i in $(seq 1 100); do [ -S "$XDG_RUNTIME_DIR/${SOCKET}" ] && break; sleep 0.1; done
[ -S "$XDG_RUNTIME_DIR/${SOCKET}" ] || { echo "kwin socket missing" >&2; exit 1; }
# The harness asks KWin to close the app's window over this session's bus, as a user closing it would
echo "$DBUS_SESSION_BUS_ADDRESS" > "${RUN_DIR}/bus"
export WAYLAND_DISPLAY=${SOCKET} GDK_BACKEND=wayland
exec "${TAURI_DRIVER}" --port ${PORT} --native-port ${NATIVE_PORT} --native-driver /usr/bin/WebKitWebDriver
`;
const group = spawn('dbus-run-session', ['--', 'bash', '-c', inner], {
  cwd: RUN_DIR, env, detached: true, stdio: ['ignore', fs.openSync(`${RUN_DIR}/driver.log`, 'a'), fs.openSync(`${RUN_DIR}/driver.log`, 'a')],
});
fs.writeFileSync(`${RUN_DIR}/group.pid`, String(group.pid));
log('process group', group.pid, 'run dir', RUN_DIR);

let cleaned = false;
async function cleanup() {
  if (cleaned) return;
  cleaned = true;
  try { if (SID) await wd('DELETE', `/session/${SID}`); } catch {}
  for (const sig of ['SIGTERM', 'SIGKILL']) {
    try { process.kill(-group.pid, sig); } catch {}
    await sleep(800);
  }
  log('cleaned up process group', group.pid);
}
process.on('SIGINT', async () => { await cleanup(); process.exit(130); });

// ---------------------------------------------------------------- webdriver
let SID = null;
const ELEMENT = 'element-6066-11e4-a52e-4f735466cecf';
async function wd(method, p, body) {
  const res = await fetch(BASE + p, {
    method, headers: { 'content-type': 'application/json' },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const json = await res.json().catch(() => ({}));
  if (!res.ok || (json.value && json.value.error)) {
    throw new Error(`${method} ${p} -> ${res.status} ${JSON.stringify(json.value || json).slice(0, 300)}`);
  }
  return json.value;
}
const s = (p) => `/session/${SID}${p}`;
const exec = (script, ...args) => wd('POST', s('/execute/sync'), { script, args });
async function find(selector, using = 'css selector') {
  const v = await wd('POST', s('/element'), { using, value: selector });
  return v[ELEMENT];
}
async function waitFor(fn, what, timeout = 15000) {
  const start = Date.now();
  let last;
  while (Date.now() - start < timeout) {
    try { const v = await fn(); if (v) return v; } catch (e) { last = e; }
    await sleep(250);
  }
  throw new Error(`timed out waiting for ${what}${last ? `: ${last.message}` : ''}`);
}
const waitEl = (selector, using) => waitFor(() => find(selector, using), selector);
const byText = (tag, text) => `//${tag}[contains(normalize-space(.), ${JSON.stringify(text)})]`;
async function click(selector, using) { const el = await waitEl(selector, using); await wd('POST', s(`/element/${el}/click`), {}); return el; }
async function type(selector, text, using) {
  const el = await waitEl(selector, using);
  await wd('POST', s(`/element/${el}/clear`), {});
  await wd('POST', s(`/element/${el}/value`), { text });
  return el;
}
const bodyText = () => exec('return document.body.innerText;');
const waitText = (text, timeout) => waitFor(async () => (await bodyText()).includes(text) && text, `text "${text}"`, timeout);
const route = () => exec('return location.pathname;');

// Closes the app's window the way a user would, by asking the compositor to, through a KWin script
function closeAppWindow() {
  const busEnv = { ...env, DBUS_SESSION_BUS_ADDRESS: fs.readFileSync(`${RUN_DIR}/bus`, 'utf8').trim() };
  const script = `${RUN_DIR}/close-window.js`;
  fs.writeFileSync(script, 'for (const window of workspace.windowList()) { if (window.caption.includes("Inner Machinations")) window.closeWindow(); }');
  const dbus = (...args) => execFileSync('dbus-send', ['--session', '--print-reply', '--dest=org.kde.KWin', ...args], { env: busEnv }).toString();
  const id = dbus('/Scripting', 'org.kde.kwin.Scripting.loadScript', `string:${script}`, 'string:close-window').match(/int32 (\d+)/)[1];
  dbus(`/Scripting/Script${id}`, 'org.kde.kwin.Script.run');
}

const isAppRunning = () => {
  try {
    execFileSync('pgrep', ['-f', APP]);
    return true;
  } catch {
    return false;
  }
};

// Click at canvas-relative pixel coordinates with real pointer events.
async function clickCanvas(x, y) {
  const r = await exec(`const c = document.querySelector(arguments[0]);
    c.scrollIntoView({block: 'nearest', inline: 'nearest'});
    const b = c.getBoundingClientRect(); return {left: b.left, top: b.top, w: b.width, h: b.height};`, 'main canvas');
  const vx = Math.round(r.left + x), vy = Math.round(r.top + y);
  await wd('POST', s('/actions'), { actions: [{ type: 'pointer', id: 'mouse', parameters: { pointerType: 'mouse' }, actions: [
    { type: 'pointerMove', duration: 0, origin: 'viewport', x: vx, y: vy },
    { type: 'pointerDown', button: 0 }, { type: 'pause', duration: 50 }, { type: 'pointerUp', button: 0 },
  ] }] });
  await wd('DELETE', s('/actions'));
  await sleep(250);
}

// ---------------------------------------------------------------- steps
let stepNo = 0;
const summary = [];
async function step(name, fn) {
  stepNo += 1;
  const id = String(stepNo).padStart(2, '0');
  const rec = { step: id, name, ok: false };
  const t0 = Date.now();
  try {
    const returned = (await fn()) || {};
    Object.assign(rec, returned);
    rec.ok = returned.ok === true;
  } catch (e) {
    rec.ok = false;
    rec.error = e.message;
  }
  rec.ms = Date.now() - t0;
  try { rec.route = await route(); } catch {}
  try {
    const png = await wd('GET', s('/screenshot'));
    fs.writeFileSync(`${OUT}/${id}-${name}.png`, Buffer.from(png, 'base64'));
    rec.screenshot = `${id}-${name}.png`;
  } catch (e) { rec.screenshot_error = e.message; }
  fs.appendFileSync(resultsFile, JSON.stringify(rec) + '\n');
  summary.push(rec);
  log(rec.ok ? 'PASS' : 'FAIL', id, name, rec.error || '');
}
const feedback = async () => (await bodyText()).match(/The string\s+was (not )?accepted/)?.[0]?.replace(/\s+/g, ' ') || null;
const dialogue = () => exec(`const d = [...document.querySelectorAll('div.bg-orange-500 div.self-center')].map(e => e.innerText.trim()).filter(Boolean); return d.join(' | ') || null;`);

try {
  await waitFor(async () => (await fetch(`${BASE}/status`)).ok, 'tauri-driver', 20000);
  const sess = await wd('POST', '/session', { capabilities: { alwaysMatch: { 'tauri:options': { application: APP } } } });
  SID = sess.sessionId;
  log('session', SID);
  try { await wd('POST', s('/window/rect'), { x: 0, y: 0, width: SCREEN.width, height: SCREEN.height }); } catch (e) { log('window rect:', e.message); }

  const languageText = () => waitFor(async () => (await bodyText()).match(/The language of the automata is:[^\n]*/)?.[0], 'language text', 8000).catch(() => null);
  const checkString = async (str) => {
    await type('#string', str);
    await click(byText('button', 'Check String'), 'xpath');
    return waitFor(feedback, `feedback for "${str}"`, 8000).catch(() => null);
  };
  const ACCEPTED = 'The string was accepted';
  const REJECTED = 'The string was not accepted';

  await step('a-app-loads', async () => {
    await waitText('Inner Machinations', 20000);
    const t = await bodyText();
    return { ok: fs.existsSync(DB), title: 'Inner Machinations', subtitle: t.match(/A desktop application[^\n]*/)?.[0], db_in_data_folder: fs.existsSync(DB) };
  });

  await step('b-old-database-imported-and-migrated', async () => {
    await click(byText('span', 'Click anywhere to begin'), 'xpath'); // span inside the page-wide link; the click bubbles to it
    await waitText('My Dashboard');
    const card = await waitFor(async () => (await bodyText()).includes('legacy-ws'), 'legacy-ws card', 8000).catch(() => false);
    const tables = sql(DB, "SELECT group_concat(name, ',') FROM (SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name);");
    const columns = sql(DB, "SELECT group_concat(name, ',') FROM pragma_table_info('saved_workspaces');");
    const legacy_tables = sql(LEGACY_DB, "SELECT group_concat(name, ',') FROM (SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name);");
    return { ok: card && !tables.includes('users') && !columns.includes('user_id') && legacy_tables.includes('users'),
      dashboard_shows_legacy_ws: card, tables, saved_workspaces_columns: columns, original_left_in_place: legacy_tables };
  });

  await step('c-open-old-workspace', async () => {
    await click(byText('button', 'legacy-ws'), 'xpath');
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    const language = await languageText();
    const b = await checkString('b');
    const a = await checkString('a');
    await click(byText('a', 'Dashboard'), 'xpath');
    await waitText('My Dashboard');
    return { ok: language === 'The language of the automata is: b' && b === ACCEPTED && a === REJECTED, language, b, a };
  });

  await step('d-create-workspace', async () => {
    await click('div.fixed.top-0 svg'); // "+" new workspace
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    await waitEl('#workspaceName');
    await sleep(800);
    await type('#workspaceName', 'e2e-ws');
    await sleep(1500);
    const name = await exec('return document.querySelector("#workspaceName").value;');
    return { ok: name === 'e2e-ws', workspace_input: name };
  });

  await step('e-build-dfa-and-test', async () => {
    await click(byText('button', 'New Start State'), 'xpath');
    await clickCanvas(200, 200);              // start state
    await clickCanvas(500, 200);              // regular state (action stays on adding states)
    await click(byText('button', 'New Final State'), 'xpath');
    await clickCanvas(500, 200);              // make it final
    await click(byText('button', 'New Connection'), 'xpath');
    await clickCanvas(200, 200);
    await clickCanvas(500, 200);
    await sleep(500);
    const results = {};
    for (const str of ['a', 'b']) {
      results[str] = await checkString(str);
      await sleep(300);
    }
    // Saved without being asked to
    results.save_status = await waitFor(async () => {
      const status = await exec('return document.querySelector("#save_status").innerText.trim();');
      return status === 'Saved' && status;
    }, 'autosave', 5000).catch(() => null);
    results.saved_states = sql(DB, "SELECT count(*) FROM saved_states s JOIN saved_workspaces w ON w.id = s.workspace_id WHERE w.workspace_name = 'e2e-ws';");
    return { ok: results.a === ACCEPTED && results.b === REJECTED && results.save_status === 'Saved' && results.saved_states === '2', ...results };
  });

  await step('f-language-and-minimize', async () => {
    const language = await languageText();
    await click(byText('button', 'Minimize DFA'), 'xpath');
    await sleep(1500);
    // start --a--> final is minimal already, so it stays as drawn
    const said_minimal = (await bodyText()).includes('This DFA is already minimal');
    const after = await languageText();
    const a = await checkString('a');
    const expected = 'The language of the automata is: a';
    return { ok: language === expected && said_minimal && after === expected && a === ACCEPTED, language, said_minimal, language_after_minimize: after, a_after_minimize: a };
  });

  await step('g-save-and-dashboard', async () => {
    await click(byText('a', 'Dashboard'), 'xpath');
    await waitText('My Dashboard');
    const card = await waitFor(async () => (await bodyText()).includes('e2e-ws') && 'e2e-ws', 'e2e-ws card', 8000).catch(() => null);
    return { ok: Boolean(card), card_title: card, last_updated: (await bodyText()).match(/Last updated:[^\n]*/)?.[0] || null };
  });

  await step('h-reopen-workspace', async () => {
    await click(byText('button', 'e2e-ws'), 'xpath');
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    const language = await languageText();
    const fb = await checkString('a');
    return { ok: Boolean(language) && fb === ACCEPTED, language, check_a: fb };
  });

  // Strict checking: the DFA (start --a--> final, alphabet a,b) has no "b" transitions, so with
  // strict checking on, checking a string must report the DFA as incomplete; off, "a" is accepted
  const toggleStrict = () => exec('const c = document.querySelector("#strict"); c.click(); return c.checked;');
  await step('h2-strict-checking', async () => {
    const check = async (str) => {
      await type('#string', str);
      await click(byText('button', 'Check String'), 'xpath');
      await sleep(1200);
      return bodyText();
    };
    const on = await toggleStrict();
    const t_on = await check('a');
    const flagged = /does not specify every connection/.test(t_on);
    const off = await toggleStrict();
    await sleep(300);
    await check('a');
    const accepted_after = await waitFor(feedback, 'feedback after strict off', 8000).catch(() => null);
    return { ok: on === true && flagged && off === false && accepted_after === ACCEPTED,
      strict_on: on, flagged_incomplete: flagged, strict_off: off, a_after_off: accepted_after };
  });

  await step('h3-options-and-renaming', async () => {
    const pressEscape = () => exec('window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape" }));');
    // Add "c" to the alphabet, and try to make the default connection character empty
    await pressEscape();
    await sleep(500);
    await click('button[title="Add a character"]');
    await sleep(300);
    await exec(`const inputs = document.querySelectorAll('input[name="alphabet"]');
      const input = inputs[inputs.length - 1];
      input.value = 'c';
      input.dispatchEvent(new Event('input', { bubbles: true }));
      const character = document.querySelector('#default_character');
      character.value = '';
      character.dispatchEvent(new Event('input', { bubbles: true }));`);
    await pressEscape();
    await sleep(1500);
    const default_character_refused = /must be a single character/.test(await bodyText());
    const alphabet_saved = sql(DB, "SELECT alphabet FROM saved_workspaces WHERE workspace_name = 'e2e-ws';");
    const still_on_workbench = (await route()).includes('workbench');

    // A blank name is explained, not used
    await type('#workspaceName', ' ');
    await exec('document.querySelector("#workspaceName").form.requestSubmit();');
    await sleep(800);
    const blank_name_refused = (await bodyText()).includes('A workspace needs a name');
    await type('#workspaceName', 'e2e-ws');
    await sleep(1500);
    const name_kept = sql(DB, "SELECT count(*) FROM saved_workspaces WHERE workspace_name = 'e2e-ws';") === '1';

    return {
      ok: default_character_refused && alphabet_saved === 'a,b,c' && still_on_workbench && blank_name_refused && name_kept,
      default_character_refused, alphabet_saved, still_on_workbench, blank_name_refused, name_kept,
    };
  });

  await step('i-regexes', async () => {
    await click(byText('a', 'Dashboard'), 'xpath');
    await waitText('My Dashboard');
    await click(byText('a', 'Go To regexes'), 'xpath');
    await waitEl('#regex');
    // The handoff's probes that the old engine got wrong, plus | as union
    const cases = [
      ['a(b+c)*', 'abc', true], ['a(b+c)*', 'ad', false],
      ['ab+c', 'c', true], ['ab+c', 'ac', false], ['a**', 'a', true],
      ['((ab)*c)*', 'ab', false], ['c+(ab)*', 'abc', false], ['a|b', 'b', true],
      ...(process.env.REGEX_PROBES ? JSON.parse(process.env.REGEX_PROBES).map(([re, str]) => [re, str, null]) : []),
    ];
    const results = {};
    let ok = true;
    for (const [re, str, expected] of cases) {
      await type('#regex', re);
      await sleep(500);
      await type('#string_test', str);
      await sleep(900);
      const fb = await feedback();
      results[`${re} / "${str}"`] = fb ?? 'no result';
      if (expected !== null && fb !== (expected ? ACCEPTED : REJECTED)) ok = false;
    }
    // A half-typed expression explains what's missing instead of failing silently
    await type('#regex', 'a+');
    await sleep(800);
    const error_shown = await exec('return document.querySelector("#regex_error").innerText.trim();');
    results['a+ error'] = error_shown;
    if (error_shown !== 'A union (+) needs something after it') ok = false;
    await type('#regex', 'a(b+c)*');
    await sleep(800);
    // Deep enough that its nodes used to overlap
    await type('#regex', '((a+b)(c+d))*((e+f)(g+h))');
    await sleep(800);
    const canvas_drawn = await exec(`const c = document.querySelector('canvas'); const d = c.getContext('2d').getImageData(0,0,c.width,c.height).data; let n = 0; for (let i = 3; i < d.length; i += 4) if (d[i]) n++; return n;`);
    return { ok, ...results, parse_tree_pixels: canvas_drawn };
  });

  // An epsilon cycle used to overflow the stack when checking strings and freeze the window when converting
  await step('j-nfa-epsilon-cycle-and-convert', async () => {
    await click(byText('a', 'Dashboard'), 'xpath');
    await waitText('My Dashboard');
    await click('div.fixed.top-0 svg'); // "+" new workspace, "Untitled Project"
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    await sleep(800);
    await click('//button[normalize-space(.)="NFA"]', 'xpath');
    await click(byText('button', 'New Start State'), 'xpath');
    await clickCanvas(200, 200);
    await clickCanvas(500, 200);
    await clickCanvas(800, 200);
    await click(byText('button', 'New Final State'), 'xpath');
    await clickCanvas(800, 200);
    for (const [from, to] of [[200, 500], [500, 200]]) {
      await click(byText('button', 'New Epsilon Connection'), 'xpath');
      await clickCanvas(from, 200);
      await clickCanvas(to, 200);
    }
    await click(byText('button', 'New Connection'), 'xpath');
    await clickCanvas(500, 200);
    await clickCanvas(800, 200);
    await sleep(500);
    const nfa = { a: await checkString('a'), aa: await checkString('aa'), language: await languageText() };

    await click(byText('button', 'Convert NFA to DFA'), 'xpath');
    const converted = await waitFor(async () => (await bodyText()).includes('Minimize DFA'), 'DFA mode after converting', 8000).catch(() => false);
    await sleep(1000);
    // Converting doesn't save by itself; Ctrl+S does
    await exec('window.dispatchEvent(new KeyboardEvent("keydown", { key: "s", ctrlKey: true }));');
    await sleep(1000);
    const states = sql(DB, "SELECT group_concat(x || ',' || y || ':' || is_start || is_final, ' ') FROM (SELECT s.x, s.y, s.is_start, s.is_final FROM saved_states s JOIN saved_workspaces w ON w.id = s.workspace_id WHERE w.workspace_name = 'Untitled Project' ORDER BY s.x, s.y);");
    const strict_on = await toggleStrict();
    await sleep(300);
    const dfa = { a: await checkString('a'), b: await checkString('b'), language: await languageText(), dialogue: await dialogue() };
    await toggleStrict();
    const expectedLanguage = 'The language of the automata is: a';
    return {
      ok: nfa.a === ACCEPTED && nfa.aa === REJECTED && nfa.language === expectedLanguage && converted
        && states === '200,200:10 300,300:00 800,200:01' && strict_on && dfa.a === ACCEPTED && dfa.b === REJECTED
        && dfa.language === expectedLanguage && !/does not specify/.test(dfa.dialogue ?? ''),
      nfa, converted_to_dfa: converted, saved_states: states, strict_on, dfa,
    };
  });

  await step('k-new-workspace-names-and-home', async () => {
    // "Untitled Project" is taken by now, so the next new workspace is numbered
    await click(byText('a', 'Dashboard'), 'xpath');
    await waitText('My Dashboard');
    await click('div.fixed.top-0 svg');
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    await sleep(800);
    const new_name = await exec('return document.querySelector("#workspaceName").value;');
    await click(byText('a', 'Home'), 'xpath');
    await waitText('Click anywhere to begin');
    if (new_name !== 'Untitled Project 2') return { ok: false, new_name };
    return { ok: (await route()) === '/', new_name, workspaces: sql(DB, "SELECT group_concat(workspace_name, ', ') FROM saved_workspaces;") };
  });

  // Last, since it ends the app: an edit made just before closing the window is saved, not lost
  await step('l-closing-the-window-saves', async () => {
    await click(byText('span', 'Click anywhere to begin'), 'xpath');
    await waitText('My Dashboard');
    await click(byText('button', 'Untitled Project'), 'xpath');
    await waitFor(async () => (await route()).includes('workbench'), 'workbench route');
    await waitText('The language of the automata is');
    await click(byText('button', 'New State'), 'xpath');
    await clickCanvas(1100, 600);
    // Well inside the autosave delay
    closeAppWindow();
    const exited = await waitFor(async () => !isAppRunning(), 'the app to exit', 10000).catch(() => false);
    const saved = sql(DB, "SELECT count(*) FROM saved_states s JOIN saved_workspaces w ON w.id = s.workspace_id WHERE w.workspace_name = 'Untitled Project' AND s.x = 1100 AND s.y = 600;");
    return { ok: exited && saved === '1', app_exited: exited, new_state_saved: saved === '1' };
  });
} catch (e) {
  log('fatal:', e.message);
  fs.appendFileSync(resultsFile, JSON.stringify({ fatal: e.message }) + '\n');
} finally {
  await cleanup();
  const passed = summary.filter((r) => r.ok).length;
  fs.writeFileSync(`${OUT}/summary.json`, JSON.stringify({ binary: BINARY, run_dir: RUN_DIR, passed, total: summary.length, steps: summary }, null, 2));
  log(`${passed}/${summary.length} steps passed; results in ${OUT}`);
  const allPassed = passed === summary.length && summary.length > 0;
  // Keep the run folder, with the app's database and the driver and compositor logs, when something failed
  if (allPassed && !process.env.E2E_KEEP_RUN) {
    fs.rmSync(RUN_DIR, { recursive: true, force: true });
  } else {
    log(`run folder kept: ${RUN_DIR}`);
  }
  process.exitCode = allPassed ? 0 : 1;
}
