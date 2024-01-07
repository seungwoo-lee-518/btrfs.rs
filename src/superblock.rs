/// Btrfs Superblock
///
/// Code is derived from https://github.com/kdave/btrfs-progs/blob/master/libbtrfs/ctree.h#L461
pub struct Superblock {
    pub csum: [u8; 32],
    pub fs_uuid: [u8; 16],
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
    pub super_roots: [RootBackup; BTRFS_NUM_BACKUP_ROOTS]
}

const BTRFS_SUPER_INFO_SIZE: usize = 4096;
const BTRFS_LABEL_SIZE: usize = 256;
const BTRFS_FSID_SIZE: usize = 16;
const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;
const BTRFS_NUM_BACKUP_ROOTS: usize = 4;

pub struct DevItem {
    pub devid: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub io_align: u32,
    pub io_width: u32,
    pub sector_size: u32,
    /// Type
    pub item_type: u64,
    pub generation: u64,
    pub start_offset: u64,
    pub dev_group: u32,
    pub seek_speed: u8,
    pub bandwidth: u8,
    pub uuid: [u8; 16],
    pub fsid: [u8; 16]
}

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
    unused_8: [u8; 10]
}
