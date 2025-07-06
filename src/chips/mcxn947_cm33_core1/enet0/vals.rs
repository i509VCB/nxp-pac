#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aal {
    #[doc = "Address-Aligned Beats is disabled"]
    DISABLE = 0x0,
    #[doc = "Address-Aligned Beats is enabled"]
    ENABLE = 0x01,
}
impl Aal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aal {
    #[inline(always)]
    fn from(val: u8) -> Aal {
        Aal::from_bits(val)
    }
}
impl From<Aal> for u8 {
    #[inline(always)]
    fn from(val: Aal) -> u8 {
        Aal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acs {
    #[doc = "Automatic Pad or CRC Stripping is disabled"]
    DISABLE = 0x0,
    #[doc = "Automatic Pad or CRC Stripping is enabled"]
    ENABLE = 0x01,
}
impl Acs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acs {
    #[inline(always)]
    fn from(val: u8) -> Acs {
        Acs::from_bits(val)
    }
}
impl From<Acs> for u8 {
    #[inline(always)]
    fn from(val: Acs) -> u8 {
        Acs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actphysel {
    #[doc = "GMII or MII"]
    GMII_MII = 0x0,
    #[doc = "RGMII"]
    RGMII = 0x01,
    #[doc = "SGMII"]
    SGMII = 0x02,
    #[doc = "TBI"]
    TBI = 0x03,
    #[doc = "RMII"]
    RMII = 0x04,
    #[doc = "RTBI"]
    RTBI = 0x05,
    #[doc = "SMII"]
    SMII = 0x06,
    #[doc = "RevMII"]
    REVMIII = 0x07,
}
impl Actphysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Actphysel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Actphysel {
    #[inline(always)]
    fn from(val: u8) -> Actphysel {
        Actphysel::from_bits(val)
    }
}
impl From<Actphysel> for u8 {
    #[inline(always)]
    fn from(val: Actphysel) -> u8 {
        Actphysel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addr64 {
    #[doc = "32"]
    M_32 = 0x0,
    #[doc = "40"]
    M_40 = 0x01,
    #[doc = "48"]
    M_48 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Addr64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr64 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr64 {
    #[inline(always)]
    fn from(val: u8) -> Addr64 {
        Addr64::from_bits(val)
    }
}
impl From<Addr64> for u8 {
    #[inline(always)]
    fn from(val: Addr64) -> u8 {
        Addr64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addsub {
    #[doc = "Add time"]
    ADD = 0x0,
    #[doc = "Subtract time"]
    SUB = 0x01,
}
impl Addsub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addsub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addsub {
    #[inline(always)]
    fn from(val: u8) -> Addsub {
        Addsub::from_bits(val)
    }
}
impl From<Addsub> for u8 {
    #[inline(always)]
    fn from(val: Addsub) -> u8 {
        Addsub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Advthword {
    #[doc = "IEEE 1588 High Word Register option is not selected"]
    INACTIVE = 0x0,
    #[doc = "IEEE 1588 High Word Register option is selected"]
    ACTIVE = 0x01,
}
impl Advthword {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Advthword {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Advthword {
    #[inline(always)]
    fn from(val: u8) -> Advthword {
        Advthword::from_bits(val)
    }
}
impl From<Advthword> for u8 {
    #[inline(always)]
    fn from(val: Advthword) -> u8 {
        Advthword::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ae {
    #[doc = "INVALID : This bit must be always set to 1"]
    DISABLE = 0x0,
    #[doc = "This bit is always set to 1"]
    ENABLE = 0x01,
}
impl Ae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ae {
    #[inline(always)]
    fn from(val: u8) -> Ae {
        Ae::from_bits(val)
    }
}
impl From<Ae> for u8 {
    #[inline(always)]
    fn from(val: Ae) -> u8 {
        Ae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arpoffsel {
    #[doc = "ARP Offload Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "ARP Offload Enable option is selected"]
    ACTIVE = 0x01,
}
impl Arpoffsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arpoffsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arpoffsel {
    #[inline(always)]
    fn from(val: u8) -> Arpoffsel {
        Arpoffsel::from_bits(val)
    }
}
impl From<Arpoffsel> for u8 {
    #[inline(always)]
    fn from(val: Arpoffsel) -> u8 {
        Arpoffsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asp {
    #[doc = "No Safety features selected"]
    NONE = 0x0,
    #[doc = "Only \"ECC protection for external memory\" feature is selected"]
    ECC_ONLY = 0x01,
    #[doc = "All the Automotive Safety features are selected without the \"Parity Port Enable for external interface\" feature"]
    AS_NPPE = 0x02,
    #[doc = "All the Automotive Safety features are selected with the \"Parity Port Enable for external interface\" feature"]
    AS_PPE = 0x03,
}
impl Asp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asp {
    #[inline(always)]
    fn from(val: u8) -> Asp {
        Asp::from_bits(val)
    }
}
impl From<Asp> for u8 {
    #[inline(always)]
    fn from(val: Asp) -> u8 {
        Asp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auxsnapnum {
    #[doc = "No auxiliary input"]
    NO_AUXI = 0x0,
    #[doc = "1 auxiliary input"]
    M_1_AUXI = 0x01,
    #[doc = "2 auxiliary input"]
    M_2_AUXI = 0x02,
    #[doc = "3 auxiliary input"]
    M_3_AUXI = 0x03,
    #[doc = "4 auxiliary input"]
    M_4_AUXI = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Auxsnapnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auxsnapnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auxsnapnum {
    #[inline(always)]
    fn from(val: u8) -> Auxsnapnum {
        Auxsnapnum::from_bits(val)
    }
}
impl From<Auxsnapnum> for u8 {
    #[inline(always)]
    fn from(val: Auxsnapnum) -> u8 {
        Auxsnapnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Av8021asmen {
    #[doc = "AV 802.1AS Mode is disabled"]
    DISABLE = 0x0,
    #[doc = "AV 802.1AS Mode is enabled"]
    ENABLE = 0x01,
}
impl Av8021asmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Av8021asmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Av8021asmen {
    #[inline(always)]
    fn from(val: u8) -> Av8021asmen {
        Av8021asmen::from_bits(val)
    }
}
impl From<Av8021asmen> for u8 {
    #[inline(always)]
    fn from(val: Av8021asmen) -> u8 {
        Av8021asmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avalg {
    #[doc = "CBS Algorithm is disabled"]
    DISABLE = 0x0,
    #[doc = "CBS Algorithm is enabled"]
    ENABLE = 0x01,
}
impl Avalg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avalg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avalg {
    #[inline(always)]
    fn from(val: u8) -> Avalg {
        Avalg::from_bits(val)
    }
}
impl From<Avalg> for u8 {
    #[inline(always)]
    fn from(val: Avalg) -> u8 {
        Avalg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avcpq {
    #[doc = "Receive Queue 0"]
    QUEUE0 = 0x0,
    #[doc = "Receive Queue 1"]
    QUEUE1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Avcpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avcpq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avcpq {
    #[inline(always)]
    fn from(val: u8) -> Avcpq {
        Avcpq::from_bits(val)
    }
}
impl From<Avcpq> for u8 {
    #[inline(always)]
    fn from(val: Avcpq) -> u8 {
        Avcpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avsel {
    #[doc = "AV Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "AV Feature is selected"]
    ACTIVE = 0x01,
}
impl Avsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avsel {
    #[inline(always)]
    fn from(val: u8) -> Avsel {
        Avsel::from_bits(val)
    }
}
impl From<Avsel> for u8 {
    #[inline(always)]
    fn from(val: Avsel) -> u8 {
        Avsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axwhsts {
    #[doc = "AXI Master Write Channel or AHB Master Status not detected"]
    INACTIVE = 0x0,
    #[doc = "AXI Master Write Channel or AHB Master Status detected"]
    ACTIVE = 0x01,
}
impl Axwhsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axwhsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axwhsts {
    #[inline(always)]
    fn from(val: u8) -> Axwhsts {
        Axwhsts::from_bits(val)
    }
}
impl From<Axwhsts> for u8 {
    #[inline(always)]
    fn from(val: Axwhsts) -> u8 {
        Axwhsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bl {
    #[doc = "k = min(n,10)"]
    MIN_N_10 = 0x0,
    #[doc = "k = min(n,8)"]
    MIN_N_8 = 0x01,
    #[doc = "k = min(n,4)"]
    MIN_N_4 = 0x02,
    #[doc = "k = min(n,1)"]
    MIN_N_1 = 0x03,
}
impl Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bl {
    #[inline(always)]
    fn from(val: u8) -> Bl {
        Bl::from_bits(val)
    }
}
impl From<Bl> for u8 {
    #[inline(always)]
    fn from(val: Bl) -> u8 {
        Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Btb {
    #[doc = "Back to Back transactions disabled"]
    DISABLE = 0x0,
    #[doc = "Back to Back transactions enabled"]
    ENABLE = 0x01,
}
impl Btb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Btb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Btb {
    #[inline(always)]
    fn from(val: u8) -> Btb {
        Btb::from_bits(val)
    }
}
impl From<Btb> for u8 {
    #[inline(always)]
    fn from(val: Btb) -> u8 {
        Btb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Busy status not detected"]
    INACTIVE = 0x0,
    #[doc = "Busy status detected"]
    ACTIVE = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum C45e {
    #[doc = "Clause 45 PHY is disabled"]
    DISABLE = 0x0,
    #[doc = "Clause 45 PHY is enabled"]
    ENABLE = 0x01,
}
impl C45e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> C45e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for C45e {
    #[inline(always)]
    fn from(val: u8) -> C45e {
        C45e::from_bits(val)
    }
}
impl From<C45e> for u8 {
    #[inline(always)]
    fn from(val: C45e) -> u8 {
        C45e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cbti {
    #[doc = "Channel based tag insertion is disabled"]
    DISABLE = 0x0,
    #[doc = "Channel based tag insertion is enabled"]
    ENABLE = 0x01,
}
impl Cbti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cbti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cbti {
    #[inline(always)]
    fn from(val: u8) -> Cbti {
        Cbti::from_bits(val)
    }
}
impl From<Cbti> for u8 {
    #[inline(always)]
    fn from(val: Cbti) -> u8 {
        Cbti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cbtisel {
    #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is selected"]
    ACTIVE = 0x01,
}
impl Cbtisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cbtisel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cbtisel {
    #[inline(always)]
    fn from(val: u8) -> Cbtisel {
        Cbtisel::from_bits(val)
    }
}
impl From<Cbtisel> for u8 {
    #[inline(always)]
    fn from(val: Cbtisel) -> u8 {
        Cbtisel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc {
    #[doc = "Credit Control is disabled"]
    DISABLE = 0x0,
    #[doc = "Credit Control is enabled"]
    ENABLE = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntclr {
    #[doc = "Counters are not reset"]
    DISABLE = 0x0,
    #[doc = "All counters are reset"]
    ENABLE = 0x01,
}
impl Cntclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntclr {
    #[inline(always)]
    fn from(val: u8) -> Cntclr {
        Cntclr::from_bits(val)
    }
}
impl From<Cntclr> for u8 {
    #[inline(always)]
    fn from(val: Cntclr) -> u8 {
        Cntclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cntprst {
    #[doc = "Counters Preset is disabled"]
    DISABLE = 0x0,
    #[doc = "Counters Preset is enabled"]
    ENABLE = 0x01,
}
impl Cntprst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cntprst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cntprst {
    #[inline(always)]
    fn from(val: u8) -> Cntprst {
        Cntprst::from_bits(val)
    }
}
impl From<Cntprst> for u8 {
    #[inline(always)]
    fn from(val: Cntprst) -> u8 {
        Cntprst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Com {
    #[doc = "Write operation"]
    WRITE = 0x0,
    #[doc = "Read operation"]
    READ = 0x01,
}
impl Com {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Com {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Com {
    #[inline(always)]
    fn from(val: u8) -> Com {
        Com::from_bits(val)
    }
}
impl From<Com> for u8 {
    #[inline(always)]
    fn from(val: Com) -> u8 {
        Com::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cst {
    #[doc = "CRC stripping for Type packets is disabled"]
    DISABLE = 0x0,
    #[doc = "CRC stripping for Type packets is enabled"]
    ENABLE = 0x01,
}
impl Cst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cst {
    #[inline(always)]
    fn from(val: u8) -> Cst {
        Cst::from_bits(val)
    }
}
impl From<Cst> for u8 {
    #[inline(always)]
    fn from(val: Cst) -> u8 {
        Cst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Da {
    #[doc = "Weighted Round-Robin with Rx:Tx or Tx:Rx"]
    WRR = 0x0,
    #[doc = "Fixed Priority"]
    FP = 0x01,
}
impl Da {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Da {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Da {
    #[inline(always)]
    fn from(val: u8) -> Da {
        Da::from_bits(val)
    }
}
impl From<Da> for u8 {
    #[inline(always)]
    fn from(val: Da) -> u8 {
        Da::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Daif {
    #[doc = "DA Inverse Filtering is disabled"]
    DISABLE = 0x0,
    #[doc = "DA Inverse Filtering is enabled"]
    ENABLE = 0x01,
}
impl Daif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Daif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Daif {
    #[inline(always)]
    fn from(val: u8) -> Daif {
        Daif::from_bits(val)
    }
}
impl From<Daif> for u8 {
    #[inline(always)]
    fn from(val: Daif) -> u8 {
        Daif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbf {
    #[doc = "Enable Broadcast Packets"]
    ENABLE = 0x0,
    #[doc = "Disable Broadcast Packets"]
    DISABLE = 0x01,
}
impl Dbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbf {
    #[inline(always)]
    fn from(val: u8) -> Dbf {
        Dbf::from_bits(val)
    }
}
impl From<Dbf> for u8 {
    #[inline(always)]
    fn from(val: Dbf) -> u8 {
        Dbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgmema {
    #[doc = "DMA Debug Registers option is not selected"]
    INACTIVE = 0x0,
    #[doc = "DMA Debug Registers option is selected"]
    ACTIVE = 0x01,
}
impl Dbgmema {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgmema {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgmema {
    #[inline(always)]
    fn from(val: u8) -> Dbgmema {
        Dbgmema::from_bits(val)
    }
}
impl From<Dbgmema> for u8 {
    #[inline(always)]
    fn from(val: Dbgmema) -> u8 {
        Dbgmema::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dc {
    #[doc = "Deferral check function is disabled"]
    DISABLE = 0x0,
    #[doc = "Deferral check function is enabled"]
    ENABLE = 0x01,
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
pub enum Dc0is {
    #[doc = "DMA Channel 0 Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "DMA Channel 0 Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl Dc0is {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dc0is {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dc0is {
    #[inline(always)]
    fn from(val: u8) -> Dc0is {
        Dc0is::from_bits(val)
    }
}
impl From<Dc0is> for u8 {
    #[inline(always)]
    fn from(val: Dc0is) -> u8 {
        Dc0is::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dc1is {
    #[doc = "DMA Channel 1 Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "DMA Channel 1 Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl Dc1is {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dc1is {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dc1is {
    #[inline(always)]
    fn from(val: u8) -> Dc1is {
        Dc1is::from_bits(val)
    }
}
impl From<Dc1is> for u8 {
    #[inline(always)]
    fn from(val: Dc1is) -> u8 {
        Dc1is::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcben {
    #[doc = "DCB Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "DCB Feature is selected"]
    ACTIVE = 0x01,
}
impl Dcben {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcben {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcben {
    #[inline(always)]
    fn from(val: u8) -> Dcben {
        Dcben::from_bits(val)
    }
}
impl From<Dcben> for u8 {
    #[inline(always)]
    fn from(val: Dcben) -> u8 {
        Dcben::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrcc {
    #[doc = "CRC Checking is enabled"]
    ENABLE = 0x0,
    #[doc = "CRC Checking is disabled"]
    DISABLE = 0x01,
}
impl Dcrcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrcc {
    #[inline(always)]
    fn from(val: u8) -> Dcrcc {
        Dcrcc::from_bits(val)
    }
}
impl From<Dcrcc> for u8 {
    #[inline(always)]
    fn from(val: Dcrcc) -> u8 {
        Dcrcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrs {
    #[doc = "Enable Carrier Sense During Transmission"]
    ENABLE = 0x0,
    #[doc = "Disable Carrier Sense During Transmission"]
    DISABLE = 0x01,
}
impl Dcrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrs {
    #[inline(always)]
    fn from(val: u8) -> Dcrs {
        Dcrs::from_bits(val)
    }
}
impl From<Dcrs> for u8 {
    #[inline(always)]
    fn from(val: Dcrs) -> u8 {
        Dcrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dm {
    #[doc = "Half-duplex mode"]
    HDUPLX = 0x0,
    #[doc = "Full-duplex mode"]
    FDUPLX = 0x01,
}
impl Dm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dm {
    #[inline(always)]
    fn from(val: u8) -> Dm {
        Dm::from_bits(val)
    }
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(val: Dm) -> u8 {
        Dm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0ControlPblx8 {
    #[doc = "8xPBL mode is disabled"]
    DISABLE = 0x0,
    #[doc = "8xPBL mode is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0ControlPblx8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0ControlPblx8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0ControlPblx8 {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0ControlPblx8 {
        DmaCh0ControlPblx8::from_bits(val)
    }
}
impl From<DmaCh0ControlPblx8> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0ControlPblx8) -> u8 {
        DmaCh0ControlPblx8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableAie {
    #[doc = "Abnormal Interrupt Summary is disabled"]
    DISABLE = 0x0,
    #[doc = "Abnormal Interrupt Summary is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableAie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableAie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableAie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableAie {
        DmaCh0InterruptEnableAie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableAie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableAie) -> u8 {
        DmaCh0InterruptEnableAie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableCdee {
    #[doc = "Context Descriptor Error is disabled"]
    DISABLE = 0x0,
    #[doc = "Context Descriptor Error is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableCdee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableCdee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableCdee {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableCdee {
        DmaCh0InterruptEnableCdee::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableCdee> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableCdee) -> u8 {
        DmaCh0InterruptEnableCdee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableErie {
    #[doc = "Early Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableErie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableErie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableErie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableErie {
        DmaCh0InterruptEnableErie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableErie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableErie) -> u8 {
        DmaCh0InterruptEnableErie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableEtie {
    #[doc = "Early Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableEtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableEtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableEtie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableEtie {
        DmaCh0InterruptEnableEtie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableEtie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableEtie) -> u8 {
        DmaCh0InterruptEnableEtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableFbee {
    #[doc = "Fatal Bus Error is disabled"]
    DISABLE = 0x0,
    #[doc = "Fatal Bus Error is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableFbee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableFbee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableFbee {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableFbee {
        DmaCh0InterruptEnableFbee::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableFbee> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableFbee) -> u8 {
        DmaCh0InterruptEnableFbee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableNie {
    #[doc = "Normal Interrupt Summary is disabled"]
    DISABLE = 0x0,
    #[doc = "Normal Interrupt Summary is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableNie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableNie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableNie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableNie {
        DmaCh0InterruptEnableNie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableNie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableNie) -> u8 {
        DmaCh0InterruptEnableNie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableRbue {
    #[doc = "Receive Buffer Unavailable is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Buffer Unavailable is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableRbue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableRbue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableRbue {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableRbue {
        DmaCh0InterruptEnableRbue::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableRbue> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableRbue) -> u8 {
        DmaCh0InterruptEnableRbue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableRie {
    #[doc = "Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableRie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableRie {
        DmaCh0InterruptEnableRie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableRie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableRie) -> u8 {
        DmaCh0InterruptEnableRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableRse {
    #[doc = "Receive Stopped is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Stopped is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableRse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableRse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableRse {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableRse {
        DmaCh0InterruptEnableRse::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableRse> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableRse) -> u8 {
        DmaCh0InterruptEnableRse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableRwte {
    #[doc = "Receive Watchdog Timeout is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Watchdog Timeout is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableRwte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableRwte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableRwte {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableRwte {
        DmaCh0InterruptEnableRwte::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableRwte> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableRwte) -> u8 {
        DmaCh0InterruptEnableRwte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableTbue {
    #[doc = "Transmit Buffer Unavailable is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Buffer Unavailable is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableTbue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableTbue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableTbue {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableTbue {
        DmaCh0InterruptEnableTbue::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableTbue> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableTbue) -> u8 {
        DmaCh0InterruptEnableTbue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableTie {
    #[doc = "Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableTie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableTie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableTie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableTie {
        DmaCh0InterruptEnableTie::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableTie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableTie) -> u8 {
        DmaCh0InterruptEnableTie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0InterruptEnableTxse {
    #[doc = "Transmit Stopped is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Stopped is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0InterruptEnableTxse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0InterruptEnableTxse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0InterruptEnableTxse {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0InterruptEnableTxse {
        DmaCh0InterruptEnableTxse::from_bits(val)
    }
}
impl From<DmaCh0InterruptEnableTxse> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0InterruptEnableTxse) -> u8 {
        DmaCh0InterruptEnableTxse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0MissFrameCntMfco {
    #[doc = "Miss Frame Counter overflow not occurred"]
    INACTIVE = 0x0,
    #[doc = "Miss Frame Counter overflow occurred"]
    ACTIVE = 0x01,
}
impl DmaCh0MissFrameCntMfco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0MissFrameCntMfco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0MissFrameCntMfco {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0MissFrameCntMfco {
        DmaCh0MissFrameCntMfco::from_bits(val)
    }
}
impl From<DmaCh0MissFrameCntMfco> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0MissFrameCntMfco) -> u8 {
        DmaCh0MissFrameCntMfco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0RxControlEric {
    #[doc = "Early Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0RxControlEric {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0RxControlEric {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0RxControlEric {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0RxControlEric {
        DmaCh0RxControlEric::from_bits(val)
    }
}
impl From<DmaCh0RxControlEric> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0RxControlEric) -> u8 {
        DmaCh0RxControlEric::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0RxControlRpf {
    #[doc = "Rx Packet Flush is disabled"]
    DISABLE = 0x0,
    #[doc = "Rx Packet Flush is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0RxControlRpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0RxControlRpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0RxControlRpf {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0RxControlRpf {
        DmaCh0RxControlRpf::from_bits(val)
    }
}
impl From<DmaCh0RxControlRpf> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0RxControlRpf) -> u8 {
        DmaCh0RxControlRpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0RxControlSr {
    #[doc = "Stop Receive"]
    STOP = 0x0,
    #[doc = "Start Receive"]
    START = 0x01,
}
impl DmaCh0RxControlSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0RxControlSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0RxControlSr {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0RxControlSr {
        DmaCh0RxControlSr::from_bits(val)
    }
}
impl From<DmaCh0RxControlSr> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0RxControlSr) -> u8 {
        DmaCh0RxControlSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0SlotFunctionControlStatusAsc {
    #[doc = "Advance Slot Check is disabled"]
    DISABLE = 0x0,
    #[doc = "Advance Slot Check is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0SlotFunctionControlStatusAsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0SlotFunctionControlStatusAsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0SlotFunctionControlStatusAsc {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0SlotFunctionControlStatusAsc {
        DmaCh0SlotFunctionControlStatusAsc::from_bits(val)
    }
}
impl From<DmaCh0SlotFunctionControlStatusAsc> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0SlotFunctionControlStatusAsc) -> u8 {
        DmaCh0SlotFunctionControlStatusAsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0SlotFunctionControlStatusEsc {
    #[doc = "Slot Comparison is disabled"]
    DISABLE = 0x0,
    #[doc = "Slot Comparison is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0SlotFunctionControlStatusEsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0SlotFunctionControlStatusEsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0SlotFunctionControlStatusEsc {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0SlotFunctionControlStatusEsc {
        DmaCh0SlotFunctionControlStatusEsc::from_bits(val)
    }
}
impl From<DmaCh0SlotFunctionControlStatusEsc> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0SlotFunctionControlStatusEsc) -> u8 {
        DmaCh0SlotFunctionControlStatusEsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusAis {
    #[doc = "Abnormal Interrupt Summary status not detected"]
    INACTIVE = 0x0,
    #[doc = "Abnormal Interrupt Summary status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusAis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusAis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusAis {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusAis {
        DmaCh0StatusAis::from_bits(val)
    }
}
impl From<DmaCh0StatusAis> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusAis) -> u8 {
        DmaCh0StatusAis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusCde {
    #[doc = "Context Descriptor Error status not detected"]
    INACTIVE = 0x0,
    #[doc = "Context Descriptor Error status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusCde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusCde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusCde {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusCde {
        DmaCh0StatusCde::from_bits(val)
    }
}
impl From<DmaCh0StatusCde> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusCde) -> u8 {
        DmaCh0StatusCde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusEri {
    #[doc = "Early Receive Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Early Receive Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusEri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusEri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusEri {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusEri {
        DmaCh0StatusEri::from_bits(val)
    }
}
impl From<DmaCh0StatusEri> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusEri) -> u8 {
        DmaCh0StatusEri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusEti {
    #[doc = "Early Transmit Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Early Transmit Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusEti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusEti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusEti {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusEti {
        DmaCh0StatusEti::from_bits(val)
    }
}
impl From<DmaCh0StatusEti> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusEti) -> u8 {
        DmaCh0StatusEti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusFbe {
    #[doc = "Fatal Bus Error status not detected"]
    INACTIVE = 0x0,
    #[doc = "Fatal Bus Error status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusFbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusFbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusFbe {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusFbe {
        DmaCh0StatusFbe::from_bits(val)
    }
}
impl From<DmaCh0StatusFbe> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusFbe) -> u8 {
        DmaCh0StatusFbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusNis {
    #[doc = "Normal Interrupt Summary status not detected"]
    INACTIVE = 0x0,
    #[doc = "Normal Interrupt Summary status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusNis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusNis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusNis {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusNis {
        DmaCh0StatusNis::from_bits(val)
    }
}
impl From<DmaCh0StatusNis> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusNis) -> u8 {
        DmaCh0StatusNis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusRbu {
    #[doc = "Receive Buffer Unavailable status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Buffer Unavailable status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusRbu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusRbu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusRbu {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusRbu {
        DmaCh0StatusRbu::from_bits(val)
    }
}
impl From<DmaCh0StatusRbu> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusRbu) -> u8 {
        DmaCh0StatusRbu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusRi {
    #[doc = "Receive Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusRi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusRi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusRi {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusRi {
        DmaCh0StatusRi::from_bits(val)
    }
}
impl From<DmaCh0StatusRi> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusRi) -> u8 {
        DmaCh0StatusRi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusRps {
    #[doc = "Receive Process Stopped status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Process Stopped status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusRps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusRps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusRps {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusRps {
        DmaCh0StatusRps::from_bits(val)
    }
}
impl From<DmaCh0StatusRps> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusRps) -> u8 {
        DmaCh0StatusRps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusRwt {
    #[doc = "Receive Watchdog Timeout status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Watchdog Timeout status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusRwt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusRwt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusRwt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusRwt {
        DmaCh0StatusRwt::from_bits(val)
    }
}
impl From<DmaCh0StatusRwt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusRwt) -> u8 {
        DmaCh0StatusRwt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusTbu {
    #[doc = "Transmit Buffer Unavailable status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Buffer Unavailable status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusTbu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusTbu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusTbu {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusTbu {
        DmaCh0StatusTbu::from_bits(val)
    }
}
impl From<DmaCh0StatusTbu> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusTbu) -> u8 {
        DmaCh0StatusTbu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusTi {
    #[doc = "Transmit Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusTi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusTi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusTi {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusTi {
        DmaCh0StatusTi::from_bits(val)
    }
}
impl From<DmaCh0StatusTi> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusTi) -> u8 {
        DmaCh0StatusTi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0StatusTps {
    #[doc = "Transmit Process Stopped status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Process Stopped status detected"]
    ACTIVE = 0x01,
}
impl DmaCh0StatusTps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0StatusTps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0StatusTps {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0StatusTps {
        DmaCh0StatusTps::from_bits(val)
    }
}
impl From<DmaCh0StatusTps> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0StatusTps) -> u8 {
        DmaCh0StatusTps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0TxControlEtic {
    #[doc = "Early Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh0TxControlEtic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0TxControlEtic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0TxControlEtic {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0TxControlEtic {
        DmaCh0TxControlEtic::from_bits(val)
    }
}
impl From<DmaCh0TxControlEtic> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0TxControlEtic) -> u8 {
        DmaCh0TxControlEtic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0TxControlOsf {
    #[doc = "Operate on Second Packet disabled"]
    DISABLE = 0x0,
    #[doc = "Operate on Second Packet enabled"]
    ENABLE = 0x01,
}
impl DmaCh0TxControlOsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0TxControlOsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0TxControlOsf {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0TxControlOsf {
        DmaCh0TxControlOsf::from_bits(val)
    }
}
impl From<DmaCh0TxControlOsf> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0TxControlOsf) -> u8 {
        DmaCh0TxControlOsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0TxControlSt {
    #[doc = "Stop Transmission Command"]
    STOP = 0x0,
    #[doc = "Start Transmission Command"]
    START = 0x01,
}
impl DmaCh0TxControlSt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0TxControlSt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0TxControlSt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0TxControlSt {
        DmaCh0TxControlSt::from_bits(val)
    }
}
impl From<DmaCh0TxControlSt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0TxControlSt) -> u8 {
        DmaCh0TxControlSt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1ControlPblx8 {
    #[doc = "8xPBL mode is disabled"]
    DISABLE = 0x0,
    #[doc = "8xPBL mode is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1ControlPblx8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1ControlPblx8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1ControlPblx8 {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1ControlPblx8 {
        DmaCh1ControlPblx8::from_bits(val)
    }
}
impl From<DmaCh1ControlPblx8> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1ControlPblx8) -> u8 {
        DmaCh1ControlPblx8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableAie {
    #[doc = "Abnormal Interrupt Summary is disabled"]
    DISABLE = 0x0,
    #[doc = "Abnormal Interrupt Summary is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableAie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableAie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableAie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableAie {
        DmaCh1InterruptEnableAie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableAie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableAie) -> u8 {
        DmaCh1InterruptEnableAie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableCdee {
    #[doc = "Context Descriptor Error is disabled"]
    DISABLE = 0x0,
    #[doc = "Context Descriptor Error is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableCdee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableCdee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableCdee {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableCdee {
        DmaCh1InterruptEnableCdee::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableCdee> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableCdee) -> u8 {
        DmaCh1InterruptEnableCdee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableErie {
    #[doc = "Early Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableErie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableErie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableErie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableErie {
        DmaCh1InterruptEnableErie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableErie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableErie) -> u8 {
        DmaCh1InterruptEnableErie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableEtie {
    #[doc = "Early Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableEtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableEtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableEtie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableEtie {
        DmaCh1InterruptEnableEtie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableEtie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableEtie) -> u8 {
        DmaCh1InterruptEnableEtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableFbee {
    #[doc = "Fatal Bus Error is disabled"]
    DISABLE = 0x0,
    #[doc = "Fatal Bus Error is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableFbee {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableFbee {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableFbee {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableFbee {
        DmaCh1InterruptEnableFbee::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableFbee> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableFbee) -> u8 {
        DmaCh1InterruptEnableFbee::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableNie {
    #[doc = "Normal Interrupt Summary is disabled"]
    DISABLE = 0x0,
    #[doc = "Normal Interrupt Summary is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableNie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableNie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableNie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableNie {
        DmaCh1InterruptEnableNie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableNie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableNie) -> u8 {
        DmaCh1InterruptEnableNie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableRbue {
    #[doc = "Receive Buffer Unavailable is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Buffer Unavailable is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableRbue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableRbue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableRbue {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableRbue {
        DmaCh1InterruptEnableRbue::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableRbue> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableRbue) -> u8 {
        DmaCh1InterruptEnableRbue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableRie {
    #[doc = "Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableRie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableRie {
        DmaCh1InterruptEnableRie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableRie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableRie) -> u8 {
        DmaCh1InterruptEnableRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableRse {
    #[doc = "Receive Stopped is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Stopped is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableRse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableRse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableRse {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableRse {
        DmaCh1InterruptEnableRse::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableRse> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableRse) -> u8 {
        DmaCh1InterruptEnableRse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableRwte {
    #[doc = "Receive Watchdog Timeout is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Watchdog Timeout is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableRwte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableRwte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableRwte {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableRwte {
        DmaCh1InterruptEnableRwte::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableRwte> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableRwte) -> u8 {
        DmaCh1InterruptEnableRwte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableTbue {
    #[doc = "Transmit Buffer Unavailable is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Buffer Unavailable is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableTbue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableTbue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableTbue {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableTbue {
        DmaCh1InterruptEnableTbue::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableTbue> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableTbue) -> u8 {
        DmaCh1InterruptEnableTbue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableTie {
    #[doc = "Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableTie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableTie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableTie {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableTie {
        DmaCh1InterruptEnableTie::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableTie> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableTie) -> u8 {
        DmaCh1InterruptEnableTie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1InterruptEnableTxse {
    #[doc = "Transmit Stopped is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Stopped is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1InterruptEnableTxse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1InterruptEnableTxse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1InterruptEnableTxse {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1InterruptEnableTxse {
        DmaCh1InterruptEnableTxse::from_bits(val)
    }
}
impl From<DmaCh1InterruptEnableTxse> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1InterruptEnableTxse) -> u8 {
        DmaCh1InterruptEnableTxse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1MissFrameCntMfco {
    #[doc = "Miss Frame Counter overflow not occurred"]
    INACTIVE = 0x0,
    #[doc = "Miss Frame Counter overflow occurred"]
    ACTIVE = 0x01,
}
impl DmaCh1MissFrameCntMfco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1MissFrameCntMfco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1MissFrameCntMfco {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1MissFrameCntMfco {
        DmaCh1MissFrameCntMfco::from_bits(val)
    }
}
impl From<DmaCh1MissFrameCntMfco> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1MissFrameCntMfco) -> u8 {
        DmaCh1MissFrameCntMfco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1RxControlEric {
    #[doc = "Early Receive Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Receive Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1RxControlEric {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1RxControlEric {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1RxControlEric {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1RxControlEric {
        DmaCh1RxControlEric::from_bits(val)
    }
}
impl From<DmaCh1RxControlEric> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1RxControlEric) -> u8 {
        DmaCh1RxControlEric::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1RxControlRpf {
    #[doc = "Rx Packet Flush is disabled"]
    DISABLE = 0x0,
    #[doc = "Rx Packet Flush is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1RxControlRpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1RxControlRpf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1RxControlRpf {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1RxControlRpf {
        DmaCh1RxControlRpf::from_bits(val)
    }
}
impl From<DmaCh1RxControlRpf> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1RxControlRpf) -> u8 {
        DmaCh1RxControlRpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1RxControlSr {
    #[doc = "Stop Receive"]
    STOP = 0x0,
    #[doc = "Start Receive"]
    START = 0x01,
}
impl DmaCh1RxControlSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1RxControlSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1RxControlSr {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1RxControlSr {
        DmaCh1RxControlSr::from_bits(val)
    }
}
impl From<DmaCh1RxControlSr> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1RxControlSr) -> u8 {
        DmaCh1RxControlSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1SlotFunctionControlStatusAsc {
    #[doc = "Advance Slot Check is disabled"]
    DISABLE = 0x0,
    #[doc = "Advance Slot Check is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1SlotFunctionControlStatusAsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1SlotFunctionControlStatusAsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1SlotFunctionControlStatusAsc {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1SlotFunctionControlStatusAsc {
        DmaCh1SlotFunctionControlStatusAsc::from_bits(val)
    }
}
impl From<DmaCh1SlotFunctionControlStatusAsc> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1SlotFunctionControlStatusAsc) -> u8 {
        DmaCh1SlotFunctionControlStatusAsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1SlotFunctionControlStatusEsc {
    #[doc = "Slot Comparison is disabled"]
    DISABLE = 0x0,
    #[doc = "Slot Comparison is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1SlotFunctionControlStatusEsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1SlotFunctionControlStatusEsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1SlotFunctionControlStatusEsc {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1SlotFunctionControlStatusEsc {
        DmaCh1SlotFunctionControlStatusEsc::from_bits(val)
    }
}
impl From<DmaCh1SlotFunctionControlStatusEsc> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1SlotFunctionControlStatusEsc) -> u8 {
        DmaCh1SlotFunctionControlStatusEsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusAis {
    #[doc = "Abnormal Interrupt Summary status not detected"]
    INACTIVE = 0x0,
    #[doc = "Abnormal Interrupt Summary status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusAis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusAis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusAis {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusAis {
        DmaCh1StatusAis::from_bits(val)
    }
}
impl From<DmaCh1StatusAis> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusAis) -> u8 {
        DmaCh1StatusAis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusCde {
    #[doc = "Context Descriptor Error status not detected"]
    INACTIVE = 0x0,
    #[doc = "Context Descriptor Error status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusCde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusCde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusCde {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusCde {
        DmaCh1StatusCde::from_bits(val)
    }
}
impl From<DmaCh1StatusCde> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusCde) -> u8 {
        DmaCh1StatusCde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusEri {
    #[doc = "Early Receive Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Early Receive Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusEri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusEri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusEri {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusEri {
        DmaCh1StatusEri::from_bits(val)
    }
}
impl From<DmaCh1StatusEri> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusEri) -> u8 {
        DmaCh1StatusEri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusEti {
    #[doc = "Early Transmit Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Early Transmit Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusEti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusEti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusEti {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusEti {
        DmaCh1StatusEti::from_bits(val)
    }
}
impl From<DmaCh1StatusEti> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusEti) -> u8 {
        DmaCh1StatusEti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusFbe {
    #[doc = "Fatal Bus Error status not detected"]
    INACTIVE = 0x0,
    #[doc = "Fatal Bus Error status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusFbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusFbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusFbe {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusFbe {
        DmaCh1StatusFbe::from_bits(val)
    }
}
impl From<DmaCh1StatusFbe> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusFbe) -> u8 {
        DmaCh1StatusFbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusNis {
    #[doc = "Normal Interrupt Summary status not detected"]
    INACTIVE = 0x0,
    #[doc = "Normal Interrupt Summary status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusNis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusNis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusNis {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusNis {
        DmaCh1StatusNis::from_bits(val)
    }
}
impl From<DmaCh1StatusNis> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusNis) -> u8 {
        DmaCh1StatusNis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusRbu {
    #[doc = "Receive Buffer Unavailable status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Buffer Unavailable status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusRbu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusRbu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusRbu {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusRbu {
        DmaCh1StatusRbu::from_bits(val)
    }
}
impl From<DmaCh1StatusRbu> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusRbu) -> u8 {
        DmaCh1StatusRbu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusRi {
    #[doc = "Receive Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusRi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusRi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusRi {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusRi {
        DmaCh1StatusRi::from_bits(val)
    }
}
impl From<DmaCh1StatusRi> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusRi) -> u8 {
        DmaCh1StatusRi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusRps {
    #[doc = "Receive Process Stopped status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Process Stopped status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusRps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusRps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusRps {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusRps {
        DmaCh1StatusRps::from_bits(val)
    }
}
impl From<DmaCh1StatusRps> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusRps) -> u8 {
        DmaCh1StatusRps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusRwt {
    #[doc = "Receive Watchdog Timeout status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Watchdog Timeout status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusRwt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusRwt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusRwt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusRwt {
        DmaCh1StatusRwt::from_bits(val)
    }
}
impl From<DmaCh1StatusRwt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusRwt) -> u8 {
        DmaCh1StatusRwt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusTbu {
    #[doc = "Transmit Buffer Unavailable status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Buffer Unavailable status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusTbu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusTbu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusTbu {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusTbu {
        DmaCh1StatusTbu::from_bits(val)
    }
}
impl From<DmaCh1StatusTbu> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusTbu) -> u8 {
        DmaCh1StatusTbu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusTi {
    #[doc = "Transmit Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Interrupt status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusTi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusTi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusTi {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusTi {
        DmaCh1StatusTi::from_bits(val)
    }
}
impl From<DmaCh1StatusTi> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusTi) -> u8 {
        DmaCh1StatusTi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1StatusTps {
    #[doc = "Transmit Process Stopped status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Process Stopped status detected"]
    ACTIVE = 0x01,
}
impl DmaCh1StatusTps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1StatusTps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1StatusTps {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1StatusTps {
        DmaCh1StatusTps::from_bits(val)
    }
}
impl From<DmaCh1StatusTps> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1StatusTps) -> u8 {
        DmaCh1StatusTps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1TxControlEtic {
    #[doc = "Early Transmit Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Early Transmit Interrupt is enabled"]
    ENABLE = 0x01,
}
impl DmaCh1TxControlEtic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1TxControlEtic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1TxControlEtic {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1TxControlEtic {
        DmaCh1TxControlEtic::from_bits(val)
    }
}
impl From<DmaCh1TxControlEtic> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1TxControlEtic) -> u8 {
        DmaCh1TxControlEtic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1TxControlOsf {
    #[doc = "Operate on Second Packet disabled"]
    DISABLE = 0x0,
    #[doc = "Operate on Second Packet enabled"]
    ENABLE = 0x01,
}
impl DmaCh1TxControlOsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1TxControlOsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1TxControlOsf {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1TxControlOsf {
        DmaCh1TxControlOsf::from_bits(val)
    }
}
impl From<DmaCh1TxControlOsf> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1TxControlOsf) -> u8 {
        DmaCh1TxControlOsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1TxControlSt {
    #[doc = "Stop Transmission Command"]
    STOP = 0x0,
    #[doc = "Start Transmission Command"]
    START = 0x01,
}
impl DmaCh1TxControlSt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1TxControlSt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1TxControlSt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1TxControlSt {
        DmaCh1TxControlSt::from_bits(val)
    }
}
impl From<DmaCh1TxControlSt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1TxControlSt) -> u8 {
        DmaCh1TxControlSt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaModePr {
    #[doc = "The priority ratio is 1:1"]
    R_1_1 = 0x0,
    #[doc = "The priority ratio is 2:1"]
    R_2_1 = 0x01,
    #[doc = "The priority ratio is 3:1"]
    R_3_1 = 0x02,
    #[doc = "The priority ratio is 4:1"]
    R_4_1 = 0x03,
    #[doc = "The priority ratio is 5:1"]
    R_5_1 = 0x04,
    #[doc = "The priority ratio is 6:1"]
    R_6_1 = 0x05,
    #[doc = "The priority ratio is 7:1"]
    R_7_1 = 0x06,
    #[doc = "The priority ratio is 8:1"]
    R_8_1 = 0x07,
}
impl DmaModePr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaModePr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaModePr {
    #[inline(always)]
    fn from(val: u8) -> DmaModePr {
        DmaModePr::from_bits(val)
    }
}
impl From<DmaModePr> for u8 {
    #[inline(always)]
    fn from(val: DmaModePr) -> u8 {
        DmaModePr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Do {
    #[doc = "Enable Receive Own"]
    ENABLE = 0x0,
    #[doc = "Disable Receive Own"]
    DISABLE = 0x01,
}
impl Do {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Do {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Do {
    #[inline(always)]
    fn from(val: u8) -> Do {
        Do::from_bits(val)
    }
}
impl From<Do> for u8 {
    #[inline(always)]
    fn from(val: Do) -> u8 {
        Do::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dovltc {
    #[doc = "VLAN Type Check is enabled"]
    ENABLE = 0x0,
    #[doc = "VLAN Type Check is disabled"]
    DISABLE = 0x01,
}
impl Dovltc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dovltc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dovltc {
    #[inline(always)]
    fn from(val: u8) -> Dovltc {
        Dovltc::from_bits(val)
    }
}
impl From<Dovltc> for u8 {
    #[inline(always)]
    fn from(val: Dovltc) -> u8 {
        Dovltc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dr {
    #[doc = "Enable Retry"]
    ENABLE = 0x0,
    #[doc = "Disable Retry"]
    DISABLE = 0x01,
}
impl Dr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dr {
    #[inline(always)]
    fn from(val: u8) -> Dr {
        Dr::from_bits(val)
    }
}
impl From<Dr> for u8 {
    #[inline(always)]
    fn from(val: Dr) -> u8 {
        Dr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtxsts {
    #[doc = "Drop Transmit Status is disabled"]
    DISABLE = 0x0,
    #[doc = "Drop Transmit Status is enabled"]
    ENABLE = 0x01,
}
impl Dtxsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtxsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtxsts {
    #[inline(always)]
    fn from(val: u8) -> Dtxsts {
        Dtxsts::from_bits(val)
    }
}
impl From<Dtxsts> for u8 {
    #[inline(always)]
    fn from(val: Dtxsts) -> u8 {
        Dtxsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvlan {
    #[doc = "Double VLAN option is not selected"]
    INACTIVE = 0x0,
    #[doc = "Double VLAN option is selected"]
    ACTIVE = 0x01,
}
impl Dvlan {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvlan {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvlan {
    #[inline(always)]
    fn from(val: u8) -> Dvlan {
        Dvlan::from_bits(val)
    }
}
impl From<Dvlan> for u8 {
    #[inline(always)]
    fn from(val: Dvlan) -> u8 {
        Dvlan::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dzpq {
    #[doc = "Zero-Quanta Pause packet generation is enabled"]
    ENABLE = 0x0,
    #[doc = "Zero-Quanta Pause packet generation is disabled"]
    DISABLE = 0x01,
}
impl Dzpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dzpq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dzpq {
    #[inline(always)]
    fn from(val: u8) -> Dzpq {
        Dzpq::from_bits(val)
    }
}
impl From<Dzpq> for u8 {
    #[inline(always)]
    fn from(val: Dzpq) -> u8 {
        Dzpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecrsfd {
    #[doc = "ECRSFD is disabled"]
    DISABLE = 0x0,
    #[doc = "ECRSFD is enabled"]
    ENABLE = 0x01,
}
impl Ecrsfd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecrsfd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecrsfd {
    #[inline(always)]
    fn from(val: u8) -> Ecrsfd {
        Ecrsfd::from_bits(val)
    }
}
impl From<Ecrsfd> for u8 {
    #[inline(always)]
    fn from(val: Ecrsfd) -> u8 {
        Ecrsfd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edvlp {
    #[doc = "Double VLAN Processing is disabled"]
    DISABLE = 0x0,
    #[doc = "Double VLAN Processing is enabled"]
    ENABLE = 0x01,
}
impl Edvlp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edvlp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edvlp {
    #[inline(always)]
    fn from(val: u8) -> Edvlp {
        Edvlp::from_bits(val)
    }
}
impl From<Edvlp> for u8 {
    #[inline(always)]
    fn from(val: Edvlp) -> u8 {
        Edvlp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eeesel {
    #[doc = "Energy Efficient Ethernet Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "Energy Efficient Ethernet Enable option is selected"]
    ACTIVE = 0x01,
}
impl Eeesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eeesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eeesel {
    #[inline(always)]
    fn from(val: u8) -> Eeesel {
        Eeesel::from_bits(val)
    }
}
impl From<Eeesel> for u8 {
    #[inline(always)]
    fn from(val: Eeesel) -> u8 {
        Eeesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eipgen {
    #[doc = "Extended Inter-Packet Gap is disabled"]
    DISABLE = 0x0,
    #[doc = "Extended Inter-Packet Gap is enabled"]
    ENABLE = 0x01,
}
impl Eipgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eipgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eipgen {
    #[inline(always)]
    fn from(val: u8) -> Eipgen {
        Eipgen::from_bits(val)
    }
}
impl From<Eipgen> for u8 {
    #[inline(always)]
    fn from(val: Eipgen) -> u8 {
        Eipgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eivlrxs {
    #[doc = "Inner VLAN Tag in Rx status is disabled"]
    DISABLE = 0x0,
    #[doc = "Inner VLAN Tag in Rx status is enabled"]
    ENABLE = 0x01,
}
impl Eivlrxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eivlrxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eivlrxs {
    #[inline(always)]
    fn from(val: u8) -> Eivlrxs {
        Eivlrxs::from_bits(val)
    }
}
impl From<Eivlrxs> for u8 {
    #[inline(always)]
    fn from(val: Eivlrxs) -> u8 {
        Eivlrxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eivls {
    #[doc = "Do not strip"]
    DONOT = 0x0,
    #[doc = "Strip if VLAN filter passes"]
    IFPASS = 0x01,
    #[doc = "Strip if VLAN filter fails"]
    IFFAIL = 0x02,
    #[doc = "Always strip"]
    ALWAYS = 0x03,
}
impl Eivls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eivls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eivls {
    #[inline(always)]
    fn from(val: u8) -> Eivls {
        Eivls::from_bits(val)
    }
}
impl From<Eivls> for u8 {
    #[inline(always)]
    fn from(val: Eivls) -> u8 {
        Eivls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erivlt {
    #[doc = "Inner VLAN tag is disabled"]
    DISABLE = 0x0,
    #[doc = "Inner VLAN tag is enabled"]
    ENABLE = 0x01,
}
impl Erivlt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erivlt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erivlt {
    #[inline(always)]
    fn from(val: u8) -> Erivlt {
        Erivlt::from_bits(val)
    }
}
impl From<Erivlt> for u8 {
    #[inline(always)]
    fn from(val: Erivlt) -> u8 {
        Erivlt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersvlm {
    #[doc = "Receive S-VLAN Match is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive S-VLAN Match is enabled"]
    ENABLE = 0x01,
}
impl Ersvlm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersvlm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersvlm {
    #[inline(always)]
    fn from(val: u8) -> Ersvlm {
        Ersvlm::from_bits(val)
    }
}
impl From<Ersvlm> for u8 {
    #[inline(always)]
    fn from(val: Ersvlm) -> u8 {
        Ersvlm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estdep {
    #[doc = "No Depth configured"]
    NODEPTH = 0x0,
    #[doc = "64"]
    DEPTH64 = 0x01,
    #[doc = "128"]
    DEPTH128 = 0x02,
    #[doc = "256"]
    DEPTH256 = 0x03,
    #[doc = "512"]
    DEPTH512 = 0x04,
    #[doc = "1024"]
    DEPTH1024 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Estdep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estdep {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estdep {
    #[inline(always)]
    fn from(val: u8) -> Estdep {
        Estdep::from_bits(val)
    }
}
impl From<Estdep> for u8 {
    #[inline(always)]
    fn from(val: Estdep) -> u8 {
        Estdep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esti {
    #[doc = "External System Time Input is disabled"]
    DISABLE = 0x0,
    #[doc = "External System Time Input is enabled"]
    ENABLE = 0x01,
}
impl Esti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esti {
    #[inline(always)]
    fn from(val: u8) -> Esti {
        Esti::from_bits(val)
    }
}
impl From<Esti> for u8 {
    #[inline(always)]
    fn from(val: Esti) -> u8 {
        Esti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estsel {
    #[doc = "Enable Enhancements to Scheduling Traffic feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Enable Enhancements to Scheduling Traffic feature is selected"]
    ACTIVE = 0x01,
}
impl Estsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estsel {
    #[inline(always)]
    fn from(val: u8) -> Estsel {
        Estsel::from_bits(val)
    }
}
impl From<Estsel> for u8 {
    #[inline(always)]
    fn from(val: Estsel) -> u8 {
        Estsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estwid {
    #[doc = "Width not configured"]
    NOWIDTH = 0x0,
    #[doc = "16"]
    WIDTH16 = 0x01,
    #[doc = "20"]
    WIDTH20 = 0x02,
    #[doc = "24"]
    WIDTH24 = 0x03,
}
impl Estwid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estwid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estwid {
    #[inline(always)]
    fn from(val: u8) -> Estwid {
        Estwid::from_bits(val)
    }
}
impl From<Estwid> for u8 {
    #[inline(always)]
    fn from(val: Estwid) -> u8 {
        Estwid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esvl {
    #[doc = "S-VLAN is disabled"]
    DISABLE = 0x0,
    #[doc = "S-VLAN is enabled"]
    ENABLE = 0x01,
}
impl Esvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esvl {
    #[inline(always)]
    fn from(val: u8) -> Esvl {
        Esvl::from_bits(val)
    }
}
impl From<Esvl> for u8 {
    #[inline(always)]
    fn from(val: Esvl) -> u8 {
        Esvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etv {
    #[doc = "12-bit VLAN Tag Comparison is disabled"]
    DISABLE = 0x0,
    #[doc = "12-bit VLAN Tag Comparison is enabled"]
    ENABLE = 0x01,
}
impl Etv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etv {
    #[inline(always)]
    fn from(val: u8) -> Etv {
        Etv::from_bits(val)
    }
}
impl From<Etv> for u8 {
    #[inline(always)]
    fn from(val: Etv) -> u8 {
        Etv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evlrxs {
    #[doc = "VLAN Tag in Rx status is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Tag in Rx status is enabled"]
    ENABLE = 0x01,
}
impl Evlrxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evlrxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evlrxs {
    #[inline(always)]
    fn from(val: u8) -> Evlrxs {
        Evlrxs::from_bits(val)
    }
}
impl From<Evlrxs> for u8 {
    #[inline(always)]
    fn from(val: Evlrxs) -> u8 {
        Evlrxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evls {
    #[doc = "Do not strip"]
    DONOT = 0x0,
    #[doc = "Strip if VLAN filter passes"]
    IFPASS = 0x01,
    #[doc = "Strip if VLAN filter fails"]
    IFFAIL = 0x02,
    #[doc = "Always strip"]
    ALWAYS = 0x03,
}
impl Evls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evls {
    #[inline(always)]
    fn from(val: u8) -> Evls {
        Evls::from_bits(val)
    }
}
impl From<Evls> for u8 {
    #[inline(always)]
    fn from(val: Evls) -> u8 {
        Evls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Excol {
    #[doc = "No collision"]
    INACTIVE = 0x0,
    #[doc = "Excessive collision is sensed"]
    ACTIVE = 0x01,
}
impl Excol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Excol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Excol {
    #[inline(always)]
    fn from(val: u8) -> Excol {
        Excol::from_bits(val)
    }
}
impl From<Excol> for u8 {
    #[inline(always)]
    fn from(val: Excol) -> u8 {
        Excol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Exdef {
    #[doc = "No Excessive deferral"]
    INACTIVE = 0x0,
    #[doc = "Excessive deferral"]
    ACTIVE = 0x01,
}
impl Exdef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Exdef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Exdef {
    #[inline(always)]
    fn from(val: u8) -> Exdef {
        Exdef::from_bits(val)
    }
}
impl From<Exdef> for u8 {
    #[inline(always)]
    fn from(val: Exdef) -> u8 {
        Exdef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fb {
    #[doc = "Fixed Burst Length is disabled"]
    DISABLE = 0x0,
    #[doc = "Fixed Burst Length is enabled"]
    ENABLE = 0x01,
}
impl Fb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fb {
    #[inline(always)]
    fn from(val: u8) -> Fb {
        Fb::from_bits(val)
    }
}
impl From<Fb> for u8 {
    #[inline(always)]
    fn from(val: Fb) -> u8 {
        Fb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcbBpa {
    #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
    DISABLE = 0x0,
    #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
    ENABLE = 0x01,
}
impl FcbBpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcbBpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcbBpa {
    #[inline(always)]
    fn from(val: u8) -> FcbBpa {
        FcbBpa::from_bits(val)
    }
}
impl From<FcbBpa> for u8 {
    #[inline(always)]
    fn from(val: FcbBpa) -> u8 {
        FcbBpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fes {
    #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0"]
    M_10_1000M = 0x0,
    #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0"]
    M_100_2500M = 0x01,
}
impl Fes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fes {
    #[inline(always)]
    fn from(val: u8) -> Fes {
        Fes::from_bits(val)
    }
}
impl From<Fes> for u8 {
    #[inline(always)]
    fn from(val: Fes) -> u8 {
        Fes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fpesel {
    #[doc = "Frame Preemption Enable feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Frame Preemption Enable feature is selected"]
    ACTIVE = 0x01,
}
impl Fpesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fpesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fpesel {
    #[inline(always)]
    fn from(val: u8) -> Fpesel {
        Fpesel::from_bits(val)
    }
}
impl From<Fpesel> for u8 {
    #[inline(always)]
    fn from(val: Fpesel) -> u8 {
        Fpesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpbs {
    #[doc = "64 Bytes"]
    M_64BYTES = 0x0,
    #[doc = "128 Bytes"]
    M_128BYTES = 0x01,
    #[doc = "256 Bytes"]
    M_256BYTES = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpbs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpbs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpbs {
    #[inline(always)]
    fn from(val: u8) -> Frpbs {
        Frpbs::from_bits(val)
    }
}
impl From<Frpbs> for u8 {
    #[inline(always)]
    fn from(val: Frpbs) -> u8 {
        Frpbs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpes {
    #[doc = "64 Entries"]
    M_64ENTR = 0x0,
    #[doc = "128 Entries"]
    M_128ENTR = 0x01,
    #[doc = "256 Entries"]
    M_256ENTR = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpes {
    #[inline(always)]
    fn from(val: u8) -> Frpes {
        Frpes::from_bits(val)
    }
}
impl From<Frpes> for u8 {
    #[inline(always)]
    fn from(val: Frpes) -> u8 {
        Frpes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpsel {
    #[doc = "Flexible Receive Parser feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Flexible Receive Parser feature is selected"]
    ACTIVE = 0x01,
}
impl Frpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpsel {
    #[inline(always)]
    fn from(val: u8) -> Frpsel {
        Frpsel::from_bits(val)
    }
}
impl From<Frpsel> for u8 {
    #[inline(always)]
    fn from(val: Frpsel) -> u8 {
        Frpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gb {
    #[doc = "GMII Busy is disabled"]
    DISABLE = 0x0,
    #[doc = "GMII Busy is enabled"]
    ENABLE = 0x01,
}
impl Gb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gb {
    #[inline(always)]
    fn from(val: u8) -> Gb {
        Gb::from_bits(val)
    }
}
impl From<Gb> for u8 {
    #[inline(always)]
    fn from(val: Gb) -> u8 {
        Gb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Glblucast {
    #[doc = "Global unicast is disabled"]
    DISABLE = 0x0,
    #[doc = "Global unicast is enabled"]
    ENABLE = 0x01,
}
impl Glblucast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Glblucast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Glblucast {
    #[inline(always)]
    fn from(val: u8) -> Glblucast {
        Glblucast::from_bits(val)
    }
}
impl From<Glblucast> for u8 {
    #[inline(always)]
    fn from(val: Glblucast) -> u8 {
        Glblucast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gmiisel {
    #[doc = "No 1000 Mbps support"]
    INACTIVE = 0x0,
    #[doc = "1000 Mbps support"]
    ACTIVE = 0x01,
}
impl Gmiisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gmiisel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gmiisel {
    #[inline(always)]
    fn from(val: u8) -> Gmiisel {
        Gmiisel::from_bits(val)
    }
}
impl From<Gmiisel> for u8 {
    #[inline(always)]
    fn from(val: Gmiisel) -> u8 {
        Gmiisel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Goc0 {
    #[doc = "GMII Operation Command 0 is disabled"]
    DISABLE = 0x0,
    #[doc = "GMII Operation Command 0 is enabled"]
    ENABLE = 0x01,
}
impl Goc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Goc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Goc0 {
    #[inline(always)]
    fn from(val: u8) -> Goc0 {
        Goc0::from_bits(val)
    }
}
impl From<Goc0> for u8 {
    #[inline(always)]
    fn from(val: Goc0) -> u8 {
        Goc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Goc1 {
    #[doc = "GMII Operation Command 1 is disabled"]
    DISABLE = 0x0,
    #[doc = "GMII Operation Command 1 is enabled"]
    ENABLE = 0x01,
}
impl Goc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Goc1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Goc1 {
    #[inline(always)]
    fn from(val: u8) -> Goc1 {
        Goc1::from_bits(val)
    }
}
impl From<Goc1> for u8 {
    #[inline(always)]
    fn from(val: Goc1) -> u8 {
        Goc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpslce {
    #[doc = "Giant Packet Size Limit Control is disabled"]
    DISABLE = 0x0,
    #[doc = "Giant Packet Size Limit Control is enabled"]
    ENABLE = 0x01,
}
impl Gpslce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpslce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpslce {
    #[inline(always)]
    fn from(val: u8) -> Gpslce {
        Gpslce::from_bits(val)
    }
}
impl From<Gpslce> for u8 {
    #[inline(always)]
    fn from(val: Gpslce) -> u8 {
        Gpslce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hashtblsz {
    #[doc = "No hash table"]
    NO_HT = 0x0,
    #[doc = "64"]
    M_64 = 0x01,
    #[doc = "128"]
    M_128 = 0x02,
    #[doc = "256"]
    M_256 = 0x03,
}
impl Hashtblsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hashtblsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hashtblsz {
    #[inline(always)]
    fn from(val: u8) -> Hashtblsz {
        Hashtblsz::from_bits(val)
    }
}
impl From<Hashtblsz> for u8 {
    #[inline(always)]
    fn from(val: Hashtblsz) -> u8 {
        Hashtblsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hdsel {
    #[doc = "No Half-duplex support"]
    INACTIVE = 0x0,
    #[doc = "Half-duplex support"]
    ACTIVE = 0x01,
}
impl Hdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hdsel {
    #[inline(always)]
    fn from(val: u8) -> Hdsel {
        Hdsel::from_bits(val)
    }
}
impl From<Hdsel> for u8 {
    #[inline(always)]
    fn from(val: Hdsel) -> u8 {
        Hdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipc {
    #[doc = "IP header/payload checksum checking is disabled"]
    DISABLE = 0x0,
    #[doc = "IP header/payload checksum checking is enabled"]
    ENABLE = 0x01,
}
impl Ipc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipc {
    #[inline(always)]
    fn from(val: u8) -> Ipc {
        Ipc::from_bits(val)
    }
}
impl From<Ipc> for u8 {
    #[inline(always)]
    fn from(val: Ipc) -> u8 {
        Ipc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipg {
    #[doc = "96 bit times IPG"]
    IPG96 = 0x0,
    #[doc = "88 bit times IPG"]
    IPG88 = 0x01,
    #[doc = "80 bit times IPG"]
    IPG80 = 0x02,
    #[doc = "72 bit times IPG"]
    IPG72 = 0x03,
    #[doc = "64 bit times IPG"]
    IPG64 = 0x04,
    #[doc = "56 bit times IPG"]
    IPG56 = 0x05,
    #[doc = "48 bit times IPG"]
    IPG48 = 0x06,
    #[doc = "40 bit times IPG"]
    IPG40 = 0x07,
}
impl Ipg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipg {
    #[inline(always)]
    fn from(val: u8) -> Ipg {
        Ipg::from_bits(val)
    }
}
impl From<Ipg> for u8 {
    #[inline(always)]
    fn from(val: Ipg) -> u8 {
        Ipg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jd {
    #[doc = "Jabber is enabled"]
    ENABLE = 0x0,
    #[doc = "Jabber is disabled"]
    DISABLE = 0x01,
}
impl Jd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jd {
    #[inline(always)]
    fn from(val: u8) -> Jd {
        Jd::from_bits(val)
    }
}
impl From<Jd> for u8 {
    #[inline(always)]
    fn from(val: Jd) -> u8 {
        Jd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Je {
    #[doc = "Jumbo packet is disabled"]
    DISABLE = 0x0,
    #[doc = "Jumbo packet is enabled"]
    ENABLE = 0x01,
}
impl Je {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Je {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Je {
    #[inline(always)]
    fn from(val: u8) -> Je {
        Je::from_bits(val)
    }
}
impl From<Je> for u8 {
    #[inline(always)]
    fn from(val: Je) -> u8 {
        Je::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L3l4fnum {
    #[doc = "No L3 or L4 Filter"]
    NOFILT = 0x0,
    #[doc = "1 L3 or L4 Filter"]
    M_1FILT = 0x01,
    #[doc = "2 L3 or L4 Filters"]
    M_2FILT = 0x02,
    #[doc = "3 L3 or L4 Filters"]
    M_3FILT = 0x03,
    #[doc = "4 L3 or L4 Filters"]
    M_4FILT = 0x04,
    #[doc = "5 L3 or L4 Filters"]
    M_5FILT = 0x05,
    #[doc = "6 L3 or L4 Filters"]
    M_6FILT = 0x06,
    #[doc = "7 L3 or L4 Filters"]
    M_7FILT = 0x07,
    #[doc = "8 L3 or L4 Filters"]
    M_8FILT = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl L3l4fnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L3l4fnum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L3l4fnum {
    #[inline(always)]
    fn from(val: u8) -> L3l4fnum {
        L3l4fnum::from_bits(val)
    }
}
impl From<L3l4fnum> for u8 {
    #[inline(always)]
    fn from(val: L3l4fnum) -> u8 {
        L3l4fnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcarr {
    #[doc = "Carrier is present"]
    INACTIVE = 0x0,
    #[doc = "Loss of carrier"]
    ACTIVE = 0x01,
}
impl Lcarr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcarr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcarr {
    #[inline(always)]
    fn from(val: u8) -> Lcarr {
        Lcarr::from_bits(val)
    }
}
impl From<Lcarr> for u8 {
    #[inline(always)]
    fn from(val: Lcarr) -> u8 {
        Lcarr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lcol {
    #[doc = "No collision"]
    INACTIVE = 0x0,
    #[doc = "Late collision is sensed"]
    ACTIVE = 0x01,
}
impl Lcol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lcol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lcol {
    #[inline(always)]
    fn from(val: u8) -> Lcol {
        Lcol::from_bits(val)
    }
}
impl From<Lcol> for u8 {
    #[inline(always)]
    fn from(val: Lcol) -> u8 {
        Lcol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lm {
    #[doc = "Loopback is disabled"]
    DISABLE = 0x0,
    #[doc = "Loopback is enabled"]
    ENABLE = 0x01,
}
impl Lm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lm {
    #[inline(always)]
    fn from(val: u8) -> Lm {
        Lm::from_bits(val)
    }
}
impl From<Lm> for u8 {
    #[inline(always)]
    fn from(val: Lm) -> u8 {
        Lm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpiate {
    #[doc = "LPI Timer is disabled"]
    DISABLE = 0x0,
    #[doc = "LPI Timer is enabled"]
    ENABLE = 0x01,
}
impl Lpiate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpiate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpiate {
    #[inline(always)]
    fn from(val: u8) -> Lpiate {
        Lpiate::from_bits(val)
    }
}
impl From<Lpiate> for u8 {
    #[inline(always)]
    fn from(val: Lpiate) -> u8 {
        Lpiate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpien {
    #[doc = "LPI state is disabled"]
    DISABLE = 0x0,
    #[doc = "LPI state is enabled"]
    ENABLE = 0x01,
}
impl Lpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpien {
    #[inline(always)]
    fn from(val: u8) -> Lpien {
        Lpien::from_bits(val)
    }
}
impl From<Lpien> for u8 {
    #[inline(always)]
    fn from(val: Lpien) -> u8 {
        Lpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpiie {
    #[doc = "LPI Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "LPI Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Lpiie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpiie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpiie {
    #[inline(always)]
    fn from(val: u8) -> Lpiie {
        Lpiie::from_bits(val)
    }
}
impl From<Lpiie> for u8 {
    #[inline(always)]
    fn from(val: Lpiie) -> u8 {
        Lpiie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpiis {
    #[doc = "LPI Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "LPI Interrupt status active"]
    ACTIVE = 0x01,
}
impl Lpiis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpiis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpiis {
    #[inline(always)]
    fn from(val: u8) -> Lpiis {
        Lpiis::from_bits(val)
    }
}
impl From<Lpiis> for u8 {
    #[inline(always)]
    fn from(val: Lpiis) -> u8 {
        Lpiis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpitcse {
    #[doc = "LPI Tx Clock Stop is disabled"]
    DISABLE = 0x0,
    #[doc = "LPI Tx Clock Stop is enabled"]
    ENABLE = 0x01,
}
impl Lpitcse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpitcse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpitcse {
    #[inline(always)]
    fn from(val: u8) -> Lpitcse {
        Lpitcse::from_bits(val)
    }
}
impl From<Lpitcse> for u8 {
    #[inline(always)]
    fn from(val: Lpitcse) -> u8 {
        Lpitcse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpitxa {
    #[doc = "LPI Tx Automate is disabled"]
    DISABLE = 0x0,
    #[doc = "LPI Tx Automate is enabled"]
    ENABLE = 0x01,
}
impl Lpitxa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpitxa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpitxa {
    #[inline(always)]
    fn from(val: u8) -> Lpitxa {
        Lpitxa::from_bits(val)
    }
}
impl From<Lpitxa> for u8 {
    #[inline(always)]
    fn from(val: Lpitxa) -> u8 {
        Lpitxa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted"]
    CVLAN = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted"]
    SVLAN = 0x01,
}
impl MacInnerVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclCsvl {
        MacInnerVlanInclCsvl::from_bits(val)
    }
}
impl From<MacInnerVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclCsvl) -> u8 {
        MacInnerVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NONE = 0x0,
    #[doc = "VLAN tag deletion"]
    DELETE = 0x01,
    #[doc = "VLAN tag insertion"]
    INSERT = 0x02,
    #[doc = "VLAN tag replacement"]
    REPLACE = 0x03,
}
impl MacInnerVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclVlc {
        MacInnerVlanInclVlc::from_bits(val)
    }
}
impl From<MacInnerVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclVlc) -> u8 {
        MacInnerVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclVlp {
    #[doc = "VLAN Priority Control is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Priority Control is enabled"]
    ENABLE = 0x01,
}
impl MacInnerVlanInclVlp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclVlp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclVlp {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclVlp {
        MacInnerVlanInclVlp::from_bits(val)
    }
}
impl From<MacInnerVlanInclVlp> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclVlp) -> u8 {
        MacInnerVlanInclVlp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclVlti {
    #[doc = "VLAN Tag Input is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Tag Input is enabled"]
    ENABLE = 0x01,
}
impl MacInnerVlanInclVlti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclVlti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclVlti {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclVlti {
        MacInnerVlanInclVlti::from_bits(val)
    }
}
impl From<MacInnerVlanInclVlti> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclVlti) -> u8 {
        MacInnerVlanInclVlti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacPacketFilterPr {
    #[doc = "Promiscuous Mode is disabled"]
    DISABLE = 0x0,
    #[doc = "Promiscuous Mode is enabled"]
    ENABLE = 0x01,
}
impl MacPacketFilterPr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacPacketFilterPr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacPacketFilterPr {
    #[inline(always)]
    fn from(val: u8) -> MacPacketFilterPr {
        MacPacketFilterPr::from_bits(val)
    }
}
impl From<MacPacketFilterPr> for u8 {
    #[inline(always)]
    fn from(val: MacPacketFilterPr) -> u8 {
        MacPacketFilterPr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacRxTxStatusRwt {
    #[doc = "No receive watchdog timeout"]
    INACTIVE = 0x0,
    #[doc = "Receive watchdog timed out"]
    ACTIVE = 0x01,
}
impl MacRxTxStatusRwt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacRxTxStatusRwt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacRxTxStatusRwt {
    #[inline(always)]
    fn from(val: u8) -> MacRxTxStatusRwt {
        MacRxTxStatusRwt::from_bits(val)
    }
}
impl From<MacRxTxStatusRwt> for u8 {
    #[inline(always)]
    fn from(val: MacRxTxStatusRwt) -> u8 {
        MacRxTxStatusRwt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted or replaced"]
    C_VLAN = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted or replaced"]
    S_VLAN = 0x01,
}
impl MacVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclCsvl {
        MacVlanInclCsvl::from_bits(val)
    }
}
impl From<MacVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclCsvl) -> u8 {
        MacVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NONE = 0x0,
    #[doc = "VLAN tag deletion"]
    DELETE = 0x01,
    #[doc = "VLAN tag insertion"]
    INSERT = 0x02,
    #[doc = "VLAN tag replacement"]
    REPLACE = 0x03,
}
impl MacVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclVlc {
        MacVlanInclVlc::from_bits(val)
    }
}
impl From<MacVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclVlc) -> u8 {
        MacVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclVlp {
    #[doc = "VLAN Priority Control is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Priority Control is enabled"]
    ENABLE = 0x01,
}
impl MacVlanInclVlp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclVlp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclVlp {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclVlp {
        MacVlanInclVlp::from_bits(val)
    }
}
impl From<MacVlanInclVlp> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclVlp) -> u8 {
        MacVlanInclVlp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclVlti {
    #[doc = "VLAN Tag Input is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Tag Input is enabled"]
    ENABLE = 0x01,
}
impl MacVlanInclVlti {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclVlti {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclVlti {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclVlti {
        MacVlanInclVlti::from_bits(val)
    }
}
impl From<MacVlanInclVlti> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclVlti) -> u8 {
        MacVlanInclVlti::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Macadr32sel {
    #[doc = "MAC Addresses 32-63 Select option is not selected"]
    INACTIVE = 0x0,
    #[doc = "MAC Addresses 32-63 Select option is selected"]
    ACTIVE = 0x01,
}
impl Macadr32sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Macadr32sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Macadr32sel {
    #[inline(always)]
    fn from(val: u8) -> Macadr32sel {
        Macadr32sel::from_bits(val)
    }
}
impl From<Macadr32sel> for u8 {
    #[inline(always)]
    fn from(val: Macadr32sel) -> u8 {
        Macadr32sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Macadr64sel {
    #[doc = "MAC Addresses 64-127 Select option is not selected"]
    INACTIVE = 0x0,
    #[doc = "MAC Addresses 64-127 Select option is selected"]
    ACTIVE = 0x01,
}
impl Macadr64sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Macadr64sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Macadr64sel {
    #[inline(always)]
    fn from(val: u8) -> Macadr64sel {
        Macadr64sel::from_bits(val)
    }
}
impl From<Macadr64sel> for u8 {
    #[inline(always)]
    fn from(val: Macadr64sel) -> u8 {
        Macadr64sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Macis {
    #[doc = "MAC Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MAC Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl Macis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Macis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Macis {
    #[inline(always)]
    fn from(val: u8) -> Macis {
        Macis::from_bits(val)
    }
}
impl From<Macis> for u8 {
    #[inline(always)]
    fn from(val: Macis) -> u8 {
        Macis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mb {
    #[doc = "Mixed Burst is disabled"]
    DISABLE = 0x0,
    #[doc = "Mixed Burst is enabled"]
    ENABLE = 0x01,
}
impl Mb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mb {
    #[inline(always)]
    fn from(val: u8) -> Mb {
        Mb::from_bits(val)
    }
}
impl From<Mb> for u8 {
    #[inline(always)]
    fn from(val: Mb) -> u8 {
        Mb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mcbcq {
    #[doc = "Receive Queue 0"]
    QUEUE0 = 0x0,
    #[doc = "Receive Queue 1"]
    QUEUE1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mcbcq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcbcq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcbcq {
    #[inline(always)]
    fn from(val: u8) -> Mcbcq {
        Mcbcq::from_bits(val)
    }
}
impl From<Mcbcq> for u8 {
    #[inline(always)]
    fn from(val: Mcbcq) -> u8 {
        Mcbcq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mcbcqen {
    #[doc = "Multicast and Broadcast Queue is disabled"]
    DISABLE = 0x0,
    #[doc = "Multicast and Broadcast Queue is enabled"]
    ENABLE = 0x01,
}
impl Mcbcqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcbcqen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcbcqen {
    #[inline(always)]
    fn from(val: u8) -> Mcbcqen {
        Mcbcqen::from_bits(val)
    }
}
impl From<Mcbcqen> for u8 {
    #[inline(always)]
    fn from(val: Mcbcqen) -> u8 {
        Mcbcqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdioie {
    #[doc = "MDIO Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "MDIO Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Mdioie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdioie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdioie {
    #[inline(always)]
    fn from(val: u8) -> Mdioie {
        Mdioie::from_bits(val)
    }
}
impl From<Mdioie> for u8 {
    #[inline(always)]
    fn from(val: Mdioie) -> u8 {
        Mdioie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdiois {
    #[doc = "MDIO Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "MDIO Interrupt status active"]
    ACTIVE = 0x01,
}
impl Mdiois {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdiois {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdiois {
    #[inline(always)]
    fn from(val: u8) -> Mdiois {
        Mdiois::from_bits(val)
    }
}
impl From<Mdiois> for u8 {
    #[inline(always)]
    fn from(val: Mdiois) -> u8 {
        Mdiois::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mffqe {
    #[doc = "Multicast Address Filter Fail Packets Queuing is disabled"]
    DISABLE = 0x0,
    #[doc = "Multicast Address Filter Fail Packets Queuing is enabled"]
    ENABLE = 0x01,
}
impl Mffqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mffqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mffqe {
    #[inline(always)]
    fn from(val: u8) -> Mffqe {
        Mffqe::from_bits(val)
    }
}
impl From<Mffqe> for u8 {
    #[inline(always)]
    fn from(val: Mffqe) -> u8 {
        Mffqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mgkpkten {
    #[doc = "Magic Packet is disabled"]
    DISABLE = 0x0,
    #[doc = "Magic Packet is enabled"]
    ENABLE = 0x01,
}
impl Mgkpkten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mgkpkten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mgkpkten {
    #[inline(always)]
    fn from(val: u8) -> Mgkpkten {
        Mgkpkten::from_bits(val)
    }
}
impl From<Mgkpkten> for u8 {
    #[inline(always)]
    fn from(val: Mgkpkten) -> u8 {
        Mgkpkten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mgkprcvd {
    #[doc = "No Magic packet is received"]
    INACTIVE = 0x0,
    #[doc = "Magic packet is received"]
    ACTIVE = 0x01,
}
impl Mgkprcvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mgkprcvd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mgkprcvd {
    #[inline(always)]
    fn from(val: u8) -> Mgkprcvd {
        Mgkprcvd::from_bits(val)
    }
}
impl From<Mgkprcvd> for u8 {
    #[inline(always)]
    fn from(val: Mgkprcvd) -> u8 {
        Mgkprcvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mgksel {
    #[doc = "PMT Magic Packet Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "PMT Magic Packet Enable option is selected"]
    ACTIVE = 0x01,
}
impl Mgksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mgksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mgksel {
    #[inline(always)]
    fn from(val: u8) -> Mgksel {
        Mgksel::from_bits(val)
    }
}
impl From<Mgksel> for u8 {
    #[inline(always)]
    fn from(val: Mgksel) -> u8 {
        Mgksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Miisel {
    #[doc = "No 10 or 100 Mbps support"]
    INACTIVE = 0x0,
    #[doc = "10 or 100 Mbps support"]
    ACTIVE = 0x01,
}
impl Miisel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Miisel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Miisel {
    #[inline(always)]
    fn from(val: u8) -> Miisel {
        Miisel::from_bits(val)
    }
}
impl From<Miisel> for u8 {
    #[inline(always)]
    fn from(val: Miisel) -> u8 {
        Miisel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mmcsel {
    #[doc = "RMON Module Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "RMON Module Enable option is selected"]
    ACTIVE = 0x01,
}
impl Mmcsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mmcsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mmcsel {
    #[inline(always)]
    fn from(val: u8) -> Mmcsel {
        Mmcsel::from_bits(val)
    }
}
impl From<Mmcsel> for u8 {
    #[inline(always)]
    fn from(val: Mmcsel) -> u8 {
        Mmcsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusAbpsie {
    #[doc = "Average Bits Per Slot Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Average Bits Per Slot Interrupt is enabled"]
    ENABLE = 0x01,
}
impl MtlQ0InterruptControlStatusAbpsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusAbpsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusAbpsie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusAbpsie {
        MtlQ0InterruptControlStatusAbpsie::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusAbpsie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusAbpsie) -> u8 {
        MtlQ0InterruptControlStatusAbpsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusAbpsis {
    #[doc = "Average Bits Per Slot Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Average Bits Per Slot Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ0InterruptControlStatusAbpsis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusAbpsis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusAbpsis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusAbpsis {
        MtlQ0InterruptControlStatusAbpsis::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusAbpsis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusAbpsis) -> u8 {
        MtlQ0InterruptControlStatusAbpsis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusRxoie {
    #[doc = "Receive Queue Overflow Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Overflow Interrupt is enabled"]
    ENABLE = 0x01,
}
impl MtlQ0InterruptControlStatusRxoie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusRxoie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusRxoie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusRxoie {
        MtlQ0InterruptControlStatusRxoie::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusRxoie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusRxoie) -> u8 {
        MtlQ0InterruptControlStatusRxoie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusRxovfis {
    #[doc = "Receive Queue Overflow Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Queue Overflow Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ0InterruptControlStatusRxovfis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusRxovfis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusRxovfis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusRxovfis {
        MtlQ0InterruptControlStatusRxovfis::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusRxovfis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusRxovfis) -> u8 {
        MtlQ0InterruptControlStatusRxovfis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusTxuie {
    #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
    ENABLE = 0x01,
}
impl MtlQ0InterruptControlStatusTxuie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusTxuie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusTxuie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusTxuie {
        MtlQ0InterruptControlStatusTxuie::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusTxuie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusTxuie) -> u8 {
        MtlQ0InterruptControlStatusTxuie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ0InterruptControlStatusTxunfis {
    #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Queue Underflow Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ0InterruptControlStatusTxunfis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ0InterruptControlStatusTxunfis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ0InterruptControlStatusTxunfis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ0InterruptControlStatusTxunfis {
        MtlQ0InterruptControlStatusTxunfis::from_bits(val)
    }
}
impl From<MtlQ0InterruptControlStatusTxunfis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ0InterruptControlStatusTxunfis) -> u8 {
        MtlQ0InterruptControlStatusTxunfis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusAbpsie {
    #[doc = "Average Bits Per Slot Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Average Bits Per Slot Interrupt is enabled"]
    ENABLE = 0x01,
}
impl MtlQ1InterruptControlStatusAbpsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusAbpsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusAbpsie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusAbpsie {
        MtlQ1InterruptControlStatusAbpsie::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusAbpsie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusAbpsie) -> u8 {
        MtlQ1InterruptControlStatusAbpsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusAbpsis {
    #[doc = "Average Bits Per Slot Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Average Bits Per Slot Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ1InterruptControlStatusAbpsis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusAbpsis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusAbpsis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusAbpsis {
        MtlQ1InterruptControlStatusAbpsis::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusAbpsis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusAbpsis) -> u8 {
        MtlQ1InterruptControlStatusAbpsis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusRxoie {
    #[doc = "Receive Queue Overflow Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Overflow Interrupt is enabled"]
    ENABLE = 0x01,
}
impl MtlQ1InterruptControlStatusRxoie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusRxoie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusRxoie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusRxoie {
        MtlQ1InterruptControlStatusRxoie::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusRxoie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusRxoie) -> u8 {
        MtlQ1InterruptControlStatusRxoie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusRxovfis {
    #[doc = "Receive Queue Overflow Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive Queue Overflow Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ1InterruptControlStatusRxovfis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusRxovfis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusRxovfis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusRxovfis {
        MtlQ1InterruptControlStatusRxovfis::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusRxovfis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusRxovfis) -> u8 {
        MtlQ1InterruptControlStatusRxovfis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusTxuie {
    #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
    ENABLE = 0x01,
}
impl MtlQ1InterruptControlStatusTxuie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusTxuie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusTxuie {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusTxuie {
        MtlQ1InterruptControlStatusTxuie::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusTxuie> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusTxuie) -> u8 {
        MtlQ1InterruptControlStatusTxuie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlQ1InterruptControlStatusTxunfis {
    #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Queue Underflow Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl MtlQ1InterruptControlStatusTxunfis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlQ1InterruptControlStatusTxunfis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlQ1InterruptControlStatusTxunfis {
    #[inline(always)]
    fn from(val: u8) -> MtlQ1InterruptControlStatusTxunfis {
        MtlQ1InterruptControlStatusTxunfis::from_bits(val)
    }
}
impl From<MtlQ1InterruptControlStatusTxunfis> for u8 {
    #[inline(always)]
    fn from(val: MtlQ1InterruptControlStatusTxunfis) -> u8 {
        MtlQ1InterruptControlStatusTxunfis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0ControlRxqFrmArbit {
    #[doc = "Receive Queue Packet Arbitration is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Packet Arbitration is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq0ControlRxqFrmArbit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0ControlRxqFrmArbit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0ControlRxqFrmArbit {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0ControlRxqFrmArbit {
        MtlRxq0ControlRxqFrmArbit::from_bits(val)
    }
}
impl From<MtlRxq0ControlRxqFrmArbit> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0ControlRxqFrmArbit) -> u8 {
        MtlRxq0ControlRxqFrmArbit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0DebugRrcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Reading packet data"]
    READ_DATA = 0x01,
    #[doc = "Reading packet status (or timestamp)"]
    READ_STS = 0x02,
    #[doc = "Flushing the packet data and status"]
    FLUSH = 0x03,
}
impl MtlRxq0DebugRrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0DebugRrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0DebugRrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0DebugRrcsts {
        MtlRxq0DebugRrcsts::from_bits(val)
    }
}
impl From<MtlRxq0DebugRrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0DebugRrcsts) -> u8 {
        MtlRxq0DebugRrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0DebugRwcsts {
    #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Rx Queue Write Controller Active Status detected"]
    ACTIVE = 0x01,
}
impl MtlRxq0DebugRwcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0DebugRwcsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0DebugRwcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0DebugRwcsts {
        MtlRxq0DebugRwcsts::from_bits(val)
    }
}
impl From<MtlRxq0DebugRwcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0DebugRwcsts) -> u8 {
        MtlRxq0DebugRwcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0DebugRxqsts {
    #[doc = "Rx Queue empty"]
    EMPTY = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
    BLW_THR = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold"]
    ABV_THR = 0x02,
    #[doc = "Rx Queue full"]
    FULL = 0x03,
}
impl MtlRxq0DebugRxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0DebugRxqsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0DebugRxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0DebugRxqsts {
        MtlRxq0DebugRxqsts::from_bits(val)
    }
}
impl From<MtlRxq0DebugRxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0DebugRxqsts) -> u8 {
        MtlRxq0DebugRxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0MissedPacketOverflowCntMiscntovf {
    #[doc = "Missed Packet Counter overflow not detected"]
    INACTIVE = 0x0,
    #[doc = "Missed Packet Counter overflow detected"]
    ACTIVE = 0x01,
}
impl MtlRxq0MissedPacketOverflowCntMiscntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0MissedPacketOverflowCntMiscntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0MissedPacketOverflowCntMiscntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0MissedPacketOverflowCntMiscntovf {
        MtlRxq0MissedPacketOverflowCntMiscntovf::from_bits(val)
    }
}
impl From<MtlRxq0MissedPacketOverflowCntMiscntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0MissedPacketOverflowCntMiscntovf) -> u8 {
        MtlRxq0MissedPacketOverflowCntMiscntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0MissedPacketOverflowCntOvfcntovf {
    #[doc = "Overflow Counter overflow not detected"]
    INACTIVE = 0x0,
    #[doc = "Overflow Counter overflow detected"]
    ACTIVE = 0x01,
}
impl MtlRxq0MissedPacketOverflowCntOvfcntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0MissedPacketOverflowCntOvfcntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0MissedPacketOverflowCntOvfcntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0MissedPacketOverflowCntOvfcntovf {
        MtlRxq0MissedPacketOverflowCntOvfcntovf::from_bits(val)
    }
}
impl From<MtlRxq0MissedPacketOverflowCntOvfcntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0MissedPacketOverflowCntOvfcntovf) -> u8 {
        MtlRxq0MissedPacketOverflowCntOvfcntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeDisTcpEf {
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
    ENABLE = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
    DISABLE = 0x01,
}
impl MtlRxq0OperationModeDisTcpEf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeDisTcpEf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeDisTcpEf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeDisTcpEf {
        MtlRxq0OperationModeDisTcpEf::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeDisTcpEf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeDisTcpEf) -> u8 {
        MtlRxq0OperationModeDisTcpEf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeFep {
    #[doc = "Forward Error Packets is disabled"]
    DISABLE = 0x0,
    #[doc = "Forward Error Packets is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq0OperationModeFep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeFep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeFep {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeFep {
        MtlRxq0OperationModeFep::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeFep> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeFep) -> u8 {
        MtlRxq0OperationModeFep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeFup {
    #[doc = "Forward Undersized Good Packets is disabled"]
    DISABLE = 0x0,
    #[doc = "Forward Undersized Good Packets is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq0OperationModeFup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeFup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeFup {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeFup {
        MtlRxq0OperationModeFup::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeFup> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeFup) -> u8 {
        MtlRxq0OperationModeFup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeRsf {
    #[doc = "Receive Queue Store and Forward is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Store and Forward is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq0OperationModeRsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeRsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeRsf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeRsf {
        MtlRxq0OperationModeRsf::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeRsf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeRsf) -> u8 {
        MtlRxq0OperationModeRsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeRtc {
    #[doc = "64"]
    M_64BYTE = 0x0,
    #[doc = "32"]
    M_32BYTE = 0x01,
    #[doc = "96"]
    M_96BYTE = 0x02,
    #[doc = "128"]
    M_128BYTE = 0x03,
}
impl MtlRxq0OperationModeRtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeRtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeRtc {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeRtc {
        MtlRxq0OperationModeRtc::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeRtc> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeRtc) -> u8 {
        MtlRxq0OperationModeRtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1ControlRxqFrmArbit {
    #[doc = "Receive Queue Packet Arbitration is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Packet Arbitration is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq1ControlRxqFrmArbit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1ControlRxqFrmArbit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1ControlRxqFrmArbit {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1ControlRxqFrmArbit {
        MtlRxq1ControlRxqFrmArbit::from_bits(val)
    }
}
impl From<MtlRxq1ControlRxqFrmArbit> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1ControlRxqFrmArbit) -> u8 {
        MtlRxq1ControlRxqFrmArbit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1DebugRrcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Reading packet data"]
    READ_DATA = 0x01,
    #[doc = "Reading packet status (or timestamp)"]
    READ_STS = 0x02,
    #[doc = "Flushing the packet data and status"]
    FLUSH = 0x03,
}
impl MtlRxq1DebugRrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1DebugRrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1DebugRrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1DebugRrcsts {
        MtlRxq1DebugRrcsts::from_bits(val)
    }
}
impl From<MtlRxq1DebugRrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1DebugRrcsts) -> u8 {
        MtlRxq1DebugRrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1DebugRwcsts {
    #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Rx Queue Write Controller Active Status detected"]
    ACTIVE = 0x01,
}
impl MtlRxq1DebugRwcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1DebugRwcsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1DebugRwcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1DebugRwcsts {
        MtlRxq1DebugRwcsts::from_bits(val)
    }
}
impl From<MtlRxq1DebugRwcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1DebugRwcsts) -> u8 {
        MtlRxq1DebugRwcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1DebugRxqsts {
    #[doc = "Rx Queue empty"]
    EMPTY = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
    BLW_THR = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold"]
    ABV_THR = 0x02,
    #[doc = "Rx Queue full"]
    FULL = 0x03,
}
impl MtlRxq1DebugRxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1DebugRxqsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1DebugRxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1DebugRxqsts {
        MtlRxq1DebugRxqsts::from_bits(val)
    }
}
impl From<MtlRxq1DebugRxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1DebugRxqsts) -> u8 {
        MtlRxq1DebugRxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1MissedPacketOverflowCntMiscntovf {
    #[doc = "Missed Packet Counter overflow not detected"]
    INACTIVE = 0x0,
    #[doc = "Missed Packet Counter overflow detected"]
    ACTIVE = 0x01,
}
impl MtlRxq1MissedPacketOverflowCntMiscntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1MissedPacketOverflowCntMiscntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1MissedPacketOverflowCntMiscntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1MissedPacketOverflowCntMiscntovf {
        MtlRxq1MissedPacketOverflowCntMiscntovf::from_bits(val)
    }
}
impl From<MtlRxq1MissedPacketOverflowCntMiscntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1MissedPacketOverflowCntMiscntovf) -> u8 {
        MtlRxq1MissedPacketOverflowCntMiscntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1MissedPacketOverflowCntOvfcntovf {
    #[doc = "Overflow Counter overflow not detected"]
    INACTIVE = 0x0,
    #[doc = "Overflow Counter overflow detected"]
    ACTIVE = 0x01,
}
impl MtlRxq1MissedPacketOverflowCntOvfcntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1MissedPacketOverflowCntOvfcntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1MissedPacketOverflowCntOvfcntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1MissedPacketOverflowCntOvfcntovf {
        MtlRxq1MissedPacketOverflowCntOvfcntovf::from_bits(val)
    }
}
impl From<MtlRxq1MissedPacketOverflowCntOvfcntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1MissedPacketOverflowCntOvfcntovf) -> u8 {
        MtlRxq1MissedPacketOverflowCntOvfcntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeDisTcpEf {
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
    ENABLE = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
    DISABLE = 0x01,
}
impl MtlRxq1OperationModeDisTcpEf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeDisTcpEf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeDisTcpEf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeDisTcpEf {
        MtlRxq1OperationModeDisTcpEf::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeDisTcpEf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeDisTcpEf) -> u8 {
        MtlRxq1OperationModeDisTcpEf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeFep {
    #[doc = "Forward Error Packets is disabled"]
    DISABLE = 0x0,
    #[doc = "Forward Error Packets is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq1OperationModeFep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeFep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeFep {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeFep {
        MtlRxq1OperationModeFep::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeFep> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeFep) -> u8 {
        MtlRxq1OperationModeFep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeFup {
    #[doc = "Forward Undersized Good Packets is disabled"]
    DISABLE = 0x0,
    #[doc = "Forward Undersized Good Packets is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq1OperationModeFup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeFup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeFup {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeFup {
        MtlRxq1OperationModeFup::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeFup> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeFup) -> u8 {
        MtlRxq1OperationModeFup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeRsf {
    #[doc = "Receive Queue Store and Forward is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Queue Store and Forward is enabled"]
    ENABLE = 0x01,
}
impl MtlRxq1OperationModeRsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeRsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeRsf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeRsf {
        MtlRxq1OperationModeRsf::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeRsf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeRsf) -> u8 {
        MtlRxq1OperationModeRsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeRtc {
    #[doc = "64"]
    M_64BYTE = 0x0,
    #[doc = "32"]
    M_32BYTE = 0x01,
    #[doc = "96"]
    M_96BYTE = 0x02,
    #[doc = "128"]
    M_128BYTE = 0x03,
}
impl MtlRxq1OperationModeRtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeRtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeRtc {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeRtc {
        MtlRxq1OperationModeRtc::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeRtc> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeRtc) -> u8 {
        MtlRxq1OperationModeRtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTrcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)"]
    READ = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
    WAIT = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
    FLUSH = 0x03,
}
impl MtlTxq0DebugTrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTrcsts {
        MtlTxq0DebugTrcsts::from_bits(val)
    }
}
impl From<MtlTxq0DebugTrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTrcsts) -> u8 {
        MtlTxq0DebugTrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTwcsts {
    #[doc = "MTL Tx Queue Write Controller status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Queue Write Controller status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq0DebugTwcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTwcsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTwcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTwcsts {
        MtlTxq0DebugTwcsts::from_bits(val)
    }
}
impl From<MtlTxq0DebugTwcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTwcsts) -> u8 {
        MtlTxq0DebugTwcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTxqpaused {
    #[doc = "Transmit Queue in Pause status is not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Queue in Pause status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq0DebugTxqpaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTxqpaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTxqpaused {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTxqpaused {
        MtlTxq0DebugTxqpaused::from_bits(val)
    }
}
impl From<MtlTxq0DebugTxqpaused> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTxqpaused) -> u8 {
        MtlTxq0DebugTxqpaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTxqsts {
    #[doc = "MTL Tx Queue Not Empty status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Queue Not Empty status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq0DebugTxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTxqsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTxqsts {
        MtlTxq0DebugTxqsts::from_bits(val)
    }
}
impl From<MtlTxq0DebugTxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTxqsts) -> u8 {
        MtlTxq0DebugTxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTxstsfsts {
    #[doc = "MTL Tx Status FIFO Full status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Status FIFO Full status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq0DebugTxstsfsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTxstsfsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTxstsfsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTxstsfsts {
        MtlTxq0DebugTxstsfsts::from_bits(val)
    }
}
impl From<MtlTxq0DebugTxstsfsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTxstsfsts) -> u8 {
        MtlTxq0DebugTxstsfsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeFtq {
    #[doc = "Flush Transmit Queue is disabled"]
    DISABLE = 0x0,
    #[doc = "Flush Transmit Queue is enabled"]
    ENABLE = 0x01,
}
impl MtlTxq0OperationModeFtq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeFtq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeFtq {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeFtq {
        MtlTxq0OperationModeFtq::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeFtq> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeFtq) -> u8 {
        MtlTxq0OperationModeFtq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeTsf {
    #[doc = "Transmit Store and Forward is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Store and Forward is enabled"]
    ENABLE = 0x01,
}
impl MtlTxq0OperationModeTsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeTsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeTsf {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeTsf {
        MtlTxq0OperationModeTsf::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeTsf> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeTsf) -> u8 {
        MtlTxq0OperationModeTsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeTtc {
    #[doc = "32"]
    M_32BYTES = 0x0,
    #[doc = "64"]
    M_64BYTES = 0x01,
    #[doc = "96"]
    M_96BYTES = 0x02,
    #[doc = "128"]
    M_128BYTES = 0x03,
    #[doc = "192"]
    M_192BYTES = 0x04,
    #[doc = "256"]
    M_256BYTES = 0x05,
    #[doc = "384"]
    M_384BYTES = 0x06,
    #[doc = "512"]
    M_512BYTES = 0x07,
}
impl MtlTxq0OperationModeTtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeTtc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeTtc {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeTtc {
        MtlTxq0OperationModeTtc::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeTtc> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeTtc) -> u8 {
        MtlTxq0OperationModeTtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeTxqen {
    #[doc = "Not enabled"]
    DISABLE = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)"]
    EN_IF_AV = 0x01,
    #[doc = "Enabled"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MtlTxq0OperationModeTxqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeTxqen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeTxqen {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeTxqen {
        MtlTxq0OperationModeTxqen::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeTxqen> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeTxqen) -> u8 {
        MtlTxq0OperationModeTxqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0UnderflowUfcntovf {
    #[doc = "Overflow not detected for Underflow Packet Counter"]
    INACTIVE = 0x0,
    #[doc = "Overflow detected for Underflow Packet Counter"]
    ACTIVE = 0x01,
}
impl MtlTxq0UnderflowUfcntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0UnderflowUfcntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0UnderflowUfcntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0UnderflowUfcntovf {
        MtlTxq0UnderflowUfcntovf::from_bits(val)
    }
}
impl From<MtlTxq0UnderflowUfcntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0UnderflowUfcntovf) -> u8 {
        MtlTxq0UnderflowUfcntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTrcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)"]
    READ = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
    WAIT = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
    FLUSH = 0x03,
}
impl MtlTxq1DebugTrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTrcsts {
        MtlTxq1DebugTrcsts::from_bits(val)
    }
}
impl From<MtlTxq1DebugTrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTrcsts) -> u8 {
        MtlTxq1DebugTrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTwcsts {
    #[doc = "MTL Tx Queue Write Controller status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Queue Write Controller status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq1DebugTwcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTwcsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTwcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTwcsts {
        MtlTxq1DebugTwcsts::from_bits(val)
    }
}
impl From<MtlTxq1DebugTwcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTwcsts) -> u8 {
        MtlTxq1DebugTwcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTxqpaused {
    #[doc = "Transmit Queue in Pause status is not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Queue in Pause status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq1DebugTxqpaused {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTxqpaused {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTxqpaused {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTxqpaused {
        MtlTxq1DebugTxqpaused::from_bits(val)
    }
}
impl From<MtlTxq1DebugTxqpaused> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTxqpaused) -> u8 {
        MtlTxq1DebugTxqpaused::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTxqsts {
    #[doc = "MTL Tx Queue Not Empty status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Queue Not Empty status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq1DebugTxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTxqsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTxqsts {
        MtlTxq1DebugTxqsts::from_bits(val)
    }
}
impl From<MtlTxq1DebugTxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTxqsts) -> u8 {
        MtlTxq1DebugTxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTxstsfsts {
    #[doc = "MTL Tx Status FIFO Full status is not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Tx Status FIFO Full status is detected"]
    ACTIVE = 0x01,
}
impl MtlTxq1DebugTxstsfsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTxstsfsts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTxstsfsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTxstsfsts {
        MtlTxq1DebugTxstsfsts::from_bits(val)
    }
}
impl From<MtlTxq1DebugTxstsfsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTxstsfsts) -> u8 {
        MtlTxq1DebugTxstsfsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeFtq {
    #[doc = "Flush Transmit Queue is disabled"]
    DISABLE = 0x0,
    #[doc = "Flush Transmit Queue is enabled"]
    ENABLE = 0x01,
}
impl MtlTxq1OperationModeFtq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeFtq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeFtq {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeFtq {
        MtlTxq1OperationModeFtq::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeFtq> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeFtq) -> u8 {
        MtlTxq1OperationModeFtq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeTsf {
    #[doc = "Transmit Store and Forward is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Store and Forward is enabled"]
    ENABLE = 0x01,
}
impl MtlTxq1OperationModeTsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeTsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeTsf {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeTsf {
        MtlTxq1OperationModeTsf::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeTsf> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeTsf) -> u8 {
        MtlTxq1OperationModeTsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeTtc {
    #[doc = "32"]
    M_32BYTES = 0x0,
    #[doc = "64"]
    M_64BYTES = 0x01,
    #[doc = "96"]
    M_96BYTES = 0x02,
    #[doc = "128"]
    M_128BYTES = 0x03,
    #[doc = "192"]
    M_192BYTES = 0x04,
    #[doc = "256"]
    M_256BYTES = 0x05,
    #[doc = "384"]
    M_384BYTES = 0x06,
    #[doc = "512"]
    M_512BYTES = 0x07,
}
impl MtlTxq1OperationModeTtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeTtc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeTtc {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeTtc {
        MtlTxq1OperationModeTtc::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeTtc> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeTtc) -> u8 {
        MtlTxq1OperationModeTtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeTxqen {
    #[doc = "Not enabled"]
    DISABLE = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)"]
    EN_IF_AV = 0x01,
    #[doc = "Enabled"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MtlTxq1OperationModeTxqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeTxqen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeTxqen {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeTxqen {
        MtlTxq1OperationModeTxqen::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeTxqen> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeTxqen) -> u8 {
        MtlTxq1OperationModeTxqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1UnderflowUfcntovf {
    #[doc = "Overflow not detected for Underflow Packet Counter"]
    INACTIVE = 0x0,
    #[doc = "Overflow detected for Underflow Packet Counter"]
    ACTIVE = 0x01,
}
impl MtlTxq1UnderflowUfcntovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1UnderflowUfcntovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1UnderflowUfcntovf {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1UnderflowUfcntovf {
        MtlTxq1UnderflowUfcntovf::from_bits(val)
    }
}
impl From<MtlTxq1UnderflowUfcntovf> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1UnderflowUfcntovf) -> u8 {
        MtlTxq1UnderflowUfcntovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mtlis {
    #[doc = "MTL Interrupt Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MTL Interrupt Status detected"]
    ACTIVE = 0x01,
}
impl Mtlis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mtlis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mtlis {
    #[inline(always)]
    fn from(val: u8) -> Mtlis {
        Mtlis::from_bits(val)
    }
}
impl From<Mtlis> for u8 {
    #[inline(always)]
    fn from(val: Mtlis) -> u8 {
        Mtlis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ncarr {
    #[doc = "Carrier is present"]
    INACTIVE = 0x0,
    #[doc = "No carrier"]
    ACTIVE = 0x01,
}
impl Ncarr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ncarr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ncarr {
    #[inline(always)]
    fn from(val: u8) -> Ncarr {
        Ncarr::from_bits(val)
    }
}
impl From<Ncarr> for u8 {
    #[inline(always)]
    fn from(val: Ncarr) -> u8 {
        Ncarr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nrvf {
    #[doc = "No Extended Rx VLAN Filters"]
    NO_ERVLAN = 0x0,
    #[doc = "4 Extended Rx VLAN Filters"]
    M_4_ERVLAN = 0x01,
    #[doc = "8 Extended Rx VLAN Filters"]
    M_8_ERVLAN = 0x02,
    #[doc = "16 Extended Rx VLAN Filters"]
    M_16_ERVLAN = 0x03,
    #[doc = "24 Extended Rx VLAN Filters"]
    M_24_ERVLAN = 0x04,
    #[doc = "32 Extended Rx VLAN Filters"]
    M_32_ERVLAN = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nrvf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nrvf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nrvf {
    #[inline(always)]
    fn from(val: u8) -> Nrvf {
        Nrvf::from_bits(val)
    }
}
impl From<Nrvf> for u8 {
    #[inline(always)]
    fn from(val: Nrvf) -> u8 {
        Nrvf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Omcbcq {
    #[doc = "overriding MCBCQ priority disabled"]
    DISABLE = 0x0,
    #[doc = "overriding MCBCQ priority enabled"]
    ENABLE = 0x01,
}
impl Omcbcq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Omcbcq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Omcbcq {
    #[inline(always)]
    fn from(val: u8) -> Omcbcq {
        Omcbcq::from_bits(val)
    }
}
impl From<Omcbcq> for u8 {
    #[inline(always)]
    fn from(val: Omcbcq) -> u8 {
        Omcbcq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osten {
    #[doc = "One-Step Timestamping feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "One-Step Timestamping feature is selected"]
    ACTIVE = 0x01,
}
impl Osten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osten {
    #[inline(always)]
    fn from(val: u8) -> Osten {
        Osten::from_bits(val)
    }
}
impl From<Osten> for u8 {
    #[inline(always)]
    fn from(val: Osten) -> u8 {
        Osten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcf {
    #[doc = "MAC filters all control packets from reaching the application"]
    FLTR_ALL = 0x0,
    #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the address filter"]
    FW_XCPT_PAU = 0x01,
    #[doc = "MAC forwards all control packets to the application even if they fail the address filter"]
    FW_ALL = 0x02,
    #[doc = "MAC forwards the control packets that pass the Address filter"]
    FW_PASS = 0x03,
}
impl Pcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcf {
    #[inline(always)]
    fn from(val: u8) -> Pcf {
        Pcf::from_bits(val)
    }
}
impl From<Pcf> for u8 {
    #[inline(always)]
    fn from(val: Pcf) -> u8 {
        Pcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcssel {
    #[doc = "No PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
    INACTIVE = 0x0,
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
    ACTIVE = 0x01,
}
impl Pcssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcssel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcssel {
    #[inline(always)]
    fn from(val: u8) -> Pcssel {
        Pcssel::from_bits(val)
    }
}
impl From<Pcssel> for u8 {
    #[inline(always)]
    fn from(val: Pcssel) -> u8 {
        Pcssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc {
    #[doc = "Packet Duplication Control is disabled"]
    DISABLE = 0x0,
    #[doc = "Packet Duplication Control is enabled"]
    ENABLE = 0x01,
}
impl Pdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc {
    #[inline(always)]
    fn from(val: u8) -> Pdc {
        Pdc::from_bits(val)
    }
}
impl From<Pdc> for u8 {
    #[inline(always)]
    fn from(val: Pdc) -> u8 {
        Pdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdupsel {
    #[doc = "Broadcast/Multicast Packet Duplication feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Broadcast/Multicast Packet Duplication feature is selected"]
    ACTIVE = 0x01,
}
impl Pdupsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdupsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdupsel {
    #[inline(always)]
    fn from(val: u8) -> Pdupsel {
        Pdupsel::from_bits(val)
    }
}
impl From<Pdupsel> for u8 {
    #[inline(always)]
    fn from(val: Pdupsel) -> u8 {
        Pdupsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyie {
    #[doc = "PHY Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "PHY Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Phyie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyie {
    #[inline(always)]
    fn from(val: u8) -> Phyie {
        Phyie::from_bits(val)
    }
}
impl From<Phyie> for u8 {
    #[inline(always)]
    fn from(val: Phyie) -> u8 {
        Phyie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyis {
    #[doc = "PHY Interrupt not detected"]
    INACTIVE = 0x0,
    #[doc = "PHY Interrupt detected"]
    ACTIVE = 0x01,
}
impl Phyis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyis {
    #[inline(always)]
    fn from(val: u8) -> Phyis {
        Phyis::from_bits(val)
    }
}
impl From<Phyis> for u8 {
    #[inline(always)]
    fn from(val: Phyis) -> u8 {
        Phyis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pls {
    #[doc = "link is down"]
    DISABLE = 0x0,
    #[doc = "link is okay (UP)"]
    ENABLE = 0x01,
}
impl Pls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pls {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pls {
    #[inline(always)]
    fn from(val: u8) -> Pls {
        Pls::from_bits(val)
    }
}
impl From<Pls> for u8 {
    #[inline(always)]
    fn from(val: Pls) -> u8 {
        Pls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plt {
    #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
    PT4 = 0x0,
    #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
    PT28 = 0x01,
    #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
    PT36 = 0x02,
    #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
    PT144 = 0x03,
    #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
    PT256 = 0x04,
    #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
    PT512 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Plt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plt {
    #[inline(always)]
    fn from(val: u8) -> Plt {
        Plt::from_bits(val)
    }
}
impl From<Plt> for u8 {
    #[inline(always)]
    fn from(val: Plt) -> u8 {
        Plt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm {
    #[doc = "Pass All Multicast is disabled"]
    DISABLE = 0x0,
    #[doc = "Pass All Multicast is enabled"]
    ENABLE = 0x01,
}
impl Pm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm {
    #[inline(always)]
    fn from(val: u8) -> Pm {
        Pm::from_bits(val)
    }
}
impl From<Pm> for u8 {
    #[inline(always)]
    fn from(val: Pm) -> u8 {
        Pm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmtie {
    #[doc = "PMT Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "PMT Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Pmtie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmtie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmtie {
    #[inline(always)]
    fn from(val: u8) -> Pmtie {
        Pmtie::from_bits(val)
    }
}
impl From<Pmtie> for u8 {
    #[inline(always)]
    fn from(val: Pmtie) -> u8 {
        Pmtie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmtis {
    #[doc = "PMT Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "PMT Interrupt status active"]
    ACTIVE = 0x01,
}
impl Pmtis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmtis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmtis {
    #[inline(always)]
    fn from(val: u8) -> Pmtis {
        Pmtis::from_bits(val)
    }
}
impl From<Pmtis> for u8 {
    #[inline(always)]
    fn from(val: Pmtis) -> u8 {
        Pmtis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pouost {
    #[doc = "One Step for PTP over UDP/IP Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "One Step for PTP over UDP/IP Feature is selected"]
    ACTIVE = 0x01,
}
impl Pouost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pouost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pouost {
    #[inline(always)]
    fn from(val: u8) -> Pouost {
        Pouost::from_bits(val)
    }
}
impl From<Pouost> for u8 {
    #[inline(always)]
    fn from(val: Pouost) -> u8 {
        Pouost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppsoutnum {
    #[doc = "No PPS output"]
    NO_PPSO = 0x0,
    #[doc = "1 PPS output"]
    M_1_PPSO = 0x01,
    #[doc = "2 PPS output"]
    M_2_PPSO = 0x02,
    #[doc = "3 PPS output"]
    M_3_PPSO = 0x03,
    #[doc = "4 PPS output"]
    M_4_PPSO = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ppsoutnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppsoutnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppsoutnum {
    #[inline(always)]
    fn from(val: u8) -> Ppsoutnum {
        Ppsoutnum::from_bits(val)
    }
}
impl From<Ppsoutnum> for u8 {
    #[inline(always)]
    fn from(val: Ppsoutnum) -> u8 {
        Ppsoutnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prelen {
    #[doc = "7 bytes of preamble"]
    M_7BYTES = 0x0,
    #[doc = "5 bytes of preamble"]
    M_5BYTES = 0x01,
    #[doc = "3 bytes of preamble"]
    M_3BYTES = 0x02,
    _RESERVED_3 = 0x03,
}
impl Prelen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prelen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prelen {
    #[inline(always)]
    fn from(val: u8) -> Prelen {
        Prelen::from_bits(val)
    }
}
impl From<Prelen> for u8 {
    #[inline(always)]
    fn from(val: Prelen) -> u8 {
        Prelen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "For 1000 or 2500 Mbps operations"]
    M_1000_2500M = 0x0,
    #[doc = "For 10 or 100 Mbps operations"]
    M_10_100M = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pse {
    #[doc = "Preamble Suppression disabled"]
    DISABLE = 0x0,
    #[doc = "Preamble Suppression enabled"]
    ENABLE = 0x01,
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
pub enum Ptoen {
    #[doc = "PTP Offload feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "PTP Offload feature is selected"]
    ACTIVE = 0x01,
}
impl Ptoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptoen {
    #[inline(always)]
    fn from(val: u8) -> Ptoen {
        Ptoen::from_bits(val)
    }
}
impl From<Ptoen> for u8 {
    #[inline(always)]
    fn from(val: Ptoen) -> u8 {
        Ptoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptpq {
    #[doc = "Receive Queue 0"]
    QUEUE0 = 0x0,
    #[doc = "Receive Queue 1"]
    QUEUE1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ptpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptpq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptpq {
    #[inline(always)]
    fn from(val: u8) -> Ptpq {
        Ptpq::from_bits(val)
    }
}
impl From<Ptpq> for u8 {
    #[inline(always)]
    fn from(val: Ptpq) -> u8 {
        Ptpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwe {
    #[doc = "Programmable Watchdog is disabled"]
    DISABLE = 0x0,
    #[doc = "Programmable Watchdog is enabled"]
    ENABLE = 0x01,
}
impl Pwe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwe {
    #[inline(always)]
    fn from(val: u8) -> Pwe {
        Pwe::from_bits(val)
    }
}
impl From<Pwe> for u8 {
    #[inline(always)]
    fn from(val: Pwe) -> u8 {
        Pwe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrdwn {
    #[doc = "Power down is disabled"]
    DISABLE = 0x0,
    #[doc = "Power down is enabled"]
    ENABLE = 0x01,
}
impl Pwrdwn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrdwn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrdwn {
    #[inline(always)]
    fn from(val: u8) -> Pwrdwn {
        Pwrdwn::from_bits(val)
    }
}
impl From<Pwrdwn> for u8 {
    #[inline(always)]
    fn from(val: Pwrdwn) -> u8 {
        Pwrdwn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Q0ddmach {
    #[doc = "Queue 0 disabled for DA-based DMA Channel Selection"]
    DISABLE = 0x0,
    #[doc = "Queue 0 enabled for DA-based DMA Channel Selection"]
    ENABLE = 0x01,
}
impl Q0ddmach {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Q0ddmach {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Q0ddmach {
    #[inline(always)]
    fn from(val: u8) -> Q0ddmach {
        Q0ddmach::from_bits(val)
    }
}
impl From<Q0ddmach> for u8 {
    #[inline(always)]
    fn from(val: Q0ddmach) -> u8 {
        Q0ddmach::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Q0is {
    #[doc = "Queue 0 Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Queue 0 Interrupt status detected"]
    ACTIVE = 0x01,
}
impl Q0is {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Q0is {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Q0is {
    #[inline(always)]
    fn from(val: u8) -> Q0is {
        Q0is::from_bits(val)
    }
}
impl From<Q0is> for u8 {
    #[inline(always)]
    fn from(val: Q0is) -> u8 {
        Q0is::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Q1ddmach {
    #[doc = "Queue 1 disabled for DA-based DMA Channel Selection"]
    DISABLE = 0x0,
    #[doc = "Queue 1 enabled for DA-based DMA Channel Selection"]
    ENABLE = 0x01,
}
impl Q1ddmach {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Q1ddmach {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Q1ddmach {
    #[inline(always)]
    fn from(val: u8) -> Q1ddmach {
        Q1ddmach::from_bits(val)
    }
}
impl From<Q1ddmach> for u8 {
    #[inline(always)]
    fn from(val: Q1ddmach) -> u8 {
        Q1ddmach::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Q1is {
    #[doc = "Queue 1 Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Queue 1 Interrupt status detected"]
    ACTIVE = 0x01,
}
impl Q1is {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Q1is {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Q1is {
    #[inline(always)]
    fn from(val: u8) -> Q1is {
        Q1is::from_bits(val)
    }
}
impl From<Q1is> for u8 {
    #[inline(always)]
    fn from(val: Q1is) -> u8 {
        Q1is::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ra {
    #[doc = "Receive All is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive All is enabled"]
    ENABLE = 0x01,
}
impl Ra {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ra {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ra {
    #[inline(always)]
    fn from(val: u8) -> Ra {
        Ra::from_bits(val)
    }
}
impl From<Ra> for u8 {
    #[inline(always)]
    fn from(val: Ra) -> u8 {
        Ra::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raa {
    #[doc = "Strict priority (SP)"]
    SP = 0x0,
    #[doc = "Weighted Strict Priority (WSP)"]
    WSP = 0x01,
}
impl Raa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raa {
    #[inline(always)]
    fn from(val: u8) -> Raa {
        Raa::from_bits(val)
    }
}
impl From<Raa> for u8 {
    #[inline(always)]
    fn from(val: Raa) -> u8 {
        Raa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ravsel {
    #[doc = "Rx Side Only AV Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Rx Side Only AV Feature is selected"]
    ACTIVE = 0x01,
}
impl Ravsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ravsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ravsel {
    #[inline(always)]
    fn from(val: u8) -> Ravsel {
        Ravsel::from_bits(val)
    }
}
impl From<Ravsel> for u8 {
    #[inline(always)]
    fn from(val: Ravsel) -> u8 {
        Ravsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rb {
    #[doc = "Rebuild INCRx Burst is disabled"]
    DISABLE = 0x0,
    #[doc = "Rebuild INCRx Burst is enabled"]
    ENABLE = 0x01,
}
impl Rb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rb {
    #[inline(always)]
    fn from(val: u8) -> Rb {
        Rb::from_bits(val)
    }
}
impl From<Rb> for u8 {
    #[inline(always)]
    fn from(val: Rb) -> u8 {
        Rb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcwe {
    #[doc = "Register Clear on Write 1 is disabled"]
    DISABLE = 0x0,
    #[doc = "Register Clear on Write 1 is enabled"]
    ENABLE = 0x01,
}
impl Rcwe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcwe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcwe {
    #[inline(always)]
    fn from(val: u8) -> Rcwe {
        Rcwe::from_bits(val)
    }
}
impl From<Rcwe> for u8 {
    #[inline(always)]
    fn from(val: Rcwe) -> u8 {
        Rcwe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdwr {
    #[doc = "Read operation of indirect access"]
    READ = 0x0,
    #[doc = "Write operation of indirect access"]
    WRITE = 0x01,
}
impl Rdwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdwr {
    #[inline(always)]
    fn from(val: u8) -> Rdwr {
        Rdwr::from_bits(val)
    }
}
impl From<Rdwr> for u8 {
    #[inline(always)]
    fn from(val: Rdwr) -> u8 {
        Rdwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Re {
    #[doc = "Receiver is disabled"]
    DISABLE = 0x0,
    #[doc = "Receiver is enabled"]
    ENABLE = 0x01,
}
impl Re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Re {
    #[inline(always)]
    fn from(val: u8) -> Re {
        Re::from_bits(val)
    }
}
impl From<Re> for u8 {
    #[inline(always)]
    fn from(val: Re) -> u8 {
        Re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfe {
    #[doc = "Receive Flow Control is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Flow Control is enabled"]
    ENABLE = 0x01,
}
impl Rfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfe {
    #[inline(always)]
    fn from(val: u8) -> Rfe {
        Rfe::from_bits(val)
    }
}
impl From<Rfe> for u8 {
    #[inline(always)]
    fn from(val: Rfe) -> u8 {
        Rfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rlpien {
    #[doc = "Receive LPI entry not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive LPI entry detected"]
    ACTIVE = 0x01,
}
impl Rlpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rlpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rlpien {
    #[inline(always)]
    fn from(val: u8) -> Rlpien {
        Rlpien::from_bits(val)
    }
}
impl From<Rlpien> for u8 {
    #[inline(always)]
    fn from(val: Rlpien) -> u8 {
        Rlpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rlpiex {
    #[doc = "Receive LPI exit not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive LPI exit detected"]
    ACTIVE = 0x01,
}
impl Rlpiex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rlpiex {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rlpiex {
    #[inline(always)]
    fn from(val: u8) -> Rlpiex {
        Rlpiex::from_bits(val)
    }
}
impl From<Rlpiex> for u8 {
    #[inline(always)]
    fn from(val: Rlpiex) -> u8 {
        Rlpiex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rlpist {
    #[doc = "Receive LPI state not detected"]
    INACTIVE = 0x0,
    #[doc = "Receive LPI state detected"]
    ACTIVE = 0x01,
}
impl Rlpist {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rlpist {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rlpist {
    #[inline(always)]
    fn from(val: u8) -> Rlpist {
        Rlpist::from_bits(val)
    }
}
impl From<Rlpist> for u8 {
    #[inline(always)]
    fn from(val: Rlpist) -> u8 {
        Rlpist::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rpests {
    #[doc = "MAC GMII or MII Receive Protocol Engine Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MAC GMII or MII Receive Protocol Engine Status detected"]
    ACTIVE = 0x01,
}
impl Rpests {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rpests {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rpests {
    #[inline(always)]
    fn from(val: u8) -> Rpests {
        Rpests::from_bits(val)
    }
}
impl From<Rpests> for u8 {
    #[inline(always)]
    fn from(val: Rpests) -> u8 {
        Rpests::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps0 {
    #[doc = "Stopped (Reset or Stop Receive Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)"]
    RUN_FRTD = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)"]
    RUN_WRP = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)"]
    SUSPND = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)"]
    RUN_CRD = 0x05,
    #[doc = "Timestamp write state"]
    TSTMP = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
    RUN_TRP = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps0 {
    #[inline(always)]
    fn from(val: u8) -> Rps0 {
        Rps0::from_bits(val)
    }
}
impl From<Rps0> for u8 {
    #[inline(always)]
    fn from(val: Rps0) -> u8 {
        Rps0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps1 {
    #[doc = "Stopped (Reset or Stop Receive Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)"]
    RUN_FRTD = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)"]
    RUN_WRP = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)"]
    SUSPND = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)"]
    RUN_CRD = 0x05,
    #[doc = "Timestamp write state"]
    TSTMP = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
    RUN_TRP = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rps1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps1 {
    #[inline(always)]
    fn from(val: u8) -> Rps1 {
        Rps1::from_bits(val)
    }
}
impl From<Rps1> for u8 {
    #[inline(always)]
    fn from(val: Rps1) -> u8 {
        Rps1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwkfiltrst {
    #[doc = "Remote Wake-Up Packet Filter Register Pointer is not Reset"]
    DISABLE = 0x0,
    #[doc = "Remote Wake-Up Packet Filter Register Pointer is Reset"]
    ENABLE = 0x01,
}
impl Rwkfiltrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwkfiltrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwkfiltrst {
    #[inline(always)]
    fn from(val: u8) -> Rwkfiltrst {
        Rwkfiltrst::from_bits(val)
    }
}
impl From<Rwkfiltrst> for u8 {
    #[inline(always)]
    fn from(val: Rwkfiltrst) -> u8 {
        Rwkfiltrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwkpfe {
    #[doc = "Remote Wake-up Packet Forwarding is disabled"]
    DISABLE = 0x0,
    #[doc = "Remote Wake-up Packet Forwarding is enabled"]
    ENABLE = 0x01,
}
impl Rwkpfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwkpfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwkpfe {
    #[inline(always)]
    fn from(val: u8) -> Rwkpfe {
        Rwkpfe::from_bits(val)
    }
}
impl From<Rwkpfe> for u8 {
    #[inline(always)]
    fn from(val: Rwkpfe) -> u8 {
        Rwkpfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwkpkten {
    #[doc = "Remote wake-up packet is disabled"]
    DISABLE = 0x0,
    #[doc = "Remote wake-up packet is enabled"]
    ENABLE = 0x01,
}
impl Rwkpkten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwkpkten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwkpkten {
    #[inline(always)]
    fn from(val: u8) -> Rwkpkten {
        Rwkpkten::from_bits(val)
    }
}
impl From<Rwkpkten> for u8 {
    #[inline(always)]
    fn from(val: Rwkpkten) -> u8 {
        Rwkpkten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwkprcvd {
    #[doc = "Remote wake-up packet is received"]
    INACTIVE = 0x0,
    #[doc = "Remote wake-up packet is received"]
    ACTIVE = 0x01,
}
impl Rwkprcvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwkprcvd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwkprcvd {
    #[inline(always)]
    fn from(val: u8) -> Rwkprcvd {
        Rwkprcvd::from_bits(val)
    }
}
impl From<Rwkprcvd> for u8 {
    #[inline(always)]
    fn from(val: Rwkprcvd) -> u8 {
        Rwkprcvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwksel {
    #[doc = "PMT Remote Wake-up Packet Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "PMT Remote Wake-up Packet Enable option is selected"]
    ACTIVE = 0x01,
}
impl Rwksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwksel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwksel {
    #[inline(always)]
    fn from(val: u8) -> Rwksel {
        Rwksel::from_bits(val)
    }
}
impl From<Rwksel> for u8 {
    #[inline(always)]
    fn from(val: Rwksel) -> u8 {
        Rwksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxchcnt {
    #[doc = "1 MTL Rx Channel"]
    M_1RXCH = 0x0,
    #[doc = "2 MTL Rx Channels"]
    M_2RXCH = 0x01,
    #[doc = "3 MTL Rx Channels"]
    M_3RXCH = 0x02,
    #[doc = "4 MTL Rx Channels"]
    M_4RXCH = 0x03,
    #[doc = "5 MTL Rx Channels"]
    M_5RXCH = 0x04,
    #[doc = "6 MTL Rx Channels"]
    M_6RXCH = 0x05,
    #[doc = "7 MTL Rx Channels"]
    M_7RXCH = 0x06,
    #[doc = "8 MTL Rx Channels"]
    M_8RXCH = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxchcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxchcnt {
        Rxchcnt::from_bits(val)
    }
}
impl From<Rxchcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxchcnt) -> u8 {
        Rxchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxcoesel {
    #[doc = "Receive Checksum Offload Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "Receive Checksum Offload Enable option is selected"]
    ACTIVE = 0x01,
}
impl Rxcoesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxcoesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxcoesel {
    #[inline(always)]
    fn from(val: u8) -> Rxcoesel {
        Rxcoesel::from_bits(val)
    }
}
impl From<Rxcoesel> for u8 {
    #[inline(always)]
    fn from(val: Rxcoesel) -> u8 {
        Rxcoesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "128 bytes"]
    M_128B = 0x0,
    #[doc = "256 bytes"]
    M_256B = 0x01,
    #[doc = "512 bytes"]
    M_512B = 0x02,
    #[doc = "1024 bytes"]
    M_1024B = 0x03,
    #[doc = "2048 bytes"]
    M_2048B = 0x04,
    #[doc = "4096 bytes"]
    M_4096B = 0x05,
    #[doc = "8192 bytes"]
    M_8192B = 0x06,
    #[doc = "16384 bytes"]
    M_16384B = 0x07,
    #[doc = "32 KB"]
    M_32KB = 0x08,
    #[doc = "64 KB"]
    M_64KB = 0x09,
    #[doc = "128 KB"]
    M_128KB = 0x0a,
    #[doc = "256 KB"]
    M_256KB = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxq0en {
    #[doc = "Queue not enabled"]
    DISABLE = 0x0,
    #[doc = "Queue enabled for AV"]
    EN_AV = 0x01,
    #[doc = "Queue enabled for DCB/Generic"]
    EN_DCB_GEN = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rxq0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxq0en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxq0en {
    #[inline(always)]
    fn from(val: u8) -> Rxq0en {
        Rxq0en::from_bits(val)
    }
}
impl From<Rxq0en> for u8 {
    #[inline(always)]
    fn from(val: Rxq0en) -> u8 {
        Rxq0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxq1en {
    #[doc = "Queue not enabled"]
    DISABLE = 0x0,
    #[doc = "Queue enabled for AV"]
    EN_AV = 0x01,
    #[doc = "Queue enabled for DCB/Generic"]
    EN_DCB_GEN = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rxq1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxq1en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxq1en {
    #[inline(always)]
    fn from(val: u8) -> Rxq1en {
        Rxq1en::from_bits(val)
    }
}
impl From<Rxq1en> for u8 {
    #[inline(always)]
    fn from(val: Rxq1en) -> u8 {
        Rxq1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxqcnt {
    #[doc = "1 MTL Rx Queue"]
    M_1RXQ = 0x0,
    #[doc = "2 MTL Rx Queues"]
    M_2RXQ = 0x01,
    #[doc = "3 MTL Rx Queues"]
    M_3RXQ = 0x02,
    #[doc = "4 MTL Rx Queues"]
    M_4RXQ = 0x03,
    #[doc = "5 MTL Rx Queues"]
    M_5RXQ = 0x04,
    #[doc = "6 MTL Rx Queues"]
    M_6RXQ = 0x05,
    #[doc = "7 MTL Rx Queues"]
    M_7RXQ = 0x06,
    #[doc = "8 MTL Rx Queues"]
    M_8RXQ = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxqcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxqcnt {
        Rxqcnt::from_bits(val)
    }
}
impl From<Rxqcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxqcnt) -> u8 {
        Rxqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxstsie {
    #[doc = "Receive Status Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Receive Status Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Rxstsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxstsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxstsie {
    #[inline(always)]
    fn from(val: u8) -> Rxstsie {
        Rxstsie::from_bits(val)
    }
}
impl From<Rxstsie> for u8 {
    #[inline(always)]
    fn from(val: Rxstsie) -> u8 {
        Rxstsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxstsis {
    #[doc = "Receive Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "Receive Interrupt status active"]
    ACTIVE = 0x01,
}
impl Rxstsis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxstsis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxstsis {
    #[inline(always)]
    fn from(val: u8) -> Rxstsis {
        Rxstsis::from_bits(val)
    }
}
impl From<Rxstsis> for u8 {
    #[inline(always)]
    fn from(val: Rxstsis) -> u8 {
        Rxstsis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S2kp {
    #[doc = "Support upto 2K packet is disabled"]
    DISABLE = 0x0,
    #[doc = "Support upto 2K packet is Enabled"]
    ENABLE = 0x01,
}
impl S2kp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S2kp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S2kp {
    #[inline(always)]
    fn from(val: u8) -> S2kp {
        S2kp::from_bits(val)
    }
}
impl From<S2kp> for u8 {
    #[inline(always)]
    fn from(val: S2kp) -> u8 {
        S2kp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarc {
    #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation"]
    SA_CTRL_IN = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Contents of MAC Addr-0 inserted in SA field"]
    MAC0_INS_SA = 0x02,
    #[doc = "Contents of MAC Addr-0 replaces SA field"]
    MAC0_REP_SA = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Contents of MAC Addr-1 inserted in SA field"]
    MAC1_INS_SA = 0x06,
    #[doc = "Contents of MAC Addr-1 replaces SA field"]
    MAC1_REP_SA = 0x07,
}
impl Sarc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarc {
    #[inline(always)]
    fn from(val: u8) -> Sarc {
        Sarc::from_bits(val)
    }
}
impl From<Sarc> for u8 {
    #[inline(always)]
    fn from(val: Sarc) -> u8 {
        Sarc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Savlanins {
    #[doc = "Source Address or VLAN Insertion Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "Source Address or VLAN Insertion Enable option is selected"]
    ACTIVE = 0x01,
}
impl Savlanins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Savlanins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Savlanins {
    #[inline(always)]
    fn from(val: u8) -> Savlanins {
        Savlanins::from_bits(val)
    }
}
impl From<Savlanins> for u8 {
    #[inline(always)]
    fn from(val: Savlanins) -> u8 {
        Savlanins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Schalg {
    #[doc = "WRR algorithm"]
    WRR = 0x0,
    #[doc = "WFQ algorithm when DCB feature is selected.Otherwise, Reserved"]
    WFQ = 0x01,
    #[doc = "DWRR algorithm when DCB feature is selected.Otherwise, Reserved"]
    DWRR = 0x02,
    #[doc = "Strict priority algorithm"]
    SP = 0x03,
}
impl Schalg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Schalg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Schalg {
    #[inline(always)]
    fn from(val: u8) -> Schalg {
        Schalg::from_bits(val)
    }
}
impl From<Schalg> for u8 {
    #[inline(always)]
    fn from(val: Schalg) -> u8 {
        Schalg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skap {
    #[doc = "Skip Address Packet is disabled"]
    DISABLE = 0x0,
    #[doc = "Skip Address Packet is enabled"]
    ENABLE = 0x01,
}
impl Skap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skap {
    #[inline(always)]
    fn from(val: u8) -> Skap {
        Skap::from_bits(val)
    }
}
impl From<Skap> for u8 {
    #[inline(always)]
    fn from(val: Skap) -> u8 {
        Skap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slc {
    #[doc = "1 slot"]
    M_1_SLOT = 0x0,
    #[doc = "2 slots"]
    M_2_SLOT = 0x01,
    #[doc = "4 slots"]
    M_4_SLOT = 0x02,
    #[doc = "8 slots"]
    M_8_SLOT = 0x03,
    #[doc = "16 slots"]
    M_16_SLOT = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Slc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slc {
    #[inline(always)]
    fn from(val: u8) -> Slc {
        Slc::from_bits(val)
    }
}
impl From<Slc> for u8 {
    #[inline(always)]
    fn from(val: Slc) -> u8 {
        Slc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smasel {
    #[doc = "SMA (MDIO) Interface not selected"]
    INACTIVE = 0x0,
    #[doc = "SMA (MDIO) Interface selected"]
    ACTIVE = 0x01,
}
impl Smasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smasel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smasel {
    #[inline(always)]
    fn from(val: u8) -> Smasel {
        Smasel::from_bits(val)
    }
}
impl From<Smasel> for u8 {
    #[inline(always)]
    fn from(val: Smasel) -> u8 {
        Smasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spen {
    #[doc = "Slow Protocol Detection is disabled"]
    DISABLE = 0x0,
    #[doc = "Slow Protocol Detection is enabled"]
    ENABLE = 0x01,
}
impl Spen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spen {
    #[inline(always)]
    fn from(val: u8) -> Spen {
        Spen::from_bits(val)
    }
}
impl From<Spen> for u8 {
    #[inline(always)]
    fn from(val: Spen) -> u8 {
        Spen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sphen {
    #[doc = "Split Header Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Split Header Feature is selected"]
    ACTIVE = 0x01,
}
impl Sphen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sphen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sphen {
    #[inline(always)]
    fn from(val: u8) -> Sphen {
        Sphen::from_bits(val)
    }
}
impl From<Sphen> for u8 {
    #[inline(always)]
    fn from(val: Sphen) -> u8 {
        Sphen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spram {
    #[doc = "Single Port RAM feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Single Port RAM feature is selected"]
    ACTIVE = 0x01,
}
impl Spram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spram {
    #[inline(always)]
    fn from(val: u8) -> Spram {
        Spram::from_bits(val)
    }
}
impl From<Spram> for u8 {
    #[inline(always)]
    fn from(val: Spram) -> u8 {
        Spram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "Software Reset is disabled"]
    DISABLE = 0x0,
    #[doc = "Software Reset is enabled"]
    ENABLE = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taa {
    #[doc = "Fixed priority"]
    FP = 0x0,
    #[doc = "Weighted Strict Priority (WSP)"]
    WSP = 0x01,
    #[doc = "Weighted Round-Robin (WRR)"]
    WRR = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Taa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taa {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taa {
    #[inline(always)]
    fn from(val: u8) -> Taa {
        Taa::from_bits(val)
    }
}
impl From<Taa> for u8 {
    #[inline(always)]
    fn from(val: Taa) -> u8 {
        Taa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tacpqe {
    #[doc = "Tagged AV Control Packets Queuing is disabled"]
    DISABLE = 0x0,
    #[doc = "Tagged AV Control Packets Queuing is enabled"]
    ENABLE = 0x01,
}
impl Tacpqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tacpqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tacpqe {
    #[inline(always)]
    fn from(val: u8) -> Tacpqe {
        Tacpqe::from_bits(val)
    }
}
impl From<Tacpqe> for u8 {
    #[inline(always)]
    fn from(val: Tacpqe) -> u8 {
        Tacpqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tbssel {
    #[doc = "Time Based Scheduling Enable feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "Time Based Scheduling Enable feature is selected"]
    ACTIVE = 0x01,
}
impl Tbssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tbssel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tbssel {
    #[inline(always)]
    fn from(val: u8) -> Tbssel {
        Tbssel::from_bits(val)
    }
}
impl From<Tbssel> for u8 {
    #[inline(always)]
    fn from(val: Tbssel) -> u8 {
        Tbssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Te {
    #[doc = "Transmitter is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmitter is enabled"]
    ENABLE = 0x01,
}
impl Te {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Te {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Te {
    #[inline(always)]
    fn from(val: u8) -> Te {
        Te::from_bits(val)
    }
}
impl From<Te> for u8 {
    #[inline(always)]
    fn from(val: Te) -> u8 {
        Te::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfcsts {
    #[doc = "Idle state"]
    IDLE = 0x0,
    #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over"]
    WAITING = 0x01,
    #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)"]
    GEN_TX_PAU = 0x02,
    #[doc = "Transferring input packet for transmission"]
    TRNSFR = 0x03,
}
impl Tfcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfcsts {
    #[inline(always)]
    fn from(val: u8) -> Tfcsts {
        Tfcsts::from_bits(val)
    }
}
impl From<Tfcsts> for u8 {
    #[inline(always)]
    fn from(val: Tfcsts) -> u8 {
        Tfcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfe {
    #[doc = "Transmit Flow Control is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Flow Control is enabled"]
    ENABLE = 0x01,
}
impl Tfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfe {
    #[inline(always)]
    fn from(val: u8) -> Tfe {
        Tfe::from_bits(val)
    }
}
impl From<Tfe> for u8 {
    #[inline(always)]
    fn from(val: Tfe) -> u8 {
        Tfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tjt {
    #[doc = "No Transmit Jabber Timeout"]
    INACTIVE = 0x0,
    #[doc = "Transmit Jabber Timeout occurred"]
    ACTIVE = 0x01,
}
impl Tjt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tjt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tjt {
    #[inline(always)]
    fn from(val: u8) -> Tjt {
        Tjt::from_bits(val)
    }
}
impl From<Tjt> for u8 {
    #[inline(always)]
    fn from(val: Tjt) -> u8 {
        Tjt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tlpien {
    #[doc = "Transmit LPI entry not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit LPI entry detected"]
    ACTIVE = 0x01,
}
impl Tlpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tlpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tlpien {
    #[inline(always)]
    fn from(val: u8) -> Tlpien {
        Tlpien::from_bits(val)
    }
}
impl From<Tlpien> for u8 {
    #[inline(always)]
    fn from(val: Tlpien) -> u8 {
        Tlpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tlpiex {
    #[doc = "Transmit LPI exit not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit LPI exit detected"]
    ACTIVE = 0x01,
}
impl Tlpiex {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tlpiex {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tlpiex {
    #[inline(always)]
    fn from(val: u8) -> Tlpiex {
        Tlpiex::from_bits(val)
    }
}
impl From<Tlpiex> for u8 {
    #[inline(always)]
    fn from(val: Tlpiex) -> u8 {
        Tlpiex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tlpist {
    #[doc = "Transmit LPI state not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit LPI state detected"]
    ACTIVE = 0x01,
}
impl Tlpist {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tlpist {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tlpist {
    #[inline(always)]
    fn from(val: u8) -> Tlpist {
        Tlpist::from_bits(val)
    }
}
impl From<Tlpist> for u8 {
    #[inline(always)]
    fn from(val: Tlpist) -> u8 {
        Tlpist::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpests {
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status not detected"]
    INACTIVE = 0x0,
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status detected"]
    ACTIVE = 0x01,
}
impl Tpests {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpests {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpests {
    #[inline(always)]
    fn from(val: u8) -> Tpests {
        Tpests::from_bits(val)
    }
}
impl From<Tpests> for u8 {
    #[inline(always)]
    fn from(val: Tpests) -> u8 {
        Tpests::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps0 {
    #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)"]
    RUN_FTTD = 0x01,
    #[doc = "Running (Waiting for status)"]
    RUN_WS = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
    RUN_RDS = 0x03,
    #[doc = "Timestamp write state"]
    TSTMP_WS = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
    SUSPND = 0x06,
    #[doc = "Running (Closing Tx Descriptor)"]
    RUN_CTD = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps0 {
    #[inline(always)]
    fn from(val: u8) -> Tps0 {
        Tps0::from_bits(val)
    }
}
impl From<Tps0> for u8 {
    #[inline(always)]
    fn from(val: Tps0) -> u8 {
        Tps0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps1 {
    #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
    STOP = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)"]
    RUN_FTTD = 0x01,
    #[doc = "Running (Waiting for status)"]
    RUN_WS = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
    RUN_RDS = 0x03,
    #[doc = "Timestamp write state"]
    TSTMP_WS = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
    SUSPND = 0x06,
    #[doc = "Running (Closing Tx Descriptor)"]
    RUN_CTD = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tps1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps1 {
    #[inline(always)]
    fn from(val: u8) -> Tps1 {
        Tps1::from_bits(val)
    }
}
impl From<Tps1> for u8 {
    #[inline(always)]
    fn from(val: Tps1) -> u8 {
        Tps1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsaddreg {
    #[doc = "Addend Register is not updated"]
    DISABLE = 0x0,
    #[doc = "Addend Register is updated"]
    ENABLE = 0x01,
}
impl Tsaddreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsaddreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsaddreg {
    #[inline(always)]
    fn from(val: u8) -> Tsaddreg {
        Tsaddreg::from_bits(val)
    }
}
impl From<Tsaddreg> for u8 {
    #[inline(always)]
    fn from(val: Tsaddreg) -> u8 {
        Tsaddreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tscfupdt {
    #[doc = "Coarse method is used to update system timestamp"]
    COARSE = 0x0,
    #[doc = "Fine method is used to update system timestamp"]
    FINE = 0x01,
}
impl Tscfupdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscfupdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscfupdt {
    #[inline(always)]
    fn from(val: u8) -> Tscfupdt {
        Tscfupdt::from_bits(val)
    }
}
impl From<Tscfupdt> for u8 {
    #[inline(always)]
    fn from(val: Tscfupdt) -> u8 {
        Tscfupdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsctrlssr {
    #[doc = "Timestamp Digital Rollover Control is disabled and Binary Rollover Control is enabled"]
    DIG_DISABLE = 0x0,
    #[doc = "Timestamp Digital Rollover Control is enabled and Binary Rollover Control is disabled"]
    DIG_ENABLE = 0x01,
}
impl Tsctrlssr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsctrlssr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsctrlssr {
    #[inline(always)]
    fn from(val: u8) -> Tsctrlssr {
        Tsctrlssr::from_bits(val)
    }
}
impl From<Tsctrlssr> for u8 {
    #[inline(always)]
    fn from(val: Tsctrlssr) -> u8 {
        Tsctrlssr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsena {
    #[doc = "Timestamp is disabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp is enabled"]
    ENABLE = 0x01,
}
impl Tsena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsena {
    #[inline(always)]
    fn from(val: u8) -> Tsena {
        Tsena::from_bits(val)
    }
}
impl From<Tsena> for u8 {
    #[inline(always)]
    fn from(val: Tsena) -> u8 {
        Tsena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsenall {
    #[doc = "Timestamp for All Packets disabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp for All Packets enabled"]
    ENABLE = 0x01,
}
impl Tsenall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsenall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsenall {
    #[inline(always)]
    fn from(val: u8) -> Tsenall {
        Tsenall::from_bits(val)
    }
}
impl From<Tsenall> for u8 {
    #[inline(always)]
    fn from(val: Tsenall) -> u8 {
        Tsenall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsenmacaddr {
    #[doc = "MAC Address for PTP Packet Filtering is disabled"]
    DISABLE = 0x0,
    #[doc = "MAC Address for PTP Packet Filtering is enabled"]
    ENABLE = 0x01,
}
impl Tsenmacaddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsenmacaddr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsenmacaddr {
    #[inline(always)]
    fn from(val: u8) -> Tsenmacaddr {
        Tsenmacaddr::from_bits(val)
    }
}
impl From<Tsenmacaddr> for u8 {
    #[inline(always)]
    fn from(val: Tsenmacaddr) -> u8 {
        Tsenmacaddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsevntena {
    #[doc = "Timestamp Snapshot for Event Messages is disabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp Snapshot for Event Messages is enabled"]
    ENABLE = 0x01,
}
impl Tsevntena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsevntena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsevntena {
    #[inline(always)]
    fn from(val: u8) -> Tsevntena {
        Tsevntena::from_bits(val)
    }
}
impl From<Tsevntena> for u8 {
    #[inline(always)]
    fn from(val: Tsevntena) -> u8 {
        Tsevntena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsie {
    #[doc = "Timestamp Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Tsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsie {
    #[inline(always)]
    fn from(val: u8) -> Tsie {
        Tsie::from_bits(val)
    }
}
impl From<Tsie> for u8 {
    #[inline(always)]
    fn from(val: Tsie) -> u8 {
        Tsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsinit {
    #[doc = "Timestamp is not initialized"]
    DISABLE = 0x0,
    #[doc = "Timestamp is initialized"]
    ENABLE = 0x01,
}
impl Tsinit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsinit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsinit {
    #[inline(always)]
    fn from(val: u8) -> Tsinit {
        Tsinit::from_bits(val)
    }
}
impl From<Tsinit> for u8 {
    #[inline(always)]
    fn from(val: Tsinit) -> u8 {
        Tsinit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsipena {
    #[doc = "Processing of PTP over Ethernet Packets is disabled"]
    DISABLE = 0x0,
    #[doc = "Processing of PTP over Ethernet Packets is enabled"]
    ENABLE = 0x01,
}
impl Tsipena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsipena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsipena {
    #[inline(always)]
    fn from(val: u8) -> Tsipena {
        Tsipena::from_bits(val)
    }
}
impl From<Tsipena> for u8 {
    #[inline(always)]
    fn from(val: Tsipena) -> u8 {
        Tsipena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsipv4ena {
    #[doc = "Processing of PTP Packets Sent over IPv4-UDP is disabled"]
    DISABLE = 0x0,
    #[doc = "Processing of PTP Packets Sent over IPv4-UDP is enabled"]
    ENABLE = 0x01,
}
impl Tsipv4ena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsipv4ena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsipv4ena {
    #[inline(always)]
    fn from(val: u8) -> Tsipv4ena {
        Tsipv4ena::from_bits(val)
    }
}
impl From<Tsipv4ena> for u8 {
    #[inline(always)]
    fn from(val: Tsipv4ena) -> u8 {
        Tsipv4ena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsipv6ena {
    #[doc = "Processing of PTP Packets Sent over IPv6-UDP is disabled"]
    DISABLE = 0x0,
    #[doc = "Processing of PTP Packets Sent over IPv6-UDP is enabled"]
    ENABLE = 0x01,
}
impl Tsipv6ena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsipv6ena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsipv6ena {
    #[inline(always)]
    fn from(val: u8) -> Tsipv6ena {
        Tsipv6ena::from_bits(val)
    }
}
impl From<Tsipv6ena> for u8 {
    #[inline(always)]
    fn from(val: Tsipv6ena) -> u8 {
        Tsipv6ena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsis {
    #[doc = "Timestamp Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "Timestamp Interrupt status active"]
    ACTIVE = 0x01,
}
impl Tsis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsis {
    #[inline(always)]
    fn from(val: u8) -> Tsis {
        Tsis::from_bits(val)
    }
}
impl From<Tsis> for u8 {
    #[inline(always)]
    fn from(val: Tsis) -> u8 {
        Tsis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsmstrena {
    #[doc = "Snapshot for Messages Relevant to Master is disabled"]
    DISABLE = 0x0,
    #[doc = "Snapshot for Messages Relevant to Master is enabled"]
    ENABLE = 0x01,
}
impl Tsmstrena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsmstrena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsmstrena {
    #[inline(always)]
    fn from(val: u8) -> Tsmstrena {
        Tsmstrena::from_bits(val)
    }
}
impl From<Tsmstrena> for u8 {
    #[inline(always)]
    fn from(val: Tsmstrena) -> u8 {
        Tsmstrena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsoen {
    #[doc = "TCP Segmentation Offload Feature is not selected"]
    INACTIVE = 0x0,
    #[doc = "TCP Segmentation Offload Feature is selected"]
    ACTIVE = 0x01,
}
impl Tsoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsoen {
    #[inline(always)]
    fn from(val: u8) -> Tsoen {
        Tsoen::from_bits(val)
    }
}
impl From<Tsoen> for u8 {
    #[inline(always)]
    fn from(val: Tsoen) -> u8 {
        Tsoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tssel {
    #[doc = "IEEE 1588-2008 Timestamp Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "IEEE 1588-2008 Timestamp Enable option is selected"]
    ACTIVE = 0x01,
}
impl Tssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tssel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tssel {
    #[inline(always)]
    fn from(val: u8) -> Tssel {
        Tssel::from_bits(val)
    }
}
impl From<Tssel> for u8 {
    #[inline(always)]
    fn from(val: Tssel) -> u8 {
        Tssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tssovf {
    #[doc = "Timestamp Seconds Overflow status not detected"]
    INACTIVE = 0x0,
    #[doc = "Timestamp Seconds Overflow status detected"]
    ACTIVE = 0x01,
}
impl Tssovf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tssovf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tssovf {
    #[inline(always)]
    fn from(val: u8) -> Tssovf {
        Tssovf::from_bits(val)
    }
}
impl From<Tssovf> for u8 {
    #[inline(always)]
    fn from(val: Tssovf) -> u8 {
        Tssovf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsstssel {
    #[doc = "Internal"]
    INTRNL = 0x0,
    #[doc = "External"]
    EXTRNL = 0x01,
    #[doc = "Both"]
    BOTH = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tsstssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsstssel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsstssel {
    #[inline(always)]
    fn from(val: u8) -> Tsstssel {
        Tsstssel::from_bits(val)
    }
}
impl From<Tsstssel> for u8 {
    #[inline(always)]
    fn from(val: Tsstssel) -> u8 {
        Tsstssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstargt0 {
    #[doc = "Timestamp Target Time Reached status not detected"]
    INACTIVE = 0x0,
    #[doc = "Timestamp Target Time Reached status detected"]
    ACTIVE = 0x01,
}
impl Tstargt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstargt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstargt0 {
    #[inline(always)]
    fn from(val: u8) -> Tstargt0 {
        Tstargt0::from_bits(val)
    }
}
impl From<Tstargt0> for u8 {
    #[inline(always)]
    fn from(val: Tstargt0) -> u8 {
        Tstargt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstrgterr0 {
    #[doc = "Timestamp Target Time Error status not detected"]
    INACTIVE = 0x0,
    #[doc = "Timestamp Target Time Error status detected"]
    ACTIVE = 0x01,
}
impl Tstrgterr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstrgterr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstrgterr0 {
    #[inline(always)]
    fn from(val: u8) -> Tstrgterr0 {
        Tstrgterr0::from_bits(val)
    }
}
impl From<Tstrgterr0> for u8 {
    #[inline(always)]
    fn from(val: Tstrgterr0) -> u8 {
        Tstrgterr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstrig {
    #[doc = "Timestamp Interrupt Trigger is not enabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp Interrupt Trigger is enabled"]
    ENABLE = 0x01,
}
impl Tstrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstrig {
    #[inline(always)]
    fn from(val: u8) -> Tstrig {
        Tstrig::from_bits(val)
    }
}
impl From<Tstrig> for u8 {
    #[inline(always)]
    fn from(val: Tstrig) -> u8 {
        Tstrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsupdt {
    #[doc = "Timestamp is not updated"]
    DISABLE = 0x0,
    #[doc = "Timestamp is updated"]
    ENABLE = 0x01,
}
impl Tsupdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsupdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsupdt {
    #[inline(always)]
    fn from(val: u8) -> Tsupdt {
        Tsupdt::from_bits(val)
    }
}
impl From<Tsupdt> for u8 {
    #[inline(always)]
    fn from(val: Tsupdt) -> u8 {
        Tsupdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsver2ena {
    #[doc = "PTP Packet Processing for Version 2 Format is disabled"]
    DISABLE = 0x0,
    #[doc = "PTP Packet Processing for Version 2 Format is enabled"]
    ENABLE = 0x01,
}
impl Tsver2ena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsver2ena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsver2ena {
    #[inline(always)]
    fn from(val: u8) -> Tsver2ena {
        Tsver2ena::from_bits(val)
    }
}
impl From<Tsver2ena> for u8 {
    #[inline(always)]
    fn from(val: Tsver2ena) -> u8 {
        Tsver2ena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txchcnt {
    #[doc = "1 MTL Tx Channel"]
    M_1TXCH = 0x0,
    #[doc = "2 MTL Tx Channels"]
    M_2TXCH = 0x01,
    #[doc = "3 MTL Tx Channels"]
    M_3TXCH = 0x02,
    #[doc = "4 MTL Tx Channels"]
    M_4TXCH = 0x03,
    #[doc = "5 MTL Tx Channels"]
    M_5TXCH = 0x04,
    #[doc = "6 MTL Tx Channels"]
    M_6TXCH = 0x05,
    #[doc = "7 MTL Tx Channels"]
    M_7TXCH = 0x06,
    #[doc = "8 MTL Tx Channels"]
    M_8TXCH = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txchcnt {
    #[inline(always)]
    fn from(val: u8) -> Txchcnt {
        Txchcnt::from_bits(val)
    }
}
impl From<Txchcnt> for u8 {
    #[inline(always)]
    fn from(val: Txchcnt) -> u8 {
        Txchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txcoesel {
    #[doc = "Transmit Checksum Offload Enable option is not selected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Checksum Offload Enable option is selected"]
    ACTIVE = 0x01,
}
impl Txcoesel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txcoesel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txcoesel {
    #[inline(always)]
    fn from(val: u8) -> Txcoesel {
        Txcoesel::from_bits(val)
    }
}
impl From<Txcoesel> for u8 {
    #[inline(always)]
    fn from(val: Txcoesel) -> u8 {
        Txcoesel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "128 bytes"]
    M_128B = 0x0,
    #[doc = "256 bytes"]
    M_256B = 0x01,
    #[doc = "512 bytes"]
    M_512B = 0x02,
    #[doc = "1024 bytes"]
    M_1024B = 0x03,
    #[doc = "2048 bytes"]
    M_2048B = 0x04,
    #[doc = "4096 bytes"]
    M_4096B = 0x05,
    #[doc = "8192 bytes"]
    M_8192B = 0x06,
    #[doc = "16384 bytes"]
    M_16384B = 0x07,
    #[doc = "32 KB"]
    M_32KB = 0x08,
    #[doc = "64 KB"]
    M_64KB = 0x09,
    #[doc = "128 KB"]
    M_128KB = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpr {
    #[doc = "Transmit Priority is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Priority is enabled"]
    ENABLE = 0x01,
}
impl Txpr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpr {
    #[inline(always)]
    fn from(val: u8) -> Txpr {
        Txpr::from_bits(val)
    }
}
impl From<Txpr> for u8 {
    #[inline(always)]
    fn from(val: Txpr) -> u8 {
        Txpr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txqcnt {
    #[doc = "1 MTL Tx Queue"]
    M_1TXQ = 0x0,
    #[doc = "2 MTL Tx Queues"]
    M_2TXQ = 0x01,
    #[doc = "3 MTL Tx Queues"]
    M_3TXQ = 0x02,
    #[doc = "4 MTL Tx Queues"]
    M_4TXQ = 0x03,
    #[doc = "5 MTL Tx Queues"]
    M_5TXQ = 0x04,
    #[doc = "6 MTL Tx Queues"]
    M_6TXQ = 0x05,
    #[doc = "7 MTL Tx Queues"]
    M_7TXQ = 0x06,
    #[doc = "8 MTL Tx Queues"]
    M_8TXQ = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txqcnt {
    #[inline(always)]
    fn from(val: u8) -> Txqcnt {
        Txqcnt::from_bits(val)
    }
}
impl From<Txqcnt> for u8 {
    #[inline(always)]
    fn from(val: Txqcnt) -> u8 {
        Txqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txstsie {
    #[doc = "Timestamp Status Interrupt is disabled"]
    DISABLE = 0x0,
    #[doc = "Timestamp Status Interrupt is enabled"]
    ENABLE = 0x01,
}
impl Txstsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txstsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txstsie {
    #[inline(always)]
    fn from(val: u8) -> Txstsie {
        Txstsie::from_bits(val)
    }
}
impl From<Txstsie> for u8 {
    #[inline(always)]
    fn from(val: Txstsie) -> u8 {
        Txstsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txstsis {
    #[doc = "Transmit Interrupt status not active"]
    INACTIVE = 0x0,
    #[doc = "Transmit Interrupt status active"]
    ACTIVE = 0x01,
}
impl Txstsis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txstsis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txstsis {
    #[inline(always)]
    fn from(val: u8) -> Txstsis {
        Txstsis::from_bits(val)
    }
}
impl From<Txstsis> for u8 {
    #[inline(always)]
    fn from(val: Txstsis) -> u8 {
        Txstsis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txtssis {
    #[doc = "Tx Timestamp Status Interrupt status not detected"]
    INACTIVE = 0x0,
    #[doc = "Tx Timestamp Status Interrupt status detected"]
    ACTIVE = 0x01,
}
impl Txtssis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txtssis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txtssis {
    #[inline(always)]
    fn from(val: u8) -> Txtssis {
        Txtssis::from_bits(val)
    }
}
impl From<Txtssis> for u8 {
    #[inline(always)]
    fn from(val: Txtssis) -> u8 {
        Txtssis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txtssmis {
    #[doc = "Transmit Timestamp Status Missed status not detected"]
    INACTIVE = 0x0,
    #[doc = "Transmit Timestamp Status Missed status detected"]
    ACTIVE = 0x01,
}
impl Txtssmis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txtssmis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txtssmis {
    #[inline(always)]
    fn from(val: u8) -> Txtssmis {
        Txtssmis::from_bits(val)
    }
}
impl From<Txtssmis> for u8 {
    #[inline(always)]
    fn from(val: Txtssmis) -> u8 {
        Txtssmis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txtsstsm {
    #[doc = "Transmit Timestamp Status Mode is disabled"]
    DISABLE = 0x0,
    #[doc = "Transmit Timestamp Status Mode is enabled"]
    ENABLE = 0x01,
}
impl Txtsstsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txtsstsm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txtsstsm {
    #[inline(always)]
    fn from(val: u8) -> Txtsstsm {
        Txtsstsm::from_bits(val)
    }
}
impl From<Txtsstsm> for u8 {
    #[inline(always)]
    fn from(val: Txtsstsm) -> u8 {
        Txtsstsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uffqe {
    #[doc = "Unicast Address Filter Fail Packets Queuing is disabled"]
    DISABLE = 0x0,
    #[doc = "Unicast Address Filter Fail Packets Queuing is enabled"]
    ENABLE = 0x01,
}
impl Uffqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uffqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uffqe {
    #[inline(always)]
    fn from(val: u8) -> Uffqe {
        Uffqe::from_bits(val)
    }
}
impl From<Uffqe> for u8 {
    #[inline(always)]
    fn from(val: Uffqe) -> u8 {
        Uffqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Up {
    #[doc = "Unicast Pause Packet Detect disabled"]
    DISABLE = 0x0,
    #[doc = "Unicast Pause Packet Detect enabled"]
    ENABLE = 0x01,
}
impl Up {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Up {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Up {
    #[inline(always)]
    fn from(val: u8) -> Up {
        Up::from_bits(val)
    }
}
impl From<Up> for u8 {
    #[inline(always)]
    fn from(val: Up) -> u8 {
        Up::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upq {
    #[doc = "Receive Queue 0"]
    QUEUE0 = 0x0,
    #[doc = "Receive Queue 1"]
    QUEUE1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Upq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upq {
    #[inline(always)]
    fn from(val: u8) -> Upq {
        Upq::from_bits(val)
    }
}
impl From<Upq> for u8 {
    #[inline(always)]
    fn from(val: Upq) -> u8 {
        Upq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usp {
    #[doc = "Unicast Slow Protocol Packet Detection is disabled"]
    DISABLE = 0x0,
    #[doc = "Unicast Slow Protocol Packet Detection is enabled"]
    ENABLE = 0x01,
}
impl Usp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usp {
    #[inline(always)]
    fn from(val: u8) -> Usp {
        Usp::from_bits(val)
    }
}
impl From<Usp> for u8 {
    #[inline(always)]
    fn from(val: Usp) -> u8 {
        Usp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vffqe {
    #[doc = "VLAN tag Filter Fail Packets Queuing is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN tag Filter Fail Packets Queuing is enabled"]
    ENABLE = 0x01,
}
impl Vffqe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vffqe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vffqe {
    #[inline(always)]
    fn from(val: u8) -> Vffqe {
        Vffqe::from_bits(val)
    }
}
impl From<Vffqe> for u8 {
    #[inline(always)]
    fn from(val: Vffqe) -> u8 {
        Vffqe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vlhash {
    #[doc = "VLAN Hash Filter not selected"]
    INACTIVE = 0x0,
    #[doc = "VLAN Hash Filter selected"]
    ACTIVE = 0x01,
}
impl Vlhash {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vlhash {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vlhash {
    #[inline(always)]
    fn from(val: u8) -> Vlhash {
        Vlhash::from_bits(val)
    }
}
impl From<Vlhash> for u8 {
    #[inline(always)]
    fn from(val: Vlhash) -> u8 {
        Vlhash::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vtfe {
    #[doc = "VLAN Tag Filter is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Tag Filter is enabled"]
    ENABLE = 0x01,
}
impl Vtfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vtfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vtfe {
    #[inline(always)]
    fn from(val: u8) -> Vtfe {
        Vtfe::from_bits(val)
    }
}
impl From<Vtfe> for u8 {
    #[inline(always)]
    fn from(val: Vtfe) -> u8 {
        Vtfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vtim {
    #[doc = "VLAN Tag Inverse Match is disabled"]
    DISABLE = 0x0,
    #[doc = "VLAN Tag Inverse Match is enabled"]
    ENABLE = 0x01,
}
impl Vtim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vtim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vtim {
    #[inline(always)]
    fn from(val: u8) -> Vtim {
        Vtim::from_bits(val)
    }
}
impl From<Vtim> for u8 {
    #[inline(always)]
    fn from(val: Vtim) -> u8 {
        Vtim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wd {
    #[doc = "Watchdog is enabled"]
    ENABLE = 0x0,
    #[doc = "Watchdog is disabled"]
    DISABLE = 0x01,
}
impl Wd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wd {
    #[inline(always)]
    fn from(val: u8) -> Wd {
        Wd::from_bits(val)
    }
}
impl From<Wd> for u8 {
    #[inline(always)]
    fn from(val: Wd) -> u8 {
        Wd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wto {
    #[doc = "2 KB"]
    BF_2KBYTES = 0x0,
    #[doc = "3 KB"]
    BF_3KBYTES = 0x01,
    #[doc = "4 KB"]
    BF_4KBYTES = 0x02,
    #[doc = "5 KB"]
    BF_5KBYTES = 0x03,
    #[doc = "6 KB"]
    BF_6KBYTES = 0x04,
    #[doc = "7 KB"]
    BF_7KBYTES = 0x05,
    #[doc = "8 KB"]
    BF_8KBYTES = 0x06,
    #[doc = "9 KB"]
    BF_9KBYTES = 0x07,
    #[doc = "10 KB"]
    BF_10KBYTES = 0x08,
    #[doc = "11 KB"]
    BF_11KBYTES = 0x09,
    #[doc = "12 KB"]
    BF_12KBYTES = 0x0a,
    #[doc = "13 KB"]
    BF_13KBYTES = 0x0b,
    #[doc = "14 KB"]
    BF_14KBYTES = 0x0c,
    #[doc = "15 KB"]
    BF_15KBYTES = 0x0d,
    #[doc = "16383 Bytes"]
    BF_16383BYTES = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Wto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wto {
    #[inline(always)]
    fn from(val: u8) -> Wto {
        Wto::from_bits(val)
    }
}
impl From<Wto> for u8 {
    #[inline(always)]
    fn from(val: Wto) -> u8 {
        Wto::to_bits(val)
    }
}
