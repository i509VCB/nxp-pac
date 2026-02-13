#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attach {
    #[doc = "Not detected"]
    INT_NO = 0x0,
    #[doc = "Detected"]
    INT_YES = 0x01,
}
impl Attach {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attach {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attach {
    #[inline(always)]
    fn from(val: u8) -> Attach {
        Attach::from_bits(val)
    }
}
impl From<Attach> for u8 {
    #[inline(always)]
    fn from(val: Attach) -> u8 {
        Attach::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Attachen {
    #[doc = "Disable"]
    DIS_ATTACH_INT = 0x0,
    #[doc = "Enable"]
    EN_ATTACH_INT = 0x01,
}
impl Attachen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Attachen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Attachen {
    #[inline(always)]
    fn from(val: u8) -> Attachen {
        Attachen::from_bits(val)
    }
}
impl From<Attachen> for u8 {
    #[inline(always)]
    fn from(val: Attachen) -> u8 {
        Attachen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btoerr {
    #[doc = "Not timed out"]
    INT_NO = 0x0,
    #[doc = "Timed out"]
    INT_YES = 0x01,
}
impl Btoerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btoerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btoerr {
    #[inline(always)]
    fn from(val: u8) -> Btoerr {
        Btoerr::from_bits(val)
    }
}
impl From<Btoerr> for u8 {
    #[inline(always)]
    fn from(val: Btoerr) -> u8 {
        Btoerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btoerren {
    #[doc = "Disable"]
    DIS_BTOERR_INT = 0x0,
    #[doc = "Enable"]
    EN_BTOERR_INT = 0x01,
}
impl Btoerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btoerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btoerren {
    #[inline(always)]
    fn from(val: u8) -> Btoerren {
        Btoerren::from_bits(val)
    }
}
impl From<Btoerren> for u8 {
    #[inline(always)]
    fn from(val: Btoerren) -> u8 {
        Btoerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btserr {
    #[doc = "Packet not rejected due to the error"]
    INT_NO = 0x0,
    #[doc = "Packet rejected due to the error"]
    INT_YES = 0x01,
}
impl Btserr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btserr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btserr {
    #[inline(always)]
    fn from(val: u8) -> Btserr {
        Btserr::from_bits(val)
    }
}
impl From<Btserr> for u8 {
    #[inline(always)]
    fn from(val: Btserr) -> u8 {
        Btserr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btserren {
    #[doc = "Disable"]
    DIS_BTSERREN_INT = 0x0,
    #[doc = "Enable"]
    EN_BTSERREN_INT = 0x01,
}
impl Btserren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btserren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btserren {
    #[inline(always)]
    fn from(val: u8) -> Btserren {
        Btserren::from_bits(val)
    }
}
impl From<Btserren> for u8 {
    #[inline(always)]
    fn from(val: Btserren) -> u8 {
        Btserren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClockRecoverEn {
    #[doc = "Disable"]
    DIS_CLK_RECOVER = 0x0,
    #[doc = "Enable"]
    EN_CLK_RECOVER = 0x01,
}
impl ClockRecoverEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClockRecoverEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClockRecoverEn {
    #[inline(always)]
    fn from(val: u8) -> ClockRecoverEn {
        ClockRecoverEn::from_bits(val)
    }
}
impl From<ClockRecoverEn> for u8 {
    #[inline(always)]
    fn from(val: ClockRecoverEn) -> u8 {
        ClockRecoverEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc16 {
    #[doc = "Not rejected"]
    INT_NO = 0x0,
    #[doc = "Rejected"]
    INT_YES = 0x01,
}
impl Crc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc16 {
    #[inline(always)]
    fn from(val: u8) -> Crc16 {
        Crc16::from_bits(val)
    }
}
impl From<Crc16> for u8 {
    #[inline(always)]
    fn from(val: Crc16) -> u8 {
        Crc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc16en {
    #[doc = "Disable"]
    DIS_CRC16_INT = 0x0,
    #[doc = "Enable"]
    EN_CRC16_INT = 0x01,
}
impl Crc16en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc16en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc16en {
    #[inline(always)]
    fn from(val: u8) -> Crc16en {
        Crc16en::from_bits(val)
    }
}
impl From<Crc16en> for u8 {
    #[inline(always)]
    fn from(val: Crc16en) -> u8 {
        Crc16en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc5eof {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Crc5eof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc5eof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc5eof {
    #[inline(always)]
    fn from(val: u8) -> Crc5eof {
        Crc5eof::from_bits(val)
    }
}
impl From<Crc5eof> for u8 {
    #[inline(always)]
    fn from(val: Crc5eof) -> u8 {
        Crc5eof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crc5eofen {
    #[doc = "Disable"]
    DIS_CRC5_INT = 0x0,
    #[doc = "Enable"]
    EN_CRC5_INT = 0x01,
}
impl Crc5eofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crc5eofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crc5eofen {
    #[inline(always)]
    fn from(val: u8) -> Crc5eofen {
        Crc5eofen::from_bits(val)
    }
}
impl From<Crc5eofen> for u8 {
    #[inline(always)]
    fn from(val: Crc5eofen) -> u8 {
        Crc5eofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfn8 {
    #[doc = "Integer number of bytes"]
    INT_NO = 0x0,
    #[doc = "Not an integer number of bytes"]
    INT_YES = 0x01,
}
impl Dfn8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfn8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfn8 {
    #[inline(always)]
    fn from(val: u8) -> Dfn8 {
        Dfn8::from_bits(val)
    }
}
impl From<Dfn8> for u8 {
    #[inline(always)]
    fn from(val: Dfn8) -> u8 {
        Dfn8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfn8en {
    #[doc = "Disable"]
    DIS_DFN8_INT = 0x0,
    #[doc = "Enable"]
    EN_DFN8_INT = 0x01,
}
impl Dfn8en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfn8en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfn8en {
    #[inline(always)]
    fn from(val: u8) -> Dfn8en {
        Dfn8en::from_bits(val)
    }
}
impl From<Dfn8en> for u8 {
    #[inline(always)]
    fn from(val: Dfn8en) -> u8 {
        Dfn8en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaerr {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Dmaerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaerr {
    #[inline(always)]
    fn from(val: u8) -> Dmaerr {
        Dmaerr::from_bits(val)
    }
}
impl From<Dmaerr> for u8 {
    #[inline(always)]
    fn from(val: Dmaerr) -> u8 {
        Dmaerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaerren {
    #[doc = "Disable"]
    DIS_DMAERR_INT = 0x0,
    #[doc = "Enable"]
    EN_DMAERR_INT = 0x01,
}
impl Dmaerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaerren {
    #[inline(always)]
    fn from(val: u8) -> Dmaerren {
        Dmaerren::from_bits(val)
    }
}
impl From<Dmaerren> for u8 {
    #[inline(always)]
    fn from(val: Dmaerren) -> u8 {
        Dmaerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmlow {
    #[doc = "Disable"]
    DIS_DM_PULLDOWN = 0x0,
    #[doc = "Enable"]
    EN_DM_PULLDOWN = 0x01,
}
impl Dmlow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmlow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmlow {
    #[inline(always)]
    fn from(val: u8) -> Dmlow {
        Dmlow::from_bits(val)
    }
}
impl From<Dmlow> for u8 {
    #[inline(always)]
    fn from(val: Dmlow) -> u8 {
        Dmlow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmpd {
    #[doc = "Disabled"]
    DM_PD_DIS_STAT = 0x0,
    #[doc = "Enabled"]
    DM_PD_EN_STAT = 0x01,
}
impl Dmpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmpd {
    #[inline(always)]
    fn from(val: u8) -> Dmpd {
        Dmpd::from_bits(val)
    }
}
impl From<Dmpd> for u8 {
    #[inline(always)]
    fn from(val: Dmpd) -> u8 {
        Dmpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdmLaneReverse {
    #[doc = "Standard USB DP and DM package pin assignment"]
    DP_DM_STANDARD = 0x0,
    #[doc = "Reverse roles of USB DP and DM package pins"]
    DP_DM_REVERSED = 0x01,
}
impl DpdmLaneReverse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdmLaneReverse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdmLaneReverse {
    #[inline(always)]
    fn from(val: u8) -> DpdmLaneReverse {
        DpdmLaneReverse::from_bits(val)
    }
}
impl From<DpdmLaneReverse> for u8 {
    #[inline(always)]
    fn from(val: DpdmLaneReverse) -> u8 {
        DpdmLaneReverse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dphigh {
    #[doc = "Disable"]
    DIS_DP_PULLUP = 0x0,
    #[doc = "Enable"]
    EN_DP_PULLUP = 0x01,
}
impl Dphigh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dphigh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dphigh {
    #[inline(always)]
    fn from(val: u8) -> Dphigh {
        Dphigh::from_bits(val)
    }
}
impl From<Dphigh> for u8 {
    #[inline(always)]
    fn from(val: Dphigh) -> u8 {
        Dphigh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dplow {
    #[doc = "Disable"]
    DIS_DP_PULLDOWN = 0x0,
    #[doc = "Enable"]
    EN_DP_PULLDOWN = 0x01,
}
impl Dplow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dplow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dplow {
    #[inline(always)]
    fn from(val: u8) -> Dplow {
        Dplow::from_bits(val)
    }
}
impl From<Dplow> for u8 {
    #[inline(always)]
    fn from(val: Dplow) -> u8 {
        Dplow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppd {
    #[doc = "Disabled"]
    DP_PD_DIS_STAT = 0x0,
    #[doc = "Enabled"]
    DP_PD_EN_STAT = 0x01,
}
impl Dppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppd {
    #[inline(always)]
    fn from(val: u8) -> Dppd {
        Dppd::from_bits(val)
    }
}
impl From<Dppd> for u8 {
    #[inline(always)]
    fn from(val: Dppd) -> u8 {
        Dppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppu {
    #[doc = "Disabled"]
    DP_PU_DIS_STAT = 0x0,
    #[doc = "Enabled"]
    DP_PU_EN_STAT = 0x01,
}
impl Dppu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppu {
    #[inline(always)]
    fn from(val: u8) -> Dppu {
        Dppu::from_bits(val)
    }
}
impl From<Dppu> for u8 {
    #[inline(always)]
    fn from(val: Dppu) -> u8 {
        Dppu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dppullupnonotg {
    #[doc = "Disable"]
    DIS_DEVICE_DP_PU = 0x0,
    #[doc = "Enabled"]
    EN_DEVICE_DP_PU = 0x01,
}
impl Dppullupnonotg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dppullupnonotg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dppullupnonotg {
    #[inline(always)]
    fn from(val: u8) -> Dppullupnonotg {
        Dppullupnonotg::from_bits(val)
    }
}
impl From<Dppullupnonotg> for u8 {
    #[inline(always)]
    fn from(val: Dppullupnonotg) -> u8 {
        Dppullupnonotg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epctldis {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl Epctldis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epctldis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epctldis {
    #[inline(always)]
    fn from(val: u8) -> Epctldis {
        Epctldis::from_bits(val)
    }
}
impl From<Epctldis> for u8 {
    #[inline(always)]
    fn from(val: Epctldis) -> u8 {
        Epctldis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "Error did not occur"]
    INT_NO = 0x0,
    #[doc = "Error occurred"]
    INT_YES = 0x01,
}
impl Error {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Error {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Error {
    #[inline(always)]
    fn from(val: u8) -> Error {
        Error::from_bits(val)
    }
}
impl From<Error> for u8 {
    #[inline(always)]
    fn from(val: Error) -> u8 {
        Error::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erroren {
    #[doc = "Disable"]
    DIS_ERROR_INT = 0x0,
    #[doc = "Enable"]
    EN_ERROR_INT = 0x01,
}
impl Erroren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erroren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erroren {
    #[inline(always)]
    fn from(val: u8) -> Erroren {
        Erroren::from_bits(val)
    }
}
impl From<Erroren> for u8 {
    #[inline(always)]
    fn from(val: Erroren) -> u8 {
        Erroren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostLsEop {
    #[doc = "Full-speed device or a low-speed device through a hub"]
    HOST_FS_RESUME_EOP = 0x0,
    #[doc = "Directly-connected low-speed device"]
    HOST_LS_RESUME_EOP = 0x01,
}
impl HostLsEop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostLsEop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostLsEop {
    #[inline(always)]
    fn from(val: u8) -> HostLsEop {
        HostLsEop::from_bits(val)
    }
}
impl From<HostLsEop> for u8 {
    #[inline(always)]
    fn from(val: HostLsEop) -> u8 {
        HostLsEop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hostmodeen {
    #[doc = "USBFS operates in Device mode."]
    EN_DEVICE_MODE = 0x0,
    #[doc = "USBFS operates in Host mode. In Host mode, USBFS performs USB transactions under the programmed control of the host processor."]
    EN_HOST_MODE = 0x01,
}
impl Hostmodeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hostmodeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hostmodeen {
    #[inline(always)]
    fn from(val: u8) -> Hostmodeen {
        Hostmodeen::from_bits(val)
    }
}
impl From<Hostmodeen> for u8 {
    #[inline(always)]
    fn from(val: Hostmodeen) -> u8 {
        Hostmodeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hostwohub {
    #[doc = "Connected using a hub (USBFS generates PRE_PID as required)"]
    LS_THRU_HUB = 0x0,
    #[doc = "Connected directly to host without a hub, or was used to attach"]
    LS_DIRECT_CONNECT = 0x01,
}
impl Hostwohub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hostwohub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hostwohub {
    #[inline(always)]
    fn from(val: u8) -> Hostwohub {
        Hostwohub::from_bits(val)
    }
}
impl From<Hostwohub> for u8 {
    #[inline(always)]
    fn from(val: Hostwohub) -> u8 {
        Hostwohub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrcEn {
    #[doc = "Disable"]
    DIS_IRC = 0x0,
    #[doc = "Enable"]
    EN_IRC = 0x01,
}
impl IrcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrcEn {
    #[inline(always)]
    fn from(val: u8) -> IrcEn {
        IrcEn::from_bits(val)
    }
}
impl From<IrcEn> for u8 {
    #[inline(always)]
    fn from(val: IrcEn) -> u8 {
        IrcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LineStateChg {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl LineStateChg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LineStateChg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LineStateChg {
    #[inline(always)]
    fn from(val: u8) -> LineStateChg {
        LineStateChg::from_bits(val)
    }
}
impl From<LineStateChg> for u8 {
    #[inline(always)]
    fn from(val: LineStateChg) -> u8 {
        LineStateChg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linestateen {
    #[doc = "Disable"]
    DIS_LINEST_INT = 0x0,
    #[doc = "Enable"]
    EN_LINEST_INT = 0x01,
}
impl Linestateen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linestateen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linestateen {
    #[inline(always)]
    fn from(val: u8) -> Linestateen {
        Linestateen::from_bits(val)
    }
}
impl From<Linestateen> for u8 {
    #[inline(always)]
    fn from(val: Linestateen) -> u8 {
        Linestateen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linestatestable {
    #[doc = "Unstable"]
    LINEST_NOT_STABLE = 0x0,
    #[doc = "Stable"]
    LINEST_STABLE = 0x01,
}
impl Linestatestable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linestatestable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linestatestable {
    #[inline(always)]
    fn from(val: u8) -> Linestatestable {
        Linestatestable::from_bits(val)
    }
}
impl From<Linestatestable> for u8 {
    #[inline(always)]
    fn from(val: Linestatestable) -> u8 {
        Linestatestable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Odd {
    #[doc = "Not in the odd bank"]
    NOT_IN_ODD_BANK = 0x0,
    #[doc = "In the odd bank"]
    ODD_BANK = 0x01,
}
impl Odd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Odd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Odd {
    #[inline(always)]
    fn from(val: u8) -> Odd {
        Odd::from_bits(val)
    }
}
impl From<Odd> for u8 {
    #[inline(always)]
    fn from(val: Odd) -> u8 {
        Odd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Onemsec {
    #[doc = "Not timed out"]
    INT_NO = 0x0,
    #[doc = "Timed out"]
    INT_YES = 0x01,
}
impl Onemsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Onemsec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Onemsec {
    #[inline(always)]
    fn from(val: u8) -> Onemsec {
        Onemsec::from_bits(val)
    }
}
impl From<Onemsec> for u8 {
    #[inline(always)]
    fn from(val: Onemsec) -> u8 {
        Onemsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Onemsecen {
    #[doc = "Disable"]
    DIS_TIMER_INT = 0x0,
    #[doc = "Enable"]
    EN_TIMER_INT = 0x01,
}
impl Onemsecen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Onemsecen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Onemsecen {
    #[inline(always)]
    fn from(val: u8) -> Onemsecen {
        Onemsecen::from_bits(val)
    }
}
impl From<Onemsecen> for u8 {
    #[inline(always)]
    fn from(val: Onemsecen) -> u8 {
        Onemsecen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Otgen {
    #[doc = "If USBENSOFEN is 1 and HOSTMODEEN is 0 in the Control Register (CTL), then the D+ Data line pullup resistors are enabled. If HOSTMODEEN is 1, then the D+ and D- Data line pulldown resistors are engaged."]
    CONFIG_RESISTORS_CTL = 0x0,
    #[doc = "Uses the pullup and pulldown controls in this register."]
    CONFIG_RESISTORS_HERE = 0x01,
}
impl Otgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Otgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Otgen {
    #[inline(always)]
    fn from(val: u8) -> Otgen {
        Otgen::from_bits(val)
    }
}
impl From<Otgen> for u8 {
    #[inline(always)]
    fn from(val: Otgen) -> u8 {
        Otgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfError {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Unmasked interrupt occurred"]
    INT_YES = 0x01,
}
impl OvfError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfError {
    #[inline(always)]
    fn from(val: u8) -> OvfError {
        OvfError::from_bits(val)
    }
}
impl From<OvfError> for u8 {
    #[inline(always)]
    fn from(val: OvfError) -> u8 {
        OvfError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvfErrorEn {
    #[doc = "The interrupt is masked"]
    MASK_OVF_ERR_INT = 0x0,
    #[doc = "The interrupt is enabled"]
    EN_OVF_ERR_INT = 0x01,
}
impl OvfErrorEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvfErrorEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvfErrorEn {
    #[inline(always)]
    fn from(val: u8) -> OvfErrorEn {
        OvfErrorEn::from_bits(val)
    }
}
impl From<OvfErrorEn> for u8 {
    #[inline(always)]
    fn from(val: OvfErrorEn) -> u8 {
        OvfErrorEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerr {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Ownerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerr {
    #[inline(always)]
    fn from(val: u8) -> Ownerr {
        Ownerr::from_bits(val)
    }
}
impl From<Ownerr> for u8 {
    #[inline(always)]
    fn from(val: Ownerr) -> u8 {
        Ownerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerren {
    #[doc = "Disable"]
    DIS_OWNERR_INT = 0x0,
    #[doc = "Enable"]
    EN_OWNERR_INT = 0x01,
}
impl Ownerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerren {
    #[inline(always)]
    fn from(val: u8) -> Ownerren {
        Ownerren::from_bits(val)
    }
}
impl From<Ownerren> for u8 {
    #[inline(always)]
    fn from(val: Ownerren) -> u8 {
        Ownerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ownerrisodis {
    #[doc = "Enable"]
    DIS_OWN_ERROR_DETECT_ISO = 0x0,
    #[doc = "Disable"]
    EN_OWN_ERROR_DETECT_ISO = 0x01,
}
impl Ownerrisodis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ownerrisodis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ownerrisodis {
    #[inline(always)]
    fn from(val: u8) -> Ownerrisodis {
        Ownerrisodis::from_bits(val)
    }
}
impl From<Ownerrisodis> for u8 {
    #[inline(always)]
    fn from(val: Ownerrisodis) -> u8 {
        Ownerrisodis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pde {
    #[doc = "Disable on D+ and D-"]
    DIS_PULLDOWNS = 0x0,
    #[doc = "Enable on D+ and D-"]
    EN_PULLDOWNS = 0x01,
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
pub enum Piderr {
    #[doc = "Did not fail"]
    INT_NO = 0x0,
    #[doc = "Failed"]
    INT_YES = 0x01,
}
impl Piderr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Piderr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Piderr {
    #[inline(always)]
    fn from(val: u8) -> Piderr {
        Piderr::from_bits(val)
    }
}
impl From<Piderr> for u8 {
    #[inline(always)]
    fn from(val: Piderr) -> u8 {
        Piderr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Piderren {
    #[doc = "Disable"]
    DIS_PIDERR_INT = 0x0,
    #[doc = "Enable"]
    EN_PIDERR_INT = 0x01,
}
impl Piderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Piderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Piderren {
    #[inline(always)]
    fn from(val: u8) -> Piderren {
        Piderren::from_bits(val)
    }
}
impl From<Piderren> for u8 {
    #[inline(always)]
    fn from(val: Piderren) -> u8 {
        Piderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ResetResumeRoughEn {
    #[doc = "Always works in tracking phase after the first time rough phase, to track transition."]
    KEEP_TRIM_FINE_ON_RESET = 0x0,
    #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
    USE_IFR_TRIM_FINE_ON_RESET = 0x01,
}
impl ResetResumeRoughEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ResetResumeRoughEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ResetResumeRoughEn {
    #[inline(always)]
    fn from(val: u8) -> ResetResumeRoughEn {
        ResetResumeRoughEn::from_bits(val)
    }
}
impl From<ResetResumeRoughEn> for u8 {
    #[inline(always)]
    fn from(val: ResetResumeRoughEn) -> u8 {
        ResetResumeRoughEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RestartIfrtrimEn {
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value."]
    LOAD_TRIM_FINE_MID = 0x0,
    #[doc = "Trim fine restarts from the IFR trim value whenever you detect bus_reset or bus_resume or deassert module enable."]
    LOAD_TRIM_FINE_IFR = 0x01,
}
impl RestartIfrtrimEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RestartIfrtrimEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RestartIfrtrimEn {
    #[inline(always)]
    fn from(val: u8) -> RestartIfrtrimEn {
        RestartIfrtrimEn::from_bits(val)
    }
}
impl From<RestartIfrtrimEn> for u8 {
    #[inline(always)]
    fn from(val: RestartIfrtrimEn) -> u8 {
        RestartIfrtrimEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resume {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Resume {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resume {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resume {
    #[inline(always)]
    fn from(val: u8) -> Resume {
        Resume::from_bits(val)
    }
}
impl From<Resume> for u8 {
    #[inline(always)]
    fn from(val: Resume) -> u8 {
        Resume::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resumeen {
    #[doc = "Disable"]
    DIS_RESUME_INT = 0x0,
    #[doc = "Enable"]
    EN_RESUME_INT = 0x01,
}
impl Resumeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resumeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resumeen {
    #[inline(always)]
    fn from(val: u8) -> Resumeen {
        Resumeen::from_bits(val)
    }
}
impl From<Resumeen> for u8 {
    #[inline(always)]
    fn from(val: Resumeen) -> u8 {
        Resumeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Retrydis {
    #[doc = "Retried NAK'ed transactions in hardware."]
    RETRIED = 0x0,
    #[doc = "Do not retry NAK'ed transactions. When a transaction is NAK'ed, the BDT PID field is updated with the NAK PID, and the TOKEN_DNE interrupt becomes 1."]
    DO_NOT_RETRIED = 0x01,
}
impl Retrydis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Retrydis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Retrydis {
    #[inline(always)]
    fn from(val: u8) -> Retrydis {
        Retrydis::from_bits(val)
    }
}
impl From<Retrydis> for u8 {
    #[inline(always)]
    fn from(val: Retrydis) -> u8 {
        Retrydis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SessVld {
    #[doc = "Below"]
    SESS_VLD_LOW = 0x0,
    #[doc = "Above"]
    SESS_VLD_HIGH = 0x01,
}
impl SessVld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SessVld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SessVld {
    #[inline(always)]
    fn from(val: u8) -> SessVld {
        SessVld::from_bits(val)
    }
}
impl From<SessVld> for u8 {
    #[inline(always)]
    fn from(val: SessVld) -> u8 {
        SessVld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleep {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Sleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleep {
    #[inline(always)]
    fn from(val: u8) -> Sleep {
        Sleep::from_bits(val)
    }
}
impl From<Sleep> for u8 {
    #[inline(always)]
    fn from(val: Sleep) -> u8 {
        Sleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepen {
    #[doc = "Disable"]
    DIS_SLEEP_INT = 0x0,
    #[doc = "Enable"]
    EN_SLEEP_INT = 0x01,
}
impl Sleepen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepen {
    #[inline(always)]
    fn from(val: u8) -> Sleepen {
        Sleepen::from_bits(val)
    }
}
impl From<Sleepen> for u8 {
    #[inline(always)]
    fn from(val: Sleepen) -> u8 {
        Sleepen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofbusset {
    #[doc = "According to the SOF threshold value"]
    SOF_TOK_INT_FROM_THRESHOLD = 0x0,
    #[doc = "When the SOF counter reaches 0"]
    SOF_TOK_INT_COUNTER_0 = 0x01,
}
impl Sofbusset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofbusset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofbusset {
    #[inline(always)]
    fn from(val: u8) -> Sofbusset {
        Sofbusset::from_bits(val)
    }
}
impl From<Sofbusset> for u8 {
    #[inline(always)]
    fn from(val: Sofbusset) -> u8 {
        Sofbusset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sofdynthld {
    #[doc = "When the byte-times SOF threshold is reached"]
    USE_DYN_SOF_THRESHOLD = 0x0,
    #[doc = "When 8 byte-times SOF threshold is reached or overstepped"]
    USE_FIXED_SOF_THRESHOLD = 0x01,
}
impl Sofdynthld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sofdynthld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sofdynthld {
    #[inline(always)]
    fn from(val: u8) -> Sofdynthld {
        Sofdynthld::from_bits(val)
    }
}
impl From<Sofdynthld> for u8 {
    #[inline(always)]
    fn from(val: Sofdynthld) -> u8 {
        Sofdynthld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softok {
    #[doc = "Did not receive"]
    INT_NO = 0x0,
    #[doc = "Received"]
    INT_YES = 0x01,
}
impl Softok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softok {
    #[inline(always)]
    fn from(val: u8) -> Softok {
        Softok::from_bits(val)
    }
}
impl From<Softok> for u8 {
    #[inline(always)]
    fn from(val: Softok) -> u8 {
        Softok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softoken {
    #[doc = "Disable"]
    DIS_SOFTOK_INT = 0x0,
    #[doc = "Enable"]
    EN_SOFTOK_INT = 0x01,
}
impl Softoken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softoken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softoken {
    #[inline(always)]
    fn from(val: u8) -> Softoken {
        Softoken::from_bits(val)
    }
}
impl From<Softoken> for u8 {
    #[inline(always)]
    fn from(val: Softoken) -> u8 {
        Softoken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stall {
    #[doc = "Interrupt did not occur"]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred"]
    INT_YES = 0x01,
}
impl Stall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stall {
    #[inline(always)]
    fn from(val: u8) -> Stall {
        Stall::from_bits(val)
    }
}
impl From<Stall> for u8 {
    #[inline(always)]
    fn from(val: Stall) -> u8 {
        Stall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis0 {
    #[doc = "Enable"]
    EN_EP0_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP0_IN_STALL = 0x01,
}
impl StallIDis0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis0 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis0 {
        StallIDis0::from_bits(val)
    }
}
impl From<StallIDis0> for u8 {
    #[inline(always)]
    fn from(val: StallIDis0) -> u8 {
        StallIDis0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis1 {
    #[doc = "Enable"]
    EN_EP1_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP1_IN_STALL = 0x01,
}
impl StallIDis1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis1 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis1 {
        StallIDis1::from_bits(val)
    }
}
impl From<StallIDis1> for u8 {
    #[inline(always)]
    fn from(val: StallIDis1) -> u8 {
        StallIDis1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis10 {
    #[doc = "Enable"]
    EN_EP10_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP10_IN_STALL = 0x01,
}
impl StallIDis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis10 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis10 {
        StallIDis10::from_bits(val)
    }
}
impl From<StallIDis10> for u8 {
    #[inline(always)]
    fn from(val: StallIDis10) -> u8 {
        StallIDis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis11 {
    #[doc = "Enable"]
    EN_EP11_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP11_IN_STALL = 0x01,
}
impl StallIDis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis11 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis11 {
        StallIDis11::from_bits(val)
    }
}
impl From<StallIDis11> for u8 {
    #[inline(always)]
    fn from(val: StallIDis11) -> u8 {
        StallIDis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis12 {
    #[doc = "Enable"]
    EN_EP12_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP12_IN_STALL = 0x01,
}
impl StallIDis12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis12 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis12 {
        StallIDis12::from_bits(val)
    }
}
impl From<StallIDis12> for u8 {
    #[inline(always)]
    fn from(val: StallIDis12) -> u8 {
        StallIDis12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis13 {
    #[doc = "Enable"]
    EN_EP13_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP13_IN_STALL = 0x01,
}
impl StallIDis13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis13 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis13 {
        StallIDis13::from_bits(val)
    }
}
impl From<StallIDis13> for u8 {
    #[inline(always)]
    fn from(val: StallIDis13) -> u8 {
        StallIDis13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis14 {
    #[doc = "Enable"]
    EN_EP14_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP14_IN_STALL = 0x01,
}
impl StallIDis14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis14 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis14 {
        StallIDis14::from_bits(val)
    }
}
impl From<StallIDis14> for u8 {
    #[inline(always)]
    fn from(val: StallIDis14) -> u8 {
        StallIDis14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis15 {
    #[doc = "Enable"]
    EN_EP15_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP15_IN_STALL = 0x01,
}
impl StallIDis15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis15 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis15 {
        StallIDis15::from_bits(val)
    }
}
impl From<StallIDis15> for u8 {
    #[inline(always)]
    fn from(val: StallIDis15) -> u8 {
        StallIDis15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis2 {
    #[doc = "Enable"]
    EN_EP2_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP2_IN_STALL = 0x01,
}
impl StallIDis2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis2 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis2 {
        StallIDis2::from_bits(val)
    }
}
impl From<StallIDis2> for u8 {
    #[inline(always)]
    fn from(val: StallIDis2) -> u8 {
        StallIDis2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis3 {
    #[doc = "Enable"]
    EN_EP3_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP3_IN_STALL = 0x01,
}
impl StallIDis3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis3 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis3 {
        StallIDis3::from_bits(val)
    }
}
impl From<StallIDis3> for u8 {
    #[inline(always)]
    fn from(val: StallIDis3) -> u8 {
        StallIDis3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis4 {
    #[doc = "Enable"]
    EN_EP4_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP4_IN_STALL = 0x01,
}
impl StallIDis4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis4 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis4 {
        StallIDis4::from_bits(val)
    }
}
impl From<StallIDis4> for u8 {
    #[inline(always)]
    fn from(val: StallIDis4) -> u8 {
        StallIDis4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis5 {
    #[doc = "Enable"]
    EN_EP5_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP5_IN_STALL = 0x01,
}
impl StallIDis5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis5 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis5 {
        StallIDis5::from_bits(val)
    }
}
impl From<StallIDis5> for u8 {
    #[inline(always)]
    fn from(val: StallIDis5) -> u8 {
        StallIDis5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis6 {
    #[doc = "Enable"]
    EN_EP6_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP6_IN_STALL = 0x01,
}
impl StallIDis6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis6 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis6 {
        StallIDis6::from_bits(val)
    }
}
impl From<StallIDis6> for u8 {
    #[inline(always)]
    fn from(val: StallIDis6) -> u8 {
        StallIDis6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis7 {
    #[doc = "Enable"]
    EN_EP7_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP7_IN_STALL = 0x01,
}
impl StallIDis7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis7 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis7 {
        StallIDis7::from_bits(val)
    }
}
impl From<StallIDis7> for u8 {
    #[inline(always)]
    fn from(val: StallIDis7) -> u8 {
        StallIDis7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis8 {
    #[doc = "Enable"]
    EN_EP8_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP8_IN_STALL = 0x01,
}
impl StallIDis8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis8 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis8 {
        StallIDis8::from_bits(val)
    }
}
impl From<StallIDis8> for u8 {
    #[inline(always)]
    fn from(val: StallIDis8) -> u8 {
        StallIDis8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallIDis9 {
    #[doc = "Enable"]
    EN_EP9_IN_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP9_IN_STALL = 0x01,
}
impl StallIDis9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallIDis9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallIDis9 {
    #[inline(always)]
    fn from(val: u8) -> StallIDis9 {
        StallIDis9::from_bits(val)
    }
}
impl From<StallIDis9> for u8 {
    #[inline(always)]
    fn from(val: StallIDis9) -> u8 {
        StallIDis9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis0 {
    #[doc = "Enable"]
    EN_EP0_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP0_OUT_STALL = 0x01,
}
impl StallODis0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis0 {
    #[inline(always)]
    fn from(val: u8) -> StallODis0 {
        StallODis0::from_bits(val)
    }
}
impl From<StallODis0> for u8 {
    #[inline(always)]
    fn from(val: StallODis0) -> u8 {
        StallODis0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis1 {
    #[doc = "Enable"]
    EN_EP1_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP1_OUT_STALL = 0x01,
}
impl StallODis1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis1 {
    #[inline(always)]
    fn from(val: u8) -> StallODis1 {
        StallODis1::from_bits(val)
    }
}
impl From<StallODis1> for u8 {
    #[inline(always)]
    fn from(val: StallODis1) -> u8 {
        StallODis1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis10 {
    #[doc = "Enable"]
    EN_EP10_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP10_OUT_STALL = 0x01,
}
impl StallODis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis10 {
    #[inline(always)]
    fn from(val: u8) -> StallODis10 {
        StallODis10::from_bits(val)
    }
}
impl From<StallODis10> for u8 {
    #[inline(always)]
    fn from(val: StallODis10) -> u8 {
        StallODis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis11 {
    #[doc = "Enable"]
    EN_EP11_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP11_OUT_STALL = 0x01,
}
impl StallODis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis11 {
    #[inline(always)]
    fn from(val: u8) -> StallODis11 {
        StallODis11::from_bits(val)
    }
}
impl From<StallODis11> for u8 {
    #[inline(always)]
    fn from(val: StallODis11) -> u8 {
        StallODis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis12 {
    #[doc = "Enable"]
    EN_EP12_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP12_OUT_STALL = 0x01,
}
impl StallODis12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis12 {
    #[inline(always)]
    fn from(val: u8) -> StallODis12 {
        StallODis12::from_bits(val)
    }
}
impl From<StallODis12> for u8 {
    #[inline(always)]
    fn from(val: StallODis12) -> u8 {
        StallODis12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis13 {
    #[doc = "Enable"]
    EN_EP13_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP13_OUT_STALL = 0x01,
}
impl StallODis13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis13 {
    #[inline(always)]
    fn from(val: u8) -> StallODis13 {
        StallODis13::from_bits(val)
    }
}
impl From<StallODis13> for u8 {
    #[inline(always)]
    fn from(val: StallODis13) -> u8 {
        StallODis13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis14 {
    #[doc = "Enable"]
    EN_EP14_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP14_OUT_STALL = 0x01,
}
impl StallODis14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis14 {
    #[inline(always)]
    fn from(val: u8) -> StallODis14 {
        StallODis14::from_bits(val)
    }
}
impl From<StallODis14> for u8 {
    #[inline(always)]
    fn from(val: StallODis14) -> u8 {
        StallODis14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis15 {
    #[doc = "Enable"]
    EN_EP15_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP15_OUT_STALL = 0x01,
}
impl StallODis15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis15 {
    #[inline(always)]
    fn from(val: u8) -> StallODis15 {
        StallODis15::from_bits(val)
    }
}
impl From<StallODis15> for u8 {
    #[inline(always)]
    fn from(val: StallODis15) -> u8 {
        StallODis15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis2 {
    #[doc = "Enable"]
    EN_EP2_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP2_OUT_STALL = 0x01,
}
impl StallODis2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis2 {
    #[inline(always)]
    fn from(val: u8) -> StallODis2 {
        StallODis2::from_bits(val)
    }
}
impl From<StallODis2> for u8 {
    #[inline(always)]
    fn from(val: StallODis2) -> u8 {
        StallODis2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis3 {
    #[doc = "Enable"]
    EN_EP3_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP3_OUT_STALL = 0x01,
}
impl StallODis3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis3 {
    #[inline(always)]
    fn from(val: u8) -> StallODis3 {
        StallODis3::from_bits(val)
    }
}
impl From<StallODis3> for u8 {
    #[inline(always)]
    fn from(val: StallODis3) -> u8 {
        StallODis3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis4 {
    #[doc = "Enable"]
    EN_EP4_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP4_OUT_STALL = 0x01,
}
impl StallODis4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis4 {
    #[inline(always)]
    fn from(val: u8) -> StallODis4 {
        StallODis4::from_bits(val)
    }
}
impl From<StallODis4> for u8 {
    #[inline(always)]
    fn from(val: StallODis4) -> u8 {
        StallODis4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis5 {
    #[doc = "Enable"]
    EN_EP5_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP5_OUT_STALL = 0x01,
}
impl StallODis5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis5 {
    #[inline(always)]
    fn from(val: u8) -> StallODis5 {
        StallODis5::from_bits(val)
    }
}
impl From<StallODis5> for u8 {
    #[inline(always)]
    fn from(val: StallODis5) -> u8 {
        StallODis5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis6 {
    #[doc = "Enable"]
    EN_EP6_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP6_OUT_STALL = 0x01,
}
impl StallODis6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis6 {
    #[inline(always)]
    fn from(val: u8) -> StallODis6 {
        StallODis6::from_bits(val)
    }
}
impl From<StallODis6> for u8 {
    #[inline(always)]
    fn from(val: StallODis6) -> u8 {
        StallODis6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis7 {
    #[doc = "Enable"]
    EN_EP7_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP7_OUT_STALL = 0x01,
}
impl StallODis7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis7 {
    #[inline(always)]
    fn from(val: u8) -> StallODis7 {
        StallODis7::from_bits(val)
    }
}
impl From<StallODis7> for u8 {
    #[inline(always)]
    fn from(val: StallODis7) -> u8 {
        StallODis7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis8 {
    #[doc = "Enable"]
    EN_EP8_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP8_OUT_STALL = 0x01,
}
impl StallODis8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis8 {
    #[inline(always)]
    fn from(val: u8) -> StallODis8 {
        StallODis8::from_bits(val)
    }
}
impl From<StallODis8> for u8 {
    #[inline(always)]
    fn from(val: StallODis8) -> u8 {
        StallODis8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StallODis9 {
    #[doc = "Enable"]
    EN_EP9_OUT_STALL = 0x0,
    #[doc = "Disable"]
    DIS_EP9_OUT_STALL = 0x01,
}
impl StallODis9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StallODis9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StallODis9 {
    #[inline(always)]
    fn from(val: u8) -> StallODis9 {
        StallODis9::from_bits(val)
    }
}
impl From<StallODis9> for u8 {
    #[inline(always)]
    fn from(val: StallODis9) -> u8 {
        StallODis9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stallen {
    #[doc = "Disable"]
    DIS_STALL_INT = 0x0,
    #[doc = "Enable"]
    EN_STALL_INT = 0x01,
}
impl Stallen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stallen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stallen {
    #[inline(always)]
    fn from(val: u8) -> Stallen {
        Stallen::from_bits(val)
    }
}
impl From<Stallen> for u8 {
    #[inline(always)]
    fn from(val: Stallen) -> u8 {
        Stallen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StlAdjEn {
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, both IN and OUT directions for the associated endpoint stalls."]
    STALL_BOTH_IN_OUT = 0x0,
    #[doc = "If ENDPTn\\[END_STALL\\] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
    STALL_SINGLE_DIRECTION = 0x01,
}
impl StlAdjEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StlAdjEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StlAdjEn {
    #[inline(always)]
    fn from(val: u8) -> StlAdjEn {
        StlAdjEn::from_bits(val)
    }
}
impl From<StlAdjEn> for u8 {
    #[inline(always)]
    fn from(val: StlAdjEn) -> u8 {
        StlAdjEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Susp {
    #[doc = "Not in Suspend state"]
    XCVR_NOT_SUSPEND = 0x0,
    #[doc = "In Suspend state"]
    XCVR_SUSPEND = 0x01,
}
impl Susp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Susp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Susp {
    #[inline(always)]
    fn from(val: u8) -> Susp {
        Susp::from_bits(val)
    }
}
impl From<Susp> for u8 {
    #[inline(always)]
    fn from(val: Susp) -> u8 {
        Susp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncDet {
    #[doc = "Not detected"]
    NO_SYNC_INT = 0x0,
    #[doc = "Detected"]
    SYNC_INT_DETECTED = 0x01,
}
impl SyncDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncDet {
    #[inline(always)]
    fn from(val: u8) -> SyncDet {
        SyncDet::from_bits(val)
    }
}
impl From<SyncDet> for u8 {
    #[inline(always)]
    fn from(val: SyncDet) -> u8 {
        SyncDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokdne {
    #[doc = "Not processed"]
    INT_NO = 0x0,
    #[doc = "Processed"]
    INT_YES = 0x01,
}
impl Tokdne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokdne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokdne {
    #[inline(always)]
    fn from(val: u8) -> Tokdne {
        Tokdne::from_bits(val)
    }
}
impl From<Tokdne> for u8 {
    #[inline(always)]
    fn from(val: Tokdne) -> u8 {
        Tokdne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokdneen {
    #[doc = "Disable"]
    DIS_TOKDNE_INT = 0x0,
    #[doc = "Enable"]
    EN_TOKDNE_INT = 0x01,
}
impl Tokdneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokdneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokdneen {
    #[inline(always)]
    fn from(val: u8) -> Tokdneen {
        Tokdneen::from_bits(val)
    }
}
impl From<Tokdneen> for u8 {
    #[inline(always)]
    fn from(val: Tokdneen) -> u8 {
        Tokdneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tokenpid {
    _RESERVED_0 = 0x0,
    #[doc = "OUT token. USBFS performs an OUT (TX) transaction."]
    EN_OUT_TOKEN = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "IN token. USBFS performs an IN (RX) transaction."]
    EN_IN_TOKEN = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "SETUP token. USBFS performs a SETUP (TX) transaction"]
    EN_SETUP_TOKEN = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tokenpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tokenpid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tokenpid {
    #[inline(always)]
    fn from(val: u8) -> Tokenpid {
        Tokenpid::from_bits(val)
    }
}
impl From<Tokenpid> for u8 {
    #[inline(always)]
    fn from(val: Tokenpid) -> u8 {
        Tokenpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimInitValSel {
    #[doc = "Mid-scale"]
    INIT_TRIM_FINE_MID = 0x0,
    #[doc = "IFR"]
    INIT_TRIM_FINE_IFR = 0x01,
}
impl TrimInitValSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimInitValSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimInitValSel {
    #[inline(always)]
    fn from(val: u8) -> TrimInitValSel {
        TrimInitValSel::from_bits(val)
    }
}
impl From<TrimInitValSel> for u8 {
    #[inline(always)]
    fn from(val: TrimInitValSel) -> u8 {
        TrimInitValSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tx {
    #[doc = "Receive"]
    RX_TRANSACTION = 0x0,
    #[doc = "Transmit"]
    TX_TRANSACTION = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartchls {
    #[doc = "USB DP and DM signals are used as UART TX/RX."]
    UART_DP_TX = 0x0,
    #[doc = "USB DP and DM signals are used as UART RX/TX."]
    UART_DM_TX = 0x01,
}
impl Uartchls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartchls {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartchls {
    #[inline(always)]
    fn from(val: u8) -> Uartchls {
        Uartchls::from_bits(val)
    }
}
impl From<Uartchls> for u8 {
    #[inline(always)]
    fn from(val: Uartchls) -> u8 {
        Uartchls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartsel {
    #[doc = "USB DP and DM external package pins are used for USB signaling."]
    USB_MODE = 0x0,
    #[doc = "USB DP and DM external package pins are used for UART signaling."]
    UART_MODE = 0x01,
}
impl Uartsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartsel {
    #[inline(always)]
    fn from(val: u8) -> Uartsel {
        Uartsel::from_bits(val)
    }
}
impl From<Uartsel> for u8 {
    #[inline(always)]
    fn from(val: Uartsel) -> u8 {
        Uartsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbResumeInt {
    #[doc = "Not generated"]
    NO_ASYNC_INT = 0x0,
    #[doc = "Generated because of the USB asynchronous interrupt"]
    SYNC_INT_GENERATED = 0x01,
}
impl UsbResumeInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbResumeInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbResumeInt {
    #[inline(always)]
    fn from(val: u8) -> UsbResumeInt {
        UsbResumeInt::from_bits(val)
    }
}
impl From<UsbResumeInt> for u8 {
    #[inline(always)]
    fn from(val: UsbResumeInt) -> u8 {
        UsbResumeInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbensofen {
    #[doc = "Disable"]
    DIS_USB_SOF = 0x0,
    #[doc = "Enable"]
    EN_USB_SOF = 0x01,
}
impl Usbensofen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbensofen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbensofen {
    #[inline(always)]
    fn from(val: u8) -> Usbensofen {
        Usbensofen::from_bits(val)
    }
}
impl From<Usbensofen> for u8 {
    #[inline(always)]
    fn from(val: Usbensofen) -> u8 {
        Usbensofen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbreset {
    #[doc = "Normal USBFS operation"]
    NORMAL_OPERATION = 0x0,
    #[doc = "Returns USBFS to its reset state"]
    FORCE_HARD_RESET = 0x01,
}
impl Usbreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbreset {
    #[inline(always)]
    fn from(val: u8) -> Usbreset {
        Usbreset::from_bits(val)
    }
}
impl From<Usbreset> for u8 {
    #[inline(always)]
    fn from(val: Usbreset) -> u8 {
        Usbreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbresmen {
    #[doc = "Disable"]
    DIS_ASYNC_WAKEUP = 0x0,
    #[doc = "Enable"]
    EN_ASYNC_WAKEUP = 0x01,
}
impl Usbresmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbresmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbresmen {
    #[inline(always)]
    fn from(val: u8) -> Usbresmen {
        Usbresmen::from_bits(val)
    }
}
impl From<Usbresmen> for u8 {
    #[inline(always)]
    fn from(val: Usbresmen) -> u8 {
        Usbresmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbrst {
    #[doc = "Not detected"]
    INT_NO = 0x0,
    #[doc = "Detected"]
    INT_YES = 0x01,
}
impl Usbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrst {
    #[inline(always)]
    fn from(val: u8) -> Usbrst {
        Usbrst::from_bits(val)
    }
}
impl From<Usbrst> for u8 {
    #[inline(always)]
    fn from(val: Usbrst) -> u8 {
        Usbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbrsten {
    #[doc = "Disable"]
    DIS_USBRST_INT = 0x0,
    #[doc = "Enable"]
    EN_USBRST_INT = 0x01,
}
impl Usbrsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbrsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbrsten {
    #[inline(always)]
    fn from(val: u8) -> Usbrsten {
        Usbrsten::from_bits(val)
    }
}
impl From<Usbrsten> for u8 {
    #[inline(always)]
    fn from(val: Usbrsten) -> u8 {
        Usbrsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfedgDet {
    #[doc = "Not detected"]
    NO_VREG_FE_INT = 0x0,
    #[doc = "Detected"]
    VREG_FE_INT_DETECTED = 0x01,
}
impl VfedgDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfedgDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfedgDet {
    #[inline(always)]
    fn from(val: u8) -> VfedgDet {
        VfedgDet::from_bits(val)
    }
}
impl From<VfedgDet> for u8 {
    #[inline(always)]
    fn from(val: VfedgDet) -> u8 {
        VfedgDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfedgEn {
    #[doc = "Disable"]
    DIS_VREGIN_FE_INT = 0x0,
    #[doc = "Enable"]
    EN_VREGIN_FE_INT = 0x01,
}
impl VfedgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfedgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfedgEn {
    #[inline(always)]
    fn from(val: u8) -> VfedgEn {
        VfedgEn::from_bits(val)
    }
}
impl From<VfedgEn> for u8 {
    #[inline(always)]
    fn from(val: VfedgEn) -> u8 {
        VfedgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VredgDet {
    #[doc = "Not detected"]
    NO_VREG_RE_INT = 0x0,
    #[doc = "Detected"]
    VREG_RE_INT_DETECTED = 0x01,
}
impl VredgDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VredgDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VredgDet {
    #[inline(always)]
    fn from(val: u8) -> VredgDet {
        VredgDet::from_bits(val)
    }
}
impl From<VredgDet> for u8 {
    #[inline(always)]
    fn from(val: VredgDet) -> u8 {
        VredgDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VredgEn {
    #[doc = "Disable"]
    DIS_VREGIN_RE_INT = 0x0,
    #[doc = "Enable"]
    EN_VREGIN_RE_INT = 0x01,
}
impl VredgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VredgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VredgEn {
    #[inline(always)]
    fn from(val: u8) -> VredgEn {
        VredgEn::from_bits(val)
    }
}
impl From<VredgEn> for u8 {
    #[inline(always)]
    fn from(val: VredgEn) -> u8 {
        VredgEn::to_bits(val)
    }
}
