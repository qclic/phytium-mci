use bare_test::mem::mmu::iomap;
use log::debug;

use crate::iopad::constants::{FioPadDelay, FioPadDelayDir, FioPadDelayType, PAD_ADDRESS};
use crate::iopad::regs::J53Reg1;
use crate::{iopad::regs::Aj49Reg1, mci::FsDifClkSpeed};
use crate::iopad::{self, IoPad};

use super::constants::{FSDIF0_ID, FSDIF1_ID};

type PadDelay = fn(iopad:&mut IoPad,sdif_id:u32);

#[derive(PartialEq)]
pub struct MCITiming {
    pub use_hold: bool,
    pub clk_div: u32,
    pub clk_src: u32,
    pub shift: u32,
    pub pad_delay: PadDelay //* 用于调整IO的延时 */
}

impl MCITiming {
    pub fn new() -> Self {
        MCITiming {
            use_hold: false,
            clk_div: 0,
            clk_src: 0,
            shift: 0,
            pad_delay: fsdif_sdifdelay_null
        }
    }
}

pub const MMC_SD_NULL: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0,
    clk_src: 0,
    shift: 0,
    pad_delay: fsdif_sdifdelay_null
};

pub const MMC_SD_400K_HZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x7e7dfa,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: fsdif_unset_sdifdelay
};

pub const SD_25MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: fsdif_unset_sdifdelay
};

pub const SD_50MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000502,
    shift: 0x0,
    pad_delay: fsdif_set_sdifdelay
};

pub const SD_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: fsdif_set_sdifdelay
};

pub const MMC_26MHZ: MCITiming = MCITiming {
    use_hold: true,
    clk_div: 0x030204,
    clk_src: 0x000302,
    shift: 0x0,
    pad_delay: fsdif_set_sdifdelay
};

pub const MMC_52MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x030204,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: fsdif_set_sdifdelay
};

pub const MMC_66MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: fsdif_sdifdelay_null
};

pub const MMC_100MHZ: MCITiming = MCITiming {
    use_hold: false,
    clk_div: 0x010002,
    clk_src: 0x000202,
    shift: 0x0,
    pad_delay: fsdif_set_sdifdelay
};

pub fn default_tuning(_clock_freq: FsDifClkSpeed, _non_removable: bool) ->  MCITiming {
    MMC_SD_NULL
}

pub fn mci_get_timing_setting(clock_freq: FsDifClkSpeed, non_removable: bool) ->  MCITiming {
    if clock_freq == FsDifClkSpeed::ClkSpeed400KHz {
        return MMC_SD_400K_HZ;
    }
    if non_removable {
        match clock_freq {
            FsDifClkSpeed::ClkSpeed26Mhz => return MMC_26MHZ,
            FsDifClkSpeed::ClkSpeed52Mhz => return MMC_52MHZ,
            FsDifClkSpeed::ClkSpeed66Mhz => return MMC_66MHZ,
            FsDifClkSpeed::ClkSpeed100Mhz => return MMC_100MHZ,
            _ => return MMC_SD_NULL,
        }
    }else {
        match clock_freq {
            FsDifClkSpeed::ClkSpeed25Mhz => return SD_25MHZ,
            FsDifClkSpeed::ClkSpeed50Mhz => return SD_50MHZ,
            FsDifClkSpeed::ClkSpeed100Mhz => return SD_100MHZ,
            _ => return MMC_SD_NULL,
        }
    }
}


pub fn fsdif_set_sdifdelay(iopad:&mut IoPad,sdif_id:u32){
    type Fsdif0SdCclkOutDelay = Aj49Reg1;
    type Fsdif1SdCclkOutDelay = J53Reg1;
    if sdif_id == FSDIF0_ID {
        iopad.delay_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayCoarseTuning,
            FioPadDelay::Delay1);
        iopad.delay_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayFineTuning,
            FioPadDelay::Delay7);
        iopad.delay_enable_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay, 
            true);
    } else if sdif_id == FSDIF1_ID {
        iopad.delay_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayCoarseTuning,
            FioPadDelay::Delay1);
        iopad.delay_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayFineTuning,
            FioPadDelay::Delay7);
        iopad.delay_enable_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay, 
            true);
    }
}

pub fn fsdif_unset_sdifdelay(iopad:&mut IoPad,sdif_id:u32){
    type Fsdif0SdCclkOutDelay = Aj49Reg1;
    type Fsdif1SdCclkOutDelay = J53Reg1;
    debug!("fsdif_unset_sdifdelay,sdif id {}",sdif_id);
    if sdif_id == FSDIF0_ID {
        iopad.delay_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayCoarseTuning,
            FioPadDelay::DelayNone);
        iopad.delay_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayFineTuning,
            FioPadDelay::DelayNone);
        iopad.delay_enable_set::<Fsdif0SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay, 
            false);
    } else if sdif_id == FSDIF1_ID {
        iopad.delay_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayCoarseTuning,
            FioPadDelay::DelayNone);
        iopad.delay_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay,
            FioPadDelayType::DelayFineTuning,
            FioPadDelay::DelayNone);
        iopad.delay_enable_set::<Fsdif1SdCclkOutDelay>(
            FioPadDelayDir::OutputDelay, 
            false);
    }
}

pub fn fsdif_sdifdelay_null(_iopad:&mut IoPad,_sdif_id:u32) {
    return;
}