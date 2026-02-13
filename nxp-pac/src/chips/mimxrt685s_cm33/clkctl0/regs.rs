#[doc = "ADC0 fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclkdiv(pub u32);
impl Adc0fclkdiv {
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
impl Default for Adc0fclkdiv {
    #[inline(always)]
    fn default() -> Adc0fclkdiv {
        Adc0fclkdiv(0)
    }
}
impl core::fmt::Debug for Adc0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc0fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "ADC0 fclk selection 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclksel0(pub u32);
impl Adc0fclksel0 {
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc0fclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc0fclksel0Sel::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Adc0fclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0fclksel0 {
    #[inline(always)]
    fn default() -> Adc0fclksel0 {
        Adc0fclksel0(0)
    }
}
impl core::fmt::Debug for Adc0fclksel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0fclksel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0fclksel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc0fclksel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ADC0 fclk selection 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclksel1(pub u32);
impl Adc0fclksel1 {
    #[doc = "ADC Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc0fclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc0fclksel1Sel::from_bits(val as u8)
    }
    #[doc = "ADC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Adc0fclksel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0fclksel1 {
    #[inline(always)]
    fn default() -> Adc0fclksel1 {
        Adc0fclksel1(0)
    }
}
impl core::fmt::Debug for Adc0fclksel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0fclksel1")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0fclksel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc0fclksel1 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "aux0 pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux0pllclkdiv(pub u32);
impl Aux0pllclkdiv {
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
impl Default for Aux0pllclkdiv {
    #[inline(always)]
    fn default() -> Aux0pllclkdiv {
        Aux0pllclkdiv(0)
    }
}
impl core::fmt::Debug for Aux0pllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux0pllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux0pllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aux0pllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "aux1 pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux1pllclkdiv(pub u32);
impl Aux1pllclkdiv {
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
impl Default for Aux1pllclkdiv {
    #[inline(always)]
    fn default() -> Aux1pllclkdiv {
        Aux1pllclkdiv(0)
    }
}
impl core::fmt::Debug for Aux1pllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aux1pllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aux1pllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aux1pllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "dsp pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsppllclkdiv(pub u32);
impl Dsppllclkdiv {
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
impl Default for Dsppllclkdiv {
    #[inline(always)]
    fn default() -> Dsppllclkdiv {
        Dsppllclkdiv(0)
    }
}
impl core::fmt::Debug for Dsppllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dsppllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dsppllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dsppllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "ESPI clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espifclksel0(pub u32);
impl Espifclksel0 {
    #[doc = "eSPI Functional Clock Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Espifclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Espifclksel0Sel::from_bits(val as u8)
    }
    #[doc = "eSPI Functional Clock Source Selection."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Espifclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Espifclksel0 {
    #[inline(always)]
    fn default() -> Espifclksel0 {
        Espifclksel0(0)
    }
}
impl core::fmt::Debug for Espifclksel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Espifclksel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Espifclksel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Espifclksel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FFRO control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffroctl0(pub u32);
impl Ffroctl0 {
    #[doc = "Trims temperature compensation of FFRO."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_tempco(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trims temperature compensation of FFRO."]
    #[inline(always)]
    pub const fn set_trim_tempco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trims coarse frequency of FFRO."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_coarse(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x3f;
        val as u8
    }
    #[doc = "Trims coarse frequency of FFRO."]
    #[inline(always)]
    pub const fn set_trim_coarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 5usize)) | (((val as u32) & 0x3f) << 5usize);
    }
    #[doc = "Trims fine frequency of FFRO."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_fine(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x7f;
        val as u8
    }
    #[doc = "Trims fine frequency of FFRO."]
    #[inline(always)]
    pub const fn set_trim_fine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 11usize)) | (((val as u32) & 0x7f) << 11usize);
    }
    #[doc = "Trims frequency range of FFRO."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_range(&self) -> super::vals::TrimRange {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::TrimRange::from_bits(val as u8)
    }
    #[doc = "Trims frequency range of FFRO."]
    #[inline(always)]
    pub const fn set_trim_range(&mut self, val: super::vals::TrimRange) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for Ffroctl0 {
    #[inline(always)]
    fn default() -> Ffroctl0 {
        Ffroctl0(0)
    }
}
impl core::fmt::Debug for Ffroctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffroctl0")
            .field("trim_tempco", &self.trim_tempco())
            .field("trim_coarse", &self.trim_coarse())
            .field("trim_fine", &self.trim_fine())
            .field("trim_range", &self.trim_range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffroctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ffroctl0 {{ trim_tempco: {=u8:?}, trim_coarse: {=u8:?}, trim_fine: {=u8:?}, trim_range: {:?} }}",
            self.trim_tempco(),
            self.trim_coarse(),
            self.trim_fine(),
            self.trim_range()
        )
    }
}
#[doc = "FFRO control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffroctl1(pub u32);
impl Ffroctl1 {
    #[doc = "Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[must_use]
    #[inline(always)]
    pub const fn update(&self) -> super::vals::Update {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Update::from_bits(val as u8)
    }
    #[doc = "Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[inline(always)]
    pub const fn set_update(&mut self, val: super::vals::Update) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ffroctl1 {
    #[inline(always)]
    fn default() -> Ffroctl1 {
        Ffroctl1(0)
    }
}
impl core::fmt::Debug for Ffroctl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffroctl1")
            .field("update", &self.update())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffroctl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ffroctl1 {{ update: {:?} }}", self.update())
    }
}
#[doc = "FlexSPI FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspifclkdiv(pub u32);
impl Flexspifclkdiv {
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
impl Default for Flexspifclkdiv {
    #[inline(always)]
    fn default() -> Flexspifclkdiv {
        Flexspifclkdiv(0)
    }
}
impl core::fmt::Debug for Flexspifclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspifclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspifclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspifclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "FlexSPI FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspifclksel(pub u32);
impl Flexspifclksel {
    #[doc = "FlexSPI Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FlexspifclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FlexspifclkselSel::from_bits(val as u8)
    }
    #[doc = "FlexSPI Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::FlexspifclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexspifclksel {
    #[inline(always)]
    fn default() -> Flexspifclksel {
        Flexspifclksel(0)
    }
}
impl core::fmt::Debug for Flexspifclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspifclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspifclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexspifclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "low power oscillator control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposcctl0(pub u32);
impl Lposcctl0 {
    #[doc = "Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[must_use]
    #[inline(always)]
    pub const fn clkrdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[inline(always)]
    pub const fn set_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lposcctl0 {
    #[inline(always)]
    fn default() -> Lposcctl0 {
        Lposcctl0(0)
    }
}
impl core::fmt::Debug for Lposcctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lposcctl0")
            .field("clkrdy", &self.clkrdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lposcctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lposcctl0 {{ clkrdy: {=bool:?} }}", self.clkrdy())
    }
}
#[doc = "main clock selection A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclksela(pub u32);
impl Mainclksela {
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselaSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MainclkselaSel::from_bits(val as u8)
    }
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MainclkselaSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mainclksela {
    #[inline(always)]
    fn default() -> Mainclksela {
        Mainclksela(0)
    }
}
impl core::fmt::Debug for Mainclksela {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainclksela")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainclksela {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mainclksela {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "main clock selection B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclkselb(pub u32);
impl Mainclkselb {
    #[doc = "Main Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselbSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MainclkselbSel::from_bits(val as u8)
    }
    #[doc = "Main Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::MainclkselbSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mainclkselb {
    #[inline(always)]
    fn default() -> Mainclkselb {
        Mainclkselb(0)
    }
}
impl core::fmt::Debug for Mainclkselb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainclkselb")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainclkselb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mainclkselb {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "main pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainpllclkdiv(pub u32);
impl Mainpllclkdiv {
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
impl Default for Mainpllclkdiv {
    #[inline(always)]
    fn default() -> Mainpllclkdiv {
        Mainpllclkdiv(0)
    }
}
impl core::fmt::Debug for Mainpllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainpllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainpllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mainpllclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "32k oscillator control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc32khzctl0(pub u32);
impl Osc32khzctl0 {
    #[doc = "32KHz Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ena32khz(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "32KHz Enable."]
    #[inline(always)]
    pub const fn set_ena32khz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Osc32khzctl0 {
    #[inline(always)]
    fn default() -> Osc32khzctl0 {
        Osc32khzctl0(0)
    }
}
impl core::fmt::Debug for Osc32khzctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osc32khzctl0")
            .field("ena32khz", &self.ena32khz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osc32khzctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Osc32khzctl0 {{ ena32khz: {=bool:?} }}", self.ena32khz())
    }
}
#[doc = "PFC divider register N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfcdiv(pub u32);
impl Pfcdiv {
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
impl Default for Pfcdiv {
    #[inline(always)]
    fn default() -> Pfcdiv {
        Pfcdiv(0)
    }
}
impl core::fmt::Debug for Pfcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfcdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "clock control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0(pub u32);
impl Pscctl0 {
    #[doc = "128KB ROM control"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_ctl_128kb(&self) -> super::vals::RomCtl128kb {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RomCtl128kb::from_bits(val as u8)
    }
    #[doc = "128KB ROM control"]
    #[inline(always)]
    pub const fn set_rom_ctl_128kb(&mut self, val: super::vals::RomCtl128kb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0PowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0PowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock control"]
    #[inline(always)]
    pub const fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0PowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0CasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0CasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock control"]
    #[inline(always)]
    pub const fn set_casper_clk(&mut self, val: super::vals::Pscctl0CasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0HashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0HashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock control"]
    #[inline(always)]
    pub const fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0HashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0PufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0PufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock control"]
    #[inline(always)]
    pub const fn set_puf_clk(&mut self, val: super::vals::Pscctl0PufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0RngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0RngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock control"]
    #[inline(always)]
    pub const fn set_rng_clk(&mut self, val: super::vals::Pscctl0RngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0FlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0FlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock control"]
    #[inline(always)]
    pub const fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0FlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0OtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0OtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock control"]
    #[inline(always)]
    pub const fn set_otp_clk(&mut self, val: super::vals::Pscctl0OtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0UsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0UsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock control"]
    #[inline(always)]
    pub const fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0UsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0UsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0UsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock control"]
    #[inline(always)]
    pub const fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0UsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0UsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0UsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock control"]
    #[inline(always)]
    pub const fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0UsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0UsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0UsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock control"]
    #[inline(always)]
    pub const fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0UsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0SctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0SctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock control"]
    #[inline(always)]
    pub const fn set_sct_clk(&mut self, val: super::vals::Pscctl0SctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("rom_ctl_128kb", &self.rom_ctl_128kb())
            .field("powerquad_clk", &self.powerquad_clk())
            .field("casper_clk", &self.casper_clk())
            .field("hashcrypt_clk", &self.hashcrypt_clk())
            .field("puf_clk", &self.puf_clk())
            .field("rng_clk", &self.rng_clk())
            .field("flexspi_otfad_clk", &self.flexspi_otfad_clk())
            .field("otp_clk", &self.otp_clk())
            .field("usbhs_phy_clk", &self.usbhs_phy_clk())
            .field("usbhs_device_clk", &self.usbhs_device_clk())
            .field("usbhs_host_clk", &self.usbhs_host_clk())
            .field("usbhs_sram_clk", &self.usbhs_sram_clk())
            .field("sct_clk", &self.sct_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0 {{ rom_ctl_128kb: {:?}, powerquad_clk: {:?}, casper_clk: {:?}, hashcrypt_clk: {:?}, puf_clk: {:?}, rng_clk: {:?}, flexspi_otfad_clk: {:?}, otp_clk: {:?}, usbhs_phy_clk: {:?}, usbhs_device_clk: {:?}, usbhs_host_clk: {:?}, usbhs_sram_clk: {:?}, sct_clk: {:?} }}",
            self.rom_ctl_128kb(),
            self.powerquad_clk(),
            self.casper_clk(),
            self.hashcrypt_clk(),
            self.puf_clk(),
            self.rng_clk(),
            self.flexspi_otfad_clk(),
            self.otp_clk(),
            self.usbhs_phy_clk(),
            self.usbhs_device_clk(),
            self.usbhs_host_clk(),
            self.usbhs_sram_clk(),
            self.sct_clk()
        )
    }
}
#[doc = "clock clear register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Clr(pub u32);
impl Pscctl0Clr {
    #[doc = "ROM controller clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_ctl_128kb_clk(&self) -> super::vals::Pscctl0ClrRomCtl128kbClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl0ClrRomCtl128kbClk::from_bits(val as u8)
    }
    #[doc = "ROM controller clock clear"]
    #[inline(always)]
    pub const fn set_rom_ctl_128kb_clk(&mut self, val: super::vals::Pscctl0ClrRomCtl128kbClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0ClrPowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0ClrPowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock clear"]
    #[inline(always)]
    pub const fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0ClrPowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0ClrCasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0ClrCasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock clear"]
    #[inline(always)]
    pub const fn set_casper_clk(&mut self, val: super::vals::Pscctl0ClrCasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0ClrHashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0ClrHashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock clear"]
    #[inline(always)]
    pub const fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0ClrHashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0ClrPufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0ClrPufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock clear"]
    #[inline(always)]
    pub const fn set_puf_clk(&mut self, val: super::vals::Pscctl0ClrPufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0ClrRngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0ClrRngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock clear"]
    #[inline(always)]
    pub const fn set_rng_clk(&mut self, val: super::vals::Pscctl0ClrRngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0ClrFlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0ClrFlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock clear"]
    #[inline(always)]
    pub const fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0ClrFlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0ClrOtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0ClrOtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock clear"]
    #[inline(always)]
    pub const fn set_otp_clk(&mut self, val: super::vals::Pscctl0ClrOtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0ClrUsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock clear"]
    #[inline(always)]
    pub const fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0ClrUsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock clear"]
    #[inline(always)]
    pub const fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0ClrUsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock clear"]
    #[inline(always)]
    pub const fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0ClrUsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock clear"]
    #[inline(always)]
    pub const fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0ClrSctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0ClrSctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock clear"]
    #[inline(always)]
    pub const fn set_sct_clk(&mut self, val: super::vals::Pscctl0ClrSctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("rom_ctl_128kb_clk", &self.rom_ctl_128kb_clk())
            .field("powerquad_clk", &self.powerquad_clk())
            .field("casper_clk", &self.casper_clk())
            .field("hashcrypt_clk", &self.hashcrypt_clk())
            .field("puf_clk", &self.puf_clk())
            .field("rng_clk", &self.rng_clk())
            .field("flexspi_otfad_clk", &self.flexspi_otfad_clk())
            .field("otp_clk", &self.otp_clk())
            .field("usbhs_phy_clk", &self.usbhs_phy_clk())
            .field("usbhs_device_clk", &self.usbhs_device_clk())
            .field("usbhs_host_clk", &self.usbhs_host_clk())
            .field("usbhs_sram_clk", &self.usbhs_sram_clk())
            .field("sct_clk", &self.sct_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0Clr {{ rom_ctl_128kb_clk: {:?}, powerquad_clk: {:?}, casper_clk: {:?}, hashcrypt_clk: {:?}, puf_clk: {:?}, rng_clk: {:?}, flexspi_otfad_clk: {:?}, otp_clk: {:?}, usbhs_phy_clk: {:?}, usbhs_device_clk: {:?}, usbhs_host_clk: {:?}, usbhs_sram_clk: {:?}, sct_clk: {:?} }}",
            self.rom_ctl_128kb_clk(),
            self.powerquad_clk(),
            self.casper_clk(),
            self.hashcrypt_clk(),
            self.puf_clk(),
            self.rng_clk(),
            self.flexspi_otfad_clk(),
            self.otp_clk(),
            self.usbhs_phy_clk(),
            self.usbhs_device_clk(),
            self.usbhs_host_clk(),
            self.usbhs_sram_clk(),
            self.sct_clk()
        )
    }
}
#[doc = "clock set register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Set(pub u32);
impl Pscctl0Set {
    #[doc = "128KB ROM controller clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_ctl_128kb_clk(&self) -> super::vals::Pscctl0SetRomCtl128kbClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl0SetRomCtl128kbClk::from_bits(val as u8)
    }
    #[doc = "128KB ROM controller clock set"]
    #[inline(always)]
    pub const fn set_rom_ctl_128kb_clk(&mut self, val: super::vals::Pscctl0SetRomCtl128kbClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0SetPowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0SetPowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock set"]
    #[inline(always)]
    pub const fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0SetPowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0SetCasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0SetCasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock set"]
    #[inline(always)]
    pub const fn set_casper_clk(&mut self, val: super::vals::Pscctl0SetCasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0SetHashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0SetHashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock set"]
    #[inline(always)]
    pub const fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0SetHashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0SetPufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0SetPufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock set"]
    #[inline(always)]
    pub const fn set_puf_clk(&mut self, val: super::vals::Pscctl0SetPufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0SetRngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0SetRngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock set"]
    #[inline(always)]
    pub const fn set_rng_clk(&mut self, val: super::vals::Pscctl0SetRngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0SetFlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0SetFlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock set"]
    #[inline(always)]
    pub const fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0SetFlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0SetOtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0SetOtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock set"]
    #[inline(always)]
    pub const fn set_otp_clk(&mut self, val: super::vals::Pscctl0SetOtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0SetUsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0SetUsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock set"]
    #[inline(always)]
    pub const fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0SetUsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0SetUsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0SetUsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock set"]
    #[inline(always)]
    pub const fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0SetUsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0SetUsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0SetUsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock set"]
    #[inline(always)]
    pub const fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0SetUsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0SetUsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0SetUsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock set"]
    #[inline(always)]
    pub const fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0SetUsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0SetSctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0SetSctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock set"]
    #[inline(always)]
    pub const fn set_sct_clk(&mut self, val: super::vals::Pscctl0SetSctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("rom_ctl_128kb_clk", &self.rom_ctl_128kb_clk())
            .field("powerquad_clk", &self.powerquad_clk())
            .field("casper_clk", &self.casper_clk())
            .field("hashcrypt_clk", &self.hashcrypt_clk())
            .field("puf_clk", &self.puf_clk())
            .field("rng_clk", &self.rng_clk())
            .field("flexspi_otfad_clk", &self.flexspi_otfad_clk())
            .field("otp_clk", &self.otp_clk())
            .field("usbhs_phy_clk", &self.usbhs_phy_clk())
            .field("usbhs_device_clk", &self.usbhs_device_clk())
            .field("usbhs_host_clk", &self.usbhs_host_clk())
            .field("usbhs_sram_clk", &self.usbhs_sram_clk())
            .field("sct_clk", &self.sct_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl0Set {{ rom_ctl_128kb_clk: {:?}, powerquad_clk: {:?}, casper_clk: {:?}, hashcrypt_clk: {:?}, puf_clk: {:?}, rng_clk: {:?}, flexspi_otfad_clk: {:?}, otp_clk: {:?}, usbhs_phy_clk: {:?}, usbhs_device_clk: {:?}, usbhs_host_clk: {:?}, usbhs_sram_clk: {:?}, sct_clk: {:?} }}",
            self.rom_ctl_128kb_clk(),
            self.powerquad_clk(),
            self.casper_clk(),
            self.hashcrypt_clk(),
            self.puf_clk(),
            self.rng_clk(),
            self.flexspi_otfad_clk(),
            self.otp_clk(),
            self.usbhs_phy_clk(),
            self.usbhs_device_clk(),
            self.usbhs_host_clk(),
            self.usbhs_sram_clk(),
            self.sct_clk()
        )
    }
}
#[doc = "clock control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1(pub u32);
impl Pscctl1 {
    #[doc = "SDIO0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1Sdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1Sdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock control"]
    #[inline(always)]
    pub const fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1Sdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1Sdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1Sdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock control"]
    #[inline(always)]
    pub const fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1Sdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1Acmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1Acmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock control"]
    #[inline(always)]
    pub const fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1Acmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1Adc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1Adc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock control"]
    #[inline(always)]
    pub const fn set_adc0_clk(&mut self, val: super::vals::Pscctl1Adc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1Shsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1Shsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock control"]
    #[inline(always)]
    pub const fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1Shsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("sdio0_clk", &self.sdio0_clk())
            .field("sdio1_clk", &self.sdio1_clk())
            .field("acmp0_clk", &self.acmp0_clk())
            .field("adc0_clk", &self.adc0_clk())
            .field("shsgpio0_clk", &self.shsgpio0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1 {{ sdio0_clk: {:?}, sdio1_clk: {:?}, acmp0_clk: {:?}, adc0_clk: {:?}, shsgpio0_clk: {:?} }}",
            self.sdio0_clk(),
            self.sdio1_clk(),
            self.acmp0_clk(),
            self.adc0_clk(),
            self.shsgpio0_clk()
        )
    }
}
#[doc = "clock clear register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Clr(pub u32);
impl Pscctl1Clr {
    #[doc = "SDIO0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1ClrSdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1ClrSdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock clear"]
    #[inline(always)]
    pub const fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1ClrSdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1ClrSdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1ClrSdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock clear"]
    #[inline(always)]
    pub const fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1ClrSdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1ClrAcmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1ClrAcmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock clear"]
    #[inline(always)]
    pub const fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1ClrAcmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1ClrAdc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1ClrAdc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock clear"]
    #[inline(always)]
    pub const fn set_adc0_clk(&mut self, val: super::vals::Pscctl1ClrAdc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1ClrShsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1ClrShsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock clear"]
    #[inline(always)]
    pub const fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1ClrShsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("sdio0_clk", &self.sdio0_clk())
            .field("sdio1_clk", &self.sdio1_clk())
            .field("acmp0_clk", &self.acmp0_clk())
            .field("adc0_clk", &self.adc0_clk())
            .field("shsgpio0_clk", &self.shsgpio0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1Clr {{ sdio0_clk: {:?}, sdio1_clk: {:?}, acmp0_clk: {:?}, adc0_clk: {:?}, shsgpio0_clk: {:?} }}",
            self.sdio0_clk(),
            self.sdio1_clk(),
            self.acmp0_clk(),
            self.adc0_clk(),
            self.shsgpio0_clk()
        )
    }
}
#[doc = "clock set register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Set(pub u32);
impl Pscctl1Set {
    #[doc = "SDIO0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1SetSdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1SetSdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock set"]
    #[inline(always)]
    pub const fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1SetSdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1SetSdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1SetSdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock set"]
    #[inline(always)]
    pub const fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1SetSdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1SetAcmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1SetAcmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock set"]
    #[inline(always)]
    pub const fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1SetAcmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1SetAdc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1SetAdc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock set"]
    #[inline(always)]
    pub const fn set_adc0_clk(&mut self, val: super::vals::Pscctl1SetAdc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1SetShsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1SetShsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock set"]
    #[inline(always)]
    pub const fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1SetShsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
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
            .field("sdio0_clk", &self.sdio0_clk())
            .field("sdio1_clk", &self.sdio1_clk())
            .field("acmp0_clk", &self.acmp0_clk())
            .field("adc0_clk", &self.adc0_clk())
            .field("shsgpio0_clk", &self.shsgpio0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl1Set {{ sdio0_clk: {:?}, sdio1_clk: {:?}, acmp0_clk: {:?}, adc0_clk: {:?}, shsgpio0_clk: {:?} }}",
            self.sdio0_clk(),
            self.sdio1_clk(),
            self.acmp0_clk(),
            self.adc0_clk(),
            self.shsgpio0_clk()
        )
    }
}
#[doc = "clock control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2(pub u32);
impl Pscctl2 {
    #[doc = "utick clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2Utick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2Utick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock control"]
    #[inline(always)]
    pub const fn set_utick0_clk(&mut self, val: super::vals::Pscctl2Utick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock control"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2Wwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2Wwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock control"]
    #[inline(always)]
    pub const fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2Wwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("utick0_clk", &self.utick0_clk())
            .field("wwdt0_clk", &self.wwdt0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2 {{ utick0_clk: {:?}, wwdt0_clk: {:?} }}",
            self.utick0_clk(),
            self.wwdt0_clk()
        )
    }
}
#[doc = "clock clear register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Clr(pub u32);
impl Pscctl2Clr {
    #[doc = "utick clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2ClrUtick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2ClrUtick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock clear"]
    #[inline(always)]
    pub const fn set_utick0_clk(&mut self, val: super::vals::Pscctl2ClrUtick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock clear"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2ClrWwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2ClrWwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock clear"]
    #[inline(always)]
    pub const fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2ClrWwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("utick0_clk", &self.utick0_clk())
            .field("wwdt0_clk", &self.wwdt0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2Clr {{ utick0_clk: {:?}, wwdt0_clk: {:?} }}",
            self.utick0_clk(),
            self.wwdt0_clk()
        )
    }
}
#[doc = "clock set register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Set(pub u32);
impl Pscctl2Set {
    #[doc = "utick clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2SetUtick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2SetUtick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock set"]
    #[inline(always)]
    pub const fn set_utick0_clk(&mut self, val: super::vals::Pscctl2SetUtick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock set"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2SetWwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2SetWwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock set"]
    #[inline(always)]
    pub const fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2SetWwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("utick0_clk", &self.utick0_clk())
            .field("wwdt0_clk", &self.wwdt0_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pscctl2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pscctl2Set {{ utick0_clk: {:?}, wwdt0_clk: {:?} }}",
            self.utick0_clk(),
            self.wwdt0_clk()
        )
    }
}
#[doc = "SCT fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctfclkdiv(pub u32);
impl Sctfclkdiv {
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
impl Default for Sctfclkdiv {
    #[inline(always)]
    fn default() -> Sctfclkdiv {
        Sctfclkdiv(0)
    }
}
impl core::fmt::Debug for Sctfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctfclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "SCT FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctfclksel(pub u32);
impl Sctfclksel {
    #[doc = "SCT Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SctfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SctfclkselSel::from_bits(val as u8)
    }
    #[doc = "SCT Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SctfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sctfclksel {
    #[inline(always)]
    fn default() -> Sctfclksel {
        Sctfclksel(0)
    }
}
impl core::fmt::Debug for Sctfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sctfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SDIO0 FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio0fclkdiv(pub u32);
impl Sdio0fclkdiv {
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
impl Default for Sdio0fclkdiv {
    #[inline(always)]
    fn default() -> Sdio0fclkdiv {
        Sdio0fclkdiv(0)
    }
}
impl core::fmt::Debug for Sdio0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdio0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdio0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdio0fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "SDIO0 FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio0fclksel(pub u32);
impl Sdio0fclksel {
    #[doc = "SDIO0 Functional Clock Source Selection. ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sdio0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sdio0fclkselSel::from_bits(val as u8)
    }
    #[doc = "SDIO0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sdio0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sdio0fclksel {
    #[inline(always)]
    fn default() -> Sdio0fclksel {
        Sdio0fclksel(0)
    }
}
impl core::fmt::Debug for Sdio0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdio0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdio0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sdio0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SDIO1 FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio1fclkdiv(pub u32);
impl Sdio1fclkdiv {
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
impl Default for Sdio1fclkdiv {
    #[inline(always)]
    fn default() -> Sdio1fclkdiv {
        Sdio1fclkdiv(0)
    }
}
impl core::fmt::Debug for Sdio1fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdio1fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdio1fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdio1fclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "SDIO1 FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio1fclksel(pub u32);
impl Sdio1fclksel {
    #[doc = "SDIO1 Functional Clock Source Selection. ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sdio1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sdio1fclkselSel::from_bits(val as u8)
    }
    #[doc = "SDIO1 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Sdio1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sdio1fclksel {
    #[inline(always)]
    fn default() -> Sdio1fclksel {
        Sdio1fclksel(0)
    }
}
impl core::fmt::Debug for Sdio1fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdio1fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdio1fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sdio1fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "system cpu AHB clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscpuahbclkdiv(pub u32);
impl Syscpuahbclkdiv {
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
impl Default for Syscpuahbclkdiv {
    #[inline(always)]
    fn default() -> Syscpuahbclkdiv {
        Syscpuahbclkdiv(0)
    }
}
impl core::fmt::Debug for Syscpuahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syscpuahbclkdiv")
            .field("div", &self.div())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syscpuahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syscpuahbclkdiv {{ div: {=u8:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reqflag()
        )
    }
}
#[doc = "system oscillator bypass"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysoscbypass(pub u32);
impl Sysoscbypass {
    #[doc = "Extenal Clock Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SysoscbypassSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SysoscbypassSel::from_bits(val as u8)
    }
    #[doc = "Extenal Clock Source Selection."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SysoscbypassSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sysoscbypass {
    #[inline(always)]
    fn default() -> Sysoscbypass {
        Sysoscbypass(0)
    }
}
impl core::fmt::Debug for Sysoscbypass {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sysoscbypass")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sysoscbypass {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sysoscbypass {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "system oscillator control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysoscctl0(pub u32);
impl Sysoscctl0 {
    #[doc = "Enable signal for low power mode. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_enable(&self) -> super::vals::LpEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpEnable::from_bits(val as u8)
    }
    #[doc = "Enable signal for low power mode. . ."]
    #[inline(always)]
    pub const fn set_lp_enable(&mut self, val: super::vals::LpEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable signal for external bypass clock. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass_enable(&self) -> super::vals::BypassEnable {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BypassEnable::from_bits(val as u8)
    }
    #[doc = "Enable signal for external bypass clock. . ."]
    #[inline(always)]
    pub const fn set_bypass_enable(&mut self, val: super::vals::BypassEnable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Sysoscctl0 {
    #[inline(always)]
    fn default() -> Sysoscctl0 {
        Sysoscctl0(0)
    }
}
impl core::fmt::Debug for Sysoscctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sysoscctl0")
            .field("lp_enable", &self.lp_enable())
            .field("bypass_enable", &self.bypass_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sysoscctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sysoscctl0 {{ lp_enable: {:?}, bypass_enable: {:?} }}",
            self.lp_enable(),
            self.bypass_enable()
        )
    }
}
#[doc = "system pll0 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0clksel(pub u32);
impl Syspll0clksel {
    #[doc = "System PLL Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Syspll0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Syspll0clkselSel::from_bits(val as u8)
    }
    #[doc = "System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Syspll0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Syspll0clksel {
    #[inline(always)]
    fn default() -> Syspll0clksel {
        Syspll0clksel(0)
    }
}
impl core::fmt::Debug for Syspll0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syspll0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syspll0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "system pll0 control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0ctl0(pub u32);
impl Syspll0ctl0 {
    #[doc = "SYSPLL0 BYPASS Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bypass::from_bits(val as u8)
    }
    #[doc = "SYSPLL0 BYPASS Mode"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SYSPLL0 Reset:"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Reset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Reset::from_bits(val as u8)
    }
    #[doc = "SYSPLL0 Reset:"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: super::vals::Reset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Hold Ring Off Control: This bit is used to avoid multi wave within the VCO."]
    #[must_use]
    #[inline(always)]
    pub const fn holdringoff_ena(&self) -> super::vals::HoldringoffEna {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::HoldringoffEna::from_bits(val as u8)
    }
    #[doc = "Hold Ring Off Control: This bit is used to avoid multi wave within the VCO."]
    #[inline(always)]
    pub const fn set_holdringoff_ena(&mut self, val: super::vals::HoldringoffEna) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Multiplication Factor for FSYSPLL0_OUTPUT:"]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> super::vals::Mult {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Mult::from_bits(val as u8)
    }
    #[doc = "Multiplication Factor for FSYSPLL0_OUTPUT:"]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: super::vals::Mult) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for Syspll0ctl0 {
    #[inline(always)]
    fn default() -> Syspll0ctl0 {
        Syspll0ctl0(0)
    }
}
impl core::fmt::Debug for Syspll0ctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0ctl0")
            .field("bypass", &self.bypass())
            .field("reset", &self.reset())
            .field("holdringoff_ena", &self.holdringoff_ena())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syspll0ctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syspll0ctl0 {{ bypass: {:?}, reset: {:?}, holdringoff_ena: {:?}, mult: {:?} }}",
            self.bypass(),
            self.reset(),
            self.holdringoff_ena(),
            self.mult()
        )
    }
}
#[doc = "system pll0 denom"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0denom(pub u32);
impl Syspll0denom {
    #[doc = "This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Syspll0denom {
    #[inline(always)]
    fn default() -> Syspll0denom {
        Syspll0denom(0)
    }
}
impl core::fmt::Debug for Syspll0denom {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0denom")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syspll0denom {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syspll0denom {{ denom: {=u32:?} }}", self.denom())
    }
}
#[doc = "system pll0 lock time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0locktimediv2(pub u32);
impl Syspll0locktimediv2 {
    #[doc = "SYSPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[must_use]
    #[inline(always)]
    pub const fn locktimediv2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SYSPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    pub const fn set_locktimediv2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Syspll0locktimediv2 {
    #[inline(always)]
    fn default() -> Syspll0locktimediv2 {
        Syspll0locktimediv2(0)
    }
}
impl core::fmt::Debug for Syspll0locktimediv2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0locktimediv2")
            .field("locktimediv2", &self.locktimediv2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syspll0locktimediv2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syspll0locktimediv2 {{ locktimediv2: {=u16:?} }}",
            self.locktimediv2()
        )
    }
}
#[doc = "system pll0 number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0num(pub u32);
impl Syspll0num {
    #[doc = "This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Syspll0num {
    #[inline(always)]
    fn default() -> Syspll0num {
        Syspll0num(0)
    }
}
impl core::fmt::Debug for Syspll0num {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0num")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syspll0num {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Syspll0num {{ num: {=u32:?} }}", self.num())
    }
}
#[doc = "sys pll0 PFD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0pfd(pub u32);
impl Syspll0pfd {
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
    #[doc = "PFD0 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkrdy(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
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
    #[doc = "PFD1 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd1_clkrdy(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
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
    #[doc = "PFD2 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd2_clkrdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
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
    #[doc = "PFD3 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd3_clkrdy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
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
impl Default for Syspll0pfd {
    #[inline(always)]
    fn default() -> Syspll0pfd {
        Syspll0pfd(0)
    }
}
impl core::fmt::Debug for Syspll0pfd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syspll0pfd")
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
impl defmt::Format for Syspll0pfd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syspll0pfd {{ pfd0: {=u8:?}, pfd0_clkrdy: {=bool:?}, pfd0_clkgate: {=bool:?}, pfd1: {=u8:?}, pfd1_clkrdy: {=bool:?}, pfd1_clkgate: {=bool:?}, pfd2: {=u8:?}, pfd2_clkrdy: {=bool:?}, pfd2_clkgate: {=bool:?}, pfd3: {=u8:?}, pfd3_clkrdy: {=bool:?}, pfd3_clkgate: {=bool:?} }}",
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
#[doc = "system tick fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickfclkdiv(pub u32);
impl Systickfclkdiv {
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
impl Default for Systickfclkdiv {
    #[inline(always)]
    fn default() -> Systickfclkdiv {
        Systickfclkdiv(0)
    }
}
impl core::fmt::Debug for Systickfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickfclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "system tick fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickfclksel(pub u32);
impl Systickfclksel {
    #[doc = "SYSTICK Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SystickfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SystickfclkselSel::from_bits(val as u8)
    }
    #[doc = "SYSTICK Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::SystickfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickfclksel {
    #[inline(always)]
    fn default() -> Systickfclksel {
        Systickfclksel(0)
    }
}
impl core::fmt::Debug for Systickfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "USBHS Fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsfclkdiv(pub u32);
impl Usbhsfclkdiv {
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
impl Default for Usbhsfclkdiv {
    #[inline(always)]
    fn default() -> Usbhsfclkdiv {
        Usbhsfclkdiv(0)
    }
}
impl core::fmt::Debug for Usbhsfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbhsfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbhsfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbhsfclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "USBHS Fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsfclksel(pub u32);
impl Usbhsfclksel {
    #[doc = "USB HS Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::UsbhsfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::UsbhsfclkselSel::from_bits(val as u8)
    }
    #[doc = "USB HS Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::UsbhsfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Usbhsfclksel {
    #[inline(always)]
    fn default() -> Usbhsfclksel {
        Usbhsfclksel(0)
    }
}
impl core::fmt::Debug for Usbhsfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbhsfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbhsfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbhsfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "UTICK fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickfclksel(pub u32);
impl Utickfclksel {
    #[doc = "uTICK Functional Clock Source Selection. ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::UtickfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::UtickfclkselSel::from_bits(val as u8)
    }
    #[doc = "uTICK Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::UtickfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Utickfclksel {
    #[inline(always)]
    fn default() -> Utickfclksel {
        Utickfclksel(0)
    }
}
impl core::fmt::Debug for Utickfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utickfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utickfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Utickfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "32k wake clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeclk32khzdiv(pub u32);
impl Wakeclk32khzdiv {
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
}
impl Default for Wakeclk32khzdiv {
    #[inline(always)]
    fn default() -> Wakeclk32khzdiv {
        Wakeclk32khzdiv(0)
    }
}
impl core::fmt::Debug for Wakeclk32khzdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeclk32khzdiv")
            .field("halt", &self.halt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeclk32khzdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeclk32khzdiv {{ halt: {=bool:?} }}", self.halt())
    }
}
#[doc = "32k wake clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeclk32khzsel(pub u32);
impl Wakeclk32khzsel {
    #[doc = "32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wakeclk32khzselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wakeclk32khzselSel::from_bits(val as u8)
    }
    #[doc = "32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Wakeclk32khzselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Wakeclk32khzsel {
    #[inline(always)]
    fn default() -> Wakeclk32khzsel {
        Wakeclk32khzsel(0)
    }
}
impl core::fmt::Debug for Wakeclk32khzsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wakeclk32khzsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wakeclk32khzsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wakeclk32khzsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "wdt clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0fclksel(pub u32);
impl Wdt0fclksel {
    #[doc = "WDT0 Functional Clock Source Selection. ."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wdt0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wdt0fclkselSel::from_bits(val as u8)
    }
    #[doc = "WDT0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: super::vals::Wdt0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Wdt0fclksel {
    #[inline(always)]
    fn default() -> Wdt0fclksel {
        Wdt0fclksel(0)
    }
}
impl core::fmt::Debug for Wdt0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdt0fclksel {{ sel: {:?} }}", self.sel())
    }
}
