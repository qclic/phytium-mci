use bitflags::bitflags;

#[derive(Debug, Default)]
pub struct MCICsd {
    pub csd_structure: u8,
    pub data_read_access_time1: u8,
    pub data_read_access_time2: u8,
    pub transfer_speed: u8,
    pub card_command_classes: u16,
    pub read_block_length: u8,
    pub flags: u16,
    pub device_size: u32,
    pub read_current_vdd_min: u8,
    pub read_current_vdd_max: u8,
    pub write_current_vdd_min: u8,
    pub write_current_vdd_max: u8,
    pub device_size_multiplier: u8,
    pub erase_sector_size: u8,
    pub write_protect_group_size: u8,
    pub write_speed_factor: u8,
    pub write_block_length: u8,
    pub file_format: u8,
}


bitflags! {
    pub struct CsdFlags: u16 {
        const READ_BLOCK_PARTIAL = 1 << 0; /* Partial blocks for read allowed [79:79] */
        const WRITE_BLOCK_MISALIGN = 1 << 1; /* Write block misalignment [78:78] */
        const READ_BLOCK_MISALIGN = 1 << 2; /* Read block misalignment [77:77] */
        const DSR_IMPLEMENTED = 1 << 3; /* DSR implemented [76:76] */
        const ERASE_BLOCK_ENABLED = 1 << 4; /* Erase single block enabled [46:46] */
        const WRITE_PROTECT_GROUP_ENABLED = 1 << 5; /* Write protect group enabled [31:31] */
        const WRITE_BLOCK_PARTIAL = 1 << 6; /* Partial blocks for write allowed [21:21] */
        const FILE_FORMAT_GROUP = 1 << 7; /* File format group [15:15] */
        const COPY = 1 << 8; /* Copy flag [14:14] */
        const PERMANENT_WRITE_PROTECT = 1 << 9; /* Permanent write protection [13:13] */
        const TEMPORARY_WRITE_PROTECT = 1 << 10; /* Temporary write protection [12:12] */
    }
}