#![allow(unused)] 
use bitflags::bitflags;
use log::info;
// 定义传输模式枚举
#[derive(Debug, PartialEq)]
pub enum FsDifTransMode {
    DmaTransMode,      // DMA传输模式
    PioTransMode,      // PIO传输模式（通过读/写Fifo）
}

// 定义中断类型枚举
#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum FsDifIntrType {
    GeneralIntr,       // 属于控制器的中断状态
    DmaIntr,           // 属于DMA的中断状态
}

// 定义事件类型枚举
#[derive(Debug, PartialEq)]
pub enum FsDifEvtType {
    CardDetected = 0,  // 卡检测事件
    CmdDone,           // 命令传输完成事件
    DataDone,          // 包含数据的命令传输完成事件
    SdioIrq,           // SDIO卡自定义事件
    ErrOccured,        // 传输中出现错误
    NumOfEvt,          // 事件数量
}

// 定义时钟速度枚举
#[derive(Debug, PartialEq)]
pub enum FsDifClkSpeed {
    ClkSpeedClose =  0,
    ClkSpeed400KHz = 400_000,
    ClkSpeed25Mhz = 25_000_000,
    ClkSpeed26Mhz = 26_000_000, // mmc
    ClkSpeed50Mhz = 50_000_000,
    ClkSpeed52Mhz = 52_000_000, // mmc
    ClkSpeed66Mhz = 66_000_000, // mmc
    ClkSpeed100Mhz = 100_000_000,
}

impl From<u32> for FsDifClkSpeed {
    fn from(value: u32) -> Self {
        match value {
            400_000 => FsDifClkSpeed::ClkSpeed400KHz,
            25_000_000 => FsDifClkSpeed::ClkSpeed25Mhz,
            26_000_000 => FsDifClkSpeed::ClkSpeed26Mhz,
            50_000_000 => FsDifClkSpeed::ClkSpeed50Mhz,
            52_000_000 => FsDifClkSpeed::ClkSpeed52Mhz,
            66_000_000 => FsDifClkSpeed::ClkSpeed66Mhz,
            100_000_000 => FsDifClkSpeed::ClkSpeed100Mhz,
            _ => FsDifClkSpeed::ClkSpeedClose,
        }
    }
}

#[inline(always)]
pub unsafe fn dsb() {
    core::arch::asm!("dsb sy", options(nostack, preserves_flags));
}

pub enum FsDifCommand {
    GoIdleState        = 0,  /*< Go Idle State */
    AllSendCid         = 2,  /*< All Send CID */
    SetDsr             = 4,  /*< Set DSR */
    SelectCard         = 7,  /*< Select Card */
    SendCsd            = 9,  /*< Send CSD */
    SendCid            = 10, /*< Send CID */
    StopTransmission   = 12, /*< Stop Transmission */
    SendStatus         = 13, /*< Send Status */
    GoInactiveState    = 15, /*< Go Inactive State */
    SetBlockLength     = 16, /*< Set Block Length */
    ReadSingleBlock    = 17, /*< Read Single Block */
    ReadMultipleBlock  = 18, /*< Read Multiple Block */
    SetBlockCount      = 23, /*< Set Block Count */
    WriteSingleBlock   = 24, /*< Write Single Block */
    WriteMultipleBlock = 25, /*< Write Multiple Block */
    ProgramCsd         = 27, /*< Program CSD */
    SetWriteProtect    = 28, /*< Set Write Protect */
    ClearWriteProtect  = 29, /*< Clear Write Protect */
    SendWriteProtect   = 30, /*< Send Write Protect */
    Erase              = 38, /*< Erase */
    LockUnlock         = 42, /*< Lock Unlock */
    ApplicationCommand = 55, /*< Send Application Command */
    GeneralCommand     = 56, /*< General Purpose Command */
    ReadOcr            = 58, /*< Read OCR */
}

pub enum FsDifSDIndivCommand {
    SendRelativeAddress    = 3,  /*< Send Relative Address */
    Switch                 = 6,  /*< Switch Function */
    SendInterfaceCondition = 8,  /*< Send Interface Condition */
    VoltageSwitch          = 11, /*< Voltage Switch */
    SpeedClassControl      = 20, /*< Speed Class control */
    EraseWriteBlockStart   = 32, /*< Write Block Start */
    EraseWriteBlockEnd     = 33, /*< Write Block End */
    SendTuningBlock        = 19, /*< Send Tuning Block */
}

