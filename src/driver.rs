use super::hw_interface::Bus;
use super::spec::{
    CS47L63_ASP1_CONTROL1, CS47L63_ASP1_CONTROL2, CS47L63_ASP1_ENABLES1,
    CS47L63_BOOT_DONE_EINT1_MASK, CS47L63_BOOT_DONE_MASK1_MASK, CS47L63_CLOCK32K,
    CS47L63_CTRLIF_ERR_MASK1_MASK, CS47L63_DEVID, CS47L63_DSP1_AHB_PACK_ERR_MASK1_MASK,
    CS47L63_DSP1_AHB_SYS_ERR_MASK1_MASK, CS47L63_DSP1_IRQ0_MASK1_MASK,
    CS47L63_DSP1_MPU_ERR_MASK1_MASK, CS47L63_DSP1_WDT_EXPIRE_STS1_MASK, CS47L63_DSP_CLOCK1,
    CS47L63_FLL1_CONTROL1, CS47L63_FLL1_CONTROL2, CS47L63_FLL1_CONTROL3, CS47L63_FLL1_CONTROL4,
    CS47L63_GPIO1_CTRL1, CS47L63_GPIO2_CTRL1, CS47L63_GPIO3_CTRL1, CS47L63_GPIO4_CTRL1,
    CS47L63_IRQ1_EINT_1, CS47L63_IRQ1_EINT_2, CS47L63_IRQ1_MASK_1, CS47L63_MCU_CTRL1,
    CS47L63_OTPID, CS47L63_REVID, CS47L63_SAMPLE_RATE1, CS47L63_SYSCLK_ERR_MASK1_MASK,
    CS47L63_SYSCLK_FAIL_MASK1_MASK, CS47L63_SYSTEM_CLOCK1,
};
use log::{error, info};

// flags passed to Notification Callback to notify BSP of specific driver events
const CS47L63_EVENT_FLAG_BOOT_DONE: u32 = 1 << 0;
const CS47L63_EVENT_FLAG_SYSCLK_FAIL: u32 = 1 << 1;
const CS47L63_EVENT_FLAG_SYSCLK_ERR: u32 = 1 << 2;
const CS47L63_EVENT_FLAG_CTRLIF_ERR: u32 = 1 << 3;
const CS47L63_EVENT_FLAG_MPU_ERR: u32 = 1 << 4;
const CS47L63_EVENT_FLAG_DSP1_IRQ0: u32 = 1 << 5;
const CS47L63_EVENT_FLAG_WDT_EXPIRE: u32 = 1 << 6;
const CS47L63_EVENT_FLAG_AHB_SYS_ERR: u32 = 1 << 7;
const CS47L63_EVENT_FLAG_AHB_PACK_ERR: u32 = 1 << 8;

struct IrqReg {
    irq_reg_offset: u32,
    mask: u32,
    event_flag: u32,
}

const EVENT_DATA: [IrqReg; 9] = [
    IrqReg {
        irq_reg_offset: 0x4,
        mask: CS47L63_BOOT_DONE_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_BOOT_DONE,
    }, // CS47L63_IRQ1_EINT_2
    IrqReg {
        irq_reg_offset: 0x0,
        mask: CS47L63_SYSCLK_FAIL_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_SYSCLK_FAIL,
    }, // CS47L63_IRQ1_EINT_1
    IrqReg {
        irq_reg_offset: 0x0,
        mask: CS47L63_SYSCLK_ERR_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_SYSCLK_ERR,
    }, // CS47L63_IRQ1_EINT_1
    IrqReg {
        irq_reg_offset: 0x0,
        mask: CS47L63_CTRLIF_ERR_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_CTRLIF_ERR,
    }, // CS47L63_IRQ1_EINT_1
    IrqReg {
        irq_reg_offset: 0x18,
        mask: CS47L63_DSP1_MPU_ERR_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_MPU_ERR,
    }, // CS47L63_IRQ1_EINT_7
    IrqReg {
        irq_reg_offset: 0x20,
        mask: CS47L63_DSP1_IRQ0_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_DSP1_IRQ0,
    }, // CS47L63_IRQ1_EINT_9
    IrqReg {
        irq_reg_offset: 0x18,
        mask: CS47L63_DSP1_WDT_EXPIRE_STS1_MASK,
        event_flag: CS47L63_EVENT_FLAG_WDT_EXPIRE,
    }, // CS47L63_IRQ1_EINT_7
    IrqReg {
        irq_reg_offset: 0x18,
        mask: CS47L63_DSP1_AHB_SYS_ERR_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_AHB_SYS_ERR,
    }, // CS47L63_IRQ1_EINT_7
    IrqReg {
        irq_reg_offset: 0x18,
        mask: CS47L63_DSP1_AHB_PACK_ERR_MASK1_MASK,
        event_flag: CS47L63_EVENT_FLAG_AHB_PACK_ERR,
    }, // CS47L63_IRQ1_EINT_7
];

