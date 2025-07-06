#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Adc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc {
    #[inline(always)]
    fn from(val: u8) -> Adc {
        Adc::from_bits(val)
    }
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(val: Adc) -> u8 {
        Adc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Not busy (transaction complete)"]
    NOT_BUSY = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
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
pub enum EccDf {
    #[doc = "No fault"]
    NO_FAULT = 0x0,
    #[doc = "Fault"]
    FAULT = 0x01,
}
impl EccDf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccDf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccDf {
    #[inline(always)]
    fn from(val: u8) -> EccDf {
        EccDf::from_bits(val)
    }
}
impl From<EccDf> for u8 {
    #[inline(always)]
    fn from(val: EccDf) -> u8 {
        EccDf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccSf {
    #[doc = "No fault"]
    NO_FAULT = 0x0,
    #[doc = "Fault"]
    FAULT = 0x01,
}
impl EccSf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccSf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccSf {
    #[inline(always)]
    fn from(val: u8) -> EccSf {
        EccSf::from_bits(val)
    }
}
impl From<EccSf> for u8 {
    #[inline(always)]
    fn from(val: EccSf) -> u8 {
        EccSf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set"]
    pub const STANDARD: Self = Self(0x0);
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
            0x0 => f.write_str("STANDARD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "STANDARD"),
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
pub enum Flc {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Flc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flc {
    #[inline(always)]
    fn from(val: u8) -> Flc {
        Flc::from_bits(val)
    }
}
impl From<Flc> for u8 {
    #[inline(always)]
    fn from(val: Flc) -> u8 {
        Flc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsc {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Fsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsc {
    #[inline(always)]
    fn from(val: u8) -> Fsc {
        Fsc::from_bits(val)
    }
}
impl From<Fsc> for u8 {
    #[inline(always)]
    fn from(val: Fsc) -> u8 {
        Fsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsm {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Fsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsm {
    #[inline(always)]
    fn from(val: u8) -> Fsm {
        Fsm::from_bits(val)
    }
}
impl From<Fsm> for u8 {
    #[inline(always)]
    fn from(val: Fsm) -> u8 {
        Fsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hvreq {
    #[doc = "Turn off"]
    TURN_OFF = 0x0,
    #[doc = "Turn on"]
    TURN_ON = 0x01,
}
impl Hvreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hvreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hvreq {
    #[inline(always)]
    fn from(val: u8) -> Hvreq {
        Hvreq::from_bits(val)
    }
}
impl From<Hvreq> for u8 {
    #[inline(always)]
    fn from(val: Hvreq) -> u8 {
        Hvreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irc {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Irc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irc {
    #[inline(always)]
    fn from(val: u8) -> Irc {
        Irc::from_bits(val)
    }
}
impl From<Irc> for u8 {
    #[inline(always)]
    fn from(val: Irc) -> u8 {
        Irc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvreq {
    #[doc = "Turn off"]
    TURN_OFF = 0x0,
    #[doc = "Turn on"]
    TURN_ON = 0x01,
}
impl Lvreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvreq {
    #[inline(always)]
    fn from(val: u8) -> Lvreq {
        Lvreq::from_bits(val)
    }
}
impl From<Lvreq> for u8 {
    #[inline(always)]
    fn from(val: Lvreq) -> u8 {
        Lvreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdreq {
    #[doc = "PD pin is set to low when OTPC is in idle state. It means EFUSE hardmacro is in standby mode. Idle state means OTPC is not in read and program modes."]
    NO_ACTION = 0x0,
    #[doc = "PD pin is set to high when OTPC is in idle state. It means EFUSE hardmacro is in power down mode."]
    POWER_DN = 0x01,
}
impl Pdreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdreq {
    #[inline(always)]
    fn from(val: u8) -> Pdreq {
        Pdreq::from_bits(val)
    }
}
impl From<Pdreq> for u8 {
    #[inline(always)]
    fn from(val: Pdreq) -> u8 {
        Pdreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RdFuseLock {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl RdFuseLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdFuseLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdFuseLock {
    #[inline(always)]
    fn from(val: u8) -> RdFuseLock {
        RdFuseLock::from_bits(val)
    }
}
impl From<RdFuseLock> for u8 {
    #[inline(always)]
    fn from(val: RdFuseLock) -> u8 {
        RdFuseLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RdRegLock {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl RdRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdRegLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdRegLock {
    #[inline(always)]
    fn from(val: u8) -> RdRegLock {
        RdRegLock::from_bits(val)
    }
}
impl From<RdRegLock> for u8 {
    #[inline(always)]
    fn from(val: RdRegLock) -> u8 {
        RdRegLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadEfuse {
    #[doc = "Starts program operation when the WR_UNLOCK value is 0x9527; otherwise, takes no action."]
    PROGRAM = 0x0,
    #[doc = "Starts read operation"]
    READ = 0x01,
}
impl ReadEfuse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadEfuse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadEfuse {
    #[inline(always)]
    fn from(val: u8) -> ReadEfuse {
        ReadEfuse::from_bits(val)
    }
}
impl From<ReadEfuse> for u8 {
    #[inline(always)]
    fn from(val: ReadEfuse) -> u8 {
        ReadEfuse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadUpdate {
    #[doc = "Shadow register does not update"]
    NO_UPDATE = 0x0,
    #[doc = "Shadow register updates"]
    UPDATE = 0x01,
}
impl ReadUpdate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadUpdate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadUpdate {
    #[inline(always)]
    fn from(val: u8) -> ReadUpdate {
        ReadUpdate::from_bits(val)
    }
}
impl From<ReadUpdate> for u8 {
    #[inline(always)]
    fn from(val: ReadUpdate) -> u8 {
        ReadUpdate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReloadShadows {
    #[doc = "No action (when writing) or reload complete (when reading)"]
    NO_ACTION = 0x0,
    #[doc = "Reload"]
    RELOAD = 0x01,
}
impl ReloadShadows {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReloadShadows {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReloadShadows {
    #[inline(always)]
    fn from(val: u8) -> ReloadShadows {
        ReloadShadows::from_bits(val)
    }
}
impl From<ReloadShadows> for u8 {
    #[inline(always)]
    fn from(val: ReloadShadows) -> u8 {
        ReloadShadows::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TriF {
    #[doc = "No fault"]
    NO_FAULT = 0x0,
    #[doc = "Fault"]
    FAULT = 0x01,
}
impl TriF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TriF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TriF {
    #[inline(always)]
    fn from(val: u8) -> TriF {
        TriF::from_bits(val)
    }
}
impl From<TriF> for u8 {
    #[inline(always)]
    fn from(val: TriF) -> u8 {
        TriF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrAll1s {
    #[doc = "Uses the WDATA value"]
    USE_WDATA = 0x0,
    #[doc = "Writes all 1s"]
    USE_ALL1S = 0x01,
}
impl WrAll1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrAll1s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrAll1s {
    #[inline(always)]
    fn from(val: u8) -> WrAll1s {
        WrAll1s::from_bits(val)
    }
}
impl From<WrAll1s> for u8 {
    #[inline(always)]
    fn from(val: WrAll1s) -> u8 {
        WrAll1s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrFuseLock {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl WrFuseLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrFuseLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrFuseLock {
    #[inline(always)]
    fn from(val: u8) -> WrFuseLock {
        WrFuseLock::from_bits(val)
    }
}
impl From<WrFuseLock> for u8 {
    #[inline(always)]
    fn from(val: WrFuseLock) -> u8 {
        WrFuseLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrPowerOff {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl WrPowerOff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrPowerOff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrPowerOff {
    #[inline(always)]
    fn from(val: u8) -> WrPowerOff {
        WrPowerOff::from_bits(val)
    }
}
impl From<WrPowerOff> for u8 {
    #[inline(always)]
    fn from(val: WrPowerOff) -> u8 {
        WrPowerOff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrRegBusy {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl WrRegBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrRegBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrRegBusy {
    #[inline(always)]
    fn from(val: u8) -> WrRegBusy {
        WrRegBusy::from_bits(val)
    }
}
impl From<WrRegBusy> for u8 {
    #[inline(always)]
    fn from(val: WrRegBusy) -> u8 {
        WrRegBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrRegLock {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl WrRegLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrRegLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrRegLock {
    #[inline(always)]
    fn from(val: u8) -> WrRegLock {
        WrRegLock::from_bits(val)
    }
}
impl From<WrRegLock> for u8 {
    #[inline(always)]
    fn from(val: WrRegLock) -> u8 {
        WrRegLock::to_bits(val)
    }
}
