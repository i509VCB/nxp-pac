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
pub enum Ftest {
    #[doc = "No fault"]
    NO_FAULT = 0x0,
    #[doc = "Cause a simulated fault"]
    FAULT = 0x01,
}
impl Ftest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ftest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ftest {
    #[inline(always)]
    fn from(val: u8) -> Ftest {
        Ftest::from_bits(val)
    }
}
impl From<Ftest> for u8 {
    #[inline(always)]
    fn from(val: Ftest) -> u8 {
        Ftest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gstr {
    #[doc = "Fault input glitch stretching is disabled."]
    DISABLED = 0x0,
    #[doc = "Input fault signals are stretched to at least 2 IPBus clock cycles."]
    ENABLED = 0x01,
}
impl Gstr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gstr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gstr {
    #[inline(always)]
    fn from(val: u8) -> Gstr {
        Gstr::from_bits(val)
    }
}
impl From<Gstr> for u8 {
    #[inline(always)]
    fn from(val: Gstr) -> u8 {
        Gstr::to_bits(val)
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
pub enum Sm0captctrlaArma {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm0captctrlaArma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaArma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaArma {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaArma {
        Sm0captctrlaArma::from_bits(val)
    }
}
impl From<Sm0captctrlaArma> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaArma) -> u8 {
        Sm0captctrlaArma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlaEdga0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlaEdga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaEdga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaEdga0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaEdga0 {
        Sm0captctrlaEdga0::from_bits(val)
    }
}
impl From<Sm0captctrlaEdga0> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaEdga0) -> u8 {
        Sm0captctrlaEdga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlaEdga1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlaEdga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaEdga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaEdga1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaEdga1 {
        Sm0captctrlaEdga1::from_bits(val)
    }
}
impl From<Sm0captctrlaEdga1> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaEdga1) -> u8 {
        Sm0captctrlaEdga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlaEdgcntaEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm0captctrlaEdgcntaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaEdgcntaEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaEdgcntaEn {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaEdgcntaEn {
        Sm0captctrlaEdgcntaEn::from_bits(val)
    }
}
impl From<Sm0captctrlaEdgcntaEn> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaEdgcntaEn) -> u8 {
        Sm0captctrlaEdgcntaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlaInpSela {
    #[doc = "Raw PWM_A input signal selected as source."]
    PWM_A = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm0captctrlaInpSela {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaInpSela {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaInpSela {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaInpSela {
        Sm0captctrlaInpSela::from_bits(val)
    }
}
impl From<Sm0captctrlaInpSela> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaInpSela) -> u8 {
        Sm0captctrlaInpSela::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlaOneshota {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm0captctrlaOneshota {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlaOneshota {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlaOneshota {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlaOneshota {
        Sm0captctrlaOneshota::from_bits(val)
    }
}
impl From<Sm0captctrlaOneshota> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlaOneshota) -> u8 {
        Sm0captctrlaOneshota::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbArmb {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm0captctrlbArmb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbArmb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbArmb {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbArmb {
        Sm0captctrlbArmb::from_bits(val)
    }
}
impl From<Sm0captctrlbArmb> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbArmb) -> u8 {
        Sm0captctrlbArmb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbEdgb0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlbEdgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbEdgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbEdgb0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbEdgb0 {
        Sm0captctrlbEdgb0::from_bits(val)
    }
}
impl From<Sm0captctrlbEdgb0> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbEdgb0) -> u8 {
        Sm0captctrlbEdgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbEdgb1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlbEdgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbEdgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbEdgb1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbEdgb1 {
        Sm0captctrlbEdgb1::from_bits(val)
    }
}
impl From<Sm0captctrlbEdgb1> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbEdgb1) -> u8 {
        Sm0captctrlbEdgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbEdgcntbEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm0captctrlbEdgcntbEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbEdgcntbEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbEdgcntbEn {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbEdgcntbEn {
        Sm0captctrlbEdgcntbEn::from_bits(val)
    }
}
impl From<Sm0captctrlbEdgcntbEn> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbEdgcntbEn) -> u8 {
        Sm0captctrlbEdgcntbEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbInpSelb {
    #[doc = "Raw PWM_B input signal selected as source."]
    PWM_B = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm0captctrlbInpSelb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbInpSelb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbInpSelb {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbInpSelb {
        Sm0captctrlbInpSelb::from_bits(val)
    }
}
impl From<Sm0captctrlbInpSelb> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbInpSelb) -> u8 {
        Sm0captctrlbInpSelb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlbOneshotb {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm0captctrlbOneshotb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlbOneshotb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlbOneshotb {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlbOneshotb {
        Sm0captctrlbOneshotb::from_bits(val)
    }
}
impl From<Sm0captctrlbOneshotb> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlbOneshotb) -> u8 {
        Sm0captctrlbOneshotb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxArmx {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm0captctrlxArmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxArmx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxArmx {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxArmx {
        Sm0captctrlxArmx::from_bits(val)
    }
}
impl From<Sm0captctrlxArmx> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxArmx) -> u8 {
        Sm0captctrlxArmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxEdgcntxEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm0captctrlxEdgcntxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxEdgcntxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxEdgcntxEn {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxEdgcntxEn {
        Sm0captctrlxEdgcntxEn::from_bits(val)
    }
}
impl From<Sm0captctrlxEdgcntxEn> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxEdgcntxEn) -> u8 {
        Sm0captctrlxEdgcntxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxEdgx0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxEdgx0 {
        Sm0captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm0captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxEdgx0) -> u8 {
        Sm0captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxEdgx1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm0captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxEdgx1 {
        Sm0captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm0captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxEdgx1) -> u8 {
        Sm0captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm0captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxInpSelx {
        Sm0captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm0captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxInpSelx) -> u8 {
        Sm0captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0captctrlxOneshotx {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm0captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm0captctrlxOneshotx {
        Sm0captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm0captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm0captctrlxOneshotx) -> u8 {
        Sm0captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm0ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ClkSel {
        Sm0ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ClkSel) -> u8 {
        Sm0ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ForceSel {
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
impl Sm0ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ForceSel {
        Sm0ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ForceSel) -> u8 {
        Sm0ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2Frcen {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    DISABLED = 0x0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    ENABLED = 0x01,
}
impl Sm0ctrl2Frcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2Frcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2Frcen {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2Frcen {
        Sm0ctrl2Frcen::from_bits(val)
    }
}
impl From<Sm0ctrl2Frcen> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2Frcen) -> u8 {
        Sm0ctrl2Frcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm0ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2Indep {
        Sm0ctrl2Indep::from_bits(val)
    }
}
impl From<Sm0ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2Indep) -> u8 {
        Sm0ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm0ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2InitSel {
        Sm0ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm0ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2InitSel) -> u8 {
        Sm0ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm0ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrl2ReloadSel {
        Sm0ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm0ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrl2ReloadSel) -> u8 {
        Sm0ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm0ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlCompmode {
        Sm0ctrlCompmode::from_bits(val)
    }
}
impl From<Sm0ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlCompmode) -> u8 {
        Sm0ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlDblen {
    #[doc = "Double switching disabled."]
    DISABLED = 0x0,
    #[doc = "Double switching enabled."]
    ENABLED = 0x01,
}
impl Sm0ctrlDblen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlDblen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlDblen {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlDblen {
        Sm0ctrlDblen::from_bits(val)
    }
}
impl From<Sm0ctrlDblen> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlDblen) -> u8 {
        Sm0ctrlDblen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlDblx {
    #[doc = "PWM_X double pulse disabled."]
    DISABLED = 0x0,
    #[doc = "PWM_X double pulse enabled."]
    ENABLED = 0x01,
}
impl Sm0ctrlDblx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlDblx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlDblx {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlDblx {
        Sm0ctrlDblx::from_bits(val)
    }
}
impl From<Sm0ctrlDblx> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlDblx) -> u8 {
        Sm0ctrlDblx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlFull {
    #[doc = "Full-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Full-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm0ctrlFull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlFull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlFull {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlFull {
        Sm0ctrlFull::from_bits(val)
    }
}
impl From<Sm0ctrlFull> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlFull) -> u8 {
        Sm0ctrlFull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlHalf {
    #[doc = "Half-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Half-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm0ctrlHalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlHalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlHalf {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlHalf {
        Sm0ctrlHalf::from_bits(val)
    }
}
impl From<Sm0ctrlHalf> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlHalf) -> u8 {
        Sm0ctrlHalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlLdfq {
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
impl Sm0ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlLdfq {
        Sm0ctrlLdfq::from_bits(val)
    }
}
impl From<Sm0ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlLdfq) -> u8 {
        Sm0ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm0ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlLdmod {
        Sm0ctrlLdmod::from_bits(val)
    }
}
impl From<Sm0ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlLdmod) -> u8 {
        Sm0ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlPrsc {
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
impl Sm0ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlPrsc {
        Sm0ctrlPrsc::from_bits(val)
    }
}
impl From<Sm0ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlPrsc) -> u8 {
        Sm0ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0ctrlSplit {
    #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
    DISABLED = 0x0,
    #[doc = "DBLPWM is split to PWM_A and PWM_B."]
    ENABLED = 0x01,
}
impl Sm0ctrlSplit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0ctrlSplit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0ctrlSplit {
    #[inline(always)]
    fn from(val: u8) -> Sm0ctrlSplit {
        Sm0ctrlSplit::from_bits(val)
    }
}
impl From<Sm0ctrlSplit> for u8 {
    #[inline(always)]
    fn from(val: Sm0ctrlSplit) -> u8 {
        Sm0ctrlSplit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm0dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm0dmaenCaptde {
        Sm0dmaenCaptde::from_bits(val)
    }
}
impl From<Sm0dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm0dmaenCaptde) -> u8 {
        Sm0dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm0dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm0dmaenFand {
        Sm0dmaenFand::from_bits(val)
    }
}
impl From<Sm0dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm0dmaenFand) -> u8 {
        Sm0dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0dmaenValde {
    #[doc = "DMA write requests disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Sm0dmaenValde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0dmaenValde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0dmaenValde {
    #[inline(always)]
    fn from(val: u8) -> Sm0dmaenValde {
        Sm0dmaenValde::from_bits(val)
    }
}
impl From<Sm0dmaenValde> for u8 {
    #[inline(always)]
    fn from(val: Sm0dmaenValde) -> u8 {
        Sm0dmaenValde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0frctrlFrac1En {
    #[doc = "Disable fractional cycle length for the PWM period."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    ENABLED = 0x01,
}
impl Sm0frctrlFrac1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0frctrlFrac1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0frctrlFrac1En {
    #[inline(always)]
    fn from(val: u8) -> Sm0frctrlFrac1En {
        Sm0frctrlFrac1En::from_bits(val)
    }
}
impl From<Sm0frctrlFrac1En> for u8 {
    #[inline(always)]
    fn from(val: Sm0frctrlFrac1En) -> u8 {
        Sm0frctrlFrac1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0frctrlFrac23En {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    ENABLED = 0x01,
}
impl Sm0frctrlFrac23En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0frctrlFrac23En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0frctrlFrac23En {
    #[inline(always)]
    fn from(val: u8) -> Sm0frctrlFrac23En {
        Sm0frctrlFrac23En::from_bits(val)
    }
}
impl From<Sm0frctrlFrac23En> for u8 {
    #[inline(always)]
    fn from(val: Sm0frctrlFrac23En) -> u8 {
        Sm0frctrlFrac23En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0frctrlFrac45En {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    ENABLED = 0x01,
}
impl Sm0frctrlFrac45En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0frctrlFrac45En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0frctrlFrac45En {
    #[inline(always)]
    fn from(val: u8) -> Sm0frctrlFrac45En {
        Sm0frctrlFrac45En::from_bits(val)
    }
}
impl From<Sm0frctrlFrac45En> for u8 {
    #[inline(always)]
    fn from(val: Sm0frctrlFrac45En) -> u8 {
        Sm0frctrlFrac45En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCa0ie {
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    ENABLED = 0x01,
}
impl Sm0intenCa0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCa0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCa0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCa0ie {
        Sm0intenCa0ie::from_bits(val)
    }
}
impl From<Sm0intenCa0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCa0ie) -> u8 {
        Sm0intenCa0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCa1ie {
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
    ENABLED = 0x01,
}
impl Sm0intenCa1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCa1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCa1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCa1ie {
        Sm0intenCa1ie::from_bits(val)
    }
}
impl From<Sm0intenCa1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCa1ie) -> u8 {
        Sm0intenCa1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCb0ie {
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    ENABLED = 0x01,
}
impl Sm0intenCb0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCb0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCb0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCb0ie {
        Sm0intenCb0ie::from_bits(val)
    }
}
impl From<Sm0intenCb0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCb0ie) -> u8 {
        Sm0intenCb0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCb1ie {
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    ENABLED = 0x01,
}
impl Sm0intenCb1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCb1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCb1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCb1ie {
        Sm0intenCb1ie::from_bits(val)
    }
}
impl From<Sm0intenCb1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCb1ie) -> u8 {
        Sm0intenCb1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCmpie {
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
impl Sm0intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCmpie {
        Sm0intenCmpie::from_bits(val)
    }
}
impl From<Sm0intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCmpie) -> u8 {
        Sm0intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCx0ie {
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    ENABLED = 0x01,
}
impl Sm0intenCx0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCx0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCx0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCx0ie {
        Sm0intenCx0ie::from_bits(val)
    }
}
impl From<Sm0intenCx0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCx0ie) -> u8 {
        Sm0intenCx0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenCx1ie {
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    ENABLED = 0x01,
}
impl Sm0intenCx1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenCx1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenCx1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenCx1ie {
        Sm0intenCx1ie::from_bits(val)
    }
}
impl From<Sm0intenCx1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenCx1ie) -> u8 {
        Sm0intenCx1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenReie {
    #[doc = "STS\\[REF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm0intenReie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenReie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenReie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenReie {
        Sm0intenReie::from_bits(val)
    }
}
impl From<Sm0intenReie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenReie) -> u8 {
        Sm0intenReie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0intenRie {
    #[doc = "STS\\[RF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm0intenRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0intenRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0intenRie {
    #[inline(always)]
    fn from(val: u8) -> Sm0intenRie {
        Sm0intenRie::from_bits(val)
    }
}
impl From<Sm0intenRie> for u8 {
    #[inline(always)]
    fn from(val: Sm0intenRie) -> u8 {
        Sm0intenRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPola {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm0octrlPola {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPola {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPola {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPola {
        Sm0octrlPola::from_bits(val)
    }
}
impl From<Sm0octrlPola> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPola) -> u8 {
        Sm0octrlPola::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPolb {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm0octrlPolb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPolb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPolb {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPolb {
        Sm0octrlPolb::from_bits(val)
    }
}
impl From<Sm0octrlPolb> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPolb) -> u8 {
        Sm0octrlPolb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPolx {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm0octrlPolx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPolx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPolx {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPolx {
        Sm0octrlPolx::from_bits(val)
    }
}
impl From<Sm0octrlPolx> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPolx) -> u8 {
        Sm0octrlPolx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmafs {
        Sm0octrlPwmafs::from_bits(val)
    }
}
impl From<Sm0octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmafs) -> u8 {
        Sm0octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmbfs {
        Sm0octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm0octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmbfs) -> u8 {
        Sm0octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm0octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm0octrlPwmxfs {
        Sm0octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm0octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm0octrlPwmxfs) -> u8 {
        Sm0octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm0out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm0out23 {
        Sm0out23::from_bits(val)
    }
}
impl From<Sm0out23> for u8 {
    #[inline(always)]
    fn from(val: Sm0out23) -> u8 {
        Sm0out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm0out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm0out45 {
        Sm0out45::from_bits(val)
    }
}
impl From<Sm0out45> for u8 {
    #[inline(always)]
    fn from(val: Sm0out45) -> u8 {
        Sm0out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0sel23 {
    #[doc = "Generated SM0PWM23 signal used by the deadtime logic."]
    SM0PWM23 = 0x0,
    #[doc = "Inverted generated SM0PWM23 signal used by the deadtime logic."]
    INVERTED_SM0PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT23\\] used by the deadtime logic."]
    SM0OUT23 = 0x02,
    #[doc = "PWM0_EXTA signal used by the deadtime logic."]
    PWM0_EXTA = 0x03,
}
impl Sm0sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel23 {
        Sm0sel23::from_bits(val)
    }
}
impl From<Sm0sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel23) -> u8 {
        Sm0sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0sel45 {
    #[doc = "Generated SM0PWM45 signal used by the deadtime logic."]
    SM0PWM45 = 0x0,
    #[doc = "Inverted generated SM0PWM45 signal used by the deadtime logic."]
    INVERTED_SM0PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM0OUT45\\] used by the deadtime logic."]
    SM0OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm0sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm0sel45 {
        Sm0sel45::from_bits(val)
    }
}
impl From<Sm0sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm0sel45) -> u8 {
        Sm0sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0stsCmpf {
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
impl Sm0stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm0stsCmpf {
        Sm0stsCmpf::from_bits(val)
    }
}
impl From<Sm0stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm0stsCmpf) -> u8 {
        Sm0stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0stsRef {
    #[doc = "No reload error occurred."]
    NO_FLAG = 0x0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    FLAG = 0x01,
}
impl Sm0stsRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0stsRef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0stsRef {
    #[inline(always)]
    fn from(val: u8) -> Sm0stsRef {
        Sm0stsRef::from_bits(val)
    }
}
impl From<Sm0stsRef> for u8 {
    #[inline(always)]
    fn from(val: Sm0stsRef) -> u8 {
        Sm0stsRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0stsRf {
    #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
    NO_FLAG = 0x0,
    #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
    FLAG = 0x01,
}
impl Sm0stsRf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0stsRf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0stsRf {
    #[inline(always)]
    fn from(val: u8) -> Sm0stsRf {
        Sm0stsRf::from_bits(val)
    }
}
impl From<Sm0stsRf> for u8 {
    #[inline(always)]
    fn from(val: Sm0stsRf) -> u8 {
        Sm0stsRf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0stsRuf {
    #[doc = "No register update has occurred since last reload."]
    NO_FLAG = 0x0,
    #[doc = "At least one of the double buffered registers has been updated since the last reload."]
    FLAG = 0x01,
}
impl Sm0stsRuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0stsRuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0stsRuf {
    #[inline(always)]
    fn from(val: u8) -> Sm0stsRuf {
        Sm0stsRuf::from_bits(val)
    }
}
impl From<Sm0stsRuf> for u8 {
    #[inline(always)]
    fn from(val: Sm0stsRuf) -> u8 {
        Sm0stsRuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlOutTrigEn {
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
impl Sm0tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlOutTrigEn {
        Sm0tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm0tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlOutTrigEn) -> u8 {
        Sm0tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm0tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlPwaot0 {
        Sm0tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm0tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlPwaot0) -> u8 {
        Sm0tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm0tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlPwbot1 {
        Sm0tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm0tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlPwbot1) -> u8 {
        Sm0tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm0tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm0tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm0tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm0tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm0tctrlTrgfrq {
        Sm0tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm0tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm0tctrlTrgfrq) -> u8 {
        Sm0tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaArma {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm1captctrlaArma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaArma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaArma {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaArma {
        Sm1captctrlaArma::from_bits(val)
    }
}
impl From<Sm1captctrlaArma> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaArma) -> u8 {
        Sm1captctrlaArma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaEdga0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlaEdga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaEdga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaEdga0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaEdga0 {
        Sm1captctrlaEdga0::from_bits(val)
    }
}
impl From<Sm1captctrlaEdga0> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaEdga0) -> u8 {
        Sm1captctrlaEdga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaEdga1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlaEdga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaEdga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaEdga1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaEdga1 {
        Sm1captctrlaEdga1::from_bits(val)
    }
}
impl From<Sm1captctrlaEdga1> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaEdga1) -> u8 {
        Sm1captctrlaEdga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaEdgcntaEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm1captctrlaEdgcntaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaEdgcntaEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaEdgcntaEn {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaEdgcntaEn {
        Sm1captctrlaEdgcntaEn::from_bits(val)
    }
}
impl From<Sm1captctrlaEdgcntaEn> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaEdgcntaEn) -> u8 {
        Sm1captctrlaEdgcntaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaInpSela {
    #[doc = "Raw PWM_A input signal selected as source."]
    PWM_A = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm1captctrlaInpSela {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaInpSela {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaInpSela {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaInpSela {
        Sm1captctrlaInpSela::from_bits(val)
    }
}
impl From<Sm1captctrlaInpSela> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaInpSela) -> u8 {
        Sm1captctrlaInpSela::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlaOneshota {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm1captctrlaOneshota {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlaOneshota {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlaOneshota {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlaOneshota {
        Sm1captctrlaOneshota::from_bits(val)
    }
}
impl From<Sm1captctrlaOneshota> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlaOneshota) -> u8 {
        Sm1captctrlaOneshota::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbArmb {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm1captctrlbArmb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbArmb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbArmb {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbArmb {
        Sm1captctrlbArmb::from_bits(val)
    }
}
impl From<Sm1captctrlbArmb> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbArmb) -> u8 {
        Sm1captctrlbArmb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbEdgb0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlbEdgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbEdgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbEdgb0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbEdgb0 {
        Sm1captctrlbEdgb0::from_bits(val)
    }
}
impl From<Sm1captctrlbEdgb0> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbEdgb0) -> u8 {
        Sm1captctrlbEdgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbEdgb1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlbEdgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbEdgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbEdgb1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbEdgb1 {
        Sm1captctrlbEdgb1::from_bits(val)
    }
}
impl From<Sm1captctrlbEdgb1> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbEdgb1) -> u8 {
        Sm1captctrlbEdgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbEdgcntbEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm1captctrlbEdgcntbEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbEdgcntbEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbEdgcntbEn {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbEdgcntbEn {
        Sm1captctrlbEdgcntbEn::from_bits(val)
    }
}
impl From<Sm1captctrlbEdgcntbEn> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbEdgcntbEn) -> u8 {
        Sm1captctrlbEdgcntbEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbInpSelb {
    #[doc = "Raw PWM_B input signal selected as source."]
    PWM_B = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm1captctrlbInpSelb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbInpSelb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbInpSelb {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbInpSelb {
        Sm1captctrlbInpSelb::from_bits(val)
    }
}
impl From<Sm1captctrlbInpSelb> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbInpSelb) -> u8 {
        Sm1captctrlbInpSelb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlbOneshotb {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm1captctrlbOneshotb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlbOneshotb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlbOneshotb {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlbOneshotb {
        Sm1captctrlbOneshotb::from_bits(val)
    }
}
impl From<Sm1captctrlbOneshotb> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlbOneshotb) -> u8 {
        Sm1captctrlbOneshotb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxArmx {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm1captctrlxArmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxArmx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxArmx {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxArmx {
        Sm1captctrlxArmx::from_bits(val)
    }
}
impl From<Sm1captctrlxArmx> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxArmx) -> u8 {
        Sm1captctrlxArmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxEdgcntxEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm1captctrlxEdgcntxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxEdgcntxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxEdgcntxEn {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxEdgcntxEn {
        Sm1captctrlxEdgcntxEn::from_bits(val)
    }
}
impl From<Sm1captctrlxEdgcntxEn> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxEdgcntxEn) -> u8 {
        Sm1captctrlxEdgcntxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxEdgx0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxEdgx0 {
        Sm1captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm1captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxEdgx0) -> u8 {
        Sm1captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxEdgx1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm1captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxEdgx1 {
        Sm1captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm1captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxEdgx1) -> u8 {
        Sm1captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm1captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxInpSelx {
        Sm1captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm1captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxInpSelx) -> u8 {
        Sm1captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1captctrlxOneshotx {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm1captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm1captctrlxOneshotx {
        Sm1captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm1captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm1captctrlxOneshotx) -> u8 {
        Sm1captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm1ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ClkSel {
        Sm1ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ClkSel) -> u8 {
        Sm1ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ForceSel {
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
impl Sm1ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ForceSel {
        Sm1ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ForceSel) -> u8 {
        Sm1ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2Frcen {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    DISABLED = 0x0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    ENABLED = 0x01,
}
impl Sm1ctrl2Frcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2Frcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2Frcen {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2Frcen {
        Sm1ctrl2Frcen::from_bits(val)
    }
}
impl From<Sm1ctrl2Frcen> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2Frcen) -> u8 {
        Sm1ctrl2Frcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm1ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2Indep {
        Sm1ctrl2Indep::from_bits(val)
    }
}
impl From<Sm1ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2Indep) -> u8 {
        Sm1ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm1ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2InitSel {
        Sm1ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm1ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2InitSel) -> u8 {
        Sm1ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm1ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrl2ReloadSel {
        Sm1ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm1ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrl2ReloadSel) -> u8 {
        Sm1ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm1ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlCompmode {
        Sm1ctrlCompmode::from_bits(val)
    }
}
impl From<Sm1ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlCompmode) -> u8 {
        Sm1ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlDblen {
    #[doc = "Double switching disabled."]
    DISABLED = 0x0,
    #[doc = "Double switching enabled."]
    ENABLED = 0x01,
}
impl Sm1ctrlDblen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlDblen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlDblen {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlDblen {
        Sm1ctrlDblen::from_bits(val)
    }
}
impl From<Sm1ctrlDblen> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlDblen) -> u8 {
        Sm1ctrlDblen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlDblx {
    #[doc = "PWM_X double pulse disabled."]
    DISABLED = 0x0,
    #[doc = "PWM_X double pulse enabled."]
    ENABLED = 0x01,
}
impl Sm1ctrlDblx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlDblx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlDblx {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlDblx {
        Sm1ctrlDblx::from_bits(val)
    }
}
impl From<Sm1ctrlDblx> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlDblx) -> u8 {
        Sm1ctrlDblx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlFull {
    #[doc = "Full-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Full-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm1ctrlFull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlFull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlFull {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlFull {
        Sm1ctrlFull::from_bits(val)
    }
}
impl From<Sm1ctrlFull> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlFull) -> u8 {
        Sm1ctrlFull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlHalf {
    #[doc = "Half-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Half-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm1ctrlHalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlHalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlHalf {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlHalf {
        Sm1ctrlHalf::from_bits(val)
    }
}
impl From<Sm1ctrlHalf> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlHalf) -> u8 {
        Sm1ctrlHalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlLdfq {
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
impl Sm1ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlLdfq {
        Sm1ctrlLdfq::from_bits(val)
    }
}
impl From<Sm1ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlLdfq) -> u8 {
        Sm1ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm1ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlLdmod {
        Sm1ctrlLdmod::from_bits(val)
    }
}
impl From<Sm1ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlLdmod) -> u8 {
        Sm1ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlPrsc {
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
impl Sm1ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlPrsc {
        Sm1ctrlPrsc::from_bits(val)
    }
}
impl From<Sm1ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlPrsc) -> u8 {
        Sm1ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1ctrlSplit {
    #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
    DISABLED = 0x0,
    #[doc = "DBLPWM is split to PWM_A and PWM_B."]
    ENABLED = 0x01,
}
impl Sm1ctrlSplit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1ctrlSplit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1ctrlSplit {
    #[inline(always)]
    fn from(val: u8) -> Sm1ctrlSplit {
        Sm1ctrlSplit::from_bits(val)
    }
}
impl From<Sm1ctrlSplit> for u8 {
    #[inline(always)]
    fn from(val: Sm1ctrlSplit) -> u8 {
        Sm1ctrlSplit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm1dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm1dmaenCaptde {
        Sm1dmaenCaptde::from_bits(val)
    }
}
impl From<Sm1dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm1dmaenCaptde) -> u8 {
        Sm1dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm1dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm1dmaenFand {
        Sm1dmaenFand::from_bits(val)
    }
}
impl From<Sm1dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm1dmaenFand) -> u8 {
        Sm1dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1dmaenValde {
    #[doc = "DMA write requests disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Sm1dmaenValde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1dmaenValde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1dmaenValde {
    #[inline(always)]
    fn from(val: u8) -> Sm1dmaenValde {
        Sm1dmaenValde::from_bits(val)
    }
}
impl From<Sm1dmaenValde> for u8 {
    #[inline(always)]
    fn from(val: Sm1dmaenValde) -> u8 {
        Sm1dmaenValde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1frctrlFrac1En {
    #[doc = "Disable fractional cycle length for the PWM period."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    ENABLED = 0x01,
}
impl Sm1frctrlFrac1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1frctrlFrac1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1frctrlFrac1En {
    #[inline(always)]
    fn from(val: u8) -> Sm1frctrlFrac1En {
        Sm1frctrlFrac1En::from_bits(val)
    }
}
impl From<Sm1frctrlFrac1En> for u8 {
    #[inline(always)]
    fn from(val: Sm1frctrlFrac1En) -> u8 {
        Sm1frctrlFrac1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1frctrlFrac23En {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    ENABLED = 0x01,
}
impl Sm1frctrlFrac23En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1frctrlFrac23En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1frctrlFrac23En {
    #[inline(always)]
    fn from(val: u8) -> Sm1frctrlFrac23En {
        Sm1frctrlFrac23En::from_bits(val)
    }
}
impl From<Sm1frctrlFrac23En> for u8 {
    #[inline(always)]
    fn from(val: Sm1frctrlFrac23En) -> u8 {
        Sm1frctrlFrac23En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1frctrlFrac45En {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    ENABLED = 0x01,
}
impl Sm1frctrlFrac45En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1frctrlFrac45En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1frctrlFrac45En {
    #[inline(always)]
    fn from(val: u8) -> Sm1frctrlFrac45En {
        Sm1frctrlFrac45En::from_bits(val)
    }
}
impl From<Sm1frctrlFrac45En> for u8 {
    #[inline(always)]
    fn from(val: Sm1frctrlFrac45En) -> u8 {
        Sm1frctrlFrac45En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCa0ie {
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    ENABLED = 0x01,
}
impl Sm1intenCa0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCa0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCa0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCa0ie {
        Sm1intenCa0ie::from_bits(val)
    }
}
impl From<Sm1intenCa0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCa0ie) -> u8 {
        Sm1intenCa0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCa1ie {
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
    ENABLED = 0x01,
}
impl Sm1intenCa1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCa1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCa1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCa1ie {
        Sm1intenCa1ie::from_bits(val)
    }
}
impl From<Sm1intenCa1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCa1ie) -> u8 {
        Sm1intenCa1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCb0ie {
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    ENABLED = 0x01,
}
impl Sm1intenCb0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCb0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCb0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCb0ie {
        Sm1intenCb0ie::from_bits(val)
    }
}
impl From<Sm1intenCb0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCb0ie) -> u8 {
        Sm1intenCb0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCb1ie {
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    ENABLED = 0x01,
}
impl Sm1intenCb1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCb1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCb1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCb1ie {
        Sm1intenCb1ie::from_bits(val)
    }
}
impl From<Sm1intenCb1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCb1ie) -> u8 {
        Sm1intenCb1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCmpie {
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
impl Sm1intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCmpie {
        Sm1intenCmpie::from_bits(val)
    }
}
impl From<Sm1intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCmpie) -> u8 {
        Sm1intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCx0ie {
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    ENABLED = 0x01,
}
impl Sm1intenCx0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCx0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCx0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCx0ie {
        Sm1intenCx0ie::from_bits(val)
    }
}
impl From<Sm1intenCx0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCx0ie) -> u8 {
        Sm1intenCx0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenCx1ie {
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    ENABLED = 0x01,
}
impl Sm1intenCx1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenCx1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenCx1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenCx1ie {
        Sm1intenCx1ie::from_bits(val)
    }
}
impl From<Sm1intenCx1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenCx1ie) -> u8 {
        Sm1intenCx1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenReie {
    #[doc = "STS\\[REF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm1intenReie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenReie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenReie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenReie {
        Sm1intenReie::from_bits(val)
    }
}
impl From<Sm1intenReie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenReie) -> u8 {
        Sm1intenReie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1intenRie {
    #[doc = "STS\\[RF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm1intenRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1intenRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1intenRie {
    #[inline(always)]
    fn from(val: u8) -> Sm1intenRie {
        Sm1intenRie::from_bits(val)
    }
}
impl From<Sm1intenRie> for u8 {
    #[inline(always)]
    fn from(val: Sm1intenRie) -> u8 {
        Sm1intenRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPola {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm1octrlPola {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPola {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPola {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPola {
        Sm1octrlPola::from_bits(val)
    }
}
impl From<Sm1octrlPola> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPola) -> u8 {
        Sm1octrlPola::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPolb {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm1octrlPolb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPolb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPolb {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPolb {
        Sm1octrlPolb::from_bits(val)
    }
}
impl From<Sm1octrlPolb> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPolb) -> u8 {
        Sm1octrlPolb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPolx {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm1octrlPolx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPolx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPolx {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPolx {
        Sm1octrlPolx::from_bits(val)
    }
}
impl From<Sm1octrlPolx> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPolx) -> u8 {
        Sm1octrlPolx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmafs {
        Sm1octrlPwmafs::from_bits(val)
    }
}
impl From<Sm1octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmafs) -> u8 {
        Sm1octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmbfs {
        Sm1octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm1octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmbfs) -> u8 {
        Sm1octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm1octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm1octrlPwmxfs {
        Sm1octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm1octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm1octrlPwmxfs) -> u8 {
        Sm1octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm1out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm1out23 {
        Sm1out23::from_bits(val)
    }
}
impl From<Sm1out23> for u8 {
    #[inline(always)]
    fn from(val: Sm1out23) -> u8 {
        Sm1out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 1 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm1out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm1out45 {
        Sm1out45::from_bits(val)
    }
}
impl From<Sm1out45> for u8 {
    #[inline(always)]
    fn from(val: Sm1out45) -> u8 {
        Sm1out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1sel23 {
    #[doc = "Generated SM1PWM23 signal used by the deadtime logic."]
    SM1PWM23 = 0x0,
    #[doc = "Inverted generated SM1PWM23 signal used by the deadtime logic."]
    INVERTED_SM1PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT23\\] used by the deadtime logic."]
    SM1OUT23 = 0x02,
    #[doc = "PWM1_EXTA signal used by the deadtime logic."]
    PWM1_EXTA = 0x03,
}
impl Sm1sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel23 {
        Sm1sel23::from_bits(val)
    }
}
impl From<Sm1sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel23) -> u8 {
        Sm1sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1sel45 {
    #[doc = "Generated SM1PWM45 signal used by the deadtime logic."]
    SM1PWM45 = 0x0,
    #[doc = "Inverted generated SM1PWM45 signal used by the deadtime logic."]
    INVERTED_SM1PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM1OUT45\\] used by the deadtime logic."]
    SM1OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm1sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm1sel45 {
        Sm1sel45::from_bits(val)
    }
}
impl From<Sm1sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm1sel45) -> u8 {
        Sm1sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1stsCmpf {
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
impl Sm1stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm1stsCmpf {
        Sm1stsCmpf::from_bits(val)
    }
}
impl From<Sm1stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm1stsCmpf) -> u8 {
        Sm1stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1stsRef {
    #[doc = "No reload error occurred."]
    NO_FLAG = 0x0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    FLAG = 0x01,
}
impl Sm1stsRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1stsRef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1stsRef {
    #[inline(always)]
    fn from(val: u8) -> Sm1stsRef {
        Sm1stsRef::from_bits(val)
    }
}
impl From<Sm1stsRef> for u8 {
    #[inline(always)]
    fn from(val: Sm1stsRef) -> u8 {
        Sm1stsRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1stsRf {
    #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
    NO_FLAG = 0x0,
    #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
    FLAG = 0x01,
}
impl Sm1stsRf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1stsRf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1stsRf {
    #[inline(always)]
    fn from(val: u8) -> Sm1stsRf {
        Sm1stsRf::from_bits(val)
    }
}
impl From<Sm1stsRf> for u8 {
    #[inline(always)]
    fn from(val: Sm1stsRf) -> u8 {
        Sm1stsRf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1stsRuf {
    #[doc = "No register update has occurred since last reload."]
    NO_FLAG = 0x0,
    #[doc = "At least one of the double buffered registers has been updated since the last reload."]
    FLAG = 0x01,
}
impl Sm1stsRuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1stsRuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1stsRuf {
    #[inline(always)]
    fn from(val: u8) -> Sm1stsRuf {
        Sm1stsRuf::from_bits(val)
    }
}
impl From<Sm1stsRuf> for u8 {
    #[inline(always)]
    fn from(val: Sm1stsRuf) -> u8 {
        Sm1stsRuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlOutTrigEn {
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
impl Sm1tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlOutTrigEn {
        Sm1tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm1tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlOutTrigEn) -> u8 {
        Sm1tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm1tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlPwaot0 {
        Sm1tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm1tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlPwaot0) -> u8 {
        Sm1tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm1tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlPwbot1 {
        Sm1tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm1tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlPwbot1) -> u8 {
        Sm1tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm1tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm1tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm1tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm1tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm1tctrlTrgfrq {
        Sm1tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm1tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm1tctrlTrgfrq) -> u8 {
        Sm1tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaArma {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm2captctrlaArma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaArma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaArma {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaArma {
        Sm2captctrlaArma::from_bits(val)
    }
}
impl From<Sm2captctrlaArma> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaArma) -> u8 {
        Sm2captctrlaArma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaEdga0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlaEdga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaEdga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaEdga0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaEdga0 {
        Sm2captctrlaEdga0::from_bits(val)
    }
}
impl From<Sm2captctrlaEdga0> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaEdga0) -> u8 {
        Sm2captctrlaEdga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaEdga1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlaEdga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaEdga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaEdga1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaEdga1 {
        Sm2captctrlaEdga1::from_bits(val)
    }
}
impl From<Sm2captctrlaEdga1> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaEdga1) -> u8 {
        Sm2captctrlaEdga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaEdgcntaEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm2captctrlaEdgcntaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaEdgcntaEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaEdgcntaEn {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaEdgcntaEn {
        Sm2captctrlaEdgcntaEn::from_bits(val)
    }
}
impl From<Sm2captctrlaEdgcntaEn> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaEdgcntaEn) -> u8 {
        Sm2captctrlaEdgcntaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaInpSela {
    #[doc = "Raw PWM_A input signal selected as source."]
    PWM_A = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm2captctrlaInpSela {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaInpSela {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaInpSela {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaInpSela {
        Sm2captctrlaInpSela::from_bits(val)
    }
}
impl From<Sm2captctrlaInpSela> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaInpSela) -> u8 {
        Sm2captctrlaInpSela::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlaOneshota {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm2captctrlaOneshota {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlaOneshota {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlaOneshota {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlaOneshota {
        Sm2captctrlaOneshota::from_bits(val)
    }
}
impl From<Sm2captctrlaOneshota> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlaOneshota) -> u8 {
        Sm2captctrlaOneshota::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbArmb {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm2captctrlbArmb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbArmb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbArmb {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbArmb {
        Sm2captctrlbArmb::from_bits(val)
    }
}
impl From<Sm2captctrlbArmb> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbArmb) -> u8 {
        Sm2captctrlbArmb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbEdgb0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlbEdgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbEdgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbEdgb0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbEdgb0 {
        Sm2captctrlbEdgb0::from_bits(val)
    }
}
impl From<Sm2captctrlbEdgb0> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbEdgb0) -> u8 {
        Sm2captctrlbEdgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbEdgb1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlbEdgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbEdgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbEdgb1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbEdgb1 {
        Sm2captctrlbEdgb1::from_bits(val)
    }
}
impl From<Sm2captctrlbEdgb1> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbEdgb1) -> u8 {
        Sm2captctrlbEdgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbEdgcntbEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm2captctrlbEdgcntbEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbEdgcntbEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbEdgcntbEn {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbEdgcntbEn {
        Sm2captctrlbEdgcntbEn::from_bits(val)
    }
}
impl From<Sm2captctrlbEdgcntbEn> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbEdgcntbEn) -> u8 {
        Sm2captctrlbEdgcntbEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbInpSelb {
    #[doc = "Raw PWM_B input signal selected as source."]
    PWM_B = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm2captctrlbInpSelb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbInpSelb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbInpSelb {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbInpSelb {
        Sm2captctrlbInpSelb::from_bits(val)
    }
}
impl From<Sm2captctrlbInpSelb> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbInpSelb) -> u8 {
        Sm2captctrlbInpSelb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlbOneshotb {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm2captctrlbOneshotb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlbOneshotb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlbOneshotb {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlbOneshotb {
        Sm2captctrlbOneshotb::from_bits(val)
    }
}
impl From<Sm2captctrlbOneshotb> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlbOneshotb) -> u8 {
        Sm2captctrlbOneshotb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxArmx {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm2captctrlxArmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxArmx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxArmx {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxArmx {
        Sm2captctrlxArmx::from_bits(val)
    }
}
impl From<Sm2captctrlxArmx> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxArmx) -> u8 {
        Sm2captctrlxArmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxEdgcntxEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm2captctrlxEdgcntxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxEdgcntxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxEdgcntxEn {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxEdgcntxEn {
        Sm2captctrlxEdgcntxEn::from_bits(val)
    }
}
impl From<Sm2captctrlxEdgcntxEn> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxEdgcntxEn) -> u8 {
        Sm2captctrlxEdgcntxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxEdgx0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxEdgx0 {
        Sm2captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm2captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxEdgx0) -> u8 {
        Sm2captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxEdgx1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm2captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxEdgx1 {
        Sm2captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm2captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxEdgx1) -> u8 {
        Sm2captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm2captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxInpSelx {
        Sm2captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm2captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxInpSelx) -> u8 {
        Sm2captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2captctrlxOneshotx {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm2captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm2captctrlxOneshotx {
        Sm2captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm2captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm2captctrlxOneshotx) -> u8 {
        Sm2captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm2ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ClkSel {
        Sm2ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ClkSel) -> u8 {
        Sm2ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ForceSel {
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
impl Sm2ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ForceSel {
        Sm2ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ForceSel) -> u8 {
        Sm2ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2Frcen {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    DISABLED = 0x0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    ENABLED = 0x01,
}
impl Sm2ctrl2Frcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2Frcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2Frcen {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2Frcen {
        Sm2ctrl2Frcen::from_bits(val)
    }
}
impl From<Sm2ctrl2Frcen> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2Frcen) -> u8 {
        Sm2ctrl2Frcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm2ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2Indep {
        Sm2ctrl2Indep::from_bits(val)
    }
}
impl From<Sm2ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2Indep) -> u8 {
        Sm2ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm2ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2InitSel {
        Sm2ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm2ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2InitSel) -> u8 {
        Sm2ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm2ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrl2ReloadSel {
        Sm2ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm2ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrl2ReloadSel) -> u8 {
        Sm2ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm2ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlCompmode {
        Sm2ctrlCompmode::from_bits(val)
    }
}
impl From<Sm2ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlCompmode) -> u8 {
        Sm2ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlDblen {
    #[doc = "Double switching disabled."]
    DISABLED = 0x0,
    #[doc = "Double switching enabled."]
    ENABLED = 0x01,
}
impl Sm2ctrlDblen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlDblen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlDblen {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlDblen {
        Sm2ctrlDblen::from_bits(val)
    }
}
impl From<Sm2ctrlDblen> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlDblen) -> u8 {
        Sm2ctrlDblen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlDblx {
    #[doc = "PWM_X double pulse disabled."]
    DISABLED = 0x0,
    #[doc = "PWM_X double pulse enabled."]
    ENABLED = 0x01,
}
impl Sm2ctrlDblx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlDblx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlDblx {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlDblx {
        Sm2ctrlDblx::from_bits(val)
    }
}
impl From<Sm2ctrlDblx> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlDblx) -> u8 {
        Sm2ctrlDblx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlFull {
    #[doc = "Full-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Full-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm2ctrlFull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlFull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlFull {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlFull {
        Sm2ctrlFull::from_bits(val)
    }
}
impl From<Sm2ctrlFull> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlFull) -> u8 {
        Sm2ctrlFull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlHalf {
    #[doc = "Half-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Half-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm2ctrlHalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlHalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlHalf {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlHalf {
        Sm2ctrlHalf::from_bits(val)
    }
}
impl From<Sm2ctrlHalf> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlHalf) -> u8 {
        Sm2ctrlHalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlLdfq {
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
impl Sm2ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlLdfq {
        Sm2ctrlLdfq::from_bits(val)
    }
}
impl From<Sm2ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlLdfq) -> u8 {
        Sm2ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm2ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlLdmod {
        Sm2ctrlLdmod::from_bits(val)
    }
}
impl From<Sm2ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlLdmod) -> u8 {
        Sm2ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlPrsc {
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
impl Sm2ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlPrsc {
        Sm2ctrlPrsc::from_bits(val)
    }
}
impl From<Sm2ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlPrsc) -> u8 {
        Sm2ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2ctrlSplit {
    #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
    DISABLED = 0x0,
    #[doc = "DBLPWM is split to PWM_A and PWM_B."]
    ENABLED = 0x01,
}
impl Sm2ctrlSplit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2ctrlSplit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2ctrlSplit {
    #[inline(always)]
    fn from(val: u8) -> Sm2ctrlSplit {
        Sm2ctrlSplit::from_bits(val)
    }
}
impl From<Sm2ctrlSplit> for u8 {
    #[inline(always)]
    fn from(val: Sm2ctrlSplit) -> u8 {
        Sm2ctrlSplit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm2dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm2dmaenCaptde {
        Sm2dmaenCaptde::from_bits(val)
    }
}
impl From<Sm2dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm2dmaenCaptde) -> u8 {
        Sm2dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm2dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm2dmaenFand {
        Sm2dmaenFand::from_bits(val)
    }
}
impl From<Sm2dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm2dmaenFand) -> u8 {
        Sm2dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2dmaenValde {
    #[doc = "DMA write requests disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Sm2dmaenValde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2dmaenValde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2dmaenValde {
    #[inline(always)]
    fn from(val: u8) -> Sm2dmaenValde {
        Sm2dmaenValde::from_bits(val)
    }
}
impl From<Sm2dmaenValde> for u8 {
    #[inline(always)]
    fn from(val: Sm2dmaenValde) -> u8 {
        Sm2dmaenValde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2frctrlFrac1En {
    #[doc = "Disable fractional cycle length for the PWM period."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    ENABLED = 0x01,
}
impl Sm2frctrlFrac1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2frctrlFrac1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2frctrlFrac1En {
    #[inline(always)]
    fn from(val: u8) -> Sm2frctrlFrac1En {
        Sm2frctrlFrac1En::from_bits(val)
    }
}
impl From<Sm2frctrlFrac1En> for u8 {
    #[inline(always)]
    fn from(val: Sm2frctrlFrac1En) -> u8 {
        Sm2frctrlFrac1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2frctrlFrac23En {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    ENABLED = 0x01,
}
impl Sm2frctrlFrac23En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2frctrlFrac23En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2frctrlFrac23En {
    #[inline(always)]
    fn from(val: u8) -> Sm2frctrlFrac23En {
        Sm2frctrlFrac23En::from_bits(val)
    }
}
impl From<Sm2frctrlFrac23En> for u8 {
    #[inline(always)]
    fn from(val: Sm2frctrlFrac23En) -> u8 {
        Sm2frctrlFrac23En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2frctrlFrac45En {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    ENABLED = 0x01,
}
impl Sm2frctrlFrac45En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2frctrlFrac45En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2frctrlFrac45En {
    #[inline(always)]
    fn from(val: u8) -> Sm2frctrlFrac45En {
        Sm2frctrlFrac45En::from_bits(val)
    }
}
impl From<Sm2frctrlFrac45En> for u8 {
    #[inline(always)]
    fn from(val: Sm2frctrlFrac45En) -> u8 {
        Sm2frctrlFrac45En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCa0ie {
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    ENABLED = 0x01,
}
impl Sm2intenCa0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCa0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCa0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCa0ie {
        Sm2intenCa0ie::from_bits(val)
    }
}
impl From<Sm2intenCa0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCa0ie) -> u8 {
        Sm2intenCa0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCa1ie {
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
    ENABLED = 0x01,
}
impl Sm2intenCa1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCa1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCa1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCa1ie {
        Sm2intenCa1ie::from_bits(val)
    }
}
impl From<Sm2intenCa1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCa1ie) -> u8 {
        Sm2intenCa1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCb0ie {
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    ENABLED = 0x01,
}
impl Sm2intenCb0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCb0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCb0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCb0ie {
        Sm2intenCb0ie::from_bits(val)
    }
}
impl From<Sm2intenCb0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCb0ie) -> u8 {
        Sm2intenCb0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCb1ie {
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    ENABLED = 0x01,
}
impl Sm2intenCb1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCb1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCb1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCb1ie {
        Sm2intenCb1ie::from_bits(val)
    }
}
impl From<Sm2intenCb1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCb1ie) -> u8 {
        Sm2intenCb1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCmpie {
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
impl Sm2intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCmpie {
        Sm2intenCmpie::from_bits(val)
    }
}
impl From<Sm2intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCmpie) -> u8 {
        Sm2intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCx0ie {
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    ENABLED = 0x01,
}
impl Sm2intenCx0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCx0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCx0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCx0ie {
        Sm2intenCx0ie::from_bits(val)
    }
}
impl From<Sm2intenCx0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCx0ie) -> u8 {
        Sm2intenCx0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenCx1ie {
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    ENABLED = 0x01,
}
impl Sm2intenCx1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenCx1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenCx1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenCx1ie {
        Sm2intenCx1ie::from_bits(val)
    }
}
impl From<Sm2intenCx1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenCx1ie) -> u8 {
        Sm2intenCx1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenReie {
    #[doc = "STS\\[REF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm2intenReie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenReie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenReie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenReie {
        Sm2intenReie::from_bits(val)
    }
}
impl From<Sm2intenReie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenReie) -> u8 {
        Sm2intenReie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2intenRie {
    #[doc = "STS\\[RF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm2intenRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2intenRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2intenRie {
    #[inline(always)]
    fn from(val: u8) -> Sm2intenRie {
        Sm2intenRie::from_bits(val)
    }
}
impl From<Sm2intenRie> for u8 {
    #[inline(always)]
    fn from(val: Sm2intenRie) -> u8 {
        Sm2intenRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPola {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm2octrlPola {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPola {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPola {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPola {
        Sm2octrlPola::from_bits(val)
    }
}
impl From<Sm2octrlPola> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPola) -> u8 {
        Sm2octrlPola::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPolb {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm2octrlPolb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPolb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPolb {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPolb {
        Sm2octrlPolb::from_bits(val)
    }
}
impl From<Sm2octrlPolb> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPolb) -> u8 {
        Sm2octrlPolb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPolx {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm2octrlPolx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPolx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPolx {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPolx {
        Sm2octrlPolx::from_bits(val)
    }
}
impl From<Sm2octrlPolx> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPolx) -> u8 {
        Sm2octrlPolx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmafs {
        Sm2octrlPwmafs::from_bits(val)
    }
}
impl From<Sm2octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmafs) -> u8 {
        Sm2octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmbfs {
        Sm2octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm2octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmbfs) -> u8 {
        Sm2octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm2octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm2octrlPwmxfs {
        Sm2octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm2octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm2octrlPwmxfs) -> u8 {
        Sm2octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm2out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm2out23 {
        Sm2out23::from_bits(val)
    }
}
impl From<Sm2out23> for u8 {
    #[inline(always)]
    fn from(val: Sm2out23) -> u8 {
        Sm2out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 2 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm2out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm2out45 {
        Sm2out45::from_bits(val)
    }
}
impl From<Sm2out45> for u8 {
    #[inline(always)]
    fn from(val: Sm2out45) -> u8 {
        Sm2out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2sel23 {
    #[doc = "Generated SM2PWM23 signal used by the deadtime logic."]
    SM2PWM23 = 0x0,
    #[doc = "Inverted generated SM2PWM23 signal used by the deadtime logic."]
    INVERTED_SM2PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT23\\] used by the deadtime logic."]
    SM2OUT23 = 0x02,
    #[doc = "PWM2_EXTA signal used by the deadtime logic."]
    PWM2_EXTA = 0x03,
}
impl Sm2sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel23 {
        Sm2sel23::from_bits(val)
    }
}
impl From<Sm2sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel23) -> u8 {
        Sm2sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2sel45 {
    #[doc = "Generated SM2PWM45 signal used by the deadtime logic."]
    SM2PWM45 = 0x0,
    #[doc = "Inverted generated SM2PWM45 signal used by the deadtime logic."]
    INVERTED_SM2PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM2OUT45\\] used by the deadtime logic."]
    SM2OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm2sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm2sel45 {
        Sm2sel45::from_bits(val)
    }
}
impl From<Sm2sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm2sel45) -> u8 {
        Sm2sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2stsCmpf {
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
impl Sm2stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm2stsCmpf {
        Sm2stsCmpf::from_bits(val)
    }
}
impl From<Sm2stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm2stsCmpf) -> u8 {
        Sm2stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2stsRef {
    #[doc = "No reload error occurred."]
    NO_FLAG = 0x0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    FLAG = 0x01,
}
impl Sm2stsRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2stsRef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2stsRef {
    #[inline(always)]
    fn from(val: u8) -> Sm2stsRef {
        Sm2stsRef::from_bits(val)
    }
}
impl From<Sm2stsRef> for u8 {
    #[inline(always)]
    fn from(val: Sm2stsRef) -> u8 {
        Sm2stsRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2stsRf {
    #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
    NO_FLAG = 0x0,
    #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
    FLAG = 0x01,
}
impl Sm2stsRf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2stsRf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2stsRf {
    #[inline(always)]
    fn from(val: u8) -> Sm2stsRf {
        Sm2stsRf::from_bits(val)
    }
}
impl From<Sm2stsRf> for u8 {
    #[inline(always)]
    fn from(val: Sm2stsRf) -> u8 {
        Sm2stsRf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2stsRuf {
    #[doc = "No register update has occurred since last reload."]
    NO_FLAG = 0x0,
    #[doc = "At least one of the double buffered registers has been updated since the last reload."]
    FLAG = 0x01,
}
impl Sm2stsRuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2stsRuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2stsRuf {
    #[inline(always)]
    fn from(val: u8) -> Sm2stsRuf {
        Sm2stsRuf::from_bits(val)
    }
}
impl From<Sm2stsRuf> for u8 {
    #[inline(always)]
    fn from(val: Sm2stsRuf) -> u8 {
        Sm2stsRuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlOutTrigEn {
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
impl Sm2tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlOutTrigEn {
        Sm2tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm2tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlOutTrigEn) -> u8 {
        Sm2tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm2tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlPwaot0 {
        Sm2tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm2tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlPwaot0) -> u8 {
        Sm2tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm2tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlPwbot1 {
        Sm2tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm2tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlPwbot1) -> u8 {
        Sm2tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm2tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm2tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm2tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm2tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm2tctrlTrgfrq {
        Sm2tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm2tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm2tctrlTrgfrq) -> u8 {
        Sm2tctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaArma {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLA\\[EDGAx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm3captctrlaArma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaArma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaArma {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaArma {
        Sm3captctrlaArma::from_bits(val)
    }
}
impl From<Sm3captctrlaArma> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaArma) -> u8 {
        Sm3captctrlaArma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaEdga0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlaEdga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaEdga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaEdga0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaEdga0 {
        Sm3captctrlaEdga0::from_bits(val)
    }
}
impl From<Sm3captctrlaEdga0> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaEdga0) -> u8 {
        Sm3captctrlaEdga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaEdga1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlaEdga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaEdga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaEdga1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaEdga1 {
        Sm3captctrlaEdga1::from_bits(val)
    }
}
impl From<Sm3captctrlaEdga1> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaEdga1) -> u8 {
        Sm3captctrlaEdga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaEdgcntaEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm3captctrlaEdgcntaEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaEdgcntaEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaEdgcntaEn {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaEdgcntaEn {
        Sm3captctrlaEdgcntaEn::from_bits(val)
    }
}
impl From<Sm3captctrlaEdgcntaEn> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaEdgcntaEn) -> u8 {
        Sm3captctrlaEdgcntaEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaInpSela {
    #[doc = "Raw PWM_A input signal selected as source."]
    PWM_A = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm3captctrlaInpSela {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaInpSela {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaInpSela {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaInpSela {
        Sm3captctrlaInpSela::from_bits(val)
    }
}
impl From<Sm3captctrlaInpSela> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaInpSela) -> u8 {
        Sm3captctrlaInpSela::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlaOneshota {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm3captctrlaOneshota {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlaOneshota {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlaOneshota {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlaOneshota {
        Sm3captctrlaOneshota::from_bits(val)
    }
}
impl From<Sm3captctrlaOneshota> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlaOneshota) -> u8 {
        Sm3captctrlaOneshota::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbArmb {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLB\\[EDGBx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm3captctrlbArmb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbArmb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbArmb {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbArmb {
        Sm3captctrlbArmb::from_bits(val)
    }
}
impl From<Sm3captctrlbArmb> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbArmb) -> u8 {
        Sm3captctrlbArmb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbEdgb0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlbEdgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbEdgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbEdgb0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbEdgb0 {
        Sm3captctrlbEdgb0::from_bits(val)
    }
}
impl From<Sm3captctrlbEdgb0> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbEdgb0) -> u8 {
        Sm3captctrlbEdgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbEdgb1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlbEdgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbEdgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbEdgb1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbEdgb1 {
        Sm3captctrlbEdgb1::from_bits(val)
    }
}
impl From<Sm3captctrlbEdgb1> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbEdgb1) -> u8 {
        Sm3captctrlbEdgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbEdgcntbEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm3captctrlbEdgcntbEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbEdgcntbEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbEdgcntbEn {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbEdgcntbEn {
        Sm3captctrlbEdgcntbEn::from_bits(val)
    }
}
impl From<Sm3captctrlbEdgcntbEn> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbEdgcntbEn) -> u8 {
        Sm3captctrlbEdgcntbEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbInpSelb {
    #[doc = "Raw PWM_B input signal selected as source."]
    PWM_B = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm3captctrlbInpSelb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbInpSelb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbInpSelb {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbInpSelb {
        Sm3captctrlbInpSelb::from_bits(val)
    }
}
impl From<Sm3captctrlbInpSelb> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbInpSelb) -> u8 {
        Sm3captctrlbInpSelb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlbOneshotb {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm3captctrlbOneshotb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlbOneshotb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlbOneshotb {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlbOneshotb {
        Sm3captctrlbOneshotb::from_bits(val)
    }
}
impl From<Sm3captctrlbOneshotb> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlbOneshotb) -> u8 {
        Sm3captctrlbOneshotb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxArmx {
    #[doc = "Input capture operation is disabled."]
    DISABLED = 0x0,
    #[doc = "Input capture operation as specified by CAPTCTRLX\\[EDGXx\\] is enabled."]
    ENABLED = 0x01,
}
impl Sm3captctrlxArmx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxArmx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxArmx {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxArmx {
        Sm3captctrlxArmx::from_bits(val)
    }
}
impl From<Sm3captctrlxArmx> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxArmx) -> u8 {
        Sm3captctrlxArmx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxEdgcntxEn {
    #[doc = "Edge counter disabled and held in reset"]
    DISABLED = 0x0,
    #[doc = "Edge counter enabled"]
    ENABLED = 0x01,
}
impl Sm3captctrlxEdgcntxEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxEdgcntxEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxEdgcntxEn {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxEdgcntxEn {
        Sm3captctrlxEdgcntxEn::from_bits(val)
    }
}
impl From<Sm3captctrlxEdgcntxEn> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxEdgcntxEn) -> u8 {
        Sm3captctrlxEdgcntxEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxEdgx0 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxEdgx0 {
        Sm3captctrlxEdgx0::from_bits(val)
    }
}
impl From<Sm3captctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxEdgx0) -> u8 {
        Sm3captctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxEdgx1 {
    #[doc = "Disabled"]
    DISABLED = 0x0,
    #[doc = "Capture falling edges"]
    FALLING_EDGE = 0x01,
    #[doc = "Capture rising edges"]
    RISING_EDGE = 0x02,
    #[doc = "Capture any edge"]
    ANY_EDGE = 0x03,
}
impl Sm3captctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxEdgx1 {
        Sm3captctrlxEdgx1::from_bits(val)
    }
}
impl From<Sm3captctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxEdgx1) -> u8 {
        Sm3captctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PWM_X = 0x0,
    #[doc = "Edge Counter"]
    EDGE_COUNTER = 0x01,
}
impl Sm3captctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxInpSelx {
        Sm3captctrlxInpSelx::from_bits(val)
    }
}
impl From<Sm3captctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxInpSelx) -> u8 {
        Sm3captctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3captctrlxOneshotx {
    #[doc = "Free Running"]
    FREE_RUNNING = 0x0,
    #[doc = "One Shot"]
    ONE_SHOT = 0x01,
}
impl Sm3captctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3captctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3captctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> Sm3captctrlxOneshotx {
        Sm3captctrlxOneshotx::from_bits(val)
    }
}
impl From<Sm3captctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: Sm3captctrlxOneshotx) -> u8 {
        Sm3captctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    IPBUS = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    EXT_CLK = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AUX_CLK = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm3ctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ClkSel {
        Sm3ctrl2ClkSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ClkSel) -> u8 {
        Sm3ctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ForceSel {
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
impl Sm3ctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ForceSel {
        Sm3ctrl2ForceSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ForceSel) -> u8 {
        Sm3ctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2Frcen {
    #[doc = "Initialization from a FORCE_OUT is disabled."]
    DISABLED = 0x0,
    #[doc = "Initialization from a FORCE_OUT is enabled."]
    ENABLED = 0x01,
}
impl Sm3ctrl2Frcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2Frcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2Frcen {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2Frcen {
        Sm3ctrl2Frcen::from_bits(val)
    }
}
impl From<Sm3ctrl2Frcen> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2Frcen) -> u8 {
        Sm3ctrl2Frcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2Indep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    COMPLEMENTARY = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    INDEPENDENT = 0x01,
}
impl Sm3ctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2Indep {
        Sm3ctrl2Indep::from_bits(val)
    }
}
impl From<Sm3ctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2Indep) -> u8 {
        Sm3ctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2InitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PWM_X = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MASTER_RELOAD = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MASTER_SYNC = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    EXT_SYNC = 0x03,
}
impl Sm3ctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2InitSel {
        Sm3ctrl2InitSel::from_bits(val)
    }
}
impl From<Sm3ctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2InitSel) -> u8 {
        Sm3ctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrl2ReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    LOCAL = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    MASTER = 0x01,
}
impl Sm3ctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrl2ReloadSel {
        Sm3ctrl2ReloadSel::from_bits(val)
    }
}
impl From<Sm3ctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrl2ReloadSel) -> u8 {
        Sm3ctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EQUAL_TO = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EQUAL_TO_OR_GREATER_THAN = 0x01,
}
impl Sm3ctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlCompmode {
        Sm3ctrlCompmode::from_bits(val)
    }
}
impl From<Sm3ctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlCompmode) -> u8 {
        Sm3ctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlDblen {
    #[doc = "Double switching disabled."]
    DISABLED = 0x0,
    #[doc = "Double switching enabled."]
    ENABLED = 0x01,
}
impl Sm3ctrlDblen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlDblen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlDblen {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlDblen {
        Sm3ctrlDblen::from_bits(val)
    }
}
impl From<Sm3ctrlDblen> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlDblen) -> u8 {
        Sm3ctrlDblen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlDblx {
    #[doc = "PWM_X double pulse disabled."]
    DISABLED = 0x0,
    #[doc = "PWM_X double pulse enabled."]
    ENABLED = 0x01,
}
impl Sm3ctrlDblx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlDblx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlDblx {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlDblx {
        Sm3ctrlDblx::from_bits(val)
    }
}
impl From<Sm3ctrlDblx> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlDblx) -> u8 {
        Sm3ctrlDblx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlFull {
    #[doc = "Full-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Full-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm3ctrlFull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlFull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlFull {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlFull {
        Sm3ctrlFull::from_bits(val)
    }
}
impl From<Sm3ctrlFull> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlFull) -> u8 {
        Sm3ctrlFull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlHalf {
    #[doc = "Half-cycle reloads disabled."]
    DISABLED = 0x0,
    #[doc = "Half-cycle reloads enabled."]
    ENABLED = 0x01,
}
impl Sm3ctrlHalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlHalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlHalf {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlHalf {
        Sm3ctrlHalf::from_bits(val)
    }
}
impl From<Sm3ctrlHalf> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlHalf) -> u8 {
        Sm3ctrlHalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlLdfq {
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
impl Sm3ctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlLdfq {
        Sm3ctrlLdfq::from_bits(val)
    }
}
impl From<Sm3ctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlLdfq) -> u8 {
        Sm3ctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NEXT_PWM_RELOAD = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MTCTRL_LDOK_SET = 0x01,
}
impl Sm3ctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlLdmod {
        Sm3ctrlLdmod::from_bits(val)
    }
}
impl From<Sm3ctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlLdmod) -> u8 {
        Sm3ctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlPrsc {
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
impl Sm3ctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlPrsc {
        Sm3ctrlPrsc::from_bits(val)
    }
}
impl From<Sm3ctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlPrsc) -> u8 {
        Sm3ctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3ctrlSplit {
    #[doc = "DBLPWM is not split. PWM_A and PWM_B each have double pulses."]
    DISABLED = 0x0,
    #[doc = "DBLPWM is split to PWM_A and PWM_B."]
    ENABLED = 0x01,
}
impl Sm3ctrlSplit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3ctrlSplit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3ctrlSplit {
    #[inline(always)]
    fn from(val: u8) -> Sm3ctrlSplit {
        Sm3ctrlSplit::from_bits(val)
    }
}
impl From<Sm3ctrlSplit> for u8 {
    #[inline(always)]
    fn from(val: Sm3ctrlSplit) -> u8 {
        Sm3ctrlSplit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3dmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    DISABLED = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    EXCEEDFIFO = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LOCAL_SYNC = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LOCAL_RELOAD = 0x03,
}
impl Sm3dmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3dmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3dmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> Sm3dmaenCaptde {
        Sm3dmaenCaptde::from_bits(val)
    }
}
impl From<Sm3dmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: Sm3dmaenCaptde) -> u8 {
        Sm3dmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3dmaenFand {
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    OR = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    AND = 0x01,
}
impl Sm3dmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3dmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3dmaenFand {
    #[inline(always)]
    fn from(val: u8) -> Sm3dmaenFand {
        Sm3dmaenFand::from_bits(val)
    }
}
impl From<Sm3dmaenFand> for u8 {
    #[inline(always)]
    fn from(val: Sm3dmaenFand) -> u8 {
        Sm3dmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3dmaenValde {
    #[doc = "DMA write requests disabled"]
    DISABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Sm3dmaenValde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3dmaenValde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3dmaenValde {
    #[inline(always)]
    fn from(val: u8) -> Sm3dmaenValde {
        Sm3dmaenValde::from_bits(val)
    }
}
impl From<Sm3dmaenValde> for u8 {
    #[inline(always)]
    fn from(val: Sm3dmaenValde) -> u8 {
        Sm3dmaenValde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3frctrlFrac1En {
    #[doc = "Disable fractional cycle length for the PWM period."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle length for the PWM period."]
    ENABLED = 0x01,
}
impl Sm3frctrlFrac1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3frctrlFrac1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3frctrlFrac1En {
    #[inline(always)]
    fn from(val: u8) -> Sm3frctrlFrac1En {
        Sm3frctrlFrac1En::from_bits(val)
    }
}
impl From<Sm3frctrlFrac1En> for u8 {
    #[inline(always)]
    fn from(val: Sm3frctrlFrac1En) -> u8 {
        Sm3frctrlFrac1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3frctrlFrac23En {
    #[doc = "Disable fractional cycle placement for PWM_A."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_A."]
    ENABLED = 0x01,
}
impl Sm3frctrlFrac23En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3frctrlFrac23En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3frctrlFrac23En {
    #[inline(always)]
    fn from(val: u8) -> Sm3frctrlFrac23En {
        Sm3frctrlFrac23En::from_bits(val)
    }
}
impl From<Sm3frctrlFrac23En> for u8 {
    #[inline(always)]
    fn from(val: Sm3frctrlFrac23En) -> u8 {
        Sm3frctrlFrac23En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3frctrlFrac45En {
    #[doc = "Disable fractional cycle placement for PWM_B."]
    DISABLED = 0x0,
    #[doc = "Enable fractional cycle placement for PWM_B."]
    ENABLED = 0x01,
}
impl Sm3frctrlFrac45En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3frctrlFrac45En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3frctrlFrac45En {
    #[inline(always)]
    fn from(val: u8) -> Sm3frctrlFrac45En {
        Sm3frctrlFrac45En::from_bits(val)
    }
}
impl From<Sm3frctrlFrac45En> for u8 {
    #[inline(always)]
    fn from(val: Sm3frctrlFrac45En) -> u8 {
        Sm3frctrlFrac45En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCa0ie {
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    ENABLED = 0x01,
}
impl Sm3intenCa0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCa0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCa0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCa0ie {
        Sm3intenCa0ie::from_bits(val)
    }
}
impl From<Sm3intenCa0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCa0ie) -> u8 {
        Sm3intenCa0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCa1ie {
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]"]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]"]
    ENABLED = 0x01,
}
impl Sm3intenCa1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCa1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCa1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCa1ie {
        Sm3intenCa1ie::from_bits(val)
    }
}
impl From<Sm3intenCa1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCa1ie) -> u8 {
        Sm3intenCa1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCb0ie {
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    ENABLED = 0x01,
}
impl Sm3intenCb0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCb0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCb0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCb0ie {
        Sm3intenCb0ie::from_bits(val)
    }
}
impl From<Sm3intenCb0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCb0ie) -> u8 {
        Sm3intenCb0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCb1ie {
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    ENABLED = 0x01,
}
impl Sm3intenCb1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCb1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCb1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCb1ie {
        Sm3intenCb1ie::from_bits(val)
    }
}
impl From<Sm3intenCb1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCb1ie) -> u8 {
        Sm3intenCb1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCmpie {
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
impl Sm3intenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCmpie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCmpie {
        Sm3intenCmpie::from_bits(val)
    }
}
impl From<Sm3intenCmpie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCmpie) -> u8 {
        Sm3intenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCx0ie {
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    ENABLED = 0x01,
}
impl Sm3intenCx0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCx0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCx0ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCx0ie {
        Sm3intenCx0ie::from_bits(val)
    }
}
impl From<Sm3intenCx0ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCx0ie) -> u8 {
        Sm3intenCx0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenCx1ie {
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    DISABLED = 0x0,
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    ENABLED = 0x01,
}
impl Sm3intenCx1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenCx1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenCx1ie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenCx1ie {
        Sm3intenCx1ie::from_bits(val)
    }
}
impl From<Sm3intenCx1ie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenCx1ie) -> u8 {
        Sm3intenCx1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenReie {
    #[doc = "STS\\[REF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[REF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm3intenReie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenReie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenReie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenReie {
        Sm3intenReie::from_bits(val)
    }
}
impl From<Sm3intenReie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenReie) -> u8 {
        Sm3intenReie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3intenRie {
    #[doc = "STS\\[RF\\] CPU interrupt requests disabled"]
    DISABLED = 0x0,
    #[doc = "STS\\[RF\\] CPU interrupt requests enabled"]
    ENABLED = 0x01,
}
impl Sm3intenRie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3intenRie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3intenRie {
    #[inline(always)]
    fn from(val: u8) -> Sm3intenRie {
        Sm3intenRie::from_bits(val)
    }
}
impl From<Sm3intenRie> for u8 {
    #[inline(always)]
    fn from(val: Sm3intenRie) -> u8 {
        Sm3intenRie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPola {
    #[doc = "PWM_A output not inverted. A high level on the PWM_A pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_A output inverted. A low level on the PWM_A pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm3octrlPola {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPola {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPola {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPola {
        Sm3octrlPola::from_bits(val)
    }
}
impl From<Sm3octrlPola> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPola) -> u8 {
        Sm3octrlPola::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPolb {
    #[doc = "PWM_B output not inverted. A high level on the PWM_B pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_B output inverted. A low level on the PWM_B pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm3octrlPolb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPolb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPolb {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPolb {
        Sm3octrlPolb::from_bits(val)
    }
}
impl From<Sm3octrlPolb> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPolb) -> u8 {
        Sm3octrlPolb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPolx {
    #[doc = "PWM_X output not inverted. A high level on the PWM_X pin represents the \"on\" or \"active\" state."]
    NOT_INVERTED = 0x0,
    #[doc = "PWM_X output inverted. A low level on the PWM_X pin represents the \"on\" or \"active\" state."]
    INVERTED = 0x01,
}
impl Sm3octrlPolx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPolx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPolx {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPolx {
        Sm3octrlPolx::from_bits(val)
    }
}
impl From<Sm3octrlPolx> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPolx) -> u8 {
        Sm3octrlPolx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmafs {
        Sm3octrlPwmafs::from_bits(val)
    }
}
impl From<Sm3octrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmafs) -> u8 {
        Sm3octrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmbfs {
        Sm3octrlPwmbfs::from_bits(val)
    }
}
impl From<Sm3octrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmbfs) -> u8 {
        Sm3octrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3octrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    LOGIC_0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    LOGIC_1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    TRISTATED_3 = 0x03,
}
impl Sm3octrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3octrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3octrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> Sm3octrlPwmxfs {
        Sm3octrlPwmxfs::from_bits(val)
    }
}
impl From<Sm3octrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: Sm3octrlPwmxfs) -> u8 {
        Sm3octrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3out23 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM23."]
    LOGIC_1 = 0x01,
}
impl Sm3out23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3out23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3out23 {
    #[inline(always)]
    fn from(val: u8) -> Sm3out23 {
        Sm3out23::from_bits(val)
    }
}
impl From<Sm3out23> for u8 {
    #[inline(always)]
    fn from(val: Sm3out23) -> u8 {
        Sm3out23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3out45 {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 3 instead of PWM45."]
    LOGIC_1 = 0x01,
}
impl Sm3out45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3out45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3out45 {
    #[inline(always)]
    fn from(val: u8) -> Sm3out45 {
        Sm3out45::from_bits(val)
    }
}
impl From<Sm3out45> for u8 {
    #[inline(always)]
    fn from(val: Sm3out45) -> u8 {
        Sm3out45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3sel23 {
    #[doc = "Generated SM3PWM23 signal used by the deadtime logic."]
    SM3PWM23 = 0x0,
    #[doc = "Inverted generated SM3PWM23 signal used by the deadtime logic."]
    INVERTED_SM3PWM23 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT23\\] used by the deadtime logic."]
    SM3OUT23 = 0x02,
    #[doc = "PWM3_EXTA signal used by the deadtime logic."]
    PWM3_EXTA = 0x03,
}
impl Sm3sel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel23 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel23 {
        Sm3sel23::from_bits(val)
    }
}
impl From<Sm3sel23> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel23) -> u8 {
        Sm3sel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3sel45 {
    #[doc = "Generated SM3PWM45 signal used by the deadtime logic."]
    SM3PWM45 = 0x0,
    #[doc = "Inverted generated SM3PWM45 signal used by the deadtime logic."]
    INVERTED_SM3PWM45 = 0x01,
    #[doc = "SWCOUT\\[SM3OUT45\\] used by the deadtime logic."]
    SM3OUT45 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sm3sel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3sel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3sel45 {
    #[inline(always)]
    fn from(val: u8) -> Sm3sel45 {
        Sm3sel45::from_bits(val)
    }
}
impl From<Sm3sel45> for u8 {
    #[inline(always)]
    fn from(val: Sm3sel45) -> u8 {
        Sm3sel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3stsCmpf {
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
impl Sm3stsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3stsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3stsCmpf {
    #[inline(always)]
    fn from(val: u8) -> Sm3stsCmpf {
        Sm3stsCmpf::from_bits(val)
    }
}
impl From<Sm3stsCmpf> for u8 {
    #[inline(always)]
    fn from(val: Sm3stsCmpf) -> u8 {
        Sm3stsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3stsRef {
    #[doc = "No reload error occurred."]
    NO_FLAG = 0x0,
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\] = 0."]
    FLAG = 0x01,
}
impl Sm3stsRef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3stsRef {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3stsRef {
    #[inline(always)]
    fn from(val: u8) -> Sm3stsRef {
        Sm3stsRef::from_bits(val)
    }
}
impl From<Sm3stsRef> for u8 {
    #[inline(always)]
    fn from(val: Sm3stsRef) -> u8 {
        Sm3stsRef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3stsRf {
    #[doc = "No new reload cycle since last STS\\[RF\\] clearing"]
    NO_FLAG = 0x0,
    #[doc = "New reload cycle since last STS\\[RF\\] clearing"]
    FLAG = 0x01,
}
impl Sm3stsRf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3stsRf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3stsRf {
    #[inline(always)]
    fn from(val: u8) -> Sm3stsRf {
        Sm3stsRf::from_bits(val)
    }
}
impl From<Sm3stsRf> for u8 {
    #[inline(always)]
    fn from(val: Sm3stsRf) -> u8 {
        Sm3stsRf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3stsRuf {
    #[doc = "No register update has occurred since last reload."]
    NO_FLAG = 0x0,
    #[doc = "At least one of the double buffered registers has been updated since the last reload."]
    FLAG = 0x01,
}
impl Sm3stsRuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3stsRuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3stsRuf {
    #[inline(always)]
    fn from(val: u8) -> Sm3stsRuf {
        Sm3stsRuf::from_bits(val)
    }
}
impl From<Sm3stsRuf> for u8 {
    #[inline(always)]
    fn from(val: Sm3stsRuf) -> u8 {
        Sm3stsRuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlOutTrigEn {
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
impl Sm3tctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlOutTrigEn {
        Sm3tctrlOutTrigEn::from_bits(val)
    }
}
impl From<Sm3tctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlOutTrigEn) -> u8 {
        Sm3tctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlPwaot0 {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PWM_OUT_TRIG0_SIGNAL = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PWMA_OUTPUT = 0x01,
}
impl Sm3tctrlPwaot0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlPwaot0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlPwaot0 {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlPwaot0 {
        Sm3tctrlPwaot0::from_bits(val)
    }
}
impl From<Sm3tctrlPwaot0> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlPwaot0) -> u8 {
        Sm3tctrlPwaot0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlPwbot1 {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PWM_OUT_TRIG1_SIGNAL = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PWMB_OUTPUT = 0x01,
}
impl Sm3tctrlPwbot1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlPwbot1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlPwbot1 {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlPwbot1 {
        Sm3tctrlPwbot1::from_bits(val)
    }
}
impl From<Sm3tctrlPwbot1> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlPwbot1) -> u8 {
        Sm3tctrlPwbot1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3tctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    EVERYPWM = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    FINALPWM = 0x01,
}
impl Sm3tctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3tctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3tctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> Sm3tctrlTrgfrq {
        Sm3tctrlTrgfrq::from_bits(val)
    }
}
impl From<Sm3tctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: Sm3tctrlTrgfrq) -> u8 {
        Sm3tctrlTrgfrq::to_bits(val)
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
