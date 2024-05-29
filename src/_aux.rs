//! aux.rs
//!
//! Module for using the UART, and other aux peripherals

#![allow(dead_code)]

use base;
use gpio;

pub const AUX_BASE: u32 = base::PERIPHERAL_BASE + 0x215000;

pub const AUX: *const u32 = AUX_BASE as *const u32;

pub const SYS_FREQ: u32 = 250000000;

// Bit values for various bytes
pub const AUX_ENA_MINIUART: u32 = (1 << 0);
pub const AUX_ENA_SPI1: u32 = (1 << 1);
pub const AUX_ENA_SPI2: u32 = (1 << 2);

pub const AUX_IRQ_SPI2: u32 = (1 << 2);
pub const AUX_IRQ_SPI1: u32 = (1 << 1);
pub const AUX_IRQ_MU: u32 = (1 << 0);

pub const AUX_MULCR_8BIT_MODE: u32 = (3 << 0); /* See errata for this value */
pub const AUX_MULCR_BREAK: u32 = (1 << 6);
pub const AUX_MULCR_DLAB_ACCESS: u32 = (1 << 7);

pub const AUX_MUMCR_RTS: u32 = (1 << 1);

pub const AUX_MULSR_DATA_READY: u32 = (1 << 0);
pub const AUX_MULSR_RX_OVERRUN: u32 = (1 << 1);
pub const AUX_MULSR_TX_EMPTY: u32 = (1 << 5);
pub const AUX_MULSR_TX_IDLE: u32 = (1 << 6);

pub const AUX_MUMSR_CTS: u32 = (1 << 5);

pub const AUX_MUCNTL_RX_ENABLE: u32 = (1 << 0);
pub const AUX_MUCNTL_TX_ENABLE: u32 = (1 << 1);
pub const AUX_MUCNTL_RTS_FLOW: u32 = (1 << 2);
pub const AUX_MUCNTL_CTS_FLOW: u32 = (1 << 3);
pub const AUX_MUCNTL_RTS_FIFO: u32 = (3 << 4);
pub const AUX_MUCNTL_RTS_ASSERT: u32 = (1 << 6);
pub const AUX_MUCNTL_CTS_ASSERT: u32 = (1 << 7);

pub const AUX_MUSTAT_SYMBOL_AV: u32 = (1 << 0);
pub const AUX_MUSTAT_SPACE_AV: u32 = (1 << 1);
pub const AUX_MUSTAT_RX_IDLE: u32 = (1 << 2);
pub const AUX_MUSTAT_TX_IDLE: u32 = (1 << 3);
pub const AUX_MUSTAT_RX_OVERRUN: u32 = (1 << 4);
pub const AUX_MUSTAT_TX_FIFO_FULL: u32 = (1 << 5);
pub const AUX_MUSTAT_RTS: u32 = (1 << 6);
pub const AUX_MUSTAT_CTS: u32 = (1 << 7);
pub const AUX_MUSTAT_TX_EMPTY: u32 = (1 << 8);
pub const AUX_MUSTAT_TX_DONE: u32 = (1 << 9);
pub const AUX_MUSTAT_RX_FIFO_LEVEL: u32 = (7 << 16);
pub const AUX_MUSTAT_TX_FIFO_LEVEL: u32 = (7 << 24);

// Byte offsets for registers
pub const IRQ: isize = 0;
pub const ENABLES: isize = 1;

pub const MU_IO: isize = 16;
pub const MU_IER: isize = 17;
pub const MU_IIR: isize = 18;
pub const MU_LCR: isize = 19;
pub const MU_MCR: isize = 20;
pub const MU_LSR: isize = 21;
pub const MU_MSR: isize = 22;
pub const MU_SCRATCH: isize = 23;
pub const MU_CNTL: isize = 24;
pub const MU_STAT: isize = 25;
pub const MU_BUAD: isize = 26;

pub const SPI0_CNTL0: isize = 32;
pub const SPI0_CNTL1: isize = 33;
pub const SPI0_STAT: isize = 34;
pub const SPI0_IO: isize = 35;
pub const SPI0_PEEK: isize = 36;

pub const SPI1_CNTL0: isize = 48;
pub const SPI1_CNTL1: isize = 49;
pub const SPI1_STAT: isize = 50;
pub const SPI1_IO: isize = 51;
pub const SPI1_PEEK: isize = 52;

pub fn init_uart(baud: u32, bits: u32) {
    unsafe {

        *(base::get_reg(AUX, ENABLES)) = AUX_ENA_MINIUART;

        *(base::get_reg(AUX, MU_IER)) = 0;

        *(base::get_reg(AUX, MU_CNTL)) = 0;

        if bits == 8 {
            *(base::get_reg(AUX, MU_LCR)) = AUX_MULCR_8BIT_MODE;
        } else {
            *(base::get_reg(AUX, MU_LCR)) = 0;
        }

        *(base::get_reg(AUX, MU_MCR)) = 0;

        *(base::get_reg(AUX, MU_IIR)) = 0xC6;

        *(base::get_reg(AUX, MU_BUAD)) = (SYS_FREQ / (8 * baud)) - 1;

        gpio::set_mode(14, gpio::Modes::Alt5);
        gpio::set_mode(15, gpio::Modes::Alt5);

        gpio::set_pull_mode(14, gpio::PullModes::Off);

        *(base::get_reg(AUX, MU_CNTL)) = AUX_MUCNTL_TX_ENABLE;

    }
}

pub fn write_uart(byte: u8) {
    unsafe {

        while (*(base::get_reg(AUX, MU_LSR)) & AUX_MULSR_TX_EMPTY) == 0 {
            base::nothing();
        }

        *(base::get_reg(AUX, MU_IO)) |= byte as u32;

    }
}
