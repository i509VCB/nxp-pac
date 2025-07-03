#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg_attr(feature = "mimxrt1011", path = "./chips/mimxrt1011/pac.rs")]
#[cfg_attr(feature = "mimxrt1062", path = "./chips/mimxrt1062/pac.rs")]
mod pac;
pub use pac::*;
