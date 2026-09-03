#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrcfg {
    #[doc = "Address match 0 (7-bit)."]
    AddressMatch07Bit = 0x0,
    #[doc = "Address match 0 (10-bit)."]
    AddressMatch010Bit = 0x01,
    #[doc = "Address match 0 (7-bit) or address match 1 (7-bit)."]
    AddressMatch07BitOrAddressMatch17Bit = 0x02,
    #[doc = "Address match 0 (10-bit) or address match 1 (10-bit)."]
    AddressMatch010BitOrAddressMatch110Bit = 0x03,
    #[doc = "Address match 0 (7-bit) or address match 1 (10-bit)."]
    AddressMatch07BitOrAddressMatch110Bit = 0x04,
    #[doc = "Address match 0 (10-bit) or address match 1 (7-bit)."]
    AddressMatch010BitOrAddressMatch17Bit = 0x05,
    #[doc = "From address match 0 (7-bit) to address match 1 (7-bit)."]
    FromAddressMatch07BitToAddressMatch17Bit = 0x06,
    #[doc = "From address match 0 (10-bit) to address match 1 (10-bit)."]
    FromAddressMatch010BitToAddressMatch110Bit = 0x07,
}
impl Addrcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addrcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addrcfg {
    #[inline(always)]
    fn from(val: u8) -> Addrcfg {
        Addrcfg::from_bits(val)
    }
}
impl From<Addrcfg> for u8 {
    #[inline(always)]
    fn from(val: Addrcfg) -> u8 {
        Addrcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alf {
    #[doc = "Controller did not lose arbitration."]
    IntNo = 0x0,
    #[doc = "Controller lost arbitration."]
    IntYes = 0x01,
}
impl Alf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alf {
    #[inline(always)]
    fn from(val: u8) -> Alf {
        Alf::from_bits(val)
    }
}
impl From<Alf> for u8 {
    #[inline(always)]
    fn from(val: Alf) -> u8 {
        Alf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Anv {
    #[doc = "Valid."]
    Valid = 0x0,
    #[doc = "Not valid."]
    NotValid = 0x01,
}
impl Anv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Anv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Anv {
    #[inline(always)]
    fn from(val: u8) -> Anv {
        Anv::from_bits(val)
    }
}
impl From<Anv> for u8 {
    #[inline(always)]
    fn from(val: Anv) -> u8 {
        Anv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bef {
    #[doc = "No bit error occurred."]
    IntNo = 0x0,
    #[doc = "Bit error occurred."]
    IntYes = 0x01,
}
impl Bef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bef {
    #[inline(always)]
    fn from(val: u8) -> Bef {
        Bef::from_bits(val)
    }
}
impl From<Bef> for u8 {
    #[inline(always)]
    fn from(val: Bef) -> u8 {
        Bef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "Transmit the value in DATA\\[7:0\\]."]
    TransmitData7Through0 = 0x0,
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes."]
    ReceiveData7Through0PlusOne = 0x01,
    #[doc = "Generate Stop condition on I2C bus."]
    GenerateStopCondition = 0x02,
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes."]
    ReceiveAndDiscardData7Through0PlusOne = 0x03,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]."]
    GenerateStartAndTransmitAddressInData7Through0 = 0x04,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] (this transfer expects a NACK to be returned)."]
    GenerateStartAndTransmitAddressInData7Through0ExpectNack = 0x05,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode."]
    GenerateStartAndTransmitAddressInData7Through0UsingHighSpeedMode = 0x06,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode (this transfer expects a NACK to be returned)."]
    GenerateStartAndTransmitAddressInData7Through0UsingHighSpeedModeExpectNack = 0x07,
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmf {
    #[doc = "Matching data not received."]
    IntNo = 0x0,
    #[doc = "Matching data received."]
    IntYes = 0x01,
}
impl Dmf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmf {
    #[inline(always)]
    fn from(val: u8) -> Dmf {
        Dmf::from_bits(val)
    }
}
impl From<Dmf> for u8 {
    #[inline(always)]
    fn from(val: Dmf) -> u8 {
        Dmf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable."]
    Enabled = 0x0,
    #[doc = "Disable."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epf {
    #[doc = "No Stop or repeated Start generated."]
    IntNo = 0x0,
    #[doc = "Stop or repeated Start generated."]
    IntYes = 0x01,
}
impl Epf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epf {
    #[inline(always)]
    fn from(val: u8) -> Epf {
        Epf::from_bits(val)
    }
}
impl From<Epf> for u8 {
    #[inline(always)]
    fn from(val: Epf) -> u8 {
        Epf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Controller only, with standard feature set."]
    pub const MasterOnly: Self = Self(0x02);
    #[doc = "Controller and target, with standard feature set."]
    pub const MasterAndSlave: Self = Self(0x03);
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
            0x02 => f.write_str("MasterOnly"),
            0x03 => f.write_str("MasterAndSlave"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x02 => defmt::write!(f, "MasterOnly"),
            0x03 => defmt::write!(f, "MasterAndSlave"),
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
pub enum Filtdz {
    #[doc = "Enable."]
    FilterEnabled = 0x0,
    #[doc = "Disable."]
    FilterDisabled = 0x01,
}
impl Filtdz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtdz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtdz {
    #[inline(always)]
    fn from(val: u8) -> Filtdz {
        Filtdz::from_bits(val)
    }
}
impl From<Filtdz> for u8 {
    #[inline(always)]
    fn from(val: Filtdz) -> u8 {
        Filtdz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrdir {
    #[doc = "HREQ pin is input (for LPI2C controller)."]
    Input = 0x0,
    #[doc = "HREQ pin is output (for LPI2C target)."]
    Output = 0x01,
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
pub enum Hrpol {
    #[doc = "Active low."]
    ActiveLow = 0x0,
    #[doc = "Active high."]
    ActiveHigh = 0x01,
}
impl Hrpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hrpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hrpol {
    #[inline(always)]
    fn from(val: u8) -> Hrpol {
        Hrpol::from_bits(val)
    }
}
impl From<Hrpol> for u8 {
    #[inline(always)]
    fn from(val: Hrpol) -> u8 {
        Hrpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Match is disabled."]
    Disabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled: first data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]."]
    FirstDataWordEqualsMatch0OrMatch1 = 0x02,
    #[doc = "Match is enabled: any data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]."]
    AnyDataWordEqualsMatch0OrMatch1 = 0x03,
    #[doc = "Match is enabled: (first data word equals MDMR\\[MATCH0\\]) AND (second data word equals MDMR\\[MATCH1)."]
    FirstDataWordMatch0AndSecondDataWordMatch1 = 0x04,
    #[doc = "Match is enabled: (any data word equals MDMR\\[MATCH0\\]) AND (next data word equals MDMR\\[MATCH1)."]
    AnyDataWordMatch0NextDataWordMatch1 = 0x05,
    #[doc = "Match is enabled: (first data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])."]
    FirstDataWordAndMatch1EqualsMatch0AndMatch1 = 0x06,
    #[doc = "Match is enabled: (any data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])."]
    AnyDataWordAndMatch1EqualsMatch0AndMatch1 = 0x07,
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
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
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
pub enum McrRrf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset receive FIFO."]
    Reset = 0x01,
}
impl McrRrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRrf {
    #[inline(always)]
    fn from(val: u8) -> McrRrf {
        McrRrf::from_bits(val)
    }
}
impl From<McrRrf> for u8 {
    #[inline(always)]
    fn from(val: McrRrf) -> u8 {
        McrRrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRtf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Reset transmit FIFO."]
    Reset = 0x01,
}
impl McrRtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRtf {
    #[inline(always)]
    fn from(val: u8) -> McrRtf {
        McrRtf::from_bits(val)
    }
}
impl From<McrRtf> for u8 {
    #[inline(always)]
    fn from(val: McrRtf) -> u8 {
        McrRtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrBbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl MsrBbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrBbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrBbf {
    #[inline(always)]
    fn from(val: u8) -> MsrBbf {
        MsrBbf::from_bits(val)
    }
}
impl From<MsrBbf> for u8 {
    #[inline(always)]
    fn from(val: MsrBbf) -> u8 {
        MsrBbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrFef {
    #[doc = "No FIFO error."]
    IntNo = 0x0,
    #[doc = "FIFO error."]
    IntYes = 0x01,
}
impl MsrFef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrFef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrFef {
    #[inline(always)]
    fn from(val: u8) -> MsrFef {
        MsrFef::from_bits(val)
    }
}
impl From<MsrFef> for u8 {
    #[inline(always)]
    fn from(val: MsrFef) -> u8 {
        MsrFef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrSdf {
    #[doc = "No Stop condition generated."]
    IntNo = 0x0,
    #[doc = "Stop condition generated."]
    IntYes = 0x01,
}
impl MsrSdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrSdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrSdf {
    #[inline(always)]
    fn from(val: u8) -> MsrSdf {
        MsrSdf::from_bits(val)
    }
}
impl From<MsrSdf> for u8 {
    #[inline(always)]
    fn from(val: MsrSdf) -> u8 {
        MsrSdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndf {
    #[doc = "No unexpected NACK detected."]
    IntNo = 0x0,
    #[doc = "Unexpected NACK detected."]
    IntYes = 0x01,
}
impl Ndf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndf {
    #[inline(always)]
    fn from(val: u8) -> Ndf {
        Ndf::from_bits(val)
    }
}
impl From<Ndf> for u8 {
    #[inline(always)]
    fn from(val: Ndf) -> u8 {
        Ndf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pincfg {
    #[doc = "Two-pin open drain mode."]
    OpenDrain2Pin = 0x0,
    #[doc = "Two-pin output only mode (Ultra-Fast mode)."]
    Output2PinOnly = 0x01,
    #[doc = "Two-pin push-pull mode."]
    PushPull2Pin = 0x02,
    #[doc = "Four-pin push-pull mode."]
    PushPull4Pin = 0x03,
    #[doc = "Two-pin open-drain mode with separate LPI2C target."]
    OpenDrain2PinWLpi2cSlave = 0x04,
    #[doc = "Two-pin output only mode (Ultra-Fast mode) with separate LPI2C target."]
    Output2PinOnlyWLpi2cSlave = 0x05,
    #[doc = "Two-pin push-pull mode with separate LPI2C target."]
    PushPull2PinWLpi2cSlave = 0x06,
    #[doc = "Four-pin push-pull mode (inverted outputs)."]
    PushPull4PinWLpi2cSlave = 0x07,
}
impl Pincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pincfg {
        unsafe { core::mem::transmute(val & 0x07) }
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
pub enum Pltf {
    #[doc = "Pin low timeout did not occur."]
    IntNo = 0x0,
    #[doc = "Pin low timeout occurred."]
    IntYes = 0x01,
}
impl Pltf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pltf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pltf {
    #[inline(always)]
    fn from(val: u8) -> Pltf {
        Pltf::from_bits(val)
    }
}
impl From<Pltf> for u8 {
    #[inline(always)]
    fn from(val: Pltf) -> u8 {
        Pltf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "Divide by 1."]
    DivideBy1 = 0x0,
    #[doc = "Divide by 2."]
    DivideBy2 = 0x01,
    #[doc = "Divide by 4."]
    DivideBy4 = 0x02,
    #[doc = "Divide by 8."]
    DivideBy8 = 0x03,
    #[doc = "Divide by 16."]
    DivideBy16 = 0x04,
    #[doc = "Divide by 32."]
    DivideBy32 = 0x05,
    #[doc = "Divide by 64."]
    DivideBy64 = 0x06,
    #[doc = "Divide by 128."]
    DivideBy128 = 0x07,
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
pub enum Relax {
    #[doc = "Normal transfer."]
    NormalTransfer = 0x0,
    #[doc = "Relaxed transfer."]
    RelaxedTransfer = 0x01,
}
impl Relax {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Relax {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Relax {
    #[inline(always)]
    fn from(val: u8) -> Relax {
        Relax::from_bits(val)
    }
}
impl From<Relax> for u8 {
    #[inline(always)]
    fn from(val: Relax) -> u8 {
        Relax::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rscfg {
    #[doc = "Any repeated Start condition following an address match."]
    AnyRepeatedStartAfterAddressMatch = 0x0,
    #[doc = "Any repeated Start condition."]
    AnyRepeatedStart = 0x01,
}
impl Rscfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rscfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rscfg {
    #[inline(always)]
    fn from(val: u8) -> Rscfg {
        Rscfg::from_bits(val)
    }
}
impl From<Rscfg> for u8 {
    #[inline(always)]
    fn from(val: Rscfg) -> u8 {
        Rscfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsf {
    #[doc = "No repeated Start detected."]
    IntNo = 0x0,
    #[doc = "Repeated Start detected."]
    IntYes = 0x01,
}
impl Rsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsf {
    #[inline(always)]
    fn from(val: u8) -> Rsf {
        Rsf::from_bits(val)
    }
}
impl From<Rsf> for u8 {
    #[inline(always)]
    fn from(val: Rsf) -> u8 {
        Rsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxcfg {
    #[doc = "Return received data, clear MSR\\[RDF\\]."]
    ReturnsReceivedDataAndClearsRxDataFlag = 0x0,
    #[doc = "Return SASR and clear SSR\\[AVF\\] when SSR\\[AVF\\] is set, return received data and clear MSR\\[RDF\\] when SSR\\[AFV\\] is not set."]
    WhenAddressValidFlagSetReturnsAddressStatusAndClearsAddressValidFlag = 0x01,
}
impl Rxcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxcfg {
    #[inline(always)]
    fn from(val: u8) -> Rxcfg {
        Rxcfg::from_bits(val)
    }
}
impl From<Rxcfg> for u8 {
    #[inline(always)]
    fn from(val: Rxcfg) -> u8 {
        Rxcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxnack {
    #[doc = "ACK or NACK always determined by STAR\\[TXNACK\\]."]
    SetByTxnack = 0x0,
    #[doc = "NACK always generated on address overrun or receive data overrun, otherwise ACK or NACK is determined by STAR\\[TXNACK\\]."]
    AlwaysGeneratedOnAddressOrReceiveDataOverrun = 0x01,
}
impl Rxnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxnack {
    #[inline(always)]
    fn from(val: u8) -> Rxnack {
        Rxnack::from_bits(val)
    }
}
impl From<Rxnack> for u8 {
    #[inline(always)]
    fn from(val: Rxnack) -> u8 {
        Rxnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl Sbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbf {
    #[inline(always)]
    fn from(val: u8) -> Sbf {
        Sbf::from_bits(val)
    }
}
impl From<Sbf> for u8 {
    #[inline(always)]
    fn from(val: Sbf) -> u8 {
        Sbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scfgr1Ignack {
    #[doc = "End transfer on NACK."]
    EndsTransferOnNack = 0x0,
    #[doc = "Do not end transfer on NACK."]
    DoesNotEndTransferOnNack = 0x01,
}
impl Scfgr1Ignack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scfgr1Ignack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scfgr1Ignack {
    #[inline(always)]
    fn from(val: u8) -> Scfgr1Ignack {
        Scfgr1Ignack::from_bits(val)
    }
}
impl From<Scfgr1Ignack> for u8 {
    #[inline(always)]
    fn from(val: Scfgr1Ignack) -> u8 {
        Scfgr1Ignack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRrf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "SRDR is now empty."]
    NowEmpty = 0x01,
}
impl ScrRrf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRrf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRrf {
    #[inline(always)]
    fn from(val: u8) -> ScrRrf {
        ScrRrf::from_bits(val)
    }
}
impl From<ScrRrf> for u8 {
    #[inline(always)]
    fn from(val: ScrRrf) -> u8 {
        ScrRrf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRtf {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "STDR is now empty."]
    NowEmpty = 0x01,
}
impl ScrRtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRtf {
    #[inline(always)]
    fn from(val: u8) -> ScrRtf {
        ScrRtf::from_bits(val)
    }
}
impl From<ScrRtf> for u8 {
    #[inline(always)]
    fn from(val: ScrRtf) -> u8 {
        ScrRtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdcfg {
    #[doc = "Any Stop condition following an address match."]
    AnyStopAfterAddressMatch = 0x0,
    #[doc = "Any Stop condition."]
    AnyStop = 0x01,
}
impl Sdcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdcfg {
    #[inline(always)]
    fn from(val: u8) -> Sdcfg {
        Sdcfg::from_bits(val)
    }
}
impl From<Sdcfg> for u8 {
    #[inline(always)]
    fn from(val: Sdcfg) -> u8 {
        Sdcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrBbf {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Busy."]
    Busy = 0x01,
}
impl SsrBbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrBbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrBbf {
    #[inline(always)]
    fn from(val: u8) -> SsrBbf {
        SsrBbf::from_bits(val)
    }
}
impl From<SsrBbf> for u8 {
    #[inline(always)]
    fn from(val: SsrBbf) -> u8 {
        SsrBbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrFef {
    #[doc = "No FIFO error."]
    IntNo = 0x0,
    #[doc = "FIFO error."]
    IntYes = 0x01,
}
impl SsrFef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrFef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrFef {
    #[inline(always)]
    fn from(val: u8) -> SsrFef {
        SsrFef::from_bits(val)
    }
}
impl From<SsrFef> for u8 {
    #[inline(always)]
    fn from(val: SsrFef) -> u8 {
        SsrFef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSdf {
    #[doc = "No Stop detected."]
    IntNo = 0x0,
    #[doc = "Stop detected."]
    IntYes = 0x01,
}
impl SsrSdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrSdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrSdf {
    #[inline(always)]
    fn from(val: u8) -> SsrSdf {
        SsrSdf::from_bits(val)
    }
}
impl From<SsrSdf> for u8 {
    #[inline(always)]
    fn from(val: SsrSdf) -> u8 {
        SsrSdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Startcfg {
    #[doc = "Sets when both I2C bus and LPI2C controller are idle."]
    BothI2cAndLpi2cIdle = 0x0,
    #[doc = "Sets when I2C bus is idle."]
    I2cIdle = 0x01,
}
impl Startcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Startcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Startcfg {
    #[inline(always)]
    fn from(val: u8) -> Startcfg {
        Startcfg::from_bits(val)
    }
}
impl From<Startcfg> for u8 {
    #[inline(always)]
    fn from(val: Startcfg) -> u8 {
        Startcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stf {
    #[doc = "Start condition not detected."]
    IntNo = 0x0,
    #[doc = "Start condition detected."]
    IntYes = 0x01,
}
impl Stf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stf {
    #[inline(always)]
    fn from(val: u8) -> Stf {
        Stf::from_bits(val)
    }
}
impl From<Stf> for u8 {
    #[inline(always)]
    fn from(val: Stf) -> u8 {
        Stf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopcfg {
    #[doc = "Any Stop condition."]
    AnyStop = 0x0,
    #[doc = "Last Stop condition."]
    LastStop = 0x01,
}
impl Stopcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopcfg {
    #[inline(always)]
    fn from(val: u8) -> Stopcfg {
        Stopcfg::from_bits(val)
    }
}
impl From<Stopcfg> for u8 {
    #[inline(always)]
    fn from(val: Stopcfg) -> u8 {
        Stopcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecfg {
    #[doc = "SCL."]
    IfSclLow = 0x0,
    #[doc = "SCL or SDA."]
    IfSclOrSdaLow = 0x01,
}
impl Timecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timecfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timecfg {
    #[inline(always)]
    fn from(val: u8) -> Timecfg {
        Timecfg::from_bits(val)
    }
}
impl From<Timecfg> for u8 {
    #[inline(always)]
    fn from(val: Timecfg) -> u8 {
        Timecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txcfg {
    #[doc = "MSR\\[TDF\\] is set only during a target-transmit transfer when STDR is empty."]
    AssertsDuringSlaveTransmitTransferWhenTxDataEmpty = 0x0,
    #[doc = "MSR\\[TDF\\] is set whenever STDR is empty."]
    AssertsWhenTxDataEmpty = 0x01,
}
impl Txcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txcfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txcfg {
    #[inline(always)]
    fn from(val: u8) -> Txcfg {
        Txcfg::from_bits(val)
    }
}
impl From<Txcfg> for u8 {
    #[inline(always)]
    fn from(val: Txcfg) -> u8 {
        Txcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txnack {
    #[doc = "Transmit ACK."]
    TransmitAck = 0x0,
    #[doc = "Transmit NACK."]
    TransmitNack = 0x01,
}
impl Txnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txnack {
    #[inline(always)]
    fn from(val: u8) -> Txnack {
        Txnack::from_bits(val)
    }
}
impl From<Txnack> for u8 {
    #[inline(always)]
    fn from(val: Txnack) -> u8 {
        Txnack::to_bits(val)
    }
}
