#![allow(unused)] 
#![feature(asm)]

mod regs;
mod err;
mod constants;
mod mci_config;
mod mci_timing;
mod sd_reg;
mod mci_cid;
mod mci_csd;
mod mci_scr;
mod mci_status;

use core::arch::asm;
use core::default;
use core::ptr::NonNull;
use core::time::Duration;
use bare_test::stdout::print;
use bare_test::time::delay;
use log::debug;
use log::error;
use log::info;
use log::warn;
use bitflags::{bitflags, Flags};
use mci_csd::CsdFlags;
use mci_scr::ScrFlags;

use crate::regs::*;
use crate::set_reg32_bits;
use crate::tools::swap_half_word_byte_sequence_u32;
use crate::IoPad;

use regs::*;
use sd_reg::*;
pub use constants::*;
use mci_timing::*;
use mci_config::*;
use err::{FsdifError, FsdifResult};

use crate::sleep;

//* 核心数据结构 */
pub struct MCI {
    pub config: MCIConfig,
    reg: FsdifReg,
    is_ready: bool,
    prev_cmd: u32,
    curr_timing: MCITiming,
    iopad: IoPad,
    pub sd_reg: SdReg,
    block_count: u32,
    block_size: u32,
    version: SdSpecificationVersion,
}
impl MCI {
    pub fn new(reg_base: NonNull<u8>) -> Self {
        MCI {
            reg: FsdifReg::new(reg_base),
            config: MCIConfig::new(),
            //* 暂时无脑True */
            is_ready: true,
            prev_cmd: 0,
            curr_timing: MCITiming::new(),
            iopad: IoPad::new(NonNull::dangling()),
            sd_reg: SdReg::new(),
            block_count: 0,
            block_size: 0,
            version: SdSpecificationVersion::from_bits_truncate(0),
        }
    }

    pub fn set_pio_mode(&mut self) {
        self.config.trans_mode = FsDifTransMode::PioTransMode;
        self.reg.modify_reg(|reg| {
            !FsdifCtrl::USE_INTERNAL_DMAC & !FsdifCtrl::INT_ENABLE & reg
        });
    }

    pub fn set_iopad(&mut self, iopad: IoPad) {
        self.iopad = iopad;
    }

    fn read(&self, offset: u32) -> u32 {
        self.reg.read_32(offset)
    }

    ///! 可能需要修改为 read_reg 版本 
    pub fn vid(&self) -> u32 {
        self.read(FSDIF_VID_OFFSET)
    }

    pub fn uid(&self) -> u32 {
        self.read(FSDIF_UID_OFFSET)
    }

    pub fn card_detected(&self) -> bool {
        self.read(FSDIF_CARD_DETECT_OFFSET) == 0
    }

    pub fn blksize(&self) -> u32 {
        self.read(FSDIF_BLK_SIZ_OFFSET)
    }

    pub fn blksize_set(&self, blksize: u32) {
        self.reg.write_reg(FsdifBlkSiz::from_bits_truncate(blksize));
    }

    pub fn convert_data_to_little_endian(&self, data: &mut [u32], word_size: usize, format: DataPacketFormat) {
       if self.config.endian_mode == EndianMode::EndianModeLittle && 
            format == DataPacketFormat::DataPacketFormatMSBFirst {
                for i in 0..word_size {
                    let val = data[i];
                    data[i] = val.swap_bytes();
                }
            }
        else if self.config.endian_mode == EndianMode::EndianModeHalfWordBig {
                for i in 0..word_size {
                    let val = data[i];
                    data[i] = swap_half_word_byte_sequence_u32(val);
                }
            }
        else if self.config.endian_mode == EndianMode::EndianModeBig &&
            format == DataPacketFormat::DataPacketFormatLSBFirst {
                for i in 0..word_size {
                    let val = data[i];
                    data[i] = val.swap_bytes();
                }
            }
    }

    pub fn polling_card_status_busy(&self,timeout_ms:u32) -> FsdifResult {
        let mut timeout_us = timeout_ms * 1000;
        loop {
            let card_busy = self.check_if_card_busy();
            if !card_busy { 
                //* CMD13 */
                if let Ok(_) = self.card_status_send() {
                    break;
                }
            }else {
                sleep(Duration::from_micros(125));
                timeout_us -= 125;
            }
            if timeout_us == 0 {
                return Err(FsdifError::Busy);
            }
        }
        Ok(())
    }

    pub fn clk_freq_set(&mut self, clk_hz: u32) -> FsdifResult {
        let mut reg_val = FsdifCmd::UPD_CLK;
        let cmd_reg = self.reg.read_reg::<FsdifCmd>();
        let cur_cmd_index =  cmd_reg.index_get();
        if cur_cmd_index == FsDifSDIndivCommand::VoltageSwitch as u32 {
            reg_val |= FsdifCmd::VOLT_SWITCH;
        }
        if clk_hz > 0 && self.config.get_tuning as usize != default_tuning as usize {
            /* select board-related time-tuning configurations */
            let target_timing = (self.config.get_tuning)(clk_hz.into(),self.config.non_removable);
            if target_timing == MMC_SD_NULL {
                error!("No available timeing !!!");
                return Err(FsdifError::InvalidTiming);
            }
            /* update pad delay */
            if target_timing.pad_delay as usize != fsdif_sdifdelay_null as usize {
                (target_timing.pad_delay)(&mut self.iopad,self.config.instance_id);
            }
            /* update clock source setting */
            self.update_exteral_clk(FsdifClkSrc::from_bits_retain(target_timing.clk_src))?;
            self.clock_set(false);
            /* update clock for clock source */
            if cur_cmd_index == FsDifSDIndivCommand::VoltageSwitch as u32 {
                self.send_private_cmd11(reg_val | cmd_reg)?;
            } else {
                self.send_private_cmd(reg_val, 0)?;
            }
            /* set clock divider */
            self.reg.write_reg(FsdifClkDiv::from_bits_truncate(target_timing.clk_div));
            self.reg.write_reg(FsdifEnableShift::from_bits_truncate(target_timing.shift));
            info!("clk_src: 0x{:x} clk_div: 0x{:x}, shift: 0x{:x}",
                self.reg.read_reg::<FsdifClkSrc>(),
                self.reg.read_reg::<FsdifClkDiv>(),
                self.reg.read_reg::<FsdifEnableShift>());
            self.clock_set(true);
            /* update clock for clock divider */
            if cur_cmd_index == FsDifSDIndivCommand::VoltageSwitch as u32 {
                self.send_private_cmd11(reg_val | cmd_reg)?;
            } else {
                self.send_private_cmd(reg_val, 0)?;
            }
        } else {
            /* close bus clock in case target clock is 0 */
            self.clock_set(false);

            if cur_cmd_index == FsDifSDIndivCommand::VoltageSwitch as u32 {
                self.send_private_cmd11(reg_val | cmd_reg)?;
            } else {
                self.send_private_cmd(reg_val, 0)?;
            }
        }
        Ok(())
    }

    pub fn select_bus_timing(&mut self) -> FsdifResult {
        /* group 1, function 1 ->high speed mode*/
        match self.select_function(GroupNum::TimingMode, 
            TimingFunctionNum::SDR25HighSpeed) {
                Err(_) => {
                    /* if not support high speed, keep the card work at default mode */
                    info!("\r\nNote: High speed mode is not supported by card\r\n");
                },
                Ok(_) => {
                    // todo kSD_TimingSDR25HighSpeedMode
                    // todo bus clock 50MHz
                    self.clk_freq_set(50_000_000);
                }
        }

        /* card is in UHS_I mode */ // todo 暂时不需要
        /* Update io strength according to different bus frequency */ // todo 暂时不需要
        /* SDR50 and SDR104 mode need tuning */ // todo 暂时不需要
        Ok(())
    }

