#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcCocoClear {
    #[doc = "No ADC COCO clear"]
    ADC_COCO_CLEAR_0 = 0x0,
    #[doc = "Set ADC COCO clear"]
    ADC_COCO_CLEAR_1 = 0x01,
}
impl AdcCocoClear {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcCocoClear {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcCocoClear {
    #[inline(always)]
    fn from(val: u8) -> AdcCocoClear {
        AdcCocoClear::from_bits(val)
    }
}
impl From<AdcCocoClear> for u8 {
    #[inline(always)]
    fn from(val: AdcCocoClear) -> u8 {
        AdcCocoClear::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcCocoClearDisable {
    #[doc = "Allow TSC hardware generates ADC COCO clear"]
    ADC_COCO_CLEAR_DISABLE_0 = 0x0,
    #[doc = "Prevent TSC from generate ADC COCO clear signal"]
    ADC_COCO_CLEAR_DISABLE_1 = 0x01,
}
impl AdcCocoClearDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcCocoClearDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcCocoClearDisable {
    #[inline(always)]
    fn from(val: u8) -> AdcCocoClearDisable {
        AdcCocoClearDisable::from_bits(val)
    }
}
impl From<AdcCocoClearDisable> for u8 {
    #[inline(always)]
    fn from(val: AdcCocoClearDisable) -> u8 {
        AdcCocoClearDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoMeasure {
    #[doc = "Disable Auto Measure"]
    AUTO_MEASURE_0 = 0x0,
    #[doc = "Auto Measure"]
    AUTO_MEASURE_1 = 0x01,
}
impl AutoMeasure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoMeasure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoMeasure {
    #[inline(always)]
    fn from(val: u8) -> AutoMeasure {
        AutoMeasure::from_bits(val)
    }
}
impl From<AutoMeasure> for u8 {
    #[inline(always)]
    fn from(val: AutoMeasure) -> u8 {
        AutoMeasure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeGlitch {
    #[doc = "Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
    DE_GLITCH_0 = 0x0,
    #[doc = "Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
    DE_GLITCH_1 = 0x01,
    #[doc = "Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
    DE_GLITCH_2 = 0x02,
    #[doc = "Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
    DE_GLITCH_3 = 0x03,
}
impl DeGlitch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeGlitch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeGlitch {
    #[inline(always)]
    fn from(val: u8) -> DeGlitch {
        DeGlitch::from_bits(val)
    }
}
impl From<DeGlitch> for u8 {
    #[inline(always)]
    fn from(val: DeGlitch) -> u8 {
        DeGlitch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugEn {
    #[doc = "Enable debug mode"]
    DEBUG_EN_0 = 0x0,
    #[doc = "Disable debug mode"]
    DEBUG_EN_1 = 0x01,
}
impl DebugEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugEn {
    #[inline(always)]
    fn from(val: u8) -> DebugEn {
        DebugEn::from_bits(val)
    }
}
impl From<DebugEn> for u8 {
    #[inline(always)]
    fn from(val: DebugEn) -> u8 {
        DebugEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Detect {
    #[doc = "Does not exist a detect signal"]
    DETECT_0 = 0x0,
    #[doc = "Exist detect signal"]
    DETECT_1 = 0x01,
}
impl Detect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Detect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Detect {
    #[inline(always)]
    fn from(val: u8) -> Detect {
        Detect::from_bits(val)
    }
}
impl From<Detect> for u8 {
    #[inline(always)]
    fn from(val: Detect) -> u8 {
        Detect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectEnableFiveWire {
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    DETECT_ENABLE_FIVE_WIRE_0 = 0x0,
    #[doc = "Read five wire detect status from analogue"]
    DETECT_ENABLE_FIVE_WIRE_1 = 0x01,
}
impl DetectEnableFiveWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectEnableFiveWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectEnableFiveWire {
    #[inline(always)]
    fn from(val: u8) -> DetectEnableFiveWire {
        DetectEnableFiveWire::from_bits(val)
    }
}
impl From<DetectEnableFiveWire> for u8 {
    #[inline(always)]
    fn from(val: DetectEnableFiveWire) -> u8 {
        DetectEnableFiveWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectEnableFourWire {
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    DETECT_ENABLE_FOUR_WIRE_0 = 0x0,
    #[doc = "Read four wire detect status from analogue"]
    DETECT_ENABLE_FOUR_WIRE_1 = 0x01,
}
impl DetectEnableFourWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectEnableFourWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectEnableFourWire {
    #[inline(always)]
    fn from(val: u8) -> DetectEnableFourWire {
        DetectEnableFourWire::from_bits(val)
    }
}
impl From<DetectEnableFourWire> for u8 {
    #[inline(always)]
    fn from(val: DetectEnableFourWire) -> u8 {
        DetectEnableFourWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectFiveWire {
    #[doc = "No detect signal"]
    DETECT_FIVE_WIRE_0 = 0x0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FIVE_WIRE_1 = 0x01,
}
impl DetectFiveWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectFiveWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectFiveWire {
    #[inline(always)]
    fn from(val: u8) -> DetectFiveWire {
        DetectFiveWire::from_bits(val)
    }
}
impl From<DetectFiveWire> for u8 {
    #[inline(always)]
    fn from(val: DetectFiveWire) -> u8 {
        DetectFiveWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectFourWire {
    #[doc = "No detect signal"]
    DETECT_FOUR_WIRE_0 = 0x0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FOUR_WIRE_1 = 0x01,
}
impl DetectFourWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectFourWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectFourWire {
    #[inline(always)]
    fn from(val: u8) -> DetectFourWire {
        DetectFourWire::from_bits(val)
    }
}
impl From<DetectFourWire> for u8 {
    #[inline(always)]
    fn from(val: DetectFourWire) -> u8 {
        DetectFourWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectIntEn {
    #[doc = "Disable detect interrupt"]
    DETECT_INT_EN_0 = 0x0,
    #[doc = "Enable detect interrupt"]
    DETECT_INT_EN_1 = 0x01,
}
impl DetectIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectIntEn {
    #[inline(always)]
    fn from(val: u8) -> DetectIntEn {
        DetectIntEn::from_bits(val)
    }
}
impl From<DetectIntEn> for u8 {
    #[inline(always)]
    fn from(val: DetectIntEn) -> u8 {
        DetectIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectSigEn {
    #[doc = "Disable detect signal"]
    DETECT_SIG_EN_0 = 0x0,
    #[doc = "Enable detect signal"]
    DETECT_SIG_EN_1 = 0x01,
}
impl DetectSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectSigEn {
    #[inline(always)]
    fn from(val: u8) -> DetectSigEn {
        DetectSigEn::from_bits(val)
    }
}
impl From<DetectSigEn> for u8 {
    #[inline(always)]
    fn from(val: DetectSigEn) -> u8 {
        DetectSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disable {
    #[doc = "Leave HW state machine control"]
    DISABLE_0 = 0x0,
    #[doc = "SW set to idle status"]
    DISABLE_1 = 0x01,
}
impl Disable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disable {
    #[inline(always)]
    fn from(val: u8) -> Disable {
        Disable::from_bits(val)
    }
}
impl From<Disable> for u8 {
    #[inline(always)]
    fn from(val: Disable) -> u8 {
        Disable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DropMeasure {
    #[doc = "Do not drop measure for now"]
    DROP_MEASURE_0 = 0x0,
    #[doc = "Drop the measure and controller return to idle status"]
    DROP_MEASURE_1 = 0x01,
}
impl DropMeasure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DropMeasure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DropMeasure {
    #[inline(always)]
    fn from(val: u8) -> DropMeasure {
        DropMeasure::from_bits(val)
    }
}
impl From<DropMeasure> for u8 {
    #[inline(always)]
    fn from(val: DropMeasure) -> u8 {
        DropMeasure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdleSw {
    #[doc = "Haven't return to idle status"]
    IDLE_SW_0 = 0x0,
    #[doc = "Already return to idle status"]
    IDLE_SW_1 = 0x01,
}
impl IdleSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdleSw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdleSw {
    #[inline(always)]
    fn from(val: u8) -> IdleSw {
        IdleSw::from_bits(val)
    }
}
impl From<IdleSw> for u8 {
    #[inline(always)]
    fn from(val: IdleSw) -> u8 {
        IdleSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdleSwIntEn {
    #[doc = "Disable idle software interrupt"]
    IDLE_SW_INT_EN_0 = 0x0,
    #[doc = "Enable idle software interrupt"]
    IDLE_SW_INT_EN_1 = 0x01,
}
impl IdleSwIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdleSwIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdleSwIntEn {
    #[inline(always)]
    fn from(val: u8) -> IdleSwIntEn {
        IdleSwIntEn::from_bits(val)
    }
}
impl From<IdleSwIntEn> for u8 {
    #[inline(always)]
    fn from(val: IdleSwIntEn) -> u8 {
        IdleSwIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdleSwSigEn {
    #[doc = "Disable idle software signal"]
    IDLE_SW_SIG_EN_0 = 0x0,
    #[doc = "Enable idle software signal"]
    IDLE_SW_SIG_EN_1 = 0x01,
}
impl IdleSwSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdleSwSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdleSwSigEn {
    #[inline(always)]
    fn from(val: u8) -> IdleSwSigEn {
        IdleSwSigEn::from_bits(val)
    }
}
impl From<IdleSwSigEn> for u8 {
    #[inline(always)]
    fn from(val: IdleSwSigEn) -> u8 {
        IdleSwSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Intermediate {
    #[doc = "Not in intermedia"]
    INTERMEDIATE_0 = 0x0,
    #[doc = "Intermedia"]
    INTERMEDIATE_1 = 0x01,
}
impl Intermediate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intermediate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intermediate {
    #[inline(always)]
    fn from(val: u8) -> Intermediate {
        Intermediate::from_bits(val)
    }
}
impl From<Intermediate> for u8 {
    #[inline(always)]
    fn from(val: Intermediate) -> u8 {
        Intermediate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Measure {
    #[doc = "Does not exist a measure signal"]
    MEASURE_0 = 0x0,
    #[doc = "Exist a measure signal"]
    MEASURE_1 = 0x01,
}
impl Measure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Measure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Measure {
    #[inline(always)]
    fn from(val: u8) -> Measure {
        Measure::from_bits(val)
    }
}
impl From<Measure> for u8 {
    #[inline(always)]
    fn from(val: Measure) -> u8 {
        Measure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MeasureIntEn {
    #[doc = "Disable measure interrupt"]
    MEASURE_INT_EN_0 = 0x0,
    #[doc = "Enable measure interrupt"]
    MEASURE_INT_EN_1 = 0x01,
}
impl MeasureIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MeasureIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MeasureIntEn {
    #[inline(always)]
    fn from(val: u8) -> MeasureIntEn {
        MeasureIntEn::from_bits(val)
    }
}
impl From<MeasureIntEn> for u8 {
    #[inline(always)]
    fn from(val: MeasureIntEn) -> u8 {
        MeasureIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StartMeasure {
    #[doc = "Do not start measure for now"]
    START_MEASURE_0 = 0x0,
    #[doc = "Start measure the X/Y coordinate value"]
    START_MEASURE_1 = 0x01,
}
impl StartMeasure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartMeasure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartMeasure {
    #[inline(always)]
    fn from(val: u8) -> StartMeasure {
        StartMeasure::from_bits(val)
    }
}
impl From<StartMeasure> for u8 {
    #[inline(always)]
    fn from(val: StartMeasure) -> u8 {
        StartMeasure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StartSense {
    #[doc = "Stay at idle status"]
    START_SENSE_0 = 0x0,
    #[doc = "Start sense detection and (if auto_measure set to 1) measure after detect a touch"]
    START_SENSE_1 = 0x01,
}
impl StartSense {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StartSense {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StartSense {
    #[inline(always)]
    fn from(val: u8) -> StartSense {
        StartSense::from_bits(val)
    }
}
impl From<StartSense> for u8 {
    #[inline(always)]
    fn from(val: StartSense) -> u8 {
        StartSense::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateMachine {
    #[doc = "Idle"]
    STATE_MACHINE_0 = 0x0,
    #[doc = "Pre-charge"]
    STATE_MACHINE_1 = 0x01,
    #[doc = "Detect"]
    STATE_MACHINE_2 = 0x02,
    #[doc = "X-measure"]
    STATE_MACHINE_3 = 0x03,
    #[doc = "Y-measure"]
    STATE_MACHINE_4 = 0x04,
    #[doc = "Pre-charge"]
    STATE_MACHINE_5 = 0x05,
    #[doc = "Detect"]
    STATE_MACHINE_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl StateMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StateMachine {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StateMachine {
    #[inline(always)]
    fn from(val: u8) -> StateMachine {
        StateMachine::from_bits(val)
    }
}
impl From<StateMachine> for u8 {
    #[inline(always)]
    fn from(val: StateMachine) -> u8 {
        StateMachine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigger {
    #[doc = "No hardware trigger signal"]
    TRIGGER_0 = 0x0,
    #[doc = "Hardware trigger signal, the signal must last at least 1 ips clock period"]
    TRIGGER_1 = 0x01,
}
impl Trigger {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigger {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigger {
    #[inline(always)]
    fn from(val: u8) -> Trigger {
        Trigger::from_bits(val)
    }
}
impl From<Trigger> for u8 {
    #[inline(always)]
    fn from(val: Trigger) -> u8 {
        Trigger::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Valid {
    #[doc = "There is no touch detected after measurement, indicates that the measured value is not valid"]
    VALID_0 = 0x0,
    #[doc = "There is touch detection after measurement, indicates that the measure is valid"]
    VALID_1 = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ValidSigEn {
    #[doc = "Disable valid signal"]
    VALID_SIG_EN_0 = 0x0,
    #[doc = "Enable valid signal"]
    VALID_SIG_EN_1 = 0x01,
}
impl ValidSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ValidSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ValidSigEn {
    #[inline(always)]
    fn from(val: u8) -> ValidSigEn {
        ValidSigEn::from_bits(val)
    }
}
impl From<ValidSigEn> for u8 {
    #[inline(always)]
    fn from(val: ValidSigEn) -> u8 {
        ValidSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wiper200kPullUp {
    #[doc = "Close the switch"]
    WIPER_200K_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    WIPER_200K_PULL_UP_1 = 0x01,
}
impl Wiper200kPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wiper200kPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wiper200kPullUp {
    #[inline(always)]
    fn from(val: u8) -> Wiper200kPullUp {
        Wiper200kPullUp::from_bits(val)
    }
}
impl From<Wiper200kPullUp> for u8 {
    #[inline(always)]
    fn from(val: Wiper200kPullUp) -> u8 {
        Wiper200kPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WiperPullDown {
    #[doc = "Close the switch"]
    WIPER_PULL_DOWN_0 = 0x0,
    #[doc = "Open up the switch"]
    WIPER_PULL_DOWN_1 = 0x01,
}
impl WiperPullDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WiperPullDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WiperPullDown {
    #[inline(always)]
    fn from(val: u8) -> WiperPullDown {
        WiperPullDown::from_bits(val)
    }
}
impl From<WiperPullDown> for u8 {
    #[inline(always)]
    fn from(val: WiperPullDown) -> u8 {
        WiperPullDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WiperPullUp {
    #[doc = "Close the switch"]
    WIPER_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    WIPER_PULL_UP_1 = 0x01,
}
impl WiperPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WiperPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WiperPullUp {
    #[inline(always)]
    fn from(val: u8) -> WiperPullUp {
        WiperPullUp::from_bits(val)
    }
}
impl From<WiperPullUp> for u8 {
    #[inline(always)]
    fn from(val: WiperPullUp) -> u8 {
        WiperPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wire45 {
    #[doc = "4-Wire Detection Mode"]
    WIRE_4_5_0 = 0x0,
    #[doc = "5-Wire Detection Mode"]
    WIRE_4_5_1 = 0x01,
}
impl Wire45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wire45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wire45 {
    #[inline(always)]
    fn from(val: u8) -> Wire45 {
        Wire45::from_bits(val)
    }
}
impl From<Wire45> for u8 {
    #[inline(always)]
    fn from(val: Wire45) -> u8 {
        Wire45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xnur200kPullUp {
    #[doc = "Close the switch"]
    XNUR_200K_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    XNUR_200K_PULL_UP_1 = 0x01,
}
impl Xnur200kPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xnur200kPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xnur200kPullUp {
    #[inline(always)]
    fn from(val: u8) -> Xnur200kPullUp {
        Xnur200kPullUp::from_bits(val)
    }
}
impl From<Xnur200kPullUp> for u8 {
    #[inline(always)]
    fn from(val: Xnur200kPullUp) -> u8 {
        Xnur200kPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XnurPullDown {
    #[doc = "Close the switch"]
    XNUR_PULL_DOWN_0 = 0x0,
    #[doc = "Open up the switch"]
    XNUR_PULL_DOWN_1 = 0x01,
}
impl XnurPullDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XnurPullDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XnurPullDown {
    #[inline(always)]
    fn from(val: u8) -> XnurPullDown {
        XnurPullDown::from_bits(val)
    }
}
impl From<XnurPullDown> for u8 {
    #[inline(always)]
    fn from(val: XnurPullDown) -> u8 {
        XnurPullDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XnurPullUp {
    #[doc = "Close the switch"]
    XNUR_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    XNUR_PULL_UP_1 = 0x01,
}
impl XnurPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XnurPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XnurPullUp {
    #[inline(always)]
    fn from(val: u8) -> XnurPullUp {
        XnurPullUp::from_bits(val)
    }
}
impl From<XnurPullUp> for u8 {
    #[inline(always)]
    fn from(val: XnurPullUp) -> u8 {
        XnurPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xpul200kPullUp {
    #[doc = "Close the switch"]
    XPUL_200K_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    XPUL_200K_PULL_UP_1 = 0x01,
}
impl Xpul200kPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xpul200kPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xpul200kPullUp {
    #[inline(always)]
    fn from(val: u8) -> Xpul200kPullUp {
        Xpul200kPullUp::from_bits(val)
    }
}
impl From<Xpul200kPullUp> for u8 {
    #[inline(always)]
    fn from(val: Xpul200kPullUp) -> u8 {
        Xpul200kPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XpulPullDown {
    #[doc = "Close the switch"]
    XPUL_PULL_DOWN_0 = 0x0,
    #[doc = "Open up the switch"]
    XPUL_PULL_DOWN_1 = 0x01,
}
impl XpulPullDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XpulPullDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XpulPullDown {
    #[inline(always)]
    fn from(val: u8) -> XpulPullDown {
        XpulPullDown::from_bits(val)
    }
}
impl From<XpulPullDown> for u8 {
    #[inline(always)]
    fn from(val: XpulPullDown) -> u8 {
        XpulPullDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XpulPullUp {
    #[doc = "Close the switch"]
    XPUL_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    XPUL_PULL_UP_1 = 0x01,
}
impl XpulPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XpulPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XpulPullUp {
    #[inline(always)]
    fn from(val: u8) -> XpulPullUp {
        XpulPullUp::from_bits(val)
    }
}
impl From<XpulPullUp> for u8 {
    #[inline(always)]
    fn from(val: XpulPullUp) -> u8 {
        XpulPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ynlr200kPullUp {
    #[doc = "Close the switch"]
    YNLR_200K_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    YNLR_200K_PULL_UP_1 = 0x01,
}
impl Ynlr200kPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ynlr200kPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ynlr200kPullUp {
    #[inline(always)]
    fn from(val: u8) -> Ynlr200kPullUp {
        Ynlr200kPullUp::from_bits(val)
    }
}
impl From<Ynlr200kPullUp> for u8 {
    #[inline(always)]
    fn from(val: Ynlr200kPullUp) -> u8 {
        Ynlr200kPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YnlrPullDown {
    #[doc = "Close the switch"]
    YNLR_PULL_DOWN_0 = 0x0,
    #[doc = "Open up the switch"]
    YNLR_PULL_DOWN_1 = 0x01,
}
impl YnlrPullDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> YnlrPullDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for YnlrPullDown {
    #[inline(always)]
    fn from(val: u8) -> YnlrPullDown {
        YnlrPullDown::from_bits(val)
    }
}
impl From<YnlrPullDown> for u8 {
    #[inline(always)]
    fn from(val: YnlrPullDown) -> u8 {
        YnlrPullDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YnlrPullUp {
    #[doc = "Close the switch"]
    YNLR_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    YNLR_PULL_UP_1 = 0x01,
}
impl YnlrPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> YnlrPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for YnlrPullUp {
    #[inline(always)]
    fn from(val: u8) -> YnlrPullUp {
        YnlrPullUp::from_bits(val)
    }
}
impl From<YnlrPullUp> for u8 {
    #[inline(always)]
    fn from(val: YnlrPullUp) -> u8 {
        YnlrPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ypll200kPullUp {
    #[doc = "Close the switch"]
    YPLL_200K_PULL_UP_0 = 0x0,
    #[doc = "Open up the switch"]
    YPLL_200K_PULL_UP_1 = 0x01,
}
impl Ypll200kPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ypll200kPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ypll200kPullUp {
    #[inline(always)]
    fn from(val: u8) -> Ypll200kPullUp {
        Ypll200kPullUp::from_bits(val)
    }
}
impl From<Ypll200kPullUp> for u8 {
    #[inline(always)]
    fn from(val: Ypll200kPullUp) -> u8 {
        Ypll200kPullUp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YpllPullDown {
    #[doc = "Close the switch"]
    YPLL_PULL_DOWN_0 = 0x0,
    #[doc = "Open up the switch"]
    YPLL_PULL_DOWN_1 = 0x01,
}
impl YpllPullDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> YpllPullDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for YpllPullDown {
    #[inline(always)]
    fn from(val: u8) -> YpllPullDown {
        YpllPullDown::from_bits(val)
    }
}
impl From<YpllPullDown> for u8 {
    #[inline(always)]
    fn from(val: YpllPullDown) -> u8 {
        YpllPullDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YpllPullUp {
    #[doc = "Close the switch"]
    YPLL_PULL_UP_0 = 0x0,
    #[doc = "Open the switch"]
    YPLL_PULL_UP_1 = 0x01,
}
impl YpllPullUp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> YpllPullUp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for YpllPullUp {
    #[inline(always)]
    fn from(val: u8) -> YpllPullUp {
        YpllPullUp::from_bits(val)
    }
}
impl From<YpllPullUp> for u8 {
    #[inline(always)]
    fn from(val: YpllPullUp) -> u8 {
        YpllPullUp::to_bits(val)
    }
}
