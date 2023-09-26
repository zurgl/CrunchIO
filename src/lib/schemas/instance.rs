#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct CpuInfo {
    description: String,
    number_of_cores: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct GpuInfo {
    description: String,
    number_of_gpus: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Memory {
    description: String,
    size_in_gigabytes: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Instance {
    gpu: GpuInfo,
    id: String,
    instance_type: String,
    memory: Memory,
    gpu_memory: Memory,
    model: String,
    name: String,
    price_per_hour: String,
    spot_price: String,
}
