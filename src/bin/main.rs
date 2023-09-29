use crunchio::CrunchIO;

fn main() -> Result<(), &'static str> {
    let client = CrunchIO::default();

    let all_instance_types = client.get_all_instance_types();
    println!("{all_instance_types:#?}");

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
