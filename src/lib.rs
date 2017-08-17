extern crate byteorder;
#[macro_use]
extern crate quick_error;

mod header;

pub use header::{MagicHeader, MagicHeaderParseError, LocateSectionError};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Section {
    Header,
    Kernel,
    Ramdisk,
    Second,
    DeviceTree,
}

impl fmt::Display for Section {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", match *self {
            Section::Header => "header",
            Section::Kernel => "kernel",
            Section::Ramdisk => "ramdisk",
            Section::Second => "second",
            Section::DeviceTree => "device_tree",
        })
    }
}