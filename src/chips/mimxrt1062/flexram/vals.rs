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
