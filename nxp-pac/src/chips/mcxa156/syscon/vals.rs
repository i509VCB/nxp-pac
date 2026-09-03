#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCpu0Sbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCpu0Sbus {
        AhbmatprioCpu0Sbus::from_bits(val)
    }
}
impl From<AhbmatprioCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCpu0Sbus) -> u8 {
        AhbmatprioCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache."]
    Enable = 0x0,
    #[doc = "Clears the cache."]
    Disable = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0Cbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl Cpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> Cpu0Cbus {
        Cpu0Cbus::from_bits(val)
    }
}
impl From<Cpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: Cpu0Cbus) -> u8 {
        Cpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup."]
    Awake = 0x0,
    #[doc = "CPU is in lockup."]
    Sleeping = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping."]
    Awake = 0x0,
    #[doc = "CPU is sleeping."]
    Sleeping = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation."]
    Enable = 0x0,
    #[doc = "Disables data speculation."]
    Disable = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation."]
    Enable = 0x0,
    #[doc = "Disables flash speculation."]
    Disable = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data."]
    Disable = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction."]
    Disable = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashSize {
    #[doc = "32KB."]
    Size32kb = 0x0,
    #[doc = "64KB."]
    Size64kb = 0x01,
    #[doc = "128KB."]
    Size128kb = 0x02,
    #[doc = "256KB."]
    Size256kb = 0x03,
    #[doc = "512KB."]
    Size512kb = 0x04,
    #[doc = "768KB."]
    Size768kb = 0x05,
    #[doc = "1MB."]
    Size1mb = 0x06,
    #[doc = "1.5MB."]
    Size1p5mb = 0x07,
    #[doc = "2MB."]
    Size2mb = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FlashSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashSize {
    #[inline(always)]
    fn from(val: u8) -> FlashSize {
        FlashSize::from_bits(val)
    }
}
impl From<FlashSize> for u8 {
    #[inline(always)]
    fn from(val: FlashSize) -> u8 {
        FlashSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy."]
    Enable = 0x0,
    #[doc = "Stall on FLASH busy."]
    Disable = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation."]
    Enable = 0x0,
    #[doc = "Forces no allocation."]
    Disable = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers."]
    Disable = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers."]
    Enable = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Noref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noref {
    #[inline(always)]
    fn from(val: u8) -> Noref {
        Noref::from_bits(val)
    }
}
impl From<Noref> for u8 {
    #[inline(always)]
    fn from(val: Noref) -> u8 {
        Noref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamSize {
    #[doc = "8KB."]
    Size8kb = 0x0,
    #[doc = "16KB."]
    Size16kb = 0x01,
    #[doc = "32KB."]
    Size32kb = 0x02,
    #[doc = "64KB."]
    Size64kb = 0x03,
    #[doc = "96KB."]
    Size96kb = 0x04,
    #[doc = "128KB."]
    Size128kb = 0x05,
    #[doc = "160KB."]
    Size160kb = 0x06,
    #[doc = "192KB."]
    Size192kb = 0x07,
    #[doc = "256KB."]
    Size256kb = 0x08,
    #[doc = "288KB."]
    Size288kb = 0x09,
    #[doc = "352KB."]
    Size352kb = 0x0a,
    #[doc = "512KB."]
    Size512kb = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RamSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamSize {
    #[inline(always)]
    fn from(val: u8) -> RamSize {
        RamSize::from_bits(val)
    }
}
impl From<RamSize> for u8 {
    #[inline(always)]
    fn from(val: RamSize) -> u8 {
        RamSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCpu0Sbus {
    #[doc = "RAMX0: 0x04000000 - 0x04001fff."]
    Cpu0Sbus0 = 0x0,
    #[doc = "RAMX0: 0x2001e000 - 0x2001ffff(for 128KB RAM chip) / 0x20016000 - 0x20017fff(for 96KB RAM chip) / 0x2000e000 - 0x2000ffff(for 64KB RAM chip)."]
    Cpu0Sbus1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> RemapCpu0Sbus {
        RemapCpu0Sbus::from_bits(val)
    }
}
impl From<RemapCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: RemapCpu0Sbus) -> u8 {
        RemapCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma0 {
    #[doc = "RAMX0: 0x04000000 - 0x04001fff."]
    Dma00 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    Dma01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma0 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma0 {
        RemapDma0::from_bits(val)
    }
}
impl From<RemapDma0> for u8 {
    #[inline(always)]
    fn from(val: RemapDma0) -> u8 {
        RemapDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapLock {
    #[doc = "This register is not locked and can be altered."]
    Lock0 = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    Lock1 = 0x01,
}
impl RemapLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapLock {
    #[inline(always)]
    fn from(val: u8) -> RemapLock {
        RemapLock::from_bits(val)
    }
}
impl From<RemapLock> for u8 {
    #[inline(always)]
    fn from(val: RemapLock) -> u8 {
        RemapLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reset {
    #[inline(always)]
    fn from(val: u8) -> Reset {
        Reset::from_bits(val)
    }
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(val: Reset) -> u8 {
        Reset::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecCode(u32);
impl SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SecCode {
    pub const fn from_bits(val: u32) -> SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SecCode {
    #[inline(always)]
    fn from(val: u32) -> SecCode {
        SecCode::from_bits(val)
    }
}
impl From<SecCode> for u32 {
    #[inline(always)]
    fn from(val: SecCode) -> u32 {
        SecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Security {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Secure version."]
    NonSec = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Non secure version."]
    Security10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Security {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Security {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Security {
    #[inline(always)]
    fn from(val: u8) -> Security {
        Security::from_bits(val)
    }
}
impl From<Security> for u8 {
    #[inline(always)]
    fn from(val: Security) -> u8 {
        Security::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Skew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skew {
    #[inline(always)]
    fn from(val: u8) -> Skew {
        Skew::from_bits(val)
    }
}
impl From<Skew> for u8 {
    #[inline(always)]
    fn from(val: Skew) -> u8 {
        Skew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramXenLock {
    #[doc = "This register is not locked and can be altered."]
    Lock0 = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    Lock1 = 0x01,
}
impl SramXenLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramXenLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramXenLock {
    #[inline(always)]
    fn from(val: u8) -> SramXenLock {
        SramXenLock::from_bits(val)
    }
}
impl From<SramXenLock> for u8 {
    #[inline(always)]
    fn from(val: SramXenLock) -> u8 {
        SramXenLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers."]
    Enable = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    Freeze = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0 {
    #[doc = "RAMX0: 0x04000000 - 0x04001fff."]
    Usb00 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    Usb01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0 {
    #[inline(always)]
    fn from(val: u8) -> Usb0 {
        Usb0::from_bits(val)
    }
}
impl From<Usb0> for u8 {
    #[inline(always)]
    fn from(val: Usb0) -> u8 {
        Usb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbFsEnet {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl UsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> UsbFsEnet {
        UsbFsEnet::from_bits(val)
    }
}
impl From<UsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: UsbFsEnet) -> u8 {
        UsbFsEnet::to_bits(val)
    }
}
