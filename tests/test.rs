#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bare_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::time::Duration;

use bare_test::{driver::device_tree::get_device_tree, mem::mmu::iomap, println, time::delay};
use log::*;
use phytium_mci::{iopad::PAD_ADDRESS, *};

bare_test::test_setup!();

#[test_case]
fn test_work() {
    let fdt = get_device_tree().unwrap();

    let mci0 = fdt.find_compatible(&["phytium,mci"]).next().unwrap();

    let reg = mci0.reg().unwrap().next().unwrap();

    info!("mci0 reg: {:#x},mci0 reg size: {:#x}", reg.address, reg.size.unwrap());

    let reg_base = iomap((reg.address as usize).into(), reg.size.unwrap());

    let mut mci0 = MCI::new(reg_base);

    let reg_base = iomap((PAD_ADDRESS as usize).into(), 0x2000);

    mci0.set_iopad(IoPad::new(reg_base));

    //? 初始化 MCI
    mci0.restart().unwrap_or_else(|e| error!("restart failed: {:?}", e));
    sleep(Duration::from_millis(1000));
    mci0.dump_register();
    mci0.reset().unwrap_or_else(|e| error!("reset failed: {:?}", e));
    sleep(Duration::from_millis(1000));
    mci0.dump_register();
    //* 设置为PIO模式 */
    mci0.set_pio_mode();

    //* 设置BusWidth为1Bit */
    let _ = mci0.bus_width_set(1);

    //* 设置时钟为400Khz */
    let _ = mci0.clk_freq_set(400000);

    mci0.dump_register();

    //* 检测当前情况 */
    let _ = mci0.prob_bus_voltage();

    let _ = mci0.all_send_cid();

    let _ = mci0.send_rca();

    let _ = mci0.send_csd();

    let _ = mci0.select_card(mci0.sd_reg.rca,true);

    //* 设置时钟为25Mhz,因为是Non-High Speed 的最大值 */
    let _ = mci0.clk_freq_set(25_000_000);

    let _ = mci0.send_scr();

    //* 设置BusWidth为4Bit */
    // todo 后边出问题源于这里出错
    let _ = mci0.data_bus_width_set(SdmmcBusWidth::BusWidth4Bit);
    let _ = mci0.bus_width_set(4);

    //* 获取卡状态 */
    let _ = mci0.read_status();

    //* 设置块大小为512 */
    let _ = mci0.block_size_set(512);

    //* 选择BusTimming */
    let _ = mci0.select_bus_timing();

    let _ = mci0.read_single_block();

    error!("test_work");


    assert!(true);
}

fn sleep(duration: Duration) {
    spin_on::spin_on(delay(duration));
}

struct KernelImpl;

impl Kernel for KernelImpl {
    fn sleep(duration: Duration) {
        sleep(duration);
    }
}

set_impl!(KernelImpl);
