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
