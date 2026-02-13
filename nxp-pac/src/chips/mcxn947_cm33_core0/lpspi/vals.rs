#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Contc {
    #[doc = "Command word for start of new transfer"]
    START = 0x0,
    #[doc = "Command word for continuing transfer"]
    CONTINUE = 0x01,
}
impl Contc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Contc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Contc {
    #[inline(always)]
    fn from(val: u8) -> Contc {
        Contc::from_bits(val)
    }
}
impl From<Contc> for u8 {
    #[inline(always)]
    fn from(val: Contc) -> u8 {
        Contc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpha {
    #[doc = "Captured"]
    CAPTURED = 0x0,
    #[doc = "Changed"]
    CHANGED = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "Inactive low"]
    INACTIVE_LOW = 0x0,
    #[doc = "Inactive high"]
    INACTIVE_HIGH = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set supporting a 32-bit shift register."]
    pub const STANDARD: Self = Self(0x04);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x04 => f.write_str("STANDARD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x04 => defmt::write!(f, "STANDARD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrdir {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Hrdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrdir {
    #[inline(always)]
    fn from(val: u8) -> Hrdir {
        Hrdir::from_bits(val)
    }
}
impl From<Hrdir> for u8 {
    #[inline(always)]
    fn from(val: Hrdir) -> u8 {
        Hrdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrsel {
    #[doc = "HREQ pin"]
    HREQPIN = 0x0,
    #[doc = "Input trigger"]
    INPUT_TRIGGER = 0x01,
}
impl Hrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrsel {
    #[inline(always)]
    fn from(val: u8) -> Hrsel {
        Hrsel::from_bits(val)
    }
}
impl From<Hrsel> for u8 {
    #[inline(always)]
    fn from(val: Hrsel) -> u8 {
        Hrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsbf {
    #[doc = "MSB first"]
    MSB_FIRST = 0x0,
    #[doc = "LSB first"]
    LSB_FIRST = 0x01,
}
impl Lsbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsbf {
    #[inline(always)]
    fn from(val: u8) -> Lsbf {
        Lsbf::from_bits(val)
    }
}
impl From<Lsbf> for u8 {
    #[inline(always)]
    fn from(val: Lsbf) -> u8 {
        Lsbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Slave mode"]
    SLAVE_MODE = 0x0,
    #[doc = "Master mode"]
    MASTER_MODE = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Match is disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match first data word with compare word"]
    ENABLED_FIRSTDATAMATCH = 0x02,
    #[doc = "Match any data word with compare word"]
    ENABLED_ANYDATAMATCH = 0x03,
    #[doc = "Sequential match, first data word"]
    ENABLED_DATAMATCH_100 = 0x04,
    #[doc = "Sequential match, any data word"]
    ENABLED_DATAMATCH_101 = 0x05,
    #[doc = "Match first data word (masked) with compare word (masked)"]
    ENABLED_DATAMATCH_110 = 0x06,
    #[doc = "Match any data word (masked) with compare word (masked)"]
    ENABLED_DATAMATCH_111 = 0x07,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbf {
    #[doc = "LPSPI is idle"]
    IDLE = 0x0,
    #[doc = "LPSPI is busy"]
    BUSY = 0x01,
}
impl Mbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbf {
    #[inline(always)]
    fn from(val: u8) -> Mbf {
        Mbf::from_bits(val)
    }
}
impl From<Mbf> for u8 {
    #[inline(always)]
    fn from(val: Mbf) -> u8 {
        Mbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outcfg {
    #[doc = "Retain last value"]
    RETAIN_LASTVALUE = 0x0,
    #[doc = "3-stated"]
    TRISTATED = 0x01,
}
impl Outcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outcfg {
    #[inline(always)]
    fn from(val: u8) -> Outcfg {
        Outcfg::from_bits(val)
    }
}
impl From<Outcfg> for u8 {
    #[inline(always)]
    fn from(val: Outcfg) -> u8 {
        Outcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Partial {
    #[doc = "Discard"]
    DISCARDED = 0x0,
    #[doc = "Store"]
    STORED = 0x01,
}
impl Partial {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Partial {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Partial {
    #[inline(always)]
    fn from(val: u8) -> Partial {
        Partial::from_bits(val)
    }
}
impl From<Partial> for u8 {
    #[inline(always)]
    fn from(val: Partial) -> u8 {
        Partial::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcs {
    #[doc = "Transfer using PCS\\[0\\]"]
    TX_PCS0 = 0x0,
    #[doc = "Transfer using PCS\\[1\\]"]
    TX_PCS1 = 0x01,
    #[doc = "Transfer using PCS\\[2\\]"]
    TX_PCS2 = 0x02,
    #[doc = "Transfer using PCS\\[3\\]"]
    TX_PCS3 = 0x03,
}
impl Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcs {
    #[inline(always)]
    fn from(val: u8) -> Pcs {
        Pcs::from_bits(val)
    }
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(val: Pcs) -> u8 {
        Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcscfg {
    #[doc = "PCS\\[3:2\\] configured for chip select function"]
    CHIP_SELECT = 0x0,
    #[doc = "PCS\\[3:2\\] configured for half-duplex 4-bit transfers (PCS\\[3:2\\] = DATA\\[3:2\\])"]
    HALFDUPLEX4BIT = 0x01,
}
impl Pcscfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcscfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcscfg {
    #[inline(always)]
    fn from(val: u8) -> Pcscfg {
        Pcscfg::from_bits(val)
    }
}
impl From<Pcscfg> for u8 {
    #[inline(always)]
    fn from(val: Pcscfg) -> u8 {
        Pcscfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcspol {
    #[doc = "Active low"]
    DISCARDED = 0x0,
    #[doc = "Active high"]
    STORED = 0x01,
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
impl Pcspol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcspol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcspol {
    #[inline(always)]
    fn from(val: u8) -> Pcspol {
        Pcspol::from_bits(val)
    }
}
impl From<Pcspol> for u8 {
    #[inline(always)]
    fn from(val: Pcspol) -> u8 {
        Pcspol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pincfg {
    #[doc = "SIN is used for input data; SOUT is used for output data"]
    SIN_IN_SOUT_OUT = 0x0,
    #[doc = "SIN is used for both input and output data; only half-duplex serial transfers are supported"]
    SIN_BOTH_IN_OUT = 0x01,
    #[doc = "SOUT is used for both input and output data; only half-duplex serial transfers are supported"]
    SOUT_BOTH_IN_OUT = 0x02,
    #[doc = "SOUT is used for input data; SIN is used for output data"]
    SOUT_IN_SIN_OUT = 0x03,
}
impl Pincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pincfg {
    #[inline(always)]
    fn from(val: u8) -> Pincfg {
        Pincfg::from_bits(val)
    }
}
impl From<Pincfg> for u8 {
    #[inline(always)]
    fn from(val: Pincfg) -> u8 {
        Pincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "Divide by 1"]
    DIVIDEBY1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDEBY2 = 0x01,
    #[doc = "Divide by 4"]
    DIVIDEBY4 = 0x02,
    #[doc = "Divide by 8"]
    DIVIDEBY8 = 0x03,
    #[doc = "Divide by 16"]
    DIVIDEBY16 = 0x04,
    #[doc = "Divide by 32"]
    DIVIDEBY32 = 0x05,
    #[doc = "Divide by 64"]
    DIVIDEBY64 = 0x06,
    #[doc = "Divide by 128"]
    DIVIDEBY128 = 0x07,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdmo {
    #[doc = "Disable"]
    STORED = 0x0,
    #[doc = "Enable"]
    DISCARDED = 0x01,
}
impl Rdmo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdmo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdmo {
    #[inline(always)]
    fn from(val: u8) -> Rdmo {
        Rdmo::from_bits(val)
    }
}
impl From<Rdmo> for u8 {
    #[inline(always)]
    fn from(val: Rdmo) -> u8 {
        Rdmo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Reset"]
    RXFIFO_RST = 0x01,
}
impl Rrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrf {
    #[inline(always)]
    fn from(val: u8) -> Rrf {
        Rrf::from_bits(val)
    }
}
impl From<Rrf> for u8 {
    #[inline(always)]
    fn from(val: Rrf) -> u8 {
        Rrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Reset"]
    TXFIFO_RST = 0x01,
}
impl Rtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtf {
    #[inline(always)]
    fn from(val: u8) -> Rtf {
        Rtf::from_bits(val)
    }
}
impl From<Rtf> for u8 {
    #[inline(always)]
    fn from(val: Rtf) -> u8 {
        Rtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxmsk {
    #[doc = "Normal transfer"]
    NORMAL = 0x0,
    #[doc = "Mask receive data"]
    MASK = 0x01,
}
impl Rxmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxmsk {
    #[inline(always)]
    fn from(val: u8) -> Rxmsk {
        Rxmsk::from_bits(val)
    }
}
impl From<Rxmsk> for u8 {
    #[inline(always)]
    fn from(val: Rxmsk) -> u8 {
        Rxmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sample {
    #[doc = "SCK edge"]
    ON_SCK_EDGE = 0x0,
    #[doc = "Delayed SCK edge"]
    ON_DELAYED_SCK_EDGE = 0x01,
}
impl Sample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sample {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sample {
    #[inline(always)]
    fn from(val: u8) -> Sample {
        Sample::from_bits(val)
    }
}
impl From<Sample> for u8 {
    #[inline(always)]
    fn from(val: Sample) -> u8 {
        Sample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sof {
    #[doc = "Subsequent data word"]
    NEXT_DATAWORD = 0x0,
    #[doc = "First data word"]
    FIRST_DATAWORD = 0x01,
}
impl Sof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sof {
    #[inline(always)]
    fn from(val: u8) -> Sof {
        Sof::from_bits(val)
    }
}
impl From<Sof> for u8 {
    #[inline(always)]
    fn from(val: Sof) -> u8 {
        Sof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdf {
    #[doc = "Transmit data not requested"]
    TXDATA_NOT_REQST = 0x0,
    #[doc = "Transmit data requested"]
    TXDATA_REQST = 0x01,
}
impl Tdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdf {
    #[inline(always)]
    fn from(val: u8) -> Tdf {
        Tdf::from_bits(val)
    }
}
impl From<Tdf> for u8 {
    #[inline(always)]
    fn from(val: Tdf) -> u8 {
        Tdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txmsk {
    #[doc = "Normal transfer"]
    NORMAL = 0x0,
    #[doc = "Mask transmit data"]
    MASK = 0x01,
}
impl Txmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txmsk {
    #[inline(always)]
    fn from(val: u8) -> Txmsk {
        Txmsk::from_bits(val)
    }
}
impl From<Txmsk> for u8 {
    #[inline(always)]
    fn from(val: Txmsk) -> u8 {
        Txmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    #[doc = "1-bit transfer"]
    ONEBIT = 0x0,
    #[doc = "2-bit transfer"]
    TWOBIT = 0x01,
    #[doc = "4-bit transfer"]
    FOURBIT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
