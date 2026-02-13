#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Activefsm {
    #[doc = "All PMC finite state machines are idle. OK to set APPLYCFG to trigger the PMC state machines."]
    ACTIVEFSM_0 = 0x0,
    #[doc = "One or more PMC finite state machines are active, do not set APPLYCFG or write to any PDRUNCFG or CTRL register values that are used by the PMC state machines."]
    ACTIVEFSM_1 = 0x01,
}
impl Activefsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Activefsm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Activefsm {
    #[inline(always)]
    fn from(val: u8) -> Activefsm {
        Activefsm::from_bits(val)
    }
}
impl From<Activefsm> for u8 {
    #[inline(always)]
    fn from(val: Activefsm) -> u8 {
        Activefsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Applycfg {
    #[doc = "Always reads 0. Write 0 has no effect"]
    APPLYCFG_0 = 0x0,
    #[doc = "Write 1 = initiate update sequencing of PMC state machines"]
    APPLYCFG_1 = 0x01,
}
impl Applycfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Applycfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Applycfg {
    #[inline(always)]
    fn from(val: u8) -> Applycfg {
        Applycfg::from_bits(val)
    }
}
impl From<Applycfg> for u8 {
    #[inline(always)]
    fn from(val: Applycfg) -> u8 {
        Applycfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autowken {
    #[doc = "Auto wakeup interrupt and counter disabled"]
    AUTOWKEN_0 = 0x0,
    #[doc = "Auto wakeup interrupt generated when PMC sequencer finishes and AUTOWAKE counter = 0 after entering deep sleep mode (but not deep powerdown mode). Interrupt will wake up the M33."]
    AUTOWKEN_1 = 0x01,
}
impl Autowken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autowken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autowken {
    #[inline(always)]
    fn from(val: u8) -> Autowken {
        Autowken::from_bits(val)
    }
}
impl From<Autowken> for u8 {
    #[inline(always)]
    fn from(val: Autowken) -> u8 {
        Autowken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autowkf {
    #[doc = "No PMC Auto Wakeup Interrupt detected since last time cleared."]
    AUTOWKF_0 = 0x0,
    #[doc = "PMC Auto wakeup caused a deep sleep wakeup and interrupt. Write 1 to clear."]
    AUTOWKF_1 = 0x01,
}
impl Autowkf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autowkf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autowkf {
    #[inline(always)]
    fn from(val: u8) -> Autowkf {
        Autowkf::from_bits(val)
    }
}
impl From<Autowkf> for u8 {
    #[inline(always)]
    fn from(val: Autowkf) -> u8 {
        Autowkf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufen {
    #[doc = "disabled"]
    BUFEN_0 = 0x0,
    #[doc = "enabled"]
    BUFEN_1 = 0x01,
}
impl Bufen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufen {
    #[inline(always)]
    fn from(val: u8) -> Bufen {
        Bufen::from_bits(val)
    }
}
impl From<Bufen> for u8 {
    #[inline(always)]
    fn from(val: Bufen) -> u8 {
        Bufen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Deeppdf {
    #[doc = "No deep powerdown wakeup since last time flag was cleared."]
    DEEPPDF_0 = 0x0,
    #[doc = "Deep powerdown was entered since the last time this flag was cleared. Write 1 to clear"]
    DEEPPDF_1 = 0x01,
}
impl Deeppdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Deeppdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Deeppdf {
    #[inline(always)]
    fn from(val: u8) -> Deeppdf {
        Deeppdf::from_bits(val)
    }
}
impl From<Deeppdf> for u8 {
    #[inline(always)]
    fn from(val: Deeppdf) -> u8 {
        Deeppdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvd1v8f {
    #[doc = "vdd1v8 HVD has not triggered an interrupt or reset since last clear"]
    HVD1V8F_0 = 0x0,
    #[doc = "vdd1v8 HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    HVD1V8F_1 = 0x01,
}
impl Hvd1v8f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvd1v8f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvd1v8f {
    #[inline(always)]
    fn from(val: u8) -> Hvd1v8f {
        Hvd1v8f::from_bits(val)
    }
}
impl From<Hvd1v8f> for u8 {
    #[inline(always)]
    fn from(val: Hvd1v8f) -> u8 {
        Hvd1v8f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvd1v8ie {
    #[doc = "vdd1v8 HVD interrupt disabled"]
    HVD1V8IE_0 = 0x0,
    #[doc = "vdd1v8 HVD causes interrupt and wakeup from deep sleep or deep power down mode"]
    HVD1V8IE_1 = 0x01,
}
impl Hvd1v8ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvd1v8ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvd1v8ie {
    #[inline(always)]
    fn from(val: u8) -> Hvd1v8ie {
        Hvd1v8ie::from_bits(val)
    }
}
impl From<Hvd1v8ie> for u8 {
    #[inline(always)]
    fn from(val: Hvd1v8ie) -> u8 {
        Hvd1v8ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvd1v8re {
    #[doc = "vdd1v8 HVD reset disabled"]
    HVD1V8RE_0 = 0x0,
    #[doc = "vdd1v8 HVD causes reset"]
    HVD1V8RE_1 = 0x01,
}
impl Hvd1v8re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvd1v8re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvd1v8re {
    #[inline(always)]
    fn from(val: u8) -> Hvd1v8re {
        Hvd1v8re::from_bits(val)
    }
}
impl From<Hvd1v8re> for u8 {
    #[inline(always)]
    fn from(val: Hvd1v8re) -> u8 {
        Hvd1v8re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvdcoref {
    #[doc = "vddcore HVD has not triggered an interrupt or reset since last clear"]
    HVDCOREF_0 = 0x0,
    #[doc = "vddcore HVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    HVDCOREF_1 = 0x01,
}
impl Hvdcoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvdcoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvdcoref {
    #[inline(always)]
    fn from(val: u8) -> Hvdcoref {
        Hvdcoref::from_bits(val)
    }
}
impl From<Hvdcoref> for u8 {
    #[inline(always)]
    fn from(val: Hvdcoref) -> u8 {
        Hvdcoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvdcoreie {
    #[doc = "vddcore HVD interrupt disabled"]
    HVDCOREIE_0 = 0x0,
    #[doc = "vddcore HVD causes interrupt and wakeup from deep sleep."]
    HVDCOREIE_1 = 0x01,
}
impl Hvdcoreie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvdcoreie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvdcoreie {
    #[inline(always)]
    fn from(val: u8) -> Hvdcoreie {
        Hvdcoreie::from_bits(val)
    }
}
impl From<Hvdcoreie> for u8 {
    #[inline(always)]
    fn from(val: Hvdcoreie) -> u8 {
        Hvdcoreie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvdcorere {
    #[doc = "vddcore HVD reset disabled"]
    HVDCORERE_0 = 0x0,
    #[doc = "vddcore HVD causes reset"]
    HVDCORERE_1 = 0x01,
}
impl Hvdcorere {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvdcorere {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvdcorere {
    #[inline(always)]
    fn from(val: u8) -> Hvdcorere {
        Hvdcorere::from_bits(val)
    }
}
impl From<Hvdcorere> for u8 {
    #[inline(always)]
    fn from(val: Hvdcorere) -> u8 {
        Hvdcorere::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intnpadf {
    #[doc = "No interrupt detected since flag last cleared."]
    INTNPADF_0 = 0x0,
    #[doc = "Pad interrupt caused a wakeup or interrupt event since the last time this flag was cleared. Write 1 to clear."]
    INTNPADF_1 = 0x01,
}
impl Intnpadf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intnpadf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intnpadf {
    #[inline(always)]
    fn from(val: u8) -> Intnpadf {
        Intnpadf::from_bits(val)
    }
}
impl From<Intnpadf> for u8 {
    #[inline(always)]
    fn from(val: Intnpadf) -> u8 {
        Intnpadf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intrpaden {
    #[doc = "Interrupt pad low has no effect"]
    INTRPADEN_0 = 0x0,
    #[doc = "Interrupt pad low triggers an interrupt and deep sleep wakeup or deep powerdown wakeup event."]
    INTRPADEN_1 = 0x01,
}
impl Intrpaden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intrpaden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intrpaden {
    #[inline(always)]
    fn from(val: u8) -> Intrpaden {
        Intrpaden::from_bits(val)
    }
}
impl From<Intrpaden> for u8 {
    #[inline(always)]
    fn from(val: Intrpaden) -> u8 {
        Intrpaden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvdcoref {
    #[doc = "vddcore LVD has not triggered an interrupt or reset since last clear"]
    LVDCOREF_0 = 0x0,
    #[doc = "vddcore LVD triggered an interrupt or reset since last time this bit was cleared. Write 1 to clear"]
    LVDCOREF_1 = 0x01,
}
impl Lvdcoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvdcoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvdcoref {
    #[inline(always)]
    fn from(val: u8) -> Lvdcoref {
        Lvdcoref::from_bits(val)
    }
}
impl From<Lvdcoref> for u8 {
    #[inline(always)]
    fn from(val: Lvdcoref) -> u8 {
        Lvdcoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvdcoreie {
    #[doc = "vddcore LVD interrupt disabled"]
    LVDCOREIE_0 = 0x0,
    #[doc = "vddcore LVD causes interrupt and wakeup from deep sleep."]
    LVDCOREIE_1 = 0x01,
}
impl Lvdcoreie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvdcoreie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvdcoreie {
    #[inline(always)]
    fn from(val: u8) -> Lvdcoreie {
        Lvdcoreie::from_bits(val)
    }
}
impl From<Lvdcoreie> for u8 {
    #[inline(always)]
    fn from(val: Lvdcoreie) -> u8 {
        Lvdcoreie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvdcorere {
    #[doc = "vddcore LVD reset disabled"]
    LVDCORERE_0 = 0x0,
    #[doc = "vddcore LVD causes reset"]
    LVDCORERE_1 = 0x01,
}
impl Lvdcorere {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvdcorere {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvdcorere {
    #[inline(always)]
    fn from(val: u8) -> Lvdcorere {
        Lvdcorere::from_bits(val)
    }
}
impl From<Lvdcorere> for u8 {
    #[inline(always)]
    fn from(val: Lvdcorere) -> u8 {
        Lvdcorere::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Por1v8f {
    #[doc = "No vdd1v8 power on event detected since last cleared."]
    POR1V8F_0 = 0x0,
    #[doc = "vdd1v8 power on detect caused a reset or deep pd wakeup. Write 1 to clear."]
    POR1V8F_1 = 0x01,
}
impl Por1v8f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Por1v8f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Por1v8f {
    #[inline(always)]
    fn from(val: u8) -> Por1v8f {
        Por1v8f::from_bits(val)
    }
}
impl From<Por1v8f> for u8 {
    #[inline(always)]
    fn from(val: Por1v8f) -> u8 {
        Por1v8f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Porao18f {
    #[doc = "No vdd_ao18 power on event detected since last cleared."]
    PORAO18F_0 = 0x0,
    #[doc = "vdd_ao18 power on detect caused a reset. Write 1 to clear."]
    PORAO18F_1 = 0x01,
}
impl Porao18f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Porao18f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Porao18f {
    #[inline(always)]
    fn from(val: u8) -> Porao18f {
        Porao18f::from_bits(val)
    }
}
impl From<Porao18f> for u8 {
    #[inline(always)]
    fn from(val: Porao18f) -> u8 {
        Porao18f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Porcoref {
    #[doc = "vddcore POR was not tripped since the last cleared."]
    PORCOREF_0 = 0x0,
    #[doc = "POR triggered by the vddcore POR monitor. Write 1 to clear"]
    PORCOREF_1 = 0x01,
}
impl Porcoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Porcoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Porcoref {
    #[inline(always)]
    fn from(val: u8) -> Porcoref {
        Porcoref::from_bits(val)
    }
}
impl From<Porcoref> for u8 {
    #[inline(always)]
    fn from(val: Porcoref) -> u8 {
        Porcoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resetnpadf {
    #[doc = "No reset detected since last time this flag was cleared."]
    RESETNPADF_0 = 0x0,
    #[doc = "Reset pad wakeup caused a wakeup or reset event since the last time this bit was cleared. Write 1 to clear."]
    RESETNPADF_1 = 0x01,
}
impl Resetnpadf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resetnpadf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resetnpadf {
    #[inline(always)]
    fn from(val: u8) -> Resetnpadf {
        Resetnpadf::from_bits(val)
    }
}
impl From<Resetnpadf> for u8 {
    #[inline(always)]
    fn from(val: Resetnpadf) -> u8 {
        Resetnpadf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtcf {
    #[doc = "No RTC wakeup detected since last time flag was cleared."]
    RTCF_0 = 0x0,
    #[doc = "RTC wakeup caused a deep powerdown wakeup. Write 1 to clear."]
    RTCF_1 = 0x01,
}
impl Rtcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtcf {
    #[inline(always)]
    fn from(val: u8) -> Rtcf {
        Rtcf::from_bits(val)
    }
}
impl From<Rtcf> for u8 {
    #[inline(always)]
    fn from(val: Rtcf) -> u8 {
        Rtcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdd1v8m0 {
    #[doc = "off"]
    VDD1V8M0_0 = 0x0,
    #[doc = "powered"]
    VDD1V8M0_1 = 0x01,
}
impl Vdd1v8m0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdd1v8m0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdd1v8m0 {
    #[inline(always)]
    fn from(val: u8) -> Vdd1v8m0 {
        Vdd1v8m0::from_bits(val)
    }
}
impl From<Vdd1v8m0> for u8 {
    #[inline(always)]
    fn from(val: Vdd1v8m0) -> u8 {
        Vdd1v8m0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdd1v8m1 {
    #[doc = "off"]
    VDD1V8M1_0 = 0x0,
    #[doc = "powered"]
    VDD1V8M1_1 = 0x01,
}
impl Vdd1v8m1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdd1v8m1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdd1v8m1 {
    #[inline(always)]
    fn from(val: u8) -> Vdd1v8m1 {
        Vdd1v8m1::from_bits(val)
    }
}
impl From<Vdd1v8m1> for u8 {
    #[inline(always)]
    fn from(val: Vdd1v8m1) -> u8 {
        Vdd1v8m1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdd1v8m2 {
    #[doc = "off"]
    VDD1V8M2_0 = 0x0,
    #[doc = "powered"]
    VDD1V8M2_1 = 0x01,
}
impl Vdd1v8m2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdd1v8m2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdd1v8m2 {
    #[inline(always)]
    fn from(val: u8) -> Vdd1v8m2 {
        Vdd1v8m2::from_bits(val)
    }
}
impl From<Vdd1v8m2> for u8 {
    #[inline(always)]
    fn from(val: Vdd1v8m2) -> u8 {
        Vdd1v8m2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdd1v8m3 {
    #[doc = "off"]
    VDD1V8M3_0 = 0x0,
    #[doc = "powered"]
    VDD1V8M3_1 = 0x01,
}
impl Vdd1v8m3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdd1v8m3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdd1v8m3 {
    #[inline(always)]
    fn from(val: u8) -> Vdd1v8m3 {
        Vdd1v8m3::from_bits(val)
    }
}
impl From<Vdd1v8m3> for u8 {
    #[inline(always)]
    fn from(val: Vdd1v8m3) -> u8 {
        Vdd1v8m3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddcorem0 {
    #[doc = "off"]
    VDDCOREM0_0 = 0x0,
    #[doc = "powered"]
    VDDCOREM0_1 = 0x01,
}
impl Vddcorem0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddcorem0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddcorem0 {
    #[inline(always)]
    fn from(val: u8) -> Vddcorem0 {
        Vddcorem0::from_bits(val)
    }
}
impl From<Vddcorem0> for u8 {
    #[inline(always)]
    fn from(val: Vddcorem0) -> u8 {
        Vddcorem0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddcorem1 {
    #[doc = "off"]
    VDDCOREM1_0 = 0x0,
    #[doc = "powered"]
    VDDCOREM1_1 = 0x01,
}
impl Vddcorem1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddcorem1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddcorem1 {
    #[inline(always)]
    fn from(val: u8) -> Vddcorem1 {
        Vddcorem1::from_bits(val)
    }
}
impl From<Vddcorem1> for u8 {
    #[inline(always)]
    fn from(val: Vddcorem1) -> u8 {
        Vddcorem1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddcorem2 {
    #[doc = "off"]
    VDDCOREM2_0 = 0x0,
    #[doc = "powered"]
    VDDCOREM2_1 = 0x01,
}
impl Vddcorem2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddcorem2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddcorem2 {
    #[inline(always)]
    fn from(val: u8) -> Vddcorem2 {
        Vddcorem2::from_bits(val)
    }
}
impl From<Vddcorem2> for u8 {
    #[inline(always)]
    fn from(val: Vddcorem2) -> u8 {
        Vddcorem2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddcorem3 {
    #[doc = "off"]
    VDDCOREM3_0 = 0x0,
    #[doc = "powered"]
    VDDCOREM3_1 = 0x01,
}
impl Vddcorem3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddcorem3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddcorem3 {
    #[inline(always)]
    fn from(val: u8) -> Vddcorem3 {
        Vddcorem3::from_bits(val)
    }
}
impl From<Vddcorem3> for u8 {
    #[inline(always)]
    fn from(val: Vddcorem3) -> u8 {
        Vddcorem3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddio0range {
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE0 level"]
    VDDIO_0RANGE_0 = 0x0,
    #[doc = "1.71 - 1.98V, vdde detector off"]
    VDDIO_0RANGE_1 = 0x01,
    #[doc = "3.00 - 3.6V, vdde detector off"]
    VDDIO_0RANGE_2 = 0x02,
    #[doc = "Not allowed (hardware should translate to 10)"]
    VDDIO_0RANGE_3 = 0x03,
}
impl Vddio0range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddio0range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddio0range {
    #[inline(always)]
    fn from(val: u8) -> Vddio0range {
        Vddio0range::from_bits(val)
    }
}
impl From<Vddio0range> for u8 {
    #[inline(always)]
    fn from(val: Vddio0range) -> u8 {
        Vddio0range::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddio1range {
    #[doc = "1.71-3.6V. Consumes static current to detect VDDE1 level"]
    VDDIO_1RANGE_0 = 0x0,
    #[doc = "1.71 - 1.98V, vdde detector off"]
    VDDIO_1RANGE_1 = 0x01,
    #[doc = "3.00 - 3.6V, vdde detector off"]
    VDDIO_1RANGE_2 = 0x02,
    #[doc = "Not allowed (hardware should translate to 10)"]
    VDDIO_1RANGE_3 = 0x03,
}
impl Vddio1range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddio1range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddio1range {
    #[inline(always)]
    fn from(val: u8) -> Vddio1range {
        Vddio1range::from_bits(val)
    }
}
impl From<Vddio1range> for u8 {
    #[inline(always)]
    fn from(val: Vddio1range) -> u8 {
        Vddio1range::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vddio2range {
    #[doc = "1.71 - 3.6V. Consumes static current to detect VDDE2 level"]
    VDDIO_2RANGE_0 = 0x0,
    #[doc = "1.71 - 1.98V, vdde detector off"]
    VDDIO_2RANGE_1 = 0x01,
    #[doc = "3.00 - 3.6V, vdde detector off"]
    VDDIO_2RANGE_2 = 0x02,
    #[doc = "Not allowed (hardware should translate to 10)"]
    VDDIO_2RANGE_3 = 0x03,
}
impl Vddio2range {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vddio2range {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vddio2range {
    #[inline(always)]
    fn from(val: u8) -> Vddio2range {
        Vddio2range::from_bits(val)
    }
}
impl From<Vddio2range> for u8 {
    #[inline(always)]
    fn from(val: Vddio2range) -> u8 {
        Vddio2range::to_bits(val)
    }
}
