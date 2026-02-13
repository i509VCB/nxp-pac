#[doc = "acomparator 0 fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmp0fclkdiv(pub u32);
impl Acmp0fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Acmp0fclkdiv {
    #[inline(always)]
    fn default() -> Acmp0fclkdiv {
        Acmp0fclkdiv(0)
    }
}
impl core::fmt::Debug for Acmp0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Acmp0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Acmp0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Acmp0fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "acomparator 0 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acmp0fclksel(pub u32);
impl Acmp0fclksel {
    #[doc = "ACMP0 Fast Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Acmp0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Acmp0fclkselSel::from_bits(val as u8)
    }
    #[doc = "ACMP0 Fast Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Acmp0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Acmp0fclksel {
    #[inline(always)]
    fn default() -> Acmp0fclksel {
        Acmp0fclksel(0)
    }
}
impl core::fmt::Debug for Acmp0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Acmp0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Acmp0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Acmp0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "audio mclock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiomclkdiv(pub u32);
impl Audiomclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Audiomclkdiv {
    #[inline(always)]
    fn default() -> Audiomclkdiv {
        Audiomclkdiv(0)
    }
}
impl core::fmt::Debug for Audiomclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiomclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiomclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Audiomclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "audio mclock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiomclksel(pub u32);
impl Audiomclksel {
    #[doc = "Audio MCLK Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::AudiomclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::AudiomclkselSel::from_bits(val as u8)
    }
    #[doc = "Audio MCLK Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::AudiomclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Audiomclksel {
    #[inline(always)]
    fn default() -> Audiomclksel {
        Audiomclksel(0)
    }
}
impl core::fmt::Debug for Audiomclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiomclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiomclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Audiomclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "audio pll0 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0clksel(pub u32);
impl Audiopll0clksel {
    #[doc = "System PLL Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Audiopll0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Audiopll0clkselSel::from_bits(val as u8)
    }
    #[doc = "System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Audiopll0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Audiopll0clksel {
    #[inline(always)]
    fn default() -> Audiopll0clksel {
        Audiopll0clksel(0)
    }
}
impl core::fmt::Debug for Audiopll0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Audiopll0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "audio pll0 control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0ctl0(pub u32);
impl Audiopll0ctl0 {
    #[doc = "AUDIOPLL0 BYPASS Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bypass::from_bits(val as u8)
    }
    #[doc = "AUDIOPLL0 BYPASS Mode"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AUDIOPLL0 Reset:"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Reset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Reset::from_bits(val as u8)
    }
    #[doc = "AUDIOPLL0 Reset:"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Reset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Hold Ring Off Control"]
    #[must_use]
    #[inline(always)]
    pub const fn holdringoff_ena(&self) -> super::vals::HoldringoffEna {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::HoldringoffEna::from_bits(val as u8)
    }
    #[doc = "Hold Ring Off Control"]
    #[inline(always)]
    pub const fn set_holdringoff_ena(&mut self, val: super::vals::HoldringoffEna) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> super::vals::Mult {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Mult::from_bits(val as u8)
    }
    #[doc = "Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: super::vals::Mult) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for Audiopll0ctl0 {
    #[inline(always)]
    fn default() -> Audiopll0ctl0 {
        Audiopll0ctl0(0)
    }
}
impl core::fmt::Debug for Audiopll0ctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0ctl0")
            .field("bypass", &self.bypass())
            .field("reset", &self.reset())
            .field("holdringoff_ena", &self.holdringoff_ena())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0ctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Audiopll0ctl0 {{ bypass: {:?}, reset: {:?}, holdringoff_ena: {:?}, mult: {:?} }}",
            self.bypass(),
            self.reset(),
            self.holdringoff_ena(),
            self.mult()
        )
    }
}
#[doc = "Audio pll0 denom"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0denom(pub u32);
impl Audiopll0denom {
    #[doc = "This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Audiopll0denom {
    #[inline(always)]
    fn default() -> Audiopll0denom {
        Audiopll0denom(0)
    }
}
impl core::fmt::Debug for Audiopll0denom {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0denom")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0denom {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Audiopll0denom {{ denom: {=u32:?} }}", self.denom())
    }
}
#[doc = "audio pll0 lock time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0locktimediv2(pub u32);
impl Audiopll0locktimediv2 {
    #[doc = "AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[must_use]
    #[inline(always)]
    pub const fn locktimediv2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    pub const fn set_locktimediv2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Audiopll0locktimediv2 {
    #[inline(always)]
    fn default() -> Audiopll0locktimediv2 {
        Audiopll0locktimediv2(0)
    }
}
impl core::fmt::Debug for Audiopll0locktimediv2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0locktimediv2")
            .field("locktimediv2", &self.locktimediv2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0locktimediv2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Audiopll0locktimediv2 {{ locktimediv2: {=u16:?} }}",
            self.locktimediv2()
        )
    }
}
#[doc = "audio pll0 number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0num(pub u32);
impl Audiopll0num {
    #[doc = "This field contains the numerator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0NUM register can only be changed when the AUDIOPLL0 is disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the numerator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0NUM register can only be changed when the AUDIOPLL0 is disabled."]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Audiopll0num {
    #[inline(always)]
    fn default() -> Audiopll0num {
        Audiopll0num(0)
    }
}
impl core::fmt::Debug for Audiopll0num {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0num")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0num {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Audiopll0num {{ num: {=u32:?} }}", self.num())
    }
}
#[doc = "audio pll0 PFD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopll0pfd(pub u32);
impl Audiopll0pfd {
    #[doc = "PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn set_pfd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkrdy(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub const fn set_pfd0_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn set_pfd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkrdy(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub const fn set_pfd1_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[inline(always)]
    pub const fn set_pfd1_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn set_pfd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkrdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub const fn set_pfd2_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[inline(always)]
    pub const fn set_pfd2_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn set_pfd3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkrdy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3 Clock Ready Status Flag: Read as 1 clock ready. Cleared by writing a 1."]
    #[inline(always)]
    pub const fn set_pfd3_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[inline(always)]
    pub const fn set_pfd3_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Audiopll0pfd {
    #[inline(always)]
    fn default() -> Audiopll0pfd {
        Audiopll0pfd(0)
    }
}
impl core::fmt::Debug for Audiopll0pfd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopll0pfd")
            .field("pfd0", &self.pfd0())
            .field("pfd0_clkrdy", &self.pfd0_clkrdy())
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd1", &self.pfd1())
            .field("pfd1_clkrdy", &self.pfd1_clkrdy())
            .field("pfd1_clkgate", &self.pfd1_clkgate())
            .field("pfd2", &self.pfd2())
            .field("pfd2_clkrdy", &self.pfd2_clkrdy())
            .field("pfd2_clkgate", &self.pfd2_clkgate())
            .field("pfd3", &self.pfd3())
            .field("pfd3_clkrdy", &self.pfd3_clkrdy())
            .field("pfd3_clkgate", &self.pfd3_clkgate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopll0pfd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Audiopll0pfd {{ pfd0: {=u8:?}, pfd0_clkrdy: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1: {=u8:?}, pfd1_clkrdy: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2: {=u8:?}, pfd2_clkrdy: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3: {=u8:?}, pfd3_clkrdy: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
            self.pfd0(),
            self.pfd0_clkrdy(),
            self.pfd0_clkgate(),
            self.pfd1(),
            self.pfd1_clkrdy(),
            self.pfd1_clkgate(),
            self.pfd2(),
            self.pfd2_clkrdy(),
            self.pfd2_clkgate(),
            self.pfd3(),
            self.pfd3_clkrdy(),
            self.pfd3_clkgate()
        )
    }
}
#[doc = "audio pll0 clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audiopllclkdiv(pub u32);
impl Audiopllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Audiopllclkdiv {
    #[inline(always)]
    fn default() -> Audiopllclkdiv {
        Audiopllclkdiv(0)
    }
}
impl core::fmt::Debug for Audiopllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Audiopllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Audiopllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Audiopllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "clock_out divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutdiv(pub u32);
impl Clkoutdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Clkoutdiv {
    #[inline(always)]
    fn default() -> Clkoutdiv {
        Clkoutdiv(0)
    }
}
impl core::fmt::Debug for Clkoutdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkoutdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "clock out selection 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutsel0(pub u32);
impl Clkoutsel0 {
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Clkoutsel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Clkoutsel0Sel::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Clkoutsel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Clkoutsel0 {
    #[inline(always)]
    fn default() -> Clkoutsel0 {
        Clkoutsel0(0)
    }
}
impl core::fmt::Debug for Clkoutsel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutsel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutsel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkoutsel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "clock out selection 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutsel1(pub u32);
impl Clkoutsel1 {
    #[doc = "Clock out clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Clkoutsel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Clkoutsel1Sel::from_bits(val as u8)
    }
    #[doc = "Clock out clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Clkoutsel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Clkoutsel1 {
    #[inline(always)]
    fn default() -> Clkoutsel1 {
        Clkoutsel1(0)
    }
}
impl core::fmt::Debug for Clkoutsel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutsel1")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutsel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkoutsel1 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ct32bit timer N clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ct32bitfclksel(pub u32);
impl Ct32bitfclksel {
    #[doc = "CT32Bit Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Ct32bitfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ct32bitfclkselSel::from_bits(val as u8)
    }
    #[doc = "CT32Bit Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Ct32bitfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Ct32bitfclksel {
    #[inline(always)]
    fn default() -> Ct32bitfclksel {
        Ct32bitfclksel(0)
    }
}
impl core::fmt::Debug for Ct32bitfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ct32bitfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ct32bitfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ct32bitfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DMIC clock clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmic0fclkdiv(pub u32);
impl Dmic0fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmic0fclkdiv {
    #[inline(always)]
    fn default() -> Dmic0fclkdiv {
        Dmic0fclkdiv(0)
    }
}
impl core::fmt::Debug for Dmic0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmic0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmic0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmic0fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "DMIC0 clk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmic0fclksel(pub u32);
impl Dmic0fclksel {
    #[doc = "DMIC Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Dmic0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Dmic0fclkselSel::from_bits(val as u8)
    }
    #[doc = "DMIC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Dmic0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Dmic0fclksel {
    #[inline(always)]
    fn default() -> Dmic0fclksel {
        Dmic0fclksel(0)
    }
}
impl core::fmt::Debug for Dmic0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmic0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmic0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmic0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DSP cpu clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspcpuclkdiv(pub u32);
impl Dspcpuclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dspcpuclkdiv {
    #[inline(always)]
    fn default() -> Dspcpuclkdiv {
        Dspcpuclkdiv(0)
    }
}
impl core::fmt::Debug for Dspcpuclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspcpuclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspcpuclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dspcpuclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "DSP clock selection A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspcpuclksela(pub u32);
impl Dspcpuclksela {
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::DspcpuclkselaSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DspcpuclkselaSel::from_bits(val as u8)
    }
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::DspcpuclkselaSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Dspcpuclksela {
    #[inline(always)]
    fn default() -> Dspcpuclksela {
        Dspcpuclksela(0)
    }
}
impl core::fmt::Debug for Dspcpuclksela {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspcpuclksela")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspcpuclksela {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dspcpuclksela {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DSP clock selection B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspcpuclkselb(pub u32);
impl Dspcpuclkselb {
    #[doc = "Main Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::DspcpuclkselbSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DspcpuclkselbSel::from_bits(val as u8)
    }
    #[doc = "Main Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::DspcpuclkselbSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Dspcpuclkselb {
    #[inline(always)]
    fn default() -> Dspcpuclkselb {
        Dspcpuclkselb(0)
    }
}
impl core::fmt::Debug for Dspcpuclkselb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspcpuclkselb")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspcpuclkselb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dspcpuclkselb {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "DSP main ram clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspmainramclkdiv(pub u32);
impl Dspmainramclkdiv {
    #[doc = "DSP MAINRAM Clock Ratio Control:"]
    #[must_use]
    #[inline(always)]
    pub const fn dspmramclkdiv(&self) -> super::vals::Dspmramclkdiv {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dspmramclkdiv::from_bits(val as u8)
    }
    #[doc = "DSP MAINRAM Clock Ratio Control:"]
    #[inline(always)]
    pub const fn set_dspmramclkdiv(&mut self, val: super::vals::Dspmramclkdiv) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Dspmainramclkdiv {
    #[inline(always)]
    fn default() -> Dspmainramclkdiv {
        Dspmainramclkdiv(0)
    }
}
impl core::fmt::Debug for Dspmainramclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspmainramclkdiv")
            .field("dspmramclkdiv", &self.dspmramclkdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspmainramclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dspmainramclkdiv {{ dspmramclkdiv: {:?} }}",
            self.dspmramclkdiv()
        )
    }
}
#[doc = "flexcomm14 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc14fclksel(pub u32);
impl Fc14fclksel {
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Fc14fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Fc14fclkselSel::from_bits(val as u8)
    }
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Fc14fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Fc14fclksel {
    #[inline(always)]
    fn default() -> Fc14fclksel {
        Fc14fclksel(0)
    }
}
impl core::fmt::Debug for Fc14fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc14fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc14fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fc14fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "flexcomm15 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc15fclksel(pub u32);
impl Fc15fclksel {
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Fc15fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Fc15fclkselSel::from_bits(val as u8)
    }
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Fc15fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Fc15fclksel {
    #[inline(always)]
    fn default() -> Fc15fclksel {
        Fc15fclksel(0)
    }
}
impl core::fmt::Debug for Fc15fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc15fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc15fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fc15fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "flexcomm clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcfclksel(pub u32);
impl Fcfclksel {
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FcfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FcfclkselSel::from_bits(val as u8)
    }
    #[doc = "Flexxcomm Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FcfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Fcfclksel {
    #[inline(always)]
    fn default() -> Fcfclksel {
        Fcfclksel(0)
    }
}
impl core::fmt::Debug for Fcfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRG clock selection register 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frg14clksel(pub u32);
impl Frg14clksel {
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Frg14clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Frg14clkselSel::from_bits(val as u8)
    }
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Frg14clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Frg14clksel {
    #[inline(always)]
    fn default() -> Frg14clksel {
        Frg14clksel(0)
    }
}
impl core::fmt::Debug for Frg14clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frg14clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frg14clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frg14clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRG clock controller 14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frg14ctl(pub u32);
impl Frg14ctl {
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Frg14ctl {
    #[inline(always)]
    fn default() -> Frg14ctl {
        Frg14ctl(0)
    }
}
impl core::fmt::Debug for Frg14ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frg14ctl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frg14ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frg14ctl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "FRG clock selection register 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frg15clksel(pub u32);
impl Frg15clksel {
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Frg15clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Frg15clkselSel::from_bits(val as u8)
    }
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Frg15clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Frg15clksel {
    #[inline(always)]
    fn default() -> Frg15clksel {
        Frg15clksel(0)
    }
}
impl core::fmt::Debug for Frg15clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frg15clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frg15clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frg15clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRG clock controller 15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frg15ctl(pub u32);
impl Frg15ctl {
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Frg15ctl {
    #[inline(always)]
    fn default() -> Frg15ctl {
        Frg15ctl(0)
    }
}
impl core::fmt::Debug for Frg15ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frg15ctl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frg15ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frg15ctl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "FRG clock selection register N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frgclksel(pub u32);
impl Frgclksel {
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FrgclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FrgclkselSel::from_bits(val as u8)
    }
    #[doc = "Fractional Gen. Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FrgclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Frgclksel {
    #[inline(always)]
    fn default() -> Frgclksel {
        Frgclksel(0)
    }
}
impl core::fmt::Debug for Frgclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frgclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frgclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frgclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRG clock controller"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frgctl(pub u32);
impl Frgctl {
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Frgctl {
    #[inline(always)]
    fn default() -> Frgctl {
        Frgctl(0)
    }
}
impl core::fmt::Debug for Frgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frgctl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frgctl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "FRG pll clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frgpllclkdiv(pub u32);
impl Frgpllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Frgpllclkdiv {
    #[inline(always)]
    fn default() -> Frgpllclkdiv {
        Frgpllclkdiv(0)
    }
}
impl core::fmt::Debug for Frgpllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frgpllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frgpllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frgpllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "I3C0 fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkdiv(pub u32);
impl I3c0fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkdiv {
    #[inline(always)]
    fn default() -> I3c0fclkdiv {
        I3c0fclkdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "I3C0 fclks divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksdiv(pub u32);
impl I3c0fclksdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclksdiv {
    #[inline(always)]
    fn default() -> I3c0fclksdiv {
        I3c0fclksdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclksdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclksdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "I3C0 fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksel(pub u32);
impl I3c0fclksel {
    #[doc = "I3C0 FClock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c0fclkselSel::from_bits(val as u8)
    }
    #[doc = "I3C0 FClock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclksel {
    #[inline(always)]
    fn default() -> I3c0fclksel {
        I3c0fclksel(0)
    }
}
impl core::fmt::Debug for I3c0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C0 fclk STC divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcdiv(pub u32);
impl I3c0fclkstcdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkstcdiv {
    #[inline(always)]
    fn default() -> I3c0fclkstcdiv {
        I3c0fclkstcdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkstcdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "I3C0 fclk STC selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcsel(pub u32);
impl I3c0fclkstcsel {
    #[doc = "I3C0 Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::I3c0fclkstcselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::I3c0fclkstcselSel::from_bits(val as u8)
    }
    #[doc = "I3C0 Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::I3c0fclkstcselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclkstcsel {
    #[inline(always)]
    fn default() -> I3c0fclkstcsel {
        I3c0fclkstcsel(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclkstcsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "OS EVENT clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oseventfclksel(pub u32);
impl Oseventfclksel {
    #[doc = "OS Event Timer Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::OseventfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::OseventfclkselSel::from_bits(val as u8)
    }
    #[doc = "OS Event Timer Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::OseventfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Oseventfclksel {
    #[inline(always)]
    fn default() -> Oseventfclksel {
        Oseventfclksel(0)
    }
}
impl core::fmt::Debug for Oseventfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Oseventfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Oseventfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Oseventfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "clock control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0(pub u32);
impl Pscctl0 {
    #[doc = "flexcomm 0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0_clk(&self) -> super::vals::Fc0Clk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fc0Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 0 clock control"]
    #[inline(always)]
    pub const fn set_fc0_clk(&mut self, val: super::vals::Fc0Clk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "flexcomm 1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1_clk(&self) -> super::vals::Fc1Clk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Fc1Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 1 clock control"]
    #[inline(always)]
    pub const fn set_fc1_clk(&mut self, val: super::vals::Fc1Clk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "flexcomm 2 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc2_clk(&self) -> super::vals::Fc2Clk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Fc2Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 2 clock control"]
    #[inline(always)]
    pub const fn set_fc2_clk(&mut self, val: super::vals::Fc2Clk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "flexcomm 3 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3_clk(&self) -> super::vals::Fc3Clk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fc3Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 3 clock control"]
    #[inline(always)]
    pub const fn set_fc3_clk(&mut self, val: super::vals::Fc3Clk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "flexcomm 4 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4_clk(&self) -> super::vals::Fc4Clk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Fc4Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 4 clock control"]
    #[inline(always)]
    pub const fn set_fc4_clk(&mut self, val: super::vals::Fc4Clk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "flexcomm 5 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5_clk(&self) -> super::vals::Fc5Clk {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Fc5Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 5 clock control"]
    #[inline(always)]
    pub const fn set_fc5_clk(&mut self, val: super::vals::Fc5Clk) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "flexcomm 6 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6_clk(&self) -> super::vals::Fc6Clk {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fc6Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 6 clock control"]
    #[inline(always)]
    pub const fn set_fc6_clk(&mut self, val: super::vals::Fc6Clk) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "flexcomm 7 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7_clk(&self) -> super::vals::Fc7Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Fc7Clk::from_bits(val as u8)
    }
    #[doc = "flexcomm 7 clock control"]
    #[inline(always)]
    pub const fn set_fc7_clk(&mut self, val: super::vals::Fc7Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "flexcomm 14 spi clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc14_spi_clk(&self) -> super::vals::Fc14SpiClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fc14SpiClk::from_bits(val as u8)
    }
    #[doc = "flexcomm 14 spi clock control"]
    #[inline(always)]
    pub const fn set_fc14_spi_clk(&mut self, val: super::vals::Fc14SpiClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "flexcomm 15 i2c clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn fc15_i2c_clk(&self) -> super::vals::Fc15I2cClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Fc15I2cClk::from_bits(val as u8)
    }
    #[doc = "flexcomm 15 i2c clock control"]
    #[inline(always)]
    pub const fn set_fc15_i2c_clk(&mut self, val: super::vals::Fc15I2cClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0_clk(&self) -> super::vals::Dmic0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0Clk::from_bits(val as u8)
    }
    #[doc = "DMIC0 clock control"]
    #[inline(always)]
    pub const fn set_dmic0_clk(&mut self, val: super::vals::Dmic0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "OS event timer clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn osevent_timer_clk(&self) -> super::vals::OseventTimerClk {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OseventTimerClk::from_bits(val as u8)
    }
    #[doc = "OS event timer clock control"]
    #[inline(always)]
    pub const fn set_osevent_timer_clk(&mut self, val: super::vals::OseventTimerClk) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Pscctl0 {
    #[inline(always)]
    fn default() -> Pscctl0 {
        Pscctl0(0)
    }
}
impl core::fmt::Debug for Pscctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl0")
            .field("fc0_clk", &self.fc0_clk())
            .field("fc1_clk", &self.fc1_clk())
            .field("fc2_clk", &self.fc2_clk())
            .field("fc3_clk", &self.fc3_clk())
            .field("fc4_clk", &self.fc4_clk())
            .field("fc5_clk", &self.fc5_clk())
            .field("fc6_clk", &self.fc6_clk())
            .field("fc7_clk", &self.fc7_clk())
            .field("fc14_spi_clk", &self.fc14_spi_clk())
            .field("fc15_i2c_clk", &self.fc15_i2c_clk())
            .field("dmic0_clk", &self.dmic0_clk())
            .field("osevent_timer_clk", &self.osevent_timer_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0 {{ fc0_clk: {:?}, fc1_clk: {:?}, fc2_clk: {:?}, fc3_clk: {:?}, fc4_clk: {:?}, fc5_clk: {:?}, fc6_clk: {:?}, fc7_clk: {:?}, fc14_spi_clk: {:?}, fc15_i2c_clk: {:?}, dmic0_clk: {:?}, osevent_timer_clk: {:?} }}",
            self.fc0_clk(),
            self.fc1_clk(),
            self.fc2_clk(),
            self.fc3_clk(),
            self.fc4_clk(),
            self.fc5_clk(),
            self.fc6_clk(),
            self.fc7_clk(),
            self.fc14_spi_clk(),
            self.fc15_i2c_clk(),
            self.dmic0_clk(),
            self.osevent_timer_clk()
        )
    }
}
#[doc = "clock clear register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Clr(pub u32);
impl Pscctl0Clr {
    #[doc = "flexcomm 0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0_clk_clr(&self) -> super::vals::Fc0ClkClr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fc0ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 0 clock clear"]
    #[inline(always)]
    pub const fn set_fc0_clk_clr(&mut self, val: super::vals::Fc0ClkClr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "flexcomm 1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1_clk_clr(&self) -> super::vals::Fc1ClkClr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Fc1ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 1 clock clear"]
    #[inline(always)]
    pub const fn set_fc1_clk_clr(&mut self, val: super::vals::Fc1ClkClr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "flexcomm 2 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc2_clk_clr(&self) -> super::vals::Fc2ClkClr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Fc2ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 2 clock clear"]
    #[inline(always)]
    pub const fn set_fc2_clk_clr(&mut self, val: super::vals::Fc2ClkClr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "flexcomm 3 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3_clk_clr(&self) -> super::vals::Fc3ClkClr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fc3ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 3 clock clear"]
    #[inline(always)]
    pub const fn set_fc3_clk_clr(&mut self, val: super::vals::Fc3ClkClr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "flexcomm 4 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4_clk_clr(&self) -> super::vals::Fc4ClkClr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Fc4ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 4 clock clear"]
    #[inline(always)]
    pub const fn set_fc4_clk_clr(&mut self, val: super::vals::Fc4ClkClr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "flexcomm 5 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5_clk_clr(&self) -> super::vals::Fc5ClkClr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Fc5ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 5 clock clear"]
    #[inline(always)]
    pub const fn set_fc5_clk_clr(&mut self, val: super::vals::Fc5ClkClr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "flexcomm 6 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6_clk_clr(&self) -> super::vals::Fc6ClkClr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fc6ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 6 clock clear"]
    #[inline(always)]
    pub const fn set_fc6_clk_clr(&mut self, val: super::vals::Fc6ClkClr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "flexcomm 7 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7_clk_clr(&self) -> super::vals::Fc7ClkClr {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Fc7ClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 7 clock clear"]
    #[inline(always)]
    pub const fn set_fc7_clk_clr(&mut self, val: super::vals::Fc7ClkClr) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "flexcomm 14 spi clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc14_spi_clk_clr(&self) -> super::vals::Fc14SpiClkClr {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fc14SpiClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 14 spi clock clear"]
    #[inline(always)]
    pub const fn set_fc14_spi_clk_clr(&mut self, val: super::vals::Fc14SpiClkClr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "flexcomm 15 i2c clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn fc15_i2c_clk_clr(&self) -> super::vals::Fc15I2cClkClr {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Fc15I2cClkClr::from_bits(val as u8)
    }
    #[doc = "flexcomm 15 i2c clock clear"]
    #[inline(always)]
    pub const fn set_fc15_i2c_clk_clr(&mut self, val: super::vals::Fc15I2cClkClr) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0_clk_clr(&self) -> super::vals::Dmic0ClkClr {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0ClkClr::from_bits(val as u8)
    }
    #[doc = "DMIC0 clock set"]
    #[inline(always)]
    pub const fn set_dmic0_clk_clr(&mut self, val: super::vals::Dmic0ClkClr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "OS event timer clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn osevent_timer_clk_clr(&self) -> super::vals::OseventTimerClkClr {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OseventTimerClkClr::from_bits(val as u8)
    }
    #[doc = "OS event timer clock clear"]
    #[inline(always)]
    pub const fn set_osevent_timer_clk_clr(&mut self, val: super::vals::OseventTimerClkClr) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Pscctl0Clr {
    #[inline(always)]
    fn default() -> Pscctl0Clr {
        Pscctl0Clr(0)
    }
}
impl core::fmt::Debug for Pscctl0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl0Clr")
            .field("fc0_clk_clr", &self.fc0_clk_clr())
            .field("fc1_clk_clr", &self.fc1_clk_clr())
            .field("fc2_clk_clr", &self.fc2_clk_clr())
            .field("fc3_clk_clr", &self.fc3_clk_clr())
            .field("fc4_clk_clr", &self.fc4_clk_clr())
            .field("fc5_clk_clr", &self.fc5_clk_clr())
            .field("fc6_clk_clr", &self.fc6_clk_clr())
            .field("fc7_clk_clr", &self.fc7_clk_clr())
            .field("fc14_spi_clk_clr", &self.fc14_spi_clk_clr())
            .field("fc15_i2c_clk_clr", &self.fc15_i2c_clk_clr())
            .field("dmic0_clk_clr", &self.dmic0_clk_clr())
            .field("osevent_timer_clk_clr", &self.osevent_timer_clk_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0Clr {{ fc0_clk_clr: {:?}, fc1_clk_clr: {:?}, fc2_clk_clr: {:?}, fc3_clk_clr: {:?}, fc4_clk_clr: {:?}, fc5_clk_clr: {:?}, fc6_clk_clr: {:?}, fc7_clk_clr: {:?}, fc14_spi_clk_clr: {:?}, fc15_i2c_clk_clr: {:?}, dmic0_clk_clr: {:?}, osevent_timer_clk_clr: {:?} }}",
            self.fc0_clk_clr(),
            self.fc1_clk_clr(),
            self.fc2_clk_clr(),
            self.fc3_clk_clr(),
            self.fc4_clk_clr(),
            self.fc5_clk_clr(),
            self.fc6_clk_clr(),
            self.fc7_clk_clr(),
            self.fc14_spi_clk_clr(),
            self.fc15_i2c_clk_clr(),
            self.dmic0_clk_clr(),
            self.osevent_timer_clk_clr()
        )
    }
}
#[doc = "clock set register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Set(pub u32);
impl Pscctl0Set {
    #[doc = "flexcomm 0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc0_clk_set(&self) -> super::vals::Fc0ClkSet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fc0ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 0 clock set"]
    #[inline(always)]
    pub const fn set_fc0_clk_set(&mut self, val: super::vals::Fc0ClkSet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "flexcomm 1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc1_clk_set(&self) -> super::vals::Fc1ClkSet {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Fc1ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 1 clock set"]
    #[inline(always)]
    pub const fn set_fc1_clk_set(&mut self, val: super::vals::Fc1ClkSet) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "flexcomm 2 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc2_clk_set(&self) -> super::vals::Fc2ClkSet {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Fc2ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 2 clock set"]
    #[inline(always)]
    pub const fn set_fc2_clk_set(&mut self, val: super::vals::Fc2ClkSet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "flexcomm 3 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc3_clk_set(&self) -> super::vals::Fc3ClkSet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fc3ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 3 clock set"]
    #[inline(always)]
    pub const fn set_fc3_clk_set(&mut self, val: super::vals::Fc3ClkSet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "flexcomm 4 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc4_clk_set(&self) -> super::vals::Fc4ClkSet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Fc4ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 4 clock set"]
    #[inline(always)]
    pub const fn set_fc4_clk_set(&mut self, val: super::vals::Fc4ClkSet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "flexcomm 5 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc5_clk_set(&self) -> super::vals::Fc5ClkSet {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Fc5ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 5 clock set"]
    #[inline(always)]
    pub const fn set_fc5_clk_set(&mut self, val: super::vals::Fc5ClkSet) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "flexcomm 6 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc6_clk_set(&self) -> super::vals::Fc6ClkSet {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fc6ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 6 clock set"]
    #[inline(always)]
    pub const fn set_fc6_clk_set(&mut self, val: super::vals::Fc6ClkSet) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "flexcomm 7 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc7_clk_set(&self) -> super::vals::Fc7ClkSet {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Fc7ClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 7 clock set"]
    #[inline(always)]
    pub const fn set_fc7_clk_set(&mut self, val: super::vals::Fc7ClkSet) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "flexcomm 14 spi clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc14_spi_clk_set(&self) -> super::vals::Fc14SpiClkSet {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fc14SpiClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 14 spi clock set"]
    #[inline(always)]
    pub const fn set_fc14_spi_clk_set(&mut self, val: super::vals::Fc14SpiClkSet) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "flexcomm 15 i2c clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn fc15_i2c_clk_set(&self) -> super::vals::Fc15I2cClkSet {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Fc15I2cClkSet::from_bits(val as u8)
    }
    #[doc = "flexcomm 15 i2c clock set"]
    #[inline(always)]
    pub const fn set_fc15_i2c_clk_set(&mut self, val: super::vals::Fc15I2cClkSet) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0_clk_set(&self) -> super::vals::Dmic0ClkSet {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0ClkSet::from_bits(val as u8)
    }
    #[doc = "DMIC0 clock set"]
    #[inline(always)]
    pub const fn set_dmic0_clk_set(&mut self, val: super::vals::Dmic0ClkSet) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "OS event timer clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn osevent_timer_clk_set(&self) -> super::vals::OseventTimerClkSet {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OseventTimerClkSet::from_bits(val as u8)
    }
    #[doc = "OS event timer clock set"]
    #[inline(always)]
    pub const fn set_osevent_timer_clk_set(&mut self, val: super::vals::OseventTimerClkSet) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Pscctl0Set {
    #[inline(always)]
    fn default() -> Pscctl0Set {
        Pscctl0Set(0)
    }
}
impl core::fmt::Debug for Pscctl0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl0Set")
            .field("fc0_clk_set", &self.fc0_clk_set())
            .field("fc1_clk_set", &self.fc1_clk_set())
            .field("fc2_clk_set", &self.fc2_clk_set())
            .field("fc3_clk_set", &self.fc3_clk_set())
            .field("fc4_clk_set", &self.fc4_clk_set())
            .field("fc5_clk_set", &self.fc5_clk_set())
            .field("fc6_clk_set", &self.fc6_clk_set())
            .field("fc7_clk_set", &self.fc7_clk_set())
            .field("fc14_spi_clk_set", &self.fc14_spi_clk_set())
            .field("fc15_i2c_clk_set", &self.fc15_i2c_clk_set())
            .field("dmic0_clk_set", &self.dmic0_clk_set())
            .field("osevent_timer_clk_set", &self.osevent_timer_clk_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0Set {{ fc0_clk_set: {:?}, fc1_clk_set: {:?}, fc2_clk_set: {:?}, fc3_clk_set: {:?}, fc4_clk_set: {:?}, fc5_clk_set: {:?}, fc6_clk_set: {:?}, fc7_clk_set: {:?}, fc14_spi_clk_set: {:?}, fc15_i2c_clk_set: {:?}, dmic0_clk_set: {:?}, osevent_timer_clk_set: {:?} }}",
            self.fc0_clk_set(),
            self.fc1_clk_set(),
            self.fc2_clk_set(),
            self.fc3_clk_set(),
            self.fc4_clk_set(),
            self.fc5_clk_set(),
            self.fc6_clk_set(),
            self.fc7_clk_set(),
            self.fc14_spi_clk_set(),
            self.fc15_i2c_clk_set(),
            self.dmic0_clk_set(),
            self.osevent_timer_clk_set()
        )
    }
}
#[doc = "clock control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1(pub u32);
impl Pscctl1 {
    #[doc = "HSGPIO0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio0_clk(&self) -> super::vals::Hsgpio0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio0_clk(&mut self, val: super::vals::Hsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio1_clk(&self) -> super::vals::Hsgpio1Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio1_clk(&mut self, val: super::vals::Hsgpio1Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio2_clk(&self) -> super::vals::Hsgpio2Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio2_clk(&mut self, val: super::vals::Hsgpio2Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio3_clk(&self) -> super::vals::Hsgpio3Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio3_clk(&mut self, val: super::vals::Hsgpio3Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio4_clk(&self) -> super::vals::Hsgpio4Clk {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio4_clk(&mut self, val: super::vals::Hsgpio4Clk) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio5_clk(&self) -> super::vals::Hsgpio5Clk {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio5_clk(&mut self, val: super::vals::Hsgpio5Clk) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio6_clk(&self) -> super::vals::Hsgpio6Clk {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio6_clk(&mut self, val: super::vals::Hsgpio6Clk) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio7_clk(&self) -> super::vals::Hsgpio7Clk {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7Clk::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 clock control"]
    #[inline(always)]
    pub const fn set_hsgpio7_clk(&mut self, val: super::vals::Hsgpio7Clk) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_clk(&self) -> super::vals::CrcClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcClk::from_bits(val as u8)
    }
    #[doc = "CRC clock control"]
    #[inline(always)]
    pub const fn set_crc_clk(&mut self, val: super::vals::CrcClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0_clk(&self) -> super::vals::Dmac0Clk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0Clk::from_bits(val as u8)
    }
    #[doc = "DMAC0 clock control"]
    #[inline(always)]
    pub const fn set_dmac0_clk(&mut self, val: super::vals::Dmac0Clk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1_clk(&self) -> super::vals::Dmac1Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1Clk::from_bits(val as u8)
    }
    #[doc = "DMAC1 clock control"]
    #[inline(always)]
    pub const fn set_dmac1_clk(&mut self, val: super::vals::Dmac1Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn mu_clk(&self) -> super::vals::MuClk {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuClk::from_bits(val as u8)
    }
    #[doc = "MU clock control"]
    #[inline(always)]
    pub const fn set_mu_clk(&mut self, val: super::vals::MuClk) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn sema_clk(&self) -> super::vals::SemaClk {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaClk::from_bits(val as u8)
    }
    #[doc = "SEMA clock control"]
    #[inline(always)]
    pub const fn set_sema_clk(&mut self, val: super::vals::SemaClk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_clk(&self) -> super::vals::FreqmeClk {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeClk::from_bits(val as u8)
    }
    #[doc = "FREQME clock control"]
    #[inline(always)]
    pub const fn set_freqme_clk(&mut self, val: super::vals::FreqmeClk) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl1 {
    #[inline(always)]
    fn default() -> Pscctl1 {
        Pscctl1(0)
    }
}
impl core::fmt::Debug for Pscctl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl1")
            .field("hsgpio0_clk", &self.hsgpio0_clk())
            .field("hsgpio1_clk", &self.hsgpio1_clk())
            .field("hsgpio2_clk", &self.hsgpio2_clk())
            .field("hsgpio3_clk", &self.hsgpio3_clk())
            .field("hsgpio4_clk", &self.hsgpio4_clk())
            .field("hsgpio5_clk", &self.hsgpio5_clk())
            .field("hsgpio6_clk", &self.hsgpio6_clk())
            .field("hsgpio7_clk", &self.hsgpio7_clk())
            .field("crc_clk", &self.crc_clk())
            .field("dmac0_clk", &self.dmac0_clk())
            .field("dmac1_clk", &self.dmac1_clk())
            .field("mu_clk", &self.mu_clk())
            .field("sema_clk", &self.sema_clk())
            .field("freqme_clk", &self.freqme_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1 {{ hsgpio0_clk: {:?}, hsgpio1_clk: {:?}, hsgpio2_clk: {:?}, hsgpio3_clk: {:?}, hsgpio4_clk: {:?}, hsgpio5_clk: {:?}, hsgpio6_clk: {:?}, hsgpio7_clk: {:?}, crc_clk: {:?}, dmac0_clk: {:?}, dmac1_clk: {:?}, mu_clk: {:?}, sema_clk: {:?}, freqme_clk: {:?} }}",
            self.hsgpio0_clk(),
            self.hsgpio1_clk(),
            self.hsgpio2_clk(),
            self.hsgpio3_clk(),
            self.hsgpio4_clk(),
            self.hsgpio5_clk(),
            self.hsgpio6_clk(),
            self.hsgpio7_clk(),
            self.crc_clk(),
            self.dmac0_clk(),
            self.dmac1_clk(),
            self.mu_clk(),
            self.sema_clk(),
            self.freqme_clk()
        )
    }
}
#[doc = "clock clear register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Clr(pub u32);
impl Pscctl1Clr {
    #[doc = "HSGPIO0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio0_clk_clr(&self) -> super::vals::Hsgpio0ClkClr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio0_clk_clr(&mut self, val: super::vals::Hsgpio0ClkClr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio1_clk_clr(&self) -> super::vals::Hsgpio1ClkClr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio1_clk_clr(&mut self, val: super::vals::Hsgpio1ClkClr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio2_clk_clr(&self) -> super::vals::Hsgpio2ClkClr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio2_clk_clr(&mut self, val: super::vals::Hsgpio2ClkClr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio3_clk_clr(&self) -> super::vals::Hsgpio3ClkClr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio3_clk_clr(&mut self, val: super::vals::Hsgpio3ClkClr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio4_clk_clr(&self) -> super::vals::Hsgpio4ClkClr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio4_clk_clr(&mut self, val: super::vals::Hsgpio4ClkClr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio5_clk_clr(&self) -> super::vals::Hsgpio5ClkClr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio5_clk_clr(&mut self, val: super::vals::Hsgpio5ClkClr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio6_clk_clr(&self) -> super::vals::Hsgpio6ClkClr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio6_clk_clr(&mut self, val: super::vals::Hsgpio6ClkClr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio7_clk_clr(&self) -> super::vals::Hsgpio7ClkClr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7ClkClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 clock clear"]
    #[inline(always)]
    pub const fn set_hsgpio7_clk_clr(&mut self, val: super::vals::Hsgpio7ClkClr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_clk_clr(&self) -> super::vals::CrcClkClr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcClkClr::from_bits(val as u8)
    }
    #[doc = "CRC clock clear"]
    #[inline(always)]
    pub const fn set_crc_clk_clr(&mut self, val: super::vals::CrcClkClr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0_clk_clr(&self) -> super::vals::Dmac0ClkClr {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ClkClr::from_bits(val as u8)
    }
    #[doc = "DMAC0 clock clear"]
    #[inline(always)]
    pub const fn set_dmac0_clk_clr(&mut self, val: super::vals::Dmac0ClkClr) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1_clk_clr(&self) -> super::vals::Dmac1ClkClr {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ClkClr::from_bits(val as u8)
    }
    #[doc = "DMAC1 clock clear"]
    #[inline(always)]
    pub const fn set_dmac1_clk_clr(&mut self, val: super::vals::Dmac1ClkClr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mu_clk_clr(&self) -> super::vals::MuClkClr {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuClkClr::from_bits(val as u8)
    }
    #[doc = "MU clock clear"]
    #[inline(always)]
    pub const fn set_mu_clk_clr(&mut self, val: super::vals::MuClkClr) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sema_clk_clr(&self) -> super::vals::SemaClkClr {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaClkClr::from_bits(val as u8)
    }
    #[doc = "SEMA clock clear"]
    #[inline(always)]
    pub const fn set_sema_clk_clr(&mut self, val: super::vals::SemaClkClr) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_clk_clr(&self) -> super::vals::FreqmeClkClr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeClkClr::from_bits(val as u8)
    }
    #[doc = "FREQME clock clear"]
    #[inline(always)]
    pub const fn set_freqme_clk_clr(&mut self, val: super::vals::FreqmeClkClr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl1Clr {
    #[inline(always)]
    fn default() -> Pscctl1Clr {
        Pscctl1Clr(0)
    }
}
impl core::fmt::Debug for Pscctl1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl1Clr")
            .field("hsgpio0_clk_clr", &self.hsgpio0_clk_clr())
            .field("hsgpio1_clk_clr", &self.hsgpio1_clk_clr())
            .field("hsgpio2_clk_clr", &self.hsgpio2_clk_clr())
            .field("hsgpio3_clk_clr", &self.hsgpio3_clk_clr())
            .field("hsgpio4_clk_clr", &self.hsgpio4_clk_clr())
            .field("hsgpio5_clk_clr", &self.hsgpio5_clk_clr())
            .field("hsgpio6_clk_clr", &self.hsgpio6_clk_clr())
            .field("hsgpio7_clk_clr", &self.hsgpio7_clk_clr())
            .field("crc_clk_clr", &self.crc_clk_clr())
            .field("dmac0_clk_clr", &self.dmac0_clk_clr())
            .field("dmac1_clk_clr", &self.dmac1_clk_clr())
            .field("mu_clk_clr", &self.mu_clk_clr())
            .field("sema_clk_clr", &self.sema_clk_clr())
            .field("freqme_clk_clr", &self.freqme_clk_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1Clr {{ hsgpio0_clk_clr: {:?}, hsgpio1_clk_clr: {:?}, hsgpio2_clk_clr: {:?}, hsgpio3_clk_clr: {:?}, hsgpio4_clk_clr: {:?}, hsgpio5_clk_clr: {:?}, hsgpio6_clk_clr: {:?}, hsgpio7_clk_clr: {:?}, crc_clk_clr: {:?}, dmac0_clk_clr: {:?}, dmac1_clk_clr: {:?}, mu_clk_clr: {:?}, sema_clk_clr: {:?}, freqme_clk_clr: {:?} }}",
            self.hsgpio0_clk_clr(),
            self.hsgpio1_clk_clr(),
            self.hsgpio2_clk_clr(),
            self.hsgpio3_clk_clr(),
            self.hsgpio4_clk_clr(),
            self.hsgpio5_clk_clr(),
            self.hsgpio6_clk_clr(),
            self.hsgpio7_clk_clr(),
            self.crc_clk_clr(),
            self.dmac0_clk_clr(),
            self.dmac1_clk_clr(),
            self.mu_clk_clr(),
            self.sema_clk_clr(),
            self.freqme_clk_clr()
        )
    }
}
#[doc = "clock set register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Set(pub u32);
impl Pscctl1Set {
    #[doc = "HSGPIO0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio0_clk_set(&self) -> super::vals::Hsgpio0ClkSet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio0_clk_set(&mut self, val: super::vals::Hsgpio0ClkSet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio1_clk_set(&self) -> super::vals::Hsgpio1ClkSet {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio1_clk_set(&mut self, val: super::vals::Hsgpio1ClkSet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio2_clk_set(&self) -> super::vals::Hsgpio2ClkSet {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio2_clk_set(&mut self, val: super::vals::Hsgpio2ClkSet) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio3_clk_set(&self) -> super::vals::Hsgpio3ClkSet {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio3_clk_set(&mut self, val: super::vals::Hsgpio3ClkSet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio4_clk_set(&self) -> super::vals::Hsgpio4ClkSet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio4_clk_set(&mut self, val: super::vals::Hsgpio4ClkSet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio5_clk_set(&self) -> super::vals::Hsgpio5ClkSet {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio5_clk_set(&mut self, val: super::vals::Hsgpio5ClkSet) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio6_clk_set(&self) -> super::vals::Hsgpio6ClkSet {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio6_clk_set(&mut self, val: super::vals::Hsgpio6ClkSet) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hsgpio7_clk_set(&self) -> super::vals::Hsgpio7ClkSet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7ClkSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 clock set"]
    #[inline(always)]
    pub const fn set_hsgpio7_clk_set(&mut self, val: super::vals::Hsgpio7ClkSet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_clk_set(&self) -> super::vals::CrcClkSet {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcClkSet::from_bits(val as u8)
    }
    #[doc = "CRC clock set"]
    #[inline(always)]
    pub const fn set_crc_clk_set(&mut self, val: super::vals::CrcClkSet) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0_clk_set(&self) -> super::vals::Dmac0ClkSet {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ClkSet::from_bits(val as u8)
    }
    #[doc = "DMAC0 clock set"]
    #[inline(always)]
    pub const fn set_dmac0_clk_set(&mut self, val: super::vals::Dmac0ClkSet) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1_clk_set(&self) -> super::vals::Dmac1ClkSet {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ClkSet::from_bits(val as u8)
    }
    #[doc = "DMAC1 clock set"]
    #[inline(always)]
    pub const fn set_dmac1_clk_set(&mut self, val: super::vals::Dmac1ClkSet) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn mu_clk_set(&self) -> super::vals::MuClkSet {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuClkSet::from_bits(val as u8)
    }
    #[doc = "MU clock set"]
    #[inline(always)]
    pub const fn set_mu_clk_set(&mut self, val: super::vals::MuClkSet) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn sema_clk_set(&self) -> super::vals::SemaClkSet {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaClkSet::from_bits(val as u8)
    }
    #[doc = "SEMA clock set"]
    #[inline(always)]
    pub const fn set_sema_clk_set(&mut self, val: super::vals::SemaClkSet) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_clk_set(&self) -> super::vals::FreqmeClkSet {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeClkSet::from_bits(val as u8)
    }
    #[doc = "FREQME clock set"]
    #[inline(always)]
    pub const fn set_freqme_clk_set(&mut self, val: super::vals::FreqmeClkSet) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl1Set {
    #[inline(always)]
    fn default() -> Pscctl1Set {
        Pscctl1Set(0)
    }
}
impl core::fmt::Debug for Pscctl1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl1Set")
            .field("hsgpio0_clk_set", &self.hsgpio0_clk_set())
            .field("hsgpio1_clk_set", &self.hsgpio1_clk_set())
            .field("hsgpio2_clk_set", &self.hsgpio2_clk_set())
            .field("hsgpio3_clk_set", &self.hsgpio3_clk_set())
            .field("hsgpio4_clk_set", &self.hsgpio4_clk_set())
            .field("hsgpio5_clk_set", &self.hsgpio5_clk_set())
            .field("hsgpio6_clk_set", &self.hsgpio6_clk_set())
            .field("hsgpio7_clk_set", &self.hsgpio7_clk_set())
            .field("crc_clk_set", &self.crc_clk_set())
            .field("dmac0_clk_set", &self.dmac0_clk_set())
            .field("dmac1_clk_set", &self.dmac1_clk_set())
            .field("mu_clk_set", &self.mu_clk_set())
            .field("sema_clk_set", &self.sema_clk_set())
            .field("freqme_clk_set", &self.freqme_clk_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1Set {{ hsgpio0_clk_set: {:?}, hsgpio1_clk_set: {:?}, hsgpio2_clk_set: {:?}, hsgpio3_clk_set: {:?}, hsgpio4_clk_set: {:?}, hsgpio5_clk_set: {:?}, hsgpio6_clk_set: {:?}, hsgpio7_clk_set: {:?}, crc_clk_set: {:?}, dmac0_clk_set: {:?}, dmac1_clk_set: {:?}, mu_clk_set: {:?}, sema_clk_set: {:?}, freqme_clk_set: {:?} }}",
            self.hsgpio0_clk_set(),
            self.hsgpio1_clk_set(),
            self.hsgpio2_clk_set(),
            self.hsgpio3_clk_set(),
            self.hsgpio4_clk_set(),
            self.hsgpio5_clk_set(),
            self.hsgpio6_clk_set(),
            self.hsgpio7_clk_set(),
            self.crc_clk_set(),
            self.dmac0_clk_set(),
            self.dmac1_clk_set(),
            self.mu_clk_set(),
            self.sema_clk_set(),
            self.freqme_clk_set()
        )
    }
}
#[doc = "clock control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2(pub u32);
impl Pscctl2 {
    #[doc = "ct32bit timer 0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0_clk(&self) -> super::vals::Ct32bit0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0Clk::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 0 clock control"]
    #[inline(always)]
    pub const fn set_ct32bit0_clk(&mut self, val: super::vals::Ct32bit0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ct32bit timer 1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1_clk(&self) -> super::vals::Ct32bit1Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1Clk::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 1 clock control"]
    #[inline(always)]
    pub const fn set_ct32bit1_clk(&mut self, val: super::vals::Ct32bit1Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ct32bit timer 2 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2_clk(&self) -> super::vals::Ct32bit2Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2Clk::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 2 clock control"]
    #[inline(always)]
    pub const fn set_ct32bit2_clk(&mut self, val: super::vals::Ct32bit2Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ct32bit timer 3 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3_clk(&self) -> super::vals::Ct32bit3Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3Clk::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 3 clock control"]
    #[inline(always)]
    pub const fn set_ct32bit3_clk(&mut self, val: super::vals::Ct32bit3Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ct32bit timer 4 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4_clk(&self) -> super::vals::Ct32bit4Clk {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4Clk::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 4 clock control"]
    #[inline(always)]
    pub const fn set_ct32bit4_clk(&mut self, val: super::vals::Ct32bit4Clk) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "rtc lite clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite_clk(&self) -> super::vals::RtcLiteClk {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RtcLiteClk::from_bits(val as u8)
    }
    #[doc = "rtc lite clock control"]
    #[inline(always)]
    pub const fn set_rtc_lite_clk(&mut self, val: super::vals::RtcLiteClk) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mrt0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0_clk(&self) -> super::vals::Mrt0Clk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0Clk::from_bits(val as u8)
    }
    #[doc = "mrt0 clock control"]
    #[inline(always)]
    pub const fn set_mrt0_clk(&mut self, val: super::vals::Mrt0Clk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "wdt1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1_clk(&self) -> super::vals::Wwdt1Clk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1Clk::from_bits(val as u8)
    }
    #[doc = "wdt1 clock control"]
    #[inline(always)]
    pub const fn set_wwdt1_clk(&mut self, val: super::vals::Wwdt1Clk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "i3c0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_clk(&self) -> super::vals::I3c0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0Clk::from_bits(val as u8)
    }
    #[doc = "i3c0 clock control"]
    #[inline(always)]
    pub const fn set_i3c0_clk(&mut self, val: super::vals::I3c0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiointctl_clk(&self) -> super::vals::GpiointctlClk {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlClk::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL clock control"]
    #[inline(always)]
    pub const fn set_gpiointctl_clk(&mut self, val: super::vals::GpiointctlClk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PIMCTL clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn pimctl_clk(&self) -> super::vals::PimctlClk {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlClk::from_bits(val as u8)
    }
    #[doc = "PIMCTL clock control"]
    #[inline(always)]
    pub const fn set_pimctl_clk(&mut self, val: super::vals::PimctlClk) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl2 {
    #[inline(always)]
    fn default() -> Pscctl2 {
        Pscctl2(0)
    }
}
impl core::fmt::Debug for Pscctl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl2")
            .field("ct32bit0_clk", &self.ct32bit0_clk())
            .field("ct32bit1_clk", &self.ct32bit1_clk())
            .field("ct32bit2_clk", &self.ct32bit2_clk())
            .field("ct32bit3_clk", &self.ct32bit3_clk())
            .field("ct32bit4_clk", &self.ct32bit4_clk())
            .field("rtc_lite_clk", &self.rtc_lite_clk())
            .field("mrt0_clk", &self.mrt0_clk())
            .field("wwdt1_clk", &self.wwdt1_clk())
            .field("i3c0_clk", &self.i3c0_clk())
            .field("gpiointctl_clk", &self.gpiointctl_clk())
            .field("pimctl_clk", &self.pimctl_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2 {{ ct32bit0_clk: {:?}, ct32bit1_clk: {:?}, ct32bit2_clk: {:?}, ct32bit3_clk: {:?}, ct32bit4_clk: {:?}, rtc_lite_clk: {:?}, mrt0_clk: {:?}, wwdt1_clk: {:?}, i3c0_clk: {:?}, gpiointctl_clk: {:?}, pimctl_clk: {:?} }}",
            self.ct32bit0_clk(),
            self.ct32bit1_clk(),
            self.ct32bit2_clk(),
            self.ct32bit3_clk(),
            self.ct32bit4_clk(),
            self.rtc_lite_clk(),
            self.mrt0_clk(),
            self.wwdt1_clk(),
            self.i3c0_clk(),
            self.gpiointctl_clk(),
            self.pimctl_clk()
        )
    }
}
#[doc = "clock clear register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Clr(pub u32);
impl Pscctl2Clr {
    #[doc = "ct32bit timer 0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0_clk_clr(&self) -> super::vals::Ct32bit0ClkClr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0ClkClr::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 0 clock clear"]
    #[inline(always)]
    pub const fn set_ct32bit0_clk_clr(&mut self, val: super::vals::Ct32bit0ClkClr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ct32bit timer 1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1_clk_clr(&self) -> super::vals::Ct32bit1ClkClr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1ClkClr::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 1 clock clear"]
    #[inline(always)]
    pub const fn set_ct32bit1_clk_clr(&mut self, val: super::vals::Ct32bit1ClkClr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ct32bit timer 2 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2_clk_clr(&self) -> super::vals::Ct32bit2ClkClr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2ClkClr::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 2 clock clear"]
    #[inline(always)]
    pub const fn set_ct32bit2_clk_clr(&mut self, val: super::vals::Ct32bit2ClkClr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ct32bit timer 3 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3_clk_clr(&self) -> super::vals::Ct32bit3ClkClr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3ClkClr::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 3 clock clear"]
    #[inline(always)]
    pub const fn set_ct32bit3_clk_clr(&mut self, val: super::vals::Ct32bit3ClkClr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ct32bit timer 4 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4_clk_clr(&self) -> super::vals::Ct32bit4ClkClr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4ClkClr::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 4 clock clear"]
    #[inline(always)]
    pub const fn set_ct32bit4_clk_clr(&mut self, val: super::vals::Ct32bit4ClkClr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "rtc lite clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite_clk_clr(&self) -> super::vals::RtcLiteClkClr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RtcLiteClkClr::from_bits(val as u8)
    }
    #[doc = "rtc lite clock clear"]
    #[inline(always)]
    pub const fn set_rtc_lite_clk_clr(&mut self, val: super::vals::RtcLiteClkClr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mrt0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0_clk_clr(&self) -> super::vals::Mrt0ClkClr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0ClkClr::from_bits(val as u8)
    }
    #[doc = "mrt0 clock clear"]
    #[inline(always)]
    pub const fn set_mrt0_clk_clr(&mut self, val: super::vals::Mrt0ClkClr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "wdt1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1_clk_clr(&self) -> super::vals::Wwdt1ClkClr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1ClkClr::from_bits(val as u8)
    }
    #[doc = "wdt1 clock clear"]
    #[inline(always)]
    pub const fn set_wwdt1_clk_clr(&mut self, val: super::vals::Wwdt1ClkClr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "i3c0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_clk_clr(&self) -> super::vals::I3c0ClkClr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0ClkClr::from_bits(val as u8)
    }
    #[doc = "i3c0 clock clear"]
    #[inline(always)]
    pub const fn set_i3c0_clk_clr(&mut self, val: super::vals::I3c0ClkClr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiointctl_clk_clr(&self) -> super::vals::GpiointctlClkClr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlClkClr::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL clock clear"]
    #[inline(always)]
    pub const fn set_gpiointctl_clk_clr(&mut self, val: super::vals::GpiointctlClkClr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PIMCTL clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn pimctl_clk_clr(&self) -> super::vals::PimctlClkClr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlClkClr::from_bits(val as u8)
    }
    #[doc = "PIMCTL clock clear"]
    #[inline(always)]
    pub const fn set_pimctl_clk_clr(&mut self, val: super::vals::PimctlClkClr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl2Clr {
    #[inline(always)]
    fn default() -> Pscctl2Clr {
        Pscctl2Clr(0)
    }
}
impl core::fmt::Debug for Pscctl2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl2Clr")
            .field("ct32bit0_clk_clr", &self.ct32bit0_clk_clr())
            .field("ct32bit1_clk_clr", &self.ct32bit1_clk_clr())
            .field("ct32bit2_clk_clr", &self.ct32bit2_clk_clr())
            .field("ct32bit3_clk_clr", &self.ct32bit3_clk_clr())
            .field("ct32bit4_clk_clr", &self.ct32bit4_clk_clr())
            .field("rtc_lite_clk_clr", &self.rtc_lite_clk_clr())
            .field("mrt0_clk_clr", &self.mrt0_clk_clr())
            .field("wwdt1_clk_clr", &self.wwdt1_clk_clr())
            .field("i3c0_clk_clr", &self.i3c0_clk_clr())
            .field("gpiointctl_clk_clr", &self.gpiointctl_clk_clr())
            .field("pimctl_clk_clr", &self.pimctl_clk_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2Clr {{ ct32bit0_clk_clr: {:?}, ct32bit1_clk_clr: {:?}, ct32bit2_clk_clr: {:?}, ct32bit3_clk_clr: {:?}, ct32bit4_clk_clr: {:?}, rtc_lite_clk_clr: {:?}, mrt0_clk_clr: {:?}, wwdt1_clk_clr: {:?}, i3c0_clk_clr: {:?}, gpiointctl_clk_clr: {:?}, pimctl_clk_clr: {:?} }}",
            self.ct32bit0_clk_clr(),
            self.ct32bit1_clk_clr(),
            self.ct32bit2_clk_clr(),
            self.ct32bit3_clk_clr(),
            self.ct32bit4_clk_clr(),
            self.rtc_lite_clk_clr(),
            self.mrt0_clk_clr(),
            self.wwdt1_clk_clr(),
            self.i3c0_clk_clr(),
            self.gpiointctl_clk_clr(),
            self.pimctl_clk_clr()
        )
    }
}
#[doc = "clock set register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Set(pub u32);
impl Pscctl2Set {
    #[doc = "ct32bit timer 0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0_clk_set(&self) -> super::vals::Ct32bit0ClkSet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0ClkSet::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 0 clock set"]
    #[inline(always)]
    pub const fn set_ct32bit0_clk_set(&mut self, val: super::vals::Ct32bit0ClkSet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ct32bit timer 1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1_clk_set(&self) -> super::vals::Ct32bit1ClkSet {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1ClkSet::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 1 clock set"]
    #[inline(always)]
    pub const fn set_ct32bit1_clk_set(&mut self, val: super::vals::Ct32bit1ClkSet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ct32bit timer 2 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2_clk_set(&self) -> super::vals::Ct32bit2ClkSet {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2ClkSet::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 2 clock set"]
    #[inline(always)]
    pub const fn set_ct32bit2_clk_set(&mut self, val: super::vals::Ct32bit2ClkSet) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ct32bit timer 3 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3_clk_set(&self) -> super::vals::Ct32bit3ClkSet {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3ClkSet::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 3 clock set"]
    #[inline(always)]
    pub const fn set_ct32bit3_clk_set(&mut self, val: super::vals::Ct32bit3ClkSet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ct32bit timer 4 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4_clk_set(&self) -> super::vals::Ct32bit4ClkSet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4ClkSet::from_bits(val as u8)
    }
    #[doc = "ct32bit timer 4 clock set"]
    #[inline(always)]
    pub const fn set_ct32bit4_clk_set(&mut self, val: super::vals::Ct32bit4ClkSet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "rtc lite clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite_clk_set(&self) -> super::vals::RtcLiteClkSet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RtcLiteClkSet::from_bits(val as u8)
    }
    #[doc = "rtc lite clock set"]
    #[inline(always)]
    pub const fn set_rtc_lite_clk_set(&mut self, val: super::vals::RtcLiteClkSet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "mrt0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0_clk_set(&self) -> super::vals::Mrt0ClkSet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0ClkSet::from_bits(val as u8)
    }
    #[doc = "mrt0 clock set"]
    #[inline(always)]
    pub const fn set_mrt0_clk_set(&mut self, val: super::vals::Mrt0ClkSet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "wdt1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1_clk_set(&self) -> super::vals::Wwdt1ClkSet {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1ClkSet::from_bits(val as u8)
    }
    #[doc = "wdt1 clock set"]
    #[inline(always)]
    pub const fn set_wwdt1_clk_set(&mut self, val: super::vals::Wwdt1ClkSet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "i3c0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_clk_set(&self) -> super::vals::I3c0ClkSet {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0ClkSet::from_bits(val as u8)
    }
    #[doc = "i3c0 clock set"]
    #[inline(always)]
    pub const fn set_i3c0_clk_set(&mut self, val: super::vals::I3c0ClkSet) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn gpiointctl_clk_set(&self) -> super::vals::GpiointctlClkSet {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlClkSet::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL clock set"]
    #[inline(always)]
    pub const fn set_gpiointctl_clk_set(&mut self, val: super::vals::GpiointctlClkSet) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PIMCTL clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn pimctl_clk_set(&self) -> super::vals::PimctlClkSet {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlClkSet::from_bits(val as u8)
    }
    #[doc = "PIMCTL clock set"]
    #[inline(always)]
    pub const fn set_pimctl_clk_set(&mut self, val: super::vals::PimctlClkSet) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pscctl2Set {
    #[inline(always)]
    fn default() -> Pscctl2Set {
        Pscctl2Set(0)
    }
}
impl core::fmt::Debug for Pscctl2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pscctl2Set")
            .field("ct32bit0_clk_set", &self.ct32bit0_clk_set())
            .field("ct32bit1_clk_set", &self.ct32bit1_clk_set())
            .field("ct32bit2_clk_set", &self.ct32bit2_clk_set())
            .field("ct32bit3_clk_set", &self.ct32bit3_clk_set())
            .field("ct32bit4_clk_set", &self.ct32bit4_clk_set())
            .field("rtc_lite_clk_set", &self.rtc_lite_clk_set())
            .field("mrt0_clk_set", &self.mrt0_clk_set())
            .field("wwdt1_clk_set", &self.wwdt1_clk_set())
            .field("i3c0_clk_set", &self.i3c0_clk_set())
            .field("gpiointctl_clk_set", &self.gpiointctl_clk_set())
            .field("pimctl_clk_set", &self.pimctl_clk_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2Set {{ ct32bit0_clk_set: {:?}, ct32bit1_clk_set: {:?}, ct32bit2_clk_set: {:?}, ct32bit3_clk_set: {:?}, ct32bit4_clk_set: {:?}, rtc_lite_clk_set: {:?}, mrt0_clk_set: {:?}, wwdt1_clk_set: {:?}, i3c0_clk_set: {:?}, gpiointctl_clk_set: {:?}, pimctl_clk_set: {:?} }}",
            self.ct32bit0_clk_set(),
            self.ct32bit1_clk_set(),
            self.ct32bit2_clk_set(),
            self.ct32bit3_clk_set(),
            self.ct32bit4_clk_set(),
            self.rtc_lite_clk_set(),
            self.mrt0_clk_set(),
            self.wwdt1_clk_set(),
            self.i3c0_clk_set(),
            self.gpiointctl_clk_set(),
            self.pimctl_clk_set()
        )
    }
}
#[doc = "WDT1 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt1fclksel(pub u32);
impl Wdt1fclksel {
    #[doc = "WDT1 Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wdt1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wdt1fclkselSel::from_bits(val as u8)
    }
    #[doc = "WDT1 Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Wdt1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Wdt1fclksel {
    #[inline(always)]
    fn default() -> Wdt1fclksel {
        Wdt1fclksel(0)
    }
}
impl core::fmt::Debug for Wdt1fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt1fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt1fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdt1fclksel {{ sel: {:?} }}", self.sel())
    }
}
