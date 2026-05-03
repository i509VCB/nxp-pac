#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "INPUTMUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux {}
unsafe impl Sync for Inputmux {}
impl Inputmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer0cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer0cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER0."]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::pac::common::Reg<Timer0trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer1cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer1cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER1."]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::pac::common::Reg<Timer1trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer2cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer2cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER2 inputs."]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::pac::common::Reg<Timer2trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SmartDMA Trigger Input Connections."]
    #[inline(always)]
    pub const fn smart_dma_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SmartDmaTrig, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "LPSPI2 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi2_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "LPSPI3 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi3_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "LPSPI4 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi4_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "LPSPI5 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi5_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn freqmeas_ref(
        self,
    ) -> crate::pac::common::Reg<FreqmeasRef, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn freqmeas_tar(
        self,
    ) -> crate::pac::common::Reg<FreqmeasTar, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer3cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer3cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER3."]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::pac::common::Reg<Timer3trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs."]
    #[inline(always)]
    pub const fn ctimer4cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer4cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER4."]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::pac::common::Reg<Timer4trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "CMP0 input connections."]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::pac::common::Reg<CmpTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc0_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections."]
    #[inline(always)]
    pub const fn adc1_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<AdcTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 trigger."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::pac::common::Reg<DacTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "DAC1 trigger."]
    #[inline(always)]
    pub const fn dac1_trig(self) -> crate::pac::common::Reg<DacTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "AOI0 trigger input connections 0."]
    #[inline(always)]
    pub const fn aoi0_input(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Aoi0Input, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 4usize) as _)
        }
    }
    #[doc = "TSI0 trigger input connections."]
    #[inline(always)]
    pub const fn tsi0_trig_input(
        self,
    ) -> crate::pac::common::Reg<Tsi0TrigInput, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "EXT trigger connections."]
    #[inline(always)]
    pub const fn trig_out(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<TrigOut, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize + n * 4usize) as _)
        }
    }
    #[doc = "LPI2C2 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c2_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "LPI2C3 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c3_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "LPI2C0 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c0_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LPI2C1 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c1_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LPSPI0 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi0_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LPSPI1 trigger input connections."]
    #[inline(always)]
    pub const fn lpspi1_trig(self) -> crate::pac::common::Reg<LpspiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LPUART0 trigger input connections."]
    #[inline(always)]
    pub const fn lpuart(self, n: usize) -> crate::pac::common::Reg<Lpuart, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 32usize) as _)
        }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexioTrig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize + n * 4usize) as _)
        }
    }
}
#[doc = "ADC Trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcTrig(pub u32);
impl AdcTrig {
    #[doc = "ADC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> AdcTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        AdcTrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: AdcTrigTrigin) {
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
#[doc = "AOI0 trigger input connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi0Input(pub u32);
impl Aoi0Input {
    #[doc = "AOI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> AoiInputInp {
        let val = (self.0 >> 0usize) & 0x7f;
        AoiInputInp::from_bits(val as u8)
    }
    #[doc = "AOI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: AoiInputInp) {
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
#[doc = "CMP0 input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpTrig(pub u32);
impl CmpTrig {
    #[doc = "CMP0 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> CmpTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        CmpTrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: CmpTrigTrigin) {
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
#[doc = "Capture select register for CTIMER inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0cap(pub u32);
impl Ctimer0cap {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer0capInp {
        let val = (self.0 >> 0usize) & 0xff;
        Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer0capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0xff;
        Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer1capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0xff;
        Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer2capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0xff;
        Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer3capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0xff;
        Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer4capInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
#[doc = "DAC0 trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacTrig(pub u32);
impl DacTrig {
    #[doc = "This register selects the DAC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> DacTrigTrigin {
        let val = (self.0 >> 0usize) & 0x7f;
        DacTrigTrigin::from_bits(val as u8)
    }
    #[doc = "This register selects the DAC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: DacTrigTrigin) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
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
#[doc = "FlexIO Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexioTrig(pub u32);
impl FlexioTrig {
    #[doc = "Input number for FlexIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FlexioTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FlexioTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for FlexIO0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FlexioTrigInp) {
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
    pub const fn inp(&self) -> FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasRefInp) {
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
    pub const fn inp(&self) -> FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x7f;
        FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasTarInp) {
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
pub struct Lpi2cTrig(pub u32);
impl Lpi2cTrig {
    #[doc = "LPI2C0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Lpi2cTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Lpi2cTrigInp::from_bits(val as u8)
    }
    #[doc = "LPI2C0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Lpi2cTrigInp) {
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
#[doc = "LPSPI0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpspiTrig(pub u32);
impl LpspiTrig {
    #[doc = "LPSPI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> LpspiTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        LpspiTrigInp::from_bits(val as u8)
    }
    #[doc = "LPSPI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: LpspiTrigInp) {
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
#[doc = "LPUART0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart(pub u32);
impl Lpuart {
    #[doc = "LPUART0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> LpuartInp {
        let val = (self.0 >> 0usize) & 0x3f;
        LpuartInp::from_bits(val as u8)
    }
    #[doc = "LPUART0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: LpuartInp) {
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
#[doc = "SmartDMA Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaTrig(pub u32);
impl SmartDmaTrig {
    #[doc = "Input number for FlexIO0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> SmartDmaTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        SmartDmaTrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for FlexIO0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: SmartDmaTrigInp) {
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
#[doc = "Trigger register for TIMER0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer0trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer0trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Timer1trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        Timer1trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer1trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Timer2trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        Timer2trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer2trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Timer3trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer3trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
    pub const fn inp(&self) -> Timer4trigInp {
        let val = (self.0 >> 0usize) & 0xff;
        Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer4trigInp) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
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
#[doc = "EXT trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigOut(pub u32);
impl TrigOut {
    #[doc = "EXT trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> TrigOutInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TrigOutInp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TrigOutInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for TrigOut {
    #[inline(always)]
    fn default() -> TrigOut {
        TrigOut(0)
    }
}
impl core::fmt::Debug for TrigOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigOut").field("inp", &self.inp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TrigOut {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "TSI0 trigger input connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsi0TrigInput(pub u32);
impl Tsi0TrigInput {
    #[doc = "TSI0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Tsi0TrigInputInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Tsi0TrigInputInp::from_bits(val as u8)
    }
    #[doc = "TSI0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Tsi0TrigInputInp) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Tsi0TrigInput {
    #[inline(always)]
    fn default() -> Tsi0TrigInput {
        Tsi0TrigInput(0)
    }
}
impl core::fmt::Debug for Tsi0TrigInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsi0TrigInput")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsi0TrigInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsi0TrigInput {{ inp: {:?} }}", self.inp())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
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
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "WUU."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO0 CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO0 CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO0 CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO0 CH3 input is selected."]
    Val48 = 0x30,
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
impl AdcTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> AdcTrigTrigin {
        AdcTrigTrigin::from_bits(val)
    }
}
impl From<AdcTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: AdcTrigTrigin) -> u8 {
        AdcTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AoiInputInp {
    _RESERVED_0 = 0x0,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val1 = 0x01,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val2 = 0x02,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val3 = 0x03,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val4 = 0x04,
    #[doc = "CMP0_OUT input is selected."]
    Val5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val15 = 0x0f,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val16 = 0x10,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val17 = 0x11,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val18 = 0x12,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val19 = 0x13,
    #[doc = "LPTMR0 input is selected."]
    Val20 = 0x14,
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
    #[doc = "TRIG_IN0 input is selected."]
    Val35 = 0x23,
    #[doc = "TRIG_IN1 input is selected."]
    Val36 = 0x24,
    #[doc = "TRIG_IN2 input is selected."]
    Val37 = 0x25,
    #[doc = "TRIG_IN3 input is selected."]
    Val38 = 0x26,
    #[doc = "TRIG_IN4 input is selected."]
    Val39 = 0x27,
    #[doc = "TRIG_IN5 input is selected."]
    Val40 = 0x28,
    #[doc = "TRIG_IN6 input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN7 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN8 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN9 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN10 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN11 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val47 = 0x2f,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val48 = 0x30,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val49 = 0x31,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val51 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val56 = 0x38,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val57 = 0x39,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val58 = 0x3a,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val59 = 0x3b,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val60 = 0x3c,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val61 = 0x3d,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val62 = 0x3e,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val63 = 0x3f,
    #[doc = "FlexIO CH0 input is selected."]
    Val64 = 0x40,
    #[doc = "FlexIO CH1 input is selected."]
    Val65 = 0x41,
    #[doc = "FlexIO CH2 input is selected."]
    Val66 = 0x42,
    #[doc = "FlexIO CH3 input is selected."]
    Val67 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val97 = 0x61,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val98 = 0x62,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val99 = 0x63,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val100 = 0x64,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val101 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl AoiInputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AoiInputInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AoiInputInp {
    #[inline(always)]
    fn from(val: u8) -> AoiInputInp {
        AoiInputInp::from_bits(val)
    }
}
impl From<AoiInputInp> for u8 {
    #[inline(always)]
    fn from(val: AoiInputInp) -> u8 {
        AoiInputInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpTrigTrigin {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer1_MAT0."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "LPTMR0 input is selected."]
    Val14 = 0x0e,
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
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val42 = 0x2a,
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
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl CmpTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> CmpTrigTrigin {
        CmpTrigTrigin::from_bits(val)
    }
}
impl From<CmpTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: CmpTrigTrigin) -> u8 {
        CmpTrigTrigin::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer0capInp(u8);
impl Ctimer0capInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Ctimer0capInp {
    pub const fn from_bits(val: u8) -> Ctimer0capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer0capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer0capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0capInp {
        Ctimer0capInp::from_bits(val)
    }
}
impl From<Ctimer0capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0capInp) -> u8 {
        Ctimer0capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer1capInp(u8);
impl Ctimer1capInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Ctimer1capInp {
    pub const fn from_bits(val: u8) -> Ctimer1capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer1capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer1capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1capInp {
        Ctimer1capInp::from_bits(val)
    }
}
impl From<Ctimer1capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1capInp) -> u8 {
        Ctimer1capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer2capInp(u8);
impl Ctimer2capInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Ctimer2capInp {
    pub const fn from_bits(val: u8) -> Ctimer2capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer2capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer2capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2capInp {
        Ctimer2capInp::from_bits(val)
    }
}
impl From<Ctimer2capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2capInp) -> u8 {
        Ctimer2capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer3capInp(u8);
impl Ctimer3capInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected."]
    pub const Val102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected."]
    pub const Val103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Ctimer3capInp {
    pub const fn from_bits(val: u8) -> Ctimer3capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer3capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x66 => f.write_str("Val102"),
            0x67 => f.write_str("Val103"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x66 => defmt::write!(f, "Val102"),
            0x67 => defmt::write!(f, "Val103"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer3capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3capInp {
        Ctimer3capInp::from_bits(val)
    }
}
impl From<Ctimer3capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3capInp) -> u8 {
        Ctimer3capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ctimer4capInp(u8);
impl Ctimer4capInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected."]
    pub const Val102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected."]
    pub const Val103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Ctimer4capInp {
    pub const fn from_bits(val: u8) -> Ctimer4capInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ctimer4capInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x66 => f.write_str("Val102"),
            0x67 => f.write_str("Val103"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4capInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x66 => defmt::write!(f, "Val102"),
            0x67 => defmt::write!(f, "Val103"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ctimer4capInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4capInp {
        Ctimer4capInp::from_bits(val)
    }
}
impl From<Ctimer4capInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4capInp) -> u8 {
        Ctimer4capInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DacTrigTrigin(u8);
impl DacTrigTrigin {
    #[doc = "ARM_TXEV."]
    pub const Val1: Self = Self(0x01);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CTimer0_MAT0 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CTimer1_MAT0 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "CTimer2_MAT0 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "LPTMR0 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "WUU input is selected."]
    pub const Val31: Self = Self(0x1f);
    #[doc = "CTimer3_MAT0 input is selected."]
    pub const Val41: Self = Self(0x29);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val42: Self = Self(0x2a);
    #[doc = "CTimer4_MAT0 input is selected."]
    pub const Val43: Self = Self(0x2b);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val44: Self = Self(0x2c);
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    pub const Val66: Self = Self(0x42);
}
impl DacTrigTrigin {
    pub const fn from_bits(val: u8) -> DacTrigTrigin {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for DacTrigTrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x1f => f.write_str("Val31"),
            0x29 => f.write_str("Val41"),
            0x2a => f.write_str("Val42"),
            0x2b => f.write_str("Val43"),
            0x2c => f.write_str("Val44"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacTrigTrigin {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x1f => defmt::write!(f, "Val31"),
            0x29 => defmt::write!(f, "Val41"),
            0x2a => defmt::write!(f, "Val42"),
            0x2b => defmt::write!(f, "Val43"),
            0x2c => defmt::write!(f, "Val44"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for DacTrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> DacTrigTrigin {
        DacTrigTrigin::from_bits(val)
    }
}
impl From<DacTrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: DacTrigTrigin) -> u8 {
        DacTrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "AOI0_OUT0 input is selected."]
    Val1 = 0x01,
    #[doc = "AOI0_OUT1 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT2 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val5 = 0x05,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val6 = 0x06,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val7 = 0x07,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val8 = 0x08,
    #[doc = "CMP0_OUT input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val17 = 0x11,
    #[doc = "LPTMR0 input is selected."]
    Val18 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val36 = 0x24,
    #[doc = "WUU input is selected."]
    Val37 = 0x25,
    #[doc = "LPI2C0 Master End of Packet."]
    Val38 = 0x26,
    #[doc = "LPI2C0 Slave End of Packet."]
    Val39 = 0x27,
    #[doc = "LPI2C1 Master End of Packet."]
    Val40 = 0x28,
    #[doc = "LPI2C1 Slave End of Packet."]
    Val41 = 0x29,
    #[doc = "LPSPI0 End of Frame."]
    Val42 = 0x2a,
    #[doc = "LPSPI0 Received Data Word."]
    Val43 = 0x2b,
    #[doc = "LPSPI1 End of Frame."]
    Val44 = 0x2c,
    #[doc = "LPSPI1 Received Data Word."]
    Val45 = 0x2d,
    #[doc = "LPUART0 Received Data Word."]
    Val46 = 0x2e,
    #[doc = "LPUART0 Transmitted Data Word."]
    Val47 = 0x2f,
    #[doc = "LPUART0 Receive Line Idle."]
    Val48 = 0x30,
    #[doc = "LPUART1 Received Data Word."]
    Val49 = 0x31,
    #[doc = "LPUART1 Transmitted Data Word."]
    Val50 = 0x32,
    #[doc = "LPUART1 Receive Line Idle."]
    Val51 = 0x33,
    #[doc = "LPUART2 Received Data Word."]
    Val52 = 0x34,
    #[doc = "LPUART2 Transmitted Data Word."]
    Val53 = 0x35,
    #[doc = "LPUART2 Receive Line Idle."]
    Val54 = 0x36,
    #[doc = "LPUART3 Received Data Word."]
    Val55 = 0x37,
    #[doc = "LPUART3 Transmitted Data Word."]
    Val56 = 0x38,
    #[doc = "LPUART3 Receive Line Idle."]
    Val57 = 0x39,
    #[doc = "LPUART4 Received Data Word."]
    Val58 = 0x3a,
    #[doc = "LPUART4 Transmitted Data Word."]
    Val59 = 0x3b,
    #[doc = "LPUART4 Receive Line Idle."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val72 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    #[doc = "LPI2C2 Master End of Packet."]
    Val77 = 0x4d,
    #[doc = "LPI2C2 Slave End of Packet."]
    Val78 = 0x4e,
    #[doc = "LPI2C3 Master End of Packet."]
    Val79 = 0x4f,
    #[doc = "LPI2C3 Slave End of Packet."]
    Val80 = 0x50,
    #[doc = "LPSPI2 End of Frame input is selected."]
    Val81 = 0x51,
    #[doc = "LPSPI2 Received Data Word input is selected."]
    Val82 = 0x52,
    #[doc = "LPSPI3 End of Frame input is selected."]
    Val83 = 0x53,
    #[doc = "LPSPI3 Received Data Word input is selected."]
    Val84 = 0x54,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val85 = 0x55,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val86 = 0x56,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val87 = 0x57,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val88 = 0x58,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val89 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl FlexioTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioTrigInp {
    #[inline(always)]
    fn from(val: u8) -> FlexioTrigInp {
        FlexioTrigInp::from_bits(val)
    }
}
impl From<FlexioTrigInp> for u8 {
    #[inline(always)]
    fn from(val: FlexioTrigInp) -> u8 {
        FlexioTrigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasRefInp(u8);
impl FreqmeasRefInp {
    #[doc = "clk_in input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "OSC32K\\[1\\] input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "pll1_clk_div input is selected."]
    pub const Val31: Self = Self(0x1f);
}
impl FreqmeasRefInp {
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasRefInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x1f => f.write_str("Val31"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRefInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x1f => defmt::write!(f, "Val31"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasRefInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasRefInp {
        FreqmeasRefInp::from_bits(val)
    }
}
impl From<FreqmeasRefInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasRefInp) -> u8 {
        FreqmeasRefInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FreqmeasTarInp(u8);
impl FreqmeasTarInp {
    #[doc = "clk_in input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "FRO_OSC_12M input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "fro_hf_div input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "OSC32K\\[1\\] input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "clk_16k\\[1\\] input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "SLOW_CLK input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "FREQME_CLK_IN0 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "FREQME_CLK_IN1 input is selected input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "AOI0_OUT1."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "pll1_clk_div input is selected."]
    pub const Val31: Self = Self(0x1f);
}
impl FreqmeasTarInp {
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        Self(val & 0x7f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for FreqmeasTarInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x1f => f.write_str("Val31"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTarInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x1f => defmt::write!(f, "Val31"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for FreqmeasTarInp {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasTarInp {
        FreqmeasTarInp::from_bits(val)
    }
}
impl From<FreqmeasTarInp> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasTarInp) -> u8 {
        FreqmeasTarInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT0 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO0 CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO0 CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO0 CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO0 CH3 input is selected."]
    Val42 = 0x2a,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val43 = 0x2b,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val44 = 0x2c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val45 = 0x2d,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val47 = 0x2f,
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
impl Lpi2cTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cTrigInp {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cTrigInp {
        Lpi2cTrigInp::from_bits(val)
    }
}
impl From<Lpi2cTrigInp> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cTrigInp) -> u8 {
        Lpi2cTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiTrigInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "WUU input is selected."]
    Val30 = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    #[doc = "CTimer3_MAT2 inputs is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO0 CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO0 CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO0 CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO0 CH3 input is selected."]
    Val42 = 0x2a,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val43 = 0x2b,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val44 = 0x2c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val45 = 0x2d,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val47 = 0x2f,
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
impl LpspiTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiTrigInp {
    #[inline(always)]
    fn from(val: u8) -> LpspiTrigInp {
        LpspiTrigInp::from_bits(val)
    }
}
impl From<LpspiTrigInp> for u8 {
    #[inline(always)]
    fn from(val: LpspiTrigInp) -> u8 {
        LpspiTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpuartInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR0 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "TRIG_IN0 input is selected."]
    Val17 = 0x11,
    #[doc = "TRIG_IN1 input is selected."]
    Val18 = 0x12,
    #[doc = "TRIG_IN2 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN3 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN4 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN5 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN6 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN7 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN8 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN9 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN10 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN11 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "WUU selected."]
    Val34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO0 CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO0 CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO0 CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO0 CH3 input is selected."]
    Val47 = 0x2f,
    #[doc = "GPIO0 Pin Event Trig 1 input is selected."]
    Val48 = 0x30,
    #[doc = "GPIO1 Pin Event Trig 1 input is selected."]
    Val49 = 0x31,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val50 = 0x32,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "GPIO4 Pin Event Trig 1 input is selected."]
    Val52 = 0x34,
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
impl LpuartInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpuartInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpuartInp {
    #[inline(always)]
    fn from(val: u8) -> LpuartInp {
        LpuartInp::from_bits(val)
    }
}
impl From<LpuartInp> for u8 {
    #[inline(always)]
    fn from(val: LpuartInp) -> u8 {
        LpuartInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "GPIO P0_16 input is selected."]
    Val1 = 0x01,
    #[doc = "GPIO P0_17 input is selected."]
    Val2 = 0x02,
    #[doc = "GPIO P1_8 input is selected."]
    Val3 = 0x03,
    #[doc = "GPIO P1_9 input is selected."]
    Val4 = 0x04,
    #[doc = "GPIO P1_10 input is selected."]
    Val5 = 0x05,
    #[doc = "GPIO P1_11 input is selected."]
    Val6 = 0x06,
    #[doc = "GPIO P1_12 input is selected."]
    Val7 = 0x07,
    #[doc = "GPIO P1_13 input is selected."]
    Val8 = 0x08,
    #[doc = "GPIO P2_0 input is selected."]
    Val9 = 0x09,
    #[doc = "GPIO P2_1 input is selected."]
    Val10 = 0x0a,
    #[doc = "GPIO P2_2 input is selected."]
    Val11 = 0x0b,
    #[doc = "GPIO P2_3 input is selected."]
    Val12 = 0x0c,
    #[doc = "GPIO P2_6 input is selected."]
    Val13 = 0x0d,
    #[doc = "GPIO P3_8 input is selected."]
    Val14 = 0x0e,
    #[doc = "GPIO P3_9 input is selected."]
    Val15 = 0x0f,
    #[doc = "GPIO P3_10 input is selected."]
    Val16 = 0x10,
    #[doc = "GPIO P3_11 input is selected."]
    Val17 = 0x11,
    #[doc = "GPIO P3_12 input is seclected."]
    Val18 = 0x12,
    #[doc = "GPIO0 Pin Event Trig input is selected."]
    Val19 = 0x13,
    #[doc = "GPIO1 Pin Event Trig input is selected."]
    Val20 = 0x14,
    #[doc = "GPIO2 Pin Event Trig input is selected."]
    Val21 = 0x15,
    #[doc = "GPIO3 Pin Event Trig input is selected."]
    Val22 = 0x16,
    #[doc = "GPIO4 Pin Event Trig input is selected."]
    Val23 = 0x17,
    #[doc = "ARM_TXEV input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT0 input is selected."]
    Val25 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "DMA_IRQ input is selected."]
    Val27 = 0x1b,
    _RESERVED_1c = 0x1c,
    #[doc = "WUU_IRQ input is selected."]
    Val29 = 0x1d,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val30 = 0x1e,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val31 = 0x1f,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val32 = 0x20,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val38 = 0x26,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val39 = 0x27,
    #[doc = "OSTIMER_IRQ input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "RTC_Alarm_IRQ input is selected."]
    Val45 = 0x2d,
    #[doc = "RTC_1Hz_IRQ input is selected."]
    Val46 = 0x2e,
    #[doc = "uTICK_IRQ input is selected."]
    Val47 = 0x2f,
    #[doc = "WDT_IRQ input is selected."]
    Val48 = 0x30,
    #[doc = "Wakeup_Timer_IRQ input is selected."]
    Val49 = 0x31,
    #[doc = "CAN0_IRQ input is selected."]
    Val50 = 0x32,
    #[doc = "CAN1_IRQ input is selected."]
    Val51 = 0x33,
    #[doc = "FlexIO0_IRQ input is selected."]
    Val52 = 0x34,
    #[doc = "FlexIO0_Shifer0_DMA_Req input is selected."]
    Val53 = 0x35,
    #[doc = "FlexIO0_Shifer1_DMA_Req input is selected."]
    Val54 = 0x36,
    #[doc = "FlexIO0_Shifer2_DMA_Req input is selected."]
    Val55 = 0x37,
    #[doc = "FlexIO0_Shifer3_DMA_Req input is selected."]
    Val56 = 0x38,
    #[doc = "I3C0_IRQ input is selected."]
    Val57 = 0x39,
    #[doc = "LPI2C0_IRQ input is selected."]
    Val58 = 0x3a,
    #[doc = "LPI2C1_IRQ input is selected."]
    Val59 = 0x3b,
    #[doc = "LPSPI0_IRQ input is selected."]
    Val60 = 0x3c,
    #[doc = "LPSPI1_IRQ input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART0_IRQ input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART1_IRQ input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2_IRQ input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3_IRQ input is selected."]
    Val65 = 0x41,
    _RESERVED_42 = 0x42,
    #[doc = "USB1 Start of Frame input is selected."]
    Val67 = 0x43,
    #[doc = "ADC0_IRQ input is selected."]
    Val68 = 0x44,
    #[doc = "ADC1_IRQ input is selected."]
    Val69 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    #[doc = "CMP0_IRQ input is selected."]
    Val72 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    #[doc = "CMP0_OUT input is selected."]
    Val75 = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    #[doc = "DAC0_IRQ input is selected."]
    Val78 = 0x4e,
    _RESERVED_4f = 0x4f,
    #[doc = "DMA1_IRQ input is selected."]
    Val80 = 0x50,
    #[doc = "DAC1_IRQ input is selected."]
    Val81 = 0x51,
    #[doc = "TSI0_End_of_Scan_IRQ input is selected."]
    Val82 = 0x52,
    #[doc = "TSI0_Out_of_Range_IRQ input is selected."]
    Val83 = 0x53,
    #[doc = "ENET QOS IRQ input is selected."]
    Val84 = 0x54,
    #[doc = "10BASE_T1S IRQ input is selected."]
    Val85 = 0x55,
    #[doc = "ERM Interrupt input is selected."]
    Val86 = 0x56,
    #[doc = "TMPR_OUT0 input is selected."]
    Val87 = 0x57,
    #[doc = "TMPR_OUT1 input is selected."]
    Val88 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl SmartDmaTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaTrigInp {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaTrigInp {
        SmartDmaTrigInp::from_bits(val)
    }
}
impl From<SmartDmaTrigInp> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaTrigInp) -> u8 {
        SmartDmaTrigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer0trigInp(u8);
impl Timer0trigInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Timer0trigInp {
    pub const fn from_bits(val: u8) -> Timer0trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer0trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer0trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer0trigInp {
        Timer0trigInp::from_bits(val)
    }
}
impl From<Timer0trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer0trigInp) -> u8 {
        Timer0trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer1trigInp(u8);
impl Timer1trigInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Timer1trigInp {
    pub const fn from_bits(val: u8) -> Timer1trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer1trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer1trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer1trigInp {
        Timer1trigInp::from_bits(val)
    }
}
impl From<Timer1trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer1trigInp) -> u8 {
        Timer1trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer2trigInp(u8);
impl Timer2trigInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\]."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\]."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\]."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer3_MAT2 is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Timer2trigInp {
    pub const fn from_bits(val: u8) -> Timer2trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer2trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer2trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer2trigInp {
        Timer2trigInp::from_bits(val)
    }
}
impl From<Timer2trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer2trigInp) -> u8 {
        Timer2trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer3trigInp(u8);
impl Timer3trigInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer4_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer4_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer4_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected."]
    pub const Val102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected."]
    pub const Val103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 input is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 input is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Timer3trigInp {
    pub const fn from_bits(val: u8) -> Timer3trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer3trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x66 => f.write_str("Val102"),
            0x67 => f.write_str("Val103"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x66 => defmt::write!(f, "Val102"),
            0x67 => defmt::write!(f, "Val103"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer3trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer3trigInp {
        Timer3trigInp::from_bits(val)
    }
}
impl From<Timer3trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer3trigInp) -> u8 {
        Timer3trigInp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Timer4trigInp(u8);
impl Timer4trigInp {
    #[doc = "CT_INP0 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "CT_INP1 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "CT_INP2 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "CT_INP3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CT_INP4 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CT_INP5 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CT_INP6 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CT_INP7 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CT_INP8 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "CT_INP9 input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "CT_INP10 input is selected."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "CT_INP12 input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "CT_INP13 input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "CT_INP14 input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "CT_INP15 input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "CT_INP16 input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "CT_INP17 input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "CT_INP18 input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "CT_INP19 input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "AOI0_OUT0 input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "AOI0_OUT1 input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "AOI0_OUT2 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "AOI0_OUT3 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "CTimer0_MAT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "CTimer0_MAT2 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "CTimer0_MAT3 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "CTimer1_MAT1 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "CTimer1_MAT2 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "CTimer1_MAT3 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "LPSPI0 End of Frame input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "LPSPI0 Received Data Word input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "LPSPI1 End of Frame input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "LPSPI1 Received Data Word input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "LPUART0 Received Data Word input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "LPUART1 Received Data Word input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "LPUART2 Received Data Word input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "LPUART3 Received Data Word input is selected."]
    pub const Val65: Self = Self(0x41);
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    pub const Val66: Self = Self(0x42);
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    pub const Val67: Self = Self(0x43);
    #[doc = "LPUART4 Received Data Word input is selected."]
    pub const Val68: Self = Self(0x44);
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    pub const Val69: Self = Self(0x45);
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    pub const Val70: Self = Self(0x46);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val75: Self = Self(0x4b);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val76: Self = Self(0x4c);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val77: Self = Self(0x4d);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val78: Self = Self(0x4e);
    #[doc = "CTimer2_MAT1 input is selected."]
    pub const Val79: Self = Self(0x4f);
    #[doc = "CTimer2_MAT2 input is selected."]
    pub const Val80: Self = Self(0x50);
    #[doc = "CTimer2_MAT3 input is selected."]
    pub const Val81: Self = Self(0x51);
    #[doc = "CTimer3_MAT1 input is selected."]
    pub const Val82: Self = Self(0x52);
    #[doc = "CTimer3_MAT2 input is selected."]
    pub const Val83: Self = Self(0x53);
    #[doc = "CTimer3_MAT3 input is selected."]
    pub const Val84: Self = Self(0x54);
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    pub const Val94: Self = Self(0x5e);
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    pub const Val95: Self = Self(0x5f);
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    pub const Val96: Self = Self(0x60);
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    pub const Val97: Self = Self(0x61);
    #[doc = "LPUART5 Received Data Word input is selected."]
    pub const Val98: Self = Self(0x62);
    #[doc = "LPUART5 Transmitted Data Word input is selected."]
    pub const Val99: Self = Self(0x63);
    #[doc = "LPUART5 Receive Line Idle input is selected."]
    pub const Val100: Self = Self(0x64);
    #[doc = "TMPR_OUT0 input is selected."]
    pub const Val102: Self = Self(0x66);
    #[doc = "TMPR_OUT1 input is selected."]
    pub const Val103: Self = Self(0x67);
    #[doc = "TRIG_IN0 input is selected."]
    pub const Val113: Self = Self(0x71);
    #[doc = "TRIG_IN1 input is selected."]
    pub const Val114: Self = Self(0x72);
    #[doc = "TRIG_IN2 input is selected."]
    pub const Val115: Self = Self(0x73);
    #[doc = "TRIG_IN3 input is selected."]
    pub const Val116: Self = Self(0x74);
    #[doc = "TRIG_IN4 is selected."]
    pub const Val117: Self = Self(0x75);
    #[doc = "TRIG_IN5 is selected."]
    pub const Val118: Self = Self(0x76);
    #[doc = "TRIG_IN6 input is selected."]
    pub const Val119: Self = Self(0x77);
    #[doc = "TRIG_IN7 input is selected."]
    pub const Val120: Self = Self(0x78);
    #[doc = "TRIG_IN8 input is selected."]
    pub const Val121: Self = Self(0x79);
    #[doc = "TRIG_IN9 input is selected."]
    pub const Val122: Self = Self(0x7a);
    #[doc = "TRIG_IN10 input is selected."]
    pub const Val123: Self = Self(0x7b);
    #[doc = "TRIG_IN11 input is selected."]
    pub const Val124: Self = Self(0x7c);
    #[doc = "USB1 Start of Frame input is selected."]
    pub const Val125: Self = Self(0x7d);
    #[doc = "LPSPI2 End of Frame input is selected."]
    pub const Val126: Self = Self(0x7e);
    #[doc = "LPSPI2 Received Data Word input is selected."]
    pub const Val127: Self = Self(0x7f);
    #[doc = "LPSPI3 End of Frame input is selected."]
    pub const Val128: Self = Self(0x80);
    #[doc = "LPSPI3 Received Data Word input is selected."]
    pub const Val129: Self = Self(0x81);
    #[doc = "LPSPI4 End of Frame input is selected."]
    pub const Val130: Self = Self(0x82);
    #[doc = "LPSPI4 Received Data Word input is selected."]
    pub const Val131: Self = Self(0x83);
    #[doc = "LPSPI5 End of Frame input is selected."]
    pub const Val132: Self = Self(0x84);
    #[doc = "LPSPI5 Received Data Word input is selected."]
    pub const Val133: Self = Self(0x85);
}
impl Timer4trigInp {
    pub const fn from_bits(val: u8) -> Timer4trigInp {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Timer4trigInp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Val1"),
            0x02 => f.write_str("Val2"),
            0x03 => f.write_str("Val3"),
            0x04 => f.write_str("Val4"),
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x30 => f.write_str("Val48"),
            0x31 => f.write_str("Val49"),
            0x32 => f.write_str("Val50"),
            0x33 => f.write_str("Val51"),
            0x34 => f.write_str("Val52"),
            0x35 => f.write_str("Val53"),
            0x36 => f.write_str("Val54"),
            0x37 => f.write_str("Val55"),
            0x38 => f.write_str("Val56"),
            0x39 => f.write_str("Val57"),
            0x3a => f.write_str("Val58"),
            0x3b => f.write_str("Val59"),
            0x3c => f.write_str("Val60"),
            0x3d => f.write_str("Val61"),
            0x3e => f.write_str("Val62"),
            0x3f => f.write_str("Val63"),
            0x40 => f.write_str("Val64"),
            0x41 => f.write_str("Val65"),
            0x42 => f.write_str("Val66"),
            0x43 => f.write_str("Val67"),
            0x44 => f.write_str("Val68"),
            0x45 => f.write_str("Val69"),
            0x46 => f.write_str("Val70"),
            0x4b => f.write_str("Val75"),
            0x4c => f.write_str("Val76"),
            0x4d => f.write_str("Val77"),
            0x4e => f.write_str("Val78"),
            0x4f => f.write_str("Val79"),
            0x50 => f.write_str("Val80"),
            0x51 => f.write_str("Val81"),
            0x52 => f.write_str("Val82"),
            0x53 => f.write_str("Val83"),
            0x54 => f.write_str("Val84"),
            0x5e => f.write_str("Val94"),
            0x5f => f.write_str("Val95"),
            0x60 => f.write_str("Val96"),
            0x61 => f.write_str("Val97"),
            0x62 => f.write_str("Val98"),
            0x63 => f.write_str("Val99"),
            0x64 => f.write_str("Val100"),
            0x66 => f.write_str("Val102"),
            0x67 => f.write_str("Val103"),
            0x71 => f.write_str("Val113"),
            0x72 => f.write_str("Val114"),
            0x73 => f.write_str("Val115"),
            0x74 => f.write_str("Val116"),
            0x75 => f.write_str("Val117"),
            0x76 => f.write_str("Val118"),
            0x77 => f.write_str("Val119"),
            0x78 => f.write_str("Val120"),
            0x79 => f.write_str("Val121"),
            0x7a => f.write_str("Val122"),
            0x7b => f.write_str("Val123"),
            0x7c => f.write_str("Val124"),
            0x7d => f.write_str("Val125"),
            0x7e => f.write_str("Val126"),
            0x7f => f.write_str("Val127"),
            0x80 => f.write_str("Val128"),
            0x81 => f.write_str("Val129"),
            0x82 => f.write_str("Val130"),
            0x83 => f.write_str("Val131"),
            0x84 => f.write_str("Val132"),
            0x85 => f.write_str("Val133"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4trigInp {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Val1"),
            0x02 => defmt::write!(f, "Val2"),
            0x03 => defmt::write!(f, "Val3"),
            0x04 => defmt::write!(f, "Val4"),
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x30 => defmt::write!(f, "Val48"),
            0x31 => defmt::write!(f, "Val49"),
            0x32 => defmt::write!(f, "Val50"),
            0x33 => defmt::write!(f, "Val51"),
            0x34 => defmt::write!(f, "Val52"),
            0x35 => defmt::write!(f, "Val53"),
            0x36 => defmt::write!(f, "Val54"),
            0x37 => defmt::write!(f, "Val55"),
            0x38 => defmt::write!(f, "Val56"),
            0x39 => defmt::write!(f, "Val57"),
            0x3a => defmt::write!(f, "Val58"),
            0x3b => defmt::write!(f, "Val59"),
            0x3c => defmt::write!(f, "Val60"),
            0x3d => defmt::write!(f, "Val61"),
            0x3e => defmt::write!(f, "Val62"),
            0x3f => defmt::write!(f, "Val63"),
            0x40 => defmt::write!(f, "Val64"),
            0x41 => defmt::write!(f, "Val65"),
            0x42 => defmt::write!(f, "Val66"),
            0x43 => defmt::write!(f, "Val67"),
            0x44 => defmt::write!(f, "Val68"),
            0x45 => defmt::write!(f, "Val69"),
            0x46 => defmt::write!(f, "Val70"),
            0x4b => defmt::write!(f, "Val75"),
            0x4c => defmt::write!(f, "Val76"),
            0x4d => defmt::write!(f, "Val77"),
            0x4e => defmt::write!(f, "Val78"),
            0x4f => defmt::write!(f, "Val79"),
            0x50 => defmt::write!(f, "Val80"),
            0x51 => defmt::write!(f, "Val81"),
            0x52 => defmt::write!(f, "Val82"),
            0x53 => defmt::write!(f, "Val83"),
            0x54 => defmt::write!(f, "Val84"),
            0x5e => defmt::write!(f, "Val94"),
            0x5f => defmt::write!(f, "Val95"),
            0x60 => defmt::write!(f, "Val96"),
            0x61 => defmt::write!(f, "Val97"),
            0x62 => defmt::write!(f, "Val98"),
            0x63 => defmt::write!(f, "Val99"),
            0x64 => defmt::write!(f, "Val100"),
            0x66 => defmt::write!(f, "Val102"),
            0x67 => defmt::write!(f, "Val103"),
            0x71 => defmt::write!(f, "Val113"),
            0x72 => defmt::write!(f, "Val114"),
            0x73 => defmt::write!(f, "Val115"),
            0x74 => defmt::write!(f, "Val116"),
            0x75 => defmt::write!(f, "Val117"),
            0x76 => defmt::write!(f, "Val118"),
            0x77 => defmt::write!(f, "Val119"),
            0x78 => defmt::write!(f, "Val120"),
            0x79 => defmt::write!(f, "Val121"),
            0x7a => defmt::write!(f, "Val122"),
            0x7b => defmt::write!(f, "Val123"),
            0x7c => defmt::write!(f, "Val124"),
            0x7d => defmt::write!(f, "Val125"),
            0x7e => defmt::write!(f, "Val126"),
            0x7f => defmt::write!(f, "Val127"),
            0x80 => defmt::write!(f, "Val128"),
            0x81 => defmt::write!(f, "Val129"),
            0x82 => defmt::write!(f, "Val130"),
            0x83 => defmt::write!(f, "Val131"),
            0x84 => defmt::write!(f, "Val132"),
            0x85 => defmt::write!(f, "Val133"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Timer4trigInp {
    #[inline(always)]
    fn from(val: u8) -> Timer4trigInp {
        Timer4trigInp::from_bits(val)
    }
}
impl From<Timer4trigInp> for u8 {
    #[inline(always)]
    fn from(val: Timer4trigInp) -> u8 {
        Timer4trigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrigOutInp {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AOI0_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "AOI0_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CMP0_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "LPUART0 ipp_do_lpuart_txd input is selected."]
    Val9 = 0x09,
    #[doc = "LPUART1 ipp_do_lpuart_txd input is selected."]
    Val10 = 0x0a,
    #[doc = "LPUART2 ipp_do_lpuart_txd input is selected."]
    Val11 = 0x0b,
    #[doc = "LPUART3 ipp_do_lpuart_txd input is selected."]
    Val12 = 0x0c,
    #[doc = "LPUART4 ipp_do_lpuart_txd input is selected."]
    Val13 = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "ARM_TXEV input is selected."]
    Val16 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "LPUART5 ipp_do_lpuart_txd input is selected."]
    Val18 = 0x12,
    #[doc = "RTC_1Hz_CLK input is selected."]
    Val19 = 0x13,
    _RESERVED_14 = 0x14,
    #[doc = "USB1 Start of Frame input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN0 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN1 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN2 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN3 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN4 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN5 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN6 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN7 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO0 Pin Event Trig 0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val35 = 0x23,
    #[doc = "WUU input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_A0_TRIG0 input is selected."]
    Val37 = 0x25,
    #[doc = "LPI2C0 Master End of Packet."]
    Val38 = 0x26,
    #[doc = "LPI2C0 Slave End of Packet."]
    Val39 = 0x27,
    #[doc = "LPI2C1 Master End of Packet."]
    Val40 = 0x28,
    #[doc = "LPI2C1 Slave End of Packet."]
    Val41 = 0x29,
    #[doc = "LPSPI0 End of Frame."]
    Val42 = 0x2a,
    #[doc = "LPSPI0 Received Data Word."]
    Val43 = 0x2b,
    #[doc = "LPSPI1 End of Frame."]
    Val44 = 0x2c,
    #[doc = "LPSPI1 Received Data Word."]
    Val45 = 0x2d,
    #[doc = "LPUART0 Received Data Word."]
    Val46 = 0x2e,
    #[doc = "LPUART0 Transmitted Data Word."]
    Val47 = 0x2f,
    #[doc = "LPUART0 Receive Line Idle."]
    Val48 = 0x30,
    #[doc = "LPUART1 Received Data Word."]
    Val49 = 0x31,
    #[doc = "LPUART1 Transmitted Data Word."]
    Val50 = 0x32,
    #[doc = "LPUART1 Receive Line Idle."]
    Val51 = 0x33,
    #[doc = "LPUART2 Received Data Word."]
    Val52 = 0x34,
    #[doc = "LPUART2 Transmitted Data Word."]
    Val53 = 0x35,
    #[doc = "LPUART2 Receive Line Idle."]
    Val54 = 0x36,
    #[doc = "LPUART3 Received Data Word."]
    Val55 = 0x37,
    #[doc = "LPUART3 Transmitted Data Word."]
    Val56 = 0x38,
    #[doc = "LPUART3 Receive Line Idle."]
    Val57 = 0x39,
    #[doc = "LPUART4 Received Data Word."]
    Val58 = 0x3a,
    #[doc = "LPUART4 Transmitted Data Word."]
    Val59 = 0x3b,
    #[doc = "LPUART4 Receive Line Idle."]
    Val60 = 0x3c,
    #[doc = "AOI1_OUT0 input is selected."]
    Val61 = 0x3d,
    #[doc = "AOI1_OUT1 input is selected."]
    Val62 = 0x3e,
    #[doc = "AOI1_OUT2 input is selected."]
    Val63 = 0x3f,
    #[doc = "AOI1_OUT3 input is selected."]
    Val64 = 0x40,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val65 = 0x41,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val66 = 0x42,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val67 = 0x43,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val68 = 0x44,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val69 = 0x45,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val70 = 0x46,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val71 = 0x47,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val73 = 0x49,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val74 = 0x4a,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val75 = 0x4b,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val76 = 0x4c,
    #[doc = "LPI2C2 Master End of Packet."]
    Val77 = 0x4d,
    #[doc = "LPI2C2 Slave End of Packet."]
    Val78 = 0x4e,
    #[doc = "LPI2C3 Master End of Packet."]
    Val79 = 0x4f,
    #[doc = "LPI2C3 Slave End of Packet."]
    Val80 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
}
impl TrigOutInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrigOutInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrigOutInp {
    #[inline(always)]
    fn from(val: u8) -> TrigOutInp {
        TrigOutInp::from_bits(val)
    }
}
impl From<TrigOutInp> for u8 {
    #[inline(always)]
    fn from(val: TrigOutInp) -> u8 {
        TrigOutInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsi0TrigInputInp {
    _RESERVED_0 = 0x0,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val1 = 0x01,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val2 = 0x02,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val3 = 0x03,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val4 = 0x04,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val5 = 0x05,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val7 = 0x07,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "WUU input is selected."]
    Val12 = 0x0c,
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
impl Tsi0TrigInputInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsi0TrigInputInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsi0TrigInputInp {
    #[inline(always)]
    fn from(val: u8) -> Tsi0TrigInputInp {
        Tsi0TrigInputInp::from_bits(val)
    }
}
impl From<Tsi0TrigInputInp> for u8 {
    #[inline(always)]
    fn from(val: Tsi0TrigInputInp) -> u8 {
        Tsi0TrigInputInp::to_bits(val)
    }
}
