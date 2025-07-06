#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BsyFil {
    #[doc = "MICFIL is stopped"]
    STOPPED = 0x0,
    #[doc = "MICFIL is running"]
    RUNNING = 0x01,
}
impl BsyFil {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BsyFil {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BsyFil {
    #[inline(always)]
    fn from(val: u8) -> BsyFil {
        BsyFil::from_bits(val)
    }
}
impl From<BsyFil> for u8 {
    #[inline(always)]
    fn from(val: BsyFil) -> u8 {
        BsyFil::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch0f {
    #[doc = "Not surpassed"]
    WM_NOTREACHED = 0x0,
    #[doc = "Surpassed"]
    WM_REACHED = 0x01,
}
impl Ch0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch0f {
    #[inline(always)]
    fn from(val: u8) -> Ch0f {
        Ch0f::from_bits(val)
    }
}
impl From<Ch0f> for u8 {
    #[inline(always)]
    fn from(val: Ch0f) -> u8 {
        Ch0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch1f {
    #[doc = "Not surpassed"]
    WM_NOTREACHED = 0x0,
    #[doc = "Surpassed"]
    WM_REACHED = 0x01,
}
impl Ch1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch1f {
    #[inline(always)]
    fn from(val: u8) -> Ch1f {
        Ch1f::from_bits(val)
    }
}
impl From<Ch1f> for u8 {
    #[inline(always)]
    fn from(val: Ch1f) -> u8 {
        Ch1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch2f {
    #[doc = "Not surpassed"]
    WM_NOTREACHED = 0x0,
    #[doc = "Surpassed"]
    WM_REACHED = 0x01,
}
impl Ch2f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch2f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch2f {
    #[inline(always)]
    fn from(val: u8) -> Ch2f {
        Ch2f::from_bits(val)
    }
}
impl From<Ch2f> for u8 {
    #[inline(always)]
    fn from(val: Ch2f) -> u8 {
        Ch2f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ch3f {
    #[doc = "Not surpassed"]
    WM_NOTREACHED = 0x0,
    #[doc = "Surpassed"]
    WM_REACHED = 0x01,
}
impl Ch3f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ch3f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ch3f {
    #[inline(always)]
    fn from(val: u8) -> Ch3f {
        Ch3f::from_bits(val)
    }
}
impl From<Ch3f> for u8 {
    #[inline(always)]
    fn from(val: Ch3f) -> u8 {
        Ch3f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cicosr {
    #[doc = "CIC oversampling rate = 0"]
    CICOSR_0 = 0x0,
    #[doc = "CIC oversampling rate = 1"]
    CICOSR_1 = 0x01,
    #[doc = "..."]
    CICOSR_2_2 = 0x02,
    #[doc = "..."]
    CICOSR_2_3 = 0x03,
    #[doc = "..."]
    CICOSR_2_4 = 0x04,
    #[doc = "..."]
    CICOSR_2_5 = 0x05,
    #[doc = "..."]
    CICOSR_2_6 = 0x06,
    #[doc = "..."]
    CICOSR_2_7 = 0x07,
    #[doc = "..."]
    CICOSR_2_8 = 0x08,
    #[doc = "..."]
    CICOSR_2_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CIC oversampling rate = 15"]
    CICOSR_15 = 0x0f,
}
impl Cicosr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cicosr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cicosr {
    #[inline(always)]
    fn from(val: u8) -> Cicosr {
        Cicosr::from_bits(val)
    }
}
impl From<Cicosr> for u8 {
    #[inline(always)]
    fn from(val: Cicosr) -> u8 {
        Cicosr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clkdiv(u8);
impl Clkdiv {
    #[doc = "Internal clock divider value = 0"]
    pub const CLKDIV_0: Self = Self(0x0);
    #[doc = "Internal clock divider value = 1"]
    pub const CLKDIV_1: Self = Self(0x01);
    #[doc = "..."]
    pub const CLKDIV_2_2: Self = Self(0x02);
    #[doc = "..."]
    pub const CLKDIV_2_3: Self = Self(0x03);
    #[doc = "..."]
    pub const CLKDIV_2_4: Self = Self(0x04);
    #[doc = "..."]
    pub const CLKDIV_2_5: Self = Self(0x05);
    #[doc = "..."]
    pub const CLKDIV_2_6: Self = Self(0x06);
    #[doc = "..."]
    pub const CLKDIV_2_7: Self = Self(0x07);
    #[doc = "..."]
    pub const CLKDIV_2_8: Self = Self(0x08);
    #[doc = "..."]
    pub const CLKDIV_2_9: Self = Self(0x09);
    #[doc = "Internal clock divider value = 255"]
    pub const CLKDIV_255: Self = Self(0xff);
}
impl Clkdiv {
    pub const fn from_bits(val: u8) -> Clkdiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLKDIV_0"),
            0x01 => f.write_str("CLKDIV_1"),
            0x02 => f.write_str("CLKDIV_2_2"),
            0x03 => f.write_str("CLKDIV_2_3"),
            0x04 => f.write_str("CLKDIV_2_4"),
            0x05 => f.write_str("CLKDIV_2_5"),
            0x06 => f.write_str("CLKDIV_2_6"),
            0x07 => f.write_str("CLKDIV_2_7"),
            0x08 => f.write_str("CLKDIV_2_8"),
            0x09 => f.write_str("CLKDIV_2_9"),
            0xff => f.write_str("CLKDIV_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLKDIV_0"),
            0x01 => defmt::write!(f, "CLKDIV_1"),
            0x02 => defmt::write!(f, "CLKDIV_2_2"),
            0x03 => defmt::write!(f, "CLKDIV_2_3"),
            0x04 => defmt::write!(f, "CLKDIV_2_4"),
            0x05 => defmt::write!(f, "CLKDIV_2_5"),
            0x06 => defmt::write!(f, "CLKDIV_2_6"),
            0x07 => defmt::write!(f, "CLKDIV_2_7"),
            0x08 => defmt::write!(f, "CLKDIV_2_8"),
            0x09 => defmt::write!(f, "CLKDIV_2_9"),
            0xff => defmt::write!(f, "CLKDIV_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Clkdiv {
        Clkdiv::from_bits(val)
    }
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Clkdiv) -> u8 {
        Clkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkdivdis {
    #[doc = "Enables"]
    ENABLE = 0x0,
    #[doc = "Disables"]
    DISABLE = 0x01,
}
impl Clkdivdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdivdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkdivdis {
    #[inline(always)]
    fn from(val: u8) -> Clkdivdis {
        Clkdivdis::from_bits(val)
    }
}
impl From<Clkdivdis> for u8 {
    #[inline(always)]
    fn from(val: Clkdivdis) -> u8 {
        Clkdivdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbg {
    #[doc = "Normal"]
    NORMAL = 0x0,
    #[doc = "Debug"]
    DEBUG = 0x01,
}
impl Dbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbg {
    #[inline(always)]
    fn from(val: u8) -> Dbg {
        Dbg::from_bits(val)
    }
}
impl From<Dbg> for u8 {
    #[inline(always)]
    fn from(val: Dbg) -> u8 {
        Dbg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbge {
    #[doc = "Disables after completing the current frame"]
    DISABLED = 0x0,
    #[doc = "Enables operation"]
    ENABLED = 0x01,
}
impl Dbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbge {
    #[inline(always)]
    fn from(val: u8) -> Dbge {
        Dbge::from_bits(val)
    }
}
impl From<Dbge> for u8 {
    #[inline(always)]
    fn from(val: Dbge) -> u8 {
        Dbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcBypass {
    #[doc = "Active"]
    DCACTIVE = 0x0,
    #[doc = "Disabled"]
    DCBYPASSED = 0x01,
}
impl DcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcBypass {
    #[inline(always)]
    fn from(val: u8) -> DcBypass {
        DcBypass::from_bits(val)
    }
}
impl From<DcBypass> for u8 {
    #[inline(always)]
    fn from(val: DcBypass) -> u8 {
        DcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcCtrlDcconfig0 {
    #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASS = 0x03,
}
impl DcCtrlDcconfig0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcCtrlDcconfig0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcCtrlDcconfig0 {
    #[inline(always)]
    fn from(val: u8) -> DcCtrlDcconfig0 {
        DcCtrlDcconfig0::from_bits(val)
    }
}
impl From<DcCtrlDcconfig0> for u8 {
    #[inline(always)]
    fn from(val: DcCtrlDcconfig0) -> u8 {
        DcCtrlDcconfig0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcCtrlDcconfig1 {
    #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASS = 0x03,
}
impl DcCtrlDcconfig1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcCtrlDcconfig1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcCtrlDcconfig1 {
    #[inline(always)]
    fn from(val: u8) -> DcCtrlDcconfig1 {
        DcCtrlDcconfig1::from_bits(val)
    }
}
impl From<DcCtrlDcconfig1> for u8 {
    #[inline(always)]
    fn from(val: DcCtrlDcconfig1) -> u8 {
        DcCtrlDcconfig1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcCtrlDcconfig2 {
    #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASS = 0x03,
}
impl DcCtrlDcconfig2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcCtrlDcconfig2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcCtrlDcconfig2 {
    #[inline(always)]
    fn from(val: u8) -> DcCtrlDcconfig2 {
        DcCtrlDcconfig2::from_bits(val)
    }
}
impl From<DcCtrlDcconfig2> for u8 {
    #[inline(always)]
    fn from(val: DcCtrlDcconfig2) -> u8 {
        DcCtrlDcconfig2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcCtrlDcconfig3 {
    #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASS = 0x03,
}
impl DcCtrlDcconfig3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcCtrlDcconfig3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcCtrlDcconfig3 {
    #[inline(always)]
    fn from(val: u8) -> DcCtrlDcconfig3 {
        DcCtrlDcconfig3::from_bits(val)
    }
}
impl From<DcCtrlDcconfig3> for u8 {
    #[inline(always)]
    fn from(val: DcCtrlDcconfig3) -> u8 {
        DcCtrlDcconfig3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutBypass {
    #[doc = "Active"]
    DCACTIVE = 0x0,
    #[doc = "Disabled"]
    DCBYPASSED = 0x01,
}
impl DcOutBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutBypass {
    #[inline(always)]
    fn from(val: u8) -> DcOutBypass {
        DcOutBypass::from_bits(val)
    }
}
impl From<DcOutBypass> for u8 {
    #[inline(always)]
    fn from(val: DcOutBypass) -> u8 {
        DcOutBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutCtrlDcconfig0 {
    #[doc = "20 Hz (FS = 48 kHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (FS = 48 kHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (FS = 48 kHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASSED = 0x03,
}
impl DcOutCtrlDcconfig0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutCtrlDcconfig0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutCtrlDcconfig0 {
    #[inline(always)]
    fn from(val: u8) -> DcOutCtrlDcconfig0 {
        DcOutCtrlDcconfig0::from_bits(val)
    }
}
impl From<DcOutCtrlDcconfig0> for u8 {
    #[inline(always)]
    fn from(val: DcOutCtrlDcconfig0) -> u8 {
        DcOutCtrlDcconfig0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutCtrlDcconfig1 {
    #[doc = "20 Hz (FS = 48 kHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (FS = 48 kHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (FS = 48 kHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASSED = 0x03,
}
impl DcOutCtrlDcconfig1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutCtrlDcconfig1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutCtrlDcconfig1 {
    #[inline(always)]
    fn from(val: u8) -> DcOutCtrlDcconfig1 {
        DcOutCtrlDcconfig1::from_bits(val)
    }
}
impl From<DcOutCtrlDcconfig1> for u8 {
    #[inline(always)]
    fn from(val: DcOutCtrlDcconfig1) -> u8 {
        DcOutCtrlDcconfig1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutCtrlDcconfig2 {
    #[doc = "20 Hz (FS = 48 kHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (FS = 48 kHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (FS = 48 kHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASSED = 0x03,
}
impl DcOutCtrlDcconfig2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutCtrlDcconfig2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutCtrlDcconfig2 {
    #[inline(always)]
    fn from(val: u8) -> DcOutCtrlDcconfig2 {
        DcOutCtrlDcconfig2::from_bits(val)
    }
}
impl From<DcOutCtrlDcconfig2> for u8 {
    #[inline(always)]
    fn from(val: DcOutCtrlDcconfig2) -> u8 {
        DcOutCtrlDcconfig2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutCtrlDcconfig3 {
    #[doc = "20 Hz (FS = 48 kHz)"]
    DC_REM_20HZ = 0x0,
    #[doc = "13.3 Hz (FS = 48 kHz)"]
    DC_REM_13P3HZ = 0x01,
    #[doc = "40 Hz (FS = 48 kHz)"]
    DC_REM_40HZ = 0x02,
    #[doc = "DC remover is bypassed"]
    DC_REM_BYPASSED = 0x03,
}
impl DcOutCtrlDcconfig3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutCtrlDcconfig3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutCtrlDcconfig3 {
    #[inline(always)]
    fn from(val: u8) -> DcOutCtrlDcconfig3 {
        DcOutCtrlDcconfig3::from_bits(val)
    }
}
impl From<DcOutCtrlDcconfig3> for u8 {
    #[inline(always)]
    fn from(val: DcOutCtrlDcconfig3) -> u8 {
        DcOutCtrlDcconfig3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decfils {
    #[doc = "Stops decimation filter"]
    STOP = 0x0,
    #[doc = "Keeps decimation filter running"]
    RUN = 0x01,
}
impl Decfils {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Decfils {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Decfils {
    #[inline(always)]
    fn from(val: u8) -> Decfils {
        Decfils::from_bits(val)
    }
}
impl From<Decfils> for u8 {
    #[inline(always)]
    fn from(val: Decfils) -> u8 {
        Decfils::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disel {
    #[doc = "Disables DMA and interrupt requests"]
    ALL_DISABLED = 0x0,
    #[doc = "Enables DMA requests"]
    DMAREQ_ENABLED = 0x01,
    #[doc = "Enables interrupt requests"]
    INTREQ_ENABLED = 0x02,
    _RESERVED_3 = 0x03,
}
impl Disel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disel {
    #[inline(always)]
    fn from(val: u8) -> Disel {
        Disel::from_bits(val)
    }
}
impl From<Disel> for u8 {
    #[inline(always)]
    fn from(val: Disel) -> u8 {
        Disel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erren {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Erren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erren {
    #[inline(always)]
    fn from(val: u8) -> Erren {
        Erren::from_bits(val)
    }
}
impl From<Erren> for u8 {
    #[inline(always)]
    fn from(val: Erren) -> u8 {
        Erren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoPtrwid {
    #[doc = "0 bits"]
    PTRWID_0 = 0x0,
    #[doc = "1 bit"]
    PTRWID_1 = 0x01,
    #[doc = "2 bits"]
    PTRWID_2 = 0x02,
    #[doc = "..."]
    PTRWID_3_3 = 0x03,
    #[doc = "..."]
    PTRWID_3_4 = 0x04,
    #[doc = "..."]
    PTRWID_3_5 = 0x05,
    #[doc = "..."]
    PTRWID_3_6 = 0x06,
    #[doc = "..."]
    PTRWID_3_7 = 0x07,
    #[doc = "..."]
    PTRWID_3_8 = 0x08,
    #[doc = "..."]
    PTRWID_3_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "15 bits"]
    PTRWID_15 = 0x0f,
}
impl FifoPtrwid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoPtrwid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoPtrwid {
    #[inline(always)]
    fn from(val: u8) -> FifoPtrwid {
        FifoPtrwid::from_bits(val)
    }
}
impl From<FifoPtrwid> for u8 {
    #[inline(always)]
    fn from(val: FifoPtrwid) -> u8 {
        FifoPtrwid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoovf0 {
    #[doc = "No exception by FIFO overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO overflow"]
    EXCEPTION = 0x01,
}
impl Fifoovf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoovf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoovf0 {
    #[inline(always)]
    fn from(val: u8) -> Fifoovf0 {
        Fifoovf0::from_bits(val)
    }
}
impl From<Fifoovf0> for u8 {
    #[inline(always)]
    fn from(val: Fifoovf0) -> u8 {
        Fifoovf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoovf1 {
    #[doc = "No exception by FIFO overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO overflow"]
    EXCEPTION = 0x01,
}
impl Fifoovf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoovf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoovf1 {
    #[inline(always)]
    fn from(val: u8) -> Fifoovf1 {
        Fifoovf1::from_bits(val)
    }
}
impl From<Fifoovf1> for u8 {
    #[inline(always)]
    fn from(val: Fifoovf1) -> u8 {
        Fifoovf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoovf2 {
    #[doc = "No exception by FIFO overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO overflow"]
    EXCEPTION = 0x01,
}
impl Fifoovf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoovf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoovf2 {
    #[inline(always)]
    fn from(val: u8) -> Fifoovf2 {
        Fifoovf2::from_bits(val)
    }
}
impl From<Fifoovf2> for u8 {
    #[inline(always)]
    fn from(val: Fifoovf2) -> u8 {
        Fifoovf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoovf3 {
    #[doc = "No exception by FIFO overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO overflow"]
    EXCEPTION = 0x01,
}
impl Fifoovf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoovf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoovf3 {
    #[inline(always)]
    fn from(val: u8) -> Fifoovf3 {
        Fifoovf3::from_bits(val)
    }
}
impl From<Fifoovf3> for u8 {
    #[inline(always)]
    fn from(val: Fifoovf3) -> u8 {
        Fifoovf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifound0 {
    #[doc = "No exception by FIFO underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO underflow"]
    EXCEPTION = 0x01,
}
impl Fifound0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifound0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifound0 {
    #[inline(always)]
    fn from(val: u8) -> Fifound0 {
        Fifound0::from_bits(val)
    }
}
impl From<Fifound0> for u8 {
    #[inline(always)]
    fn from(val: Fifound0) -> u8 {
        Fifound0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifound1 {
    #[doc = "No exception by FIFO underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO underflow"]
    EXCEPTION = 0x01,
}
impl Fifound1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifound1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifound1 {
    #[inline(always)]
    fn from(val: u8) -> Fifound1 {
        Fifound1::from_bits(val)
    }
}
impl From<Fifound1> for u8 {
    #[inline(always)]
    fn from(val: Fifound1) -> u8 {
        Fifound1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifound2 {
    #[doc = "No exception by FIFO underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO underflow"]
    EXCEPTION = 0x01,
}
impl Fifound2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifound2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifound2 {
    #[inline(always)]
    fn from(val: u8) -> Fifound2 {
        Fifound2::from_bits(val)
    }
}
impl From<Fifound2> for u8 {
    #[inline(always)]
    fn from(val: Fifound2) -> u8 {
        Fifound2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifound3 {
    #[doc = "No exception by FIFO underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by FIFO underflow"]
    EXCEPTION = 0x01,
}
impl Fifound3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifound3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifound3 {
    #[inline(always)]
    fn from(val: u8) -> Fifound3 {
        Fifound3::from_bits(val)
    }
}
impl From<Fifound3> for u8 {
    #[inline(always)]
    fn from(val: Fifound3) -> u8 {
        Fifound3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilOutWidth24b {
    #[doc = "16 bits"]
    WID16B = 0x0,
    #[doc = "24 bits"]
    WID24B = 0x01,
}
impl FilOutWidth24b {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilOutWidth24b {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilOutWidth24b {
    #[inline(always)]
    fn from(val: u8) -> FilOutWidth24b {
        FilOutWidth24b::from_bits(val)
    }
}
impl From<FilOutWidth24b> for u8 {
    #[inline(always)]
    fn from(val: FilOutWidth24b) -> u8 {
        FilOutWidth24b::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsyncen {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Fsyncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsyncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsyncen {
    #[inline(always)]
    fn from(val: u8) -> Fsyncen {
        Fsyncen::from_bits(val)
    }
}
impl From<Fsyncen> for u8 {
    #[inline(always)]
    fn from(val: Fsyncen) -> u8 {
        Fsyncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LowPower {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl LowPower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LowPower {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LowPower {
    #[inline(always)]
    fn from(val: u8) -> LowPower {
        LowPower::from_bits(val)
    }
}
impl From<LowPower> for u8 {
    #[inline(always)]
    fn from(val: LowPower) -> u8 {
        LowPower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Normal mode"]
    NORMAL = 0x0,
    #[doc = "DLL mode"]
    LOW_LEAKAGE = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npair {
    #[doc = "None"]
    NPAIR_0 = 0x0,
    #[doc = "1 pair"]
    NPAIR_1 = 0x01,
    #[doc = "2 pairs"]
    NPAIR_2 = 0x02,
    #[doc = "..."]
    NPAIR_3_3 = 0x03,
    #[doc = "..."]
    NPAIR_3_4 = 0x04,
    #[doc = "..."]
    NPAIR_3_5 = 0x05,
    #[doc = "..."]
    NPAIR_3_6 = 0x06,
    #[doc = "..."]
    NPAIR_3_7 = 0x07,
    #[doc = "..."]
    NPAIR_3_8 = 0x08,
    #[doc = "..."]
    NPAIR_3_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "15 pairs"]
    NPAIR_15 = 0x0f,
}
impl Npair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npair {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npair {
    #[inline(always)]
    fn from(val: u8) -> Npair {
        Npair::from_bits(val)
    }
}
impl From<Npair> for u8 {
    #[inline(always)]
    fn from(val: Npair) -> u8 {
        Npair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdmien {
    #[doc = "Stops MICFIL operation"]
    STOPPED = 0x0,
    #[doc = "Starts MICFIL operation"]
    STARTED = 0x01,
}
impl Pdmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdmien {
    #[inline(always)]
    fn from(val: u8) -> Pdmien {
        Pdmien::from_bits(val)
    }
}
impl From<Pdmien> for u8 {
    #[inline(always)]
    fn from(val: Pdmien) -> u8 {
        Pdmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qsel {
    #[doc = "Medium-Quality mode"]
    MQ_MODE = 0x0,
    #[doc = "High-Quality mode"]
    HQ_MODE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Very-Low-Quality 2 mode"]
    VLQ2_MODE = 0x04,
    #[doc = "Very-Low-Quality 1 mode"]
    VLQ1_MODE = 0x05,
    #[doc = "Very-Low-Quality 0 mode"]
    VLQ0_MODE = 0x06,
    #[doc = "Low-Quality mode"]
    LQ_MODE = 0x07,
}
impl Qsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qsel {
    #[inline(always)]
    fn from(val: u8) -> Qsel {
        Qsel::from_bits(val)
    }
}
impl From<Qsel> for u8 {
    #[inline(always)]
    fn from(val: Qsel) -> u8 {
        Qsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeovf0 {
    #[doc = "No exception by range overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range overflow"]
    EXCEPTION = 0x01,
}
impl Rangeovf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeovf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeovf0 {
    #[inline(always)]
    fn from(val: u8) -> Rangeovf0 {
        Rangeovf0::from_bits(val)
    }
}
impl From<Rangeovf0> for u8 {
    #[inline(always)]
    fn from(val: Rangeovf0) -> u8 {
        Rangeovf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeovf1 {
    #[doc = "No exception by range overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range overflow"]
    EXCEPTION = 0x01,
}
impl Rangeovf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeovf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeovf1 {
    #[inline(always)]
    fn from(val: u8) -> Rangeovf1 {
        Rangeovf1::from_bits(val)
    }
}
impl From<Rangeovf1> for u8 {
    #[inline(always)]
    fn from(val: Rangeovf1) -> u8 {
        Rangeovf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeovf2 {
    #[doc = "No exception by range overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range overflow"]
    EXCEPTION = 0x01,
}
impl Rangeovf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeovf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeovf2 {
    #[inline(always)]
    fn from(val: u8) -> Rangeovf2 {
        Rangeovf2::from_bits(val)
    }
}
impl From<Rangeovf2> for u8 {
    #[inline(always)]
    fn from(val: Rangeovf2) -> u8 {
        Rangeovf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeovf3 {
    #[doc = "No exception by range overflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range overflow"]
    EXCEPTION = 0x01,
}
impl Rangeovf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeovf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeovf3 {
    #[inline(always)]
    fn from(val: u8) -> Rangeovf3 {
        Rangeovf3::from_bits(val)
    }
}
impl From<Rangeovf3> for u8 {
    #[inline(always)]
    fn from(val: Rangeovf3) -> u8 {
        Rangeovf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeunf0 {
    #[doc = "No exception by range underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range underflow"]
    EXCEPTION = 0x01,
}
impl Rangeunf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeunf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeunf0 {
    #[inline(always)]
    fn from(val: u8) -> Rangeunf0 {
        Rangeunf0::from_bits(val)
    }
}
impl From<Rangeunf0> for u8 {
    #[inline(always)]
    fn from(val: Rangeunf0) -> u8 {
        Rangeunf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeunf1 {
    #[doc = "No exception by range underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range underflow"]
    EXCEPTION = 0x01,
}
impl Rangeunf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeunf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeunf1 {
    #[inline(always)]
    fn from(val: u8) -> Rangeunf1 {
        Rangeunf1::from_bits(val)
    }
}
impl From<Rangeunf1> for u8 {
    #[inline(always)]
    fn from(val: Rangeunf1) -> u8 {
        Rangeunf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeunf2 {
    #[doc = "No exception by range underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range underflow"]
    EXCEPTION = 0x01,
}
impl Rangeunf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeunf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeunf2 {
    #[inline(always)]
    fn from(val: u8) -> Rangeunf2 {
        Rangeunf2::from_bits(val)
    }
}
impl From<Rangeunf2> for u8 {
    #[inline(always)]
    fn from(val: Rangeunf2) -> u8 {
        Rangeunf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rangeunf3 {
    #[doc = "No exception by range underflow"]
    NO_EXCEPTION = 0x0,
    #[doc = "Exception by range underflow"]
    EXCEPTION = 0x01,
}
impl Rangeunf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rangeunf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rangeunf3 {
    #[inline(always)]
    fn from(val: u8) -> Rangeunf3 {
        Rangeunf3::from_bits(val)
    }
}
impl From<Rangeunf3> for u8 {
    #[inline(always)]
    fn from(val: Rangeunf3) -> u8 {
        Rangeunf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sres {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Software reset"]
    SW_RESET = 0x01,
}
impl Sres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sres {
    #[inline(always)]
    fn from(val: u8) -> Sres {
        Sres::from_bits(val)
    }
}
impl From<Sres> for u8 {
    #[inline(always)]
    fn from(val: Sres) -> u8 {
        Sres::to_bits(val)
    }
}
