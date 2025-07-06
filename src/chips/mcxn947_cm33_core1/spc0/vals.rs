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
pub enum ActiveCfgCoreHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgCoreHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreHvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreHvde {
        ActiveCfgCoreHvde::from_bits(val)
    }
}
impl From<ActiveCfgCoreHvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreHvde) -> u8 {
        ActiveCfgCoreHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgCoreLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreLvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreLvde {
        ActiveCfgCoreLvde::from_bits(val)
    }
}
impl From<ActiveCfgCoreLvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreLvde) -> u8 {
        ActiveCfgCoreLvde::to_bits(val)
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
pub enum ActiveCfgGlitchDetectDisable {
    #[doc = "Low Voltage Glitch Detect enabled"]
    DISABLED = 0x0,
    #[doc = "Low Voltage Glitch Detect disabled"]
    ENABLED = 0x01,
}
impl ActiveCfgGlitchDetectDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgGlitchDetectDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgGlitchDetectDisable {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgGlitchDetectDisable {
        ActiveCfgGlitchDetectDisable::from_bits(val)
    }
}
impl From<ActiveCfgGlitchDetectDisable> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgGlitchDetectDisable) -> u8 {
        ActiveCfgGlitchDetectDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgIoHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgIoHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgIoHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgIoHvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgIoHvde {
        ActiveCfgIoHvde::from_bits(val)
    }
}
impl From<ActiveCfgIoHvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgIoHvde) -> u8 {
        ActiveCfgIoHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgIoLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgIoLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgIoLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgIoLvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgIoLvde {
        ActiveCfgIoLvde::from_bits(val)
    }
}
impl From<ActiveCfgIoLvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgIoLvde) -> u8 {
        ActiveCfgIoLvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgLpbuffEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgLpbuffEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgLpbuffEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgLpbuffEn {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgLpbuffEn {
        ActiveCfgLpbuffEn::from_bits(val)
    }
}
impl From<ActiveCfgLpbuffEn> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgLpbuffEn) -> u8 {
        ActiveCfgLpbuffEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgSysHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgSysHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgSysHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgSysHvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgSysHvde {
        ActiveCfgSysHvde::from_bits(val)
    }
}
impl From<ActiveCfgSysHvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgSysHvde) -> u8 {
        ActiveCfgSysHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgSysLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ActiveCfgSysLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgSysLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgSysLvde {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgSysLvde {
        ActiveCfgSysLvde::from_bits(val)
    }
}
impl From<ActiveCfgSysLvde> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgSysLvde) -> u8 {
        ActiveCfgSysLvde::to_bits(val)
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
pub enum CoreldoEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CoreldoEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoreldoEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoreldoEn {
    #[inline(always)]
    fn from(val: u8) -> CoreldoEn {
        CoreldoEn::from_bits(val)
    }
}
impl From<CoreldoEn> for u8 {
    #[inline(always)]
    fn from(val: CoreldoEn) -> u8 {
        CoreldoEn::to_bits(val)
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
pub enum CorevddIvsEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl CorevddIvsEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddIvsEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddIvsEn {
    #[inline(always)]
    fn from(val: u8) -> CorevddIvsEn {
        CorevddIvsEn::from_bits(val)
    }
}
impl From<CorevddIvsEn> for u8 {
    #[inline(always)]
    fn from(val: CorevddIvsEn) -> u8 {
        CorevddIvsEn::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcdcEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl DcdcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcdcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcdcEn {
    #[inline(always)]
    fn from(val: u8) -> DcdcEn {
        DcdcEn::from_bits(val)
    }
}
impl From<DcdcEn> for u8 {
    #[inline(always)]
    fn from(val: DcdcEn) -> u8 {
        DcdcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpdownPulldownDisable {
    #[doc = "LDO_CORE pulldown in Deep Power Down not disabled"]
    DISABLED = 0x0,
    #[doc = "LDO_CORE pulldown in Deep Power Down disabled"]
    ENABLED = 0x01,
}
impl DpdownPulldownDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpdownPulldownDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpdownPulldownDisable {
    #[inline(always)]
    fn from(val: u8) -> DpdownPulldownDisable {
        DpdownPulldownDisable::from_bits(val)
    }
}
impl From<DpdownPulldownDisable> for u8 {
    #[inline(always)]
    fn from(val: DpdownPulldownDisable) -> u8 {
        DpdownPulldownDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtBurstEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl ExtBurstEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtBurstEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtBurstEn {
    #[inline(always)]
    fn from(val: u8) -> ExtBurstEn {
        ExtBurstEn::from_bits(val)
    }
}
impl From<ExtBurstEn> for u8 {
    #[inline(always)]
    fn from(val: ExtBurstEn) -> u8 {
        ExtBurstEn::to_bits(val)
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
pub enum FreqCntrlOn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl FreqCntrlOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqCntrlOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqCntrlOn {
    #[inline(always)]
    fn from(val: u8) -> FreqCntrlOn {
        FreqCntrlOn::from_bits(val)
    }
}
impl From<FreqCntrlOn> for u8 {
    #[inline(always)]
    fn from(val: FreqCntrlOn) -> u8 {
        FreqCntrlOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GlitchDetectScLock {
    #[doc = "Writes to RE are allowed."]
    DISABLED = 0x0,
    #[doc = "Writes to RE are ignored."]
    ENABLED = 0x01,
}
impl GlitchDetectScLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GlitchDetectScLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GlitchDetectScLock {
    #[inline(always)]
    fn from(val: u8) -> GlitchDetectScLock {
        GlitchDetectScLock::from_bits(val)
    }
}
impl From<GlitchDetectScLock> for u8 {
    #[inline(always)]
    fn from(val: GlitchDetectScLock) -> u8 {
        GlitchDetectScLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ie {
    #[doc = "GLITCH_DETECT_FLAG\\[CNT_SELECT\\] does not generate hardware interrupt (user polling)"]
    DISABLED = 0x0,
    #[doc = "GLITCH_DETECT_FLAG\\[CNT_SELECT\\] does generate hardware interrupt"]
    ENABLED = 0x01,
}
impl Ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ie {
    #[inline(always)]
    fn from(val: u8) -> Ie {
        Ie::from_bits(val)
    }
}
impl From<Ie> for u8 {
    #[inline(always)]
    fn from(val: Ie) -> u8 {
        Ie::to_bits(val)
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
pub enum Isinken {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl Isinken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isinken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isinken {
    #[inline(always)]
    fn from(val: u8) -> Isinken {
        Isinken::from_bits(val)
    }
}
impl From<Isinken> for u8 {
    #[inline(always)]
    fn from(val: Isinken) -> u8 {
        Isinken::to_bits(val)
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
pub enum LpCfgCoreHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgCoreHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreHvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreHvde {
        LpCfgCoreHvde::from_bits(val)
    }
}
impl From<LpCfgCoreHvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreHvde) -> u8 {
        LpCfgCoreHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgCoreLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreLvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreLvde {
        LpCfgCoreLvde::from_bits(val)
    }
}
impl From<LpCfgCoreLvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreLvde) -> u8 {
        LpCfgCoreLvde::to_bits(val)
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
pub enum LpCfgIoHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgIoHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgIoHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgIoHvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgIoHvde {
        LpCfgIoHvde::from_bits(val)
    }
}
impl From<LpCfgIoHvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgIoHvde) -> u8 {
        LpCfgIoHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgIoLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgIoLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgIoLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgIoLvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgIoLvde {
        LpCfgIoLvde::from_bits(val)
    }
}
impl From<LpCfgIoLvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgIoLvde) -> u8 {
        LpCfgIoLvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgLpbuffEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgLpbuffEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgLpbuffEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgLpbuffEn {
    #[inline(always)]
    fn from(val: u8) -> LpCfgLpbuffEn {
        LpCfgLpbuffEn::from_bits(val)
    }
}
impl From<LpCfgLpbuffEn> for u8 {
    #[inline(always)]
    fn from(val: LpCfgLpbuffEn) -> u8 {
        LpCfgLpbuffEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgSysHvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgSysHvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgSysHvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgSysHvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgSysHvde {
        LpCfgSysHvde::from_bits(val)
    }
}
impl From<LpCfgSysHvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgSysHvde) -> u8 {
        LpCfgSysHvde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgSysLvde {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpCfgSysLvde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgSysLvde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgSysLvde {
    #[inline(always)]
    fn from(val: u8) -> LpCfgSysLvde {
        LpCfgSysLvde::from_bits(val)
    }
}
impl From<LpCfgSysLvde> for u8 {
    #[inline(always)]
    fn from(val: LpCfgSysLvde) -> u8 {
        LpCfgSysLvde::to_bits(val)
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
pub enum LpIrefen {
    #[doc = "Disable for power saving in Deep Power Down mode"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl LpIrefen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpIrefen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpIrefen {
    #[inline(always)]
    fn from(val: u8) -> LpIrefen {
        LpIrefen::from_bits(val)
    }
}
impl From<LpIrefen> for u8 {
    #[inline(always)]
    fn from(val: LpIrefen) -> u8 {
        LpIrefen::to_bits(val)
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
pub enum Lpreqoe {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Lpreqoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqoe {
    #[inline(always)]
    fn from(val: u8) -> Lpreqoe {
        Lpreqoe::from_bits(val)
    }
}
impl From<Lpreqoe> for u8 {
    #[inline(always)]
    fn from(val: Lpreqoe) -> u8 {
        Lpreqoe::to_bits(val)
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
pub enum Re {
    #[doc = "GLITCH_DETECT_FLAG\\[CNT_SELECT\\] does not generate POR/LVD reset"]
    DISABLED = 0x0,
    #[doc = "GLITCH_DETECT_FLAG\\[CNT_SELECT\\] does generate POR/LVD reset"]
    ENABLED = 0x01,
}
impl Re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Re {
    #[inline(always)]
    fn from(val: u8) -> Re {
        Re::from_bits(val)
    }
}
impl From<Re> for u8 {
    #[inline(always)]
    fn from(val: Re) -> u8 {
        Re::to_bits(val)
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
pub enum SysldoEn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SysldoEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysldoEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysldoEn {
    #[inline(always)]
    fn from(val: u8) -> SysldoEn {
        SysldoEn::from_bits(val)
    }
}
impl From<SysldoEn> for u8 {
    #[inline(always)]
    fn from(val: SysldoEn) -> u8 {
        SysldoEn::to_bits(val)
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
pub enum VdCoreCfgHvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdCoreCfgHvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgHvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgHvdie {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgHvdie {
        VdCoreCfgHvdie::from_bits(val)
    }
}
impl From<VdCoreCfgHvdie> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgHvdie) -> u8 {
        VdCoreCfgHvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdCoreCfgHvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdCoreCfgHvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgHvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgHvdre {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgHvdre {
        VdCoreCfgHvdre::from_bits(val)
    }
}
impl From<VdCoreCfgHvdre> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgHvdre) -> u8 {
        VdCoreCfgHvdre::to_bits(val)
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
pub enum VdCoreCfgLvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdCoreCfgLvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgLvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgLvdie {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgLvdie {
        VdCoreCfgLvdie::from_bits(val)
    }
}
impl From<VdCoreCfgLvdie> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgLvdie) -> u8 {
        VdCoreCfgLvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdCoreCfgLvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdCoreCfgLvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgLvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgLvdre {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgLvdre {
        VdCoreCfgLvdre::from_bits(val)
    }
}
impl From<VdCoreCfgLvdre> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgLvdre) -> u8 {
        VdCoreCfgLvdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdIoCfgHvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdIoCfgHvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgHvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgHvdie {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgHvdie {
        VdIoCfgHvdie::from_bits(val)
    }
}
impl From<VdIoCfgHvdie> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgHvdie) -> u8 {
        VdIoCfgHvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdIoCfgHvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdIoCfgHvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgHvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgHvdre {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgHvdre {
        VdIoCfgHvdre::from_bits(val)
    }
}
impl From<VdIoCfgHvdre> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgHvdre) -> u8 {
        VdIoCfgHvdre::to_bits(val)
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
pub enum VdIoCfgLvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdIoCfgLvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgLvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgLvdie {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgLvdie {
        VdIoCfgLvdie::from_bits(val)
    }
}
impl From<VdIoCfgLvdie> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgLvdie) -> u8 {
        VdIoCfgLvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdIoCfgLvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdIoCfgLvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgLvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgLvdre {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgLvdre {
        VdIoCfgLvdre::from_bits(val)
    }
}
impl From<VdIoCfgLvdre> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgLvdre) -> u8 {
        VdIoCfgLvdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgHvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdSysCfgHvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgHvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgHvdie {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgHvdie {
        VdSysCfgHvdie::from_bits(val)
    }
}
impl From<VdSysCfgHvdie> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgHvdie) -> u8 {
        VdSysCfgHvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgHvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdSysCfgHvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgHvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgHvdre {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgHvdre {
        VdSysCfgHvdre::from_bits(val)
    }
}
impl From<VdSysCfgHvdre> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgHvdre) -> u8 {
        VdSysCfgHvdre::to_bits(val)
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
pub enum VdSysCfgLvdie {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdSysCfgLvdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgLvdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgLvdie {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgLvdie {
        VdSysCfgLvdie::from_bits(val)
    }
}
impl From<VdSysCfgLvdie> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgLvdie) -> u8 {
        VdSysCfgLvdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgLvdre {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl VdSysCfgLvdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgLvdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgLvdre {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgLvdre {
        VdSysCfgLvdre::from_bits(val)
    }
}
impl From<VdSysCfgLvdre> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgLvdre) -> u8 {
        VdSysCfgLvdre::to_bits(val)
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
