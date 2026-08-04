#[doc = "Analog switch input control."]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioAsw {
    #[doc = "Analog switch is disabled."]
    disabled = 0x0,
    #[doc = "Analog switch is enabled."]
    enabled = 0x01,
}
impl PioAsw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioAsw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioAsw {
    #[inline(always)]
    fn from(val: u8) -> PioAsw {
        PioAsw::from_bits(val)
    }
}
impl From<PioAsw> for u8 {
    #[inline(always)]
    fn from(val: PioAsw) -> u8 {
        PioAsw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioDigimode {
    #[doc = "Disable digital mode. Digital input set to 0."]
    Analog = 0x0,
    #[doc = "Enable Digital mode. Digital input is enabled."]
    Digital = 0x01,
}
impl PioDigimode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioDigimode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioDigimode {
    #[inline(always)]
    fn from(val: u8) -> PioDigimode {
        PioDigimode::from_bits(val)
    }
}
impl From<PioDigimode> for u8 {
    #[inline(always)]
    fn from(val: PioDigimode) -> u8 {
        PioDigimode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioEgp {
    #[doc = "I2C mode."]
    I2cMode = 0x0,
    #[doc = "GPIO mode."]
    GpioMode = 0x01,
}
impl PioEgp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioEgp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioEgp {
    #[inline(always)]
    fn from(val: u8) -> PioEgp {
        PioEgp::from_bits(val)
    }
}
impl From<PioEgp> for u8 {
    #[inline(always)]
    fn from(val: PioEgp) -> u8 {
        PioEgp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioFilteroff {
    #[doc = "Filter enabled."]
    Enabled = 0x0,
    #[doc = "Filter disabled."]
    Disabled = 0x01,
}
impl PioFilteroff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioFilteroff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioFilteroff {
    #[inline(always)]
    fn from(val: u8) -> PioFilteroff {
        PioFilteroff::from_bits(val)
    }
}
impl From<PioFilteroff> for u8 {
    #[inline(always)]
    fn from(val: PioFilteroff) -> u8 {
        PioFilteroff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioFunc {
    #[doc = "Alternative connection 0."]
    Alt0 = 0x0,
    #[doc = "Alternative connection 1."]
    Alt1 = 0x01,
    #[doc = "Alternative connection 2."]
    Alt2 = 0x02,
    #[doc = "Alternative connection 3."]
    Alt3 = 0x03,
    #[doc = "Alternative connection 4."]
    Alt4 = 0x04,
    #[doc = "Alternative connection 5."]
    Alt5 = 0x05,
    #[doc = "Alternative connection 6."]
    Alt6 = 0x06,
    #[doc = "Alternative connection 7."]
    Alt7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PioFunc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioFunc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioFunc {
    #[inline(always)]
    fn from(val: u8) -> PioFunc {
        PioFunc::from_bits(val)
    }
}
impl From<PioFunc> for u8 {
    #[inline(always)]
    fn from(val: PioFunc) -> u8 {
        PioFunc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioI2cfilter {
    #[doc = "I2C 50 ns glitch filter enabled. Typically used for Standard-mode, Fast-mode and Fast-mode Plus I2C."]
    FastMode = 0x0,
    #[doc = "I2C 10 ns glitch filter enabled. Typically used for High-speed mode I2C."]
    StandardMode = 0x01,
}
impl PioI2cfilter {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioI2cfilter {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioI2cfilter {
    #[inline(always)]
    fn from(val: u8) -> PioI2cfilter {
        PioI2cfilter::from_bits(val)
    }
}
impl From<PioI2cfilter> for u8 {
    #[inline(always)]
    fn from(val: PioI2cfilter) -> u8 {
        PioI2cfilter::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioMode {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0x0,
    #[doc = "Pull-down. Pull-down resistor enabled."]
    PullDown = 0x01,
    #[doc = "Pull-up. Pull-up resistor enabled."]
    PullUp = 0x02,
    #[doc = "Repeater. Repeater mode."]
    Repeater = 0x03,
}
impl PioMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioMode {
    #[inline(always)]
    fn from(val: u8) -> PioMode {
        PioMode::from_bits(val)
    }
}
impl From<PioMode> for u8 {
    #[inline(always)]
    fn from(val: PioMode) -> u8 {
        PioMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioOd {
    #[doc = "Normal. Normal push-pull output."]
    Normal = 0x0,
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    OpenDrain = 0x01,
}
impl PioOd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioOd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioOd {
    #[inline(always)]
    fn from(val: u8) -> PioOd {
        PioOd::from_bits(val)
    }
}
impl From<PioOd> for u8 {
    #[inline(always)]
    fn from(val: PioOd) -> u8 {
        PioOd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioSlew {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    Standard = 0x0,
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    Fast = 0x01,
}
impl PioSlew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioSlew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioSlew {
    #[inline(always)]
    fn from(val: u8) -> PioSlew {
        PioSlew::from_bits(val)
    }
}
impl From<PioSlew> for u8 {
    #[inline(always)]
    fn from(val: PioSlew) -> u8 {
        PioSlew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PioSsel {
    #[doc = "3V3 Signaling in I2C Mode."]
    Sel3v3 = 0x0,
    #[doc = "1V8 Signaling in I2C Mode."]
    Sel1v8 = 0x01,
}
impl PioSsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PioSsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PioSsel {
    #[inline(always)]
    fn from(val: u8) -> PioSsel {
        PioSsel::from_bits(val)
    }
}
impl From<PioSsel> for u8 {
    #[inline(always)]
    fn from(val: PioSsel) -> u8 {
        PioSsel::to_bits(val)
    }
}
