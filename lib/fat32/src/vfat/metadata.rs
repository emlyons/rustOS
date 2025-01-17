use core::fmt;
use shim::const_assert_size;

use alloc::string::String;

use crate::traits;

/// A date as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date(u16);

/// Time as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time(u16);

/// File attributes as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Attributes(u8);

#[repr(u8)]
enum attr {
    READ_ONLY = 0x01,
    HIDDEN = 0x02,
    SYSTEM = 0x04,
    VOLUME_ID = 0x08,
    DIRECTORY = 0x10,
    ARCHIVE = 0x20,
    LFN = 0x0f,
}

impl Attributes {
    
    /// Whether the associated entry is read only.
    pub fn read_only(&self) -> bool {
	(self.0 & attr::READ_ONLY as u8) == attr::READ_ONLY as u8
    }

    /// Whether the entry should be "hidden" from directory traversals.
    pub fn hidden(&self) -> bool {
	(self.0 & attr::HIDDEN as u8) == attr::HIDDEN as u8
    }

    /// Whether the entry is a system file entry.
    pub fn system(&self) -> bool {
	(self.0 & attr::SYSTEM as u8) == attr::SYSTEM as u8
    }

    /// Whether the entry is a volume ID entry.
    pub fn volume_id(&self) -> bool {
	(self.0 & attr::VOLUME_ID as u8) == attr::VOLUME_ID as u8
    }

    /// Whether the entry is another directory.
    pub fn directory(&self) -> bool {
	(self.0 & attr::DIRECTORY as u8) == attr::DIRECTORY as u8
    }

    /// Whether the entry is an archive.
    pub fn archive(&self) -> bool {
	(self.0 & attr::ARCHIVE as u8) == attr::ARCHIVE as u8
    }

    /// Whether the entry is a 'long file name' (LFN) entry.
    pub fn lfn(&self) -> bool {
	self.0 == attr::LFN as u8
    }
}

/// A structure containing a date and time.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub date: Date,
    pub time: Time,
}

/// Metadata for a directory entry.
#[derive(Default, Copy, Clone)]
#[repr(C, packed)]
pub struct Metadata {
    pub(super) attributes: Attributes,
    reserved: u8,
    create_time_tenths: u8,
    create_time: Time,
    create_date: Date,
    access_date: Date,
    cluster_high: u16,
    modified_time: Time,
    modified_date: Date,
    cluster_low: u16,
    file_size: u32
}

const_assert_size!(Metadata, 21);

fn truncate_bits(val: u16, least_sigbit: u16, num_bits: u16) -> u16 {
    assert!(num_bits > 0);
    assert!(least_sigbit + num_bits <= 16);
    let mask: u16 = 0xFFFF >> 16 - num_bits;
    let shift_down: u16 = least_sigbit;
    let masked_val = (val >> least_sigbit) & mask;
    masked_val
}

// FIXME: Implement `traits::Timestamp` for `Timestamp`.
impl traits::Timestamp for Timestamp {

    /// The calendar year.
    /// 7-bits
    /// year is offset from 0 == 1980.
    fn year(&self) -> usize {
	truncate_bits(self.date.0, 9, 7) as usize + 1980
    }

    /// The calendar month, starting at 1 for January. Always in range [1, 12].
    /// 4-bits
    /// January is 1, Feburary is 2, ..., December is 12.
    fn month(&self) -> u8 {
	truncate_bits(self.date.0, 5, 4) as u8
    }

    /// 5-bits
    /// The calendar day, starting at 1. Always in range [1, 31].
    fn day(&self) -> u8 {
	truncate_bits(self.date.0, 0, 5) as u8
    }

    /// 5-bits
    /// The 24-hour hour. Always in range [0, 24).
    fn hour(&self) -> u8 {
	truncate_bits(self.time.0, 11, 5) as u8
    }

    /// 6-bits
    /// The minute. Always in range [0, 60).
    fn minute(&self) -> u8 {
	truncate_bits(self.time.0, 5, 6) as u8
    }

    /// 5-bits
    /// The second. Always in range [0, 60). Seconds are stored as Seconds/2 to compensate for not enough bits.
    fn second(&self) -> u8 {
	(truncate_bits(self.time.0, 0, 5) * 2) as u8
    }
}

impl traits::Metadata for Metadata {
    
    /// Type corresponding to a point in time.
    type Timestamp = Timestamp;

    
    /// Whether the associated entry is read only.
    fn read_only(&self) -> bool {
	self.attributes.read_only()
    }

    /// Whether the entry should be "hidden" from directory traversals.
    fn hidden(&self) -> bool {
	self.attributes.hidden()
    }

