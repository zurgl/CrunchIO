use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_instances() {
    let client = CrunchIO::default();

    let instances = client.get_all_instance_types();
    println!("{instances:#?}");
    // assert_ne!(instances.len(), 0);
}
/*
{
  best_for: [
    "Gargantuan ML models", "Multi-GPU training", "FP64 HPC", "NVLINK"
  ],
  cpu: {
    description: 45 CPU,
    number_of_cores: 45
  },
  deploy_warning: null,
  description: "Dedicated Hardware Instance",
  gpu: {
    description: "1x H100 SXM5 80GB",
    number_of_gpus: 1
  },
  gpu_memory: {
    description: "80GB GPU RAM",
    size_in_gigabytes: 80
  },
  id: "c01dd00d-0000-4972-ae4e-d429115d055b",
  instance_type: "1H100.80S.45V",
  memory: {
    description: "190GB RAM",
    size_in_gigabytes: 190
  },
  model: "H100 80GB",
  name: "H100 SXM5 80GB",
  p2p: "",
  price_per_hour: "3.50",
  spot_price: "1.50",
  storage:  {
    "description": "dynamic"
  }
}
*/
