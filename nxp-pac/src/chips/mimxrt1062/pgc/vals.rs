#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuCtrlPcr {
    #[doc = "Do not switch off power even if pdn_req is asserted."]
    PCR_0 = 0x0,
    #[doc = "Switch off power when pdn_req is asserted."]
    PCR_1 = 0x01,
}
impl CpuCtrlPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpuCtrlPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpuCtrlPcr {
    #[inline(always)]
    fn from(val: u8) -> CpuCtrlPcr {
        CpuCtrlPcr::from_bits(val)
    }
}
impl From<CpuCtrlPcr> for u8 {
    #[inline(always)]
    fn from(val: CpuCtrlPcr) -> u8 {
        CpuCtrlPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuSrPsr {
    #[doc = "The target subsystem was not powered down for the previous power-down request."]
    PSR_0 = 0x0,
    #[doc = "The target subsystem was powered down for the previous power-down request."]
    PSR_1 = 0x01,
}
impl CpuSrPsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpuSrPsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpuSrPsr {
    #[inline(always)]
    fn from(val: u8) -> CpuSrPsr {
        CpuSrPsr::from_bits(val)
    }
}
impl From<CpuSrPsr> for u8 {
    #[inline(always)]
    fn from(val: CpuSrPsr) -> u8 {
        CpuSrPsr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MegaCtrlPcr {
    #[doc = "Do not switch off power even if pdn_req is asserted."]
    PCR_0 = 0x0,
    #[doc = "Switch off power when pdn_req is asserted."]
    PCR_1 = 0x01,
}
impl MegaCtrlPcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MegaCtrlPcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MegaCtrlPcr {
    #[inline(always)]
    fn from(val: u8) -> MegaCtrlPcr {
        MegaCtrlPcr::from_bits(val)
    }
}
impl From<MegaCtrlPcr> for u8 {
    #[inline(always)]
    fn from(val: MegaCtrlPcr) -> u8 {
        MegaCtrlPcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MegaSrPsr {
    #[doc = "The target subsystem was not powered down for the previous power-down request."]
    PSR_0 = 0x0,
    #[doc = "The target subsystem was powered down for the previous power-down request."]
    PSR_1 = 0x01,
}
impl MegaSrPsr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MegaSrPsr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MegaSrPsr {
    #[inline(always)]
    fn from(val: u8) -> MegaSrPsr {
        MegaSrPsr::from_bits(val)
    }
}
impl From<MegaSrPsr> for u8 {
    #[inline(always)]
    fn from(val: MegaSrPsr) -> u8 {
        MegaSrPsr::to_bits(val)
    }
}
