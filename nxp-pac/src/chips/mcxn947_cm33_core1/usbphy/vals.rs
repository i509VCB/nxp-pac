#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avalid {
    #[doc = "Below threshold."]
    AvalidLo = 0x0,
    #[doc = "Above threshold."]
    AvalidHi = 0x01,
}
impl Avalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avalid {
    #[inline(always)]
    fn from(val: u8) -> Avalid {
        Avalid::from_bits(val)
    }
}
impl From<Avalid> for u8 {
    #[inline(always)]
    fn from(val: Avalid) -> u8 {
        Avalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bvalid {
    #[doc = "Below threshold."]
    BvalidLo = 0x0,
    #[doc = "Above threshold."]
    BvalidHi = 0x01,
}
impl Bvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bvalid {
    #[inline(always)]
    fn from(val: u8) -> Bvalid {
        Bvalid::from_bits(val)
    }
}
impl From<Bvalid> for u8 {
    #[inline(always)]
    fn from(val: Bvalid) -> u8 {
        Bvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChkChrgB {
    #[doc = "Enable."]
    BcChrgdetEnable = 0x0,
    #[doc = "Disable."]
    BcChrgdetDisable = 0x01,
}
impl ChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> ChkChrgB {
        ChkChrgB::from_bits(val)
    }
}
impl From<ChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: ChkChrgB) -> u8 {
        ChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChkContact {
    #[doc = "Disable."]
    BcDcdDisable = 0x0,
    #[doc = "Enable."]
    BcDcdEnable = 0x01,
}
impl ChkContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChkContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChkContact {
    #[inline(always)]
    fn from(val: u8) -> ChkContact {
        ChkContact::from_bits(val)
    }
}
impl From<ChkContact> for u8 {
    #[inline(always)]
    fn from(val: ChkContact) -> u8 {
        ChkContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChrgDetected {
    #[doc = "SDP detected."]
    SdpDetect = 0x0,
    #[doc = "Charging port detected."]
    ChrgPortDetect = 0x01,
}
impl ChrgDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChrgDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChrgDetected {
    #[inline(always)]
    fn from(val: u8) -> ChrgDetected {
        ChrgDetected::from_bits(val)
    }
}
impl From<ChrgDetected> for u8 {
    #[inline(always)]
    fn from(val: ChrgDetected) -> u8 {
        ChrgDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkgate {
    #[doc = "Run clocks."]
    RunClocks = 0x0,
    #[doc = "Gate clocks."]
    GateClocks = 0x01,
}
impl Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Clkgate {
        Clkgate::from_bits(val)
    }
}
impl From<Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Clkgate) -> u8 {
        Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MaxCurrent = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    Nominal = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MinCurrent = 0x0f,
}
impl DCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCal {
    #[inline(always)]
    fn from(val: u8) -> DCal {
        DCal::from_bits(val)
    }
}
impl From<DCal> for u8 {
    #[inline(always)]
    fn from(val: DCal) -> u8 {
        DCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcdsel {
    #[doc = "Fields in USB1_CHRG_DETECT."]
    ChrgdetCtrl = 0x0,
    #[doc = "Fields and state machines in the USBHSDCD module."]
    UsbhsdcdCtrl = 0x01,
}
impl Dcdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcdsel {
    #[inline(always)]
    fn from(val: u8) -> Dcdsel {
        Dcdsel::from_bits(val)
    }
}
impl From<Dcdsel> for u8 {
    #[inline(always)]
    fn from(val: Dcdsel) -> u8 {
        Dcdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectSec {
    #[doc = "Disable."]
    BcSecdetDisable = 0x0,
    #[doc = "Enable."]
    BcSecdetEnable = 0x01,
}
impl DetectSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectSec {
    #[inline(always)]
    fn from(val: u8) -> DetectSec {
        DetectSec::from_bits(val)
    }
}
impl From<DetectSec> for u8 {
    #[inline(always)]
    fn from(val: DetectSec) -> u8 {
        DetectSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevPulldown {
    #[doc = "Disable."]
    DevPulldownDis = 0x0,
    #[doc = "Enable."]
    DevPulldownEn = 0x01,
}
impl DevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevPulldown {
    #[inline(always)]
    fn from(val: u8) -> DevPulldown {
        DevPulldown::from_bits(val)
    }
}
impl From<DevPulldown> for u8 {
    #[inline(always)]
    fn from(val: DevPulldown) -> u8 {
        DevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginPolarity {
    #[doc = "Plugged in."]
    PluggedIn = 0x0,
    #[doc = "Unplugged."]
    Unplugged = 0x01,
}
impl DevpluginPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevpluginPolarity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevpluginPolarity {
    #[inline(always)]
    fn from(val: u8) -> DevpluginPolarity {
        DevpluginPolarity::from_bits(val)
    }
}
impl From<DevpluginPolarity> for u8 {
    #[inline(always)]
    fn from(val: DevpluginPolarity) -> u8 {
        DevpluginPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginStatus {
    #[doc = "No attachment detected."]
    NoCable = 0x0,
    #[doc = "Cable attachment detected."]
    CableAttach = 0x01,
}
impl DevpluginStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevpluginStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevpluginStatus {
    #[inline(always)]
    fn from(val: u8) -> DevpluginStatus {
        DevpluginStatus::from_bits(val)
    }
}
impl From<DevpluginStatus> for u8 {
    #[inline(always)]
    fn from(val: DevpluginStatus) -> u8 {
        DevpluginStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DischargeVbus {
    #[doc = "Disable."]
    VbusDchgOff = 0x0,
    #[doc = "Enable."]
    VbusDchgOn = 0x01,
}
impl DischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> DischargeVbus {
        DischargeVbus::from_bits(val)
    }
}
impl From<DischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: DischargeVbus) -> u8 {
        DischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disconadj {
    #[doc = "0.56875 V."]
    DisconTrimNom = 0x0,
    #[doc = "0.55000 V."]
    DisconTrimLo = 0x01,
    #[doc = "0.58125 V."]
    DisconTrimMedhi = 0x02,
    #[doc = "0.60000 V."]
    DisconTrimHi = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Disconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disconadj {
    #[inline(always)]
    fn from(val: u8) -> Disconadj {
        Disconadj::from_bits(val)
    }
}
impl From<Disconadj> for u8 {
    #[inline(always)]
    fn from(val: Disconadj) -> u8 {
        Disconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivSelOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Plldiv = 0x0,
    #[doc = "PLL_SIC."]
    UsePllSicPlldiv = 0x01,
}
impl DivSelOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivSelOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivSelOverride {
    #[inline(always)]
    fn from(val: u8) -> DivSelOverride {
        DivSelOverride::from_bits(val)
    }
}
impl From<DivSelOverride> for u8 {
    #[inline(always)]
    fn from(val: DivSelOverride) -> u8 {
        DivSelOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmState {
    #[doc = "USB_DM pin voltage is <= 0.8 V."]
    DmSerxLo = 0x0,
    #[doc = "USB_DM pin voltage is >= 2.0 V."]
    DmSerxHi = 0x01,
}
impl DmState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmState {
    #[inline(always)]
    fn from(val: u8) -> DmState {
        DmState::from_bits(val)
    }
}
impl From<DmState> for u8 {
    #[inline(always)]
    fn from(val: DmState) -> u8 {
        DmState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpState {
    #[doc = "USB_DP pin voltage is <= 0.8 V."]
    DpSerxLo = 0x0,
    #[doc = "USB_DP pin voltage is >= 2.0 V."]
    DpSerxHi = 0x01,
}
impl DpState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpState {
    #[inline(always)]
    fn from(val: u8) -> DpState {
        DpState::from_bits(val)
    }
}
impl From<DpState> for u8 {
    #[inline(always)]
    fn from(val: DpState) -> u8 {
        DpState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnB {
    #[doc = "Enable."]
    BcEnable = 0x0,
    #[doc = "Disable."]
    BcDisable = 0x01,
}
impl EnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnB {
    #[inline(always)]
    fn from(val: u8) -> EnB {
        EnB::from_bits(val)
    }
}
impl From<EnB> for u8 {
    #[inline(always)]
    fn from(val: EnB) -> u8 {
        EnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endevplugindetect {
    #[doc = "Disable."]
    PluginDisable = 0x0,
    #[doc = "Enable."]
    PluginEnable = 0x01,
}
impl Endevplugindetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endevplugindetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endevplugindetect {
    #[inline(always)]
    fn from(val: u8) -> Endevplugindetect {
        Endevplugindetect::from_bits(val)
    }
}
impl From<Endevplugindetect> for u8 {
    #[inline(always)]
    fn from(val: Endevplugindetect) -> u8 {
        Endevplugindetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enhstpulldown {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable."]
    Enable = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enhstpulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enhstpulldown {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enhstpulldown {
    #[inline(always)]
    fn from(val: u8) -> Enhstpulldown {
        Enhstpulldown::from_bits(val)
    }
}
impl From<Enhstpulldown> for u8 {
    #[inline(always)]
    fn from(val: Enhstpulldown) -> u8 {
        Enhstpulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enotgiddetect {
    #[doc = "Disable."]
    IdDetDisable = 0x0,
    #[doc = "Enable."]
    IdDetEnable = 0x01,
}
impl Enotgiddetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enotgiddetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enotgiddetect {
    #[inline(always)]
    fn from(val: u8) -> Enotgiddetect {
        Enotgiddetect::from_bits(val)
    }
}
impl From<Enotgiddetect> for u8 {
    #[inline(always)]
    fn from(val: Enotgiddetect) -> u8 {
        Enotgiddetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Envadj {
    #[doc = "0.1000 V."]
    EnvTrimNom = 0x0,
    #[doc = "0.1125 V."]
    EnvTrimMedhi = 0x01,
    #[doc = "0.1250 V."]
    EnvTrimHi = 0x02,
    #[doc = "0.0875 V."]
    EnvTrimLo = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Envadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Envadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Envadj {
    #[inline(always)]
    fn from(val: u8) -> Envadj {
        Envadj::from_bits(val)
    }
}
impl From<Envadj> for u8 {
    #[inline(always)]
    fn from(val: Envadj) -> u8 {
        Envadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtIdOverrideEn {
    #[doc = "Internal detector or local override."]
    UsePhyId = 0x0,
    #[doc = "External ID signal value."]
    UseExtId = 0x01,
}
impl ExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> ExtIdOverrideEn {
        ExtIdOverrideEn::from_bits(val)
    }
}
impl From<ExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: ExtIdOverrideEn) -> u8 {
        ExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtVbusOverrideEn {
    #[doc = "Internal detector or local override."]
    UsePhyVbus = 0x0,
    #[doc = "External VBUS_VALID value."]
    UsbExtVbus = 0x01,
}
impl ExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> ExtVbusOverrideEn {
        ExtVbusOverrideEn::from_bits(val)
    }
}
impl From<ExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: ExtVbusOverrideEn) -> u8 {
        ExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostdiscondetectIrq {
    #[doc = "Connected."]
    Connected = 0x0,
    #[doc = "Disconnected."]
    Disconnected = 0x01,
}
impl HostdiscondetectIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostdiscondetectIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostdiscondetectIrq {
    #[inline(always)]
    fn from(val: u8) -> HostdiscondetectIrq {
        HostdiscondetectIrq::from_bits(val)
    }
}
impl From<HostdiscondetectIrq> for u8 {
    #[inline(always)]
    fn from(val: HostdiscondetectIrq) -> u8 {
        HostdiscondetectIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hstpulldown {
    #[doc = "Disconnect."]
    Disconnect = 0x0,
    #[doc = "Connect."]
    Connect = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hstpulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hstpulldown {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hstpulldown {
    #[inline(always)]
    fn from(val: u8) -> Hstpulldown {
        Hstpulldown::from_bits(val)
    }
}
impl From<Hstpulldown> for u8 {
    #[inline(always)]
    fn from(val: Hstpulldown) -> u8 {
        Hstpulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdOverrideEn {
    #[doc = "Use ID pin detector or external override."]
    NoPhyIdOverride = 0x0,
    #[doc = "Allow local override of ID pin detection status."]
    UsePhyIdOverride = 0x01,
}
impl IdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> IdOverrideEn {
        IdOverrideEn::from_bits(val)
    }
}
impl From<IdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: IdOverrideEn) -> u8 {
        IdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LviEn {
    #[doc = "Disable."]
    Lvi3vDisable = 0x0,
    #[doc = "Enable."]
    Lvi3vEnable = 0x01,
}
impl LviEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LviEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LviEn {
    #[inline(always)]
    fn from(val: u8) -> LviEn {
        LviEn::from_bits(val)
    }
}
impl From<LviEn> for u8 {
    #[inline(always)]
    fn from(val: LviEn) -> u8 {
        LviEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Control0 {
    #[doc = "Power up PLL."]
    PllOnSuspend = 0x0,
    #[doc = "Power down PLL."]
    PllOffSuspend = 0x01,
}
impl Misc2Control0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Control0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Control0 {
    #[inline(always)]
    fn from(val: u8) -> Misc2Control0 {
        Misc2Control0::from_bits(val)
    }
}
impl From<Misc2Control0> for u8 {
    #[inline(always)]
    fn from(val: Misc2Control0) -> u8 {
        Misc2Control0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OkStatus3v {
    #[doc = "Not powered."]
    Power31p8Ok = 0x0,
    #[doc = "Powered."]
    Power31p8Bad = 0x01,
}
impl OkStatus3v {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OkStatus3v {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OkStatus3v {
    #[inline(always)]
    fn from(val: u8) -> OkStatus3v {
        OkStatus3v::from_bits(val)
    }
}
impl From<OkStatus3v> for u8 {
    #[inline(always)]
    fn from(val: OkStatus3v) -> u8 {
        OkStatus3v::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OtgIdValue {
    #[doc = "Host."]
    IdHost = 0x0,
    #[doc = "Device."]
    IdDevice = 0x01,
}
impl OtgIdValue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OtgIdValue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OtgIdValue {
    #[inline(always)]
    fn from(val: u8) -> OtgIdValue {
        OtgIdValue::from_bits(val)
    }
}
impl From<OtgIdValue> for u8 {
    #[inline(always)]
    fn from(val: OtgIdValue) -> u8 {
        OtgIdValue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OtgidStatus {
    #[doc = "Host."]
    IdHost = 0x0,
    #[doc = "Device."]
    IdDevice = 0x01,
}
impl OtgidStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OtgidStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OtgidStatus {
    #[inline(always)]
    fn from(val: u8) -> OtgidStatus {
        OtgidStatus::from_bits(val)
    }
}
impl From<OtgidStatus> for u8 {
    #[inline(always)]
    fn from(val: OtgidStatus) -> u8 {
        OtgidStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfd0Clkgate {
    #[doc = "Enable."]
    Pfd0ClkEn = 0x0,
    #[doc = "Disable."]
    Pfd0ClkDis = 0x01,
}
impl Pfd0Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd0Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd0Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd0Clkgate {
        Pfd0Clkgate::from_bits(val)
    }
}
impl From<Pfd0Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd0Clkgate) -> u8 {
        Pfd0Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfdClkSel {
    #[doc = "USB1PFDCLK = USB PLL reference clock."]
    PfdClkBypass = 0x0,
    #[doc = "USB1PFDCLK = pfd_clk / 4."]
    PfdClkDiv4 = 0x01,
    #[doc = "USB1PFDCLK frequency = pfd_clk / 2."]
    PfdClkDiv2 = 0x02,
    #[doc = "USB1PFDCLK = pfd_clk."]
    PfdClkDiv1 = 0x03,
}
impl PfdClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfdClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfdClkSel {
    #[inline(always)]
    fn from(val: u8) -> PfdClkSel {
        PfdClkSel::from_bits(val)
    }
}
impl From<PfdClkSel> for u8 {
    #[inline(always)]
    fn from(val: PfdClkSel) -> u8 {
        PfdClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllBypass {
    #[doc = "480 MHz output clock."]
    PllNoBypass = 0x0,
    #[doc = "Input reference clock."]
    PllBypass = 0x01,
}
impl PllBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllBypass {
    #[inline(always)]
    fn from(val: u8) -> PllBypass {
        PllBypass::from_bits(val)
    }
}
impl From<PllBypass> for u8 {
    #[inline(always)]
    fn from(val: PllBypass) -> u8 {
        PllBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllDivSel {
    #[doc = "Configure for a 32 MHz input clock (divide by 15)."]
    PllDiv15 = 0x0,
    #[doc = "Configure for a 30 MHz input clock (divide by 16)."]
    PllDiv16 = 0x01,
    #[doc = "Configure for a 24 MHz input clock (divide by 20)."]
    PllDiv20 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Configure for a 20 MHz input clock (divide by 24)."]
    PllDiv24 = 0x04,
    #[doc = "Configure for a 19.2 MHz input clock (divide by 25)."]
    PllDiv25 = 0x05,
    #[doc = "Configure for a 16 MHz input clock (divide by 30)."]
    PllDiv30 = 0x06,
    #[doc = "Configure for a 12 MHz input clock (divide by 40)."]
    PllDiv32 = 0x07,
}
impl PllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllDivSel {
        PllDivSel::from_bits(val)
    }
}
impl From<PllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllDivSel) -> u8 {
        PllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnUsbClks {
    #[doc = "Disable."]
    PllMpDisable = 0x0,
    #[doc = "Enable."]
    PllMpEnable = 0x01,
}
impl PllEnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllEnUsbClks {
        PllEnUsbClks::from_bits(val)
    }
}
impl From<PllEnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllEnUsbClks) -> u8 {
        PllEnUsbClks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnable {
    #[doc = "Disable."]
    PllOutDisable = 0x0,
    #[doc = "Enable."]
    PllOutEnable = 0x01,
}
impl PllEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnable {
    #[inline(always)]
    fn from(val: u8) -> PllEnable {
        PllEnable::from_bits(val)
    }
}
impl From<PllEnable> for u8 {
    #[inline(always)]
    fn from(val: PllEnable) -> u8 {
        PllEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllLock {
    #[doc = "Not locked."]
    PllNotLocked = 0x0,
    #[doc = "Locked."]
    PllLocked = 0x01,
}
impl PllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllLock {
    #[inline(always)]
    fn from(val: u8) -> PllLock {
        PllLock::from_bits(val)
    }
}
impl From<PllLock> for u8 {
    #[inline(always)]
    fn from(val: PllLock) -> u8 {
        PllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllPower {
    #[doc = "Power down."]
    PllForcePwd = 0x0,
    #[doc = "Allow powerup."]
    PllAllowPowerup = 0x01,
}
impl PllPower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllPower {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllPower {
    #[inline(always)]
    fn from(val: u8) -> PllPower {
        PllPower::from_bits(val)
    }
}
impl From<PllPower> for u8 {
    #[inline(always)]
    fn from(val: PllPower) -> u8 {
        PllPower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllRegEnable {
    #[doc = "Disable."]
    PllRegDisable = 0x0,
    #[doc = "Enable."]
    PllRegEnable = 0x01,
}
impl PllRegEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllRegEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllRegEnable {
    #[inline(always)]
    fn from(val: u8) -> PllRegEnable {
        PllRegEnable::from_bits(val)
    }
}
impl From<PllRegEnable> for u8 {
    #[inline(always)]
    fn from(val: PllRegEnable) -> u8 {
        PllRegEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugContact {
    #[doc = "Not detected."]
    NoDcDetected = 0x0,
    #[doc = "Detected."]
    DcDeteced = 0x01,
}
impl PlugContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlugContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlugContact {
    #[inline(always)]
    fn from(val: u8) -> PlugContact {
        PlugContact::from_bits(val)
    }
}
impl From<PlugContact> for u8 {
    #[inline(always)]
    fn from(val: PlugContact) -> u8 {
        PlugContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PullupDp {
    #[doc = "Disable."]
    DpPueNormal = 0x0,
    #[doc = "Enable."]
    DpPueOverride = 0x01,
}
impl PullupDp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PullupDp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PullupDp {
    #[inline(always)]
    fn from(val: u8) -> PullupDp {
        PullupDp::from_bits(val)
    }
}
impl From<PullupDp> for u8 {
    #[inline(always)]
    fn from(val: PullupDp) -> u8 {
        PullupDp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefbiasPwd {
    #[doc = "Enable."]
    RefbiasEnabled = 0x0,
    #[doc = "Disable or power down."]
    RefbiasPwd = 0x01,
}
impl RefbiasPwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefbiasPwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefbiasPwd {
    #[inline(always)]
    fn from(val: u8) -> RefbiasPwd {
        RefbiasPwd::from_bits(val)
    }
}
impl From<RefbiasPwd> for u8 {
    #[inline(always)]
    fn from(val: RefbiasPwd) -> u8 {
        RefbiasPwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefbiasPwdSel {
    #[doc = "PLL_POWER internal state signal."]
    BiasPllpower = 0x0,
    #[doc = "REFBIAS_PWD."]
    BiasRefbiasPwd = 0x01,
}
impl RefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> RefbiasPwdSel {
        RefbiasPwdSel::from_bits(val)
    }
}
impl From<RefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: RefbiasPwdSel) -> u8 {
        RefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwd1pt1 {
    #[doc = "Enable."]
    FsRxdiffEnable = 0x0,
    #[doc = "Disable or power down."]
    FsRxdiffPwd = 0x01,
}
impl Rxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> Rxpwd1pt1 {
        Rxpwd1pt1::from_bits(val)
    }
}
impl From<Rxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: Rxpwd1pt1) -> u8 {
        Rxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwddiff {
    #[doc = "Enable."]
    HsRxdiffEnable = 0x0,
    #[doc = "Disable or power down."]
    HsRxdiffPwd = 0x01,
}
impl Rxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> Rxpwddiff {
        Rxpwddiff::from_bits(val)
    }
}
impl From<Rxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: Rxpwddiff) -> u8 {
        Rxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwdenv {
    #[doc = "Enable."]
    RxEnvhdEnable = 0x0,
    #[doc = "Disable or power down."]
    RxEnvhdPwd = 0x01,
}
impl Rxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> Rxpwdenv {
        Rxpwdenv::from_bits(val)
    }
}
impl From<Rxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: Rxpwdenv) -> u8 {
        Rxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwdrx {
    #[doc = "Enable."]
    RxBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    RxBiasPwd = 0x01,
}
impl Rxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> Rxpwdrx {
        Rxpwdrx::from_bits(val)
    }
}
impl From<Rxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: Rxpwdrx) -> u8 {
        Rxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecdetDcp {
    #[doc = "CDP detected."]
    SecdetCdp = 0x0,
    #[doc = "DCP detected."]
    SecdetDcp = 0x01,
}
impl SecdetDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecdetDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecdetDcp {
    #[inline(always)]
    fn from(val: u8) -> SecdetDcp {
        SecdetDcp::from_bits(val)
    }
}
impl From<SecdetDcp> for u8 {
    #[inline(always)]
    fn from(val: SecdetDcp) -> u8 {
        SecdetDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sessend {
    #[doc = "Above threshold."]
    SessendLo = 0x0,
    #[doc = "Below threshold."]
    SessendHi = 0x01,
}
impl Sessend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sessend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sessend {
    #[inline(always)]
    fn from(val: u8) -> Sessend {
        Sessend::from_bits(val)
    }
}
impl From<Sessend> for u8 {
    #[inline(always)]
    fn from(val: Sessend) -> u8 {
        Sessend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sftrst {
    #[doc = "Release from reset."]
    ReleaseReset = 0x0,
    #[doc = "Soft-reset."]
    SoftReset = 0x01,
}
impl Sftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftrst {
    #[inline(always)]
    fn from(val: u8) -> Sftrst {
        Sftrst::from_bits(val)
    }
}
impl From<Sftrst> for u8 {
    #[inline(always)]
    fn from(val: Sftrst) -> u8 {
        Sftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCal45dmOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Cal45dn = 0x0,
    #[doc = "TX."]
    UseTxCal45dn = 0x01,
}
impl TxCal45dmOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCal45dmOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCal45dmOverride {
    #[inline(always)]
    fn from(val: u8) -> TxCal45dmOverride {
        TxCal45dmOverride::from_bits(val)
    }
}
impl From<TxCal45dmOverride> for u8 {
    #[inline(always)]
    fn from(val: TxCal45dmOverride) -> u8 {
        TxCal45dmOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCal45dpOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Cal45dp = 0x0,
    #[doc = "TX."]
    UseTxCal45dp = 0x01,
}
impl TxCal45dpOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCal45dpOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCal45dpOverride {
    #[inline(always)]
    fn from(val: u8) -> TxCal45dpOverride {
        TxCal45dpOverride::from_bits(val)
    }
}
impl From<TxCal45dpOverride> for u8 {
    #[inline(always)]
    fn from(val: TxCal45dpOverride) -> u8 {
        TxCal45dpOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxDCalOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Dcal = 0x0,
    #[doc = "TX."]
    UseTxDcal = 0x01,
}
impl TxDCalOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxDCalOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxDCalOverride {
    #[inline(always)]
    fn from(val: u8) -> TxDCalOverride {
        TxDCalOverride::from_bits(val)
    }
}
impl From<TxDCalOverride> for u8 {
    #[inline(always)]
    fn from(val: TxDCalOverride) -> u8 {
        TxDCalOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdfs {
    #[doc = "Provide bias to enable."]
    FstxBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    FstxBiasPwd = 0x01,
}
impl Txpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdfs {
    #[inline(always)]
    fn from(val: u8) -> Txpwdfs {
        Txpwdfs::from_bits(val)
    }
}
impl From<Txpwdfs> for u8 {
    #[inline(always)]
    fn from(val: Txpwdfs) -> u8 {
        Txpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdibias {
    #[doc = "Enable."]
    IbiasEnable = 0x0,
    #[doc = "Disable or power down."]
    IbiasPwd = 0x01,
}
impl Txpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdibias {
    #[inline(always)]
    fn from(val: u8) -> Txpwdibias {
        Txpwdibias::from_bits(val)
    }
}
impl From<Txpwdibias> for u8 {
    #[inline(always)]
    fn from(val: Txpwdibias) -> u8 {
        Txpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdv2i {
    #[doc = "Enable."]
    V2iBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    V2iBiasPwd = 0x01,
}
impl Txpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> Txpwdv2i {
        Txpwdv2i::from_bits(val)
    }
}
impl From<Txpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: Txpwdv2i) -> u8 {
        Txpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbphyTxDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MaxCurrent = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    Nominal = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MinCurrent = 0x0f,
}
impl UsbphyTxDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbphyTxDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbphyTxDCal {
    #[inline(always)]
    fn from(val: u8) -> UsbphyTxDCal {
        UsbphyTxDCal::from_bits(val)
    }
}
impl From<UsbphyTxDCal> for u8 {
    #[inline(always)]
    fn from(val: UsbphyTxDCal) -> u8 {
        UsbphyTxDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusOverrideEn {
    #[doc = "Results of VBUS_VALID and session valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VbusNoOverride = 0x0,
    #[doc = "Override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VbusOverride = 0x01,
}
impl VbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> VbusOverrideEn {
        VbusOverrideEn::from_bits(val)
    }
}
impl From<VbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: VbusOverrideEn) -> u8 {
        VbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSourceSel {
    #[doc = "VBUS_VALID comparator result."]
    UseVbusVld = 0x0,
    #[doc = "Session valid comparator result."]
    UseAsessVld = 0x01,
    #[doc = "Session valid comparator result."]
    UseBsessVld = 0x02,
    _RESERVED_3 = 0x03,
}
impl VbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> VbusSourceSel {
        VbusSourceSel::from_bits(val)
    }
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: VbusSourceSel) -> u8 {
        VbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid {
    #[doc = "Below threshold."]
    VbusLo = 0x0,
    #[doc = "Above threshold."]
    VbusHi = 0x01,
}
impl VbusValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid {
    #[inline(always)]
    fn from(val: u8) -> VbusValid {
        VbusValid::from_bits(val)
    }
}
impl From<VbusValid> for u8 {
    #[inline(always)]
    fn from(val: VbusValid) -> u8 {
        VbusValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid3v {
    #[doc = "Below threshold."]
    VbusVld3vLo = 0x0,
    #[doc = "Above threshold."]
    VbusVld3vHi = 0x01,
}
impl VbusValid3v {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid3v {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid3v {
    #[inline(always)]
    fn from(val: u8) -> VbusValid3v {
        VbusValid3v::from_bits(val)
    }
}
impl From<VbusValid3v> for u8 {
    #[inline(always)]
    fn from(val: VbusValid3v) -> u8 {
        VbusValid3v::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidPwrupCmps {
    #[doc = "Disable or power down the VBUS_VALID comparator."]
    VbusValidDisable = 0x0,
    #[doc = "Enable the VBUS_VALID comparator."]
    VbusValidEnable = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl VbusvalidPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidPwrupCmps {
        VbusvalidPwrupCmps::from_bits(val)
    }
}
impl From<VbusvalidPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidPwrupCmps) -> u8 {
        VbusvalidPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidSel {
    #[doc = "VBUS_VALID comparator result."]
    VbusVldOut = 0x0,
    #[doc = "VBUS_VALID_3V comparator result."]
    VbusVld3vOut = 0x01,
}
impl VbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidSel {
        VbusvalidSel::from_bits(val)
    }
}
impl From<VbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidSel) -> u8 {
        VbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidThresh {
    #[doc = "4.0 V."]
    VbusVld4p0 = 0x0,
    #[doc = "4.1 V."]
    VbusVld4p1 = 0x01,
    #[doc = "4.2 V."]
    VbusVld4p2 = 0x02,
    #[doc = "4.3 V."]
    VbusVld4p3 = 0x03,
    #[doc = "4.4 V."]
    VbusVld4p4 = 0x04,
    #[doc = "4.5 V."]
    VbusVld4p5 = 0x05,
    #[doc = "4.6 V."]
    VbusVld4p6 = 0x06,
    #[doc = "4.7 V."]
    VbusVld4p7 = 0x07,
}
impl VbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidThresh {
        VbusvalidThresh::from_bits(val)
    }
}
impl From<VbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidThresh) -> u8 {
        VbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidToB {
    #[doc = "VBUS_VALID comparator."]
    UseVbusVld = 0x0,
    #[doc = "Session valid detector."]
    UseSessVld = 0x01,
}
impl VbusvalidToB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidToB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidToB {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidToB {
        VbusvalidToB::from_bits(val)
    }
}
impl From<VbusvalidToB> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidToB) -> u8 {
        VbusvalidToB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdmSrcEnable {
    #[doc = "Disable."]
    DcdVdmSrcDisable = 0x0,
    #[doc = "Enable."]
    DcdVdmSrcEnable = 0x01,
}
impl VdmSrcEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdmSrcEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdmSrcEnable {
    #[inline(always)]
    fn from(val: u8) -> VdmSrcEnable {
        VdmSrcEnable::from_bits(val)
    }
}
impl From<VdmSrcEnable> for u8 {
    #[inline(always)]
    fn from(val: VdmSrcEnable) -> u8 {
        VdmSrcEnable::to_bits(val)
    }
}