    pub fn select_function(&self, group: GroupNum, function: TimingFunctionNum) -> FsdifResult {
        /* check if card support CMD6 */ // todo 这里暂时不需要写，后续需要写上，因为需要相应的结构体
        /* CMD6 */
        let mut status  = [0u32; 64];
        /* Check if card support high speed mode. */
        self.switch_function(SwitchMode::Check, group, function, &mut status);
        /* convert to little endian sequence */
        self.convert_data_to_little_endian(&mut status, 5, DataPacketFormat::DataPacketFormatMSBFirst);
        /* 
        -functionStatus[0U]---bit511~bit480;
        -functionStatus[1U]---bit479~bit448;
        -functionStatus[2U]---bit447~bit416;
        -functionStatus[3U]---bit415~bit384;
        -functionStatus[4U]---bit383~bit352;
        According to the "switch function status[bits 511~0]" return by switch command in mode "check function":
            -Check if function 1(high speed) in function group 1 is supported by checking if bit 401 is set;
            -check if function 1 is ready and can be switched by checking if bits 379~376 equal value 1;
        */
        let mut function_grop_info = [0u16;6];
        function_grop_info[5] = status[0] as u16;
        function_grop_info[4] = (status[1] >> 16) as u16;
        function_grop_info[3] = status[1] as u16;
        function_grop_info[2] = (status[2] >> 16) as u16;
        function_grop_info[1] = status[2] as u16;
        function_grop_info[0] = (status[3] >> 16) as u16;
        let current_function_status  = ((status[3] & 0xff) << 8)|(status[4] >> 24);
        /* check if function is support */
        let _group = group as usize;
        let _function = function as u32;
        if (function_grop_info[_group] & (1<<_function) == 0)||
            (current_function_status >> (_group * 4) & 0xf != _function) {
                info!("\r\nError: current card not support function {}\r\n",_function);
                return Err(FsdifError::NotSupport);
        }
        /* Switch to high speed mode. */
        self.switch_function(SwitchMode::Set, group, function, &mut status);
        /* convert to little endian sequence */
        self.convert_data_to_little_endian(&mut status[3..], 2, DataPacketFormat::DataPacketFormatMSBFirst);
        /* 
        According to the "switch function status[bits 511~0]" return by switch command in mode "set function":
            -check if group 1 is successfully changed to function 1 by checking if bits 379~376 equal value 1;
        */
        let current_function_status = ((status[3] & 0xff) << 8)|(status[4] >> 24);
        if (current_function_status >> (_group * 4) & 0xf != _function) {
            info!("\r\nError: switch function {} failed\r\n",_function);
            return Err(FsdifError::NotSupport);
        }
        Ok(())
    }

    pub fn check_if_card_busy(&self) -> bool {
        let reg_val = self.reg.read_reg::<FsdifStatus>();
        (FsdifStatus::DATA_BUSY & reg_val).bits() != 0
    }

    pub fn prob_bus_voltage(&mut self) -> FsdifResult {
        let mut application_command41_argument = SdOcrFlag::empty(); /* OCR arguments */
        /* 3.3V voltage should be supported as default */
        application_command41_argument |= SdOcrFlag::VDD_29_30 | SdOcrFlag::VDD_32_33 | SdOcrFlag::VDD_34_35;
        /*
        * If card is high capacity (SDXC or SDHC), and supports 1.8V signaling,
        * switch to new signal voltage using "signal voltage switch procedure"
        * described in SD specification
        */
        // todo 这里高速的部分用不到
        // todo card->operationVoltage = kSDMMC_OperationVoltage330V;
        /* send card active */ //* 空函数 */

        loop {
            /* card go idle */
            self.go_idle()?; // CMD0
            match self.send_interface_condition() { /* CMD8 */
                Ok(_) => {
                    /* SDHC or SDXC card */
                    warn!("Card is SDHC or SDXC");
                    application_command41_argument |= SdOcrFlag::CARD_CAPACITY_SUPPORT;
                    // todo card->flags |= (uint32_t)kSD_SupportSdhcFlag;
                },
                Err(_) => {
                    /* SDSC card */
                    warn!("Card is SDSC");
                    self.go_idle()?; /* make up for legacy card which do not support CMD8 */
                }
            }
            /* Set card interface condition according to SDHC capability and card's supported interface condition. */
            self.application_send_opration_condition(application_command41_argument)?; /* ACMD41 */
            /* check if card support 1.8V */ //* 这里可以暂时跳过 */
            break;
        }

        Ok(())
    }

    pub fn card_power_set(&self, enable: bool) {
        self.reg.modify_reg(|reg| {
            if enable {
                FsdifPwrEn::ENABLE | reg
            } else {
                !FsdifPwrEn::ENABLE & reg
            }
        });
    }

    //* 因为暂时不会被调用而不去实现 */
    pub fn io_voltage_switch(&self) {
        todo!()
    }

    pub fn switch_to_voltage(&self,voltage:SdmmcOperationVoltage)  {
        match voltage {
            SdmmcOperationVoltage::V330V => {
                self.voltage_1_8v_set(false);
            }
            SdmmcOperationVoltage::V300V => {
                self.voltage_1_8v_set(false);
            }
            SdmmcOperationVoltage::V180V => {
                self.voltage_1_8v_set(true);
            }
            _ => {
                error!("Invalid voltage value !!!");
            }
        }
    }

