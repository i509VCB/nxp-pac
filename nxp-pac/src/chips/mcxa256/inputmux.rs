#[doc = "INPUTMUX"]
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
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer0cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer0cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER0"]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::common::Reg<regs::Timer0trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer1cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer1cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER1"]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::common::Reg<regs::Timer1trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer2cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer2cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Trigger register for TIMER2 inputs"]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::common::Reg<regs::Timer2trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SmartDMA Trigger Input Connections"]
    #[inline(always)]
    pub const fn smart_dma_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SmartDmaTrig, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_ref(self) -> crate::common::Reg<regs::FreqmeasRef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for frequency measurement reference clock"]
    #[inline(always)]
    pub const fn freqmeas_tar(self) -> crate::common::Reg<regs::FreqmeasTar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer3cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer3cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER3"]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::common::Reg<regs::Timer3trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture select register for CTIMER inputs"]
    #[inline(always)]
    pub const fn ctimer4cap(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ctimer4cap, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger register for TIMER4"]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::common::Reg<regs::Timer4trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "AOI1 trigger input connections 0"]
    #[inline(always)]
    pub const fn aoi1_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AoiInput, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "CMP0 input connections"]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::common::Reg<regs::CmpTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc0_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc2_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc1_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger input connections"]
    #[inline(always)]
    pub const fn adc3_trig(self, n: usize) -> crate::common::Reg<regs::AdcTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 Trigger input connections."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::common::Reg<regs::DacTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_trig(self) -> crate::common::Reg<regs::QdcTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_home(self) -> crate::common::Reg<regs::Qdc0Home, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_index(self) -> crate::common::Reg<regs::Qdc0Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_phaseb(self) -> crate::common::Reg<regs::Qdc0Phaseb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_phasea(self) -> crate::common::Reg<regs::Qdc0Phasea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_icap1(self) -> crate::common::Reg<regs::Qdc0Icap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_icap2(self) -> crate::common::Reg<regs::Qdc0Icap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "QDC0 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc0_icap3(self) -> crate::common::Reg<regs::Qdc0Icap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_trig(self) -> crate::common::Reg<regs::QdcTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_home(self) -> crate::common::Reg<regs::Qdc1Home, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_index(self) -> crate::common::Reg<regs::Qdc1Index, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_phaseb(self) -> crate::common::Reg<regs::Qdc1Phaseb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_phasea(self) -> crate::common::Reg<regs::Qdc1Phasea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_icap1(self) -> crate::common::Reg<regs::Qdc1Icap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_icap2(self) -> crate::common::Reg<regs::Qdc1Icap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "QDC1 Trigger Input Connections"]
    #[inline(always)]
    pub const fn qdc1_icap3(self) -> crate::common::Reg<regs::Qdc1Icap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_exta0(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm0_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_exta(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm1_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_exta(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm2_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm3_exta0(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_sm3_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "PWM0 Fault Input Trigger Connections"]
    #[inline(always)]
    pub const fn flex_pwm0_fault(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm0_force(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_exta0(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm0_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_exta(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm1_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_exta(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm2_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm3_exta0(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_sm3_extsync(
        self,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "PWM1 Fault Input Trigger Connections"]
    #[inline(always)]
    pub const fn flex_pwm1_fault(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 input trigger connections"]
    #[inline(always)]
    pub const fn flex_pwm1_force(self) -> crate::common::Reg<regs::FlexPwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "PWM0 external clock trigger"]
    #[inline(always)]
    pub const fn pwm0_ext_clk(self) -> crate::common::Reg<regs::Pwm0ExtClk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "PWM1 external clock trigger"]
    #[inline(always)]
    pub const fn pwm1_ext_clk(self) -> crate::common::Reg<regs::Pwm1ExtClk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "AOI0 trigger input connections 0"]
    #[inline(always)]
    pub const fn aoi0_input(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AoiInput, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 4usize) as _)
        }
    }
    #[doc = "USB-FS trigger input connections"]
    #[inline(always)]
    pub const fn usbfs_trig(self) -> crate::common::Reg<regs::UsbfsTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "EXT trigger connections"]
    #[inline(always)]
    pub const fn ext_trig(self, n: usize) -> crate::common::Reg<regs::ExtTrig, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize + n * 4usize) as _)
        }
    }
    #[doc = "CMP1 input connections"]
    #[inline(always)]
    pub const fn cmp1_trig(self) -> crate::common::Reg<regs::CmpTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "CMP2 input connections"]
    #[inline(always)]
    pub const fn cmp2_trig(self) -> crate::common::Reg<regs::CmpTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "LPI2C2 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c2_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "LPI2C3 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c3_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "LPI2C0 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c0_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LPI2C1 trigger input connections"]
    #[inline(always)]
    pub const fn lpi2c1_trig(self) -> crate::common::Reg<regs::Lpi2cTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LPSPI0 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi0_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LPSPI1 trigger input connections"]
    #[inline(always)]
    pub const fn lpspi1_trig(self) -> crate::common::Reg<regs::LpspiTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LPUART0 trigger input connections"]
    #[inline(always)]
    pub const fn lpuart(self, n: usize) -> crate::common::Reg<regs::Lpuart, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize + n * 32usize) as _)
        }
    }
    #[doc = "FlexIO Trigger Input Connections"]
    #[inline(always)]
    pub const fn flexio_trig(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FlexioTrig, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger filter prescaller"]
    #[inline(always)]
    pub const fn trigfil_prsc(self) -> crate::common::Reg<regs::TrigfilPrsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "Trigger filter stat"]
    #[inline(always)]
    pub const fn trigfil_stat0(self) -> crate::common::Reg<regs::TrigfilStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a04usize) as _) }
    }
    #[doc = "TRIGFIL control"]
    #[inline(always)]
    pub const fn trigfil(self, n: usize) -> crate::common::Reg<regs::Trigfil, crate::common::RW> {
        assert!(n < 12usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a10usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
