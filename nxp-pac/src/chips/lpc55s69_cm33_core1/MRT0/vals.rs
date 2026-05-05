#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODE {
    #[doc = "Repeat interrupt mode."]
    REPEAT_INTERRUPT_MODE = 0x0,
    #[doc = "One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_MODE = 0x01,
    #[doc = "One-shot stall mode."]
    ONE_SHOT_STALL_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODE {
    #[inline(always)]
    fn from(val: u8) -> MODE {
        MODE::from_bits(val)
    }
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(val: MODE) -> u8 {
        MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MULTITASK {
    #[doc = "Hardware status mode. In this mode, the INUSE(n) flags for all channels are reset."]
    HARDWARE_STATUS_MODE = 0x0,
    #[doc = "Multi-task mode."]
    MULTI_TASK_MODE = 0x01,
}
impl MULTITASK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MULTITASK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MULTITASK {
    #[inline(always)]
    fn from(val: u8) -> MULTITASK {
        MULTITASK::from_bits(val)
    }
}
impl From<MULTITASK> for u8 {
    #[inline(always)]
    fn from(val: MULTITASK) -> u8 {
        MULTITASK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RUN {
    #[doc = "Idle state. TIMERn is stopped."]
    IDLE_STATE = 0x0,
    #[doc = "Running. TIMERn is running."]
    RUNNING = 0x01,
}
impl RUN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RUN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RUN {
    #[inline(always)]
    fn from(val: u8) -> RUN {
        RUN::from_bits(val)
    }
}
impl From<RUN> for u8 {
    #[inline(always)]
    fn from(val: RUN) -> u8 {
        RUN::to_bits(val)
    }
}
