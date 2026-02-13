#[doc = "ADC Trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcTrig(pub u32);
impl AdcTrig {
    #[doc = "ADC0 trigger inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::AdcTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::AdcTrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::AdcTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for AdcTrig {
    #[inline(always)]
    fn default() -> AdcTrig {
        AdcTrig(0)
    }
}
impl core::fmt::Debug for AdcTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "AOI0 trigger input connections 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AoiInput(pub u32);
impl AoiInput {
    #[doc = "AOI0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::AoiInputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::AoiInputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::AoiInputInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for AoiInput {
    #[inline(always)]
    fn default() -> AoiInput {
        AoiInput(0)
    }
}
impl core::fmt::Debug for AoiInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AoiInput")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AoiInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AoiInput {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "CMP0 input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpTrig(pub u32);
impl CmpTrig {
    #[doc = "CMP0 input trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::CmpTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::CmpTrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::CmpTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for CmpTrig {
    #[inline(always)]
    fn default() -> CmpTrig {
        CmpTrig(0)
    }
}
impl core::fmt::Debug for CmpTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap(pub u32);
impl Ctimer0cap {
    #[doc = "Input number for CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap {
    #[inline(always)]
    fn default() -> Ctimer0cap {
        Ctimer0cap(0)
    }
}
impl core::fmt::Debug for Ctimer0cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap(pub u32);
impl Ctimer1cap {
    #[doc = "Input number for CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap {
    #[inline(always)]
    fn default() -> Ctimer1cap {
        Ctimer1cap(0)
    }
}
impl core::fmt::Debug for Ctimer1cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap(pub u32);
impl Ctimer2cap {
    #[doc = "Input number for CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap {
    #[inline(always)]
    fn default() -> Ctimer2cap {
        Ctimer2cap(0)
    }
}
impl core::fmt::Debug for Ctimer2cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap(pub u32);
impl Ctimer3cap {
    #[doc = "Input number for CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap {
    #[inline(always)]
    fn default() -> Ctimer3cap {
        Ctimer3cap(0)
    }
}
impl core::fmt::Debug for Ctimer3cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture select register for CTIMER inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap(pub u32);
impl Ctimer4cap {
    #[doc = "Input number for CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4capInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap {
    #[inline(always)]
    fn default() -> Ctimer4cap {
        Ctimer4cap(0)
    }
}
impl core::fmt::Debug for Ctimer4cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DAC0 Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacTrig(pub u32);
impl DacTrig {
    #[doc = "DAC0 trigger input"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::DacTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::DacTrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::DacTrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for DacTrig {
    #[inline(always)]
    fn default() -> DacTrig {
        DacTrig(0)
    }
}
impl core::fmt::Debug for DacTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacTrig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacTrig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "EXT trigger connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig(pub u32);
impl ExtTrig {
    #[doc = "EXT trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::ExtTrigInp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::ExtTrigInp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::ExtTrigInp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for ExtTrig {
    #[inline(always)]
    fn default() -> ExtTrig {
        ExtTrig(0)
    }
}
impl core::fmt::Debug for ExtTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtTrig").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 Fault Input Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm(pub u32);
impl FlexPwm {
    #[doc = "FAULT input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwmTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwmTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwmTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm {
    #[inline(always)]
    fn default() -> FlexPwm {
        FlexPwm(0)
    }
}
impl core::fmt::Debug for FlexPwm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "FlexIO Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioTrig(pub u32);
impl FlexioTrig {
    #[doc = "Input number for FlexIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FlexioTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::FlexioTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for FlexIO0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FlexioTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FlexioTrig {
    #[inline(always)]
    fn default() -> FlexioTrig {
        FlexioTrig(0)
    }
}
impl core::fmt::Debug for FlexioTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexioTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexioTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexioTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasRefInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FreqmeasRef {
    #[inline(always)]
    fn default() -> FreqmeasRef {
        FreqmeasRef(0)
    }
}
impl core::fmt::Debug for FreqmeasRef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasRef")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasRef {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Selection for frequency measurement reference clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTar(pub u32);
impl FreqmeasTar {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasTarInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for FreqmeasTar {
    #[inline(always)]
    fn default() -> FreqmeasTar {
        FreqmeasTar(0)
    }
}
impl core::fmt::Debug for FreqmeasTar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasTar")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasTar {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPI2C0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2cTrig(pub u32);
impl Lpi2cTrig {
    #[doc = "LPI2C0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpi2cTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpi2cTrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpi2cTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2cTrig {
    #[inline(always)]
    fn default() -> Lpi2cTrig {
        Lpi2cTrig(0)
    }
}
impl core::fmt::Debug for Lpi2cTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2cTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2cTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2cTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPSPI0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiTrig(pub u32);
impl LpspiTrig {
    #[doc = "LPSPI0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::LpspiTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LpspiTrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::LpspiTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LpspiTrig {
    #[inline(always)]
    fn default() -> LpspiTrig {
        LpspiTrig(0)
    }
}
impl core::fmt::Debug for LpspiTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpspiTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpspiTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpspiTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART0 trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart(pub u32);
impl Lpuart {
    #[doc = "LPUART0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::LpuartInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::LpuartInp::from_bits(val as u8)
    }
    #[doc = "LPUART0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::LpuartInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart {
    #[inline(always)]
    fn default() -> Lpuart {
        Lpuart(0)
    }
}
impl core::fmt::Debug for Lpuart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 external clock trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0ExtClk(pub u32);
impl Pwm0ExtClk {
    #[doc = "Trigger input connections for PWM"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm0ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwm0ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Pwm0ExtClkTrigin) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pwm0ExtClk {
    #[inline(always)]
    fn default() -> Pwm0ExtClk {
        Pwm0ExtClk(0)
    }
}
impl core::fmt::Debug for Pwm0ExtClk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0ExtClk")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0ExtClk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwm0ExtClk {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 external clock trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1ExtClk(pub u32);
impl Pwm1ExtClk {
    #[doc = "Trigger input connections for PWM"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm1ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwm1ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Pwm1ExtClkTrigin) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pwm1ExtClk {
    #[inline(always)]
    fn default() -> Pwm1ExtClk {
        Pwm1ExtClk(0)
    }
}
impl core::fmt::Debug for Pwm1ExtClk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1ExtClk")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1ExtClk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pwm1ExtClk {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Home(pub u32);
impl Qdc0Home {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcHomeInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Home {
    #[inline(always)]
    fn default() -> Qdc0Home {
        Qdc0Home(0)
    }
}
impl core::fmt::Debug for Qdc0Home {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Home")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Home {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Home {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap1(pub u32);
impl Qdc0Icap1 {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Icap1 {
    #[inline(always)]
    fn default() -> Qdc0Icap1 {
        Qdc0Icap1(0)
    }
}
impl core::fmt::Debug for Qdc0Icap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Icap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Icap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Icap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap2(pub u32);
impl Qdc0Icap2 {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Icap2 {
    #[inline(always)]
    fn default() -> Qdc0Icap2 {
        Qdc0Icap2(0)
    }
}
impl core::fmt::Debug for Qdc0Icap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Icap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Icap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Icap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap3(pub u32);
impl Qdc0Icap3 {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Icap3 {
    #[inline(always)]
    fn default() -> Qdc0Icap3 {
        Qdc0Icap3(0)
    }
}
impl core::fmt::Debug for Qdc0Icap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Icap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Icap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Icap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Index(pub u32);
impl Qdc0Index {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIndexInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Index {
    #[inline(always)]
    fn default() -> Qdc0Index {
        Qdc0Index(0)
    }
}
impl core::fmt::Debug for Qdc0Index {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Index")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Index {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Index {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Phasea(pub u32);
impl Qdc0Phasea {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0PhaseaInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Phasea {
    #[inline(always)]
    fn default() -> Qdc0Phasea {
        Qdc0Phasea(0)
    }
}
impl core::fmt::Debug for Qdc0Phasea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Phasea")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Phasea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Phasea {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Phaseb(pub u32);
impl Qdc0Phaseb {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0PhasebInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Phaseb {
    #[inline(always)]
    fn default() -> Qdc0Phaseb {
        Qdc0Phaseb(0)
    }
}
impl core::fmt::Debug for Qdc0Phaseb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Phaseb")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Phaseb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Phaseb {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Home(pub u32);
impl Qdc1Home {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcHomeInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Home {
    #[inline(always)]
    fn default() -> Qdc1Home {
        Qdc1Home(0)
    }
}
impl core::fmt::Debug for Qdc1Home {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Home")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Home {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Home {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap1(pub u32);
impl Qdc1Icap1 {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Icap1 {
    #[inline(always)]
    fn default() -> Qdc1Icap1 {
        Qdc1Icap1(0)
    }
}
impl core::fmt::Debug for Qdc1Icap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Icap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Icap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Icap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap2(pub u32);
impl Qdc1Icap2 {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Icap2 {
    #[inline(always)]
    fn default() -> Qdc1Icap2 {
        Qdc1Icap2(0)
    }
}
impl core::fmt::Debug for Qdc1Icap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Icap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Icap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Icap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap3(pub u32);
impl Qdc1Icap3 {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIcapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Icap3 {
    #[inline(always)]
    fn default() -> Qdc1Icap3 {
        Qdc1Icap3(0)
    }
}
impl core::fmt::Debug for Qdc1Icap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Icap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Icap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Icap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Index(pub u32);
impl Qdc1Index {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIndexInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Index {
    #[inline(always)]
    fn default() -> Qdc1Index {
        Qdc1Index(0)
    }
}
impl core::fmt::Debug for Qdc1Index {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Index")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Index {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Index {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Phasea(pub u32);
impl Qdc1Phasea {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1PhaseaInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Phasea {
    #[inline(always)]
    fn default() -> Qdc1Phasea {
        Qdc1Phasea(0)
    }
}
impl core::fmt::Debug for Qdc1Phasea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Phasea")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Phasea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Phasea {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Phaseb(pub u32);
impl Qdc1Phaseb {
    #[doc = "QDC1 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1PhasebInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Phaseb {
    #[inline(always)]
    fn default() -> Qdc1Phaseb {
        Qdc1Phaseb(0)
    }
}
impl core::fmt::Debug for Qdc1Phaseb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Phaseb")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Phaseb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Phaseb {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcTrig(pub u32);
impl QdcTrig {
    #[doc = "QDC0 input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for QdcTrig {
    #[inline(always)]
    fn default() -> QdcTrig {
        QdcTrig(0)
    }
}
impl core::fmt::Debug for QdcTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcTrig").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "SmartDMA Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaTrig(pub u32);
impl SmartDmaTrig {
    #[doc = "Input number for SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::SmartDmaTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::SmartDmaTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for SmartDMA."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::SmartDmaTrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for SmartDmaTrig {
    #[inline(always)]
    fn default() -> SmartDmaTrig {
        SmartDmaTrig(0)
    }
}
impl core::fmt::Debug for SmartDmaTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmartDmaTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER0"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer0trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer0trig {
    #[inline(always)]
    fn default() -> Timer0trig {
        Timer0trig(0)
    }
}
impl core::fmt::Debug for Timer0trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer0trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER1"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer1trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer1trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer1trig {
    #[inline(always)]
    fn default() -> Timer1trig {
        Timer1trig(0)
    }
}
impl core::fmt::Debug for Timer1trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer1trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER2 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER2"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer2trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer2trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer2trig {
    #[inline(always)]
    fn default() -> Timer2trig {
        Timer2trig(0)
    }
}
impl core::fmt::Debug for Timer2trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer2trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER3"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer3trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer3trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer3trig {
    #[inline(always)]
    fn default() -> Timer3trig {
        Timer3trig(0)
    }
}
impl core::fmt::Debug for Timer3trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer3trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer3trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER4"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer4trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Timer4trigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Timer4trig {
    #[inline(always)]
    fn default() -> Timer4trig {
        Timer4trig(0)
    }
}
impl core::fmt::Debug for Timer4trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer4trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer4trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "TRIGFIL control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigfil(pub u32);
impl Trigfil {
    #[doc = "Input Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Input Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Trigfil {
    #[inline(always)]
    fn default() -> Trigfil {
        Trigfil(0)
    }
}
impl core::fmt::Debug for Trigfil {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trigfil")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trigfil {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trigfil {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Trigger filter prescaller"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigfilPrsc(pub u32);
impl TrigfilPrsc {
    #[doc = "Filter Prescaller Value"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_scale_val(&self) -> super::vals::FiltScaleVal {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FiltScaleVal::from_bits(val as u8)
    }
    #[doc = "Filter Prescaller Value"]
    #[inline(always)]
    pub const fn set_filt_scale_val(&mut self, val: super::vals::FiltScaleVal) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Enable trigger filter prescaller"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_scale_en(&self) -> super::vals::FiltScaleEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FiltScaleEn::from_bits(val as u8)
    }
    #[doc = "Enable trigger filter prescaller"]
    #[inline(always)]
    pub const fn set_filt_scale_en(&mut self, val: super::vals::FiltScaleEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TrigfilPrsc {
    #[inline(always)]
    fn default() -> TrigfilPrsc {
        TrigfilPrsc(0)
    }
}
impl core::fmt::Debug for TrigfilPrsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigfilPrsc")
            .field("filt_scale_val", &self.filt_scale_val())
            .field("filt_scale_en", &self.filt_scale_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigfilPrsc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrigfilPrsc {{ filt_scale_val: {:?}, filt_scale_en: {:?} }}",
            self.filt_scale_val(),
            self.filt_scale_en()
        )
    }
}
#[doc = "Trigger filter stat"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigfilStat(pub u32);
impl TrigfilStat {
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in0_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in0_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in1_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in1_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in2_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in2_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in3_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in3_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in4_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in4_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in5_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in5_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in6_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in6_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in7_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in7_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in8_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in8_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in9_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in9_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in10_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in10_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TRIG_IN value"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_in11_val(&self) -> super::vals::TrigInVal {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::TrigInVal::from_bits(val as u8)
    }
    #[doc = "TRIG_IN value"]
    #[inline(always)]
    pub const fn set_trig_in11_val(&mut self, val: super::vals::TrigInVal) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for TrigfilStat {
    #[inline(always)]
    fn default() -> TrigfilStat {
        TrigfilStat(0)
    }
}
impl core::fmt::Debug for TrigfilStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigfilStat")
            .field("trig_in0_val", &self.trig_in0_val())
            .field("trig_in1_val", &self.trig_in1_val())
            .field("trig_in2_val", &self.trig_in2_val())
            .field("trig_in3_val", &self.trig_in3_val())
            .field("trig_in4_val", &self.trig_in4_val())
            .field("trig_in5_val", &self.trig_in5_val())
            .field("trig_in6_val", &self.trig_in6_val())
            .field("trig_in7_val", &self.trig_in7_val())
            .field("trig_in8_val", &self.trig_in8_val())
            .field("trig_in9_val", &self.trig_in9_val())
            .field("trig_in10_val", &self.trig_in10_val())
            .field("trig_in11_val", &self.trig_in11_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigfilStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrigfilStat {{ trig_in0_val: {:?}, trig_in1_val: {:?}, trig_in2_val: {:?}, trig_in3_val: {:?}, trig_in4_val: {:?}, trig_in5_val: {:?}, trig_in6_val: {:?}, trig_in7_val: {:?}, trig_in8_val: {:?}, trig_in9_val: {:?}, trig_in10_val: {:?}, trig_in11_val: {:?} }}",
            self.trig_in0_val(),
            self.trig_in1_val(),
            self.trig_in2_val(),
            self.trig_in3_val(),
            self.trig_in4_val(),
            self.trig_in5_val(),
            self.trig_in6_val(),
            self.trig_in7_val(),
            self.trig_in8_val(),
            self.trig_in9_val(),
            self.trig_in10_val(),
            self.trig_in11_val()
        )
    }
}
#[doc = "USB-FS trigger input connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsTrig(pub u32);
impl UsbfsTrig {
    #[doc = "USB-FS trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::UsbfsTrigInp {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::UsbfsTrigInp::from_bits(val as u8)
    }
    #[doc = "USB-FS trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::UsbfsTrigInp) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for UsbfsTrig {
    #[inline(always)]
    fn default() -> UsbfsTrig {
        UsbfsTrig(0)
    }
}
impl core::fmt::Debug for UsbfsTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbfsTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbfsTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbfsTrig {{ inp: {:?} }}", self.inp())
    }
}
