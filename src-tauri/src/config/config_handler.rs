use opencl3::device::{get_all_devices, CL_DEVICE_TYPE_GPU, Device};


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

    pub fn calculate_device_memory(&mut self) -> (i32, i32) {
        let devices = get_all_devices(CL_DEVICE_TYPE_GPU).unwrap();

        for (i, device_id) in devices.iter().enumerate() {
            let device = Device::new(*device_id);

            let name = device.name().unwrap();
            let global_mem = device.global_mem_size().unwrap(); // in bytes
            let local_mem = device.local_mem_size().unwrap();   // in bytes

            println!("GPU Device {}:", i);
            println!("  Name: {}", name);
            println!("  Global Memory: {:.2} MB", self.convert_byte_to_mega_byte(global_mem));
            println!("  Local Memory: {:.2} KB", self.convert_byte_to_mega_byte(local_mem));
        }

        (32, 32)
    }

    fn convert_byte_to_mega_byte(&mut self, size_in_byte: u64) -> u64 {
        return size_in_byte / (1024 * 1024);
    }
}
