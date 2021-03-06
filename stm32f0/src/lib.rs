//! Peripheral access API for STM32F0 microcontrollers
//! (generated using [svd2rust] 0.13.1)
//! [svd2rust]: https://github.com/japaric/svd2rust
//!
//! You can find an overview of the API here:
//! https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api
//!
//! For more details see the README here:
//! https://github.com/adamgreig/stm32-rs

#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]

#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]

extern crate vcell;
extern crate bare_metal;
extern crate cortex_m;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

#[cfg(feature = "stm32f0x0")]
pub mod stm32f0x0;

#[cfg(feature = "stm32f0x1")]
pub mod stm32f0x1;

#[cfg(feature = "stm32f0x2")]
pub mod stm32f0x2;

#[cfg(feature = "stm32f0x8")]
pub mod stm32f0x8;

