#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbrst {
    #[doc = "Incremental burst of unspecified length only"]
    AHBBRST_0 = 0x0,
    #[doc = "INCR4 burst, then single transfer"]
    AHBBRST_1 = 0x01,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_2 = 0x02,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    AHBBRST_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_5 = 0x05,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_6 = 0x06,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    AHBBRST_7 = 0x07,
}
impl Ahbbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbrst {
    #[inline(always)]
    fn from(val: u8) -> Ahbbrst {
        Ahbbrst::from_bits(val)
    }
}
impl From<Ahbbrst> for u8 {
    #[inline(always)]
    fn from(val: Ahbbrst) -> u8 {
        Ahbbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ase {
    #[doc = "Do not process the Asynchronous Schedule."]
    ASE_0 = 0x0,
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    ASE_1 = 0x01,
}
impl Ase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ase {
    #[inline(always)]
    fn from(val: u8) -> Ase {
        Ase::from_bits(val)
    }
}
impl From<Ase> for u8 {
    #[inline(always)]
    fn from(val: Ase) -> u8 {
        Ase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cf {
    #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller."]
    CF_0 = 0x0,
    #[doc = "Port routing control logic default-routes all ports to this host controller."]
    CF_1 = 0x01,
}
impl Cf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cf {
    #[inline(always)]
    fn from(val: u8) -> Cf {
        Cf::from_bits(val)
    }
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(val: Cf) -> u8 {
        Cf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Idle \\[Default for combination host/device\\]"]
    CM_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Device Controller \\[Default for device only controller\\]"]
    CM_2 = 0x02,
    #[doc = "Host Controller \\[Default for host only controller\\]"]
    CM_3 = 0x03,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dc {
    #[doc = "Not supported"]
    DC_0 = 0x0,
    #[doc = "Supported"]
    DC_1 = 0x01,
}
impl Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dc {
    #[inline(always)]
    fn from(val: u8) -> Dc {
        Dc::from_bits(val)
    }
}
impl From<Dc> for u8 {
    #[inline(always)]
    fn from(val: Dc) -> u8 {
        Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Es {
    #[doc = "Little Endian \\[Default\\]"]
    ES_0 = 0x0,
    #[doc = "Big Endian"]
    ES_1 = 0x01,
}
impl Es {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Es {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Es {
    #[inline(always)]
    fn from(val: u8) -> Es {
        Es::from_bits(val)
    }
}
impl From<Es> for u8 {
    #[inline(always)]
    fn from(val: Es) -> u8 {
        Es::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frindex(u16);
impl Frindex {
    #[doc = "(1024) 12"]
    pub const FRINDEX_0: Self = Self(0x0);
    #[doc = "(512) 11"]
    pub const FRINDEX_1: Self = Self(0x01);
    #[doc = "(256) 10"]
    pub const FRINDEX_2: Self = Self(0x02);
    #[doc = "(128) 9"]
    pub const FRINDEX_3: Self = Self(0x03);
    #[doc = "(64) 8"]
    pub const FRINDEX_4: Self = Self(0x04);
    #[doc = "(32) 7"]
    pub const FRINDEX_5: Self = Self(0x05);
    #[doc = "(16) 6"]
    pub const FRINDEX_6: Self = Self(0x06);
    #[doc = "(8) 5"]
    pub const FRINDEX_7: Self = Self(0x07);
}
impl Frindex {
    pub const fn from_bits(val: u16) -> Frindex {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Frindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FRINDEX_0"),
            0x01 => f.write_str("FRINDEX_1"),
            0x02 => f.write_str("FRINDEX_2"),
            0x03 => f.write_str("FRINDEX_3"),
            0x04 => f.write_str("FRINDEX_4"),
            0x05 => f.write_str("FRINDEX_5"),
            0x06 => f.write_str("FRINDEX_6"),
            0x07 => f.write_str("FRINDEX_7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frindex {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FRINDEX_0"),
            0x01 => defmt::write!(f, "FRINDEX_1"),
            0x02 => defmt::write!(f, "FRINDEX_2"),
            0x03 => defmt::write!(f, "FRINDEX_3"),
            0x04 => defmt::write!(f, "FRINDEX_4"),
            0x05 => defmt::write!(f, "FRINDEX_5"),
            0x06 => defmt::write!(f, "FRINDEX_6"),
            0x07 => defmt::write!(f, "FRINDEX_7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Frindex {
    #[inline(always)]
    fn from(val: u16) -> Frindex {
        Frindex::from_bits(val)
    }
}
impl From<Frindex> for u16 {
    #[inline(always)]
    fn from(val: Frindex) -> u16 {
        Frindex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fs2 {
    #[doc = "1024 elements (4096 bytes) Default value"]
    FS_2_0 = 0x0,
    #[doc = "512 elements (2048 bytes)"]
    FS_2_1 = 0x01,
}
impl Fs2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fs2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fs2 {
    #[inline(always)]
    fn from(val: u8) -> Fs2 {
        Fs2::from_bits(val)
    }
}
impl From<Fs2> for u8 {
    #[inline(always)]
    fn from(val: Fs2) -> u8 {
        Fs2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptmode {
    #[doc = "One Shot Mode"]
    GPTMODE_0 = 0x0,
    #[doc = "Repeat Mode"]
    GPTMODE_1 = 0x01,
}
impl Gptimer0ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptmode {
        Gptimer0ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptmode) -> u8 {
        Gptimer0ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrst {
    #[doc = "No action"]
    GPTRST_0 = 0x0,
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    GPTRST_1 = 0x01,
}
impl Gptimer0ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrst {
        Gptimer0ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrst) -> u8 {
        Gptimer0ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrun {
    #[doc = "Stop counting"]
    GPTRUN_0 = 0x0,
    #[doc = "Run"]
    GPTRUN_1 = 0x01,
}
impl Gptimer0ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrun {
        Gptimer0ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrun) -> u8 {
        Gptimer0ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptmode {
    #[doc = "One Shot Mode"]
    GPTMODE_0 = 0x0,
    #[doc = "Repeat Mode"]
    GPTMODE_1 = 0x01,
}
impl Gptimer1ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptmode {
        Gptimer1ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptmode) -> u8 {
        Gptimer1ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrst {
    #[doc = "No action"]
    GPTRST_0 = 0x0,
    #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
    GPTRST_1 = 0x01,
}
impl Gptimer1ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrst {
        Gptimer1ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrst) -> u8 {
        Gptimer1ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrun {
    #[doc = "Stop counting"]
    GPTRUN_0 = 0x0,
    #[doc = "Run"]
    GPTRUN_1 = 0x01,
}
impl Gptimer1ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrun {
        Gptimer1ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrun) -> u8 {
        Gptimer1ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc {
    #[doc = "Not supported"]
    HC_0 = 0x0,
    #[doc = "Supported"]
    HC_1 = 0x01,
}
impl Hc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc {
    #[inline(always)]
    fn from(val: u8) -> Hc {
        Hc::from_bits(val)
    }
}
impl From<Hc> for u8 {
    #[inline(always)]
    fn from(val: Hc) -> u8 {
        Hc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Itc(u8);
impl Itc {
    #[doc = "Immediate (no threshold)"]
    pub const ITC_0: Self = Self(0x0);
    #[doc = "1 micro-frame"]
    pub const ITC_1: Self = Self(0x01);
    #[doc = "2 micro-frames"]
    pub const ITC_2: Self = Self(0x02);
    #[doc = "4 micro-frames"]
    pub const ITC_4: Self = Self(0x04);
    #[doc = "8 micro-frames"]
    pub const ITC_8: Self = Self(0x08);
    #[doc = "16 micro-frames"]
    pub const ITC_16: Self = Self(0x10);
    #[doc = "32 micro-frames"]
    pub const ITC_32: Self = Self(0x20);
    #[doc = "64 micro-frames"]
    pub const ITC_64: Self = Self(0x40);
}
impl Itc {
    pub const fn from_bits(val: u8) -> Itc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Itc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ITC_0"),
            0x01 => f.write_str("ITC_1"),
            0x02 => f.write_str("ITC_2"),
            0x04 => f.write_str("ITC_4"),
            0x08 => f.write_str("ITC_8"),
            0x10 => f.write_str("ITC_16"),
            0x20 => f.write_str("ITC_32"),
            0x40 => f.write_str("ITC_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ITC_0"),
            0x01 => defmt::write!(f, "ITC_1"),
            0x02 => defmt::write!(f, "ITC_2"),
            0x04 => defmt::write!(f, "ITC_4"),
            0x08 => defmt::write!(f, "ITC_8"),
            0x10 => defmt::write!(f, "ITC_16"),
            0x20 => defmt::write!(f, "ITC_32"),
            0x40 => defmt::write!(f, "ITC_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Itc {
    #[inline(always)]
    fn from(val: u8) -> Itc {
        Itc::from_bits(val)
    }
}
impl From<Itc> for u8 {
    #[inline(always)]
    fn from(val: Itc) -> u8 {
        Itc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ls {
    #[doc = "SE0"]
    LS_0 = 0x0,
    #[doc = "K-state"]
    LS_1 = 0x01,
    #[doc = "J-state"]
    LS_2 = 0x02,
    #[doc = "Undefined"]
    LS_3 = 0x03,
}
impl Ls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ls {
    #[inline(always)]
    fn from(val: u8) -> Ls {
        Ls::from_bits(val)
    }
}
impl From<Ls> for u8 {
    #[inline(always)]
    fn from(val: Ls) -> u8 {
        Ls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NCc {
    #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported."]
    N_CC_0 = 0x0,
    #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported."]
    N_CC_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NCc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NCc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NCc {
    #[inline(always)]
    fn from(val: u8) -> NCc {
        NCc::from_bits(val)
    }
}
impl From<NCc> for u8 {
    #[inline(always)]
    fn from(val: NCc) -> u8 {
        NCc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oca {
    #[doc = "This port does not have an over-current condition."]
    OCA_0 = 0x0,
    #[doc = "This port currently has an over-current condition"]
    OCA_1 = 0x01,
}
impl Oca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oca {
    #[inline(always)]
    fn from(val: u8) -> Oca {
        Oca::from_bits(val)
    }
}
impl From<Oca> for u8 {
    #[inline(always)]
    fn from(val: Oca) -> u8 {
        Oca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsc {
    #[doc = "Normal operation"]
    PFSC_0 = 0x0,
    #[doc = "Forced to full speed"]
    PFSC_1 = 0x01,
}
impl Pfsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsc {
    #[inline(always)]
    fn from(val: u8) -> Pfsc {
        Pfsc::from_bits(val)
    }
}
impl From<Pfsc> for u8 {
    #[inline(always)]
    fn from(val: Pfsc) -> u8 {
        Pfsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phcd {
    #[doc = "Enable PHY clock"]
    PHCD_0 = 0x0,
    #[doc = "Disable PHY clock"]
    PHCD_1 = 0x01,
}
impl Phcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phcd {
    #[inline(always)]
    fn from(val: u8) -> Phcd {
        Phcd::from_bits(val)
    }
}
impl From<Phcd> for u8 {
    #[inline(always)]
    fn from(val: Phcd) -> u8 {
        Phcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phym {
    #[doc = "UTMI/UMTI+"]
    PHYM_0 = 0x0,
    #[doc = "ULPI DDR"]
    PHYM_1 = 0x01,
    #[doc = "ULPI"]
    PHYM_2 = 0x02,
    #[doc = "Serial Only"]
    PHYM_3 = 0x03,
    #[doc = "Software programmable - reset to UTMI/UTMI+"]
    PHYM_4 = 0x04,
    #[doc = "Software programmable - reset to ULPI DDR"]
    PHYM_5 = 0x05,
    #[doc = "Software programmable - reset to ULPI"]
    PHYM_6 = 0x06,
    #[doc = "Software programmable - reset to Serial"]
    PHYM_7 = 0x07,
}
impl Phym {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phym {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phym {
    #[inline(always)]
    fn from(val: u8) -> Phym {
        Phym::from_bits(val)
    }
}
impl From<Phym> for u8 {
    #[inline(always)]
    fn from(val: Phym) -> u8 {
        Phym::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyw {
    #[doc = "8 bit wide data bus Software non-programmable"]
    PHYW_0 = 0x0,
    #[doc = "16 bit wide data bus Software non-programmable"]
    PHYW_1 = 0x01,
    #[doc = "Reset to 8 bit wide data bus Software programmable"]
    PHYW_2 = 0x02,
    #[doc = "Reset to 16 bit wide data bus Software programmable"]
    PHYW_3 = 0x03,
}
impl Phyw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyw {
    #[inline(always)]
    fn from(val: u8) -> Phyw {
        Phyw::from_bits(val)
    }
}
impl From<Phyw> for u8 {
    #[inline(always)]
    fn from(val: Phyw) -> u8 {
        Phyw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pic {
    #[doc = "Port indicators are off"]
    PIC_0 = 0x0,
    #[doc = "Amber"]
    PIC_1 = 0x01,
    #[doc = "Green"]
    PIC_2 = 0x02,
    #[doc = "Undefined"]
    PIC_3 = 0x03,
}
impl Pic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pic {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pic {
    #[inline(always)]
    fn from(val: u8) -> Pic {
        Pic::from_bits(val)
    }
}
impl From<Pic> for u8 {
    #[inline(always)]
    fn from(val: Pic) -> u8 {
        Pic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pse {
    #[doc = "Do not process the Periodic Schedule"]
    PSE_0 = 0x0,
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    PSE_1 = 0x01,
}
impl Pse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pse {
    #[inline(always)]
    fn from(val: u8) -> Pse {
        Pse::from_bits(val)
    }
}
impl From<Pse> for u8 {
    #[inline(always)]
    fn from(val: Pse) -> u8 {
        Pse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pspd {
    #[doc = "Full Speed"]
    PSPD_0 = 0x0,
    #[doc = "Low Speed"]
    PSPD_1 = 0x01,
    #[doc = "High Speed"]
    PSPD_2 = 0x02,
    #[doc = "Undefined"]
    PSPD_3 = 0x03,
}
impl Pspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pspd {
    #[inline(always)]
    fn from(val: u8) -> Pspd {
        Pspd::from_bits(val)
    }
}
impl From<Pspd> for u8 {
    #[inline(always)]
    fn from(val: Pspd) -> u8 {
        Pspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptc {
    #[doc = "TEST_MODE_DISABLE"]
    PTC_0 = 0x0,
    #[doc = "J_STATE"]
    PTC_1 = 0x01,
    #[doc = "K_STATE"]
    PTC_2 = 0x02,
    #[doc = "SE0 (host) / NAK (device)"]
    PTC_3 = 0x03,
    #[doc = "Packet"]
    PTC_4 = 0x04,
    #[doc = "FORCE_ENABLE_HS"]
    PTC_5 = 0x05,
    #[doc = "FORCE_ENABLE_FS"]
    PTC_6 = 0x06,
    #[doc = "FORCE_ENABLE_LS"]
    PTC_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ptc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptc {
    #[inline(always)]
    fn from(val: u8) -> Ptc {
        Ptc::from_bits(val)
    }
}
impl From<Ptc> for u8 {
    #[inline(always)]
    fn from(val: Ptc) -> u8 {
        Ptc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptw {
    #[doc = "Select the 8-bit UTMI interface \\[60MHz\\]"]
    PTW_0 = 0x0,
    #[doc = "Select the 16-bit UTMI interface \\[30MHz\\]"]
    PTW_1 = 0x01,
}
impl Ptw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptw {
    #[inline(always)]
    fn from(val: u8) -> Ptw {
        Ptw::from_bits(val)
    }
}
impl From<Ptw> for u8 {
    #[inline(always)]
    fn from(val: Ptw) -> u8 {
        Ptw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slom {
    #[doc = "Setup Lockouts On (default);"]
    SLOM_0 = 0x0,
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    SLOM_1 = 0x01,
}
impl Slom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slom {
    #[inline(always)]
    fn from(val: u8) -> Slom {
        Slom::from_bits(val)
    }
}
impl From<Slom> for u8 {
    #[inline(always)]
    fn from(val: Slom) -> u8 {
        Slom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm {
    #[doc = "No Serial Engine, always use parallel signalling."]
    SM_0 = 0x0,
    #[doc = "Serial Engine present, always use serial signalling for FS/LS."]
    SM_1 = 0x01,
    #[doc = "Software programmable - Reset to use parallel signalling for FS/LS"]
    SM_2 = 0x02,
    #[doc = "Software programmable - Reset to use serial signalling for FS/LS"]
    SM_3 = 0x03,
}
impl Sm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm {
    #[inline(always)]
    fn from(val: u8) -> Sm {
        Sm::from_bits(val)
    }
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(val: Sm) -> u8 {
        Sm::to_bits(val)
    }
}