    pub fn update_exteral_clk(&self,uhs_reg:FsdifClkSrc) -> FsdifResult {
        self.reg.write_reg(FsdifClkSrc::from_bits_truncate(0));
        self.reg.write_reg(uhs_reg);
        self.reg.wait_for(|reg| {
            (FsdifClkSts::READY & reg).bits() != 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        Ok(())
    }

    pub fn init_external_clk(&self) -> FsdifResult {
        let reg_val = 
        FsdifClkSrc::uhs_reg(0, 0, 0x5) | 
        FsdifClkSrc::UHS_EXT_CLK_ENA;
        if 0x502 == reg_val.bits() {
            info!("invalid uhs config"); 
        }
        self.update_exteral_clk(reg_val)?;
        Ok(())
    }

    /*
    trans_size: Burst size of multiple transaction;
    rx_wmark: FIFO threshold watermark level when receiving data to card.
    tx_wmark: FIFO threshold watermark level when transmitting data to card
    */
    pub fn fifoth_set(&self,
        trans_size:FsdifFifoThDmaTransSize,
        rx_wmark:u32,
        tx_wmark:u32){
        let trans_size:u32 = trans_size.into();
        let val = 
        (FsdifFifoTh::DMA_TRANS_MASK & (trans_size << 28).into()) |
        (FsdifFifoTh::RX_WMARK_MASK & (rx_wmark << 16).into()) |
        (FsdifFifoTh::TX_WMARK_MASK & tx_wmark.into());
        info!("fifoth set to 0x{:x}",val); //* 经检查无问题 */
        self.reg.write_reg(val);
    }

    /* FSDIF_CLK_SRC_OFFSET 和 FSDIF_CLKDIV_OFFSET 两个寄存器配合完成卡时钟和驱动采样相位调整
    UHS_REG_EXT 配置一级分频，CLK_DIV 决定CARD工作时钟频率, DRV 和 SAMP 分别控制驱动相位和采样相位粗调
        分配系数 = bit [14 : 8] + 1
    CLKDIV 配置二级分频, DIVIDER , DRV 和 SAMP 分别控制驱动相位和采样相位精调
        分配系数 = bit [7: 0] * 2
    */
    pub fn uhs_reg_set(&self,drv_phase: u32, samp_phase: u32, clk_div: u32) {
        self.reg.modify_reg(|reg|{
            FsdifClkSrc::uhs_reg(drv_phase, samp_phase, clk_div)}
        );
    }

    pub fn power_set(&self, enable:bool){
        self.reg.modify_reg(|reg| {
            if enable {
                FsdifPwrEn::ENABLE | reg
            } else {
                !FsdifPwrEn::ENABLE & reg
            }
        });
        info!("power set to 0x{:x}",self.reg.read_reg::<FsdifPwrEn>());
    }

    pub fn clock_set(&self, enable:bool){
        self.reg.modify_reg(|reg| {
            if enable {
                FsdifClkEn::CCLK_ENABLE | reg
            } else {
                !FsdifClkEn::CCLK_ENABLE & reg
            }
        });
    }

    pub fn clock_src_set(&self, enable:bool){
        self.reg.modify_reg(|reg| {
            if enable {
                FsdifClkSrc::UHS_EXT_CLK_ENA |reg
            } else {
                !FsdifClkSrc::UHS_EXT_CLK_ENA & reg
            }
        });
        info!("clock src set to 0x{:x}",self.reg.read_reg::<FsdifClkSrc>());  
    }

    pub fn voltage_1_8v_set(&self,enable:bool){
        self.reg.modify_reg(|reg| {
            if enable {
                FsdifUhsReg::VOLT_180 | reg
            } else {
                !FsdifUhsReg::VOLT_180 & reg
            }
        });
        info!("voltage set to 0x{:x}",self.reg.read_reg::<FsdifUhsReg>());  
    }

    pub fn bus_width_set(&self, width: u32) -> FsdifResult {
        let reg_val:FsdifCType;
        if width == 1 {
            reg_val = FsdifCType::CARD0_WIDTH2_1BIT;
        } else if width == 4 {
            reg_val = FsdifCType::CARD0_WIDTH2_4BIT;
        } else if width == 8 {
            reg_val = FsdifCType::CARD0_WIDTH1_8BIT;
        } else {
            return Err(FsdifError::NotSupport);
        }
        self.reg.write_reg(reg_val);
        info!("bus width set to 0x{:x}",self.reg.read_reg::<FsdifCType>());
        Ok(())
    }

    pub fn ctrl_reset(&self, reset_bits: FsdifCtrl) -> FsdifResult {
        self.reg.modify_reg(|reg| {
            reset_bits | reg
        });
        self.reg.wait_for(|reg| {
            (reset_bits & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        /* update clock after reset */
        self.send_private_cmd(FsdifCmd::UPD_CLK, 0)?;
        /* for fifo reset, need to check if fifo empty */
        if reset_bits.contains(FsdifCtrl::FIFO_RESET) {
            self.reg.wait_for(|reg| {
                (FsdifStatus::FIFO_EMPTY & reg).bits() != 0
            }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        }
        Ok(())
    }

    //** 专门给CMD11 也就是 Switch Voltage 设计的 */
    pub fn send_private_cmd11(&self,cmd:FsdifCmd) -> FsdifResult {
        // unsafe { dsb() };/* drain writebuffer */
        self.reg.write_reg(FsdifCmd::START | cmd);
        self.reg.wait_for(|reg|{
            (FsdifCmd::START & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        Ok(())
    }

    pub fn send_private_cmd(&self, cmd:FsdifCmd, arg: u32) -> FsdifResult {
        self.reg.wait_for(|reg| {
            (FsdifStatus::DATA_BUSY & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        self.reg.write_reg(FsdifCmdArg::from_bits_truncate(arg));
        // unsafe { dsb() };/* drain writebuffer */
        let cmd_reg = FsdifCmd::START | cmd;
        self.reg.write_reg(cmd_reg);
        self.reg.wait_for(|reg|{
            (FsdifCmd::START & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        Ok(())
    }

    pub fn descriptor_set(&self, desc: u32) {
        self.reg.write_reg(FsdifDescListAddrH::empty());
        self.reg.write_reg(FsdifDescListAddrL::from_bits_truncate(desc));
    }

    pub fn idma_reset(&self) {
        let mut reg_val = self.reg.read_reg::<FsdifBusMode>();
        reg_val |= FsdifBusMode::SWR;
        self.reg.write_reg(reg_val);
    }

    pub fn poll_wait_busy_card(&self) -> FsdifResult {
        let busy_bits = FsdifStatus::DATA_BUSY | FsdifStatus::DATA_STATE_MC_BUSY;
        let reg_val = self.reg.read_reg::<FsdifStatus>();
        if reg_val.contains(busy_bits.clone()) {
           warn!("Card is busy, waiting ...");
        }
        self.reg.wait_for(|reg|{
            (busy_bits & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        Ok(())
    }

    pub fn trans_bytes_set(&self, bytes: u32) {
        self.reg.write_reg(FsdifBytCnt::from_bits_truncate(bytes));
    }

    pub fn raw_status_get(&self) -> FsdifRawInts{
        return self.reg.read_reg::<FsdifRawInts>();
    }

    pub fn raw_status_clear(&self) {
        let reg_val = self.raw_status_get();
        self.reg.write_reg(reg_val);
    }

    pub fn busy_card_reset(&self) -> FsdifResult {
        self.reg.modify_reg(|reg| {
            FsdifCtrl::CONTROLLER_RESET | reg
        });
        self.reg.wait_for(|reg|{
            self.reg.modify_reg(|reg| {
                FsdifCtrl::CONTROLLER_RESET | reg
            });
            (FsdifStatus::DATA_BUSY & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        Ok(())
    }

    pub fn clk_restart(&self) -> FsdifResult {
        /* wait command finish if previous command is in error state */
        self.reg.wait_for(|reg|{
            (FsdifCmd::START & reg).bits() == 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        /* update clock */
        self.clock_set(false);
        let clk_div = self.reg.read_reg::<FsdifClkDiv>();
        let uhs = self.reg.read_reg::<FsdifClkSrc>();
        self.update_exteral_clk(uhs)?;
        self.reg.write_reg(clk_div);
        self.clock_set(true);
        self.send_private_cmd(FsdifCmd::UPD_CLK, 0)?;
        Ok(())
    }

    pub fn restart(&self) -> FsdifResult {
        /* reset controller */
        self.ctrl_reset(FsdifCtrl::FIFO_RESET)?;
        /* reset controller if in busy state */
        self.busy_card_reset()?;
        /* reset clock */
        self.clk_restart()?;
        /* reset internal DMA */
        // ! 测试性代码
        if self.config.trans_mode == FsDifTransMode::DmaTransMode {
            self.idma_reset();
        }
        Ok(())
    }

    pub fn reset(&self) -> FsdifResult {
        /* check FsdifCtrl */
        info!("FsdifCtrl: 0x{:x}",self.reg.read_reg::<FsdifCtrl>());
        /* set fifo */
        self.fifoth_set(
            FsdifFifoThDmaTransSize::DmaTrans8, 
            FSDIF_RX_WMARK, 
            FSDIF_TX_WMARK);  
        debug!("fifoth set success");
        /* set card threshold */
        self.reg.write_reg( 
            FsdifCardThrctl::CARDRD |
            FsdifFifoDepth::Depth8.card_thrctl_threshold().into());
        info!("card threshold set to 0x{:x}",
        (FsdifCardThrctl::CARDRD |
        FsdifFifoDepth::Depth8.card_thrctl_threshold().into()).bits());  
        debug!("card threshold set success");
        /* disable clock and update ext clk */
        self.clock_set(false);  
        debug!("clock set success");
        /* set 1st clock */
        self.init_external_clk()?;  
        debug!("external clock init success");
        /* power on */
        self.power_set(true);  
        self.clock_set(true);  
        self.clock_src_set(true);  
        debug!("power on success");
        /* set voltage as 3.3v */
        self.voltage_1_8v_set(false);  
        debug!("voltage set to 3.3v");
        /* set bus width as 1 */
        self.bus_width_set(1)?;  
        debug!("bus width set to 1");
        /* reset controller and card */
        if self.config.trans_mode == FsDifTransMode::DmaTransMode {
            self.ctrl_reset(FsdifCtrl::FIFO_RESET | FsdifCtrl::DMA_RESET)?;
        } else {
            self.ctrl_reset(FsdifCtrl::FIFO_RESET)?;  
        }
        debug!("controller reset success");
        /* send private command to update clock */
        self.send_private_cmd(FsdifCmd::UPD_CLK, 0)?;
        debug!("send private command success");
        /* reset card for no-removeable media, e.g. eMMC */
        if self.config.non_removable {
            self.reg.modify_reg(|reg|{
                FsdifCardReset::ENABLE | reg
            });
        }else {
            self.reg.modify_reg(|reg|{
                !FsdifCardReset::ENABLE & reg
            });
        }
        info!("card reset to 0x{:x}",self.reg.read_reg::<FsdifCardReset>());
        debug!("card reset success");
        /* clear interrupt status */  
        self.reg.write_reg(FsdifInt::empty());
        info!("clear interrupt status to 0x{:x}",self.reg.read_reg::<FsdifInt>());
        let reg_val = self.reg.read_reg::<FsdifRawInts>();
        self.reg.write_reg(reg_val);
        info!("clear interrupt status to 0x{:x}",self.reg.read_reg::<FsdifRawInts>());
        self.reg.write_reg(FsdifDmacIntEn::empty());
        info!("clear interrupt status to 0x{:x}",self.reg.read_reg::<FsdifDmacIntEn>());
        let reg_val = self.reg.read_reg::<FsdifDmacStatus>();
        self.reg.write_reg(reg_val);
        info!("clear interrupt status to 0x{:x}",self.reg.read_reg::<FsdifDmacStatus>());
        debug!("clear interrupt status success");
        /* enable card detect interrupt */
        if !self.config.non_removable {
            self.reg.modify_reg(|reg|{
                FsdifInt::CD_BIT | reg
            });
        }
        info!("enable card detect interrupt to 0x{:x}",self.reg.read_reg::<FsdifInt>());
        debug!("enable card detect interrupt success");
        /* enable controller and internal DMA */
        self.reg.modify_reg(|reg|{
            FsdifCtrl::INT_ENABLE | FsdifCtrl::USE_INTERNAL_DMAC | reg
        });
        info!("enable controller and internal DMA to 0x{:x}",self.reg.read_reg::<FsdifCtrl>());  
        debug!("enable controller and internal DMA success");
        /* set data and resp timeout */
        self.reg.write_reg(FsdifTimeout::timeout_data(
            FsdifTimeout::MAX_DATA_TIMEOUT, 
            FsdifTimeout::MAX_RESP_TIMEOUT));
        info!("set data and resp timeout to 0x{:x}",self.reg.read_reg::<FsdifTimeout>());
        debug!("set data and resp timeout success");
        /* reset descriptors and dma */
        if self.config.trans_mode == FsDifTransMode::DmaTransMode {
            self.descriptor_set(0);
            self.idma_reset();
        }
        debug!("reset descriptors and dma success");
        Ok(())
    }
}

//* Decode 相关的函数 */
impl MCI {
    pub fn decode_cid(&mut self, rawcid: &[u32]) {
        self.sd_reg.cid.manufacturer_id = ((rawcid[3] & 0xFF000000 ) >> 24 ) as u8;
        self.sd_reg.cid.application_id = ((rawcid[3] & 0xFFFF00 ) >> 8 ) as u16;

        self.sd_reg.cid.product_name[0] = (rawcid[3] & 0xFF) as u8;
        self.sd_reg.cid.product_name[1] = ((rawcid[2] & 0xFF000000 ) >> 24 ) as u8;
        self.sd_reg.cid.product_name[2] = ((rawcid[2] & 0xFF0000 ) >> 16 ) as u8;
        self.sd_reg.cid.product_name[3] = ((rawcid[2] & 0xFF00 ) >> 8 ) as u8;
        self.sd_reg.cid.product_name[4] = (rawcid[2] & 0xFF ) as u8;

        self.sd_reg.cid.product_version = ((rawcid[1] & 0xFF000000 ) >> 24 ) as u8;
        self.sd_reg.cid.serial_number = ((rawcid[1] & 0xFFFFFF ) << 8 ) |
                                        ((rawcid[0] & 0xFF000000 ) >> 24 );
        
        self.sd_reg.cid.manufacturing_data = ((rawcid[0] & 0xFFF00 ) >> 8 ) as u16;
    }

    pub fn decode_csd(&mut self, rawcsd: &[u32]) {
        self.sd_reg.csd.csd_structure = ((rawcsd[3] & 0xC0000000 ) >> 30 ) as u8;
        self.sd_reg.csd.data_read_access_time1 = ((rawcsd[3] & 0xFF0000 ) >> 16 ) as u8;
        self.sd_reg.csd.data_read_access_time2 = ((rawcsd[3] & 0xFF00 ) >> 8 ) as u8;
        self.sd_reg.csd.transfer_speed = (rawcsd[3] & 0xFF) as u8;
        self.sd_reg.csd.card_command_classes = ((rawcsd[2] & 0xFFF00000 ) >> 20 ) as u16;
        self.sd_reg.csd.read_block_length = ((rawcsd[2] & 0xF0000 ) >> 16 ) as u8;
        if rawcsd[2] & 0x8000 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::READ_BLOCK_PARTIAL.bits();
        }
        if rawcsd[2] & 0x4000 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::READ_BLOCK_PARTIAL.bits();
        }
        if rawcsd[2] & 0x2000 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::READ_BLOCK_MISALIGN.bits();
        }
        if rawcsd[2] & 0x1000 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::DSR_IMPLEMENTED.bits();
        }
        if self.sd_reg.csd.csd_structure == 0 {
            info!("   csd structure: 1.0");
            self.sd_reg.csd.device_size = ((rawcsd[2] & 0x3FF) << 2) | 
                                          ((rawcsd[1] & 0xC0000000) >> 30);
            self.sd_reg.csd.read_current_vdd_min = ((rawcsd[1] & 0x38000000) >> 27) as u8;
            self.sd_reg.csd.read_current_vdd_max = ((rawcsd[1] & 0x7000000) >> 24) as u8;
            self.sd_reg.csd.write_current_vdd_min = ((rawcsd[1] & 0xE00000) >> 20) as u8;
            self.sd_reg.csd.write_current_vdd_max = ((rawcsd[1] & 0x1C0000) >> 18) as u8;
            self.sd_reg.csd.device_size_multiplier = ((rawcsd[1] & 0x38000) >> 15) as u8;
            /* Get card total block count and block size. */
            self.block_count = (self.sd_reg.csd.device_size + 1) << 
                               (self.sd_reg.csd.device_size_multiplier + 2);
            self.block_size = 1 << self.sd_reg.csd.read_block_length;
            // ! 这里的512 不知道要不要设置为常量
            if self.block_size > 512 {
                self.block_count = self.block_count * self.block_size;
                self.block_size = 512;
                self.block_count = self.block_count / self.block_size;
            }
        } else if self.sd_reg.csd.csd_structure == 1 {
            info!("   csd structure: 2.0");
            self.block_size = 512;
            self.sd_reg.csd.device_size = ((rawcsd[2] & 0x3F) << 16) | 
                                          ((rawcsd[1] & 0xFFFF0000) >> 16);
            if self.sd_reg.csd.device_size >= 0xFFF {
                // todo card->flags |= (uint32_t)kSD_SupportSdxcFlag;
            }
            self.block_count = (self.sd_reg.csd.device_size + 1) * 1024;
        } else {
            info!("unknown SD CSD structure version 0x{:x}",
            self.sd_reg.csd.csd_structure);
            /* not support csd version */
        }

        if ((rawcsd[1] & 0x4000) >> 14) as u8!= 0 {
            self.sd_reg.csd.flags |= CsdFlags::ERASE_BLOCK_ENABLED.bits();
        }

        self.sd_reg.csd.erase_sector_size = ((rawcsd[1] & 0x3F80) >> 7) as u8;
        self.sd_reg.csd.write_protect_group_size = (rawcsd[1] & 0x7F) as u8;

        if (rawcsd[0] & 0x80000000) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::WRITE_PROTECT_GROUP_ENABLED.bits();
        }

        self.sd_reg.csd.write_speed_factor = ((rawcsd[0] & 0x1C000000) >> 26) as u8;
        self.sd_reg.csd.write_block_length = ((rawcsd[0] & 0x3C00000) >> 22) as u8;

        if ((rawcsd[0] & 0x200000) >> 21) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::WRITE_BLOCK_PARTIAL.bits();
        }
        if ((rawcsd[0] & 0x8000) >> 15) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::FILE_FORMAT_GROUP.bits();
        }
        if ((rawcsd[0] & 0x4000) >> 14) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::COPY.bits();
        }
        if ((rawcsd[0] & 0x2000) >> 13) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::PERMANENT_WRITE_PROTECT.bits();
        }
        if ((rawcsd[0] & 0x1000) >> 12) as u8 != 0 {
            self.sd_reg.csd.flags |= CsdFlags::TEMPORARY_WRITE_PROTECT.bits();
        }
        self.sd_reg.csd.file_format = ((rawcsd[0] & 0xC00) >> 10) as u8;

        info!("Card block count {}, block size {}",self.block_count,self.block_size);
    }
    pub fn decode_scr(&mut self, rawscr: &[u32]) {
        self.sd_reg.scr.scr_structure = ((rawscr[0] & 0xF0000000) >> 28) as u8;
        self.sd_reg.scr.sd_specification = ((rawscr[0] & 0xF000000) >> 24) as u8;
        if ((rawscr[0] & 0x800000) >> 23) as u8 != 0 {
            self.sd_reg.scr.flags |= ScrFlags::DATA_STATUS_AFTER_ERASE.bits();
        }
        self.sd_reg.scr.sd_security = ((rawscr[0] & 0x700000) >> 20) as u8;
        self.sd_reg.scr.sd_bus_widths = ((rawscr[0] & 0xF0000) >> 16) as u8;
        if ((rawscr[0] & 0x8000) >> 15) as u8 != 0 {
            self.sd_reg.scr.flags |= ScrFlags::SD_SPECIFICATION3.bits();
        }
        self.sd_reg.scr.extended_security = ((rawscr[0] & 0x7800) >> 10) as u8;
        self.sd_reg.scr.command_support = (rawscr[0] & 0x3) as u8;
        self.sd_reg.scr.reserved_for_manufacturer = rawscr[1];
        /* Get specification version. */
        if self.sd_reg.scr.sd_specification == 0 {
            info!("   SCR version: 1.0");
            self.version = SdSpecificationVersion::VERSION_1_0;
        } else if self.sd_reg.scr.sd_specification == 1 {
            info!("   SCR version: 1.1");
            self.version = SdSpecificationVersion::VERSION_1_1;
        } else if self.sd_reg.scr.sd_specification == 2 {
            info!("   SCR version: 2.0");
            self.version = SdSpecificationVersion::VERSION_2_0;
            if self.sd_reg.scr.flags & ScrFlags::SD_SPECIFICATION3.bits() != 0 {
                info!("   SCR version: 3.0");
                self.version = SdSpecificationVersion::VERSION_3_0;
            }
        } else {
            info!("   SCR version: unknown");
        }
        /* Check card supported bus width */
        if self.sd_reg.scr.sd_bus_widths & 0x4 != 0 {
            info!("   Card support 4-bit bus width");
            // todo card->flags |= (uint32_t)kSD_Support4BitWidthFlag;
        }
        /* Check if card supports speed class command (CMD20) */
        if self.sd_reg.scr.command_support & 0x1 != 0 {
            info!("   Card support speed class control command");
            // todo card->flags |= (uint32_t)kSD_SupportSpeedClassControlCmdFlag;
        }
        /* Check if card supports set block count command (CMD23) */
        if self.sd_reg.scr.command_support & 0x2 != 0 {
            info!("   Card support set block count command");
            // todo card->flags |= (uint32_t)kSD_SupportSetBlockCountCmdFlag;
        }
    }
    pub fn decode_status(&mut self, src: &[u32]) {
        self.sd_reg.status.bus_width = ((src[0] & 0xC0000000) >> 30) as u8;
        self.sd_reg.status.secure_mode = ((src[0] & 0x20000000) >> 29) as u8;
        self.sd_reg.status.card_type = (src[0] & 0x0000FFFF) as u16;
        self.sd_reg.status.protected_size = src[1];
        self.sd_reg.status.speed_class = ((src[2] & 0xFF000000) >> 24) as u8;
        self.sd_reg.status.performance_move = ((src[2] & 0x00FF0000) >> 16) as u8;
        self.sd_reg.status.erase_size = (((src[2] & 0x000000FF) << 8) | 
                                        ((src[3] & 0xFF000000) >> 24)) as u16;
        self.sd_reg.status.erase_timeout = ((((src[3] & 0x00FF0000) >> 16) & 0xFC) >> 2) as u8;
        self.sd_reg.status.erase_offset = (((src[3] & 0x00FF0000) >> 16) & 0x3)as u8;
        self.sd_reg.status.uhs_speed_grade = ((((src[3] & 0x0000FF00) >> 8) & 0xF0) >> 4) as u8;
        self.sd_reg.status.uhs_au_size = (((src[3] & 0x0000FF00) >> 8) & 0xF) as u8;
    }
}


//* CMD 相关的函数 */
impl MCI {

    //* CMD0 */
    pub fn go_idle(&self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::GoIdleState as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::empty(),
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        Ok(())
    }

    //* CMD2 */
    pub fn all_send_cid(&mut self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::AllSendCid as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::EXP_LONG_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        self.decode_cid(&cmd_data.response);
        Ok(())
    }

    //* CMD3 */
    pub fn send_rca(&mut self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifSDIndivCommand::SendRelativeAddress as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        self.sd_reg.rca = cmd_data.response[0] >> 16;
        Ok(())
    }

    //* CMD6 */
    pub fn switch_function(&self, mode: SwitchMode, group: GroupNum, number: TimingFunctionNum, status: &mut [u32]) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifSDIndivCommand::Switch as u32,
            cmdarg: {
                let mut arg = 0;
                let mode = mode as u32;
                let group = group as u32;
                let number = number as u32;
                arg |= mode << 31 | 0x00FFFFFF;
                arg &= !(0xF << (group * 4));
                arg |= number << (group * 4);
                arg
            },
            response: [0; 4],
            flag: CmdFlag::READ_DATA | CmdFlag::EXP_DATA | CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: status,
                buf_dma: 0,
                blksz: 64,
                blkcnt: 1,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    //* CMD7 */
    pub fn select_card(&self,relative_address:u32,is_selected:bool) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::SelectCard as u32,
            cmdarg: if is_selected { relative_address << 16 } else { 0 },
            response: [0; 4],
            flag: if is_selected{CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC} else {CmdFlag::empty()},
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    //* CMD8 */
    pub fn send_interface_condition(&self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifSDIndivCommand::SendInterfaceCondition as u32,
            cmdarg: 0x1AA,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        let mut i = 0;
        while i < FSL_SDMMC_MAX_CMD_RETRIES {
            if let Err(_)=self.pio_transfer(&cmd_data) {
                i = i + 1;
                continue;
            }
            if let Err(_)=self.poll_wait_pio_end(&mut cmd_data) {
                i = i + 1;
                continue;
            }
            if cmd_data.response[0] & 0xFF != 0xAA {
                warn!("CMD8 NotSupport");
                return Err(FsdifError::NotSupport);
            }
            break;
        }
        if i >= FSL_SDMMC_MAX_CMD_RETRIES {
            warn!("CMD8 TimeOut");
            return Err(FsdifError::CmdTimeout);
        }
        Ok(())
    }

    //* CMD9 */
    pub fn send_csd(&mut self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::SendCsd as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::EXP_LONG_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        self.decode_csd(&cmd_data.response);
        Ok(())
    }

    //* CMD11 */
    pub fn voltage_switch(&self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifSDIndivCommand::VoltageSwitch as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        /*
        * Card should drive CMD and DAT[3:0] signals low at the next clock
        * cycle. Some cards will only drive these
        * lines low briefly, so we should check as soon as possible
        */
        if !self.check_if_card_busy() {
            /* Delay 1ms to allow card to drive lines low */
            sleep(Duration::from_millis(1));
            if !self.check_if_card_busy() {
                /* Card did not drive CMD and DAT lines low */
                info!("Card did not drive DAT lines low");
                return Err(FsdifError::Busy);
            }
        }

        /*
        * Per SD spec (section "Timing to Switch Signal Voltage"),
        * host must gate clock at least 5ms.
        */
        // todo 暂时好像不需要switch
        // self.clk_freq_set(0);
        // /* switch io voltage */
        // self.io_voltage_switch(); 

        // /* Gate for 10ms, even though spec requires 5 */
        // sleep(Duration::from_millis(10));

        // /* Restart the clock */
        // self.clk_freq_set(self.config.bus_clk_hz)?;

        /*
        * If SD does not drive at least one of
        * DAT[3:0] high within 1ms, switch failed
        */
        sleep(Duration::from_millis(1));
        if self.check_if_card_busy() {
            info!("Card failed to switch voltages");
            return Err(FsdifError::Busy);
        }
        info!("Card switched to 1.8V signaling");
        Ok(())
    }

    //* CMD12 */
    pub fn stop_transmission(&self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::StopTransmission as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    //* CMD13 */
    pub fn card_status_send(&self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::SendStatus as u32,
            cmdarg: self.sd_reg.rca << 16,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        let mut retry = 10;
        while retry > 0 {
            if let Err(_) = self.pio_transfer(&cmd_data) {
                retry -= 1;
                continue;
            }
            if let Err(_) = self.poll_wait_pio_end(&mut cmd_data) {
                retry -= 1;
                continue;
            }
            if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
                return Err(FsdifError::InvalidState);
            }
            if cmd_data.response[0] & SdmmcR1CardStatusFlag::READY_FOR_DATA.bits() != 0 {
                break;
            }
            retry -= 1;
        }
        Ok(())
    }

    //* CMD16 */
    pub fn block_size_set(&self, blksize: u32) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::SetBlockLength as u32,
            cmdarg: blksize,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    pub fn read_single_block(&self) -> FsdifResult {
        /* read command are not allowed while card is programming */
        self.polling_card_status_busy(600)?;
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::ReadSingleBlock as u32,
            cmdarg: 131072+100,
            response: [0; 4],
            flag: CmdFlag::READ_DATA | CmdFlag::EXP_DATA | CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [0;512],
                buf_dma: 0,
                blksz: 512,
                blkcnt: 1,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        for i in 0..cmd_data.data.buf.len() {
            warn!("{:x},{:x},{:x},{:x}",
                cmd_data.data.buf[i] as u8,
                (cmd_data.data.buf[i] >> 8) as u8,
                (cmd_data.data.buf[i] >> 16) as u8,
                (cmd_data.data.buf[i] >> 24) as u8);
        }
        Ok(())
    }

    //* CMD23 */
    pub fn block_count_set(&self, blkcnt: u32) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::SetBlockCount as u32,
            cmdarg: blkcnt,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    //* CMD55 */
    pub fn send_application_command(&mut self, relative_address:u32) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: FsDifCommand::ApplicationCommand as u32,
            cmdarg: relative_address << 16,
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            let flag = SdmmcR1CardStatusFlag::from_bits_truncate(cmd_data.response[0]);
            flag.check();
            return Err(FsdifError::InvalidState);
        }
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::APPLICATION_COMMAND.bits() == 0 {
            let flag = SdmmcR1CardStatusFlag::from_bits_truncate(cmd_data.response[0]);
            flag.check();
            return Err(FsdifError::NotSupport);
        }
        self.prev_cmd = Self::FSDIF_EXT_APP_CMD;
        Ok(())
    }
}

//* ACMD */
impl MCI {

    //* ACMD6 */
    pub fn data_bus_width_set(&mut self,width:SdmmcBusWidth) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: SdApplicationCommand::SetBusWidth as u32,
            cmdarg: match width {
                SdmmcBusWidth::BusWidth1Bit => 0,
                SdmmcBusWidth::BusWidth4Bit => 2,
                default => {
                    return Err(FsdifError::NotSupport);
                }
            }, 
            response: [0; 4],
            flag: CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        self.send_application_command(self.sd_reg.rca)?;
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }
        Ok(())
    }

    //* ACMD13 */
    pub fn read_status(&mut self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: SdApplicationCommand::Status as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::READ_DATA | CmdFlag::EXP_DATA | CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [0; 64],
                buf_dma: 0,
                blksz: 64,
                blkcnt: 1,
            },
            success: false,
        };
        /* wait card status ready. */
        self.polling_card_status_busy(600)?;
        self.send_application_command(self.sd_reg.rca)?;
        self.transfer(&mut cmd_data,3)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            return Err(FsdifError::InvalidState);
        }

        /* switch to little endian sequence, as width data, SD Status are also sent in MSB */
        self.convert_data_to_little_endian(cmd_data.data.buf, 16,DataPacketFormat::DataPacketFormatMSBFirst);
        self.decode_status(cmd_data.data.buf);
        Ok(())
    }

    //* ACMD41 */
    pub fn application_send_opration_condition(&mut self,arg: SdOcrFlag) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: SdApplicationCommand::SendOperationCondition as u32,
            cmdarg: arg.bits(),
            response: [0; 4],
            flag: CmdFlag::EXP_RESP,
            data: FsdifBuf {
                buf: &mut [],
                buf_dma: 0,
                blksz: 0,
                blkcnt: 0,
            },
            success: false,
        };
        let mut i = 0;
        while i < FSL_SDMMC_MAX_CMD_RETRIES {
            if let Err(_) = self.send_application_command(0){
                debug!("send application command failed");
                i = i + 1;
                continue;
            }
            if let Err(_)=self.pio_transfer(&cmd_data) {
                i = i + 1;
                continue;
            }
            if let Err(_)=self.poll_wait_pio_end(&mut cmd_data) {
                i = i + 1;
                continue;
            }
            /* Wait until card exit busy state. */
            if cmd_data.response[0] & SdOcrFlag::POWER_UP_BUSY.bits() != 0 {
                /* high capacity check */
                if  cmd_data.response[0] & SdOcrFlag::CARD_CAPACITY_SUPPORT.bits() !=0 {
                    // todo card->flags |= (uint32_t)kSD_SupportHighCapacityFlag;
                    warn!("Is high capcity card > 2GB");
                }
                /* 1.8V support */
                if cmd_data.response[0] & SdOcrFlag::SWITCH_18_ACCEPT.bits() != 0 {
                    // todo card->flags |= (uint32_t)kSD_SupportVoltage180v;
                    warn!("Is UHS card support 1.8v");
                }else {
                    warn!("Not UHS card only support 3.3v")
                }
                self.sd_reg.ocr = cmd_data.response[0];
                return Ok(());
            }
            sleep(Duration::from_millis(10));
        }
        if i >= FSL_SDMMC_MAX_CMD_RETRIES {
            return Err(FsdifError::CmdTimeout);
        }
        Ok(())
    }

    //* ACMD51 */
    pub fn send_scr(&mut self) -> FsdifResult {
        let mut cmd_data = FSdifCmdData {
            cmdidx: SdApplicationCommand::SendScr as u32,
            cmdarg: 0,
            response: [0; 4],
            flag: CmdFlag::READ_DATA | CmdFlag::EXP_DATA | CmdFlag::EXP_RESP | CmdFlag::NEED_RESP_CRC,
            data: FsdifBuf {
                buf: &mut [0;8],
                buf_dma: 0,
                blksz: 8,
                blkcnt: 1,
            },
            success: false,
        };
        self.send_application_command(self.sd_reg.rca)?;
        self.pio_transfer(&cmd_data)?;
        self.poll_wait_pio_end(&mut cmd_data)?;
        if cmd_data.response[0] & SdmmcR1CardStatusFlag::SDMMC_R1_ALL_ERROR_FLAG.bits() != 0 {
            info!("\r\nError: send ACMD51 failed with host error, response {:x}\r\n",cmd_data.response[0]);
            return Err(FsdifError::InvalidState);
        }
         /* according to spec. there are two types of Data packet format for SD card
            1. Usual data (8-bit width), are sent in LSB first
            2. Wide width data (SD Memory register), are shifted from the MSB bit, 
        e.g. ACMD13 (SD Status), ACMD51 (SCR) */
        self.convert_data_to_little_endian(cmd_data.data.buf, 2,DataPacketFormat::DataPacketFormatMSBFirst);
        /* decode scr */
        self.decode_scr(cmd_data.data.buf);
        Ok(())
    }

}


//* PIO 相关的函数 */
impl MCI {
    pub fn pio_write_data<'a>(&self, data: &[u32]) -> FsdifResult {
        self.reg.write_reg(FsdifCmd::DAT_WRITE);
        for i in 0..data.len() {
            self.reg.write_reg(FsdifData::from_bits_truncate(data[i]));
        }
        Ok(())
    }

    pub fn pio_read_data(&self, data: &mut [u32]) -> FsdifResult {
        if data.len() > FSDIF_MAX_FIFO_CNT as usize {
            return Err(FsdifError::NotSupport);
        }
        for i in 0..data.len() {
            data[i] = self.reg.read_reg::<FsdifData>().bits();
        }
        Ok(())
    }

    pub fn pio_transfer(&self, cmd_data: &FSdifCmdData) -> FsdifResult {
        let read = cmd_data.flag.contains(CmdFlag::READ_DATA);
        if !self.is_ready{
            error!("device is not yet initialized!!!");
            return Err(FsdifError::NotInit);
        }
        if self.config.trans_mode != FsDifTransMode::PioTransMode {
            return Err(FsdifError::InvalidState);
        }
        /* for removable media, check if card exists */
        if !self.config.non_removable && !self.card_detected() {
            error!("card is not detected !!!");
            return Err(FsdifError::NoCard);
        }
        /* wait previous command finished and card not busy */
        self.poll_wait_busy_card()?;
        /* reset fifo and not use DMA */
        self.reg.modify_reg(|reg|{
            !FsdifCtrl::USE_INTERNAL_DMAC & reg
        });
        self.ctrl_reset(FsdifCtrl::FIFO_RESET)?;
        self.reg.modify_reg(|reg|{
            !FsdifBusMode::DE & reg
        });
        /* write data */
        if cmd_data.data.buf.len() > 0 {
            warn!("pio transfer data len: 0x{:x}",cmd_data.data.buf.len());
            /* while in PIO mode, max data transferred is 0x800 */
            if cmd_data.data.buf.len() > FSDIF_MAX_FIFO_CNT as usize {
                error!("Fifo do not support writing more than {:x}.",FSDIF_MAX_FIFO_CNT);
                return Err(FsdifError::NotSupport);
            }
            /* set transfer data length and block size */
            self.trans_bytes_set(cmd_data.data.buf.len() as u32);
            self.blksize_set(cmd_data.data.blksz);
            /* if need to write, write to fifo before send command */
            if !read { 
                warn!("pio write data");
                /* invalide buffer for data to write */
                // unsafe { dsb() };
                self.pio_write_data(cmd_data.data.buf)?;
            }
        }
        self.transfer_cmd(cmd_data)?;
        Ok(())
    }

    pub fn poll_wait_pio_end(&self,cmd_data: &mut FSdifCmdData) -> FsdifResult{
        let read = cmd_data.flag.contains(CmdFlag::READ_DATA);
        if !self.is_ready {
            error!("device is not yet initialized!!!");
            return Err(FsdifError::NotInit);
        }
        if FsDifTransMode::PioTransMode != self.config.trans_mode {
            error!("device is not configure in PIO transfer mode.");
            return Err(FsdifError::InvalidState);
        }
        info!("wait for PIO cmd to finish ...");
        self.reg.wait_for(|reg|{
            (FsdifRawInts::CMD_BIT & reg).bits() != 0
        }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100))?;
        /* if need to read data, read fifo after send command */
        if read {
            info!("wait for PIO data to read ...");
            if let Err(_)=self.reg.wait_for(|reg|{
                info!("raw ints reg: 0x{:x}",reg);
                (FsdifRawInts::DTO_BIT & reg).bits() != 0
            }, Duration::from_millis((FSDIF_TIMEOUT / 100).into()), Some(100)){
                self.raw_status_clear();
                return Err(FsdifError::CmdTimeout);
            }
            /* clear status to ack */
            self.raw_status_clear();
            info!("card cnt: 0x{:x}, fifo cnt: 0x{:x}",
            self.reg.read_reg::<FsdifTranCardCnt>(),
            self.reg.read_reg::<FsdifTranFifoCnt>());
        }
        /* clear status to ack cmd done */
        self.raw_status_clear();
        self.get_cmd_response(cmd_data)?;
        Ok(())
    }

    pub fn transfer(&self,cmd_data: &mut FSdifCmdData,retry: u32) -> FsdifResult {
        let mut retry = retry;
        loop {
            if let Err(_) = self.pio_transfer(cmd_data) {
                retry -= 1;
                continue;
            }
            if let Err(_) = self.poll_wait_pio_end(cmd_data) {
                /* if transfer data failed, send cmd12 to abort current transfer */
                if cmd_data.data.buf.len() > 0 {
                    /* when transfer error occur, polling card status until it is ready for next data transfer, otherwise the
                    * retry transfer will fail again */
                    let _ = self.stop_transmission();
                    warn!("pio transfer failed, polling card status until it is ready for next data transfer");
                    self.polling_card_status_busy(600)?;
                }
                if retry ==0 {
                    // todo 为选择timing之后进行操作
                }
                retry -= 1;
                continue;
            }
            break;
        }
        Ok(())
    }
}
pub struct FsdifBuf<'a> {
    pub buf: &'a mut [u32],
    pub buf_dma: u32,
    pub blksz: u32,
    pub blkcnt: u32,
}
pub struct FSdifCmdData<'a> {
    pub cmdidx: u32,
    pub cmdarg: u32,
    pub response: [u32; 4],
    pub flag: CmdFlag,
    pub data: FsdifBuf<'a>,
    pub success: bool
}

bitflags! {
    pub struct CmdFlag: u32 {
        const NEED_INIT = 0x1;
        const EXP_RESP = 0x2;
        const EXP_LONG_RESP = 0x4;
        const NEED_RESP_CRC = 0x8;
        const EXP_DATA = 0x10;
        const WRITE_DATA = 0x20;
        const READ_DATA = 0x40;
        const NEED_AUTO_STOP = 0x80;
        const ADTC = 0x100;
        const SWITCH_VOLTAGE = 0x200;
        const ABORT = 0x400;
        const AUTO_CMD12 = 0x800;
    }
}

//* CMD 相关的方法 */
impl MCI {
    const FSDIF_EXT_APP_CMD: u32 = 55;
    const FSDIF_SWITCH_VOLTAGE:u32 = 11;
    pub fn transfer_cmd(&self, cmd_data: &FSdifCmdData) -> FsdifResult {
        let mut raw_cmd = FsdifCmd::empty();
        if self.curr_timing.use_hold {
            raw_cmd |= FsdifCmd::USE_HOLD_REG;
        }
        if cmd_data.flag.contains(CmdFlag::ABORT) {
            raw_cmd |= FsdifCmd::STOP_ABORT;
        }
        /* 命令需要进行卡初始化，如CMD-0 */
        if cmd_data.flag.contains(CmdFlag::NEED_INIT) {
            raw_cmd |= FsdifCmd::INIT;
        }
        /* 命令涉及电压切换 */
        if cmd_data.flag.contains(CmdFlag::SWITCH_VOLTAGE) {
            raw_cmd |= FsdifCmd::VOLT_SWITCH;
        }
        /* 命令传输过程伴随数据传输 */
        if cmd_data.flag.contains(CmdFlag::EXP_DATA) {
            raw_cmd |= FsdifCmd::DAT_EXP;
            if cmd_data.flag.contains(CmdFlag::WRITE_DATA) {
                raw_cmd |= FsdifCmd::DAT_WRITE;
            }
        }
        /* 命令需要进行CRC校验 */
        if cmd_data.flag.contains(CmdFlag::NEED_RESP_CRC) {
            raw_cmd |= FsdifCmd::RESP_CRC;
        }
        /* 命令需要响应回复 */
        if cmd_data.flag.contains(CmdFlag::EXP_RESP) {
            raw_cmd |= FsdifCmd::RESP_EXP;
            if cmd_data.flag.contains(CmdFlag::EXP_LONG_RESP) {
                raw_cmd |= FsdifCmd::RESP_LONG;
            }
        }
        raw_cmd |= FsdifCmd::from_bits_truncate(set_reg32_bits!(cmd_data.cmdidx, 5, 0));
        debug!("============[{}-{}]@0x{:x} begin ============",
        {
            if self.prev_cmd == Self::FSDIF_EXT_APP_CMD {
                "ACMD"
            } else {
                "CMD"
            }
        },
        cmd_data.cmdidx,
        self.reg.addr.as_ptr() as usize );
        debug!("    cmd: 0x{:x}", raw_cmd.bits());
        debug!("    arg: 0x{:x}", cmd_data.cmdarg);
        /* enable related interrupt */
        self.interrupt_mask_set(FsDifIntrType::GeneralIntr, FsdifInt::INTS_CMD_MASK.bits(), true);
        self.send_private_cmd(raw_cmd, cmd_data.cmdarg);
        info!("cmd send done");
        Ok(())
    }
    pub fn get_cmd_response(&self,cmd_data: &mut FSdifCmdData) -> FsdifResult{
        let read = cmd_data.flag.contains(CmdFlag::READ_DATA);
        if !self.is_ready {
            error!("device is not yet initialized!!!");
            return Err(FsdifError::NotInit);
        }
        if read {
            if FsDifTransMode::PioTransMode == self.config.trans_mode {
                self.pio_read_data(cmd_data.data.buf)?;
            }
        }
        /* check response of cmd */
        if cmd_data.flag.contains(CmdFlag::EXP_RESP) {
            if cmd_data.flag.contains(CmdFlag::EXP_LONG_RESP) {
                cmd_data.response[0] = self.reg.read_reg::<FsdifResp0>().bits();
                cmd_data.response[1] = self.reg.read_reg::<FsdifResp1>().bits();
                cmd_data.response[2] = self.reg.read_reg::<FsdifResp2>().bits();
                cmd_data.response[3] = self.reg.read_reg::<FsdifResp3>().bits();
                debug!("    resp: 0x{:x}-0x{:x}-0x{:x}-0x{:x}",
                cmd_data.response[0],cmd_data.response[1],cmd_data.response[2],cmd_data.response[3]);
            }else {
                cmd_data.response[0] = self.reg.read_reg::<FsdifResp0>().bits();
                cmd_data.response[1] = 0;
                cmd_data.response[2] = 0;
                cmd_data.response[3] = 0;
                debug!("    resp: 0x{:x}",cmd_data.response[0]);
            }
        }
        cmd_data.success = true;
        debug!("============[{}-{}]@0x{:x} end ============",
        {
            if self.prev_cmd == Self::FSDIF_EXT_APP_CMD {
                "ACMD"
            } else {
                "CMD"
            }
        },
        cmd_data.cmdidx,
        self.reg.addr.as_ptr() as usize );
        /* disable related interrupt */
        self.interrupt_mask_set(FsDifIntrType::GeneralIntr,(FsdifInt::INTS_CMD_MASK|FsdifInt::INTS_DATA_MASK).bits(),false);
        self.interrupt_mask_set(FsDifIntrType::DmaIntr,FsdifDmacIntEn::INTS_MASK.bits(),false);
        info!("cmd send done ...");
        Ok(())
    }
}

//* Interrupt 相关的方法 */
impl MCI {
    pub fn interrupt_mask_get(&self, tp: FsDifIntrType) -> u32 {
        let mut mask = 0;
        if FsDifIntrType::GeneralIntr == tp {
            mask = self.reg.read_reg::<FsdifInt>().bits();
        } else if FsDifIntrType::DmaIntr == tp {
            mask = self.reg.read_reg::<FsdifDmacIntEn>().bits();
        }
        //? 这里不知道要不要用Some作为返回值 
        mask
    }

