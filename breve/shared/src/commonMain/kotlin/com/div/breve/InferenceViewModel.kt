package com.div.breve

import kotlinx.coroutines.flow.MutableStateFlow

class InferenceViewModel(modelPath: String) {
    val modelPath = LlamaBridge.getModelPath("Smollm-360m.gguf")



    // UI state to hold the generated text
    val response = MutableStateFlow("")

    suspend fun generateResponse(prompt: String) {
        response.value = "" // Clear previous
        LlamaBridge.initGenerateModel(modelPath)

        LlamaBridge.updateGenerateParams(
            temperature    = 0.7f,
            maxTokens      = 512,
            topP           = 0.95f,
            topK           = 40,
            repeatPenalty  = 1.1f,
            contextLength  = 4096,
            numThreads     = 4,
            useMmap        = true,
            flashAttention = false,
            gpuLayers      = 0,
        )

        // Llamatik provides a streaming API
        LlamaBridge.generate(prompt).collect { token: String ->
            response.value += token
        }
    }
}