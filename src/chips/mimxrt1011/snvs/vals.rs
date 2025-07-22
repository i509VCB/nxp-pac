#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctd {
    #[doc = "No clock tamper."]
    NOREPORT = 0x0,
    #[doc = "Clock tamper is detected."]
    REPORTED = 0x01,
}
impl Ctd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctd {
    #[inline(always)]
    fn from(val: u8) -> Ctd {
        Ctd::from_bits(val)
    }
}
impl From<Ctd> for u8 {
    #[inline(always)]
    fn from(val: Ctd) -> u8 {
        Ctd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eo {
    #[doc = "Emergency off was not detected."]
    NOREPORT = 0x0,
    #[doc = "Emergency off was detected."]
    REPORTED = 0x01,
}
impl Eo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eo {
    #[inline(always)]
    fn from(val: u8) -> Eo {
        Eo::from_bits(val)
    }
}
impl From<Eo> for u8 {
    #[inline(always)]
    fn from(val: Eo) -> u8 {
        Eo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esvd {
    #[doc = "No external security violation."]
    NOREPORT = 0x0,
    #[doc = "External security violation is detected."]
    REPORTED = 0x01,
}
impl Esvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esvd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esvd {
    #[inline(always)]
    fn from(val: u8) -> Esvd {
        Esvd::from_bits(val)
    }
}
impl From<Esvd> for u8 {
    #[inline(always)]
    fn from(val: Esvd) -> u8 {
        Esvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Et1d {
    #[doc = "External tampering 1 not detected."]
    NOREPORT = 0x0,
    #[doc = "External tampering 1 detected."]
    REPORTED = 0x01,
}
impl Et1d {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Et1d {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Et1d {
    #[inline(always)]
    fn from(val: u8) -> Et1d {
        Et1d::from_bits(val)
    }
}
impl From<Et1d> for u8 {
    #[inline(always)]
    fn from(val: Et1d) -> u8 {
        Et1d::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Et2d {
    #[doc = "External tampering 2 not detected."]
    NOREPORT = 0x0,
    #[doc = "External tampering 2 detected."]
    REPORTED = 0x01,
}
impl Et2d {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Et2d {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Et2d {
    #[inline(always)]
    fn from(val: u8) -> Et2d {
        Et2d::from_bits(val)
    }
}
impl From<Et2d> for u8 {
    #[inline(always)]
    fn from(val: Et2d) -> u8 {
        Et2d::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprHl {
    #[doc = "Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0x0,
    #[doc = "Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 0x01,
}
impl GprHl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprHl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprHl {
    #[inline(always)]
    fn from(val: u8) -> GprHl {
        GprHl::from_bits(val)
    }
}
impl From<GprHl> for u8 {
    #[inline(always)]
    fn from(val: GprHl) -> u8 {
        GprHl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GprSl {
    #[doc = "Write access is allowed"]
    WRITE_ALLOWED = 0x0,
    #[doc = "Write access is not allowed"]
    WRITE_NOT_ALLOWED = 0x01,
}
impl GprSl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GprSl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GprSl {
    #[inline(always)]
    fn from(val: u8) -> GprSl {
        GprSl::from_bits(val)
    }
}
impl From<GprSl> for u8 {
    #[inline(always)]
    fn from(val: GprSl) -> u8 {
        GprSl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcalbVal {
    #[doc = "+0 counts per each 32768 ticks of the counter"]
    ADD_0_PER_32768_TICKS = 0x0,
    #[doc = "+1 counts per each 32768 ticks of the counter"]
    ADD_1_PER_32768_TICKS = 0x01,
    #[doc = "+2 counts per each 32768 ticks of the counter"]
    ADD_2_PER_32768_TICKS = 0x02,
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
    #[doc = "+15 counts per each 32768 ticks of the counter"]
    ADD_15_PER_32768_TICKS = 0x0f,
    #[doc = "-16 counts per each 32768 ticks of the counter"]
    SUB_16_PER_32768_TICKS = 0x10,
    #[doc = "-15 counts per each 32768 ticks of the counter"]
    SUB_15_PER_32768_TICKS = 0x11,
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
    #[doc = "-2 counts per each 32768 ticks of the counter"]
    SUB_2_PER_32768_TICKS = 0x1e,
    #[doc = "-1 counts per each 32768 ticks of the counter"]
    SUB_1_PER_32768_TICKS = 0x1f,
}
impl HpcalbVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcalbVal {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcalbVal {
    #[inline(always)]
    fn from(val: u8) -> HpcalbVal {
        HpcalbVal::from_bits(val)
    }
}
impl From<HpcalbVal> for u8 {
    #[inline(always)]
    fn from(val: HpcalbVal) -> u8 {
        HpcalbVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpcalbVal {
    #[doc = "+0 counts per each 32768 ticks of the counter clock"]
    ADD_0_PER_32768_TICKS = 0x0,
    #[doc = "+1 counts per each 32768 ticks of the counter clock"]
    ADD_1_PER_32768_TICKS = 0x01,
    #[doc = "+2 counts per each 32768 ticks of the counter clock"]
    ADD_2_PER_32768_TICKS = 0x02,
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
    #[doc = "+15 counts per each 32768 ticks of the counter clock"]
    ADD_15_PER_32768_TICKS = 0x0f,
    #[doc = "-16 counts per each 32768 ticks of the counter clock"]
    SUB_16_PER_32768_TICKS = 0x10,
    #[doc = "-15 counts per each 32768 ticks of the counter clock"]
    SUB_15_PER_32768_TICKS = 0x11,
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
    #[doc = "-2 counts per each 32768 ticks of the counter clock"]
    SUB_2_PER_32768_TICKS = 0x1e,
    #[doc = "-1 counts per each 32768 ticks of the counter clock"]
    SUB_1_PER_32768_TICKS = 0x1f,
}
impl LpcalbVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpcalbVal {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpcalbVal {
    #[inline(always)]
    fn from(val: u8) -> LpcalbVal {
        LpcalbVal::from_bits(val)
    }
}
impl From<LpcalbVal> for u8 {
    #[inline(always)]
    fn from(val: LpcalbVal) -> u8 {
        LpcalbVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpns {
    #[doc = "LP section was not programmed in the non-secure state."]
    NOT_PRGRMD_IN_NON_SECURE_STATE = 0x0,
    #[doc = "LP section was programmed in the non-secure state."]
    WAS_PRGRMD_IN_NON_SECURE_STATE = 0x01,
}
impl Lpns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpns {
    #[inline(always)]
    fn from(val: u8) -> Lpns {
        Lpns::from_bits(val)
    }
}
impl From<Lpns> for u8 {
    #[inline(always)]
    fn from(val: Lpns) -> u8 {
        Lpns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lps {
    #[doc = "LP section was not programmed in secure or trusted state."]
    NOT_PRGRMD_IN_SECURE_OR_TRUSTED_STATE = 0x0,
    #[doc = "LP section was programmed in secure or trusted state."]
    WAS_PRGRMD_IN_SECURE_OR_TRUSTED_STATE = 0x01,
}
impl Lps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lps {
    #[inline(always)]
    fn from(val: u8) -> Lps {
        Lps::from_bits(val)
    }
}
impl From<Lps> for u8 {
    #[inline(always)]
    fn from(val: Lps) -> u8 {
        Lps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpsvCfg {
    #[doc = "LP security violation is disabled"]
    DISABLED = 0x0,
    #[doc = "LP security violation is a non-fatal violation"]
    NON_FATAL = 0x01,
    #[doc = "LP security violation is a fatal violation"]
    FATAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpsvCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpsvCfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpsvCfg {
    #[inline(always)]
    fn from(val: u8) -> LpsvCfg {
        LpsvCfg::from_bits(val)
    }
}
impl From<LpsvCfg> for u8 {
    #[inline(always)]
    fn from(val: LpsvCfg) -> u8 {
        LpsvCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvd {
    #[doc = "No low voltage event detected."]
    NOLOWVOLT = 0x0,
    #[doc = "Low voltage event is detected."]
    LOWVOLTDETECTED = 0x01,
}
impl Lvd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvd {
    #[inline(always)]
    fn from(val: u8) -> Lvd {
        Lvd::from_bits(val)
    }
}
impl From<Lvd> for u8 {
    #[inline(always)]
    fn from(val: Lvd) -> u8 {
        Lvd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterKeySel {
    #[doc = "Select one time programmable master key."]
    SELECT_OTPMK = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Select zeroizable master key when MKS_EN bit is set ."]
    SELECT_ZMK = 0x02,
    #[doc = "Select combined master key when MKS_EN bit is set ."]
    SELECT_COMBO = 0x03,
}
impl MasterKeySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterKeySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterKeySel {
    #[inline(always)]
    fn from(val: u8) -> MasterKeySel {
        MasterKeySel::from_bits(val)
    }
}
impl From<MasterKeySel> for u8 {
    #[inline(always)]
    fn from(val: MasterKeySel) -> u8 {
        MasterKeySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McHl {
    #[doc = "Write access (increment) is allowed."]
    WRITE_ACCESS_ALLOWED = 0x0,
    #[doc = "Write access (increment) is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 0x01,
}
impl McHl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McHl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McHl {
    #[inline(always)]
    fn from(val: u8) -> McHl {
        McHl::from_bits(val)
    }
}
impl From<McHl> for u8 {
    #[inline(always)]
    fn from(val: McHl) -> u8 {
        McHl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum McSl {
    #[doc = "Write access (increment) is allowed"]
    WRITE_ALLOWED = 0x0,
    #[doc = "Write access (increment) is not allowed"]
    WRITE_NOT_ALLOWED = 0x01,
}
impl McSl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> McSl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for McSl {
    #[inline(always)]
    fn from(val: u8) -> McSl {
        McSl::from_bits(val)
    }
}
impl From<McSl> for u8 {
    #[inline(always)]
    fn from(val: McSl) -> u8 {
        McSl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mcr {
    #[doc = "MC has not reached its maximum value."]
    NOREPORT = 0x0,
    #[doc = "MC has reached its maximum value."]
    REPORTED = 0x01,
}
impl Mcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcr {
    #[inline(always)]
    fn from(val: u8) -> Mcr {
        Mcr::from_bits(val)
    }
}
impl From<Mcr> for u8 {
    #[inline(always)]
    fn from(val: Mcr) -> u8 {
        Mcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OtpmkZero {
    #[doc = "The OTPMK is not zero."]
    OTPMK_NOT_ZERO = 0x0,
    #[doc = "The OTPMK is zero."]
    OTPMK_IS_ZERO = 0x01,
}
impl OtpmkZero {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OtpmkZero {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OtpmkZero {
    #[inline(always)]
    fn from(val: u8) -> OtpmkZero {
        OtpmkZero::from_bits(val)
    }
}
impl From<OtpmkZero> for u8 {
    #[inline(always)]
    fn from(val: OtpmkZero) -> u8 {
        OtpmkZero::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pi {
    #[doc = "No periodic interrupt occurred."]
    NOREPORT = 0x0,
    #[doc = "A periodic interrupt occurred."]
    REPORTED = 0x01,
}
impl Pi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pi {
    #[inline(always)]
    fn from(val: u8) -> Pi {
        Pi::from_bits(val)
    }
}
impl From<Pi> for u8 {
    #[inline(always)]
    fn from(val: Pi) -> u8 {
        Pi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PiFreq {
    #[doc = "- bit 0 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_0 = 0x0,
    #[doc = "- bit 1 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1 = 0x01,
    #[doc = "- bit 2 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_2 = 0x02,
    #[doc = "- bit 3 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_3 = 0x03,
    #[doc = "- bit 4 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_4 = 0x04,
    #[doc = "- bit 5 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_5 = 0x05,
    #[doc = "- bit 6 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_6 = 0x06,
    #[doc = "- bit 7 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_7 = 0x07,
    #[doc = "- bit 8 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_8 = 0x08,
    #[doc = "- bit 9 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_9 = 0x09,
    #[doc = "- bit 10 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_10 = 0x0a,
    #[doc = "- bit 11 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_11 = 0x0b,
    #[doc = "- bit 12 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_12 = 0x0c,
    #[doc = "- bit 13 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_13 = 0x0d,
    #[doc = "- bit 14 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_14 = 0x0e,
    #[doc = "- bit 15 of the HPRTCLR is selected as a source of the periodic interrupt"]
    USE_BIT_1R5 = 0x0f,
}
impl PiFreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PiFreq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PiFreq {
    #[inline(always)]
    fn from(val: u8) -> PiFreq {
        PiFreq::from_bits(val)
    }
}
impl From<PiFreq> for u8 {
    #[inline(always)]
    fn from(val: PiFreq) -> u8 {
        PiFreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgZmk {
    #[doc = "No Action"]
    NO_ACTION = 0x0,
    #[doc = "Activate hardware key programming mechanism"]
    PROGRAM_KEY = 0x01,
}
impl ProgZmk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProgZmk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProgZmk {
    #[inline(always)]
    fn from(val: u8) -> ProgZmk {
        ProgZmk::from_bits(val)
    }
}
impl From<ProgZmk> for u8 {
    #[inline(always)]
    fn from(val: ProgZmk) -> u8 {
        ProgZmk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spof {
    #[doc = "Set Power Off was not detected."]
    NOREPORT = 0x0,
    #[doc = "Set Power Off was detected."]
    REPORTED = 0x01,
}
impl Spof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spof {
    #[inline(always)]
    fn from(val: u8) -> Spof {
        Spof::from_bits(val)
    }
}
impl From<Spof> for u8 {
    #[inline(always)]
    fn from(val: Spof) -> u8 {
        Spof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spon {
    #[doc = "Set Power On Interrupt was not detected."]
    NOREPORT = 0x0,
    #[doc = "Set Power On Interrupt was detected."]
    REPORTED = 0x01,
}
impl Spon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spon {
    #[inline(always)]
    fn from(val: u8) -> Spon {
        Spon::from_bits(val)
    }
}
impl From<Spon> for u8 {
    #[inline(always)]
    fn from(val: Spon) -> u8 {
        Spon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrtcHl {
    #[doc = "Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0x0,
    #[doc = "Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 0x01,
}
impl SrtcHl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrtcHl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrtcHl {
    #[inline(always)]
    fn from(val: u8) -> SrtcHl {
        SrtcHl::from_bits(val)
    }
}
impl From<SrtcHl> for u8 {
    #[inline(always)]
    fn from(val: SrtcHl) -> u8 {
        SrtcHl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SrtcSl {
    #[doc = "Write access is allowed"]
    WRITE_ALLOWED = 0x0,
    #[doc = "Write access is not allowed"]
    WRITE_NOT_ALLOWED = 0x01,
}
impl SrtcSl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SrtcSl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SrtcSl {
    #[inline(always)]
    fn from(val: u8) -> SrtcSl {
        SrtcSl::from_bits(val)
    }
}
impl From<SrtcSl> for u8 {
    #[inline(always)]
    fn from(val: SrtcSl) -> u8 {
        SrtcSl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srtcr {
    #[doc = "SRTC has not reached its maximum value."]
    NOREPORT = 0x0,
    #[doc = "SRTC has reached its maximum value."]
    REPORTED = 0x01,
}
impl Srtcr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srtcr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srtcr {
    #[inline(always)]
    fn from(val: u8) -> Srtcr {
        Srtcr::from_bits(val)
    }
}
impl From<Srtcr> for u8 {
    #[inline(always)]
    fn from(val: Srtcr) -> u8 {
        Srtcr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsmState {
    #[doc = "Init"]
    INIT = 0x0,
    #[doc = "Hard Fail"]
    HARD_FAIL = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Soft Fail"]
    SOFT_FAIL = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Init Intermediate (transition state between Init and Check - SSM stays in this state only one clock cycle)"]
    INTERMEDIATE = 0x08,
    #[doc = "Check"]
    CHECK = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "Non-Secure"]
    NON_SECURE = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "Trusted"]
    TRUSTED = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Secure"]
    SECURE = 0x0f,
}
impl SsmState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsmState {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsmState {
    #[inline(always)]
    fn from(val: u8) -> SsmState {
        SsmState::from_bits(val)
    }
}
impl From<SsmState> for u8 {
    #[inline(always)]
    fn from(val: SsmState) -> u8 {
        SsmState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv0 {
    #[doc = "No Security Violation 0 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 0 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv0 {
    #[inline(always)]
    fn from(val: u8) -> Sv0 {
        Sv0::from_bits(val)
    }
}
impl From<Sv0> for u8 {
    #[inline(always)]
    fn from(val: Sv0) -> u8 {
        Sv0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv0Cfg {
    #[doc = "Security Violation 0 is a non-fatal violation"]
    NON_FATAL = 0x0,
    #[doc = "Security Violation 0 is a fatal violation"]
    FATAL = 0x01,
}
impl Sv0Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv0Cfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv0Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv0Cfg {
        Sv0Cfg::from_bits(val)
    }
}
impl From<Sv0Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv0Cfg) -> u8 {
        Sv0Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv1 {
    #[doc = "No Security Violation 1 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 1 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv1 {
    #[inline(always)]
    fn from(val: u8) -> Sv1 {
        Sv1::from_bits(val)
    }
}
impl From<Sv1> for u8 {
    #[inline(always)]
    fn from(val: Sv1) -> u8 {
        Sv1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv1Cfg {
    #[doc = "Security Violation 1 is a non-fatal violation"]
    NON_FATAL = 0x0,
    #[doc = "Security Violation 1 is a fatal violation"]
    FATAL = 0x01,
}
impl Sv1Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv1Cfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv1Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv1Cfg {
        Sv1Cfg::from_bits(val)
    }
}
impl From<Sv1Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv1Cfg) -> u8 {
        Sv1Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv2 {
    #[doc = "No Security Violation 2 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 2 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv2 {
    #[inline(always)]
    fn from(val: u8) -> Sv2 {
        Sv2::from_bits(val)
    }
}
impl From<Sv2> for u8 {
    #[inline(always)]
    fn from(val: Sv2) -> u8 {
        Sv2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv2Cfg {
    #[doc = "Security Violation 2 is a non-fatal violation"]
    NON_FATAL = 0x0,
    #[doc = "Security Violation 2 is a fatal violation"]
    FATAL = 0x01,
}
impl Sv2Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv2Cfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv2Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv2Cfg {
        Sv2Cfg::from_bits(val)
    }
}
impl From<Sv2Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv2Cfg) -> u8 {
        Sv2Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv3 {
    #[doc = "No Security Violation 3 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 3 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv3 {
    #[inline(always)]
    fn from(val: u8) -> Sv3 {
        Sv3::from_bits(val)
    }
}
impl From<Sv3> for u8 {
    #[inline(always)]
    fn from(val: Sv3) -> u8 {
        Sv3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv3Cfg {
    #[doc = "Security Violation 3 is a non-fatal violation"]
    NON_FATAL = 0x0,
    #[doc = "Security Violation 3 is a fatal violation"]
    FATAL = 0x01,
}
impl Sv3Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv3Cfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv3Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv3Cfg {
        Sv3Cfg::from_bits(val)
    }
}
impl From<Sv3Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv3Cfg) -> u8 {
        Sv3Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv4 {
    #[doc = "No Security Violation 4 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 4 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv4 {
    #[inline(always)]
    fn from(val: u8) -> Sv4 {
        Sv4::from_bits(val)
    }
}
impl From<Sv4> for u8 {
    #[inline(always)]
    fn from(val: Sv4) -> u8 {
        Sv4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv4Cfg {
    #[doc = "Security Violation 4 is a non-fatal violation"]
    NON_FATAL = 0x0,
    #[doc = "Security Violation 4 is a fatal violation"]
    FATAL = 0x01,
}
impl Sv4Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv4Cfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv4Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv4Cfg {
        Sv4Cfg::from_bits(val)
    }
}
impl From<Sv4Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv4Cfg) -> u8 {
        Sv4Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv5 {
    #[doc = "No Security Violation 5 security violation was detected."]
    NOREPORT = 0x0,
    #[doc = "Security Violation 5 security violation was detected."]
    REPORTED = 0x01,
}
impl Sv5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv5 {
    #[inline(always)]
    fn from(val: u8) -> Sv5 {
        Sv5::from_bits(val)
    }
}
impl From<Sv5> for u8 {
    #[inline(always)]
    fn from(val: Sv5) -> u8 {
        Sv5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sv5Cfg {
    #[doc = "Security Violation 5 is disabled"]
    DISABLED = 0x0,
    #[doc = "Security Violation 5 is a non-fatal violation"]
    NON_FATAL = 0x01,
    #[doc = "Security Violation 5 is a fatal violation"]
    FATAL = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sv5Cfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sv5Cfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sv5Cfg {
    #[inline(always)]
    fn from(val: u8) -> Sv5Cfg {
        Sv5Cfg::from_bits(val)
    }
}
impl From<Sv5Cfg> for u8 {
    #[inline(always)]
    fn from(val: Sv5Cfg) -> u8 {
        Sv5Cfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysSecurityCfg {
    #[doc = "Fab Configuration - the default configuration of newly fabricated chips"]
    FAB_CONFIG = 0x0,
    #[doc = "Open Configuration - the configuration after NXP-programmable fuses have been blown"]
    OPEN_CONFIG = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Closed Configuration - the configuration after OEM-programmable fuses have been blown"]
    CLOSED_CONFIG = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Field Return Configuration - the configuration of chips that are returned to NXP for analysis"]
    FIELD_RETURN_CONFIG = 0x07,
}
impl SysSecurityCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysSecurityCfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysSecurityCfg {
    #[inline(always)]
    fn from(val: u8) -> SysSecurityCfg {
        SysSecurityCfg::from_bits(val)
    }
}
impl From<SysSecurityCfg> for u8 {
    #[inline(always)]
    fn from(val: SysSecurityCfg) -> u8 {
        SysSecurityCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Top {
    #[doc = "Leave system power on."]
    KEEP_ON = 0x0,
    #[doc = "Turn off system power."]
    TURN_OFF = 0x01,
}
impl Top {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Top {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Top {
    #[inline(always)]
    fn from(val: u8) -> Top {
        Top::from_bits(val)
    }
}
impl From<Top> for u8 {
    #[inline(always)]
    fn from(val: Top) -> u8 {
        Top::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ttd {
    #[doc = "No temperature tamper."]
    NOREPORT = 0x0,
    #[doc = "Temperature tamper is detected."]
    REPORTED = 0x01,
}
impl Ttd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ttd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ttd {
    #[inline(always)]
    fn from(val: u8) -> Ttd {
        Ttd::from_bits(val)
    }
}
impl From<Ttd> for u8 {
    #[inline(always)]
    fn from(val: Ttd) -> u8 {
        Ttd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vtd {
    #[doc = "Voltage tampering not detected."]
    NOREPORT = 0x0,
    #[doc = "Voltage tampering detected."]
    REPORTED = 0x01,
}
impl Vtd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vtd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vtd {
    #[inline(always)]
    fn from(val: u8) -> Vtd {
        Vtd::from_bits(val)
    }
}
impl From<Vtd> for u8 {
    #[inline(always)]
    fn from(val: Vtd) -> u8 {
        Vtd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmt1d {
    #[doc = "Wire-mesh tampering 1 not detected."]
    NOREPORT = 0x0,
    #[doc = "Wire-mesh tampering 1 detected."]
    REPORTED = 0x01,
}
impl Wmt1d {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmt1d {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmt1d {
    #[inline(always)]
    fn from(val: u8) -> Wmt1d {
        Wmt1d::from_bits(val)
    }
}
impl From<Wmt1d> for u8 {
    #[inline(always)]
    fn from(val: Wmt1d) -> u8 {
        Wmt1d::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmt2d {
    #[doc = "Wire-mesh tampering 2 not detected."]
    NOREPORT = 0x0,
    #[doc = "Wire-mesh tampering 2 detected."]
    REPORTED = 0x01,
}
impl Wmt2d {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmt2d {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmt2d {
    #[inline(always)]
    fn from(val: u8) -> Wmt2d {
        Wmt2d::from_bits(val)
    }
}
impl From<Wmt2d> for u8 {
    #[inline(always)]
    fn from(val: Wmt2d) -> u8 {
        Wmt2d::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkEccFail {
    #[doc = "ZMK ECC Failure was not detected."]
    NOREPORT = 0x0,
    #[doc = "ZMK ECC Failure was detected."]
    REPORTED = 0x01,
}
impl ZmkEccFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkEccFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkEccFail {
    #[inline(always)]
    fn from(val: u8) -> ZmkEccFail {
        ZmkEccFail::from_bits(val)
    }
}
impl From<ZmkEccFail> for u8 {
    #[inline(always)]
    fn from(val: ZmkEccFail) -> u8 {
        ZmkEccFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkHwp {
    #[doc = "ZMK is in the software programming mode."]
    SW_PROG_MODE = 0x0,
    #[doc = "ZMK is in the hardware programming mode."]
    HW_PROG_MODE = 0x01,
}
impl ZmkHwp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkHwp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkHwp {
    #[inline(always)]
    fn from(val: u8) -> ZmkHwp {
        ZmkHwp::from_bits(val)
    }
}
impl From<ZmkHwp> for u8 {
    #[inline(always)]
    fn from(val: ZmkHwp) -> u8 {
        ZmkHwp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkRhl {
    #[doc = "Read access is allowed (only in software programming mode)."]
    READ_ACCESS_ALLOWED = 0x0,
    #[doc = "Read access is not allowed."]
    READ_ACCESS_NOT_ALLOWED = 0x01,
}
impl ZmkRhl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkRhl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkRhl {
    #[inline(always)]
    fn from(val: u8) -> ZmkRhl {
        ZmkRhl::from_bits(val)
    }
}
impl From<ZmkRhl> for u8 {
    #[inline(always)]
    fn from(val: ZmkRhl) -> u8 {
        ZmkRhl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkRsl {
    #[doc = "Read access is allowed (only in software Programming mode)"]
    READ_ALLOWED = 0x0,
    #[doc = "Read access is not allowed"]
    READ_NOT_ALLOWED = 0x01,
}
impl ZmkRsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkRsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkRsl {
    #[inline(always)]
    fn from(val: u8) -> ZmkRsl {
        ZmkRsl::from_bits(val)
    }
}
impl From<ZmkRsl> for u8 {
    #[inline(always)]
    fn from(val: ZmkRsl) -> u8 {
        ZmkRsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkWhl {
    #[doc = "Write access is allowed."]
    WRITE_ACCESS_ALLOWED = 0x0,
    #[doc = "Write access is not allowed."]
    WRITE_ACCESS_NOT_ALLOWED = 0x01,
}
impl ZmkWhl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkWhl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkWhl {
    #[inline(always)]
    fn from(val: u8) -> ZmkWhl {
        ZmkWhl::from_bits(val)
    }
}
impl From<ZmkWhl> for u8 {
    #[inline(always)]
    fn from(val: ZmkWhl) -> u8 {
        ZmkWhl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkWsl {
    #[doc = "Write access is allowed"]
    WRITE_ALLOWED = 0x0,
    #[doc = "Write access is not allowed"]
    WRITE_NOT_ALLOWED = 0x01,
}
impl ZmkWsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkWsl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkWsl {
    #[inline(always)]
    fn from(val: u8) -> ZmkWsl {
        ZmkWsl::from_bits(val)
    }
}
impl From<ZmkWsl> for u8 {
    #[inline(always)]
    fn from(val: ZmkWsl) -> u8 {
        ZmkWsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZmkZero {
    #[doc = "The ZMK is not zero."]
    ZMK_NOT_ZERO = 0x0,
    #[doc = "The ZMK is zero."]
    ZMK_IS_ZERO = 0x01,
}
impl ZmkZero {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZmkZero {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZmkZero {
    #[inline(always)]
    fn from(val: u8) -> ZmkZero {
        ZmkZero::from_bits(val)
    }
}
impl From<ZmkZero> for u8 {
    #[inline(always)]
    fn from(val: ZmkZero) -> u8 {
        ZmkZero::to_bits(val)
    }
}
