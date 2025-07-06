#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abort {
    #[doc = "Normal transfer"]
    DISABLED = 0x0,
    #[doc = "Abort existing transfer and do not start a new one"]
    ENABLED = 0x01,
}
impl Abort {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abort {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abort {
    #[inline(always)]
    fn from(val: u8) -> Abort {
        Abort::from_bits(val)
    }
}
impl From<Abort> for u8 {
    #[inline(always)]
    fn from(val: Abort) -> u8 {
        Abort::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackstall {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ackstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackstall {
    #[inline(always)]
    fn from(val: u8) -> Ackstall {
        Ackstall::from_bits(val)
    }
}
impl From<Ackstall> for u8 {
    #[inline(always)]
    fn from(val: Ackstall) -> u8 {
        Ackstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addrcfg {
    #[doc = "Address match 0 (7-bit)"]
    ADDRESS_MATCH0_7_BIT = 0x0,
    #[doc = "Address match 0 (10-bit)"]
    ADDRESS_MATCH0_10_BIT = 0x01,
    #[doc = "Address match 0 (7-bit) or address match 1 (7-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT = 0x02,
    #[doc = "Address match 0 (10-bit) or address match 1 (10-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT = 0x03,
    #[doc = "Address match 0 (7-bit) or address match 1 (10-bit)"]
    ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT = 0x04,
    #[doc = "Address match 0 (10-bit) or address match 1 (7-bit)"]
    ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT = 0x05,
    #[doc = "From address match 0 (7-bit) to address match 1 (7-bit)"]
    FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT = 0x06,
    #[doc = "From address match 0 (10-bit) to address match 1 (10-bit)"]
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
pub enum Adrstall {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Adrstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adrstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adrstall {
    #[inline(always)]
    fn from(val: u8) -> Adrstall {
        Adrstall::from_bits(val)
    }
}
impl From<Adrstall> for u8 {
    #[inline(always)]
    fn from(val: Adrstall) -> u8 {
        Adrstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alf {
    #[doc = "Controller did not lose arbitration"]
    INT_NO = 0x0,
    #[doc = "Controller lost arbitration"]
    INT_YES = 0x01,
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
pub enum Alie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Alie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alie {
    #[inline(always)]
    fn from(val: u8) -> Alie {
        Alie::from_bits(val)
    }
}
impl From<Alie> for u8 {
    #[inline(always)]
    fn from(val: Alie) -> u8 {
        Alie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Am0f {
    #[doc = "ADDR0 matching address not received"]
    NO_FLAG = 0x0,
    #[doc = "ADDR0 matching address received"]
    FLAG = 0x01,
}
impl Am0f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Am0f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Am0f {
    #[inline(always)]
    fn from(val: u8) -> Am0f {
        Am0f::from_bits(val)
    }
}
impl From<Am0f> for u8 {
    #[inline(always)]
    fn from(val: Am0f) -> u8 {
        Am0f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Am0ie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Am0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Am0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Am0ie {
    #[inline(always)]
    fn from(val: u8) -> Am0ie {
        Am0ie::from_bits(val)
    }
}
impl From<Am0ie> for u8 {
    #[inline(always)]
    fn from(val: Am0ie) -> u8 {
        Am0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Am1f {
    #[doc = "Matching address not received"]
    NO_FLAG = 0x0,
    #[doc = "Matching address received"]
    FLAG = 0x01,
}
impl Am1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Am1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Am1f {
    #[inline(always)]
    fn from(val: u8) -> Am1f {
        Am1f::from_bits(val)
    }
}
impl From<Am1f> for u8 {
    #[inline(always)]
    fn from(val: Am1f) -> u8 {
        Am1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Am1ie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Am1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Am1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Am1ie {
    #[inline(always)]
    fn from(val: u8) -> Am1ie {
        Am1ie::from_bits(val)
    }
}
impl From<Am1ie> for u8 {
    #[inline(always)]
    fn from(val: Am1ie) -> u8 {
        Am1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Anv {
    #[doc = "Valid"]
    VALID = 0x0,
    #[doc = "Not valid"]
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
pub enum Autostop {
    #[doc = "No effect"]
    DISABLED = 0x0,
    #[doc = "Stop automatically generated"]
    ENABLED = 0x01,
}
impl Autostop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autostop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autostop {
    #[inline(always)]
    fn from(val: u8) -> Autostop {
        Autostop::from_bits(val)
    }
}
impl From<Autostop> for u8 {
    #[inline(always)]
    fn from(val: Autostop) -> u8 {
        Autostop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Avde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avde {
    #[inline(always)]
    fn from(val: u8) -> Avde {
        Avde::from_bits(val)
    }
}
impl From<Avde> for u8 {
    #[inline(always)]
    fn from(val: Avde) -> u8 {
        Avde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avf {
    #[doc = "Not valid"]
    NOT_VALID = 0x0,
    #[doc = "Valid"]
    VALID = 0x01,
}
impl Avf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avf {
    #[inline(always)]
    fn from(val: u8) -> Avf {
        Avf::from_bits(val)
    }
}
impl From<Avf> for u8 {
    #[inline(always)]
    fn from(val: Avf) -> u8 {
        Avf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Avie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avie {
    #[inline(always)]
    fn from(val: u8) -> Avie {
        Avie::from_bits(val)
    }
}
impl From<Avie> for u8 {
    #[inline(always)]
    fn from(val: Avie) -> u8 {
        Avie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bef {
    #[doc = "No bit error occurred"]
    INT_NO = 0x0,
    #[doc = "Bit error occurred"]
    INT_YES = 0x01,
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
pub enum Beie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Beie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Beie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Beie {
    #[inline(always)]
    fn from(val: u8) -> Beie {
        Beie::from_bits(val)
    }
}
impl From<Beie> for u8 {
    #[inline(always)]
    fn from(val: Beie) -> u8 {
        Beie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cirfifo {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Cirfifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cirfifo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cirfifo {
    #[inline(always)]
    fn from(val: u8) -> Cirfifo {
        Cirfifo::from_bits(val)
    }
}
impl From<Cirfifo> for u8 {
    #[inline(always)]
    fn from(val: Cirfifo) -> u8 {
        Cirfifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "Transmit the value in DATA\\[7:0\\]"]
    TRANSMIT_DATA_7_THROUGH_0 = 0x0,
    #[doc = "Receive (DATA\\[7:0\\] + 1) bytes"]
    RECEIVE_DATA_7_THROUGH_0_PLUS_ONE = 0x01,
    #[doc = "Generate Stop condition on I2C bus"]
    GENERATE_STOP_CONDITION = 0x02,
    #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes"]
    RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE = 0x03,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0 = 0x04,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] (this transfer expects a NACK to be returned)"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK = 0x05,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE = 0x06,
    #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode (this transfer expects a NACK to be returned)"]
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
pub enum Dbgen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgen {
    #[inline(always)]
    fn from(val: u8) -> Dbgen {
        Dbgen::from_bits(val)
    }
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(val: Dbgen) -> u8 {
        Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmf {
    #[doc = "Matching data not received"]
    INT_NO = 0x0,
    #[doc = "Matching data received"]
    INT_YES = 0x01,
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
pub enum Dmie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Dmie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmie {
    #[inline(always)]
    fn from(val: u8) -> Dmie {
        Dmie::from_bits(val)
    }
}
impl From<Dmie> for u8 {
    #[inline(always)]
    fn from(val: Dmie) -> u8 {
        Dmie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable"]
    ENABLED = 0x0,
    #[doc = "Disable"]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epf {
    #[doc = "No Stop or repeated Start generated"]
    INT_NO = 0x0,
    #[doc = "Stop or repeated Start generated"]
    INT_YES = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Epie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Epie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Epie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Epie {
    #[inline(always)]
    fn from(val: u8) -> Epie {
        Epie::from_bits(val)
    }
}
impl From<Epie> for u8 {
    #[inline(always)]
    fn from(val: Epie) -> u8 {
        Epie::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Controller only, with standard feature set"]
    pub const MASTER_ONLY: Self = Self(0x02);
    #[doc = "Controller and target, with standard feature set"]
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
    #[doc = "Enable"]
    FILTER_ENABLED = 0x0,
    #[doc = "Disable"]
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
pub enum Filten {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Filten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filten {
    #[inline(always)]
    fn from(val: u8) -> Filten {
        Filten::from_bits(val)
    }
}
impl From<Filten> for u8 {
    #[inline(always)]
    fn from(val: Filten) -> u8 {
        Filten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gcen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Gcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gcen {
    #[inline(always)]
    fn from(val: u8) -> Gcen {
        Gcen::from_bits(val)
    }
}
impl From<Gcen> for u8 {
    #[inline(always)]
    fn from(val: Gcen) -> u8 {
        Gcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gcf {
    #[doc = "General call address disabled or not detected"]
    NO_FLAG = 0x0,
    #[doc = "General call address detected"]
    FLAG = 0x01,
}
impl Gcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gcf {
    #[inline(always)]
    fn from(val: u8) -> Gcf {
        Gcf::from_bits(val)
    }
}
impl From<Gcf> for u8 {
    #[inline(always)]
    fn from(val: Gcf) -> u8 {
        Gcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gcie {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Gcie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gcie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gcie {
    #[inline(always)]
    fn from(val: u8) -> Gcie {
        Gcie::from_bits(val)
    }
}
impl From<Gcie> for u8 {
    #[inline(always)]
    fn from(val: Gcie) -> u8 {
        Gcie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hrdir {
    #[doc = "HREQ pin is input (for LPI2C controller)"]
    INPUT = 0x0,
    #[doc = "HREQ pin is output (for LPI2C target)"]
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
pub enum Hren {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Hren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hren {
    #[inline(always)]
    fn from(val: u8) -> Hren {
        Hren::from_bits(val)
    }
}
impl From<Hren> for u8 {
    #[inline(always)]
    fn from(val: Hren) -> u8 {
        Hren::to_bits(val)
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
pub enum Hrsel {
    #[doc = "Host request input is pin HREQ"]
    DISABLED = 0x0,
    #[doc = "Host request input is input trigger"]
    ENABLED = 0x01,
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
pub enum Hsmen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Hsmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsmen {
    #[inline(always)]
    fn from(val: u8) -> Hsmen {
        Hsmen::from_bits(val)
    }
}
impl From<Hsmen> for u8 {
    #[inline(always)]
    fn from(val: Hsmen) -> u8 {
        Hsmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Match is disabled"]
    DISABLED = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Match is enabled: first data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]"]
    FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 0x02,
    #[doc = "Match is enabled: any data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]"]
    ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1 = 0x03,
    #[doc = "Match is enabled: (first data word equals MDMR\\[MATCH0\\]) AND (second data word equals MDMR\\[MATCH1)"]
    FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1 = 0x04,
    #[doc = "Match is enabled: (any data word equals MDMR\\[MATCH0\\]) AND (next data word equals MDMR\\[MATCH1)"]
    ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1 = 0x05,
    #[doc = "Match is enabled: (first data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
    FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1 = 0x06,
    #[doc = "Match is enabled: (any data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
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
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
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
pub enum Mcfgr1Ignack {
    #[doc = "No effect"]
    DISABLED = 0x0,
    #[doc = "Treat a received NACK as an ACK"]
    ENABLED = 0x01,
}
impl Mcfgr1Ignack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcfgr1Ignack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcfgr1Ignack {
    #[inline(always)]
    fn from(val: u8) -> Mcfgr1Ignack {
        Mcfgr1Ignack::from_bits(val)
    }
}
impl From<Mcfgr1Ignack> for u8 {
    #[inline(always)]
    fn from(val: Mcfgr1Ignack) -> u8 {
        Mcfgr1Ignack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRrf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Reset receive FIFO"]
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
pub enum McrRst {
    #[doc = "No effect"]
    NOT_RESET = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl McrRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McrRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McrRst {
    #[inline(always)]
    fn from(val: u8) -> McrRst {
        McrRst::from_bits(val)
    }
}
impl From<McrRst> for u8 {
    #[inline(always)]
    fn from(val: McrRst) -> u8 {
        McrRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McrRtf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Reset transmit FIFO"]
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
pub enum MderRdde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MderRdde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MderRdde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MderRdde {
    #[inline(always)]
    fn from(val: u8) -> MderRdde {
        MderRdde::from_bits(val)
    }
}
impl From<MderRdde> for u8 {
    #[inline(always)]
    fn from(val: MderRdde) -> u8 {
        MderRdde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MderTdde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MderTdde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MderTdde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MderTdde {
    #[inline(always)]
    fn from(val: u8) -> MderTdde {
        MderTdde::from_bits(val)
    }
}
impl From<MderTdde> for u8 {
    #[inline(always)]
    fn from(val: MderTdde) -> u8 {
        MderTdde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Men {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Men {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Men {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Men {
    #[inline(always)]
    fn from(val: u8) -> Men {
        Men::from_bits(val)
    }
}
impl From<Men> for u8 {
    #[inline(always)]
    fn from(val: Men) -> u8 {
        Men::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierFeie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MierFeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierFeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierFeie {
    #[inline(always)]
    fn from(val: u8) -> MierFeie {
        MierFeie::from_bits(val)
    }
}
impl From<MierFeie> for u8 {
    #[inline(always)]
    fn from(val: MierFeie) -> u8 {
        MierFeie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierRdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MierRdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierRdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierRdie {
    #[inline(always)]
    fn from(val: u8) -> MierRdie {
        MierRdie::from_bits(val)
    }
}
impl From<MierRdie> for u8 {
    #[inline(always)]
    fn from(val: MierRdie) -> u8 {
        MierRdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierSdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MierSdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierSdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierSdie {
    #[inline(always)]
    fn from(val: u8) -> MierSdie {
        MierSdie::from_bits(val)
    }
}
impl From<MierSdie> for u8 {
    #[inline(always)]
    fn from(val: MierSdie) -> u8 {
        MierSdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MierTdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl MierTdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MierTdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MierTdie {
    #[inline(always)]
    fn from(val: u8) -> MierTdie {
        MierTdie::from_bits(val)
    }
}
impl From<MierTdie> for u8 {
    #[inline(always)]
    fn from(val: MierTdie) -> u8 {
        MierTdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrdrRxempty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl MrdrRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrdrRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrdrRxempty {
    #[inline(always)]
    fn from(val: u8) -> MrdrRxempty {
        MrdrRxempty::from_bits(val)
    }
}
impl From<MrdrRxempty> for u8 {
    #[inline(always)]
    fn from(val: MrdrRxempty) -> u8 {
        MrdrRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrdrorRxempty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl MrdrorRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrdrorRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrdrorRxempty {
    #[inline(always)]
    fn from(val: u8) -> MrdrorRxempty {
        MrdrorRxempty::from_bits(val)
    }
}
impl From<MrdrorRxempty> for u8 {
    #[inline(always)]
    fn from(val: MrdrorRxempty) -> u8 {
        MrdrorRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrBbf {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
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
pub enum MsrFef {
    #[doc = "No FIFO error"]
    INT_NO = 0x0,
    #[doc = "FIFO error"]
    INT_YES = 0x01,
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
pub enum MsrRdf {
    #[doc = "Receive data not ready"]
    DISABLED = 0x0,
    #[doc = "Receive data ready"]
    ENABLED = 0x01,
}
impl MsrRdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrRdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrRdf {
    #[inline(always)]
    fn from(val: u8) -> MsrRdf {
        MsrRdf::from_bits(val)
    }
}
impl From<MsrRdf> for u8 {
    #[inline(always)]
    fn from(val: MsrRdf) -> u8 {
        MsrRdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MsrSdf {
    #[doc = "No Stop condition generated"]
    INT_NO = 0x0,
    #[doc = "Stop condition generated"]
    INT_YES = 0x01,
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
pub enum MsrTdf {
    #[doc = "Transmit data not requested"]
    DISABLED = 0x0,
    #[doc = "Transmit data requested"]
    ENABLED = 0x01,
}
impl MsrTdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MsrTdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MsrTdf {
    #[inline(always)]
    fn from(val: u8) -> MsrTdf {
        MsrTdf::from_bits(val)
    }
}
impl From<MsrTdf> for u8 {
    #[inline(always)]
    fn from(val: MsrTdf) -> u8 {
        MsrTdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndf {
    #[doc = "No unexpected NACK detected"]
    INT_NO = 0x0,
    #[doc = "Unexpected NACK detected"]
    INT_YES = 0x01,
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
pub enum Ndie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Ndie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndie {
    #[inline(always)]
    fn from(val: u8) -> Ndie {
        Ndie::from_bits(val)
    }
}
impl From<Ndie> for u8 {
    #[inline(always)]
    fn from(val: Ndie) -> u8 {
        Ndie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pincfg {
    #[doc = "Two-pin open drain mode"]
    OPEN_DRAIN_2_PIN = 0x0,
    #[doc = "Two-pin output only mode (Ultra-Fast mode)"]
    OUTPUT_2_PIN_ONLY = 0x01,
    #[doc = "Two-pin push-pull mode"]
    PUSH_PULL_2_PIN = 0x02,
    #[doc = "Four-pin push-pull mode"]
    PUSH_PULL_4_PIN = 0x03,
    #[doc = "Two-pin open-drain mode with separate LPI2C target"]
    OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE = 0x04,
    #[doc = "Two-pin output only mode (Ultra-Fast mode) with separate LPI2C target"]
    OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE = 0x05,
    #[doc = "Two-pin push-pull mode with separate LPI2C target"]
    PUSH_PULL_2_PIN_W_LPI2C_SLAVE = 0x06,
    #[doc = "Four-pin push-pull mode (inverted outputs)"]
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
pub enum Pltf {
    #[doc = "Pin low timeout did not occur"]
    INT_NO = 0x0,
    #[doc = "Pin low timeout occurred"]
    INT_YES = 0x01,
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
pub enum Pltie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Pltie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pltie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pltie {
    #[inline(always)]
    fn from(val: u8) -> Pltie {
        Pltie::from_bits(val)
    }
}
impl From<Pltie> for u8 {
    #[inline(always)]
    fn from(val: Pltie) -> u8 {
        Pltie::to_bits(val)
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
pub enum Rdack {
    #[doc = "Read Request not acknowledged"]
    NOT_ACKNOWLEDGED = 0x0,
    #[doc = "Read Request acknowledged"]
    ACKNOWLEDGED = 0x01,
}
impl Rdack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdack {
    #[inline(always)]
    fn from(val: u8) -> Rdack {
        Rdack::from_bits(val)
    }
}
impl From<Rdack> for u8 {
    #[inline(always)]
    fn from(val: Rdack) -> u8 {
        Rdack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdmo {
    #[doc = "Received data is stored in the receive FIFO"]
    DISABLED = 0x0,
    #[doc = "Received data is discarded unless MSR\\[DMF\\] is set"]
    ENABLED = 0x01,
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
pub enum Rdreq {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rdreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdreq {
    #[inline(always)]
    fn from(val: u8) -> Rdreq {
        Rdreq::from_bits(val)
    }
}
impl From<Rdreq> for u8 {
    #[inline(always)]
    fn from(val: Rdreq) -> u8 {
        Rdreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Relax {
    #[doc = "Normal transfer"]
    NORMAL_TRANSFER = 0x0,
    #[doc = "Relaxed transfer"]
    RELAXED_TRANSFER = 0x01,
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
    #[doc = "Any repeated Start condition following an address match"]
    ANY_REPEATED_START_AFTER_ADDRESS_MATCH = 0x0,
    #[doc = "Any repeated Start condition"]
    ANY_REPEATED_START = 0x01,
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
pub enum Rsde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rsde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsde {
    #[inline(always)]
    fn from(val: u8) -> Rsde {
        Rsde::from_bits(val)
    }
}
impl From<Rsde> for u8 {
    #[inline(always)]
    fn from(val: Rsde) -> u8 {
        Rsde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsf {
    #[doc = "No repeated Start detected"]
    INT_NO = 0x0,
    #[doc = "Repeated Start detected"]
    INT_YES = 0x01,
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
pub enum Rsie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rsie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsie {
    #[inline(always)]
    fn from(val: u8) -> Rsie {
        Rsie::from_bits(val)
    }
}
impl From<Rsie> for u8 {
    #[inline(always)]
    fn from(val: Rsie) -> u8 {
        Rsie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxall {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rxall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxall {
    #[inline(always)]
    fn from(val: u8) -> Rxall {
        Rxall::from_bits(val)
    }
}
impl From<Rxall> for u8 {
    #[inline(always)]
    fn from(val: Rxall) -> u8 {
        Rxall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxcfg {
    #[doc = "Return received data, clear MSR\\[RDF\\]"]
    RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG = 0x0,
    #[doc = "Return SASR and clear SSR\\[AVF\\] when SSR\\[AVF\\] is set, return received data and clear MSR\\[RDF\\] when SSR\\[AFV\\] is not set"]
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
pub enum Rxnack {
    #[doc = "ACK or NACK always determined by STAR\\[TXNACK\\]"]
    SET_BY_TXNACK = 0x0,
    #[doc = "NACK always generated on address overrun or receive data overrun, otherwise ACK or NACK is determined by STAR\\[TXNACK\\]"]
    ALWAYS_GENERATED_ON_ADDRESS_OR_RECEIVE_DATA_OVERRUN = 0x01,
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
pub enum Rxstall {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Rxstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxstall {
    #[inline(always)]
    fn from(val: u8) -> Rxstall {
        Rxstall::from_bits(val)
    }
}
impl From<Rxstall> for u8 {
    #[inline(always)]
    fn from(val: Rxstall) -> u8 {
        Rxstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saen {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Saen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saen {
    #[inline(always)]
    fn from(val: u8) -> Saen {
        Saen::from_bits(val)
    }
}
impl From<Saen> for u8 {
    #[inline(always)]
    fn from(val: Saen) -> u8 {
        Saen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarf {
    #[doc = "Disabled or not detected"]
    NO_FLAG = 0x0,
    #[doc = "Enabled and detected"]
    FLAG = 0x01,
}
impl Sarf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarf {
    #[inline(always)]
    fn from(val: u8) -> Sarf {
        Sarf::from_bits(val)
    }
}
impl From<Sarf> for u8 {
    #[inline(always)]
    fn from(val: Sarf) -> u8 {
        Sarf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Sarie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarie {
    #[inline(always)]
    fn from(val: u8) -> Sarie {
        Sarie::from_bits(val)
    }
}
impl From<Sarie> for u8 {
    #[inline(always)]
    fn from(val: Sarie) -> u8 {
        Sarie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbf {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
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
    #[doc = "End transfer on NACK"]
    ENDS_TRANSFER_ON_NACK = 0x0,
    #[doc = "Do not end transfer on NACK"]
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
    #[doc = "SRDR is now empty"]
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
pub enum ScrRst {
    #[doc = "Not reset"]
    NOT_RESET = 0x0,
    #[doc = "Reset"]
    RESET = 0x01,
}
impl ScrRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrRst {
    #[inline(always)]
    fn from(val: u8) -> ScrRst {
        ScrRst::from_bits(val)
    }
}
impl From<ScrRst> for u8 {
    #[inline(always)]
    fn from(val: ScrRst) -> u8 {
        ScrRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrRtf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "STDR is now empty"]
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
pub enum Sdcfg {
    #[doc = "Any Stop condition following an address match"]
    ANY_STOP_AFTER_ADDRESS_MATCH = 0x0,
    #[doc = "Any Stop condition"]
    ANY_STOP = 0x01,
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
pub enum Sdde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Sdde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdde {
    #[inline(always)]
    fn from(val: u8) -> Sdde {
        Sdde::from_bits(val)
    }
}
impl From<Sdde> for u8 {
    #[inline(always)]
    fn from(val: Sdde) -> u8 {
        Sdde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SderRdde {
    #[doc = "Disable DMA request"]
    DISABLED = 0x0,
    #[doc = "Enable DMA request"]
    ENABLED = 0x01,
}
impl SderRdde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SderRdde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SderRdde {
    #[inline(always)]
    fn from(val: u8) -> SderRdde {
        SderRdde::from_bits(val)
    }
}
impl From<SderRdde> for u8 {
    #[inline(always)]
    fn from(val: SderRdde) -> u8 {
        SderRdde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SderTdde {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SderTdde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SderTdde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SderTdde {
    #[inline(always)]
    fn from(val: u8) -> SderTdde {
        SderTdde::from_bits(val)
    }
}
impl From<SderTdde> for u8 {
    #[inline(always)]
    fn from(val: SderTdde) -> u8 {
        SderTdde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sen {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Sen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sen {
    #[inline(always)]
    fn from(val: u8) -> Sen {
        Sen::from_bits(val)
    }
}
impl From<Sen> for u8 {
    #[inline(always)]
    fn from(val: Sen) -> u8 {
        Sen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SierFeie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SierFeie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SierFeie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SierFeie {
    #[inline(always)]
    fn from(val: u8) -> SierFeie {
        SierFeie::from_bits(val)
    }
}
impl From<SierFeie> for u8 {
    #[inline(always)]
    fn from(val: SierFeie) -> u8 {
        SierFeie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SierRdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SierRdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SierRdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SierRdie {
    #[inline(always)]
    fn from(val: u8) -> SierRdie {
        SierRdie::from_bits(val)
    }
}
impl From<SierRdie> for u8 {
    #[inline(always)]
    fn from(val: SierRdie) -> u8 {
        SierRdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SierSdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SierSdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SierSdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SierSdie {
    #[inline(always)]
    fn from(val: u8) -> SierSdie {
        SierSdie::from_bits(val)
    }
}
impl From<SierSdie> for u8 {
    #[inline(always)]
    fn from(val: SierSdie) -> u8 {
        SierSdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SierTdie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SierTdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SierTdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SierTdie {
    #[inline(always)]
    fn from(val: u8) -> SierTdie {
        SierTdie::from_bits(val)
    }
}
impl From<SierTdie> for u8 {
    #[inline(always)]
    fn from(val: SierTdie) -> u8 {
        SierTdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrdrRxempty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl SrdrRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrdrRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrdrRxempty {
    #[inline(always)]
    fn from(val: u8) -> SrdrRxempty {
        SrdrRxempty::from_bits(val)
    }
}
impl From<SrdrRxempty> for u8 {
    #[inline(always)]
    fn from(val: SrdrRxempty) -> u8 {
        SrdrRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrdrSof {
    #[doc = "Not first"]
    NOT_FIRST_DATA_WORD = 0x0,
    #[doc = "First"]
    FIRST_DATA_WORD = 0x01,
}
impl SrdrSof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrdrSof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrdrSof {
    #[inline(always)]
    fn from(val: u8) -> SrdrSof {
        SrdrSof::from_bits(val)
    }
}
impl From<SrdrSof> for u8 {
    #[inline(always)]
    fn from(val: SrdrSof) -> u8 {
        SrdrSof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrdrorRxempty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl SrdrorRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrdrorRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrdrorRxempty {
    #[inline(always)]
    fn from(val: u8) -> SrdrorRxempty {
        SrdrorRxempty::from_bits(val)
    }
}
impl From<SrdrorRxempty> for u8 {
    #[inline(always)]
    fn from(val: SrdrorRxempty) -> u8 {
        SrdrorRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrdrorSof {
    #[doc = "Not the first"]
    NOT_FIRST_DATA_WORD = 0x0,
    #[doc = "First"]
    FIRST_DATA_WORD = 0x01,
}
impl SrdrorSof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrdrorSof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrdrorSof {
    #[inline(always)]
    fn from(val: u8) -> SrdrorSof {
        SrdrorSof::from_bits(val)
    }
}
impl From<SrdrorSof> for u8 {
    #[inline(always)]
    fn from(val: SrdrorSof) -> u8 {
        SrdrorSof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrBbf {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
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
pub enum SsrFef {
    #[doc = "No FIFO error"]
    INT_NO = 0x0,
    #[doc = "FIFO error"]
    INT_YES = 0x01,
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
pub enum SsrRdf {
    #[doc = "Not ready"]
    NOT_READY = 0x0,
    #[doc = "Ready"]
    READY = 0x01,
}
impl SsrRdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrRdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrRdf {
    #[inline(always)]
    fn from(val: u8) -> SsrRdf {
        SsrRdf::from_bits(val)
    }
}
impl From<SsrRdf> for u8 {
    #[inline(always)]
    fn from(val: SsrRdf) -> u8 {
        SsrRdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsrSdf {
    #[doc = "No Stop detected"]
    INT_NO = 0x0,
    #[doc = "Stop detected"]
    INT_YES = 0x01,
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
pub enum SsrTdf {
    #[doc = "Transmit data not requested"]
    NO_FLAG = 0x0,
    #[doc = "Transmit data is requested"]
    FLAG = 0x01,
}
impl SsrTdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsrTdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsrTdf {
    #[inline(always)]
    fn from(val: u8) -> SsrTdf {
        SsrTdf::from_bits(val)
    }
}
impl From<SsrTdf> for u8 {
    #[inline(always)]
    fn from(val: SsrTdf) -> u8 {
        SsrTdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Startcfg {
    #[doc = "Sets when both I2C bus and LPI2C controller are idle"]
    BOTH_I2C_AND_LPI2C_IDLE = 0x0,
    #[doc = "Sets when I2C bus is idle"]
    I2C_IDLE = 0x01,
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
    #[doc = "Start condition not detected"]
    INT_NO = 0x0,
    #[doc = "Start condition detected"]
    INT_YES = 0x01,
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
pub enum Stie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Stie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stie {
    #[inline(always)]
    fn from(val: u8) -> Stie {
        Stie::from_bits(val)
    }
}
impl From<Stie> for u8 {
    #[inline(always)]
    fn from(val: Stie) -> u8 {
        Stie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopcfg {
    #[doc = "Any Stop condition"]
    ANY_STOP = 0x0,
    #[doc = "Last Stop condition"]
    LAST_STOP = 0x01,
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
pub enum Taf {
    #[doc = "Not required"]
    NOT_REQUIRED = 0x0,
    #[doc = "Required"]
    REQUIRED = 0x01,
}
impl Taf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taf {
    #[inline(always)]
    fn from(val: u8) -> Taf {
        Taf::from_bits(val)
    }
}
impl From<Taf> for u8 {
    #[inline(always)]
    fn from(val: Taf) -> u8 {
        Taf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taie {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Taie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taie {
    #[inline(always)]
    fn from(val: u8) -> Taie {
        Taie::from_bits(val)
    }
}
impl From<Taie> for u8 {
    #[inline(always)]
    fn from(val: Taie) -> u8 {
        Taie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timecfg {
    #[doc = "SCL"]
    IF_SCL_LOW = 0x0,
    #[doc = "SCL or SDA"]
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
    #[doc = "MSR\\[TDF\\] is set only during a target-transmit transfer when STDR is empty"]
    ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY = 0x0,
    #[doc = "MSR\\[TDF\\] is set whenever STDR is empty"]
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
pub enum Txdstall {
    #[doc = "Disable"]
    DISABLED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Txdstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdstall {
    #[inline(always)]
    fn from(val: u8) -> Txdstall {
        Txdstall::from_bits(val)
    }
}
impl From<Txdstall> for u8 {
    #[inline(always)]
    fn from(val: Txdstall) -> u8 {
        Txdstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txnack {
    #[doc = "Transmit ACK"]
    TRANSMIT_ACK = 0x0,
    #[doc = "Transmit NACK"]
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
