#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

// IMXRT
#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/mod.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/mod.rs")]
// MCX
#[cfg_attr(
    feature = "mcxn947_cm33_core0",
    path = "./chips/mcxn947_cm33_core0/mod.rs"
)]
#[cfg_attr(
    feature = "mcxn947_cm33_core1",
    path = "./chips/mcxn947_cm33_core1/mod.rs"
)]
mod pac;
pub use pac::*;

#[cfg(feature = "metadata")]
pub mod metadata;
