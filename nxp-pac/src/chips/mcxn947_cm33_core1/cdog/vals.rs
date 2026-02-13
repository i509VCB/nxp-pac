#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddressCtrl {
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
impl AddressCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddressCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddressCtrl {
    #[inline(always)]
    fn from(val: u8) -> AddressCtrl {
        AddressCtrl::from_bits(val)
    }
}
impl From<AddressCtrl> for u8 {
    #[inline(always)]
    fn from(val: AddressCtrl) -> u8 {
        AddressCtrl::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MiscompareCtrl {
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
impl MiscompareCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MiscompareCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MiscompareCtrl {
    #[inline(always)]
    fn from(val: u8) -> MiscompareCtrl {
        MiscompareCtrl::from_bits(val)
    }
}
impl From<MiscompareCtrl> for u8 {
    #[inline(always)]
    fn from(val: MiscompareCtrl) -> u8 {
        MiscompareCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SequenceCtrl {
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
impl SequenceCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SequenceCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SequenceCtrl {
    #[inline(always)]
    fn from(val: u8) -> SequenceCtrl {
        SequenceCtrl::from_bits(val)
    }
}
impl From<SequenceCtrl> for u8 {
    #[inline(always)]
    fn from(val: SequenceCtrl) -> u8 {
        SequenceCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateCtrl {
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
impl StateCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StateCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StateCtrl {
    #[inline(always)]
    fn from(val: u8) -> StateCtrl {
        StateCtrl::from_bits(val)
    }
}
impl From<StateCtrl> for u8 {
    #[inline(always)]
    fn from(val: StateCtrl) -> u8 {
        StateCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimeoutCtrl {
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
impl TimeoutCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimeoutCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimeoutCtrl {
    #[inline(always)]
    fn from(val: u8) -> TimeoutCtrl {
        TimeoutCtrl::from_bits(val)
    }
}
impl From<TimeoutCtrl> for u8 {
    #[inline(always)]
    fn from(val: TimeoutCtrl) -> u8 {
        TimeoutCtrl::to_bits(val)
    }
}
