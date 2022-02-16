mod device;

use std::io::Error;

use hdd::{ata::ATADevice, list_devices, scsi::SCSIDevice, Device};

use crate::device::{Device as HMDevice, DeviceType};

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let devices = list_devices()?;

    println!("Devices: \n{:?}", devices);

    let devices = devices
        .iter()
        .map(|x| (Device::open(x), x))
        .filter(|(dev, path)| !dev.is_err())
        .map(|(dev, path)| (dev.unwrap(), path))
        .map(|(dev, path)| {
            let new_dev = HMDevice {
                path: path.to_path_buf(),
                device_type: match dev.get_type() {
                    Err(e) => DeviceType::Unknown,
                    Ok(hdd::Type::SCSI) => {
                        // Stolen from crate hdd source code
                        // check whether devices replies to ATA PASS-THROUGH
                        let satdev = ATADevice::new(SCSIDevice::new(dev));
                        match hdd::ata::misc::Misc::get_device_id(&satdev) {
                            // this is really an ATA device
                            Ok(id) => DeviceType::ATA(satdev),
                            // nnnnope, plain SCSI
                            Err(hdd::ata::misc::Error::SCSI(hdd::scsi::ATAError::NotSupported)) => {
                                DeviceType::SCSI(satdev.unwrap())
                            }
                            _ => DeviceType::SCSI(satdev.unwrap()),
                        }
                    }
                    _ => DeviceType::Unknown,
                },
            };
            return new_dev;
        })
        .collect::<Vec<HMDevice>>();

    for dev in devices {
        println!("Device {:?} is type \"{}\"", dev.path, dev.device_type)
    }

    Ok(())
}