pub enum SdApplicationCommand {
    SetBusWidth = 6,               // Set Bus Width
    Status = 13,                   // Send SD status
    SendNumberWriteBlocks = 22,    // Send Number Of Written Blocks
    SetWriteBlockEraseCount = 23,  // Set Write Block Erase Count
    SendOperationCondition = 41,   // Send Operation Condition
    SetClearCardDetect = 42,       // Set Connect/Disconnect pull up on detect pin
    SendScr = 51,                  // Send Scr
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SdmmcR1CardStatusFlag: u32 {
        const OUT_OF_RANGE                  = 1 << 31; // Out of range status bit
        const ADDRESS_ERROR                 = 1 << 30; // Address error status bit
        const BLOCK_LENGTH_ERROR            = 1 << 29; // Block length error status bit
        const ERASE_SEQUENCE_ERROR          = 1 << 28; // Erase sequence error status bit
        const ERASE_PARAMETER_ERROR         = 1 << 27; // Erase parameter error status bit
        const WRITE_PROTECT_VIOLATION       = 1 << 26; // Write protection violation status bit
        const CARD_IS_LOCKED                = 1 << 25; // Card locked status bit
        const LOCK_UNLOCK_FAILED            = 1 << 24; // Lock/unlock error status bit
        const COMMAND_CRC_ERROR             = 1 << 23; // CRC error status bit
        const ILLEGAL_COMMAND               = 1 << 22; // Illegal command status bit
        const CARD_ECC_FAILED               = 1 << 21; // Card ECC error status bit
        const CARD_CONTROLLER_ERROR         = 1 << 20; // Internal card controller error status bit
        const ERROR                         = 1 << 19; // A general or an unknown error status bit
        const CID_CSD_OVERWRITE             = 1 << 16; // CID/CSD overwrite status bit
        const WRITE_PROTECT_ERASE_SKIP      = 1 << 15; // Write protection erase skip status bit
        const CARD_ECC_DISABLED             = 1 << 14; // Card ECC disabled status bit
        const ERASE_RESET                   = 1 << 13; // Erase reset status bit
        const READY_FOR_DATA                = 1 << 8;  // Ready for data status bit
        const SWITCH_ERROR                  = 1 << 7;  // Switch error status bit
        const APPLICATION_COMMAND           = 1 << 5;  // Application command enabled status bit
        const AUTHENTICATION_SEQUENCE_ERROR = 1 << 3;  // Error in the sequence of authentication process
        const SDMMC_R1_ALL_ERROR_FLAG = 0xFFF0008;    // All error status bits
    }
}

