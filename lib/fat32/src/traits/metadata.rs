/// Trait for a timestamp (year, month, day, hour, minute, second).
pub trait Timestamp: Copy + Clone + Sized {
    /// The calendar year.
    ///
    /// The year is not offset. 2009 is 2009.
    fn year(&self) -> usize;

    /// The calendar month, starting at 1 for January. Always in range [1, 12].
    ///
    /// January is 1, Feburary is 2, ..., December is 12.
    fn month(&self) -> u8;

    /// The calendar day, starting at 1. Always in range [1, 31].
    fn day(&self) -> u8;

    /// The 24-hour hour. Always in range [0, 24).
    fn hour(&self) -> u8;

    /// The minute. Always in range [0, 60).
    fn minute(&self) -> u8;

    /// The second. Always in range [0, 60).
    fn second(&self) -> u8;
}

/// Trait for directory entry metadata.
pub trait Metadata: Sized {
    /// Type corresponding to a point in time.
    type Timestamp: Timestamp;

    /// Whether the associated entry is read only.
    fn read_only(&self) -> bool;

    /// Whether the entry should be "hidden" from directory traversals.
    fn hidden(&self) -> bool;

    /// Whether the entry is a system file entry.
    fn system(&self) -> bool;

    /// Whether the entry is a volume ID entry.
    fn volume_id(&self) -> bool;

    /// Whether the entry is another directory.
    fn directory(&self) -> bool;

    /// Whether the entry is an archive.
    fn archive(&self) -> bool;

    /// Whether the entry is a 'long file name' (LFN) entry.
    fn lfn(&self) -> bool;

    /// The timestamp when the entry was created.
    fn created(&self) -> Self::Timestamp;

    /// The timestamp for the entry's last access.
    fn accessed(&self) -> Self::Timestamp;

    /// The timestamp for the entry's last modification.
    fn modified(&self) -> Self::Timestamp;

    /// The file's first cluster
    fn cluster(&self) -> u32;

    /// The file's size in bytes
    fn file_size(&self) -> u32;
}
