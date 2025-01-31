import { invoke } from "@tauri-apps/api/core";
import type { Log } from "./lib";

export const load = async () => {
    return {logs: (await invoke("log_list")) as Log[]};
}
