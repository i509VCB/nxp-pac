#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Contc {
    #[doc = "Command word for start of new transfer."]
    Start = 0x0,
    #[doc = "Command word for continuing transfer."]
    Continue = 0x01,
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
    #[doc = "Captured."]
    Captured = 0x0,
    #[doc = "Changed."]
    Changed = 0x01,
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
    #[doc = "The inactive state value of SCK is low."]
    InactiveLow = 0x0,
    #[doc = "The inactive state value of SCK is high."]
    InactiveHigh = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "LPSPI module is enabled in Doze mode."]
    Enabled = 0x0,
    #[doc = "LPSPI module is disabled in Doze mode."]
    Disabled = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set supporting a 32-bit shift register."]
    pub const Standard: Self = Self(0x04);
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
            0x04 => f.write_str("Standard"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x04 => defmt::write!(f, "Standard"),
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
pub enum Hrsel {
    #[doc = "Host request input is the HREQ pin."]
    Hreqpin = 0x0,
    #[doc = "Host request input is the input trigger."]
    InputTrigger = 0x01,
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
    #[doc = "Data is transferred MSB first."]
    MsbFirst = 0x0,
    #[doc = "Data is transferred LSB first."]
    LsbFirst = 0x01,
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
    #[doc = "Slave mode."]
    SlaveMode = 0x0,
    #[doc = "Master mode."]
    MasterMode = 0x01,
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
    #[doc = "Match is disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled is 1st data word is MATCH0 or MATCH1."]
    EnabledFirstdatamatch = 0x02,
    #[doc = "Match is enabled on any data word equal MATCH0 or MATCH1."]
    EnabledAnydatamatch = 0x03,
    #[doc = "Match is enabled on data match sequence."]
    EnabledDatamatch100 = 0x04,
    #[doc = "Match is enabled on data match sequence."]
    EnabledDatamatch101 = 0x05,
    #[doc = "Match is enabled."]
    EnabledDatamatch110 = 0x06,
    #[doc = "Match is enabled."]
    EnabledDatamatch111 = 0x07,
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
    #[doc = "LPSPI is idle."]
    Idle = 0x0,
    #[doc = "LPSPI is busy."]
    Busy = 0x01,
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
    #[doc = "Output data retains last value when chip select is negated."]
    RetainLastvalue = 0x0,
    #[doc = "Output data is tristated when chip select is negated."]
    Tristated = 0x01,
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
pub enum Pcs {
    #[doc = "Transfer using PCS\\[0\\]."]
    TxPcs0 = 0x0,
    #[doc = "Transfer using PCS\\[1\\]."]
    TxPcs1 = 0x01,
    #[doc = "Transfer using PCS\\[2\\]."]
    TxPcs2 = 0x02,
    #[doc = "Transfer using PCS\\[3\\]."]
    TxPcs3 = 0x03,
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
    #[doc = "PCS\\[3:2\\] are configured for chip select function."]
    ChipSelect = 0x0,
    #[doc = "PCS\\[3:2\\] are configured for half-duplex 4-bit transfers (PCS\\[3:2\\] = DATA\\[3:2\\])."]
    Halfduplex4bit = 0x01,
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
pub enum Pincfg {
    #[doc = "SIN is used for input data and SOUT is used for output data."]
    SinInSoutOut = 0x0,
    #[doc = "SIN is used for both input and output data, only half-duplex serial transfers are supported."]
    SinBothInOut = 0x01,
    #[doc = "SOUT is used for both input and output data, only half-duplex serial transfers are supported."]
    SoutBothInOut = 0x02,
    #[doc = "SOUT is used for input data and SIN is used for output data."]
    SoutInSinOut = 0x03,
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
    #[doc = "Divide by 1."]
    Divideby1 = 0x0,
    #[doc = "Divide by 2."]
    Divideby2 = 0x01,
    #[doc = "Divide by 4."]
    Divideby4 = 0x02,
    #[doc = "Divide by 8."]
    Divideby8 = 0x03,
    #[doc = "Divide by 16."]
    Divideby16 = 0x04,
    #[doc = "Divide by 32."]
    Divideby32 = 0x05,
    #[doc = "Divide by 64."]
    Divideby64 = 0x06,
    #[doc = "Divide by 128."]
    Divideby128 = 0x07,
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
    #[doc = "Received data is stored in the receive FIFO as in normal operations."]
    Stored = 0x0,
    #[doc = "Received data is discarded unless the SR\\[DMF\\] = 1."]
    Discarded = 0x01,
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
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset the Receive FIFO. The register bit always reads zero."]
    RxfifoRst = 0x01,
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
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset the Transmit FIFO. The register bit always reads zero."]
    TxfifoRst = 0x01,
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
    #[doc = "Normal transfer."]
    Normal = 0x0,
    #[doc = "Receive data is masked."]
    Mask = 0x01,
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
    #[doc = "Input data is sampled on SCK edge."]
    OnSckEdge = 0x0,
    #[doc = "Input data is sampled on delayed SCK edge."]
    OnDelayedSckEdge = 0x01,
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
    #[doc = "Subsequent data word received after PCS assertion."]
    NextDataword = 0x0,
    #[doc = "First data word received after PCS assertion."]
    FirstDataword = 0x01,
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
    #[doc = "Transmit data not requested."]
    TxdataNotReqst = 0x0,
    #[doc = "Transmit data is requested."]
    TxdataReqst = 0x01,
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
    #[doc = "Normal transfer."]
    Normal = 0x0,
    #[doc = "Mask transmit data."]
    Mask = 0x01,
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
    #[doc = "1 bit transfer."]
    Onebit = 0x0,
    #[doc = "2 bit transfer."]
    Twobit = 0x01,
    #[doc = "4 bit transfer."]
    Fourbit = 0x02,
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
