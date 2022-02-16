mod device;

use std::io::Error;

use hdd::list_devices;

use crate::device::Device as HMDevice;

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let devices = list_devices()?;

    println!("Devices: \n{:?}", devices);

    //Get the devices, filter out ones that failed, and turn them into our special device struct
    let devices = devices
        .iter()
        .map(|x| HMDevice::new(x))
        .filter(|dev| !dev.is_err())
        .map(|dev| dev.unwrap())
        .collect::<Vec<HMDevice>>();

    for dev in devices {
        println!("Device {:?} is type \"{}\"", dev.path, dev.device_type)
    }

    Ok(())
}
