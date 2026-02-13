#[doc = "ADC Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0Trig(pub u32);
impl Adc0Trig {
    #[doc = "ADC0 trigger inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Adc0TrigTrigin {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Adc0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Adc0TrigTrigin) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
#[doc = "ADC Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1Trig(pub u32);
impl Adc1Trig {
    #[doc = "ADC1 trigger inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Adc1TrigTrigin {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Adc1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC1 trigger inputs"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Adc1TrigTrigin) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
#[doc = "CMP0 Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0Trig(pub u32);
impl Cmp0Trig {
    #[doc = "CMP0 input trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Cmp0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmp0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger"]
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
#[doc = "CMP1 Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1Trig(pub u32);
impl Cmp1Trig {
    #[doc = "CMP1 input trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Cmp1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmp1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP1 input trigger"]
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
#[doc = "CMP2 Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2Trig(pub u32);
impl Cmp2Trig {
    #[doc = "CMP2 input trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Cmp2TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Cmp2TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP2 input trigger"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Cmp2TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Cmp2Trig {
    #[inline(always)]
    fn default() -> Cmp2Trig {
        Cmp2Trig(0)
    }
}
impl core::fmt::Debug for Cmp2Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp2Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap0(pub u32);
impl Ctimer0cap0 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0cap0Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0cap0Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0cap0Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap0 {
    #[inline(always)]
    fn default() -> Ctimer0cap0 {
        Ctimer0cap0(0)
    }
}
impl core::fmt::Debug for Ctimer0cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap1(pub u32);
impl Ctimer0cap1 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0cap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0cap1Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0cap1Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap1 {
    #[inline(always)]
    fn default() -> Ctimer0cap1 {
        Ctimer0cap1(0)
    }
}
impl core::fmt::Debug for Ctimer0cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap2(pub u32);
impl Ctimer0cap2 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0cap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0cap2Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0cap2Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap2 {
    #[inline(always)]
    fn default() -> Ctimer0cap2 {
        Ctimer0cap2(0)
    }
}
impl core::fmt::Debug for Ctimer0cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap3(pub u32);
impl Ctimer0cap3 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer0cap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer0cap3Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer0cap3Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0cap3 {
    #[inline(always)]
    fn default() -> Ctimer0cap3 {
        Ctimer0cap3(0)
    }
}
impl core::fmt::Debug for Ctimer0cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0cap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0cap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap0(pub u32);
impl Ctimer1cap0 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1cap0Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1cap0Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1cap0Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap0 {
    #[inline(always)]
    fn default() -> Ctimer1cap0 {
        Ctimer1cap0(0)
    }
}
impl core::fmt::Debug for Ctimer1cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap1(pub u32);
impl Ctimer1cap1 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1cap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1cap1Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1cap1Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap1 {
    #[inline(always)]
    fn default() -> Ctimer1cap1 {
        Ctimer1cap1(0)
    }
}
impl core::fmt::Debug for Ctimer1cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap2(pub u32);
impl Ctimer1cap2 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1cap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1cap2Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1cap2Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap2 {
    #[inline(always)]
    fn default() -> Ctimer1cap2 {
        Ctimer1cap2(0)
    }
}
impl core::fmt::Debug for Ctimer1cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1cap3(pub u32);
impl Ctimer1cap3 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer1cap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer1cap3Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer1cap3Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1cap3 {
    #[inline(always)]
    fn default() -> Ctimer1cap3 {
        Ctimer1cap3(0)
    }
}
impl core::fmt::Debug for Ctimer1cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1cap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1cap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap0(pub u32);
impl Ctimer2cap0 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2cap0Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2cap0Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2cap0Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap0 {
    #[inline(always)]
    fn default() -> Ctimer2cap0 {
        Ctimer2cap0(0)
    }
}
impl core::fmt::Debug for Ctimer2cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap1(pub u32);
impl Ctimer2cap1 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2cap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2cap1Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2cap1Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap1 {
    #[inline(always)]
    fn default() -> Ctimer2cap1 {
        Ctimer2cap1(0)
    }
}
impl core::fmt::Debug for Ctimer2cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap2(pub u32);
impl Ctimer2cap2 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2cap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2cap2Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2cap2Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap2 {
    #[inline(always)]
    fn default() -> Ctimer2cap2 {
        Ctimer2cap2(0)
    }
}
impl core::fmt::Debug for Ctimer2cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2cap3(pub u32);
impl Ctimer2cap3 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer2cap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer2cap3Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer2cap3Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2cap3 {
    #[inline(always)]
    fn default() -> Ctimer2cap3 {
        Ctimer2cap3(0)
    }
}
impl core::fmt::Debug for Ctimer2cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2cap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2cap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap0(pub u32);
impl Ctimer3cap0 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3cap0Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3cap0Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3cap0Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap0 {
    #[inline(always)]
    fn default() -> Ctimer3cap0 {
        Ctimer3cap0(0)
    }
}
impl core::fmt::Debug for Ctimer3cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap1(pub u32);
impl Ctimer3cap1 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3cap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3cap1Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3cap1Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap1 {
    #[inline(always)]
    fn default() -> Ctimer3cap1 {
        Ctimer3cap1(0)
    }
}
impl core::fmt::Debug for Ctimer3cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap2(pub u32);
impl Ctimer3cap2 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3cap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3cap2Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3cap2Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap2 {
    #[inline(always)]
    fn default() -> Ctimer3cap2 {
        Ctimer3cap2(0)
    }
}
impl core::fmt::Debug for Ctimer3cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3cap3(pub u32);
impl Ctimer3cap3 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer3cap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer3cap3Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer3cap3Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3cap3 {
    #[inline(always)]
    fn default() -> Ctimer3cap3 {
        Ctimer3cap3(0)
    }
}
impl core::fmt::Debug for Ctimer3cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3cap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3cap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap0(pub u32);
impl Ctimer4cap0 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4cap0Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4cap0Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4cap0Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap0 {
    #[inline(always)]
    fn default() -> Ctimer4cap0 {
        Ctimer4cap0(0)
    }
}
impl core::fmt::Debug for Ctimer4cap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap0")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap0 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap1(pub u32);
impl Ctimer4cap1 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4cap1Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4cap1Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4cap1Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap1 {
    #[inline(always)]
    fn default() -> Ctimer4cap1 {
        Ctimer4cap1(0)
    }
}
impl core::fmt::Debug for Ctimer4cap1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap1")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap1 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap2(pub u32);
impl Ctimer4cap2 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4cap2Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4cap2Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4cap2Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap2 {
    #[inline(always)]
    fn default() -> Ctimer4cap2 {
        Ctimer4cap2(0)
    }
}
impl core::fmt::Debug for Ctimer4cap2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap2")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap2 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4cap3(pub u32);
impl Ctimer4cap3 {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Ctimer4cap3Inp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Ctimer4cap3Inp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Ctimer4cap3Inp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4cap3 {
    #[inline(always)]
    fn default() -> Ctimer4cap3 {
        Ctimer4cap3(0)
    }
}
impl core::fmt::Debug for Ctimer4cap3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4cap3")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4cap3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4cap3 {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DAC0 Trigger Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac0Trig(pub u32);
impl Dac0Trig {
    #[doc = "DAC0 trigger input"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Dac0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dac0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input"]
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
#[doc = "DAC1 Trigger Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac1Trig(pub u32);
impl Dac1Trig {
    #[doc = "DAC1 trigger input"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Dac1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dac1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC1 trigger input"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Dac1TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Dac1Trig {
    #[inline(always)]
    fn default() -> Dac1Trig {
        Dac1Trig(0)
    }
}
impl core::fmt::Debug for Dac1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac1Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac1Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "DAC2 Trigger Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac2Trig(pub u32);
impl Dac2Trig {
    #[doc = "DAC2 trigger input"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Dac2TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dac2TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC2 trigger input"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Dac2TrigTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Dac2Trig {
    #[inline(always)]
    fn default() -> Dac2Trig {
        Dac2Trig(0)
    }
}
impl core::fmt::Debug for Dac2Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac2Trig")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac2Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dac2Trig {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "DMA0 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0(pub u32);
impl Dma0ReqEnable0 {
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en0(&self) -> super::vals::Req1En0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req1En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[inline(always)]
    pub const fn set_req1_en0(&mut self, val: super::vals::Req1En0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en0(&self) -> super::vals::Req2En0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req2En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[inline(always)]
    pub const fn set_req2_en0(&mut self, val: super::vals::Req2En0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en0(&self) -> super::vals::Req3En0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req3En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[inline(always)]
    pub const fn set_req3_en0(&mut self, val: super::vals::Req3En0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en0(&self) -> super::vals::Req4En0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req4En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[inline(always)]
    pub const fn set_req4_en0(&mut self, val: super::vals::Req4En0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en0(&self) -> super::vals::Req5En0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req5En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[inline(always)]
    pub const fn set_req5_en0(&mut self, val: super::vals::Req5En0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en0(&self) -> super::vals::Req6En0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req6En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[inline(always)]
    pub const fn set_req6_en0(&mut self, val: super::vals::Req6En0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en0(&self) -> super::vals::Req7En0 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req7En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req7_en0(&mut self, val: super::vals::Req7En0) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en0(&self) -> super::vals::Req8En0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req8En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req8_en0(&mut self, val: super::vals::Req8En0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en0(&self) -> super::vals::Req9En0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req9En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req9_en0(&mut self, val: super::vals::Req9En0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en0(&self) -> super::vals::Req10En0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req10En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req10_en0(&mut self, val: super::vals::Req10En0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en0(&self) -> super::vals::Req11En0 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req11En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req11_en0(&mut self, val: super::vals::Req11En0) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en0(&self) -> super::vals::Req12En0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req12En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req12_en0(&mut self, val: super::vals::Req12En0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en0(&self) -> super::vals::Req13En0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req13En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req13_en0(&mut self, val: super::vals::Req13En0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en0(&self) -> super::vals::Req14En0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req14En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req14_en0(&mut self, val: super::vals::Req14En0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en0(&self) -> super::vals::Req15En0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req15En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req15_en0(&mut self, val: super::vals::Req15En0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en0(&self) -> super::vals::Req16En0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req16En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req16_en0(&mut self, val: super::vals::Req16En0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en0(&self) -> super::vals::Req17En0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req17En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[inline(always)]
    pub const fn set_req17_en0(&mut self, val: super::vals::Req17En0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en0(&self) -> super::vals::Req18En0 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req18En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req18_en0(&mut self, val: super::vals::Req18En0) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en0(&self) -> super::vals::Req19En0 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req19En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[inline(always)]
    pub const fn set_req19_en0(&mut self, val: super::vals::Req19En0) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en0(&self) -> super::vals::Req20En0 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req20En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[inline(always)]
    pub const fn set_req20_en0(&mut self, val: super::vals::Req20En0) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en0(&self) -> super::vals::Req21En0 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req21En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[inline(always)]
    pub const fn set_req21_en0(&mut self, val: super::vals::Req21En0) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en0(&self) -> super::vals::Req22En0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req22En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[inline(always)]
    pub const fn set_req22_en0(&mut self, val: super::vals::Req22En0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en0(&self) -> super::vals::Req23En0 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req23En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[inline(always)]
    pub const fn set_req23_en0(&mut self, val: super::vals::Req23En0) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en0(&self) -> super::vals::Req24En0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req24En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[inline(always)]
    pub const fn set_req24_en0(&mut self, val: super::vals::Req24En0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en0(&self) -> super::vals::Req25En0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req25En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req25_en0(&mut self, val: super::vals::Req25En0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en0(&self) -> super::vals::Req26En0 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Req26En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[inline(always)]
    pub const fn set_req26_en0(&mut self, val: super::vals::Req26En0) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en0(&self) -> super::vals::Req27En0 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req27En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[inline(always)]
    pub const fn set_req27_en0(&mut self, val: super::vals::Req27En0) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en0(&self) -> super::vals::Req28En0 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req28En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[inline(always)]
    pub const fn set_req28_en0(&mut self, val: super::vals::Req28En0) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en0(&self) -> super::vals::Req29En0 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req29En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[inline(always)]
    pub const fn set_req29_en0(&mut self, val: super::vals::Req29En0) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en0(&self) -> super::vals::Req30En0 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req30En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[inline(always)]
    pub const fn set_req30_en0(&mut self, val: super::vals::Req30En0) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en0(&self) -> super::vals::Req31En0 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req31En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[inline(always)]
    pub const fn set_req31_en0(&mut self, val: super::vals::Req31En0) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable0 {
    #[inline(always)]
    fn default() -> Dma0ReqEnable0 {
        Dma0ReqEnable0(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable0")
            .field("req1_en0", &self.req1_en0())
            .field("req2_en0", &self.req2_en0())
            .field("req3_en0", &self.req3_en0())
            .field("req4_en0", &self.req4_en0())
            .field("req5_en0", &self.req5_en0())
            .field("req6_en0", &self.req6_en0())
            .field("req7_en0", &self.req7_en0())
            .field("req8_en0", &self.req8_en0())
            .field("req9_en0", &self.req9_en0())
            .field("req10_en0", &self.req10_en0())
            .field("req11_en0", &self.req11_en0())
            .field("req12_en0", &self.req12_en0())
            .field("req13_en0", &self.req13_en0())
            .field("req14_en0", &self.req14_en0())
            .field("req15_en0", &self.req15_en0())
            .field("req16_en0", &self.req16_en0())
            .field("req17_en0", &self.req17_en0())
            .field("req18_en0", &self.req18_en0())
            .field("req19_en0", &self.req19_en0())
            .field("req20_en0", &self.req20_en0())
            .field("req21_en0", &self.req21_en0())
            .field("req22_en0", &self.req22_en0())
            .field("req23_en0", &self.req23_en0())
            .field("req24_en0", &self.req24_en0())
            .field("req25_en0", &self.req25_en0())
            .field("req26_en0", &self.req26_en0())
            .field("req27_en0", &self.req27_en0())
            .field("req28_en0", &self.req28_en0())
            .field("req29_en0", &self.req29_en0())
            .field("req30_en0", &self.req30_en0())
            .field("req31_en0", &self.req31_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0 {{ req1_en0: {:?}, req2_en0: {:?}, req3_en0: {:?}, req4_en0: {:?}, req5_en0: {:?}, req6_en0: {:?}, req7_en0: {:?}, req8_en0: {:?}, req9_en0: {:?}, req10_en0: {:?}, req11_en0: {:?}, req12_en0: {:?}, req13_en0: {:?}, req14_en0: {:?}, req15_en0: {:?}, req16_en0: {:?}, req17_en0: {:?}, req18_en0: {:?}, req19_en0: {:?}, req20_en0: {:?}, req21_en0: {:?}, req22_en0: {:?}, req23_en0: {:?}, req24_en0: {:?}, req25_en0: {:?}, req26_en0: {:?}, req27_en0: {:?}, req28_en0: {:?}, req29_en0: {:?}, req30_en0: {:?}, req31_en0: {:?} }}",
            self.req1_en0(),
            self.req2_en0(),
            self.req3_en0(),
            self.req4_en0(),
            self.req5_en0(),
            self.req6_en0(),
            self.req7_en0(),
            self.req8_en0(),
            self.req9_en0(),
            self.req10_en0(),
            self.req11_en0(),
            self.req12_en0(),
            self.req13_en0(),
            self.req14_en0(),
            self.req15_en0(),
            self.req16_en0(),
            self.req17_en0(),
            self.req18_en0(),
            self.req19_en0(),
            self.req20_en0(),
            self.req21_en0(),
            self.req22_en0(),
            self.req23_en0(),
            self.req24_en0(),
            self.req25_en0(),
            self.req26_en0(),
            self.req27_en0(),
            self.req28_en0(),
            self.req29_en0(),
            self.req30_en0(),
            self.req31_en0()
        )
    }
}
#[doc = "DMA0 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Clr(pub u32);
impl Dma0ReqEnable0Clr {
    #[doc = "Writing a 1 to REQ1_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ9_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ9_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable0Clr {
    #[inline(always)]
    fn default() -> Dma0ReqEnable0Clr {
        Dma0ReqEnable0Clr(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable0Clr")
            .field("req1_en0", &self.req1_en0())
            .field("req2_en0", &self.req2_en0())
            .field("req3_en0", &self.req3_en0())
            .field("req4_en0", &self.req4_en0())
            .field("req5_en0", &self.req5_en0())
            .field("req6_en0", &self.req6_en0())
            .field("req7_en0", &self.req7_en0())
            .field("req8_en0", &self.req8_en0())
            .field("req9_en0", &self.req9_en0())
            .field("req10_en0", &self.req10_en0())
            .field("req11_en0", &self.req11_en0())
            .field("req12_en0", &self.req12_en0())
            .field("req13_en0", &self.req13_en0())
            .field("req14_en0", &self.req14_en0())
            .field("req15_en0", &self.req15_en0())
            .field("req16_en0", &self.req16_en0())
            .field("req17_en0", &self.req17_en0())
            .field("req18_en0", &self.req18_en0())
            .field("req19_en0", &self.req19_en0())
            .field("req20_en0", &self.req20_en0())
            .field("req21_en0", &self.req21_en0())
            .field("req22_en0", &self.req22_en0())
            .field("req23_en0", &self.req23_en0())
            .field("req24_en0", &self.req24_en0())
            .field("req25_en0", &self.req25_en0())
            .field("req26_en0", &self.req26_en0())
            .field("req27_en0", &self.req27_en0())
            .field("req28_en0", &self.req28_en0())
            .field("req29_en0", &self.req29_en0())
            .field("req30_en0", &self.req30_en0())
            .field("req31_en0", &self.req31_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Clr {{ req1_en0: {=bool:?}, req2_en0: {=bool:?}, req3_en0: {=bool:?}, req4_en0: {=bool:?}, req5_en0: {=bool:?}, req6_en0: {=bool:?}, req7_en0: {=bool:?}, req8_en0: {=bool:?}, req9_en0: {=bool:?}, req10_en0: {=bool:?}, req11_en0: {=bool:?}, req12_en0: {=bool:?}, req13_en0: {=bool:?}, req14_en0: {=bool:?}, req15_en0: {=bool:?}, req16_en0: {=bool:?}, req17_en0: {=bool:?}, req18_en0: {=bool:?}, req19_en0: {=bool:?}, req20_en0: {=bool:?}, req21_en0: {=bool:?}, req22_en0: {=bool:?}, req23_en0: {=bool:?}, req24_en0: {=bool:?}, req25_en0: {=bool:?}, req26_en0: {=bool:?}, req27_en0: {=bool:?}, req28_en0: {=bool:?}, req29_en0: {=bool:?}, req30_en0: {=bool:?}, req31_en0: {=bool:?} }}",
            self.req1_en0(),
            self.req2_en0(),
            self.req3_en0(),
            self.req4_en0(),
            self.req5_en0(),
            self.req6_en0(),
            self.req7_en0(),
            self.req8_en0(),
            self.req9_en0(),
            self.req10_en0(),
            self.req11_en0(),
            self.req12_en0(),
            self.req13_en0(),
            self.req14_en0(),
            self.req15_en0(),
            self.req16_en0(),
            self.req17_en0(),
            self.req18_en0(),
            self.req19_en0(),
            self.req20_en0(),
            self.req21_en0(),
            self.req22_en0(),
            self.req23_en0(),
            self.req24_en0(),
            self.req25_en0(),
            self.req26_en0(),
            self.req27_en0(),
            self.req28_en0(),
            self.req29_en0(),
            self.req30_en0(),
            self.req31_en0()
        )
    }
}
#[doc = "DMA0 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Set(pub u32);
impl Dma0ReqEnable0Set {
    #[doc = "Writing a 1 to REQ1_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ9_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ9_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable0Set {
    #[inline(always)]
    fn default() -> Dma0ReqEnable0Set {
        Dma0ReqEnable0Set(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable0Set")
            .field("req1_en0", &self.req1_en0())
            .field("req2_en0", &self.req2_en0())
            .field("req3_en0", &self.req3_en0())
            .field("req4_en0", &self.req4_en0())
            .field("req5_en0", &self.req5_en0())
            .field("req6_en0", &self.req6_en0())
            .field("req7_en0", &self.req7_en0())
            .field("req8_en0", &self.req8_en0())
            .field("req9_en0", &self.req9_en0())
            .field("req10_en0", &self.req10_en0())
            .field("req11_en0", &self.req11_en0())
            .field("req12_en0", &self.req12_en0())
            .field("req13_en0", &self.req13_en0())
            .field("req14_en0", &self.req14_en0())
            .field("req15_en0", &self.req15_en0())
            .field("req16_en0", &self.req16_en0())
            .field("req17_en0", &self.req17_en0())
            .field("req18_en0", &self.req18_en0())
            .field("req19_en0", &self.req19_en0())
            .field("req20_en0", &self.req20_en0())
            .field("req21_en0", &self.req21_en0())
            .field("req22_en0", &self.req22_en0())
            .field("req23_en0", &self.req23_en0())
            .field("req24_en0", &self.req24_en0())
            .field("req25_en0", &self.req25_en0())
            .field("req26_en0", &self.req26_en0())
            .field("req27_en0", &self.req27_en0())
            .field("req28_en0", &self.req28_en0())
            .field("req29_en0", &self.req29_en0())
            .field("req30_en0", &self.req30_en0())
            .field("req31_en0", &self.req31_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Set {{ req1_en0: {=bool:?}, req2_en0: {=bool:?}, req3_en0: {=bool:?}, req4_en0: {=bool:?}, req5_en0: {=bool:?}, req6_en0: {=bool:?}, req7_en0: {=bool:?}, req8_en0: {=bool:?}, req9_en0: {=bool:?}, req10_en0: {=bool:?}, req11_en0: {=bool:?}, req12_en0: {=bool:?}, req13_en0: {=bool:?}, req14_en0: {=bool:?}, req15_en0: {=bool:?}, req16_en0: {=bool:?}, req17_en0: {=bool:?}, req18_en0: {=bool:?}, req19_en0: {=bool:?}, req20_en0: {=bool:?}, req21_en0: {=bool:?}, req22_en0: {=bool:?}, req23_en0: {=bool:?}, req24_en0: {=bool:?}, req25_en0: {=bool:?}, req26_en0: {=bool:?}, req27_en0: {=bool:?}, req28_en0: {=bool:?}, req29_en0: {=bool:?}, req30_en0: {=bool:?}, req31_en0: {=bool:?} }}",
            self.req1_en0(),
            self.req2_en0(),
            self.req3_en0(),
            self.req4_en0(),
            self.req5_en0(),
            self.req6_en0(),
            self.req7_en0(),
            self.req8_en0(),
            self.req9_en0(),
            self.req10_en0(),
            self.req11_en0(),
            self.req12_en0(),
            self.req13_en0(),
            self.req14_en0(),
            self.req15_en0(),
            self.req16_en0(),
            self.req17_en0(),
            self.req18_en0(),
            self.req19_en0(),
            self.req20_en0(),
            self.req21_en0(),
            self.req22_en0(),
            self.req23_en0(),
            self.req24_en0(),
            self.req25_en0(),
            self.req26_en0(),
            self.req27_en0(),
            self.req28_en0(),
            self.req29_en0(),
            self.req30_en0(),
            self.req31_en0()
        )
    }
}
#[doc = "DMA0 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Tog(pub u32);
impl Dma0ReqEnable0Tog {
    #[doc = "Writing a 1 to REQ1_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to RE9_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to RE9_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable0Tog {
    #[inline(always)]
    fn default() -> Dma0ReqEnable0Tog {
        Dma0ReqEnable0Tog(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable0Tog")
            .field("req1_en0", &self.req1_en0())
            .field("req2_en0", &self.req2_en0())
            .field("req3_en0", &self.req3_en0())
            .field("req4_en0", &self.req4_en0())
            .field("req5_en0", &self.req5_en0())
            .field("req6_en0", &self.req6_en0())
            .field("req7_en0", &self.req7_en0())
            .field("req8_en0", &self.req8_en0())
            .field("req9_en0", &self.req9_en0())
            .field("req10_en0", &self.req10_en0())
            .field("req11_en0", &self.req11_en0())
            .field("req12_en0", &self.req12_en0())
            .field("req13_en0", &self.req13_en0())
            .field("req14_en0", &self.req14_en0())
            .field("req15_en0", &self.req15_en0())
            .field("req16_en0", &self.req16_en0())
            .field("req17_en0", &self.req17_en0())
            .field("req18_en0", &self.req18_en0())
            .field("req19_en0", &self.req19_en0())
            .field("req20_en0", &self.req20_en0())
            .field("req21_en0", &self.req21_en0())
            .field("req22_en0", &self.req22_en0())
            .field("req23_en0", &self.req23_en0())
            .field("req24_en0", &self.req24_en0())
            .field("req25_en0", &self.req25_en0())
            .field("req26_en0", &self.req26_en0())
            .field("req27_en0", &self.req27_en0())
            .field("req28_en0", &self.req28_en0())
            .field("req29_en0", &self.req29_en0())
            .field("req30_en0", &self.req30_en0())
            .field("req31_en0", &self.req31_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Tog {{ req1_en0: {=bool:?}, req2_en0: {=bool:?}, req3_en0: {=bool:?}, req4_en0: {=bool:?}, req5_en0: {=bool:?}, req6_en0: {=bool:?}, req7_en0: {=bool:?}, req8_en0: {=bool:?}, req9_en0: {=bool:?}, req10_en0: {=bool:?}, req11_en0: {=bool:?}, req12_en0: {=bool:?}, req13_en0: {=bool:?}, req14_en0: {=bool:?}, req15_en0: {=bool:?}, req16_en0: {=bool:?}, req17_en0: {=bool:?}, req18_en0: {=bool:?}, req19_en0: {=bool:?}, req20_en0: {=bool:?}, req21_en0: {=bool:?}, req22_en0: {=bool:?}, req23_en0: {=bool:?}, req24_en0: {=bool:?}, req25_en0: {=bool:?}, req26_en0: {=bool:?}, req27_en0: {=bool:?}, req28_en0: {=bool:?}, req29_en0: {=bool:?}, req30_en0: {=bool:?}, req31_en0: {=bool:?} }}",
            self.req1_en0(),
            self.req2_en0(),
            self.req3_en0(),
            self.req4_en0(),
            self.req5_en0(),
            self.req6_en0(),
            self.req7_en0(),
            self.req8_en0(),
            self.req9_en0(),
            self.req10_en0(),
            self.req11_en0(),
            self.req12_en0(),
            self.req13_en0(),
            self.req14_en0(),
            self.req15_en0(),
            self.req16_en0(),
            self.req17_en0(),
            self.req18_en0(),
            self.req19_en0(),
            self.req20_en0(),
            self.req21_en0(),
            self.req22_en0(),
            self.req23_en0(),
            self.req24_en0(),
            self.req25_en0(),
            self.req26_en0(),
            self.req27_en0(),
            self.req28_en0(),
            self.req29_en0(),
            self.req30_en0(),
            self.req31_en0()
        )
    }
}
#[doc = "DMA0 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable1(pub u32);
impl Dma0ReqEnable1 {
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en0(&self) -> super::vals::Req32En0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req32En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[inline(always)]
    pub const fn set_req32_en0(&mut self, val: super::vals::Req32En0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en0(&self) -> super::vals::Req33En0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req33En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[inline(always)]
    pub const fn set_req33_en0(&mut self, val: super::vals::Req33En0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en0(&self) -> super::vals::Req34En0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req34En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[inline(always)]
    pub const fn set_req34_en0(&mut self, val: super::vals::Req34En0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en0(&self) -> super::vals::Req35En0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req35En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[inline(always)]
    pub const fn set_req35_en0(&mut self, val: super::vals::Req35En0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en0(&self) -> super::vals::Req36En0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req36En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[inline(always)]
    pub const fn set_req36_en0(&mut self, val: super::vals::Req36En0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en0(&self) -> super::vals::Req37En0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req37En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[inline(always)]
    pub const fn set_req37_en0(&mut self, val: super::vals::Req37En0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en0(&self) -> super::vals::Req38En0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req38En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[inline(always)]
    pub const fn set_req38_en0(&mut self, val: super::vals::Req38En0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en0(&self) -> super::vals::Req39En0 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req39En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req39_en0(&mut self, val: super::vals::Req39En0) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en0(&self) -> super::vals::Req40En0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req40En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req40_en0(&mut self, val: super::vals::Req40En0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en0(&self) -> super::vals::Req41En0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req41En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req41_en0(&mut self, val: super::vals::Req41En0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en0(&self) -> super::vals::Req42En0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req42En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req42_en0(&mut self, val: super::vals::Req42En0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en0(&self) -> super::vals::Req43En0 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req43En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req43_en0(&mut self, val: super::vals::Req43En0) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en0(&self) -> super::vals::Req44En0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req44En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req44_en0(&mut self, val: super::vals::Req44En0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en0(&self) -> super::vals::Req45En0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req45En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req45_en0(&mut self, val: super::vals::Req45En0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en0(&self) -> super::vals::Req46En0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req46En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req46_en0(&mut self, val: super::vals::Req46En0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en0(&self) -> super::vals::Req47En0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req47En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req47_en0(&mut self, val: super::vals::Req47En0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en0(&self) -> super::vals::Req48En0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req48En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req48_en0(&mut self, val: super::vals::Req48En0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en0(&self) -> super::vals::Req49En0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req49En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req49_en0(&mut self, val: super::vals::Req49En0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en0(&self) -> super::vals::Req50En0 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req50En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req50_en0(&mut self, val: super::vals::Req50En0) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en0(&self) -> super::vals::Req51En0 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req51En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req51_en0(&mut self, val: super::vals::Req51En0) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en0(&self) -> super::vals::Req52En0 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req52En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req52_en0(&mut self, val: super::vals::Req52En0) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en0(&self) -> super::vals::Req53En0 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req53En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req53_en0(&mut self, val: super::vals::Req53En0) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en0(&self) -> super::vals::Req54En0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req54En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req54_en0(&mut self, val: super::vals::Req54En0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en0(&self) -> super::vals::Req57En0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req57En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[inline(always)]
    pub const fn set_req57_en0(&mut self, val: super::vals::Req57En0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en0(&self) -> super::vals::Req58En0 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Req58En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[inline(always)]
    pub const fn set_req58_en0(&mut self, val: super::vals::Req58En0) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en0(&self) -> super::vals::Req59En0 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req59En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[inline(always)]
    pub const fn set_req59_en0(&mut self, val: super::vals::Req59En0) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en0(&self) -> super::vals::Req60En0 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req60En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[inline(always)]
    pub const fn set_req60_en0(&mut self, val: super::vals::Req60En0) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en0(&self) -> super::vals::Req61En0 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req61En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[inline(always)]
    pub const fn set_req61_en0(&mut self, val: super::vals::Req61En0) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en0(&self) -> super::vals::Req62En0 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req62En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[inline(always)]
    pub const fn set_req62_en0(&mut self, val: super::vals::Req62En0) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en0(&self) -> super::vals::Req63En0 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req63En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[inline(always)]
    pub const fn set_req63_en0(&mut self, val: super::vals::Req63En0) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable1 {
    #[inline(always)]
    fn default() -> Dma0ReqEnable1 {
        Dma0ReqEnable1(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable1")
            .field("req32_en0", &self.req32_en0())
            .field("req33_en0", &self.req33_en0())
            .field("req34_en0", &self.req34_en0())
            .field("req35_en0", &self.req35_en0())
            .field("req36_en0", &self.req36_en0())
            .field("req37_en0", &self.req37_en0())
            .field("req38_en0", &self.req38_en0())
            .field("req39_en0", &self.req39_en0())
            .field("req40_en0", &self.req40_en0())
            .field("req41_en0", &self.req41_en0())
            .field("req42_en0", &self.req42_en0())
            .field("req43_en0", &self.req43_en0())
            .field("req44_en0", &self.req44_en0())
            .field("req45_en0", &self.req45_en0())
            .field("req46_en0", &self.req46_en0())
            .field("req47_en0", &self.req47_en0())
            .field("req48_en0", &self.req48_en0())
            .field("req49_en0", &self.req49_en0())
            .field("req50_en0", &self.req50_en0())
            .field("req51_en0", &self.req51_en0())
            .field("req52_en0", &self.req52_en0())
            .field("req53_en0", &self.req53_en0())
            .field("req54_en0", &self.req54_en0())
            .field("req57_en0", &self.req57_en0())
            .field("req58_en0", &self.req58_en0())
            .field("req59_en0", &self.req59_en0())
            .field("req60_en0", &self.req60_en0())
            .field("req61_en0", &self.req61_en0())
            .field("req62_en0", &self.req62_en0())
            .field("req63_en0", &self.req63_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable1 {{ req32_en0: {:?}, req33_en0: {:?}, req34_en0: {:?}, req35_en0: {:?}, req36_en0: {:?}, req37_en0: {:?}, req38_en0: {:?}, req39_en0: {:?}, req40_en0: {:?}, req41_en0: {:?}, req42_en0: {:?}, req43_en0: {:?}, req44_en0: {:?}, req45_en0: {:?}, req46_en0: {:?}, req47_en0: {:?}, req48_en0: {:?}, req49_en0: {:?}, req50_en0: {:?}, req51_en0: {:?}, req52_en0: {:?}, req53_en0: {:?}, req54_en0: {:?}, req57_en0: {:?}, req58_en0: {:?}, req59_en0: {:?}, req60_en0: {:?}, req61_en0: {:?}, req62_en0: {:?}, req63_en0: {:?} }}",
            self.req32_en0(),
            self.req33_en0(),
            self.req34_en0(),
            self.req35_en0(),
            self.req36_en0(),
            self.req37_en0(),
            self.req38_en0(),
            self.req39_en0(),
            self.req40_en0(),
            self.req41_en0(),
            self.req42_en0(),
            self.req43_en0(),
            self.req44_en0(),
            self.req45_en0(),
            self.req46_en0(),
            self.req47_en0(),
            self.req48_en0(),
            self.req49_en0(),
            self.req50_en0(),
            self.req51_en0(),
            self.req52_en0(),
            self.req53_en0(),
            self.req54_en0(),
            self.req57_en0(),
            self.req58_en0(),
            self.req59_en0(),
            self.req60_en0(),
            self.req61_en0(),
            self.req62_en0(),
            self.req63_en0()
        )
    }
}
#[doc = "DMA0 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable1Clr(pub u32);
impl Dma0ReqEnable1Clr {
    #[doc = "Writing a 1 to REQ32_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ45_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ45_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable1Clr {
    #[inline(always)]
    fn default() -> Dma0ReqEnable1Clr {
        Dma0ReqEnable1Clr(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable1Clr")
            .field("req32_en0", &self.req32_en0())
            .field("req33_en0", &self.req33_en0())
            .field("req34_en0", &self.req34_en0())
            .field("req35_en0", &self.req35_en0())
            .field("req36_en0", &self.req36_en0())
            .field("req37_en0", &self.req37_en0())
            .field("req38_en0", &self.req38_en0())
            .field("req39_en0", &self.req39_en0())
            .field("req40_en0", &self.req40_en0())
            .field("req41_en0", &self.req41_en0())
            .field("req42_en0", &self.req42_en0())
            .field("req43_en0", &self.req43_en0())
            .field("req44_en0", &self.req44_en0())
            .field("req45_en0", &self.req45_en0())
            .field("req46_en0", &self.req46_en0())
            .field("req47_en0", &self.req47_en0())
            .field("req48_en0", &self.req48_en0())
            .field("req49_en0", &self.req49_en0())
            .field("req50_en0", &self.req50_en0())
            .field("req51_en0", &self.req51_en0())
            .field("req52_en0", &self.req52_en0())
            .field("req53_en0", &self.req53_en0())
            .field("req54_en0", &self.req54_en0())
            .field("req57_en0", &self.req57_en0())
            .field("req58_en0", &self.req58_en0())
            .field("req59_en0", &self.req59_en0())
            .field("req60_en0", &self.req60_en0())
            .field("req61_en0", &self.req61_en0())
            .field("req62_en0", &self.req62_en0())
            .field("req63_en0", &self.req63_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable1Clr {{ req32_en0: {=bool:?}, req33_en0: {=bool:?}, req34_en0: {=bool:?}, req35_en0: {=bool:?}, req36_en0: {=bool:?}, req37_en0: {=bool:?}, req38_en0: {=bool:?}, req39_en0: {=bool:?}, req40_en0: {=bool:?}, req41_en0: {=bool:?}, req42_en0: {=bool:?}, req43_en0: {=bool:?}, req44_en0: {=bool:?}, req45_en0: {=bool:?}, req46_en0: {=bool:?}, req47_en0: {=bool:?}, req48_en0: {=bool:?}, req49_en0: {=bool:?}, req50_en0: {=bool:?}, req51_en0: {=bool:?}, req52_en0: {=bool:?}, req53_en0: {=bool:?}, req54_en0: {=bool:?}, req57_en0: {=bool:?}, req58_en0: {=bool:?}, req59_en0: {=bool:?}, req60_en0: {=bool:?}, req61_en0: {=bool:?}, req62_en0: {=bool:?}, req63_en0: {=bool:?} }}",
            self.req32_en0(),
            self.req33_en0(),
            self.req34_en0(),
            self.req35_en0(),
            self.req36_en0(),
            self.req37_en0(),
            self.req38_en0(),
            self.req39_en0(),
            self.req40_en0(),
            self.req41_en0(),
            self.req42_en0(),
            self.req43_en0(),
            self.req44_en0(),
            self.req45_en0(),
            self.req46_en0(),
            self.req47_en0(),
            self.req48_en0(),
            self.req49_en0(),
            self.req50_en0(),
            self.req51_en0(),
            self.req52_en0(),
            self.req53_en0(),
            self.req54_en0(),
            self.req57_en0(),
            self.req58_en0(),
            self.req59_en0(),
            self.req60_en0(),
            self.req61_en0(),
            self.req62_en0(),
            self.req63_en0()
        )
    }
}
#[doc = "DMA0 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable1Set(pub u32);
impl Dma0ReqEnable1Set {
    #[doc = "Writing a 1 to REQ32_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ45_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ45_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable1Set {
    #[inline(always)]
    fn default() -> Dma0ReqEnable1Set {
        Dma0ReqEnable1Set(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable1Set")
            .field("req32_en0", &self.req32_en0())
            .field("req33_en0", &self.req33_en0())
            .field("req34_en0", &self.req34_en0())
            .field("req35_en0", &self.req35_en0())
            .field("req36_en0", &self.req36_en0())
            .field("req37_en0", &self.req37_en0())
            .field("req38_en0", &self.req38_en0())
            .field("req39_en0", &self.req39_en0())
            .field("req40_en0", &self.req40_en0())
            .field("req41_en0", &self.req41_en0())
            .field("req42_en0", &self.req42_en0())
            .field("req43_en0", &self.req43_en0())
            .field("req44_en0", &self.req44_en0())
            .field("req45_en0", &self.req45_en0())
            .field("req46_en0", &self.req46_en0())
            .field("req47_en0", &self.req47_en0())
            .field("req48_en0", &self.req48_en0())
            .field("req49_en0", &self.req49_en0())
            .field("req50_en0", &self.req50_en0())
            .field("req51_en0", &self.req51_en0())
            .field("req52_en0", &self.req52_en0())
            .field("req53_en0", &self.req53_en0())
            .field("req54_en0", &self.req54_en0())
            .field("req57_en0", &self.req57_en0())
            .field("req58_en0", &self.req58_en0())
            .field("req59_en0", &self.req59_en0())
            .field("req60_en0", &self.req60_en0())
            .field("req61_en0", &self.req61_en0())
            .field("req62_en0", &self.req62_en0())
            .field("req63_en0", &self.req63_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable1Set {{ req32_en0: {=bool:?}, req33_en0: {=bool:?}, req34_en0: {=bool:?}, req35_en0: {=bool:?}, req36_en0: {=bool:?}, req37_en0: {=bool:?}, req38_en0: {=bool:?}, req39_en0: {=bool:?}, req40_en0: {=bool:?}, req41_en0: {=bool:?}, req42_en0: {=bool:?}, req43_en0: {=bool:?}, req44_en0: {=bool:?}, req45_en0: {=bool:?}, req46_en0: {=bool:?}, req47_en0: {=bool:?}, req48_en0: {=bool:?}, req49_en0: {=bool:?}, req50_en0: {=bool:?}, req51_en0: {=bool:?}, req52_en0: {=bool:?}, req53_en0: {=bool:?}, req54_en0: {=bool:?}, req57_en0: {=bool:?}, req58_en0: {=bool:?}, req59_en0: {=bool:?}, req60_en0: {=bool:?}, req61_en0: {=bool:?}, req62_en0: {=bool:?}, req63_en0: {=bool:?} }}",
            self.req32_en0(),
            self.req33_en0(),
            self.req34_en0(),
            self.req35_en0(),
            self.req36_en0(),
            self.req37_en0(),
            self.req38_en0(),
            self.req39_en0(),
            self.req40_en0(),
            self.req41_en0(),
            self.req42_en0(),
            self.req43_en0(),
            self.req44_en0(),
            self.req45_en0(),
            self.req46_en0(),
            self.req47_en0(),
            self.req48_en0(),
            self.req49_en0(),
            self.req50_en0(),
            self.req51_en0(),
            self.req52_en0(),
            self.req53_en0(),
            self.req54_en0(),
            self.req57_en0(),
            self.req58_en0(),
            self.req59_en0(),
            self.req60_en0(),
            self.req61_en0(),
            self.req62_en0(),
            self.req63_en0()
        )
    }
}
#[doc = "DMA0 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable1Tog(pub u32);
impl Dma0ReqEnable1Tog {
    #[doc = "Writing a 1 to REQ32_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ55_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ55_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable1Tog {
    #[inline(always)]
    fn default() -> Dma0ReqEnable1Tog {
        Dma0ReqEnable1Tog(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable1Tog")
            .field("req32_en0", &self.req32_en0())
            .field("req33_en0", &self.req33_en0())
            .field("req34_en0", &self.req34_en0())
            .field("req35_en0", &self.req35_en0())
            .field("req36_en0", &self.req36_en0())
            .field("req37_en0", &self.req37_en0())
            .field("req38_en0", &self.req38_en0())
            .field("req39_en0", &self.req39_en0())
            .field("req40_en0", &self.req40_en0())
            .field("req41_en0", &self.req41_en0())
            .field("req42_en0", &self.req42_en0())
            .field("req43_en0", &self.req43_en0())
            .field("req44_en0", &self.req44_en0())
            .field("req45_en0", &self.req45_en0())
            .field("req46_en0", &self.req46_en0())
            .field("req47_en0", &self.req47_en0())
            .field("req48_en0", &self.req48_en0())
            .field("req49_en0", &self.req49_en0())
            .field("req50_en0", &self.req50_en0())
            .field("req51_en0", &self.req51_en0())
            .field("req52_en0", &self.req52_en0())
            .field("req53_en0", &self.req53_en0())
            .field("req54_en0", &self.req54_en0())
            .field("req57_en0", &self.req57_en0())
            .field("req58_en0", &self.req58_en0())
            .field("req59_en0", &self.req59_en0())
            .field("req60_en0", &self.req60_en0())
            .field("req61_en0", &self.req61_en0())
            .field("req62_en0", &self.req62_en0())
            .field("req63_en0", &self.req63_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable1Tog {{ req32_en0: {=bool:?}, req33_en0: {=bool:?}, req34_en0: {=bool:?}, req35_en0: {=bool:?}, req36_en0: {=bool:?}, req37_en0: {=bool:?}, req38_en0: {=bool:?}, req39_en0: {=bool:?}, req40_en0: {=bool:?}, req41_en0: {=bool:?}, req42_en0: {=bool:?}, req43_en0: {=bool:?}, req44_en0: {=bool:?}, req45_en0: {=bool:?}, req46_en0: {=bool:?}, req47_en0: {=bool:?}, req48_en0: {=bool:?}, req49_en0: {=bool:?}, req50_en0: {=bool:?}, req51_en0: {=bool:?}, req52_en0: {=bool:?}, req53_en0: {=bool:?}, req54_en0: {=bool:?}, req57_en0: {=bool:?}, req58_en0: {=bool:?}, req59_en0: {=bool:?}, req60_en0: {=bool:?}, req61_en0: {=bool:?}, req62_en0: {=bool:?}, req63_en0: {=bool:?} }}",
            self.req32_en0(),
            self.req33_en0(),
            self.req34_en0(),
            self.req35_en0(),
            self.req36_en0(),
            self.req37_en0(),
            self.req38_en0(),
            self.req39_en0(),
            self.req40_en0(),
            self.req41_en0(),
            self.req42_en0(),
            self.req43_en0(),
            self.req44_en0(),
            self.req45_en0(),
            self.req46_en0(),
            self.req47_en0(),
            self.req48_en0(),
            self.req49_en0(),
            self.req50_en0(),
            self.req51_en0(),
            self.req52_en0(),
            self.req53_en0(),
            self.req54_en0(),
            self.req57_en0(),
            self.req58_en0(),
            self.req59_en0(),
            self.req60_en0(),
            self.req61_en0(),
            self.req62_en0(),
            self.req63_en0()
        )
    }
}
#[doc = "DMA0 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable2(pub u32);
impl Dma0ReqEnable2 {
    #[doc = "This register is used to enable and disable FlexIO0 shift register 3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en0(&self) -> super::vals::Req64En0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req64En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 3 request."]
    #[inline(always)]
    pub const fn set_req64_en0(&mut self, val: super::vals::Req64En0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 4 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en0(&self) -> super::vals::Req65En0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req65En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 4 request."]
    #[inline(always)]
    pub const fn set_req65_en0(&mut self, val: super::vals::Req65En0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 5 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en0(&self) -> super::vals::Req66En0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req66En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 5 request."]
    #[inline(always)]
    pub const fn set_req66_en0(&mut self, val: super::vals::Req66En0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 6 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en0(&self) -> super::vals::Req67En0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req67En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 6 request."]
    #[inline(always)]
    pub const fn set_req67_en0(&mut self, val: super::vals::Req67En0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 7 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en0(&self) -> super::vals::Req68En0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req68En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 7 request."]
    #[inline(always)]
    pub const fn set_req68_en0(&mut self, val: super::vals::Req68En0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en0(&self) -> super::vals::Req69En0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req69En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[inline(always)]
    pub const fn set_req69_en0(&mut self, val: super::vals::Req69En0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en0(&self) -> super::vals::Req70En0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req70En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[inline(always)]
    pub const fn set_req70_en0(&mut self, val: super::vals::Req70En0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en0(&self) -> super::vals::Req71En0 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req71En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[inline(always)]
    pub const fn set_req71_en0(&mut self, val: super::vals::Req71En0) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en0(&self) -> super::vals::Req72En0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req72En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[inline(always)]
    pub const fn set_req72_en0(&mut self, val: super::vals::Req72En0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en0(&self) -> super::vals::Req73En0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req73En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[inline(always)]
    pub const fn set_req73_en0(&mut self, val: super::vals::Req73En0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en0(&self) -> super::vals::Req74En0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req74En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[inline(always)]
    pub const fn set_req74_en0(&mut self, val: super::vals::Req74En0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en0(&self) -> super::vals::Req75En0 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req75En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[inline(always)]
    pub const fn set_req75_en0(&mut self, val: super::vals::Req75En0) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en0(&self) -> super::vals::Req76En0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req76En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[inline(always)]
    pub const fn set_req76_en0(&mut self, val: super::vals::Req76En0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en0(&self) -> super::vals::Req77En0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req77En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[inline(always)]
    pub const fn set_req77_en0(&mut self, val: super::vals::Req77En0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en0(&self) -> super::vals::Req78En0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req78En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[inline(always)]
    pub const fn set_req78_en0(&mut self, val: super::vals::Req78En0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en0(&self) -> super::vals::Req79En0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req79En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[inline(always)]
    pub const fn set_req79_en0(&mut self, val: super::vals::Req79En0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en0(&self) -> super::vals::Req80En0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req80En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[inline(always)]
    pub const fn set_req80_en0(&mut self, val: super::vals::Req80En0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en0(&self) -> super::vals::Req81En0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req81En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[inline(always)]
    pub const fn set_req81_en0(&mut self, val: super::vals::Req81En0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en0(&self) -> super::vals::Req82En0 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req82En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[inline(always)]
    pub const fn set_req82_en0(&mut self, val: super::vals::Req82En0) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en0(&self) -> super::vals::Req83En0 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req83En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[inline(always)]
    pub const fn set_req83_en0(&mut self, val: super::vals::Req83En0) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en0(&self) -> super::vals::Req84En0 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req84En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[inline(always)]
    pub const fn set_req84_en0(&mut self, val: super::vals::Req84En0) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en0(&self) -> super::vals::Req85En0 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req85En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[inline(always)]
    pub const fn set_req85_en0(&mut self, val: super::vals::Req85En0) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en0(&self) -> super::vals::Req86En0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req86En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[inline(always)]
    pub const fn set_req86_en0(&mut self, val: super::vals::Req86En0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en0(&self) -> super::vals::Req87En0 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req87En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[inline(always)]
    pub const fn set_req87_en0(&mut self, val: super::vals::Req87En0) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en0(&self) -> super::vals::Req88En0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req88En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[inline(always)]
    pub const fn set_req88_en0(&mut self, val: super::vals::Req88En0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en0(&self) -> super::vals::Req91En0 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req91En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[inline(always)]
    pub const fn set_req91_en0(&mut self, val: super::vals::Req91En0) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en0(&self) -> super::vals::Req92En0 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req92En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[inline(always)]
    pub const fn set_req92_en0(&mut self, val: super::vals::Req92En0) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en0(&self) -> super::vals::Req93En0 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req93En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[inline(always)]
    pub const fn set_req93_en0(&mut self, val: super::vals::Req93En0) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en0(&self) -> super::vals::Req94En0 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req94En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[inline(always)]
    pub const fn set_req94_en0(&mut self, val: super::vals::Req94En0) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en0(&self) -> super::vals::Req95En0 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req95En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[inline(always)]
    pub const fn set_req95_en0(&mut self, val: super::vals::Req95En0) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable2 {
    #[inline(always)]
    fn default() -> Dma0ReqEnable2 {
        Dma0ReqEnable2(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable2")
            .field("req64_en0", &self.req64_en0())
            .field("req65_en0", &self.req65_en0())
            .field("req66_en0", &self.req66_en0())
            .field("req67_en0", &self.req67_en0())
            .field("req68_en0", &self.req68_en0())
            .field("req69_en0", &self.req69_en0())
            .field("req70_en0", &self.req70_en0())
            .field("req71_en0", &self.req71_en0())
            .field("req72_en0", &self.req72_en0())
            .field("req73_en0", &self.req73_en0())
            .field("req74_en0", &self.req74_en0())
            .field("req75_en0", &self.req75_en0())
            .field("req76_en0", &self.req76_en0())
            .field("req77_en0", &self.req77_en0())
            .field("req78_en0", &self.req78_en0())
            .field("req79_en0", &self.req79_en0())
            .field("req80_en0", &self.req80_en0())
            .field("req81_en0", &self.req81_en0())
            .field("req82_en0", &self.req82_en0())
            .field("req83_en0", &self.req83_en0())
            .field("req84_en0", &self.req84_en0())
            .field("req85_en0", &self.req85_en0())
            .field("req86_en0", &self.req86_en0())
            .field("req87_en0", &self.req87_en0())
            .field("req88_en0", &self.req88_en0())
            .field("req91_en0", &self.req91_en0())
            .field("req92_en0", &self.req92_en0())
            .field("req93_en0", &self.req93_en0())
            .field("req94_en0", &self.req94_en0())
            .field("req95_en0", &self.req95_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable2 {{ req64_en0: {:?}, req65_en0: {:?}, req66_en0: {:?}, req67_en0: {:?}, req68_en0: {:?}, req69_en0: {:?}, req70_en0: {:?}, req71_en0: {:?}, req72_en0: {:?}, req73_en0: {:?}, req74_en0: {:?}, req75_en0: {:?}, req76_en0: {:?}, req77_en0: {:?}, req78_en0: {:?}, req79_en0: {:?}, req80_en0: {:?}, req81_en0: {:?}, req82_en0: {:?}, req83_en0: {:?}, req84_en0: {:?}, req85_en0: {:?}, req86_en0: {:?}, req87_en0: {:?}, req88_en0: {:?}, req91_en0: {:?}, req92_en0: {:?}, req93_en0: {:?}, req94_en0: {:?}, req95_en0: {:?} }}",
            self.req64_en0(),
            self.req65_en0(),
            self.req66_en0(),
            self.req67_en0(),
            self.req68_en0(),
            self.req69_en0(),
            self.req70_en0(),
            self.req71_en0(),
            self.req72_en0(),
            self.req73_en0(),
            self.req74_en0(),
            self.req75_en0(),
            self.req76_en0(),
            self.req77_en0(),
            self.req78_en0(),
            self.req79_en0(),
            self.req80_en0(),
            self.req81_en0(),
            self.req82_en0(),
            self.req83_en0(),
            self.req84_en0(),
            self.req85_en0(),
            self.req86_en0(),
            self.req87_en0(),
            self.req88_en0(),
            self.req91_en0(),
            self.req92_en0(),
            self.req93_en0(),
            self.req94_en0(),
            self.req95_en0()
        )
    }
}
#[doc = "DMA0 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable2Clr(pub u32);
impl Dma0ReqEnable2Clr {
    #[doc = "Writing a 1 to REQ64_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ76_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ76_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable2Clr {
    #[inline(always)]
    fn default() -> Dma0ReqEnable2Clr {
        Dma0ReqEnable2Clr(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable2Clr")
            .field("req64_en0", &self.req64_en0())
            .field("req65_en0", &self.req65_en0())
            .field("req66_en0", &self.req66_en0())
            .field("req67_en0", &self.req67_en0())
            .field("req68_en0", &self.req68_en0())
            .field("req69_en0", &self.req69_en0())
            .field("req70_en0", &self.req70_en0())
            .field("req71_en0", &self.req71_en0())
            .field("req72_en0", &self.req72_en0())
            .field("req73_en0", &self.req73_en0())
            .field("req74_en0", &self.req74_en0())
            .field("req75_en0", &self.req75_en0())
            .field("req76_en0", &self.req76_en0())
            .field("req77_en0", &self.req77_en0())
            .field("req78_en0", &self.req78_en0())
            .field("req79_en0", &self.req79_en0())
            .field("req80_en0", &self.req80_en0())
            .field("req81_en0", &self.req81_en0())
            .field("req82_en0", &self.req82_en0())
            .field("req83_en0", &self.req83_en0())
            .field("req84_en0", &self.req84_en0())
            .field("req85_en0", &self.req85_en0())
            .field("req86_en0", &self.req86_en0())
            .field("req87_en0", &self.req87_en0())
            .field("req88_en0", &self.req88_en0())
            .field("req91_en0", &self.req91_en0())
            .field("req92_en0", &self.req92_en0())
            .field("req93_en0", &self.req93_en0())
            .field("req94_en0", &self.req94_en0())
            .field("req95_en0", &self.req95_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable2Clr {{ req64_en0: {=bool:?}, req65_en0: {=bool:?}, req66_en0: {=bool:?}, req67_en0: {=bool:?}, req68_en0: {=bool:?}, req69_en0: {=bool:?}, req70_en0: {=bool:?}, req71_en0: {=bool:?}, req72_en0: {=bool:?}, req73_en0: {=bool:?}, req74_en0: {=bool:?}, req75_en0: {=bool:?}, req76_en0: {=bool:?}, req77_en0: {=bool:?}, req78_en0: {=bool:?}, req79_en0: {=bool:?}, req80_en0: {=bool:?}, req81_en0: {=bool:?}, req82_en0: {=bool:?}, req83_en0: {=bool:?}, req84_en0: {=bool:?}, req85_en0: {=bool:?}, req86_en0: {=bool:?}, req87_en0: {=bool:?}, req88_en0: {=bool:?}, req91_en0: {=bool:?}, req92_en0: {=bool:?}, req93_en0: {=bool:?}, req94_en0: {=bool:?}, req95_en0: {=bool:?} }}",
            self.req64_en0(),
            self.req65_en0(),
            self.req66_en0(),
            self.req67_en0(),
            self.req68_en0(),
            self.req69_en0(),
            self.req70_en0(),
            self.req71_en0(),
            self.req72_en0(),
            self.req73_en0(),
            self.req74_en0(),
            self.req75_en0(),
            self.req76_en0(),
            self.req77_en0(),
            self.req78_en0(),
            self.req79_en0(),
            self.req80_en0(),
            self.req81_en0(),
            self.req82_en0(),
            self.req83_en0(),
            self.req84_en0(),
            self.req85_en0(),
            self.req86_en0(),
            self.req87_en0(),
            self.req88_en0(),
            self.req91_en0(),
            self.req92_en0(),
            self.req93_en0(),
            self.req94_en0(),
            self.req95_en0()
        )
    }
}
#[doc = "DMA0 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable2Set(pub u32);
impl Dma0ReqEnable2Set {
    #[doc = "Writing a 1 to REQ64_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ876_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ876_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable2Set {
    #[inline(always)]
    fn default() -> Dma0ReqEnable2Set {
        Dma0ReqEnable2Set(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable2Set")
            .field("req64_en0", &self.req64_en0())
            .field("req65_en0", &self.req65_en0())
            .field("req66_en0", &self.req66_en0())
            .field("req67_en0", &self.req67_en0())
            .field("req68_en0", &self.req68_en0())
            .field("req69_en0", &self.req69_en0())
            .field("req70_en0", &self.req70_en0())
            .field("req71_en0", &self.req71_en0())
            .field("req72_en0", &self.req72_en0())
            .field("req73_en0", &self.req73_en0())
            .field("req74_en0", &self.req74_en0())
            .field("req75_en0", &self.req75_en0())
            .field("req76_en0", &self.req76_en0())
            .field("req77_en0", &self.req77_en0())
            .field("req78_en0", &self.req78_en0())
            .field("req79_en0", &self.req79_en0())
            .field("req80_en0", &self.req80_en0())
            .field("req81_en0", &self.req81_en0())
            .field("req82_en0", &self.req82_en0())
            .field("req83_en0", &self.req83_en0())
            .field("req84_en0", &self.req84_en0())
            .field("req85_en0", &self.req85_en0())
            .field("req86_en0", &self.req86_en0())
            .field("req87_en0", &self.req87_en0())
            .field("req88_en0", &self.req88_en0())
            .field("req91_en0", &self.req91_en0())
            .field("req92_en0", &self.req92_en0())
            .field("req93_en0", &self.req93_en0())
            .field("req94_en0", &self.req94_en0())
            .field("req95_en0", &self.req95_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable2Set {{ req64_en0: {=bool:?}, req65_en0: {=bool:?}, req66_en0: {=bool:?}, req67_en0: {=bool:?}, req68_en0: {=bool:?}, req69_en0: {=bool:?}, req70_en0: {=bool:?}, req71_en0: {=bool:?}, req72_en0: {=bool:?}, req73_en0: {=bool:?}, req74_en0: {=bool:?}, req75_en0: {=bool:?}, req76_en0: {=bool:?}, req77_en0: {=bool:?}, req78_en0: {=bool:?}, req79_en0: {=bool:?}, req80_en0: {=bool:?}, req81_en0: {=bool:?}, req82_en0: {=bool:?}, req83_en0: {=bool:?}, req84_en0: {=bool:?}, req85_en0: {=bool:?}, req86_en0: {=bool:?}, req87_en0: {=bool:?}, req88_en0: {=bool:?}, req91_en0: {=bool:?}, req92_en0: {=bool:?}, req93_en0: {=bool:?}, req94_en0: {=bool:?}, req95_en0: {=bool:?} }}",
            self.req64_en0(),
            self.req65_en0(),
            self.req66_en0(),
            self.req67_en0(),
            self.req68_en0(),
            self.req69_en0(),
            self.req70_en0(),
            self.req71_en0(),
            self.req72_en0(),
            self.req73_en0(),
            self.req74_en0(),
            self.req75_en0(),
            self.req76_en0(),
            self.req77_en0(),
            self.req78_en0(),
            self.req79_en0(),
            self.req80_en0(),
            self.req81_en0(),
            self.req82_en0(),
            self.req83_en0(),
            self.req84_en0(),
            self.req85_en0(),
            self.req86_en0(),
            self.req87_en0(),
            self.req88_en0(),
            self.req91_en0(),
            self.req92_en0(),
            self.req93_en0(),
            self.req94_en0(),
            self.req95_en0()
        )
    }
}
#[doc = "DMA0 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable2Tog(pub u32);
impl Dma0ReqEnable2Tog {
    #[doc = "Writing a 1 to REQ64_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ76_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ76_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en0(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma0ReqEnable2Tog {
    #[inline(always)]
    fn default() -> Dma0ReqEnable2Tog {
        Dma0ReqEnable2Tog(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable2Tog")
            .field("req64_en0", &self.req64_en0())
            .field("req65_en0", &self.req65_en0())
            .field("req66_en0", &self.req66_en0())
            .field("req67_en0", &self.req67_en0())
            .field("req68_en0", &self.req68_en0())
            .field("req69_en0", &self.req69_en0())
            .field("req70_en0", &self.req70_en0())
            .field("req71_en0", &self.req71_en0())
            .field("req72_en0", &self.req72_en0())
            .field("req73_en0", &self.req73_en0())
            .field("req74_en0", &self.req74_en0())
            .field("req75_en0", &self.req75_en0())
            .field("req76_en0", &self.req76_en0())
            .field("req77_en0", &self.req77_en0())
            .field("req78_en0", &self.req78_en0())
            .field("req79_en0", &self.req79_en0())
            .field("req80_en0", &self.req80_en0())
            .field("req81_en0", &self.req81_en0())
            .field("req82_en0", &self.req82_en0())
            .field("req83_en0", &self.req83_en0())
            .field("req84_en0", &self.req84_en0())
            .field("req85_en0", &self.req85_en0())
            .field("req86_en0", &self.req86_en0())
            .field("req87_en0", &self.req87_en0())
            .field("req88_en0", &self.req88_en0())
            .field("req91_en0", &self.req91_en0())
            .field("req92_en0", &self.req92_en0())
            .field("req93_en0", &self.req93_en0())
            .field("req94_en0", &self.req94_en0())
            .field("req95_en0", &self.req95_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable2Tog {{ req64_en0: {=bool:?}, req65_en0: {=bool:?}, req66_en0: {=bool:?}, req67_en0: {=bool:?}, req68_en0: {=bool:?}, req69_en0: {=bool:?}, req70_en0: {=bool:?}, req71_en0: {=bool:?}, req72_en0: {=bool:?}, req73_en0: {=bool:?}, req74_en0: {=bool:?}, req75_en0: {=bool:?}, req76_en0: {=bool:?}, req77_en0: {=bool:?}, req78_en0: {=bool:?}, req79_en0: {=bool:?}, req80_en0: {=bool:?}, req81_en0: {=bool:?}, req82_en0: {=bool:?}, req83_en0: {=bool:?}, req84_en0: {=bool:?}, req85_en0: {=bool:?}, req86_en0: {=bool:?}, req87_en0: {=bool:?}, req88_en0: {=bool:?}, req91_en0: {=bool:?}, req92_en0: {=bool:?}, req93_en0: {=bool:?}, req94_en0: {=bool:?}, req95_en0: {=bool:?} }}",
            self.req64_en0(),
            self.req65_en0(),
            self.req66_en0(),
            self.req67_en0(),
            self.req68_en0(),
            self.req69_en0(),
            self.req70_en0(),
            self.req71_en0(),
            self.req72_en0(),
            self.req73_en0(),
            self.req74_en0(),
            self.req75_en0(),
            self.req76_en0(),
            self.req77_en0(),
            self.req78_en0(),
            self.req79_en0(),
            self.req80_en0(),
            self.req81_en0(),
            self.req82_en0(),
            self.req83_en0(),
            self.req84_en0(),
            self.req85_en0(),
            self.req86_en0(),
            self.req87_en0(),
            self.req88_en0(),
            self.req91_en0(),
            self.req92_en0(),
            self.req93_en0(),
            self.req94_en0(),
            self.req95_en0()
        )
    }
}
#[doc = "DMA0 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3(pub u32);
impl Dma0ReqEnable3 {
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en0(&self) -> super::vals::Req96En0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req96En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[inline(always)]
    pub const fn set_req96_en0(&mut self, val: super::vals::Req96En0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en0(&self) -> super::vals::Req97En0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req97En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[inline(always)]
    pub const fn set_req97_en0(&mut self, val: super::vals::Req97En0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en0(&self) -> super::vals::Req98En0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req98En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[inline(always)]
    pub const fn set_req98_en0(&mut self, val: super::vals::Req98En0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en0(&self) -> super::vals::Req99En0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req99En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[inline(always)]
    pub const fn set_req99_en0(&mut self, val: super::vals::Req99En0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en0(&self) -> super::vals::Req100En0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req100En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[inline(always)]
    pub const fn set_req100_en0(&mut self, val: super::vals::Req100En0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en0(&self) -> super::vals::Req101En0 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req101En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[inline(always)]
    pub const fn set_req101_en0(&mut self, val: super::vals::Req101En0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en0(&self) -> super::vals::Req102En0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req102En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[inline(always)]
    pub const fn set_req102_en0(&mut self, val: super::vals::Req102En0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en0(&self) -> super::vals::Req103En0 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req103En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[inline(always)]
    pub const fn set_req103_en0(&mut self, val: super::vals::Req103En0) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en0(&self) -> super::vals::Req104En0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req104En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[inline(always)]
    pub const fn set_req104_en0(&mut self, val: super::vals::Req104En0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en0(&self) -> super::vals::Req105En0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req105En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[inline(always)]
    pub const fn set_req105_en0(&mut self, val: super::vals::Req105En0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en0(&self) -> super::vals::Req106En0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req106En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[inline(always)]
    pub const fn set_req106_en0(&mut self, val: super::vals::Req106En0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en0(&self) -> super::vals::Req107En0 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req107En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[inline(always)]
    pub const fn set_req107_en0(&mut self, val: super::vals::Req107En0) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en0(&self) -> super::vals::Req108En0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req108En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[inline(always)]
    pub const fn set_req108_en0(&mut self, val: super::vals::Req108En0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en0(&self) -> super::vals::Req109En0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req109En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[inline(always)]
    pub const fn set_req109_en0(&mut self, val: super::vals::Req109En0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en0(&self) -> super::vals::Req110En0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req110En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[inline(always)]
    pub const fn set_req110_en0(&mut self, val: super::vals::Req110En0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en0(&self) -> super::vals::Req111En0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req111En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[inline(always)]
    pub const fn set_req111_en0(&mut self, val: super::vals::Req111En0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en0(&self) -> super::vals::Req112En0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req112En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[inline(always)]
    pub const fn set_req112_en0(&mut self, val: super::vals::Req112En0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en0(&self) -> super::vals::Req113En0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req113En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[inline(always)]
    pub const fn set_req113_en0(&mut self, val: super::vals::Req113En0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en0(&self) -> super::vals::Req114En0 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req114En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[inline(always)]
    pub const fn set_req114_en0(&mut self, val: super::vals::Req114En0) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en0(&self) -> super::vals::Req115En0 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req115En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[inline(always)]
    pub const fn set_req115_en0(&mut self, val: super::vals::Req115En0) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en0(&self) -> super::vals::Req116En0 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req116En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[inline(always)]
    pub const fn set_req116_en0(&mut self, val: super::vals::Req116En0) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en0(&self) -> super::vals::Req117En0 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req117En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[inline(always)]
    pub const fn set_req117_en0(&mut self, val: super::vals::Req117En0) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en0(&self) -> super::vals::Req118En0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req118En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[inline(always)]
    pub const fn set_req118_en0(&mut self, val: super::vals::Req118En0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en0(&self) -> super::vals::Req119En0 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req119En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[inline(always)]
    pub const fn set_req119_en0(&mut self, val: super::vals::Req119En0) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en0(&self) -> super::vals::Req120En0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req120En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[inline(always)]
    pub const fn set_req120_en0(&mut self, val: super::vals::Req120En0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en0(&self) -> super::vals::Req121En0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req121En0::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[inline(always)]
    pub const fn set_req121_en0(&mut self, val: super::vals::Req121En0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma0ReqEnable3 {
    #[inline(always)]
    fn default() -> Dma0ReqEnable3 {
        Dma0ReqEnable3(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable3")
            .field("req96_en0", &self.req96_en0())
            .field("req97_en0", &self.req97_en0())
            .field("req98_en0", &self.req98_en0())
            .field("req99_en0", &self.req99_en0())
            .field("req100_en0", &self.req100_en0())
            .field("req101_en0", &self.req101_en0())
            .field("req102_en0", &self.req102_en0())
            .field("req103_en0", &self.req103_en0())
            .field("req104_en0", &self.req104_en0())
            .field("req105_en0", &self.req105_en0())
            .field("req106_en0", &self.req106_en0())
            .field("req107_en0", &self.req107_en0())
            .field("req108_en0", &self.req108_en0())
            .field("req109_en0", &self.req109_en0())
            .field("req110_en0", &self.req110_en0())
            .field("req111_en0", &self.req111_en0())
            .field("req112_en0", &self.req112_en0())
            .field("req113_en0", &self.req113_en0())
            .field("req114_en0", &self.req114_en0())
            .field("req115_en0", &self.req115_en0())
            .field("req116_en0", &self.req116_en0())
            .field("req117_en0", &self.req117_en0())
            .field("req118_en0", &self.req118_en0())
            .field("req119_en0", &self.req119_en0())
            .field("req120_en0", &self.req120_en0())
            .field("req121_en0", &self.req121_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable3 {{ req96_en0: {:?}, req97_en0: {:?}, req98_en0: {:?}, req99_en0: {:?}, req100_en0: {:?}, req101_en0: {:?}, req102_en0: {:?}, req103_en0: {:?}, req104_en0: {:?}, req105_en0: {:?}, req106_en0: {:?}, req107_en0: {:?}, req108_en0: {:?}, req109_en0: {:?}, req110_en0: {:?}, req111_en0: {:?}, req112_en0: {:?}, req113_en0: {:?}, req114_en0: {:?}, req115_en0: {:?}, req116_en0: {:?}, req117_en0: {:?}, req118_en0: {:?}, req119_en0: {:?}, req120_en0: {:?}, req121_en0: {:?} }}",
            self.req96_en0(),
            self.req97_en0(),
            self.req98_en0(),
            self.req99_en0(),
            self.req100_en0(),
            self.req101_en0(),
            self.req102_en0(),
            self.req103_en0(),
            self.req104_en0(),
            self.req105_en0(),
            self.req106_en0(),
            self.req107_en0(),
            self.req108_en0(),
            self.req109_en0(),
            self.req110_en0(),
            self.req111_en0(),
            self.req112_en0(),
            self.req113_en0(),
            self.req114_en0(),
            self.req115_en0(),
            self.req116_en0(),
            self.req117_en0(),
            self.req118_en0(),
            self.req119_en0(),
            self.req120_en0(),
            self.req121_en0()
        )
    }
}
#[doc = "DMA0 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3Clr(pub u32);
impl Dma0ReqEnable3Clr {
    #[doc = "Writing a 1 to REQ96_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ96_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req96_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ97_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ97_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req97_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ98_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ98_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req98_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ99_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ99_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req99_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ100_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ100_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req100_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ101_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ101_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req101_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ102_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ102_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req102_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ103_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ103_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req103_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ104_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ104_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req104_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ105_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ105_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req105_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ106_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ106_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req106_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ107_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ107_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req107_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ108_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ108_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req108_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ109_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ109_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req109_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ110_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ110_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req110_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ111_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ111_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req111_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ112_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ112_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req112_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ113_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ113_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req113_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ114_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ114_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req114_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ115_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ115_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req115_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ116_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ116_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req116_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ117_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ117_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req117_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ118_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ118_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req118_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ119_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ119_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req119_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ120_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ120_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req120_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ121_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ121_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req121_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma0ReqEnable3Clr {
    #[inline(always)]
    fn default() -> Dma0ReqEnable3Clr {
        Dma0ReqEnable3Clr(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable3Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable3Clr")
            .field("req96_en0", &self.req96_en0())
            .field("req97_en0", &self.req97_en0())
            .field("req98_en0", &self.req98_en0())
            .field("req99_en0", &self.req99_en0())
            .field("req100_en0", &self.req100_en0())
            .field("req101_en0", &self.req101_en0())
            .field("req102_en0", &self.req102_en0())
            .field("req103_en0", &self.req103_en0())
            .field("req104_en0", &self.req104_en0())
            .field("req105_en0", &self.req105_en0())
            .field("req106_en0", &self.req106_en0())
            .field("req107_en0", &self.req107_en0())
            .field("req108_en0", &self.req108_en0())
            .field("req109_en0", &self.req109_en0())
            .field("req110_en0", &self.req110_en0())
            .field("req111_en0", &self.req111_en0())
            .field("req112_en0", &self.req112_en0())
            .field("req113_en0", &self.req113_en0())
            .field("req114_en0", &self.req114_en0())
            .field("req115_en0", &self.req115_en0())
            .field("req116_en0", &self.req116_en0())
            .field("req117_en0", &self.req117_en0())
            .field("req118_en0", &self.req118_en0())
            .field("req119_en0", &self.req119_en0())
            .field("req120_en0", &self.req120_en0())
            .field("req121_en0", &self.req121_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable3Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable3Clr {{ req96_en0: {=bool:?}, req97_en0: {=bool:?}, req98_en0: {=bool:?}, req99_en0: {=bool:?}, req100_en0: {=bool:?}, req101_en0: {=bool:?}, req102_en0: {=bool:?}, req103_en0: {=bool:?}, req104_en0: {=bool:?}, req105_en0: {=bool:?}, req106_en0: {=bool:?}, req107_en0: {=bool:?}, req108_en0: {=bool:?}, req109_en0: {=bool:?}, req110_en0: {=bool:?}, req111_en0: {=bool:?}, req112_en0: {=bool:?}, req113_en0: {=bool:?}, req114_en0: {=bool:?}, req115_en0: {=bool:?}, req116_en0: {=bool:?}, req117_en0: {=bool:?}, req118_en0: {=bool:?}, req119_en0: {=bool:?}, req120_en0: {=bool:?}, req121_en0: {=bool:?} }}",
            self.req96_en0(),
            self.req97_en0(),
            self.req98_en0(),
            self.req99_en0(),
            self.req100_en0(),
            self.req101_en0(),
            self.req102_en0(),
            self.req103_en0(),
            self.req104_en0(),
            self.req105_en0(),
            self.req106_en0(),
            self.req107_en0(),
            self.req108_en0(),
            self.req109_en0(),
            self.req110_en0(),
            self.req111_en0(),
            self.req112_en0(),
            self.req113_en0(),
            self.req114_en0(),
            self.req115_en0(),
            self.req116_en0(),
            self.req117_en0(),
            self.req118_en0(),
            self.req119_en0(),
            self.req120_en0(),
            self.req121_en0()
        )
    }
}
#[doc = "DMA0 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3Set(pub u32);
impl Dma0ReqEnable3Set {
    #[doc = "Writing a 1 to REQ96_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ96_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req96_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ97_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ97_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req97_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ98_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ98_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req98_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ99_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ99_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req99_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ100_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ100_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req100_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ101_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ101_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req101_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ102_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ102_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req102_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ103_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ103_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req103_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ104_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ104_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req104_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ105_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ105_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req105_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ106_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ106_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req106_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ107_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ107_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req107_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ108_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ108_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req108_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ109_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ109_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req109_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ110_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ110_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req110_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ111_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ111_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req111_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ112_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ112_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req112_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ113_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ113_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req113_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ114_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ114_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req114_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ115_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ115_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req115_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ116_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ116_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req116_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ117_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en0(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ117_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req117_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ118_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ118_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req118_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ119_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ119_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req119_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ120_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ120_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req120_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ121_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ121_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req121_en0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma0ReqEnable3Set {
    #[inline(always)]
    fn default() -> Dma0ReqEnable3Set {
        Dma0ReqEnable3Set(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnable3Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnable3Set")
            .field("req96_en0", &self.req96_en0())
            .field("req97_en0", &self.req97_en0())
            .field("req98_en0", &self.req98_en0())
            .field("req99_en0", &self.req99_en0())
            .field("req100_en0", &self.req100_en0())
            .field("req101_en0", &self.req101_en0())
            .field("req102_en0", &self.req102_en0())
            .field("req103_en0", &self.req103_en0())
            .field("req104_en0", &self.req104_en0())
            .field("req105_en0", &self.req105_en0())
            .field("req106_en0", &self.req106_en0())
            .field("req107_en0", &self.req107_en0())
            .field("req108_en0", &self.req108_en0())
            .field("req109_en0", &self.req109_en0())
            .field("req110_en0", &self.req110_en0())
            .field("req111_en0", &self.req111_en0())
            .field("req112_en0", &self.req112_en0())
            .field("req113_en0", &self.req113_en0())
            .field("req114_en0", &self.req114_en0())
            .field("req115_en0", &self.req115_en0())
            .field("req116_en0", &self.req116_en0())
            .field("req117_en0", &self.req117_en0())
            .field("req118_en0", &self.req118_en0())
            .field("req119_en0", &self.req119_en0())
            .field("req120_en0", &self.req120_en0())
            .field("req121_en0", &self.req121_en0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable3Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable3Set {{ req96_en0: {=bool:?}, req97_en0: {=bool:?}, req98_en0: {=bool:?}, req99_en0: {=bool:?}, req100_en0: {=bool:?}, req101_en0: {=bool:?}, req102_en0: {=bool:?}, req103_en0: {=bool:?}, req104_en0: {=bool:?}, req105_en0: {=bool:?}, req106_en0: {=bool:?}, req107_en0: {=bool:?}, req108_en0: {=bool:?}, req109_en0: {=bool:?}, req110_en0: {=bool:?}, req111_en0: {=bool:?}, req112_en0: {=bool:?}, req113_en0: {=bool:?}, req114_en0: {=bool:?}, req115_en0: {=bool:?}, req116_en0: {=bool:?}, req117_en0: {=bool:?}, req118_en0: {=bool:?}, req119_en0: {=bool:?}, req120_en0: {=bool:?}, req121_en0: {=bool:?} }}",
            self.req96_en0(),
            self.req97_en0(),
            self.req98_en0(),
            self.req99_en0(),
            self.req100_en0(),
            self.req101_en0(),
            self.req102_en0(),
            self.req103_en0(),
            self.req104_en0(),
            self.req105_en0(),
            self.req106_en0(),
            self.req107_en0(),
            self.req108_en0(),
            self.req109_en0(),
            self.req110_en0(),
            self.req111_en0(),
            self.req112_en0(),
            self.req113_en0(),
            self.req114_en0(),
            self.req115_en0(),
            self.req116_en0(),
            self.req117_en0(),
            self.req118_en0(),
            self.req119_en0(),
            self.req120_en0(),
            self.req121_en0()
        )
    }
}
#[doc = "DMA1 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0(pub u32);
impl Dma1ReqEnable0 {
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en1(&self) -> super::vals::Req1En1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req1En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[inline(always)]
    pub const fn set_req1_en1(&mut self, val: super::vals::Req1En1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en1(&self) -> super::vals::Req2En1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req2En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[inline(always)]
    pub const fn set_req2_en1(&mut self, val: super::vals::Req2En1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en1(&self) -> super::vals::Req3En1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req3En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[inline(always)]
    pub const fn set_req3_en1(&mut self, val: super::vals::Req3En1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en1(&self) -> super::vals::Req4En1 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req4En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[inline(always)]
    pub const fn set_req4_en1(&mut self, val: super::vals::Req4En1) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en1(&self) -> super::vals::Req5En1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req5En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[inline(always)]
    pub const fn set_req5_en1(&mut self, val: super::vals::Req5En1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en1(&self) -> super::vals::Req6En1 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req6En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[inline(always)]
    pub const fn set_req6_en1(&mut self, val: super::vals::Req6En1) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en1(&self) -> super::vals::Req7En1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req7En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req7_en1(&mut self, val: super::vals::Req7En1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en1(&self) -> super::vals::Req8En1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req8En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req8_en1(&mut self, val: super::vals::Req8En1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en1(&self) -> super::vals::Req9En1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req9En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req9_en1(&mut self, val: super::vals::Req9En1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en1(&self) -> super::vals::Req10En1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req10En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req10_en1(&mut self, val: super::vals::Req10En1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en1(&self) -> super::vals::Req11En1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req11En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req11_en1(&mut self, val: super::vals::Req11En1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en1(&self) -> super::vals::Req12En1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req12En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req12_en1(&mut self, val: super::vals::Req12En1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en1(&self) -> super::vals::Req13En1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req13En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req13_en1(&mut self, val: super::vals::Req13En1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en1(&self) -> super::vals::Req14En1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req14En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req14_en1(&mut self, val: super::vals::Req14En1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en1(&self) -> super::vals::Req15En1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req15En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req15_en1(&mut self, val: super::vals::Req15En1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en1(&self) -> super::vals::Req16En1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req16En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req16_en1(&mut self, val: super::vals::Req16En1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en1(&self) -> super::vals::Req17En1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req17En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[inline(always)]
    pub const fn set_req17_en1(&mut self, val: super::vals::Req17En1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en1(&self) -> super::vals::Req18En1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req18En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req18_en1(&mut self, val: super::vals::Req18En1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en1(&self) -> super::vals::Req19En1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req19En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[inline(always)]
    pub const fn set_req19_en1(&mut self, val: super::vals::Req19En1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en1(&self) -> super::vals::Req20En1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req20En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[inline(always)]
    pub const fn set_req20_en1(&mut self, val: super::vals::Req20En1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en1(&self) -> super::vals::Req21En1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req21En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[inline(always)]
    pub const fn set_req21_en1(&mut self, val: super::vals::Req21En1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en1(&self) -> super::vals::Req22En1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req22En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[inline(always)]
    pub const fn set_req22_en1(&mut self, val: super::vals::Req22En1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en1(&self) -> super::vals::Req23En1 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req23En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[inline(always)]
    pub const fn set_req23_en1(&mut self, val: super::vals::Req23En1) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en1(&self) -> super::vals::Req24En1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req24En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[inline(always)]
    pub const fn set_req24_en1(&mut self, val: super::vals::Req24En1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en1(&self) -> super::vals::Req25En1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req25En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req25_en1(&mut self, val: super::vals::Req25En1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en1(&self) -> super::vals::Req26En1 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Req26En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[inline(always)]
    pub const fn set_req26_en1(&mut self, val: super::vals::Req26En1) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en1(&self) -> super::vals::Req27En1 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req27En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[inline(always)]
    pub const fn set_req27_en1(&mut self, val: super::vals::Req27En1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en1(&self) -> super::vals::Req28En1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req28En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[inline(always)]
    pub const fn set_req28_en1(&mut self, val: super::vals::Req28En1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en1(&self) -> super::vals::Req29En1 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req29En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[inline(always)]
    pub const fn set_req29_en1(&mut self, val: super::vals::Req29En1) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en1(&self) -> super::vals::Req30En1 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req30En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[inline(always)]
    pub const fn set_req30_en1(&mut self, val: super::vals::Req30En1) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en1(&self) -> super::vals::Req31En1 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req31En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[inline(always)]
    pub const fn set_req31_en1(&mut self, val: super::vals::Req31En1) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable0 {
    #[inline(always)]
    fn default() -> Dma1ReqEnable0 {
        Dma1ReqEnable0(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable0")
            .field("req1_en1", &self.req1_en1())
            .field("req2_en1", &self.req2_en1())
            .field("req3_en1", &self.req3_en1())
            .field("req4_en1", &self.req4_en1())
            .field("req5_en1", &self.req5_en1())
            .field("req6_en1", &self.req6_en1())
            .field("req7_en1", &self.req7_en1())
            .field("req8_en1", &self.req8_en1())
            .field("req9_en1", &self.req9_en1())
            .field("req10_en1", &self.req10_en1())
            .field("req11_en1", &self.req11_en1())
            .field("req12_en1", &self.req12_en1())
            .field("req13_en1", &self.req13_en1())
            .field("req14_en1", &self.req14_en1())
            .field("req15_en1", &self.req15_en1())
            .field("req16_en1", &self.req16_en1())
            .field("req17_en1", &self.req17_en1())
            .field("req18_en1", &self.req18_en1())
            .field("req19_en1", &self.req19_en1())
            .field("req20_en1", &self.req20_en1())
            .field("req21_en1", &self.req21_en1())
            .field("req22_en1", &self.req22_en1())
            .field("req23_en1", &self.req23_en1())
            .field("req24_en1", &self.req24_en1())
            .field("req25_en1", &self.req25_en1())
            .field("req26_en1", &self.req26_en1())
            .field("req27_en1", &self.req27_en1())
            .field("req28_en1", &self.req28_en1())
            .field("req29_en1", &self.req29_en1())
            .field("req30_en1", &self.req30_en1())
            .field("req31_en1", &self.req31_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0 {{ req1_en1: {:?}, req2_en1: {:?}, req3_en1: {:?}, req4_en1: {:?}, req5_en1: {:?}, req6_en1: {:?}, req7_en1: {:?}, req8_en1: {:?}, req9_en1: {:?}, req10_en1: {:?}, req11_en1: {:?}, req12_en1: {:?}, req13_en1: {:?}, req14_en1: {:?}, req15_en1: {:?}, req16_en1: {:?}, req17_en1: {:?}, req18_en1: {:?}, req19_en1: {:?}, req20_en1: {:?}, req21_en1: {:?}, req22_en1: {:?}, req23_en1: {:?}, req24_en1: {:?}, req25_en1: {:?}, req26_en1: {:?}, req27_en1: {:?}, req28_en1: {:?}, req29_en1: {:?}, req30_en1: {:?}, req31_en1: {:?} }}",
            self.req1_en1(),
            self.req2_en1(),
            self.req3_en1(),
            self.req4_en1(),
            self.req5_en1(),
            self.req6_en1(),
            self.req7_en1(),
            self.req8_en1(),
            self.req9_en1(),
            self.req10_en1(),
            self.req11_en1(),
            self.req12_en1(),
            self.req13_en1(),
            self.req14_en1(),
            self.req15_en1(),
            self.req16_en1(),
            self.req17_en1(),
            self.req18_en1(),
            self.req19_en1(),
            self.req20_en1(),
            self.req21_en1(),
            self.req22_en1(),
            self.req23_en1(),
            self.req24_en1(),
            self.req25_en1(),
            self.req26_en1(),
            self.req27_en1(),
            self.req28_en1(),
            self.req29_en1(),
            self.req30_en1(),
            self.req31_en1()
        )
    }
}
#[doc = "DMA1 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Clr(pub u32);
impl Dma1ReqEnable0Clr {
    #[doc = "Writing a 1 to REQ1_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ9_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ9_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable0Clr {
    #[inline(always)]
    fn default() -> Dma1ReqEnable0Clr {
        Dma1ReqEnable0Clr(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable0Clr")
            .field("req1_en1", &self.req1_en1())
            .field("req2_en1", &self.req2_en1())
            .field("req3_en1", &self.req3_en1())
            .field("req4_en1", &self.req4_en1())
            .field("req5_en1", &self.req5_en1())
            .field("req6_en1", &self.req6_en1())
            .field("req7_en1", &self.req7_en1())
            .field("req8_en1", &self.req8_en1())
            .field("req9_en1", &self.req9_en1())
            .field("req10_en1", &self.req10_en1())
            .field("req11_en1", &self.req11_en1())
            .field("req12_en1", &self.req12_en1())
            .field("req13_en1", &self.req13_en1())
            .field("req14_en1", &self.req14_en1())
            .field("req15_en1", &self.req15_en1())
            .field("req16_en1", &self.req16_en1())
            .field("req17_en1", &self.req17_en1())
            .field("req18_en1", &self.req18_en1())
            .field("req19_en1", &self.req19_en1())
            .field("req20_en1", &self.req20_en1())
            .field("req21_en1", &self.req21_en1())
            .field("req22_en1", &self.req22_en1())
            .field("req23_en1", &self.req23_en1())
            .field("req24_en1", &self.req24_en1())
            .field("req25_en1", &self.req25_en1())
            .field("req26_en1", &self.req26_en1())
            .field("req27_en1", &self.req27_en1())
            .field("req28_en1", &self.req28_en1())
            .field("req29_en1", &self.req29_en1())
            .field("req30_en1", &self.req30_en1())
            .field("req31_en1", &self.req31_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Clr {{ req1_en1: {=bool:?}, req2_en1: {=bool:?}, req3_en1: {=bool:?}, req4_en1: {=bool:?}, req5_en1: {=bool:?}, req6_en1: {=bool:?}, req7_en1: {=bool:?}, req8_en1: {=bool:?}, req9_en1: {=bool:?}, req10_en1: {=bool:?}, req11_en1: {=bool:?}, req12_en1: {=bool:?}, req13_en1: {=bool:?}, req14_en1: {=bool:?}, req15_en1: {=bool:?}, req16_en1: {=bool:?}, req17_en1: {=bool:?}, req18_en1: {=bool:?}, req19_en1: {=bool:?}, req20_en1: {=bool:?}, req21_en1: {=bool:?}, req22_en1: {=bool:?}, req23_en1: {=bool:?}, req24_en1: {=bool:?}, req25_en1: {=bool:?}, req26_en1: {=bool:?}, req27_en1: {=bool:?}, req28_en1: {=bool:?}, req29_en1: {=bool:?}, req30_en1: {=bool:?}, req31_en1: {=bool:?} }}",
            self.req1_en1(),
            self.req2_en1(),
            self.req3_en1(),
            self.req4_en1(),
            self.req5_en1(),
            self.req6_en1(),
            self.req7_en1(),
            self.req8_en1(),
            self.req9_en1(),
            self.req10_en1(),
            self.req11_en1(),
            self.req12_en1(),
            self.req13_en1(),
            self.req14_en1(),
            self.req15_en1(),
            self.req16_en1(),
            self.req17_en1(),
            self.req18_en1(),
            self.req19_en1(),
            self.req20_en1(),
            self.req21_en1(),
            self.req22_en1(),
            self.req23_en1(),
            self.req24_en1(),
            self.req25_en1(),
            self.req26_en1(),
            self.req27_en1(),
            self.req28_en1(),
            self.req29_en1(),
            self.req30_en1(),
            self.req31_en1()
        )
    }
}
#[doc = "DMA1 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Set(pub u32);
impl Dma1ReqEnable0Set {
    #[doc = "Writing a 1 to REQ1_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ9_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ9_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable0Set {
    #[inline(always)]
    fn default() -> Dma1ReqEnable0Set {
        Dma1ReqEnable0Set(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable0Set")
            .field("req1_en1", &self.req1_en1())
            .field("req2_en1", &self.req2_en1())
            .field("req3_en1", &self.req3_en1())
            .field("req4_en1", &self.req4_en1())
            .field("req5_en1", &self.req5_en1())
            .field("req6_en1", &self.req6_en1())
            .field("req7_en1", &self.req7_en1())
            .field("req8_en1", &self.req8_en1())
            .field("req9_en1", &self.req9_en1())
            .field("req10_en1", &self.req10_en1())
            .field("req11_en1", &self.req11_en1())
            .field("req12_en1", &self.req12_en1())
            .field("req13_en1", &self.req13_en1())
            .field("req14_en1", &self.req14_en1())
            .field("req15_en1", &self.req15_en1())
            .field("req16_en1", &self.req16_en1())
            .field("req17_en1", &self.req17_en1())
            .field("req18_en1", &self.req18_en1())
            .field("req19_en1", &self.req19_en1())
            .field("req20_en1", &self.req20_en1())
            .field("req21_en1", &self.req21_en1())
            .field("req22_en1", &self.req22_en1())
            .field("req23_en1", &self.req23_en1())
            .field("req24_en1", &self.req24_en1())
            .field("req25_en1", &self.req25_en1())
            .field("req26_en1", &self.req26_en1())
            .field("req27_en1", &self.req27_en1())
            .field("req28_en1", &self.req28_en1())
            .field("req29_en1", &self.req29_en1())
            .field("req30_en1", &self.req30_en1())
            .field("req31_en1", &self.req31_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Set {{ req1_en1: {=bool:?}, req2_en1: {=bool:?}, req3_en1: {=bool:?}, req4_en1: {=bool:?}, req5_en1: {=bool:?}, req6_en1: {=bool:?}, req7_en1: {=bool:?}, req8_en1: {=bool:?}, req9_en1: {=bool:?}, req10_en1: {=bool:?}, req11_en1: {=bool:?}, req12_en1: {=bool:?}, req13_en1: {=bool:?}, req14_en1: {=bool:?}, req15_en1: {=bool:?}, req16_en1: {=bool:?}, req17_en1: {=bool:?}, req18_en1: {=bool:?}, req19_en1: {=bool:?}, req20_en1: {=bool:?}, req21_en1: {=bool:?}, req22_en1: {=bool:?}, req23_en1: {=bool:?}, req24_en1: {=bool:?}, req25_en1: {=bool:?}, req26_en1: {=bool:?}, req27_en1: {=bool:?}, req28_en1: {=bool:?}, req29_en1: {=bool:?}, req30_en1: {=bool:?}, req31_en1: {=bool:?} }}",
            self.req1_en1(),
            self.req2_en1(),
            self.req3_en1(),
            self.req4_en1(),
            self.req5_en1(),
            self.req6_en1(),
            self.req7_en1(),
            self.req8_en1(),
            self.req9_en1(),
            self.req10_en1(),
            self.req11_en1(),
            self.req12_en1(),
            self.req13_en1(),
            self.req14_en1(),
            self.req15_en1(),
            self.req16_en1(),
            self.req17_en1(),
            self.req18_en1(),
            self.req19_en1(),
            self.req20_en1(),
            self.req21_en1(),
            self.req22_en1(),
            self.req23_en1(),
            self.req24_en1(),
            self.req25_en1(),
            self.req26_en1(),
            self.req27_en1(),
            self.req28_en1(),
            self.req29_en1(),
            self.req30_en1(),
            self.req31_en1()
        )
    }
}
#[doc = "DMA1 Request Enable0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Tog(pub u32);
impl Dma1ReqEnable0Tog {
    #[doc = "Writing a 1 to REQ1_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ1_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req1_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ2_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req2_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ3_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req3_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ4_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req4_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ5_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req5_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ6_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req6_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ7_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req7_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ8_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req8_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to RE9_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to RE9_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req9_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ10_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req10_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ11_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req11_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ12_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req12_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ13_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req13_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ14_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req14_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ15_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req15_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ16_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req16_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ17_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req17_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ18_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req18_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ19_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req19_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ20_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req20_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ21_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req21_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ22_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req22_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ23_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req23_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ24_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req24_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ25_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req25_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ26_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req26_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ27_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req27_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ28_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req28_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ29_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req29_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ30_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req30_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ31_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req31_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable0Tog {
    #[inline(always)]
    fn default() -> Dma1ReqEnable0Tog {
        Dma1ReqEnable0Tog(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable0Tog")
            .field("req1_en1", &self.req1_en1())
            .field("req2_en1", &self.req2_en1())
            .field("req3_en1", &self.req3_en1())
            .field("req4_en1", &self.req4_en1())
            .field("req5_en1", &self.req5_en1())
            .field("req6_en1", &self.req6_en1())
            .field("req7_en1", &self.req7_en1())
            .field("req8_en1", &self.req8_en1())
            .field("req9_en1", &self.req9_en1())
            .field("req10_en1", &self.req10_en1())
            .field("req11_en1", &self.req11_en1())
            .field("req12_en1", &self.req12_en1())
            .field("req13_en1", &self.req13_en1())
            .field("req14_en1", &self.req14_en1())
            .field("req15_en1", &self.req15_en1())
            .field("req16_en1", &self.req16_en1())
            .field("req17_en1", &self.req17_en1())
            .field("req18_en1", &self.req18_en1())
            .field("req19_en1", &self.req19_en1())
            .field("req20_en1", &self.req20_en1())
            .field("req21_en1", &self.req21_en1())
            .field("req22_en1", &self.req22_en1())
            .field("req23_en1", &self.req23_en1())
            .field("req24_en1", &self.req24_en1())
            .field("req25_en1", &self.req25_en1())
            .field("req26_en1", &self.req26_en1())
            .field("req27_en1", &self.req27_en1())
            .field("req28_en1", &self.req28_en1())
            .field("req29_en1", &self.req29_en1())
            .field("req30_en1", &self.req30_en1())
            .field("req31_en1", &self.req31_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Tog {{ req1_en1: {=bool:?}, req2_en1: {=bool:?}, req3_en1: {=bool:?}, req4_en1: {=bool:?}, req5_en1: {=bool:?}, req6_en1: {=bool:?}, req7_en1: {=bool:?}, req8_en1: {=bool:?}, req9_en1: {=bool:?}, req10_en1: {=bool:?}, req11_en1: {=bool:?}, req12_en1: {=bool:?}, req13_en1: {=bool:?}, req14_en1: {=bool:?}, req15_en1: {=bool:?}, req16_en1: {=bool:?}, req17_en1: {=bool:?}, req18_en1: {=bool:?}, req19_en1: {=bool:?}, req20_en1: {=bool:?}, req21_en1: {=bool:?}, req22_en1: {=bool:?}, req23_en1: {=bool:?}, req24_en1: {=bool:?}, req25_en1: {=bool:?}, req26_en1: {=bool:?}, req27_en1: {=bool:?}, req28_en1: {=bool:?}, req29_en1: {=bool:?}, req30_en1: {=bool:?}, req31_en1: {=bool:?} }}",
            self.req1_en1(),
            self.req2_en1(),
            self.req3_en1(),
            self.req4_en1(),
            self.req5_en1(),
            self.req6_en1(),
            self.req7_en1(),
            self.req8_en1(),
            self.req9_en1(),
            self.req10_en1(),
            self.req11_en1(),
            self.req12_en1(),
            self.req13_en1(),
            self.req14_en1(),
            self.req15_en1(),
            self.req16_en1(),
            self.req17_en1(),
            self.req18_en1(),
            self.req19_en1(),
            self.req20_en1(),
            self.req21_en1(),
            self.req22_en1(),
            self.req23_en1(),
            self.req24_en1(),
            self.req25_en1(),
            self.req26_en1(),
            self.req27_en1(),
            self.req28_en1(),
            self.req29_en1(),
            self.req30_en1(),
            self.req31_en1()
        )
    }
}
#[doc = "DMA1 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable1(pub u32);
impl Dma1ReqEnable1 {
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en1(&self) -> super::vals::Req32En1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req32En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[inline(always)]
    pub const fn set_req32_en1(&mut self, val: super::vals::Req32En1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en1(&self) -> super::vals::Req33En1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req33En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[inline(always)]
    pub const fn set_req33_en1(&mut self, val: super::vals::Req33En1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en1(&self) -> super::vals::Req34En1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req34En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[inline(always)]
    pub const fn set_req34_en1(&mut self, val: super::vals::Req34En1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en1(&self) -> super::vals::Req35En1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req35En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[inline(always)]
    pub const fn set_req35_en1(&mut self, val: super::vals::Req35En1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en1(&self) -> super::vals::Req36En1 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req36En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[inline(always)]
    pub const fn set_req36_en1(&mut self, val: super::vals::Req36En1) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en1(&self) -> super::vals::Req37En1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req37En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[inline(always)]
    pub const fn set_req37_en1(&mut self, val: super::vals::Req37En1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en1(&self) -> super::vals::Req38En1 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req38En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[inline(always)]
    pub const fn set_req38_en1(&mut self, val: super::vals::Req38En1) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en1(&self) -> super::vals::Req39En1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req39En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req39_en1(&mut self, val: super::vals::Req39En1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en1(&self) -> super::vals::Req40En1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req40En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req40_en1(&mut self, val: super::vals::Req40En1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en1(&self) -> super::vals::Req41En1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req41En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req41_en1(&mut self, val: super::vals::Req41En1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en1(&self) -> super::vals::Req42En1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req42En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req42_en1(&mut self, val: super::vals::Req42En1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en1(&self) -> super::vals::Req43En1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req43En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req43_en1(&mut self, val: super::vals::Req43En1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en1(&self) -> super::vals::Req44En1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req44En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req44_en1(&mut self, val: super::vals::Req44En1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en1(&self) -> super::vals::Req45En1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req45En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req45_en1(&mut self, val: super::vals::Req45En1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en1(&self) -> super::vals::Req46En1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req46En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req46_en1(&mut self, val: super::vals::Req46En1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en1(&self) -> super::vals::Req47En1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req47En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req47_en1(&mut self, val: super::vals::Req47En1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en1(&self) -> super::vals::Req48En1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req48En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req48_en1(&mut self, val: super::vals::Req48En1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en1(&self) -> super::vals::Req49En1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req49En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req49_en1(&mut self, val: super::vals::Req49En1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en1(&self) -> super::vals::Req50En1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req50En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req50_en1(&mut self, val: super::vals::Req50En1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en1(&self) -> super::vals::Req51En1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req51En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req51_en1(&mut self, val: super::vals::Req51En1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en1(&self) -> super::vals::Req52En1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req52En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req52_en1(&mut self, val: super::vals::Req52En1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en1(&self) -> super::vals::Req53En1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req53En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req53_en1(&mut self, val: super::vals::Req53En1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en1(&self) -> super::vals::Req54En1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req54En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req54_en1(&mut self, val: super::vals::Req54En1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en1(&self) -> super::vals::Req57En1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req57En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[inline(always)]
    pub const fn set_req57_en1(&mut self, val: super::vals::Req57En1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en1(&self) -> super::vals::Req58En1 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Req58En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[inline(always)]
    pub const fn set_req58_en1(&mut self, val: super::vals::Req58En1) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en1(&self) -> super::vals::Req59En1 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req59En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[inline(always)]
    pub const fn set_req59_en1(&mut self, val: super::vals::Req59En1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en1(&self) -> super::vals::Req60En1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req60En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[inline(always)]
    pub const fn set_req60_en1(&mut self, val: super::vals::Req60En1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en1(&self) -> super::vals::Req61En1 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req61En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[inline(always)]
    pub const fn set_req61_en1(&mut self, val: super::vals::Req61En1) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en1(&self) -> super::vals::Req62En1 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req62En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[inline(always)]
    pub const fn set_req62_en1(&mut self, val: super::vals::Req62En1) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en1(&self) -> super::vals::Req63En1 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req63En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[inline(always)]
    pub const fn set_req63_en1(&mut self, val: super::vals::Req63En1) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable1 {
    #[inline(always)]
    fn default() -> Dma1ReqEnable1 {
        Dma1ReqEnable1(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable1")
            .field("req32_en1", &self.req32_en1())
            .field("req33_en1", &self.req33_en1())
            .field("req34_en1", &self.req34_en1())
            .field("req35_en1", &self.req35_en1())
            .field("req36_en1", &self.req36_en1())
            .field("req37_en1", &self.req37_en1())
            .field("req38_en1", &self.req38_en1())
            .field("req39_en1", &self.req39_en1())
            .field("req40_en1", &self.req40_en1())
            .field("req41_en1", &self.req41_en1())
            .field("req42_en1", &self.req42_en1())
            .field("req43_en1", &self.req43_en1())
            .field("req44_en1", &self.req44_en1())
            .field("req45_en1", &self.req45_en1())
            .field("req46_en1", &self.req46_en1())
            .field("req47_en1", &self.req47_en1())
            .field("req48_en1", &self.req48_en1())
            .field("req49_en1", &self.req49_en1())
            .field("req50_en1", &self.req50_en1())
            .field("req51_en1", &self.req51_en1())
            .field("req52_en1", &self.req52_en1())
            .field("req53_en1", &self.req53_en1())
            .field("req54_en1", &self.req54_en1())
            .field("req57_en1", &self.req57_en1())
            .field("req58_en1", &self.req58_en1())
            .field("req59_en1", &self.req59_en1())
            .field("req60_en1", &self.req60_en1())
            .field("req61_en1", &self.req61_en1())
            .field("req62_en1", &self.req62_en1())
            .field("req63_en1", &self.req63_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable1 {{ req32_en1: {:?}, req33_en1: {:?}, req34_en1: {:?}, req35_en1: {:?}, req36_en1: {:?}, req37_en1: {:?}, req38_en1: {:?}, req39_en1: {:?}, req40_en1: {:?}, req41_en1: {:?}, req42_en1: {:?}, req43_en1: {:?}, req44_en1: {:?}, req45_en1: {:?}, req46_en1: {:?}, req47_en1: {:?}, req48_en1: {:?}, req49_en1: {:?}, req50_en1: {:?}, req51_en1: {:?}, req52_en1: {:?}, req53_en1: {:?}, req54_en1: {:?}, req57_en1: {:?}, req58_en1: {:?}, req59_en1: {:?}, req60_en1: {:?}, req61_en1: {:?}, req62_en1: {:?}, req63_en1: {:?} }}",
            self.req32_en1(),
            self.req33_en1(),
            self.req34_en1(),
            self.req35_en1(),
            self.req36_en1(),
            self.req37_en1(),
            self.req38_en1(),
            self.req39_en1(),
            self.req40_en1(),
            self.req41_en1(),
            self.req42_en1(),
            self.req43_en1(),
            self.req44_en1(),
            self.req45_en1(),
            self.req46_en1(),
            self.req47_en1(),
            self.req48_en1(),
            self.req49_en1(),
            self.req50_en1(),
            self.req51_en1(),
            self.req52_en1(),
            self.req53_en1(),
            self.req54_en1(),
            self.req57_en1(),
            self.req58_en1(),
            self.req59_en1(),
            self.req60_en1(),
            self.req61_en1(),
            self.req62_en1(),
            self.req63_en1()
        )
    }
}
#[doc = "DMA1 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable1Clr(pub u32);
impl Dma1ReqEnable1Clr {
    #[doc = "Writing a 1 to REQ32_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ45_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ45_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable1Clr {
    #[inline(always)]
    fn default() -> Dma1ReqEnable1Clr {
        Dma1ReqEnable1Clr(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable1Clr")
            .field("req32_en1", &self.req32_en1())
            .field("req33_en1", &self.req33_en1())
            .field("req34_en1", &self.req34_en1())
            .field("req35_en1", &self.req35_en1())
            .field("req36_en1", &self.req36_en1())
            .field("req37_en1", &self.req37_en1())
            .field("req38_en1", &self.req38_en1())
            .field("req39_en1", &self.req39_en1())
            .field("req40_en1", &self.req40_en1())
            .field("req41_en1", &self.req41_en1())
            .field("req42_en1", &self.req42_en1())
            .field("req43_en1", &self.req43_en1())
            .field("req44_en1", &self.req44_en1())
            .field("req45_en1", &self.req45_en1())
            .field("req46_en1", &self.req46_en1())
            .field("req47_en1", &self.req47_en1())
            .field("req48_en1", &self.req48_en1())
            .field("req49_en1", &self.req49_en1())
            .field("req50_en1", &self.req50_en1())
            .field("req51_en1", &self.req51_en1())
            .field("req52_en1", &self.req52_en1())
            .field("req53_en1", &self.req53_en1())
            .field("req54_en1", &self.req54_en1())
            .field("req57_en1", &self.req57_en1())
            .field("req58_en1", &self.req58_en1())
            .field("req59_en1", &self.req59_en1())
            .field("req60_en1", &self.req60_en1())
            .field("req61_en1", &self.req61_en1())
            .field("req62_en1", &self.req62_en1())
            .field("req63_en1", &self.req63_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable1Clr {{ req32_en1: {=bool:?}, req33_en1: {=bool:?}, req34_en1: {=bool:?}, req35_en1: {=bool:?}, req36_en1: {=bool:?}, req37_en1: {=bool:?}, req38_en1: {=bool:?}, req39_en1: {=bool:?}, req40_en1: {=bool:?}, req41_en1: {=bool:?}, req42_en1: {=bool:?}, req43_en1: {=bool:?}, req44_en1: {=bool:?}, req45_en1: {=bool:?}, req46_en1: {=bool:?}, req47_en1: {=bool:?}, req48_en1: {=bool:?}, req49_en1: {=bool:?}, req50_en1: {=bool:?}, req51_en1: {=bool:?}, req52_en1: {=bool:?}, req53_en1: {=bool:?}, req54_en1: {=bool:?}, req57_en1: {=bool:?}, req58_en1: {=bool:?}, req59_en1: {=bool:?}, req60_en1: {=bool:?}, req61_en1: {=bool:?}, req62_en1: {=bool:?}, req63_en1: {=bool:?} }}",
            self.req32_en1(),
            self.req33_en1(),
            self.req34_en1(),
            self.req35_en1(),
            self.req36_en1(),
            self.req37_en1(),
            self.req38_en1(),
            self.req39_en1(),
            self.req40_en1(),
            self.req41_en1(),
            self.req42_en1(),
            self.req43_en1(),
            self.req44_en1(),
            self.req45_en1(),
            self.req46_en1(),
            self.req47_en1(),
            self.req48_en1(),
            self.req49_en1(),
            self.req50_en1(),
            self.req51_en1(),
            self.req52_en1(),
            self.req53_en1(),
            self.req54_en1(),
            self.req57_en1(),
            self.req58_en1(),
            self.req59_en1(),
            self.req60_en1(),
            self.req61_en1(),
            self.req62_en1(),
            self.req63_en1()
        )
    }
}
#[doc = "DMA1 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable1Set(pub u32);
impl Dma1ReqEnable1Set {
    #[doc = "Writing a 1 to REQ32_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ45_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ45_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable1Set {
    #[inline(always)]
    fn default() -> Dma1ReqEnable1Set {
        Dma1ReqEnable1Set(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable1Set")
            .field("req32_en1", &self.req32_en1())
            .field("req33_en1", &self.req33_en1())
            .field("req34_en1", &self.req34_en1())
            .field("req35_en1", &self.req35_en1())
            .field("req36_en1", &self.req36_en1())
            .field("req37_en1", &self.req37_en1())
            .field("req38_en1", &self.req38_en1())
            .field("req39_en1", &self.req39_en1())
            .field("req40_en1", &self.req40_en1())
            .field("req41_en1", &self.req41_en1())
            .field("req42_en1", &self.req42_en1())
            .field("req43_en1", &self.req43_en1())
            .field("req44_en1", &self.req44_en1())
            .field("req45_en1", &self.req45_en1())
            .field("req46_en1", &self.req46_en1())
            .field("req47_en1", &self.req47_en1())
            .field("req48_en1", &self.req48_en1())
            .field("req49_en1", &self.req49_en1())
            .field("req50_en1", &self.req50_en1())
            .field("req51_en1", &self.req51_en1())
            .field("req52_en1", &self.req52_en1())
            .field("req53_en1", &self.req53_en1())
            .field("req54_en1", &self.req54_en1())
            .field("req57_en1", &self.req57_en1())
            .field("req58_en1", &self.req58_en1())
            .field("req59_en1", &self.req59_en1())
            .field("req60_en1", &self.req60_en1())
            .field("req61_en1", &self.req61_en1())
            .field("req62_en1", &self.req62_en1())
            .field("req63_en1", &self.req63_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable1Set {{ req32_en1: {=bool:?}, req33_en1: {=bool:?}, req34_en1: {=bool:?}, req35_en1: {=bool:?}, req36_en1: {=bool:?}, req37_en1: {=bool:?}, req38_en1: {=bool:?}, req39_en1: {=bool:?}, req40_en1: {=bool:?}, req41_en1: {=bool:?}, req42_en1: {=bool:?}, req43_en1: {=bool:?}, req44_en1: {=bool:?}, req45_en1: {=bool:?}, req46_en1: {=bool:?}, req47_en1: {=bool:?}, req48_en1: {=bool:?}, req49_en1: {=bool:?}, req50_en1: {=bool:?}, req51_en1: {=bool:?}, req52_en1: {=bool:?}, req53_en1: {=bool:?}, req54_en1: {=bool:?}, req57_en1: {=bool:?}, req58_en1: {=bool:?}, req59_en1: {=bool:?}, req60_en1: {=bool:?}, req61_en1: {=bool:?}, req62_en1: {=bool:?}, req63_en1: {=bool:?} }}",
            self.req32_en1(),
            self.req33_en1(),
            self.req34_en1(),
            self.req35_en1(),
            self.req36_en1(),
            self.req37_en1(),
            self.req38_en1(),
            self.req39_en1(),
            self.req40_en1(),
            self.req41_en1(),
            self.req42_en1(),
            self.req43_en1(),
            self.req44_en1(),
            self.req45_en1(),
            self.req46_en1(),
            self.req47_en1(),
            self.req48_en1(),
            self.req49_en1(),
            self.req50_en1(),
            self.req51_en1(),
            self.req52_en1(),
            self.req53_en1(),
            self.req54_en1(),
            self.req57_en1(),
            self.req58_en1(),
            self.req59_en1(),
            self.req60_en1(),
            self.req61_en1(),
            self.req62_en1(),
            self.req63_en1()
        )
    }
}
#[doc = "DMA1 Request Enable1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable1Tog(pub u32);
impl Dma1ReqEnable1Tog {
    #[doc = "Writing a 1 to REQ32_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ32_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req32_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ33_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req33_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ34_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req34_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ35_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req35_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ36_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req36_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ37_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req37_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ38_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req38_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ39_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req39_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ40_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req40_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ41_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req41_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ42_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req42_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ43_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req43_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ44_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req44_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ55_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ55_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req45_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ46_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req46_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ47_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req47_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ48_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req48_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ49_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req49_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ50_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req50_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ51_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req51_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ52_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req52_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ53_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req53_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ54_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req54_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ57_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req57_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ58_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req58_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ59_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req59_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ60_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req60_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ61_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req61_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ62_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req62_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ63_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE1."]
    #[inline(always)]
    pub const fn set_req63_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable1Tog {
    #[inline(always)]
    fn default() -> Dma1ReqEnable1Tog {
        Dma1ReqEnable1Tog(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable1Tog")
            .field("req32_en1", &self.req32_en1())
            .field("req33_en1", &self.req33_en1())
            .field("req34_en1", &self.req34_en1())
            .field("req35_en1", &self.req35_en1())
            .field("req36_en1", &self.req36_en1())
            .field("req37_en1", &self.req37_en1())
            .field("req38_en1", &self.req38_en1())
            .field("req39_en1", &self.req39_en1())
            .field("req40_en1", &self.req40_en1())
            .field("req41_en1", &self.req41_en1())
            .field("req42_en1", &self.req42_en1())
            .field("req43_en1", &self.req43_en1())
            .field("req44_en1", &self.req44_en1())
            .field("req45_en1", &self.req45_en1())
            .field("req46_en1", &self.req46_en1())
            .field("req47_en1", &self.req47_en1())
            .field("req48_en1", &self.req48_en1())
            .field("req49_en1", &self.req49_en1())
            .field("req50_en1", &self.req50_en1())
            .field("req51_en1", &self.req51_en1())
            .field("req52_en1", &self.req52_en1())
            .field("req53_en1", &self.req53_en1())
            .field("req54_en1", &self.req54_en1())
            .field("req57_en1", &self.req57_en1())
            .field("req58_en1", &self.req58_en1())
            .field("req59_en1", &self.req59_en1())
            .field("req60_en1", &self.req60_en1())
            .field("req61_en1", &self.req61_en1())
            .field("req62_en1", &self.req62_en1())
            .field("req63_en1", &self.req63_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable1Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable1Tog {{ req32_en1: {=bool:?}, req33_en1: {=bool:?}, req34_en1: {=bool:?}, req35_en1: {=bool:?}, req36_en1: {=bool:?}, req37_en1: {=bool:?}, req38_en1: {=bool:?}, req39_en1: {=bool:?}, req40_en1: {=bool:?}, req41_en1: {=bool:?}, req42_en1: {=bool:?}, req43_en1: {=bool:?}, req44_en1: {=bool:?}, req45_en1: {=bool:?}, req46_en1: {=bool:?}, req47_en1: {=bool:?}, req48_en1: {=bool:?}, req49_en1: {=bool:?}, req50_en1: {=bool:?}, req51_en1: {=bool:?}, req52_en1: {=bool:?}, req53_en1: {=bool:?}, req54_en1: {=bool:?}, req57_en1: {=bool:?}, req58_en1: {=bool:?}, req59_en1: {=bool:?}, req60_en1: {=bool:?}, req61_en1: {=bool:?}, req62_en1: {=bool:?}, req63_en1: {=bool:?} }}",
            self.req32_en1(),
            self.req33_en1(),
            self.req34_en1(),
            self.req35_en1(),
            self.req36_en1(),
            self.req37_en1(),
            self.req38_en1(),
            self.req39_en1(),
            self.req40_en1(),
            self.req41_en1(),
            self.req42_en1(),
            self.req43_en1(),
            self.req44_en1(),
            self.req45_en1(),
            self.req46_en1(),
            self.req47_en1(),
            self.req48_en1(),
            self.req49_en1(),
            self.req50_en1(),
            self.req51_en1(),
            self.req52_en1(),
            self.req53_en1(),
            self.req54_en1(),
            self.req57_en1(),
            self.req58_en1(),
            self.req59_en1(),
            self.req60_en1(),
            self.req61_en1(),
            self.req62_en1(),
            self.req63_en1()
        )
    }
}
#[doc = "DMA1 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2(pub u32);
impl Dma1ReqEnable2 {
    #[doc = "This register is used to enable and disable FlexIO0 Shifter3 Status DMA request OR Timer3 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en1(&self) -> super::vals::Req64En1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req64En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter3 Status DMA request OR Timer3 Status DMA request."]
    #[inline(always)]
    pub const fn set_req64_en1(&mut self, val: super::vals::Req64En1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter4 Status DMA request OR Timer4 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en1(&self) -> super::vals::Req65En1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req65En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter4 Status DMA request OR Timer4 Status DMA request."]
    #[inline(always)]
    pub const fn set_req65_en1(&mut self, val: super::vals::Req65En1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter5 Status DMA request OR Timer5 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en1(&self) -> super::vals::Req66En1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req66En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter5 Status DMA request OR Timer5 Status DMA request."]
    #[inline(always)]
    pub const fn set_req66_en1(&mut self, val: super::vals::Req66En1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter6 Status DMA request OR Timer6 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en1(&self) -> super::vals::Req67En1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req67En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter6 Status DMA request OR Timer6 Status DMA request."]
    #[inline(always)]
    pub const fn set_req67_en1(&mut self, val: super::vals::Req67En1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter7 Status DMA request OR Timer7 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en1(&self) -> super::vals::Req68En1 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req68En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter7 Status DMA request OR Timer7 Status DMA request."]
    #[inline(always)]
    pub const fn set_req68_en1(&mut self, val: super::vals::Req68En1) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en1(&self) -> super::vals::Req69En1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req69En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[inline(always)]
    pub const fn set_req69_en1(&mut self, val: super::vals::Req69En1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en1(&self) -> super::vals::Req70En1 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req70En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[inline(always)]
    pub const fn set_req70_en1(&mut self, val: super::vals::Req70En1) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en1(&self) -> super::vals::Req71En1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req71En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[inline(always)]
    pub const fn set_req71_en1(&mut self, val: super::vals::Req71En1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en1(&self) -> super::vals::Req72En1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req72En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[inline(always)]
    pub const fn set_req72_en1(&mut self, val: super::vals::Req72En1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en1(&self) -> super::vals::Req73En1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req73En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[inline(always)]
    pub const fn set_req73_en1(&mut self, val: super::vals::Req73En1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en1(&self) -> super::vals::Req74En1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req74En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[inline(always)]
    pub const fn set_req74_en1(&mut self, val: super::vals::Req74En1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en1(&self) -> super::vals::Req75En1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req75En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[inline(always)]
    pub const fn set_req75_en1(&mut self, val: super::vals::Req75En1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en1(&self) -> super::vals::Req76En1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req76En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[inline(always)]
    pub const fn set_req76_en1(&mut self, val: super::vals::Req76En1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en1(&self) -> super::vals::Req77En1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req77En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[inline(always)]
    pub const fn set_req77_en1(&mut self, val: super::vals::Req77En1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en1(&self) -> super::vals::Req78En1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req78En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[inline(always)]
    pub const fn set_req78_en1(&mut self, val: super::vals::Req78En1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en1(&self) -> super::vals::Req79En1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req79En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[inline(always)]
    pub const fn set_req79_en1(&mut self, val: super::vals::Req79En1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en1(&self) -> super::vals::Req80En1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req80En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[inline(always)]
    pub const fn set_req80_en1(&mut self, val: super::vals::Req80En1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en1(&self) -> super::vals::Req81En1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req81En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[inline(always)]
    pub const fn set_req81_en1(&mut self, val: super::vals::Req81En1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en1(&self) -> super::vals::Req82En1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req82En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[inline(always)]
    pub const fn set_req82_en1(&mut self, val: super::vals::Req82En1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en1(&self) -> super::vals::Req83En1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req83En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[inline(always)]
    pub const fn set_req83_en1(&mut self, val: super::vals::Req83En1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en1(&self) -> super::vals::Req84En1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req84En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[inline(always)]
    pub const fn set_req84_en1(&mut self, val: super::vals::Req84En1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en1(&self) -> super::vals::Req85En1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req85En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[inline(always)]
    pub const fn set_req85_en1(&mut self, val: super::vals::Req85En1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en1(&self) -> super::vals::Req86En1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req86En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[inline(always)]
    pub const fn set_req86_en1(&mut self, val: super::vals::Req86En1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en1(&self) -> super::vals::Req87En1 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req87En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[inline(always)]
    pub const fn set_req87_en1(&mut self, val: super::vals::Req87En1) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en1(&self) -> super::vals::Req88En1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req88En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[inline(always)]
    pub const fn set_req88_en1(&mut self, val: super::vals::Req88En1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en1(&self) -> super::vals::Req91En1 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Req91En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[inline(always)]
    pub const fn set_req91_en1(&mut self, val: super::vals::Req91En1) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en1(&self) -> super::vals::Req92En1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Req92En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[inline(always)]
    pub const fn set_req92_en1(&mut self, val: super::vals::Req92En1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en1(&self) -> super::vals::Req93En1 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Req93En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[inline(always)]
    pub const fn set_req93_en1(&mut self, val: super::vals::Req93En1) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en1(&self) -> super::vals::Req94En1 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req94En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[inline(always)]
    pub const fn set_req94_en1(&mut self, val: super::vals::Req94En1) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en1(&self) -> super::vals::Req95En1 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Req95En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[inline(always)]
    pub const fn set_req95_en1(&mut self, val: super::vals::Req95En1) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable2 {
    #[inline(always)]
    fn default() -> Dma1ReqEnable2 {
        Dma1ReqEnable2(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable2")
            .field("req64_en1", &self.req64_en1())
            .field("req65_en1", &self.req65_en1())
            .field("req66_en1", &self.req66_en1())
            .field("req67_en1", &self.req67_en1())
            .field("req68_en1", &self.req68_en1())
            .field("req69_en1", &self.req69_en1())
            .field("req70_en1", &self.req70_en1())
            .field("req71_en1", &self.req71_en1())
            .field("req72_en1", &self.req72_en1())
            .field("req73_en1", &self.req73_en1())
            .field("req74_en1", &self.req74_en1())
            .field("req75_en1", &self.req75_en1())
            .field("req76_en1", &self.req76_en1())
            .field("req77_en1", &self.req77_en1())
            .field("req78_en1", &self.req78_en1())
            .field("req79_en1", &self.req79_en1())
            .field("req80_en1", &self.req80_en1())
            .field("req81_en1", &self.req81_en1())
            .field("req82_en1", &self.req82_en1())
            .field("req83_en1", &self.req83_en1())
            .field("req84_en1", &self.req84_en1())
            .field("req85_en1", &self.req85_en1())
            .field("req86_en1", &self.req86_en1())
            .field("req87_en1", &self.req87_en1())
            .field("req88_en1", &self.req88_en1())
            .field("req91_en1", &self.req91_en1())
            .field("req92_en1", &self.req92_en1())
            .field("req93_en1", &self.req93_en1())
            .field("req94_en1", &self.req94_en1())
            .field("req95_en1", &self.req95_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2 {{ req64_en1: {:?}, req65_en1: {:?}, req66_en1: {:?}, req67_en1: {:?}, req68_en1: {:?}, req69_en1: {:?}, req70_en1: {:?}, req71_en1: {:?}, req72_en1: {:?}, req73_en1: {:?}, req74_en1: {:?}, req75_en1: {:?}, req76_en1: {:?}, req77_en1: {:?}, req78_en1: {:?}, req79_en1: {:?}, req80_en1: {:?}, req81_en1: {:?}, req82_en1: {:?}, req83_en1: {:?}, req84_en1: {:?}, req85_en1: {:?}, req86_en1: {:?}, req87_en1: {:?}, req88_en1: {:?}, req91_en1: {:?}, req92_en1: {:?}, req93_en1: {:?}, req94_en1: {:?}, req95_en1: {:?} }}",
            self.req64_en1(),
            self.req65_en1(),
            self.req66_en1(),
            self.req67_en1(),
            self.req68_en1(),
            self.req69_en1(),
            self.req70_en1(),
            self.req71_en1(),
            self.req72_en1(),
            self.req73_en1(),
            self.req74_en1(),
            self.req75_en1(),
            self.req76_en1(),
            self.req77_en1(),
            self.req78_en1(),
            self.req79_en1(),
            self.req80_en1(),
            self.req81_en1(),
            self.req82_en1(),
            self.req83_en1(),
            self.req84_en1(),
            self.req85_en1(),
            self.req86_en1(),
            self.req87_en1(),
            self.req88_en1(),
            self.req91_en1(),
            self.req92_en1(),
            self.req93_en1(),
            self.req94_en1(),
            self.req95_en1()
        )
    }
}
#[doc = "DMA1 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Clr(pub u32);
impl Dma1ReqEnable2Clr {
    #[doc = "Writing a 1 to REQ64_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ76_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ76_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req89_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req89_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req90_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req90_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable2Clr {
    #[inline(always)]
    fn default() -> Dma1ReqEnable2Clr {
        Dma1ReqEnable2Clr(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable2Clr")
            .field("req64_en1", &self.req64_en1())
            .field("req65_en1", &self.req65_en1())
            .field("req66_en1", &self.req66_en1())
            .field("req67_en1", &self.req67_en1())
            .field("req68_en1", &self.req68_en1())
            .field("req69_en1", &self.req69_en1())
            .field("req70_en1", &self.req70_en1())
            .field("req71_en1", &self.req71_en1())
            .field("req72_en1", &self.req72_en1())
            .field("req73_en1", &self.req73_en1())
            .field("req74_en1", &self.req74_en1())
            .field("req75_en1", &self.req75_en1())
            .field("req76_en1", &self.req76_en1())
            .field("req77_en1", &self.req77_en1())
            .field("req78_en1", &self.req78_en1())
            .field("req79_en1", &self.req79_en1())
            .field("req80_en1", &self.req80_en1())
            .field("req81_en1", &self.req81_en1())
            .field("req82_en1", &self.req82_en1())
            .field("req83_en1", &self.req83_en1())
            .field("req84_en1", &self.req84_en1())
            .field("req85_en1", &self.req85_en1())
            .field("req86_en1", &self.req86_en1())
            .field("req87_en1", &self.req87_en1())
            .field("req88_en1", &self.req88_en1())
            .field("req89_en1", &self.req89_en1())
            .field("req90_en1", &self.req90_en1())
            .field("req91_en1", &self.req91_en1())
            .field("req92_en1", &self.req92_en1())
            .field("req93_en1", &self.req93_en1())
            .field("req94_en1", &self.req94_en1())
            .field("req95_en1", &self.req95_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Clr {{ req64_en1: {=bool:?}, req65_en1: {=bool:?}, req66_en1: {=bool:?}, req67_en1: {=bool:?}, req68_en1: {=bool:?}, req69_en1: {=bool:?}, req70_en1: {=bool:?}, req71_en1: {=bool:?}, req72_en1: {=bool:?}, req73_en1: {=bool:?}, req74_en1: {=bool:?}, req75_en1: {=bool:?}, req76_en1: {=bool:?}, req77_en1: {=bool:?}, req78_en1: {=bool:?}, req79_en1: {=bool:?}, req80_en1: {=bool:?}, req81_en1: {=bool:?}, req82_en1: {=bool:?}, req83_en1: {=bool:?}, req84_en1: {=bool:?}, req85_en1: {=bool:?}, req86_en1: {=bool:?}, req87_en1: {=bool:?}, req88_en1: {=bool:?}, req89_en1: {=bool:?}, req90_en1: {=bool:?}, req91_en1: {=bool:?}, req92_en1: {=bool:?}, req93_en1: {=bool:?}, req94_en1: {=bool:?}, req95_en1: {=bool:?} }}",
            self.req64_en1(),
            self.req65_en1(),
            self.req66_en1(),
            self.req67_en1(),
            self.req68_en1(),
            self.req69_en1(),
            self.req70_en1(),
            self.req71_en1(),
            self.req72_en1(),
            self.req73_en1(),
            self.req74_en1(),
            self.req75_en1(),
            self.req76_en1(),
            self.req77_en1(),
            self.req78_en1(),
            self.req79_en1(),
            self.req80_en1(),
            self.req81_en1(),
            self.req82_en1(),
            self.req83_en1(),
            self.req84_en1(),
            self.req85_en1(),
            self.req86_en1(),
            self.req87_en1(),
            self.req88_en1(),
            self.req89_en1(),
            self.req90_en1(),
            self.req91_en1(),
            self.req92_en1(),
            self.req93_en1(),
            self.req94_en1(),
            self.req95_en1()
        )
    }
}
#[doc = "DMA1 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Set(pub u32);
impl Dma1ReqEnable2Set {
    #[doc = "Writing a 1 to REQ64_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ876_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ876_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req89_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req89_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req90_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req90_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable2Set {
    #[inline(always)]
    fn default() -> Dma1ReqEnable2Set {
        Dma1ReqEnable2Set(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable2Set")
            .field("req64_en1", &self.req64_en1())
            .field("req65_en1", &self.req65_en1())
            .field("req66_en1", &self.req66_en1())
            .field("req67_en1", &self.req67_en1())
            .field("req68_en1", &self.req68_en1())
            .field("req69_en1", &self.req69_en1())
            .field("req70_en1", &self.req70_en1())
            .field("req71_en1", &self.req71_en1())
            .field("req72_en1", &self.req72_en1())
            .field("req73_en1", &self.req73_en1())
            .field("req74_en1", &self.req74_en1())
            .field("req75_en1", &self.req75_en1())
            .field("req76_en1", &self.req76_en1())
            .field("req77_en1", &self.req77_en1())
            .field("req78_en1", &self.req78_en1())
            .field("req79_en1", &self.req79_en1())
            .field("req80_en1", &self.req80_en1())
            .field("req81_en1", &self.req81_en1())
            .field("req82_en1", &self.req82_en1())
            .field("req83_en1", &self.req83_en1())
            .field("req84_en1", &self.req84_en1())
            .field("req85_en1", &self.req85_en1())
            .field("req86_en1", &self.req86_en1())
            .field("req87_en1", &self.req87_en1())
            .field("req88_en1", &self.req88_en1())
            .field("req89_en1", &self.req89_en1())
            .field("req90_en1", &self.req90_en1())
            .field("req91_en1", &self.req91_en1())
            .field("req92_en1", &self.req92_en1())
            .field("req93_en1", &self.req93_en1())
            .field("req94_en1", &self.req94_en1())
            .field("req95_en1", &self.req95_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Set {{ req64_en1: {=bool:?}, req65_en1: {=bool:?}, req66_en1: {=bool:?}, req67_en1: {=bool:?}, req68_en1: {=bool:?}, req69_en1: {=bool:?}, req70_en1: {=bool:?}, req71_en1: {=bool:?}, req72_en1: {=bool:?}, req73_en1: {=bool:?}, req74_en1: {=bool:?}, req75_en1: {=bool:?}, req76_en1: {=bool:?}, req77_en1: {=bool:?}, req78_en1: {=bool:?}, req79_en1: {=bool:?}, req80_en1: {=bool:?}, req81_en1: {=bool:?}, req82_en1: {=bool:?}, req83_en1: {=bool:?}, req84_en1: {=bool:?}, req85_en1: {=bool:?}, req86_en1: {=bool:?}, req87_en1: {=bool:?}, req88_en1: {=bool:?}, req89_en1: {=bool:?}, req90_en1: {=bool:?}, req91_en1: {=bool:?}, req92_en1: {=bool:?}, req93_en1: {=bool:?}, req94_en1: {=bool:?}, req95_en1: {=bool:?} }}",
            self.req64_en1(),
            self.req65_en1(),
            self.req66_en1(),
            self.req67_en1(),
            self.req68_en1(),
            self.req69_en1(),
            self.req70_en1(),
            self.req71_en1(),
            self.req72_en1(),
            self.req73_en1(),
            self.req74_en1(),
            self.req75_en1(),
            self.req76_en1(),
            self.req77_en1(),
            self.req78_en1(),
            self.req79_en1(),
            self.req80_en1(),
            self.req81_en1(),
            self.req82_en1(),
            self.req83_en1(),
            self.req84_en1(),
            self.req85_en1(),
            self.req86_en1(),
            self.req87_en1(),
            self.req88_en1(),
            self.req89_en1(),
            self.req90_en1(),
            self.req91_en1(),
            self.req92_en1(),
            self.req93_en1(),
            self.req94_en1(),
            self.req95_en1()
        )
    }
}
#[doc = "DMA1 Request Enable2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Tog(pub u32);
impl Dma1ReqEnable2Tog {
    #[doc = "Writing a 1 to REQ64_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ64_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req64_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ65_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req65_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ66_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req66_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ67_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req67_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ68_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req68_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ69_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req69_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ70_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req70_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ71_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req71_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ72_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req72_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ73_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req73_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ74_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req74_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ75_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req75_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ76_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ76_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req76_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ77_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req77_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ78_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req78_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ79_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req79_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ80_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req80_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ81_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req81_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ82_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req82_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ83_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req83_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ84_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req84_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ85_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req85_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ86_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req86_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ87_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req87_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ88_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req88_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req89_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ89_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req89_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req90_en1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ90_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req90_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ91_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req91_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ92_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req92_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ93_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req93_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ94_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req94_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ95_EN1 in this register toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req95_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dma1ReqEnable2Tog {
    #[inline(always)]
    fn default() -> Dma1ReqEnable2Tog {
        Dma1ReqEnable2Tog(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable2Tog")
            .field("req64_en1", &self.req64_en1())
            .field("req65_en1", &self.req65_en1())
            .field("req66_en1", &self.req66_en1())
            .field("req67_en1", &self.req67_en1())
            .field("req68_en1", &self.req68_en1())
            .field("req69_en1", &self.req69_en1())
            .field("req70_en1", &self.req70_en1())
            .field("req71_en1", &self.req71_en1())
            .field("req72_en1", &self.req72_en1())
            .field("req73_en1", &self.req73_en1())
            .field("req74_en1", &self.req74_en1())
            .field("req75_en1", &self.req75_en1())
            .field("req76_en1", &self.req76_en1())
            .field("req77_en1", &self.req77_en1())
            .field("req78_en1", &self.req78_en1())
            .field("req79_en1", &self.req79_en1())
            .field("req80_en1", &self.req80_en1())
            .field("req81_en1", &self.req81_en1())
            .field("req82_en1", &self.req82_en1())
            .field("req83_en1", &self.req83_en1())
            .field("req84_en1", &self.req84_en1())
            .field("req85_en1", &self.req85_en1())
            .field("req86_en1", &self.req86_en1())
            .field("req87_en1", &self.req87_en1())
            .field("req88_en1", &self.req88_en1())
            .field("req89_en1", &self.req89_en1())
            .field("req90_en1", &self.req90_en1())
            .field("req91_en1", &self.req91_en1())
            .field("req92_en1", &self.req92_en1())
            .field("req93_en1", &self.req93_en1())
            .field("req94_en1", &self.req94_en1())
            .field("req95_en1", &self.req95_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Tog {{ req64_en1: {=bool:?}, req65_en1: {=bool:?}, req66_en1: {=bool:?}, req67_en1: {=bool:?}, req68_en1: {=bool:?}, req69_en1: {=bool:?}, req70_en1: {=bool:?}, req71_en1: {=bool:?}, req72_en1: {=bool:?}, req73_en1: {=bool:?}, req74_en1: {=bool:?}, req75_en1: {=bool:?}, req76_en1: {=bool:?}, req77_en1: {=bool:?}, req78_en1: {=bool:?}, req79_en1: {=bool:?}, req80_en1: {=bool:?}, req81_en1: {=bool:?}, req82_en1: {=bool:?}, req83_en1: {=bool:?}, req84_en1: {=bool:?}, req85_en1: {=bool:?}, req86_en1: {=bool:?}, req87_en1: {=bool:?}, req88_en1: {=bool:?}, req89_en1: {=bool:?}, req90_en1: {=bool:?}, req91_en1: {=bool:?}, req92_en1: {=bool:?}, req93_en1: {=bool:?}, req94_en1: {=bool:?}, req95_en1: {=bool:?} }}",
            self.req64_en1(),
            self.req65_en1(),
            self.req66_en1(),
            self.req67_en1(),
            self.req68_en1(),
            self.req69_en1(),
            self.req70_en1(),
            self.req71_en1(),
            self.req72_en1(),
            self.req73_en1(),
            self.req74_en1(),
            self.req75_en1(),
            self.req76_en1(),
            self.req77_en1(),
            self.req78_en1(),
            self.req79_en1(),
            self.req80_en1(),
            self.req81_en1(),
            self.req82_en1(),
            self.req83_en1(),
            self.req84_en1(),
            self.req85_en1(),
            self.req86_en1(),
            self.req87_en1(),
            self.req88_en1(),
            self.req89_en1(),
            self.req90_en1(),
            self.req91_en1(),
            self.req92_en1(),
            self.req93_en1(),
            self.req94_en1(),
            self.req95_en1()
        )
    }
}
#[doc = "DMA1 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3(pub u32);
impl Dma1ReqEnable3 {
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en1(&self) -> super::vals::Req96En1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Req96En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[inline(always)]
    pub const fn set_req96_en1(&mut self, val: super::vals::Req96En1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en1(&self) -> super::vals::Req97En1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Req97En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[inline(always)]
    pub const fn set_req97_en1(&mut self, val: super::vals::Req97En1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en1(&self) -> super::vals::Req98En1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Req98En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[inline(always)]
    pub const fn set_req98_en1(&mut self, val: super::vals::Req98En1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en1(&self) -> super::vals::Req99En1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Req99En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[inline(always)]
    pub const fn set_req99_en1(&mut self, val: super::vals::Req99En1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en1(&self) -> super::vals::Req100En1 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Req100En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[inline(always)]
    pub const fn set_req100_en1(&mut self, val: super::vals::Req100En1) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en1(&self) -> super::vals::Req101En1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Req101En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[inline(always)]
    pub const fn set_req101_en1(&mut self, val: super::vals::Req101En1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en1(&self) -> super::vals::Req102En1 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Req102En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[inline(always)]
    pub const fn set_req102_en1(&mut self, val: super::vals::Req102En1) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en1(&self) -> super::vals::Req103En1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Req103En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[inline(always)]
    pub const fn set_req103_en1(&mut self, val: super::vals::Req103En1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en1(&self) -> super::vals::Req104En1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Req104En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[inline(always)]
    pub const fn set_req104_en1(&mut self, val: super::vals::Req104En1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en1(&self) -> super::vals::Req105En1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Req105En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[inline(always)]
    pub const fn set_req105_en1(&mut self, val: super::vals::Req105En1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en1(&self) -> super::vals::Req106En1 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Req106En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[inline(always)]
    pub const fn set_req106_en1(&mut self, val: super::vals::Req106En1) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en1(&self) -> super::vals::Req107En1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Req107En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[inline(always)]
    pub const fn set_req107_en1(&mut self, val: super::vals::Req107En1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en1(&self) -> super::vals::Req108En1 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Req108En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[inline(always)]
    pub const fn set_req108_en1(&mut self, val: super::vals::Req108En1) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en1(&self) -> super::vals::Req109En1 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Req109En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[inline(always)]
    pub const fn set_req109_en1(&mut self, val: super::vals::Req109En1) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en1(&self) -> super::vals::Req110En1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Req110En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[inline(always)]
    pub const fn set_req110_en1(&mut self, val: super::vals::Req110En1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en1(&self) -> super::vals::Req111En1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Req111En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[inline(always)]
    pub const fn set_req111_en1(&mut self, val: super::vals::Req111En1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en1(&self) -> super::vals::Req112En1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Req112En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[inline(always)]
    pub const fn set_req112_en1(&mut self, val: super::vals::Req112En1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en1(&self) -> super::vals::Req113En1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Req113En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[inline(always)]
    pub const fn set_req113_en1(&mut self, val: super::vals::Req113En1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en1(&self) -> super::vals::Req114En1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Req114En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[inline(always)]
    pub const fn set_req114_en1(&mut self, val: super::vals::Req114En1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en1(&self) -> super::vals::Req115En1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Req115En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[inline(always)]
    pub const fn set_req115_en1(&mut self, val: super::vals::Req115En1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en1(&self) -> super::vals::Req116En1 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Req116En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[inline(always)]
    pub const fn set_req116_en1(&mut self, val: super::vals::Req116En1) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en1(&self) -> super::vals::Req117En1 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Req117En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[inline(always)]
    pub const fn set_req117_en1(&mut self, val: super::vals::Req117En1) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en1(&self) -> super::vals::Req118En1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Req118En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[inline(always)]
    pub const fn set_req118_en1(&mut self, val: super::vals::Req118En1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en1(&self) -> super::vals::Req119En1 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Req119En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[inline(always)]
    pub const fn set_req119_en1(&mut self, val: super::vals::Req119En1) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en1(&self) -> super::vals::Req120En1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Req120En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[inline(always)]
    pub const fn set_req120_en1(&mut self, val: super::vals::Req120En1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en1(&self) -> super::vals::Req121En1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Req121En1::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[inline(always)]
    pub const fn set_req121_en1(&mut self, val: super::vals::Req121En1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma1ReqEnable3 {
    #[inline(always)]
    fn default() -> Dma1ReqEnable3 {
        Dma1ReqEnable3(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable3")
            .field("req96_en1", &self.req96_en1())
            .field("req97_en1", &self.req97_en1())
            .field("req98_en1", &self.req98_en1())
            .field("req99_en1", &self.req99_en1())
            .field("req100_en1", &self.req100_en1())
            .field("req101_en1", &self.req101_en1())
            .field("req102_en1", &self.req102_en1())
            .field("req103_en1", &self.req103_en1())
            .field("req104_en1", &self.req104_en1())
            .field("req105_en1", &self.req105_en1())
            .field("req106_en1", &self.req106_en1())
            .field("req107_en1", &self.req107_en1())
            .field("req108_en1", &self.req108_en1())
            .field("req109_en1", &self.req109_en1())
            .field("req110_en1", &self.req110_en1())
            .field("req111_en1", &self.req111_en1())
            .field("req112_en1", &self.req112_en1())
            .field("req113_en1", &self.req113_en1())
            .field("req114_en1", &self.req114_en1())
            .field("req115_en1", &self.req115_en1())
            .field("req116_en1", &self.req116_en1())
            .field("req117_en1", &self.req117_en1())
            .field("req118_en1", &self.req118_en1())
            .field("req119_en1", &self.req119_en1())
            .field("req120_en1", &self.req120_en1())
            .field("req121_en1", &self.req121_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable3 {{ req96_en1: {:?}, req97_en1: {:?}, req98_en1: {:?}, req99_en1: {:?}, req100_en1: {:?}, req101_en1: {:?}, req102_en1: {:?}, req103_en1: {:?}, req104_en1: {:?}, req105_en1: {:?}, req106_en1: {:?}, req107_en1: {:?}, req108_en1: {:?}, req109_en1: {:?}, req110_en1: {:?}, req111_en1: {:?}, req112_en1: {:?}, req113_en1: {:?}, req114_en1: {:?}, req115_en1: {:?}, req116_en1: {:?}, req117_en1: {:?}, req118_en1: {:?}, req119_en1: {:?}, req120_en1: {:?}, req121_en1: {:?} }}",
            self.req96_en1(),
            self.req97_en1(),
            self.req98_en1(),
            self.req99_en1(),
            self.req100_en1(),
            self.req101_en1(),
            self.req102_en1(),
            self.req103_en1(),
            self.req104_en1(),
            self.req105_en1(),
            self.req106_en1(),
            self.req107_en1(),
            self.req108_en1(),
            self.req109_en1(),
            self.req110_en1(),
            self.req111_en1(),
            self.req112_en1(),
            self.req113_en1(),
            self.req114_en1(),
            self.req115_en1(),
            self.req116_en1(),
            self.req117_en1(),
            self.req118_en1(),
            self.req119_en1(),
            self.req120_en1(),
            self.req121_en1()
        )
    }
}
#[doc = "DMA1 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3Clr(pub u32);
impl Dma1ReqEnable3Clr {
    #[doc = "Writing a 1 to REQ96_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ96_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req96_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ97_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ97_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req97_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ98_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ98_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req98_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ99_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ99_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req99_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ100_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ100_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req100_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ101_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ101_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req101_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ102_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ102_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req102_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ103_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ103_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req103_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ104_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ104_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req104_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ105_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ105_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req105_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ106_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ106_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req106_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ107_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ107_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req107_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ108_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ108_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req108_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ109_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ109_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req109_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ110_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ110_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req110_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ111_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ111_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req111_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ112_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ112_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req112_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ113_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ113_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req113_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ114_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ114_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req114_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ115_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ115_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req115_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ116_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ116_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req116_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ117_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ117_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req117_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ118_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ118_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req118_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ119_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ119_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req119_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ120_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ120_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req120_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ121_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3."]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ121_EN1 in this register clears the corresponding bit in DMA0_REQ_ENABLE3."]
    #[inline(always)]
    pub const fn set_req121_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma1ReqEnable3Clr {
    #[inline(always)]
    fn default() -> Dma1ReqEnable3Clr {
        Dma1ReqEnable3Clr(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable3Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable3Clr")
            .field("req96_en1", &self.req96_en1())
            .field("req97_en1", &self.req97_en1())
            .field("req98_en1", &self.req98_en1())
            .field("req99_en1", &self.req99_en1())
            .field("req100_en1", &self.req100_en1())
            .field("req101_en1", &self.req101_en1())
            .field("req102_en1", &self.req102_en1())
            .field("req103_en1", &self.req103_en1())
            .field("req104_en1", &self.req104_en1())
            .field("req105_en1", &self.req105_en1())
            .field("req106_en1", &self.req106_en1())
            .field("req107_en1", &self.req107_en1())
            .field("req108_en1", &self.req108_en1())
            .field("req109_en1", &self.req109_en1())
            .field("req110_en1", &self.req110_en1())
            .field("req111_en1", &self.req111_en1())
            .field("req112_en1", &self.req112_en1())
            .field("req113_en1", &self.req113_en1())
            .field("req114_en1", &self.req114_en1())
            .field("req115_en1", &self.req115_en1())
            .field("req116_en1", &self.req116_en1())
            .field("req117_en1", &self.req117_en1())
            .field("req118_en1", &self.req118_en1())
            .field("req119_en1", &self.req119_en1())
            .field("req120_en1", &self.req120_en1())
            .field("req121_en1", &self.req121_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable3Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable3Clr {{ req96_en1: {=bool:?}, req97_en1: {=bool:?}, req98_en1: {=bool:?}, req99_en1: {=bool:?}, req100_en1: {=bool:?}, req101_en1: {=bool:?}, req102_en1: {=bool:?}, req103_en1: {=bool:?}, req104_en1: {=bool:?}, req105_en1: {=bool:?}, req106_en1: {=bool:?}, req107_en1: {=bool:?}, req108_en1: {=bool:?}, req109_en1: {=bool:?}, req110_en1: {=bool:?}, req111_en1: {=bool:?}, req112_en1: {=bool:?}, req113_en1: {=bool:?}, req114_en1: {=bool:?}, req115_en1: {=bool:?}, req116_en1: {=bool:?}, req117_en1: {=bool:?}, req118_en1: {=bool:?}, req119_en1: {=bool:?}, req120_en1: {=bool:?}, req121_en1: {=bool:?} }}",
            self.req96_en1(),
            self.req97_en1(),
            self.req98_en1(),
            self.req99_en1(),
            self.req100_en1(),
            self.req101_en1(),
            self.req102_en1(),
            self.req103_en1(),
            self.req104_en1(),
            self.req105_en1(),
            self.req106_en1(),
            self.req107_en1(),
            self.req108_en1(),
            self.req109_en1(),
            self.req110_en1(),
            self.req111_en1(),
            self.req112_en1(),
            self.req113_en1(),
            self.req114_en1(),
            self.req115_en1(),
            self.req116_en1(),
            self.req117_en1(),
            self.req118_en1(),
            self.req119_en1(),
            self.req120_en1(),
            self.req121_en1()
        )
    }
}
#[doc = "DMA1 Request Enable3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3Set(pub u32);
impl Dma1ReqEnable3Set {
    #[doc = "Writing a 1 to REQ96_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ96_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req96_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing a 1 to REQ97_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ97_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req97_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing a 1 to REQ98_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ98_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req98_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to REQ99_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ99_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req99_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Writing a 1 to REQ100_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ100_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req100_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing a 1 to REQ101_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ101_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req101_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing a 1 to REQ102_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ102_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req102_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Writing a 1 to REQ103_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ103_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req103_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Writing a 1 to REQ104_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ104_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req104_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Writing a 1 to REQ105_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ105_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req105_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Writing a 1 to REQ106_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ106_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req106_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Writing a 1 to REQ107_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ107_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req107_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Writing a 1 to REQ108_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ108_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req108_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Writing a 1 to REQ109_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en1(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ109_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req109_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Writing a 1 to REQ110_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ110_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req110_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Writing a 1 to REQ111_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ111_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req111_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Writing a 1 to REQ112_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ112_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req112_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Writing a 1 to REQ113_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ113_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req113_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Writing a 1 to REQ114_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ114_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req114_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to REQ115_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ115_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req115_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Writing a 1 to REQ116_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ116_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req116_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Writing a 1 to REQ117_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ117_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req117_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Writing a 1 to REQ118_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ118_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req118_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Writing a 1 to REQ119_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ119_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req119_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Writing a 1 to REQ120_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ120_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req120_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Writing a 1 to REQ121_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to REQ121_EN1 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
    #[inline(always)]
    pub const fn set_req121_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Dma1ReqEnable3Set {
    #[inline(always)]
    fn default() -> Dma1ReqEnable3Set {
        Dma1ReqEnable3Set(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnable3Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnable3Set")
            .field("req96_en1", &self.req96_en1())
            .field("req97_en1", &self.req97_en1())
            .field("req98_en1", &self.req98_en1())
            .field("req99_en1", &self.req99_en1())
            .field("req100_en1", &self.req100_en1())
            .field("req101_en1", &self.req101_en1())
            .field("req102_en1", &self.req102_en1())
            .field("req103_en1", &self.req103_en1())
            .field("req104_en1", &self.req104_en1())
            .field("req105_en1", &self.req105_en1())
            .field("req106_en1", &self.req106_en1())
            .field("req107_en1", &self.req107_en1())
            .field("req108_en1", &self.req108_en1())
            .field("req109_en1", &self.req109_en1())
            .field("req110_en1", &self.req110_en1())
            .field("req111_en1", &self.req111_en1())
            .field("req112_en1", &self.req112_en1())
            .field("req113_en1", &self.req113_en1())
            .field("req114_en1", &self.req114_en1())
            .field("req115_en1", &self.req115_en1())
            .field("req116_en1", &self.req116_en1())
            .field("req117_en1", &self.req117_en1())
            .field("req118_en1", &self.req118_en1())
            .field("req119_en1", &self.req119_en1())
            .field("req120_en1", &self.req120_en1())
            .field("req121_en1", &self.req121_en1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable3Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable3Set {{ req96_en1: {=bool:?}, req97_en1: {=bool:?}, req98_en1: {=bool:?}, req99_en1: {=bool:?}, req100_en1: {=bool:?}, req101_en1: {=bool:?}, req102_en1: {=bool:?}, req103_en1: {=bool:?}, req104_en1: {=bool:?}, req105_en1: {=bool:?}, req106_en1: {=bool:?}, req107_en1: {=bool:?}, req108_en1: {=bool:?}, req109_en1: {=bool:?}, req110_en1: {=bool:?}, req111_en1: {=bool:?}, req112_en1: {=bool:?}, req113_en1: {=bool:?}, req114_en1: {=bool:?}, req115_en1: {=bool:?}, req116_en1: {=bool:?}, req117_en1: {=bool:?}, req118_en1: {=bool:?}, req119_en1: {=bool:?}, req120_en1: {=bool:?}, req121_en1: {=bool:?} }}",
            self.req96_en1(),
            self.req97_en1(),
            self.req98_en1(),
            self.req99_en1(),
            self.req100_en1(),
            self.req101_en1(),
            self.req102_en1(),
            self.req103_en1(),
            self.req104_en1(),
            self.req105_en1(),
            self.req106_en1(),
            self.req107_en1(),
            self.req108_en1(),
            self.req109_en1(),
            self.req110_en1(),
            self.req111_en1(),
            self.req112_en1(),
            self.req113_en1(),
            self.req114_en1(),
            self.req115_en1(),
            self.req116_en1(),
            self.req117_en1(),
            self.req118_en1(),
            self.req119_en1(),
            self.req120_en1(),
            self.req121_en1()
        )
    }
}
#[doc = "EVTG Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgTrig(pub u32);
impl EvtgTrig {
    #[doc = "EVTG trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::EvtgTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::EvtgTrigInp::from_bits(val as u8)
    }
    #[doc = "EVTG trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::EvtgTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for EvtgTrig {
    #[inline(always)]
    fn default() -> EvtgTrig {
        EvtgTrig(0)
    }
}
impl core::fmt::Debug for EvtgTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EvtgTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "EXT Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig(pub u32);
impl ExtTrig {
    #[doc = "TRIG_OUTa pin input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::ExtTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::ExtTrigInp::from_bits(val as u8)
    }
    #[doc = "TRIG_OUTa pin input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::ExtTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
#[doc = "PWM0 External Force Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Extforce(pub u32);
impl FlexPwm0Extforce {
    #[doc = "EXTFORCE input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0ExtforceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0ExtforceTrigin::from_bits(val as u8)
    }
    #[doc = "EXTFORCE input connections for PWM0"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0ExtforceTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0Extforce {
    #[inline(always)]
    fn default() -> FlexPwm0Extforce {
        FlexPwm0Extforce(0)
    }
}
impl core::fmt::Debug for FlexPwm0Extforce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0Extforce")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0Extforce {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0Extforce {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 Fault Input Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Fault(pub u32);
impl FlexPwm0Fault {
    #[doc = "FAULT input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0"]
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
#[doc = "PWM0 Input Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0SmExta(pub u32);
impl FlexPwm0SmExta {
    #[doc = "EXTA input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0SmExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0SmExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0SmExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0SmExta {
    #[inline(always)]
    fn default() -> FlexPwm0SmExta {
        FlexPwm0SmExta(0)
    }
}
impl core::fmt::Debug for FlexPwm0SmExta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0SmExta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0SmExta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0SmExta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM0 External Synchronization"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0SmExtsync(pub u32);
impl FlexPwm0SmExtsync {
    #[doc = "EXTSYNC input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm0SmExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm0SmExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm0SmExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm0SmExtsync {
    #[inline(always)]
    fn default() -> FlexPwm0SmExtsync {
        FlexPwm0SmExtsync(0)
    }
}
impl core::fmt::Debug for FlexPwm0SmExtsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm0SmExtsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm0SmExtsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm0SmExtsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 External Force Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Extforce(pub u32);
impl FlexPwm1Extforce {
    #[doc = "EXTFORCE input connections for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1ExtforceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1ExtforceTrigin::from_bits(val as u8)
    }
    #[doc = "EXTFORCE input connections for PWM1"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1ExtforceTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1Extforce {
    #[inline(always)]
    fn default() -> FlexPwm1Extforce {
        FlexPwm1Extforce(0)
    }
}
impl core::fmt::Debug for FlexPwm1Extforce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1Extforce")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1Extforce {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1Extforce {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 Fault Input Trigger Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Fault(pub u32);
impl FlexPwm1Fault {
    #[doc = "FAULT input connections for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM1"]
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
#[doc = "PWM1 Input EXTA Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1SmExta(pub u32);
impl FlexPwm1SmExta {
    #[doc = "EXTA input connections for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1SmExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1SmExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM1"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1SmExtaTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1SmExta {
    #[inline(always)]
    fn default() -> FlexPwm1SmExta {
        FlexPwm1SmExta(0)
    }
}
impl core::fmt::Debug for FlexPwm1SmExta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1SmExta")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1SmExta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1SmExta {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "PWM1 External Synchronization"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1SmExtsync(pub u32);
impl FlexPwm1SmExtsync {
    #[doc = "EXTSYNC input connections for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::FlexPwm1SmExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FlexPwm1SmExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM1"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::FlexPwm1SmExtsyncTrigin) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for FlexPwm1SmExtsync {
    #[inline(always)]
    fn default() -> FlexPwm1SmExtsync {
        FlexPwm1SmExtsync(0)
    }
}
impl core::fmt::Debug for FlexPwm1SmExtsync {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexPwm1SmExtsync")
            .field("trigin", &self.trigin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexPwm1SmExtsync {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexPwm1SmExtsync {{ trigin: {:?} }}", self.trigin())
    }
}
#[doc = "LP_FLEXCOMM0 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm0Trig(pub u32);
impl Flexcomm0Trig {
    #[doc = "LP_FLEXCOMM0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm0TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm0TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm0TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm0Trig {
    #[inline(always)]
    fn default() -> Flexcomm0Trig {
        Flexcomm0Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm0Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm0Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm0Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM1 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm1Trig(pub u32);
impl Flexcomm1Trig {
    #[doc = "LP_FLEXCOMM1 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm1TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm1TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM1 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm1TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm1Trig {
    #[inline(always)]
    fn default() -> Flexcomm1Trig {
        Flexcomm1Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm1Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm1Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm1Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm1Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM2 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm2Trig(pub u32);
impl Flexcomm2Trig {
    #[doc = "LP_FLEXCOMM2 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm2TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm2TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM2 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm2TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm2Trig {
    #[inline(always)]
    fn default() -> Flexcomm2Trig {
        Flexcomm2Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm2Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm2Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm2Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm2Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM3 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm3Trig(pub u32);
impl Flexcomm3Trig {
    #[doc = "LP_FLEXCOMM3 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm3TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm3TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM3 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm3TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm3Trig {
    #[inline(always)]
    fn default() -> Flexcomm3Trig {
        Flexcomm3Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm3Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm3Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm3Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm3Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM4 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm4Trig(pub u32);
impl Flexcomm4Trig {
    #[doc = "LP_FLEXCOMM4 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm4TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm4TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM4 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm4TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm4Trig {
    #[inline(always)]
    fn default() -> Flexcomm4Trig {
        Flexcomm4Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm4Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm4Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm4Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm4Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM5 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm5Trig(pub u32);
impl Flexcomm5Trig {
    #[doc = "LP_FLEXCOMM5 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm5TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm5TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM5 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm5TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm5Trig {
    #[inline(always)]
    fn default() -> Flexcomm5Trig {
        Flexcomm5Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm5Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm5Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm5Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm5Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM6 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm6Trig(pub u32);
impl Flexcomm6Trig {
    #[doc = "LP_FLEXCOMM6 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm6TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm6TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM6 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm6TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm6Trig {
    #[inline(always)]
    fn default() -> Flexcomm6Trig {
        Flexcomm6Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm6Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm6Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm6Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm6Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM7 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm7Trig(pub u32);
impl Flexcomm7Trig {
    #[doc = "LP_FLEXCOMM7 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm7TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm7TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM7 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm7TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm7Trig {
    #[inline(always)]
    fn default() -> Flexcomm7Trig {
        Flexcomm7Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm7Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm7Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm7Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm7Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM8 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm8Trig(pub u32);
impl Flexcomm8Trig {
    #[doc = "LP_FLEXCOMM8 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm8TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm8TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM8 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm8TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm8Trig {
    #[inline(always)]
    fn default() -> Flexcomm8Trig {
        Flexcomm8Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm8Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm8Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm8Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm8Trig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "LP_FLEXCOMM9 Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm9Trig(pub u32);
impl Flexcomm9Trig {
    #[doc = "LP_FLEXCOMM9 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Flexcomm9TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Flexcomm9TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM9 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Flexcomm9TrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Flexcomm9Trig {
    #[inline(always)]
    fn default() -> Flexcomm9Trig {
        Flexcomm9Trig(0)
    }
}
impl core::fmt::Debug for Flexcomm9Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcomm9Trig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcomm9Trig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcomm9Trig {{ inp: {:?} }}", self.inp())
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
#[doc = "Selection for Frequency Measurement Reference Clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (binary value) for frequency measure function reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function reference clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasRefInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
#[doc = "Selection for Frequency Measurement Target Clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTar(pub u32);
impl FreqmeasTar {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::FreqmeasTarInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
#[doc = "OPAMP Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampTrig(pub u32);
impl OpampTrig {
    #[doc = "OPAMP trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::OpampTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::OpampTrigInp::from_bits(val as u8)
    }
    #[doc = "OPAMP trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::OpampTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for OpampTrig {
    #[inline(always)]
    fn default() -> OpampTrig {
        OpampTrig(0)
    }
}
impl core::fmt::Debug for OpampTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpampTrig")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpampTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OpampTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Pin Interrupt Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pintsel(pub u32);
impl Pintsel {
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INP = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::PintselInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::PintselInp::from_bits(val as u8)
    }
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INP = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::PintselInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Pintsel {
    #[inline(always)]
    fn default() -> Pintsel {
        Pintsel(0)
    }
}
impl core::fmt::Debug for Pintsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pintsel").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pintsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pintsel {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "PWM0 External Clock Trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0ExtClk(pub u32);
impl Pwm0ExtClk {
    #[doc = "EXT_CLK input connections for PWM0"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm0ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Pwm0ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "EXT_CLK input connections for PWM0"]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: super::vals::Pwm0ExtClkTrigin) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
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
#[doc = "PWM1 External Clock Trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1ExtClk(pub u32);
impl Pwm1ExtClk {
    #[doc = "EXT_CLK input connections for PWM1"]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> super::vals::Pwm1ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Pwm1ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "EXT_CLK input connections for PWM1"]
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
#[doc = "QDCouter_loop Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcHome(pub u32);
impl QdcHome {
    #[doc = "QDC0 HOME input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 HOME input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcHomeInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for QdcHome {
    #[inline(always)]
    fn default() -> QdcHome {
        QdcHome(0)
    }
}
impl core::fmt::Debug for QdcHome {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcHome").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcHome {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcHome {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDCouter_loop Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcIndex(pub u32);
impl QdcIndex {
    #[doc = "QDC0 INDEX input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 INDEX input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcIndexInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for QdcIndex {
    #[inline(always)]
    fn default() -> QdcIndex {
        QdcIndex(0)
    }
}
impl core::fmt::Debug for QdcIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcIndex")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcIndex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcIndex {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDCouter_loop Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcPhasea(pub u32);
impl QdcPhasea {
    #[doc = "QDC0 PHASEA input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcPhaseaInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::QdcPhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 PHASEA input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcPhaseaInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for QdcPhasea {
    #[inline(always)]
    fn default() -> QdcPhasea {
        QdcPhasea(0)
    }
}
impl core::fmt::Debug for QdcPhasea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcPhasea")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcPhasea {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcPhasea {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDCouter_loop Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcPhaseb(pub u32);
impl QdcPhaseb {
    #[doc = "QDC0 PHASEB input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcPhasebInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::QdcPhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 PHASEB input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcPhasebInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for QdcPhaseb {
    #[inline(always)]
    fn default() -> QdcPhaseb {
        QdcPhaseb(0)
    }
}
impl core::fmt::Debug for QdcPhaseb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QdcPhaseb")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QdcPhaseb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "QdcPhaseb {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "QDCouter_loop Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcTrig(pub u32);
impl QdcTrig {
    #[doc = "QDC0 trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::QdcTrigInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
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
#[doc = "Inputmux Register for SCT0 Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0Inmux(pub u32);
impl Sct0Inmux {
    #[doc = "Input number to SCT0 inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Sct0InmuxInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Sct0InmuxInp::from_bits(val as u8)
    }
    #[doc = "Input number to SCT0 inputs."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Sct0InmuxInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Sct0Inmux {
    #[inline(always)]
    fn default() -> Sct0Inmux {
        Sct0Inmux(0)
    }
}
impl core::fmt::Debug for Sct0Inmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sct0Inmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sct0Inmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sct0Inmux {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "SINC Filter Channel Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SincFilterCh(pub u32);
impl SincFilterCh {
    #[doc = "SINC FILTER trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::SincFilterChInp {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::SincFilterChInp::from_bits(val as u8)
    }
    #[doc = "SINC FILTER trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::SincFilterChInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for SincFilterCh {
    #[inline(always)]
    fn default() -> SincFilterCh {
        SincFilterCh(0)
    }
}
impl core::fmt::Debug for SincFilterCh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SincFilterCh")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SincFilterCh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SincFilterCh {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Inputmux Register for SMARTDMA Arch B Inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartdmaarchbInmux(pub u32);
impl SmartdmaarchbInmux {
    #[doc = "Input number select to SmartDMA ARCHB input"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::SmartdmaarchbInmuxInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::SmartdmaarchbInmuxInp::from_bits(val as u8)
    }
    #[doc = "Input number select to SmartDMA ARCHB input"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::SmartdmaarchbInmuxInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for SmartdmaarchbInmux {
    #[inline(always)]
    fn default() -> SmartdmaarchbInmux {
        SmartdmaarchbInmux(0)
    }
}
impl core::fmt::Debug for SmartdmaarchbInmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartdmaarchbInmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartdmaarchbInmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmartdmaarchbInmux {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Trigger Register for CTIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
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
#[doc = "Trigger Register for CTIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer1trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
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
#[doc = "Trigger Register for CTIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer2trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
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
#[doc = "Trigger Register for CTIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer3trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
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
#[doc = "Trigger Register for CTIMER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Timer4trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        super::vals::Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER"]
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
#[doc = "TSI Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsiTrig(pub u32);
impl TsiTrig {
    #[doc = "TSI trigger input connections"]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::TsiTrigInp {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TsiTrigInp::from_bits(val as u8)
    }
    #[doc = "TSI trigger input connections"]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::TsiTrigInp) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for TsiTrig {
    #[inline(always)]
    fn default() -> TsiTrig {
        TsiTrig(0)
    }
}
impl core::fmt::Debug for TsiTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TsiTrig").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TsiTrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TsiTrig {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "USB-FS Trigger Input Connections"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsTrig(pub u32);
impl UsbfsTrig {
    #[doc = "USB-FS trigger input connections. The trigger output of LP_FLEXCOMM is an input of peripheral INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::UsbfsTrigInp {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::UsbfsTrigInp::from_bits(val as u8)
    }
    #[doc = "USB-FS trigger input connections. The trigger output of LP_FLEXCOMM is an input of peripheral INPUTMUX."]
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
