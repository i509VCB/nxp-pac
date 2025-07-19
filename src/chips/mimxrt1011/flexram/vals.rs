#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmErrStatus {
    #[doc = "DTCM access error does not happen"]
    DTCM_ERR_STATUS_0 = 0x0,
    #[doc = "DTCM access error happens."]
    DTCM_ERR_STATUS_1 = 0x01,
}
impl DtcmErrStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmErrStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmErrStatus {
    #[inline(always)]
    fn from(val: u8) -> DtcmErrStatus {
        DtcmErrStatus::from_bits(val)
    }
}
impl From<DtcmErrStatus> for u8 {
    #[inline(always)]
    fn from(val: DtcmErrStatus) -> u8 {
        DtcmErrStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmMamStatus {
    #[doc = "DTCM did not access magic address."]
    DTCM_MAM_STATUS_0 = 0x0,
    #[doc = "DTCM accessed magic address."]
    DTCM_MAM_STATUS_1 = 0x01,
}
impl DtcmMamStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmMamStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmMamStatus {
    #[inline(always)]
    fn from(val: u8) -> DtcmMamStatus {
        DtcmMamStatus::from_bits(val)
    }
}
impl From<DtcmMamStatus> for u8 {
    #[inline(always)]
    fn from(val: DtcmMamStatus) -> u8 {
        DtcmMamStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmWrRdSel {
    #[doc = "When DTCM read access hits magic address, it will generate interrupt."]
    DTCM_WR_RD_SEL_0 = 0x0,
    #[doc = "When DTCM write access hits magic address, it will generate interrupt."]
    DTCM_WR_RD_SEL_1 = 0x01,
}
impl DtcmWrRdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmWrRdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmWrRdSel {
    #[inline(always)]
    fn from(val: u8) -> DtcmWrRdSel {
        DtcmWrRdSel::from_bits(val)
    }
}
impl From<DtcmWrRdSel> for u8 {
    #[inline(always)]
    fn from(val: DtcmWrRdSel) -> u8 {
        DtcmWrRdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ItcmErrStatus {
    #[doc = "ITCM access error does not happen"]
    ITCM_ERR_STATUS_0 = 0x0,
    #[doc = "ITCM access error happens."]
    ITCM_ERR_STATUS_1 = 0x01,
}
impl ItcmErrStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmErrStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmErrStatus {
    #[inline(always)]
    fn from(val: u8) -> ItcmErrStatus {
        ItcmErrStatus::from_bits(val)
    }
}
impl From<ItcmErrStatus> for u8 {
    #[inline(always)]
    fn from(val: ItcmErrStatus) -> u8 {
        ItcmErrStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ItcmMamStatus {
    #[doc = "ITCM did not access magic address."]
    ITCM_MAM_STATUS_0 = 0x0,
    #[doc = "ITCM accessed magic address."]
    ITCM_MAM_STATUS_1 = 0x01,
}
impl ItcmMamStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmMamStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmMamStatus {
    #[inline(always)]
    fn from(val: u8) -> ItcmMamStatus {
        ItcmMamStatus::from_bits(val)
    }
}
impl From<ItcmMamStatus> for u8 {
    #[inline(always)]
    fn from(val: ItcmMamStatus) -> u8 {
        ItcmMamStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ItcmWrRdSel {
    #[doc = "When ITCM read access hits magic address, it will generate interrupt."]
    ITCM_WR_RD_SEL_0 = 0x0,
    #[doc = "When ITCM write access hits magic address, it will generate interrupt."]
    ITCM_WR_RD_SEL_1 = 0x01,
}
impl ItcmWrRdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmWrRdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmWrRdSel {
    #[inline(always)]
    fn from(val: u8) -> ItcmWrRdSel {
        ItcmWrRdSel::from_bits(val)
    }
}
impl From<ItcmWrRdSel> for u8 {
    #[inline(always)]
    fn from(val: ItcmWrRdSel) -> u8 {
        ItcmWrRdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramErrStatus {
    #[doc = "OCRAM access error does not happen"]
    OCRAM_ERR_STATUS_0 = 0x0,
    #[doc = "OCRAM access error happens."]
    OCRAM_ERR_STATUS_1 = 0x01,
}
impl OcramErrStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramErrStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramErrStatus {
    #[inline(always)]
    fn from(val: u8) -> OcramErrStatus {
        OcramErrStatus::from_bits(val)
    }
}
impl From<OcramErrStatus> for u8 {
    #[inline(always)]
    fn from(val: OcramErrStatus) -> u8 {
        OcramErrStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramMamStatus {
    #[doc = "OCRAM did not access magic address."]
    OCRAM_MAM_STATUS_0 = 0x0,
    #[doc = "OCRAM accessed magic address."]
    OCRAM_MAM_STATUS_1 = 0x01,
}
impl OcramMamStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramMamStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramMamStatus {
    #[inline(always)]
    fn from(val: u8) -> OcramMamStatus {
        OcramMamStatus::from_bits(val)
    }
}
impl From<OcramMamStatus> for u8 {
    #[inline(always)]
    fn from(val: OcramMamStatus) -> u8 {
        OcramMamStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramWrRdSel {
    #[doc = "When OCRAM read access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_0 = 0x0,
    #[doc = "When OCRAM write access hits magic address, it will generate interrupt."]
    OCRAM_WR_RD_SEL_1 = 0x01,
}
impl OcramWrRdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramWrRdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramWrRdSel {
    #[inline(always)]
    fn from(val: u8) -> OcramWrRdSel {
        OcramWrRdSel::from_bits(val)
    }
}
impl From<OcramWrRdSel> for u8 {
    #[inline(always)]
    fn from(val: OcramWrRdSel) -> u8 {
        OcramWrRdSel::to_bits(val)
    }
}
