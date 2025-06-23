pub struct Config {
    pub model: String,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_context_size: i32,
}

impl Config {
    pub fn init() -> Result<Config, String> {
        return Ok(Config {
            model: "res/Llama-3.2-3B-Instruct-Q4_K_L.gguf".to_string(),
            batch_size: 10240,
            max_context_length: 10240 - 2048,
            max_context_size: 2048,
        });
    }

    pub fn get_max_context_size(&mut self) -> i32 {
        return self.max_context_size;
    }

    pub fn get_max_context_length(&mut self) -> i32 {
        return self.max_context_length;
    }

    pub fn get_batch_size(&mut self) -> i32 {
        return self.batch_size;
    }

    pub fn get_model_name(&mut self) -> String {
        return self.model.clone();
    }
}
