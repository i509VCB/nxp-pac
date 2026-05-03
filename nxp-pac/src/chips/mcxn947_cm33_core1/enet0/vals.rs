#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actphysel {
    #[doc = "GMII or MII."]
    GmiiMii = 0x0,
    #[doc = "RGMII."]
    Rgmii = 0x01,
    #[doc = "SGMII."]
    Sgmii = 0x02,
    #[doc = "TBI."]
    Tbi = 0x03,
    #[doc = "RMII."]
    Rmii = 0x04,
    #[doc = "RTBI."]
    Rtbi = 0x05,
    #[doc = "SMII."]
    Smii = 0x06,
    #[doc = "RevMII."]
    Revmiii = 0x07,
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
    #[doc = "32."]
    M32 = 0x0,
    #[doc = "40."]
    M40 = 0x01,
    #[doc = "48."]
    M48 = 0x02,
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
    #[doc = "Add time."]
    Add = 0x0,
    #[doc = "Subtract time."]
    Sub = 0x01,
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
    #[doc = "No Safety features selected."]
    None = 0x0,
    #[doc = "Only \"ECC protection for external memory\" feature is selected."]
    EccOnly = 0x01,
    #[doc = "All the Automotive Safety features are selected without the \"Parity Port Enable for external interface\" feature."]
    AsNppe = 0x02,
    #[doc = "All the Automotive Safety features are selected with the \"Parity Port Enable for external interface\" feature."]
    AsPpe = 0x03,
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
    #[doc = "No auxiliary input."]
    NoAuxi = 0x0,
    #[doc = "1 auxiliary input."]
    M1Auxi = 0x01,
    #[doc = "2 auxiliary input."]
    M2Auxi = 0x02,
    #[doc = "3 auxiliary input."]
    M3Auxi = 0x03,
    #[doc = "4 auxiliary input."]
    M4Auxi = 0x04,
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
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
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
    #[doc = "k = min(n,10)."]
    MinN10 = 0x0,
    #[doc = "k = min(n,8)."]
    MinN8 = 0x01,
    #[doc = "k = min(n,4)."]
    MinN4 = 0x02,
    #[doc = "k = min(n,1)."]
    MinN1 = 0x03,
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
    #[doc = "Write operation."]
    Write = 0x0,
    #[doc = "Read operation."]
    Read = 0x01,
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
    #[doc = "Weighted Round-Robin with Rx:Tx or Tx:Rx."]
    Wrr = 0x0,
    #[doc = "Fixed Priority."]
    Fp = 0x01,
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
    #[doc = "Enable Broadcast Packets."]
    Enable = 0x0,
    #[doc = "Disable Broadcast Packets."]
    Disable = 0x01,
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
    #[doc = "CRC Checking is enabled."]
    Enable = 0x0,
    #[doc = "CRC Checking is disabled."]
    Disable = 0x01,
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
    #[doc = "Enable Carrier Sense During Transmission."]
    Enable = 0x0,
    #[doc = "Disable Carrier Sense During Transmission."]
    Disable = 0x01,
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
    #[doc = "Half-duplex mode."]
    Hduplx = 0x0,
    #[doc = "Full-duplex mode."]
    Fduplx = 0x01,
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
    #[doc = "Stop Receive."]
    Stop = 0x0,
    #[doc = "Start Receive."]
    Start = 0x01,
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
    #[doc = "Stop Transmission Command."]
    Stop = 0x0,
    #[doc = "Start Transmission Command."]
    Start = 0x01,
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
    #[doc = "Stop Receive."]
    Stop = 0x0,
    #[doc = "Start Receive."]
    Start = 0x01,
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
    #[doc = "Stop Transmission Command."]
    Stop = 0x0,
    #[doc = "Start Transmission Command."]
    Start = 0x01,
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
    #[doc = "The priority ratio is 1:1."]
    R11 = 0x0,
    #[doc = "The priority ratio is 2:1."]
    R21 = 0x01,
    #[doc = "The priority ratio is 3:1."]
    R31 = 0x02,
    #[doc = "The priority ratio is 4:1."]
    R41 = 0x03,
    #[doc = "The priority ratio is 5:1."]
    R51 = 0x04,
    #[doc = "The priority ratio is 6:1."]
    R61 = 0x05,
    #[doc = "The priority ratio is 7:1."]
    R71 = 0x06,
    #[doc = "The priority ratio is 8:1."]
    R81 = 0x07,
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
    #[doc = "Enable Receive Own."]
    Enable = 0x0,
    #[doc = "Disable Receive Own."]
    Disable = 0x01,
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
    #[doc = "VLAN Type Check is enabled."]
    Enable = 0x0,
    #[doc = "VLAN Type Check is disabled."]
    Disable = 0x01,
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
    #[doc = "Enable Retry."]
    Enable = 0x0,
    #[doc = "Disable Retry."]
    Disable = 0x01,
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
    #[doc = "Zero-Quanta Pause packet generation is enabled."]
    Enable = 0x0,
    #[doc = "Zero-Quanta Pause packet generation is disabled."]
    Disable = 0x01,
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
    #[doc = "Do not strip."]
    Donot = 0x0,
    #[doc = "Strip if VLAN filter passes."]
    Ifpass = 0x01,
    #[doc = "Strip if VLAN filter fails."]
    Iffail = 0x02,
    #[doc = "Always strip."]
    Always = 0x03,
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
    #[doc = "No Depth configured."]
    Nodepth = 0x0,
    #[doc = "64."]
    Depth64 = 0x01,
    #[doc = "128."]
    Depth128 = 0x02,
    #[doc = "256."]
    Depth256 = 0x03,
    #[doc = "512."]
    Depth512 = 0x04,
    #[doc = "1024."]
    Depth1024 = 0x05,
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
    #[doc = "Width not configured."]
    Nowidth = 0x0,
    #[doc = "16."]
    Width16 = 0x01,
    #[doc = "20."]
    Width20 = 0x02,
    #[doc = "24."]
    Width24 = 0x03,
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
    #[doc = "Do not strip."]
    Donot = 0x0,
    #[doc = "Strip if VLAN filter passes."]
    Ifpass = 0x01,
    #[doc = "Strip if VLAN filter fails."]
    Iffail = 0x02,
    #[doc = "Always strip."]
    Always = 0x03,
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
    #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0."]
    M101000m = 0x0,
    #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0."]
    M1002500m = 0x01,
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
    #[doc = "64 Bytes."]
    M64bytes = 0x0,
    #[doc = "128 Bytes."]
    M128bytes = 0x01,
    #[doc = "256 Bytes."]
    M256bytes = 0x02,
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
    #[doc = "64 Entries."]
    M64entr = 0x0,
    #[doc = "128 Entries."]
    M128entr = 0x01,
    #[doc = "256 Entries."]
    M256entr = 0x02,
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
    #[doc = "No hash table."]
    NoHt = 0x0,
    #[doc = "64."]
    M64 = 0x01,
    #[doc = "128."]
    M128 = 0x02,
    #[doc = "256."]
    M256 = 0x03,
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
    #[doc = "96 bit times IPG."]
    Ipg96 = 0x0,
    #[doc = "88 bit times IPG."]
    Ipg88 = 0x01,
    #[doc = "80 bit times IPG."]
    Ipg80 = 0x02,
    #[doc = "72 bit times IPG."]
    Ipg72 = 0x03,
    #[doc = "64 bit times IPG."]
    Ipg64 = 0x04,
    #[doc = "56 bit times IPG."]
    Ipg56 = 0x05,
    #[doc = "48 bit times IPG."]
    Ipg48 = 0x06,
    #[doc = "40 bit times IPG."]
    Ipg40 = 0x07,
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
    #[doc = "Jabber is enabled."]
    Enable = 0x0,
    #[doc = "Jabber is disabled."]
    Disable = 0x01,
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
    #[doc = "No L3 or L4 Filter."]
    Nofilt = 0x0,
    #[doc = "1 L3 or L4 Filter."]
    M1filt = 0x01,
    #[doc = "2 L3 or L4 Filters."]
    M2filt = 0x02,
    #[doc = "3 L3 or L4 Filters."]
    M3filt = 0x03,
    #[doc = "4 L3 or L4 Filters."]
    M4filt = 0x04,
    #[doc = "5 L3 or L4 Filters."]
    M5filt = 0x05,
    #[doc = "6 L3 or L4 Filters."]
    M6filt = 0x06,
    #[doc = "7 L3 or L4 Filters."]
    M7filt = 0x07,
    #[doc = "8 L3 or L4 Filters."]
    M8filt = 0x08,
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
    #[doc = "C-VLAN type (0x8100) is inserted."]
    Cvlan = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted."]
    Svlan = 0x01,
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
    #[doc = "No VLAN tag deletion, insertion, or replacement."]
    None = 0x0,
    #[doc = "VLAN tag deletion."]
    Delete = 0x01,
    #[doc = "VLAN tag insertion."]
    Insert = 0x02,
    #[doc = "VLAN tag replacement."]
    Replace = 0x03,
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
    #[doc = "C-VLAN type (0x8100) is inserted or replaced."]
    CVlan = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted or replaced."]
    SVlan = 0x01,
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
    #[doc = "No VLAN tag deletion, insertion, or replacement."]
    None = 0x0,
    #[doc = "VLAN tag deletion."]
    Delete = 0x01,
    #[doc = "VLAN tag insertion."]
    Insert = 0x02,
    #[doc = "VLAN tag replacement."]
    Replace = 0x03,
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
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
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
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Reading packet data."]
    ReadData = 0x01,
    #[doc = "Reading packet status (or timestamp)."]
    ReadSts = 0x02,
    #[doc = "Flushing the packet data and status."]
    Flush = 0x03,
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
    #[doc = "Rx Queue empty."]
    Empty = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold."]
    BlwThr = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold."]
    AbvThr = 0x02,
    #[doc = "Rx Queue full."]
    Full = 0x03,
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
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled."]
    Enable = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled."]
    Disable = 0x01,
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
    #[doc = "64."]
    M64byte = 0x0,
    #[doc = "32."]
    M32byte = 0x01,
    #[doc = "96."]
    M96byte = 0x02,
    #[doc = "128."]
    M128byte = 0x03,
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
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Reading packet data."]
    ReadData = 0x01,
    #[doc = "Reading packet status (or timestamp)."]
    ReadSts = 0x02,
    #[doc = "Flushing the packet data and status."]
    Flush = 0x03,
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
    #[doc = "Rx Queue empty."]
    Empty = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold."]
    BlwThr = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold."]
    AbvThr = 0x02,
    #[doc = "Rx Queue full."]
    Full = 0x03,
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
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled."]
    Enable = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled."]
    Disable = 0x01,
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
    #[doc = "64."]
    M64byte = 0x0,
    #[doc = "32."]
    M32byte = 0x01,
    #[doc = "96."]
    M96byte = 0x02,
    #[doc = "128."]
    M128byte = 0x03,
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
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)."]
    Read = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter."]
    Wait = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC."]
    Flush = 0x03,
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
    #[doc = "32."]
    M32bytes = 0x0,
    #[doc = "64."]
    M64bytes = 0x01,
    #[doc = "96."]
    M96bytes = 0x02,
    #[doc = "128."]
    M128bytes = 0x03,
    #[doc = "192."]
    M192bytes = 0x04,
    #[doc = "256."]
    M256bytes = 0x05,
    #[doc = "384."]
    M384bytes = 0x06,
    #[doc = "512."]
    M512bytes = 0x07,
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
    #[doc = "Not enabled."]
    Disable = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)."]
    EnIfAv = 0x01,
    #[doc = "Enabled."]
    Enable = 0x02,
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
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)."]
    Read = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter."]
    Wait = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC."]
    Flush = 0x03,
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
    #[doc = "32."]
    M32bytes = 0x0,
    #[doc = "64."]
    M64bytes = 0x01,
    #[doc = "96."]
    M96bytes = 0x02,
    #[doc = "128."]
    M128bytes = 0x03,
    #[doc = "192."]
    M192bytes = 0x04,
    #[doc = "256."]
    M256bytes = 0x05,
    #[doc = "384."]
    M384bytes = 0x06,
    #[doc = "512."]
    M512bytes = 0x07,
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
    #[doc = "Not enabled."]
    Disable = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)."]
    EnIfAv = 0x01,
    #[doc = "Enabled."]
    Enable = 0x02,
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
    #[doc = "No Extended Rx VLAN Filters."]
    NoErvlan = 0x0,
    #[doc = "4 Extended Rx VLAN Filters."]
    M4Ervlan = 0x01,
    #[doc = "8 Extended Rx VLAN Filters."]
    M8Ervlan = 0x02,
    #[doc = "16 Extended Rx VLAN Filters."]
    M16Ervlan = 0x03,
    #[doc = "24 Extended Rx VLAN Filters."]
    M24Ervlan = 0x04,
    #[doc = "32 Extended Rx VLAN Filters."]
    M32Ervlan = 0x05,
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
    #[doc = "MAC filters all control packets from reaching the application."]
    FltrAll = 0x0,
    #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the address filter."]
    FwXcptPau = 0x01,
    #[doc = "MAC forwards all control packets to the application even if they fail the address filter."]
    FwAll = 0x02,
    #[doc = "MAC forwards the control packets that pass the Address filter."]
    FwPass = 0x03,
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
    #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)."]
    Pt4 = 0x0,
    #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)."]
    Pt28 = 0x01,
    #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)."]
    Pt36 = 0x02,
    #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)."]
    Pt144 = 0x03,
    #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)."]
    Pt256 = 0x04,
    #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)."]
    Pt512 = 0x05,
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
    #[doc = "No PPS output."]
    NoPpso = 0x0,
    #[doc = "1 PPS output."]
    M1Ppso = 0x01,
    #[doc = "2 PPS output."]
    M2Ppso = 0x02,
    #[doc = "3 PPS output."]
    M3Ppso = 0x03,
    #[doc = "4 PPS output."]
    M4Ppso = 0x04,
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
    #[doc = "7 bytes of preamble."]
    M7bytes = 0x0,
    #[doc = "5 bytes of preamble."]
    M5bytes = 0x01,
    #[doc = "3 bytes of preamble."]
    M3bytes = 0x02,
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
    #[doc = "For 1000 or 2500 Mbps operations."]
    M10002500m = 0x0,
    #[doc = "For 10 or 100 Mbps operations."]
    M10100m = 0x01,
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
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
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
    #[doc = "Strict priority (SP)."]
    Sp = 0x0,
    #[doc = "Weighted Strict Priority (WSP)."]
    Wsp = 0x01,
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
    #[doc = "Read operation of indirect access."]
    Read = 0x0,
    #[doc = "Write operation of indirect access."]
    Write = 0x01,
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
    #[doc = "Stopped (Reset or Stop Receive Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)."]
    RunFrtd = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)."]
    RunWrp = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)."]
    Suspnd = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)."]
    RunCrd = 0x05,
    #[doc = "Timestamp write state."]
    Tstmp = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)."]
    RunTrp = 0x07,
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
    #[doc = "Stopped (Reset or Stop Receive Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)."]
    RunFrtd = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)."]
    RunWrp = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)."]
    Suspnd = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)."]
    RunCrd = 0x05,
    #[doc = "Timestamp write state."]
    Tstmp = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)."]
    RunTrp = 0x07,
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
    #[doc = "1 MTL Rx Channel."]
    M1rxch = 0x0,
    #[doc = "2 MTL Rx Channels."]
    M2rxch = 0x01,
    #[doc = "3 MTL Rx Channels."]
    M3rxch = 0x02,
    #[doc = "4 MTL Rx Channels."]
    M4rxch = 0x03,
    #[doc = "5 MTL Rx Channels."]
    M5rxch = 0x04,
    #[doc = "6 MTL Rx Channels."]
    M6rxch = 0x05,
    #[doc = "7 MTL Rx Channels."]
    M7rxch = 0x06,
    #[doc = "8 MTL Rx Channels."]
    M8rxch = 0x07,
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
    #[doc = "128 bytes."]
    M128b = 0x0,
    #[doc = "256 bytes."]
    M256b = 0x01,
    #[doc = "512 bytes."]
    M512b = 0x02,
    #[doc = "1024 bytes."]
    M1024b = 0x03,
    #[doc = "2048 bytes."]
    M2048b = 0x04,
    #[doc = "4096 bytes."]
    M4096b = 0x05,
    #[doc = "8192 bytes."]
    M8192b = 0x06,
    #[doc = "16384 bytes."]
    M16384b = 0x07,
    #[doc = "32 KB."]
    M32kb = 0x08,
    #[doc = "64 KB."]
    M64kb = 0x09,
    #[doc = "128 KB."]
    M128kb = 0x0a,
    #[doc = "256 KB."]
    M256kb = 0x0b,
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
    #[doc = "Queue not enabled."]
    Disable = 0x0,
    #[doc = "Queue enabled for AV."]
    EnAv = 0x01,
    #[doc = "Queue enabled for DCB/Generic."]
    EnDcbGen = 0x02,
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
    #[doc = "Queue not enabled."]
    Disable = 0x0,
    #[doc = "Queue enabled for AV."]
    EnAv = 0x01,
    #[doc = "Queue enabled for DCB/Generic."]
    EnDcbGen = 0x02,
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
    #[doc = "1 MTL Rx Queue."]
    M1rxq = 0x0,
    #[doc = "2 MTL Rx Queues."]
    M2rxq = 0x01,
    #[doc = "3 MTL Rx Queues."]
    M3rxq = 0x02,
    #[doc = "4 MTL Rx Queues."]
    M4rxq = 0x03,
    #[doc = "5 MTL Rx Queues."]
    M5rxq = 0x04,
    #[doc = "6 MTL Rx Queues."]
    M6rxq = 0x05,
    #[doc = "7 MTL Rx Queues."]
    M7rxq = 0x06,
    #[doc = "8 MTL Rx Queues."]
    M8rxq = 0x07,
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
    #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation."]
    SaCtrlIn = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Contents of MAC Addr-0 inserted in SA field."]
    Mac0InsSa = 0x02,
    #[doc = "Contents of MAC Addr-0 replaces SA field."]
    Mac0RepSa = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Contents of MAC Addr-1 inserted in SA field."]
    Mac1InsSa = 0x06,
    #[doc = "Contents of MAC Addr-1 replaces SA field."]
    Mac1RepSa = 0x07,
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
    #[doc = "WRR algorithm."]
    Wrr = 0x0,
    #[doc = "WFQ algorithm when DCB feature is selected.Otherwise, Reserved."]
    Wfq = 0x01,
    #[doc = "DWRR algorithm when DCB feature is selected.Otherwise, Reserved."]
    Dwrr = 0x02,
    #[doc = "Strict priority algorithm."]
    Sp = 0x03,
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
    #[doc = "1 slot."]
    M1Slot = 0x0,
    #[doc = "2 slots."]
    M2Slot = 0x01,
    #[doc = "4 slots."]
    M4Slot = 0x02,
    #[doc = "8 slots."]
    M8Slot = 0x03,
    #[doc = "16 slots."]
    M16Slot = 0x04,
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
    #[doc = "Fixed priority."]
    Fp = 0x0,
    #[doc = "Weighted Strict Priority (WSP)."]
    Wsp = 0x01,
    #[doc = "Weighted Round-Robin (WRR)."]
    Wrr = 0x02,
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
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over."]
    Waiting = 0x01,
    #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)."]
    GenTxPau = 0x02,
    #[doc = "Transferring input packet for transmission."]
    Trnsfr = 0x03,
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
    #[doc = "Stopped (Reset or Stop Transmit Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)."]
    RunFttd = 0x01,
    #[doc = "Running (Waiting for status)."]
    RunWs = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))."]
    RunRds = 0x03,
    #[doc = "Timestamp write state."]
    TstmpWs = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)."]
    Suspnd = 0x06,
    #[doc = "Running (Closing Tx Descriptor)."]
    RunCtd = 0x07,
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
    #[doc = "Stopped (Reset or Stop Transmit Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)."]
    RunFttd = 0x01,
    #[doc = "Running (Waiting for status)."]
    RunWs = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))."]
    RunRds = 0x03,
    #[doc = "Timestamp write state."]
    TstmpWs = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)."]
    Suspnd = 0x06,
    #[doc = "Running (Closing Tx Descriptor)."]
    RunCtd = 0x07,
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
    #[doc = "Coarse method is used to update system timestamp."]
    Coarse = 0x0,
    #[doc = "Fine method is used to update system timestamp."]
    Fine = 0x01,
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
    #[doc = "Timestamp Digital Rollover Control is disabled and Binary Rollover Control is enabled."]
    DigDisable = 0x0,
    #[doc = "Timestamp Digital Rollover Control is enabled and Binary Rollover Control is disabled."]
    DigEnable = 0x01,
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
    #[doc = "Internal."]
    Intrnl = 0x0,
    #[doc = "External."]
    Extrnl = 0x01,
    #[doc = "Both."]
    Both = 0x02,
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
    #[doc = "1 MTL Tx Channel."]
    M1txch = 0x0,
    #[doc = "2 MTL Tx Channels."]
    M2txch = 0x01,
    #[doc = "3 MTL Tx Channels."]
    M3txch = 0x02,
    #[doc = "4 MTL Tx Channels."]
    M4txch = 0x03,
    #[doc = "5 MTL Tx Channels."]
    M5txch = 0x04,
    #[doc = "6 MTL Tx Channels."]
    M6txch = 0x05,
    #[doc = "7 MTL Tx Channels."]
    M7txch = 0x06,
    #[doc = "8 MTL Tx Channels."]
    M8txch = 0x07,
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
    #[doc = "128 bytes."]
    M128b = 0x0,
    #[doc = "256 bytes."]
    M256b = 0x01,
    #[doc = "512 bytes."]
    M512b = 0x02,
    #[doc = "1024 bytes."]
    M1024b = 0x03,
    #[doc = "2048 bytes."]
    M2048b = 0x04,
    #[doc = "4096 bytes."]
    M4096b = 0x05,
    #[doc = "8192 bytes."]
    M8192b = 0x06,
    #[doc = "16384 bytes."]
    M16384b = 0x07,
    #[doc = "32 KB."]
    M32kb = 0x08,
    #[doc = "64 KB."]
    M64kb = 0x09,
    #[doc = "128 KB."]
    M128kb = 0x0a,
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
    #[doc = "1 MTL Tx Queue."]
    M1txq = 0x0,
    #[doc = "2 MTL Tx Queues."]
    M2txq = 0x01,
    #[doc = "3 MTL Tx Queues."]
    M3txq = 0x02,
    #[doc = "4 MTL Tx Queues."]
    M4txq = 0x03,
    #[doc = "5 MTL Tx Queues."]
    M5txq = 0x04,
    #[doc = "6 MTL Tx Queues."]
    M6txq = 0x05,
    #[doc = "7 MTL Tx Queues."]
    M7txq = 0x06,
    #[doc = "8 MTL Tx Queues."]
    M8txq = 0x07,
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
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
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
    #[doc = "Watchdog is enabled."]
    Enable = 0x0,
    #[doc = "Watchdog is disabled."]
    Disable = 0x01,
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
    #[doc = "2 KB."]
    Bf2kbytes = 0x0,
    #[doc = "3 KB."]
    Bf3kbytes = 0x01,
    #[doc = "4 KB."]
    Bf4kbytes = 0x02,
    #[doc = "5 KB."]
    Bf5kbytes = 0x03,
    #[doc = "6 KB."]
    Bf6kbytes = 0x04,
    #[doc = "7 KB."]
    Bf7kbytes = 0x05,
    #[doc = "8 KB."]
    Bf8kbytes = 0x06,
    #[doc = "9 KB."]
    Bf9kbytes = 0x07,
    #[doc = "10 KB."]
    Bf10kbytes = 0x08,
    #[doc = "11 KB."]
    Bf11kbytes = 0x09,
    #[doc = "12 KB."]
    Bf12kbytes = 0x0a,
    #[doc = "13 KB."]
    Bf13kbytes = 0x0b,
    #[doc = "14 KB."]
    Bf14kbytes = 0x0c,
    #[doc = "15 KB."]
    Bf15kbytes = 0x0d,
    #[doc = "16383 Bytes."]
    Bf16383bytes = 0x0e,
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
