#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pde {
    #[doc = "Power Down Counter of WDOG is disabled."]
    PDE_0 = 0x0,
    #[doc = "Power Down Counter of WDOG is enabled (Default)."]
    PDE_1 = 0x01,
}
impl Pde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pde {
    #[inline(always)]
    fn from(val: u8) -> Pde {
        Pde::from_bits(val)
    }
}
impl From<Pde> for u8 {
    #[inline(always)]
    fn from(val: Pde) -> u8 {
        Pde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Por {
    #[doc = "Reset is not the result of a power on reset."]
    POR_0 = 0x0,
    #[doc = "Reset is the result of a power on reset."]
    POR_1 = 0x01,
}
impl Por {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Por {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Por {
    #[inline(always)]
    fn from(val: u8) -> Por {
        Por::from_bits(val)
    }
}
impl From<Por> for u8 {
    #[inline(always)]
    fn from(val: Por) -> u8 {
        Por::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sftw {
    #[doc = "Reset is not the result of a software reset."]
    SFTW_0 = 0x0,
    #[doc = "Reset is the result of a software reset."]
    SFTW_1 = 0x01,
}
impl Sftw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftw {
    #[inline(always)]
    fn from(val: u8) -> Sftw {
        Sftw::from_bits(val)
    }
}
impl From<Sftw> for u8 {
    #[inline(always)]
    fn from(val: Sftw) -> u8 {
        Sftw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sre {
    #[doc = "using original way to generate software reset (default)"]
    SRE_0 = 0x0,
    #[doc = "using new way to generate software reset."]
    SRE_1 = 0x01,
}
impl Sre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sre {
    #[inline(always)]
    fn from(val: u8) -> Sre {
        Sre::from_bits(val)
    }
}
impl From<Sre> for u8 {
    #[inline(always)]
    fn from(val: Sre) -> u8 {
        Sre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srs {
    #[doc = "Assert system reset signal."]
    SRS_0 = 0x0,
    #[doc = "No effect on the system (Default)."]
    SRS_1 = 0x01,
}
impl Srs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srs {
    #[inline(always)]
    fn from(val: u8) -> Srs {
        Srs::from_bits(val)
    }
}
impl From<Srs> for u8 {
    #[inline(always)]
    fn from(val: Srs) -> u8 {
        Srs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tout {
    #[doc = "Reset is not the result of a WDOG timeout."]
    TOUT_0 = 0x0,
    #[doc = "Reset is the result of a WDOG timeout."]
    TOUT_1 = 0x01,
}
impl Tout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tout {
    #[inline(always)]
    fn from(val: u8) -> Tout {
        Tout::from_bits(val)
    }
}
impl From<Tout> for u8 {
    #[inline(always)]
    fn from(val: Tout) -> u8 {
        Tout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wda {
    #[doc = "Assert WDOG_B output."]
    WDA_0 = 0x0,
    #[doc = "No effect on system (Default)."]
    WDA_1 = 0x01,
}
impl Wda {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wda {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wda {
    #[inline(always)]
    fn from(val: u8) -> Wda {
        Wda::from_bits(val)
    }
}
impl From<Wda> for u8 {
    #[inline(always)]
    fn from(val: Wda) -> u8 {
        Wda::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdbg {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDBG_0 = 0x0,
    #[doc = "Suspend the watchdog timer."]
    WDBG_1 = 0x01,
}
impl Wdbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdbg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdbg {
    #[inline(always)]
    fn from(val: u8) -> Wdbg {
        Wdbg::from_bits(val)
    }
}
impl From<Wdbg> for u8 {
    #[inline(always)]
    fn from(val: Wdbg) -> u8 {
        Wdbg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wde {
    #[doc = "Disable the Watchdog (Default)."]
    WDE_0 = 0x0,
    #[doc = "Enable the Watchdog."]
    WDE_1 = 0x01,
}
impl Wde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wde {
    #[inline(always)]
    fn from(val: u8) -> Wde {
        Wde::from_bits(val)
    }
}
impl From<Wde> for u8 {
    #[inline(always)]
    fn from(val: Wde) -> u8 {
        Wde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt {
    #[doc = "No effect on WDOG_B (Default)."]
    WDT_0 = 0x0,
    #[doc = "Assert WDOG_B upon a Watchdog Time-out event."]
    WDT_1 = 0x01,
}
impl Wdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt {
    #[inline(always)]
    fn from(val: u8) -> Wdt {
        Wdt::from_bits(val)
    }
}
impl From<Wdt> for u8 {
    #[inline(always)]
    fn from(val: Wdt) -> u8 {
        Wdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdw {
    #[doc = "Continue WDOG timer operation (Default)."]
    WDW_0 = 0x0,
    #[doc = "Suspend WDOG timer operation."]
    WDW_1 = 0x01,
}
impl Wdw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdw {
    #[inline(always)]
    fn from(val: u8) -> Wdw {
        Wdw::from_bits(val)
    }
}
impl From<Wdw> for u8 {
    #[inline(always)]
    fn from(val: Wdw) -> u8 {
        Wdw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdzst {
    #[doc = "Continue timer operation (Default)."]
    WDZST_0 = 0x0,
    #[doc = "Suspend the watchdog timer."]
    WDZST_1 = 0x01,
}
impl Wdzst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdzst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdzst {
    #[inline(always)]
    fn from(val: u8) -> Wdzst {
        Wdzst::from_bits(val)
    }
}
impl From<Wdzst> for u8 {
    #[inline(always)]
    fn from(val: Wdzst) -> u8 {
        Wdzst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Wict(u8);
impl Wict {
    #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0 seconds."]
    pub const WICT_0: Self = Self(0x0);
    #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 0.5 seconds."]
    pub const WICT_1: Self = Self(0x01);
    #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 2 seconds (Default)."]
    pub const WICT_4: Self = Self(0x04);
    #[doc = "WICT\\[7:0\\] = Time duration between interrupt and time-out is 127.5 seconds."]
    pub const WICT_255: Self = Self(0xff);
}
impl Wict {
    pub const fn from_bits(val: u8) -> Wict {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Wict {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WICT_0"),
            0x01 => f.write_str("WICT_1"),
            0x04 => f.write_str("WICT_4"),
            0xff => f.write_str("WICT_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wict {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WICT_0"),
            0x01 => defmt::write!(f, "WICT_1"),
            0x04 => defmt::write!(f, "WICT_4"),
            0xff => defmt::write!(f, "WICT_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Wict {
    #[inline(always)]
    fn from(val: u8) -> Wict {
        Wict::from_bits(val)
    }
}
impl From<Wict> for u8 {
    #[inline(always)]
    fn from(val: Wict) -> u8 {
        Wict::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wie {
    #[doc = "Disable Interrupt (Default)."]
    WIE_0 = 0x0,
    #[doc = "Enable Interrupt."]
    WIE_1 = 0x01,
}
impl Wie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wie {
    #[inline(always)]
    fn from(val: u8) -> Wie {
        Wie::from_bits(val)
    }
}
impl From<Wie> for u8 {
    #[inline(always)]
    fn from(val: Wie) -> u8 {
        Wie::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Wsr(u16);
impl Wsr {
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    pub const WSR_21845: Self = Self(0x5555);
    #[doc = "Write to the Watchdog Service Register (WDOG_WSR)."]
    pub const WSR_43690: Self = Self(0xaaaa);
}
impl Wsr {
    pub const fn from_bits(val: u16) -> Wsr {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Wsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x5555 => f.write_str("WSR_21845"),
            0xaaaa => f.write_str("WSR_43690"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wsr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x5555 => defmt::write!(f, "WSR_21845"),
            0xaaaa => defmt::write!(f, "WSR_43690"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Wsr {
    #[inline(always)]
    fn from(val: u16) -> Wsr {
        Wsr::from_bits(val)
    }
}
impl From<Wsr> for u16 {
    #[inline(always)]
    fn from(val: Wsr) -> u16 {
        Wsr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Wt(u8);
impl Wt {
    #[doc = "- 0.5 Seconds (Default)."]
    pub const WT_0: Self = Self(0x0);
    #[doc = "- 1.0 Seconds."]
    pub const WT_1: Self = Self(0x01);
    #[doc = "- 1.5 Seconds."]
    pub const WT_2: Self = Self(0x02);
    #[doc = "- 2.0 Seconds."]
    pub const WT_3: Self = Self(0x03);
    #[doc = "- 128 Seconds."]
    pub const WT_255: Self = Self(0xff);
}
impl Wt {
    pub const fn from_bits(val: u8) -> Wt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Wt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("WT_0"),
            0x01 => f.write_str("WT_1"),
            0x02 => f.write_str("WT_2"),
            0x03 => f.write_str("WT_3"),
            0xff => f.write_str("WT_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "WT_0"),
            0x01 => defmt::write!(f, "WT_1"),
            0x02 => defmt::write!(f, "WT_2"),
            0x03 => defmt::write!(f, "WT_3"),
            0xff => defmt::write!(f, "WT_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Wt {
    #[inline(always)]
    fn from(val: u8) -> Wt {
        Wt::from_bits(val)
    }
}
impl From<Wt> for u8 {
    #[inline(always)]
    fn from(val: Wt) -> u8 {
        Wt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wtis {
    #[doc = "No interrupt has occurred (Default)."]
    WTIS_0 = 0x0,
    #[doc = "Interrupt has occurred"]
    WTIS_1 = 0x01,
}
impl Wtis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wtis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wtis {
    #[inline(always)]
    fn from(val: u8) -> Wtis {
        Wtis::from_bits(val)
    }
}
impl From<Wtis> for u8 {
    #[inline(always)]
    fn from(val: Wtis) -> u8 {
        Wtis::to_bits(val)
    }
}
