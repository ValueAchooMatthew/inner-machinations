// Stands in for the Rust side in frontend tests. src/tests/setup.ts routes every invoke from
// @tauri-apps/api/core here, so tests can see what the frontend sent and choose what comes back.

export interface BackendCall {
  command: string;
  args: any;
}

type Handler = (args: any) => unknown;

const calls: Array<BackendCall> = [];
const handlers = new Map<string, Handler>();

// Tauri sends arguments as JSON, turning Maps into plain objects on the way
const toPlainData = (value: unknown): any =>
  JSON.parse(JSON.stringify(value ?? {}, (_key, inner) => (inner instanceof Map ? Object.fromEntries(inner) : inner)));

export const backend = {
  calls,

  reset(): void {
    calls.length = 0;
    handlers.clear();
  },

  // What the command returns from now on. Commands without a handler return undefined
  respond(command: string, handler: Handler): void {
    handlers.set(command, handler);
  },

  callsTo(command: string): Array<any> {
    return calls.filter((call) => call.command === command).map((call) => call.args);
  },

  async invoke(command: string, args?: unknown): Promise<unknown> {
    const plainArgs = toPlainData(args);
    calls.push({ command, args: plainArgs });
    const handler = handlers.get(command);
    return handler ? handler(plainArgs) : undefined;
  },
};
