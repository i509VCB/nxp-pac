#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "pac")]
// IMXRT
#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/mod.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/mod.rs")]
#[cfg_attr(feature = "mimxrt1064", path = "./chips/mimxrt1064/mod.rs")]
#[cfg_attr(feature = "mimxrt685s_cm33", path = "./chips/mimxrt685s_cm33/mod.rs")]
// MCX
#[cfg_attr(
    feature = "mcxn947_cm33_core0",
    path = "./chips/mcxn947_cm33_core0/mod.rs"
)]
#[cfg_attr(
    feature = "mcxn947_cm33_core1",
    path = "./chips/mcxn947_cm33_core1/mod.rs"
)]
#[cfg_attr(feature = "mcxa156", path = "./chips/mcxa156/mod.rs")]
#[cfg_attr(feature = "mcxa256", path = "./chips/mcxa256/mod.rs")]
#[cfg_attr(feature = "mcxa577", path = "./chips/mcxa577/mod.rs")]
// LPC55S16
#[cfg_attr(feature = "lpc55s16", path = "./chips/lpc55s16/mod.rs")]
// LPC55S69
#[cfg_attr(
    feature = "lpc55s69_cm33_core0",
    path = "./chips/lpc55s69_cm33_core0/mod.rs"
)]
#[cfg_attr(
    feature = "lpc55s69_cm33_core1",
    path = "./chips/lpc55s69_cm33_core1/mod.rs"
)]
mod pac;
#[cfg(feature = "pac")]
pub use pac::*;

#[cfg(feature = "metadata")]
pub mod metadata;
