#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbrst {
    #[doc = "Incremental burst of unspecified length only"]
    INCR_BURST = 0x0,
    #[doc = "INCR4 burst, then single transfer"]
    INCR4_BURST = 0x01,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer"]
    INCR8_BURST = 0x02,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer"]
    INCR16_BURST = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "INCR4 burst, then incremental burst of unspecified length"]
    INCR4_UNSPEC = 0x05,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    INCR8_4_UNSPEC = 0x06,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    INCR16_8_4_UNSPEC = 0x07,
}
impl Ahbbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbrst {
    #[inline(always)]
    fn from(val: u8) -> Ahbbrst {
        Ahbbrst::from_bits(val)
    }
}
impl From<Ahbbrst> for u8 {
    #[inline(always)]
    fn from(val: Ahbbrst) -> u8 {
        Ahbbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ase {
    #[doc = "Do not process the Asynchronous Schedule"]
    DONT_PROCESS_ASYNC = 0x0,
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule"]
    ACCESS_ASYNC = 0x01,
}
impl Ase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ase {
    #[inline(always)]
    fn from(val: u8) -> Ase {
        Ase::from_bits(val)
    }
}
impl From<Ase> for u8 {
    #[inline(always)]
    fn from(val: Ase) -> u8 {
        Ase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccs {
    #[doc = "In Host mode: No device is present. In Device mode: Not attached"]
    DISABLE = 0x0,
    #[doc = "In Host mode: Device is present on port. In Device mode: Attached"]
    ENABLE = 0x01,
}
impl Ccs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccs {
    #[inline(always)]
    fn from(val: u8) -> Ccs {
        Ccs::from_bits(val)
    }
}
impl From<Ccs> for u8 {
    #[inline(always)]
    fn from(val: Ccs) -> u8 {
        Ccs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cf {
    #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller"]
    PORT_ROUTING_CLASSIC_HOST = 0x0,
    #[doc = "Port routing control logic default-routes all ports to this host controller"]
    PORT_ROUTING_HOST = 0x01,
}
impl Cf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cf {
    #[inline(always)]
    fn from(val: u8) -> Cf {
        Cf::from_bits(val)
    }
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(val: Cf) -> u8 {
        Cf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Idle \\[Default for combination host/device\\]"]
    IDL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Device Controller \\[Default for device only controller\\]"]
    DEVICE_CONTR = 0x02,
    #[doc = "Host Controller \\[Default for host only controller\\]"]
    HOST_CONTR = 0x03,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csc {
    #[doc = "No change"]
    DISABLE = 0x0,
    #[doc = "Change in current connect status"]
    ENABLE = 0x01,
}
impl Csc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csc {
    #[inline(always)]
    fn from(val: u8) -> Csc {
        Csc::from_bits(val)
    }
}
impl From<Csc> for u8 {
    #[inline(always)]
    fn from(val: Csc) -> u8 {
        Csc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dc {
    #[doc = "Not supported"]
    DEVICE_OP_DIS = 0x0,
    #[doc = "Supported"]
    DEVICE_OP_EN = 0x01,
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
pub enum Endptctrl0Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl0Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Rxe {
        Endptctrl0Rxe::from_bits(val)
    }
}
impl From<Endptctrl0Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Rxe) -> u8 {
        Endptctrl0Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl0Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Rxs {
        Endptctrl0Rxs::from_bits(val)
    }
}
impl From<Endptctrl0Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Rxs) -> u8 {
        Endptctrl0Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Rxt {
    #[doc = "Control"]
    CTRL = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Endptctrl0Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Rxt {
        Endptctrl0Rxt::from_bits(val)
    }
}
impl From<Endptctrl0Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Rxt) -> u8 {
        Endptctrl0Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl0Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Txe {
        Endptctrl0Txe::from_bits(val)
    }
}
impl From<Endptctrl0Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Txe) -> u8 {
        Endptctrl0Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl0Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Txs {
        Endptctrl0Txs::from_bits(val)
    }
}
impl From<Endptctrl0Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Txs) -> u8 {
        Endptctrl0Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Txt {
    #[doc = "Control"]
    CTRL = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Endptctrl0Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Txt {
        Endptctrl0Txt::from_bits(val)
    }
}
impl From<Endptctrl0Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Txt) -> u8 {
        Endptctrl0Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl1Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxd {
        Endptctrl1Rxd::from_bits(val)
    }
}
impl From<Endptctrl1Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxd) -> u8 {
        Endptctrl1Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl1Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxe {
        Endptctrl1Rxe::from_bits(val)
    }
}
impl From<Endptctrl1Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxe) -> u8 {
        Endptctrl1Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl1Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxi {
        Endptctrl1Rxi::from_bits(val)
    }
}
impl From<Endptctrl1Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxi) -> u8 {
        Endptctrl1Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl1Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxr {
        Endptctrl1Rxr::from_bits(val)
    }
}
impl From<Endptctrl1Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxr) -> u8 {
        Endptctrl1Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl1Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxs {
        Endptctrl1Rxs::from_bits(val)
    }
}
impl From<Endptctrl1Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxs) -> u8 {
        Endptctrl1Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl1Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Rxt {
        Endptctrl1Rxt::from_bits(val)
    }
}
impl From<Endptctrl1Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Rxt) -> u8 {
        Endptctrl1Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl1Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txd {
        Endptctrl1Txd::from_bits(val)
    }
}
impl From<Endptctrl1Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txd) -> u8 {
        Endptctrl1Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl1Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txe {
        Endptctrl1Txe::from_bits(val)
    }
}
impl From<Endptctrl1Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txe) -> u8 {
        Endptctrl1Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl1Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txi {
        Endptctrl1Txi::from_bits(val)
    }
}
impl From<Endptctrl1Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txi) -> u8 {
        Endptctrl1Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl1Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txr {
        Endptctrl1Txr::from_bits(val)
    }
}
impl From<Endptctrl1Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txr) -> u8 {
        Endptctrl1Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl1Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txs {
        Endptctrl1Txs::from_bits(val)
    }
}
impl From<Endptctrl1Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txs) -> u8 {
        Endptctrl1Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl1Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl1Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl1Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl1Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl1Txt {
        Endptctrl1Txt::from_bits(val)
    }
}
impl From<Endptctrl1Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl1Txt) -> u8 {
        Endptctrl1Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl2Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxd {
        Endptctrl2Rxd::from_bits(val)
    }
}
impl From<Endptctrl2Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxd) -> u8 {
        Endptctrl2Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl2Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxe {
        Endptctrl2Rxe::from_bits(val)
    }
}
impl From<Endptctrl2Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxe) -> u8 {
        Endptctrl2Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl2Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxi {
        Endptctrl2Rxi::from_bits(val)
    }
}
impl From<Endptctrl2Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxi) -> u8 {
        Endptctrl2Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl2Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxr {
        Endptctrl2Rxr::from_bits(val)
    }
}
impl From<Endptctrl2Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxr) -> u8 {
        Endptctrl2Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl2Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxs {
        Endptctrl2Rxs::from_bits(val)
    }
}
impl From<Endptctrl2Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxs) -> u8 {
        Endptctrl2Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl2Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Rxt {
        Endptctrl2Rxt::from_bits(val)
    }
}
impl From<Endptctrl2Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Rxt) -> u8 {
        Endptctrl2Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl2Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txd {
        Endptctrl2Txd::from_bits(val)
    }
}
impl From<Endptctrl2Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txd) -> u8 {
        Endptctrl2Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl2Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txe {
        Endptctrl2Txe::from_bits(val)
    }
}
impl From<Endptctrl2Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txe) -> u8 {
        Endptctrl2Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl2Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txi {
        Endptctrl2Txi::from_bits(val)
    }
}
impl From<Endptctrl2Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txi) -> u8 {
        Endptctrl2Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl2Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txr {
        Endptctrl2Txr::from_bits(val)
    }
}
impl From<Endptctrl2Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txr) -> u8 {
        Endptctrl2Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl2Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txs {
        Endptctrl2Txs::from_bits(val)
    }
}
impl From<Endptctrl2Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txs) -> u8 {
        Endptctrl2Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl2Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl2Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl2Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl2Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl2Txt {
        Endptctrl2Txt::from_bits(val)
    }
}
impl From<Endptctrl2Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl2Txt) -> u8 {
        Endptctrl2Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl3Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxd {
        Endptctrl3Rxd::from_bits(val)
    }
}
impl From<Endptctrl3Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxd) -> u8 {
        Endptctrl3Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl3Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxe {
        Endptctrl3Rxe::from_bits(val)
    }
}
impl From<Endptctrl3Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxe) -> u8 {
        Endptctrl3Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl3Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxi {
        Endptctrl3Rxi::from_bits(val)
    }
}
impl From<Endptctrl3Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxi) -> u8 {
        Endptctrl3Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl3Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxr {
        Endptctrl3Rxr::from_bits(val)
    }
}
impl From<Endptctrl3Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxr) -> u8 {
        Endptctrl3Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl3Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxs {
        Endptctrl3Rxs::from_bits(val)
    }
}
impl From<Endptctrl3Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxs) -> u8 {
        Endptctrl3Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl3Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Rxt {
        Endptctrl3Rxt::from_bits(val)
    }
}
impl From<Endptctrl3Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Rxt) -> u8 {
        Endptctrl3Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl3Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txd {
        Endptctrl3Txd::from_bits(val)
    }
}
impl From<Endptctrl3Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txd) -> u8 {
        Endptctrl3Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl3Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txe {
        Endptctrl3Txe::from_bits(val)
    }
}
impl From<Endptctrl3Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txe) -> u8 {
        Endptctrl3Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl3Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txi {
        Endptctrl3Txi::from_bits(val)
    }
}
impl From<Endptctrl3Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txi) -> u8 {
        Endptctrl3Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl3Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txr {
        Endptctrl3Txr::from_bits(val)
    }
}
impl From<Endptctrl3Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txr) -> u8 {
        Endptctrl3Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl3Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txs {
        Endptctrl3Txs::from_bits(val)
    }
}
impl From<Endptctrl3Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txs) -> u8 {
        Endptctrl3Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl3Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl3Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl3Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl3Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl3Txt {
        Endptctrl3Txt::from_bits(val)
    }
}
impl From<Endptctrl3Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl3Txt) -> u8 {
        Endptctrl3Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl4Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxd {
        Endptctrl4Rxd::from_bits(val)
    }
}
impl From<Endptctrl4Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxd) -> u8 {
        Endptctrl4Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl4Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxe {
        Endptctrl4Rxe::from_bits(val)
    }
}
impl From<Endptctrl4Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxe) -> u8 {
        Endptctrl4Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl4Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxi {
        Endptctrl4Rxi::from_bits(val)
    }
}
impl From<Endptctrl4Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxi) -> u8 {
        Endptctrl4Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl4Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxr {
        Endptctrl4Rxr::from_bits(val)
    }
}
impl From<Endptctrl4Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxr) -> u8 {
        Endptctrl4Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl4Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxs {
        Endptctrl4Rxs::from_bits(val)
    }
}
impl From<Endptctrl4Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxs) -> u8 {
        Endptctrl4Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl4Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Rxt {
        Endptctrl4Rxt::from_bits(val)
    }
}
impl From<Endptctrl4Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Rxt) -> u8 {
        Endptctrl4Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl4Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txd {
        Endptctrl4Txd::from_bits(val)
    }
}
impl From<Endptctrl4Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txd) -> u8 {
        Endptctrl4Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl4Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txe {
        Endptctrl4Txe::from_bits(val)
    }
}
impl From<Endptctrl4Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txe) -> u8 {
        Endptctrl4Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl4Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txi {
        Endptctrl4Txi::from_bits(val)
    }
}
impl From<Endptctrl4Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txi) -> u8 {
        Endptctrl4Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl4Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txr {
        Endptctrl4Txr::from_bits(val)
    }
}
impl From<Endptctrl4Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txr) -> u8 {
        Endptctrl4Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl4Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txs {
        Endptctrl4Txs::from_bits(val)
    }
}
impl From<Endptctrl4Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txs) -> u8 {
        Endptctrl4Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl4Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl4Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl4Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl4Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl4Txt {
        Endptctrl4Txt::from_bits(val)
    }
}
impl From<Endptctrl4Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl4Txt) -> u8 {
        Endptctrl4Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl5Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxd {
        Endptctrl5Rxd::from_bits(val)
    }
}
impl From<Endptctrl5Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxd) -> u8 {
        Endptctrl5Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl5Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxe {
        Endptctrl5Rxe::from_bits(val)
    }
}
impl From<Endptctrl5Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxe) -> u8 {
        Endptctrl5Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl5Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxi {
        Endptctrl5Rxi::from_bits(val)
    }
}
impl From<Endptctrl5Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxi) -> u8 {
        Endptctrl5Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl5Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxr {
        Endptctrl5Rxr::from_bits(val)
    }
}
impl From<Endptctrl5Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxr) -> u8 {
        Endptctrl5Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl5Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxs {
        Endptctrl5Rxs::from_bits(val)
    }
}
impl From<Endptctrl5Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxs) -> u8 {
        Endptctrl5Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl5Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Rxt {
        Endptctrl5Rxt::from_bits(val)
    }
}
impl From<Endptctrl5Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Rxt) -> u8 {
        Endptctrl5Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl5Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txd {
        Endptctrl5Txd::from_bits(val)
    }
}
impl From<Endptctrl5Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txd) -> u8 {
        Endptctrl5Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl5Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txe {
        Endptctrl5Txe::from_bits(val)
    }
}
impl From<Endptctrl5Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txe) -> u8 {
        Endptctrl5Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl5Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txi {
        Endptctrl5Txi::from_bits(val)
    }
}
impl From<Endptctrl5Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txi) -> u8 {
        Endptctrl5Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl5Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txr {
        Endptctrl5Txr::from_bits(val)
    }
}
impl From<Endptctrl5Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txr) -> u8 {
        Endptctrl5Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl5Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txs {
        Endptctrl5Txs::from_bits(val)
    }
}
impl From<Endptctrl5Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txs) -> u8 {
        Endptctrl5Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl5Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl5Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl5Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl5Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl5Txt {
        Endptctrl5Txt::from_bits(val)
    }
}
impl From<Endptctrl5Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl5Txt) -> u8 {
        Endptctrl5Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl6Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxd {
        Endptctrl6Rxd::from_bits(val)
    }
}
impl From<Endptctrl6Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxd) -> u8 {
        Endptctrl6Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl6Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxe {
        Endptctrl6Rxe::from_bits(val)
    }
}
impl From<Endptctrl6Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxe) -> u8 {
        Endptctrl6Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl6Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxi {
        Endptctrl6Rxi::from_bits(val)
    }
}
impl From<Endptctrl6Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxi) -> u8 {
        Endptctrl6Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl6Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxr {
        Endptctrl6Rxr::from_bits(val)
    }
}
impl From<Endptctrl6Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxr) -> u8 {
        Endptctrl6Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl6Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxs {
        Endptctrl6Rxs::from_bits(val)
    }
}
impl From<Endptctrl6Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxs) -> u8 {
        Endptctrl6Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl6Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Rxt {
        Endptctrl6Rxt::from_bits(val)
    }
}
impl From<Endptctrl6Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Rxt) -> u8 {
        Endptctrl6Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl6Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txd {
        Endptctrl6Txd::from_bits(val)
    }
}
impl From<Endptctrl6Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txd) -> u8 {
        Endptctrl6Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl6Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txe {
        Endptctrl6Txe::from_bits(val)
    }
}
impl From<Endptctrl6Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txe) -> u8 {
        Endptctrl6Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl6Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txi {
        Endptctrl6Txi::from_bits(val)
    }
}
impl From<Endptctrl6Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txi) -> u8 {
        Endptctrl6Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl6Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txr {
        Endptctrl6Txr::from_bits(val)
    }
}
impl From<Endptctrl6Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txr) -> u8 {
        Endptctrl6Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl6Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txs {
        Endptctrl6Txs::from_bits(val)
    }
}
impl From<Endptctrl6Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txs) -> u8 {
        Endptctrl6Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl6Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl6Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl6Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl6Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl6Txt {
        Endptctrl6Txt::from_bits(val)
    }
}
impl From<Endptctrl6Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl6Txt) -> u8 {
        Endptctrl6Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DISABLE = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl7Rxd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxd {
        Endptctrl7Rxd::from_bits(val)
    }
}
impl From<Endptctrl7Rxd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxd) -> u8 {
        Endptctrl7Rxd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl7Rxe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxe {
        Endptctrl7Rxe::from_bits(val)
    }
}
impl From<Endptctrl7Rxe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxe) -> u8 {
        Endptctrl7Rxe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxi {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl7Rxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxi {
        Endptctrl7Rxi::from_bits(val)
    }
}
impl From<Endptctrl7Rxi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxi) -> u8 {
        Endptctrl7Rxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl7Rxr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxr {
        Endptctrl7Rxr::from_bits(val)
    }
}
impl From<Endptctrl7Rxr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxr) -> u8 {
        Endptctrl7Rxr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl7Rxs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxs {
        Endptctrl7Rxs::from_bits(val)
    }
}
impl From<Endptctrl7Rxs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxs) -> u8 {
        Endptctrl7Rxs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Rxt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl7Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Rxt {
        Endptctrl7Rxt::from_bits(val)
    }
}
impl From<Endptctrl7Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Rxt) -> u8 {
        Endptctrl7Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txd {
    #[doc = "Dual Port Memory Buffer/DMA Engine"]
    DMA = 0x0,
    _RESERVED_1 = 0x01,
}
impl Endptctrl7Txd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txd {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txd {
        Endptctrl7Txd::from_bits(val)
    }
}
impl From<Endptctrl7Txd> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txd) -> u8 {
        Endptctrl7Txd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txe {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Endptctrl7Txe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txe {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txe {
        Endptctrl7Txe::from_bits(val)
    }
}
impl From<Endptctrl7Txe> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txe) -> u8 {
        Endptctrl7Txe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txi {
    #[doc = "PID sequencing enabled"]
    PID_EN = 0x0,
    #[doc = "PID sequencing disabled"]
    PID_DIS = 0x01,
}
impl Endptctrl7Txi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txi {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txi {
        Endptctrl7Txi::from_bits(val)
    }
}
impl From<Endptctrl7Txi> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txi) -> u8 {
        Endptctrl7Txi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txr {
    _RESERVED_0 = 0x0,
    #[doc = "Reset PID sequence"]
    W_1 = 0x01,
}
impl Endptctrl7Txr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txr {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txr {
        Endptctrl7Txr::from_bits(val)
    }
}
impl From<Endptctrl7Txr> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txr) -> u8 {
        Endptctrl7Txr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txs {
    #[doc = "Endpoint OK"]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled"]
    ENABLE = 0x01,
}
impl Endptctrl7Txs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txs {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txs {
        Endptctrl7Txs::from_bits(val)
    }
}
impl From<Endptctrl7Txs> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txs) -> u8 {
        Endptctrl7Txs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl7Txt {
    #[doc = "Control"]
    CTL = 0x0,
    #[doc = "Isochronous"]
    ISO = 0x01,
    #[doc = "Bulk"]
    BLK = 0x02,
    #[doc = "Interrupt"]
    IRQ = 0x03,
}
impl Endptctrl7Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl7Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl7Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl7Txt {
        Endptctrl7Txt::from_bits(val)
    }
}
impl From<Endptctrl7Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl7Txt) -> u8 {
        Endptctrl7Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Es {
    #[doc = "Little Endian"]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big Endian"]
    BIG_ENDIAN = 0x01,
}
impl Es {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Es {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Es {
    #[inline(always)]
    fn from(val: u8) -> Es {
        Es::from_bits(val)
    }
}
impl From<Es> for u8 {
    #[inline(always)]
    fn from(val: Es) -> u8 {
        Es::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fpr {
    #[doc = "No resume (K-state) detected/driven on port"]
    DISABLE = 0x0,
    #[doc = "Resume detected/driven on port"]
    ENABLE = 0x01,
}
impl Fpr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fpr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fpr {
    #[inline(always)]
    fn from(val: u8) -> Fpr {
        Fpr::from_bits(val)
    }
}
impl From<Fpr> for u8 {
    #[inline(always)]
    fn from(val: Fpr) -> u8 {
        Fpr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frindex(u16);
impl Frindex {
    #[doc = "(1024) 12"]
    pub const FRINDEX_1024: Self = Self(0x0);
    #[doc = "(512) 11"]
    pub const FRINDEX_512: Self = Self(0x01);
    #[doc = "(256) 10"]
    pub const FRINDEX_256: Self = Self(0x02);
    #[doc = "(128) 9"]
    pub const FRINDEX_128: Self = Self(0x03);
    #[doc = "(64) 8"]
    pub const FRINDEX_64: Self = Self(0x04);
    #[doc = "(32) 7"]
    pub const FRINDEX_32: Self = Self(0x05);
    #[doc = "(16) 6"]
    pub const FRINDEX_16: Self = Self(0x06);
    #[doc = "(8) 5"]
    pub const FRINDEX_8: Self = Self(0x07);
}
impl Frindex {
    pub const fn from_bits(val: u16) -> Frindex {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Frindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FRINDEX_1024"),
            0x01 => f.write_str("FRINDEX_512"),
            0x02 => f.write_str("FRINDEX_256"),
            0x03 => f.write_str("FRINDEX_128"),
            0x04 => f.write_str("FRINDEX_64"),
            0x05 => f.write_str("FRINDEX_32"),
            0x06 => f.write_str("FRINDEX_16"),
            0x07 => f.write_str("FRINDEX_8"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frindex {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FRINDEX_1024"),
            0x01 => defmt::write!(f, "FRINDEX_512"),
            0x02 => defmt::write!(f, "FRINDEX_256"),
            0x03 => defmt::write!(f, "FRINDEX_128"),
            0x04 => defmt::write!(f, "FRINDEX_64"),
            0x05 => defmt::write!(f, "FRINDEX_32"),
            0x06 => defmt::write!(f, "FRINDEX_16"),
            0x07 => defmt::write!(f, "FRINDEX_8"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Frindex {
    #[inline(always)]
    fn from(val: u16) -> Frindex {
        Frindex::from_bits(val)
    }
}
impl From<Frindex> for u16 {
    #[inline(always)]
    fn from(val: Frindex) -> u16 {
        Frindex::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptmode {
    #[doc = "One Shot Mode"]
    ONE_SHOT = 0x0,
    #[doc = "Repeat Mode"]
    REPEAT = 0x01,
}
impl Gptimer0ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptmode {
        Gptimer0ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptmode) -> u8 {
        Gptimer0ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrst {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD"]
    LOAD_CNTR = 0x01,
}
impl Gptimer0ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrst {
        Gptimer0ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrst) -> u8 {
        Gptimer0ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrun {
    #[doc = "Stop counting"]
    STOP_CNTR = 0x0,
    #[doc = "Run"]
    RUN = 0x01,
}
impl Gptimer0ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrun {
        Gptimer0ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrun) -> u8 {
        Gptimer0ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptmode {
    #[doc = "One Shot Mode"]
    ONE_SHOT = 0x0,
    #[doc = "Repeat Mode"]
    REPEAT = 0x01,
}
impl Gptimer1ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptmode {
        Gptimer1ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptmode) -> u8 {
        Gptimer1ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrst {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
    LOAD_CNTR = 0x01,
}
impl Gptimer1ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrst {
        Gptimer1ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrst) -> u8 {
        Gptimer1ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrun {
    #[doc = "Stop counting"]
    STOP_CNTR = 0x0,
    #[doc = "Run"]
    RUN = 0x01,
}
impl Gptimer1ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrun {
        Gptimer1ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrun) -> u8 {
        Gptimer1ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc {
    #[doc = "Not supported"]
    HOST_OP_DIS = 0x0,
    #[doc = "Supported"]
    HOST_OP_EN = 0x01,
}
impl Hc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc {
    #[inline(always)]
    fn from(val: u8) -> Hc {
        Hc::from_bits(val)
    }
}
impl From<Hc> for u8 {
    #[inline(always)]
    fn from(val: Hc) -> u8 {
        Hc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Id {
    #[doc = "A device"]
    DISABLE = 0x0,
    #[doc = "B device"]
    ENABLE = 0x01,
}
impl Id {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Id {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Id {
    #[inline(always)]
    fn from(val: u8) -> Id {
        Id::from_bits(val)
    }
}
impl From<Id> for u8 {
    #[inline(always)]
    fn from(val: Id) -> u8 {
        Id::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idpu {
    #[doc = "Off"]
    DISABLE = 0x0,
    #[doc = "On"]
    ENABLE = 0x01,
}
impl Idpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idpu {
    #[inline(always)]
    fn from(val: u8) -> Idpu {
        Idpu::from_bits(val)
    }
}
impl From<Idpu> for u8 {
    #[inline(always)]
    fn from(val: Idpu) -> u8 {
        Idpu::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Itc(u8);
impl Itc {
    #[doc = "Immediate (no threshold)"]
    pub const IMMEDIATE: Self = Self(0x0);
    #[doc = "1 micro-frame"]
    pub const MICROFRAME_1: Self = Self(0x01);
    #[doc = "2 micro-frames"]
    pub const MICROFRAME_2: Self = Self(0x02);
    #[doc = "4 micro-frames"]
    pub const MICROFRAME_4: Self = Self(0x04);
    #[doc = "8 micro-frames"]
    pub const MICROFRAME_8: Self = Self(0x08);
    #[doc = "16 micro-frames"]
    pub const MICROFRAME_16: Self = Self(0x10);
    #[doc = "32 micro-frames"]
    pub const MICROFRAME_32: Self = Self(0x20);
    #[doc = "64 micro-frames"]
    pub const MICROFRAME_64: Self = Self(0x40);
}
impl Itc {
    pub const fn from_bits(val: u8) -> Itc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Itc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IMMEDIATE"),
            0x01 => f.write_str("MICROFRAME_1"),
            0x02 => f.write_str("MICROFRAME_2"),
            0x04 => f.write_str("MICROFRAME_4"),
            0x08 => f.write_str("MICROFRAME_8"),
            0x10 => f.write_str("MICROFRAME_16"),
            0x20 => f.write_str("MICROFRAME_32"),
            0x40 => f.write_str("MICROFRAME_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IMMEDIATE"),
            0x01 => defmt::write!(f, "MICROFRAME_1"),
            0x02 => defmt::write!(f, "MICROFRAME_2"),
            0x04 => defmt::write!(f, "MICROFRAME_4"),
            0x08 => defmt::write!(f, "MICROFRAME_8"),
            0x10 => defmt::write!(f, "MICROFRAME_16"),
            0x20 => defmt::write!(f, "MICROFRAME_32"),
            0x40 => defmt::write!(f, "MICROFRAME_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Itc {
    #[inline(always)]
    fn from(val: u8) -> Itc {
        Itc::from_bits(val)
    }
}
impl From<Itc> for u8 {
    #[inline(always)]
    fn from(val: Itc) -> u8 {
        Itc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ls {
    #[doc = "SE0"]
    SE0 = 0x0,
    #[doc = "K-state"]
    K_STATE = 0x01,
    #[doc = "J-state"]
    J_STATE = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Ls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ls {
    #[inline(always)]
    fn from(val: u8) -> Ls {
        Ls::from_bits(val)
    }
}
impl From<Ls> for u8 {
    #[inline(always)]
    fn from(val: Ls) -> u8 {
        Ls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NCc {
    #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported"]
    NO_COMP_CONTROLLER = 0x0,
    #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported"]
    COMP_CONTROLLER = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl NCc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NCc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NCc {
    #[inline(always)]
    fn from(val: u8) -> NCc {
        NCc::from_bits(val)
    }
}
impl From<NCc> for u8 {
    #[inline(always)]
    fn from(val: NCc) -> u8 {
        NCc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oca {
    #[doc = "This port does not have an over-current condition"]
    NO_OVERCURRENT = 0x0,
    #[doc = "This port currently has an over-current condition"]
    OVERCURRENT = 0x01,
}
impl Oca {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oca {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oca {
    #[inline(always)]
    fn from(val: u8) -> Oca {
        Oca::from_bits(val)
    }
}
impl From<Oca> for u8 {
    #[inline(always)]
    fn from(val: Oca) -> u8 {
        Oca::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Pe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe {
    #[inline(always)]
    fn from(val: u8) -> Pe {
        Pe::from_bits(val)
    }
}
impl From<Pe> for u8 {
    #[inline(always)]
    fn from(val: Pe) -> u8 {
        Pe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pec {
    #[doc = "No change"]
    DISABLE = 0x0,
    #[doc = "Port enabled/disabled status has changed"]
    ENABLE = 0x01,
}
impl Pec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pec {
    #[inline(always)]
    fn from(val: u8) -> Pec {
        Pec::from_bits(val)
    }
}
impl From<Pec> for u8 {
    #[inline(always)]
    fn from(val: Pec) -> u8 {
        Pec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsc {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "Forced to full speed"]
    FULL_SPEED = 0x01,
}
impl Pfsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsc {
    #[inline(always)]
    fn from(val: u8) -> Pfsc {
        Pfsc::from_bits(val)
    }
}
impl From<Pfsc> for u8 {
    #[inline(always)]
    fn from(val: Pfsc) -> u8 {
        Pfsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phcd {
    #[doc = "Enable PHY clock"]
    PHY_CLK_EN = 0x0,
    #[doc = "Disable PHY clock"]
    PHY_CLK_DIS = 0x01,
}
impl Phcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phcd {
    #[inline(always)]
    fn from(val: u8) -> Phcd {
        Phcd::from_bits(val)
    }
}
impl From<Phcd> for u8 {
    #[inline(always)]
    fn from(val: Phcd) -> u8 {
        Phcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phym {
    #[doc = "UTMI/UMTI+"]
    UTMI = 0x0,
    #[doc = "ULPI DDR"]
    ULPI_DDR = 0x01,
    #[doc = "ULPI"]
    ULPI = 0x02,
    #[doc = "Serial Only"]
    SERIAL = 0x03,
    #[doc = "Software programmable - reset to UTMI/UTMI+"]
    SW_RST_UTMI = 0x04,
    #[doc = "Software programmable - reset to ULPI DDR"]
    SW_RST_ULPI_DDR = 0x05,
    #[doc = "Software programmable - reset to ULPI"]
    SW_RST_ULPI = 0x06,
    #[doc = "Software programmable - reset to Serial"]
    SW_RST_SERIAL = 0x07,
}
impl Phym {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phym {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phym {
    #[inline(always)]
    fn from(val: u8) -> Phym {
        Phym::from_bits(val)
    }
}
impl From<Phym> for u8 {
    #[inline(always)]
    fn from(val: Phym) -> u8 {
        Phym::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyw {
    #[doc = "8 bit wide data bus (Software non-programmable)"]
    DATA_BUS_8 = 0x0,
    #[doc = "16 bit wide data bus (Software non-programmable)"]
    DATA_BUS_16 = 0x01,
    #[doc = "Reset to 8 bit wide data bus (Software programmable)"]
    SW_RST_8 = 0x02,
    #[doc = "Reset to 16 bit wide data bus (Software programmable)"]
    SW_RST_16 = 0x03,
}
impl Phyw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyw {
    #[inline(always)]
    fn from(val: u8) -> Phyw {
        Phyw::from_bits(val)
    }
}
impl From<Phyw> for u8 {
    #[inline(always)]
    fn from(val: Phyw) -> u8 {
        Phyw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pic {
    #[doc = "Port indicators are off"]
    PORT_INDICATOR_OFF = 0x0,
    #[doc = "Amber"]
    PORT_IND_AMBER = 0x01,
    #[doc = "Green"]
    PORT_IND_GREEN = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Pic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pic {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pic {
    #[inline(always)]
    fn from(val: u8) -> Pic {
        Pic::from_bits(val)
    }
}
impl From<Pic> for u8 {
    #[inline(always)]
    fn from(val: Pic) -> u8 {
        Pic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pr {
    #[doc = "Port is not in reset"]
    DISABLE = 0x0,
    #[doc = "Port is in reset"]
    ENABLE = 0x01,
}
impl Pr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pr {
    #[inline(always)]
    fn from(val: u8) -> Pr {
        Pr::from_bits(val)
    }
}
impl From<Pr> for u8 {
    #[inline(always)]
    fn from(val: Pr) -> u8 {
        Pr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pse {
    #[doc = "Do not process the Periodic Schedule"]
    DONT_PROCESS_PT = 0x0,
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule"]
    PROCESS_PT_PERIODICLISTBASE = 0x01,
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
pub enum Pspd {
    #[doc = "Full Speed"]
    FS = 0x0,
    #[doc = "Low Speed"]
    LS = 0x01,
    #[doc = "High Speed"]
    HS = 0x02,
    #[doc = "Undefined"]
    UNDEFINED = 0x03,
}
impl Pspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pspd {
    #[inline(always)]
    fn from(val: u8) -> Pspd {
        Pspd::from_bits(val)
    }
}
impl From<Pspd> for u8 {
    #[inline(always)]
    fn from(val: Pspd) -> u8 {
        Pspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptc {
    #[doc = "TEST_MODE_DISABLE"]
    TST_MODE_DIS = 0x0,
    #[doc = "J_STATE"]
    J_STATE = 0x01,
    #[doc = "K_STATE"]
    K_STATE = 0x02,
    #[doc = "SE0 (host) / NAK (device)"]
    SE0 = 0x03,
    #[doc = "Packet"]
    PCKT = 0x04,
    #[doc = "FORCE_ENABLE_HS"]
    HS = 0x05,
    #[doc = "FORCE_ENABLE_FS"]
    FS = 0x06,
    #[doc = "FORCE_ENABLE_LS"]
    LS = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ptc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptc {
    #[inline(always)]
    fn from(val: u8) -> Ptc {
        Ptc::from_bits(val)
    }
}
impl From<Ptc> for u8 {
    #[inline(always)]
    fn from(val: Ptc) -> u8 {
        Ptc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptw {
    #[doc = "Select the 8-bit UTMI interface \\[60 MHz\\]"]
    UTMI_8 = 0x0,
    #[doc = "Select the 16-bit UTMI interface \\[30 MHz\\]"]
    UTMI_16 = 0x01,
}
impl Ptw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptw {
    #[inline(always)]
    fn from(val: u8) -> Ptw {
        Ptw::from_bits(val)
    }
}
impl From<Ptw> for u8 {
    #[inline(always)]
    fn from(val: Ptw) -> u8 {
        Ptw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rs {
    #[doc = "Stop"]
    DISABLE = 0x0,
    #[doc = "Run"]
    ENABLE = 0x01,
}
impl Rs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rs {
    #[inline(always)]
    fn from(val: u8) -> Rs {
        Rs::from_bits(val)
    }
}
impl From<Rs> for u8 {
    #[inline(always)]
    fn from(val: Rs) -> u8 {
        Rs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdis {
    #[doc = "Inactive"]
    DISABLE = 0x0,
    #[doc = "Active"]
    ENABLE = 0x01,
}
impl Sdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdis {
    #[inline(always)]
    fn from(val: u8) -> Sdis {
        Sdis::from_bits(val)
    }
}
impl From<Sdis> for u8 {
    #[inline(always)]
    fn from(val: Sdis) -> u8 {
        Sdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slom {
    #[doc = "Setup Lockouts On (default);"]
    LOCKOUT_ON = 0x0,
    #[doc = "Setup Lockouts Off"]
    LOCKOUT_OFF = 0x01,
}
impl Slom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slom {
    #[inline(always)]
    fn from(val: u8) -> Slom {
        Slom::from_bits(val)
    }
}
impl From<Slom> for u8 {
    #[inline(always)]
    fn from(val: Slom) -> u8 {
        Slom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm {
    #[doc = "No Serial Engine, always use parallel signalling"]
    SERIAL_ENGINE_NO = 0x0,
    #[doc = "Serial Engine present, always use serial signalling for FS/LS"]
    SERIAL_ENGINE_EN = 0x01,
    #[doc = "Software programmable - Reset to use parallel signalling for FS/LS"]
    SW_RST_PARALLEL = 0x02,
    #[doc = "Software programmable - Reset to use serial signalling for FS/LS"]
    SW_RST_SERIAL_ENG = 0x03,
}
impl Sm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm {
    #[inline(always)]
    fn from(val: u8) -> Sm {
        Sm::from_bits(val)
    }
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(val: Sm) -> u8 {
        Sm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sts {
    #[doc = "Parallel Interface signals is selected"]
    DISABLE = 0x0,
    #[doc = "Serial Interface Engine is selected"]
    ENABLE = 0x01,
}
impl Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sts {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sts {
    #[inline(always)]
    fn from(val: u8) -> Sts {
        Sts::from_bits(val)
    }
}
impl From<Sts> for u8 {
    #[inline(always)]
    fn from(val: Sts) -> u8 {
        Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Susp {
    #[doc = "Port not in suspend state"]
    DISABLE = 0x0,
    #[doc = "Port in suspend state"]
    ENABLE = 0x01,
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
