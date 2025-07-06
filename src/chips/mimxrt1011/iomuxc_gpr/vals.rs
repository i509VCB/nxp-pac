#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AxbsPForceRoundRobin {
    #[doc = "AXBS_P masters are not arbitored in round robin, depending on M0/M1 master priority settings."]
    AXBS_P_FORCE_ROUND_ROBIN_0 = 0x0,
    #[doc = "AXBS_P masters are arbitored in round robin"]
    AXBS_P_FORCE_ROUND_ROBIN_1 = 0x01,
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
    #[doc = "AXBS_P M0 master doesn't have high priority"]
    AXBS_P_M0_HIGH_PRIORITY_0 = 0x0,
    #[doc = "AXBS_P M0 master has high priority"]
    AXBS_P_M0_HIGH_PRIORITY_1 = 0x01,
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
    #[doc = "AXBS_P M1 master does not have high priority"]
    AXBS_P_M1_HIGH_PRIORITY_0 = 0x0,
    #[doc = "AXBS_P M1 master has high priority"]
    AXBS_P_M1_HIGH_PRIORITY_1 = 0x01,
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
pub enum CacheUsb {
    #[doc = "Cacheable attribute is off for read/write transactions."]
    CACHE_USB_0 = 0x0,
    #[doc = "Cacheable attribute is on for read/write transactions."]
    CACHE_USB_1 = 0x01,
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
pub enum Cm7ForceHclkEn {
    #[doc = "AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible."]
    CM7_FORCE_HCLK_EN_0 = 0x0,
    #[doc = "AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible."]
    CM7_FORCE_HCLK_EN_1 = 0x01,
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
    DBG_EN_0 = 0x0,
    #[doc = "Debug enabled (default)."]
    DBG_EN_1 = 0x01,
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
    #[doc = "Select \\[127:0\\] from SNVS Master Key as DCP key"]
    DCP_KEY_SEL_0 = 0x0,
    #[doc = "Select \\[255:128\\] from SNVS Master Key as DCP key"]
    DCP_KEY_SEL_1 = 0x01,
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
    DCPKEY_OCOTP_OR_KEYMUX_0 = 0x0,
    #[doc = "Select key from OCOTP (SW_GP2)."]
    DCPKEY_OCOTP_OR_KEYMUX_1 = 0x01,
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
    #[doc = "EDMA stop acknowledge is not asserted"]
    EDMA_STOP_ACK_0 = 0x0,
    #[doc = "EDMA stop acknowledge is asserted (EDMA is in STOP mode)."]
    EDMA_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    EDMA_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    EDMA_STOP_REQ_1 = 0x01,
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
pub enum ExcMon {
    #[doc = "OKAY response"]
    EXC_MON_0 = 0x0,
    #[doc = "SLVError response"]
    EXC_MON_1 = 0x01,
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
    #[doc = "FLEXIO1 is not in doze mode"]
    FLEXIO1_IPG_DOZE_0 = 0x0,
    #[doc = "FLEXIO1 is in doze mode"]
    FLEXIO1_IPG_DOZE_1 = 0x01,
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
    FLEXIO1_IPG_STOP_MODE_0 = 0x0,
    #[doc = "When this bit is equal to 1'b1 and ipg_stop is asserted, FlexIO1 is not functional in Stop mode."]
    FLEXIO1_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "FLEXIO1 stop acknowledge is not asserted"]
    FLEXIO1_STOP_ACK_0 = 0x0,
    #[doc = "FLEXIO1 stop acknowledge is asserted"]
    FLEXIO1_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    FLEXIO1_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    FLEXIO1_STOP_REQ_1 = 0x01,
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
pub enum FlexramBankCfgSel {
    #[doc = "use fuse value to config"]
    FLEXRAM_BANK_CFG_SEL_0 = 0x0,
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    FLEXRAM_BANK_CFG_SEL_1 = 0x01,
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
pub enum FlexspiStopAck {
    #[doc = "FLEXSPI stop acknowledge is not asserted"]
    FLEXSPI_STOP_ACK_0 = 0x0,
    #[doc = "FLEXSPI stop acknowledge is asserted"]
    FLEXSPI_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    FLEXSPI_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    FLEXSPI_STOP_REQ_1 = 0x01,
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
    GINT_0 = 0x0,
    #[doc = "Global interrupt request is asserted."]
    GINT_1 = 0x01,
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
pub enum IomuxcXbarDirSel2 {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_2_0 = 0x0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_2_1 = 0x01,
}
impl IomuxcXbarDirSel2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel2 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel2 {
        IomuxcXbarDirSel2::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel2> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel2) -> u8 {
        IomuxcXbarDirSel2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IomuxcXbarDirSel3 {
    #[doc = "XBAR_INOUT as input"]
    IOMUXC_XBAR_DIR_SEL_3_0 = 0x0,
    #[doc = "XBAR_INOUT as output"]
    IOMUXC_XBAR_DIR_SEL_3_1 = 0x01,
}
impl IomuxcXbarDirSel3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IomuxcXbarDirSel3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IomuxcXbarDirSel3 {
    #[inline(always)]
    fn from(val: u8) -> IomuxcXbarDirSel3 {
        IomuxcXbarDirSel3::from_bits(val)
    }
}
impl From<IomuxcXbarDirSel3> for u8 {
    #[inline(always)]
    fn from(val: IomuxcXbarDirSel3) -> u8 {
        IomuxcXbarDirSel3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L2MemDeepsleep {
    #[doc = "No force sleep control supported, memory deep sleep mode only entered when whole system in stop mode (OCRAM in normal mode)"]
    L2_MEM_DEEPSLEEP_0 = 0x0,
    #[doc = "Force memory into deep sleep mode (OCRAM in power saving mode)"]
    L2_MEM_DEEPSLEEP_1 = 0x01,
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
    #[doc = "Enters power saving mode only when chip is in SUSPEND mode"]
    L2_MEM_EN_POWERSAVING_0 = 0x0,
    #[doc = "Controlled by L2_MEM_DEEPSLEEP bitfield"]
    L2_MEM_EN_POWERSAVING_1 = 0x01,
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
    #[doc = "Field is not locked"]
    LOCK_DBG_EN_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DBG_EN_1 = 0x01,
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
    #[doc = "Field is not locked"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_DCPKEY_OCOTP_OR_KEYMUX_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R0_BOT_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R0_BOT_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R0_TOP_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R0_TOP_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R1_BOT_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R1_BOT_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R1_TOP_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R1_TOP_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R2_BOT_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R2_BOT_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R2_TOP_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R2_TOP_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R3_BOT_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R3_BOT_1 = 0x01,
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
    #[doc = "Register field \\[31:1\\] is not locked"]
    LOCK_M7_APC_AC_R3_TOP_0 = 0x0,
    #[doc = "Register field \\[31:1\\] is locked (read access only)"]
    LOCK_M7_APC_AC_R3_TOP_1 = 0x01,
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
    #[doc = "Field is not locked"]
    LOCK_NIDEN_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_NIDEN_1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockOcramTzAddr {
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_ADDR_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_ADDR_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl LockOcramTzAddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockOcramTzAddr {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
    #[doc = "Field is not locked"]
    LOCK_OCRAM_TZ_EN_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_OCRAM_TZ_EN_1 = 0x01,
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
    #[doc = "Field is not locked"]
    LOCK_SEC_ERR_RESP_0 = 0x0,
    #[doc = "Field is locked (read access only)"]
    LOCK_SEC_ERR_RESP_1 = 0x01,
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
pub enum LockVtor {
    #[doc = "CM7_INIT_VTOR field is not locked."]
    LOCK_VTOR_0 = 0x0,
    #[doc = "CM7_INIT_VTOR field is locked (read access only)."]
    LOCK_VTOR_1 = 0x01,
}
impl LockVtor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockVtor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockVtor {
    #[inline(always)]
    fn from(val: u8) -> LockVtor {
        LockVtor::from_bits(val)
    }
}
impl From<LockVtor> for u8 {
    #[inline(always)]
    fn from(val: LockVtor) -> u8 {
        LockVtor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2c1IpgDoze {
    #[doc = "not in doze mode"]
    LPI2C1_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPI2C1_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPI2C1_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C1_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPI2C1_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted (the module is in Stop mode)"]
    LPI2C1_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPI2C1_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPI2C1_STOP_REQ_1 = 0x01,
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
    #[doc = "not in doze mode"]
    LPI2C2_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPI2C2_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPI2C2_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPI2C2_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPI2C2_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPI2C2_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPI2C2_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPI2C2_STOP_REQ_1 = 0x01,
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
pub enum Lpspi1IpgDoze {
    #[doc = "not in doze mode"]
    LPSPI1_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPSPI1_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPSPI1_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI1_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPSPI1_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI1_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPSPI1_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPSPI1_STOP_REQ_1 = 0x01,
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
    #[doc = "not in doze mode"]
    LPSPI2_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPSPI2_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPSPI2_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPSPI2_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPSPI2_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPSPI2_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPSPI2_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPSPI2_STOP_REQ_1 = 0x01,
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
pub enum Lpuart1IpgDoze {
    #[doc = "not in doze mode"]
    LPUART1_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPUART1_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPUART1_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART1_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPUART1_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPUART1_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPUART1_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPUART1_STOP_REQ_1 = 0x01,
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
    #[doc = "not in doze mode"]
    LPUART2_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPUART2_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPUART2_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART2_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPUART2_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPUART2_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPUART2_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPUART2_STOP_REQ_1 = 0x01,
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
    #[doc = "not in doze mode"]
    LPUART3_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPUART3_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPUART3_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART3_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPUART3_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPUART3_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPUART3_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPUART3_STOP_REQ_1 = 0x01,
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
    #[doc = "not in doze mode"]
    LPUART4_IPG_DOZE_0 = 0x0,
    #[doc = "in doze mode"]
    LPUART4_IPG_DOZE_1 = 0x01,
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
    #[doc = "the module is functional in Stop mode"]
    LPUART4_IPG_STOP_MODE_0 = 0x0,
    #[doc = "the module is NOT functional in Stop mode, when this bit is equal to 1 and ipg_stop is asserted"]
    LPUART4_IPG_STOP_MODE_1 = 0x01,
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
    #[doc = "stop acknowledge is not asserted"]
    LPUART4_STOP_ACK_0 = 0x0,
    #[doc = "stop acknowledge is asserted"]
    LPUART4_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    LPUART4_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    LPUART4_STOP_REQ_1 = 0x01,
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
pub enum M7ApcAcR0Ctrl {
    #[doc = "No access protection"]
    M7_APC_AC_R0_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R0_CTRL_1 = 0x01,
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
    #[doc = "No access protection"]
    M7_APC_AC_R1_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R1_CTRL_1 = 0x01,
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
    #[doc = "No access protection"]
    M7_APC_AC_R2_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R2_CTRL_1 = 0x01,
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
    #[doc = "No access protection"]
    M7_APC_AC_R3_CTRL_0 = 0x0,
    #[doc = "M7 debug protection enabled"]
    M7_APC_AC_R3_CTRL_1 = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MqsClkDiv(u8);
impl MqsClkDiv {
    #[doc = "mclk frequency = hmclk frequency"]
    pub const MQS_CLK_DIV_0: Self = Self(0x0);
    #[doc = "mclk frequency = 1/2 * hmclk frequency"]
    pub const MQS_CLK_DIV_1: Self = Self(0x01);
    #[doc = "mclk frequency = 1/3 * hmclk frequency"]
    pub const MQS_CLK_DIV_2: Self = Self(0x02);
    #[doc = "mclk frequency = 1/256 * hmclk frequency"]
    pub const MQS_CLK_DIV_255: Self = Self(0xff);
}
impl MqsClkDiv {
    pub const fn from_bits(val: u8) -> MqsClkDiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for MqsClkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MQS_CLK_DIV_0"),
            0x01 => f.write_str("MQS_CLK_DIV_1"),
            0x02 => f.write_str("MQS_CLK_DIV_2"),
            0xff => f.write_str("MQS_CLK_DIV_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MqsClkDiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MQS_CLK_DIV_0"),
            0x01 => defmt::write!(f, "MQS_CLK_DIV_1"),
            0x02 => defmt::write!(f, "MQS_CLK_DIV_2"),
            0xff => defmt::write!(f, "MQS_CLK_DIV_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
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
    #[doc = "Disable MQS"]
    MQS_EN_0 = 0x0,
    #[doc = "Enable MQS"]
    MQS_EN_1 = 0x01,
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
    #[doc = "32"]
    MQS_OVERSAMPLE_0 = 0x0,
    #[doc = "64"]
    MQS_OVERSAMPLE_1 = 0x01,
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
    #[doc = "Exit software reset for MQS"]
    MQS_SW_RST_0 = 0x0,
    #[doc = "Enable software reset for MQS"]
    MQS_SW_RST_1 = 0x01,
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
    NIDEN_0 = 0x0,
    #[doc = "Debug enabled (default)."]
    NIDEN_1 = 0x01,
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
pub enum OcramTzEn {
    #[doc = "The TrustZone feature is disabled. Entire OCRAM space is available for all access types (secure/non-secure/user/supervisor)."]
    OCRAM_TZ_EN_0 = 0x0,
    #[doc = "The TrustZone feature is enabled. Access to address in the range specified by \\[ENDADDR:STARTADDR\\] follows the execution mode access policy described in CSU chapter."]
    OCRAM_TZ_EN_1 = 0x01,
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
    #[doc = "PIT stop acknowledge is not asserted"]
    PIT_STOP_ACK_0 = 0x0,
    #[doc = "PIT stop acknowledge is asserted"]
    PIT_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    PIT_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    PIT_STOP_REQ_1 = 0x01,
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
pub enum RamAutoClkGatingEn {
    #[doc = "disable automatically gate off RAM clock"]
    RAM_AUTO_CLK_GATING_EN_0 = 0x0,
    #[doc = "enable automatically gate off RAM clock"]
    RAM_AUTO_CLK_GATING_EN_1 = 0x01,
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
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_5 = 0x05,
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
    #[doc = "ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 0x02,
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_5 = 0x05,
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
    #[doc = "ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0x0,
    #[doc = "SPDIF_EXT_CLK"]
    SAI1_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3 = 0x03,
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
    #[doc = "sai1.MCLK is input signal"]
    SAI1_MCLK_DIR_0 = 0x0,
    #[doc = "sai1.MCLK is output signal"]
    SAI1_MCLK_DIR_1 = 0x01,
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
    #[doc = "SAI1 stop acknowledge is not asserted"]
    SAI1_STOP_ACK_0 = 0x0,
    #[doc = "SAI1 stop acknowledge is asserted"]
    SAI1_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    SAI1_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    SAI1_STOP_REQ_1 = 0x01,
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
pub enum Sai3Mclk3Sel {
    #[doc = "ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0x0,
    #[doc = "SPDIF_EXT_CLK"]
    SAI3_MCLK3_SEL_1 = 0x01,
    #[doc = "spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2 = 0x02,
    #[doc = "spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3 = 0x03,
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
    #[doc = "sai3.MCLK is input signal"]
    SAI3_MCLK_DIR_0 = 0x0,
    #[doc = "sai3.MCLK is output signal"]
    SAI3_MCLK_DIR_1 = 0x01,
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
    #[doc = "SAI3 stop acknowledge is not asserted"]
    SAI3_STOP_ACK_0 = 0x0,
    #[doc = "SAI3 stop acknowledge is asserted"]
    SAI3_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    SAI3_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    SAI3_STOP_REQ_1 = 0x01,
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
    #[doc = "OKEY response"]
    SEC_ERR_RESP_0 = 0x0,
    #[doc = "SLVError (default)"]
    SEC_ERR_RESP_1 = 0x01,
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
pub enum TrngStopAck {
    #[doc = "TRNG stop acknowledge is not asserted"]
    TRNG_STOP_ACK_0 = 0x0,
    #[doc = "TRNG stop acknowledge is asserted"]
    TRNG_STOP_ACK_1 = 0x01,
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
    #[doc = "stop request off"]
    TRNG_STOP_REQ_0 = 0x0,
    #[doc = "stop request on"]
    TRNG_STOP_REQ_1 = 0x01,
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
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT1_0 = 0x0,
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT1_1 = 0x01,
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
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK"]
    VREF_1M_CLK_GPT2_0 = 0x0,
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock"]
    VREF_1M_CLK_GPT2_1 = 0x01,
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
    #[doc = "WDOG1 Timeout behaves normally"]
    WDOG1_MASK_0 = 0x0,
    #[doc = "WDOG1 Timeout is masked"]
    WDOG1_MASK_1 = 0x01,
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
    #[doc = "WDOG2 Timeout behaves normally"]
    WDOG2_MASK_0 = 0x0,
    #[doc = "WDOG2 Timeout is masked"]
    WDOG2_MASK_1 = 0x01,
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
