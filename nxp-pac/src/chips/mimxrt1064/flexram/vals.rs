#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtcmErrSigEn {
    #[doc = "Masked."]
    DtcmErrSigEn0 = 0x0,
    #[doc = "Enabled."]
    DtcmErrSigEn1 = 0x01,
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
    #[doc = "Masked."]
    DtcmErrStatEn0 = 0x0,
    #[doc = "Enabled."]
    DtcmErrStatEn1 = 0x01,
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
    #[doc = "DTCM access error does not happen."]
    DtcmErrStatus0 = 0x0,
    #[doc = "DTCM access error happens."]
    DtcmErrStatus1 = 0x01,
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
pub enum ItcmErrSigEn {
    #[doc = "Masked."]
    ItcmErrSigEn0 = 0x0,
    #[doc = "Enabled."]
    ItcmErrSigEn1 = 0x01,
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
    #[doc = "Masked."]
    ItcmErrStatEn0 = 0x0,
    #[doc = "Enabled."]
    ItcmErrStatEn1 = 0x01,
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
    #[doc = "ITCM access error does not happen."]
    ItcmErrStatus0 = 0x0,
    #[doc = "ITCM access error happens."]
    ItcmErrStatus1 = 0x01,
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
pub enum OcramErrSigEn {
    #[doc = "Masked."]
    OcramErrSigEn0 = 0x0,
    #[doc = "Enabled."]
    OcramErrSigEn1 = 0x01,
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
    #[doc = "Masked."]
    OcramErrStatEn0 = 0x0,
    #[doc = "Enabled."]
    OcramErrStatEn1 = 0x01,
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
    #[doc = "OCRAM access error does not happen."]
    OcramErrStatus0 = 0x0,
    #[doc = "OCRAM access error happens."]
    OcramErrStatus1 = 0x01,
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
pub enum TcmRwaitEn {
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TcmRwaitEn0 = 0x0,
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TcmRwaitEn1 = 0x01,
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
    TcmWwaitEn0 = 0x0,
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TcmWwaitEn1 = 0x01,
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
