#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl {
    _RESERVED_0 = 0x0,
    #[doc = "Enable reset"]
    ENABLE_RESET = 0x01,
    #[doc = "Enable interrupt"]
    ENABLE_INTERRUPT = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Disable both reset and interrupt"]
    DISABLE_BOTH = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl {
    #[inline(always)]
    fn from(val: u8) -> Ctrl {
        Ctrl::from_bits(val)
    }
}
impl From<Ctrl> for u8 {
    #[inline(always)]
    fn from(val: Ctrl) -> u8 {
        Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugHaltCtrl {
    _RESERVED_0 = 0x0,
    #[doc = "Keep the timer running"]
    RUN_TIMER = 0x01,
    #[doc = "Stop the timer"]
    PAUSE_TIMER = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugHaltCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugHaltCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugHaltCtrl {
    #[inline(always)]
    fn from(val: u8) -> DebugHaltCtrl {
        DebugHaltCtrl::from_bits(val)
    }
}
impl From<DebugHaltCtrl> for u8 {
    #[inline(always)]
    fn from(val: DebugHaltCtrl) -> u8 {
        DebugHaltCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqPause {
    _RESERVED_0 = 0x0,
    #[doc = "Keep the timer running"]
    RUN_TIMER = 0x01,
    #[doc = "Stop the timer"]
    PAUSE_TIMER = 0x02,
    _RESERVED_3 = 0x03,
}
impl IrqPause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqPause {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqPause {
    #[inline(always)]
    fn from(val: u8) -> IrqPause {
        IrqPause::from_bits(val)
    }
}
impl From<IrqPause> for u8 {
    #[inline(always)]
    fn from(val: IrqPause) -> u8 {
        IrqPause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockCtrl {
    _RESERVED_0 = 0x0,
    #[doc = "Locked"]
    LOCKED = 0x01,
    #[doc = "Unlocked"]
    UNLOCKED = 0x02,
    _RESERVED_3 = 0x03,
}
impl LockCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockCtrl {
    #[inline(always)]
    fn from(val: u8) -> LockCtrl {
        LockCtrl::from_bits(val)
    }
}
impl From<LockCtrl> for u8 {
    #[inline(always)]
    fn from(val: LockCtrl) -> u8 {
        LockCtrl::to_bits(val)
    }
}
