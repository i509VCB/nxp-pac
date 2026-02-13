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
