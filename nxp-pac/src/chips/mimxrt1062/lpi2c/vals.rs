#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrcfg {
    #[doc = "Address match 0 (7-bit)"]
    ADDRESS_MATCH0_7_BIT = 0x0,
    #[doc = "Address match 0 (10-bit)"]
    ADDRESS_MATCH0_10_BIT = 0x01,
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT = 0x02,
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT = 0x03,
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT = 0x04,
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT = 0x05,
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
    FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT = 0x06,
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
    FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT = 0x07,
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
pub enum Anv {
    #[doc = "Received Address (RADDR) is valid"]
    VALID = 0x0,
    #[doc = "Received Address (RADDR) is not valid"]
    NOT_VALID = 0x01,
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
pub enum Cmd {
    #[doc = "Transmit DATA\\[7:0\\]"]
    TRANSMIT_DATA_7_THROUGH_0 = 0x0,
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes"]
    RECEIVE_DATA_7_THROUGH_0_PLUS_ONE = 0x01,
    #[doc = "Generate STOP condition"]
    GENERATE_STOP_CONDITION = 0x02,
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes"]
    RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE = 0x03,
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0 = 0x04,
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK = 0x05,
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE = 0x06,
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\] using high speed mode. This transfer expects a NACK to be returned."]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK =
        0x07,
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
pub enum Dozen {
    #[doc = "Master is enabled in Doze mode"]
    ENABLED = 0x0,
    #[doc = "Master is disabled in Doze mode"]
    DISABLED = 0x01,
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
    #[doc = "Master only, with standard feature set"]
    pub const MASTER_ONLY: Self = Self(0x02);
    #[doc = "Master and slave, with standard feature set"]
    pub const MASTER_AND_SLAVE: Self = Self(0x03);
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
            0x02 => f.write_str("MASTER_ONLY"),
            0x03 => f.write_str("MASTER_AND_SLAVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x02 => defmt::write!(f, "MASTER_ONLY"),
            0x03 => defmt::write!(f, "MASTER_AND_SLAVE"),
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
    #[doc = "Filter remains enabled in Doze mode"]
    FILTER_ENABLED = 0x0,
    #[doc = "Filter is disabled in Doze mode"]
    FILTER_DISABLED = 0x01,
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
pub enum Hrpol {
    #[doc = "Active low"]
    ACTIVE_LOW = 0x0,
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x01,
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
    #[doc = "Match is disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled (1st data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\])"]
    FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 0x02,
    #[doc = "Match is enabled (any data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\])"]
    ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 0x03,
    #[doc = "Match is enabled (1st data word equals MDMR\\[MATCH0\\] AND 2nd data word equals MDMR\\[MATCH1)"]
    FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1 = 0x04,
    #[doc = "Match is enabled (any data word equals MDMR\\[MATCH0\\] AND next data word equals MDMR\\[MATCH1)"]
    ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1 = 0x05,
    #[doc = "Match is enabled (1st data word AND MDMR\\[MATCH1\\] equals MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
    FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1 = 0x06,
    #[doc = "Match is enabled (any data word AND MDMR\\[MATCH1\\] equals MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
    ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1 = 0x07,
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
    #[doc = "I2C Master is idle"]
    IDLE = 0x0,
    #[doc = "I2C Master is busy"]
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
pub enum McrRrf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Receive FIFO is reset"]
    RESET = 0x01,
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
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Transmit FIFO is reset"]
    RESET = 0x01,
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
    #[doc = "I2C Bus is idle"]
    IDLE = 0x0,
    #[doc = "I2C Bus is busy"]
    BUSY = 0x01,
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
pub enum Pincfg {
    #[doc = "2-pin open drain mode"]
    OPEN_DRAIN_2_PIN = 0x0,
    #[doc = "2-pin output only mode (ultra-fast mode)"]
    OUTPUT_2_PIN_ONLY = 0x01,
    #[doc = "2-pin push-pull mode"]
    PUSH_PULL_2_PIN = 0x02,
    #[doc = "4-pin push-pull mode"]
    PUSH_PULL_4_PIN = 0x03,
    #[doc = "2-pin open drain mode with separate LPI2C slave"]
    OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE = 0x04,
    #[doc = "2-pin output only mode (ultra-fast mode) with separate LPI2C slave"]
    OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE = 0x05,
    #[doc = "2-pin push-pull mode with separate LPI2C slave"]
    PUSH_PULL_2_PIN_W_LPI2C_SLAVE = 0x06,
    #[doc = "4-pin push-pull mode (inverted outputs)"]
    PUSH_PULL_4_PIN_W_LPI2C_SLAVE = 0x07,
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
pub enum Prescale {
    #[doc = "Divide by 1"]
    DIVIDE_BY_1 = 0x0,
    #[doc = "Divide by 2"]
    DIVIDE_BY_2 = 0x01,
    #[doc = "Divide by 4"]
    DIVIDE_BY_4 = 0x02,
    #[doc = "Divide by 8"]
    DIVIDE_BY_8 = 0x03,
    #[doc = "Divide by 16"]
    DIVIDE_BY_16 = 0x04,
    #[doc = "Divide by 32"]
    DIVIDE_BY_32 = 0x05,
    #[doc = "Divide by 64"]
    DIVIDE_BY_64 = 0x06,
    #[doc = "Divide by 128"]
    DIVIDE_BY_128 = 0x07,
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
pub enum Rxcfg {
    #[doc = "Reading the Receive Data register returns received data and clears the Receive Data flag."]
    RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG = 0x0,
    #[doc = "Reading the Receive Data register when the Address Valid flag (SSR\\[AVF\\]) is set, returns the Address Status register and clear the Address Valid flag. Reading the Receive Data register when the Address Valid flag is clear, returns received data and clears the Receive Data flag (MSR\\[RDF\\])."]
    WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG = 0x01,
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
pub enum Sbf {
    #[doc = "I2C Slave is idle"]
    IDLE = 0x0,
    #[doc = "I2C Slave is busy"]
    BUSY = 0x01,
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
    #[doc = "Slave ends transfer when NACK is detected"]
    ENDS_TRANSFER_ON_NACK = 0x0,
    #[doc = "Slave does not end transfer when NACK detected"]
    DOES_NOT_END_TRANSFER_ON_NACK = 0x01,
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
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Receive Data Register is now empty"]
    NOW_EMPTY = 0x01,
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
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Transmit Data Register is now empty"]
    NOW_EMPTY = 0x01,
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
pub enum SsrBbf {
    #[doc = "I2C Bus is idle"]
    IDLE = 0x0,
    #[doc = "I2C Bus is busy"]
    BUSY = 0x01,
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
pub enum Timecfg {
    #[doc = "MSR\\[PLTF\\] sets if SCL is low for longer than the configured timeout"]
    IF_SCL_LOW = 0x0,
    #[doc = "MSR\\[PLTF\\] sets if either SCL or SDA is low for longer than the configured timeout"]
    IF_SCL_OR_SDA_LOW = 0x01,
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
    #[doc = "Transmit Data Flag only asserts during a slave-transmit transfer when the Transmit Data register is empty"]
    ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY = 0x0,
    #[doc = "Transmit Data Flag asserts whenever the Transmit Data register is empty"]
    ASSERTS_WHEN_TX_DATA_EMPTY = 0x01,
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
    #[doc = "Write a Transmit ACK for each received word"]
    TRANSMIT_ACK = 0x0,
    #[doc = "Write a Transmit NACK for each received word"]
    TRANSMIT_NACK = 0x01,
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
