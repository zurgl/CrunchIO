use crunchio::CrunchIO;

#[test]
fn test_get_all_startup_scripts() {
    let client = CrunchIO::default();

    let name = "new_startup";
    let script = r"
#!/bin/bash
apt update
apt upgrade -y
";

    let startup_script_id = client.add_startup_script(name, script);
    assert!(!startup_script_id.is_nil());

    let startup_script_id = client.add_startup_script(name, script);
    assert!(!startup_script_id.is_nil());

    let startup_script = client.get_startup_script_by_id(&startup_script_id);
    assert_eq!(startup_script.len(), 1);

    let startup_scripts = client.get_all_startup_scripts();
    assert_eq!(startup_scripts.len(), 2);

    let ids = startup_scripts
        .iter()
        .skip(1)
        .map(|startup_script| startup_script.id)
        .collect::<Vec<uuid::Uuid>>();

    let deleted = client.delete_startup_scripts(&ids);
    assert_eq!(deleted.count, 1);

    let startup_scripts = client.get_all_startup_scripts();
    assert_eq!(startup_scripts.len(), 1);

    let deleted = client.delete_startup_script_by_id(&startup_script_id);
    assert_eq!(deleted.count, 1);
}

// #[test]
// fn test_delete_startup_scripts() {
//     let client = CrunchIO::default();
//     let _ = client.delete_startup_scripts();
// }

// #[test]
// fn test_get_all_startup_scripts() {
//     let client = CrunchIO::default();
//     let scripts = client.get_all_startup_scripts();
//     println!("{scripts:#?}")
// }

// #[test]
// fn test_add_startup_script() {
//     let client = CrunchIO::default();
//     let name = "new_startup";
//     let script = r"
//       #!/bin/bash
//       apt update
//       apt upgrade -y
//       apt -y install tree unzip zip plocate docker-compose\
//         libssl-dev ffmpeg imagemagick nvtop zlib1g \
//         ninja-build python3-pip direnv git-crypt
//       useradd -m -p $(openssl passwd -1 ${PASSWORD}) -s /bin/bash -G sudo ${USERNAME}
//     ";
//     let _ = client.add_startup_script(name, script);
// }

// #[test]
// fn test_get_startup_script_by_id() {
//     let client = CrunchIO::default();
//     let _ = client.get_startup_script_by_id();
// }

// #[test]
// fn test_delete_startup_script_by_id() {
//     let client = CrunchIO::default();
//     let _ = client.delete_startup_script_by_id();
// }
