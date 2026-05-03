#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Repeat interrupt mode."]
    RepeatInterruptMode = 0x0,
    #[doc = "One-shot interrupt mode."]
    OneShotInterruptMode = 0x01,
    #[doc = "One-shot stall mode."]
    OneShotStallMode = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Multitask {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HardwareStatusMode = 0x0,
    #[doc = "Multi-task mode."]
    MultiTaskMode = 0x01,
}
impl Multitask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Multitask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Multitask {
    #[inline(always)]
    fn from(val: u8) -> Multitask {
        Multitask::from_bits(val)
    }
}
impl From<Multitask> for u8 {
    #[inline(always)]
    fn from(val: Multitask) -> u8 {
        Multitask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "Idle state. TIMERn is stopped."]
    IdleState = 0x0,
    #[doc = "Running. TIMERn is running."]
    Running = 0x01,
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