impl SdmmcR1CardStatusFlag {
    pub fn check(&self) {
        if self.contains(SdmmcR1CardStatusFlag::OUT_OF_RANGE) {
            info!("OUT_OF_RANGE");
        }
        if self.contains(SdmmcR1CardStatusFlag::ADDRESS_ERROR) {
            info!("ADDRESS_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::BLOCK_LENGTH_ERROR) {
            info!("BLOCK_LENGTH_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::ERASE_SEQUENCE_ERROR) {
            info!("ERASE_SEQUENCE_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::ERASE_PARAMETER_ERROR) {
            info!("ERASE_PARAMETER_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::WRITE_PROTECT_VIOLATION) {
            info!("WRITE_PROTECT_VIOLATION");
        }
        if self.contains(SdmmcR1CardStatusFlag::CARD_IS_LOCKED) {
            info!("CARD_IS_LOCKED");
        }
        if self.contains(SdmmcR1CardStatusFlag::LOCK_UNLOCK_FAILED) {
            info!("LOCK_UNLOCK_FAILED");
        }
        if self.contains(SdmmcR1CardStatusFlag::COMMAND_CRC_ERROR) {
            info!("COMMAND_CRC_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::ILLEGAL_COMMAND) {
            info!("ILLEGAL_COMMAND");
        }
        if self.contains(SdmmcR1CardStatusFlag::CARD_ECC_FAILED) {
            info!("CARD_ECC_FAILED");
        }
        if self.contains(SdmmcR1CardStatusFlag::CARD_CONTROLLER_ERROR) {
            info!("CARD_CONTROLLER_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::ERROR) {
            info!("ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::CID_CSD_OVERWRITE) {
            info!("CID_CSD_OVERWRITE");
        }
        if self.contains(SdmmcR1CardStatusFlag::WRITE_PROTECT_ERASE_SKIP) {
            info!("WRITE_PROTECT_ERASE_SKIP");
        }
        if self.contains(SdmmcR1CardStatusFlag::CARD_ECC_DISABLED) {
            info!("CARD_ECC_DISABLED");
        }
        if self.contains(SdmmcR1CardStatusFlag::ERASE_RESET) {
            info!("ERASE_RESET");
        }
        if self.contains(SdmmcR1CardStatusFlag::READY_FOR_DATA) {
            info!("READY_FOR_DATA");
        }
        if self.contains(SdmmcR1CardStatusFlag::SWITCH_ERROR) {
            info!("SWITCH_ERROR");
        }
        if self.contains(SdmmcR1CardStatusFlag::APPLICATION_COMMAND) {
            info!("APPLICATION_COMMAND");
        }
        if self.contains(SdmmcR1CardStatusFlag::AUTHENTICATION_SEQUENCE_ERROR) {
            info!("AUTHENTICATION_SEQUENCE_ERROR");
        }
    }
}


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SdOcrFlag: u32 {
        const POWER_UP_BUSY = 1 << 31;                  // Power up busy status
        const HOST_CAPACITY_SUPPORT = 1 << 30;          // Card capacity status
        const CARD_CAPACITY_SUPPORT = 1 << 30;          // Card capacity status (same as HostCapacitySupport)
        const SWITCH_18_REQUEST = 1 << 24;              // Switch to 1.8V request
        const SWITCH_18_ACCEPT = 1 << 24;               // Switch to 1.8V accepted (same as Switch18Request)
        const VDD_27_28 = 1 << 15;                      // VDD 2.7-2.8
        const VDD_28_29 = 1 << 16;                      // VDD 2.8-2.9
        const VDD_29_30 = 1 << 17;                      // VDD 2.9-3.0
        const VDD_30_31 = 1 << 18;                      // VDD 2.9-3.0
        const VDD_31_32 = 1 << 19;                      // VDD 3.0-3.1
        const VDD_32_33 = 1 << 20;                      // VDD 3.1-3.2
        const VDD_33_34 = 1 << 21;                      // VDD 3.2-3.3
        const VDD_34_35 = 1 << 22;                      // VDD 3.3-3.4
        const VDD_35_36 = 1 << 23;                      // VDD 3.4-3.5
    }
}

pub enum SdmmcBusWidth {
    BusWidth1Bit = 0, // 1-bit bus width
    BusWidth4Bit = 1, // 4-bit bus width
    BusWidth8Bit = 2, // 8-bit bus width
}

pub enum SdmmcOperationVoltage {
    None = 0,    // Indicate current voltage setting is not set by user
    V330V = 1,   // Card operation voltage around 3.3V
    V300V = 2,   // Card operation voltage around 3.0V
    V180V = 3,   // Card operation voltage around 1.8V
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum EndianMode {
    EndianModeBig = 0, /* Big endian mode */
    EndianModeHalfWordBig = 1, /* Half word big endian mode */
    EndianModeLittle = 2, /* Little endian mode */
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum DataPacketFormat {
    DataPacketFormatLSBFirst, /* usual data packet format LSB first, MSB last */
    DataPacketFormatMSBFirst, /* Wide width data packet format MSB first, LSB last */
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// SD card specification version number
    pub struct SdSpecificationVersion: u8 {
        /// SD card version 1.0-1.01
        const VERSION_1_0 = 1 << 0;
        /// SD card version 1.10
        const VERSION_1_1 = 1 << 1;
        /// SD card version 2.00
        const VERSION_2_0 = 1 << 2;
        /// SD card version 3.0
        const VERSION_3_0 = 1 << 3;
    }
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum SwitchMode {
    Check = 0, /* SD switch mode 0: check function */
    Set = 1, /* SD switch mode 1: set function */
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum GroupNum {
    TimingMode = 0, /* acess mode group */
    CommandSystem = 1, /* command system group */
    DriverStrength = 2, /* driver strength group */
    CurrentLimit = 3, /* current limit group */
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum TimingFunctionNum {
    SDR12Deafult = 0, /* SDR12 mode & default */
    SDR25HighSpeed = 1, /* SDR25 mode & high speed */
    SDR50 = 2, /* SDR50 mode */
    SDR104 = 3, /* SDR104 mode */
    DDR50 = 4, /* DDR50 mode */
}


/** @name Register Map
 *
 * Register offsets from the base address of an SD device.
 * @{
 */
pub const FSDIF_CNTRL_OFFSET: u32 = 0x00; // the controller config reg
pub const FSDIF_PWREN_OFFSET: u32 = 0x04; // the power enable reg
pub const FSDIF_CLKDIV_OFFSET: u32 = 0x08; // the clock divider reg
pub const FSDIF_CLKENA_OFFSET: u32 = 0x10; // the clock enable reg
pub const FSDIF_TMOUT_OFFSET: u32 = 0x14; // the timeout reg
pub const FSDIF_CTYPE_OFFSET: u32 = 0x18; // the card type reg
pub const FSDIF_BLK_SIZ_OFFSET: u32 = 0x1C; // the block size reg
pub const FSDIF_BYT_CNT_OFFSET: u32 = 0x20; // the byte count reg
pub const FSDIF_INT_MASK_OFFSET: u32 = 0x24; // the interrupt mask reg
pub const FSDIF_CMD_ARG_OFFSET: u32 = 0x28; // the command argument reg
pub const FSDIF_CMD_OFFSET: u32 = 0x2C; // the command reg
pub const FSDIF_RESP0_OFFSET: u32 = 0x30; // the response reg0
pub const FSDIF_RESP1_OFFSET: u32 = 0x34; // the response reg1
pub const FSDIF_RESP2_OFFSET: u32 = 0x38; // the response reg2
pub const FSDIF_RESP3_OFFSET: u32 = 0x3C; // the response reg3
pub const FSDIF_MASKED_INTS_OFFSET: u32 = 0x40; // the masked interrupt status reg
pub const FSDIF_RAW_INTS_OFFSET: u32 = 0x44; // the raw interrupt status reg
pub const FSDIF_STATUS_OFFSET: u32 = 0x48; // the status reg
pub const FSDIF_FIFOTH_OFFSET: u32 = 0x4C; // the FIFO threshold watermark reg
pub const FSDIF_CARD_DETECT_OFFSET: u32 = 0x50; // the card detect reg
pub const FSDIF_CARD_WRTPRT_OFFSET: u32 = 0x54; // the card write protect reg
pub const FSDIF_CKSTS_OFFSET: u32 = 0x58; // the ciu ready
pub const FSDIF_TRAN_CARD_CNT_OFFSET: u32 = 0x5C; // the transferred CIU card byte count reg
pub const FSDIF_TRAN_FIFO_CNT_OFFSET: u32 = 0x60; // the transferred host to FIFO byte count reg
pub const FSDIF_DEBNCE_OFFSET: u32 = 0x64; // the debounce count reg
pub const FSDIF_UID_OFFSET: u32 = 0x68; // the user ID reg
pub const FSDIF_VID_OFFSET: u32 = 0x6C; // the controller version ID reg
pub const FSDIF_HWCONF_OFFSET: u32 = 0x70; // the hardware configuration reg
pub const FSDIF_UHS_REG_OFFSET: u32 = 0x74; // the UHS-I reg
pub const FSDIF_CARD_RESET_OFFSET: u32 = 0x78; // the card reset reg
pub const FSDIF_BUS_MODE_OFFSET: u32 = 0x80; // the bus mode reg
pub const FSDIF_DESC_LIST_ADDRL_OFFSET: u32 = 0x88; // the descriptor list low base address reg
pub const FSDIF_DESC_LIST_ADDRH_OFFSET: u32 = 0x8C; // the descriptor list high base address reg
pub const FSDIF_DMAC_STATUS_OFFSET: u32 = 0x90; // the internal DMAC status reg
pub const FSDIF_DMAC_INT_EN_OFFSET: u32 = 0x94; // the internal DMAC interrupt enable reg
pub const FSDIF_CUR_DESC_ADDRL_OFFSET: u32 = 0x98; // the current host descriptor low address reg
pub const FSDIF_CUR_DESC_ADDRH_OFFSET: u32 = 0x9C; // the current host descriptor high address reg
pub const FSDIF_CUR_BUF_ADDRL_OFFSET: u32 = 0xA0; // the current buffer low address reg
pub const FSDIF_CUR_BUF_ADDRH_OFFSET: u32 = 0xA4; // the current buffer high address reg
pub const FSDIF_CARD_THRCTL_OFFSET: u32 = 0x100; // the card threshold control reg
pub const FSDIF_CLK_SRC_OFFSET: u32 = 0x108; // the UHS register extension
pub const FSDIF_EMMC_DDR_REG_OFFSET: u32 = 0x10C; // the EMMC DDR reg
pub const FSDIF_ENABLE_SHIFT_OFFSET: u32 = 0x110; // the enable phase shift reg
pub const FSDIF_DATA_OFFSET: u32 = 0x200; // the data FIFO access

pub const FSDIF_TIMEOUT:u32 = 50000; /* timeout for retries */
pub const FSDIF_DELAY_US:u32 = 5;
pub const FSDIF_MAX_FIFO_CNT:u32 = 0x800;

pub const FSL_SDMMC_MAX_CMD_RETRIES:u32 = 10;

pub const FSDIF0_ID: u32 = 0;
pub const FSDIF1_ID: u32 = 1;