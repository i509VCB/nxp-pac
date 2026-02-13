#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "The sequence is not running."]
    SEQ_NOT_RUNNING = 0x0,
    #[doc = "The sequence is running."]
    SEQ_RUNNING = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bc12 {
    #[doc = "Compatible with BC1.1 (default)"]
    BC11 = 0x0,
    #[doc = "Compatible with BC1.2"]
    BC12 = 0x01,
}
impl Bc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bc12 {
    #[inline(always)]
    fn from(val: u8) -> Bc12 {
        Bc12::from_bits(val)
    }
}
impl From<Bc12> for u8 {
    #[inline(always)]
    fn from(val: Bc12) -> u8 {
        Bc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockUnit {
    #[doc = "kHz Speed (between 1 kHz and 1023 kHz)"]
    KHZ_CLK = 0x0,
    #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
    MHZ_CLK = 0x01,
}
impl ClockUnit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockUnit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockUnit {
    #[inline(always)]
    fn from(val: u8) -> ClockUnit {
        ClockUnit::from_bits(val)
    }
}
impl From<ClockUnit> for u8 {
    #[inline(always)]
    fn from(val: ClockUnit) -> u8 {
        ClockUnit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iack {
    #[doc = "Do not clear the interrupt."]
    INT_NOCLEAR = 0x0,
    #[doc = "Clear the IF bit (interrupt flag)."]
    INT_CLEAR = 0x01,
}
impl Iack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iack {
    #[inline(always)]
    fn from(val: u8) -> Iack {
        Iack::from_bits(val)
    }
}
impl From<Iack> for u8 {
    #[inline(always)]
    fn from(val: Iack) -> u8 {
        Iack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie {
    #[doc = "Disable interrupts to the system."]
    DIS_INT = 0x0,
    #[doc = "Enable interrupts to the system."]
    EN_INT = 0x01,
}
impl Ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie {
    #[inline(always)]
    fn from(val: u8) -> Ie {
        Ie::from_bits(val)
    }
}
impl From<Ie> for u8 {
    #[inline(always)]
    fn from(val: Ie) -> u8 {
        Ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If {
    #[doc = "No interrupt is pending."]
    INT_PEND = 0x0,
    #[doc = "An interrupt is pending."]
    INT_NOPEND = 0x01,
}
impl If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If {
    #[inline(always)]
    fn from(val: u8) -> If {
        If::from_bits(val)
    }
}
impl From<If> for u8 {
    #[inline(always)]
    fn from(val: If) -> u8 {
        If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "No overrides. Bit field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
    NO_OVERRIDE = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
    PRI_DET_OVERRIDE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqRes {
    #[doc = "No results to report."]
    NO_RESULT = 0x0,
    #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
    CONN_SDP = 0x01,
    #[doc = "Attached to a charging port. The exact meaning depends on bit 18 (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)"]
    CONN_CP = 0x02,
    #[doc = "Attached to a DCP."]
    CONN_DCP = 0x03,
}
impl SeqRes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqRes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqRes {
    #[inline(always)]
    fn from(val: u8) -> SeqRes {
        SeqRes::from_bits(val)
    }
}
impl From<SeqRes> for u8 {
    #[inline(always)]
    fn from(val: SeqRes) -> u8 {
        SeqRes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SeqStat {
    #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
    NO_DATA_PIN_CONN = 0x0,
    #[doc = "Data pin contact detection is complete."]
    DATA_PIN_CONN = 0x01,
    #[doc = "Charging port detection is complete."]
    CP_DET_DONE = 0x02,
    #[doc = "Charger type detection is complete."]
    CT_DET_DONE = 0x03,
}
impl SeqStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SeqStat {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SeqStat {
    #[inline(always)]
    fn from(val: u8) -> SeqStat {
        SeqStat::from_bits(val)
    }
}
impl From<SeqStat> for u8 {
    #[inline(always)]
    fn from(val: SeqStat) -> u8 {
        SeqStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sr {
    #[doc = "Do not perform a software reset."]
    NO_RESET = 0x0,
    #[doc = "Perform a software reset."]
    SW_RESET = 0x01,
}
impl Sr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sr {
    #[inline(always)]
    fn from(val: u8) -> Sr {
        Sr::from_bits(val)
    }
}
impl From<Sr> for u8 {
    #[inline(always)]
    fn from(val: Sr) -> u8 {
        Sr::to_bits(val)
    }
}
