use crate::mci::{mci_timing::*, FsDifClkSpeed, FsDifTransMode, EndianMode};

type GetTuning = fn(clock_freq: FsDifClkSpeed, non_removable: bool) -> MCITiming;
pub struct MCIConfig {
    pub instance_id: u32,           /* Device instance id */
    pub irq_num: u32,               /* Device IRQ number */
    pub trans_mode: FsDifTransMode, /* Trans mode, PIO/DMA */
    pub non_removable: bool,        /* Non-removable media, e.g. eMMC */
    pub get_tuning: GetTuning,
    pub endian_mode: EndianMode,
}


impl MCIConfig {
    pub fn new() -> Self {
        MCIConfig {
            instance_id: 0,
            irq_num: 104,
            trans_mode: FsDifTransMode::DmaTransMode,
            non_removable: false,
            get_tuning: mci_get_timing_setting,
            endian_mode: EndianMode::EndianModeLittle,
        }
    }
}