    pub fn interrupt_mask_set(&self, tp: FsDifIntrType, set_mask: u32, enable: bool) {
        let mut mask = self.interrupt_mask_get(tp);
        if enable {
            mask |= set_mask;
        } else {
            mask &= !set_mask;
        }
        if FsDifIntrType::GeneralIntr == tp {
            self.reg.write_reg(FsdifInt::from_bits_truncate(mask));
        } else if FsDifIntrType::DmaIntr == tp {
            self.reg.write_reg(FsdifDmacIntEn::from_bits_truncate(mask));
        }
    }
}

impl MCI {
    pub fn show_status(&self) {
        warn!("status: 0x{:x}", self.reg.read_reg::<FsdifStatus>());
    }
    pub fn dump_register(&self) {
        warn!("cntrl: 0x{:x}", self.reg.read_reg::<FsdifCtrl>());
        warn!("pwren: 0x{:x}", self.reg.read_reg::<FsdifPwrEn>());
        warn!("clkdiv: 0x{:x}", self.reg.read_reg::<FsdifClkDiv>());
        warn!("clkena: 0x{:x}", self.reg.read_reg::<FsdifClkEn>());
        warn!("tmout: 0x{:x}", self.reg.read_reg::<FsdifTimeout>());
        warn!("ctype: 0x{:x}", self.reg.read_reg::<FsdifCType>());
        warn!("blksz: 0x{:x}", self.reg.read_reg::<FsdifBlkSiz>());
        warn!("blkcnt: 0x{:x}", self.reg.read_reg::<FsdifBytCnt>());
        warn!("intmask: 0x{:x}", self.reg.read_reg::<FsdifInt>());
        warn!("cmdarg: 0x{:x}", self.reg.read_reg::<FsdifCmdArg>());
        warn!("cmd: 0x{:x}", self.reg.read_reg::<FsdifCmd>());
        warn!("resp0: 0x{:x}", self.reg.read_reg::<FsdifResp0>());
        warn!("reps1: 0x{:x}", self.reg.read_reg::<FsdifResp1>());
        warn!("resp2: 0x{:x}", self.reg.read_reg::<FsdifResp2>());
        warn!("resp3: 0x{:x}", self.reg.read_reg::<FsdifResp3>());
        warn!("maskints: 0x{:x}", self.reg.read_reg::<FsdifMaskedInts>());
        warn!("rawints: 0x{:x}", self.reg.read_reg::<FsdifRawInts>());
        warn!("status: 0x{:x}", self.reg.read_reg::<FsdifStatus>());
        warn!("fifoth: 0x{:x}", self.reg.read_reg::<FsdifFifoTh>());
        warn!("carddet: 0x{:x}", self.reg.read_reg::<FsdifCardDetect>());
        warn!("wrtprt: 0x{:x}", self.reg.read_reg::<FsdifCardWrtp>());
        warn!("cksts: 0x{:x}", self.reg.read_reg::<FsdifClkSts>());
        warn!("trans_cardcnt: 0x{:x}", self.reg.read_reg::<FsdifTranCardCnt>());
        warn!("trans_fifocnt: 0x{:x}", self.reg.read_reg::<FsdifTranFifoCnt>());
        warn!("debnce: 0x{:x}", self.reg.read_reg::<FsdifDebnce>());
        warn!("uid: 0x{:x}", self.reg.read_reg::<FsdifUid>());
        warn!("vid: 0x{:x}", self.reg.read_reg::<FsdifVid>());
        warn!("hwconf: 0x{:x}", self.reg.read_reg::<FsdifHwconf>());
        warn!("uhsreg: 0x{:x}", self.reg.read_reg::<FsdifUhsReg>());
        warn!("cardreset: 0x{:x}", self.reg.read_reg::<FsdifCardReset>());
        warn!("busmode: 0x{:x}", self.reg.read_reg::<FsdifBusMode>());
        warn!("descaddrl: 0x{:x}", self.reg.read_reg::<FsdifDescListAddrL>());
        warn!("descaddrh: 0x{:x}", self.reg.read_reg::<FsdifDescListAddrH>());
        warn!("dmacstatus: 0x{:x}", self.reg.read_reg::<FsdifDmacStatus>());
        warn!("dmacinten: 0x{:x}", self.reg.read_reg::<FsdifDmacIntEn>());
        warn!("curdescaddrl: 0x{:x}", self.reg.read_reg::<FsdifCurDescAddrL>());
        warn!("curdescaddrh: 0x{:x}", self.reg.read_reg::<FsdifCurDescAddrH>());
        warn!("curbufaddrl: 0x{:x}", self.reg.read_reg::<FsdifCurBufAddrL>());
        warn!("curbufaddrh: 0x{:x}", self.reg.read_reg::<FsdifCurBufAddrH>());
        warn!("card_thrctl: 0x{:x}", self.reg.read_reg::<FsdifCardThrctl>());
        warn!("clock_src: 0x{:x}", self.reg.read_reg::<FsdifClkSrc>());
        warn!("emmcddr: 0x{:x}", self.reg.read_reg::<FsdifEmmcDdrReg>());
        warn!("enableshift: 0x{:x}", self.reg.read_reg::<FsdifEnableShift>());

    }
}

