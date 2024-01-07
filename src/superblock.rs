use anyhow::{anyhow, Result};
use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite, BinWriterExt};
use std::{fs::File, os::unix::fs::FileExt};

/// Btrfs Superblock
///
/// Code is derived from https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L461
#[derive(BinRead, BinWrite, Clone, Copy, Debug)]
#[brw(little)]
pub struct Superblock {
    pub csum: [u8; 32],
    pub fs_uuid: [u8; BTRFS_FSID_SIZE],
    pub bytenr: u64,
    pub flags: u64,
    pub magic: [u8; 8],
    pub generation: u64,
    pub root: u64,
    pub chunk_root: u64,
    pub log_root: u64,
    pub log_root_transid: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub root_dir_objectid: u64,
    pub num_devices: u64,
    pub sectorsize: u32,
    pub nodesize: u32,
    unused_leafsize: u32,
    pub stripesize: u32,
    pub sys_chunk_array_size: u32,
    pub chunk_root_generation: u64,
    pub compat_flags: u64,
    pub compat_ro_flags: u64,
    pub incompat_flags: u64,
    pub csum_type: u16,
    pub root_level: u8,
    pub chunk_root_level: u8,
    pub log_root_level: u8,
    pub dev_item: DevItem,
    pub label: [u8; BTRFS_LABEL_SIZE],
    pub cache_generation: u64,
    pub uuid_tree_generation: u64,
    pub metadata_uuid: [u8; BTRFS_FSID_SIZE],
    pub nr_global_roots: u64,
    pub block_group_root: u64,
    pub block_group_root_generation: u64,
    pub block_group_root_level: u8,
    reserved8: [u8; 7],
    reserved: [u64; 24],
    pub sys_chunk_array: [u8; BTRFS_SYSTEM_CHUNK_ARRAY_SIZE],
    pub super_roots: [RootBackup; BTRFS_NUM_BACKUP_ROOTS],
    padding: [u8; 565],
}

impl Superblock {
    #[allow(dead_code)]
    /// Read from Block and return Superblock
    pub fn read_from_block(f: &File) -> Result<Superblock> {
        let mut buf = [0u8; BTRFS_SUPER_INFO_SIZE];
        f.read_at(&mut buf, BTRFS_SUPER_POS as u64)?;
        let mut reader = Cursor::new(buf.to_vec());
        let sb: Superblock = reader.read_le()?;
        Ok(sb)
    }

    #[allow(dead_code)]
    pub fn to_bytes(self) -> Result<[u8; BTRFS_SUPER_INFO_SIZE]> {
        let mut writer = Cursor::new(Vec::new());
        writer.write_le(&self)?;

        let v = writer.into_inner().to_vec();

        if v.len().ne(&BTRFS_SUPER_INFO_SIZE) {
            return Err(anyhow!("size does not match: {:?}", v.len()));
        }

        Ok(v.try_into().unwrap())
    }

    #[allow(dead_code)]
    /// Get CSUM Type
    pub fn get_csum_type(self) -> CSUMType {
        match self.csum_type {
            1 => CSUMType::Xxhash,
            2 => CSUMType::Sha256,
            3 => CSUMType::Blake2,
            _ => CSUMType::Crc32,
        }
    }

    #[allow(dead_code)]
    pub fn get_crc32(self) -> Result<u32> {
        if self.get_csum_type().ne(&CSUMType::Crc32) {
            return Err(anyhow!("csum_type is not crc32"));
        }
        let crc_value: [u8; 4] = self.csum[0..4].try_into()?;
        let v: u32 = u32::from_le_bytes(crc_value);
        Ok(v)
    }
}

const BTRFS_SUPER_POS: usize = 0x10000;
#[allow(dead_code)]
const BTRFS_SUPER_INFO_SIZE: usize = 4096;
const BTRFS_LABEL_SIZE: usize = 256;
const BTRFS_UUID_SIZE: usize = 16;
const BTRFS_FSID_SIZE: usize = 16;
const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;
const BTRFS_NUM_BACKUP_ROOTS: usize = 4;

#[derive(PartialEq, Eq, Debug)]
#[allow(dead_code)]
/// CSUM Types
///
/// Code is Derived from https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L165
pub enum CSUMType {
    Crc32 = 0,
    Xxhash = 1,
    Sha256 = 2,
    Blake2 = 3,
}

#[derive(BinRead, BinWrite, Clone, Copy, Debug)]
#[brw(little)]
/// Dev Item
///
/// Code is Derived from https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L258
pub struct DevItem {
    pub devid: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub io_align: u32,
    pub io_width: u32,
    pub sector_size: u32,
    /// Type
    ///
    /// https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L278
    pub item_type: u64,
    pub generation: u64,
    pub start_offset: u64,
    pub dev_group: u32,
    pub seek_speed: u8,
    pub bandwidth: u8,
    pub uuid: [u8; BTRFS_UUID_SIZE],
    pub fsid: [u8; BTRFS_FSID_SIZE],
}

#[derive(BinRead, BinWrite, Clone, Copy, Debug)]
#[brw(little)]
/// Root Backup
///
/// Code is Derived from https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L419
pub struct RootBackup {
    pub tree_root: u64,
    pub tree_root_gen: u64,

    pub chunk_root: u64,
    pub chunk_root_gen: u64,

    pub extent_root: u64,
    pub extent_root_gen: u64,

    pub fs_root: u64,
    pub fs_root_gen: u64,

    pub dev_root: u64,
    pub dev_root_gen: u64,

    pub csum_root: u64,
    pub csum_root_gen: u64,

    pub total_bytes: u64,
    pub bytes_used: u64,
    pub num_devices: u64,
    /// Use for Future Expansion
    unused_64: [u64; 4],

    pub tree_root_level: u8,
    pub chunk_root_level: u8,
    pub extent_root_level: u8,
    pub fs_root_level: u8,
    pub dev_root_level: u8,
    pub csum_root_level: u8,
    /// Use for Future and to align
    unused_8: [u8; 10],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test to Read Superblock
    fn read_superblock() {
        let f = File::open("./btrfs.img").unwrap();
        let sb = Superblock::read_from_block(&f).unwrap();
        let csum_type = sb.get_csum_type();
        let crc32_value = sb.get_crc32().unwrap();
        let b = sb.to_bytes().unwrap();
        assert_eq!(csum_type, CSUMType::Crc32);
        assert_ne!(crc32_value, 0);
        assert_eq!(b.len(), BTRFS_SUPER_INFO_SIZE);
    }

    #[test]
    /// Check CRC32 is Matched
    fn check_crc32_is_equal() {
        let f = File::open("./btrfs.img").unwrap();
        let sb = Superblock::read_from_block(&f).unwrap();
        let crc32_value = sb.get_crc32().unwrap();
        let b = sb.to_bytes().unwrap();

        let crc = crc32c::crc32c(&b[32..]);

        assert_eq!(crc32_value, crc)
    }
}
