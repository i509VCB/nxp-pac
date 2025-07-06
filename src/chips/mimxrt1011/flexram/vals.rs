#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmErrSigEn {
    #[doc = "Masked"]
    DTCM_ERR_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    DTCM_ERR_SIG_EN_1 = 0x01,
}
impl DtcmErrSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmErrSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmErrSigEn {
    #[inline(always)]
    fn from(val: u8) -> DtcmErrSigEn {
        DtcmErrSigEn::from_bits(val)
    }
}
impl From<DtcmErrSigEn> for u8 {
    #[inline(always)]
    fn from(val: DtcmErrSigEn) -> u8 {
        DtcmErrSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmErrStatEn {
    #[doc = "Masked"]
    DTCM_ERR_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    DTCM_ERR_STAT_EN_1 = 0x01,
}
impl DtcmErrStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmErrStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmErrStatEn {
    #[inline(always)]
    fn from(val: u8) -> DtcmErrStatEn {
        DtcmErrStatEn::from_bits(val)
    }
}
impl From<DtcmErrStatEn> for u8 {
    #[inline(always)]
    fn from(val: DtcmErrStatEn) -> u8 {
        DtcmErrStatEn::to_bits(val)
    }
}
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
pub enum DtcmMamSigEn {
    #[doc = "Masked"]
    DTCM_MAM_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    DTCM_MAM_SIG_EN_1 = 0x01,
}
impl DtcmMamSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmMamSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmMamSigEn {
    #[inline(always)]
    fn from(val: u8) -> DtcmMamSigEn {
        DtcmMamSigEn::from_bits(val)
    }
}
impl From<DtcmMamSigEn> for u8 {
    #[inline(always)]
    fn from(val: DtcmMamSigEn) -> u8 {
        DtcmMamSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmMamStatEn {
    #[doc = "Masked"]
    DTCM_MAM_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    DTCM_MAM_STAT_EN_1 = 0x01,
}
impl DtcmMamStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtcmMamStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtcmMamStatEn {
    #[inline(always)]
    fn from(val: u8) -> DtcmMamStatEn {
        DtcmMamStatEn::from_bits(val)
    }
}
impl From<DtcmMamStatEn> for u8 {
    #[inline(always)]
    fn from(val: DtcmMamStatEn) -> u8 {
        DtcmMamStatEn::to_bits(val)
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
pub enum ItcmErrSigEn {
    #[doc = "Masked"]
    ITCM_ERR_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    ITCM_ERR_SIG_EN_1 = 0x01,
}
impl ItcmErrSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmErrSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmErrSigEn {
    #[inline(always)]
    fn from(val: u8) -> ItcmErrSigEn {
        ItcmErrSigEn::from_bits(val)
    }
}
impl From<ItcmErrSigEn> for u8 {
    #[inline(always)]
    fn from(val: ItcmErrSigEn) -> u8 {
        ItcmErrSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ItcmErrStatEn {
    #[doc = "Masked"]
    ITCM_ERR_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    ITCM_ERR_STAT_EN_1 = 0x01,
}
impl ItcmErrStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmErrStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmErrStatEn {
    #[inline(always)]
    fn from(val: u8) -> ItcmErrStatEn {
        ItcmErrStatEn::from_bits(val)
    }
}
impl From<ItcmErrStatEn> for u8 {
    #[inline(always)]
    fn from(val: ItcmErrStatEn) -> u8 {
        ItcmErrStatEn::to_bits(val)
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
pub enum ItcmMamSigEn {
    #[doc = "Masked"]
    ITCM_MAM_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    ITCM_MAM_SIG_EN_1 = 0x01,
}
impl ItcmMamSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmMamSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmMamSigEn {
    #[inline(always)]
    fn from(val: u8) -> ItcmMamSigEn {
        ItcmMamSigEn::from_bits(val)
    }
}
impl From<ItcmMamSigEn> for u8 {
    #[inline(always)]
    fn from(val: ItcmMamSigEn) -> u8 {
        ItcmMamSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ItcmMamStatEn {
    #[doc = "Masked"]
    ITCM_MAM_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    ITCM_MAM_STAT_EN_1 = 0x01,
}
impl ItcmMamStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ItcmMamStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ItcmMamStatEn {
    #[inline(always)]
    fn from(val: u8) -> ItcmMamStatEn {
        ItcmMamStatEn::from_bits(val)
    }
}
impl From<ItcmMamStatEn> for u8 {
    #[inline(always)]
    fn from(val: ItcmMamStatEn) -> u8 {
        ItcmMamStatEn::to_bits(val)
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
pub enum OcramErrSigEn {
    #[doc = "Masked"]
    OCRAM_ERR_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    OCRAM_ERR_SIG_EN_1 = 0x01,
}
impl OcramErrSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramErrSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramErrSigEn {
    #[inline(always)]
    fn from(val: u8) -> OcramErrSigEn {
        OcramErrSigEn::from_bits(val)
    }
}
impl From<OcramErrSigEn> for u8 {
    #[inline(always)]
    fn from(val: OcramErrSigEn) -> u8 {
        OcramErrSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramErrStatEn {
    #[doc = "Masked"]
    OCRAM_ERR_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    OCRAM_ERR_STAT_EN_1 = 0x01,
}
impl OcramErrStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramErrStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramErrStatEn {
    #[inline(always)]
    fn from(val: u8) -> OcramErrStatEn {
        OcramErrStatEn::from_bits(val)
    }
}
impl From<OcramErrStatEn> for u8 {
    #[inline(always)]
    fn from(val: OcramErrStatEn) -> u8 {
        OcramErrStatEn::to_bits(val)
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
pub enum OcramMamSigEn {
    #[doc = "Masked"]
    OCRAM_MAM_SIG_EN_0 = 0x0,
    #[doc = "Enabled"]
    OCRAM_MAM_SIG_EN_1 = 0x01,
}
impl OcramMamSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramMamSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramMamSigEn {
    #[inline(always)]
    fn from(val: u8) -> OcramMamSigEn {
        OcramMamSigEn::from_bits(val)
    }
}
impl From<OcramMamSigEn> for u8 {
    #[inline(always)]
    fn from(val: OcramMamSigEn) -> u8 {
        OcramMamSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramMamStatEn {
    #[doc = "Masked"]
    OCRAM_MAM_STAT_EN_0 = 0x0,
    #[doc = "Enabled"]
    OCRAM_MAM_STAT_EN_1 = 0x01,
}
impl OcramMamStatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramMamStatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramMamStatEn {
    #[inline(always)]
    fn from(val: u8) -> OcramMamStatEn {
        OcramMamStatEn::from_bits(val)
    }
}
impl From<OcramMamStatEn> for u8 {
    #[inline(always)]
    fn from(val: OcramMamStatEn) -> u8 {
        OcramMamStatEn::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcmRwaitEn {
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TCM_RWAIT_EN_0 = 0x0,
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TCM_RWAIT_EN_1 = 0x01,
}
impl TcmRwaitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcmRwaitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcmRwaitEn {
    #[inline(always)]
    fn from(val: u8) -> TcmRwaitEn {
        TcmRwaitEn::from_bits(val)
    }
}
impl From<TcmRwaitEn> for u8 {
    #[inline(always)]
    fn from(val: TcmRwaitEn) -> u8 {
        TcmRwaitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcmWwaitEn {
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    TCM_WWAIT_EN_0 = 0x0,
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TCM_WWAIT_EN_1 = 0x01,
}
impl TcmWwaitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcmWwaitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcmWwaitEn {
    #[inline(always)]
    fn from(val: u8) -> TcmWwaitEn {
        TcmWwaitEn::from_bits(val)
    }
}
impl From<TcmWwaitEn> for u8 {
    #[inline(always)]
    fn from(val: TcmWwaitEn) -> u8 {
        TcmWwaitEn::to_bits(val)
    }
}
