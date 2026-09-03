#[doc = "ADC Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0Trig(pub u32);
impl Adc0Trig {
    #[doc = "ADC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Adc0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Adc0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Adc0TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Adc0Trig {
    #[inline(always)]
    fn default() -> Adc0Trig {
        Adc0Trig(0)
    }
}
impl core::fmt::Debug for Adc0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc0Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "ADC Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1Trig(pub u32);
impl Adc1Trig {
    #[doc = "ADC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Adc1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Adc1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Adc1TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Adc1Trig {
    #[inline(always)]
    fn default() -> Adc1Trig {
        Adc1Trig(0)
    }
}
impl core::fmt::Debug for Adc1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc1Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "AOI0 trigger input connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi0Input(pub u32);
impl Aoi0Input {
    #[doc = "AOI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Aoi0InputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Aoi0InputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Aoi0InputInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Aoi0Input {
    #[inline(always)]
    fn default() -> Aoi0Input {
        Aoi0Input(0)
    }
}
impl core::fmt::Debug for Aoi0Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aoi0Input")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aoi0Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aoi0Input {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "AOI1 trigger input connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi1Input(pub u32);
impl Aoi1Input {
    #[doc = "AOI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Aoi1InputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Aoi1InputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Aoi1InputInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Aoi1Input {
    #[inline(always)]
    fn default() -> Aoi1Input {
        Aoi1Input(0)
    }
}
impl core::fmt::Debug for Aoi1Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aoi1Input")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aoi1Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Aoi1Input {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "CMP0 input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0Trig(pub u32);
impl Cmp0Trig {
    #[doc = "CMP0 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Cmp0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmp0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Cmp0TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Cmp0Trig {
    #[inline(always)]
    fn default() -> Cmp0Trig {
        Cmp0Trig(0)
    }
}
impl core::fmt::Debug for Cmp0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp0Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "CMP1 input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1Trig(pub u32);
impl Cmp1Trig {
    #[doc = "CMP0 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Cmp1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmp1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Cmp1TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Cmp1Trig {
    #[inline(always)]
    fn default() -> Cmp1Trig {
        Cmp1Trig(0)
    }
}
impl core::fmt::Debug for Cmp1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp1Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap(pub u32);
impl Ctimer0cap {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
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
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap(pub u32);
impl Ctimer1cap {
    #[doc = "Input number for CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
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
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap(pub u32);
impl Ctimer2cap {
    #[doc = "Input number for CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
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
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap(pub u32);
impl Ctimer3cap {
    #[doc = "Input number for CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
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
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap(pub u32);
impl Ctimer4cap {
    #[doc = "Input number for CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
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
#[doc = "This register selects the DAC0 trigger inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac0Trig(pub u32);
impl Dac0Trig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Dac0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dac0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Dac0TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Dac0Trig {
    #[inline(always)]
    fn default() -> Dac0Trig {
        Dac0Trig(0)
    }
}
impl core::fmt::Debug for Dac0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac0Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac0Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "EXT trigger connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig0(pub u32);
impl ExtTrig0 {
    #[doc = "EXT trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::ExtTrig0Inp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::ExtTrig0Inp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::ExtTrig0Inp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for ExtTrig0 {
    #[inline(always)]
    fn default() -> ExtTrig0 {
        ExtTrig0(0)
    }
}
impl core::fmt::Debug for ExtTrig0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ExtTrig0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ExtTrig0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ExtTrig0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Fault(pub u32);
impl FlexPwm0Fault {
    #[doc = "FAULT input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0FaultTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Fault {
    #[inline(always)]
    fn default() -> FlexPwm0Fault {
        FlexPwm0Fault(0)
    }
}
impl core::fmt::Debug for FlexPwm0Fault {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Fault")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Fault {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Fault {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Force(pub u32);
impl FlexPwm0Force {
    #[doc = "Trigger input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0ForceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0ForceTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0ForceTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Force {
    #[inline(always)]
    fn default() -> FlexPwm0Force {
        FlexPwm0Force(0)
    }
}
impl core::fmt::Debug for FlexPwm0Force {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Force")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Force {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Force {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm0Exta0(pub u32);
impl FlexPwm0Sm0Exta0 {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm0Exta0Trigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm0Exta0Trigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm0Exta0Trigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm0Exta0 {
    #[inline(always)]
    fn default() -> FlexPwm0Sm0Exta0 {
        FlexPwm0Sm0Exta0(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm0Exta0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm0Exta0")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm0Exta0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm0Exta0 {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm0Extsync(pub u32);
impl FlexPwm0Sm0Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm0ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm0ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm0ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm0Extsync {
    #[inline(always)]
    fn default() -> FlexPwm0Sm0Extsync {
        FlexPwm0Sm0Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm0Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm0Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm0Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm0Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm1Exta(pub u32);
impl FlexPwm0Sm1Exta {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm1ExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm1ExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm1ExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm1Exta {
    #[inline(always)]
    fn default() -> FlexPwm0Sm1Exta {
        FlexPwm0Sm1Exta(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm1Exta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm1Exta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm1Exta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm1Exta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm1Extsync(pub u32);
impl FlexPwm0Sm1Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm1ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm1ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm1ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm1Extsync {
    #[inline(always)]
    fn default() -> FlexPwm0Sm1Extsync {
        FlexPwm0Sm1Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm1Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm1Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm1Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm1Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm2Exta(pub u32);
impl FlexPwm0Sm2Exta {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm2ExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm2ExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm2ExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm2Exta {
    #[inline(always)]
    fn default() -> FlexPwm0Sm2Exta {
        FlexPwm0Sm2Exta(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm2Exta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm2Exta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm2Exta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm2Exta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Sm2Extsync(pub u32);
impl FlexPwm0Sm2Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0Sm2ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0Sm2ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0Sm2ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Sm2Extsync {
    #[inline(always)]
    fn default() -> FlexPwm0Sm2Extsync {
        FlexPwm0Sm2Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm0Sm2Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Sm2Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Sm2Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Sm2Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Fault(pub u32);
impl FlexPwm1Fault {
    #[doc = "FAULT input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1FaultTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Fault {
    #[inline(always)]
    fn default() -> FlexPwm1Fault {
        FlexPwm1Fault(0)
    }
}
impl core::fmt::Debug for FlexPwm1Fault {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Fault")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Fault {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Fault {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Force(pub u32);
impl FlexPwm1Force {
    #[doc = "Trigger input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1ForceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1ForceTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1ForceTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Force {
    #[inline(always)]
    fn default() -> FlexPwm1Force {
        FlexPwm1Force(0)
    }
}
impl core::fmt::Debug for FlexPwm1Force {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Force")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Force {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Force {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm0Exta0(pub u32);
impl FlexPwm1Sm0Exta0 {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm0Exta0Trigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm0Exta0Trigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm0Exta0Trigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm0Exta0 {
    #[inline(always)]
    fn default() -> FlexPwm1Sm0Exta0 {
        FlexPwm1Sm0Exta0(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm0Exta0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm0Exta0")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm0Exta0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm0Exta0 {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm0Extsync(pub u32);
impl FlexPwm1Sm0Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm0ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm0ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm0ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm0Extsync {
    #[inline(always)]
    fn default() -> FlexPwm1Sm0Extsync {
        FlexPwm1Sm0Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm0Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm0Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm0Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm0Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm1Exta(pub u32);
impl FlexPwm1Sm1Exta {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm1ExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm1ExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm1ExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm1Exta {
    #[inline(always)]
    fn default() -> FlexPwm1Sm1Exta {
        FlexPwm1Sm1Exta(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm1Exta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm1Exta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm1Exta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm1Exta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm1Extsync(pub u32);
impl FlexPwm1Sm1Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm1ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm1ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm1ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm1Extsync {
    #[inline(always)]
    fn default() -> FlexPwm1Sm1Extsync {
        FlexPwm1Sm1Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm1Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm1Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm1Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm1Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm2Exta(pub u32);
impl FlexPwm1Sm2Exta {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm2ExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm2ExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm2ExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm2Exta {
    #[inline(always)]
    fn default() -> FlexPwm1Sm2Exta {
        FlexPwm1Sm2Exta(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm2Exta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm2Exta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm2Exta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm2Exta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 input trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Sm2Extsync(pub u32);
impl FlexPwm1Sm2Extsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1Sm2ExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1Sm2ExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1Sm2ExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Sm2Extsync {
    #[inline(always)]
    fn default() -> FlexPwm1Sm2Extsync {
        FlexPwm1Sm2Extsync(0)
    }
}
impl core::fmt::Debug for FlexPwm1Sm2Extsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Sm2Extsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Sm2Extsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Sm2Extsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "FlexIO Trigger Input Connections."]
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
#[doc = "Selection for frequency measurement reference clock."]
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
#[doc = "Selection for frequency measurement reference clock."]
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
#[doc = "LPI2C0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c0Trig(pub u32);
impl Lpi2c0Trig {
    #[doc = "LPI2C0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpi2c0TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpi2c0TrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpi2c0TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2c0Trig {
    #[inline(always)]
    fn default() -> Lpi2c0Trig {
        Lpi2c0Trig(0)
    }
}
impl core::fmt::Debug for Lpi2c0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c0Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c0Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPI2C1 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1Trig(pub u32);
impl Lpi2c1Trig {
    #[doc = "LPI2C1 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpi2c1TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpi2c1TrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C1 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpi2c1TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2c1Trig {
    #[inline(always)]
    fn default() -> Lpi2c1Trig {
        Lpi2c1Trig(0)
    }
}
impl core::fmt::Debug for Lpi2c1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPI2C2 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2Trig(pub u32);
impl Lpi2c2Trig {
    #[doc = "LPI2C2 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpi2c2TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpi2c2TrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C2 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpi2c2TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpi2c2Trig {
    #[inline(always)]
    fn default() -> Lpi2c2Trig {
        Lpi2c2Trig(0)
    }
}
impl core::fmt::Debug for Lpi2c2Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c2Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPSPI0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi0Trig(pub u32);
impl Lpspi0Trig {
    #[doc = "LPSPI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpspi0TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpspi0TrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpspi0TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpspi0Trig {
    #[inline(always)]
    fn default() -> Lpspi0Trig {
        Lpspi0Trig(0)
    }
}
impl core::fmt::Debug for Lpspi0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi0Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi0Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPSPI1 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1Trig(pub u32);
impl Lpspi1Trig {
    #[doc = "LPSPI1 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpspi1TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpspi1TrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI1 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpspi1TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpspi1Trig {
    #[inline(always)]
    fn default() -> Lpspi1Trig {
        Lpspi1Trig(0)
    }
}
impl core::fmt::Debug for Lpspi1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart0(pub u32);
impl Lpuart0 {
    #[doc = "LPUART0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpuart0Inp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpuart0Inp::from_bits(val as u8)
    }
    #[doc = "LPUART0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpuart0Inp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart0 {
    #[inline(always)]
    fn default() -> Lpuart0 {
        Lpuart0(0)
    }
}
impl core::fmt::Debug for Lpuart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart0").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART1 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1(pub u32);
impl Lpuart1 {
    #[doc = "LPUART1 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpuart1Inp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpuart1Inp::from_bits(val as u8)
    }
    #[doc = "LPUART1 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpuart1Inp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart1 {
    #[inline(always)]
    fn default() -> Lpuart1 {
        Lpuart1(0)
    }
}
impl core::fmt::Debug for Lpuart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART2 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2(pub u32);
impl Lpuart2 {
    #[doc = "LPUART2 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpuart2Inp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpuart2Inp::from_bits(val as u8)
    }
    #[doc = "LPUART2 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpuart2Inp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart2 {
    #[inline(always)]
    fn default() -> Lpuart2 {
        Lpuart2(0)
    }
}
impl core::fmt::Debug for Lpuart2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART3 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3(pub u32);
impl Lpuart3 {
    #[doc = "LPUART3 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpuart3Inp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpuart3Inp::from_bits(val as u8)
    }
    #[doc = "LPUART3 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpuart3Inp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart3 {
    #[inline(always)]
    fn default() -> Lpuart3 {
        Lpuart3(0)
    }
}
impl core::fmt::Debug for Lpuart3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LPUART4 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4(pub u32);
impl Lpuart4 {
    #[doc = "LPUART4 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Lpuart4Inp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Lpuart4Inp::from_bits(val as u8)
    }
    #[doc = "LPUART4 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Lpuart4Inp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Lpuart4 {
    #[inline(always)]
    fn default() -> Lpuart4 {
        Lpuart4(0)
    }
}
impl core::fmt::Debug for Lpuart4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart4 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "OPAMP0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp0Trig(pub u32);
impl Opamp0Trig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Opamp0TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Opamp0TrigInp::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Opamp0TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Opamp0Trig {
    #[inline(always)]
    fn default() -> Opamp0Trig {
        Opamp0Trig(0)
    }
}
impl core::fmt::Debug for Opamp0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Opamp0Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Opamp0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Opamp0Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 external clock trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0ExtClk(pub u32);
impl Pwm0ExtClk {
    #[doc = "Trigger input connections for PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm0ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwm0ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
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
#[doc = "PWM1 external clock trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1ExtClk(pub u32);
impl Pwm1ExtClk {
    #[doc = "Trigger input connections for PWM."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm1ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwm1ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Home(pub u32);
impl Qdc0Home {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0HomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0HomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0HomeInp) {
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap1(pub u32);
impl Qdc0Icap1 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0Icap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0Icap1Inp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0Icap1Inp) {
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap2(pub u32);
impl Qdc0Icap2 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0Icap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0Icap2Inp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0Icap2Inp) {
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Icap3(pub u32);
impl Qdc0Icap3 {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0Icap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0Icap3Inp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0Icap3Inp) {
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Index(pub u32);
impl Qdc0Index {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0IndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0IndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0IndexInp) {
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Phasea(pub u32);
impl Qdc0Phasea {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Phaseb(pub u32);
impl Qdc0Phaseb {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
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
#[doc = "QDC0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc0Trig(pub u32);
impl Qdc0Trig {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc0TrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc0TrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc0TrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc0Trig {
    #[inline(always)]
    fn default() -> Qdc0Trig {
        Qdc0Trig(0)
    }
}
impl core::fmt::Debug for Qdc0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc0Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc0Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Home(pub u32);
impl Qdc1Home {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1HomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1HomeInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1HomeInp) {
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap1(pub u32);
impl Qdc1Icap1 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1Icap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1Icap1Inp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1Icap1Inp) {
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap2(pub u32);
impl Qdc1Icap2 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1Icap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1Icap2Inp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1Icap2Inp) {
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Icap3(pub u32);
impl Qdc1Icap3 {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1Icap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1Icap3Inp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1Icap3Inp) {
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Index(pub u32);
impl Qdc1Index {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1IndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1IndexInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1IndexInp) {
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Phasea(pub u32);
impl Qdc1Phasea {
    #[doc = "QDC0 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Phaseb(pub u32);
impl Qdc1Phaseb {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
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
#[doc = "QDC1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc1Trig(pub u32);
impl Qdc1Trig {
    #[doc = "QDC1 input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Qdc1TrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Qdc1TrigInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Qdc1TrigInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Qdc1Trig {
    #[inline(always)]
    fn default() -> Qdc1Trig {
        Qdc1Trig(0)
    }
}
impl core::fmt::Debug for Qdc1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdc1Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdc1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Qdc1Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger register for TIMER0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
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
#[doc = "Trigger register for TIMER1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer1trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
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
#[doc = "Trigger register for TIMER2 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer2trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
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
#[doc = "Trigger register for TIMER3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer3trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
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
#[doc = "Trigger register for TIMER4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer4trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
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
#[doc = "USB-FS trigger input connections."]
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
