#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set"]
    pub const STANDARD: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("STANDARD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvreq {
    #[doc = "Turn off"]
    TURN_OFF = 0x0,
    #[doc = "Turn on"]
    TURN_ON = 0x01,
}
impl Hvreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvreq {
    #[inline(always)]
    fn from(val: u8) -> Hvreq {
        Hvreq::from_bits(val)
    }
}
impl From<Hvreq> for u8 {
    #[inline(always)]
    fn from(val: Hvreq) -> u8 {
        Hvreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvreq {
    #[doc = "Turn off"]
    TURN_OFF = 0x0,
    #[doc = "Turn on"]
    TURN_ON = 0x01,
}
impl Lvreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvreq {
    #[inline(always)]
    fn from(val: u8) -> Lvreq {
        Lvreq::from_bits(val)
    }
}
impl From<Lvreq> for u8 {
    #[inline(always)]
    fn from(val: Lvreq) -> u8 {
        Lvreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdreq {
    #[doc = "PD pin is set to low when OTPC is in idle state. It means EFUSE hardmacro is in standby mode. Idle state means OTPC is not in read and program modes."]
    NO_ACTION = 0x0,
    #[doc = "PD pin is set to high when OTPC is in idle state. It means EFUSE hardmacro is in power down mode."]
    POWER_DN = 0x01,
}
impl Pdreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdreq {
    #[inline(always)]
    fn from(val: u8) -> Pdreq {
        Pdreq::from_bits(val)
    }
}
impl From<Pdreq> for u8 {
    #[inline(always)]
    fn from(val: Pdreq) -> u8 {
        Pdreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadEfuse {
    #[doc = "Starts program operation when the WR_UNLOCK value is 0x9527; otherwise, takes no action."]
    PROGRAM = 0x0,
    #[doc = "Starts read operation"]
    READ = 0x01,
}
impl ReadEfuse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadEfuse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadEfuse {
    #[inline(always)]
    fn from(val: u8) -> ReadEfuse {
        ReadEfuse::from_bits(val)
    }
}
impl From<ReadEfuse> for u8 {
    #[inline(always)]
    fn from(val: ReadEfuse) -> u8 {
        ReadEfuse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReloadShadows {
    #[doc = "No action (when writing) or reload complete (when reading)"]
    NO_ACTION = 0x0,
    #[doc = "Reload"]
    RELOAD = 0x01,
}
impl ReloadShadows {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReloadShadows {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReloadShadows {
    #[inline(always)]
    fn from(val: u8) -> ReloadShadows {
        ReloadShadows::from_bits(val)
    }
}
impl From<ReloadShadows> for u8 {
    #[inline(always)]
    fn from(val: ReloadShadows) -> u8 {
        ReloadShadows::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrAll1s {
    #[doc = "Uses the WDATA value"]
    USE_WDATA = 0x0,
    #[doc = "Writes all 1s"]
    USE_ALL1S = 0x01,
}
impl WrAll1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrAll1s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrAll1s {
    #[inline(always)]
    fn from(val: u8) -> WrAll1s {
        WrAll1s::from_bits(val)
    }
}
impl From<WrAll1s> for u8 {
    #[inline(always)]
    fn from(val: WrAll1s) -> u8 {
        WrAll1s::to_bits(val)
    }
}
