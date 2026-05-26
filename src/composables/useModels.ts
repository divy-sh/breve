import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import {
  getProductStatus,
  purchase,
  PurchaseState,
} from "@choochmeque/tauri-plugin-iap-api";

const modelStatus = ref<string>("UNSET");
const availableModels = ref<Record<string, string>>({});
const downloadedModels = ref<string[]>([]);
const userModels = ref<Record<string, string>>({});

const defaultModel = ref<string>("");
const isSubscribed = ref(false);

export function useModels() {
  async function downloadModel(modelName?: string) {
    try {
      await invoke("download_model", { modelName: modelName });
    } catch (error) {
      console.error("Error downloading model: ", error);
      throw error;
    }
  }

  async function getAvailableModels() {
    try {
      const res = (await invoke("get_available_models")) as Record<
        string,
        string
      >;
      return res;
    } catch (err) {
      console.error("Error fetching available models", err);
      return {};
    }
  }

  async function listDownloadedModels() {
    try {
      const res = (await invoke("list_downloaded_models")) as string[];
      return res.sort();
    } catch (err) {
      console.error("Error fetching downloaded models", err);
      return [] as string[];
    }
  }

  async function deleteModel(modelName: string) {
    try {
      await invoke("delete_model", { modelName });
    } catch (err) {
      console.error("Error deleting model", err);
      throw err;
    }
  }

  async function setDefaultModel(modelName: string) {
    try {
      await invoke("set_default_model", { modelName });
    } catch (err) {
      console.error("Error setting default model", err);
      throw err;
    }
  }

  async function getDefaultModel() {
    try {
      const res = (await invoke("get_default_model")) as string;
      return res;
    } catch (err) {
      console.error("Error fetching default model", err);
      return "";
    }
  }

  async function getModelStatus() {
    try {
      const res = (await invoke("get_model_status")) as string;
      return res;
    } catch (err) {
      console.error("Error fetching model status", err);
      return "";
    }
  }

  async function abortGeneration() {
    try {
      await invoke("abort_generation");
    } catch (err) {
      console.error("Error aborting generation", err);
    }
  }

  async function checkSubscription() {
    try {
      const status = await getProductStatus("breve_monthly_1_99", "subs");
      isSubscribed.value =
        status.isOwned && status.purchaseState === PurchaseState.PURCHASED;
    } catch (err) {
      console.error("IAP Check Failed, defaulting to unsubscribed:", err);
      isSubscribed.value = false;
    }
  }

  async function onSubscribe() {
    try {
      const result = await purchase("breve_monthly_1_99", "subs");
      if (result.purchaseState === PurchaseState.PURCHASED) {
        isSubscribed.value = true;
        await refreshVariables();
      }
    } catch (err) {}
  }

  async function refreshVariables() {
    availableModels.value = await getAvailableModels();
    downloadedModels.value = await listDownloadedModels();
    defaultModel.value = await getDefaultModel();
    modelStatus.value = await getModelStatus();
  }

  return {
    modelStatus,
    availableModels,
    downloadedModels,
    defaultModel,
    isSubscribed,
    downloadModel,
    getAvailableModels,
    listDownloadedModels,
    deleteModel,
    setDefaultModel,
    getDefaultModel,
    getModelStatus,
    abortGeneration,
    refreshVariables,
    checkSubscription,
    onSubscribe,
  };
}