    /// Whether the entry is a system file entry.
    fn system(&self) -> bool {
	self.attributes.system()
    }

    /// Whether the entry is a volume ID entry.
    fn volume_id(&self) -> bool {
	self.attributes.volume_id()
    }

    /// Whether the entry is another directory.
    fn directory(&self) -> bool {
	self.attributes.directory()
    }

    /// Whether the entry is an archive.
    fn archive(&self) -> bool {
	self.attributes.archive()
    }

    /// Whether the entry is a 'long file name' (LFN) entry.
    fn lfn(&self) -> bool {
	self.attributes.lfn()
    }

    /// The timestamp when the entry was created.
    fn created(&self) -> Self::Timestamp {
	Timestamp {
	    date: self.create_date,
	    time: self.create_time,
	}
    }

    /// The timestamp for the entry's last access.
    fn accessed(&self) -> Self::Timestamp {
	Timestamp {
	    date: self.access_date,
	    time: Time(0),
	}
    }

    /// The timestamp for the entry's last modification.
    fn modified(&self) -> Self::Timestamp {
	Timestamp {
	    date: self.modified_date,
	    time: self.modified_time,
	}
    }

    /// The file's first cluster
    fn cluster(&self) -> u32 {
	((self.cluster_high as u32) << 16) + (self.cluster_low as u32)
    }

    /// The file's size in bytes
    fn file_size(&self) -> u32 {
	self.file_size
    }
}

impl Metadata {
    pub fn root () -> Metadata {
	Metadata {
	    attributes: Attributes(attr::DIRECTORY as u8),
	    reserved: 0,
	    create_time_tenths: 0,
	    create_time: Time(0),
	    create_date: Date(0),
	    access_date: Date(0),
	    cluster_high: 0,
	    modified_time: Time(0),
	    modified_date: Date(0),
	    cluster_low: 0,
	    file_size: 0,
	}
    }
}

// Implement `fmt::Display` (to your liking) for `Metadata`.
impl fmt::Debug for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Metadata")
            .field("attributes", &self.attributes.0)
            .field("create time (tenths of seconds)", &self.create_time_tenths)
	    .field("create time", &self.create_time.0)
	    .field("create date", &self.create_date.0)
	    .field("access date", &self.access_date.0)
	    .field("cluster address high 16-bits", &self.cluster_high)
	    .field("modified time", &self.modified_time.0)
	    .field("modified date", &self.modified_date.0)
	    .field("cluster address low 16-bits", &self.cluster_low)
	    .field("file size (bytes):", &self.file_size)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_truncator() {
	let val_1: u16 = 0b1111111111111111;

	assert_eq!(truncate_bits(val_1, 0, 16), 0b1111111111111111);
	assert_eq!(truncate_bits(val_1, 0, 15), 0b111111111111111);
	assert_eq!(truncate_bits(val_1, 0, 14), 0b11111111111111);
	assert_eq!(truncate_bits(val_1, 0, 13), 0b1111111111111);
	assert_eq!(truncate_bits(val_1, 0, 12), 0b111111111111);
	assert_eq!(truncate_bits(val_1, 0, 11), 0b11111111111);
	assert_eq!(truncate_bits(val_1, 0, 10), 0b1111111111);
	assert_eq!(truncate_bits(val_1, 0, 9), 0b111111111);
	assert_eq!(truncate_bits(val_1, 0, 8), 0b11111111);
	assert_eq!(truncate_bits(val_1, 0, 7), 0b1111111);
	assert_eq!(truncate_bits(val_1, 0, 6), 0b111111);
	assert_eq!(truncate_bits(val_1, 0, 5), 0b11111);
	assert_eq!(truncate_bits(val_1, 0, 4), 0b1111);
	assert_eq!(truncate_bits(val_1, 0, 3), 0b111);
	assert_eq!(truncate_bits(val_1, 0, 2), 0b11);
	assert_eq!(truncate_bits(val_1, 0, 1), 0b1);

	assert_eq!(truncate_bits(val_1, 1, 15), 0b111111111111111);
	assert_eq!(truncate_bits(val_1, 2, 14), 0b11111111111111);
	assert_eq!(truncate_bits(val_1, 4, 12), 0b111111111111);
	assert_eq!(truncate_bits(val_1, 8, 8), 0b11111111);

	assert_eq!(truncate_bits(0b1000101010101110, 11, 5), 0b10001);
	assert_eq!(truncate_bits(0b1000101010101110, 4, 6), 0b101010);;
	
	
    }
}
