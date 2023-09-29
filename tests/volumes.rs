use crunchio::CrunchIO;

#[test]
fn test_all_http_method_for_volumes() {
    let client = CrunchIO::default();

    let volumes = client.get_all_storage_volumes();
    // println!("{volumes:#?}");
    assert_ne!(volumes.len(), 0);

    let volume_types = client.get_volumes_types();
    println!("{volume_types:#?}");
    assert_ne!(volume_types.len(), 0);
}
