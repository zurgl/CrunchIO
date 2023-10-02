use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crunchio::locations::LocationCode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum VolumeAction {
  #[default]
  Attach,
  Detach,
  Delete,
  Rename,
  #[serde(rename(deserialize = "increase-size"))]
  IncreaseSize,
  Restore,
  Clone,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum VolumeType {
  #[default]
  NVMe,
  HDD,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VolumeActionBody {
  id: Uuid,
  action: VolumeAction,
  #[serde(skip_serializing_if = "Option::is_none")]
  size: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  instance_id: Option<Uuid>,
  #[serde(rename = "type")]
  #[serde(skip_serializing_if = "Option::is_none")]
  volume_type: Option<VolumeType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VolumeCreateBody<'a> {
  name: &'a str,
  size: u64,
  #[serde(rename = "type")]
  volume_type: VolumeType,
  location_code: LocationCode,
  #[serde(skip_serializing_if = "Option::is_none")]
  instance_id: Option<Uuid>,
}

fn main() -> Result<(), &'static str> {
  // let client = CrunchIO::default();

  // let all_instance_types = client.get_all_instance_types();
  // let id = Uuid::new_v4();
  // let default_payload = VolumeActionBody {
  //   id,
  //   ..Default::default()
  // };
  // let payload = json!(default_payload);
  // println!("{payload}");

  let name = "machin";
  let size = 40;

  let payload = json!(VolumeCreateBody {
    name,
    size,
    ..Default::default()
  });

  println!("{payload}");

  // let scripts = client.scripts();
  // println!("{:#?}", scripts);

  // let ssh_keys = client.ssh_keys();
  // println!("{:#?}", ssh_keys);

  // let instances = client.instances();
  // println!("{:#?}", instances);

  // let volumes = client.volumes();
  // println!("{:#?}", volumes);

  // let instance_id = client.deploy();
  // println!("{:#?}", instance_id);

  // let instance_availabilty = client.instance_availability();
  // println!("{:#?}", instance_availabilty);

  // let instance_types = client.all_instance_types();
  // println!("{instance_types:#?}");

  // let instance_id = client.deploy();
  // println!("{:#?}", instance_id);

  // let mut n = 0;
  // loop {
  //     let instance = client.instance_by_id(&instance_id);
  //     let ip = instance.ip;
  //     println!("{ip:#?}");
  //     std::thread::sleep(std::time::Duration::new(20, 0));
  //     n += 1;
  //     if n == 6 {
  //         break;
  //     }
  // }

  // let info = client.hibernate(&instance_id);
  // println!("{:#?}", info);

  // let images = client.images();
  // println!("{images:#?}");

  Ok(())
}
