#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp1CmpIgenTrimDn {
    #[doc = "no reduce."]
    Acmp1CmpIgenTrimDn0 = 0x0,
    #[doc = "reduces."]
    Acmp1CmpIgenTrimDn1 = 0x01,
}
impl Acmp1CmpIgenTrimDn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp1CmpIgenTrimDn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp1CmpIgenTrimDn {
    #[inline(always)]
    fn from(val: u8) -> Acmp1CmpIgenTrimDn {
        Acmp1CmpIgenTrimDn::from_bits(val)
    }
}
impl From<Acmp1CmpIgenTrimDn> for u8 {
    #[inline(always)]
    fn from(val: Acmp1CmpIgenTrimDn) -> u8 {
        Acmp1CmpIgenTrimDn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp1CmpIgenTrimUp {
    #[doc = "no increase."]
    Acmp1CmpIgenTrimUp0 = 0x0,
    #[doc = "increases."]
    Acmp1CmpIgenTrimUp1 = 0x01,
}
impl Acmp1CmpIgenTrimUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp1CmpIgenTrimUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp1CmpIgenTrimUp {
    #[inline(always)]
    fn from(val: u8) -> Acmp1CmpIgenTrimUp {
        Acmp1CmpIgenTrimUp::from_bits(val)
    }
}
impl From<Acmp1CmpIgenTrimUp> for u8 {
    #[inline(always)]
    fn from(val: Acmp1CmpIgenTrimUp) -> u8 {
        Acmp1CmpIgenTrimUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp1SampleSyncEn {
    #[doc = "select XBAR output."]
    Acmp1SampleSyncEn0 = 0x0,
    #[doc = "select synced sample_lv."]
    Acmp1SampleSyncEn1 = 0x01,
}
impl Acmp1SampleSyncEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp1SampleSyncEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp1SampleSyncEn {
    #[inline(always)]
    fn from(val: u8) -> Acmp1SampleSyncEn {
        Acmp1SampleSyncEn::from_bits(val)
    }
}
impl From<Acmp1SampleSyncEn> for u8 {
    #[inline(always)]
    fn from(val: Acmp1SampleSyncEn) -> u8 {
        Acmp1SampleSyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp2CmpIgenTrimDn {
    #[doc = "no reduce."]
    Acmp2CmpIgenTrimDn0 = 0x0,
    #[doc = "reduces."]
    Acmp2CmpIgenTrimDn1 = 0x01,
}
impl Acmp2CmpIgenTrimDn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp2CmpIgenTrimDn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp2CmpIgenTrimDn {
    #[inline(always)]
    fn from(val: u8) -> Acmp2CmpIgenTrimDn {
        Acmp2CmpIgenTrimDn::from_bits(val)
    }
}
impl From<Acmp2CmpIgenTrimDn> for u8 {
    #[inline(always)]
    fn from(val: Acmp2CmpIgenTrimDn) -> u8 {
        Acmp2CmpIgenTrimDn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp2CmpIgenTrimUp {
    #[doc = "no increase."]
    Acmp2CmpIgenTrimUp0 = 0x0,
    #[doc = "increases."]
    Acmp2CmpIgenTrimUp1 = 0x01,
}
impl Acmp2CmpIgenTrimUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp2CmpIgenTrimUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp2CmpIgenTrimUp {
    #[inline(always)]
    fn from(val: u8) -> Acmp2CmpIgenTrimUp {
        Acmp2CmpIgenTrimUp::from_bits(val)
    }
}
impl From<Acmp2CmpIgenTrimUp> for u8 {
    #[inline(always)]
    fn from(val: Acmp2CmpIgenTrimUp) -> u8 {
        Acmp2CmpIgenTrimUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp2SampleSyncEn {
    #[doc = "select XBAR output."]
    Acmp2SampleSyncEn0 = 0x0,
    #[doc = "select synced sample_lv."]
    Acmp2SampleSyncEn1 = 0x01,
}
impl Acmp2SampleSyncEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp2SampleSyncEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp2SampleSyncEn {
    #[inline(always)]
    fn from(val: u8) -> Acmp2SampleSyncEn {
        Acmp2SampleSyncEn::from_bits(val)
    }
}
impl From<Acmp2SampleSyncEn> for u8 {
    #[inline(always)]
    fn from(val: Acmp2SampleSyncEn) -> u8 {
        Acmp2SampleSyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp3CmpIgenTrimDn {
    #[doc = "no reduce."]
    Acmp3CmpIgenTrimDn0 = 0x0,
    #[doc = "reduces."]
    Acmp3CmpIgenTrimDn1 = 0x01,
}
impl Acmp3CmpIgenTrimDn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp3CmpIgenTrimDn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp3CmpIgenTrimDn {
    #[inline(always)]
    fn from(val: u8) -> Acmp3CmpIgenTrimDn {
        Acmp3CmpIgenTrimDn::from_bits(val)
    }
}
impl From<Acmp3CmpIgenTrimDn> for u8 {
    #[inline(always)]
    fn from(val: Acmp3CmpIgenTrimDn) -> u8 {
        Acmp3CmpIgenTrimDn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp3CmpIgenTrimUp {
    #[doc = "no increase."]
    Acmp3CmpIgenTrimUp0 = 0x0,
    #[doc = "increases."]
    Acmp3CmpIgenTrimUp1 = 0x01,
}
impl Acmp3CmpIgenTrimUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp3CmpIgenTrimUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp3CmpIgenTrimUp {
    #[inline(always)]
    fn from(val: u8) -> Acmp3CmpIgenTrimUp {
        Acmp3CmpIgenTrimUp::from_bits(val)
    }
}
impl From<Acmp3CmpIgenTrimUp> for u8 {
    #[inline(always)]
    fn from(val: Acmp3CmpIgenTrimUp) -> u8 {
        Acmp3CmpIgenTrimUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp3SampleSyncEn {
    #[doc = "select XBAR output."]
    Acmp3SampleSyncEn0 = 0x0,
    #[doc = "select synced sample_lv."]
    Acmp3SampleSyncEn1 = 0x01,
}
impl Acmp3SampleSyncEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp3SampleSyncEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp3SampleSyncEn {
    #[inline(always)]
    fn from(val: u8) -> Acmp3SampleSyncEn {
        Acmp3SampleSyncEn::from_bits(val)
    }
}
impl From<Acmp3SampleSyncEn> for u8 {
    #[inline(always)]
    fn from(val: Acmp3SampleSyncEn) -> u8 {
        Acmp3SampleSyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp4CmpIgenTrimDn {
    #[doc = "no reduce."]
    Acmp4CmpIgenTrimDn0 = 0x0,
    #[doc = "reduces."]
    Acmp4CmpIgenTrimDn1 = 0x01,
}
impl Acmp4CmpIgenTrimDn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp4CmpIgenTrimDn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp4CmpIgenTrimDn {
    #[inline(always)]
    fn from(val: u8) -> Acmp4CmpIgenTrimDn {
        Acmp4CmpIgenTrimDn::from_bits(val)
    }
}
impl From<Acmp4CmpIgenTrimDn> for u8 {
    #[inline(always)]
    fn from(val: Acmp4CmpIgenTrimDn) -> u8 {
        Acmp4CmpIgenTrimDn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp4CmpIgenTrimUp {
    #[doc = "no increase."]
    Acmp4CmpIgenTrimUp0 = 0x0,
    #[doc = "increases."]
    Acmp4CmpIgenTrimUp1 = 0x01,
}
impl Acmp4CmpIgenTrimUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp4CmpIgenTrimUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp4CmpIgenTrimUp {
    #[inline(always)]
    fn from(val: u8) -> Acmp4CmpIgenTrimUp {
        Acmp4CmpIgenTrimUp::from_bits(val)
    }
}
impl From<Acmp4CmpIgenTrimUp> for u8 {
    #[inline(always)]
    fn from(val: Acmp4CmpIgenTrimUp) -> u8 {
        Acmp4CmpIgenTrimUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmp4SampleSyncEn {
    #[doc = "select XBAR output."]
    Acmp4SampleSyncEn0 = 0x0,
    #[doc = "select synced sample_lv."]
    Acmp4SampleSyncEn1 = 0x01,
}
impl Acmp4SampleSyncEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmp4SampleSyncEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmp4SampleSyncEn {
    #[inline(always)]
    fn from(val: u8) -> Acmp4SampleSyncEn {
        Acmp4SampleSyncEn::from_bits(val)
    }
}
impl From<Acmp4SampleSyncEn> for u8 {
    #[inline(always)]
    fn from(val: Acmp4SampleSyncEn) -> u8 {
        Acmp4SampleSyncEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AcmpIpgStopMode {
    #[doc = "ACMP is functional in Stop mode."]
    AcmpIpgStopMode0 = 0x0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, ACMP is not functional in Stop mode."]
    AcmpIpgStopMode1 = 0x01,
}
impl AcmpIpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AcmpIpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AcmpIpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> AcmpIpgStopMode {
        AcmpIpgStopMode::from_bits(val)
    }
}
impl From<AcmpIpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: AcmpIpgStopMode) -> u8 {
        AcmpIpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArcacheUsdhc {
    #[doc = "Cacheable attribute is off for read transactions."]
    ArcacheUsdhc0 = 0x0,
    #[doc = "Cacheable attribute is on for read transactions."]
    ArcacheUsdhc1 = 0x01,
}
impl ArcacheUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArcacheUsdhc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArcacheUsdhc {
    #[inline(always)]
    fn from(val: u8) -> ArcacheUsdhc {
        ArcacheUsdhc::from_bits(val)
    }
}
impl From<ArcacheUsdhc> for u8 {
    #[inline(always)]
    fn from(val: ArcacheUsdhc) -> u8 {
        ArcacheUsdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AwcacheUsdhc {
    #[doc = "Cacheable attribute is off for write transactions."]
    AwcacheUsdhc0 = 0x0,
    #[doc = "Cacheable attribute is on for write transactions."]
    AwcacheUsdhc1 = 0x01,
}
impl AwcacheUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AwcacheUsdhc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AwcacheUsdhc {
    #[inline(always)]
    fn from(val: u8) -> AwcacheUsdhc {
        AwcacheUsdhc::from_bits(val)
    }
}
impl From<AwcacheUsdhc> for u8 {
    #[inline(always)]
    fn from(val: AwcacheUsdhc) -> u8 {
        AwcacheUsdhc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsLAhbxlHighPriority {
    #[doc = "AXBS_L AHBXL master does not have high priority."]
    AxbsLAhbxlHighPriority0 = 0x0,
    #[doc = "AXBS_P AHBXL master has high priority."]
    AxbsLAhbxlHighPriority1 = 0x01,
}
impl AxbsLAhbxlHighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsLAhbxlHighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsLAhbxlHighPriority {
    #[inline(always)]
    fn from(val: u8) -> AxbsLAhbxlHighPriority {
        AxbsLAhbxlHighPriority::from_bits(val)
    }
}
impl From<AxbsLAhbxlHighPriority> for u8 {
    #[inline(always)]
    fn from(val: AxbsLAhbxlHighPriority) -> u8 {
        AxbsLAhbxlHighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsLDmaHighPriority {
    #[doc = "AXBS_L DMA master does not have high priority."]
    AxbsLDmaHighPriority0 = 0x0,
    #[doc = "AXBS_L DMA master has high priority."]
    AxbsLDmaHighPriority1 = 0x01,
}
impl AxbsLDmaHighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsLDmaHighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsLDmaHighPriority {
    #[inline(always)]
    fn from(val: u8) -> AxbsLDmaHighPriority {
        AxbsLDmaHighPriority::from_bits(val)
    }
}
impl From<AxbsLDmaHighPriority> for u8 {
    #[inline(always)]
    fn from(val: AxbsLDmaHighPriority) -> u8 {
        AxbsLDmaHighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsLForceRoundRobin {
    #[doc = "AXBS_L masters are not arbitored in round robin, depending on DMA and AHBXL master priority settings."]
    AxbsLForceRoundRobin0 = 0x0,
    #[doc = "AXBS_L masters are arbitored in round robin."]
    AxbsLForceRoundRobin1 = 0x01,
}
impl AxbsLForceRoundRobin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsLForceRoundRobin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsLForceRoundRobin {
    #[inline(always)]
    fn from(val: u8) -> AxbsLForceRoundRobin {
        AxbsLForceRoundRobin::from_bits(val)
    }
}
impl From<AxbsLForceRoundRobin> for u8 {
    #[inline(always)]
    fn from(val: AxbsLForceRoundRobin) -> u8 {
        AxbsLForceRoundRobin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsLHaltReq {
    #[doc = "axbs_l normal run."]
    AxbsLHaltReq0 = 0x0,
    #[doc = "request to halt axbs_l."]
    AxbsLHaltReq1 = 0x01,
}
impl AxbsLHaltReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsLHaltReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsLHaltReq {
    #[inline(always)]
    fn from(val: u8) -> AxbsLHaltReq {
        AxbsLHaltReq::from_bits(val)
    }
}
impl From<AxbsLHaltReq> for u8 {
    #[inline(always)]
    fn from(val: AxbsLHaltReq) -> u8 {
        AxbsLHaltReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsLHalted {
    #[doc = "axbs_l is not halted."]
    AxbsLHalted0 = 0x0,
    #[doc = "axbs_l is in halted status."]
    AxbsLHalted1 = 0x01,
}
impl AxbsLHalted {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsLHalted {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsLHalted {
    #[inline(always)]
    fn from(val: u8) -> AxbsLHalted {
        AxbsLHalted::from_bits(val)
    }
}
impl From<AxbsLHalted> for u8 {
    #[inline(always)]
    fn from(val: AxbsLHalted) -> u8 {
        AxbsLHalted::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsPForceRoundRobin {
    #[doc = "AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings."]
    AxbsPForceRoundRobin0 = 0x0,
    #[doc = "AXBS_P masters are arbitored in round robin."]
    AxbsPForceRoundRobin1 = 0x01,
}
impl AxbsPForceRoundRobin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsPForceRoundRobin {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsPForceRoundRobin {
    #[inline(always)]
    fn from(val: u8) -> AxbsPForceRoundRobin {
        AxbsPForceRoundRobin::from_bits(val)
    }
}
impl From<AxbsPForceRoundRobin> for u8 {
    #[inline(always)]
    fn from(val: AxbsPForceRoundRobin) -> u8 {
        AxbsPForceRoundRobin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsPM0HighPriority {
    #[doc = "AXBS_P M0 master doesn't have high priority."]
    AxbsPM0HighPriority0 = 0x0,
    #[doc = "AXBS_P M0 master has high priority."]
    AxbsPM0HighPriority1 = 0x01,
}
impl AxbsPM0HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsPM0HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsPM0HighPriority {
    #[inline(always)]
    fn from(val: u8) -> AxbsPM0HighPriority {
        AxbsPM0HighPriority::from_bits(val)
    }
}
impl From<AxbsPM0HighPriority> for u8 {
    #[inline(always)]
    fn from(val: AxbsPM0HighPriority) -> u8 {
        AxbsPM0HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsPM1HighPriority {
    #[doc = "AXBS_P M1 master does not have high priority."]
    AxbsPM1HighPriority0 = 0x0,
    #[doc = "AXBS_P M1 master has high priority."]
    AxbsPM1HighPriority1 = 0x01,
}
impl AxbsPM1HighPriority {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AxbsPM1HighPriority {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AxbsPM1HighPriority {
    #[inline(always)]
    fn from(val: u8) -> AxbsPM1HighPriority {
        AxbsPM1HighPriority::from_bits(val)
    }
}
impl From<AxbsPM1HighPriority> for u8 {
    #[inline(always)]
    fn from(val: AxbsPM1HighPriority) -> u8 {
        AxbsPM1HighPriority::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CacheEnet {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CacheEnet0 = 0x0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CacheEnet1 = 0x01,
}
impl CacheEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CacheEnet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CacheEnet {
    #[inline(always)]
    fn from(val: u8) -> CacheEnet {
        CacheEnet::from_bits(val)
    }
}
impl From<CacheEnet> for u8 {
    #[inline(always)]
    fn from(val: CacheEnet) -> u8 {
        CacheEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CacheUsb {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CacheUsb0 = 0x0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CacheUsb1 = 0x01,
}
impl CacheUsb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CacheUsb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CacheUsb {
    #[inline(always)]
    fn from(val: u8) -> CacheUsb {
        CacheUsb::from_bits(val)
    }
}
impl From<CacheUsb> for u8 {
    #[inline(always)]
    fn from(val: CacheUsb) -> u8 {
        CacheUsb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1StopAck {
    #[doc = "CAN1 stop acknowledge is not asserted."]
    Can1StopAck0 = 0x0,
    #[doc = "CAN1 stop acknowledge is asserted."]
    Can1StopAck1 = 0x01,
}
impl Can1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Can1StopAck {
        Can1StopAck::from_bits(val)
    }
}
impl From<Can1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Can1StopAck) -> u8 {
        Can1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can1StopReq {
    #[doc = "stop request off."]
    Can1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Can1StopReq1 = 0x01,
}
impl Can1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Can1StopReq {
        Can1StopReq::from_bits(val)
    }
}
impl From<Can1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Can1StopReq) -> u8 {
        Can1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can2StopAck {
    #[doc = "CAN2 stop acknowledge is not asserted."]
    Can2StopAck0 = 0x0,
    #[doc = "CAN2 stop acknowledge is asserted."]
    Can2StopAck1 = 0x01,
}
impl Can2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Can2StopAck {
        Can2StopAck::from_bits(val)
    }
}
impl From<Can2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Can2StopAck) -> u8 {
        Can2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Can2StopReq {
    #[doc = "stop request off."]
    Can2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Can2StopReq1 = 0x01,
}
impl Can2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Can2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Can2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Can2StopReq {
        Can2StopReq::from_bits(val)
    }
}
impl From<Can2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Can2StopReq) -> u8 {
        Can2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanfdFilterBypass {
    #[doc = "enable CANFD filter."]
    CanfdFilterBypass0 = 0x0,
    #[doc = "disable CANFD filter."]
    CanfdFilterBypass1 = 0x01,
}
impl CanfdFilterBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanfdFilterBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanfdFilterBypass {
    #[inline(always)]
    fn from(val: u8) -> CanfdFilterBypass {
        CanfdFilterBypass::from_bits(val)
    }
}
impl From<CanfdFilterBypass> for u8 {
    #[inline(always)]
    fn from(val: CanfdFilterBypass) -> u8 {
        CanfdFilterBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanfdStopAck {
    #[doc = "CANFD stop acknowledge is not asserted."]
    CanfdStopAck0 = 0x0,
    #[doc = "CANFD stop acknowledge is asserted."]
    CanfdStopAck1 = 0x01,
}
impl CanfdStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanfdStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanfdStopAck {
    #[inline(always)]
    fn from(val: u8) -> CanfdStopAck {
        CanfdStopAck::from_bits(val)
    }
}
impl From<CanfdStopAck> for u8 {
    #[inline(always)]
    fn from(val: CanfdStopAck) -> u8 {
        CanfdStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CanfdStopReq {
    #[doc = "stop request off."]
    CanfdStopReq0 = 0x0,
    #[doc = "stop request on."]
    CanfdStopReq1 = 0x01,
}
impl CanfdStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CanfdStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CanfdStopReq {
    #[inline(always)]
    fn from(val: u8) -> CanfdStopReq {
        CanfdStopReq::from_bits(val)
    }
}
impl From<CanfdStopReq> for u8 {
    #[inline(always)]
    fn from(val: CanfdStopReq) -> u8 {
        CanfdStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm7ForceHclkEn {
    #[doc = "AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible."]
    Cm7ForceHclkEn0 = 0x0,
    #[doc = "AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible."]
    Cm7ForceHclkEn1 = 0x01,
}
impl Cm7ForceHclkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm7ForceHclkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm7ForceHclkEn {
    #[inline(always)]
    fn from(val: u8) -> Cm7ForceHclkEn {
        Cm7ForceHclkEn::from_bits(val)
    }
}
impl From<Cm7ForceHclkEn> for u8 {
    #[inline(always)]
    fn from(val: Cm7ForceHclkEn) -> u8 {
        Cm7ForceHclkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgEn {
    #[doc = "Debug turned off."]
    DbgEn0 = 0x0,
    #[doc = "Debug enabled (default)."]
    DbgEn1 = 0x01,
}
impl DbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgEn {
    #[inline(always)]
    fn from(val: u8) -> DbgEn {
        DbgEn::from_bits(val)
    }
}
impl From<DbgEn> for u8 {
    #[inline(always)]
    fn from(val: DbgEn) -> u8 {
        DbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpKeySel {
    #[doc = "Select \\[127:0\\] from SNVS Master Key as dcp key."]
    DcpKeySel0 = 0x0,
    #[doc = "Select \\[255:128\\] from SNVS Master Key as dcp key."]
    DcpKeySel1 = 0x01,
}
impl DcpKeySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpKeySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpKeySel {
    #[inline(always)]
    fn from(val: u8) -> DcpKeySel {
        DcpKeySel::from_bits(val)
    }
}
impl From<DcpKeySel> for u8 {
    #[inline(always)]
    fn from(val: DcpKeySel) -> u8 {
        DcpKeySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcpkeyOcotpOrKeymux {
    #[doc = "Select key from SNVS Master Key."]
    DcpkeyOcotpOrKeymux0 = 0x0,
    #[doc = "Select key from OCOTP (SW_GP2)."]
    DcpkeyOcotpOrKeymux1 = 0x01,
}
impl DcpkeyOcotpOrKeymux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcpkeyOcotpOrKeymux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcpkeyOcotpOrKeymux {
    #[inline(always)]
    fn from(val: u8) -> DcpkeyOcotpOrKeymux {
        DcpkeyOcotpOrKeymux::from_bits(val)
    }
}
impl From<DcpkeyOcotpOrKeymux> for u8 {
    #[inline(always)]
    fn from(val: DcpkeyOcotpOrKeymux) -> u8 {
        DcpkeyOcotpOrKeymux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdmaStopAck {
    #[doc = "EDMA stop acknowledge is not asserted."]
    EdmaStopAck0 = 0x0,
    #[doc = "EDMA stop acknowledge is asserted (EDMA is in STOP mode)."]
    EdmaStopAck1 = 0x01,
}
impl EdmaStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdmaStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdmaStopAck {
    #[inline(always)]
    fn from(val: u8) -> EdmaStopAck {
        EdmaStopAck::from_bits(val)
    }
}
impl From<EdmaStopAck> for u8 {
    #[inline(always)]
    fn from(val: EdmaStopAck) -> u8 {
        EdmaStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EdmaStopReq {
    #[doc = "stop request off."]
    EdmaStopReq0 = 0x0,
    #[doc = "stop request on."]
    EdmaStopReq1 = 0x01,
}
impl EdmaStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EdmaStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EdmaStopReq {
    #[inline(always)]
    fn from(val: u8) -> EdmaStopReq {
        EdmaStopReq::from_bits(val)
    }
}
impl From<EdmaStopReq> for u8 {
    #[inline(always)]
    fn from(val: EdmaStopReq) -> u8 {
        EdmaStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet1ClkSel {
    #[doc = "ENET1 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET_REF_CLK1 function."]
    Enet1ClkSel0 = 0x0,
    #[doc = "Gets ENET1 TX reference clock from the ENET1_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    Enet1ClkSel1 = 0x01,
}
impl Enet1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet1ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Enet1ClkSel {
        Enet1ClkSel::from_bits(val)
    }
}
impl From<Enet1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Enet1ClkSel) -> u8 {
        Enet1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet1TxClkDir {
    #[doc = "ENET1_TX_CLK output driver is disabled."]
    Enet1TxClkDir0 = 0x0,
    #[doc = "ENET1_TX_CLK output driver is enabled."]
    Enet1TxClkDir1 = 0x01,
}
impl Enet1TxClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet1TxClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet1TxClkDir {
    #[inline(always)]
    fn from(val: u8) -> Enet1TxClkDir {
        Enet1TxClkDir::from_bits(val)
    }
}
impl From<Enet1TxClkDir> for u8 {
    #[inline(always)]
    fn from(val: Enet1TxClkDir) -> u8 {
        Enet1TxClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2ClkSel {
    #[doc = "ENET2 TX reference clock driven by ref_enetpll. This clock is also output to pins via the IOMUX. ENET2_REF_CLK function."]
    Enet2ClkSel0 = 0x0,
    #[doc = "Gets ENET2 TX reference clock from the ENET2_TX_CLK pin. In this use case, an external OSC provides the clock for both the external PHY and the internal controller."]
    Enet2ClkSel1 = 0x01,
}
impl Enet2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2ClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Enet2ClkSel {
        Enet2ClkSel::from_bits(val)
    }
}
impl From<Enet2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Enet2ClkSel) -> u8 {
        Enet2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2Event3inSel {
    #[doc = "event3 source input from ENET2_1588_EVENT3_IN."]
    Enet2Event3inSel0 = 0x0,
    #[doc = "event3 source input from GPT2.GPT_COMPARE2."]
    Enet2Event3inSel1 = 0x01,
}
impl Enet2Event3inSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2Event3inSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2Event3inSel {
    #[inline(always)]
    fn from(val: u8) -> Enet2Event3inSel {
        Enet2Event3inSel::from_bits(val)
    }
}
impl From<Enet2Event3inSel> for u8 {
    #[inline(always)]
    fn from(val: Enet2Event3inSel) -> u8 {
        Enet2Event3inSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2StopAck {
    #[doc = "ENET2 stop acknowledge is not asserted."]
    Enet2StopAck0 = 0x0,
    #[doc = "ENET2 stop acknowledge is asserted."]
    Enet2StopAck1 = 0x01,
}
impl Enet2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Enet2StopAck {
        Enet2StopAck::from_bits(val)
    }
}
impl From<Enet2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Enet2StopAck) -> u8 {
        Enet2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2StopReq {
    #[doc = "stop request off."]
    Enet2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Enet2StopReq1 = 0x01,
}
impl Enet2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Enet2StopReq {
        Enet2StopReq::from_bits(val)
    }
}
impl From<Enet2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Enet2StopReq) -> u8 {
        Enet2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enet2TxClkDir {
    #[doc = "ENET2_TX_CLK output driver is disabled."]
    Enet2TxClkDir0 = 0x0,
    #[doc = "ENET2_TX_CLK output driver is enabled."]
    Enet2TxClkDir1 = 0x01,
}
impl Enet2TxClkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enet2TxClkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enet2TxClkDir {
    #[inline(always)]
    fn from(val: u8) -> Enet2TxClkDir {
        Enet2TxClkDir::from_bits(val)
    }
}
impl From<Enet2TxClkDir> for u8 {
    #[inline(always)]
    fn from(val: Enet2TxClkDir) -> u8 {
        Enet2TxClkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetEvent3inSel {
    #[doc = "event3 source input from ENET_1588_EVENT3_IN."]
    EnetEvent3inSel0 = 0x0,
    #[doc = "event3 source input from GPT2.GPT_COMPARE1."]
    EnetEvent3inSel1 = 0x01,
}
impl EnetEvent3inSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetEvent3inSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetEvent3inSel {
    #[inline(always)]
    fn from(val: u8) -> EnetEvent3inSel {
        EnetEvent3inSel::from_bits(val)
    }
}
impl From<EnetEvent3inSel> for u8 {
    #[inline(always)]
    fn from(val: EnetEvent3inSel) -> u8 {
        EnetEvent3inSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetIpgClkSEn {
    #[doc = "ipg_clk_s is gated when there is no IPS access."]
    EnetIpgClkSEn0 = 0x0,
    #[doc = "ipg_clk_s is always on."]
    EnetIpgClkSEn1 = 0x01,
}
impl EnetIpgClkSEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetIpgClkSEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetIpgClkSEn {
    #[inline(always)]
    fn from(val: u8) -> EnetIpgClkSEn {
        EnetIpgClkSEn::from_bits(val)
    }
}
impl From<EnetIpgClkSEn> for u8 {
    #[inline(always)]
    fn from(val: EnetIpgClkSEn) -> u8 {
        EnetIpgClkSEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetStopAck {
    #[doc = "ENET1 stop acknowledge is not asserted."]
    EnetStopAck0 = 0x0,
    #[doc = "ENET1 stop acknowledge is asserted."]
    EnetStopAck1 = 0x01,
}
impl EnetStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetStopAck {
    #[inline(always)]
    fn from(val: u8) -> EnetStopAck {
        EnetStopAck::from_bits(val)
    }
}
impl From<EnetStopAck> for u8 {
    #[inline(always)]
    fn from(val: EnetStopAck) -> u8 {
        EnetStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetStopReq {
    #[doc = "stop request off."]
    EnetStopReq0 = 0x0,
    #[doc = "stop request on."]
    EnetStopReq1 = 0x01,
}
impl EnetStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetStopReq {
    #[inline(always)]
    fn from(val: u8) -> EnetStopReq {
        EnetStopReq::from_bits(val)
    }
}
impl From<EnetStopReq> for u8 {
    #[inline(always)]
    fn from(val: EnetStopReq) -> u8 {
        EnetStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExcMon {
    #[doc = "OKAY response."]
    ExcMon0 = 0x0,
    #[doc = "SLVError response (default)."]
    ExcMon1 = 0x01,
}
impl ExcMon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExcMon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExcMon {
    #[inline(always)]
    fn from(val: u8) -> ExcMon {
        ExcMon::from_bits(val)
    }
}
impl From<ExcMon> for u8 {
    #[inline(always)]
    fn from(val: ExcMon) -> u8 {
        ExcMon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1IpgDoze {
    #[doc = "FLEXIO1 is not in doze mode."]
    Flexio1IpgDoze0 = 0x0,
    #[doc = "FLEXIO1 is in doze mode."]
    Flexio1IpgDoze1 = 0x01,
}
impl Flexio1IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Flexio1IpgDoze {
        Flexio1IpgDoze::from_bits(val)
    }
}
impl From<Flexio1IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Flexio1IpgDoze) -> u8 {
        Flexio1IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1IpgStopMode {
    #[doc = "FlexIO1 is functional in Stop mode."]
    Flexio1IpgStopMode0 = 0x0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    Flexio1IpgStopMode1 = 0x01,
}
impl Flexio1IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Flexio1IpgStopMode {
        Flexio1IpgStopMode::from_bits(val)
    }
}
impl From<Flexio1IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Flexio1IpgStopMode) -> u8 {
        Flexio1IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1StopAck {
    #[doc = "FLEXIO1 stop acknowledge is not asserted."]
    Flexio1StopAck0 = 0x0,
    #[doc = "FLEXIO1 stop acknowledge is asserted."]
    Flexio1StopAck1 = 0x01,
}
impl Flexio1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Flexio1StopAck {
        Flexio1StopAck::from_bits(val)
    }
}
impl From<Flexio1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Flexio1StopAck) -> u8 {
        Flexio1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1StopReq {
    #[doc = "stop request off."]
    Flexio1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Flexio1StopReq1 = 0x01,
}
impl Flexio1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Flexio1StopReq {
        Flexio1StopReq::from_bits(val)
    }
}
impl From<Flexio1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Flexio1StopReq) -> u8 {
        Flexio1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2IpgDoze {
    #[doc = "FLEXIO2 is not in doze mode."]
    Flexio2IpgDoze0 = 0x0,
    #[doc = "FLEXIO2 is in doze mode."]
    Flexio2IpgDoze1 = 0x01,
}
impl Flexio2IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Flexio2IpgDoze {
        Flexio2IpgDoze::from_bits(val)
    }
}
impl From<Flexio2IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Flexio2IpgDoze) -> u8 {
        Flexio2IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2IpgStopMode {
    #[doc = "FlexIO2 is functional in Stop mode."]
    Flexio2IpgStopMode0 = 0x0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO2 is not functional in Stop mode."]
    Flexio2IpgStopMode1 = 0x01,
}
impl Flexio2IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Flexio2IpgStopMode {
        Flexio2IpgStopMode::from_bits(val)
    }
}
impl From<Flexio2IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Flexio2IpgStopMode) -> u8 {
        Flexio2IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2StopAck {
    #[doc = "FLEXIO2 stop acknowledge is not asserted."]
    Flexio2StopAck0 = 0x0,
    #[doc = "FLEXIO2 stop acknowledge is asserted (FLEXIO2 is in STOP mode)."]
    Flexio2StopAck1 = 0x01,
}
impl Flexio2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Flexio2StopAck {
        Flexio2StopAck::from_bits(val)
    }
}
impl From<Flexio2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Flexio2StopAck) -> u8 {
        Flexio2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio2StopReq {
    #[doc = "stop request off."]
    Flexio2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Flexio2StopReq1 = 0x01,
}
impl Flexio2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Flexio2StopReq {
        Flexio2StopReq::from_bits(val)
    }
}
impl From<Flexio2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Flexio2StopReq) -> u8 {
        Flexio2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio3IpgDoze {
    #[doc = "FLEXIO3 is not in doze mode."]
    Flexio3IpgDoze0 = 0x0,
    #[doc = "FLEXIO3 is in doze mode."]
    Flexio3IpgDoze1 = 0x01,
}
impl Flexio3IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio3IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio3IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Flexio3IpgDoze {
        Flexio3IpgDoze::from_bits(val)
    }
}
impl From<Flexio3IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Flexio3IpgDoze) -> u8 {
        Flexio3IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio3IpgStopMode {
    #[doc = "FlexIO3 is functional in Stop mode."]
    Flexio3IpgStopMode0 = 0x0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO3 is not functional in Stop mode."]
    Flexio3IpgStopMode1 = 0x01,
}
impl Flexio3IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio3IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio3IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Flexio3IpgStopMode {
        Flexio3IpgStopMode::from_bits(val)
    }
}
impl From<Flexio3IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Flexio3IpgStopMode) -> u8 {
        Flexio3IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio3StopAck {
    #[doc = "FLEXIO3 stop acknowledge is not asserted."]
    Flexio3StopAck0 = 0x0,
    #[doc = "FLEXIO3 stop acknowledge is asserted."]
    Flexio3StopAck1 = 0x01,
}
impl Flexio3StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio3StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio3StopAck {
    #[inline(always)]
    fn from(val: u8) -> Flexio3StopAck {
        Flexio3StopAck::from_bits(val)
    }
}
impl From<Flexio3StopAck> for u8 {
    #[inline(always)]
    fn from(val: Flexio3StopAck) -> u8 {
        Flexio3StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio3StopReq {
    #[doc = "stop request off."]
    Flexio3StopReq0 = 0x0,
    #[doc = "stop request on."]
    Flexio3StopReq1 = 0x01,
}
impl Flexio3StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio3StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio3StopReq {
    #[inline(always)]
    fn from(val: u8) -> Flexio3StopReq {
        Flexio3StopReq::from_bits(val)
    }
}
impl From<Flexio3StopReq> for u8 {
    #[inline(always)]
    fn from(val: Flexio3StopReq) -> u8 {
        Flexio3StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexramBankCfgSel {
    #[doc = "use fuse value to config."]
    FlexramBankCfgSel0 = 0x0,
    #[doc = "use FLEXRAM_BANK_CFG to config."]
    FlexramBankCfgSel1 = 0x01,
}
impl FlexramBankCfgSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexramBankCfgSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexramBankCfgSel {
    #[inline(always)]
    fn from(val: u8) -> FlexramBankCfgSel {
        FlexramBankCfgSel::from_bits(val)
    }
}
impl From<FlexramBankCfgSel> for u8 {
    #[inline(always)]
    fn from(val: FlexramBankCfgSel) -> u8 {
        FlexramBankCfgSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2StopAck {
    #[doc = "FLEXSPI2 stop acknowledge is not asserted."]
    Flexspi2StopAck0 = 0x0,
    #[doc = "FLEXSPI2 stop acknowledge is asserted."]
    Flexspi2StopAck1 = 0x01,
}
impl Flexspi2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2StopAck {
        Flexspi2StopAck::from_bits(val)
    }
}
impl From<Flexspi2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2StopAck) -> u8 {
        Flexspi2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexspi2StopReq {
    #[doc = "stop request off."]
    Flexspi2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Flexspi2StopReq1 = 0x01,
}
impl Flexspi2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexspi2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexspi2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Flexspi2StopReq {
        Flexspi2StopReq::from_bits(val)
    }
}
impl From<Flexspi2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Flexspi2StopReq) -> u8 {
        Flexspi2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiStopAck {
    #[doc = "FLEXSPI stop acknowledge is not asserted."]
    FlexspiStopAck0 = 0x0,
    #[doc = "FLEXSPI stop acknowledge is asserted."]
    FlexspiStopAck1 = 0x01,
}
impl FlexspiStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiStopAck {
    #[inline(always)]
    fn from(val: u8) -> FlexspiStopAck {
        FlexspiStopAck::from_bits(val)
    }
}
impl From<FlexspiStopAck> for u8 {
    #[inline(always)]
    fn from(val: FlexspiStopAck) -> u8 {
        FlexspiStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiStopReq {
    #[doc = "stop request off."]
    FlexspiStopReq0 = 0x0,
    #[doc = "stop request on."]
    FlexspiStopReq1 = 0x01,
}
impl FlexspiStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiStopReq {
    #[inline(always)]
    fn from(val: u8) -> FlexspiStopReq {
        FlexspiStopReq::from_bits(val)
    }
}
impl From<FlexspiStopReq> for u8 {
    #[inline(always)]
    fn from(val: FlexspiStopReq) -> u8 {
        FlexspiStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gint {
    #[doc = "Global interrupt request is not asserted."]
    Gint0 = 0x0,
    #[doc = "Global interrupt request is asserted."]
    Gint1 = 0x01,
}
impl Gint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gint {
    #[inline(always)]
    fn from(val: u8) -> Gint {
        Gint::from_bits(val)
    }
}
impl From<Gint> for u8 {
    #[inline(always)]
    fn from(val: Gint) -> u8 {
        Gint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt2Capin1Sel {
    #[doc = "source from GPT2_CAPTURE1."]
    Gpt2Capin1Sel0 = 0x0,
    #[doc = "source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)."]
    Gpt2Capin1Sel1 = 0x01,
}
impl Gpt2Capin1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt2Capin1Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt2Capin1Sel {
    #[inline(always)]
    fn from(val: u8) -> Gpt2Capin1Sel {
        Gpt2Capin1Sel::from_bits(val)
    }
}
impl From<Gpt2Capin1Sel> for u8 {
    #[inline(always)]
    fn from(val: Gpt2Capin1Sel) -> u8 {
        Gpt2Capin1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpt2Capin2Sel {
    #[doc = "source from GPT2_CAPTURE2."]
    Gpt2Capin2Sel0 = 0x0,
    #[doc = "source from ENET2_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)."]
    Gpt2Capin2Sel1 = 0x01,
}
impl Gpt2Capin2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpt2Capin2Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpt2Capin2Sel {
    #[inline(always)]
    fn from(val: u8) -> Gpt2Capin2Sel {
        Gpt2Capin2Sel::from_bits(val)
    }
}
impl From<Gpt2Capin2Sel> for u8 {
    #[inline(always)]
    fn from(val: Gpt2Capin2Sel) -> u8 {
        Gpt2Capin2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel10 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel100 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel101 = 0x01,
}
impl IomuxcXbarDirSel10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel10 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel10 {
        IomuxcXbarDirSel10::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel10> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel10) -> u8 {
        IomuxcXbarDirSel10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel11 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel110 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel111 = 0x01,
}
impl IomuxcXbarDirSel11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel11 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel11 {
        IomuxcXbarDirSel11::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel11> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel11) -> u8 {
        IomuxcXbarDirSel11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel12 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel120 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel121 = 0x01,
}
impl IomuxcXbarDirSel12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel12 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel12 {
        IomuxcXbarDirSel12::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel12> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel12) -> u8 {
        IomuxcXbarDirSel12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel13 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel130 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel131 = 0x01,
}
impl IomuxcXbarDirSel13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel13 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel13 {
        IomuxcXbarDirSel13::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel13> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel13) -> u8 {
        IomuxcXbarDirSel13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel14 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel140 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel141 = 0x01,
}
impl IomuxcXbarDirSel14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel14 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel14 {
        IomuxcXbarDirSel14::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel14> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel14) -> u8 {
        IomuxcXbarDirSel14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel15 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel150 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel151 = 0x01,
}
impl IomuxcXbarDirSel15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel15 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel15 {
        IomuxcXbarDirSel15::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel15> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel15) -> u8 {
        IomuxcXbarDirSel15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel16 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel160 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel161 = 0x01,
}
impl IomuxcXbarDirSel16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel16 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel16 {
        IomuxcXbarDirSel16::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel16> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel16) -> u8 {
        IomuxcXbarDirSel16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel17 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel170 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel171 = 0x01,
}
impl IomuxcXbarDirSel17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel17 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel17 {
        IomuxcXbarDirSel17::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel17> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel17) -> u8 {
        IomuxcXbarDirSel17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel18 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel180 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel181 = 0x01,
}
impl IomuxcXbarDirSel18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel18 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel18 {
        IomuxcXbarDirSel18::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel18> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel18) -> u8 {
        IomuxcXbarDirSel18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel19 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel190 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel191 = 0x01,
}
impl IomuxcXbarDirSel19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel19 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel19 {
        IomuxcXbarDirSel19::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel19> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel19) -> u8 {
        IomuxcXbarDirSel19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel4 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel40 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel41 = 0x01,
}
impl IomuxcXbarDirSel4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel4 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel4 {
        IomuxcXbarDirSel4::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel4> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel4) -> u8 {
        IomuxcXbarDirSel4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel5 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel50 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel51 = 0x01,
}
impl IomuxcXbarDirSel5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel5 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel5 {
        IomuxcXbarDirSel5::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel5> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel5) -> u8 {
        IomuxcXbarDirSel5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel6 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel60 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel61 = 0x01,
}
impl IomuxcXbarDirSel6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel6 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel6 {
        IomuxcXbarDirSel6::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel6> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel6) -> u8 {
        IomuxcXbarDirSel6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel7 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel70 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel71 = 0x01,
}
impl IomuxcXbarDirSel7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel7 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel7 {
        IomuxcXbarDirSel7::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel7> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel7) -> u8 {
        IomuxcXbarDirSel7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel8 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel80 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel81 = 0x01,
}
impl IomuxcXbarDirSel8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel8 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel8 {
        IomuxcXbarDirSel8::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel8> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel8) -> u8 {
        IomuxcXbarDirSel8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel9 {
    #[doc = "XBAR_INOUT as input."]
    IomuxcXbarDirSel90 = 0x0,
    #[doc = "XBAR_INOUT as output."]
    IomuxcXbarDirSel91 = 0x01,
}
impl IomuxcXbarDirSel9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel9 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel9 {
        IomuxcXbarDirSel9::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel9> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel9) -> u8 {
        IomuxcXbarDirSel9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L2MemDeepsleep {
    #[doc = "no force sleep control supported, memory deep sleep mode only entered when whole system in stop mode."]
    L2MemDeepsleep0 = 0x0,
    #[doc = "force memory into deep sleep mode."]
    L2MemDeepsleep1 = 0x01,
}
impl L2MemDeepsleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L2MemDeepsleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L2MemDeepsleep {
    #[inline(always)]
    fn from(val: u8) -> L2MemDeepsleep {
        L2MemDeepsleep::from_bits(val)
    }
}
impl From<L2MemDeepsleep> for u8 {
    #[inline(always)]
    fn from(val: L2MemDeepsleep) -> u8 {
        L2MemDeepsleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L2MemEnPowersaving {
    #[doc = "none memory power saving features enabled, SHUTDOWN/DEEPSLEEP/LIGHTSLEEP will have no effect."]
    L2MemEnPowersaving0 = 0x0,
    #[doc = "memory power saving features enabled, set SHUTDOWN/DEEPSLEEP/LIGHTSLEEP (priority high to low) to enable power saving levels."]
    L2MemEnPowersaving1 = 0x01,
}
impl L2MemEnPowersaving {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L2MemEnPowersaving {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L2MemEnPowersaving {
    #[inline(always)]
    fn from(val: u8) -> L2MemEnPowersaving {
        L2MemEnPowersaving::from_bits(val)
    }
}
impl From<L2MemEnPowersaving> for u8 {
    #[inline(always)]
    fn from(val: L2MemEnPowersaving) -> u8 {
        L2MemEnPowersaving::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockDbgEn {
    #[doc = "Field is not locked."]
    LockDbgEn0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockDbgEn1 = 0x01,
}
impl LockDbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockDbgEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockDbgEn {
    #[inline(always)]
    fn from(val: u8) -> LockDbgEn {
        LockDbgEn::from_bits(val)
    }
}
impl From<LockDbgEn> for u8 {
    #[inline(always)]
    fn from(val: LockDbgEn) -> u8 {
        LockDbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockDcpkeyOcotpOrKeymux {
    #[doc = "Field is not locked."]
    LockDcpkeyOcotpOrKeymux0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockDcpkeyOcotpOrKeymux1 = 0x01,
}
impl LockDcpkeyOcotpOrKeymux {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockDcpkeyOcotpOrKeymux {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockDcpkeyOcotpOrKeymux {
    #[inline(always)]
    fn from(val: u8) -> LockDcpkeyOcotpOrKeymux {
        LockDcpkeyOcotpOrKeymux::from_bits(val)
    }
}
impl From<LockDcpkeyOcotpOrKeymux> for u8 {
    #[inline(always)]
    fn from(val: LockDcpkeyOcotpOrKeymux) -> u8 {
        LockDcpkeyOcotpOrKeymux::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR0Bot {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR0Bot0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR0Bot1 = 0x01,
}
impl LockM7ApcAcR0Bot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR0Bot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR0Bot {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR0Bot {
        LockM7ApcAcR0Bot::from_bits(val)
    }
}
impl From<LockM7ApcAcR0Bot> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR0Bot) -> u8 {
        LockM7ApcAcR0Bot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR0Top {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR0Top0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR0Top1 = 0x01,
}
impl LockM7ApcAcR0Top {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR0Top {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR0Top {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR0Top {
        LockM7ApcAcR0Top::from_bits(val)
    }
}
impl From<LockM7ApcAcR0Top> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR0Top) -> u8 {
        LockM7ApcAcR0Top::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR1Bot {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR1Bot0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR1Bot1 = 0x01,
}
impl LockM7ApcAcR1Bot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR1Bot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR1Bot {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR1Bot {
        LockM7ApcAcR1Bot::from_bits(val)
    }
}
impl From<LockM7ApcAcR1Bot> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR1Bot) -> u8 {
        LockM7ApcAcR1Bot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR1Top {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR1Top0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR1Top1 = 0x01,
}
impl LockM7ApcAcR1Top {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR1Top {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR1Top {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR1Top {
        LockM7ApcAcR1Top::from_bits(val)
    }
}
impl From<LockM7ApcAcR1Top> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR1Top) -> u8 {
        LockM7ApcAcR1Top::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR2Bot {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR2Bot0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR2Bot1 = 0x01,
}
impl LockM7ApcAcR2Bot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR2Bot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR2Bot {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR2Bot {
        LockM7ApcAcR2Bot::from_bits(val)
    }
}
impl From<LockM7ApcAcR2Bot> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR2Bot) -> u8 {
        LockM7ApcAcR2Bot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR2Top {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR2Top0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR2Top1 = 0x01,
}
impl LockM7ApcAcR2Top {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR2Top {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR2Top {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR2Top {
        LockM7ApcAcR2Top::from_bits(val)
    }
}
impl From<LockM7ApcAcR2Top> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR2Top) -> u8 {
        LockM7ApcAcR2Top::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR3Bot {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR3Bot0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR3Bot1 = 0x01,
}
impl LockM7ApcAcR3Bot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR3Bot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR3Bot {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR3Bot {
        LockM7ApcAcR3Bot::from_bits(val)
    }
}
impl From<LockM7ApcAcR3Bot> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR3Bot) -> u8 {
        LockM7ApcAcR3Bot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockM7ApcAcR3Top {
    #[doc = "Register field \\[31:1\\] is not locked."]
    LockM7ApcAcR3Top0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)."]
    LockM7ApcAcR3Top1 = 0x01,
}
impl LockM7ApcAcR3Top {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockM7ApcAcR3Top {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockM7ApcAcR3Top {
    #[inline(always)]
    fn from(val: u8) -> LockM7ApcAcR3Top {
        LockM7ApcAcR3Top::from_bits(val)
    }
}
impl From<LockM7ApcAcR3Top> for u8 {
    #[inline(always)]
    fn from(val: LockM7ApcAcR3Top) -> u8 {
        LockM7ApcAcR3Top::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockNiden {
    #[doc = "Field is not locked."]
    LockNiden0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockNiden1 = 0x01,
}
impl LockNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockNiden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockNiden {
    #[inline(always)]
    fn from(val: u8) -> LockNiden {
        LockNiden::from_bits(val)
    }
}
impl From<LockNiden> for u8 {
    #[inline(always)]
    fn from(val: LockNiden) -> u8 {
        LockNiden::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LockOcram2TzAddr(u8);
impl LockOcram2TzAddr {
    #[doc = "Field is not locked."]
    pub const LockOcram2TzAddr0: Self = Self(0x0);
    #[doc = "Field is locked (read access only)."]
    pub const LockOcram2TzAddr1: Self = Self(0x01);
}
impl LockOcram2TzAddr {
    pub const fn from_bits(val: u8) -> LockOcram2TzAddr {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LockOcram2TzAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LockOcram2TzAddr0"),
            0x01 => f.write_str("LockOcram2TzAddr1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LockOcram2TzAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LockOcram2TzAddr0"),
            0x01 => defmt::write!(f, "LockOcram2TzAddr1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LockOcram2TzAddr {
    #[inline(always)]
    fn from(val: u8) -> LockOcram2TzAddr {
        LockOcram2TzAddr::from_bits(val)
    }
}
impl From<LockOcram2TzAddr> for u8 {
    #[inline(always)]
    fn from(val: LockOcram2TzAddr) -> u8 {
        LockOcram2TzAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockOcram2TzEn {
    #[doc = "Field is not locked."]
    LockOcram2TzEn0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockOcram2TzEn1 = 0x01,
}
impl LockOcram2TzEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockOcram2TzEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockOcram2TzEn {
    #[inline(always)]
    fn from(val: u8) -> LockOcram2TzEn {
        LockOcram2TzEn::from_bits(val)
    }
}
impl From<LockOcram2TzEn> for u8 {
    #[inline(always)]
    fn from(val: LockOcram2TzEn) -> u8 {
        LockOcram2TzEn::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LockOcramTzAddr(u8);
impl LockOcramTzAddr {
    #[doc = "Field is not locked."]
    pub const LockOcramTzAddr0: Self = Self(0x0);
    #[doc = "Field is locked (read access only)."]
    pub const LockOcramTzAddr1: Self = Self(0x01);
}
impl LockOcramTzAddr {
    pub const fn from_bits(val: u8) -> LockOcramTzAddr {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LockOcramTzAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LockOcramTzAddr0"),
            0x01 => f.write_str("LockOcramTzAddr1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LockOcramTzAddr {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LockOcramTzAddr0"),
            0x01 => defmt::write!(f, "LockOcramTzAddr1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LockOcramTzAddr {
    #[inline(always)]
    fn from(val: u8) -> LockOcramTzAddr {
        LockOcramTzAddr::from_bits(val)
    }
}
impl From<LockOcramTzAddr> for u8 {
    #[inline(always)]
    fn from(val: LockOcramTzAddr) -> u8 {
        LockOcramTzAddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockOcramTzEn {
    #[doc = "Field is not locked."]
    LockOcramTzEn0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockOcramTzEn1 = 0x01,
}
impl LockOcramTzEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockOcramTzEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockOcramTzEn {
    #[inline(always)]
    fn from(val: u8) -> LockOcramTzEn {
        LockOcramTzEn::from_bits(val)
    }
}
impl From<LockOcramTzEn> for u8 {
    #[inline(always)]
    fn from(val: LockOcramTzEn) -> u8 {
        LockOcramTzEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSecErrResp {
    #[doc = "Field is not locked."]
    LockSecErrResp0 = 0x0,
    #[doc = "Field is locked (read access only)."]
    LockSecErrResp1 = 0x01,
}
impl LockSecErrResp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSecErrResp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSecErrResp {
    #[inline(always)]
    fn from(val: u8) -> LockSecErrResp {
        LockSecErrResp::from_bits(val)
    }
}
impl From<LockSecErrResp> for u8 {
    #[inline(always)]
    fn from(val: LockSecErrResp) -> u8 {
        LockSecErrResp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1IpgDoze {
    #[doc = "not in doze mode."]
    Lpi2c1IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpi2c1IpgDoze1 = 0x01,
}
impl Lpi2c1IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1IpgDoze {
        Lpi2c1IpgDoze::from_bits(val)
    }
}
impl From<Lpi2c1IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1IpgDoze) -> u8 {
        Lpi2c1IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpi2c1IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpi2c1IpgStopMode1 = 0x01,
}
impl Lpi2c1IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1IpgStopMode {
        Lpi2c1IpgStopMode::from_bits(val)
    }
}
impl From<Lpi2c1IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1IpgStopMode) -> u8 {
        Lpi2c1IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpi2c1StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted (the module is in Stop mode)."]
    Lpi2c1StopAck1 = 0x01,
}
impl Lpi2c1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1StopAck {
        Lpi2c1StopAck::from_bits(val)
    }
}
impl From<Lpi2c1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1StopAck) -> u8 {
        Lpi2c1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1StopReq {
    #[doc = "stop request off."]
    Lpi2c1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpi2c1StopReq1 = 0x01,
}
impl Lpi2c1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c1StopReq {
        Lpi2c1StopReq::from_bits(val)
    }
}
impl From<Lpi2c1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c1StopReq) -> u8 {
        Lpi2c1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2IpgDoze {
    #[doc = "not in doze mode."]
    Lpi2c2IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpi2c2IpgDoze1 = 0x01,
}
impl Lpi2c2IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2IpgDoze {
        Lpi2c2IpgDoze::from_bits(val)
    }
}
impl From<Lpi2c2IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2IpgDoze) -> u8 {
        Lpi2c2IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpi2c2IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpi2c2IpgStopMode1 = 0x01,
}
impl Lpi2c2IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2IpgStopMode {
        Lpi2c2IpgStopMode::from_bits(val)
    }
}
impl From<Lpi2c2IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2IpgStopMode) -> u8 {
        Lpi2c2IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpi2c2StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpi2c2StopAck1 = 0x01,
}
impl Lpi2c2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2StopAck {
        Lpi2c2StopAck::from_bits(val)
    }
}
impl From<Lpi2c2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2StopAck) -> u8 {
        Lpi2c2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c2StopReq {
    #[doc = "stop request off."]
    Lpi2c2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpi2c2StopReq1 = 0x01,
}
impl Lpi2c2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c2StopReq {
        Lpi2c2StopReq::from_bits(val)
    }
}
impl From<Lpi2c2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c2StopReq) -> u8 {
        Lpi2c2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3IpgDoze {
    #[doc = "not in doze mode."]
    Lpi2c3IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpi2c3IpgDoze1 = 0x01,
}
impl Lpi2c3IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3IpgDoze {
        Lpi2c3IpgDoze::from_bits(val)
    }
}
impl From<Lpi2c3IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3IpgDoze) -> u8 {
        Lpi2c3IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpi2c3IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpi2c3IpgStopMode1 = 0x01,
}
impl Lpi2c3IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3IpgStopMode {
        Lpi2c3IpgStopMode::from_bits(val)
    }
}
impl From<Lpi2c3IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3IpgStopMode) -> u8 {
        Lpi2c3IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpi2c3StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpi2c3StopAck1 = 0x01,
}
impl Lpi2c3StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3StopAck {
        Lpi2c3StopAck::from_bits(val)
    }
}
impl From<Lpi2c3StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3StopAck) -> u8 {
        Lpi2c3StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c3StopReq {
    #[doc = "stop request off."]
    Lpi2c3StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpi2c3StopReq1 = 0x01,
}
impl Lpi2c3StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c3StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c3StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c3StopReq {
        Lpi2c3StopReq::from_bits(val)
    }
}
impl From<Lpi2c3StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c3StopReq) -> u8 {
        Lpi2c3StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4IpgDoze {
    #[doc = "not in doze mode."]
    Lpi2c4IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpi2c4IpgDoze1 = 0x01,
}
impl Lpi2c4IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4IpgDoze {
        Lpi2c4IpgDoze::from_bits(val)
    }
}
impl From<Lpi2c4IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4IpgDoze) -> u8 {
        Lpi2c4IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpi2c4IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpi2c4IpgStopMode1 = 0x01,
}
impl Lpi2c4IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4IpgStopMode {
        Lpi2c4IpgStopMode::from_bits(val)
    }
}
impl From<Lpi2c4IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4IpgStopMode) -> u8 {
        Lpi2c4IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpi2c4StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpi2c4StopAck1 = 0x01,
}
impl Lpi2c4StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4StopAck {
        Lpi2c4StopAck::from_bits(val)
    }
}
impl From<Lpi2c4StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4StopAck) -> u8 {
        Lpi2c4StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c4StopReq {
    #[doc = "stop request off."]
    Lpi2c4StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpi2c4StopReq1 = 0x01,
}
impl Lpi2c4StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2c4StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2c4StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpi2c4StopReq {
        Lpi2c4StopReq::from_bits(val)
    }
}
impl From<Lpi2c4StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpi2c4StopReq) -> u8 {
        Lpi2c4StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IpgDoze {
    #[doc = "not in doze mode."]
    Lpspi1IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpspi1IpgDoze1 = 0x01,
}
impl Lpspi1IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IpgDoze {
        Lpspi1IpgDoze::from_bits(val)
    }
}
impl From<Lpspi1IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IpgDoze) -> u8 {
        Lpspi1IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpspi1IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpspi1IpgStopMode1 = 0x01,
}
impl Lpspi1IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1IpgStopMode {
        Lpspi1IpgStopMode::from_bits(val)
    }
}
impl From<Lpspi1IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1IpgStopMode) -> u8 {
        Lpspi1IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpspi1StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpspi1StopAck1 = 0x01,
}
impl Lpspi1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1StopAck {
        Lpspi1StopAck::from_bits(val)
    }
}
impl From<Lpspi1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1StopAck) -> u8 {
        Lpspi1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi1StopReq {
    #[doc = "stop request off."]
    Lpspi1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpspi1StopReq1 = 0x01,
}
impl Lpspi1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpspi1StopReq {
        Lpspi1StopReq::from_bits(val)
    }
}
impl From<Lpspi1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpspi1StopReq) -> u8 {
        Lpspi1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IpgDoze {
    #[doc = "not in doze mode."]
    Lpspi2IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpspi2IpgDoze1 = 0x01,
}
impl Lpspi2IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IpgDoze {
        Lpspi2IpgDoze::from_bits(val)
    }
}
impl From<Lpspi2IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IpgDoze) -> u8 {
        Lpspi2IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpspi2IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpspi2IpgStopMode1 = 0x01,
}
impl Lpspi2IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2IpgStopMode {
        Lpspi2IpgStopMode::from_bits(val)
    }
}
impl From<Lpspi2IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2IpgStopMode) -> u8 {
        Lpspi2IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpspi2StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpspi2StopAck1 = 0x01,
}
impl Lpspi2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2StopAck {
        Lpspi2StopAck::from_bits(val)
    }
}
impl From<Lpspi2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2StopAck) -> u8 {
        Lpspi2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi2StopReq {
    #[doc = "stop request off."]
    Lpspi2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpspi2StopReq1 = 0x01,
}
impl Lpspi2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpspi2StopReq {
        Lpspi2StopReq::from_bits(val)
    }
}
impl From<Lpspi2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpspi2StopReq) -> u8 {
        Lpspi2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IpgDoze {
    #[doc = "not in doze mode."]
    Lpspi3IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpspi3IpgDoze1 = 0x01,
}
impl Lpspi3IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IpgDoze {
        Lpspi3IpgDoze::from_bits(val)
    }
}
impl From<Lpspi3IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IpgDoze) -> u8 {
        Lpspi3IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpspi3IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpspi3IpgStopMode1 = 0x01,
}
impl Lpspi3IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3IpgStopMode {
        Lpspi3IpgStopMode::from_bits(val)
    }
}
impl From<Lpspi3IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3IpgStopMode) -> u8 {
        Lpspi3IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpspi3StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpspi3StopAck1 = 0x01,
}
impl Lpspi3StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3StopAck {
        Lpspi3StopAck::from_bits(val)
    }
}
impl From<Lpspi3StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3StopAck) -> u8 {
        Lpspi3StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi3StopReq {
    #[doc = "stop request off."]
    Lpspi3StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpspi3StopReq1 = 0x01,
}
impl Lpspi3StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi3StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi3StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpspi3StopReq {
        Lpspi3StopReq::from_bits(val)
    }
}
impl From<Lpspi3StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpspi3StopReq) -> u8 {
        Lpspi3StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IpgDoze {
    #[doc = "not in doze mode."]
    Lpspi4IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpspi4IpgDoze1 = 0x01,
}
impl Lpspi4IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IpgDoze {
        Lpspi4IpgDoze::from_bits(val)
    }
}
impl From<Lpspi4IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IpgDoze) -> u8 {
        Lpspi4IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpspi4IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpspi4IpgStopMode1 = 0x01,
}
impl Lpspi4IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4IpgStopMode {
        Lpspi4IpgStopMode::from_bits(val)
    }
}
impl From<Lpspi4IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4IpgStopMode) -> u8 {
        Lpspi4IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpspi4StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpspi4StopAck1 = 0x01,
}
impl Lpspi4StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4StopAck {
        Lpspi4StopAck::from_bits(val)
    }
}
impl From<Lpspi4StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4StopAck) -> u8 {
        Lpspi4StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpspi4StopReq {
    #[doc = "stop request off."]
    Lpspi4StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpspi4StopReq1 = 0x01,
}
impl Lpspi4StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpspi4StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpspi4StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpspi4StopReq {
        Lpspi4StopReq::from_bits(val)
    }
}
impl From<Lpspi4StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpspi4StopReq) -> u8 {
        Lpspi4StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart1IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart1IpgDoze1 = 0x01,
}
impl Lpuart1IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1IpgDoze {
        Lpuart1IpgDoze::from_bits(val)
    }
}
impl From<Lpuart1IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1IpgDoze) -> u8 {
        Lpuart1IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart1IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart1IpgStopMode1 = 0x01,
}
impl Lpuart1IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1IpgStopMode {
        Lpuart1IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart1IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1IpgStopMode) -> u8 {
        Lpuart1IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart1StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart1StopAck1 = 0x01,
}
impl Lpuart1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1StopAck {
        Lpuart1StopAck::from_bits(val)
    }
}
impl From<Lpuart1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1StopAck) -> u8 {
        Lpuart1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart1StopReq {
    #[doc = "stop request off."]
    Lpuart1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart1StopReq1 = 0x01,
}
impl Lpuart1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart1StopReq {
        Lpuart1StopReq::from_bits(val)
    }
}
impl From<Lpuart1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart1StopReq) -> u8 {
        Lpuart1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart2IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart2IpgDoze1 = 0x01,
}
impl Lpuart2IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2IpgDoze {
        Lpuart2IpgDoze::from_bits(val)
    }
}
impl From<Lpuart2IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2IpgDoze) -> u8 {
        Lpuart2IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart2IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart2IpgStopMode1 = 0x01,
}
impl Lpuart2IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2IpgStopMode {
        Lpuart2IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart2IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2IpgStopMode) -> u8 {
        Lpuart2IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart2StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart2StopAck1 = 0x01,
}
impl Lpuart2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2StopAck {
        Lpuart2StopAck::from_bits(val)
    }
}
impl From<Lpuart2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2StopAck) -> u8 {
        Lpuart2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart2StopReq {
    #[doc = "stop request off."]
    Lpuart2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart2StopReq1 = 0x01,
}
impl Lpuart2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart2StopReq {
        Lpuart2StopReq::from_bits(val)
    }
}
impl From<Lpuart2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart2StopReq) -> u8 {
        Lpuart2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart3IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart3IpgDoze1 = 0x01,
}
impl Lpuart3IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3IpgDoze {
        Lpuart3IpgDoze::from_bits(val)
    }
}
impl From<Lpuart3IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3IpgDoze) -> u8 {
        Lpuart3IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart3IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart3IpgStopMode1 = 0x01,
}
impl Lpuart3IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3IpgStopMode {
        Lpuart3IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart3IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3IpgStopMode) -> u8 {
        Lpuart3IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart3StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart3StopAck1 = 0x01,
}
impl Lpuart3StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3StopAck {
        Lpuart3StopAck::from_bits(val)
    }
}
impl From<Lpuart3StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3StopAck) -> u8 {
        Lpuart3StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart3StopReq {
    #[doc = "stop request off."]
    Lpuart3StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart3StopReq1 = 0x01,
}
impl Lpuart3StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart3StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart3StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart3StopReq {
        Lpuart3StopReq::from_bits(val)
    }
}
impl From<Lpuart3StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart3StopReq) -> u8 {
        Lpuart3StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart4IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart4IpgDoze1 = 0x01,
}
impl Lpuart4IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4IpgDoze {
        Lpuart4IpgDoze::from_bits(val)
    }
}
impl From<Lpuart4IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4IpgDoze) -> u8 {
        Lpuart4IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart4IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart4IpgStopMode1 = 0x01,
}
impl Lpuart4IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4IpgStopMode {
        Lpuart4IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart4IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4IpgStopMode) -> u8 {
        Lpuart4IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart4StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart4StopAck1 = 0x01,
}
impl Lpuart4StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4StopAck {
        Lpuart4StopAck::from_bits(val)
    }
}
impl From<Lpuart4StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4StopAck) -> u8 {
        Lpuart4StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart4StopReq {
    #[doc = "stop request off."]
    Lpuart4StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart4StopReq1 = 0x01,
}
impl Lpuart4StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart4StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart4StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart4StopReq {
        Lpuart4StopReq::from_bits(val)
    }
}
impl From<Lpuart4StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart4StopReq) -> u8 {
        Lpuart4StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart5IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart5IpgDoze1 = 0x01,
}
impl Lpuart5IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IpgDoze {
        Lpuart5IpgDoze::from_bits(val)
    }
}
impl From<Lpuart5IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IpgDoze) -> u8 {
        Lpuart5IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart5IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart5IpgStopMode1 = 0x01,
}
impl Lpuart5IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5IpgStopMode {
        Lpuart5IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart5IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5IpgStopMode) -> u8 {
        Lpuart5IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart5StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart5StopAck1 = 0x01,
}
impl Lpuart5StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5StopAck {
        Lpuart5StopAck::from_bits(val)
    }
}
impl From<Lpuart5StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5StopAck) -> u8 {
        Lpuart5StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart5StopReq {
    #[doc = "stop request off."]
    Lpuart5StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart5StopReq1 = 0x01,
}
impl Lpuart5StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart5StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart5StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart5StopReq {
        Lpuart5StopReq::from_bits(val)
    }
}
impl From<Lpuart5StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart5StopReq) -> u8 {
        Lpuart5StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart6IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart6IpgDoze1 = 0x01,
}
impl Lpuart6IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IpgDoze {
        Lpuart6IpgDoze::from_bits(val)
    }
}
impl From<Lpuart6IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IpgDoze) -> u8 {
        Lpuart6IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart6IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart6IpgStopMode1 = 0x01,
}
impl Lpuart6IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6IpgStopMode {
        Lpuart6IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart6IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6IpgStopMode) -> u8 {
        Lpuart6IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart6StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart6StopAck1 = 0x01,
}
impl Lpuart6StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6StopAck {
        Lpuart6StopAck::from_bits(val)
    }
}
impl From<Lpuart6StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6StopAck) -> u8 {
        Lpuart6StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart6StopReq {
    #[doc = "stop request off."]
    Lpuart6StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart6StopReq1 = 0x01,
}
impl Lpuart6StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart6StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart6StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart6StopReq {
        Lpuart6StopReq::from_bits(val)
    }
}
impl From<Lpuart6StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart6StopReq) -> u8 {
        Lpuart6StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart7IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart7IpgDoze1 = 0x01,
}
impl Lpuart7IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7IpgDoze {
        Lpuart7IpgDoze::from_bits(val)
    }
}
impl From<Lpuart7IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7IpgDoze) -> u8 {
        Lpuart7IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart7IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart7IpgStopMode1 = 0x01,
}
impl Lpuart7IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7IpgStopMode {
        Lpuart7IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart7IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7IpgStopMode) -> u8 {
        Lpuart7IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart7StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted."]
    Lpuart7StopAck1 = 0x01,
}
impl Lpuart7StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7StopAck {
        Lpuart7StopAck::from_bits(val)
    }
}
impl From<Lpuart7StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7StopAck) -> u8 {
        Lpuart7StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart7StopReq {
    #[doc = "stop request off."]
    Lpuart7StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart7StopReq1 = 0x01,
}
impl Lpuart7StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart7StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart7StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart7StopReq {
        Lpuart7StopReq::from_bits(val)
    }
}
impl From<Lpuart7StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart7StopReq) -> u8 {
        Lpuart7StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8IpgDoze {
    #[doc = "not in doze mode."]
    Lpuart8IpgDoze0 = 0x0,
    #[doc = "in doze mode."]
    Lpuart8IpgDoze1 = 0x01,
}
impl Lpuart8IpgDoze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8IpgDoze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8IpgDoze {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8IpgDoze {
        Lpuart8IpgDoze::from_bits(val)
    }
}
impl From<Lpuart8IpgDoze> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8IpgDoze) -> u8 {
        Lpuart8IpgDoze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8IpgStopMode {
    #[doc = "the module is functional in Stop mode."]
    Lpuart8IpgStopMode0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted."]
    Lpuart8IpgStopMode1 = 0x01,
}
impl Lpuart8IpgStopMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8IpgStopMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8IpgStopMode {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8IpgStopMode {
        Lpuart8IpgStopMode::from_bits(val)
    }
}
impl From<Lpuart8IpgStopMode> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8IpgStopMode) -> u8 {
        Lpuart8IpgStopMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8StopAck {
    #[doc = "stop acknowledge is not asserted."]
    Lpuart8StopAck0 = 0x0,
    #[doc = "stop acknowledge is asserted (the module is in Stop mode)."]
    Lpuart8StopAck1 = 0x01,
}
impl Lpuart8StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8StopAck {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8StopAck {
        Lpuart8StopAck::from_bits(val)
    }
}
impl From<Lpuart8StopAck> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8StopAck) -> u8 {
        Lpuart8StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpuart8StopReq {
    #[doc = "stop request off."]
    Lpuart8StopReq0 = 0x0,
    #[doc = "stop request on."]
    Lpuart8StopReq1 = 0x01,
}
impl Lpuart8StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpuart8StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpuart8StopReq {
    #[inline(always)]
    fn from(val: u8) -> Lpuart8StopReq {
        Lpuart8StopReq::from_bits(val)
    }
}
impl From<Lpuart8StopReq> for u8 {
    #[inline(always)]
    fn from(val: Lpuart8StopReq) -> u8 {
        Lpuart8StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR0Ctrl {
    #[doc = "No access protection."]
    M7ApcAcR0Ctrl0 = 0x0,
    #[doc = "M7 debug protection enabled."]
    M7ApcAcR0Ctrl1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR0Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR0Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR0Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR0Ctrl {
        M7ApcAcR0Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR0Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR0Ctrl) -> u8 {
        M7ApcAcR0Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR1Ctrl {
    #[doc = "No access protection."]
    M7ApcAcR1Ctrl0 = 0x0,
    #[doc = "M7 debug protection enabled."]
    M7ApcAcR1Ctrl1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR1Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR1Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR1Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR1Ctrl {
        M7ApcAcR1Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR1Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR1Ctrl) -> u8 {
        M7ApcAcR1Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR2Ctrl {
    #[doc = "No access protection."]
    M7ApcAcR2Ctrl0 = 0x0,
    #[doc = "M7 debug protection enabled."]
    M7ApcAcR2Ctrl1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR2Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR2Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR2Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR2Ctrl {
        M7ApcAcR2Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR2Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR2Ctrl) -> u8 {
        M7ApcAcR2Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7ApcAcR3Ctrl {
    #[doc = "No access protection."]
    M7ApcAcR3Ctrl0 = 0x0,
    #[doc = "M7 debug protection enabled."]
    M7ApcAcR3Ctrl1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl M7ApcAcR3Ctrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7ApcAcR3Ctrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7ApcAcR3Ctrl {
    #[inline(always)]
    fn from(val: u8) -> M7ApcAcR3Ctrl {
        M7ApcAcR3Ctrl::from_bits(val)
    }
}
impl From<M7ApcAcR3Ctrl> for u8 {
    #[inline(always)]
    fn from(val: M7ApcAcR3Ctrl) -> u8 {
        M7ApcAcR3Ctrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MqsClkDiv {
    #[doc = "mclk frequency = 1/1 * hmclk frequency."]
    Divide1 = 0x0,
    #[doc = "mclk frequency = 1/2 * hmclk frequency."]
    Divide2 = 0x01,
    #[doc = "mclk frequency = 1/3 * hmclk frequency."]
    Divide3 = 0x02,
    #[doc = "mclk frequency = 1/4 * hmclk frequency."]
    Divide4 = 0x03,
    #[doc = "mclk frequency = 1/5 * hmclk frequency."]
    Divide5 = 0x04,
    #[doc = "mclk frequency = 1/6 * hmclk frequency."]
    Divide6 = 0x05,
    #[doc = "mclk frequency = 1/7 * hmclk frequency."]
    Divide7 = 0x06,
    #[doc = "mclk frequency = 1/8 * hmclk frequency."]
    Divide8 = 0x07,
    #[doc = "mclk frequency = 1/9 * hmclk frequency."]
    Divide9 = 0x08,
    #[doc = "mclk frequency = 1/10 * hmclk frequency."]
    Divide10 = 0x09,
    #[doc = "mclk frequency = 1/11 * hmclk frequency."]
    Divide11 = 0x0a,
    #[doc = "mclk frequency = 1/12 * hmclk frequency."]
    Divide12 = 0x0b,
    #[doc = "mclk frequency = 1/13 * hmclk frequency."]
    Divide13 = 0x0c,
    #[doc = "mclk frequency = 1/14 * hmclk frequency."]
    Divide14 = 0x0d,
    #[doc = "mclk frequency = 1/15 * hmclk frequency."]
    Divide15 = 0x0e,
    #[doc = "mclk frequency = 1/16 * hmclk frequency."]
    Divide16 = 0x0f,
    #[doc = "mclk frequency = 1/17 * hmclk frequency."]
    Divide17 = 0x10,
    #[doc = "mclk frequency = 1/18 * hmclk frequency."]
    Divide18 = 0x11,
    #[doc = "mclk frequency = 1/19 * hmclk frequency."]
    Divide19 = 0x12,
    #[doc = "mclk frequency = 1/20 * hmclk frequency."]
    Divide20 = 0x13,
    #[doc = "mclk frequency = 1/21 * hmclk frequency."]
    Divide21 = 0x14,
    #[doc = "mclk frequency = 1/22 * hmclk frequency."]
    Divide22 = 0x15,
    #[doc = "mclk frequency = 1/23 * hmclk frequency."]
    Divide23 = 0x16,
    #[doc = "mclk frequency = 1/24 * hmclk frequency."]
    Divide24 = 0x17,
    #[doc = "mclk frequency = 1/25 * hmclk frequency."]
    Divide25 = 0x18,
    #[doc = "mclk frequency = 1/26 * hmclk frequency."]
    Divide26 = 0x19,
    #[doc = "mclk frequency = 1/27 * hmclk frequency."]
    Divide27 = 0x1a,
    #[doc = "mclk frequency = 1/28 * hmclk frequency."]
    Divide28 = 0x1b,
    #[doc = "mclk frequency = 1/29 * hmclk frequency."]
    Divide29 = 0x1c,
    #[doc = "mclk frequency = 1/30 * hmclk frequency."]
    Divide30 = 0x1d,
    #[doc = "mclk frequency = 1/31 * hmclk frequency."]
    Divide31 = 0x1e,
    #[doc = "mclk frequency = 1/32 * hmclk frequency."]
    Divide32 = 0x1f,
    #[doc = "mclk frequency = 1/33 * hmclk frequency."]
    Divide33 = 0x20,
    #[doc = "mclk frequency = 1/34 * hmclk frequency."]
    Divide34 = 0x21,
    #[doc = "mclk frequency = 1/35 * hmclk frequency."]
    Divide35 = 0x22,
    #[doc = "mclk frequency = 1/36 * hmclk frequency."]
    Divide36 = 0x23,
    #[doc = "mclk frequency = 1/37 * hmclk frequency."]
    Divide37 = 0x24,
    #[doc = "mclk frequency = 1/38 * hmclk frequency."]
    Divide38 = 0x25,
    #[doc = "mclk frequency = 1/39 * hmclk frequency."]
    Divide39 = 0x26,
    #[doc = "mclk frequency = 1/40 * hmclk frequency."]
    Divide40 = 0x27,
    #[doc = "mclk frequency = 1/41 * hmclk frequency."]
    Divide41 = 0x28,
    #[doc = "mclk frequency = 1/42 * hmclk frequency."]
    Divide42 = 0x29,
    #[doc = "mclk frequency = 1/43 * hmclk frequency."]
    Divide43 = 0x2a,
    #[doc = "mclk frequency = 1/44 * hmclk frequency."]
    Divide44 = 0x2b,
    #[doc = "mclk frequency = 1/45 * hmclk frequency."]
    Divide45 = 0x2c,
    #[doc = "mclk frequency = 1/46 * hmclk frequency."]
    Divide46 = 0x2d,
    #[doc = "mclk frequency = 1/47 * hmclk frequency."]
    Divide47 = 0x2e,
    #[doc = "mclk frequency = 1/48 * hmclk frequency."]
    Divide48 = 0x2f,
    #[doc = "mclk frequency = 1/49 * hmclk frequency."]
    Divide49 = 0x30,
    #[doc = "mclk frequency = 1/50 * hmclk frequency."]
    Divide50 = 0x31,
    #[doc = "mclk frequency = 1/51 * hmclk frequency."]
    Divide51 = 0x32,
    #[doc = "mclk frequency = 1/52 * hmclk frequency."]
    Divide52 = 0x33,
    #[doc = "mclk frequency = 1/53 * hmclk frequency."]
    Divide53 = 0x34,
    #[doc = "mclk frequency = 1/54 * hmclk frequency."]
    Divide54 = 0x35,
    #[doc = "mclk frequency = 1/55 * hmclk frequency."]
    Divide55 = 0x36,
    #[doc = "mclk frequency = 1/56 * hmclk frequency."]
    Divide56 = 0x37,
    #[doc = "mclk frequency = 1/57 * hmclk frequency."]
    Divide57 = 0x38,
    #[doc = "mclk frequency = 1/58 * hmclk frequency."]
    Divide58 = 0x39,
    #[doc = "mclk frequency = 1/59 * hmclk frequency."]
    Divide59 = 0x3a,
    #[doc = "mclk frequency = 1/60 * hmclk frequency."]
    Divide60 = 0x3b,
    #[doc = "mclk frequency = 1/61 * hmclk frequency."]
    Divide61 = 0x3c,
    #[doc = "mclk frequency = 1/62 * hmclk frequency."]
    Divide62 = 0x3d,
    #[doc = "mclk frequency = 1/63 * hmclk frequency."]
    Divide63 = 0x3e,
    #[doc = "mclk frequency = 1/64 * hmclk frequency."]
    Divide64 = 0x3f,
    #[doc = "mclk frequency = 1/65 * hmclk frequency."]
    Divide65 = 0x40,
    #[doc = "mclk frequency = 1/66 * hmclk frequency."]
    Divide66 = 0x41,
    #[doc = "mclk frequency = 1/67 * hmclk frequency."]
    Divide67 = 0x42,
    #[doc = "mclk frequency = 1/68 * hmclk frequency."]
    Divide68 = 0x43,
    #[doc = "mclk frequency = 1/69 * hmclk frequency."]
    Divide69 = 0x44,
    #[doc = "mclk frequency = 1/70 * hmclk frequency."]
    Divide70 = 0x45,
    #[doc = "mclk frequency = 1/71 * hmclk frequency."]
    Divide71 = 0x46,
    #[doc = "mclk frequency = 1/72 * hmclk frequency."]
    Divide72 = 0x47,
    #[doc = "mclk frequency = 1/73 * hmclk frequency."]
    Divide73 = 0x48,
    #[doc = "mclk frequency = 1/74 * hmclk frequency."]
    Divide74 = 0x49,
    #[doc = "mclk frequency = 1/75 * hmclk frequency."]
    Divide75 = 0x4a,
    #[doc = "mclk frequency = 1/76 * hmclk frequency."]
    Divide76 = 0x4b,
    #[doc = "mclk frequency = 1/77 * hmclk frequency."]
    Divide77 = 0x4c,
    #[doc = "mclk frequency = 1/78 * hmclk frequency."]
    Divide78 = 0x4d,
    #[doc = "mclk frequency = 1/79 * hmclk frequency."]
    Divide79 = 0x4e,
    #[doc = "mclk frequency = 1/80 * hmclk frequency."]
    Divide80 = 0x4f,
    #[doc = "mclk frequency = 1/81 * hmclk frequency."]
    Divide81 = 0x50,
    #[doc = "mclk frequency = 1/82 * hmclk frequency."]
    Divide82 = 0x51,
    #[doc = "mclk frequency = 1/83 * hmclk frequency."]
    Divide83 = 0x52,
    #[doc = "mclk frequency = 1/84 * hmclk frequency."]
    Divide84 = 0x53,
    #[doc = "mclk frequency = 1/85 * hmclk frequency."]
    Divide85 = 0x54,
    #[doc = "mclk frequency = 1/86 * hmclk frequency."]
    Divide86 = 0x55,
    #[doc = "mclk frequency = 1/87 * hmclk frequency."]
    Divide87 = 0x56,
    #[doc = "mclk frequency = 1/88 * hmclk frequency."]
    Divide88 = 0x57,
    #[doc = "mclk frequency = 1/89 * hmclk frequency."]
    Divide89 = 0x58,
    #[doc = "mclk frequency = 1/90 * hmclk frequency."]
    Divide90 = 0x59,
    #[doc = "mclk frequency = 1/91 * hmclk frequency."]
    Divide91 = 0x5a,
    #[doc = "mclk frequency = 1/92 * hmclk frequency."]
    Divide92 = 0x5b,
    #[doc = "mclk frequency = 1/93 * hmclk frequency."]
    Divide93 = 0x5c,
    #[doc = "mclk frequency = 1/94 * hmclk frequency."]
    Divide94 = 0x5d,
    #[doc = "mclk frequency = 1/95 * hmclk frequency."]
    Divide95 = 0x5e,
    #[doc = "mclk frequency = 1/96 * hmclk frequency."]
    Divide96 = 0x5f,
    #[doc = "mclk frequency = 1/97 * hmclk frequency."]
    Divide97 = 0x60,
    #[doc = "mclk frequency = 1/98 * hmclk frequency."]
    Divide98 = 0x61,
    #[doc = "mclk frequency = 1/99 * hmclk frequency."]
    Divide99 = 0x62,
    #[doc = "mclk frequency = 1/100 * hmclk frequency."]
    Divide100 = 0x63,
    #[doc = "mclk frequency = 1/101 * hmclk frequency."]
    Divide101 = 0x64,
    #[doc = "mclk frequency = 1/102 * hmclk frequency."]
    Divide102 = 0x65,
    #[doc = "mclk frequency = 1/103 * hmclk frequency."]
    Divide103 = 0x66,
    #[doc = "mclk frequency = 1/104 * hmclk frequency."]
    Divide104 = 0x67,
    #[doc = "mclk frequency = 1/105 * hmclk frequency."]
    Divide105 = 0x68,
    #[doc = "mclk frequency = 1/106 * hmclk frequency."]
    Divide106 = 0x69,
    #[doc = "mclk frequency = 1/107 * hmclk frequency."]
    Divide107 = 0x6a,
    #[doc = "mclk frequency = 1/108 * hmclk frequency."]
    Divide108 = 0x6b,
    #[doc = "mclk frequency = 1/109 * hmclk frequency."]
    Divide109 = 0x6c,
    #[doc = "mclk frequency = 1/110 * hmclk frequency."]
    Divide110 = 0x6d,
    #[doc = "mclk frequency = 1/111 * hmclk frequency."]
    Divide111 = 0x6e,
    #[doc = "mclk frequency = 1/112 * hmclk frequency."]
    Divide112 = 0x6f,
    #[doc = "mclk frequency = 1/113 * hmclk frequency."]
    Divide113 = 0x70,
    #[doc = "mclk frequency = 1/114 * hmclk frequency."]
    Divide114 = 0x71,
    #[doc = "mclk frequency = 1/115 * hmclk frequency."]
    Divide115 = 0x72,
    #[doc = "mclk frequency = 1/116 * hmclk frequency."]
    Divide116 = 0x73,
    #[doc = "mclk frequency = 1/117 * hmclk frequency."]
    Divide117 = 0x74,
    #[doc = "mclk frequency = 1/118 * hmclk frequency."]
    Divide118 = 0x75,
    #[doc = "mclk frequency = 1/119 * hmclk frequency."]
    Divide119 = 0x76,
    #[doc = "mclk frequency = 1/120 * hmclk frequency."]
    Divide120 = 0x77,
    #[doc = "mclk frequency = 1/121 * hmclk frequency."]
    Divide121 = 0x78,
    #[doc = "mclk frequency = 1/122 * hmclk frequency."]
    Divide122 = 0x79,
    #[doc = "mclk frequency = 1/123 * hmclk frequency."]
    Divide123 = 0x7a,
    #[doc = "mclk frequency = 1/124 * hmclk frequency."]
    Divide124 = 0x7b,
    #[doc = "mclk frequency = 1/125 * hmclk frequency."]
    Divide125 = 0x7c,
    #[doc = "mclk frequency = 1/126 * hmclk frequency."]
    Divide126 = 0x7d,
    #[doc = "mclk frequency = 1/127 * hmclk frequency."]
    Divide127 = 0x7e,
    #[doc = "mclk frequency = 1/128 * hmclk frequency."]
    Divide128 = 0x7f,
    #[doc = "mclk frequency = 1/129 * hmclk frequency."]
    Divide129 = 0x80,
    #[doc = "mclk frequency = 1/130 * hmclk frequency."]
    Divide130 = 0x81,
    #[doc = "mclk frequency = 1/131 * hmclk frequency."]
    Divide131 = 0x82,
    #[doc = "mclk frequency = 1/132 * hmclk frequency."]
    Divide132 = 0x83,
    #[doc = "mclk frequency = 1/133 * hmclk frequency."]
    Divide133 = 0x84,
    #[doc = "mclk frequency = 1/134 * hmclk frequency."]
    Divide134 = 0x85,
    #[doc = "mclk frequency = 1/135 * hmclk frequency."]
    Divide135 = 0x86,
    #[doc = "mclk frequency = 1/136 * hmclk frequency."]
    Divide136 = 0x87,
    #[doc = "mclk frequency = 1/137 * hmclk frequency."]
    Divide137 = 0x88,
    #[doc = "mclk frequency = 1/138 * hmclk frequency."]
    Divide138 = 0x89,
    #[doc = "mclk frequency = 1/139 * hmclk frequency."]
    Divide139 = 0x8a,
    #[doc = "mclk frequency = 1/140 * hmclk frequency."]
    Divide140 = 0x8b,
    #[doc = "mclk frequency = 1/141 * hmclk frequency."]
    Divide141 = 0x8c,
    #[doc = "mclk frequency = 1/142 * hmclk frequency."]
    Divide142 = 0x8d,
    #[doc = "mclk frequency = 1/143 * hmclk frequency."]
    Divide143 = 0x8e,
    #[doc = "mclk frequency = 1/144 * hmclk frequency."]
    Divide144 = 0x8f,
    #[doc = "mclk frequency = 1/145 * hmclk frequency."]
    Divide145 = 0x90,
    #[doc = "mclk frequency = 1/146 * hmclk frequency."]
    Divide146 = 0x91,
    #[doc = "mclk frequency = 1/147 * hmclk frequency."]
    Divide147 = 0x92,
    #[doc = "mclk frequency = 1/148 * hmclk frequency."]
    Divide148 = 0x93,
    #[doc = "mclk frequency = 1/149 * hmclk frequency."]
    Divide149 = 0x94,
    #[doc = "mclk frequency = 1/150 * hmclk frequency."]
    Divide150 = 0x95,
    #[doc = "mclk frequency = 1/151 * hmclk frequency."]
    Divide151 = 0x96,
    #[doc = "mclk frequency = 1/152 * hmclk frequency."]
    Divide152 = 0x97,
    #[doc = "mclk frequency = 1/153 * hmclk frequency."]
    Divide153 = 0x98,
    #[doc = "mclk frequency = 1/154 * hmclk frequency."]
    Divide154 = 0x99,
    #[doc = "mclk frequency = 1/155 * hmclk frequency."]
    Divide155 = 0x9a,
    #[doc = "mclk frequency = 1/156 * hmclk frequency."]
    Divide156 = 0x9b,
    #[doc = "mclk frequency = 1/157 * hmclk frequency."]
    Divide157 = 0x9c,
    #[doc = "mclk frequency = 1/158 * hmclk frequency."]
    Divide158 = 0x9d,
    #[doc = "mclk frequency = 1/159 * hmclk frequency."]
    Divide159 = 0x9e,
    #[doc = "mclk frequency = 1/160 * hmclk frequency."]
    Divide160 = 0x9f,
    #[doc = "mclk frequency = 1/161 * hmclk frequency."]
    Divide161 = 0xa0,
    #[doc = "mclk frequency = 1/162 * hmclk frequency."]
    Divide162 = 0xa1,
    #[doc = "mclk frequency = 1/163 * hmclk frequency."]
    Divide163 = 0xa2,
    #[doc = "mclk frequency = 1/164 * hmclk frequency."]
    Divide164 = 0xa3,
    #[doc = "mclk frequency = 1/165 * hmclk frequency."]
    Divide165 = 0xa4,
    #[doc = "mclk frequency = 1/166 * hmclk frequency."]
    Divide166 = 0xa5,
    #[doc = "mclk frequency = 1/167 * hmclk frequency."]
    Divide167 = 0xa6,
    #[doc = "mclk frequency = 1/168 * hmclk frequency."]
    Divide168 = 0xa7,
    #[doc = "mclk frequency = 1/169 * hmclk frequency."]
    Divide169 = 0xa8,
    #[doc = "mclk frequency = 1/170 * hmclk frequency."]
    Divide170 = 0xa9,
    #[doc = "mclk frequency = 1/171 * hmclk frequency."]
    Divide171 = 0xaa,
    #[doc = "mclk frequency = 1/172 * hmclk frequency."]
    Divide172 = 0xab,
    #[doc = "mclk frequency = 1/173 * hmclk frequency."]
    Divide173 = 0xac,
    #[doc = "mclk frequency = 1/174 * hmclk frequency."]
    Divide174 = 0xad,
    #[doc = "mclk frequency = 1/175 * hmclk frequency."]
    Divide175 = 0xae,
    #[doc = "mclk frequency = 1/176 * hmclk frequency."]
    Divide176 = 0xaf,
    #[doc = "mclk frequency = 1/177 * hmclk frequency."]
    Divide177 = 0xb0,
    #[doc = "mclk frequency = 1/178 * hmclk frequency."]
    Divide178 = 0xb1,
    #[doc = "mclk frequency = 1/179 * hmclk frequency."]
    Divide179 = 0xb2,
    #[doc = "mclk frequency = 1/180 * hmclk frequency."]
    Divide180 = 0xb3,
    #[doc = "mclk frequency = 1/181 * hmclk frequency."]
    Divide181 = 0xb4,
    #[doc = "mclk frequency = 1/182 * hmclk frequency."]
    Divide182 = 0xb5,
    #[doc = "mclk frequency = 1/183 * hmclk frequency."]
    Divide183 = 0xb6,
    #[doc = "mclk frequency = 1/184 * hmclk frequency."]
    Divide184 = 0xb7,
    #[doc = "mclk frequency = 1/185 * hmclk frequency."]
    Divide185 = 0xb8,
    #[doc = "mclk frequency = 1/186 * hmclk frequency."]
    Divide186 = 0xb9,
    #[doc = "mclk frequency = 1/187 * hmclk frequency."]
    Divide187 = 0xba,
    #[doc = "mclk frequency = 1/188 * hmclk frequency."]
    Divide188 = 0xbb,
    #[doc = "mclk frequency = 1/189 * hmclk frequency."]
    Divide189 = 0xbc,
    #[doc = "mclk frequency = 1/190 * hmclk frequency."]
    Divide190 = 0xbd,
    #[doc = "mclk frequency = 1/191 * hmclk frequency."]
    Divide191 = 0xbe,
    #[doc = "mclk frequency = 1/192 * hmclk frequency."]
    Divide192 = 0xbf,
    #[doc = "mclk frequency = 1/193 * hmclk frequency."]
    Divide193 = 0xc0,
    #[doc = "mclk frequency = 1/194 * hmclk frequency."]
    Divide194 = 0xc1,
    #[doc = "mclk frequency = 1/195 * hmclk frequency."]
    Divide195 = 0xc2,
    #[doc = "mclk frequency = 1/196 * hmclk frequency."]
    Divide196 = 0xc3,
    #[doc = "mclk frequency = 1/197 * hmclk frequency."]
    Divide197 = 0xc4,
    #[doc = "mclk frequency = 1/198 * hmclk frequency."]
    Divide198 = 0xc5,
    #[doc = "mclk frequency = 1/199 * hmclk frequency."]
    Divide199 = 0xc6,
    #[doc = "mclk frequency = 1/200 * hmclk frequency."]
    Divide200 = 0xc7,
    #[doc = "mclk frequency = 1/201 * hmclk frequency."]
    Divide201 = 0xc8,
    #[doc = "mclk frequency = 1/202 * hmclk frequency."]
    Divide202 = 0xc9,
    #[doc = "mclk frequency = 1/203 * hmclk frequency."]
    Divide203 = 0xca,
    #[doc = "mclk frequency = 1/204 * hmclk frequency."]
    Divide204 = 0xcb,
    #[doc = "mclk frequency = 1/205 * hmclk frequency."]
    Divide205 = 0xcc,
    #[doc = "mclk frequency = 1/206 * hmclk frequency."]
    Divide206 = 0xcd,
    #[doc = "mclk frequency = 1/207 * hmclk frequency."]
    Divide207 = 0xce,
    #[doc = "mclk frequency = 1/208 * hmclk frequency."]
    Divide208 = 0xcf,
    #[doc = "mclk frequency = 1/209 * hmclk frequency."]
    Divide209 = 0xd0,
    #[doc = "mclk frequency = 1/210 * hmclk frequency."]
    Divide210 = 0xd1,
    #[doc = "mclk frequency = 1/211 * hmclk frequency."]
    Divide211 = 0xd2,
    #[doc = "mclk frequency = 1/212 * hmclk frequency."]
    Divide212 = 0xd3,
    #[doc = "mclk frequency = 1/213 * hmclk frequency."]
    Divide213 = 0xd4,
    #[doc = "mclk frequency = 1/214 * hmclk frequency."]
    Divide214 = 0xd5,
    #[doc = "mclk frequency = 1/215 * hmclk frequency."]
    Divide215 = 0xd6,
    #[doc = "mclk frequency = 1/216 * hmclk frequency."]
    Divide216 = 0xd7,
    #[doc = "mclk frequency = 1/217 * hmclk frequency."]
    Divide217 = 0xd8,
    #[doc = "mclk frequency = 1/218 * hmclk frequency."]
    Divide218 = 0xd9,
    #[doc = "mclk frequency = 1/219 * hmclk frequency."]
    Divide219 = 0xda,
    #[doc = "mclk frequency = 1/220 * hmclk frequency."]
    Divide220 = 0xdb,
    #[doc = "mclk frequency = 1/221 * hmclk frequency."]
    Divide221 = 0xdc,
    #[doc = "mclk frequency = 1/222 * hmclk frequency."]
    Divide222 = 0xdd,
    #[doc = "mclk frequency = 1/223 * hmclk frequency."]
    Divide223 = 0xde,
    #[doc = "mclk frequency = 1/224 * hmclk frequency."]
    Divide224 = 0xdf,
    #[doc = "mclk frequency = 1/225 * hmclk frequency."]
    Divide225 = 0xe0,
    #[doc = "mclk frequency = 1/226 * hmclk frequency."]
    Divide226 = 0xe1,
    #[doc = "mclk frequency = 1/227 * hmclk frequency."]
    Divide227 = 0xe2,
    #[doc = "mclk frequency = 1/228 * hmclk frequency."]
    Divide228 = 0xe3,
    #[doc = "mclk frequency = 1/229 * hmclk frequency."]
    Divide229 = 0xe4,
    #[doc = "mclk frequency = 1/230 * hmclk frequency."]
    Divide230 = 0xe5,
    #[doc = "mclk frequency = 1/231 * hmclk frequency."]
    Divide231 = 0xe6,
    #[doc = "mclk frequency = 1/232 * hmclk frequency."]
    Divide232 = 0xe7,
    #[doc = "mclk frequency = 1/233 * hmclk frequency."]
    Divide233 = 0xe8,
    #[doc = "mclk frequency = 1/234 * hmclk frequency."]
    Divide234 = 0xe9,
    #[doc = "mclk frequency = 1/235 * hmclk frequency."]
    Divide235 = 0xea,
    #[doc = "mclk frequency = 1/236 * hmclk frequency."]
    Divide236 = 0xeb,
    #[doc = "mclk frequency = 1/237 * hmclk frequency."]
    Divide237 = 0xec,
    #[doc = "mclk frequency = 1/238 * hmclk frequency."]
    Divide238 = 0xed,
    #[doc = "mclk frequency = 1/239 * hmclk frequency."]
    Divide239 = 0xee,
    #[doc = "mclk frequency = 1/240 * hmclk frequency."]
    Divide240 = 0xef,
    #[doc = "mclk frequency = 1/241 * hmclk frequency."]
    Divide241 = 0xf0,
    #[doc = "mclk frequency = 1/242 * hmclk frequency."]
    Divide242 = 0xf1,
    #[doc = "mclk frequency = 1/243 * hmclk frequency."]
    Divide243 = 0xf2,
    #[doc = "mclk frequency = 1/244 * hmclk frequency."]
    Divide244 = 0xf3,
    #[doc = "mclk frequency = 1/245 * hmclk frequency."]
    Divide245 = 0xf4,
    #[doc = "mclk frequency = 1/246 * hmclk frequency."]
    Divide246 = 0xf5,
    #[doc = "mclk frequency = 1/247 * hmclk frequency."]
    Divide247 = 0xf6,
    #[doc = "mclk frequency = 1/248 * hmclk frequency."]
    Divide248 = 0xf7,
    #[doc = "mclk frequency = 1/249 * hmclk frequency."]
    Divide249 = 0xf8,
    #[doc = "mclk frequency = 1/250 * hmclk frequency."]
    Divide250 = 0xf9,
    #[doc = "mclk frequency = 1/251 * hmclk frequency."]
    Divide251 = 0xfa,
    #[doc = "mclk frequency = 1/252 * hmclk frequency."]
    Divide252 = 0xfb,
    #[doc = "mclk frequency = 1/253 * hmclk frequency."]
    Divide253 = 0xfc,
    #[doc = "mclk frequency = 1/254 * hmclk frequency."]
    Divide254 = 0xfd,
    #[doc = "mclk frequency = 1/255 * hmclk frequency."]
    Divide255 = 0xfe,
    #[doc = "mclk frequency = 1/256 * hmclk frequency."]
    Divide256 = 0xff,
}
impl MqsClkDiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MqsClkDiv {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MqsClkDiv {
    #[inline(always)]
    fn from(val: u8) -> MqsClkDiv {
        MqsClkDiv::from_bits(val)
    }
}
impl From<MqsClkDiv> for u8 {
    #[inline(always)]
    fn from(val: MqsClkDiv) -> u8 {
        MqsClkDiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MqsEn {
    #[doc = "Disable MQS."]
    MqsEn0 = 0x0,
    #[doc = "Enable MQS."]
    MqsEn1 = 0x01,
}
impl MqsEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MqsEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MqsEn {
    #[inline(always)]
    fn from(val: u8) -> MqsEn {
        MqsEn::from_bits(val)
    }
}
impl From<MqsEn> for u8 {
    #[inline(always)]
    fn from(val: MqsEn) -> u8 {
        MqsEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MqsOversample {
    #[doc = "32."]
    MqsOversample0 = 0x0,
    #[doc = "64."]
    MqsOversample1 = 0x01,
}
impl MqsOversample {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MqsOversample {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MqsOversample {
    #[inline(always)]
    fn from(val: u8) -> MqsOversample {
        MqsOversample::from_bits(val)
    }
}
impl From<MqsOversample> for u8 {
    #[inline(always)]
    fn from(val: MqsOversample) -> u8 {
        MqsOversample::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MqsSwRst {
    #[doc = "Exit software reset for MQS."]
    MqsSwRst0 = 0x0,
    #[doc = "Enable software reset for MQS."]
    MqsSwRst1 = 0x01,
}
impl MqsSwRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MqsSwRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MqsSwRst {
    #[inline(always)]
    fn from(val: u8) -> MqsSwRst {
        MqsSwRst::from_bits(val)
    }
}
impl From<MqsSwRst> for u8 {
    #[inline(always)]
    fn from(val: MqsSwRst) -> u8 {
        MqsSwRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Niden {
    #[doc = "Debug turned off."]
    Niden0 = 0x0,
    #[doc = "Debug enabled (default)."]
    Niden1 = 0x01,
}
impl Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Niden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Niden {
    #[inline(always)]
    fn from(val: u8) -> Niden {
        Niden::from_bits(val)
    }
}
impl From<Niden> for u8 {
    #[inline(always)]
    fn from(val: Niden) -> u8 {
        Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ocram2TzEn {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM2 space is available for all access types (secure/non-secure/user/supervisor)."]
    Ocram2TzEn0 = 0x0,
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    Ocram2TzEn1 = 0x01,
}
impl Ocram2TzEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ocram2TzEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ocram2TzEn {
    #[inline(always)]
    fn from(val: u8) -> Ocram2TzEn {
        Ocram2TzEn::from_bits(val)
    }
}
impl From<Ocram2TzEn> for u8 {
    #[inline(always)]
    fn from(val: Ocram2TzEn) -> u8 {
        Ocram2TzEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OcramTzEn {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    OcramTzEn0 = 0x0,
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    OcramTzEn1 = 0x01,
}
impl OcramTzEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OcramTzEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OcramTzEn {
    #[inline(always)]
    fn from(val: u8) -> OcramTzEn {
        OcramTzEn::from_bits(val)
    }
}
impl From<OcramTzEn> for u8 {
    #[inline(always)]
    fn from(val: OcramTzEn) -> u8 {
        OcramTzEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PitStopAck {
    #[doc = "PIT stop acknowledge is not asserted."]
    PitStopAck0 = 0x0,
    #[doc = "PIT stop acknowledge is asserted."]
    PitStopAck1 = 0x01,
}
impl PitStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PitStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PitStopAck {
    #[inline(always)]
    fn from(val: u8) -> PitStopAck {
        PitStopAck::from_bits(val)
    }
}
impl From<PitStopAck> for u8 {
    #[inline(always)]
    fn from(val: PitStopAck) -> u8 {
        PitStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PitStopReq {
    #[doc = "stop request off."]
    PitStopReq0 = 0x0,
    #[doc = "stop request on."]
    PitStopReq1 = 0x01,
}
impl PitStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PitStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PitStopReq {
    #[inline(always)]
    fn from(val: u8) -> PitStopReq {
        PitStopReq::from_bits(val)
    }
}
impl From<PitStopReq> for u8 {
    #[inline(always)]
    fn from(val: PitStopReq) -> u8 {
        PitStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1TmrCntsFreeze {
    #[doc = "timer counter work normally."]
    Qtimer1TmrCntsFreeze0 = 0x0,
    #[doc = "reset counter and ouput flags."]
    Qtimer1TmrCntsFreeze1 = 0x01,
}
impl Qtimer1TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1TmrCntsFreeze {
        Qtimer1TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer1TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1TmrCntsFreeze) -> u8 {
        Qtimer1TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Trm0InputSel {
    #[doc = "input from IOMUX."]
    Qtimer1Trm0InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer1Trm0InputSel1 = 0x01,
}
impl Qtimer1Trm0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Trm0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Trm0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Trm0InputSel {
        Qtimer1Trm0InputSel::from_bits(val)
    }
}
impl From<Qtimer1Trm0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Trm0InputSel) -> u8 {
        Qtimer1Trm0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Trm1InputSel {
    #[doc = "input from IOMUX."]
    Qtimer1Trm1InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer1Trm1InputSel1 = 0x01,
}
impl Qtimer1Trm1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Trm1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Trm1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Trm1InputSel {
        Qtimer1Trm1InputSel::from_bits(val)
    }
}
impl From<Qtimer1Trm1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Trm1InputSel) -> u8 {
        Qtimer1Trm1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Trm2InputSel {
    #[doc = "input from IOMUX."]
    Qtimer1Trm2InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer1Trm2InputSel1 = 0x01,
}
impl Qtimer1Trm2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Trm2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Trm2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Trm2InputSel {
        Qtimer1Trm2InputSel::from_bits(val)
    }
}
impl From<Qtimer1Trm2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Trm2InputSel) -> u8 {
        Qtimer1Trm2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer1Trm3InputSel {
    #[doc = "input from IOMUX."]
    Qtimer1Trm3InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer1Trm3InputSel1 = 0x01,
}
impl Qtimer1Trm3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer1Trm3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer1Trm3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer1Trm3InputSel {
        Qtimer1Trm3InputSel::from_bits(val)
    }
}
impl From<Qtimer1Trm3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer1Trm3InputSel) -> u8 {
        Qtimer1Trm3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2TmrCntsFreeze {
    #[doc = "timer counter work normally."]
    Qtimer2TmrCntsFreeze0 = 0x0,
    #[doc = "reset counter and ouput flags."]
    Qtimer2TmrCntsFreeze1 = 0x01,
}
impl Qtimer2TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2TmrCntsFreeze {
        Qtimer2TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer2TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2TmrCntsFreeze) -> u8 {
        Qtimer2TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Trm0InputSel {
    #[doc = "input from IOMUX."]
    Qtimer2Trm0InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer2Trm0InputSel1 = 0x01,
}
impl Qtimer2Trm0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Trm0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Trm0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Trm0InputSel {
        Qtimer2Trm0InputSel::from_bits(val)
    }
}
impl From<Qtimer2Trm0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Trm0InputSel) -> u8 {
        Qtimer2Trm0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Trm1InputSel {
    #[doc = "input from IOMUX."]
    Qtimer2Trm1InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer2Trm1InputSel1 = 0x01,
}
impl Qtimer2Trm1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Trm1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Trm1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Trm1InputSel {
        Qtimer2Trm1InputSel::from_bits(val)
    }
}
impl From<Qtimer2Trm1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Trm1InputSel) -> u8 {
        Qtimer2Trm1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Trm2InputSel {
    #[doc = "input from IOMUX."]
    Qtimer2Trm2InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer2Trm2InputSel1 = 0x01,
}
impl Qtimer2Trm2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Trm2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Trm2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Trm2InputSel {
        Qtimer2Trm2InputSel::from_bits(val)
    }
}
impl From<Qtimer2Trm2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Trm2InputSel) -> u8 {
        Qtimer2Trm2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer2Trm3InputSel {
    #[doc = "input from IOMUX."]
    Qtimer2Trm3InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer2Trm3InputSel1 = 0x01,
}
impl Qtimer2Trm3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer2Trm3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer2Trm3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer2Trm3InputSel {
        Qtimer2Trm3InputSel::from_bits(val)
    }
}
impl From<Qtimer2Trm3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer2Trm3InputSel) -> u8 {
        Qtimer2Trm3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3TmrCntsFreeze {
    #[doc = "timer counter work normally."]
    Qtimer3TmrCntsFreeze0 = 0x0,
    #[doc = "reset counter and ouput flags."]
    Qtimer3TmrCntsFreeze1 = 0x01,
}
impl Qtimer3TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3TmrCntsFreeze {
        Qtimer3TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer3TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3TmrCntsFreeze) -> u8 {
        Qtimer3TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Trm0InputSel {
    #[doc = "input from IOMUX."]
    Qtimer3Trm0InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer3Trm0InputSel1 = 0x01,
}
impl Qtimer3Trm0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Trm0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Trm0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Trm0InputSel {
        Qtimer3Trm0InputSel::from_bits(val)
    }
}
impl From<Qtimer3Trm0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Trm0InputSel) -> u8 {
        Qtimer3Trm0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Trm1InputSel {
    #[doc = "input from IOMUX."]
    Qtimer3Trm1InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer3Trm1InputSel1 = 0x01,
}
impl Qtimer3Trm1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Trm1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Trm1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Trm1InputSel {
        Qtimer3Trm1InputSel::from_bits(val)
    }
}
impl From<Qtimer3Trm1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Trm1InputSel) -> u8 {
        Qtimer3Trm1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Trm2InputSel {
    #[doc = "input from IOMUX."]
    Qtimer3Trm2InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer3Trm2InputSel1 = 0x01,
}
impl Qtimer3Trm2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Trm2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Trm2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Trm2InputSel {
        Qtimer3Trm2InputSel::from_bits(val)
    }
}
impl From<Qtimer3Trm2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Trm2InputSel) -> u8 {
        Qtimer3Trm2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer3Trm3InputSel {
    #[doc = "input from IOMUX."]
    Qtimer3Trm3InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer3Trm3InputSel1 = 0x01,
}
impl Qtimer3Trm3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer3Trm3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer3Trm3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer3Trm3InputSel {
        Qtimer3Trm3InputSel::from_bits(val)
    }
}
impl From<Qtimer3Trm3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer3Trm3InputSel) -> u8 {
        Qtimer3Trm3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4TmrCntsFreeze {
    #[doc = "timer counter work normally."]
    Qtimer4TmrCntsFreeze0 = 0x0,
    #[doc = "reset counter and ouput flags."]
    Qtimer4TmrCntsFreeze1 = 0x01,
}
impl Qtimer4TmrCntsFreeze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4TmrCntsFreeze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4TmrCntsFreeze {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4TmrCntsFreeze {
        Qtimer4TmrCntsFreeze::from_bits(val)
    }
}
impl From<Qtimer4TmrCntsFreeze> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4TmrCntsFreeze) -> u8 {
        Qtimer4TmrCntsFreeze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Trm0InputSel {
    #[doc = "input from IOMUX."]
    Qtimer4Trm0InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer4Trm0InputSel1 = 0x01,
}
impl Qtimer4Trm0InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Trm0InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Trm0InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Trm0InputSel {
        Qtimer4Trm0InputSel::from_bits(val)
    }
}
impl From<Qtimer4Trm0InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Trm0InputSel) -> u8 {
        Qtimer4Trm0InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Trm1InputSel {
    #[doc = "input from IOMUX."]
    Qtimer4Trm1InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer4Trm1InputSel1 = 0x01,
}
impl Qtimer4Trm1InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Trm1InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Trm1InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Trm1InputSel {
        Qtimer4Trm1InputSel::from_bits(val)
    }
}
impl From<Qtimer4Trm1InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Trm1InputSel) -> u8 {
        Qtimer4Trm1InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Trm2InputSel {
    #[doc = "input from IOMUX."]
    Qtimer4Trm2InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer4Trm2InputSel1 = 0x01,
}
impl Qtimer4Trm2InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Trm2InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Trm2InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Trm2InputSel {
        Qtimer4Trm2InputSel::from_bits(val)
    }
}
impl From<Qtimer4Trm2InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Trm2InputSel) -> u8 {
        Qtimer4Trm2InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qtimer4Trm3InputSel {
    #[doc = "input from IOMUX."]
    Qtimer4Trm3InputSel0 = 0x0,
    #[doc = "input from XBAR."]
    Qtimer4Trm3InputSel1 = 0x01,
}
impl Qtimer4Trm3InputSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qtimer4Trm3InputSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qtimer4Trm3InputSel {
    #[inline(always)]
    fn from(val: u8) -> Qtimer4Trm3InputSel {
        Qtimer4Trm3InputSel::from_bits(val)
    }
}
impl From<Qtimer4Trm3InputSel> for u8 {
    #[inline(always)]
    fn from(val: Qtimer4Trm3InputSel) -> u8 {
        Qtimer4Trm3InputSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamAutoClkGatingEn {
    #[doc = "disable automatically gate off RAM clock."]
    RamAutoClkGatingEn0 = 0x0,
    #[doc = "enable automatically gate off RAM clock."]
    RamAutoClkGatingEn1 = 0x01,
}
impl RamAutoClkGatingEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamAutoClkGatingEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamAutoClkGatingEn {
    #[inline(always)]
    fn from(val: u8) -> RamAutoClkGatingEn {
        RamAutoClkGatingEn::from_bits(val)
    }
}
impl From<RamAutoClkGatingEn> for u8 {
    #[inline(always)]
    fn from(val: RamAutoClkGatingEn) -> u8 {
        RamAutoClkGatingEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk1Sel {
    #[doc = "ccm.ssi1_clk_root."]
    Sai1Mclk1Sel0 = 0x0,
    #[doc = "ccm.ssi2_clk_root."]
    Sai1Mclk1Sel1 = 0x01,
    #[doc = "ccm.ssi3_clk_root."]
    Sai1Mclk1Sel2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk."]
    Sai1Mclk1Sel3 = 0x03,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk."]
    Sai1Mclk1Sel4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk."]
    Sai1Mclk1Sel5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1Mclk1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk1Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk1Sel {
        Sai1Mclk1Sel::from_bits(val)
    }
}
impl From<Sai1Mclk1Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk1Sel) -> u8 {
        Sai1Mclk1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk2Sel {
    #[doc = "ccm.ssi1_clk_root."]
    Sai1Mclk2Sel0 = 0x0,
    #[doc = "ccm.ssi2_clk_root."]
    Sai1Mclk2Sel1 = 0x01,
    #[doc = "ccm.ssi3_clk_root."]
    Sai1Mclk2Sel2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk."]
    Sai1Mclk2Sel3 = 0x03,
    #[doc = "iomux.sai2_ipg_clk_sai_mclk."]
    Sai1Mclk2Sel4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk."]
    Sai1Mclk2Sel5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Sai1Mclk2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk2Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk2Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk2Sel {
        Sai1Mclk2Sel::from_bits(val)
    }
}
impl From<Sai1Mclk2Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk2Sel) -> u8 {
        Sai1Mclk2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root."]
    Sai1Mclk3Sel0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2."]
    Sai1Mclk3Sel1 = 0x01,
    #[doc = "spdif.spdif_srclk."]
    Sai1Mclk3Sel2 = 0x02,
    #[doc = "spdif.spdif_outclock."]
    Sai1Mclk3Sel3 = 0x03,
}
impl Sai1Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai1Mclk3Sel {
        Sai1Mclk3Sel::from_bits(val)
    }
}
impl From<Sai1Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai1Mclk3Sel) -> u8 {
        Sai1Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1MclkDir {
    #[doc = "sai1.MCLK is input signal."]
    Sai1MclkDir0 = 0x0,
    #[doc = "sai1.MCLK is output signal."]
    Sai1MclkDir1 = 0x01,
}
impl Sai1MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai1MclkDir {
        Sai1MclkDir::from_bits(val)
    }
}
impl From<Sai1MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai1MclkDir) -> u8 {
        Sai1MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1StopAck {
    #[doc = "SAI1 stop acknowledge is not asserted."]
    Sai1StopAck0 = 0x0,
    #[doc = "SAI1 stop acknowledge is asserted."]
    Sai1StopAck1 = 0x01,
}
impl Sai1StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1StopAck {
    #[inline(always)]
    fn from(val: u8) -> Sai1StopAck {
        Sai1StopAck::from_bits(val)
    }
}
impl From<Sai1StopAck> for u8 {
    #[inline(always)]
    fn from(val: Sai1StopAck) -> u8 {
        Sai1StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1StopReq {
    #[doc = "stop request off."]
    Sai1StopReq0 = 0x0,
    #[doc = "stop request on."]
    Sai1StopReq1 = 0x01,
}
impl Sai1StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1StopReq {
    #[inline(always)]
    fn from(val: u8) -> Sai1StopReq {
        Sai1StopReq::from_bits(val)
    }
}
impl From<Sai1StopReq> for u8 {
    #[inline(always)]
    fn from(val: Sai1StopReq) -> u8 {
        Sai1StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root."]
    Sai2Mclk3Sel0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2."]
    Sai2Mclk3Sel1 = 0x01,
    #[doc = "spdif.spdif_srclk."]
    Sai2Mclk3Sel2 = 0x02,
    #[doc = "spdif.spdif_outclock."]
    Sai2Mclk3Sel3 = 0x03,
}
impl Sai2Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai2Mclk3Sel {
        Sai2Mclk3Sel::from_bits(val)
    }
}
impl From<Sai2Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai2Mclk3Sel) -> u8 {
        Sai2Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2MclkDir {
    #[doc = "sai2.MCLK is input signal."]
    Sai2MclkDir0 = 0x0,
    #[doc = "sai2.MCLK is output signal."]
    Sai2MclkDir1 = 0x01,
}
impl Sai2MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai2MclkDir {
        Sai2MclkDir::from_bits(val)
    }
}
impl From<Sai2MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai2MclkDir) -> u8 {
        Sai2MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2StopAck {
    #[doc = "SAI2 stop acknowledge is not asserted."]
    Sai2StopAck0 = 0x0,
    #[doc = "SAI2 stop acknowledge is asserted."]
    Sai2StopAck1 = 0x01,
}
impl Sai2StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2StopAck {
    #[inline(always)]
    fn from(val: u8) -> Sai2StopAck {
        Sai2StopAck::from_bits(val)
    }
}
impl From<Sai2StopAck> for u8 {
    #[inline(always)]
    fn from(val: Sai2StopAck) -> u8 {
        Sai2StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai2StopReq {
    #[doc = "stop request off."]
    Sai2StopReq0 = 0x0,
    #[doc = "stop request on."]
    Sai2StopReq1 = 0x01,
}
impl Sai2StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai2StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai2StopReq {
    #[inline(always)]
    fn from(val: u8) -> Sai2StopReq {
        Sai2StopReq::from_bits(val)
    }
}
impl From<Sai2StopReq> for u8 {
    #[inline(always)]
    fn from(val: Sai2StopReq) -> u8 {
        Sai2StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root."]
    Sai3Mclk3Sel0 = 0x0,
    #[doc = "iomux.spdif_tx_clk2."]
    Sai3Mclk3Sel1 = 0x01,
    #[doc = "spdif.spdif_srclk."]
    Sai3Mclk3Sel2 = 0x02,
    #[doc = "spdif.spdif_outclock."]
    Sai3Mclk3Sel3 = 0x03,
}
impl Sai3Mclk3Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3Mclk3Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3Mclk3Sel {
    #[inline(always)]
    fn from(val: u8) -> Sai3Mclk3Sel {
        Sai3Mclk3Sel::from_bits(val)
    }
}
impl From<Sai3Mclk3Sel> for u8 {
    #[inline(always)]
    fn from(val: Sai3Mclk3Sel) -> u8 {
        Sai3Mclk3Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3MclkDir {
    #[doc = "sai3.MCLK is input signal."]
    Sai3MclkDir0 = 0x0,
    #[doc = "sai3.MCLK is output signal."]
    Sai3MclkDir1 = 0x01,
}
impl Sai3MclkDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3MclkDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3MclkDir {
    #[inline(always)]
    fn from(val: u8) -> Sai3MclkDir {
        Sai3MclkDir::from_bits(val)
    }
}
impl From<Sai3MclkDir> for u8 {
    #[inline(always)]
    fn from(val: Sai3MclkDir) -> u8 {
        Sai3MclkDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3StopAck {
    #[doc = "SAI3 stop acknowledge is not asserted."]
    Sai3StopAck0 = 0x0,
    #[doc = "SAI3 stop acknowledge is asserted."]
    Sai3StopAck1 = 0x01,
}
impl Sai3StopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3StopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3StopAck {
    #[inline(always)]
    fn from(val: u8) -> Sai3StopAck {
        Sai3StopAck::from_bits(val)
    }
}
impl From<Sai3StopAck> for u8 {
    #[inline(always)]
    fn from(val: Sai3StopAck) -> u8 {
        Sai3StopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3StopReq {
    #[doc = "stop request off."]
    Sai3StopReq0 = 0x0,
    #[doc = "stop request on."]
    Sai3StopReq1 = 0x01,
}
impl Sai3StopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3StopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3StopReq {
    #[inline(always)]
    fn from(val: u8) -> Sai3StopReq {
        Sai3StopReq::from_bits(val)
    }
}
impl From<Sai3StopReq> for u8 {
    #[inline(always)]
    fn from(val: Sai3StopReq) -> u8 {
        Sai3StopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecErrResp {
    #[doc = "OKEY response."]
    SecErrResp0 = 0x0,
    #[doc = "SLVError (default)."]
    SecErrResp1 = 0x01,
}
impl SecErrResp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecErrResp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecErrResp {
    #[inline(always)]
    fn from(val: u8) -> SecErrResp {
        SecErrResp::from_bits(val)
    }
}
impl From<SecErrResp> for u8 {
    #[inline(always)]
    fn from(val: SecErrResp) -> u8 {
        SecErrResp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcStopAck {
    #[doc = "SEMC stop acknowledge is not asserted."]
    SemcStopAck0 = 0x0,
    #[doc = "SEMC stop acknowledge is asserted."]
    SemcStopAck1 = 0x01,
}
impl SemcStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcStopAck {
    #[inline(always)]
    fn from(val: u8) -> SemcStopAck {
        SemcStopAck::from_bits(val)
    }
}
impl From<SemcStopAck> for u8 {
    #[inline(always)]
    fn from(val: SemcStopAck) -> u8 {
        SemcStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SemcStopReq {
    #[doc = "stop request off."]
    SemcStopReq0 = 0x0,
    #[doc = "stop request on."]
    SemcStopReq1 = 0x01,
}
impl SemcStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SemcStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SemcStopReq {
    #[inline(always)]
    fn from(val: u8) -> SemcStopReq {
        SemcStopReq::from_bits(val)
    }
}
impl From<SemcStopReq> for u8 {
    #[inline(always)]
    fn from(val: SemcStopReq) -> u8 {
        SemcStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SipTestMuxQspiSipEn {
    #[doc = "SIP_TEST_MUX is disabled."]
    SipTestMuxQspiSipEn0 = 0x0,
    #[doc = "SIP_TEST_MUX is enabled."]
    SipTestMuxQspiSipEn1 = 0x01,
}
impl SipTestMuxQspiSipEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SipTestMuxQspiSipEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SipTestMuxQspiSipEn {
    #[inline(always)]
    fn from(val: u8) -> SipTestMuxQspiSipEn {
        SipTestMuxQspiSipEn::from_bits(val)
    }
}
impl From<SipTestMuxQspiSipEn> for u8 {
    #[inline(always)]
    fn from(val: SipTestMuxQspiSipEn) -> u8 {
        SipTestMuxQspiSipEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngStopAck {
    #[doc = "TRNG stop acknowledge is not asserted."]
    TrngStopAck0 = 0x0,
    #[doc = "TRNG stop acknowledge is asserted."]
    TrngStopAck1 = 0x01,
}
impl TrngStopAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngStopAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngStopAck {
    #[inline(always)]
    fn from(val: u8) -> TrngStopAck {
        TrngStopAck::from_bits(val)
    }
}
impl From<TrngStopAck> for u8 {
    #[inline(always)]
    fn from(val: TrngStopAck) -> u8 {
        TrngStopAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngStopReq {
    #[doc = "stop request off."]
    TrngStopReq0 = 0x0,
    #[doc = "stop request on."]
    TrngStopReq1 = 0x01,
}
impl TrngStopReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngStopReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngStopReq {
    #[inline(always)]
    fn from(val: u8) -> TrngStopReq {
        TrngStopReq::from_bits(val)
    }
}
impl From<TrngStopReq> for u8 {
    #[inline(always)]
    fn from(val: TrngStopReq) -> u8 {
        TrngStopReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref1mClkGpt1 {
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK."]
    Vref1mClkGpt10 = 0x0,
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock."]
    Vref1mClkGpt11 = 0x01,
}
impl Vref1mClkGpt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref1mClkGpt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref1mClkGpt1 {
    #[inline(always)]
    fn from(val: u8) -> Vref1mClkGpt1 {
        Vref1mClkGpt1::from_bits(val)
    }
}
impl From<Vref1mClkGpt1> for u8 {
    #[inline(always)]
    fn from(val: Vref1mClkGpt1) -> u8 {
        Vref1mClkGpt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vref1mClkGpt2 {
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK."]
    Vref1mClkGpt20 = 0x0,
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock."]
    Vref1mClkGpt21 = 0x01,
}
impl Vref1mClkGpt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vref1mClkGpt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vref1mClkGpt2 {
    #[inline(always)]
    fn from(val: u8) -> Vref1mClkGpt2 {
        Vref1mClkGpt2::from_bits(val)
    }
}
impl From<Vref1mClkGpt2> for u8 {
    #[inline(always)]
    fn from(val: Vref1mClkGpt2) -> u8 {
        Vref1mClkGpt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog1Mask {
    #[doc = "WDOG1 Timeout behaves normally."]
    Wdog1Mask0 = 0x0,
    #[doc = "WDOG1 Timeout is masked."]
    Wdog1Mask1 = 0x01,
}
impl Wdog1Mask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog1Mask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog1Mask {
    #[inline(always)]
    fn from(val: u8) -> Wdog1Mask {
        Wdog1Mask::from_bits(val)
    }
}
impl From<Wdog1Mask> for u8 {
    #[inline(always)]
    fn from(val: Wdog1Mask) -> u8 {
        Wdog1Mask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog2Mask {
    #[doc = "WDOG2 Timeout behaves normally."]
    Wdog2Mask0 = 0x0,
    #[doc = "WDOG2 Timeout is masked."]
    Wdog2Mask1 = 0x01,
}
impl Wdog2Mask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog2Mask {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog2Mask {
    #[inline(always)]
    fn from(val: u8) -> Wdog2Mask {
        Wdog2Mask::from_bits(val)
    }
}
impl From<Wdog2Mask> for u8 {
    #[inline(always)]
    fn from(val: Wdog2Mask) -> u8 {
        Wdog2Mask::to_bits(val)
    }
}
