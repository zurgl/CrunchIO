// use crunchio::{locations::LocationCode, CrunchIO, Result};

// #[test]
// fn test_all_http_method_for_locations() -> Result<()> {
//   let client = CrunchIO::default();

//   let locations = client.get_locations()?;
//   assert_eq!(locations.len(), 2);

//   let as_fin_01 = locations
//     .iter()
//     .find(|location| location.code == LocationCode::FIN01);

//   assert!(as_fin_01.is_some());

//   let as_ice_01 = locations
//     .iter()
//     .find(|location| location.code == LocationCode::ICE01);

//   assert!(as_ice_01.is_some());
//   Ok(())
// }
