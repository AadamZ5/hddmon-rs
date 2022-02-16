use std::path::{Path, PathBuf};

use hdd::{ata::ATADevice, device::Type, scsi::SCSIDevice};

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
