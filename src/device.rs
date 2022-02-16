use hdd::{ata::ATADevice, scsi::SCSIDevice, Device as HddDevice};
use std::io::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum DeviceType {
    ATA(ATADevice<SCSIDevice>),
    SCSI(SCSIDevice),
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceType::ATA(_) => write!(f, "ATA"),
            DeviceType::SCSI(_) => write!(f, "SCSI"),
            DeviceType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct Device {
    pub path: PathBuf,
    pub device_type: DeviceType,
}

impl Device {
    pub fn new(path: &Path) -> Result<Device> {
        let dev = HddDevice::open(path)?;

        let new_dev = Device {
            path: path.to_path_buf(),
            device_type: match dev.get_type() {
                Err(_) => DeviceType::Unknown,
                Ok(hdd::Type::SCSI) => {
                    // Stolen from crate hdd source code
                    // check whether devices replies to ATA PASS-THROUGH
                    let satdev = ATADevice::new(SCSIDevice::new(dev));
                    match hdd::ata::misc::Misc::get_device_id(&satdev) {
                        // this is really an ATA device
                        Ok(_) => DeviceType::ATA(satdev),
                        // nnnnope, plain SCSI
                        Err(hdd::ata::misc::Error::SCSI(hdd::scsi::ATAError::NotSupported)) => {
                            DeviceType::SCSI(satdev.unwrap())
                        }
                        _ => DeviceType::SCSI(satdev.unwrap()),
                    }
                }
            },
        };
        return Ok(new_dev);
    }
}
