#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ack {
    #[doc = "Not acknowledged"]
    ACK_NO = 0x0,
    #[doc = "Acknowledged"]
    ACK_YES = 0x01,
}
impl Ack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ack {
    #[inline(always)]
    fn from(val: u8) -> Ack {
        Ack::from_bits(val)
    }
}
impl From<Ack> for u8 {
    #[inline(always)]
    fn from(val: Ack) -> u8 {
        Ack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgBgmode {
    #[doc = "Bandgap disabled"]
    BGMODE0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled"]
    BGMODE01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled"]
    BGMODE10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ActiveCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgBgmode {
        ActiveCfgBgmode::from_bits(val)
    }
}
impl From<ActiveCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgBgmode) -> u8 {
        ActiveCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl ActiveCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddDs {
        ActiveCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddDs) -> u8 {
        ActiveCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Regulate to mid voltage (1.0 V)"]
    MID = 0x01,
    #[doc = "Regulate to normal voltage (1.1 V)"]
    NORMAL = 0x02,
    #[doc = "Regulate to overdrive voltage (1.2 V)"]
    OVER = 0x03,
}
impl ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddLvl {
        ActiveCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddLvl) -> u8 {
        ActiveCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgDcdcVddDs {
    _RESERVED_0 = 0x0,
    #[doc = "Low"]
    LOW = 0x01,
    #[doc = "Normal"]
    NORMAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl ActiveCfgDcdcVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgDcdcVddDs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgDcdcVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgDcdcVddDs {
        ActiveCfgDcdcVddDs::from_bits(val)
    }
}
impl From<ActiveCfgDcdcVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgDcdcVddDs) -> u8 {
        ActiveCfgDcdcVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgDcdcVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Midvoltage (1.0 V)"]
    DCDC01 = 0x01,
    #[doc = "Normal voltage (1.1 V)"]
    DCDC10 = 0x02,
    #[doc = "Overdrive voltage (1.2 V)"]
    DCDC11 = 0x03,
}
impl ActiveCfgDcdcVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgDcdcVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgDcdcVddLvl {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgDcdcVddLvl {
        ActiveCfgDcdcVddLvl::from_bits(val)
    }
}
impl From<ActiveCfgDcdcVddLvl> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgDcdcVddLvl) -> u8 {
        ActiveCfgDcdcVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgSysldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl ActiveCfgSysldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgSysldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgSysldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgSysldoVddDs {
        ActiveCfgSysldoVddDs::from_bits(val)
    }
}
impl From<ActiveCfgSysldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgSysldoVddDs) -> u8 {
        ActiveCfgSysldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BleedEn {
    #[doc = "Do not add"]
    ADD_NO = 0x0,
    #[doc = "Add"]
    ADD_YES = 0x01,
}
impl BleedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BleedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BleedEn {
    #[inline(always)]
    fn from(val: u8) -> BleedEn {
        BleedEn::from_bits(val)
    }
}
impl From<BleedEn> for u8 {
    #[inline(always)]
    fn from(val: BleedEn) -> u8 {
        BleedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstAck {
    #[doc = "Did not complete"]
    COMPL_NO = 0x0,
    #[doc = "Completed"]
    COMPL_YES = 0x01,
}
impl BurstAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstAck {
    #[inline(always)]
    fn from(val: u8) -> BurstAck {
        BurstAck::from_bits(val)
    }
}
impl From<BurstAck> for u8 {
    #[inline(always)]
    fn from(val: BurstAck) -> u8 {
        BurstAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstReq {
    #[doc = "Do not generate"]
    GEN_NO = 0x0,
    #[doc = "Generate"]
    GEN_YES = 0x01,
}
impl BurstReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstReq {
    #[inline(always)]
    fn from(val: u8) -> BurstReq {
        BurstReq::from_bits(val)
    }
}
impl From<BurstReq> for u8 {
    #[inline(always)]
    fn from(val: BurstReq) -> u8 {
        BurstReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Not busy"]
    BUSY_NO = 0x0,
    #[doc = "Busy"]
    BUSY_YES = 0x01,
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
pub enum CntSelect {
    #[doc = "0"]
    BIT0 = 0x0,
    #[doc = "1"]
    BIT1 = 0x01,
    #[doc = "2"]
    BIT2 = 0x02,
    #[doc = "3"]
    BIT3 = 0x03,
}
impl CntSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CntSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CntSelect {
    #[inline(always)]
    fn from(val: u8) -> CntSelect {
        CntSelect::from_bits(val)
    }
}
impl From<CntSelect> for u8 {
    #[inline(always)]
    fn from(val: CntSelect) -> u8 {
        CntSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddHvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddHvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddHvdf {
        CorevddHvdf::from_bits(val)
    }
}
impl From<CorevddHvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddHvdf) -> u8 {
        CorevddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddLvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl CorevddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddLvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddLvdf {
        CorevddLvdf::from_bits(val)
    }
}
impl From<CorevddLvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddLvdf) -> u8 {
        CorevddLvdf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features"]
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
pub enum IovddHvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl IovddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IovddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IovddHvdf {
    #[inline(always)]
    fn from(val: u8) -> IovddHvdf {
        IovddHvdf::from_bits(val)
    }
}
impl From<IovddHvdf> for u8 {
    #[inline(always)]
    fn from(val: IovddHvdf) -> u8 {
        IovddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IovddLvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl IovddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IovddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IovddLvdf {
    #[inline(always)]
    fn from(val: u8) -> IovddLvdf {
        IovddLvdf::from_bits(val)
    }
}
impl From<IovddLvdf> for u8 {
    #[inline(always)]
    fn from(val: IovddLvdf) -> u8 {
        IovddLvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgBgmode {
    #[doc = "Bandgap disabled"]
    BGMODE0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled"]
    BGMODE01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled"]
    BGMODE10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> LpCfgBgmode {
        LpCfgBgmode::from_bits(val)
    }
}
impl From<LpCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: LpCfgBgmode) -> u8 {
        LpCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl LpCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddDs {
        LpCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddDs) -> u8 {
        LpCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddLvl {
    #[doc = "Retention voltage"]
    UNDER = 0x0,
    #[doc = "Mid voltage (1.0 V)"]
    MID = 0x01,
    #[doc = "Normal voltage (1.1 V)"]
    NORMAL = 0x02,
    #[doc = "Overdrive voltage (1.2 V)"]
    OVER = 0x03,
}
impl LpCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddLvl {
        LpCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddLvl) -> u8 {
        LpCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgDcdcVddDs {
    #[doc = "Pulse refresh"]
    PULSE = 0x0,
    #[doc = "Low"]
    LOW = 0x01,
    #[doc = "Normal"]
    NORMAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpCfgDcdcVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgDcdcVddDs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgDcdcVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgDcdcVddDs {
        LpCfgDcdcVddDs::from_bits(val)
    }
}
impl From<LpCfgDcdcVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgDcdcVddDs) -> u8 {
        LpCfgDcdcVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgDcdcVddLvl {
    #[doc = "Retention voltage (0.7 V)"]
    VDD00 = 0x0,
    #[doc = "Mid voltage (1.0 V)"]
    VDD01 = 0x01,
    #[doc = "Normal voltage (1.1 V)"]
    VDD10 = 0x02,
    #[doc = "Overdrive voltage (1.2 V)"]
    VDD11 = 0x03,
}
impl LpCfgDcdcVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgDcdcVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgDcdcVddLvl {
    #[inline(always)]
    fn from(val: u8) -> LpCfgDcdcVddLvl {
        LpCfgDcdcVddLvl::from_bits(val)
    }
}
impl From<LpCfgDcdcVddLvl> for u8 {
    #[inline(always)]
    fn from(val: LpCfgDcdcVddLvl) -> u8 {
        LpCfgDcdcVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgGlitchDetectDisable {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl LpCfgGlitchDetectDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgGlitchDetectDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgGlitchDetectDisable {
    #[inline(always)]
    fn from(val: u8) -> LpCfgGlitchDetectDisable {
        LpCfgGlitchDetectDisable::from_bits(val)
    }
}
impl From<LpCfgGlitchDetectDisable> for u8 {
    #[inline(always)]
    fn from(val: LpCfgGlitchDetectDisable) -> u8 {
        LpCfgGlitchDetectDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgSysldoVddDs {
    #[doc = "Low"]
    LOW = 0x0,
    #[doc = "Normal"]
    NORMAL = 0x01,
}
impl LpCfgSysldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgSysldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgSysldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgSysldoVddDs {
        LpCfgSysldoVddDs::from_bits(val)
    }
}
impl From<LpCfgSysldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgSysldoVddDs) -> u8 {
        LpCfgSysldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpMode {
    #[doc = "SLEEP with system clock running"]
    MODE0 = 0x0,
    #[doc = "DSLEEP with system clock off"]
    MODE1 = 0x01,
    #[doc = "PDOWN with system clock off"]
    MODE2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off"]
    MODE8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpMode {
    #[inline(always)]
    fn from(val: u8) -> LpMode {
        LpMode::from_bits(val)
    }
}
impl From<LpMode> for u8 {
    #[inline(always)]
    fn from(val: LpMode) -> u8 {
        LpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqov {
    #[doc = "Not forced"]
    FORCE_NO = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Forced low (ignore LPREQPOL settings)"]
    FORCE_LOW = 0x02,
    #[doc = "Forced high (ignore LPREQPOL settings)"]
    FORCE_HIGH = 0x03,
}
impl Lpreqov {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqov {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqov {
    #[inline(always)]
    fn from(val: u8) -> Lpreqov {
        Lpreqov::from_bits(val)
    }
}
impl From<Lpreqov> for u8 {
    #[inline(always)]
    fn from(val: Lpreqov) -> u8 {
        Lpreqov::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqpol {
    #[doc = "High"]
    HIGH = 0x0,
    #[doc = "Low"]
    LOW = 0x01,
}
impl Lpreqpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqpol {
    #[inline(always)]
    fn from(val: u8) -> Lpreqpol {
        Lpreqpol::from_bits(val)
    }
}
impl From<Lpreqpol> for u8 {
    #[inline(always)]
    fn from(val: Lpreqpol) -> u8 {
        Lpreqpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvsel {
    #[doc = "High range"]
    NORMAL = 0x0,
    #[doc = "Low range"]
    SAFE = 0x01,
}
impl Lvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvsel {
    #[inline(always)]
    fn from(val: u8) -> Lvsel {
        Lvsel::from_bits(val)
    }
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(val: Lvsel) -> u8 {
        Lvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdLpReq {
    #[doc = "Did not request"]
    REQ_NO = 0x0,
    #[doc = "Requested"]
    REQ_YES = 0x01,
}
impl PdLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdLpReq {
    #[inline(always)]
    fn from(val: u8) -> PdLpReq {
        PdLpReq::from_bits(val)
    }
}
impl From<PdLpReq> for u8 {
    #[inline(always)]
    fn from(val: PdLpReq) -> u8 {
        PdLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrReqStatus {
    #[doc = "Did not request"]
    REQ_NO = 0x0,
    #[doc = "Requested"]
    REQ_YES = 0x01,
}
impl PwrReqStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrReqStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrReqStatus {
    #[inline(always)]
    fn from(val: u8) -> PwrReqStatus {
        PwrReqStatus::from_bits(val)
    }
}
impl From<PwrReqStatus> for u8 {
    #[inline(always)]
    fn from(val: PwrReqStatus) -> u8 {
        PwrReqStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req {
    #[doc = "Do not request"]
    REQ_NO = 0x0,
    #[doc = "Request"]
    REQ_YES = 0x01,
}
impl Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req {
    #[inline(always)]
    fn from(val: u8) -> Req {
        Req::from_bits(val)
    }
}
impl From<Req> for u8 {
    #[inline(always)]
    fn from(val: Req) -> u8 {
        Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpMode {
    #[doc = "Sleep mode with system clock running"]
    MODE0 = 0x0,
    #[doc = "DSLEEP with system clock off"]
    MODE1 = 0x01,
    #[doc = "PDOWN with system clock off"]
    MODE2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off"]
    MODE8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SpcLpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpMode {
    #[inline(always)]
    fn from(val: u8) -> SpcLpMode {
        SpcLpMode::from_bits(val)
    }
}
impl From<SpcLpMode> for u8 {
    #[inline(always)]
    fn from(val: SpcLpMode) -> u8 {
        SpcLpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpReq {
    #[doc = "SPC is in Active or Sleep mode; the ACTIVE_CFG register has control"]
    ACTIVE = 0x0,
    #[doc = "All power domains requested low-power mode; SPC entered a low-power state; power-mode configuration based on the LP_CFG register"]
    LOW_POWER = 0x01,
}
impl SpcLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpReq {
    #[inline(always)]
    fn from(val: u8) -> SpcLpReq {
        SpcLpReq::from_bits(val)
    }
}
impl From<SpcLpReq> for u8 {
    #[inline(always)]
    fn from(val: SpcLpReq) -> u8 {
        SpcLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysldoVddLvl {
    #[doc = "Normal voltage (1.8 V)"]
    NORMAL = 0x0,
    #[doc = "Overdrive voltage (2.5 V)"]
    OVER = 0x01,
}
impl SysldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysldoVddLvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> SysldoVddLvl {
        SysldoVddLvl::from_bits(val)
    }
}
impl From<SysldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: SysldoVddLvl) -> u8 {
        SysldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddHvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddHvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddHvdf {
        SysvddHvdf::from_bits(val)
    }
}
impl From<SysvddHvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddHvdf) -> u8 {
        SysvddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddLvdf {
    #[doc = "Event not detected"]
    EVENT_NO = 0x0,
    #[doc = "Event detected"]
    EVENT_YES = 0x01,
}
impl SysvddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddLvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddLvdf {
        SysvddLvdf::from_bits(val)
    }
}
impl From<SysvddLvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddLvdf) -> u8 {
        SysvddLvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdCoreCfgLock {
    #[doc = "Allow"]
    ALLOW = 0x0,
    #[doc = "Deny"]
    DENY = 0x01,
}
impl VdCoreCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgLock {
        VdCoreCfgLock::from_bits(val)
    }
}
impl From<VdCoreCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgLock) -> u8 {
        VdCoreCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdIoCfgLock {
    #[doc = "Allow"]
    ALLOW = 0x0,
    #[doc = "Deny"]
    DENY = 0x01,
}
impl VdIoCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgLock {
        VdIoCfgLock::from_bits(val)
    }
}
impl From<VdIoCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgLock) -> u8 {
        VdIoCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgLock {
    #[doc = "Allow"]
    ALLOW = 0x0,
    #[doc = "Deny"]
    DENY = 0x01,
}
impl VdSysCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgLock {
        VdSysCfgLock::from_bits(val)
    }
}
impl From<VdSysCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgLock) -> u8 {
        VdSysCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VddVdDisable {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl VddVdDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VddVdDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VddVdDisable {
    #[inline(always)]
    fn from(val: u8) -> VddVdDisable {
        VddVdDisable::from_bits(val)
    }
}
impl From<VddVdDisable> for u8 {
    #[inline(always)]
    fn from(val: VddVdDisable) -> u8 {
        VddVdDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vsm {
    _RESERVED_0 = 0x0,
    #[doc = "1.0 V"]
    VSM1 = 0x01,
    #[doc = "1.1 V"]
    VSM2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Vsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vsm {
    #[inline(always)]
    fn from(val: u8) -> Vsm {
        Vsm::from_bits(val)
    }
}
impl From<Vsm> for u8 {
    #[inline(always)]
    fn from(val: Vsm) -> u8 {
        Vsm::to_bits(val)
    }
}
