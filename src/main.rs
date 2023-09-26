use crunchio::CrunchIO;

fn main() -> Result<(), &'static str> {
    let client = CrunchIO::default();

    // let balance = client.balance();
    // println!("{balance:#?}");

    // let locations = client.locations();
    // println!("{locations:#?}");

    // let scripts = client.scripts();
    // println!("{:#?}", scripts);

    // let ssh_keys = client.ssh_keys();
    // println!("{:#?}", ssh_keys);

    // let instances = client.instances();
    // println!("{:#?}", instances);

    // let volumes = client.volumes();
    // println!("{:#?}", volumes);

    let deploy_id = client.deploy();
    println!("{:#?}", deploy_id);

    // let instance_availabilty = client.instance_availability();
    // println!("{:#?}", instance_availabilty);

    // let instance_types = client.all_instance_types();
    // println!("{instance_types:#?}");

    // let instance_types = client.all_instance_types();
    // println!("{instance_types:#?}");

    // let images = client.images();
    // println!("{images:#?}");

    Ok(())
}
