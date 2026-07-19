package com.div.breve

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.safeContentPadding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview

import kotlinx.coroutines.launch

@Composable
@Preview
fun App() {
    // Ideally, inject this ViewModel using Koin or simple instantiation
    val viewModel = remember { InferenceViewModel("Smollm-360m.gguf") }
    val generatedText by viewModel.response.collectAsState()
    val scope = rememberCoroutineScope()

    var inputText by remember { mutableStateOf("") }

    MaterialTheme {
        Column(modifier = Modifier.fillMaxSize().safeContentPadding()) {
            // Your input UI
            Button(onClick = {
                scope.launch { viewModel.generateResponse("Hello, who are you?") }
            }) {
                Text("Generate AI Response")
            }

            // Displaying the streaming output
            Text(text = generatedText)
        }
    }
}