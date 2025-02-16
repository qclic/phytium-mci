#[derive(Debug, Default)]
/// SD card status information
pub struct MCIStatus {
    /// Current bus width
    pub bus_width: u8,
    
    /// Secured mode status
    pub secure_mode: u8,
    
    /// SD card type
    pub card_type: u16,
    
    /// Size of protected area
    pub protected_size: u32,
    
    /// Speed class of card
    pub speed_class: u8,
    
    /// Performance of move indicated by 1[MB/S] step
    pub performance_move: u8,
    
    /// Size of Allocation Unit (AU)
    pub au_size: u8,
    
    /// Number of AUs to be erased at a time
    pub erase_size: u16,
    
    /// Timeout value for erasing areas specified by UNIT OF ERASE AU
    pub erase_timeout: u8,
    
    /// Fixed offset value added to erase time
    pub erase_offset: u8,
    
    /// Speed grade for UHS (Ultra High Speed) mode
    pub uhs_speed_grade: u8,
    
    /// Size of Allocation Unit (AU) for UHS mode
    pub uhs_au_size: u8,
}