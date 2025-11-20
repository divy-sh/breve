use crate::config::path_resolver;

#[derive(Clone)]
pub struct Config {
    pub model_name: String,
    pub model_url: String,
    pub batch_size: i32,
    pub max_context_length: i32,
    pub max_context_size: i32,
    db_name: String,
}

impl Config {
    pub fn init() -> Result<Config, String> {
        // Disabling for now until we can better handle systems without OpenCL
        // let (global_mem_bytes, _) = Config::calculate_device_memory();

        let global_mem_bytes: u64 = 4 * 1024 * 1024 * 1024; // Assume 8GB for now
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
            model_name: "Llama-3.2-1B-Instruct-Q4_K_S.gguf".to_string(),
            model_url: "bartowski/Llama-3.2-1B-Instruct-GGUF".to_string(),
            db_name: "data_store.sqlite".to_string(),
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
        self.model_name.clone()
    }

    pub fn get_model_path(&self) -> String {
        path_resolver::paths().app_local_data(&self.model_name).unwrap().to_str().unwrap().to_string()
    }

    pub fn get_db_path(&self) -> String {
        path_resolver::paths().app_local_data(&self.db_name).unwrap().to_str().unwrap().to_string()
    }

    // fn calculate_device_memory() -> (u64, u64) {
    //     let devices = match get_all_devices(CL_DEVICE_TYPE_GPU) {
    //         Ok(gpu_devices) => gpu_devices,
    //         Err(_) => match get_all_devices(CL_DEVICE_TYPE_ALL) {
    //             Ok(all_devices) => all_devices,
    //             Err(e) => {
    //                 panic!("Failed to get OpenCL devices: {:?}", e);
    //             }
    //         }
    //     };

    //     let mut global_mem: u64 = 0;
    //     let mut local_mem: u64 = 0;

    //     for (_, device_id) in devices.iter().enumerate() {
    //         let device = Device::new(*device_id);

    //         global_mem = device.global_mem_size().unwrap();
    //         local_mem = device.local_mem_size().unwrap();
    //         break;
    //     }

    //     (global_mem, local_mem)
    // }
}