// register values to be applied after CS47L63 Driver boot
#[rustfmt::skip]
const SYSCFG_REGS: [[u32; 2]; 15] = [
    [CS47L63_CLOCK32K, 0x0041],
    [CS47L63_FLL1_CONTROL2, 0x28601177],
    [CS47L63_FLL1_CONTROL3, 0x10000],
    [CS47L63_FLL1_CONTROL4, 0x23f05004],
    [CS47L63_FLL1_CONTROL1, 0x0006],
    [CS47L63_SYSTEM_CLOCK1, 0x0404],
    [CS47L63_SAMPLE_RATE1, 0x0003],
    [CS47L63_DSP_CLOCK1, 0x25800004],
    [CS47L63_GPIO1_CTRL1, 0xe1000000],
    [CS47L63_GPIO2_CTRL1, 0xe1000000],
    [CS47L63_GPIO3_CTRL1, 0xe1000000],
    [CS47L63_GPIO4_CTRL1, 0xe1000000],
    [CS47L63_ASP1_CONTROL1, 0x0033],
    [CS47L63_ASP1_CONTROL2, 0x20200200],
    [CS47L63_ASP1_ENABLES1, 0x30003],
];

/// Reset the CS47L63 to get it in a state where it can be used. Device cannot be used without a reset being called.
///
/// Assuming that the reset line has been toggled, this function does the following:
/// - waits for device to boot after reset
/// - reads the device id and revision id
/// - applies required patches
/// - writes default config data
/// - unmasks interrupts
pub async fn reset<E>(bus: &mut impl Bus<E>) -> Result<(), E> {
    // NOTE: this function assumes that the user has already driven the hw_codec_reset_out pin low then waited for 4 ms then high

    // wait for IRQ1 register to return no errors
    loop {
        bus.delay_ms(10).await;
        let result = bus.read(CS47L63_IRQ1_EINT_2).await?;
        info!("Read CS47L63_IRQ1_EINT_2: 0x{:X}", result);
        if result & CS47L63_BOOT_DONE_EINT1_MASK != 0 {
            break;
        }
    }

    info!("Codec reset success");

    // check the device id
    let device_id = bus.read(CS47L63_DEVID).await?;
    pub const CS47L63_DEVID_VAL: u32 = 0x47A63; // expected device id
    if device_id != CS47L63_DEVID_VAL {
        error!(
            "Invalid CS47L63 device id: 0x{:X} expected: 0x{:X}",
            device_id, CS47L63_DEVID_VAL
        );
        // TODO: return error
        let rev_id = bus.read(CS47L63_REVID).await?;
        info!(
            "Found CS47L63 Device Id: 0x{:X}, Rev: 0x{:X}",
            device_id, rev_id
        );
    } else {
        let rev_id = bus.read(CS47L63_REVID).await?;
        info!(
            "Found CS47L63 Device Id: 0x{:X}, Rev: 0x{:X}",
            device_id, rev_id
        );
    }

    // patch the codec
    apply_patch(bus).await?;

    // write configuration data
    write_array(bus, &SYSCFG_REGS).await?;

    // unmask interrupts
    // omit first mask register, as BOOT_DONE_EINT1 is enabled by default
    for event in EVENT_DATA.iter().skip(1) {
        let addr = CS47L63_IRQ1_MASK_1 + event.irq_reg_offset;
        update_reg(bus, addr, event.mask, 0).await?;
    }

    Ok(())
}

