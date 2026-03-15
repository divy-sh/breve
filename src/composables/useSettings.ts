import { invoke } from "@tauri-apps/api/core";
import { ModelConfig } from "../types";

export function useSettings() {
    async function getConfig(key: string) {
        return await invoke<string>("get_config", { key });
    }

    async function setConfig(key: string, value: string) {
        return await invoke<void>("set_config", { key, value });
    }

    async function getModelConfig() {
        return await invoke<JSON>("get_model_config")
    }

    async function setModelConfig(payload: ModelConfig) {
        return await invoke<void>("set_model_config", { payload })
    }

    return {
        getConfig,
        setConfig,
        getModelConfig,
        setModelConfig,
    };
}