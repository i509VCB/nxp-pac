#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmApdReset {
    #[doc = "No event detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "ARM reset event detected. (Writing a 1 to this bit clears this status)."]
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
pub enum CrcRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl CrcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRst {
    #[inline(always)]
    fn from(val: u8) -> CrcRst {
        CrcRst::from_bits(val)
    }
}
impl From<CrcRst> for u8 {
    #[inline(always)]
    fn from(val: CrcRst) -> u8 {
        CrcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl CrcRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRstClr {
    #[inline(always)]
    fn from(val: u8) -> CrcRstClr {
        CrcRstClr::from_bits(val)
    }
}
impl From<CrcRstClr> for u8 {
    #[inline(always)]
    fn from(val: CrcRstClr) -> u8 {
        CrcRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl CrcRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRstSet {
    #[inline(always)]
    fn from(val: u8) -> CrcRstSet {
        CrcRstSet::from_bits(val)
    }
}
impl From<CrcRstSet> for u8 {
    #[inline(always)]
    fn from(val: CrcRstSet) -> u8 {
        CrcRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Ct32bit0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0Rst {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0Rst {
        Ct32bit0Rst::from_bits(val)
    }
}
impl From<Ct32bit0Rst> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0Rst) -> u8 {
        Ct32bit0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Ct32bit0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0RstClr {
        Ct32bit0RstClr::from_bits(val)
    }
}
impl From<Ct32bit0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0RstClr) -> u8 {
        Ct32bit0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Ct32bit0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit0RstSet {
        Ct32bit0RstSet::from_bits(val)
    }
}
impl From<Ct32bit0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit0RstSet) -> u8 {
        Ct32bit0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Ct32bit1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1Rst {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1Rst {
        Ct32bit1Rst::from_bits(val)
    }
}
impl From<Ct32bit1Rst> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1Rst) -> u8 {
        Ct32bit1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Ct32bit1RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1RstClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1RstClr {
        Ct32bit1RstClr::from_bits(val)
    }
}
impl From<Ct32bit1RstClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1RstClr) -> u8 {
        Ct32bit1RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit1RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Ct32bit1RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit1RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit1RstSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit1RstSet {
        Ct32bit1RstSet::from_bits(val)
    }
}
impl From<Ct32bit1RstSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit1RstSet) -> u8 {
        Ct32bit1RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Ct32bit2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2Rst {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2Rst {
        Ct32bit2Rst::from_bits(val)
    }
}
impl From<Ct32bit2Rst> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2Rst) -> u8 {
        Ct32bit2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Ct32bit2RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2RstClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2RstClr {
        Ct32bit2RstClr::from_bits(val)
    }
}
impl From<Ct32bit2RstClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2RstClr) -> u8 {
        Ct32bit2RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit2RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Ct32bit2RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit2RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit2RstSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit2RstSet {
        Ct32bit2RstSet::from_bits(val)
    }
}
impl From<Ct32bit2RstSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit2RstSet) -> u8 {
        Ct32bit2RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Ct32bit3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3Rst {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3Rst {
        Ct32bit3Rst::from_bits(val)
    }
}
impl From<Ct32bit3Rst> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3Rst) -> u8 {
        Ct32bit3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Ct32bit3RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3RstClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3RstClr {
        Ct32bit3RstClr::from_bits(val)
    }
}
impl From<Ct32bit3RstClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3RstClr) -> u8 {
        Ct32bit3RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit3RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Ct32bit3RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit3RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit3RstSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit3RstSet {
        Ct32bit3RstSet::from_bits(val)
    }
}
impl From<Ct32bit3RstSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit3RstSet) -> u8 {
        Ct32bit3RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Ct32bit4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4Rst {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4Rst {
        Ct32bit4Rst::from_bits(val)
    }
}
impl From<Ct32bit4Rst> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4Rst) -> u8 {
        Ct32bit4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Ct32bit4RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4RstClr {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4RstClr {
        Ct32bit4RstClr::from_bits(val)
    }
}
impl From<Ct32bit4RstClr> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4RstClr) -> u8 {
        Ct32bit4RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ct32bit4RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Ct32bit4RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ct32bit4RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ct32bit4RstSet {
    #[inline(always)]
    fn from(val: u8) -> Ct32bit4RstSet {
        Ct32bit4RstSet::from_bits(val)
    }
}
impl From<Ct32bit4RstSet> for u8 {
    #[inline(always)]
    fn from(val: Ct32bit4RstSet) -> u8 {
        Ct32bit4RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Dmac0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dmac0Rst {
        Dmac0Rst::from_bits(val)
    }
}
impl From<Dmac0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dmac0Rst) -> u8 {
        Dmac0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Dmac0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Dmac0RstClr {
        Dmac0RstClr::from_bits(val)
    }
}
impl From<Dmac0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Dmac0RstClr) -> u8 {
        Dmac0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Dmac0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Dmac0RstSet {
        Dmac0RstSet::from_bits(val)
    }
}
impl From<Dmac0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Dmac0RstSet) -> u8 {
        Dmac0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Dmac1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dmac1Rst {
        Dmac1Rst::from_bits(val)
    }
}
impl From<Dmac1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dmac1Rst) -> u8 {
        Dmac1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Dmac1RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1RstClr {
    #[inline(always)]
    fn from(val: u8) -> Dmac1RstClr {
        Dmac1RstClr::from_bits(val)
    }
}
impl From<Dmac1RstClr> for u8 {
    #[inline(always)]
    fn from(val: Dmac1RstClr) -> u8 {
        Dmac1RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Dmac1RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1RstSet {
    #[inline(always)]
    fn from(val: u8) -> Dmac1RstSet {
        Dmac1RstSet::from_bits(val)
    }
}
impl From<Dmac1RstSet> for u8 {
    #[inline(always)]
    fn from(val: Dmac1RstSet) -> u8 {
        Dmac1RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Dmic0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dmic0Rst {
        Dmic0Rst::from_bits(val)
    }
}
impl From<Dmic0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dmic0Rst) -> u8 {
        Dmic0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Dmic0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Dmic0RstClr {
        Dmic0RstClr::from_bits(val)
    }
}
impl From<Dmic0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Dmic0RstClr) -> u8 {
        Dmic0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmic0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Dmic0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmic0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmic0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Dmic0RstSet {
        Dmic0RstSet::from_bits(val)
    }
}
impl From<Dmic0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Dmic0RstSet) -> u8 {
        Dmic0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0Rst {
        Flexcomm0Rst::from_bits(val)
    }
}
impl From<Flexcomm0Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0Rst) -> u8 {
        Flexcomm0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0RstClr {
        Flexcomm0RstClr::from_bits(val)
    }
}
impl From<Flexcomm0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0RstClr) -> u8 {
        Flexcomm0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0RstSet {
        Flexcomm0RstSet::from_bits(val)
    }
}
impl From<Flexcomm0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0RstSet) -> u8 {
        Flexcomm0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm14SpiRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm14SpiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm14SpiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm14SpiRst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm14SpiRst {
        Flexcomm14SpiRst::from_bits(val)
    }
}
impl From<Flexcomm14SpiRst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm14SpiRst) -> u8 {
        Flexcomm14SpiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm14SpiRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm14SpiRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm14SpiRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm14SpiRstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm14SpiRstClr {
        Flexcomm14SpiRstClr::from_bits(val)
    }
}
impl From<Flexcomm14SpiRstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm14SpiRstClr) -> u8 {
        Flexcomm14SpiRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm14SpiRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm14SpiRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm14SpiRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm14SpiRstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm14SpiRstSet {
        Flexcomm14SpiRstSet::from_bits(val)
    }
}
impl From<Flexcomm14SpiRstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm14SpiRstSet) -> u8 {
        Flexcomm14SpiRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm15I2cRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm15I2cRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm15I2cRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm15I2cRst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm15I2cRst {
        Flexcomm15I2cRst::from_bits(val)
    }
}
impl From<Flexcomm15I2cRst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm15I2cRst) -> u8 {
        Flexcomm15I2cRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm15I2cRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm15I2cRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm15I2cRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm15I2cRstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm15I2cRstClr {
        Flexcomm15I2cRstClr::from_bits(val)
    }
}
impl From<Flexcomm15I2cRstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm15I2cRstClr) -> u8 {
        Flexcomm15I2cRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm15I2cRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm15I2cRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm15I2cRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm15I2cRstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm15I2cRstSet {
        Flexcomm15I2cRstSet::from_bits(val)
    }
}
impl From<Flexcomm15I2cRstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm15I2cRstSet) -> u8 {
        Flexcomm15I2cRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1Rst {
        Flexcomm1Rst::from_bits(val)
    }
}
impl From<Flexcomm1Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1Rst) -> u8 {
        Flexcomm1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm1RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1RstClr {
        Flexcomm1RstClr::from_bits(val)
    }
}
impl From<Flexcomm1RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1RstClr) -> u8 {
        Flexcomm1RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm1RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1RstSet {
        Flexcomm1RstSet::from_bits(val)
    }
}
impl From<Flexcomm1RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1RstSet) -> u8 {
        Flexcomm1RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2Rst {
        Flexcomm2Rst::from_bits(val)
    }
}
impl From<Flexcomm2Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2Rst) -> u8 {
        Flexcomm2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm2RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2RstClr {
        Flexcomm2RstClr::from_bits(val)
    }
}
impl From<Flexcomm2RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2RstClr) -> u8 {
        Flexcomm2RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm2RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2RstSet {
        Flexcomm2RstSet::from_bits(val)
    }
}
impl From<Flexcomm2RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2RstSet) -> u8 {
        Flexcomm2RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3Rst {
        Flexcomm3Rst::from_bits(val)
    }
}
impl From<Flexcomm3Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3Rst) -> u8 {
        Flexcomm3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm3RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3RstClr {
        Flexcomm3RstClr::from_bits(val)
    }
}
impl From<Flexcomm3RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3RstClr) -> u8 {
        Flexcomm3RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm3RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3RstSet {
        Flexcomm3RstSet::from_bits(val)
    }
}
impl From<Flexcomm3RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3RstSet) -> u8 {
        Flexcomm3RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4Rst {
        Flexcomm4Rst::from_bits(val)
    }
}
impl From<Flexcomm4Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4Rst) -> u8 {
        Flexcomm4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm4RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4RstClr {
        Flexcomm4RstClr::from_bits(val)
    }
}
impl From<Flexcomm4RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4RstClr) -> u8 {
        Flexcomm4RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm4RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4RstSet {
        Flexcomm4RstSet::from_bits(val)
    }
}
impl From<Flexcomm4RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4RstSet) -> u8 {
        Flexcomm4RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm5Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5Rst {
        Flexcomm5Rst::from_bits(val)
    }
}
impl From<Flexcomm5Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5Rst) -> u8 {
        Flexcomm5Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm5RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5RstClr {
        Flexcomm5RstClr::from_bits(val)
    }
}
impl From<Flexcomm5RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5RstClr) -> u8 {
        Flexcomm5RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm5RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5RstSet {
        Flexcomm5RstSet::from_bits(val)
    }
}
impl From<Flexcomm5RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5RstSet) -> u8 {
        Flexcomm5RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm6Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6Rst {
        Flexcomm6Rst::from_bits(val)
    }
}
impl From<Flexcomm6Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6Rst) -> u8 {
        Flexcomm6Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm6RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6RstClr {
        Flexcomm6RstClr::from_bits(val)
    }
}
impl From<Flexcomm6RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6RstClr) -> u8 {
        Flexcomm6RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm6RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6RstSet {
        Flexcomm6RstSet::from_bits(val)
    }
}
impl From<Flexcomm6RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6RstSet) -> u8 {
        Flexcomm6RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Flexcomm7Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7Rst {
        Flexcomm7Rst::from_bits(val)
    }
}
impl From<Flexcomm7Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7Rst) -> u8 {
        Flexcomm7Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl Flexcomm7RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7RstClr {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7RstClr {
        Flexcomm7RstClr::from_bits(val)
    }
}
impl From<Flexcomm7RstClr> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7RstClr) -> u8 {
        Flexcomm7RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm7RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl Flexcomm7RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm7RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm7RstSet {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm7RstSet {
        Flexcomm7RstSet::from_bits(val)
    }
}
impl From<Flexcomm7RstSet> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm7RstSet) -> u8 {
        Flexcomm7RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl FreqmeRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRstClr {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRstClr {
        FreqmeRstClr::from_bits(val)
    }
}
impl From<FreqmeRstClr> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRstClr) -> u8 {
        FreqmeRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl FreqmeRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRstSet {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRstSet {
        FreqmeRstSet::from_bits(val)
    }
}
impl From<FreqmeRstSet> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRstSet) -> u8 {
        FreqmeRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl GpiointctlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlRst {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlRst {
        GpiointctlRst::from_bits(val)
    }
}
impl From<GpiointctlRst> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlRst) -> u8 {
        GpiointctlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl GpiointctlRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlRstClr {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlRstClr {
        GpiointctlRstClr::from_bits(val)
    }
}
impl From<GpiointctlRstClr> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlRstClr) -> u8 {
        GpiointctlRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointctlRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl GpiointctlRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointctlRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointctlRstSet {
    #[inline(always)]
    fn from(val: u8) -> GpiointctlRstSet {
        GpiointctlRstSet::from_bits(val)
    }
}
impl From<GpiointctlRstSet> for u8 {
    #[inline(always)]
    fn from(val: GpiointctlRstSet) -> u8 {
        GpiointctlRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0Rst {
        Hsgpio0Rst::from_bits(val)
    }
}
impl From<Hsgpio0Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0Rst) -> u8 {
        Hsgpio0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0RstClr {
        Hsgpio0RstClr::from_bits(val)
    }
}
impl From<Hsgpio0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0RstClr) -> u8 {
        Hsgpio0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio0RstSet {
        Hsgpio0RstSet::from_bits(val)
    }
}
impl From<Hsgpio0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio0RstSet) -> u8 {
        Hsgpio0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1Rst {
        Hsgpio1Rst::from_bits(val)
    }
}
impl From<Hsgpio1Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1Rst) -> u8 {
        Hsgpio1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio1RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1RstClr {
        Hsgpio1RstClr::from_bits(val)
    }
}
impl From<Hsgpio1RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1RstClr) -> u8 {
        Hsgpio1RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio1RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio1RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio1RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio1RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio1RstSet {
        Hsgpio1RstSet::from_bits(val)
    }
}
impl From<Hsgpio1RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio1RstSet) -> u8 {
        Hsgpio1RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2Rst {
        Hsgpio2Rst::from_bits(val)
    }
}
impl From<Hsgpio2Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2Rst) -> u8 {
        Hsgpio2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio2RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2RstClr {
        Hsgpio2RstClr::from_bits(val)
    }
}
impl From<Hsgpio2RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2RstClr) -> u8 {
        Hsgpio2RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio2RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio2RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio2RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio2RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio2RstSet {
        Hsgpio2RstSet::from_bits(val)
    }
}
impl From<Hsgpio2RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio2RstSet) -> u8 {
        Hsgpio2RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3Rst {
        Hsgpio3Rst::from_bits(val)
    }
}
impl From<Hsgpio3Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3Rst) -> u8 {
        Hsgpio3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio3RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3RstClr {
        Hsgpio3RstClr::from_bits(val)
    }
}
impl From<Hsgpio3RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3RstClr) -> u8 {
        Hsgpio3RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio3RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio3RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio3RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio3RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio3RstSet {
        Hsgpio3RstSet::from_bits(val)
    }
}
impl From<Hsgpio3RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio3RstSet) -> u8 {
        Hsgpio3RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4Rst {
        Hsgpio4Rst::from_bits(val)
    }
}
impl From<Hsgpio4Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4Rst) -> u8 {
        Hsgpio4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio4RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4RstClr {
        Hsgpio4RstClr::from_bits(val)
    }
}
impl From<Hsgpio4RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4RstClr) -> u8 {
        Hsgpio4RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio4RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio4RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio4RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio4RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio4RstSet {
        Hsgpio4RstSet::from_bits(val)
    }
}
impl From<Hsgpio4RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio4RstSet) -> u8 {
        Hsgpio4RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio5Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5Rst {
        Hsgpio5Rst::from_bits(val)
    }
}
impl From<Hsgpio5Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5Rst) -> u8 {
        Hsgpio5Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio5RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5RstClr {
        Hsgpio5RstClr::from_bits(val)
    }
}
impl From<Hsgpio5RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5RstClr) -> u8 {
        Hsgpio5RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio5RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio5RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio5RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio5RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio5RstSet {
        Hsgpio5RstSet::from_bits(val)
    }
}
impl From<Hsgpio5RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio5RstSet) -> u8 {
        Hsgpio5RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio6Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6Rst {
        Hsgpio6Rst::from_bits(val)
    }
}
impl From<Hsgpio6Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6Rst) -> u8 {
        Hsgpio6Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio6RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6RstClr {
        Hsgpio6RstClr::from_bits(val)
    }
}
impl From<Hsgpio6RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6RstClr) -> u8 {
        Hsgpio6RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio6RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio6RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio6RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio6RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio6RstSet {
        Hsgpio6RstSet::from_bits(val)
    }
}
impl From<Hsgpio6RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio6RstSet) -> u8 {
        Hsgpio6RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Hsgpio7Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7Rst {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7Rst {
        Hsgpio7Rst::from_bits(val)
    }
}
impl From<Hsgpio7Rst> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7Rst) -> u8 {
        Hsgpio7Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl Hsgpio7RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7RstClr {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7RstClr {
        Hsgpio7RstClr::from_bits(val)
    }
}
impl From<Hsgpio7RstClr> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7RstClr) -> u8 {
        Hsgpio7RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsgpio7RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl Hsgpio7RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsgpio7RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsgpio7RstSet {
    #[inline(always)]
    fn from(val: u8) -> Hsgpio7RstSet {
        Hsgpio7RstSet::from_bits(val)
    }
}
impl From<Hsgpio7RstSet> for u8 {
    #[inline(always)]
    fn from(val: Hsgpio7RstSet) -> u8 {
        Hsgpio7RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl I3c0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c0Rst {
        I3c0Rst::from_bits(val)
    }
}
impl From<I3c0Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c0Rst) -> u8 {
        I3c0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl I3c0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0RstSet {
    #[inline(always)]
    fn from(val: u8) -> I3c0RstSet {
        I3c0RstSet::from_bits(val)
    }
}
impl From<I3c0RstSet> for u8 {
    #[inline(always)]
    fn from(val: I3c0RstSet) -> u8 {
        I3c0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Mrt0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0Rst {
    #[inline(always)]
    fn from(val: u8) -> Mrt0Rst {
        Mrt0Rst::from_bits(val)
    }
}
impl From<Mrt0Rst> for u8 {
    #[inline(always)]
    fn from(val: Mrt0Rst) -> u8 {
        Mrt0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Mrt0RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0RstClr {
    #[inline(always)]
    fn from(val: u8) -> Mrt0RstClr {
        Mrt0RstClr::from_bits(val)
    }
}
impl From<Mrt0RstClr> for u8 {
    #[inline(always)]
    fn from(val: Mrt0RstClr) -> u8 {
        Mrt0RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrt0RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Mrt0RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrt0RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrt0RstSet {
    #[inline(always)]
    fn from(val: u8) -> Mrt0RstSet {
        Mrt0RstSet::from_bits(val)
    }
}
impl From<Mrt0RstSet> for u8 {
    #[inline(always)]
    fn from(val: Mrt0RstSet) -> u8 {
        Mrt0RstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl MuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuRst {
    #[inline(always)]
    fn from(val: u8) -> MuRst {
        MuRst::from_bits(val)
    }
}
impl From<MuRst> for u8 {
    #[inline(always)]
    fn from(val: MuRst) -> u8 {
        MuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl MuRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuRstClr {
    #[inline(always)]
    fn from(val: u8) -> MuRstClr {
        MuRstClr::from_bits(val)
    }
}
impl From<MuRstClr> for u8 {
    #[inline(always)]
    fn from(val: MuRstClr) -> u8 {
        MuRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl MuRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuRstSet {
    #[inline(always)]
    fn from(val: u8) -> MuRstSet {
        MuRstSet::from_bits(val)
    }
}
impl From<MuRstSet> for u8 {
    #[inline(always)]
    fn from(val: MuRstSet) -> u8 {
        MuRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsevtTimerRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl OsevtTimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsevtTimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsevtTimerRst {
    #[inline(always)]
    fn from(val: u8) -> OsevtTimerRst {
        OsevtTimerRst::from_bits(val)
    }
}
impl From<OsevtTimerRst> for u8 {
    #[inline(always)]
    fn from(val: OsevtTimerRst) -> u8 {
        OsevtTimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsevtTimerRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL0 Bit"]
    CLR_RESET = 0x01,
}
impl OsevtTimerRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsevtTimerRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsevtTimerRstClr {
    #[inline(always)]
    fn from(val: u8) -> OsevtTimerRstClr {
        OsevtTimerRstClr::from_bits(val)
    }
}
impl From<OsevtTimerRstClr> for u8 {
    #[inline(always)]
    fn from(val: OsevtTimerRstClr) -> u8 {
        OsevtTimerRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsevtTimerRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL0 Bit"]
    SET_RESET = 0x01,
}
impl OsevtTimerRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsevtTimerRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsevtTimerRstSet {
    #[inline(always)]
    fn from(val: u8) -> OsevtTimerRstSet {
        OsevtTimerRstSet::from_bits(val)
    }
}
impl From<OsevtTimerRstSet> for u8 {
    #[inline(always)]
    fn from(val: OsevtTimerRstSet) -> u8 {
        OsevtTimerRstSet::to_bits(val)
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
pub enum PimctlRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl PimctlRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlRst {
    #[inline(always)]
    fn from(val: u8) -> PimctlRst {
        PimctlRst::from_bits(val)
    }
}
impl From<PimctlRst> for u8 {
    #[inline(always)]
    fn from(val: PimctlRst) -> u8 {
        PimctlRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PimctlRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl PimctlRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlRstClr {
    #[inline(always)]
    fn from(val: u8) -> PimctlRstClr {
        PimctlRstClr::from_bits(val)
    }
}
impl From<PimctlRstClr> for u8 {
    #[inline(always)]
    fn from(val: PimctlRstClr) -> u8 {
        PimctlRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PimctlRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl PimctlRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PimctlRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PimctlRstSet {
    #[inline(always)]
    fn from(val: u8) -> PimctlRstSet {
        PimctlRstSet::from_bits(val)
    }
}
impl From<PimctlRstSet> for u8 {
    #[inline(always)]
    fn from(val: PimctlRstSet) -> u8 {
        PimctlRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaRst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl SemaRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaRst {
    #[inline(always)]
    fn from(val: u8) -> SemaRst {
        SemaRst::from_bits(val)
    }
}
impl From<SemaRst> for u8 {
    #[inline(always)]
    fn from(val: SemaRst) -> u8 {
        SemaRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaRstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL1 Bit"]
    CLR_RESET = 0x01,
}
impl SemaRstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaRstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaRstClr {
    #[inline(always)]
    fn from(val: u8) -> SemaRstClr {
        SemaRstClr::from_bits(val)
    }
}
impl From<SemaRstClr> for u8 {
    #[inline(always)]
    fn from(val: SemaRstClr) -> u8 {
        SemaRstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemaRstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL1 Bit"]
    SET_RESET = 0x01,
}
impl SemaRstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemaRstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemaRstSet {
    #[inline(always)]
    fn from(val: u8) -> SemaRstSet {
        SemaRstSet::from_bits(val)
    }
}
impl From<SemaRstSet> for u8 {
    #[inline(always)]
    fn from(val: SemaRstSet) -> u8 {
        SemaRstSet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0Reset {
    #[doc = "No EVENT Detected."]
    NO_EVENT_DETECTED = 0x0,
    #[doc = "WDT0 reset event detected. (Writing a 1 to this bit clears this status)."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1Rst {
    #[doc = "clear reset"]
    CLEAR_RESET = 0x0,
    #[doc = "set reset"]
    SET_RESET = 0x01,
}
impl Wwdt1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1Rst {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1Rst {
        Wwdt1Rst::from_bits(val)
    }
}
impl From<Wwdt1Rst> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1Rst) -> u8 {
        Wwdt1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1RstClr {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "clears the PRSTCTL2 Bit"]
    CLR_RESET = 0x01,
}
impl Wwdt1RstClr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1RstClr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1RstClr {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1RstClr {
        Wwdt1RstClr::from_bits(val)
    }
}
impl From<Wwdt1RstClr> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1RstClr) -> u8 {
        Wwdt1RstClr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wwdt1RstSet {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PRSTCTL2 Bit"]
    SET_RESET = 0x01,
}
impl Wwdt1RstSet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wwdt1RstSet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wwdt1RstSet {
    #[inline(always)]
    fn from(val: u8) -> Wwdt1RstSet {
        Wwdt1RstSet::from_bits(val)
    }
}
impl From<Wwdt1RstSet> for u8 {
    #[inline(always)]
    fn from(val: Wwdt1RstSet) -> u8 {
        Wwdt1RstSet::to_bits(val)
    }
}
