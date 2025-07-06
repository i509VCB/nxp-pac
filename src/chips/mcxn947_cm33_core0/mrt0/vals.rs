#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gflag0 {
    #[doc = "No pending interrupt."]
    NO_PENDING_INTERRUPT = 0x0,
    #[doc = "Pending interrupt"]
    PENDING_INTERRUPT = 0x01,
}
impl Gflag0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gflag0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gflag0 {
    #[inline(always)]
    fn from(val: u8) -> Gflag0 {
        Gflag0::from_bits(val)
    }
}
impl From<Gflag0> for u8 {
    #[inline(always)]
    fn from(val: Gflag0) -> u8 {
        Gflag0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inten {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Inten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inten {
    #[inline(always)]
    fn from(val: u8) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u8 {
    #[inline(always)]
    fn from(val: Inten) -> u8 {
        Inten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intflag {
    #[doc = "No pending interrupt."]
    NO_PENDING_INTERRUPT = 0x0,
    #[doc = "Pending interrupt."]
    PENDING_INTERRUPT = 0x01,
}
impl Intflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intflag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intflag {
    #[inline(always)]
    fn from(val: u8) -> Intflag {
        Intflag::from_bits(val)
    }
}
impl From<Intflag> for u8 {
    #[inline(always)]
    fn from(val: Intflag) -> u8 {
        Intflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Inuse {
    #[doc = "This timer channel is not in use."]
    NO = 0x0,
    #[doc = "This timer channel is in use."]
    YES = 0x01,
}
impl Inuse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inuse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inuse {
    #[inline(always)]
    fn from(val: u8) -> Inuse {
        Inuse::from_bits(val)
    }
}
impl From<Inuse> for u8 {
    #[inline(always)]
    fn from(val: Inuse) -> u8 {
        Inuse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Load {
    #[doc = "No force load"]
    NO_FORCE_LOAD = 0x0,
    #[doc = "Force load"]
    FORCE_LOAD = 0x01,
}
impl Load {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Load {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Load {
    #[inline(always)]
    fn from(val: u8) -> Load {
        Load::from_bits(val)
    }
}
impl From<Load> for u8 {
    #[inline(always)]
    fn from(val: Load) -> u8 {
        Load::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "Repeat Interrupt mode"]
    REPEAT_INTERRUPT_MODE = 0x0,
    #[doc = "One-Shot Interrupt mode"]
    ONE_SHOT_INTERRUPT_MODE = 0x01,
    #[doc = "One-Shot Stall mode"]
    ONE_SHOT_STALL_MODE = 0x02,
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
    #[doc = "Hardware status mode."]
    HARDWARE_STATUS_MODE = 0x0,
    #[doc = "Multitask mode"]
    MULTI_TASK_MODE = 0x01,
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
    #[doc = "Idle state."]
    IDLE_STATE = 0x0,
    #[doc = "Running."]
    RUNNING = 0x01,
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
