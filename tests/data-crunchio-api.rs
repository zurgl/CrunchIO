use crunchio::CrunchIO;

#[test]
fn test_default_client() {
    let _ = CrunchIO::default();
}

#[test]
fn test_get_balance() {
    let client = CrunchIO::default();
    let _ = client.get_balance();
}

#[test]
fn test_get_locations() {
    let client = CrunchIO::default();
    let _ = client.get_locations();
}

#[test]
fn test_get_volumes() {
    let client = CrunchIO::default();
    let _ = client.get_volumes();
}

#[test]
fn test_get_images() {
    let client = CrunchIO::default();
    let _ = client.get_images();
}

#[test]
fn test_get_instantes_availability() {
    let client = CrunchIO::default();
    let _ = client.get_instance_availabilities();
}

#[test]
fn test_get_ssh_keys() {
    let client = CrunchIO::default();
    let _ = client.get_ssh_keys();
}

#[test]
fn test_get_scripts() {
    let client = CrunchIO::default();
    let _ = client.get_all_startup_scripts();
}

#[test]
fn test_get_all_instance_types() {
    let client = CrunchIO::default();
    let _ = client.get_all_instance_types();
}

#[test]
#[ignore = "doesn't have any instance id"]
fn test_get_instance_by_id() {
    let client = CrunchIO::default();
    let id = "someid";
    let _ = client.get_instance_by_id(id);
}

#[test]
fn test_get_instances() {
    let client = CrunchIO::default();
    let _ = client.get_instances();
}
