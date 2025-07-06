#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Core0DbgRst {
    #[doc = "do not assert core0 debug reset"]
    CORE0_DBG_RST_0 = 0x0,
    #[doc = "assert core0 debug reset"]
    CORE0_DBG_RST_1 = 0x01,
}
impl Core0DbgRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Core0DbgRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Core0DbgRst {
    #[inline(always)]
    fn from(val: u8) -> Core0DbgRst {
        Core0DbgRst::from_bits(val)
    }
}
impl From<Core0DbgRst> for u8 {
    #[inline(always)]
    fn from(val: Core0DbgRst) -> u8 {
        Core0DbgRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Core0Rst {
    #[doc = "do not assert core0 reset"]
    CORE0_RST_0 = 0x0,
    #[doc = "assert core0 reset"]
    CORE0_RST_1 = 0x01,
}
impl Core0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Core0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Core0Rst {
    #[inline(always)]
    fn from(val: u8) -> Core0Rst {
        Core0Rst::from_bits(val)
    }
}
impl From<Core0Rst> for u8 {
    #[inline(always)]
    fn from(val: Core0Rst) -> u8 {
        Core0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsuResetB {
    #[doc = "Reset is not a result of the csu_reset_b event."]
    CSU_RESET_B_0 = 0x0,
    #[doc = "Reset is a result of the csu_reset_b event."]
    CSU_RESET_B_1 = 0x01,
}
impl CsuResetB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsuResetB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsuResetB {
    #[inline(always)]
    fn from(val: u8) -> CsuResetB {
        CsuResetB::from_bits(val)
    }
}
impl From<CsuResetB> for u8 {
    #[inline(always)]
    fn from(val: CsuResetB) -> u8 {
        CsuResetB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgRstMskPg {
    #[doc = "do not mask core debug resets (debug resets will be asserted after power gating event)"]
    DBG_RST_MSK_PG_0 = 0x0,
    #[doc = "mask core debug resets (debug resets won't be asserted after power gating event)"]
    DBG_RST_MSK_PG_1 = 0x01,
}
impl DbgRstMskPg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgRstMskPg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgRstMskPg {
    #[inline(always)]
    fn from(val: u8) -> DbgRstMskPg {
        DbgRstMskPg::from_bits(val)
    }
}
impl From<DbgRstMskPg> for u8 {
    #[inline(always)]
    fn from(val: DbgRstMskPg) -> u8 {
        DbgRstMskPg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IppResetB {
    #[doc = "Reset is not a result of ipp_reset_b pin."]
    IPP_RESET_B_0 = 0x0,
    #[doc = "Reset is a result of ipp_reset_b pin."]
    IPP_RESET_B_1 = 0x01,
}
impl IppResetB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IppResetB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IppResetB {
    #[inline(always)]
    fn from(val: u8) -> IppResetB {
        IppResetB::from_bits(val)
    }
}
impl From<IppResetB> for u8 {
    #[inline(always)]
    fn from(val: IppResetB) -> u8 {
        IppResetB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IppUserResetB {
    #[doc = "Reset is not a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_0 = 0x0,
    #[doc = "Reset is a result of the ipp_user_reset_b qualified as COLD reset event."]
    IPP_USER_RESET_B_1 = 0x01,
}
impl IppUserResetB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IppUserResetB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IppUserResetB {
    #[inline(always)]
    fn from(val: u8) -> IppUserResetB {
        IppUserResetB::from_bits(val)
    }
}
impl From<IppUserResetB> for u8 {
    #[inline(always)]
    fn from(val: IppUserResetB) -> u8 {
        IppUserResetB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JtagRstB {
    #[doc = "Reset is not a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_0 = 0x0,
    #[doc = "Reset is a result of HIGH-Z reset from JTAG."]
    JTAG_RST_B_1 = 0x01,
}
impl JtagRstB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> JtagRstB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for JtagRstB {
    #[inline(always)]
    fn from(val: u8) -> JtagRstB {
        JtagRstB::from_bits(val)
    }
}
impl From<JtagRstB> for u8 {
    #[inline(always)]
    fn from(val: JtagRstB) -> u8 {
        JtagRstB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum JtagSwRst {
    #[doc = "Reset is not a result of the mentioned case."]
    JTAG_SW_RST_0 = 0x0,
    #[doc = "Reset is a result of the mentioned case."]
    JTAG_SW_RST_1 = 0x01,
}
impl JtagSwRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> JtagSwRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for JtagSwRst {
    #[inline(always)]
    fn from(val: u8) -> JtagSwRst {
        JtagSwRst::from_bits(val)
    }
}
impl From<JtagSwRst> for u8 {
    #[inline(always)]
    fn from(val: JtagSwRst) -> u8 {
        JtagSwRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lockup {
    #[doc = "Reset is not a result of the mentioned case."]
    LOCKUP_0 = 0x0,
    #[doc = "Reset is a result of the mentioned case."]
    LOCKUP_1 = 0x01,
}
impl Lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lockup {
    #[inline(always)]
    fn from(val: u8) -> Lockup {
        Lockup::from_bits(val)
    }
}
impl From<Lockup> for u8 {
    #[inline(always)]
    fn from(val: Lockup) -> u8 {
        Lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockupRst {
    #[doc = "disabled"]
    LOCKUP_RST_0 = 0x0,
    #[doc = "enabled"]
    LOCKUP_RST_1 = 0x01,
}
impl LockupRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockupRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockupRst {
    #[inline(always)]
    fn from(val: u8) -> LockupRst {
        LockupRst::from_bits(val)
    }
}
impl From<LockupRst> for u8 {
    #[inline(always)]
    fn from(val: LockupRst) -> u8 {
        LockupRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskWdog3Rst {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "wdog3_rst_b is masked"]
    MASK_WDOG3_RST_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "wdog3_rst_b is not masked"]
    MASK_WDOG3_RST_10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl MaskWdog3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskWdog3Rst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskWdog3Rst {
    #[inline(always)]
    fn from(val: u8) -> MaskWdog3Rst {
        MaskWdog3Rst::from_bits(val)
    }
}
impl From<MaskWdog3Rst> for u8 {
    #[inline(always)]
    fn from(val: MaskWdog3Rst) -> u8 {
        MaskWdog3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskWdogRst {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "wdog_rst_b is masked"]
    MASK_WDOG_RST_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "wdog_rst_b is not masked (default)"]
    MASK_WDOG_RST_10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl MaskWdogRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskWdogRst {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskWdogRst {
    #[inline(always)]
    fn from(val: u8) -> MaskWdogRst {
        MaskWdogRst::from_bits(val)
    }
}
impl From<MaskWdogRst> for u8 {
    #[inline(always)]
    fn from(val: MaskWdogRst) -> u8 {
        MaskWdogRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TempsenseRstB {
    #[doc = "Reset is not a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_0 = 0x0,
    #[doc = "Reset is a result of software reset from Temperature Sensor."]
    TEMPSENSE_RST_B_1 = 0x01,
}
impl TempsenseRstB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TempsenseRstB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TempsenseRstB {
    #[inline(always)]
    fn from(val: u8) -> TempsenseRstB {
        TempsenseRstB::from_bits(val)
    }
}
impl From<TempsenseRstB> for u8 {
    #[inline(always)]
    fn from(val: TempsenseRstB) -> u8 {
        TempsenseRstB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdog3RstB {
    #[doc = "Reset is not a result of the watchdog3 time-out event."]
    WDOG3_RST_B_0 = 0x0,
    #[doc = "Reset is a result of the watchdog3 time-out event."]
    WDOG3_RST_B_1 = 0x01,
}
impl Wdog3RstB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdog3RstB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdog3RstB {
    #[inline(always)]
    fn from(val: u8) -> Wdog3RstB {
        Wdog3RstB::from_bits(val)
    }
}
impl From<Wdog3RstB> for u8 {
    #[inline(always)]
    fn from(val: Wdog3RstB) -> u8 {
        Wdog3RstB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WdogRstB {
    #[doc = "Reset is not a result of the watchdog time-out event."]
    WDOG_RST_B_0 = 0x0,
    #[doc = "Reset is a result of the watchdog time-out event."]
    WDOG_RST_B_1 = 0x01,
}
impl WdogRstB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WdogRstB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WdogRstB {
    #[inline(always)]
    fn from(val: u8) -> WdogRstB {
        WdogRstB::from_bits(val)
    }
}
impl From<WdogRstB> for u8 {
    #[inline(always)]
    fn from(val: WdogRstB) -> u8 {
        WdogRstB::to_bits(val)
    }
}
