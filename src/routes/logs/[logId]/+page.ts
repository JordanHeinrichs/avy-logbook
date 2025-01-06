import { invoke } from "@tauri-apps/api/core";
import type { Log } from "../../lib";

export const load = async ({params}) => {
    return {log: (await invoke("fetch_log", {id: Number(params.logId)})) as Log};
}
