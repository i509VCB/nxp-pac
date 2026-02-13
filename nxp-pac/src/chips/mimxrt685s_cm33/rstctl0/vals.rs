#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmApdReset {
    #[doc = "No event detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "ARM reset event detected. (Writing a '1' to this bit clears this status)."]
    RESET_DETECTED = 0x01,
}
impl ArmApdReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmApdReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmApdReset {
    #[inline(always)]
    fn from(val: u8) -> ArmApdReset {
        ArmApdReset::from_bits(val)
    }
}
impl From<ArmApdReset> for u8 {
    #[inline(always)]
    fn from(val: ArmApdReset) -> u8 {
        ArmApdReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PadReset {
    #[doc = "No EVENT Detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "RESET Detected. (Write 1 to CLR),"]
    RESET_DETECTED = 0x01,
}
impl PadReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PadReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PadReset {
    #[inline(always)]
    fn from(val: u8) -> PadReset {
        PadReset::from_bits(val)
    }
}
impl From<PadReset> for u8 {
    #[inline(always)]
    fn from(val: PadReset) -> u8 {
        PadReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Casper {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Casper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Casper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Casper {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Casper {
        Prstctl0Casper::from_bits(val)
    }
}
impl From<Prstctl0Casper> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Casper) -> u8 {
        Prstctl0Casper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrCasper {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrCasper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrCasper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrCasper {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrCasper {
        Prstctl0ClrCasper::from_bits(val)
    }
}
impl From<Prstctl0ClrCasper> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrCasper) -> u8 {
        Prstctl0ClrCasper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrFlexspiOtfad {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrFlexspiOtfad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrFlexspiOtfad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrFlexspiOtfad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrFlexspiOtfad {
        Prstctl0ClrFlexspiOtfad::from_bits(val)
    }
}
impl From<Prstctl0ClrFlexspiOtfad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrFlexspiOtfad) -> u8 {
        Prstctl0ClrFlexspiOtfad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrHashcrypt {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrHashcrypt {
        Prstctl0ClrHashcrypt::from_bits(val)
    }
}
impl From<Prstctl0ClrHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrHashcrypt) -> u8 {
        Prstctl0ClrHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrHifiDsp {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrHifiDsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrHifiDsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrHifiDsp {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrHifiDsp {
        Prstctl0ClrHifiDsp::from_bits(val)
    }
}
impl From<Prstctl0ClrHifiDsp> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrHifiDsp) -> u8 {
        Prstctl0ClrHifiDsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrPowerquad {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrPowerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrPowerquad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrPowerquad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrPowerquad {
        Prstctl0ClrPowerquad::from_bits(val)
    }
}
impl From<Prstctl0ClrPowerquad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrPowerquad) -> u8 {
        Prstctl0ClrPowerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrPuf {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrPuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrPuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrPuf {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrPuf {
        Prstctl0ClrPuf::from_bits(val)
    }
}
impl From<Prstctl0ClrPuf> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrPuf) -> u8 {
        Prstctl0ClrPuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrRng {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrRng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrRng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrRng {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrRng {
        Prstctl0ClrRng::from_bits(val)
    }
}
impl From<Prstctl0ClrRng> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrRng) -> u8 {
        Prstctl0ClrRng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrSct {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrSct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrSct {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrSct {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrSct {
        Prstctl0ClrSct::from_bits(val)
    }
}
impl From<Prstctl0ClrSct> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrSct) -> u8 {
        Prstctl0ClrSct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrUsbhsDevice {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrUsbhsDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrUsbhsDevice {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrUsbhsDevice {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrUsbhsDevice {
        Prstctl0ClrUsbhsDevice::from_bits(val)
    }
}
impl From<Prstctl0ClrUsbhsDevice> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrUsbhsDevice) -> u8 {
        Prstctl0ClrUsbhsDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrUsbhsHost {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrUsbhsHost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrUsbhsHost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrUsbhsHost {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrUsbhsHost {
        Prstctl0ClrUsbhsHost::from_bits(val)
    }
}
impl From<Prstctl0ClrUsbhsHost> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrUsbhsHost) -> u8 {
        Prstctl0ClrUsbhsHost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrUsbhsPhy {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrUsbhsPhy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrUsbhsPhy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrUsbhsPhy {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrUsbhsPhy {
        Prstctl0ClrUsbhsPhy::from_bits(val)
    }
}
impl From<Prstctl0ClrUsbhsPhy> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrUsbhsPhy) -> u8 {
        Prstctl0ClrUsbhsPhy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0ClrUsbhsSram {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl0ClrUsbhsSram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0ClrUsbhsSram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0ClrUsbhsSram {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0ClrUsbhsSram {
        Prstctl0ClrUsbhsSram::from_bits(val)
    }
}
impl From<Prstctl0ClrUsbhsSram> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0ClrUsbhsSram) -> u8 {
        Prstctl0ClrUsbhsSram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0FlexspiOtfad {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0FlexspiOtfad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0FlexspiOtfad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0FlexspiOtfad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0FlexspiOtfad {
        Prstctl0FlexspiOtfad::from_bits(val)
    }
}
impl From<Prstctl0FlexspiOtfad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0FlexspiOtfad) -> u8 {
        Prstctl0FlexspiOtfad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Hashcrypt {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Hashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Hashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Hashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Hashcrypt {
        Prstctl0Hashcrypt::from_bits(val)
    }
}
impl From<Prstctl0Hashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Hashcrypt) -> u8 {
        Prstctl0Hashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0HifiDsp {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0HifiDsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0HifiDsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0HifiDsp {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0HifiDsp {
        Prstctl0HifiDsp::from_bits(val)
    }
}
impl From<Prstctl0HifiDsp> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0HifiDsp) -> u8 {
        Prstctl0HifiDsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Powerquad {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Powerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Powerquad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Powerquad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Powerquad {
        Prstctl0Powerquad::from_bits(val)
    }
}
impl From<Prstctl0Powerquad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Powerquad) -> u8 {
        Prstctl0Powerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Puf {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Puf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Puf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Puf {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Puf {
        Prstctl0Puf::from_bits(val)
    }
}
impl From<Prstctl0Puf> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Puf) -> u8 {
        Prstctl0Puf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Rng {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Rng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Rng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Rng {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Rng {
        Prstctl0Rng::from_bits(val)
    }
}
impl From<Prstctl0Rng> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Rng) -> u8 {
        Prstctl0Rng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0Sct {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0Sct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0Sct {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0Sct {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0Sct {
        Prstctl0Sct::from_bits(val)
    }
}
impl From<Prstctl0Sct> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0Sct) -> u8 {
        Prstctl0Sct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetCasper {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetCasper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetCasper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetCasper {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetCasper {
        Prstctl0SetCasper::from_bits(val)
    }
}
impl From<Prstctl0SetCasper> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetCasper) -> u8 {
        Prstctl0SetCasper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetFlexspiOtfad {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetFlexspiOtfad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetFlexspiOtfad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetFlexspiOtfad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetFlexspiOtfad {
        Prstctl0SetFlexspiOtfad::from_bits(val)
    }
}
impl From<Prstctl0SetFlexspiOtfad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetFlexspiOtfad) -> u8 {
        Prstctl0SetFlexspiOtfad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetHashcrypt {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetHashcrypt {
        Prstctl0SetHashcrypt::from_bits(val)
    }
}
impl From<Prstctl0SetHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetHashcrypt) -> u8 {
        Prstctl0SetHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetHifiDsp {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetHifiDsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetHifiDsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetHifiDsp {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetHifiDsp {
        Prstctl0SetHifiDsp::from_bits(val)
    }
}
impl From<Prstctl0SetHifiDsp> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetHifiDsp) -> u8 {
        Prstctl0SetHifiDsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetPowerquad {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetPowerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetPowerquad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetPowerquad {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetPowerquad {
        Prstctl0SetPowerquad::from_bits(val)
    }
}
impl From<Prstctl0SetPowerquad> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetPowerquad) -> u8 {
        Prstctl0SetPowerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetPuf {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetPuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetPuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetPuf {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetPuf {
        Prstctl0SetPuf::from_bits(val)
    }
}
impl From<Prstctl0SetPuf> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetPuf) -> u8 {
        Prstctl0SetPuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetRng {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetRng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetRng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetRng {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetRng {
        Prstctl0SetRng::from_bits(val)
    }
}
impl From<Prstctl0SetRng> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetRng) -> u8 {
        Prstctl0SetRng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetSct {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetSct {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetSct {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetSct {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetSct {
        Prstctl0SetSct::from_bits(val)
    }
}
impl From<Prstctl0SetSct> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetSct) -> u8 {
        Prstctl0SetSct::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetUsbhsDevice {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetUsbhsDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetUsbhsDevice {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetUsbhsDevice {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetUsbhsDevice {
        Prstctl0SetUsbhsDevice::from_bits(val)
    }
}
impl From<Prstctl0SetUsbhsDevice> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetUsbhsDevice) -> u8 {
        Prstctl0SetUsbhsDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetUsbhsHost {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetUsbhsHost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetUsbhsHost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetUsbhsHost {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetUsbhsHost {
        Prstctl0SetUsbhsHost::from_bits(val)
    }
}
impl From<Prstctl0SetUsbhsHost> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetUsbhsHost) -> u8 {
        Prstctl0SetUsbhsHost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetUsbhsPhy {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetUsbhsPhy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetUsbhsPhy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetUsbhsPhy {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetUsbhsPhy {
        Prstctl0SetUsbhsPhy::from_bits(val)
    }
}
impl From<Prstctl0SetUsbhsPhy> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetUsbhsPhy) -> u8 {
        Prstctl0SetUsbhsPhy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0SetUsbhsSram {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl0SetUsbhsSram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0SetUsbhsSram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0SetUsbhsSram {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0SetUsbhsSram {
        Prstctl0SetUsbhsSram::from_bits(val)
    }
}
impl From<Prstctl0SetUsbhsSram> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0SetUsbhsSram) -> u8 {
        Prstctl0SetUsbhsSram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0UsbhsDevice {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0UsbhsDevice {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0UsbhsDevice {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0UsbhsDevice {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0UsbhsDevice {
        Prstctl0UsbhsDevice::from_bits(val)
    }
}
impl From<Prstctl0UsbhsDevice> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0UsbhsDevice) -> u8 {
        Prstctl0UsbhsDevice::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0UsbhsHost {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0UsbhsHost {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0UsbhsHost {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0UsbhsHost {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0UsbhsHost {
        Prstctl0UsbhsHost::from_bits(val)
    }
}
impl From<Prstctl0UsbhsHost> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0UsbhsHost) -> u8 {
        Prstctl0UsbhsHost::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0UsbhsPhy {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0UsbhsPhy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0UsbhsPhy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0UsbhsPhy {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0UsbhsPhy {
        Prstctl0UsbhsPhy::from_bits(val)
    }
}
impl From<Prstctl0UsbhsPhy> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0UsbhsPhy) -> u8 {
        Prstctl0UsbhsPhy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl0UsbhsSram {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl0UsbhsSram {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl0UsbhsSram {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl0UsbhsSram {
    #[inline(always)]
    fn from(val: u8) -> Prstctl0UsbhsSram {
        Prstctl0UsbhsSram::from_bits(val)
    }
}
impl From<Prstctl0UsbhsSram> for u8 {
    #[inline(always)]
    fn from(val: Prstctl0UsbhsSram) -> u8 {
        Prstctl0UsbhsSram::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1Acmp0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl1Acmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1Acmp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1Acmp0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1Acmp0 {
        Prstctl1Acmp0::from_bits(val)
    }
}
impl From<Prstctl1Acmp0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1Acmp0) -> u8 {
        Prstctl1Acmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1Adc0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl1Adc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1Adc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1Adc0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1Adc0 {
        Prstctl1Adc0::from_bits(val)
    }
}
impl From<Prstctl1Adc0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1Adc0) -> u8 {
        Prstctl1Adc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1ClrAcmp0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl1ClrAcmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1ClrAcmp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1ClrAcmp0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1ClrAcmp0 {
        Prstctl1ClrAcmp0::from_bits(val)
    }
}
impl From<Prstctl1ClrAcmp0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1ClrAcmp0) -> u8 {
        Prstctl1ClrAcmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1ClrAdc0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl1ClrAdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1ClrAdc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1ClrAdc0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1ClrAdc0 {
        Prstctl1ClrAdc0::from_bits(val)
    }
}
impl From<Prstctl1ClrAdc0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1ClrAdc0) -> u8 {
        Prstctl1ClrAdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1ClrSdio0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl1ClrSdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1ClrSdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1ClrSdio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1ClrSdio0 {
        Prstctl1ClrSdio0::from_bits(val)
    }
}
impl From<Prstctl1ClrSdio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1ClrSdio0) -> u8 {
        Prstctl1ClrSdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1ClrSdio1 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl1ClrSdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1ClrSdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1ClrSdio1 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1ClrSdio1 {
        Prstctl1ClrSdio1::from_bits(val)
    }
}
impl From<Prstctl1ClrSdio1> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1ClrSdio1) -> u8 {
        Prstctl1ClrSdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1ClrShsgpio0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl1ClrShsgpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1ClrShsgpio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1ClrShsgpio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1ClrShsgpio0 {
        Prstctl1ClrShsgpio0::from_bits(val)
    }
}
impl From<Prstctl1ClrShsgpio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1ClrShsgpio0) -> u8 {
        Prstctl1ClrShsgpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1Sdio0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl1Sdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1Sdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1Sdio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1Sdio0 {
        Prstctl1Sdio0::from_bits(val)
    }
}
impl From<Prstctl1Sdio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1Sdio0) -> u8 {
        Prstctl1Sdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1Sdio1 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl1Sdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1Sdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1Sdio1 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1Sdio1 {
        Prstctl1Sdio1::from_bits(val)
    }
}
impl From<Prstctl1Sdio1> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1Sdio1) -> u8 {
        Prstctl1Sdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1SetAcmp0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl1SetAcmp0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1SetAcmp0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1SetAcmp0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1SetAcmp0 {
        Prstctl1SetAcmp0::from_bits(val)
    }
}
impl From<Prstctl1SetAcmp0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1SetAcmp0) -> u8 {
        Prstctl1SetAcmp0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1SetAdc0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl1SetAdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1SetAdc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1SetAdc0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1SetAdc0 {
        Prstctl1SetAdc0::from_bits(val)
    }
}
impl From<Prstctl1SetAdc0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1SetAdc0) -> u8 {
        Prstctl1SetAdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1SetSdio0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl1SetSdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1SetSdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1SetSdio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1SetSdio0 {
        Prstctl1SetSdio0::from_bits(val)
    }
}
impl From<Prstctl1SetSdio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1SetSdio0) -> u8 {
        Prstctl1SetSdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1SetSdio1 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl1SetSdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1SetSdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1SetSdio1 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1SetSdio1 {
        Prstctl1SetSdio1::from_bits(val)
    }
}
impl From<Prstctl1SetSdio1> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1SetSdio1) -> u8 {
        Prstctl1SetSdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1SetShsgpio0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl1SetShsgpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1SetShsgpio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1SetShsgpio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1SetShsgpio0 {
        Prstctl1SetShsgpio0::from_bits(val)
    }
}
impl From<Prstctl1SetShsgpio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1SetShsgpio0) -> u8 {
        Prstctl1SetShsgpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl1Shsgpio0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl1Shsgpio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl1Shsgpio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl1Shsgpio0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl1Shsgpio0 {
        Prstctl1Shsgpio0::from_bits(val)
    }
}
impl From<Prstctl1Shsgpio0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl1Shsgpio0) -> u8 {
        Prstctl1Shsgpio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2ClrUtick0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl2ClrUtick0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2ClrUtick0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2ClrUtick0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2ClrUtick0 {
        Prstctl2ClrUtick0::from_bits(val)
    }
}
impl From<Prstctl2ClrUtick0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2ClrUtick0) -> u8 {
        Prstctl2ClrUtick0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2ClrWwdt0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Prstctl2ClrWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2ClrWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2ClrWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2ClrWwdt0 {
        Prstctl2ClrWwdt0::from_bits(val)
    }
}
impl From<Prstctl2ClrWwdt0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2ClrWwdt0) -> u8 {
        Prstctl2ClrWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2SetUtick0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl2SetUtick0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2SetUtick0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2SetUtick0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2SetUtick0 {
        Prstctl2SetUtick0::from_bits(val)
    }
}
impl From<Prstctl2SetUtick0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2SetUtick0) -> u8 {
        Prstctl2SetUtick0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2SetWwdt0 {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Prstctl2SetWwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2SetWwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2SetWwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2SetWwdt0 {
        Prstctl2SetWwdt0::from_bits(val)
    }
}
impl From<Prstctl2SetWwdt0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2SetWwdt0) -> u8 {
        Prstctl2SetWwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2Utick0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl2Utick0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2Utick0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2Utick0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2Utick0 {
        Prstctl2Utick0::from_bits(val)
    }
}
impl From<Prstctl2Utick0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2Utick0) -> u8 {
        Prstctl2Utick0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prstctl2Wwdt0 {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Prstctl2Wwdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prstctl2Wwdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prstctl2Wwdt0 {
    #[inline(always)]
    fn from(val: u8) -> Prstctl2Wwdt0 {
        Prstctl2Wwdt0::from_bits(val)
    }
}
impl From<Prstctl2Wwdt0> for u8 {
    #[inline(always)]
    fn from(val: Prstctl2Wwdt0) -> u8 {
        Prstctl2Wwdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VddPor {
    #[doc = "No event detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "VDD POR event detected. (Writing a '1' to this bit clears this status)."]
    POR_EVENT_DETECTED = 0x01,
}
impl VddPor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VddPor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VddPor {
    #[inline(always)]
    fn from(val: u8) -> VddPor {
        VddPor::from_bits(val)
    }
}
impl From<VddPor> for u8 {
    #[inline(always)]
    fn from(val: VddPor) -> u8 {
        VddPor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0Reset {
    #[doc = "No EVENT Detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "WDT0 reset event detected. (Writing a '1' to this bit clears this status)."]
    WDT0_RESET_DETECTED = 0x01,
}
impl Wdt0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0Reset {
    #[inline(always)]
    fn from(val: u8) -> Wdt0Reset {
        Wdt0Reset::from_bits(val)
    }
}
impl From<Wdt0Reset> for u8 {
    #[inline(always)]
    fn from(val: Wdt0Reset) -> u8 {
        Wdt0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1Reset {
    #[doc = "No EVENT Detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "WDT1 reset event detected. (Writing a 1 to this bit clears this status)."]
    WDT1_RESET_DETECTED = 0x01,
}
impl Wdt1Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1Reset {
    #[inline(always)]
    fn from(val: u8) -> Wdt1Reset {
        Wdt1Reset::from_bits(val)
    }
}
impl From<Wdt1Reset> for u8 {
    #[inline(always)]
    fn from(val: Wdt1Reset) -> u8 {
        Wdt1Reset::to_bits(val)
    }
}