/// Should be called every time the IRQ pin goes high in order to handle events
///
/// Resets interrupts and returns event_flags for whatever events fired
pub async fn event_handler<E>(bus: &mut impl Bus<E>) -> Result<u32, E> {
    let mut old_reg = 0;
    let mut value = 0;
    let mut event_flags = 0;
    for event in EVENT_DATA.iter() {
        let new_reg = CS47L63_IRQ1_EINT_1 + event.irq_reg_offset;
        if new_reg != old_reg {
            value = bus.read(new_reg).await?;
        }
        old_reg = new_reg;
        if value & event.mask != 0 {
            event_flags |= event.event_flag;
            bus.write(new_reg, event.mask).await?;
        }
    }

    Ok(event_flags)
}

async fn update_reg<E>(bus: &mut impl Bus<E>, addr: u32, mask: u32, val: u32) -> Result<(), E> {
    let data = bus.read(addr).await?;
    let temp_val = (data & !mask) | val;
    if data == temp_val {
        // nothing to do
        return Ok(());
    }

    bus.write(addr, temp_val).await?;
    Ok(())
}

async fn write_array<E>(bus: &mut impl Bus<E>, config: &[[u32; 2]]) -> Result<(), E> {
    for [reg, value] in config {
        bus.write(*reg, *value).await?;
    }

    Ok(())
}

async fn apply_patch<E>(bus: &mut impl Bus<E>) -> Result<(), E> {
    let otpid = bus.read(CS47L63_OTPID).await?;
    info!("Apply patch for otpid {}", otpid);

    match otpid {
        0 => return Ok(()),
        8 => {
            otpid_8_patch(bus).await?;
        }
        _ => {}
    }

    common_patch(bus).await?;
    Ok(())
}

async fn otpid_8_patch<E>(bus: &mut impl Bus<E>) -> Result<(), E> {
    info!("Writing cs47l63_otpid_8_patch");
    bus.write(0x0030, 0x0055).await?;
    bus.write(0x0030, 0x00aa).await?;
    bus.write(0x0034, 0x0055).await?;
    bus.write(0x0034, 0x00aa).await?;
    bus.write(0x4d68, 0x1db10000).await?;
    bus.write(0x4d70, 0x700249b8).await?;
    bus.write(0x24ac, 0x10000).await?;
    bus.write(0x24b4, 0x05ff).await?;
    bus.write(0x2420, 0x4150415).await?;
    bus.write(0x2424, 0x0415).await?;
    bus.write(0x0030, 0x00cc).await?;
    bus.write(0x0030, 0x0033).await?;
    bus.write(0x0034, 0x00cc).await?;
    bus.write(0x0034, 0x0033).await?;
    Ok(())
}

