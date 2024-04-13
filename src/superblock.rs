use binrw::{BinRead, BinWrite};

const BTRFS_CSUM_SIZE: usize = 32;
const BTRFS_FSID_SIZE: usize = 16;
const BTRFS_LABEL_SIZE: usize = 256;
const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;
const BTRFS_NUM_BACKUP_ROOTS: usize = 4;

#[allow(dead_code)]
const BTRFS_SUPER_INFO_OFFSET: usize = 65536;
#[allow(dead_code)]
const BTRFS_SUPER_INFO_SIZE: usize = 4096;

#[derive(BinRead, BinWrite)]
/// Btrfs Superblock
struct Superblock {
    csum: [u8; BTRFS_CSUM_SIZE],
    fsid: [u8; BTRFS_FSID_SIZE],
    bytenr: u64,
    flags: u64,

    magic: u64,
    generation: u64,
    root: u64,
    chunk_root: u64,
    log_root: u64,

    log_root_transid: u64,
    total_bytes: u64,
    bytes_used: u64,
    root_dir_objectid: u64,
    num_devices: u64,

    sectorsize: u32,
    nodesize: u32,
    unused_leafsize: u32,
    stripesize: u32,
    sys_chunk_array_size: u32,
    chunk_root_generation: u64,
    compat_flags: u64,
    compat_ro_flags: u64,
    incompat_flags: u64,
    csum_type: u16,
    root_level: u8,
    chunk_root_level: u8,
    log_root_level: u8,
    btrfs_dev_item: DevItem,

    label: [u8; BTRFS_LABEL_SIZE],

    cache_generation: u64,
    uuid_tree_generation: u64,

    metadata_uuid: [u8; BTRFS_FSID_SIZE],

    nr_global_roots: u64,

    block_group_root: u64,
    block_group_root_generation: u64,
    block_group_root_level: u8,

    reserved: [u8; 7],
    reserved64: [u64; 24],
    sys_chunk_array: [u8; BTRFS_SYSTEM_CHUNK_ARRAY_SIZE],
    super_roots: [RootBackup; BTRFS_NUM_BACKUP_ROOTS],

    padding: [u8; 565],
}

#[derive(BinRead, BinWrite)]
struct DevItem {
    /// the internal btrfs device id
    devid: u64,

    /// size of the device
    total_bytes: u64,

    /// bytes used
    bytes_used: u64,

    /// optimal io alignment for this device
    io_align: u32,

    /// optimal io width for this device
    io_width: u32,

    /// minimal io size for this device
    sector_size: u32,

    /// type and info about this device
    dev_type: u64,

    /// expected generation for this device
    generation: u64,

    /// starting byte of this partition on the device,
    /// to allow for stripe alignment in the future
    start_offset: u64,

    /// grouping information for allocation decisions
    dev_group: u32,

    /// seek speed 0-100 where 100 is fastest
    seek_speed: u8,

    /// bandwidth 0-100 where 100 is fastest
    bandwidth: u8,

    /// btrfs generated uuid for this device
    dev_uuid: [u8; BTRFS_FSID_SIZE],

    /// uuid of FS who owns this device
    dev_fsid: [u8; BTRFS_FSID_SIZE],
}

#[derive(BinRead, BinWrite)]
struct RootBackup {
    tree_root: u64,
    tree_root_gen: u64,

    chunk_root: u64,
    chunk_root_gen: u64,

    extent_root: u64,
    extent_root_gen: u64,

    fs_root: u64,
    fs_root_gen: u64,

    dev_root: u64,
    dev_root_gen: u64,

    csum_root: u64,
    csum_root_gen: u64,

    total_bytes: u64,
    bytes_used: u64,
    num_devices: u64,
    /// future expansion
    unused_64: [u64; 4],

    tree_root_level: u8,
    chunk_root_level: u8,
    extent_root_level: u8,
    fs_root_level: u8,
    dev_root_level: u8,
    csum_root_level: u8,
    /// future and to align
    unused_8: [u8; 10],
}
