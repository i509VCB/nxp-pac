#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fauto {
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    MANUAL = 0x0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    AUTOMATIC = 0x01,
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
}
impl Fauto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fauto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fauto {
    #[inline(always)]
    fn from(val: u8) -> Fauto {
        Fauto::from_bits(val)
    }
}
impl From<Fauto> for u8 {
    #[inline(always)]
    fn from(val: Fauto) -> u8 {
        Fauto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fflag {
    #[doc = "No fault on the FAULTx pin."]
    NO_FLAG = 0x0,
    #[doc = "Fault on the FAULTx pin."]
    FLAG = 0x01,
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
}
impl Fflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fflag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fflag {
    #[inline(always)]
    fn from(val: u8) -> Fflag {
        Fflag::from_bits(val)
    }
}
impl From<Fflag> for u8 {
    #[inline(always)]
    fn from(val: Fflag) -> u8 {
        Fflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ffull {
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_NOT_REENABLED = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle"]
    PWM_OUTPUTS_REENABLED = 0x01,
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
}
impl Ffull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ffull {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ffull {
    #[inline(always)]
    fn from(val: u8) -> Ffull {
        Ffull::from_bits(val)
    }
}
impl From<Ffull> for u8 {
    #[inline(always)]
    fn from(val: Ffull) -> u8 {
        Ffull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fhalf {
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    PWM_OUTPUTS_NOT_REENABLED = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    PWM_OUTPUTS_REENABLED = 0x01,
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
}
impl Fhalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fhalf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fhalf {
    #[inline(always)]
    fn from(val: u8) -> Fhalf {
        Fhalf::from_bits(val)
    }
}
impl From<Fhalf> for u8 {
    #[inline(always)]
    fn from(val: Fhalf) -> u8 {
        Fhalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fie {
    #[doc = "FAULTx CPU interrupt requests disabled."]
    DISABLED = 0x0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
    ENABLED = 0x01,
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
}
impl Fie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fie {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fie {
    #[inline(always)]
    fn from(val: u8) -> Fie {
        Fie::from_bits(val)
    }
}
impl From<Fie> for u8 {
    #[inline(always)]
    fn from(val: Fie) -> u8 {
        Fie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flvl {
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    LOGIC_1 = 0x01,
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
}
impl Flvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flvl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flvl {
    #[inline(always)]
    fn from(val: u8) -> Flvl {
        Flvl::from_bits(val)
    }
}
impl From<Flvl> for u8 {
    #[inline(always)]
    fn from(val: Flvl) -> u8 {
        Flvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsafe {
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set, then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    NORMAL = 0x0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    SAFE = 0x01,
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
}
impl Fsafe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsafe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsafe {
    #[inline(always)]
    fn from(val: u8) -> Fsafe {
        Fsafe::from_bits(val)
    }
}
impl From<Fsafe> for u8 {
    #[inline(always)]
    fn from(val: Fsafe) -> u8 {
        Fsafe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipol {
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM23 = 0x0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    PWM45 = 0x01,
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
}
impl Ipol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipol {
    #[inline(always)]
    fn from(val: u8) -> Ipol {
        Ipol::from_bits(val)
    }
}
impl From<Ipol> for u8 {
    #[inline(always)]
    fn from(val: Ipol) -> u8 {
        Ipol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldok {
    #[doc = "Do not load new values."]
    DISABLED = 0x0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
    ENABLED = 0x01,
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
}
impl Ldok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldok {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldok {
    #[inline(always)]
    fn from(val: u8) -> Ldok {
        Ldok::from_bits(val)
    }
}
impl From<Ldok> for u8 {
    #[inline(always)]
    fn from(val: Ldok) -> u8 {
        Ldok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nocomb {
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    ENABLED = 0x0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    DISABLED = 0x01,
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
}
impl Nocomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nocomb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nocomb {
    #[inline(always)]
    fn from(val: u8) -> Nocomb {
        Nocomb::from_bits(val)
    }
}
impl From<Nocomb> for u8 {
    #[inline(always)]
    fn from(val: Nocomb) -> u8 {
        Nocomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "PWM counter is stopped, but PWM outputs hold the current state."]
    DISABLED = 0x0,
    #[doc = "PWM counter is started in the corresponding submodule."]
    ENABLED = 0x01,
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
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxEdgx {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl SmcaptctrlxEdgx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxEdgx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxEdgx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxEdgx {
        SmcaptctrlxEdgx::from_bits(val)
    }
}
impl From<SmcaptctrlxEdgx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxEdgx) -> u8 {
        SmcaptctrlxEdgx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl SmcaptctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxInpSelx {
        SmcaptctrlxInpSelx::from_bits(val)
    }
}
impl From<SmcaptctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxInpSelx) -> u8 {
        SmcaptctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxOneshotx {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl SmcaptctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxOneshotx {
        SmcaptctrlxOneshotx::from_bits(val)
    }
}
impl From<SmcaptctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxOneshotx) -> u8 {
        SmcaptctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmctrlClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlClkSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlClkSel {
        SmctrlClkSel::from_bits(val)
    }
}
impl From<SmctrlClkSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlClkSel) -> u8 {
        SmctrlClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl SmctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> SmctrlCompmode {
        SmctrlCompmode::from_bits(val)
    }
}
impl From<SmctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: SmctrlCompmode) -> u8 {
        SmctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    LOCAL = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LOCAL_RELOAD = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_RELOAD = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LOCAL_SYNC = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MASTER_SYNC = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    EXT_FORCE = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    EXT_SYNC = 0x07,
}
impl SmctrlForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlForceSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlForceSel {
        SmctrlForceSel::from_bits(val)
    }
}
impl From<SmctrlForceSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlForceSel) -> u8 {
        SmctrlForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlIndep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl SmctrlIndep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlIndep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlIndep {
    #[inline(always)]
    fn from(val: u8) -> SmctrlIndep {
        SmctrlIndep::from_bits(val)
    }
}
impl From<SmctrlIndep> for u8 {
    #[inline(always)]
    fn from(val: SmctrlIndep) -> u8 {
        SmctrlIndep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlInitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl SmctrlInitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlInitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlInitSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlInitSel {
        SmctrlInitSel::from_bits(val)
    }
}
impl From<SmctrlInitSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlInitSel) -> u8 {
        SmctrlInitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlLdfq {
    #[doc = "Every PWM opportunity"]
    EVERYPWM = 0x0,
    #[doc = "Every 2 PWM opportunities"]
    EVERY2PWM = 0x01,
    #[doc = "Every 3 PWM opportunities"]
    EVERY3PWM = 0x02,
    #[doc = "Every 4 PWM opportunities"]
    EVERY4PWM = 0x03,
    #[doc = "Every 5 PWM opportunities"]
    EVERY5PWM = 0x04,
    #[doc = "Every 6 PWM opportunities"]
    EVERY6PWM = 0x05,
    #[doc = "Every 7 PWM opportunities"]
    EVERY7PWM = 0x06,
    #[doc = "Every 8 PWM opportunities"]
    EVERY8PWM = 0x07,
    #[doc = "Every 9 PWM opportunities"]
    EVERY9PWM = 0x08,
    #[doc = "Every 10 PWM opportunities"]
    EVERY10PWM = 0x09,
    #[doc = "Every 11 PWM opportunities"]
    EVERY11PWM = 0x0a,
    #[doc = "Every 12 PWM opportunities"]
    EVERY12PWM = 0x0b,
    #[doc = "Every 13 PWM opportunities"]
    EVERY13PWM = 0x0c,
    #[doc = "Every 14 PWM opportunities"]
    EVERY14PWM = 0x0d,
    #[doc = "Every 15 PWM opportunities"]
    EVERY15PWM = 0x0e,
    #[doc = "Every 16 PWM opportunities"]
    EVERY16PWM = 0x0f,
}
impl SmctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> SmctrlLdfq {
        SmctrlLdfq::from_bits(val)
    }
}
impl From<SmctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: SmctrlLdfq) -> u8 {
        SmctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl SmctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> SmctrlLdmod {
        SmctrlLdmod::from_bits(val)
    }
}
impl From<SmctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: SmctrlLdmod) -> u8 {
        SmctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlPrsc {
    #[doc = "Prescaler 1"]
    ONE = 0x0,
    #[doc = "Prescaler 2"]
    TWO = 0x01,
    #[doc = "Prescaler 4"]
    FOUR = 0x02,
    #[doc = "Prescaler 8"]
    EIGHT = 0x03,
    #[doc = "Prescaler 16"]
    SIXTEEN = 0x04,
    #[doc = "Prescaler 32"]
    THIRTYTWO = 0x05,
    #[doc = "Prescaler 64"]
    SIXTYFOUR = 0x06,
    #[doc = "Prescaler 128"]
    HUNDREDTWENTYEIGHT = 0x07,
}
impl SmctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> SmctrlPrsc {
        SmctrlPrsc::from_bits(val)
    }
}
impl From<SmctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: SmctrlPrsc) -> u8 {
        SmctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl SmctrlReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlReloadSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlReloadSel {
        SmctrlReloadSel::from_bits(val)
    }
}
impl From<SmctrlReloadSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlReloadSel) -> u8 {
        SmctrlReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmdmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl SmdmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmdmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmdmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> SmdmaenCaptde {
        SmdmaenCaptde::from_bits(val)
    }
}
impl From<SmdmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: SmdmaenCaptde) -> u8 {
        SmdmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmdmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl SmdmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmdmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmdmaenFand {
    #[inline(always)]
    fn from(val: u8) -> SmdmaenFand {
        SmdmaenFand::from_bits(val)
    }
}
impl From<SmdmaenFand> for u8 {
    #[inline(always)]
    fn from(val: SmdmaenFand) -> u8 {
        SmdmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmintenCmpie {
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    DISABLED = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
    ENABLED = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl SmintenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmintenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmintenCmpie {
    #[inline(always)]
    fn from(val: u8) -> SmintenCmpie {
        SmintenCmpie::from_bits(val)
    }
}
impl From<SmintenCmpie> for u8 {
    #[inline(always)]
    fn from(val: SmintenCmpie) -> u8 {
        SmintenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl SmoctrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmafs {
        SmoctrlPwmafs::from_bits(val)
    }
}
impl From<SmoctrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmafs) -> u8 {
        SmoctrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl SmoctrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmbfs {
        SmoctrlPwmbfs::from_bits(val)
    }
}
impl From<SmoctrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmbfs) -> u8 {
        SmoctrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl SmoctrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmxfs {
        SmoctrlPwmxfs::from_bits(val)
    }
}
impl From<SmoctrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmxfs) -> u8 {
        SmoctrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smout {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Smout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smout {
    #[inline(always)]
    fn from(val: u8) -> Smout {
        Smout::from_bits(val)
    }
}
impl From<Smout> for u8 {
    #[inline(always)]
    fn from(val: Smout) -> u8 {
        Smout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smsel23 {
    #[doc = "Generated SM0PWM23 signal used by the deadtime logic."]
    PWM = 0x0,
    #[doc = "Inverted generated SM0PWM23 signal used by the deadtime logic."]
    INVERTED_PWM = 0x01,
    #[doc = "SWCOUT\\[SM0OUT23\\] used by the deadtime logic."]
    OUT = 0x02,
    #[doc = "PWM0_EXTA signal used by the deadtime logic."]
    PWM_EXTA = 0x03,
}
impl Smsel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smsel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smsel23 {
    #[inline(always)]
    fn from(val: u8) -> Smsel23 {
        Smsel23::from_bits(val)
    }
}
impl From<Smsel23> for u8 {
    #[inline(always)]
    fn from(val: Smsel23) -> u8 {
        Smsel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smsel45 {
    #[doc = "Generated SM0PWM45 signal used by the deadtime logic."]
    PWM = 0x0,
    #[doc = "Inverted generated SM0PWM45 signal used by the deadtime logic."]
    INVERTED_PWM = 0x01,
    #[doc = "SWCOUT\\[SM0OUT45\\] used by the deadtime logic."]
    OUT = 0x02,
    _RESERVED_3 = 0x03,
}
impl Smsel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smsel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smsel45 {
    #[inline(always)]
    fn from(val: u8) -> Smsel45 {
        Smsel45::from_bits(val)
    }
}
impl From<Smsel45> for u8 {
    #[inline(always)]
    fn from(val: Smsel45) -> u8 {
        Smsel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmstsCmpf {
    #[doc = "No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    EVENT = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl SmstsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmstsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmstsCmpf {
    #[inline(always)]
    fn from(val: u8) -> SmstsCmpf {
        SmstsCmpf::from_bits(val)
    }
}
impl From<SmstsCmpf> for u8 {
    #[inline(always)]
    fn from(val: SmstsCmpf) -> u8 {
        SmstsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    VAL0 = 0x01,
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
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl SmtctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlOutTrigEn {
        SmtctrlOutTrigEn::from_bits(val)
    }
}
impl From<SmtctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlOutTrigEn) -> u8 {
        SmtctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlPwaot {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl SmtctrlPwaot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlPwaot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlPwaot {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlPwaot {
        SmtctrlPwaot::from_bits(val)
    }
}
impl From<SmtctrlPwaot> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlPwaot) -> u8 {
        SmtctrlPwaot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlPwbot {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl SmtctrlPwbot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlPwbot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlPwbot {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlPwbot {
        SmtctrlPwbot::from_bits(val)
    }
}
impl From<SmtctrlPwbot> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlPwbot) -> u8 {
        SmtctrlPwbot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl SmtctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlTrgfrq {
        SmtctrlTrgfrq::from_bits(val)
    }
}
impl From<SmtctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlTrgfrq) -> u8 {
        SmtctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StretchCntPrsc {
    #[doc = "Stretch count is zero, no stretch."]
    DISABLED = 0x0,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 2 IPBus clock period."]
    ENABLED = 0x01,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 4 IPBus clock period."]
    DISABLED_LOCKED = 0x02,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 8 IPBus clock period."]
    ENABLED_LOCKED = 0x03,
}
impl StretchCntPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StretchCntPrsc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StretchCntPrsc {
    #[inline(always)]
    fn from(val: u8) -> StretchCntPrsc {
        StretchCntPrsc::from_bits(val)
    }
}
impl From<StretchCntPrsc> for u8 {
    #[inline(always)]
    fn from(val: StretchCntPrsc) -> u8 {
        StretchCntPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrprot {
    #[doc = "Write protection off (default)."]
    DISABLED = 0x0,
    #[doc = "Write protection on."]
    ENABLED = 0x01,
    #[doc = "Write protection off and locked until chip reset."]
    DISABLED_LOCKED = 0x02,
    #[doc = "Write protection on and locked until chip reset."]
    ENABLED_LOCKED = 0x03,
}
impl Wrprot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrprot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrprot {
    #[inline(always)]
    fn from(val: u8) -> Wrprot {
        Wrprot::from_bits(val)
    }
}
impl From<Wrprot> for u8 {
    #[inline(always)]
    fn from(val: Wrprot) -> u8 {
        Wrprot::to_bits(val)
    }
}
