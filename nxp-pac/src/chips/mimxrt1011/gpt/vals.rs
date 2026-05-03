#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clksrc {
    #[doc = "No clock."]
    Clksrc0 = 0x0,
    #[doc = "Peripheral Clock (ipg_clk)."]
    Clksrc1 = 0x01,
    #[doc = "High Frequency Reference Clock (ipg_clk_highfreq)."]
    Clksrc2 = 0x02,
    #[doc = "External Clock."]
    Clksrc3 = 0x03,
    #[doc = "Low Frequency Reference Clock (ipg_clk_32k)."]
    Clksrc4 = 0x04,
    #[doc = "Crystal oscillator as Reference Clock (ipg_clk_24M)."]
    Clksrc5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Clksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clksrc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clksrc {
    #[inline(always)]
    fn from(val: u8) -> Clksrc {
        Clksrc::from_bits(val)
    }
}
impl From<Clksrc> for u8 {
    #[inline(always)]
    fn from(val: Clksrc) -> u8 {
        Clksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgen {
    #[doc = "GPT is disabled in debug mode."]
    Dbgen0 = 0x0,
    #[doc = "GPT is enabled in debug mode."]
    Dbgen1 = 0x01,
}
impl Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgen {
    #[inline(always)]
    fn from(val: u8) -> Dbgen {
        Dbgen::from_bits(val)
    }
}
impl From<Dbgen> for u8 {
    #[inline(always)]
    fn from(val: Dbgen) -> u8 {
        Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "GPT is disabled in doze mode."]
    Dozeen0 = 0x0,
    #[doc = "GPT is enabled in doze mode."]
    Dozeen1 = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "GPT is disabled."]
    En0 = 0x0,
    #[doc = "GPT is enabled."]
    En1 = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En24m {
    #[doc = "24M clock disabled."]
    En24m0 = 0x0,
    #[doc = "24M clock enabled."]
    En24m1 = 0x01,
}
impl En24m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En24m {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En24m {
    #[inline(always)]
    fn from(val: u8) -> En24m {
        En24m::from_bits(val)
    }
}
impl From<En24m> for u8 {
    #[inline(always)]
    fn from(val: En24m) -> u8 {
        En24m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enmod {
    #[doc = "GPT counter will retain its value when it is disabled."]
    Enmod0 = 0x0,
    #[doc = "GPT counter value is reset to 0 when it is disabled."]
    Enmod1 = 0x01,
}
impl Enmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enmod {
    #[inline(always)]
    fn from(val: u8) -> Enmod {
        Enmod::from_bits(val)
    }
}
impl From<Enmod> for u8 {
    #[inline(always)]
    fn from(val: Enmod) -> u8 {
        Enmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fo3 {
    #[doc = "Writing a 0 has no effect."]
    Fo30 = 0x0,
    #[doc = "Causes the programmed pin action on the timer Output Compare n pin; the OFn flag is not set."]
    Fo31 = 0x01,
}
impl Fo3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fo3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fo3 {
    #[inline(always)]
    fn from(val: u8) -> Fo3 {
        Fo3::from_bits(val)
    }
}
impl From<Fo3> for u8 {
    #[inline(always)]
    fn from(val: Fo3) -> u8 {
        Fo3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frr {
    #[doc = "Restart mode."]
    Frr0 = 0x0,
    #[doc = "Free-Run mode."]
    Frr1 = 0x01,
}
impl Frr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frr {
    #[inline(always)]
    fn from(val: u8) -> Frr {
        Frr::from_bits(val)
    }
}
impl From<Frr> for u8 {
    #[inline(always)]
    fn from(val: Frr) -> u8 {
        Frr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If2 {
    #[doc = "Capture event has not occurred."]
    If20 = 0x0,
    #[doc = "Capture event has occurred."]
    If21 = 0x01,
}
impl If2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If2 {
    #[inline(always)]
    fn from(val: u8) -> If2 {
        If2::from_bits(val)
    }
}
impl From<If2> for u8 {
    #[inline(always)]
    fn from(val: If2) -> u8 {
        If2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum If2ie {
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    If2ie0 = 0x0,
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    If2ie1 = 0x01,
}
impl If2ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> If2ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for If2ie {
    #[inline(always)]
    fn from(val: u8) -> If2ie {
        If2ie::from_bits(val)
    }
}
impl From<If2ie> for u8 {
    #[inline(always)]
    fn from(val: If2ie) -> u8 {
        If2ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Im2 {
    #[doc = "capture disabled."]
    Im20 = 0x0,
    #[doc = "capture on rising edge only."]
    Im21 = 0x01,
    #[doc = "capture on falling edge only."]
    Im22 = 0x02,
    #[doc = "capture on both edges."]
    Im23 = 0x03,
}
impl Im2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Im2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Im2 {
    #[inline(always)]
    fn from(val: u8) -> Im2 {
        Im2::from_bits(val)
    }
}
impl From<Im2> for u8 {
    #[inline(always)]
    fn from(val: Im2) -> u8 {
        Im2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Of3 {
    #[doc = "Compare event has not occurred."]
    Of30 = 0x0,
    #[doc = "Compare event has occurred."]
    Of31 = 0x01,
}
impl Of3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Of3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Of3 {
    #[inline(always)]
    fn from(val: u8) -> Of3 {
        Of3::from_bits(val)
    }
}
impl From<Of3> for u8 {
    #[inline(always)]
    fn from(val: Of3) -> u8 {
        Of3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Of3ie {
    #[doc = "Output Compare Channel n interrupt is disabled."]
    Of3ie0 = 0x0,
    #[doc = "Output Compare Channel n interrupt is enabled."]
    Of3ie1 = 0x01,
}
impl Of3ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Of3ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Of3ie {
    #[inline(always)]
    fn from(val: u8) -> Of3ie {
        Of3ie::from_bits(val)
    }
}
impl From<Of3ie> for u8 {
    #[inline(always)]
    fn from(val: Of3ie) -> u8 {
        Of3ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Om3 {
    #[doc = "Output disconnected. No response on pin."]
    Om30 = 0x0,
    #[doc = "Toggle output pin."]
    Om31 = 0x01,
    #[doc = "Clear output pin."]
    Om32 = 0x02,
    #[doc = "Set output pin."]
    Om33 = 0x03,
    #[doc = "Generate an active low pulse (that is one input clock wide) on the output pin."]
    Om34 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Om3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Om3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Om3 {
    #[inline(always)]
    fn from(val: u8) -> Om3 {
        Om3::from_bits(val)
    }
}
impl From<Om3> for u8 {
    #[inline(always)]
    fn from(val: Om3) -> u8 {
        Om3::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prescaler(u16);
impl Prescaler {
    #[doc = "Divide by 1."]
    pub const Prescaler0: Self = Self(0x0);
    #[doc = "Divide by 2."]
    pub const Prescaler1: Self = Self(0x01);
    #[doc = "Divide by 4096."]
    pub const Prescaler4095: Self = Self(0x0fff);
}
impl Prescaler {
    pub const fn from_bits(val: u16) -> Prescaler {
        Self(val & 0x0fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Prescaler {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Prescaler0"),
            0x01 => f.write_str("Prescaler1"),
            0x0fff => f.write_str("Prescaler4095"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prescaler {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Prescaler0"),
            0x01 => defmt::write!(f, "Prescaler1"),
            0x0fff => defmt::write!(f, "Prescaler4095"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Prescaler {
    #[inline(always)]
    fn from(val: u16) -> Prescaler {
        Prescaler::from_bits(val)
    }
}
impl From<Prescaler> for u16 {
    #[inline(always)]
    fn from(val: Prescaler) -> u16 {
        Prescaler::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescaler24m {
    #[doc = "Divide by 1."]
    Prescaler24m0 = 0x0,
    #[doc = "Divide by 2."]
    Prescaler24m1 = 0x01,
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
    #[doc = "Divide by 16."]
    Prescaler24m15 = 0x0f,
}
impl Prescaler24m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescaler24m {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescaler24m {
    #[inline(always)]
    fn from(val: u8) -> Prescaler24m {
        Prescaler24m::from_bits(val)
    }
}
impl From<Prescaler24m> for u8 {
    #[inline(always)]
    fn from(val: Prescaler24m) -> u8 {
        Prescaler24m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rov {
    #[doc = "Rollover has not occurred."]
    Rov0 = 0x0,
    #[doc = "Rollover has occurred."]
    Rov1 = 0x01,
}
impl Rov {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rov {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rov {
    #[inline(always)]
    fn from(val: u8) -> Rov {
        Rov::from_bits(val)
    }
}
impl From<Rov> for u8 {
    #[inline(always)]
    fn from(val: Rov) -> u8 {
        Rov::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rovie {
    #[doc = "Rollover interrupt is disabled."]
    Rovie0 = 0x0,
    #[doc = "Rollover interrupt enabled."]
    Rovie1 = 0x01,
}
impl Rovie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rovie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rovie {
    #[inline(always)]
    fn from(val: u8) -> Rovie {
        Rovie::from_bits(val)
    }
}
impl From<Rovie> for u8 {
    #[inline(always)]
    fn from(val: Rovie) -> u8 {
        Rovie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stopen {
    #[doc = "GPT is disabled in Stop mode."]
    Stopen0 = 0x0,
    #[doc = "GPT is enabled in Stop mode."]
    Stopen1 = 0x01,
}
impl Stopen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stopen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stopen {
    #[inline(always)]
    fn from(val: u8) -> Stopen {
        Stopen::from_bits(val)
    }
}
impl From<Stopen> for u8 {
    #[inline(always)]
    fn from(val: Stopen) -> u8 {
        Stopen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swr {
    #[doc = "GPT is not in reset state."]
    Swr0 = 0x0,
    #[doc = "GPT is in reset state."]
    Swr1 = 0x01,
}
impl Swr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swr {
    #[inline(always)]
    fn from(val: u8) -> Swr {
        Swr::from_bits(val)
    }
}
impl From<Swr> for u8 {
    #[inline(always)]
    fn from(val: Swr) -> u8 {
        Swr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Waiten {
    #[doc = "GPT is disabled in wait mode."]
    Waiten0 = 0x0,
    #[doc = "GPT is enabled in wait mode."]
    Waiten1 = 0x01,
}
impl Waiten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Waiten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Waiten {
    #[inline(always)]
    fn from(val: u8) -> Waiten {
        Waiten::from_bits(val)
    }
}
impl From<Waiten> for u8 {
    #[inline(always)]
    fn from(val: Waiten) -> u8 {
        Waiten::to_bits(val)
    }
}
