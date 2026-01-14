import { invoke } from "@tauri-apps/api/core";

export function useSettings() {
    async function getConfig(key: string) {
        return await invoke<string>("get_config", { key });
    }

    async function setConfig(key: string, value: string) {
        return await invoke<void>("set_config", { key, value });
    }

    return {
        getConfig,
        setConfig
    };
}