async fn common_patch<E>(bus: &mut impl Bus<E>) -> Result<(), E> {
    info!("Writing cs47l63_common_patch");
    bus.write(0x0808, 0x0002).await?;
    let mut iter_timeout = 0;

    // wait for value at CS47L63_MCU_CTRL1 to change to something expected
    loop {
        bus.delay_ms(5).await;
        let temp_reg_val = bus.read(CS47L63_MCU_CTRL1).await?;

        iter_timeout += 1;
        if iter_timeout > 20 {
            error!("Timeout expired when reading CS47L63_MCU_CTRL1");
            // TODO: return error here
            return Ok(());
        }

        if temp_reg_val & 0x2 == 0x2 {
            break;
        }
    }

    bus.write(0x0808, 0x0003).await?;
    bus.write_block(0x410ac, &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
        .await?;
    bus.write_block(
        0x4c8a0,
        &[
            0x00, 0x4D, 0x68, 0x0B, 0x69, 0x0B, 0x9F, 0x00, 0x42, 0x00, 0x00, 0x4D, 0x00, 0x4D,
            0x69, 0x0B, 0x38, 0x0F, 0x40, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x78, 0x08, 0x0F, 0x00,
            0x00, 0x00, 0x00, 0x26, 0x88, 0x10, 0x00, 0x0E, 0x00, 0x01, 0x00, 0x01,
        ],
    )
    .await?;
    bus.write_block(
        0x4c8d0,
        &[
            0x4D, 0x68, 0x38, 0x0F, 0x0F, 0x80, 0x00, 0x00, 0x00, 0x4D, 0x69, 0x04, 0x28, 0x0F,
            0x02, 0x00, 0x00, 0x00, 0x4D, 0x69, 0x68, 0x00, 0x0F, 0x20, 0x00, 0x00, 0x00, 0x4D,
            0x4D, 0x68, 0x08, 0x0F, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x20, 0x08, 0x0E,
            0x00, 0x00, 0x00, 0x00, 0x26, 0x78, 0x00, 0x00, 0x00, 0x02,
        ],
    )
    .await?;
    bus.write_block(
        0x4c910,
        &[
            0x48, 0x14, 0x10, 0x1F, 0x01, 0x04, 0x00, 0x00, 0x4C, 0xA4, 0x08, 0x0F, 0x1F, 0x02,
            0x00, 0x00, 0x00, 0x4D, 0x68, 0x38, 0x1F, 0x01, 0x80, 0x00, 0x00, 0x4D, 0x69, 0x04,
            0x1F, 0x01, 0x02, 0x00, 0x00, 0x4D, 0x69, 0x28, 0x0F, 0x01, 0x20, 0x00, 0x00, 0x4D,
            0x78, 0x10, 0x30, 0x0F, 0x04, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x73, 0x08, 0x0F, 0x40,
            0x02, 0x00, 0x00, 0x4D, 0x4D, 0x68, 0x18, 0x1F, 0x01, 0x00, 0x00, 0x00, 0x4D, 0x78,
            0x08, 0x0F, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x69, 0x04, 0x1F, 0x01, 0x00, 0x00,
            0x00, 0x4D, 0x68, 0x00, 0x1F, 0x01, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x08, 0x1F, 0x01,
            0x00, 0x00, 0x00, 0x4D, 0x6A, 0x08, 0x1F, 0x01, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x20,
            0x1F, 0x01, 0x00, 0x00, 0x00, 0x48, 0x14, 0x00, 0x1F, 0x01, 0x00, 0x00, 0x00, 0x4D,
            0x68, 0x10, 0x1F, 0x02, 0x00, 0x00, 0x00, 0x4D, 0x78, 0x00, 0x1F, 0x01, 0x01, 0x00,
            0x00, 0x4D, 0x78, 0x10, 0x1F, 0x01, 0x00, 0x00, 0x00, 0x4D, 0x68, 0x38, 0x0F, 0x01,
            0x00, 0x00, 0x00, 0x4D, 0x69, 0x30, 0x10, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x48, 0x14,
            0xA4, 0x08, 0x0F, 0x00, 0x00, 0x00, 0x00, 0x4C, 0x26, 0x78, 0x08, 0x1F, 0x01, 0x02,
            0x00, 0x00, 0x26, 0x78, 0x00, 0x0F, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x24, 0xAD, 0x00,
            0x0F, 0x01, 0x01, 0x00, 0x01, 0x88, 0x10, 0x08, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x01,
            0x88, 0x10, 0x24, 0x31, 0x0A, 0x00, 0x00, 0x80, 0x00, 0x00,
        ],
    )
    .await?;
    bus.write_block(
        0x4108c,
        &[
            0x49, 0x00, 0x40, 0x2F, 0x48, 0xA0, 0x48, 0x10, 0x00, 0xAE, 0x00, 0xD0,
        ],
    )
    .await?;
    bus.write(0x0808, 0x0002).await?;
    bus.write(0x0808, 0x0000).await?;
    Ok(())
}
