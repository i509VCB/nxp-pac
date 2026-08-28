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
    #[doc = "Selection for frequency measurement reference clock."]
    #[inline(always)]
    pub const fn freqmeas_ref(
        self,
    ) -> crate::pac::common::Reg<FreqmeasRef, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement target clock."]
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
    #[doc = "AOI1 trigger input connections 0."]
    #[inline(always)]
    pub const fn aoi1_input(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Aoi1Input, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
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
    #[doc = "This register selects the DAC0 trigger inputs."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::pac::common::Reg<DacTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_trig(self) -> crate::pac::common::Reg<Qdc0Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_home(self) -> crate::pac::common::Reg<Qdc0Home, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_index(self) -> crate::pac::common::Reg<Qdc0Index, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phaseb(self) -> crate::pac::common::Reg<Qdc0Phaseb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_phasea(self) -> crate::pac::common::Reg<Qdc0Phasea, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap1(self) -> crate::pac::common::Reg<Qdc0Icap1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap2(self) -> crate::pac::common::Reg<Qdc0Icap2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc0_icap3(self) -> crate::pac::common::Reg<Qdc0Icap3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_trig(self) -> crate::pac::common::Reg<Qdc1Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_home(self) -> crate::pac::common::Reg<Qdc1Home, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_index(self) -> crate::pac::common::Reg<Qdc1Index, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phaseb(self) -> crate::pac::common::Reg<Qdc1Phaseb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_phasea(self) -> crate::pac::common::Reg<Qdc1Phasea, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap1(self) -> crate::pac::common::Reg<Qdc1Icap1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap2(self) -> crate::pac::common::Reg<Qdc1Icap2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc1_icap3(self) -> crate::pac::common::Reg<Qdc1Icap3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "PWM0 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm0_force(self) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_exta0(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_exta(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_extsync(
        self,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "PWM1 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 input trigger connections."]
    #[inline(always)]
    pub const fn flex_pwm1_force(self) -> crate::pac::common::Reg<FlexPwm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "PWM0 external clock trigger."]
    #[inline(always)]
    pub const fn pwm0_ext_clk(self) -> crate::pac::common::Reg<Pwm0ExtClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "PWM1 external clock trigger."]
    #[inline(always)]
    pub const fn pwm1_ext_clk(self) -> crate::pac::common::Reg<Pwm1ExtClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
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
    #[doc = "USB-FS trigger input connections."]
    #[inline(always)]
    pub const fn usbfs_trig(self) -> crate::pac::common::Reg<UsbfsTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "EXT trigger connections."]
    #[inline(always)]
    pub const fn ext_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ExtTrig, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize + n * 4usize) as _)
        }
    }
    #[doc = "CMP1 input connections."]
    #[inline(always)]
    pub const fn cmp1_trig(self) -> crate::pac::common::Reg<CmpTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "LPI2C2 trigger input connections."]
    #[inline(always)]
    pub const fn lpi2c2_trig(self) -> crate::pac::common::Reg<Lpi2cTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "OPAMP0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn opamp0_trig(self) -> crate::pac::common::Reg<Opamp0Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
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
        assert!(n < 5usize);
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
#[doc = "AOI1 trigger input connections 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aoi1Input(pub u32);
impl Aoi1Input {
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
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer0capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer0capInp) {
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
    pub const fn inp(&self) -> Ctimer1capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer1capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer1capInp) {
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
    pub const fn inp(&self) -> Ctimer2capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer2capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer2capInp) {
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
    pub const fn inp(&self) -> Ctimer3capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer3capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer3capInp) {
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
    pub const fn inp(&self) -> Ctimer4capInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer4capInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer4capInp) {
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
pub struct DacTrig(pub u32);
impl DacTrig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> DacTrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        DacTrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: DacTrigTrigin) {
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
#[doc = "EXT trigger connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig(pub u32);
impl ExtTrig {
    #[doc = "EXT trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> ExtTrigInp {
        let val = (self.0 >> 0usize) & 0x1f;
        ExtTrigInp::from_bits(val as u8)
    }
    #[doc = "EXT trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: ExtTrigInp) {
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
#[doc = "PWM0 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm(pub u32);
impl FlexPwm {
    #[doc = "FAULT input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwmTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwmTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwmTrigin) {
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
#[doc = "Selection for frequency measurement target clock."]
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
#[doc = "OPAMP0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opamp0Trig(pub u32);
impl Opamp0Trig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> OpampTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        OpampTrigInp::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: OpampTrigInp) {
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
    pub const fn trigin(&self) -> Pwm0ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        Pwm0ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Pwm0ExtClkTrigin) {
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
    pub const fn trigin(&self) -> Pwm1ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        Pwm1ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "Trigger input connections for PWM."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Pwm1ExtClkTrigin) {
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
    pub const fn inp(&self) -> QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcHomeInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIndexInp) {
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
    pub const fn inp(&self) -> Qdc0PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc0PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc0PhaseaInp) {
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
    pub const fn inp(&self) -> Qdc0PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc0PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc0PhasebInp) {
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
    pub const fn inp(&self) -> QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcTrigInp) {
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
    pub const fn inp(&self) -> QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcHomeInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIcapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIcapInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIcapInp) {
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
    pub const fn inp(&self) -> QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIndexInp) {
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
    pub const fn inp(&self) -> Qdc1PhaseaInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc1PhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc1PhaseaInp) {
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
    pub const fn inp(&self) -> Qdc1PhasebInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Qdc1PhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Qdc1PhasebInp) {
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
    pub const fn inp(&self) -> QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC1 input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcTrigInp) {
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
    pub const fn inp(&self) -> TimertrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TimertrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER0."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TimertrigInp) {
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
    pub const fn inp(&self) -> TimertrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TimertrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER1."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TimertrigInp) {
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
    pub const fn inp(&self) -> TimertrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TimertrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER2."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TimertrigInp) {
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
    pub const fn inp(&self) -> TimertrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TimertrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER3."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TimertrigInp) {
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
    pub const fn inp(&self) -> TimertrigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        TimertrigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER4."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TimertrigInp) {
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
    pub const fn inp(&self) -> UsbfsTrigInp {
        let val = (self.0 >> 0usize) & 0x0f;
        UsbfsTrigInp::from_bits(val as u8)
    }
    #[doc = "USB-FS trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: UsbfsTrigInp) {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_OUT_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM0_OUT_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_OUT_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_OUT_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_OUT_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM2_OUT_TRIG1 input is selected."]
    Val23 = 0x17,
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
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
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
    #[doc = "FlexIO CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
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
    #[doc = "CMP1_OUT input is selected."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "CTimer0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT0."]
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
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val22 = 0x16,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val23 = 0x17,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val25 = 0x19,
    #[doc = "QDC0_POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 0 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
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
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val52 = 0x34,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val53 = 0x35,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val54 = 0x36,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val55 = 0x37,
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
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val68 = 0x44,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val69 = 0x45,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val70 = 0x46,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val71 = 0x47,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val72 = 0x48,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val73 = 0x49,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val74 = 0x4a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val75 = 0x4b,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val76 = 0x4c,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val77 = 0x4d,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val78 = 0x4e,
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
    #[doc = "CMP1_OUT input is selected."]
    Val6 = 0x06,
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
    #[doc = "QDC0_POS_MATCH0."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
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
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    #[doc = "CTimer3_MAT0."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT1."]
    Val40 = 0x28,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val42 = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val47 = 0x2f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val48 = 0x30,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val49 = 0x31,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl Ctimer0capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl Ctimer1capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl Ctimer2capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl Ctimer3capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4capInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer0_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl Ctimer4capInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4capInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacTrigTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "ARM_TXEV."]
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "WUU input is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
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
impl DacTrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacTrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
pub enum ExtTrigInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "LPUART0 input is selected."]
    Val9 = 0x09,
    #[doc = "LPUART1 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPUART2 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPUART3 input is selected."]
    Val12 = 0x0c,
    #[doc = "LPUART4 input is selected."]
    Val13 = 0x0d,
    #[doc = "AOI1_OUT0 input is selected."]
    Val14 = 0x0e,
    #[doc = "AOI1_OUT1 input is selected."]
    Val15 = 0x0f,
    #[doc = "AOI1_OUT2 input is selected."]
    Val16 = 0x10,
    #[doc = "AOI1_OUT3 input is selected."]
    Val17 = 0x11,
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
}
impl ExtTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtTrigInp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtTrigInp {
    #[inline(always)]
    fn from(val: u8) -> ExtTrigInp {
        ExtTrigInp::from_bits(val)
    }
}
impl From<ExtTrigInp> for u8 {
    #[inline(always)]
    fn from(val: ExtTrigInp) -> u8 {
        ExtTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwmTrigin {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "QDC0_CMP_FLAG0 input is selected."]
    Val15 = 0x0f,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val16 = 0x10,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val17 = 0x11,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val18 = 0x12,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val19 = 0x13,
    #[doc = "TRIG_IN0 input is selected."]
    Val20 = 0x14,
    #[doc = "TRIG_IN1 input is selected."]
    Val21 = 0x15,
    #[doc = "TRIG_IN2 input is selected."]
    Val22 = 0x16,
    #[doc = "TRIG_IN3 input is selected."]
    Val23 = 0x17,
    #[doc = "TRIG_IN4 input is selected."]
    Val24 = 0x18,
    #[doc = "TRIG_IN5 input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN6 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN7 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN8 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN9 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN10 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN11 input is selected."]
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
    #[doc = "AOI1_OUT0 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT1 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT2 input is selected."]
    Val39 = 0x27,
    #[doc = "AOI1_OUT3 input is selected."]
    Val40 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val45 = 0x2d,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val46 = 0x2e,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val47 = 0x2f,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val48 = 0x30,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val49 = 0x31,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val50 = 0x32,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val51 = 0x33,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val52 = 0x34,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val56 = 0x38,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val58 = 0x3a,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val59 = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwmTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwmTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwmTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwmTrigin {
        FlexPwmTrigin::from_bits(val)
    }
}
impl From<FlexPwmTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwmTrigin) -> u8 {
        FlexPwmTrigin::to_bits(val)
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
    #[doc = "CMP1_OUT input is selected."]
    Val10 = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "CTimer0_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val14 = 0x0e,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val15 = 0x0f,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val16 = 0x10,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val17 = 0x11,
    #[doc = "LPTMR0 input is selected."]
    Val18 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val22 = 0x16,
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
    _RESERVED_4c = 0x4c,
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
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const Val16: Self = Self(0x10);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
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
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
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
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
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
    #[doc = "PWM0_SM0_MUX_TRIG0."]
    pub const Val11: Self = Self(0x0b);
    #[doc = "PWM0_SM0_MUX_TRIG1."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "PWM0_SM1_MUX_TRIG0."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "PWM0_SM1_MUX_TRIG1."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "PWM0_SM2_MUX_TRIG0."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "PWM0_SM2_MUX_TRIG1."]
    pub const Val16: Self = Self(0x10);
    #[doc = "AOI1_OUT0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "AOI1_OUT1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
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
            0x05 => f.write_str("Val5"),
            0x06 => f.write_str("Val6"),
            0x07 => f.write_str("Val7"),
            0x08 => f.write_str("Val8"),
            0x09 => f.write_str("Val9"),
            0x0a => f.write_str("Val10"),
            0x0b => f.write_str("Val11"),
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
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
            0x05 => defmt::write!(f, "Val5"),
            0x06 => defmt::write!(f, "Val6"),
            0x07 => defmt::write!(f, "Val7"),
            0x08 => defmt::write!(f, "Val8"),
            0x09 => defmt::write!(f, "Val9"),
            0x0a => defmt::write!(f, "Val10"),
            0x0b => defmt::write!(f, "Val11"),
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "AOI1_OUT0 input is selected."]
    Val31 = 0x1f,
    #[doc = "AOI1_OUT1 input is selected."]
    Val32 = 0x20,
    #[doc = "AOI1_OUT2 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT3 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "FlexIO CH0 input is selected."]
    Val39 = 0x27,
    #[doc = "FlexIO CH1 input is selected."]
    Val40 = 0x28,
    #[doc = "FlexIO CH2 input is selected."]
    Val41 = 0x29,
    #[doc = "FlexIO CH3 input is selected."]
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT0 input is selected."]
    Val36 = 0x24,
    #[doc = "AOI1_OUT1 input is selected."]
    Val37 = 0x25,
    #[doc = "AOI1_OUT2 input is selected."]
    Val38 = 0x26,
    #[doc = "AOI1_OUT3 input is selected."]
    Val39 = 0x27,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val43 = 0x2b,
    #[doc = "FlexIO CH0 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH1 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH2 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH3 input is selected."]
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
pub enum OpampTrigInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
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
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val23 = 0x17,
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
    #[doc = "WUU input is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "AOI1_OUT0 input is selected."]
    Val33 = 0x21,
    #[doc = "AOI1_OUT1 input is selected."]
    Val34 = 0x22,
    #[doc = "AOI1_OUT2 input is selected."]
    Val35 = 0x23,
    #[doc = "AOI1_OUT3 input is selected."]
    Val36 = 0x24,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val37 = 0x25,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val38 = 0x26,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val39 = 0x27,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val40 = 0x28,
    #[doc = "CTimer3_MAT0 input is selected."]
    Val41 = 0x29,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "CTimer4_MAT0 input is selected."]
    Val43 = 0x2b,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val44 = 0x2c,
    #[doc = "FlexIO CH0 input is selected."]
    Val45 = 0x2d,
    #[doc = "FlexIO CH1 input is selected."]
    Val46 = 0x2e,
    #[doc = "FlexIO CH2 input is selected."]
    Val47 = 0x2f,
    #[doc = "FlexIO CH3 input is selected."]
    Val48 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val50 = 0x32,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val51 = 0x33,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val52 = 0x34,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val53 = 0x35,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val54 = 0x36,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl OpampTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpampTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpampTrigInp {
    #[inline(always)]
    fn from(val: u8) -> OpampTrigInp {
        OpampTrigInp::from_bits(val)
    }
}
impl From<OpampTrigInp> for u8 {
    #[inline(always)]
    fn from(val: OpampTrigInp) -> u8 {
        OpampTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0ExtClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    Val1 = 0x01,
    #[doc = "clk_in input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    Val5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    Val6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    Val7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    Val8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm0ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0ExtClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm0ExtClkTrigin {
        Pwm0ExtClkTrigin::from_bits(val)
    }
}
impl From<Pwm0ExtClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm0ExtClkTrigin) -> u8 {
        Pwm0ExtClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1ExtClkTrigin {
    _RESERVED_0 = 0x0,
    #[doc = "clk_16k\\[1\\] input is selected."]
    Val1 = 0x01,
    #[doc = "clk_in input is selected."]
    Val2 = 0x02,
    #[doc = "AOI0_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "AOI0_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "EXTTRIG_IN0 input is selected."]
    Val5 = 0x05,
    #[doc = "EXTTRIG_IN7 input is selected."]
    Val6 = 0x06,
    #[doc = "AOI1_OUT0 input is selected."]
    Val7 = 0x07,
    #[doc = "AOI1_OUT1 input is selected."]
    Val8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pwm1ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1ExtClkTrigin {
    #[inline(always)]
    fn from(val: u8) -> Pwm1ExtClkTrigin {
        Pwm1ExtClkTrigin::from_bits(val)
    }
}
impl From<Pwm1ExtClkTrigin> for u8 {
    #[inline(always)]
    fn from(val: Pwm1ExtClkTrigin) -> u8 {
        Pwm1ExtClkTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0PhaseaInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl Qdc0PhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0PhaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0PhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0PhaseaInp {
        Qdc0PhaseaInp::from_bits(val)
    }
}
impl From<Qdc0PhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0PhaseaInp) -> u8 {
        Qdc0PhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0PhasebInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl Qdc0PhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0PhasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0PhasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc0PhasebInp {
        Qdc0PhasebInp::from_bits(val)
    }
}
impl From<Qdc0PhasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc0PhasebInp) -> u8 {
        Qdc0PhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1PhaseaInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl Qdc1PhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1PhaseaInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1PhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1PhaseaInp {
        Qdc1PhaseaInp::from_bits(val)
    }
}
impl From<Qdc1PhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1PhaseaInp) -> u8 {
        Qdc1PhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1PhasebInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl Qdc1PhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1PhasebInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1PhasebInp {
    #[inline(always)]
    fn from(val: u8) -> Qdc1PhasebInp {
        Qdc1PhasebInp::from_bits(val)
    }
}
impl From<Qdc1PhasebInp> for u8 {
    #[inline(always)]
    fn from(val: Qdc1PhasebInp) -> u8 {
        Qdc1PhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcHomeInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl QdcHomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcHomeInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcHomeInp {
    #[inline(always)]
    fn from(val: u8) -> QdcHomeInp {
        QdcHomeInp::from_bits(val)
    }
}
impl From<QdcHomeInp> for u8 {
    #[inline(always)]
    fn from(val: QdcHomeInp) -> u8 {
        QdcHomeInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcIcapInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl QdcIcapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIcapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcIcapInp {
    #[inline(always)]
    fn from(val: u8) -> QdcIcapInp {
        QdcIcapInp::from_bits(val)
    }
}
impl From<QdcIcapInp> for u8 {
    #[inline(always)]
    fn from(val: QdcIcapInp) -> u8 {
        QdcIcapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcIndexInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl QdcIndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIndexInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcIndexInp {
    #[inline(always)]
    fn from(val: u8) -> QdcIndexInp {
        QdcIndexInp::from_bits(val)
    }
}
impl From<QdcIndexInp> for u8 {
    #[inline(always)]
    fn from(val: QdcIndexInp) -> u8 {
        QdcIndexInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcTrigInp {
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
    #[doc = "CMP1_OUT input is selected."]
    Val7 = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "CTimer0_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTimer0_MAT3."]
    Val10 = 0x0a,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val13 = 0x0d,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val16 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
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
    #[doc = "TRIG_IN8 input is selected."]
    Val32 = 0x20,
    #[doc = "TRIG_IN9 input is selected."]
    Val33 = 0x21,
    #[doc = "TRIG_IN10 input is selected."]
    Val34 = 0x22,
    #[doc = "TRIG_IN11 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO0 Pin Event Trig 0 is selected."]
    Val36 = 0x24,
    #[doc = "GPIO1 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO4 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "AOI1_OUT0 input is selected."]
    Val41 = 0x29,
    #[doc = "AOI1_OUT1 input is selected."]
    Val42 = 0x2a,
    #[doc = "AOI1_OUT2 input is selected."]
    Val43 = 0x2b,
    #[doc = "AOI1_OUT3 input is selected."]
    Val44 = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val49 = 0x31,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val50 = 0x32,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val51 = 0x33,
    #[doc = "CTimer4_MAT3 End of Frame input is selected."]
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
    #[doc = "PWM1_SM0_OUT_TRIG0 input is selected."]
    Val62 = 0x3e,
    #[doc = "PWM1_SM0_OUT_TRIG1 input is selected."]
    Val63 = 0x3f,
    #[doc = "PWM1_SM1_OUT_TRIG0 input is selected."]
    Val64 = 0x40,
    #[doc = "PWM1_SM1_OUT_TRIG1 input is selected."]
    Val65 = 0x41,
    #[doc = "PWM1_SM2_OUT_TRIG0 input is selected."]
    Val66 = 0x42,
    #[doc = "PWM1_SM2_OUT_TRIG1 input is selected."]
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
impl QdcTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcTrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcTrigInp {
    #[inline(always)]
    fn from(val: u8) -> QdcTrigInp {
        QdcTrigInp::from_bits(val)
    }
}
impl From<QdcTrigInp> for u8 {
    #[inline(always)]
    fn from(val: QdcTrigInp) -> u8 {
        QdcTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimertrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "CT_INP0 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP1 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP2 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP3 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP4 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP5 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP6 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP7 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP8 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP9 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP10 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP11 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP12 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP13 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP14 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP15 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP16 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP17 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP18 input is selected."]
    Val19 = 0x13,
    #[doc = "CT_INP19 input is selected."]
    Val20 = 0x14,
    #[doc = "USB0 usb0 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "AOI0_OUT0 input is selected."]
    Val22 = 0x16,
    #[doc = "AOI0_OUT1 input is selected."]
    Val23 = 0x17,
    #[doc = "AOI0_OUT2 input is selected."]
    Val24 = 0x18,
    #[doc = "AOI0_OUT3 input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[0\\]."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[1\\]."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[2\\]."]
    Val28 = 0x1c,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "CMP0_OUT is selected."]
    Val30 = 0x1e,
    #[doc = "CMP1_OUT is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "CTimer1_MAT1 input is selected."]
    Val33 = 0x21,
    #[doc = "CTimer1_MAT2 input is selected."]
    Val34 = 0x22,
    #[doc = "CTimer1_MAT3 input is selected."]
    Val35 = 0x23,
    #[doc = "CTimer2_MAT1 input is selected."]
    Val36 = 0x24,
    #[doc = "CTimer2_MAT2 input is selected."]
    Val37 = 0x25,
    #[doc = "CTimer2_MAT3 input is selected."]
    Val38 = 0x26,
    #[doc = "QDC0_CMP_FLAG0 is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP_FLAG1 input is selected."]
    Val40 = 0x28,
    #[doc = "QDC0_CMP_FLAG2 input is selected."]
    Val41 = 0x29,
    #[doc = "QDC0_CMP_FLAG3 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_POS_MATCH0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val44 = 0x2c,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val45 = 0x2d,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    #[doc = "LPI2C0 Master End of Packet input is selected."]
    Val48 = 0x30,
    #[doc = "LPI2C0 Slave End of Packet input is selected."]
    Val49 = 0x31,
    #[doc = "LPI2C1 Master End of Packet input is selected."]
    Val50 = 0x32,
    #[doc = "LPI2C1 Slave End of Packet input is selected."]
    Val51 = 0x33,
    #[doc = "LPSPI0 End of Frame input is selected."]
    Val52 = 0x34,
    #[doc = "LPSPI0 Received Data Word input is selected."]
    Val53 = 0x35,
    #[doc = "LPSPI1 End of Frame input is selected."]
    Val54 = 0x36,
    #[doc = "LPSPI1 Received Data Word input is selected."]
    Val55 = 0x37,
    #[doc = "LPUART0 Received Data Word input is selected."]
    Val56 = 0x38,
    #[doc = "LPUART0 Transmitted Data Word input is selected."]
    Val57 = 0x39,
    #[doc = "LPUART0 Receive Line Idle input is selected."]
    Val58 = 0x3a,
    #[doc = "LPUART1 Received Data Word input is selected."]
    Val59 = 0x3b,
    #[doc = "LPUART1 Transmitted Data Word input is selected."]
    Val60 = 0x3c,
    #[doc = "LPUART1 Receive Line Idle input is selected."]
    Val61 = 0x3d,
    #[doc = "LPUART2 Received Data Word input is selected."]
    Val62 = 0x3e,
    #[doc = "LPUART2 Transmitted Data Word input is selected."]
    Val63 = 0x3f,
    #[doc = "LPUART2 Receive Line Idle input is selected."]
    Val64 = 0x40,
    #[doc = "LPUART3 Received Data Word input is selected."]
    Val65 = 0x41,
    #[doc = "LPUART3 Transmitted Data Word input is selected."]
    Val66 = 0x42,
    #[doc = "LPUART3 Receive Line Idle input is selected."]
    Val67 = 0x43,
    #[doc = "LPUART4 Received Data Word input is selected."]
    Val68 = 0x44,
    #[doc = "LPUART4 Transmitted Data Word input is selected."]
    Val69 = 0x45,
    #[doc = "LPUART4 Receive Line Idle input is selected."]
    Val70 = 0x46,
    #[doc = "AOI1_OUT0 input is selected."]
    Val71 = 0x47,
    #[doc = "AOI1_OUT1 input is selected."]
    Val72 = 0x48,
    #[doc = "AOI1_OUT2 input is selected."]
    Val73 = 0x49,
    #[doc = "AOI1_OUT3 input is selected."]
    Val74 = 0x4a,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val75 = 0x4b,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val76 = 0x4c,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val77 = 0x4d,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val78 = 0x4e,
    #[doc = "CTimer3_MAT1 input is selected."]
    Val79 = 0x4f,
    #[doc = "CTimer3_MAT2 input is selected."]
    Val80 = 0x50,
    #[doc = "CTimer3_MAT3 input is selected."]
    Val81 = 0x51,
    #[doc = "CTimer4_MAT1 input is selected."]
    Val82 = 0x52,
    #[doc = "CTimer4_MAT2 input is selected."]
    Val83 = 0x53,
    #[doc = "CTimer4_MAT3 input is selected."]
    Val84 = 0x54,
    #[doc = "QDC1_CMP_FLAG0 input is selected."]
    Val85 = 0x55,
    #[doc = "QDC1_CMP_FLAG1 input is selected."]
    Val86 = 0x56,
    #[doc = "QDC1_CMP_FLAG2 input is selected."]
    Val87 = 0x57,
    #[doc = "QDC1_CMP_FLAG3 input is selected."]
    Val88 = 0x58,
    #[doc = "QDC1_POS_MATCH0 input is selected."]
    Val89 = 0x59,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val90 = 0x5a,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val91 = 0x5b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val92 = 0x5c,
    _RESERVED_5d = 0x5d,
    #[doc = "LPI2C2 Master End of Packet input is selected."]
    Val94 = 0x5e,
    #[doc = "LPI2C2 Slave End of Packet input is selected."]
    Val95 = 0x5f,
    #[doc = "LPI2C3 Master End of Packet input is selected."]
    Val96 = 0x60,
    #[doc = "LPI2C3 Slave End of Packet input is selected."]
    Val97 = 0x61,
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
impl TimertrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimertrigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimertrigInp {
    #[inline(always)]
    fn from(val: u8) -> TimertrigInp {
        TimertrigInp::from_bits(val)
    }
}
impl From<TimertrigInp> for u8 {
    #[inline(always)]
    fn from(val: TimertrigInp) -> u8 {
        TimertrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbfsTrigInp {
    _RESERVED_0 = 0x0,
    #[doc = "LPUART0 lpuart_trg_txdata input is selected."]
    Val1 = 0x01,
    #[doc = "LPUART1 lpuart_trg_txdata input is selected."]
    Val2 = 0x02,
    #[doc = "LPUART2 lpuart_trg_txdata input is selected."]
    Val3 = 0x03,
    #[doc = "LPUART3 lpuart_trg_txdata input is selected."]
    Val4 = 0x04,
    #[doc = "LPUART4 lpuart_trg_txda input is selected."]
    Val5 = 0x05,
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
impl UsbfsTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbfsTrigInp {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbfsTrigInp {
    #[inline(always)]
    fn from(val: u8) -> UsbfsTrigInp {
        UsbfsTrigInp::from_bits(val)
    }
}
impl From<UsbfsTrigInp> for u8 {
    #[inline(always)]
    fn from(val: UsbfsTrigInp) -> u8 {
        UsbfsTrigInp::to_bits(val)
    }
}
