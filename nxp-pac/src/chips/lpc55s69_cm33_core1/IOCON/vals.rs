#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_0_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_ASW {
        PIO0_0_ASW::from_bits(val)
    }
}
impl From<PIO0_0_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_ASW) -> u8 {
        PIO0_0_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_0_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_DIGIMODE {
        PIO0_0_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_0_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_DIGIMODE) -> u8 {
        PIO0_0_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_0_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_FUNC {
        PIO0_0_FUNC::from_bits(val)
    }
}
impl From<PIO0_0_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_FUNC) -> u8 {
        PIO0_0_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_0_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_MODE {
        PIO0_0_MODE::from_bits(val)
    }
}
impl From<PIO0_0_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_MODE) -> u8 {
        PIO0_0_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_0_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_OD {
        PIO0_0_OD::from_bits(val)
    }
}
impl From<PIO0_0_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_OD) -> u8 {
        PIO0_0_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_0_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_0_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_0_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_0_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_0_SLEW {
        PIO0_0_SLEW::from_bits(val)
    }
}
impl From<PIO0_0_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_0_SLEW) -> u8 {
        PIO0_0_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_10_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_ASW {
        PIO0_10_ASW::from_bits(val)
    }
}
impl From<PIO0_10_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_ASW) -> u8 {
        PIO0_10_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_10_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_DIGIMODE {
        PIO0_10_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_10_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_DIGIMODE) -> u8 {
        PIO0_10_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_10_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_FUNC {
        PIO0_10_FUNC::from_bits(val)
    }
}
impl From<PIO0_10_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_FUNC) -> u8 {
        PIO0_10_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_10_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_MODE {
        PIO0_10_MODE::from_bits(val)
    }
}
impl From<PIO0_10_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_MODE) -> u8 {
        PIO0_10_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_10_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_OD {
        PIO0_10_OD::from_bits(val)
    }
}
impl From<PIO0_10_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_OD) -> u8 {
        PIO0_10_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_10_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_10_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_10_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_10_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_10_SLEW {
        PIO0_10_SLEW::from_bits(val)
    }
}
impl From<PIO0_10_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_10_SLEW) -> u8 {
        PIO0_10_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_11_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_ASW {
        PIO0_11_ASW::from_bits(val)
    }
}
impl From<PIO0_11_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_ASW) -> u8 {
        PIO0_11_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_11_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_DIGIMODE {
        PIO0_11_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_11_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_DIGIMODE) -> u8 {
        PIO0_11_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_11_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_FUNC {
        PIO0_11_FUNC::from_bits(val)
    }
}
impl From<PIO0_11_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_FUNC) -> u8 {
        PIO0_11_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_11_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_MODE {
        PIO0_11_MODE::from_bits(val)
    }
}
impl From<PIO0_11_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_MODE) -> u8 {
        PIO0_11_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_11_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_OD {
        PIO0_11_OD::from_bits(val)
    }
}
impl From<PIO0_11_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_OD) -> u8 {
        PIO0_11_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_11_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_11_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_11_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_11_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_11_SLEW {
        PIO0_11_SLEW::from_bits(val)
    }
}
impl From<PIO0_11_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_11_SLEW) -> u8 {
        PIO0_11_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_12_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_ASW {
        PIO0_12_ASW::from_bits(val)
    }
}
impl From<PIO0_12_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_ASW) -> u8 {
        PIO0_12_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_12_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_DIGIMODE {
        PIO0_12_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_12_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_DIGIMODE) -> u8 {
        PIO0_12_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_12_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_FUNC {
        PIO0_12_FUNC::from_bits(val)
    }
}
impl From<PIO0_12_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_FUNC) -> u8 {
        PIO0_12_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_12_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_MODE {
        PIO0_12_MODE::from_bits(val)
    }
}
impl From<PIO0_12_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_MODE) -> u8 {
        PIO0_12_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_12_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_OD {
        PIO0_12_OD::from_bits(val)
    }
}
impl From<PIO0_12_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_OD) -> u8 {
        PIO0_12_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_12_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_12_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_12_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_12_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_12_SLEW {
        PIO0_12_SLEW::from_bits(val)
    }
}
impl From<PIO0_12_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_12_SLEW) -> u8 {
        PIO0_12_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_13_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_DIGIMODE {
        PIO0_13_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_13_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_DIGIMODE) -> u8 {
        PIO0_13_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_EGP {
    #[doc = "I2C mode."]
    I2C_MODE = 0x0,
    #[doc = "GPIO mode."]
    GPIO_MODE = 0x01,
}
impl PIO0_13_EGP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_EGP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_EGP {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_EGP {
        PIO0_13_EGP::from_bits(val)
    }
}
impl From<PIO0_13_EGP> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_EGP) -> u8 {
        PIO0_13_EGP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_FILTEROFF {
    #[doc = "Filter enabled."]
    ENABLED = 0x0,
    #[doc = "Filter disabled."]
    DISABLED = 0x01,
}
impl PIO0_13_FILTEROFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_FILTEROFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_FILTEROFF {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_FILTEROFF {
        PIO0_13_FILTEROFF::from_bits(val)
    }
}
impl From<PIO0_13_FILTEROFF> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_FILTEROFF) -> u8 {
        PIO0_13_FILTEROFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_13_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_FUNC {
        PIO0_13_FUNC::from_bits(val)
    }
}
impl From<PIO0_13_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_FUNC) -> u8 {
        PIO0_13_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_I2CFILTER {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 0x01,
}
impl PIO0_13_I2CFILTER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_I2CFILTER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_I2CFILTER {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_I2CFILTER {
        PIO0_13_I2CFILTER::from_bits(val)
    }
}
impl From<PIO0_13_I2CFILTER> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_I2CFILTER) -> u8 {
        PIO0_13_I2CFILTER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_13_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_MODE {
        PIO0_13_MODE::from_bits(val)
    }
}
impl From<PIO0_13_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_MODE) -> u8 {
        PIO0_13_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_13_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_OD {
        PIO0_13_OD::from_bits(val)
    }
}
impl From<PIO0_13_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_OD) -> u8 {
        PIO0_13_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_13_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_SLEW {
        PIO0_13_SLEW::from_bits(val)
    }
}
impl From<PIO0_13_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_SLEW) -> u8 {
        PIO0_13_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_13_SSEL {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8 = 0x01,
}
impl PIO0_13_SSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_13_SSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_13_SSEL {
    #[inline(always)]
    fn from(val: u8) -> PIO0_13_SSEL {
        PIO0_13_SSEL::from_bits(val)
    }
}
impl From<PIO0_13_SSEL> for u8 {
    #[inline(always)]
    fn from(val: PIO0_13_SSEL) -> u8 {
        PIO0_13_SSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_14_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_DIGIMODE {
        PIO0_14_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_14_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_DIGIMODE) -> u8 {
        PIO0_14_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_EGP {
    #[doc = "I2C mode."]
    I2C_MODE = 0x0,
    #[doc = "GPIO mode."]
    GPIO_MODE = 0x01,
}
impl PIO0_14_EGP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_EGP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_EGP {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_EGP {
        PIO0_14_EGP::from_bits(val)
    }
}
impl From<PIO0_14_EGP> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_EGP) -> u8 {
        PIO0_14_EGP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_FILTEROFF {
    #[doc = "Filter enabled."]
    ENABLED = 0x0,
    #[doc = "Filter disabled."]
    DISABLED = 0x01,
}
impl PIO0_14_FILTEROFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_FILTEROFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_FILTEROFF {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_FILTEROFF {
        PIO0_14_FILTEROFF::from_bits(val)
    }
}
impl From<PIO0_14_FILTEROFF> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_FILTEROFF) -> u8 {
        PIO0_14_FILTEROFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_14_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_FUNC {
        PIO0_14_FUNC::from_bits(val)
    }
}
impl From<PIO0_14_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_FUNC) -> u8 {
        PIO0_14_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_I2CFILTER {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FAST_MODE = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    STANDARD_MODE = 0x01,
}
impl PIO0_14_I2CFILTER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_I2CFILTER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_I2CFILTER {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_I2CFILTER {
        PIO0_14_I2CFILTER::from_bits(val)
    }
}
impl From<PIO0_14_I2CFILTER> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_I2CFILTER) -> u8 {
        PIO0_14_I2CFILTER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_14_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_MODE {
        PIO0_14_MODE::from_bits(val)
    }
}
impl From<PIO0_14_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_MODE) -> u8 {
        PIO0_14_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_14_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_OD {
        PIO0_14_OD::from_bits(val)
    }
}
impl From<PIO0_14_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_OD) -> u8 {
        PIO0_14_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_14_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_SLEW {
        PIO0_14_SLEW::from_bits(val)
    }
}
impl From<PIO0_14_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_SLEW) -> u8 {
        PIO0_14_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_14_SSEL {
    #[doc = "3V3 Signaling in I2C Mode."]
    SEL3V3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    SEL1V8 = 0x01,
}
impl PIO0_14_SSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_14_SSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_14_SSEL {
    #[inline(always)]
    fn from(val: u8) -> PIO0_14_SSEL {
        PIO0_14_SSEL::from_bits(val)
    }
}
impl From<PIO0_14_SSEL> for u8 {
    #[inline(always)]
    fn from(val: PIO0_14_SSEL) -> u8 {
        PIO0_14_SSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_15_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_ASW {
        PIO0_15_ASW::from_bits(val)
    }
}
impl From<PIO0_15_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_ASW) -> u8 {
        PIO0_15_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_15_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_DIGIMODE {
        PIO0_15_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_15_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_DIGIMODE) -> u8 {
        PIO0_15_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_15_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_FUNC {
        PIO0_15_FUNC::from_bits(val)
    }
}
impl From<PIO0_15_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_FUNC) -> u8 {
        PIO0_15_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_15_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_MODE {
        PIO0_15_MODE::from_bits(val)
    }
}
impl From<PIO0_15_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_MODE) -> u8 {
        PIO0_15_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_15_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_OD {
        PIO0_15_OD::from_bits(val)
    }
}
impl From<PIO0_15_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_OD) -> u8 {
        PIO0_15_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_15_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_15_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_15_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_15_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_15_SLEW {
        PIO0_15_SLEW::from_bits(val)
    }
}
impl From<PIO0_15_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_15_SLEW) -> u8 {
        PIO0_15_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_16_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_ASW {
        PIO0_16_ASW::from_bits(val)
    }
}
impl From<PIO0_16_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_ASW) -> u8 {
        PIO0_16_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_16_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_DIGIMODE {
        PIO0_16_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_16_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_DIGIMODE) -> u8 {
        PIO0_16_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_16_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_FUNC {
        PIO0_16_FUNC::from_bits(val)
    }
}
impl From<PIO0_16_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_FUNC) -> u8 {
        PIO0_16_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_16_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_MODE {
        PIO0_16_MODE::from_bits(val)
    }
}
impl From<PIO0_16_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_MODE) -> u8 {
        PIO0_16_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_16_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_OD {
        PIO0_16_OD::from_bits(val)
    }
}
impl From<PIO0_16_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_OD) -> u8 {
        PIO0_16_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_16_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_16_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_16_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_16_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_16_SLEW {
        PIO0_16_SLEW::from_bits(val)
    }
}
impl From<PIO0_16_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_16_SLEW) -> u8 {
        PIO0_16_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_17_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_17_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_17_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_17_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_17_DIGIMODE {
        PIO0_17_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_17_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_17_DIGIMODE) -> u8 {
        PIO0_17_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_17_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_17_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_17_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_17_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_17_FUNC {
        PIO0_17_FUNC::from_bits(val)
    }
}
impl From<PIO0_17_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_17_FUNC) -> u8 {
        PIO0_17_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_17_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_17_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_17_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_17_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_17_MODE {
        PIO0_17_MODE::from_bits(val)
    }
}
impl From<PIO0_17_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_17_MODE) -> u8 {
        PIO0_17_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_17_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_17_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_17_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_17_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_17_OD {
        PIO0_17_OD::from_bits(val)
    }
}
impl From<PIO0_17_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_17_OD) -> u8 {
        PIO0_17_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_17_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_17_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_17_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_17_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_17_SLEW {
        PIO0_17_SLEW::from_bits(val)
    }
}
impl From<PIO0_17_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_17_SLEW) -> u8 {
        PIO0_17_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_18_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_ASW {
        PIO0_18_ASW::from_bits(val)
    }
}
impl From<PIO0_18_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_ASW) -> u8 {
        PIO0_18_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_18_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_DIGIMODE {
        PIO0_18_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_18_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_DIGIMODE) -> u8 {
        PIO0_18_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_18_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_FUNC {
        PIO0_18_FUNC::from_bits(val)
    }
}
impl From<PIO0_18_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_FUNC) -> u8 {
        PIO0_18_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_18_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_MODE {
        PIO0_18_MODE::from_bits(val)
    }
}
impl From<PIO0_18_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_MODE) -> u8 {
        PIO0_18_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_18_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_OD {
        PIO0_18_OD::from_bits(val)
    }
}
impl From<PIO0_18_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_OD) -> u8 {
        PIO0_18_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_18_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_18_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_18_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_18_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_18_SLEW {
        PIO0_18_SLEW::from_bits(val)
    }
}
impl From<PIO0_18_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_18_SLEW) -> u8 {
        PIO0_18_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_19_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_19_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_19_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_19_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_19_DIGIMODE {
        PIO0_19_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_19_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_19_DIGIMODE) -> u8 {
        PIO0_19_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_19_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_19_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_19_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_19_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_19_FUNC {
        PIO0_19_FUNC::from_bits(val)
    }
}
impl From<PIO0_19_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_19_FUNC) -> u8 {
        PIO0_19_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_19_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_19_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_19_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_19_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_19_MODE {
        PIO0_19_MODE::from_bits(val)
    }
}
impl From<PIO0_19_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_19_MODE) -> u8 {
        PIO0_19_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_19_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_19_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_19_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_19_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_19_OD {
        PIO0_19_OD::from_bits(val)
    }
}
impl From<PIO0_19_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_19_OD) -> u8 {
        PIO0_19_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_19_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_19_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_19_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_19_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_19_SLEW {
        PIO0_19_SLEW::from_bits(val)
    }
}
impl From<PIO0_19_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_19_SLEW) -> u8 {
        PIO0_19_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_1_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_1_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_1_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_1_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_1_DIGIMODE {
        PIO0_1_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_1_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_1_DIGIMODE) -> u8 {
        PIO0_1_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_1_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_1_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_1_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_1_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_1_FUNC {
        PIO0_1_FUNC::from_bits(val)
    }
}
impl From<PIO0_1_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_1_FUNC) -> u8 {
        PIO0_1_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_1_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_1_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_1_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_1_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_1_MODE {
        PIO0_1_MODE::from_bits(val)
    }
}
impl From<PIO0_1_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_1_MODE) -> u8 {
        PIO0_1_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_1_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_1_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_1_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_1_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_1_OD {
        PIO0_1_OD::from_bits(val)
    }
}
impl From<PIO0_1_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_1_OD) -> u8 {
        PIO0_1_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_1_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_1_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_1_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_1_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_1_SLEW {
        PIO0_1_SLEW::from_bits(val)
    }
}
impl From<PIO0_1_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_1_SLEW) -> u8 {
        PIO0_1_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_20_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_20_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_20_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_20_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_20_DIGIMODE {
        PIO0_20_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_20_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_20_DIGIMODE) -> u8 {
        PIO0_20_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_20_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_20_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_20_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_20_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_20_FUNC {
        PIO0_20_FUNC::from_bits(val)
    }
}
impl From<PIO0_20_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_20_FUNC) -> u8 {
        PIO0_20_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_20_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_20_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_20_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_20_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_20_MODE {
        PIO0_20_MODE::from_bits(val)
    }
}
impl From<PIO0_20_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_20_MODE) -> u8 {
        PIO0_20_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_20_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_20_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_20_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_20_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_20_OD {
        PIO0_20_OD::from_bits(val)
    }
}
impl From<PIO0_20_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_20_OD) -> u8 {
        PIO0_20_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_20_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_20_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_20_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_20_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_20_SLEW {
        PIO0_20_SLEW::from_bits(val)
    }
}
impl From<PIO0_20_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_20_SLEW) -> u8 {
        PIO0_20_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_21_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_21_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_21_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_21_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_21_DIGIMODE {
        PIO0_21_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_21_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_21_DIGIMODE) -> u8 {
        PIO0_21_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_21_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_21_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_21_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_21_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_21_FUNC {
        PIO0_21_FUNC::from_bits(val)
    }
}
impl From<PIO0_21_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_21_FUNC) -> u8 {
        PIO0_21_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_21_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_21_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_21_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_21_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_21_MODE {
        PIO0_21_MODE::from_bits(val)
    }
}
impl From<PIO0_21_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_21_MODE) -> u8 {
        PIO0_21_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_21_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_21_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_21_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_21_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_21_OD {
        PIO0_21_OD::from_bits(val)
    }
}
impl From<PIO0_21_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_21_OD) -> u8 {
        PIO0_21_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_21_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_21_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_21_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_21_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_21_SLEW {
        PIO0_21_SLEW::from_bits(val)
    }
}
impl From<PIO0_21_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_21_SLEW) -> u8 {
        PIO0_21_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_22_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_22_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_22_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_22_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_22_DIGIMODE {
        PIO0_22_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_22_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_22_DIGIMODE) -> u8 {
        PIO0_22_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_22_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_22_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_22_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_22_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_22_FUNC {
        PIO0_22_FUNC::from_bits(val)
    }
}
impl From<PIO0_22_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_22_FUNC) -> u8 {
        PIO0_22_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_22_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_22_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_22_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_22_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_22_MODE {
        PIO0_22_MODE::from_bits(val)
    }
}
impl From<PIO0_22_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_22_MODE) -> u8 {
        PIO0_22_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_22_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_22_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_22_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_22_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_22_OD {
        PIO0_22_OD::from_bits(val)
    }
}
impl From<PIO0_22_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_22_OD) -> u8 {
        PIO0_22_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_22_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_22_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_22_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_22_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_22_SLEW {
        PIO0_22_SLEW::from_bits(val)
    }
}
impl From<PIO0_22_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_22_SLEW) -> u8 {
        PIO0_22_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_23_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_ASW {
        PIO0_23_ASW::from_bits(val)
    }
}
impl From<PIO0_23_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_ASW) -> u8 {
        PIO0_23_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_23_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_DIGIMODE {
        PIO0_23_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_23_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_DIGIMODE) -> u8 {
        PIO0_23_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_23_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_FUNC {
        PIO0_23_FUNC::from_bits(val)
    }
}
impl From<PIO0_23_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_FUNC) -> u8 {
        PIO0_23_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_23_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_MODE {
        PIO0_23_MODE::from_bits(val)
    }
}
impl From<PIO0_23_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_MODE) -> u8 {
        PIO0_23_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_23_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_OD {
        PIO0_23_OD::from_bits(val)
    }
}
impl From<PIO0_23_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_OD) -> u8 {
        PIO0_23_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_23_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_23_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_23_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_23_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_23_SLEW {
        PIO0_23_SLEW::from_bits(val)
    }
}
impl From<PIO0_23_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_23_SLEW) -> u8 {
        PIO0_23_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_24_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_24_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_24_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_24_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_24_DIGIMODE {
        PIO0_24_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_24_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_24_DIGIMODE) -> u8 {
        PIO0_24_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_24_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_24_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_24_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_24_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_24_FUNC {
        PIO0_24_FUNC::from_bits(val)
    }
}
impl From<PIO0_24_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_24_FUNC) -> u8 {
        PIO0_24_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_24_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_24_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_24_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_24_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_24_MODE {
        PIO0_24_MODE::from_bits(val)
    }
}
impl From<PIO0_24_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_24_MODE) -> u8 {
        PIO0_24_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_24_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_24_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_24_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_24_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_24_OD {
        PIO0_24_OD::from_bits(val)
    }
}
impl From<PIO0_24_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_24_OD) -> u8 {
        PIO0_24_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_24_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_24_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_24_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_24_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_24_SLEW {
        PIO0_24_SLEW::from_bits(val)
    }
}
impl From<PIO0_24_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_24_SLEW) -> u8 {
        PIO0_24_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_25_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_25_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_25_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_25_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_25_DIGIMODE {
        PIO0_25_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_25_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_25_DIGIMODE) -> u8 {
        PIO0_25_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_25_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_25_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_25_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_25_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_25_FUNC {
        PIO0_25_FUNC::from_bits(val)
    }
}
impl From<PIO0_25_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_25_FUNC) -> u8 {
        PIO0_25_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_25_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_25_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_25_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_25_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_25_MODE {
        PIO0_25_MODE::from_bits(val)
    }
}
impl From<PIO0_25_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_25_MODE) -> u8 {
        PIO0_25_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_25_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_25_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_25_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_25_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_25_OD {
        PIO0_25_OD::from_bits(val)
    }
}
impl From<PIO0_25_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_25_OD) -> u8 {
        PIO0_25_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_25_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_25_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_25_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_25_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_25_SLEW {
        PIO0_25_SLEW::from_bits(val)
    }
}
impl From<PIO0_25_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_25_SLEW) -> u8 {
        PIO0_25_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_26_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_26_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_26_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_26_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_26_DIGIMODE {
        PIO0_26_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_26_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_26_DIGIMODE) -> u8 {
        PIO0_26_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_26_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_26_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_26_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_26_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_26_FUNC {
        PIO0_26_FUNC::from_bits(val)
    }
}
impl From<PIO0_26_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_26_FUNC) -> u8 {
        PIO0_26_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_26_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_26_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_26_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_26_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_26_MODE {
        PIO0_26_MODE::from_bits(val)
    }
}
impl From<PIO0_26_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_26_MODE) -> u8 {
        PIO0_26_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_26_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_26_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_26_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_26_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_26_OD {
        PIO0_26_OD::from_bits(val)
    }
}
impl From<PIO0_26_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_26_OD) -> u8 {
        PIO0_26_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_26_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_26_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_26_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_26_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_26_SLEW {
        PIO0_26_SLEW::from_bits(val)
    }
}
impl From<PIO0_26_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_26_SLEW) -> u8 {
        PIO0_26_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_27_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_27_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_27_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_27_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_27_DIGIMODE {
        PIO0_27_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_27_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_27_DIGIMODE) -> u8 {
        PIO0_27_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_27_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_27_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_27_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_27_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_27_FUNC {
        PIO0_27_FUNC::from_bits(val)
    }
}
impl From<PIO0_27_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_27_FUNC) -> u8 {
        PIO0_27_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_27_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_27_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_27_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_27_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_27_MODE {
        PIO0_27_MODE::from_bits(val)
    }
}
impl From<PIO0_27_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_27_MODE) -> u8 {
        PIO0_27_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_27_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_27_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_27_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_27_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_27_OD {
        PIO0_27_OD::from_bits(val)
    }
}
impl From<PIO0_27_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_27_OD) -> u8 {
        PIO0_27_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_27_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_27_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_27_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_27_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_27_SLEW {
        PIO0_27_SLEW::from_bits(val)
    }
}
impl From<PIO0_27_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_27_SLEW) -> u8 {
        PIO0_27_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_28_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_28_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_28_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_28_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_28_DIGIMODE {
        PIO0_28_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_28_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_28_DIGIMODE) -> u8 {
        PIO0_28_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_28_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_28_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_28_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_28_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_28_FUNC {
        PIO0_28_FUNC::from_bits(val)
    }
}
impl From<PIO0_28_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_28_FUNC) -> u8 {
        PIO0_28_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_28_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_28_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_28_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_28_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_28_MODE {
        PIO0_28_MODE::from_bits(val)
    }
}
impl From<PIO0_28_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_28_MODE) -> u8 {
        PIO0_28_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_28_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_28_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_28_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_28_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_28_OD {
        PIO0_28_OD::from_bits(val)
    }
}
impl From<PIO0_28_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_28_OD) -> u8 {
        PIO0_28_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_28_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_28_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_28_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_28_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_28_SLEW {
        PIO0_28_SLEW::from_bits(val)
    }
}
impl From<PIO0_28_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_28_SLEW) -> u8 {
        PIO0_28_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_29_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_29_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_29_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_29_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_29_DIGIMODE {
        PIO0_29_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_29_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_29_DIGIMODE) -> u8 {
        PIO0_29_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_29_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_29_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_29_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_29_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_29_FUNC {
        PIO0_29_FUNC::from_bits(val)
    }
}
impl From<PIO0_29_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_29_FUNC) -> u8 {
        PIO0_29_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_29_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_29_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_29_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_29_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_29_MODE {
        PIO0_29_MODE::from_bits(val)
    }
}
impl From<PIO0_29_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_29_MODE) -> u8 {
        PIO0_29_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_29_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_29_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_29_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_29_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_29_OD {
        PIO0_29_OD::from_bits(val)
    }
}
impl From<PIO0_29_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_29_OD) -> u8 {
        PIO0_29_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_29_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_29_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_29_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_29_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_29_SLEW {
        PIO0_29_SLEW::from_bits(val)
    }
}
impl From<PIO0_29_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_29_SLEW) -> u8 {
        PIO0_29_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_2_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_2_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_2_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_2_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_2_DIGIMODE {
        PIO0_2_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_2_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_2_DIGIMODE) -> u8 {
        PIO0_2_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_2_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_2_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_2_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_2_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_2_FUNC {
        PIO0_2_FUNC::from_bits(val)
    }
}
impl From<PIO0_2_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_2_FUNC) -> u8 {
        PIO0_2_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_2_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_2_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_2_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_2_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_2_MODE {
        PIO0_2_MODE::from_bits(val)
    }
}
impl From<PIO0_2_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_2_MODE) -> u8 {
        PIO0_2_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_2_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_2_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_2_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_2_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_2_OD {
        PIO0_2_OD::from_bits(val)
    }
}
impl From<PIO0_2_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_2_OD) -> u8 {
        PIO0_2_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_2_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_2_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_2_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_2_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_2_SLEW {
        PIO0_2_SLEW::from_bits(val)
    }
}
impl From<PIO0_2_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_2_SLEW) -> u8 {
        PIO0_2_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_30_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_30_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_30_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_30_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_30_DIGIMODE {
        PIO0_30_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_30_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_30_DIGIMODE) -> u8 {
        PIO0_30_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_30_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_30_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_30_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_30_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_30_FUNC {
        PIO0_30_FUNC::from_bits(val)
    }
}
impl From<PIO0_30_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_30_FUNC) -> u8 {
        PIO0_30_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_30_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_30_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_30_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_30_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_30_MODE {
        PIO0_30_MODE::from_bits(val)
    }
}
impl From<PIO0_30_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_30_MODE) -> u8 {
        PIO0_30_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_30_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_30_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_30_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_30_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_30_OD {
        PIO0_30_OD::from_bits(val)
    }
}
impl From<PIO0_30_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_30_OD) -> u8 {
        PIO0_30_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_30_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_30_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_30_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_30_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_30_SLEW {
        PIO0_30_SLEW::from_bits(val)
    }
}
impl From<PIO0_30_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_30_SLEW) -> u8 {
        PIO0_30_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_31_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_ASW {
        PIO0_31_ASW::from_bits(val)
    }
}
impl From<PIO0_31_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_ASW) -> u8 {
        PIO0_31_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_31_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_DIGIMODE {
        PIO0_31_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_31_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_DIGIMODE) -> u8 {
        PIO0_31_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_31_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_FUNC {
        PIO0_31_FUNC::from_bits(val)
    }
}
impl From<PIO0_31_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_FUNC) -> u8 {
        PIO0_31_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_31_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_MODE {
        PIO0_31_MODE::from_bits(val)
    }
}
impl From<PIO0_31_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_MODE) -> u8 {
        PIO0_31_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_31_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_OD {
        PIO0_31_OD::from_bits(val)
    }
}
impl From<PIO0_31_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_OD) -> u8 {
        PIO0_31_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_31_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_31_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_31_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_31_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_31_SLEW {
        PIO0_31_SLEW::from_bits(val)
    }
}
impl From<PIO0_31_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_31_SLEW) -> u8 {
        PIO0_31_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_3_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_3_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_3_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_3_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_3_DIGIMODE {
        PIO0_3_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_3_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_3_DIGIMODE) -> u8 {
        PIO0_3_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_3_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_3_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_3_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_3_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_3_FUNC {
        PIO0_3_FUNC::from_bits(val)
    }
}
impl From<PIO0_3_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_3_FUNC) -> u8 {
        PIO0_3_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_3_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_3_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_3_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_3_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_3_MODE {
        PIO0_3_MODE::from_bits(val)
    }
}
impl From<PIO0_3_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_3_MODE) -> u8 {
        PIO0_3_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_3_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_3_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_3_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_3_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_3_OD {
        PIO0_3_OD::from_bits(val)
    }
}
impl From<PIO0_3_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_3_OD) -> u8 {
        PIO0_3_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_3_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_3_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_3_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_3_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_3_SLEW {
        PIO0_3_SLEW::from_bits(val)
    }
}
impl From<PIO0_3_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_3_SLEW) -> u8 {
        PIO0_3_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_4_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_4_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_4_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_4_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_4_DIGIMODE {
        PIO0_4_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_4_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_4_DIGIMODE) -> u8 {
        PIO0_4_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_4_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_4_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_4_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_4_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_4_FUNC {
        PIO0_4_FUNC::from_bits(val)
    }
}
impl From<PIO0_4_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_4_FUNC) -> u8 {
        PIO0_4_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_4_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_4_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_4_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_4_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_4_MODE {
        PIO0_4_MODE::from_bits(val)
    }
}
impl From<PIO0_4_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_4_MODE) -> u8 {
        PIO0_4_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_4_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_4_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_4_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_4_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_4_OD {
        PIO0_4_OD::from_bits(val)
    }
}
impl From<PIO0_4_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_4_OD) -> u8 {
        PIO0_4_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_4_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_4_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_4_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_4_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_4_SLEW {
        PIO0_4_SLEW::from_bits(val)
    }
}
impl From<PIO0_4_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_4_SLEW) -> u8 {
        PIO0_4_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_5_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_5_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_5_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_5_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_5_DIGIMODE {
        PIO0_5_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_5_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_5_DIGIMODE) -> u8 {
        PIO0_5_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_5_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_5_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_5_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_5_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_5_FUNC {
        PIO0_5_FUNC::from_bits(val)
    }
}
impl From<PIO0_5_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_5_FUNC) -> u8 {
        PIO0_5_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_5_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_5_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_5_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_5_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_5_MODE {
        PIO0_5_MODE::from_bits(val)
    }
}
impl From<PIO0_5_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_5_MODE) -> u8 {
        PIO0_5_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_5_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_5_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_5_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_5_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_5_OD {
        PIO0_5_OD::from_bits(val)
    }
}
impl From<PIO0_5_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_5_OD) -> u8 {
        PIO0_5_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_5_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_5_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_5_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_5_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_5_SLEW {
        PIO0_5_SLEW::from_bits(val)
    }
}
impl From<PIO0_5_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_5_SLEW) -> u8 {
        PIO0_5_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_6_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_6_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_6_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_6_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_6_DIGIMODE {
        PIO0_6_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_6_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_6_DIGIMODE) -> u8 {
        PIO0_6_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_6_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_6_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_6_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_6_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_6_FUNC {
        PIO0_6_FUNC::from_bits(val)
    }
}
impl From<PIO0_6_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_6_FUNC) -> u8 {
        PIO0_6_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_6_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_6_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_6_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_6_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_6_MODE {
        PIO0_6_MODE::from_bits(val)
    }
}
impl From<PIO0_6_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_6_MODE) -> u8 {
        PIO0_6_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_6_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_6_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_6_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_6_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_6_OD {
        PIO0_6_OD::from_bits(val)
    }
}
impl From<PIO0_6_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_6_OD) -> u8 {
        PIO0_6_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_6_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_6_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_6_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_6_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_6_SLEW {
        PIO0_6_SLEW::from_bits(val)
    }
}
impl From<PIO0_6_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_6_SLEW) -> u8 {
        PIO0_6_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_7_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_7_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_7_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_7_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_7_DIGIMODE {
        PIO0_7_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_7_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_7_DIGIMODE) -> u8 {
        PIO0_7_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_7_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_7_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_7_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_7_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_7_FUNC {
        PIO0_7_FUNC::from_bits(val)
    }
}
impl From<PIO0_7_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_7_FUNC) -> u8 {
        PIO0_7_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_7_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_7_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_7_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_7_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_7_MODE {
        PIO0_7_MODE::from_bits(val)
    }
}
impl From<PIO0_7_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_7_MODE) -> u8 {
        PIO0_7_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_7_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_7_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_7_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_7_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_7_OD {
        PIO0_7_OD::from_bits(val)
    }
}
impl From<PIO0_7_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_7_OD) -> u8 {
        PIO0_7_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_7_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_7_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_7_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_7_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_7_SLEW {
        PIO0_7_SLEW::from_bits(val)
    }
}
impl From<PIO0_7_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_7_SLEW) -> u8 {
        PIO0_7_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_8_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_8_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_8_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_8_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_8_DIGIMODE {
        PIO0_8_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_8_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_8_DIGIMODE) -> u8 {
        PIO0_8_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_8_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_8_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_8_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_8_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_8_FUNC {
        PIO0_8_FUNC::from_bits(val)
    }
}
impl From<PIO0_8_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_8_FUNC) -> u8 {
        PIO0_8_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_8_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_8_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_8_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_8_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_8_MODE {
        PIO0_8_MODE::from_bits(val)
    }
}
impl From<PIO0_8_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_8_MODE) -> u8 {
        PIO0_8_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_8_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_8_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_8_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_8_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_8_OD {
        PIO0_8_OD::from_bits(val)
    }
}
impl From<PIO0_8_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_8_OD) -> u8 {
        PIO0_8_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_8_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_8_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_8_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_8_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_8_SLEW {
        PIO0_8_SLEW::from_bits(val)
    }
}
impl From<PIO0_8_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_8_SLEW) -> u8 {
        PIO0_8_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO0_9_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_ASW {
        PIO0_9_ASW::from_bits(val)
    }
}
impl From<PIO0_9_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_ASW) -> u8 {
        PIO0_9_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO0_9_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_DIGIMODE {
        PIO0_9_DIGIMODE::from_bits(val)
    }
}
impl From<PIO0_9_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_DIGIMODE) -> u8 {
        PIO0_9_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO0_9_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_FUNC {
        PIO0_9_FUNC::from_bits(val)
    }
}
impl From<PIO0_9_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_FUNC) -> u8 {
        PIO0_9_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO0_9_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_MODE {
        PIO0_9_MODE::from_bits(val)
    }
}
impl From<PIO0_9_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_MODE) -> u8 {
        PIO0_9_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO0_9_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_OD {
        PIO0_9_OD::from_bits(val)
    }
}
impl From<PIO0_9_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_OD) -> u8 {
        PIO0_9_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO0_9_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO0_9_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO0_9_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO0_9_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO0_9_SLEW {
        PIO0_9_SLEW::from_bits(val)
    }
}
impl From<PIO0_9_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO0_9_SLEW) -> u8 {
        PIO0_9_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO1_0_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_ASW {
        PIO1_0_ASW::from_bits(val)
    }
}
impl From<PIO1_0_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_ASW) -> u8 {
        PIO1_0_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_0_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_DIGIMODE {
        PIO1_0_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_0_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_DIGIMODE) -> u8 {
        PIO1_0_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_0_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_FUNC {
        PIO1_0_FUNC::from_bits(val)
    }
}
impl From<PIO1_0_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_FUNC) -> u8 {
        PIO1_0_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_0_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_MODE {
        PIO1_0_MODE::from_bits(val)
    }
}
impl From<PIO1_0_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_MODE) -> u8 {
        PIO1_0_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_0_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_OD {
        PIO1_0_OD::from_bits(val)
    }
}
impl From<PIO1_0_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_OD) -> u8 {
        PIO1_0_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_0_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_0_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_0_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_0_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_0_SLEW {
        PIO1_0_SLEW::from_bits(val)
    }
}
impl From<PIO1_0_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_0_SLEW) -> u8 {
        PIO1_0_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_10_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_10_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_10_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_10_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_10_DIGIMODE {
        PIO1_10_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_10_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_10_DIGIMODE) -> u8 {
        PIO1_10_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_10_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_10_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_10_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_10_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_10_FUNC {
        PIO1_10_FUNC::from_bits(val)
    }
}
impl From<PIO1_10_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_10_FUNC) -> u8 {
        PIO1_10_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_10_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_10_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_10_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_10_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_10_MODE {
        PIO1_10_MODE::from_bits(val)
    }
}
impl From<PIO1_10_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_10_MODE) -> u8 {
        PIO1_10_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_10_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_10_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_10_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_10_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_10_OD {
        PIO1_10_OD::from_bits(val)
    }
}
impl From<PIO1_10_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_10_OD) -> u8 {
        PIO1_10_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_10_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_10_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_10_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_10_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_10_SLEW {
        PIO1_10_SLEW::from_bits(val)
    }
}
impl From<PIO1_10_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_10_SLEW) -> u8 {
        PIO1_10_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_11_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_11_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_11_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_11_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_11_DIGIMODE {
        PIO1_11_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_11_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_11_DIGIMODE) -> u8 {
        PIO1_11_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_11_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_11_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_11_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_11_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_11_FUNC {
        PIO1_11_FUNC::from_bits(val)
    }
}
impl From<PIO1_11_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_11_FUNC) -> u8 {
        PIO1_11_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_11_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_11_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_11_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_11_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_11_MODE {
        PIO1_11_MODE::from_bits(val)
    }
}
impl From<PIO1_11_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_11_MODE) -> u8 {
        PIO1_11_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_11_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_11_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_11_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_11_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_11_OD {
        PIO1_11_OD::from_bits(val)
    }
}
impl From<PIO1_11_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_11_OD) -> u8 {
        PIO1_11_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_11_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_11_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_11_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_11_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_11_SLEW {
        PIO1_11_SLEW::from_bits(val)
    }
}
impl From<PIO1_11_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_11_SLEW) -> u8 {
        PIO1_11_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_12_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_12_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_12_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_12_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_12_DIGIMODE {
        PIO1_12_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_12_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_12_DIGIMODE) -> u8 {
        PIO1_12_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_12_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_12_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_12_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_12_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_12_FUNC {
        PIO1_12_FUNC::from_bits(val)
    }
}
impl From<PIO1_12_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_12_FUNC) -> u8 {
        PIO1_12_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_12_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_12_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_12_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_12_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_12_MODE {
        PIO1_12_MODE::from_bits(val)
    }
}
impl From<PIO1_12_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_12_MODE) -> u8 {
        PIO1_12_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_12_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_12_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_12_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_12_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_12_OD {
        PIO1_12_OD::from_bits(val)
    }
}
impl From<PIO1_12_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_12_OD) -> u8 {
        PIO1_12_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_12_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_12_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_12_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_12_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_12_SLEW {
        PIO1_12_SLEW::from_bits(val)
    }
}
impl From<PIO1_12_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_12_SLEW) -> u8 {
        PIO1_12_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_13_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_13_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_13_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_13_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_13_DIGIMODE {
        PIO1_13_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_13_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_13_DIGIMODE) -> u8 {
        PIO1_13_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_13_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_13_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_13_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_13_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_13_FUNC {
        PIO1_13_FUNC::from_bits(val)
    }
}
impl From<PIO1_13_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_13_FUNC) -> u8 {
        PIO1_13_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_13_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_13_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_13_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_13_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_13_MODE {
        PIO1_13_MODE::from_bits(val)
    }
}
impl From<PIO1_13_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_13_MODE) -> u8 {
        PIO1_13_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_13_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_13_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_13_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_13_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_13_OD {
        PIO1_13_OD::from_bits(val)
    }
}
impl From<PIO1_13_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_13_OD) -> u8 {
        PIO1_13_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_13_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_13_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_13_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_13_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_13_SLEW {
        PIO1_13_SLEW::from_bits(val)
    }
}
impl From<PIO1_13_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_13_SLEW) -> u8 {
        PIO1_13_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO1_14_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_ASW {
        PIO1_14_ASW::from_bits(val)
    }
}
impl From<PIO1_14_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_ASW) -> u8 {
        PIO1_14_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_14_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_DIGIMODE {
        PIO1_14_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_14_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_DIGIMODE) -> u8 {
        PIO1_14_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_14_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_FUNC {
        PIO1_14_FUNC::from_bits(val)
    }
}
impl From<PIO1_14_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_FUNC) -> u8 {
        PIO1_14_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_14_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_MODE {
        PIO1_14_MODE::from_bits(val)
    }
}
impl From<PIO1_14_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_MODE) -> u8 {
        PIO1_14_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_14_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_OD {
        PIO1_14_OD::from_bits(val)
    }
}
impl From<PIO1_14_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_OD) -> u8 {
        PIO1_14_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_14_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_14_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_14_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_14_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_14_SLEW {
        PIO1_14_SLEW::from_bits(val)
    }
}
impl From<PIO1_14_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_14_SLEW) -> u8 {
        PIO1_14_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_15_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_15_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_15_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_15_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_15_DIGIMODE {
        PIO1_15_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_15_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_15_DIGIMODE) -> u8 {
        PIO1_15_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_15_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_15_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_15_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_15_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_15_FUNC {
        PIO1_15_FUNC::from_bits(val)
    }
}
impl From<PIO1_15_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_15_FUNC) -> u8 {
        PIO1_15_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_15_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_15_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_15_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_15_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_15_MODE {
        PIO1_15_MODE::from_bits(val)
    }
}
impl From<PIO1_15_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_15_MODE) -> u8 {
        PIO1_15_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_15_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_15_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_15_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_15_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_15_OD {
        PIO1_15_OD::from_bits(val)
    }
}
impl From<PIO1_15_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_15_OD) -> u8 {
        PIO1_15_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_15_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_15_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_15_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_15_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_15_SLEW {
        PIO1_15_SLEW::from_bits(val)
    }
}
impl From<PIO1_15_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_15_SLEW) -> u8 {
        PIO1_15_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_16_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_16_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_16_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_16_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_16_DIGIMODE {
        PIO1_16_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_16_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_16_DIGIMODE) -> u8 {
        PIO1_16_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_16_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_16_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_16_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_16_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_16_FUNC {
        PIO1_16_FUNC::from_bits(val)
    }
}
impl From<PIO1_16_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_16_FUNC) -> u8 {
        PIO1_16_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_16_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_16_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_16_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_16_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_16_MODE {
        PIO1_16_MODE::from_bits(val)
    }
}
impl From<PIO1_16_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_16_MODE) -> u8 {
        PIO1_16_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_16_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_16_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_16_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_16_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_16_OD {
        PIO1_16_OD::from_bits(val)
    }
}
impl From<PIO1_16_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_16_OD) -> u8 {
        PIO1_16_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_16_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_16_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_16_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_16_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_16_SLEW {
        PIO1_16_SLEW::from_bits(val)
    }
}
impl From<PIO1_16_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_16_SLEW) -> u8 {
        PIO1_16_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_17_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_17_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_17_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_17_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_17_DIGIMODE {
        PIO1_17_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_17_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_17_DIGIMODE) -> u8 {
        PIO1_17_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_17_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_17_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_17_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_17_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_17_FUNC {
        PIO1_17_FUNC::from_bits(val)
    }
}
impl From<PIO1_17_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_17_FUNC) -> u8 {
        PIO1_17_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_17_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_17_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_17_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_17_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_17_MODE {
        PIO1_17_MODE::from_bits(val)
    }
}
impl From<PIO1_17_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_17_MODE) -> u8 {
        PIO1_17_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_17_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_17_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_17_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_17_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_17_OD {
        PIO1_17_OD::from_bits(val)
    }
}
impl From<PIO1_17_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_17_OD) -> u8 {
        PIO1_17_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_17_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_17_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_17_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_17_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_17_SLEW {
        PIO1_17_SLEW::from_bits(val)
    }
}
impl From<PIO1_17_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_17_SLEW) -> u8 {
        PIO1_17_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_18_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_18_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_18_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_18_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_18_DIGIMODE {
        PIO1_18_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_18_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_18_DIGIMODE) -> u8 {
        PIO1_18_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_18_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_18_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_18_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_18_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_18_FUNC {
        PIO1_18_FUNC::from_bits(val)
    }
}
impl From<PIO1_18_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_18_FUNC) -> u8 {
        PIO1_18_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_18_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_18_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_18_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_18_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_18_MODE {
        PIO1_18_MODE::from_bits(val)
    }
}
impl From<PIO1_18_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_18_MODE) -> u8 {
        PIO1_18_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_18_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_18_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_18_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_18_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_18_OD {
        PIO1_18_OD::from_bits(val)
    }
}
impl From<PIO1_18_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_18_OD) -> u8 {
        PIO1_18_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_18_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_18_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_18_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_18_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_18_SLEW {
        PIO1_18_SLEW::from_bits(val)
    }
}
impl From<PIO1_18_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_18_SLEW) -> u8 {
        PIO1_18_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO1_19_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_ASW {
        PIO1_19_ASW::from_bits(val)
    }
}
impl From<PIO1_19_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_ASW) -> u8 {
        PIO1_19_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_19_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_DIGIMODE {
        PIO1_19_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_19_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_DIGIMODE) -> u8 {
        PIO1_19_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_19_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_FUNC {
        PIO1_19_FUNC::from_bits(val)
    }
}
impl From<PIO1_19_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_FUNC) -> u8 {
        PIO1_19_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_19_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_MODE {
        PIO1_19_MODE::from_bits(val)
    }
}
impl From<PIO1_19_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_MODE) -> u8 {
        PIO1_19_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_19_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_OD {
        PIO1_19_OD::from_bits(val)
    }
}
impl From<PIO1_19_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_OD) -> u8 {
        PIO1_19_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_19_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_19_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_19_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_19_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_19_SLEW {
        PIO1_19_SLEW::from_bits(val)
    }
}
impl From<PIO1_19_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_19_SLEW) -> u8 {
        PIO1_19_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_1_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_1_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_1_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_1_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_1_DIGIMODE {
        PIO1_1_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_1_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_1_DIGIMODE) -> u8 {
        PIO1_1_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_1_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_1_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_1_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_1_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_1_FUNC {
        PIO1_1_FUNC::from_bits(val)
    }
}
impl From<PIO1_1_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_1_FUNC) -> u8 {
        PIO1_1_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_1_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_1_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_1_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_1_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_1_MODE {
        PIO1_1_MODE::from_bits(val)
    }
}
impl From<PIO1_1_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_1_MODE) -> u8 {
        PIO1_1_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_1_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_1_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_1_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_1_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_1_OD {
        PIO1_1_OD::from_bits(val)
    }
}
impl From<PIO1_1_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_1_OD) -> u8 {
        PIO1_1_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_1_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_1_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_1_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_1_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_1_SLEW {
        PIO1_1_SLEW::from_bits(val)
    }
}
impl From<PIO1_1_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_1_SLEW) -> u8 {
        PIO1_1_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_20_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_20_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_20_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_20_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_20_DIGIMODE {
        PIO1_20_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_20_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_20_DIGIMODE) -> u8 {
        PIO1_20_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_20_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_20_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_20_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_20_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_20_FUNC {
        PIO1_20_FUNC::from_bits(val)
    }
}
impl From<PIO1_20_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_20_FUNC) -> u8 {
        PIO1_20_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_20_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_20_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_20_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_20_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_20_MODE {
        PIO1_20_MODE::from_bits(val)
    }
}
impl From<PIO1_20_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_20_MODE) -> u8 {
        PIO1_20_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_20_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_20_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_20_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_20_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_20_OD {
        PIO1_20_OD::from_bits(val)
    }
}
impl From<PIO1_20_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_20_OD) -> u8 {
        PIO1_20_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_20_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_20_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_20_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_20_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_20_SLEW {
        PIO1_20_SLEW::from_bits(val)
    }
}
impl From<PIO1_20_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_20_SLEW) -> u8 {
        PIO1_20_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_21_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_21_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_21_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_21_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_21_DIGIMODE {
        PIO1_21_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_21_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_21_DIGIMODE) -> u8 {
        PIO1_21_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_21_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_21_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_21_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_21_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_21_FUNC {
        PIO1_21_FUNC::from_bits(val)
    }
}
impl From<PIO1_21_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_21_FUNC) -> u8 {
        PIO1_21_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_21_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_21_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_21_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_21_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_21_MODE {
        PIO1_21_MODE::from_bits(val)
    }
}
impl From<PIO1_21_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_21_MODE) -> u8 {
        PIO1_21_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_21_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_21_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_21_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_21_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_21_OD {
        PIO1_21_OD::from_bits(val)
    }
}
impl From<PIO1_21_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_21_OD) -> u8 {
        PIO1_21_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_21_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_21_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_21_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_21_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_21_SLEW {
        PIO1_21_SLEW::from_bits(val)
    }
}
impl From<PIO1_21_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_21_SLEW) -> u8 {
        PIO1_21_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_22_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_22_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_22_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_22_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_22_DIGIMODE {
        PIO1_22_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_22_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_22_DIGIMODE) -> u8 {
        PIO1_22_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_22_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_22_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_22_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_22_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_22_FUNC {
        PIO1_22_FUNC::from_bits(val)
    }
}
impl From<PIO1_22_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_22_FUNC) -> u8 {
        PIO1_22_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_22_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_22_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_22_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_22_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_22_MODE {
        PIO1_22_MODE::from_bits(val)
    }
}
impl From<PIO1_22_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_22_MODE) -> u8 {
        PIO1_22_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_22_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_22_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_22_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_22_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_22_OD {
        PIO1_22_OD::from_bits(val)
    }
}
impl From<PIO1_22_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_22_OD) -> u8 {
        PIO1_22_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_22_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_22_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_22_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_22_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_22_SLEW {
        PIO1_22_SLEW::from_bits(val)
    }
}
impl From<PIO1_22_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_22_SLEW) -> u8 {
        PIO1_22_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_23_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_23_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_23_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_23_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_23_DIGIMODE {
        PIO1_23_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_23_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_23_DIGIMODE) -> u8 {
        PIO1_23_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_23_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_23_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_23_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_23_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_23_FUNC {
        PIO1_23_FUNC::from_bits(val)
    }
}
impl From<PIO1_23_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_23_FUNC) -> u8 {
        PIO1_23_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_23_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_23_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_23_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_23_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_23_MODE {
        PIO1_23_MODE::from_bits(val)
    }
}
impl From<PIO1_23_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_23_MODE) -> u8 {
        PIO1_23_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_23_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_23_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_23_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_23_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_23_OD {
        PIO1_23_OD::from_bits(val)
    }
}
impl From<PIO1_23_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_23_OD) -> u8 {
        PIO1_23_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_23_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_23_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_23_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_23_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_23_SLEW {
        PIO1_23_SLEW::from_bits(val)
    }
}
impl From<PIO1_23_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_23_SLEW) -> u8 {
        PIO1_23_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_24_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_24_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_24_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_24_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_24_DIGIMODE {
        PIO1_24_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_24_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_24_DIGIMODE) -> u8 {
        PIO1_24_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_24_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_24_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_24_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_24_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_24_FUNC {
        PIO1_24_FUNC::from_bits(val)
    }
}
impl From<PIO1_24_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_24_FUNC) -> u8 {
        PIO1_24_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_24_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_24_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_24_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_24_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_24_MODE {
        PIO1_24_MODE::from_bits(val)
    }
}
impl From<PIO1_24_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_24_MODE) -> u8 {
        PIO1_24_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_24_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_24_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_24_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_24_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_24_OD {
        PIO1_24_OD::from_bits(val)
    }
}
impl From<PIO1_24_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_24_OD) -> u8 {
        PIO1_24_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_24_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_24_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_24_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_24_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_24_SLEW {
        PIO1_24_SLEW::from_bits(val)
    }
}
impl From<PIO1_24_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_24_SLEW) -> u8 {
        PIO1_24_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_25_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_25_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_25_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_25_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_25_DIGIMODE {
        PIO1_25_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_25_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_25_DIGIMODE) -> u8 {
        PIO1_25_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_25_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_25_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_25_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_25_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_25_FUNC {
        PIO1_25_FUNC::from_bits(val)
    }
}
impl From<PIO1_25_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_25_FUNC) -> u8 {
        PIO1_25_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_25_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_25_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_25_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_25_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_25_MODE {
        PIO1_25_MODE::from_bits(val)
    }
}
impl From<PIO1_25_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_25_MODE) -> u8 {
        PIO1_25_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_25_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_25_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_25_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_25_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_25_OD {
        PIO1_25_OD::from_bits(val)
    }
}
impl From<PIO1_25_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_25_OD) -> u8 {
        PIO1_25_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_25_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_25_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_25_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_25_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_25_SLEW {
        PIO1_25_SLEW::from_bits(val)
    }
}
impl From<PIO1_25_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_25_SLEW) -> u8 {
        PIO1_25_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_26_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_26_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_26_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_26_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_26_DIGIMODE {
        PIO1_26_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_26_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_26_DIGIMODE) -> u8 {
        PIO1_26_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_26_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_26_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_26_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_26_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_26_FUNC {
        PIO1_26_FUNC::from_bits(val)
    }
}
impl From<PIO1_26_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_26_FUNC) -> u8 {
        PIO1_26_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_26_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_26_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_26_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_26_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_26_MODE {
        PIO1_26_MODE::from_bits(val)
    }
}
impl From<PIO1_26_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_26_MODE) -> u8 {
        PIO1_26_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_26_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_26_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_26_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_26_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_26_OD {
        PIO1_26_OD::from_bits(val)
    }
}
impl From<PIO1_26_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_26_OD) -> u8 {
        PIO1_26_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_26_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_26_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_26_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_26_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_26_SLEW {
        PIO1_26_SLEW::from_bits(val)
    }
}
impl From<PIO1_26_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_26_SLEW) -> u8 {
        PIO1_26_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_27_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_27_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_27_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_27_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_27_DIGIMODE {
        PIO1_27_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_27_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_27_DIGIMODE) -> u8 {
        PIO1_27_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_27_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_27_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_27_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_27_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_27_FUNC {
        PIO1_27_FUNC::from_bits(val)
    }
}
impl From<PIO1_27_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_27_FUNC) -> u8 {
        PIO1_27_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_27_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_27_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_27_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_27_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_27_MODE {
        PIO1_27_MODE::from_bits(val)
    }
}
impl From<PIO1_27_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_27_MODE) -> u8 {
        PIO1_27_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_27_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_27_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_27_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_27_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_27_OD {
        PIO1_27_OD::from_bits(val)
    }
}
impl From<PIO1_27_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_27_OD) -> u8 {
        PIO1_27_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_27_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_27_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_27_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_27_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_27_SLEW {
        PIO1_27_SLEW::from_bits(val)
    }
}
impl From<PIO1_27_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_27_SLEW) -> u8 {
        PIO1_27_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_28_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_28_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_28_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_28_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_28_DIGIMODE {
        PIO1_28_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_28_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_28_DIGIMODE) -> u8 {
        PIO1_28_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_28_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_28_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_28_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_28_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_28_FUNC {
        PIO1_28_FUNC::from_bits(val)
    }
}
impl From<PIO1_28_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_28_FUNC) -> u8 {
        PIO1_28_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_28_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_28_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_28_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_28_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_28_MODE {
        PIO1_28_MODE::from_bits(val)
    }
}
impl From<PIO1_28_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_28_MODE) -> u8 {
        PIO1_28_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_28_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_28_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_28_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_28_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_28_OD {
        PIO1_28_OD::from_bits(val)
    }
}
impl From<PIO1_28_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_28_OD) -> u8 {
        PIO1_28_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_28_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_28_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_28_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_28_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_28_SLEW {
        PIO1_28_SLEW::from_bits(val)
    }
}
impl From<PIO1_28_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_28_SLEW) -> u8 {
        PIO1_28_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_29_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_29_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_29_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_29_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_29_DIGIMODE {
        PIO1_29_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_29_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_29_DIGIMODE) -> u8 {
        PIO1_29_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_29_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_29_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_29_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_29_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_29_FUNC {
        PIO1_29_FUNC::from_bits(val)
    }
}
impl From<PIO1_29_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_29_FUNC) -> u8 {
        PIO1_29_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_29_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_29_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_29_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_29_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_29_MODE {
        PIO1_29_MODE::from_bits(val)
    }
}
impl From<PIO1_29_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_29_MODE) -> u8 {
        PIO1_29_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_29_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_29_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_29_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_29_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_29_OD {
        PIO1_29_OD::from_bits(val)
    }
}
impl From<PIO1_29_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_29_OD) -> u8 {
        PIO1_29_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_29_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_29_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_29_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_29_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_29_SLEW {
        PIO1_29_SLEW::from_bits(val)
    }
}
impl From<PIO1_29_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_29_SLEW) -> u8 {
        PIO1_29_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_2_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_2_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_2_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_2_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_2_DIGIMODE {
        PIO1_2_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_2_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_2_DIGIMODE) -> u8 {
        PIO1_2_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_2_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_2_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_2_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_2_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_2_FUNC {
        PIO1_2_FUNC::from_bits(val)
    }
}
impl From<PIO1_2_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_2_FUNC) -> u8 {
        PIO1_2_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_2_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_2_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_2_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_2_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_2_MODE {
        PIO1_2_MODE::from_bits(val)
    }
}
impl From<PIO1_2_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_2_MODE) -> u8 {
        PIO1_2_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_2_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_2_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_2_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_2_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_2_OD {
        PIO1_2_OD::from_bits(val)
    }
}
impl From<PIO1_2_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_2_OD) -> u8 {
        PIO1_2_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_2_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_2_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_2_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_2_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_2_SLEW {
        PIO1_2_SLEW::from_bits(val)
    }
}
impl From<PIO1_2_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_2_SLEW) -> u8 {
        PIO1_2_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_30_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_30_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_30_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_30_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_30_DIGIMODE {
        PIO1_30_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_30_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_30_DIGIMODE) -> u8 {
        PIO1_30_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_30_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_30_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_30_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_30_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_30_FUNC {
        PIO1_30_FUNC::from_bits(val)
    }
}
impl From<PIO1_30_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_30_FUNC) -> u8 {
        PIO1_30_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_30_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_30_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_30_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_30_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_30_MODE {
        PIO1_30_MODE::from_bits(val)
    }
}
impl From<PIO1_30_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_30_MODE) -> u8 {
        PIO1_30_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_30_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_30_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_30_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_30_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_30_OD {
        PIO1_30_OD::from_bits(val)
    }
}
impl From<PIO1_30_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_30_OD) -> u8 {
        PIO1_30_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_30_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_30_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_30_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_30_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_30_SLEW {
        PIO1_30_SLEW::from_bits(val)
    }
}
impl From<PIO1_30_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_30_SLEW) -> u8 {
        PIO1_30_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_31_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_31_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_31_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_31_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_31_DIGIMODE {
        PIO1_31_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_31_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_31_DIGIMODE) -> u8 {
        PIO1_31_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_31_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_31_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_31_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_31_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_31_FUNC {
        PIO1_31_FUNC::from_bits(val)
    }
}
impl From<PIO1_31_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_31_FUNC) -> u8 {
        PIO1_31_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_31_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_31_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_31_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_31_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_31_MODE {
        PIO1_31_MODE::from_bits(val)
    }
}
impl From<PIO1_31_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_31_MODE) -> u8 {
        PIO1_31_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_31_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_31_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_31_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_31_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_31_OD {
        PIO1_31_OD::from_bits(val)
    }
}
impl From<PIO1_31_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_31_OD) -> u8 {
        PIO1_31_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_31_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_31_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_31_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_31_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_31_SLEW {
        PIO1_31_SLEW::from_bits(val)
    }
}
impl From<PIO1_31_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_31_SLEW) -> u8 {
        PIO1_31_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_3_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_3_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_3_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_3_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_3_DIGIMODE {
        PIO1_3_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_3_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_3_DIGIMODE) -> u8 {
        PIO1_3_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_3_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_3_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_3_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_3_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_3_FUNC {
        PIO1_3_FUNC::from_bits(val)
    }
}
impl From<PIO1_3_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_3_FUNC) -> u8 {
        PIO1_3_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_3_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_3_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_3_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_3_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_3_MODE {
        PIO1_3_MODE::from_bits(val)
    }
}
impl From<PIO1_3_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_3_MODE) -> u8 {
        PIO1_3_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_3_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_3_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_3_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_3_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_3_OD {
        PIO1_3_OD::from_bits(val)
    }
}
impl From<PIO1_3_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_3_OD) -> u8 {
        PIO1_3_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_3_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_3_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_3_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_3_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_3_SLEW {
        PIO1_3_SLEW::from_bits(val)
    }
}
impl From<PIO1_3_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_3_SLEW) -> u8 {
        PIO1_3_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_4_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_4_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_4_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_4_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_4_DIGIMODE {
        PIO1_4_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_4_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_4_DIGIMODE) -> u8 {
        PIO1_4_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_4_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_4_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_4_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_4_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_4_FUNC {
        PIO1_4_FUNC::from_bits(val)
    }
}
impl From<PIO1_4_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_4_FUNC) -> u8 {
        PIO1_4_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_4_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_4_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_4_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_4_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_4_MODE {
        PIO1_4_MODE::from_bits(val)
    }
}
impl From<PIO1_4_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_4_MODE) -> u8 {
        PIO1_4_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_4_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_4_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_4_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_4_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_4_OD {
        PIO1_4_OD::from_bits(val)
    }
}
impl From<PIO1_4_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_4_OD) -> u8 {
        PIO1_4_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_4_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_4_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_4_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_4_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_4_SLEW {
        PIO1_4_SLEW::from_bits(val)
    }
}
impl From<PIO1_4_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_4_SLEW) -> u8 {
        PIO1_4_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_5_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_5_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_5_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_5_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_5_DIGIMODE {
        PIO1_5_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_5_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_5_DIGIMODE) -> u8 {
        PIO1_5_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_5_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_5_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_5_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_5_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_5_FUNC {
        PIO1_5_FUNC::from_bits(val)
    }
}
impl From<PIO1_5_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_5_FUNC) -> u8 {
        PIO1_5_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_5_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_5_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_5_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_5_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_5_MODE {
        PIO1_5_MODE::from_bits(val)
    }
}
impl From<PIO1_5_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_5_MODE) -> u8 {
        PIO1_5_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_5_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_5_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_5_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_5_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_5_OD {
        PIO1_5_OD::from_bits(val)
    }
}
impl From<PIO1_5_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_5_OD) -> u8 {
        PIO1_5_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_5_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_5_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_5_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_5_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_5_SLEW {
        PIO1_5_SLEW::from_bits(val)
    }
}
impl From<PIO1_5_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_5_SLEW) -> u8 {
        PIO1_5_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_6_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_6_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_6_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_6_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_6_DIGIMODE {
        PIO1_6_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_6_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_6_DIGIMODE) -> u8 {
        PIO1_6_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_6_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_6_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_6_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_6_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_6_FUNC {
        PIO1_6_FUNC::from_bits(val)
    }
}
impl From<PIO1_6_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_6_FUNC) -> u8 {
        PIO1_6_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_6_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_6_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_6_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_6_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_6_MODE {
        PIO1_6_MODE::from_bits(val)
    }
}
impl From<PIO1_6_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_6_MODE) -> u8 {
        PIO1_6_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_6_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_6_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_6_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_6_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_6_OD {
        PIO1_6_OD::from_bits(val)
    }
}
impl From<PIO1_6_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_6_OD) -> u8 {
        PIO1_6_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_6_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_6_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_6_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_6_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_6_SLEW {
        PIO1_6_SLEW::from_bits(val)
    }
}
impl From<PIO1_6_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_6_SLEW) -> u8 {
        PIO1_6_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_7_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_7_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_7_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_7_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_7_DIGIMODE {
        PIO1_7_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_7_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_7_DIGIMODE) -> u8 {
        PIO1_7_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_7_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_7_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_7_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_7_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_7_FUNC {
        PIO1_7_FUNC::from_bits(val)
    }
}
impl From<PIO1_7_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_7_FUNC) -> u8 {
        PIO1_7_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_7_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_7_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_7_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_7_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_7_MODE {
        PIO1_7_MODE::from_bits(val)
    }
}
impl From<PIO1_7_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_7_MODE) -> u8 {
        PIO1_7_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_7_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_7_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_7_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_7_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_7_OD {
        PIO1_7_OD::from_bits(val)
    }
}
impl From<PIO1_7_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_7_OD) -> u8 {
        PIO1_7_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_7_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_7_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_7_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_7_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_7_SLEW {
        PIO1_7_SLEW::from_bits(val)
    }
}
impl From<PIO1_7_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_7_SLEW) -> u8 {
        PIO1_7_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO1_8_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_ASW {
        PIO1_8_ASW::from_bits(val)
    }
}
impl From<PIO1_8_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_ASW) -> u8 {
        PIO1_8_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_8_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_DIGIMODE {
        PIO1_8_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_8_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_DIGIMODE) -> u8 {
        PIO1_8_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_8_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_FUNC {
        PIO1_8_FUNC::from_bits(val)
    }
}
impl From<PIO1_8_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_FUNC) -> u8 {
        PIO1_8_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_8_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_MODE {
        PIO1_8_MODE::from_bits(val)
    }
}
impl From<PIO1_8_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_MODE) -> u8 {
        PIO1_8_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_8_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_OD {
        PIO1_8_OD::from_bits(val)
    }
}
impl From<PIO1_8_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_OD) -> u8 {
        PIO1_8_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_8_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_8_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_8_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_8_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_8_SLEW {
        PIO1_8_SLEW::from_bits(val)
    }
}
impl From<PIO1_8_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_8_SLEW) -> u8 {
        PIO1_8_SLEW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_ASW {
    #[doc = "For pins PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9, analog switch is closed (enabled). For the other pins, analog switch is open (disabled)."]
    VALUE0 = 0x0,
    #[doc = "For all pins except PIO0_9, PIO0_11, PIO0_12, PIO0_15, PIO0_18, PIO0_31, PIO1_0 and PIO1_9 analog switch is closed (enabled)."]
    VALUE1 = 0x01,
}
impl PIO1_9_ASW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_ASW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_ASW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_ASW {
        PIO1_9_ASW::from_bits(val)
    }
}
impl From<PIO1_9_ASW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_ASW) -> u8 {
        PIO1_9_ASW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_DIGIMODE {
    #[doc = "Disable digital mode. Digital input set to 0."]
    ANALOG = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    DIGITAL = 0x01,
}
impl PIO1_9_DIGIMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_DIGIMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_DIGIMODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_DIGIMODE {
        PIO1_9_DIGIMODE::from_bits(val)
    }
}
impl From<PIO1_9_DIGIMODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_DIGIMODE) -> u8 {
        PIO1_9_DIGIMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_FUNC {
    #[doc = "Alternative connection 0."]
    ALT0 = 0x0,
    #[doc = "Alternative connection 1."]
    ALT1 = 0x01,
    #[doc = "Alternative connection 2."]
    ALT2 = 0x02,
    #[doc = "Alternative connection 3."]
    ALT3 = 0x03,
    #[doc = "Alternative connection 4."]
    ALT4 = 0x04,
    #[doc = "Alternative connection 5."]
    ALT5 = 0x05,
    #[doc = "Alternative connection 6."]
    ALT6 = 0x06,
    #[doc = "Alternative connection 7."]
    ALT7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PIO1_9_FUNC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_FUNC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_FUNC {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_FUNC {
        PIO1_9_FUNC::from_bits(val)
    }
}
impl From<PIO1_9_FUNC> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_FUNC) -> u8 {
        PIO1_9_FUNC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_MODE {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PULL_UP = 0x02,
    #[doc = "Repeater. Repeater mode."]
    REPEATER = 0x03,
}
impl PIO1_9_MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_MODE {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_MODE {
        PIO1_9_MODE::from_bits(val)
    }
}
impl From<PIO1_9_MODE> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_MODE) -> u8 {
        PIO1_9_MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_OD {
    #[doc = "Normal. Normal push-pull output."]
    NORMAL = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 0x01,
}
impl PIO1_9_OD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_OD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_OD {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_OD {
        PIO1_9_OD::from_bits(val)
    }
}
impl From<PIO1_9_OD> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_OD) -> u8 {
        PIO1_9_OD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIO1_9_SLEW {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 0x01,
}
impl PIO1_9_SLEW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIO1_9_SLEW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIO1_9_SLEW {
    #[inline(always)]
    fn from(val: u8) -> PIO1_9_SLEW {
        PIO1_9_SLEW::from_bits(val)
    }
}
impl From<PIO1_9_SLEW> for u8 {
    #[inline(always)]
    fn from(val: PIO1_9_SLEW) -> u8 {
        PIO1_9_SLEW::to_bits(val)
    }
}
