# Need to retrieve complete api data type

## SSH Keys

### Get ssh keys

```rust
struct GetSSHKey {
  id: String,
  name: String,
  key: String
}
```

```json
[
  {
    "id": "26d43424-229e-4b45-bae2-86eb1e919514",
    "name": "Ukkos laptop",
    "key": "..."
  },
...
]
```

### Images

```json
[
  {
    id: "2088da25-bb0d-41cc-a191-dccae45d96fd",
    details: ["Ubuntu 22.04", "CUDA 12.0", "Docker"],
    image_type: "ubuntu-22.04-cuda-12.0-docker",
    name: "Ubuntu 22.04 + CUDA 12.0 + Docker"
  },
  {
    id: "1988da25-bb0d-41cc-a191-dccae45d96fd",
    details: ["Ubuntu 22.04", "CUDA 12.0"],
    image_type:  "ubuntu-22.04-cuda-12.0",
    name: "Ubuntu 22.04 + CUDA 12.0"
  },
  {
    id: "1888da25-bb0d-41cc-a191-dccae45d96fd",
    details: ["Ubuntu 20.04", "CUDA 11.6", "PyTorch 1.12"],
    image_type: "jupyter",
    name: "Jupyter"
  },
...
   {
    id: "0488da25-bb0d-41cc-a191-dccae45d96fd",
    details: ["Ubuntu 18.04", "Minimal Image"], 
    image_type: "ubuntu-18.04",
    name: "Ubuntu 18.04"
  }
]
```

### instance_availabilty

```json
[
  {
    location_code: "FIN-01",
    availabilities: [ 
      "1A100.22V", 
      "1A100.40S.22V", 
      "1RTX6000ADA.10V", 
      "1A6000.10V", 
      "2A6000.20V", 
      "1V100.6V", 
      "CPU.4V.16G", 
      "CPU.8V.32G", 
      "CPU.16V.64G",
      "CPU.32V.128G",
      "CPU.64V.256G",
      "CPU.96V.384G",
      "CPU.120V.480G"
    ]
  },
  {
    location_code: "ICE-01", 
    availabilities:[
      "CPU.4V.16G",
      "CPU.8V.32G",
      "CPU.16V.64G",
      "CPU.32V.128G",
      "CPU.64V.256G",
      "CPU.96V.384G",
      "CPU.120V.480G"
    ]
  }
]
```

### Instances

```json
[
  {
    id: "55f99186-c7bd-4c16-b07f-75d162f92ad3",
    ip: "65.108.32.177",
    status: "running", 
    created_at: "2023-09-26T21:57:40.186Z",
    cpu: {
      description: "6 CPU",
      number_of_cores: 6
    },
    gpu: {
      description: "1x NVidia Tesla V100 16GB",
      number_of_gpus: 1
    },
    gpu_memory: {
      description: "16GB GPU RAM",
      size_in_gigabytes: 16
    },
    memory: { 
      description: "23GB RAM",
      size_in_gigabytes: 23
    },
    storage: {
      description: "225GB NVME", 
      size_in_gigabytes: 225
    }, 
    hostname: "cuda",
    description: "Rust cuda gpu server", 
    location: "FIN-01", 
    price_per_hour: 0.89, 
    is_spot: false, 
    instance_type: "1V100.6V",
    image: "ubuntu-22.04-cuda-12.0-docker",
    os_name: "OS-NVMe-gGUp9QVj", 
    startup_script_id: null,
    ssh_key_ids: [
      "f42ab232-eb48-485f-b188-d528ebbf1beb"
    ],
    os_volume_id: "0cf3cae6-5be0-4876-8772-203ce7094cc9",
    jupyter_token: null
  }
]"
```

### Volumes

```json
[
  {
    id: "9eb0f166-f119-49c9-ba55-e857dd055500",
    instance_id: null,
    status: "detached",
    name: "OS-NVMe-w7JCitk3",
    size: 75,
    is_os_volume: true,
    created_at: "2023-09-16T22:04:14.786Z", 
    target: null,
    type: "NVMe",
    location: "FIN-01",
    ssh_key_ids: [
      "f42ab232-eb48-485f-b188-d528ebbf1beb"
    ]
  }
]
```



"4755734c-63fc-42df-9bfc-ff5ec2802cfa"
"[{\"id\":\"4755734c-63fc-42df-9bfc-ff5ec2802cfa\",\"ip\":null,\"status\":\"provisioning\",\"created_at\":\"2023-09-27T11:18:43.250Z\",\"cpu\":{\"description\":\"6 CPU\",\"number_of_cores\":6},\"gpu\":{\"description\":\"1x NVidia Tesla V100 16GB\",\"number_of_gpus\":1},\"gpu_memory\":{\"description\":\"16GB GPU RAM\",\"size_in_gigabytes\":16},\"memory\":{\"description\":\"23GB RAM\",\"size_in_gigabytes\":23},\"storage\":{\"description\":\"225GB NVME\",\"size_in_gigabytes\":225},\"hostname\":\"cuda\",\"description\":\"Rust cuda gpu server\",\"location\":\"FIN-01\",\"price_per_hour\":0.25,\"is_spot\":true,\"instance_type\":\"1V100.6V\",\"image\":\"ubuntu-22.04-cuda-12.0-docker\",\"os_name\":\"OS-NVMe-w7JCitk3\",\"startup_script_id\":null,\"ssh_key_ids\":[\"f42ab232-eb48-485f-b188-d528ebbf1beb\"],\"os_volume_id\":\"9eb0f166-f119-49c9-ba55-e857dd055500\",\"jupyter_token\":null}]"
