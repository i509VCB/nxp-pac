#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapTrim {
    #[doc = "Default (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 00 )"]
    VAL0 = 0x0,
    #[doc = "-1us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 01)"]
    VAL1 = 0x01,
    #[doc = "-2us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 10) or or +3.5us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 10)"]
    VAL2 = 0x02,
    #[doc = "-2.5us (when CAP2_TRIM = 0 and CAP_TRIM\\[1:0\\] = 11) or +1us (when CAP2_TRIM = 1 and CAP_TRIM\\[1:0\\] = 11)"]
    VAL3 = 0x03,
}
impl CapTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapTrim {
    #[inline(always)]
    fn from(val: u8) -> CapTrim {
        CapTrim::from_bits(val)
    }
}
impl From<CapTrim> for u8 {
    #[inline(always)]
    fn from(val: CapTrim) -> u8 {
        CapTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrim {
    #[doc = "760 mV"]
    CMP_TRIM_0 = 0x0,
    #[doc = "770 mV"]
    CMP_TRIM_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "740 mV"]
    CMP_TRIM_3 = 0x03,
}
impl CmpTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrim {
    #[inline(always)]
    fn from(val: u8) -> CmpTrim {
        CmpTrim::from_bits(val)
    }
}
impl From<CmpTrim> for u8 {
    #[inline(always)]
    fn from(val: CmpTrim) -> u8 {
        CmpTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoarseAmpGain {
    #[doc = "ESR Range 0"]
    GAIN05 = 0x0,
    #[doc = "ESR Range 1"]
    GAIN10 = 0x01,
    #[doc = "ESR Range 2"]
    GAIN18 = 0x02,
    #[doc = "ESR Range 3"]
    GAIN33 = 0x03,
}
impl CoarseAmpGain {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoarseAmpGain {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoarseAmpGain {
    #[inline(always)]
    fn from(val: u8) -> CoarseAmpGain {
        CoarseAmpGain::from_bits(val)
    }
}
impl From<CoarseAmpGain> for u8 {
    #[inline(always)]
    fn from(val: CoarseAmpGain) -> u8 {
        CoarseAmpGain::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivideTrim {
    #[doc = "Clock monitor operates at 1 kHz"]
    CFG0 = 0x0,
    #[doc = "Clock monitor operates at 64 Hz"]
    CFG1 = 0x01,
}
impl DivideTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivideTrim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivideTrim {
    #[inline(always)]
    fn from(val: u8) -> DivideTrim {
        DivideTrim::from_bits(val)
    }
}
impl From<DivideTrim> for u8 {
    #[inline(always)]
    fn from(val: DivideTrim) -> u8 {
        DivideTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DlyTrim {
    #[doc = "P current 9(nA) and N Current 6(nA)"]
    DLY_TRIM_0 = 0x0,
    #[doc = "P current 13(nA) and N Current 6(nA)"]
    DLY_TRIM_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "P current 4(nA) and N Current 6(nA)"]
    DLY_TRIM_3 = 0x03,
    #[doc = "P current 9(nA) and N Current 4(nA)"]
    DLY_TRIM_4 = 0x04,
    #[doc = "P current 13(nA) and N Current 4(nA)"]
    DLY_TRIM_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "P current 4(nA) and N Current 4(nA)"]
    DLY_TRIM_7 = 0x07,
    #[doc = "P current 9(nA) and N Current 2(nA)"]
    DLY_TRIM_8 = 0x08,
    #[doc = "P current 13(nA) and N Current 2(nA)"]
    DLY_TRIM_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "P current 4(nA) and N Current 2(nA)"]
    DLY_TRIM_11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DlyTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DlyTrim {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DlyTrim {
    #[inline(always)]
    fn from(val: u8) -> DlyTrim {
        DlyTrim::from_bits(val)
    }
}
impl From<DlyTrim> for u8 {
    #[inline(always)]
    fn from(val: DlyTrim) -> u8 {
        DlyTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtalCapSel {
    #[doc = "0 pF"]
    SEL0 = 0x0,
    #[doc = "2 pF"]
    SEL2 = 0x01,
    #[doc = "4 pF"]
    SEL4 = 0x02,
    #[doc = "6 pF"]
    SEL6 = 0x03,
    #[doc = "8 pF"]
    SEL8 = 0x04,
    #[doc = "10 pF"]
    SEL10 = 0x05,
    #[doc = "12 pF"]
    SEL12 = 0x06,
    #[doc = "14 pF"]
    SEL14 = 0x07,
    #[doc = "16 pF"]
    SEL16 = 0x08,
    #[doc = "18 pF"]
    SEL18 = 0x09,
    #[doc = "20 pF"]
    SEL20 = 0x0a,
    #[doc = "22 pF"]
    SEL22 = 0x0b,
    #[doc = "24 pF"]
    SEL24 = 0x0c,
    #[doc = "26 pF"]
    SEL26 = 0x0d,
    #[doc = "28 pF"]
    SEL28 = 0x0e,
    #[doc = "30 pF"]
    SEL30 = 0x0f,
}
impl ExtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> ExtalCapSel {
        ExtalCapSel::from_bits(val)
    }
}
impl From<ExtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: ExtalCapSel) -> u8 {
        ExtalCapSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqTrim {
    #[doc = "Clock monitor asserts 2 cycle after expected edge"]
    CFG0 = 0x0,
    #[doc = "Clock monitor asserts 4 cycles after expected edge"]
    CFG1 = 0x01,
    #[doc = "Clock monitor asserts 6 cycles after expected edge"]
    CFG2 = 0x02,
    #[doc = "Clock monitor asserts 8 cycles after expected edge"]
    CFG3 = 0x03,
}
impl FreqTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqTrim {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqTrim {
    #[inline(always)]
    fn from(val: u8) -> FreqTrim {
        FreqTrim::from_bits(val)
    }
}
impl From<FreqTrim> for u8 {
    #[inline(always)]
    fn from(val: FreqTrim) -> u8 {
        FreqTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolckbLock {
    #[doc = "Block"]
    ENABLE = 0x0,
    #[doc = "Do not block"]
    DISABLE = 0x01,
}
impl FrolckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolckbLock {
    #[inline(always)]
    fn from(val: u8) -> FrolckbLock {
        FrolckbLock::from_bits(val)
    }
}
impl From<FrolckbLock> for u8 {
    #[inline(always)]
    fn from(val: FrolckbLock) -> u8 {
        FrolckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitTrim {
    #[doc = "8 s"]
    SEL0 = 0x0,
    #[doc = "4 s"]
    SEL1 = 0x01,
    #[doc = "2 s"]
    SEL2 = 0x02,
    #[doc = "1 s"]
    SEL3 = 0x03,
    #[doc = "0.5 s"]
    SEL4 = 0x04,
    #[doc = "0.25 s"]
    SEL5 = 0x05,
    #[doc = "0.125 s"]
    SEL6 = 0x06,
    #[doc = "0.5 ms"]
    SEL7 = 0x07,
}
impl InitTrim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitTrim {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitTrim {
    #[inline(always)]
    fn from(val: u8) -> InitTrim {
        InitTrim::from_bits(val)
    }
}
impl From<InitTrim> for u8 {
    #[inline(always)]
    fn from(val: InitTrim) -> u8 {
        InitTrim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqenaConfigDet {
    #[doc = "Disable"]
    CLR = 0x0,
    #[doc = "Enable"]
    SET = 0x01,
}
impl IrqenaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqenaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqenaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> IrqenaConfigDet {
        IrqenaConfigDet::from_bits(val)
    }
}
impl From<IrqenaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: IrqenaConfigDet) -> u8 {
        IrqenaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IrqenaWakeupFlag {
    #[doc = "Disable"]
    CLR = 0x0,
    #[doc = "Enable"]
    SET = 0x01,
}
impl IrqenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IrqenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IrqenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> IrqenaWakeupFlag {
        IrqenaWakeupFlag::from_bits(val)
    }
}
impl From<IrqenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: IrqenaWakeupFlag) -> u8 {
        IrqenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LdolckbLock {
    #[doc = "Block"]
    ENABLE = 0x0,
    #[doc = "Do not block"]
    DISABLE = 0x01,
}
impl LdolckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LdolckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LdolckbLock {
    #[inline(always)]
    fn from(val: u8) -> LdolckbLock {
        LdolckbLock::from_bits(val)
    }
}
impl From<LdolckbLock> for u8 {
    #[inline(always)]
    fn from(val: LdolckbLock) -> u8 {
        LdolckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockbLock {
    #[doc = "Enables lock"]
    ENABLE = 0x0,
    #[doc = "Disables lock"]
    DISABLE = 0x01,
}
impl LockbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockbLock {
    #[inline(always)]
    fn from(val: u8) -> LockbLock {
        LockbLock::from_bits(val)
    }
}
impl From<LockbLock> for u8 {
    #[inline(always)]
    fn from(val: LockbLock) -> u8 {
        LockbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModeEn {
    #[doc = "Normal mode"]
    HP = 0x0,
    #[doc = "Startup mode"]
    LP = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Low power mode"]
    SW = 0x03,
}
impl ModeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModeEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModeEn {
    #[inline(always)]
    fn from(val: u8) -> ModeEn {
        ModeEn::from_bits(val)
    }
}
impl From<ModeEn> for u8 {
    #[inline(always)]
    fn from(val: ModeEn) -> u8 {
        ModeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MonlckbLock {
    #[doc = "Lock is enabled"]
    ENABLE = 0x0,
    #[doc = "Lock is disabled"]
    DISABLE = 0x01,
}
impl MonlckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MonlckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MonlckbLock {
    #[inline(always)]
    fn from(val: u8) -> MonlckbLock {
        MonlckbLock::from_bits(val)
    }
}
impl From<MonlckbLock> for u8 {
    #[inline(always)]
    fn from(val: MonlckbLock) -> u8 {
        MonlckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OsclckbLock {
    #[doc = "Block"]
    ENABLE = 0x0,
    #[doc = "Do not block"]
    DISABLE = 0x01,
}
impl OsclckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OsclckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OsclckbLock {
    #[inline(always)]
    fn from(val: u8) -> OsclckbLock {
        OsclckbLock::from_bits(val)
    }
}
impl From<OsclckbLock> for u8 {
    #[inline(always)]
    fn from(val: OsclckbLock) -> u8 {
        OsclckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Out {
    #[doc = "Logic zero (asserted)"]
    ON = 0x0,
    #[doc = "Logic one"]
    OFF = 0x01,
}
impl Out {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Out {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Out {
    #[inline(always)]
    fn from(val: u8) -> Out {
        Out::from_bits(val)
    }
}
impl From<Out> for u8 {
    #[inline(always)]
    fn from(val: Out) -> u8 {
        Out::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaClockDet {
    #[doc = "Clock error not detected"]
    CLR = 0x0,
    #[doc = "Clock error detected"]
    SET = 0x01,
}
impl StatusaClockDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaClockDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaClockDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaClockDet {
        StatusaClockDet::from_bits(val)
    }
}
impl From<StatusaClockDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaClockDet) -> u8 {
        StatusaClockDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaConfigDet {
    #[doc = "Not detected"]
    CLR = 0x0,
    #[doc = "Detected"]
    SET = 0x01,
}
impl StatusaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaConfigDet {
        StatusaConfigDet::from_bits(val)
    }
}
impl From<StatusaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaConfigDet) -> u8 {
        StatusaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaIrq0Det {
    #[doc = "Not asserted"]
    CLR = 0x0,
    #[doc = "Asserted"]
    SET = 0x01,
}
impl StatusaIrq0Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaIrq0Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaIrq0Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaIrq0Det {
        StatusaIrq0Det::from_bits(val)
    }
}
impl From<StatusaIrq0Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaIrq0Det) -> u8 {
        StatusaIrq0Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaIrq1Det {
    #[doc = "Not asserted"]
    CLR = 0x0,
    #[doc = "Asserted"]
    SET = 0x01,
}
impl StatusaIrq1Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaIrq1Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaIrq1Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaIrq1Det {
        StatusaIrq1Det::from_bits(val)
    }
}
impl From<StatusaIrq1Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaIrq1Det) -> u8 {
        StatusaIrq1Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaIrq2Det {
    #[doc = "Not asserted"]
    CLR = 0x0,
    #[doc = "Asserted"]
    SET = 0x01,
}
impl StatusaIrq2Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaIrq2Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaIrq2Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaIrq2Det {
        StatusaIrq2Det::from_bits(val)
    }
}
impl From<StatusaIrq2Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaIrq2Det) -> u8 {
        StatusaIrq2Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaIrq3Det {
    #[doc = "Not asserted"]
    CLR = 0x0,
    #[doc = "Asserted"]
    SET = 0x01,
}
impl StatusaIrq3Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaIrq3Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaIrq3Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaIrq3Det {
        StatusaIrq3Det::from_bits(val)
    }
}
impl From<StatusaIrq3Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaIrq3Det) -> u8 {
        StatusaIrq3Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaLdoRdy {
    #[doc = "Disabled (not ready)"]
    CLR = 0x0,
    #[doc = "Enabled (ready)"]
    SET = 0x01,
}
impl StatusaLdoRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaLdoRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaLdoRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaLdoRdy {
        StatusaLdoRdy::from_bits(val)
    }
}
impl From<StatusaLdoRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaLdoRdy) -> u8 {
        StatusaLdoRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaLightDet {
    #[doc = "Light error not detected"]
    CLR = 0x0,
    #[doc = "Light error detected"]
    SET = 0x01,
}
impl StatusaLightDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaLightDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaLightDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaLightDet {
        StatusaLightDet::from_bits(val)
    }
}
impl From<StatusaLightDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaLightDet) -> u8 {
        StatusaLightDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaOscRdy {
    #[doc = "Disabled (clock not ready)"]
    CLR = 0x0,
    #[doc = "Enabled (clock ready)"]
    SET = 0x01,
}
impl StatusaOscRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaOscRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaOscRdy {
    #[inline(always)]
    fn from(val: u8) -> StatusaOscRdy {
        StatusaOscRdy::from_bits(val)
    }
}
impl From<StatusaOscRdy> for u8 {
    #[inline(always)]
    fn from(val: StatusaOscRdy) -> u8 {
        StatusaOscRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaPorDet {
    #[doc = "Not reset"]
    CLR = 0x0,
    #[doc = "Reset"]
    SET = 0x01,
}
impl StatusaPorDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaPorDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaPorDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaPorDet {
        StatusaPorDet::from_bits(val)
    }
}
impl From<StatusaPorDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaPorDet) -> u8 {
        StatusaPorDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaSec0Det {
    #[doc = "Security input 0 not detected"]
    CLR = 0x0,
    #[doc = "Security input 0 detected"]
    SET = 0x01,
}
impl StatusaSec0Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaSec0Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaSec0Det {
    #[inline(always)]
    fn from(val: u8) -> StatusaSec0Det {
        StatusaSec0Det::from_bits(val)
    }
}
impl From<StatusaSec0Det> for u8 {
    #[inline(always)]
    fn from(val: StatusaSec0Det) -> u8 {
        StatusaSec0Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTempDet {
    #[doc = "Temperature error not detected"]
    CLR = 0x0,
    #[doc = "Temperature error detected"]
    SET = 0x01,
}
impl StatusaTempDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTempDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTempDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaTempDet {
        StatusaTempDet::from_bits(val)
    }
}
impl From<StatusaTempDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaTempDet) -> u8 {
        StatusaTempDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer0Flag {
    #[doc = "Not reached"]
    CLR = 0x0,
    #[doc = "Reached"]
    SET = 0x01,
}
impl StatusaTimer0Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer0Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer0Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer0Flag {
        StatusaTimer0Flag::from_bits(val)
    }
}
impl From<StatusaTimer0Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer0Flag) -> u8 {
        StatusaTimer0Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaTimer1Flag {
    #[doc = "Not reached"]
    CLR = 0x0,
    #[doc = "Reached"]
    SET = 0x01,
}
impl StatusaTimer1Flag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaTimer1Flag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaTimer1Flag {
    #[inline(always)]
    fn from(val: u8) -> StatusaTimer1Flag {
        StatusaTimer1Flag::from_bits(val)
    }
}
impl From<StatusaTimer1Flag> for u8 {
    #[inline(always)]
    fn from(val: StatusaTimer1Flag) -> u8 {
        StatusaTimer1Flag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaVoltDet {
    #[doc = "Not detected"]
    CLR = 0x0,
    #[doc = "Detected"]
    SET = 0x01,
}
impl StatusaVoltDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaVoltDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaVoltDet {
    #[inline(always)]
    fn from(val: u8) -> StatusaVoltDet {
        StatusaVoltDet::from_bits(val)
    }
}
impl From<StatusaVoltDet> for u8 {
    #[inline(always)]
    fn from(val: StatusaVoltDet) -> u8 {
        StatusaVoltDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatusaWakeupFlag {
    #[doc = "Not asserted"]
    CLR = 0x0,
    #[doc = "Asserted"]
    SET = 0x01,
}
impl StatusaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> StatusaWakeupFlag {
        StatusaWakeupFlag::from_bits(val)
    }
}
impl From<StatusaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: StatusaWakeupFlag) -> u8 {
        StatusaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SupplyDet {
    #[doc = "VBAT supply is less than 3V"]
    SUPPLY_DET_0 = 0x0,
    #[doc = "VBAT supply is greater than 3V"]
    SUPPLY_DET_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SupplyDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SupplyDet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SupplyDet {
    #[inline(always)]
    fn from(val: u8) -> SupplyDet {
        SupplyDet::from_bits(val)
    }
}
impl From<SupplyDet> for u8 {
    #[inline(always)]
    fn from(val: SupplyDet) -> u8 {
        SupplyDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwilckbLock {
    #[doc = "Block"]
    ENABLE = 0x0,
    #[doc = "Do not block"]
    DISABLE = 0x01,
}
impl SwilckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwilckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwilckbLock {
    #[inline(always)]
    fn from(val: u8) -> SwilckbLock {
        SwilckbLock::from_bits(val)
    }
}
impl From<SwilckbLock> for u8 {
    #[inline(always)]
    fn from(val: SwilckbLock) -> u8 {
        SwilckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TamlckbLock {
    #[doc = "Lock is enabled"]
    ENABLE = 0x0,
    #[doc = "Lock is disabled"]
    DISABLE = 0x01,
}
impl TamlckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TamlckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TamlckbLock {
    #[inline(always)]
    fn from(val: u8) -> TamlckbLock {
        TamlckbLock::from_bits(val)
    }
}
impl From<TamlckbLock> for u8 {
    #[inline(always)]
    fn from(val: TamlckbLock) -> u8 {
        TamlckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TamperaConfigDet {
    #[doc = "Tamper disabled"]
    CLR = 0x0,
    #[doc = "Tamper enabled"]
    SET = 0x01,
}
impl TamperaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TamperaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TamperaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> TamperaConfigDet {
        TamperaConfigDet::from_bits(val)
    }
}
impl From<TamperaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: TamperaConfigDet) -> u8 {
        TamperaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timcfg {
    #[doc = "1 s"]
    CFG1000 = 0x0,
    #[doc = "500 ms"]
    CFG500 = 0x01,
    #[doc = "250 ms"]
    CFG250 = 0x02,
    #[doc = "125 ms"]
    CFG125 = 0x03,
    #[doc = "62.5 ms"]
    CFG62 = 0x04,
    #[doc = "31.25 ms"]
    CFG31 = 0x05,
    #[doc = "15.625 ms"]
    CFG15 = 0x06,
    #[doc = "7.8125 ms"]
    CFG7 = 0x07,
}
impl Timcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timcfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timcfg {
    #[inline(always)]
    fn from(val: u8) -> Timcfg {
        Timcfg::from_bits(val)
    }
}
impl From<Timcfg> for u8 {
    #[inline(always)]
    fn from(val: Timcfg) -> u8 {
        Timcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakenaConfigDet {
    #[doc = "Disable"]
    CLR = 0x0,
    #[doc = "Enable"]
    SET = 0x01,
}
impl WakenaConfigDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakenaConfigDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakenaConfigDet {
    #[inline(always)]
    fn from(val: u8) -> WakenaConfigDet {
        WakenaConfigDet::from_bits(val)
    }
}
impl From<WakenaConfigDet> for u8 {
    #[inline(always)]
    fn from(val: WakenaConfigDet) -> u8 {
        WakenaConfigDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WakenaWakeupFlag {
    #[doc = "Disable"]
    CLR = 0x0,
    #[doc = "Enable"]
    SET = 0x01,
}
impl WakenaWakeupFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WakenaWakeupFlag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WakenaWakeupFlag {
    #[inline(always)]
    fn from(val: u8) -> WakenaWakeupFlag {
        WakenaWakeupFlag::from_bits(val)
    }
}
impl From<WakenaWakeupFlag> for u8 {
    #[inline(always)]
    fn from(val: WakenaWakeupFlag) -> u8 {
        WakenaWakeupFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WaklckbLock {
    #[doc = "Lock is enabled"]
    ENABLE = 0x0,
    #[doc = "Lock is disabled"]
    DISABLE = 0x01,
}
impl WaklckbLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WaklckbLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WaklckbLock {
    #[inline(always)]
    fn from(val: u8) -> WaklckbLock {
        WaklckbLock::from_bits(val)
    }
}
impl From<WaklckbLock> for u8 {
    #[inline(always)]
    fn from(val: WaklckbLock) -> u8 {
        WaklckbLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XtalCapSel {
    #[doc = "0 pF"]
    SEL0 = 0x0,
    #[doc = "2 pF"]
    SEL2 = 0x01,
    #[doc = "4 pF"]
    SEL4 = 0x02,
    #[doc = "6 pF"]
    SEL6 = 0x03,
    #[doc = "8 pF"]
    SEL8 = 0x04,
    #[doc = "10 pF"]
    SEL10 = 0x05,
    #[doc = "12 pF"]
    SEL12 = 0x06,
    #[doc = "14 pF"]
    SEL14 = 0x07,
    #[doc = "16 pF"]
    SEL16 = 0x08,
    #[doc = "18 pF"]
    SEL18 = 0x09,
    #[doc = "20 pF"]
    SEL20 = 0x0a,
    #[doc = "22 pF"]
    SEL22 = 0x0b,
    #[doc = "24 pF"]
    SEL24 = 0x0c,
    #[doc = "26 pF"]
    SEL26 = 0x0d,
    #[doc = "28 pF"]
    SEL28 = 0x0e,
    #[doc = "30 pF"]
    SEL30 = 0x0f,
}
impl XtalCapSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XtalCapSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XtalCapSel {
    #[inline(always)]
    fn from(val: u8) -> XtalCapSel {
        XtalCapSel::from_bits(val)
    }
}
impl From<XtalCapSel> for u8 {
    #[inline(always)]
    fn from(val: XtalCapSel) -> u8 {
        XtalCapSel::to_bits(val)
    }
}
