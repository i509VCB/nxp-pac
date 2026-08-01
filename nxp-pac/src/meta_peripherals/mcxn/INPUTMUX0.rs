#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "INPUTMUX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux0 {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux0 {}
unsafe impl Sync for Inputmux0 {}
impl Inputmux0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: QDC_TRIG, QDC_HOME, QDC_INDEX, QDC_PHASEB, QDC_PHASEA."]
    #[inline(always)]
    pub const fn qd_cn(self, n: usize) -> QdCn {
        assert!(n < 2usize);
        unsafe { QdCn::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Inputmux Register for SCT0 Input."]
    #[inline(always)]
    pub const fn sct0_inmux(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Sct0Inmux, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer0cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer0Cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer0trig(self) -> crate::pac::common::Reg<Timer0trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer1cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer1Cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer1trig(self) -> crate::pac::common::Reg<Timer1trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer2cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer2Cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer2trig(self) -> crate::pac::common::Reg<Timer2trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Inputmux Register for SMARTDMA Arch B Inputs."]
    #[inline(always)]
    pub const fn smartdmaarchb_inmux(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SmartdmaarchbInmux, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "Pin Interrupt Select."]
    #[inline(always)]
    pub const fn pintsel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Pintsel, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _)
        }
    }
    #[doc = "Selection for Frequency Measurement Reference Clock."]
    #[inline(always)]
    pub const fn freqmeas_ref(
        self,
    ) -> crate::pac::common::Reg<FreqmeasRef, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Selection for Frequency Measurement Target Clock."]
    #[inline(always)]
    pub const fn freqmeas_tar(
        self,
    ) -> crate::pac::common::Reg<FreqmeasTar, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer3cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer3Cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer3trig(self) -> crate::pac::common::Reg<Timer3trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Capture Select Register for CTIMER Inputs."]
    #[inline(always)]
    pub const fn ctimer4cap(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimer4Cap, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize + n * 4usize) as _)
        }
    }
    #[doc = "Trigger Register for CTIMER."]
    #[inline(always)]
    pub const fn timer4trig(self) -> crate::pac::common::Reg<Timer4trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "CMP0 Input Connections."]
    #[inline(always)]
    pub const fn cmp0_trig(self) -> crate::pac::common::Reg<Cmp0Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "ADC Trigger Input Connections."]
    #[inline(always)]
    pub const fn adc0_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Adc0Trig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Trigger Input Connections."]
    #[inline(always)]
    pub const fn adc1_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Adc1Trig, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize + n * 4usize) as _)
        }
    }
    #[doc = "DAC0 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac0_trig(self) -> crate::pac::common::Reg<Dac0Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "DAC1 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac1_trig(self) -> crate::pac::common::Reg<Dac1Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "DAC2 Trigger Inputs."]
    #[inline(always)]
    pub const fn dac2_trig(self) -> crate::pac::common::Reg<Dac2Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "PWM0 External Synchronization."]
    #[inline(always)]
    pub const fn flex_pwm0_sm_extsync(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm0SmExtsync, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_sm_exta(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm0SmExta, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 External Force Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_extforce(
        self,
    ) -> crate::pac::common::Reg<FlexPwm0Extforce, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "PWM0 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm0_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm0Fault, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 External Synchronization."]
    #[inline(always)]
    pub const fn flex_pwm1_sm_extsync(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm1SmExtsync, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 Input EXTA Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_sm_exta(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm1SmExta, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM1 External Force Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_extforce(
        self,
    ) -> crate::pac::common::Reg<FlexPwm1Extforce, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "PWM1 Fault Input Trigger Connections."]
    #[inline(always)]
    pub const fn flex_pwm1_fault(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexPwm1Fault, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize + n * 4usize) as _)
        }
    }
    #[doc = "PWM0 External Clock Trigger."]
    #[inline(always)]
    pub const fn pwm0_ext_clk(self) -> crate::pac::common::Reg<Pwm0ExtClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "PWM1 External Clock Trigger."]
    #[inline(always)]
    pub const fn pwm1_ext_clk(self) -> crate::pac::common::Reg<Pwm1ExtClk, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "EVTG Trigger Input Connections."]
    #[inline(always)]
    pub const fn evtg_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<EvtgTrig, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 4usize) as _)
        }
    }
    #[doc = "USB-FS Trigger Input Connections."]
    #[inline(always)]
    pub const fn usbfs_trig(self) -> crate::pac::common::Reg<UsbfsTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "TSI Trigger Input Connections."]
    #[inline(always)]
    pub const fn tsi_trig(self) -> crate::pac::common::Reg<TsiTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "EXT Trigger Connections."]
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
    #[doc = "CMP1 Input Connections."]
    #[inline(always)]
    pub const fn cmp1_trig(self) -> crate::pac::common::Reg<Cmp1Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "CMP2 Input Connections."]
    #[inline(always)]
    pub const fn cmp2_trig(self) -> crate::pac::common::Reg<Cmp2Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "SINC Filter Channel Trigger Input Connections."]
    #[inline(always)]
    pub const fn sinc_filter_ch(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<SincFilterCh, crate::pac::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize + n * 4usize) as _)
        }
    }
    #[doc = "OPAMP Trigger Input Connections."]
    #[inline(always)]
    pub const fn opamp_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<OpampTrig, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize + n * 4usize) as _)
        }
    }
    #[doc = "LP_FLEXCOMM0 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm0_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm0Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM1 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm1_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm1Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM2 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm2_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm2Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM3 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm3_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm3Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "LP_FLEXCOMM4 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm4_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm4Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "LP_FLEXCOMM5 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm5_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm5Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "LP_FLEXCOMM6 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm6_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm6Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "LP_FLEXCOMM7 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm7_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm7Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "LP_FLEXCOMM8 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm8_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm8Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "LP_FLEXCOMM9 Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexcomm9_trig(
        self,
    ) -> crate::pac::common::Reg<Flexcomm9Trig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "FlexIO Trigger Input Connections."]
    #[inline(always)]
    pub const fn flexio_trig(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<FlexioTrig, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize + n * 4usize) as _)
        }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_set(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable0Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_clr(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable0Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "DMA0 Request Enable0."]
    #[inline(always)]
    pub const fn dma0_req_enable0_tog(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable0Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_set(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable1Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_clr(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable1Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "DMA0 Request Enable1."]
    #[inline(always)]
    pub const fn dma0_req_enable1_tog(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable1Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_set(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable2Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_clr(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable2Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "DMA0 Request Enable2."]
    #[inline(always)]
    pub const fn dma0_req_enable2_tog(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable2Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3_set(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable3Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "DMA0 Request Enable3."]
    #[inline(always)]
    pub const fn dma0_req_enable3_clr(
        self,
    ) -> crate::pac::common::Reg<Dma0ReqEnable3Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0738usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_set(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable0Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_clr(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable0Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "DMA1 Request Enable0."]
    #[inline(always)]
    pub const fn dma1_req_enable0_tog(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable0Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_set(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable1Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_clr(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable1Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[doc = "DMA1 Request Enable1."]
    #[inline(always)]
    pub const fn dma1_req_enable1_tog(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable1Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x079cusize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_set(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable2Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_clr(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable2Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "DMA1 Request Enable2."]
    #[inline(always)]
    pub const fn dma1_req_enable2_tog(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable2Tog, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07acusize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3_set(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable3Set, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    #[doc = "DMA1 Request Enable3."]
    #[inline(always)]
    pub const fn dma1_req_enable3_clr(
        self,
    ) -> crate::pac::common::Reg<Dma1ReqEnable3Clr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b8usize) as _) }
    }
}
#[doc = "Array of registers: QDC_TRIG, QDC_HOME, QDC_INDEX, QDC_PHASEB, QDC_PHASEA."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdCn {
    ptr: *mut u8,
}
unsafe impl Send for QdCn {}
unsafe impl Sync for QdCn {}
impl QdCn {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "QDCouter_loop Trigger Input Connections."]
    #[inline(always)]
    pub const fn qdc_trig(self) -> crate::pac::common::Reg<QdcTrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "QDCouter_loop Input Connections."]
    #[inline(always)]
    pub const fn qdc_home(self) -> crate::pac::common::Reg<QdcHome, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "QDCouter_loop Input Connections."]
    #[inline(always)]
    pub const fn qdc_index(self) -> crate::pac::common::Reg<QdcIndex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "QDCouter_loop Input Connections."]
    #[inline(always)]
    pub const fn qdc_phaseb(self) -> crate::pac::common::Reg<QdcPhaseb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "QDCouter_loop Input Connections."]
    #[inline(always)]
    pub const fn qdc_phasea(self) -> crate::pac::common::Reg<QdcPhasea, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
}
#[doc = "ADC Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0Trig(pub u32);
impl Adc0Trig {
    #[doc = "ADC0 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Adc0TrigTrigin {
        let val = (self.0 >> 0usize) & 0xff;
        Adc0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC0 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Adc0TrigTrigin) {
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
#[doc = "ADC Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1Trig(pub u32);
impl Adc1Trig {
    #[doc = "ADC1 trigger inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Adc1TrigTrigin {
        let val = (self.0 >> 0usize) & 0xff;
        Adc1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "ADC1 trigger inputs."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Adc1TrigTrigin) {
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
#[doc = "CMP0 Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0Trig(pub u32);
impl Cmp0Trig {
    #[doc = "CMP0 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Cmp0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Cmp0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP0 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Cmp0TrigTrigin) {
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
#[doc = "CMP1 Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1Trig(pub u32);
impl Cmp1Trig {
    #[doc = "CMP1 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Cmp1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Cmp1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP1 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Cmp1TrigTrigin) {
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
#[doc = "CMP2 Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2Trig(pub u32);
impl Cmp2Trig {
    #[doc = "CMP2 input trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Cmp2TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Cmp2TrigTrigin::from_bits(val as u8)
    }
    #[doc = "CMP2 input trigger."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Cmp2TrigTrigin) {
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
#[doc = "Capture Select Register for CTIMER Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer0Cap(pub u32);
impl Ctimer0Cap {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer0CapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer0CapInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer0CapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer0Cap {
    #[inline(always)]
    fn default() -> Ctimer0Cap {
        Ctimer0Cap(0)
    }
}
impl core::fmt::Debug for Ctimer0Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer0Cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer0Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer0Cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer1Cap(pub u32);
impl Ctimer1Cap {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer1CapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer1CapInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer1CapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer1Cap {
    #[inline(always)]
    fn default() -> Ctimer1Cap {
        Ctimer1Cap(0)
    }
}
impl core::fmt::Debug for Ctimer1Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer1Cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer1Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer1Cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer2Cap(pub u32);
impl Ctimer2Cap {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer2CapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer2CapInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer2CapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer2Cap {
    #[inline(always)]
    fn default() -> Ctimer2Cap {
        Ctimer2Cap(0)
    }
}
impl core::fmt::Debug for Ctimer2Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer2Cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer2Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer2Cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer3Cap(pub u32);
impl Ctimer3Cap {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer3CapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer3CapInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer3CapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer3Cap {
    #[inline(always)]
    fn default() -> Ctimer3Cap {
        Ctimer3Cap(0)
    }
}
impl core::fmt::Debug for Ctimer3Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer3Cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer3Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer3Cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "Capture Select Register for CTIMER Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer4Cap(pub u32);
impl Ctimer4Cap {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Ctimer4CapInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Ctimer4CapInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Ctimer4CapInp) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val.to_bits() as u32) & 0x7f) << 0usize);
    }
}
impl Default for Ctimer4Cap {
    #[inline(always)]
    fn default() -> Ctimer4Cap {
        Ctimer4Cap(0)
    }
}
impl core::fmt::Debug for Ctimer4Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimer4Cap")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimer4Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimer4Cap {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DAC0 Trigger Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac0Trig(pub u32);
impl Dac0Trig {
    #[doc = "DAC0 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Dac0TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Dac0TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC0 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Dac0TrigTrigin) {
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
#[doc = "DAC1 Trigger Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac1Trig(pub u32);
impl Dac1Trig {
    #[doc = "DAC1 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Dac1TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Dac1TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC1 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Dac1TrigTrigin) {
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
#[doc = "DAC2 Trigger Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac2Trig(pub u32);
impl Dac2Trig {
    #[doc = "DAC2 trigger input."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Dac2TrigTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        Dac2TrigTrigin::from_bits(val as u8)
    }
    #[doc = "DAC2 trigger input."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Dac2TrigTrigin) {
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
#[doc = "DMA0 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0(pub u32);
impl Dma0ReqEnable0 {
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en0(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[inline(always)]
    pub const fn set_req1_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en0(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[inline(always)]
    pub const fn set_req2_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en0(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[inline(always)]
    pub const fn set_req3_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en0(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[inline(always)]
    pub const fn set_req4_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en0(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[inline(always)]
    pub const fn set_req5_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en0(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[inline(always)]
    pub const fn set_req6_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en0(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req7_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en0(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req8_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en0(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req9_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en0(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req10_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en0(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req11_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en0(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req12_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en0(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req13_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en0(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req14_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en0(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req15_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en0(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req16_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en0(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[inline(always)]
    pub const fn set_req17_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en0(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req18_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en0(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[inline(always)]
    pub const fn set_req19_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en0(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[inline(always)]
    pub const fn set_req20_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en0(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[inline(always)]
    pub const fn set_req21_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en0(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[inline(always)]
    pub const fn set_req22_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en0(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[inline(always)]
    pub const fn set_req23_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en0(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[inline(always)]
    pub const fn set_req24_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en0(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req25_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en0(&self) -> ReqEn {
        let val = (self.0 >> 26usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[inline(always)]
    pub const fn set_req26_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en0(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[inline(always)]
    pub const fn set_req27_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en0(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[inline(always)]
    pub const fn set_req28_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en0(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[inline(always)]
    pub const fn set_req29_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en0(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[inline(always)]
    pub const fn set_req30_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en0(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[inline(always)]
    pub const fn set_req31_en0(&mut self, val: ReqEn) {
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
#[doc = "DMA0 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Clr(pub u32);
impl Dma0ReqEnable0Clr {
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en0(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en0(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en0[0]", &self.req_en0(0usize))
            .field("req_en0[1]", &self.req_en0(1usize))
            .field("req_en0[2]", &self.req_en0(2usize))
            .field("req_en0[3]", &self.req_en0(3usize))
            .field("req_en0[4]", &self.req_en0(4usize))
            .field("req_en0[5]", &self.req_en0(5usize))
            .field("req_en0[6]", &self.req_en0(6usize))
            .field("req_en0[7]", &self.req_en0(7usize))
            .field("req_en0[8]", &self.req_en0(8usize))
            .field("req_en0[9]", &self.req_en0(9usize))
            .field("req_en0[10]", &self.req_en0(10usize))
            .field("req_en0[11]", &self.req_en0(11usize))
            .field("req_en0[12]", &self.req_en0(12usize))
            .field("req_en0[13]", &self.req_en0(13usize))
            .field("req_en0[14]", &self.req_en0(14usize))
            .field("req_en0[15]", &self.req_en0(15usize))
            .field("req_en0[16]", &self.req_en0(16usize))
            .field("req_en0[17]", &self.req_en0(17usize))
            .field("req_en0[18]", &self.req_en0(18usize))
            .field("req_en0[19]", &self.req_en0(19usize))
            .field("req_en0[20]", &self.req_en0(20usize))
            .field("req_en0[21]", &self.req_en0(21usize))
            .field("req_en0[22]", &self.req_en0(22usize))
            .field("req_en0[23]", &self.req_en0(23usize))
            .field("req_en0[24]", &self.req_en0(24usize))
            .field("req_en0[25]", &self.req_en0(25usize))
            .field("req_en0[26]", &self.req_en0(26usize))
            .field("req_en0[27]", &self.req_en0(27usize))
            .field("req_en0[28]", &self.req_en0(28usize))
            .field("req_en0[29]", &self.req_en0(29usize))
            .field("req_en0[30]", &self.req_en0(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Clr {{ req_en0[0]: {=bool:?}, req_en0[1]: {=bool:?}, req_en0[2]: {=bool:?}, req_en0[3]: {=bool:?}, req_en0[4]: {=bool:?}, req_en0[5]: {=bool:?}, req_en0[6]: {=bool:?}, req_en0[7]: {=bool:?}, req_en0[8]: {=bool:?}, req_en0[9]: {=bool:?}, req_en0[10]: {=bool:?}, req_en0[11]: {=bool:?}, req_en0[12]: {=bool:?}, req_en0[13]: {=bool:?}, req_en0[14]: {=bool:?}, req_en0[15]: {=bool:?}, req_en0[16]: {=bool:?}, req_en0[17]: {=bool:?}, req_en0[18]: {=bool:?}, req_en0[19]: {=bool:?}, req_en0[20]: {=bool:?}, req_en0[21]: {=bool:?}, req_en0[22]: {=bool:?}, req_en0[23]: {=bool:?}, req_en0[24]: {=bool:?}, req_en0[25]: {=bool:?}, req_en0[26]: {=bool:?}, req_en0[27]: {=bool:?}, req_en0[28]: {=bool:?}, req_en0[29]: {=bool:?}, req_en0[30]: {=bool:?} }}",
            self.req_en0(0usize),
            self.req_en0(1usize),
            self.req_en0(2usize),
            self.req_en0(3usize),
            self.req_en0(4usize),
            self.req_en0(5usize),
            self.req_en0(6usize),
            self.req_en0(7usize),
            self.req_en0(8usize),
            self.req_en0(9usize),
            self.req_en0(10usize),
            self.req_en0(11usize),
            self.req_en0(12usize),
            self.req_en0(13usize),
            self.req_en0(14usize),
            self.req_en0(15usize),
            self.req_en0(16usize),
            self.req_en0(17usize),
            self.req_en0(18usize),
            self.req_en0(19usize),
            self.req_en0(20usize),
            self.req_en0(21usize),
            self.req_en0(22usize),
            self.req_en0(23usize),
            self.req_en0(24usize),
            self.req_en0(25usize),
            self.req_en0(26usize),
            self.req_en0(27usize),
            self.req_en0(28usize),
            self.req_en0(29usize),
            self.req_en0(30usize)
        )
    }
}
#[doc = "DMA0 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Set(pub u32);
impl Dma0ReqEnable0Set {
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en0(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en0(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en0[0]", &self.req_en0(0usize))
            .field("req_en0[1]", &self.req_en0(1usize))
            .field("req_en0[2]", &self.req_en0(2usize))
            .field("req_en0[3]", &self.req_en0(3usize))
            .field("req_en0[4]", &self.req_en0(4usize))
            .field("req_en0[5]", &self.req_en0(5usize))
            .field("req_en0[6]", &self.req_en0(6usize))
            .field("req_en0[7]", &self.req_en0(7usize))
            .field("req_en0[8]", &self.req_en0(8usize))
            .field("req_en0[9]", &self.req_en0(9usize))
            .field("req_en0[10]", &self.req_en0(10usize))
            .field("req_en0[11]", &self.req_en0(11usize))
            .field("req_en0[12]", &self.req_en0(12usize))
            .field("req_en0[13]", &self.req_en0(13usize))
            .field("req_en0[14]", &self.req_en0(14usize))
            .field("req_en0[15]", &self.req_en0(15usize))
            .field("req_en0[16]", &self.req_en0(16usize))
            .field("req_en0[17]", &self.req_en0(17usize))
            .field("req_en0[18]", &self.req_en0(18usize))
            .field("req_en0[19]", &self.req_en0(19usize))
            .field("req_en0[20]", &self.req_en0(20usize))
            .field("req_en0[21]", &self.req_en0(21usize))
            .field("req_en0[22]", &self.req_en0(22usize))
            .field("req_en0[23]", &self.req_en0(23usize))
            .field("req_en0[24]", &self.req_en0(24usize))
            .field("req_en0[25]", &self.req_en0(25usize))
            .field("req_en0[26]", &self.req_en0(26usize))
            .field("req_en0[27]", &self.req_en0(27usize))
            .field("req_en0[28]", &self.req_en0(28usize))
            .field("req_en0[29]", &self.req_en0(29usize))
            .field("req_en0[30]", &self.req_en0(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Set {{ req_en0[0]: {=bool:?}, req_en0[1]: {=bool:?}, req_en0[2]: {=bool:?}, req_en0[3]: {=bool:?}, req_en0[4]: {=bool:?}, req_en0[5]: {=bool:?}, req_en0[6]: {=bool:?}, req_en0[7]: {=bool:?}, req_en0[8]: {=bool:?}, req_en0[9]: {=bool:?}, req_en0[10]: {=bool:?}, req_en0[11]: {=bool:?}, req_en0[12]: {=bool:?}, req_en0[13]: {=bool:?}, req_en0[14]: {=bool:?}, req_en0[15]: {=bool:?}, req_en0[16]: {=bool:?}, req_en0[17]: {=bool:?}, req_en0[18]: {=bool:?}, req_en0[19]: {=bool:?}, req_en0[20]: {=bool:?}, req_en0[21]: {=bool:?}, req_en0[22]: {=bool:?}, req_en0[23]: {=bool:?}, req_en0[24]: {=bool:?}, req_en0[25]: {=bool:?}, req_en0[26]: {=bool:?}, req_en0[27]: {=bool:?}, req_en0[28]: {=bool:?}, req_en0[29]: {=bool:?}, req_en0[30]: {=bool:?} }}",
            self.req_en0(0usize),
            self.req_en0(1usize),
            self.req_en0(2usize),
            self.req_en0(3usize),
            self.req_en0(4usize),
            self.req_en0(5usize),
            self.req_en0(6usize),
            self.req_en0(7usize),
            self.req_en0(8usize),
            self.req_en0(9usize),
            self.req_en0(10usize),
            self.req_en0(11usize),
            self.req_en0(12usize),
            self.req_en0(13usize),
            self.req_en0(14usize),
            self.req_en0(15usize),
            self.req_en0(16usize),
            self.req_en0(17usize),
            self.req_en0(18usize),
            self.req_en0(19usize),
            self.req_en0(20usize),
            self.req_en0(21usize),
            self.req_en0(22usize),
            self.req_en0(23usize),
            self.req_en0(24usize),
            self.req_en0(25usize),
            self.req_en0(26usize),
            self.req_en0(27usize),
            self.req_en0(28usize),
            self.req_en0(29usize),
            self.req_en0(30usize)
        )
    }
}
#[doc = "DMA0 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable0Tog(pub u32);
impl Dma0ReqEnable0Tog {
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en0(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA0_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en0(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en0[0]", &self.req_en0(0usize))
            .field("req_en0[1]", &self.req_en0(1usize))
            .field("req_en0[2]", &self.req_en0(2usize))
            .field("req_en0[3]", &self.req_en0(3usize))
            .field("req_en0[4]", &self.req_en0(4usize))
            .field("req_en0[5]", &self.req_en0(5usize))
            .field("req_en0[6]", &self.req_en0(6usize))
            .field("req_en0[7]", &self.req_en0(7usize))
            .field("req_en0[8]", &self.req_en0(8usize))
            .field("req_en0[9]", &self.req_en0(9usize))
            .field("req_en0[10]", &self.req_en0(10usize))
            .field("req_en0[11]", &self.req_en0(11usize))
            .field("req_en0[12]", &self.req_en0(12usize))
            .field("req_en0[13]", &self.req_en0(13usize))
            .field("req_en0[14]", &self.req_en0(14usize))
            .field("req_en0[15]", &self.req_en0(15usize))
            .field("req_en0[16]", &self.req_en0(16usize))
            .field("req_en0[17]", &self.req_en0(17usize))
            .field("req_en0[18]", &self.req_en0(18usize))
            .field("req_en0[19]", &self.req_en0(19usize))
            .field("req_en0[20]", &self.req_en0(20usize))
            .field("req_en0[21]", &self.req_en0(21usize))
            .field("req_en0[22]", &self.req_en0(22usize))
            .field("req_en0[23]", &self.req_en0(23usize))
            .field("req_en0[24]", &self.req_en0(24usize))
            .field("req_en0[25]", &self.req_en0(25usize))
            .field("req_en0[26]", &self.req_en0(26usize))
            .field("req_en0[27]", &self.req_en0(27usize))
            .field("req_en0[28]", &self.req_en0(28usize))
            .field("req_en0[29]", &self.req_en0(29usize))
            .field("req_en0[30]", &self.req_en0(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable0Tog {{ req_en0[0]: {=bool:?}, req_en0[1]: {=bool:?}, req_en0[2]: {=bool:?}, req_en0[3]: {=bool:?}, req_en0[4]: {=bool:?}, req_en0[5]: {=bool:?}, req_en0[6]: {=bool:?}, req_en0[7]: {=bool:?}, req_en0[8]: {=bool:?}, req_en0[9]: {=bool:?}, req_en0[10]: {=bool:?}, req_en0[11]: {=bool:?}, req_en0[12]: {=bool:?}, req_en0[13]: {=bool:?}, req_en0[14]: {=bool:?}, req_en0[15]: {=bool:?}, req_en0[16]: {=bool:?}, req_en0[17]: {=bool:?}, req_en0[18]: {=bool:?}, req_en0[19]: {=bool:?}, req_en0[20]: {=bool:?}, req_en0[21]: {=bool:?}, req_en0[22]: {=bool:?}, req_en0[23]: {=bool:?}, req_en0[24]: {=bool:?}, req_en0[25]: {=bool:?}, req_en0[26]: {=bool:?}, req_en0[27]: {=bool:?}, req_en0[28]: {=bool:?}, req_en0[29]: {=bool:?}, req_en0[30]: {=bool:?} }}",
            self.req_en0(0usize),
            self.req_en0(1usize),
            self.req_en0(2usize),
            self.req_en0(3usize),
            self.req_en0(4usize),
            self.req_en0(5usize),
            self.req_en0(6usize),
            self.req_en0(7usize),
            self.req_en0(8usize),
            self.req_en0(9usize),
            self.req_en0(10usize),
            self.req_en0(11usize),
            self.req_en0(12usize),
            self.req_en0(13usize),
            self.req_en0(14usize),
            self.req_en0(15usize),
            self.req_en0(16usize),
            self.req_en0(17usize),
            self.req_en0(18usize),
            self.req_en0(19usize),
            self.req_en0(20usize),
            self.req_en0(21usize),
            self.req_en0(22usize),
            self.req_en0(23usize),
            self.req_en0(24usize),
            self.req_en0(25usize),
            self.req_en0(26usize),
            self.req_en0(27usize),
            self.req_en0(28usize),
            self.req_en0(29usize),
            self.req_en0(30usize)
        )
    }
}
#[doc = "DMA0 Request Enable1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable1(pub u32);
impl Dma0ReqEnable1 {
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en0(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[inline(always)]
    pub const fn set_req32_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en0(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[inline(always)]
    pub const fn set_req33_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en0(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[inline(always)]
    pub const fn set_req34_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en0(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[inline(always)]
    pub const fn set_req35_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en0(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[inline(always)]
    pub const fn set_req36_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en0(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[inline(always)]
    pub const fn set_req37_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en0(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[inline(always)]
    pub const fn set_req38_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en0(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req39_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en0(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req40_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en0(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req41_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en0(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req42_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en0(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req43_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en0(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req44_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en0(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req45_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en0(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req46_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en0(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req47_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en0(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req48_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en0(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req49_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en0(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req50_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en0(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req51_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en0(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req52_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en0(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req53_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en0(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req54_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en0(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[inline(always)]
    pub const fn set_req57_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en0(&self) -> ReqEn {
        let val = (self.0 >> 26usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[inline(always)]
    pub const fn set_req58_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en0(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[inline(always)]
    pub const fn set_req59_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en0(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[inline(always)]
    pub const fn set_req60_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en0(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[inline(always)]
    pub const fn set_req61_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en0(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[inline(always)]
    pub const fn set_req62_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en0(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[inline(always)]
    pub const fn set_req63_en0(&mut self, val: ReqEn) {
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
#[doc = "DMA0 Request Enable1."]
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
#[doc = "DMA0 Request Enable1."]
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
#[doc = "DMA0 Request Enable1."]
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
#[doc = "DMA0 Request Enable2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable2(pub u32);
impl Dma0ReqEnable2 {
    #[doc = "This register is used to enable and disable FlexIO0 shift register 3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en0(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 3 request."]
    #[inline(always)]
    pub const fn set_req64_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 4 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en0(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 4 request."]
    #[inline(always)]
    pub const fn set_req65_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 5 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en0(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 5 request."]
    #[inline(always)]
    pub const fn set_req66_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 6 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en0(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 6 request."]
    #[inline(always)]
    pub const fn set_req67_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 7 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en0(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 shift register 7 request."]
    #[inline(always)]
    pub const fn set_req68_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en0(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[inline(always)]
    pub const fn set_req69_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en0(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[inline(always)]
    pub const fn set_req70_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en0(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[inline(always)]
    pub const fn set_req71_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en0(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[inline(always)]
    pub const fn set_req72_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en0(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[inline(always)]
    pub const fn set_req73_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en0(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[inline(always)]
    pub const fn set_req74_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en0(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[inline(always)]
    pub const fn set_req75_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en0(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[inline(always)]
    pub const fn set_req76_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en0(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[inline(always)]
    pub const fn set_req77_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en0(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[inline(always)]
    pub const fn set_req78_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en0(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[inline(always)]
    pub const fn set_req79_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en0(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[inline(always)]
    pub const fn set_req80_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en0(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[inline(always)]
    pub const fn set_req81_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en0(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[inline(always)]
    pub const fn set_req82_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en0(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[inline(always)]
    pub const fn set_req83_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en0(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[inline(always)]
    pub const fn set_req84_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en0(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[inline(always)]
    pub const fn set_req85_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en0(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[inline(always)]
    pub const fn set_req86_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en0(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[inline(always)]
    pub const fn set_req87_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en0(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[inline(always)]
    pub const fn set_req88_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en0(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[inline(always)]
    pub const fn set_req91_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en0(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[inline(always)]
    pub const fn set_req92_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en0(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[inline(always)]
    pub const fn set_req93_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en0(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[inline(always)]
    pub const fn set_req94_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en0(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[inline(always)]
    pub const fn set_req95_en0(&mut self, val: ReqEn) {
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
#[doc = "DMA0 Request Enable2."]
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
#[doc = "DMA0 Request Enable2."]
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
#[doc = "DMA0 Request Enable2."]
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
#[doc = "DMA0 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3(pub u32);
impl Dma0ReqEnable3 {
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en0(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[inline(always)]
    pub const fn set_req96_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en0(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[inline(always)]
    pub const fn set_req97_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en0(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[inline(always)]
    pub const fn set_req98_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en0(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[inline(always)]
    pub const fn set_req99_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en0(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[inline(always)]
    pub const fn set_req100_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en0(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[inline(always)]
    pub const fn set_req101_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en0(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[inline(always)]
    pub const fn set_req102_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en0(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[inline(always)]
    pub const fn set_req103_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en0(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[inline(always)]
    pub const fn set_req104_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en0(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[inline(always)]
    pub const fn set_req105_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en0(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[inline(always)]
    pub const fn set_req106_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en0(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[inline(always)]
    pub const fn set_req107_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en0(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[inline(always)]
    pub const fn set_req108_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en0(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[inline(always)]
    pub const fn set_req109_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en0(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[inline(always)]
    pub const fn set_req110_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en0(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[inline(always)]
    pub const fn set_req111_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en0(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[inline(always)]
    pub const fn set_req112_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en0(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[inline(always)]
    pub const fn set_req113_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en0(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[inline(always)]
    pub const fn set_req114_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en0(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[inline(always)]
    pub const fn set_req115_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en0(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[inline(always)]
    pub const fn set_req116_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en0(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[inline(always)]
    pub const fn set_req117_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en0(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[inline(always)]
    pub const fn set_req118_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en0(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[inline(always)]
    pub const fn set_req119_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en0(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[inline(always)]
    pub const fn set_req120_en0(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en0(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[inline(always)]
    pub const fn set_req121_en0(&mut self, val: ReqEn) {
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
#[doc = "DMA0 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3Clr(pub u32);
impl Dma0ReqEnable3Clr {
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA0_REQ_ENABLE3."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en0(&self, n: usize) -> bool {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA0_REQ_ENABLE3."]
    #[inline(always)]
    pub const fn set_req_en0(&mut self, n: usize, val: bool) {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en0[0]", &self.req_en0(0usize))
            .field("req_en0[1]", &self.req_en0(1usize))
            .field("req_en0[2]", &self.req_en0(2usize))
            .field("req_en0[3]", &self.req_en0(3usize))
            .field("req_en0[4]", &self.req_en0(4usize))
            .field("req_en0[5]", &self.req_en0(5usize))
            .field("req_en0[6]", &self.req_en0(6usize))
            .field("req_en0[7]", &self.req_en0(7usize))
            .field("req_en0[8]", &self.req_en0(8usize))
            .field("req_en0[9]", &self.req_en0(9usize))
            .field("req_en0[10]", &self.req_en0(10usize))
            .field("req_en0[11]", &self.req_en0(11usize))
            .field("req_en0[12]", &self.req_en0(12usize))
            .field("req_en0[13]", &self.req_en0(13usize))
            .field("req_en0[14]", &self.req_en0(14usize))
            .field("req_en0[15]", &self.req_en0(15usize))
            .field("req_en0[16]", &self.req_en0(16usize))
            .field("req_en0[17]", &self.req_en0(17usize))
            .field("req_en0[18]", &self.req_en0(18usize))
            .field("req_en0[19]", &self.req_en0(19usize))
            .field("req_en0[20]", &self.req_en0(20usize))
            .field("req_en0[21]", &self.req_en0(21usize))
            .field("req_en0[22]", &self.req_en0(22usize))
            .field("req_en0[23]", &self.req_en0(23usize))
            .field("req_en0[24]", &self.req_en0(24usize))
            .field("req_en0[25]", &self.req_en0(25usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable3Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable3Clr {{ req_en0[0]: {=bool:?}, req_en0[1]: {=bool:?}, req_en0[2]: {=bool:?}, req_en0[3]: {=bool:?}, req_en0[4]: {=bool:?}, req_en0[5]: {=bool:?}, req_en0[6]: {=bool:?}, req_en0[7]: {=bool:?}, req_en0[8]: {=bool:?}, req_en0[9]: {=bool:?}, req_en0[10]: {=bool:?}, req_en0[11]: {=bool:?}, req_en0[12]: {=bool:?}, req_en0[13]: {=bool:?}, req_en0[14]: {=bool:?}, req_en0[15]: {=bool:?}, req_en0[16]: {=bool:?}, req_en0[17]: {=bool:?}, req_en0[18]: {=bool:?}, req_en0[19]: {=bool:?}, req_en0[20]: {=bool:?}, req_en0[21]: {=bool:?}, req_en0[22]: {=bool:?}, req_en0[23]: {=bool:?}, req_en0[24]: {=bool:?}, req_en0[25]: {=bool:?} }}",
            self.req_en0(0usize),
            self.req_en0(1usize),
            self.req_en0(2usize),
            self.req_en0(3usize),
            self.req_en0(4usize),
            self.req_en0(5usize),
            self.req_en0(6usize),
            self.req_en0(7usize),
            self.req_en0(8usize),
            self.req_en0(9usize),
            self.req_en0(10usize),
            self.req_en0(11usize),
            self.req_en0(12usize),
            self.req_en0(13usize),
            self.req_en0(14usize),
            self.req_en0(15usize),
            self.req_en0(16usize),
            self.req_en0(17usize),
            self.req_en0(18usize),
            self.req_en0(19usize),
            self.req_en0(20usize),
            self.req_en0(21usize),
            self.req_en0(22usize),
            self.req_en0(23usize),
            self.req_en0(24usize),
            self.req_en0(25usize)
        )
    }
}
#[doc = "DMA0 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnable3Set(pub u32);
impl Dma0ReqEnable3Set {
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA0_REQ_ENABLE3."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en0(&self, n: usize) -> bool {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA0_REQ_ENABLE3."]
    #[inline(always)]
    pub const fn set_req_en0(&mut self, n: usize, val: bool) {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en0[0]", &self.req_en0(0usize))
            .field("req_en0[1]", &self.req_en0(1usize))
            .field("req_en0[2]", &self.req_en0(2usize))
            .field("req_en0[3]", &self.req_en0(3usize))
            .field("req_en0[4]", &self.req_en0(4usize))
            .field("req_en0[5]", &self.req_en0(5usize))
            .field("req_en0[6]", &self.req_en0(6usize))
            .field("req_en0[7]", &self.req_en0(7usize))
            .field("req_en0[8]", &self.req_en0(8usize))
            .field("req_en0[9]", &self.req_en0(9usize))
            .field("req_en0[10]", &self.req_en0(10usize))
            .field("req_en0[11]", &self.req_en0(11usize))
            .field("req_en0[12]", &self.req_en0(12usize))
            .field("req_en0[13]", &self.req_en0(13usize))
            .field("req_en0[14]", &self.req_en0(14usize))
            .field("req_en0[15]", &self.req_en0(15usize))
            .field("req_en0[16]", &self.req_en0(16usize))
            .field("req_en0[17]", &self.req_en0(17usize))
            .field("req_en0[18]", &self.req_en0(18usize))
            .field("req_en0[19]", &self.req_en0(19usize))
            .field("req_en0[20]", &self.req_en0(20usize))
            .field("req_en0[21]", &self.req_en0(21usize))
            .field("req_en0[22]", &self.req_en0(22usize))
            .field("req_en0[23]", &self.req_en0(23usize))
            .field("req_en0[24]", &self.req_en0(24usize))
            .field("req_en0[25]", &self.req_en0(25usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnable3Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ReqEnable3Set {{ req_en0[0]: {=bool:?}, req_en0[1]: {=bool:?}, req_en0[2]: {=bool:?}, req_en0[3]: {=bool:?}, req_en0[4]: {=bool:?}, req_en0[5]: {=bool:?}, req_en0[6]: {=bool:?}, req_en0[7]: {=bool:?}, req_en0[8]: {=bool:?}, req_en0[9]: {=bool:?}, req_en0[10]: {=bool:?}, req_en0[11]: {=bool:?}, req_en0[12]: {=bool:?}, req_en0[13]: {=bool:?}, req_en0[14]: {=bool:?}, req_en0[15]: {=bool:?}, req_en0[16]: {=bool:?}, req_en0[17]: {=bool:?}, req_en0[18]: {=bool:?}, req_en0[19]: {=bool:?}, req_en0[20]: {=bool:?}, req_en0[21]: {=bool:?}, req_en0[22]: {=bool:?}, req_en0[23]: {=bool:?}, req_en0[24]: {=bool:?}, req_en0[25]: {=bool:?} }}",
            self.req_en0(0usize),
            self.req_en0(1usize),
            self.req_en0(2usize),
            self.req_en0(3usize),
            self.req_en0(4usize),
            self.req_en0(5usize),
            self.req_en0(6usize),
            self.req_en0(7usize),
            self.req_en0(8usize),
            self.req_en0(9usize),
            self.req_en0(10usize),
            self.req_en0(11usize),
            self.req_en0(12usize),
            self.req_en0(13usize),
            self.req_en0(14usize),
            self.req_en0(15usize),
            self.req_en0(16usize),
            self.req_en0(17usize),
            self.req_en0(18usize),
            self.req_en0(19usize),
            self.req_en0(20usize),
            self.req_en0(21usize),
            self.req_en0(22usize),
            self.req_en0(23usize),
            self.req_en0(24usize),
            self.req_en0(25usize)
        )
    }
}
#[doc = "DMA1 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0(pub u32);
impl Dma1ReqEnable0 {
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req1_en1(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
    #[inline(always)]
    pub const fn set_req1_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req2_en1(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
    #[inline(always)]
    pub const fn set_req2_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req3_en1(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT0 request."]
    #[inline(always)]
    pub const fn set_req3_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req4_en1(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT1 request."]
    #[inline(always)]
    pub const fn set_req4_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req5_en1(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT2 request."]
    #[inline(always)]
    pub const fn set_req5_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req6_en1(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PINT0 INT3 request."]
    #[inline(always)]
    pub const fn set_req6_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req7_en1(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req7_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req8_en1(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER0 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req8_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req9_en1(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req9_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req10_en1(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER1 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req10_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req11_en1(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req11_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req12_en1(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER2 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req12_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req13_en1(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req13_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req14_en1(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER3 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req14_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req15_en1(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M0 request."]
    #[inline(always)]
    pub const fn set_req15_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req16_en1(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CTIMER4 DMAREQ_M1 request."]
    #[inline(always)]
    pub const fn set_req16_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req17_en1(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable WUU0 wake up event request."]
    #[inline(always)]
    pub const fn set_req17_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req18_en1(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable MICFIL0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req18_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req19_en1(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA0 request."]
    #[inline(always)]
    pub const fn set_req19_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req20_en1(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SCT0 DMA1 request."]
    #[inline(always)]
    pub const fn set_req20_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req21_en1(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO A request."]
    #[inline(always)]
    pub const fn set_req21_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req22_en1(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC0 FIFO B request."]
    #[inline(always)]
    pub const fn set_req22_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req23_en1(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO A request."]
    #[inline(always)]
    pub const fn set_req23_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req24_en1(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable ADC1 FIFO B request."]
    #[inline(always)]
    pub const fn set_req24_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req25_en1(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC0 FIFO_request."]
    #[inline(always)]
    pub const fn set_req25_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req26_en1(&self) -> ReqEn {
        let val = (self.0 >> 26usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC1 FIFO_request."]
    #[inline(always)]
    pub const fn set_req26_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req27_en1(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable DAC2 FIFO_request."]
    #[inline(always)]
    pub const fn set_req27_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req28_en1(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP0 DMA_request."]
    #[inline(always)]
    pub const fn set_req28_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req29_en1(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP1 DMA_request."]
    #[inline(always)]
    pub const fn set_req29_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[must_use]
    #[inline(always)]
    pub const fn req30_en1(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CMP2 DMA_request."]
    #[inline(always)]
    pub const fn set_req30_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req31_en1(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0A request."]
    #[inline(always)]
    pub const fn set_req31_en1(&mut self, val: ReqEn) {
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
#[doc = "DMA1 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Clr(pub u32);
impl Dma1ReqEnable0Clr {
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Clr {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize)
        )
    }
}
#[doc = "DMA1 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Set(pub u32);
impl Dma1ReqEnable0Set {
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Set {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize)
        )
    }
}
#[doc = "DMA1 Request Enable0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable0Tog(pub u32);
impl Dma1ReqEnable0Tog {
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA1_REQ_ENABLE0."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 31usize);
        let offs = 1usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable0Tog {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize)
        )
    }
}
#[doc = "DMA1 Request Enable1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable1(pub u32);
impl Dma1ReqEnable1 {
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req32_en1(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT0B request."]
    #[inline(always)]
    pub const fn set_req32_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req33_en1(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1A request."]
    #[inline(always)]
    pub const fn set_req33_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req34_en1(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT1B request."]
    #[inline(always)]
    pub const fn set_req34_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req35_en1(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2A request."]
    #[inline(always)]
    pub const fn set_req35_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req36_en1(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT2B request."]
    #[inline(always)]
    pub const fn set_req36_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[must_use]
    #[inline(always)]
    pub const fn req37_en1(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3A request."]
    #[inline(always)]
    pub const fn set_req37_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[must_use]
    #[inline(always)]
    pub const fn req38_en1(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EVTG0 OUT3B request."]
    #[inline(always)]
    pub const fn set_req38_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req39_en1(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req39_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req40_en1(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req40_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req41_en1(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req41_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req42_en1(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req42_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req43_en1(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req43_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req44_en1(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req44_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req45_en1(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req45_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req46_en1(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM0 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req46_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req47_en1(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt0 request."]
    #[inline(always)]
    pub const fn set_req47_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req48_en1(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt1 request."]
    #[inline(always)]
    pub const fn set_req48_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req49_en1(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt2 request."]
    #[inline(always)]
    pub const fn set_req49_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req50_en1(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_capt3 request."]
    #[inline(always)]
    pub const fn set_req50_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req51_en1(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val0 request."]
    #[inline(always)]
    pub const fn set_req51_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req52_en1(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val1 request."]
    #[inline(always)]
    pub const fn set_req52_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req53_en1(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val2 request."]
    #[inline(always)]
    pub const fn set_req53_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[must_use]
    #[inline(always)]
    pub const fn req54_en1(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable PWM1 Req_val3 request."]
    #[inline(always)]
    pub const fn set_req54_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req57_en1(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR0 counter match event request."]
    #[inline(always)]
    pub const fn set_req57_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[must_use]
    #[inline(always)]
    pub const fn req58_en1(&self) -> ReqEn {
        let val = (self.0 >> 26usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LPTMR1 counter match event request."]
    #[inline(always)]
    pub const fn set_req58_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req59_en1(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN0 DMA request."]
    #[inline(always)]
    pub const fn set_req59_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req60_en1(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable CAN1 DMA request."]
    #[inline(always)]
    pub const fn set_req60_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req61_en1(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter0 Status DMA request OR Timer0 Status DMA request."]
    #[inline(always)]
    pub const fn set_req61_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req62_en1(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter1 Status DMA request OR Timer1 Status DMA request."]
    #[inline(always)]
    pub const fn set_req62_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req63_en1(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter2 Status DMA request OR Timer2 Status DMA request."]
    #[inline(always)]
    pub const fn set_req63_en1(&mut self, val: ReqEn) {
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
#[doc = "DMA1 Request Enable1."]
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
#[doc = "DMA1 Request Enable1."]
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
#[doc = "DMA1 Request Enable1."]
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
#[doc = "DMA1 Request Enable2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2(pub u32);
impl Dma1ReqEnable2 {
    #[doc = "This register is used to enable and disable FlexIO0 Shifter3 Status DMA request OR Timer3 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req64_en1(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter3 Status DMA request OR Timer3 Status DMA request."]
    #[inline(always)]
    pub const fn set_req64_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter4 Status DMA request OR Timer4 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req65_en1(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter4 Status DMA request OR Timer4 Status DMA request."]
    #[inline(always)]
    pub const fn set_req65_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter5 Status DMA request OR Timer5 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req66_en1(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter5 Status DMA request OR Timer5 Status DMA request."]
    #[inline(always)]
    pub const fn set_req66_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter6 Status DMA request OR Timer6 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req67_en1(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter6 Status DMA request OR Timer6 Status DMA request."]
    #[inline(always)]
    pub const fn set_req67_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter7 Status DMA request OR Timer7 Status DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn req68_en1(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable FlexIO0 Shifter7 Status DMA request OR Timer7 Status DMA request."]
    #[inline(always)]
    pub const fn set_req68_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req69_en1(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 receive request."]
    #[inline(always)]
    pub const fn set_req69_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req70_en1(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM0 transmit request."]
    #[inline(always)]
    pub const fn set_req70_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req71_en1(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 receive request."]
    #[inline(always)]
    pub const fn set_req71_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req72_en1(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM1 transmit request."]
    #[inline(always)]
    pub const fn set_req72_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req73_en1(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 receive request."]
    #[inline(always)]
    pub const fn set_req73_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req74_en1(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM2 transmit request."]
    #[inline(always)]
    pub const fn set_req74_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req75_en1(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 receive request."]
    #[inline(always)]
    pub const fn set_req75_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req76_en1(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM3 transmit request."]
    #[inline(always)]
    pub const fn set_req76_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req77_en1(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 receive request."]
    #[inline(always)]
    pub const fn set_req77_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req78_en1(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM4 transmit request."]
    #[inline(always)]
    pub const fn set_req78_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req79_en1(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 receive request."]
    #[inline(always)]
    pub const fn set_req79_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req80_en1(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM5 transmit request."]
    #[inline(always)]
    pub const fn set_req80_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req81_en1(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 receive request."]
    #[inline(always)]
    pub const fn set_req81_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req82_en1(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM6 transmit request."]
    #[inline(always)]
    pub const fn set_req82_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req83_en1(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 receive request."]
    #[inline(always)]
    pub const fn set_req83_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req84_en1(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM7 transmit request."]
    #[inline(always)]
    pub const fn set_req84_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req85_en1(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 receive request."]
    #[inline(always)]
    pub const fn set_req85_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req86_en1(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM8 transmit request."]
    #[inline(always)]
    pub const fn set_req86_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req87_en1(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 receive request."]
    #[inline(always)]
    pub const fn set_req87_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req88_en1(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable LP_FLEXCOMM9 transmit request."]
    #[inline(always)]
    pub const fn set_req88_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req91_en1(&self) -> ReqEn {
        let val = (self.0 >> 27usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 receive request."]
    #[inline(always)]
    pub const fn set_req91_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req92_en1(&self) -> ReqEn {
        let val = (self.0 >> 28usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM0 transmit request."]
    #[inline(always)]
    pub const fn set_req92_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req93_en1(&self) -> ReqEn {
        let val = (self.0 >> 29usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 receive request."]
    #[inline(always)]
    pub const fn set_req93_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req94_en1(&self) -> ReqEn {
        let val = (self.0 >> 30usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable EMVSIM1 transmit request."]
    #[inline(always)]
    pub const fn set_req94_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req95_en1(&self) -> ReqEn {
        let val = (self.0 >> 31usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 receive request."]
    #[inline(always)]
    pub const fn set_req95_en1(&mut self, val: ReqEn) {
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
#[doc = "DMA1 Request Enable2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Clr(pub u32);
impl Dma1ReqEnable2Clr {
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .field("req_en1[31]", &self.req_en1(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Clr {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?}, req_en1[31]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize),
            self.req_en1(31usize)
        )
    }
}
#[doc = "DMA1 Request Enable2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Set(pub u32);
impl Dma1ReqEnable2Set {
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .field("req_en1[31]", &self.req_en1(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Set {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?}, req_en1[31]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize),
            self.req_en1(31usize)
        )
    }
}
#[doc = "DMA1 Request Enable2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable2Tog(pub u32);
impl Dma1ReqEnable2Tog {
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit toggles the corresponding bit in DMA1_REQ_ENABLE2."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .field("req_en1[26]", &self.req_en1(26usize))
            .field("req_en1[27]", &self.req_en1(27usize))
            .field("req_en1[28]", &self.req_en1(28usize))
            .field("req_en1[29]", &self.req_en1(29usize))
            .field("req_en1[30]", &self.req_en1(30usize))
            .field("req_en1[31]", &self.req_en1(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable2Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable2Tog {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?}, req_en1[26]: {=bool:?}, req_en1[27]: {=bool:?}, req_en1[28]: {=bool:?}, req_en1[29]: {=bool:?}, req_en1[30]: {=bool:?}, req_en1[31]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize),
            self.req_en1(26usize),
            self.req_en1(27usize),
            self.req_en1(28usize),
            self.req_en1(29usize),
            self.req_en1(30usize),
            self.req_en1(31usize)
        )
    }
}
#[doc = "DMA1 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3(pub u32);
impl Dma1ReqEnable3 {
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req96_en1(&self) -> ReqEn {
        let val = (self.0 >> 0usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C0 transmit request."]
    #[inline(always)]
    pub const fn set_req96_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req97_en1(&self) -> ReqEn {
        let val = (self.0 >> 1usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 receive request."]
    #[inline(always)]
    pub const fn set_req97_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req98_en1(&self) -> ReqEn {
        let val = (self.0 >> 2usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable I3C1 transmit request."]
    #[inline(always)]
    pub const fn set_req98_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req99_en1(&self) -> ReqEn {
        let val = (self.0 >> 3usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 receive request."]
    #[inline(always)]
    pub const fn set_req99_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req100_en1(&self) -> ReqEn {
        let val = (self.0 >> 4usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI0 transmit request."]
    #[inline(always)]
    pub const fn set_req100_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[must_use]
    #[inline(always)]
    pub const fn req101_en1(&self) -> ReqEn {
        let val = (self.0 >> 5usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 receive request."]
    #[inline(always)]
    pub const fn set_req101_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[must_use]
    #[inline(always)]
    pub const fn req102_en1(&self) -> ReqEn {
        let val = (self.0 >> 6usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SAI1 transmit request."]
    #[inline(always)]
    pub const fn set_req102_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req103_en1(&self) -> ReqEn {
        let val = (self.0 >> 7usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[0\\] or ipd_req_alt \\[0\\] request."]
    #[inline(always)]
    pub const fn set_req103_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req104_en1(&self) -> ReqEn {
        let val = (self.0 >> 8usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[1\\] or ipd_req_alt \\[1\\] request."]
    #[inline(always)]
    pub const fn set_req104_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req105_en1(&self) -> ReqEn {
        let val = (self.0 >> 9usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[2\\] or ipd_req_alt \\[2\\] request."]
    #[inline(always)]
    pub const fn set_req105_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req106_en1(&self) -> ReqEn {
        let val = (self.0 >> 10usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[3\\] or ipd_req_alt \\[3\\] request."]
    #[inline(always)]
    pub const fn set_req106_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[must_use]
    #[inline(always)]
    pub const fn req107_en1(&self) -> ReqEn {
        let val = (self.0 >> 11usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable SINC0 ipd_req_sinc\\[4\\] or ipd_req_alt \\[4\\] request."]
    #[inline(always)]
    pub const fn set_req107_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req108_en1(&self) -> ReqEn {
        let val = (self.0 >> 12usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 0."]
    #[inline(always)]
    pub const fn set_req108_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req109_en1(&self) -> ReqEn {
        let val = (self.0 >> 13usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO0 pin event request 1."]
    #[inline(always)]
    pub const fn set_req109_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req110_en1(&self) -> ReqEn {
        let val = (self.0 >> 14usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 0."]
    #[inline(always)]
    pub const fn set_req110_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req111_en1(&self) -> ReqEn {
        let val = (self.0 >> 15usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO1 pin event request 1."]
    #[inline(always)]
    pub const fn set_req111_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req112_en1(&self) -> ReqEn {
        let val = (self.0 >> 16usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 0."]
    #[inline(always)]
    pub const fn set_req112_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req113_en1(&self) -> ReqEn {
        let val = (self.0 >> 17usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO2 pin event request 1."]
    #[inline(always)]
    pub const fn set_req113_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req114_en1(&self) -> ReqEn {
        let val = (self.0 >> 18usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 0."]
    #[inline(always)]
    pub const fn set_req114_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req115_en1(&self) -> ReqEn {
        let val = (self.0 >> 19usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO3 pin event request 1."]
    #[inline(always)]
    pub const fn set_req115_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req116_en1(&self) -> ReqEn {
        let val = (self.0 >> 20usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 0."]
    #[inline(always)]
    pub const fn set_req116_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req117_en1(&self) -> ReqEn {
        let val = (self.0 >> 21usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO4 pin event request 1."]
    #[inline(always)]
    pub const fn set_req117_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[must_use]
    #[inline(always)]
    pub const fn req118_en1(&self) -> ReqEn {
        let val = (self.0 >> 22usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 0."]
    #[inline(always)]
    pub const fn set_req118_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[must_use]
    #[inline(always)]
    pub const fn req119_en1(&self) -> ReqEn {
        let val = (self.0 >> 23usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable GPIO5 pin event request 1."]
    #[inline(always)]
    pub const fn set_req119_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[must_use]
    #[inline(always)]
    pub const fn req120_en1(&self) -> ReqEn {
        let val = (self.0 >> 24usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 end of scan request."]
    #[inline(always)]
    pub const fn set_req120_en1(&mut self, val: ReqEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[must_use]
    #[inline(always)]
    pub const fn req121_en1(&self) -> ReqEn {
        let val = (self.0 >> 25usize) & 0x01;
        ReqEn::from_bits(val as u8)
    }
    #[doc = "This register is used to enable and disable TSI0 out of range request."]
    #[inline(always)]
    pub const fn set_req121_en1(&mut self, val: ReqEn) {
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
#[doc = "DMA1 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3Clr(pub u32);
impl Dma1ReqEnable3Clr {
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE3."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the corresponding bit in DMA1_REQ_ENABLE3."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable3Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable3Clr {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize)
        )
    }
}
#[doc = "DMA1 Request Enable3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnable3Set(pub u32);
impl Dma1ReqEnable3Set {
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE3."]
    #[must_use]
    #[inline(always)]
    pub const fn req_en1(&self, n: usize) -> bool {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit sets the corresponding bit in DMA1_REQ_ENABLE3."]
    #[inline(always)]
    pub const fn set_req_en1(&mut self, n: usize, val: bool) {
        assert!(n < 26usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("req_en1[0]", &self.req_en1(0usize))
            .field("req_en1[1]", &self.req_en1(1usize))
            .field("req_en1[2]", &self.req_en1(2usize))
            .field("req_en1[3]", &self.req_en1(3usize))
            .field("req_en1[4]", &self.req_en1(4usize))
            .field("req_en1[5]", &self.req_en1(5usize))
            .field("req_en1[6]", &self.req_en1(6usize))
            .field("req_en1[7]", &self.req_en1(7usize))
            .field("req_en1[8]", &self.req_en1(8usize))
            .field("req_en1[9]", &self.req_en1(9usize))
            .field("req_en1[10]", &self.req_en1(10usize))
            .field("req_en1[11]", &self.req_en1(11usize))
            .field("req_en1[12]", &self.req_en1(12usize))
            .field("req_en1[13]", &self.req_en1(13usize))
            .field("req_en1[14]", &self.req_en1(14usize))
            .field("req_en1[15]", &self.req_en1(15usize))
            .field("req_en1[16]", &self.req_en1(16usize))
            .field("req_en1[17]", &self.req_en1(17usize))
            .field("req_en1[18]", &self.req_en1(18usize))
            .field("req_en1[19]", &self.req_en1(19usize))
            .field("req_en1[20]", &self.req_en1(20usize))
            .field("req_en1[21]", &self.req_en1(21usize))
            .field("req_en1[22]", &self.req_en1(22usize))
            .field("req_en1[23]", &self.req_en1(23usize))
            .field("req_en1[24]", &self.req_en1(24usize))
            .field("req_en1[25]", &self.req_en1(25usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnable3Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ReqEnable3Set {{ req_en1[0]: {=bool:?}, req_en1[1]: {=bool:?}, req_en1[2]: {=bool:?}, req_en1[3]: {=bool:?}, req_en1[4]: {=bool:?}, req_en1[5]: {=bool:?}, req_en1[6]: {=bool:?}, req_en1[7]: {=bool:?}, req_en1[8]: {=bool:?}, req_en1[9]: {=bool:?}, req_en1[10]: {=bool:?}, req_en1[11]: {=bool:?}, req_en1[12]: {=bool:?}, req_en1[13]: {=bool:?}, req_en1[14]: {=bool:?}, req_en1[15]: {=bool:?}, req_en1[16]: {=bool:?}, req_en1[17]: {=bool:?}, req_en1[18]: {=bool:?}, req_en1[19]: {=bool:?}, req_en1[20]: {=bool:?}, req_en1[21]: {=bool:?}, req_en1[22]: {=bool:?}, req_en1[23]: {=bool:?}, req_en1[24]: {=bool:?}, req_en1[25]: {=bool:?} }}",
            self.req_en1(0usize),
            self.req_en1(1usize),
            self.req_en1(2usize),
            self.req_en1(3usize),
            self.req_en1(4usize),
            self.req_en1(5usize),
            self.req_en1(6usize),
            self.req_en1(7usize),
            self.req_en1(8usize),
            self.req_en1(9usize),
            self.req_en1(10usize),
            self.req_en1(11usize),
            self.req_en1(12usize),
            self.req_en1(13usize),
            self.req_en1(14usize),
            self.req_en1(15usize),
            self.req_en1(16usize),
            self.req_en1(17usize),
            self.req_en1(18usize),
            self.req_en1(19usize),
            self.req_en1(20usize),
            self.req_en1(21usize),
            self.req_en1(22usize),
            self.req_en1(23usize),
            self.req_en1(24usize),
            self.req_en1(25usize)
        )
    }
}
#[doc = "EVTG Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgTrig(pub u32);
impl EvtgTrig {
    #[doc = "EVTG trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> EvtgTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        EvtgTrigInp::from_bits(val as u8)
    }
    #[doc = "EVTG trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: EvtgTrigInp) {
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
#[doc = "EXT Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTrig(pub u32);
impl ExtTrig {
    #[doc = "TRIG_OUTa pin input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> ExtTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        ExtTrigInp::from_bits(val as u8)
    }
    #[doc = "TRIG_OUTa pin input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: ExtTrigInp) {
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
#[doc = "PWM0 External Force Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Extforce(pub u32);
impl FlexPwm0Extforce {
    #[doc = "EXTFORCE input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm0ExtforceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm0ExtforceTrigin::from_bits(val as u8)
    }
    #[doc = "EXTFORCE input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm0ExtforceTrigin) {
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
#[doc = "PWM0 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0Fault(pub u32);
impl FlexPwm0Fault {
    #[doc = "FAULT input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm0FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm0FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm0FaultTrigin) {
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
#[doc = "PWM0 Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0SmExta(pub u32);
impl FlexPwm0SmExta {
    #[doc = "EXTA input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm0SmExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm0SmExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm0SmExtaTrigin) {
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
#[doc = "PWM0 External Synchronization."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm0SmExtsync(pub u32);
impl FlexPwm0SmExtsync {
    #[doc = "EXTSYNC input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm0SmExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm0SmExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm0SmExtsyncTrigin) {
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
#[doc = "PWM1 External Force Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Extforce(pub u32);
impl FlexPwm1Extforce {
    #[doc = "EXTFORCE input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm1ExtforceTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm1ExtforceTrigin::from_bits(val as u8)
    }
    #[doc = "EXTFORCE input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm1ExtforceTrigin) {
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
#[doc = "PWM1 Fault Input Trigger Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1Fault(pub u32);
impl FlexPwm1Fault {
    #[doc = "FAULT input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm1FaultTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm1FaultTrigin::from_bits(val as u8)
    }
    #[doc = "FAULT input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm1FaultTrigin) {
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
#[doc = "PWM1 Input EXTA Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1SmExta(pub u32);
impl FlexPwm1SmExta {
    #[doc = "EXTA input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm1SmExtaTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm1SmExtaTrigin::from_bits(val as u8)
    }
    #[doc = "EXTA input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm1SmExtaTrigin) {
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
#[doc = "PWM1 External Synchronization."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexPwm1SmExtsync(pub u32);
impl FlexPwm1SmExtsync {
    #[doc = "EXTSYNC input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> FlexPwm1SmExtsyncTrigin {
        let val = (self.0 >> 0usize) & 0x3f;
        FlexPwm1SmExtsyncTrigin::from_bits(val as u8)
    }
    #[doc = "EXTSYNC input connections for PWM1."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: FlexPwm1SmExtsyncTrigin) {
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
#[doc = "LP_FLEXCOMM0 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm0Trig(pub u32);
impl Flexcomm0Trig {
    #[doc = "LP_FLEXCOMM0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm0TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm0TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm0TrigInp) {
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
#[doc = "LP_FLEXCOMM1 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm1Trig(pub u32);
impl Flexcomm1Trig {
    #[doc = "LP_FLEXCOMM1 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm1TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm1TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM1 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm1TrigInp) {
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
#[doc = "LP_FLEXCOMM2 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm2Trig(pub u32);
impl Flexcomm2Trig {
    #[doc = "LP_FLEXCOMM2 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm2TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm2TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM2 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm2TrigInp) {
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
#[doc = "LP_FLEXCOMM3 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm3Trig(pub u32);
impl Flexcomm3Trig {
    #[doc = "LP_FLEXCOMM3 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm3TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm3TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM3 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm3TrigInp) {
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
#[doc = "LP_FLEXCOMM4 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm4Trig(pub u32);
impl Flexcomm4Trig {
    #[doc = "LP_FLEXCOMM4 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm4TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm4TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM4 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm4TrigInp) {
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
#[doc = "LP_FLEXCOMM5 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm5Trig(pub u32);
impl Flexcomm5Trig {
    #[doc = "LP_FLEXCOMM5 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm5TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm5TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM5 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm5TrigInp) {
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
#[doc = "LP_FLEXCOMM6 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm6Trig(pub u32);
impl Flexcomm6Trig {
    #[doc = "LP_FLEXCOMM6 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm6TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm6TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM6 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm6TrigInp) {
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
#[doc = "LP_FLEXCOMM7 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm7Trig(pub u32);
impl Flexcomm7Trig {
    #[doc = "LP_FLEXCOMM7 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm6TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm6TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM7 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm6TrigInp) {
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
#[doc = "LP_FLEXCOMM8 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm8Trig(pub u32);
impl Flexcomm8Trig {
    #[doc = "LP_FLEXCOMM8 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm6TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm6TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM8 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm6TrigInp) {
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
#[doc = "LP_FLEXCOMM9 Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm9Trig(pub u32);
impl Flexcomm9Trig {
    #[doc = "LP_FLEXCOMM9 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Flexcomm9TrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        Flexcomm9TrigInp::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM9 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Flexcomm9TrigInp) {
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
#[doc = "Selection for Frequency Measurement Reference Clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (binary value) for frequency measure function reference clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FreqmeasRefInp {
        let val = (self.0 >> 0usize) & 0x3f;
        FreqmeasRefInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function reference clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasRefInp) {
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
#[doc = "Selection for Frequency Measurement Target Clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTar(pub u32);
impl FreqmeasTar {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> FreqmeasTarInp {
        let val = (self.0 >> 0usize) & 0x3f;
        FreqmeasTarInp::from_bits(val as u8)
    }
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: FreqmeasTarInp) {
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
#[doc = "OPAMP Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpampTrig(pub u32);
impl OpampTrig {
    #[doc = "OPAMP trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> OpampTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        OpampTrigInp::from_bits(val as u8)
    }
    #[doc = "OPAMP trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: OpampTrigInp) {
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
#[doc = "Pin Interrupt Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pintsel(pub u32);
impl Pintsel {
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INP = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> PintselInp {
        let val = (self.0 >> 0usize) & 0x7f;
        PintselInp::from_bits(val as u8)
    }
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INP = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: PintselInp) {
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
#[doc = "PWM0 External Clock Trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0ExtClk(pub u32);
impl Pwm0ExtClk {
    #[doc = "EXT_CLK input connections for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Pwm0ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x07;
        Pwm0ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "EXT_CLK input connections for PWM0."]
    #[inline(always)]
    pub const fn set_trigin(&mut self, val: Pwm0ExtClkTrigin) {
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
#[doc = "PWM1 External Clock Trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1ExtClk(pub u32);
impl Pwm1ExtClk {
    #[doc = "EXT_CLK input connections for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigin(&self) -> Pwm1ExtClkTrigin {
        let val = (self.0 >> 0usize) & 0x0f;
        Pwm1ExtClkTrigin::from_bits(val as u8)
    }
    #[doc = "EXT_CLK input connections for PWM1."]
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
#[doc = "QDCouter_loop Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcHome(pub u32);
impl QdcHome {
    #[doc = "QDC0 HOME input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcHomeInp {
        let val = (self.0 >> 0usize) & 0x3f;
        QdcHomeInp::from_bits(val as u8)
    }
    #[doc = "QDC0 HOME input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcHomeInp) {
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
#[doc = "QDCouter_loop Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcIndex(pub u32);
impl QdcIndex {
    #[doc = "QDC0 INDEX input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcIndexInp {
        let val = (self.0 >> 0usize) & 0x3f;
        QdcIndexInp::from_bits(val as u8)
    }
    #[doc = "QDC0 INDEX input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcIndexInp) {
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
#[doc = "QDCouter_loop Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcPhasea(pub u32);
impl QdcPhasea {
    #[doc = "QDC0 PHASEA input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcPhaseaInp {
        let val = (self.0 >> 0usize) & 0x3f;
        QdcPhaseaInp::from_bits(val as u8)
    }
    #[doc = "QDC0 PHASEA input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcPhaseaInp) {
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
#[doc = "QDCouter_loop Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcPhaseb(pub u32);
impl QdcPhaseb {
    #[doc = "QDC0 PHASEB input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcPhasebInp {
        let val = (self.0 >> 0usize) & 0x3f;
        QdcPhasebInp::from_bits(val as u8)
    }
    #[doc = "QDC0 PHASEB input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcPhasebInp) {
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
#[doc = "QDCouter_loop Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QdcTrig(pub u32);
impl QdcTrig {
    #[doc = "QDC0 trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> QdcTrigInp {
        let val = (self.0 >> 0usize) & 0x3f;
        QdcTrigInp::from_bits(val as u8)
    }
    #[doc = "QDC0 trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: QdcTrigInp) {
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
#[doc = "Inputmux Register for SCT0 Input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0Inmux(pub u32);
impl Sct0Inmux {
    #[doc = "Input number to SCT0 inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Sct0InmuxInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Sct0InmuxInp::from_bits(val as u8)
    }
    #[doc = "Input number to SCT0 inputs."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Sct0InmuxInp) {
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
#[doc = "SINC Filter Channel Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SincFilterCh(pub u32);
impl SincFilterCh {
    #[doc = "SINC FILTER trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> SincFilterChInp {
        let val = (self.0 >> 0usize) & 0x3f;
        SincFilterChInp::from_bits(val as u8)
    }
    #[doc = "SINC FILTER trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: SincFilterChInp) {
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
#[doc = "Inputmux Register for SMARTDMA Arch B Inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartdmaarchbInmux(pub u32);
impl SmartdmaarchbInmux {
    #[doc = "Input number select to SmartDMA ARCHB input."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> SmartdmaarchbInmuxInp {
        let val = (self.0 >> 0usize) & 0x7f;
        SmartdmaarchbInmuxInp::from_bits(val as u8)
    }
    #[doc = "Input number select to SmartDMA ARCHB input."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: SmartdmaarchbInmuxInp) {
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
#[doc = "Trigger Register for CTIMER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0trig(pub u32);
impl Timer0trig {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer0trigInp) {
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
#[doc = "Trigger Register for CTIMER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1trig(pub u32);
impl Timer1trig {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer0trigInp) {
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
#[doc = "Trigger Register for CTIMER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2trig(pub u32);
impl Timer2trig {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer0trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer0trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer0trigInp) {
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
#[doc = "Trigger Register for CTIMER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3trig(pub u32);
impl Timer3trig {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer3trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer3trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer3trigInp) {
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
#[doc = "Trigger Register for CTIMER."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4trig(pub u32);
impl Timer4trig {
    #[doc = "Input number for CTIMER."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> Timer4trigInp {
        let val = (self.0 >> 0usize) & 0x7f;
        Timer4trigInp::from_bits(val as u8)
    }
    #[doc = "Input number for CTIMER."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: Timer4trigInp) {
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
#[doc = "TSI Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsiTrig(pub u32);
impl TsiTrig {
    #[doc = "TSI trigger input connections."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> TsiTrigInp {
        let val = (self.0 >> 0usize) & 0x03;
        TsiTrigInp::from_bits(val as u8)
    }
    #[doc = "TSI trigger input connections."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: TsiTrigInp) {
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
#[doc = "USB-FS Trigger Input Connections."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbfsTrig(pub u32);
impl UsbfsTrig {
    #[doc = "USB-FS trigger input connections. The trigger output of LP_FLEXCOMM is an input of peripheral INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> UsbfsTrigInp {
        let val = (self.0 >> 0usize) & 0x0f;
        UsbfsTrigInp::from_bits(val as u8)
    }
    #[doc = "USB-FS trigger input connections. The trigger output of LP_FLEXCOMM is an input of peripheral INPUTMUX."]
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Adc0TrigTrigin(u8);
impl Adc0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    pub const Val0: Self = Self(0x0);
    #[doc = "PINT PIN_INT1 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "SCT0 SCT_OUT9 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CTIMER0_MAT3 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CTIMER1_MAT3 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CTIMER2_MAT3 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CTIMER3_MAT3 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CTIMER4_MAT3 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "DCDC_Burst_Done_Trig input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val21: Self = Self(0x15);
    #[doc = "CMP1_OUT input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "CMP2_OUT input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    pub const Val31: Self = Self(0x1f);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    pub const Val40: Self = Self(0x28);
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    pub const Val41: Self = Self(0x29);
    #[doc = "EVTG_OUT0A input is selected."]
    pub const Val42: Self = Self(0x2a);
    #[doc = "EVTG_OUT0B input is selected."]
    pub const Val43: Self = Self(0x2b);
    #[doc = "EVTG_OUT1A input is selected."]
    pub const Val44: Self = Self(0x2c);
    #[doc = "EVTG_OUT1B input is selected."]
    pub const Val45: Self = Self(0x2d);
    #[doc = "EVTG_OUT2A input is selected."]
    pub const Val46: Self = Self(0x2e);
    #[doc = "EVTG_OUT2B input is selected."]
    pub const Val47: Self = Self(0x2f);
    #[doc = "EVTG_OUT3A input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "EVTG_OUT3B input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPTMR0 input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPTMR1 input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "FlexIO CH0 input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "FlexIO CH1 input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "FlexIO CH2 input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "FlexIO CH3 input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "SINC Filter CH0 Conversion Complete input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "SINC Filter CH1 Conversion Complete input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "SINC Filter CH2 Conversion Complete input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "SINC Filter CH3 Conversion Complete input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "SINC Filter CH4 Conversion Complete input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "WUU input is selected."]
    pub const Val65: Self = Self(0x41);
}
impl Adc0TrigTrigin {
    pub const fn from_bits(val: u8) -> Adc0TrigTrigin {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Adc0TrigTrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Val0"),
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
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x15 => f.write_str("Val21"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x1f => f.write_str("Val31"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
            0x28 => f.write_str("Val40"),
            0x29 => f.write_str("Val41"),
            0x2a => f.write_str("Val42"),
            0x2b => f.write_str("Val43"),
            0x2c => f.write_str("Val44"),
            0x2d => f.write_str("Val45"),
            0x2e => f.write_str("Val46"),
            0x2f => f.write_str("Val47"),
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
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0TrigTrigin {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Val0"),
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
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x15 => defmt::write!(f, "Val21"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x1f => defmt::write!(f, "Val31"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
            0x28 => defmt::write!(f, "Val40"),
            0x29 => defmt::write!(f, "Val41"),
            0x2a => defmt::write!(f, "Val42"),
            0x2b => defmt::write!(f, "Val43"),
            0x2c => defmt::write!(f, "Val44"),
            0x2d => defmt::write!(f, "Val45"),
            0x2e => defmt::write!(f, "Val46"),
            0x2f => defmt::write!(f, "Val47"),
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
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Adc0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Adc0TrigTrigin {
        Adc0TrigTrigin::from_bits(val)
    }
}
impl From<Adc0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Adc0TrigTrigin) -> u8 {
        Adc0TrigTrigin::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Adc1TrigTrigin(u8);
impl Adc1TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    pub const Val0: Self = Self(0x0);
    #[doc = "PINT PIN_INT2 input is selected."]
    pub const Val1: Self = Self(0x01);
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    pub const Val2: Self = Self(0x02);
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    pub const Val3: Self = Self(0x03);
    #[doc = "SCT0 SCT_OUT3 input is selected."]
    pub const Val4: Self = Self(0x04);
    #[doc = "CTIMER0_MAT3 input is selected."]
    pub const Val5: Self = Self(0x05);
    #[doc = "CTIMER1_MAT3 input is selected."]
    pub const Val6: Self = Self(0x06);
    #[doc = "CTIMER2_MAT3 input is selected."]
    pub const Val7: Self = Self(0x07);
    #[doc = "CTIMER3_MAT2 input is selected."]
    pub const Val8: Self = Self(0x08);
    #[doc = "CTIMER4_MAT1 input is selected."]
    pub const Val9: Self = Self(0x09);
    #[doc = "DCDC_Burst_Done_Trig input is selected."]
    pub const Val10: Self = Self(0x0a);
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    pub const Val12: Self = Self(0x0c);
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    pub const Val13: Self = Self(0x0d);
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    pub const Val14: Self = Self(0x0e);
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    pub const Val15: Self = Self(0x0f);
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    pub const Val16: Self = Self(0x10);
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    pub const Val17: Self = Self(0x11);
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    pub const Val18: Self = Self(0x12);
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    pub const Val19: Self = Self(0x13);
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    pub const Val20: Self = Self(0x14);
    #[doc = "CMP0_OUT input is selected."]
    pub const Val21: Self = Self(0x15);
    #[doc = "CMP1_OUT input is selected."]
    pub const Val22: Self = Self(0x16);
    #[doc = "CMP2_OUT input is selected."]
    pub const Val23: Self = Self(0x17);
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    pub const Val24: Self = Self(0x18);
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    pub const Val25: Self = Self(0x19);
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    pub const Val26: Self = Self(0x1a);
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    pub const Val27: Self = Self(0x1b);
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    pub const Val28: Self = Self(0x1c);
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    pub const Val29: Self = Self(0x1d);
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    pub const Val30: Self = Self(0x1e);
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    pub const Val31: Self = Self(0x1f);
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    pub const Val32: Self = Self(0x20);
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    pub const Val33: Self = Self(0x21);
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    pub const Val34: Self = Self(0x22);
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    pub const Val35: Self = Self(0x23);
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    pub const Val36: Self = Self(0x24);
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    pub const Val37: Self = Self(0x25);
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    pub const Val38: Self = Self(0x26);
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    pub const Val39: Self = Self(0x27);
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    pub const Val40: Self = Self(0x28);
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    pub const Val41: Self = Self(0x29);
    #[doc = "EVTG_OUT0A input is selected."]
    pub const Val42: Self = Self(0x2a);
    #[doc = "EVTG_OUT0B input is selected."]
    pub const Val43: Self = Self(0x2b);
    #[doc = "EVTG_OUT1A input is selected."]
    pub const Val44: Self = Self(0x2c);
    #[doc = "EVTG_OUT1B input is selected."]
    pub const Val45: Self = Self(0x2d);
    #[doc = "EVTG_OUT2A input is selected."]
    pub const Val46: Self = Self(0x2e);
    #[doc = "EVTG_OUT2B input is selected."]
    pub const Val47: Self = Self(0x2f);
    #[doc = "EVTG_OUT3A input is selected."]
    pub const Val48: Self = Self(0x30);
    #[doc = "EVTG_OUT3B input is selected."]
    pub const Val49: Self = Self(0x31);
    #[doc = "LPTMR0 input is selected."]
    pub const Val50: Self = Self(0x32);
    #[doc = "LPTMR1 input is selected."]
    pub const Val51: Self = Self(0x33);
    #[doc = "FlexIO CH0 input is selected."]
    pub const Val52: Self = Self(0x34);
    #[doc = "FlexIO CH1 input is selected."]
    pub const Val53: Self = Self(0x35);
    #[doc = "FlexIO CH2 input is selected."]
    pub const Val54: Self = Self(0x36);
    #[doc = "FlexIO CH3 input is selected."]
    pub const Val55: Self = Self(0x37);
    #[doc = "SINC Filter CH0 Conversion Complete input is selected."]
    pub const Val56: Self = Self(0x38);
    #[doc = "SINC Filter CH1 Conversion Complete input is selected."]
    pub const Val57: Self = Self(0x39);
    #[doc = "SINC Filter CH2 Conversion Complete input is selected."]
    pub const Val58: Self = Self(0x3a);
    #[doc = "SINC Filter CH3 Conversion Complete input is selected."]
    pub const Val59: Self = Self(0x3b);
    #[doc = "SINC Filter CH4 Conversion Complete input is selected."]
    pub const Val60: Self = Self(0x3c);
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    pub const Val61: Self = Self(0x3d);
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    pub const Val62: Self = Self(0x3e);
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    pub const Val63: Self = Self(0x3f);
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    pub const Val64: Self = Self(0x40);
    #[doc = "WUU input is selected."]
    pub const Val65: Self = Self(0x41);
}
impl Adc1TrigTrigin {
    pub const fn from_bits(val: u8) -> Adc1TrigTrigin {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Adc1TrigTrigin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Val0"),
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
            0x0c => f.write_str("Val12"),
            0x0d => f.write_str("Val13"),
            0x0e => f.write_str("Val14"),
            0x0f => f.write_str("Val15"),
            0x10 => f.write_str("Val16"),
            0x11 => f.write_str("Val17"),
            0x12 => f.write_str("Val18"),
            0x13 => f.write_str("Val19"),
            0x14 => f.write_str("Val20"),
            0x15 => f.write_str("Val21"),
            0x16 => f.write_str("Val22"),
            0x17 => f.write_str("Val23"),
            0x18 => f.write_str("Val24"),
            0x19 => f.write_str("Val25"),
            0x1a => f.write_str("Val26"),
            0x1b => f.write_str("Val27"),
            0x1c => f.write_str("Val28"),
            0x1d => f.write_str("Val29"),
            0x1e => f.write_str("Val30"),
            0x1f => f.write_str("Val31"),
            0x20 => f.write_str("Val32"),
            0x21 => f.write_str("Val33"),
            0x22 => f.write_str("Val34"),
            0x23 => f.write_str("Val35"),
            0x24 => f.write_str("Val36"),
            0x25 => f.write_str("Val37"),
            0x26 => f.write_str("Val38"),
            0x27 => f.write_str("Val39"),
            0x28 => f.write_str("Val40"),
            0x29 => f.write_str("Val41"),
            0x2a => f.write_str("Val42"),
            0x2b => f.write_str("Val43"),
            0x2c => f.write_str("Val44"),
            0x2d => f.write_str("Val45"),
            0x2e => f.write_str("Val46"),
            0x2f => f.write_str("Val47"),
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
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1TrigTrigin {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Val0"),
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
            0x0c => defmt::write!(f, "Val12"),
            0x0d => defmt::write!(f, "Val13"),
            0x0e => defmt::write!(f, "Val14"),
            0x0f => defmt::write!(f, "Val15"),
            0x10 => defmt::write!(f, "Val16"),
            0x11 => defmt::write!(f, "Val17"),
            0x12 => defmt::write!(f, "Val18"),
            0x13 => defmt::write!(f, "Val19"),
            0x14 => defmt::write!(f, "Val20"),
            0x15 => defmt::write!(f, "Val21"),
            0x16 => defmt::write!(f, "Val22"),
            0x17 => defmt::write!(f, "Val23"),
            0x18 => defmt::write!(f, "Val24"),
            0x19 => defmt::write!(f, "Val25"),
            0x1a => defmt::write!(f, "Val26"),
            0x1b => defmt::write!(f, "Val27"),
            0x1c => defmt::write!(f, "Val28"),
            0x1d => defmt::write!(f, "Val29"),
            0x1e => defmt::write!(f, "Val30"),
            0x1f => defmt::write!(f, "Val31"),
            0x20 => defmt::write!(f, "Val32"),
            0x21 => defmt::write!(f, "Val33"),
            0x22 => defmt::write!(f, "Val34"),
            0x23 => defmt::write!(f, "Val35"),
            0x24 => defmt::write!(f, "Val36"),
            0x25 => defmt::write!(f, "Val37"),
            0x26 => defmt::write!(f, "Val38"),
            0x27 => defmt::write!(f, "Val39"),
            0x28 => defmt::write!(f, "Val40"),
            0x29 => defmt::write!(f, "Val41"),
            0x2a => defmt::write!(f, "Val42"),
            0x2b => defmt::write!(f, "Val43"),
            0x2c => defmt::write!(f, "Val44"),
            0x2d => defmt::write!(f, "Val45"),
            0x2e => defmt::write!(f, "Val46"),
            0x2f => defmt::write!(f, "Val47"),
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
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Adc1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Adc1TrigTrigin {
        Adc1TrigTrigin::from_bits(val)
    }
}
impl From<Adc1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Adc1TrigTrigin) -> u8 {
        Adc1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT6 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT6 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected."]
    Val27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected."]
    Val28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected."]
    Val29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected."]
    Val30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected."]
    Val31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected."]
    Val32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected."]
    Val34 = 0x22,
    #[doc = "LPTMR0 input is selected."]
    Val35 = 0x23,
    #[doc = "LPTMR1 input is selected."]
    Val36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val40 = 0x28,
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
impl Cmp0TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp0TrigTrigin {
        Cmp0TrigTrigin::from_bits(val)
    }
}
impl From<Cmp0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp0TrigTrigin) -> u8 {
        Cmp0TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT7 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER3_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected."]
    Val27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected."]
    Val28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected."]
    Val29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected."]
    Val30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected."]
    Val31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected."]
    Val32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected."]
    Val34 = 0x22,
    #[doc = "LPTMR0 input is selected."]
    Val35 = 0x23,
    #[doc = "LPTMR1 input is selected."]
    Val36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val40 = 0x28,
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
impl Cmp1TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp1TrigTrigin {
        Cmp1TrigTrigin::from_bits(val)
    }
}
impl From<Cmp1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp1TrigTrigin) -> u8 {
        Cmp1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT8 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT2 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val14 = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val17 = 0x11,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val18 = 0x12,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val19 = 0x13,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val20 = 0x14,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val21 = 0x15,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val22 = 0x16,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val24 = 0x18,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val25 = 0x19,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val26 = 0x1a,
    #[doc = "EVTG_OUT0A input is selected."]
    Val27 = 0x1b,
    #[doc = "EVTG_OUT0B input is selected."]
    Val28 = 0x1c,
    #[doc = "EVTG_OUT1A input is selected."]
    Val29 = 0x1d,
    #[doc = "EVTG_OUT1B input is selected."]
    Val30 = 0x1e,
    #[doc = "EVTG_OUT2A input is selected."]
    Val31 = 0x1f,
    #[doc = "EVTG_OUT2B input is selected."]
    Val32 = 0x20,
    #[doc = "EVTG_OUT3A input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT3B input is selected."]
    Val34 = 0x22,
    #[doc = "LPTMR0 input is selected."]
    Val35 = 0x23,
    #[doc = "LPTMR1 input is selected."]
    Val36 = 0x24,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val40 = 0x28,
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
impl Cmp2TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Cmp2TrigTrigin {
        Cmp2TrigTrigin::from_bits(val)
    }
}
impl From<Cmp2TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Cmp2TrigTrigin) -> u8 {
        Cmp2TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer0CapInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer0CapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer0CapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer0CapInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer0CapInp {
        Ctimer0CapInp::from_bits(val)
    }
}
impl From<Ctimer0CapInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer0CapInp) -> u8 {
        Ctimer0CapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer1CapInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer1CapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer1CapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer1CapInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer1CapInp {
        Ctimer1CapInp::from_bits(val)
    }
}
impl From<Ctimer1CapInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer1CapInp) -> u8 {
        Ctimer1CapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer2CapInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer2CapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer2CapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer2CapInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer2CapInp {
        Ctimer2CapInp::from_bits(val)
    }
}
impl From<Ctimer2CapInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer2CapInp) -> u8 {
        Ctimer2CapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer3CapInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer3CapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer3CapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer3CapInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer3CapInp {
        Ctimer3CapInp::from_bits(val)
    }
}
impl From<Ctimer3CapInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer3CapInp) -> u8 {
        Ctimer3CapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctimer4CapInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Ctimer4CapInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctimer4CapInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctimer4CapInp {
    #[inline(always)]
    fn from(val: u8) -> Ctimer4CapInp {
        Ctimer4CapInp::from_bits(val)
    }
}
impl From<Ctimer4CapInp> for u8 {
    #[inline(always)]
    fn from(val: Ctimer4CapInp) -> u8 {
        Ctimer4CapInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT3 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT0 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "LPTMR0 input is selected."]
    Val26 = 0x1a,
    #[doc = "LPTMR1 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val31 = 0x1f,
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
impl Dac0TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac0TrigTrigin {
        Dac0TrigTrigin::from_bits(val)
    }
}
impl From<Dac0TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac0TrigTrigin) -> u8 {
        Dac0TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "LPTMR0 input is selected."]
    Val26 = 0x1a,
    #[doc = "LPTMR1 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val31 = 0x1f,
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
impl Dac1TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac1TrigTrigin {
        Dac1TrigTrigin::from_bits(val)
    }
}
impl From<Dac1TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac1TrigTrigin) -> u8 {
        Dac1TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2TrigTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "LPTMR0 input is selected."]
    Val26 = 0x1a,
    #[doc = "LPTMR1 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val31 = 0x1f,
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
impl Dac2TrigTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2TrigTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2TrigTrigin {
    #[inline(always)]
    fn from(val: u8) -> Dac2TrigTrigin {
        Dac2TrigTrigin::from_bits(val)
    }
}
impl From<Dac2TrigTrigin> for u8 {
    #[inline(always)]
    fn from(val: Dac2TrigTrigin) -> u8 {
        Dac2TrigTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgTrigInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT0 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT1 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER2_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTIMER4_MAT2 input is selected."]
    Val11 = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_IRQ input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC1_IRQ input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val20 = 0x14,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val21 = 0x15,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val22 = 0x16,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val23 = 0x17,
    #[doc = "CMP0_OUT input is selected."]
    Val24 = 0x18,
    #[doc = "CMP1_OUT input is selected."]
    Val25 = 0x19,
    #[doc = "CMP2_OUT input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
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
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val37 = 0x25,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val38 = 0x26,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val39 = 0x27,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val40 = 0x28,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val41 = 0x29,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val42 = 0x2a,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val43 = 0x2b,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN0 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN1 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN2 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN3 input is selected."]
    Val48 = 0x30,
    #[doc = "LPTMR0 input is selected."]
    Val49 = 0x31,
    #[doc = "LPTMR1 input is selected."]
    Val50 = 0x32,
    #[doc = "SINC Filter CH0 Break input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH1 Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH2 Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH3 Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH4 Break input is selected."]
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
impl EvtgTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgTrigInp {
    #[inline(always)]
    fn from(val: u8) -> EvtgTrigInp {
        EvtgTrigInp::from_bits(val)
    }
}
impl From<EvtgTrigInp> for u8 {
    #[inline(always)]
    fn from(val: EvtgTrigInp) -> u8 {
        EvtgTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtTrigInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected."]
    Val1 = 0x01,
    #[doc = "ADC0_IRQ input is selected."]
    Val2 = 0x02,
    #[doc = "ADC1_IRQ input is selected."]
    Val3 = 0x03,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val4 = 0x04,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val5 = 0x05,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val6 = 0x06,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val7 = 0x07,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val8 = 0x08,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val9 = 0x09,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val10 = 0x0a,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val11 = 0x0b,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val12 = 0x0c,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val13 = 0x0d,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val14 = 0x0e,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val15 = 0x0f,
    #[doc = "EVTG_OUT0A input is selected."]
    Val16 = 0x10,
    #[doc = "EVTG_OUT0B input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT1A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT1B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT2A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT2B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT3A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT3B input is selected."]
    Val23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    #[doc = "LPTMR0 input is selected."]
    Val26 = 0x1a,
    #[doc = "LPTMR1 input is selected."]
    Val27 = 0x1b,
    #[doc = "SCT Out0 input is selected."]
    Val28 = 0x1c,
    #[doc = "SCT Out1 input is selected."]
    Val29 = 0x1d,
    #[doc = "SCT Out2 input is selected."]
    Val30 = 0x1e,
    #[doc = "SCT Out3 input is selected."]
    Val31 = 0x1f,
    #[doc = "SCT Out4 input is selected."]
    Val32 = 0x20,
    #[doc = "SCT Out5 input is selected."]
    Val33 = 0x21,
    #[doc = "LP_FLEXCOMM0 trigger output 3 input is selected."]
    Val34 = 0x22,
    #[doc = "LP_FLEXCOMM1 trigger output 3 input is selected."]
    Val35 = 0x23,
    #[doc = "LP_FLEXCOMM2 trigger output 3 input is selected."]
    Val36 = 0x24,
    #[doc = "LP_FLEXCOMM3 trigger output 3 input is selected."]
    Val37 = 0x25,
    #[doc = "LP_FLEXCOMM4 trigger output 3 input is selected."]
    Val38 = 0x26,
    #[doc = "LP_FLEXCOMM5 trigger output 3 input is selected."]
    Val39 = 0x27,
    #[doc = "LP_FLEXCOMM6 trigger output 3 input is selected."]
    Val40 = 0x28,
    #[doc = "LP_FLEXCOMM7 trigger output 3 input is selected."]
    Val41 = 0x29,
    #[doc = "LP_FLEXCOMM8 trigger output 3 input is selected."]
    Val42 = 0x2a,
    #[doc = "LP_FLEXCOMM9 trigger output 3 input is selected."]
    Val43 = 0x2b,
    #[doc = "CMP0_OUT input is selected."]
    Val44 = 0x2c,
    #[doc = "CMP1_OUT input is selected."]
    Val45 = 0x2d,
    #[doc = "CMP2_OUT input is selected."]
    Val46 = 0x2e,
    #[doc = "ENET_PPS_OUT_0 input is selected."]
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
impl ExtTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
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
pub enum FlexPwm0ExtforceTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0ExtforceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0ExtforceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0ExtforceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0ExtforceTrigin {
        FlexPwm0ExtforceTrigin::from_bits(val)
    }
}
impl From<FlexPwm0ExtforceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0ExtforceTrigin) -> u8 {
        FlexPwm0ExtforceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0FaultTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0FaultTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0FaultTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0FaultTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0FaultTrigin {
        FlexPwm0FaultTrigin::from_bits(val)
    }
}
impl From<FlexPwm0FaultTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0FaultTrigin) -> u8 {
        FlexPwm0FaultTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0SmExtaTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0SmExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0SmExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0SmExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0SmExtaTrigin {
        FlexPwm0SmExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm0SmExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0SmExtaTrigin) -> u8 {
        FlexPwm0SmExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm0SmExtsyncTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT2 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm0SmExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm0SmExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm0SmExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm0SmExtsyncTrigin {
        FlexPwm0SmExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm0SmExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm0SmExtsyncTrigin) -> u8 {
        FlexPwm0SmExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1ExtforceTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1ExtforceTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1ExtforceTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1ExtforceTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1ExtforceTrigin {
        FlexPwm1ExtforceTrigin::from_bits(val)
    }
}
impl From<FlexPwm1ExtforceTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1ExtforceTrigin) -> u8 {
        FlexPwm1ExtforceTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1FaultTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1FaultTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1FaultTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1FaultTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1FaultTrigin {
        FlexPwm1FaultTrigin::from_bits(val)
    }
}
impl From<FlexPwm1FaultTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1FaultTrigin) -> u8 {
        FlexPwm1FaultTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1SmExtaTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1SmExtaTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1SmExtaTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1SmExtaTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1SmExtaTrigin {
        FlexPwm1SmExtaTrigin::from_bits(val)
    }
}
impl From<FlexPwm1SmExtaTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1SmExtaTrigin) -> u8 {
        FlexPwm1SmExtaTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexPwm1SmExtsyncTrigin {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT2 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
    #[doc = "SINC Filter CH0 sync Break input is selected."]
    Val52 = 0x34,
    #[doc = "SINC Filter CH1 sync Break input is selected."]
    Val53 = 0x35,
    #[doc = "SINC Filter CH2 sync Break input is selected."]
    Val54 = 0x36,
    #[doc = "SINC Filter CH3 sync Break input is selected."]
    Val55 = 0x37,
    #[doc = "SINC Filter CH4 sync Break input is selected."]
    Val56 = 0x38,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val57 = 0x39,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val58 = 0x3a,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val60 = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl FlexPwm1SmExtsyncTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexPwm1SmExtsyncTrigin {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexPwm1SmExtsyncTrigin {
    #[inline(always)]
    fn from(val: u8) -> FlexPwm1SmExtsyncTrigin {
        FlexPwm1SmExtsyncTrigin::from_bits(val)
    }
}
impl From<FlexPwm1SmExtsyncTrigin> for u8 {
    #[inline(always)]
    fn from(val: FlexPwm1SmExtsyncTrigin) -> u8 {
        FlexPwm1SmExtsyncTrigin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm0TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT6 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT7 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm0TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm0TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm0TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm0TrigInp {
        Flexcomm0TrigInp::from_bits(val)
    }
}
impl From<Flexcomm0TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm0TrigInp) -> u8 {
        Flexcomm0TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm1TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT6 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT7 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm1TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm1TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm1TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm1TrigInp {
        Flexcomm1TrigInp::from_bits(val)
    }
}
impl From<Flexcomm1TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm1TrigInp) -> u8 {
        Flexcomm1TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm2TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT6 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT8 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT9 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm2TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm2TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm2TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm2TrigInp {
        Flexcomm2TrigInp::from_bits(val)
    }
}
impl From<Flexcomm2TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm2TrigInp) -> u8 {
        Flexcomm2TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm3TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT8 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT9 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT1 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm3TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm3TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm3TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm3TrigInp {
        Flexcomm3TrigInp::from_bits(val)
    }
}
impl From<Flexcomm3TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm3TrigInp) -> u8 {
        Flexcomm3TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm4TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT2 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm4TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm4TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm4TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm4TrigInp {
        Flexcomm4TrigInp::from_bits(val)
    }
}
impl From<Flexcomm4TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm4TrigInp) -> u8 {
        Flexcomm4TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm5TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT2 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT2 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT2 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm5TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm5TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm5TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm5TrigInp {
        Flexcomm5TrigInp::from_bits(val)
    }
}
impl From<Flexcomm5TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm5TrigInp) -> u8 {
        Flexcomm5TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm6TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT4 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT3 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm6TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm6TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm6TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm6TrigInp {
        Flexcomm6TrigInp::from_bits(val)
    }
}
impl From<Flexcomm6TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm6TrigInp) -> u8 {
        Flexcomm6TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcomm9TrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT0 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT3 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT4 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER0_MAT1 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER1_MAT1 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val10 = 0x0a,
    #[doc = "LPTMR0 input is selected."]
    Val11 = 0x0b,
    #[doc = "LPTMR1 input is selected."]
    Val12 = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "CMP0_OUT input is selected."]
    Val15 = 0x0f,
    #[doc = "CMP1_OUT input is selected."]
    Val16 = 0x10,
    #[doc = "CMP2_OUT input is selected."]
    Val17 = 0x11,
    #[doc = "EVTG_OUT0A input is selected."]
    Val18 = 0x12,
    #[doc = "EVTG_OUT0B input is selected."]
    Val19 = 0x13,
    #[doc = "EVTG_OUT1A input is selected."]
    Val20 = 0x14,
    #[doc = "EVTG_OUT1B input is selected."]
    Val21 = 0x15,
    #[doc = "EVTG_OUT2A input is selected."]
    Val22 = 0x16,
    #[doc = "EVTG_OUT2B input is selected."]
    Val23 = 0x17,
    #[doc = "EVTG_OUT3A input is selected."]
    Val24 = 0x18,
    #[doc = "EVTG_OUT3B input is selected."]
    Val25 = 0x19,
    #[doc = "TRIG_IN0 input is selected."]
    Val26 = 0x1a,
    #[doc = "TRIG_IN1 input is selected."]
    Val27 = 0x1b,
    #[doc = "TRIG_IN2 input is selected."]
    Val28 = 0x1c,
    #[doc = "TRIG_IN3 input is selected."]
    Val29 = 0x1d,
    #[doc = "TRIG_IN4 input is selected."]
    Val30 = 0x1e,
    #[doc = "TRIG_IN10 input is selected."]
    Val31 = 0x1f,
    #[doc = "TRIG_IN11 input is selected."]
    Val32 = 0x20,
    #[doc = "FlexIO CH4 input is selected."]
    Val33 = 0x21,
    #[doc = "FlexIO CH5 input is selected."]
    Val34 = 0x22,
    #[doc = "FlexIO CH6 input is selected."]
    Val35 = 0x23,
    #[doc = "FlexIO CH7 input is selected."]
    Val36 = 0x24,
    #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO2 Pin Event Trig 0 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO2 Pin Event Trig 1 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO3 Pin Event Trig 0 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO3 Pin Event Trig 1 input is selected."]
    Val41 = 0x29,
    #[doc = "WUU input is selected."]
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
impl Flexcomm9TrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcomm9TrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcomm9TrigInp {
    #[inline(always)]
    fn from(val: u8) -> Flexcomm9TrigInp {
        Flexcomm9TrigInp::from_bits(val)
    }
}
impl From<Flexcomm9TrigInp> for u8 {
    #[inline(always)]
    fn from(val: Flexcomm9TrigInp) -> u8 {
        Flexcomm9TrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioTrigInp {
    #[doc = "PINT PIN_INT4 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT5 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT6 input is selected."]
    Val2 = 0x02,
    #[doc = "PINT PIN_INT7 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT5 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT6 input is selected."]
    Val5 = 0x05,
    #[doc = "SCT_OUT7 input is selected."]
    Val6 = 0x06,
    #[doc = "SCT_OUT8 input is selected."]
    Val7 = 0x07,
    #[doc = "SCT_OUT9 input is selected."]
    Val8 = 0x08,
    #[doc = "T0_MAT1 input is selected."]
    Val9 = 0x09,
    #[doc = "T1_MAT1 input is selected."]
    Val10 = 0x0a,
    #[doc = "T2_MAT1 input is selected."]
    Val11 = 0x0b,
    #[doc = "T3_MAT1 input is selected."]
    Val12 = 0x0c,
    #[doc = "T4_MAT1 input is selected."]
    Val13 = 0x0d,
    #[doc = "LPTMR0 input is selected."]
    Val14 = 0x0e,
    #[doc = "LPTMR1 input is selected."]
    Val15 = 0x0f,
    _RESERVED_10 = 0x10,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val17 = 0x11,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val20 = 0x14,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val21 = 0x15,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val22 = 0x16,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val23 = 0x17,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val24 = 0x18,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val25 = 0x19,
    #[doc = "CMP0_OUT input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP1_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP2_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val37 = 0x25,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val38 = 0x26,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val39 = 0x27,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val40 = 0x28,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val41 = 0x29,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val42 = 0x2a,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val43 = 0x2b,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT0A input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT0B input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT1A input is selected."]
    Val47 = 0x2f,
    #[doc = "EVTG_OUT1B input is selected."]
    Val48 = 0x30,
    #[doc = "EVTG_OUT2A input is selected."]
    Val49 = 0x31,
    #[doc = "EVTG_OUT2B input is selected."]
    Val50 = 0x32,
    #[doc = "EVTG_OUT3A input is selected."]
    Val51 = 0x33,
    #[doc = "EVTG_OUT3B input is selected."]
    Val52 = 0x34,
    #[doc = "TRIG_IN0 input is selected."]
    Val53 = 0x35,
    #[doc = "TRIG_IN1 input is selected."]
    Val54 = 0x36,
    #[doc = "TRIG_IN2 input is selected."]
    Val55 = 0x37,
    #[doc = "TRIG_IN3 input is selected."]
    Val56 = 0x38,
    #[doc = "TRIG_IN4 input is selected."]
    Val57 = 0x39,
    #[doc = "SINC Filter CH0 Conversion Complete input is selected."]
    Val58 = 0x3a,
    #[doc = "SINC Filter CH1 Conversion Complete input is selected."]
    Val59 = 0x3b,
    #[doc = "SINC Filter CH2 Conversion Complete input is selected."]
    Val60 = 0x3c,
    #[doc = "SINC Filter CH3 Conversion Complete input is selected."]
    Val61 = 0x3d,
    #[doc = "SINC Filter CH4 Conversion Complete input is selected."]
    Val62 = 0x3e,
    #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected."]
    Val63 = 0x3f,
    #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected."]
    Val64 = 0x40,
    #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected."]
    Val65 = 0x41,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val66 = 0x42,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val67 = 0x43,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val68 = 0x44,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val69 = 0x45,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val70 = 0x46,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val71 = 0x47,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val72 = 0x48,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val73 = 0x49,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val74 = 0x4a,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val75 = 0x4b,
    #[doc = "WUU input is selected."]
    Val76 = 0x4c,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasRefInp {
    #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected."]
    Val0 = 0x0,
    #[doc = "FRO_12M input is selected."]
    Val1 = 0x01,
    #[doc = "FRO_144M input is selected."]
    Val2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "OSC_32K input is selected."]
    Val4 = 0x04,
    #[doc = "CPU/system_clk input is selected."]
    Val5 = 0x05,
    #[doc = "FREQME_CLK_IN0 input is selected."]
    Val6 = 0x06,
    #[doc = "FREQME_CLK_IN1 input is selected."]
    Val7 = 0x07,
    #[doc = "EVTG_OUT0A input is selected."]
    Val8 = 0x08,
    #[doc = "EVTG_OUT1A input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
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
impl FreqmeasRefInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasRefInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasTarInp {
    #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected."]
    Val0 = 0x0,
    #[doc = "FRO_12M input is selected."]
    Val1 = 0x01,
    #[doc = "FRO_144M input is selected."]
    Val2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "OSC_32K input is selected."]
    Val4 = 0x04,
    #[doc = "CPU/system_clk input is selected."]
    Val5 = 0x05,
    #[doc = "FREQME_CLK_IN0 input is selected."]
    Val6 = 0x06,
    #[doc = "FREQME_CLK_IN1 input is selected."]
    Val7 = 0x07,
    #[doc = "EVTG_OUT0A input is selected."]
    Val8 = 0x08,
    #[doc = "EVTG_OUT1A input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
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
impl FreqmeasTarInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasTarInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
pub enum OpampTrigInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected."]
    Val1 = 0x01,
    #[doc = "PINT PIN_INT2 input is selected."]
    Val2 = 0x02,
    #[doc = "PINT PIN_INT3 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT4 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT_OUT5 input is selected."]
    Val5 = 0x05,
    #[doc = "SCT_OUT6 input is selected."]
    Val6 = 0x06,
    #[doc = "SCT_OUT7 input is selected."]
    Val7 = 0x07,
    #[doc = "SCT_OUT8 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTIMER3_MAT3 input is selected."]
    Val12 = 0x0c,
    #[doc = "CTIMER4_MAT3 input is selected."]
    Val13 = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val20 = 0x14,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val21 = 0x15,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val22 = 0x16,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val37 = 0x25,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT0A input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0B input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT1A input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1B input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT2A input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2B input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT3A input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3B input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN0 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN1 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN2 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN3 input is selected."]
    Val50 = 0x32,
    #[doc = "FlexIO CH4 input is selected."]
    Val51 = 0x33,
    #[doc = "FlexIO CH5 input is selected."]
    Val52 = 0x34,
    #[doc = "FlexIO CH6 input is selected."]
    Val53 = 0x35,
    #[doc = "FlexIO CH7 input is selected."]
    Val54 = 0x36,
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
pub enum PintselInp {
    #[doc = "GPIO P0_0 input is selected."]
    Val0 = 0x0,
    #[doc = "GPIO P0_1 input is selected."]
    Val1 = 0x01,
    #[doc = "GPIO P0_2 input is selected."]
    Val2 = 0x02,
    #[doc = "GPIO P0_3 input is selected."]
    Val3 = 0x03,
    #[doc = "GPIO P0_4 input is selected."]
    Val4 = 0x04,
    #[doc = "GPIO P0_5 input is selected."]
    Val5 = 0x05,
    #[doc = "GPIO P0_6 input is selected."]
    Val6 = 0x06,
    #[doc = "GPIO P0_7 input is selected."]
    Val7 = 0x07,
    #[doc = "GPIO P0_8 input is selected."]
    Val8 = 0x08,
    #[doc = "GPIO P0_9 input is selected."]
    Val9 = 0x09,
    #[doc = "GPIO P0_10 input is selected."]
    Val10 = 0x0a,
    #[doc = "GPIO P0_11 input is selected."]
    Val11 = 0x0b,
    #[doc = "GPIO P0_12 input is selected."]
    Val12 = 0x0c,
    #[doc = "GPIO P0_13 input is selected."]
    Val13 = 0x0d,
    #[doc = "GPIO P0_14 input is selected."]
    Val14 = 0x0e,
    #[doc = "GPIO P0_15 input is selected."]
    Val15 = 0x0f,
    #[doc = "GPIO P0_16 input is selected."]
    Val16 = 0x10,
    #[doc = "GPIO P0_17 input is selected."]
    Val17 = 0x11,
    #[doc = "GPIO P0_18 input is selected."]
    Val18 = 0x12,
    #[doc = "GPIO P0_19 input is selected."]
    Val19 = 0x13,
    #[doc = "GPIO P0_20 input is selected."]
    Val20 = 0x14,
    #[doc = "GPIO P0_21 input is selected."]
    Val21 = 0x15,
    #[doc = "GPIO P0_22 input is selected."]
    Val22 = 0x16,
    #[doc = "GPIO P0_23 input is selected."]
    Val23 = 0x17,
    #[doc = "GPIO P0_24 input is selected."]
    Val24 = 0x18,
    #[doc = "GPIO P0_25 input is selected."]
    Val25 = 0x19,
    #[doc = "GPIO P0_26 input is selected."]
    Val26 = 0x1a,
    #[doc = "GPIO P0_27 input is selected."]
    Val27 = 0x1b,
    #[doc = "GPIO P0_28 input is selected."]
    Val28 = 0x1c,
    #[doc = "GPIO P0_29 input is selected."]
    Val29 = 0x1d,
    #[doc = "GPIO P0_30 input is selected."]
    Val30 = 0x1e,
    #[doc = "GPIO P0_31 input is selected."]
    Val31 = 0x1f,
    #[doc = "GPIO P1_0 input is selected."]
    Val32 = 0x20,
    #[doc = "GPIO P1_1 input is selected."]
    Val33 = 0x21,
    #[doc = "GPIO P1_2 input is selected."]
    Val34 = 0x22,
    #[doc = "GPIO P1_3 input is selected."]
    Val35 = 0x23,
    #[doc = "GPIO P1_4 input is selected."]
    Val36 = 0x24,
    #[doc = "GPIO P1_5 input is selected."]
    Val37 = 0x25,
    #[doc = "GPIO P1_6 input is selected."]
    Val38 = 0x26,
    #[doc = "GPIO P1_7 input is selected."]
    Val39 = 0x27,
    #[doc = "GPIO P1_8 input is selected."]
    Val40 = 0x28,
    #[doc = "GPIO P1_9 input is selected."]
    Val41 = 0x29,
    #[doc = "GPIO P1_10 input is selected."]
    Val42 = 0x2a,
    #[doc = "GPIO P1_11 input is selected."]
    Val43 = 0x2b,
    #[doc = "GPIO P1_12 input is selected."]
    Val44 = 0x2c,
    #[doc = "GPIO P1_13 input is selected."]
    Val45 = 0x2d,
    #[doc = "GPIO P1_14 input is selected."]
    Val46 = 0x2e,
    #[doc = "GPIO P1_15 input is selected."]
    Val47 = 0x2f,
    #[doc = "GPIO P1_16 input is selected."]
    Val48 = 0x30,
    #[doc = "GPIO P1_17 input is selected."]
    Val49 = 0x31,
    #[doc = "GPIO P1_18 input is selected."]
    Val50 = 0x32,
    #[doc = "GPIO P1_19 input is selected."]
    Val51 = 0x33,
    #[doc = "GPIO P1_20 input is selected."]
    Val52 = 0x34,
    #[doc = "GPIO P1_21 input is selected."]
    Val53 = 0x35,
    #[doc = "GPIO P1_22 input is selected."]
    Val54 = 0x36,
    #[doc = "GPIO P1_23 input is selected."]
    Val55 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    #[doc = "GPIO P1_30 input is selected."]
    Val62 = 0x3e,
    #[doc = "GPIO P1_31 input is selected."]
    Val63 = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl PintselInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintselInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintselInp {
    #[inline(always)]
    fn from(val: u8) -> PintselInp {
        PintselInp::from_bits(val)
    }
}
impl From<PintselInp> for u8 {
    #[inline(always)]
    fn from(val: PintselInp) -> u8 {
        PintselInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0ExtClkTrigin {
    #[doc = "FRO16K input is selected."]
    Val0 = 0x0,
    #[doc = "OSC_32k input is selected."]
    Val1 = 0x01,
    #[doc = "EVTG_OUT0A input is selected."]
    Val2 = 0x02,
    #[doc = "EVTG_OUT1A input is selected."]
    Val3 = 0x03,
    #[doc = "TRIG_IN0 input is selected."]
    Val4 = 0x04,
    #[doc = "TRIG_IN7 input is selected."]
    Val5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pwm0ExtClkTrigin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0ExtClkTrigin {
        unsafe { core::mem::transmute(val & 0x07) }
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
    #[doc = "FRO16K input is selected."]
    Val0 = 0x0,
    #[doc = "OSC_32k input is selected."]
    Val1 = 0x01,
    #[doc = "EVTG_OUT0A input is selected."]
    Val2 = 0x02,
    #[doc = "EVTG_OUT1A input is selected."]
    Val3 = 0x03,
    #[doc = "TRIG_IN0 input is selected."]
    Val4 = 0x04,
    #[doc = "TRIG_IN7 input is selected."]
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
pub enum QdcHomeInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0 SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0 SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0 SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
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
impl QdcHomeInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcHomeInp {
        unsafe { core::mem::transmute(val & 0x3f) }
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
pub enum QdcIndexInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
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
impl QdcIndexInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcIndexInp {
        unsafe { core::mem::transmute(val & 0x3f) }
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
pub enum QdcPhaseaInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
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
impl QdcPhaseaInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcPhaseaInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcPhaseaInp {
    #[inline(always)]
    fn from(val: u8) -> QdcPhaseaInp {
        QdcPhaseaInp::from_bits(val)
    }
}
impl From<QdcPhaseaInp> for u8 {
    #[inline(always)]
    fn from(val: QdcPhaseaInp) -> u8 {
        QdcPhaseaInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcPhasebInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
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
impl QdcPhasebInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcPhasebInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for QdcPhasebInp {
    #[inline(always)]
    fn from(val: u8) -> QdcPhasebInp {
        QdcPhasebInp::from_bits(val)
    }
}
impl From<QdcPhasebInp> for u8 {
    #[inline(always)]
    fn from(val: QdcPhasebInp) -> u8 {
        QdcPhasebInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QdcTrigInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT4 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT1 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "ARM_TXEV input is selected."]
    Val11 = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val32 = 0x20,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val33 = 0x21,
    #[doc = "EVTG_OUT0A input is selected."]
    Val34 = 0x22,
    #[doc = "EVTG_OUT0B input is selected."]
    Val35 = 0x23,
    #[doc = "EVTG_OUT1A input is selected."]
    Val36 = 0x24,
    #[doc = "EVTG_OUT1B input is selected."]
    Val37 = 0x25,
    #[doc = "EVTG_OUT2A input is selected."]
    Val38 = 0x26,
    #[doc = "EVTG_OUT2B input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT3A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT3B input is selected."]
    Val41 = 0x29,
    #[doc = "TRIG_IN0 input is selected."]
    Val42 = 0x2a,
    #[doc = "TRIG_IN1 input is selected."]
    Val43 = 0x2b,
    #[doc = "TRIG_IN2 input is selected."]
    Val44 = 0x2c,
    #[doc = "TRIG_IN3 input is selected."]
    Val45 = 0x2d,
    #[doc = "TRIG_IN4 input is selected."]
    Val46 = 0x2e,
    #[doc = "TRIG_IN5 input is selected."]
    Val47 = 0x2f,
    #[doc = "TRIG_IN6 input is selected."]
    Val48 = 0x30,
    #[doc = "TRIG_IN7 input is selected."]
    Val49 = 0x31,
    #[doc = "TRIG_IN8 input is selected."]
    Val50 = 0x32,
    #[doc = "TRIG_IN9 input is selected."]
    Val51 = 0x33,
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
impl QdcTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> QdcTrigInp {
        unsafe { core::mem::transmute(val & 0x3f) }
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
pub enum ReqEn {
    #[doc = "Disable."]
    Disabled = 0x0,
    #[doc = "Enable."]
    Enabled = 0x01,
}
impl ReqEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReqEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReqEn {
    #[inline(always)]
    fn from(val: u8) -> ReqEn {
        ReqEn::from_bits(val)
    }
}
impl From<ReqEn> for u8 {
    #[inline(always)]
    fn from(val: ReqEn) -> u8 {
        ReqEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sct0InmuxInp {
    #[doc = "SCT0_IN0 input is selected."]
    Val0 = 0x0,
    #[doc = "SCT0_IN1 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT0_IN2 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT0_IN3 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT0_IN4 input is selected."]
    Val4 = 0x04,
    #[doc = "SCT0_IN5 input is selected."]
    Val5 = 0x05,
    #[doc = "SCT0_IN6 input is selected."]
    Val6 = 0x06,
    #[doc = "SCT0_IN7 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER0_MAT0 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER1_MAT0 input is selected."]
    Val9 = 0x09,
    #[doc = "CTIMER2_MAT0 input is selected."]
    Val10 = 0x0a,
    #[doc = "CTIMER3_MAT0 input is selected."]
    Val11 = 0x0b,
    #[doc = "CTIMER4_MAT0 input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val13 = 0x0d,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val14 = 0x0e,
    #[doc = "usb0 start of frame input is selected."]
    Val15 = 0x0f,
    #[doc = "usb1 start of frame input is selected."]
    Val16 = 0x10,
    #[doc = "SINC Filter CH0 Conversion Complete input is selected."]
    Val17 = 0x11,
    #[doc = "SINC Filter CH1 Conversion Complete input is selected."]
    Val18 = 0x12,
    #[doc = "SINC Filter CH2 Conversion Complete input is selected."]
    Val19 = 0x13,
    #[doc = "SINC Filter CH3 Conversion Complete input is selected."]
    Val20 = 0x14,
    #[doc = "SINC Filter CH4 Conversion Complete input is selected."]
    Val21 = 0x15,
    _RESERVED_16 = 0x16,
    #[doc = "DEBUG_HALTED input is selected."]
    Val23 = 0x17,
    #[doc = "ADC1_IRQ input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val26 = 0x1a,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val27 = 0x1b,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val28 = 0x1c,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val29 = 0x1d,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val30 = 0x1e,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val31 = 0x1f,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val32 = 0x20,
    #[doc = "CMP0_OUT input is selected."]
    Val33 = 0x21,
    #[doc = "CMP1_OUT input is selected."]
    Val34 = 0x22,
    #[doc = "CMP2_OUT input is selected."]
    Val35 = 0x23,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val38 = 0x26,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val39 = 0x27,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val40 = 0x28,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val41 = 0x29,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val42 = 0x2a,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val43 = 0x2b,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val44 = 0x2c,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT0A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT0B input is selected."]
    Val47 = 0x2f,
    #[doc = "EVTG_OUT1A input is selected."]
    Val48 = 0x30,
    #[doc = "EVTG_OUT1B input is selected."]
    Val49 = 0x31,
    #[doc = "EVTG_OUT2A input is selected."]
    Val50 = 0x32,
    #[doc = "EVTG_OUT2B input is selected."]
    Val51 = 0x33,
    #[doc = "EVTG_OUT3A input is selected."]
    Val52 = 0x34,
    #[doc = "EVTG_OUT3B input is selected."]
    Val53 = 0x35,
    #[doc = "FC3_P0 (SDO, SDA) input is selected."]
    Val54 = 0x36,
    #[doc = "FC3_P1 (SCK, TXD, SCL) input is selected."]
    Val55 = 0x37,
    #[doc = "FC3_P2 (RTS, SCLS, TXD) input is selected."]
    Val56 = 0x38,
    #[doc = "FC3_P3 (PCS\\[0\\], CTS, SDAS) input is selected."]
    Val57 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected."]
    Val62 = 0x3e,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val63 = 0x3f,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val64 = 0x40,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val65 = 0x41,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val66 = 0x42,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val67 = 0x43,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val68 = 0x44,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val69 = 0x45,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val70 = 0x46,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val71 = 0x47,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val72 = 0x48,
    #[doc = "SAI0 TX BCLK input is selected."]
    Val73 = 0x49,
    #[doc = "SAI0 RX BCLK input is selected."]
    Val74 = 0x4a,
    #[doc = "SAI1 TX BCLK input is selected."]
    Val75 = 0x4b,
    #[doc = "SAI1 RX BCLK input is selected."]
    Val76 = 0x4c,
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
impl Sct0InmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sct0InmuxInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sct0InmuxInp {
    #[inline(always)]
    fn from(val: u8) -> Sct0InmuxInp {
        Sct0InmuxInp::from_bits(val)
    }
}
impl From<Sct0InmuxInp> for u8 {
    #[inline(always)]
    fn from(val: Sct0InmuxInp) -> u8 {
        Sct0InmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincFilterChInp {
    #[doc = "PINT PIN_INT0 input is selected."]
    Val0 = 0x0,
    #[doc = "PINT PIN_INT1 input is selected."]
    Val1 = 0x01,
    #[doc = "SCT_OUT4 input is selected."]
    Val2 = 0x02,
    #[doc = "SCT_OUT5 input is selected."]
    Val3 = 0x03,
    #[doc = "SCT_OUT9 input is selected."]
    Val4 = 0x04,
    #[doc = "CTIMER0_MAT3 input is selected."]
    Val5 = 0x05,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val6 = 0x06,
    #[doc = "CTIMER2_MAT3 input is selected."]
    Val7 = 0x07,
    #[doc = "CTIMER3_MAT3 input is selected."]
    Val8 = 0x08,
    #[doc = "CTIMER4_MAT3 input is selected."]
    Val9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "PINT GPIO_INT_BMAT input is selected."]
    Val12 = 0x0c,
    #[doc = "ADC0_tcomp\\[0\\] input is selected."]
    Val13 = 0x0d,
    #[doc = "ADC0_tcomp\\[1\\] input is selected."]
    Val14 = 0x0e,
    #[doc = "ADC0_tcomp\\[2\\] input is selected."]
    Val15 = 0x0f,
    #[doc = "ADC0_tcomp\\[3\\] input is selected."]
    Val16 = 0x10,
    #[doc = "ADC1_tcomp\\[0\\] input is selected."]
    Val17 = 0x11,
    #[doc = "ADC1_tcomp\\[1\\] input is selected."]
    Val18 = 0x12,
    #[doc = "ADC1_tcomp\\[2\\] input is selected."]
    Val19 = 0x13,
    #[doc = "ADC1_tcomp\\[3\\] input is selected."]
    Val20 = 0x14,
    #[doc = "CMP0_OUT input is selected."]
    Val21 = 0x15,
    #[doc = "CMP1_OUT input is selected."]
    Val22 = 0x16,
    #[doc = "CMP2_OUT input is selected."]
    Val23 = 0x17,
    #[doc = "PWM0_SM0_MUX_TRIG0 input is selected."]
    Val24 = 0x18,
    #[doc = "PWM0_SM0_MUX_TRIG1 input is selected."]
    Val25 = 0x19,
    #[doc = "PWM0_SM1_MUX_TRIG0 input is selected."]
    Val26 = 0x1a,
    #[doc = "PWM0_SM1_MUX_TRIG1 input is selected."]
    Val27 = 0x1b,
    #[doc = "PWM0_SM2_MUX_TRIG0 input is selected."]
    Val28 = 0x1c,
    #[doc = "PWM0_SM2_MUX_TRIG1 input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM3_MUX_TRIG0 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM3_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM1_SM0_MUX_TRIG0 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM1_SM0_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM1_MUX_TRIG0 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM2_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "PWM1_SM3_MUX_TRIG0 input is selected."]
    Val38 = 0x26,
    #[doc = "PWM1_SM3_MUX_TRIG1 input is selected."]
    Val39 = 0x27,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val40 = 0x28,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT0A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT0B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT1A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT1B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT2A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT2B input is selected."]
    Val47 = 0x2f,
    #[doc = "EVTG_OUT3A input is selected."]
    Val48 = 0x30,
    #[doc = "EVTG_OUT3B input is selected."]
    Val49 = 0x31,
    #[doc = "LPTMR0 input is selected."]
    Val50 = 0x32,
    #[doc = "LPTMR1 input is selected."]
    Val51 = 0x33,
    #[doc = "FlexIO CH0 input is selected."]
    Val52 = 0x34,
    #[doc = "FlexIO CH1 input is selected."]
    Val53 = 0x35,
    #[doc = "FlexIO CH2 input is selected."]
    Val54 = 0x36,
    #[doc = "FlexIO CH3 input is selected."]
    Val55 = 0x37,
    #[doc = "WUU input is selected."]
    Val56 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl SincFilterChInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincFilterChInp {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincFilterChInp {
    #[inline(always)]
    fn from(val: u8) -> SincFilterChInp {
        SincFilterChInp::from_bits(val)
    }
}
impl From<SincFilterChInp> for u8 {
    #[inline(always)]
    fn from(val: SincFilterChInp) -> u8 {
        SincFilterChInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartdmaarchbInmuxInp {
    #[doc = "FlexIO interrupt is selected as input."]
    Val0 = 0x0,
    #[doc = "GPIO P0_1 input is selected."]
    Val1 = 0x01,
    #[doc = "GPIO P0_2 input is selected."]
    Val2 = 0x02,
    #[doc = "GPIO P0_3 input is selected."]
    Val3 = 0x03,
    #[doc = "GPIO P0_4 input is selected."]
    Val4 = 0x04,
    #[doc = "GPIO P0_5 input is selected."]
    Val5 = 0x05,
    #[doc = "GPIO P0_6 input is selected."]
    Val6 = 0x06,
    #[doc = "GPIO P0_7 input is selected."]
    Val7 = 0x07,
    #[doc = "GPIO P0_8 input is selected."]
    Val8 = 0x08,
    #[doc = "GPIO P0_9 input is selected."]
    Val9 = 0x09,
    #[doc = "GPIO P0_10 input is selected."]
    Val10 = 0x0a,
    #[doc = "GPIO P0_11 input is selected."]
    Val11 = 0x0b,
    #[doc = "GPIO P0_12 input is selected."]
    Val12 = 0x0c,
    #[doc = "GPIO P0_13 input is selected."]
    Val13 = 0x0d,
    #[doc = "GPIO P0_14 input is selected."]
    Val14 = 0x0e,
    #[doc = "GPIO P0_15 input is selected."]
    Val15 = 0x0f,
    #[doc = "SCT0 SCT_OUT8 input is selected."]
    Val16 = 0x10,
    #[doc = "SCT0 SCT_OUT9 input is selected."]
    Val17 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "MRT0 MRT_CH0_IRQ input is selected."]
    Val20 = 0x14,
    #[doc = "MRT0 MRT_CH1_IRQ input is selected."]
    Val21 = 0x15,
    #[doc = "CTIMER4_MAT3 input is selected."]
    Val22 = 0x16,
    #[doc = "CTIMER4_MAT2 input is selected."]
    Val23 = 0x17,
    #[doc = "CTIMER3_MAT3 input is selected."]
    Val24 = 0x18,
    #[doc = "CTIMER3_MAT2 input is selected."]
    Val25 = 0x19,
    #[doc = "CTIMER1_MAT3 input is selected."]
    Val26 = 0x1a,
    #[doc = "CTIMER1_MAT2 input is selected."]
    Val27 = 0x1b,
    #[doc = "UTICK0 UTICK_IRQ input is selected."]
    Val28 = 0x1c,
    #[doc = "WWDT0 WDT0_IRQ input is selected."]
    Val29 = 0x1d,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val30 = 0x1e,
    #[doc = "CMP0_IRQ input is selected."]
    Val31 = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "LP_FLEXCOMM7_IRQ input is selected."]
    Val33 = 0x21,
    #[doc = "LP_FLEXCOMM6_IRQ input is selected."]
    Val34 = 0x22,
    #[doc = "LP_FLEXCOMM5_IRQ input is selected."]
    Val35 = 0x23,
    #[doc = "LP_FLEXCOMM4_IRQ input is selected."]
    Val36 = 0x24,
    #[doc = "LP_FLEXCOMM3_IRQ input is selected."]
    Val37 = 0x25,
    #[doc = "LP_FLEXCOMM2_IRQ input is selected."]
    Val38 = 0x26,
    #[doc = "LP_FLEXCOMM1_IRQ input is selected."]
    Val39 = 0x27,
    #[doc = "LP_FLEXCOMM0_IRQ input is selected."]
    Val40 = 0x28,
    #[doc = "DMA0_IRQ input is selected."]
    Val41 = 0x29,
    #[doc = "DMA1_IRQ input is selected."]
    Val42 = 0x2a,
    #[doc = "SYS_IRQSYS_IRQ combines the CDOG IRQ, WWDT IRQ, MBC secure violation IRQ, Secure AHB Matrix secure violation IRQ, GDET IRQ, ELS S50 error IRQ, PKC error IRQ, and VBAT IRQ using the logical OR operation. input is selected."]
    Val43 = 0x2b,
    #[doc = "RTC_COMBO_IRQ input is selected."]
    Val44 = 0x2c,
    #[doc = "ARM_TXEV input is selected."]
    Val45 = 0x2d,
    #[doc = "PINT0 GPIO_INT_BMATCH input is selected."]
    Val46 = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    #[doc = "CMP0_OUT input is selected."]
    Val49 = 0x31,
    #[doc = "usb0 start of frame input is selected."]
    Val50 = 0x32,
    #[doc = "usb1 start of frame input is selected."]
    Val51 = 0x33,
    #[doc = "OSTIMER0 OS_EVENT_TIMER_IRQ input is selected."]
    Val52 = 0x34,
    #[doc = "ADC1_IRQ input is selected."]
    Val53 = 0x35,
    #[doc = "CMP0_IRQ/CMP1_IRQ/CMP2_IRQ input is selected."]
    Val54 = 0x36,
    #[doc = "DAC0_IRQ input is selected."]
    Val55 = 0x37,
    #[doc = "DAC1_IRQ/DAC2_IRQ input is selected."]
    Val56 = 0x38,
    #[doc = "PWM0_IRQ input is selected."]
    Val57 = 0x39,
    #[doc = "PWM1_IRQ input is selected."]
    Val58 = 0x3a,
    #[doc = "QDC0_IRQ input is selected."]
    Val59 = 0x3b,
    #[doc = "QDC1_IRQ input is selected."]
    Val60 = 0x3c,
    #[doc = "EVTG_OUT0A input is selected."]
    Val61 = 0x3d,
    #[doc = "EVTG_OUT1A input is selected."]
    Val62 = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    #[doc = "GPIO1_alias0 GPIO1 Pin Event Trig 0 input is selected."]
    Val65 = 0x41,
    #[doc = "GPIO1_alias1 GPIO1 Pin Event Trig 1 input is selected."]
    Val66 = 0x42,
    #[doc = "GPIO2_alias0 GPIO2 Pin Event Trig 0 input is selected."]
    Val67 = 0x43,
    #[doc = "GPIO2_alias1 GPIO2 Pin Event Trig 1 input is selected."]
    Val68 = 0x44,
    #[doc = "GPIO3_alias0 GPIO3 Pin Event Trig 0 input is selected."]
    Val69 = 0x45,
    #[doc = "GPIO3_alias1 GPIO3 Pin Event Trig 1 input is selected."]
    Val70 = 0x46,
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
impl SmartdmaarchbInmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartdmaarchbInmuxInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartdmaarchbInmuxInp {
    #[inline(always)]
    fn from(val: u8) -> SmartdmaarchbInmuxInp {
        SmartdmaarchbInmuxInp::from_bits(val)
    }
}
impl From<SmartdmaarchbInmuxInp> for u8 {
    #[inline(always)]
    fn from(val: SmartdmaarchbInmuxInp) -> u8 {
        SmartdmaarchbInmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0trigInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Timer0trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3trigInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Timer3trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4trigInp {
    #[doc = "CT_INP0 input is selected."]
    Val0 = 0x0,
    #[doc = "CT_INP1 input is selected."]
    Val1 = 0x01,
    #[doc = "CT_INP2 input is selected."]
    Val2 = 0x02,
    #[doc = "CT_INP3 input is selected."]
    Val3 = 0x03,
    #[doc = "CT_INP4 input is selected."]
    Val4 = 0x04,
    #[doc = "CT_INP5 input is selected."]
    Val5 = 0x05,
    #[doc = "CT_INP6 input is selected."]
    Val6 = 0x06,
    #[doc = "CT_INP7 input is selected."]
    Val7 = 0x07,
    #[doc = "CT_INP8 input is selected."]
    Val8 = 0x08,
    #[doc = "CT_INP9 input is selected."]
    Val9 = 0x09,
    #[doc = "CT_INP10 input is selected."]
    Val10 = 0x0a,
    #[doc = "CT_INP11 input is selected."]
    Val11 = 0x0b,
    #[doc = "CT_INP12 input is selected."]
    Val12 = 0x0c,
    #[doc = "CT_INP13 input is selected."]
    Val13 = 0x0d,
    #[doc = "CT_INP14 input is selected."]
    Val14 = 0x0e,
    #[doc = "CT_INP15 input is selected."]
    Val15 = 0x0f,
    #[doc = "CT_INP16 input is selected."]
    Val16 = 0x10,
    #[doc = "CT_INP17 input is selected."]
    Val17 = 0x11,
    #[doc = "CT_INP18 input is selected."]
    Val18 = 0x12,
    #[doc = "CT_INP19 input is selected."]
    Val19 = 0x13,
    #[doc = "usb0 start of frame input is selected."]
    Val20 = 0x14,
    #[doc = "usb1 start of frame input is selected."]
    Val21 = 0x15,
    #[doc = "DCDC_BURST_ACTIVE input is selected."]
    Val22 = 0x16,
    #[doc = "sai0_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val23 = 0x17,
    #[doc = "sai0_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val24 = 0x18,
    #[doc = "ADC0 ADC0_IRQ input is selected."]
    Val25 = 0x19,
    #[doc = "ADC0 ADC1_IRQ input is selected."]
    Val26 = 0x1a,
    #[doc = "CMP0_OUT input is selected."]
    Val27 = 0x1b,
    #[doc = "CMP1_OUT input is selected."]
    Val28 = 0x1c,
    #[doc = "CMP2_OUT input is selected."]
    Val29 = 0x1d,
    #[doc = "PWM0_SM0_MUX_TRIG0/PWM0_SM0_MUX_TRIG1 input is selected."]
    Val30 = 0x1e,
    #[doc = "PWM0_SM1_MUX_TRIG0/PWM0_SM1_MUX_TRIG1 input is selected."]
    Val31 = 0x1f,
    #[doc = "PWM0_SM2_MUX_TRIG0/PWM0_SM2_MUX_TRIG1 input is selected."]
    Val32 = 0x20,
    #[doc = "PWM0_SM3_MUX_TRIG0/PWM0_SM3_MUX_TRIG1 input is selected."]
    Val33 = 0x21,
    #[doc = "PWM1_SM0_MUX_TRIG0/PWM1_SM0_MUX_TRIG1 input is selected."]
    Val34 = 0x22,
    #[doc = "PWM1_SM1_MUX_TRIG0/PWM1_SM1_MUX_TRIG1 input is selected."]
    Val35 = 0x23,
    #[doc = "PWM1_SM2_MUX_TRIG0/PWM1_SM2_MUX_TRIG1 input is selected."]
    Val36 = 0x24,
    #[doc = "PWM1_SM3_MUX_TRIG0/PWM1_SM3_MUX_TRIG1 input is selected."]
    Val37 = 0x25,
    #[doc = "QDC0_CMP/POS_MATCH input is selected."]
    Val38 = 0x26,
    #[doc = "QDC1_CMP/POS_MATCH input is selected."]
    Val39 = 0x27,
    #[doc = "EVTG_OUT0A input is selected."]
    Val40 = 0x28,
    #[doc = "EVTG_OUT0B input is selected."]
    Val41 = 0x29,
    #[doc = "EVTG_OUT1A input is selected."]
    Val42 = 0x2a,
    #[doc = "EVTG_OUT1B input is selected."]
    Val43 = 0x2b,
    #[doc = "EVTG_OUT2A input is selected."]
    Val44 = 0x2c,
    #[doc = "EVTG_OUT2B input is selected."]
    Val45 = 0x2d,
    #[doc = "EVTG_OUT3A input is selected."]
    Val46 = 0x2e,
    #[doc = "EVTG_OUT3B input is selected."]
    Val47 = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    #[doc = "LP_FLEXCOMM0 trig 0 input is selected."]
    Val50 = 0x32,
    #[doc = "LP_FLEXCOMM0 trig 1 input is selected."]
    Val51 = 0x33,
    #[doc = "LP_FLEXCOMM0 trig 2 input is selected."]
    Val52 = 0x34,
    #[doc = "LP_FLEXCOMM1 trig 0 input is selected."]
    Val53 = 0x35,
    #[doc = "LP_FLEXCOMM1 trig 1 input is selected."]
    Val54 = 0x36,
    #[doc = "LP_FLEXCOMM1 trig 2 input is selected."]
    Val55 = 0x37,
    #[doc = "LP_FLEXCOMM2 trig 0 input is selected."]
    Val56 = 0x38,
    #[doc = "LP_FLEXCOMM2 trig 1 input is selected."]
    Val57 = 0x39,
    #[doc = "LP_FLEXCOMM2 trig 2 input is selected."]
    Val58 = 0x3a,
    #[doc = "LP_FLEXCOMM3 trig 0 input is selected."]
    Val59 = 0x3b,
    #[doc = "LP_FLEXCOMM3 trig 1 input is selected."]
    Val60 = 0x3c,
    #[doc = "LP_FLEXCOMM3 trig 2 input is selected."]
    Val61 = 0x3d,
    #[doc = "LP_FLEXCOMM3 trig 3 input is selected."]
    Val62 = 0x3e,
    #[doc = "sai1_tx_sync_outsai_tx_sync_out is Transmit Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val63 = 0x3f,
    #[doc = "sai1_rx_sync_outsai_rx_sync_out is Receive Frame Sync for multi-SAI synchronous operation. input is selected."]
    Val64 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
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
impl Timer4trigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4trigInp {
        unsafe { core::mem::transmute(val & 0x7f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
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
pub enum TsiTrigInp {
    #[doc = "LPTMR0 input is selected."]
    Val0 = 0x0,
    #[doc = "LPTMR1 input is selected."]
    Val1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl TsiTrigInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiTrigInp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiTrigInp {
    #[inline(always)]
    fn from(val: u8) -> TsiTrigInp {
        TsiTrigInp::from_bits(val)
    }
}
impl From<TsiTrigInp> for u8 {
    #[inline(always)]
    fn from(val: TsiTrigInp) -> u8 {
        TsiTrigInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbfsTrigInp {
    #[doc = "LP_FLEXCOMM 0 trigger out \\[3\\] input is selected."]
    Val0 = 0x0,
    #[doc = "LP_FLEXCOMM 1 trigger out \\[3\\] input is selected."]
    Val1 = 0x01,
    #[doc = "LP_FLEXCOMM 2 trigger out \\[3\\] input is selected."]
    Val2 = 0x02,
    #[doc = "LP_FLEXCOMM 3 trigger out \\[3\\] input is selected."]
    Val3 = 0x03,
    #[doc = "LP_FLEXCOMM 4 trigger out \\[3\\] input is selected."]
    Val4 = 0x04,
    #[doc = "LP_FLEXCOMM 5 trigger out \\[3\\] input is selected."]
    Val5 = 0x05,
    #[doc = "LP_FLEXCOMM 6 trigger out \\[3\\] input is selected."]
    Val6 = 0x06,
    #[doc = "LP_FLEXCOMM 7 trigger out \\[3\\] input is selected."]
    Val7 = 0x07,
    #[doc = "LP_FLEXCOMM 8 trigger out \\[3\\] input is selected."]
    Val8 = 0x08,
    #[doc = "LP_FLEXCOMM 9 trigger out \\[3\\] input is selected."]
    Val9 = 0x09,
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
