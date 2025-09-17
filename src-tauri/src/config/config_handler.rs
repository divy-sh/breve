use opencl3::device::{CL_DEVICE_TYPE_GPU, Device, get_all_devices};

pub struct Config {
    pub model: String,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_context_size: i32,
}

impl Config {
    pub fn init() -> Result<Config, String> {
        let (global_mem_bytes, _) = Config::calculate_device_memory();

        // Memory usage estimates
        let model_size_bytes: u64 = 1024 * 1024 * 1024; //1GB quantized model
        let memory_for_context = global_mem_bytes.saturating_sub(model_size_bytes);

        // Approx memory per token estimate: 8KB
        let bytes_per_token: u64 = 8 * 1024;

        let max_context_tokens = (memory_for_context / bytes_per_token) as i32;

        // Cap at a sane upper bound (for stability)
        let max_tokens_clamped = max_context_tokens.clamp(512, 10240);

        let max_context_size = 2048;
        let batch_size = max_tokens_clamped;

        Ok(Config {
            model: "res/Llama-3.2-1B-Instruct-Q4_K_L.gguf".to_string(),
            batch_size,
            max_context_length: max_tokens_clamped - max_context_size,
            max_context_size,
        })
    }

    pub fn get_max_context_size(&self) -> i32 {
        self.max_context_size
    }

    pub fn get_max_context_length(&self) -> i32 {
        self.max_context_length
    }

    pub fn get_batch_size(&self) -> i32 {
        self.batch_size
    }

    pub fn get_model_name(&self) -> String {
        self.model.clone()
    }

    fn calculate_device_memory() -> (u64, u64) {
        let devices = get_all_devices(CL_DEVICE_TYPE_GPU).unwrap();

        let mut global_mem: u64 = 0;
        let mut local_mem: u64 = 0;

        for (_, device_id) in devices.iter().enumerate() {
            let device = Device::new(*device_id);

            global_mem = device.global_mem_size().unwrap();
            local_mem = device.local_mem_size().unwrap();
            break;
        }

        (global_mem, local_mem)
    }
}
