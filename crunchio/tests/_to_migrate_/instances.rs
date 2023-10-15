// use crunchio::{instances::ActionType, CrunchIO, InstanceCreateBody, Result};

// #[test]
// #[ignore = "not ready yet"]
// fn test_all_http_method_for_instances() -> Result<()> {
//   let client = CrunchIO::default();

//   let instance_type = "1V100.6V";
//   let image = "9eb0f166-f119-49c9-ba55-e857dd055500";
//   let ssh_key_ids = "f42ab232-eb48-485f-b188-d528ebbf1beb";
//   let hostname = "cuda";
//   let description = "Rust cuda gpu server";
//   let is_spot = true;

//   let _body = InstanceCreateBody {
//     instance_type,
//     image,
//     ssh_key_ids,
//     hostname,
//     description,
//     is_spot,
//     ..Default::default()
//   };

//   // let instance_id = client.create_new_instance(&body);
//   // println!("{instance_id}");

//   let id = "d698dbaf-4bb9-4299-b686-9042e21f909b";
//   let instance = client.get_instance_by_id(id)?;
//   println!("{instance:?}");

//   let info = client.perform_action_on_instance(id, ActionType::Delete)?;
//   println!("{info:?}");

//   Ok(())
//   // let instance = client.get_instance_by_id(id);
//   // println!("{instance:?}");
// }
