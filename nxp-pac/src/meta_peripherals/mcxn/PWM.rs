#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "PWM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn cnt(self, n: usize) -> crate::pac::common::Reg<Smcnt, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 96usize) as _)
        }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn init(self, n: usize) -> crate::pac::common::Reg<Sminit, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize + n * 96usize) as _)
        }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn ctrl2(self, n: usize) -> crate::pac::common::Reg<Smctrl2, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 96usize) as _)
        }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self, n: usize) -> crate::pac::common::Reg<Smctrl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn val0(self, n: usize) -> crate::pac::common::Reg<Smval0, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Value Register 1."]
    #[inline(always)]
    pub const fn fracval1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfracval1, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn val1(self, n: usize) -> crate::pac::common::Reg<Smval1, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Value Register 2."]
    #[inline(always)]
    pub const fn fracval2(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfracval2, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn val2(self, n: usize) -> crate::pac::common::Reg<Smval2, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Value Register 3."]
    #[inline(always)]
    pub const fn fracval3(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfracval3, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn val3(self, n: usize) -> crate::pac::common::Reg<Smval3, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Value Register 4."]
    #[inline(always)]
    pub const fn fracval4(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfracval4, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn val4(self, n: usize) -> crate::pac::common::Reg<Smval4, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Value Register 5."]
    #[inline(always)]
    pub const fn fracval5(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfracval5, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize + n * 96usize) as _)
        }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn val5(self, n: usize) -> crate::pac::common::Reg<Smval5, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize + n * 96usize) as _)
        }
    }
    #[doc = "Fractional Control Register."]
    #[inline(always)]
    pub const fn frctrl(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smfrctrl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 96usize) as _)
        }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn octrl(self, n: usize) -> crate::pac::common::Reg<Smoctrl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize + n * 96usize) as _)
        }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sts(self, n: usize) -> crate::pac::common::Reg<Smsts, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 96usize) as _)
        }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn inten(self, n: usize) -> crate::pac::common::Reg<Sminten, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize + n * 96usize) as _)
        }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn dmaen(self, n: usize) -> crate::pac::common::Reg<Smdmaen, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize + n * 96usize) as _)
        }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn tctrl(self, n: usize) -> crate::pac::common::Reg<Smtctrl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize + n * 96usize) as _)
        }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn dismap0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smdismap0, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize + n * 96usize) as _)
        }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn dtcnt0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smdtcnt0, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 96usize) as _)
        }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn dtcnt1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smdtcnt1, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Control A Register."]
    #[inline(always)]
    pub const fn captctrla(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptctrla, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Compare A Register."]
    #[inline(always)]
    pub const fn captcompa(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptcompa, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Control B Register."]
    #[inline(always)]
    pub const fn captctrlb(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptctrlb, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Compare B Register."]
    #[inline(always)]
    pub const fn captcompb(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptcompb, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn captctrlx(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptctrlx, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn captcompx(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptcompx, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn cval0(self, n: usize) -> crate::pac::common::Reg<Smcval0, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn cval0cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval0cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn cval1(self, n: usize) -> crate::pac::common::Reg<Smcval1, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn cval1cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval1cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 2 Register."]
    #[inline(always)]
    pub const fn cval2(self, n: usize) -> crate::pac::common::Reg<Smcval2, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 2 Cycle Register."]
    #[inline(always)]
    pub const fn cval2cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval2cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4ausize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 3 Register."]
    #[inline(always)]
    pub const fn cval3(self, n: usize) -> crate::pac::common::Reg<Smcval3, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 3 Cycle Register."]
    #[inline(always)]
    pub const fn cval3cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval3cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4eusize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 4 Register."]
    #[inline(always)]
    pub const fn cval4(self, n: usize) -> crate::pac::common::Reg<Smcval4, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 4 Cycle Register."]
    #[inline(always)]
    pub const fn cval4cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval4cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 5 Register."]
    #[inline(always)]
    pub const fn cval5(self, n: usize) -> crate::pac::common::Reg<Smcval5, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture Value 5 Cycle Register."]
    #[inline(always)]
    pub const fn cval5cyc(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcval5cyc, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize + n * 96usize) as _)
        }
    }
    #[doc = "Capture PWM_A Input Filter Register."]
    #[inline(always)]
    pub const fn captfilta(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptfilta, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize + n * 96usize) as _)
        }
    }
    #[doc = "Capture PWM_B Input Filter Register."]
    #[inline(always)]
    pub const fn captfiltb(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptfiltb, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize + n * 96usize) as _)
        }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn captfiltx(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smcaptfiltx, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize + n * 96usize) as _)
        }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn phasedly(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Smphasedly, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize + n * 96usize) as _)
        }
    }
    #[doc = "Output Enable Register."]
    #[inline(always)]
    pub const fn outen(self) -> crate::pac::common::Reg<Outen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Mask Register."]
    #[inline(always)]
    pub const fn mask(self) -> crate::pac::common::Reg<Mask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "Software Controlled Output Register."]
    #[inline(always)]
    pub const fn swcout(self) -> crate::pac::common::Reg<Swcout, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "PWM Source Select Register."]
    #[inline(always)]
    pub const fn dtsrcsel(self) -> crate::pac::common::Reg<Dtsrcsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0186usize) as _) }
    }
    #[doc = "Master Control Register."]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::pac::common::Reg<Mctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Master Control 2 Register."]
    #[inline(always)]
    pub const fn mctrl2(self) -> crate::pac::common::Reg<Mctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018ausize) as _) }
    }
    #[doc = "Fault Control Register."]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::pac::common::Reg<Fctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Fault Status Register."]
    #[inline(always)]
    pub const fn fsts0(self) -> crate::pac::common::Reg<Fsts0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018eusize) as _) }
    }
    #[doc = "Fault Filter Register."]
    #[inline(always)]
    pub const fn ffilt0(self) -> crate::pac::common::Reg<Ffilt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Fault Test Register."]
    #[inline(always)]
    pub const fn ftst0(self) -> crate::pac::common::Reg<Ftst0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0192usize) as _) }
    }
    #[doc = "Fault Control 2 Register."]
    #[inline(always)]
    pub const fn fctrl20(self) -> crate::pac::common::Reg<Fctrl20, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
#[doc = "PWM Source Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsrcsel(pub u16);
impl Dtsrcsel {
    #[doc = "Submodule PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel45(&self, n: usize) -> Smsel45 {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Smsel45::from_bits(val as u8)
    }
    #[doc = "Submodule PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sel45(&mut self, n: usize, val: Smsel45) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u16) & 0x03) << offs);
    }
    #[doc = "Submodule PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel23(&self, n: usize) -> Smsel23 {
        assert!(n < 4usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x03;
        Smsel23::from_bits(val as u8)
    }
    #[doc = "Submodule PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sel23(&mut self, n: usize, val: Smsel23) {
        assert!(n < 4usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u16) & 0x03) << offs);
    }
}
impl Default for Dtsrcsel {
    #[inline(always)]
    fn default() -> Dtsrcsel {
        Dtsrcsel(0)
    }
}
impl core::fmt::Debug for Dtsrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dtsrcsel")
            .field("sel45[0]", &self.sel45(0usize))
            .field("sel45[1]", &self.sel45(1usize))
            .field("sel45[2]", &self.sel45(2usize))
            .field("sel45[3]", &self.sel45(3usize))
            .field("sel23[0]", &self.sel23(0usize))
            .field("sel23[1]", &self.sel23(1usize))
            .field("sel23[2]", &self.sel23(2usize))
            .field("sel23[3]", &self.sel23(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtsrcsel {{ sel45[0]: {:?}, sel45[1]: {:?}, sel45[2]: {:?}, sel45[3]: {:?}, sel23[0]: {:?}, sel23[1]: {:?}, sel23[2]: {:?}, sel23[3]: {:?} }}",
            self.sel45(0usize),
            self.sel45(1usize),
            self.sel45(2usize),
            self.sel45(3usize),
            self.sel23(0usize),
            self.sel23(1usize),
            self.sel23(2usize),
            self.sel23(3usize)
        )
    }
}
#[doc = "Fault Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl0(pub u16);
impl Fctrl0 {
    #[doc = "Fault Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn fie(&self) -> Fie {
        let val = (self.0 >> 0usize) & 0x0f;
        Fie::from_bits(val as u8)
    }
    #[doc = "Fault Interrupt Enables."]
    #[inline(always)]
    pub const fn set_fie(&mut self, val: Fie) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Fault Safety Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn fsafe(&self) -> Fsafe {
        let val = (self.0 >> 4usize) & 0x0f;
        Fsafe::from_bits(val as u8)
    }
    #[doc = "Fault Safety Mode."]
    #[inline(always)]
    pub const fn set_fsafe(&mut self, val: Fsafe) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Automatic Fault Clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn fauto(&self) -> Fauto {
        let val = (self.0 >> 8usize) & 0x0f;
        Fauto::from_bits(val as u8)
    }
    #[doc = "Automatic Fault Clearing."]
    #[inline(always)]
    pub const fn set_fauto(&mut self, val: Fauto) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Fault Level."]
    #[must_use]
    #[inline(always)]
    pub const fn flvl(&self) -> Flvl {
        let val = (self.0 >> 12usize) & 0x0f;
        Flvl::from_bits(val as u8)
    }
    #[doc = "Fault Level."]
    #[inline(always)]
    pub const fn set_flvl(&mut self, val: Flvl) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fctrl0 {
    #[inline(always)]
    fn default() -> Fctrl0 {
        Fctrl0(0)
    }
}
impl core::fmt::Debug for Fctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl0")
            .field("fie", &self.fie())
            .field("fsafe", &self.fsafe())
            .field("fauto", &self.fauto())
            .field("flvl", &self.flvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl0 {{ fie: {:?}, fsafe: {:?}, fauto: {:?}, flvl: {:?} }}",
            self.fie(),
            self.fsafe(),
            self.fauto(),
            self.flvl()
        )
    }
}
#[doc = "Fault Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl20(pub u16);
impl Fctrl20 {
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
    #[must_use]
    #[inline(always)]
    pub const fn nocomb(&self) -> Nocomb {
        let val = (self.0 >> 0usize) & 0x0f;
        Nocomb::from_bits(val as u8)
    }
    #[doc = "No Combinational Path From Fault Input To PWM Output."]
    #[inline(always)]
    pub const fn set_nocomb(&mut self, val: Nocomb) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
}
impl Default for Fctrl20 {
    #[inline(always)]
    fn default() -> Fctrl20 {
        Fctrl20(0)
    }
}
impl core::fmt::Debug for Fctrl20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl20")
            .field("nocomb", &self.nocomb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fctrl20 {{ nocomb: {:?} }}", self.nocomb())
    }
}
#[doc = "Fault Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffilt0(pub u16);
impl Ffilt0 {
    #[doc = "Fault Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fault Filter Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Fault Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Fault Filter Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
    #[doc = "Fault Glitch Stretch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gstr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Glitch Stretch Enable."]
    #[inline(always)]
    pub const fn set_gstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Ffilt0 {
    #[inline(always)]
    fn default() -> Ffilt0 {
        Ffilt0(0)
    }
}
impl core::fmt::Debug for Ffilt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ffilt0")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .field("gstr", &self.gstr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ffilt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ffilt0 {{ filt_per: {=u8:?}, filt_cnt: {=u8:?}, gstr: {=bool:?} }}",
            self.filt_per(),
            self.filt_cnt(),
            self.gstr()
        )
    }
}
#[doc = "Fault Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsts0(pub u16);
impl Fsts0 {
    #[doc = "Fault Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn fflag(&self) -> Fflag {
        let val = (self.0 >> 0usize) & 0x0f;
        Fflag::from_bits(val as u8)
    }
    #[doc = "Fault Flags."]
    #[inline(always)]
    pub const fn set_fflag(&mut self, val: Fflag) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Full Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn ffull(&self) -> Ffull {
        let val = (self.0 >> 4usize) & 0x0f;
        Ffull::from_bits(val as u8)
    }
    #[doc = "Full Cycle."]
    #[inline(always)]
    pub const fn set_ffull(&mut self, val: Ffull) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u16) & 0x0f) << 4usize);
    }
    #[doc = "Filtered Fault Pins."]
    #[must_use]
    #[inline(always)]
    pub const fn ffpin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filtered Fault Pins."]
    #[inline(always)]
    pub const fn set_ffpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Half Cycle Fault Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn fhalf(&self) -> Fhalf {
        let val = (self.0 >> 12usize) & 0x0f;
        Fhalf::from_bits(val as u8)
    }
    #[doc = "Half Cycle Fault Recovery."]
    #[inline(always)]
    pub const fn set_fhalf(&mut self, val: Fhalf) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Fsts0 {
    #[inline(always)]
    fn default() -> Fsts0 {
        Fsts0(0)
    }
}
impl core::fmt::Debug for Fsts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsts0")
            .field("fflag", &self.fflag())
            .field("ffull", &self.ffull())
            .field("ffpin", &self.ffpin())
            .field("fhalf", &self.fhalf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsts0 {{ fflag: {:?}, ffull: {:?}, ffpin: {=u8:?}, fhalf: {:?} }}",
            self.fflag(),
            self.ffull(),
            self.ffpin(),
            self.fhalf()
        )
    }
}
#[doc = "Fault Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftst0(pub u16);
impl Ftst0 {
    #[doc = "Fault Test."]
    #[must_use]
    #[inline(always)]
    pub const fn ftest(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fault Test."]
    #[inline(always)]
    pub const fn set_ftest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
}
impl Default for Ftst0 {
    #[inline(always)]
    fn default() -> Ftst0 {
        Ftst0(0)
    }
}
impl core::fmt::Debug for Ftst0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftst0")
            .field("ftest", &self.ftest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftst0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ftst0 {{ ftest: {=bool:?} }}", self.ftest())
    }
}
#[doc = "Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u16);
impl Mask {
    #[doc = "PWM_X Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Masks."]
    #[inline(always)]
    pub const fn set_maskx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maskb(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Masks."]
    #[inline(always)]
    pub const fn set_maskb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Masks."]
    #[must_use]
    #[inline(always)]
    pub const fn maska(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Masks."]
    #[inline(always)]
    pub const fn set_maska(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Update Mask Bits Immediately."]
    #[must_use]
    #[inline(always)]
    pub const fn update_mask(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Update Mask Bits Immediately."]
    #[inline(always)]
    pub const fn set_update_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
impl core::fmt::Debug for Mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mask")
            .field("maskx", &self.maskx())
            .field("maskb", &self.maskb())
            .field("maska", &self.maska())
            .field("update_mask", &self.update_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mask {{ maskx: {=u8:?}, maskb: {=u8:?}, maska: {=u8:?}, update_mask: {=u8:?} }}",
            self.maskx(),
            self.maskb(),
            self.maska(),
            self.update_mask()
        )
    }
}
#[doc = "Master Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u16);
impl Mctrl {
    #[doc = "Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn ldok(&self) -> Ldok {
        let val = (self.0 >> 0usize) & 0x0f;
        Ldok::from_bits(val as u8)
    }
    #[doc = "Load Okay."]
    #[inline(always)]
    pub const fn set_ldok(&mut self, val: Ldok) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u16) & 0x0f) << 0usize);
    }
    #[doc = "Clear Load Okay."]
    #[must_use]
    #[inline(always)]
    pub const fn cldok(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Load Okay."]
    #[inline(always)]
    pub const fn set_cldok(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Run."]
    #[must_use]
    #[inline(always)]
    pub const fn run(&self) -> Run {
        let val = (self.0 >> 8usize) & 0x0f;
        Run::from_bits(val as u8)
    }
    #[doc = "Run."]
    #[inline(always)]
    pub const fn set_run(&mut self, val: Run) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Current Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn ipol(&self) -> Ipol {
        let val = (self.0 >> 12usize) & 0x0f;
        Ipol::from_bits(val as u8)
    }
    #[doc = "Current Polarity."]
    #[inline(always)]
    pub const fn set_ipol(&mut self, val: Ipol) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("ldok", &self.ldok())
            .field("cldok", &self.cldok())
            .field("run", &self.run())
            .field("ipol", &self.ipol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ ldok: {:?}, cldok: {=u8:?}, run: {:?}, ipol: {:?} }}",
            self.ldok(),
            self.cldok(),
            self.run(),
            self.ipol()
        )
    }
}
#[doc = "Master Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl2(pub u16);
impl Mctrl2 {
    #[doc = "Write protect."]
    #[must_use]
    #[inline(always)]
    pub const fn wrprot(&self) -> Wrprot {
        let val = (self.0 >> 2usize) & 0x03;
        Wrprot::from_bits(val as u8)
    }
    #[doc = "Write protect."]
    #[inline(always)]
    pub const fn set_wrprot(&mut self, val: Wrprot) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
    #[must_use]
    #[inline(always)]
    pub const fn stretch_cnt_prsc(&self) -> StretchCntPrsc {
        let val = (self.0 >> 6usize) & 0x03;
        StretchCntPrsc::from_bits(val as u8)
    }
    #[doc = "Stretch IPBus clock count prescaler for mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig."]
    #[inline(always)]
    pub const fn set_stretch_cnt_prsc(&mut self, val: StretchCntPrsc) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
}
impl Default for Mctrl2 {
    #[inline(always)]
    fn default() -> Mctrl2 {
        Mctrl2(0)
    }
}
impl core::fmt::Debug for Mctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl2")
            .field("wrprot", &self.wrprot())
            .field("stretch_cnt_prsc", &self.stretch_cnt_prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl2 {{ wrprot: {:?}, stretch_cnt_prsc: {:?} }}",
            self.wrprot(),
            self.stretch_cnt_prsc()
        )
    }
}
#[doc = "Output Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outen(pub u16);
impl Outen {
    #[doc = "PWM_X Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Output Enables."]
    #[inline(always)]
    pub const fn set_pwmx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Output Enables."]
    #[inline(always)]
    pub const fn set_pwmb_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_A Output Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Output Enables."]
    #[inline(always)]
    pub const fn set_pwma_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Outen {
    #[inline(always)]
    fn default() -> Outen {
        Outen(0)
    }
}
impl core::fmt::Debug for Outen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outen")
            .field("pwmx_en", &self.pwmx_en())
            .field("pwmb_en", &self.pwmb_en())
            .field("pwma_en", &self.pwma_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outen {{ pwmx_en: {=u8:?}, pwmb_en: {=u8:?}, pwma_en: {=u8:?} }}",
            self.pwmx_en(),
            self.pwmb_en(),
            self.pwma_en()
        )
    }
}
#[doc = "Capture Compare A Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptcompa(pub u16);
impl Smcaptcompa {
    #[doc = "Edge Compare A."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpa(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare A."]
    #[inline(always)]
    pub const fn set_edgcmpa(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter A."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter A."]
    #[inline(always)]
    pub const fn set_edgcnta(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompa {
    #[inline(always)]
    fn default() -> Smcaptcompa {
        Smcaptcompa(0)
    }
}
impl core::fmt::Debug for Smcaptcompa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptcompa")
            .field("edgcmpa", &self.edgcmpa())
            .field("edgcnta", &self.edgcnta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptcompa {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptcompa {{ edgcmpa: {=u8:?}, edgcnta: {=u8:?} }}",
            self.edgcmpa(),
            self.edgcnta()
        )
    }
}
#[doc = "Capture Compare B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptcompb(pub u16);
impl Smcaptcompb {
    #[doc = "Edge Compare B."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare B."]
    #[inline(always)]
    pub const fn set_edgcmpb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter B."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter B."]
    #[inline(always)]
    pub const fn set_edgcntb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompb {
    #[inline(always)]
    fn default() -> Smcaptcompb {
        Smcaptcompb(0)
    }
}
impl core::fmt::Debug for Smcaptcompb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptcompb")
            .field("edgcmpb", &self.edgcmpb())
            .field("edgcntb", &self.edgcntb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptcompb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptcompb {{ edgcmpb: {=u8:?}, edgcntb: {=u8:?} }}",
            self.edgcmpb(),
            self.edgcntb()
        )
    }
}
#[doc = "Capture Compare X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptcompx(pub u16);
impl Smcaptcompx {
    #[doc = "Edge Compare X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcmpx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Compare X."]
    #[inline(always)]
    pub const fn set_edgcmpx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Edge Counter X."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Edge Counter X."]
    #[inline(always)]
    pub const fn set_edgcntx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Smcaptcompx {
    #[inline(always)]
    fn default() -> Smcaptcompx {
        Smcaptcompx(0)
    }
}
impl core::fmt::Debug for Smcaptcompx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptcompx")
            .field("edgcmpx", &self.edgcmpx())
            .field("edgcntx", &self.edgcntx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptcompx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptcompx {{ edgcmpx: {=u8:?}, edgcntx: {=u8:?} }}",
            self.edgcmpx(),
            self.edgcntx()
        )
    }
}
#[doc = "Capture Control A Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptctrla(pub u16);
impl Smcaptctrla {
    #[doc = "Arm A."]
    #[must_use]
    #[inline(always)]
    pub const fn arma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm A."]
    #[inline(always)]
    pub const fn set_arma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode A."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshota(&self) -> SmcaptctrlaOneshota {
        let val = (self.0 >> 1usize) & 0x01;
        SmcaptctrlaOneshota::from_bits(val as u8)
    }
    #[doc = "One Shot Mode A."]
    #[inline(always)]
    pub const fn set_oneshota(&mut self, val: SmcaptctrlaOneshota) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge A 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edga0(&self) -> SmcaptctrlaEdga0 {
        let val = (self.0 >> 2usize) & 0x03;
        SmcaptctrlaEdga0::from_bits(val as u8)
    }
    #[doc = "Edge A 0."]
    #[inline(always)]
    pub const fn set_edga0(&mut self, val: SmcaptctrlaEdga0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge A 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edga1(&self) -> SmcaptctrlaEdga1 {
        let val = (self.0 >> 4usize) & 0x03;
        SmcaptctrlaEdga1::from_bits(val as u8)
    }
    #[doc = "Edge A 1."]
    #[inline(always)]
    pub const fn set_edga1(&mut self, val: SmcaptctrlaEdga1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select A."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_sela(&self) -> SmcaptctrlaInpSela {
        let val = (self.0 >> 6usize) & 0x01;
        SmcaptctrlaInpSela::from_bits(val as u8)
    }
    #[doc = "Input Select A."]
    #[inline(always)]
    pub const fn set_inp_sela(&mut self, val: SmcaptctrlaInpSela) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter A Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcnta_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter A Enable."]
    #[inline(always)]
    pub const fn set_edgcnta_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture A FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfawm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture A FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfawm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture A0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn ca0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_ca0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture A1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn ca1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture A1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_ca1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrla {
    #[inline(always)]
    fn default() -> Smcaptctrla {
        Smcaptctrla(0)
    }
}
impl core::fmt::Debug for Smcaptctrla {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptctrla")
            .field("arma", &self.arma())
            .field("oneshota", &self.oneshota())
            .field("edga0", &self.edga0())
            .field("edga1", &self.edga1())
            .field("inp_sela", &self.inp_sela())
            .field("edgcnta_en", &self.edgcnta_en())
            .field("cfawm", &self.cfawm())
            .field("ca0cnt", &self.ca0cnt())
            .field("ca1cnt", &self.ca1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptctrla {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptctrla {{ arma: {=bool:?}, oneshota: {:?}, edga0: {:?}, edga1: {:?}, inp_sela: {:?}, edgcnta_en: {=bool:?}, cfawm: {=u8:?}, ca0cnt: {=u8:?}, ca1cnt: {=u8:?} }}",
            self.arma(),
            self.oneshota(),
            self.edga0(),
            self.edga1(),
            self.inp_sela(),
            self.edgcnta_en(),
            self.cfawm(),
            self.ca0cnt(),
            self.ca1cnt()
        )
    }
}
#[doc = "Capture Control B Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptctrlb(pub u16);
impl Smcaptctrlb {
    #[doc = "Arm B."]
    #[must_use]
    #[inline(always)]
    pub const fn armb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm B."]
    #[inline(always)]
    pub const fn set_armb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode B."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotb(&self) -> SmcaptctrlbOneshotb {
        let val = (self.0 >> 1usize) & 0x01;
        SmcaptctrlbOneshotb::from_bits(val as u8)
    }
    #[doc = "One Shot Mode B."]
    #[inline(always)]
    pub const fn set_oneshotb(&mut self, val: SmcaptctrlbOneshotb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge B 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgb0(&self) -> SmcaptctrlbEdgb0 {
        let val = (self.0 >> 2usize) & 0x03;
        SmcaptctrlbEdgb0::from_bits(val as u8)
    }
    #[doc = "Edge B 0."]
    #[inline(always)]
    pub const fn set_edgb0(&mut self, val: SmcaptctrlbEdgb0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge B 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgb1(&self) -> SmcaptctrlbEdgb1 {
        let val = (self.0 >> 4usize) & 0x03;
        SmcaptctrlbEdgb1::from_bits(val as u8)
    }
    #[doc = "Edge B 1."]
    #[inline(always)]
    pub const fn set_edgb1(&mut self, val: SmcaptctrlbEdgb1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select B."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selb(&self) -> SmcaptctrlbInpSelb {
        let val = (self.0 >> 6usize) & 0x01;
        SmcaptctrlbInpSelb::from_bits(val as u8)
    }
    #[doc = "Input Select B."]
    #[inline(always)]
    pub const fn set_inp_selb(&mut self, val: SmcaptctrlbInpSelb) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter B Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntb_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter B Enable."]
    #[inline(always)]
    pub const fn set_edgcntb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture B FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfbwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture B FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfbwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture B0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cb0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cb0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture B1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cb1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture B1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cb1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrlb {
    #[inline(always)]
    fn default() -> Smcaptctrlb {
        Smcaptctrlb(0)
    }
}
impl core::fmt::Debug for Smcaptctrlb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptctrlb")
            .field("armb", &self.armb())
            .field("oneshotb", &self.oneshotb())
            .field("edgb0", &self.edgb0())
            .field("edgb1", &self.edgb1())
            .field("inp_selb", &self.inp_selb())
            .field("edgcntb_en", &self.edgcntb_en())
            .field("cfbwm", &self.cfbwm())
            .field("cb0cnt", &self.cb0cnt())
            .field("cb1cnt", &self.cb1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptctrlb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptctrlb {{ armb: {=bool:?}, oneshotb: {:?}, edgb0: {:?}, edgb1: {:?}, inp_selb: {:?}, edgcntb_en: {=bool:?}, cfbwm: {=u8:?}, cb0cnt: {=u8:?}, cb1cnt: {=u8:?} }}",
            self.armb(),
            self.oneshotb(),
            self.edgb0(),
            self.edgb1(),
            self.inp_selb(),
            self.edgcntb_en(),
            self.cfbwm(),
            self.cb0cnt(),
            self.cb1cnt()
        )
    }
}
#[doc = "Capture Control X Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptctrlx(pub u16);
impl Smcaptctrlx {
    #[doc = "Arm X."]
    #[must_use]
    #[inline(always)]
    pub const fn armx(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm X."]
    #[inline(always)]
    pub const fn set_armx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "One Shot Mode Aux."]
    #[must_use]
    #[inline(always)]
    pub const fn oneshotx(&self) -> SmcaptctrlxOneshotx {
        let val = (self.0 >> 1usize) & 0x01;
        SmcaptctrlxOneshotx::from_bits(val as u8)
    }
    #[doc = "One Shot Mode Aux."]
    #[inline(always)]
    pub const fn set_oneshotx(&mut self, val: SmcaptctrlxOneshotx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Edge X 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx0(&self) -> SmcaptctrlxEdgx0 {
        let val = (self.0 >> 2usize) & 0x03;
        SmcaptctrlxEdgx0::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: SmcaptctrlxEdgx0) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> SmcaptctrlxEdgx1 {
        let val = (self.0 >> 4usize) & 0x03;
        SmcaptctrlxEdgx1::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: SmcaptctrlxEdgx1) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Input Select X."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_selx(&self) -> SmcaptctrlxInpSelx {
        let val = (self.0 >> 6usize) & 0x01;
        SmcaptctrlxInpSelx::from_bits(val as u8)
    }
    #[doc = "Input Select X."]
    #[inline(always)]
    pub const fn set_inp_selx(&mut self, val: SmcaptctrlxInpSelx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Edge Counter X Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn edgcntx_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Edge Counter X Enable."]
    #[inline(always)]
    pub const fn set_edgcntx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Capture X FIFOs Water Mark."]
    #[inline(always)]
    pub const fn set_cfxwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0cnt(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X0 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx0cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u16) & 0x07) << 10usize);
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1cnt(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Capture X1 FIFO Word Count."]
    #[inline(always)]
    pub const fn set_cx1cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
    }
}
impl Default for Smcaptctrlx {
    #[inline(always)]
    fn default() -> Smcaptctrlx {
        Smcaptctrlx(0)
    }
}
impl core::fmt::Debug for Smcaptctrlx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptctrlx")
            .field("armx", &self.armx())
            .field("oneshotx", &self.oneshotx())
            .field("edgx0", &self.edgx0())
            .field("edgx1", &self.edgx1())
            .field("inp_selx", &self.inp_selx())
            .field("edgcntx_en", &self.edgcntx_en())
            .field("cfxwm", &self.cfxwm())
            .field("cx0cnt", &self.cx0cnt())
            .field("cx1cnt", &self.cx1cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptctrlx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptctrlx {{ armx: {=bool:?}, oneshotx: {:?}, edgx0: {:?}, edgx1: {:?}, inp_selx: {:?}, edgcntx_en: {=bool:?}, cfxwm: {=u8:?}, cx0cnt: {=u8:?}, cx1cnt: {=u8:?} }}",
            self.armx(),
            self.oneshotx(),
            self.edgx0(),
            self.edgx1(),
            self.inp_selx(),
            self.edgcntx_en(),
            self.cfxwm(),
            self.cx0cnt(),
            self.cx1cnt()
        )
    }
}
#[doc = "Capture PWM_A Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptfilta(pub u16);
impl Smcaptfilta {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_capta_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn capta_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_capta_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Smcaptfilta {
    #[inline(always)]
    fn default() -> Smcaptfilta {
        Smcaptfilta(0)
    }
}
impl core::fmt::Debug for Smcaptfilta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptfilta")
            .field("capta_filt_per", &self.capta_filt_per())
            .field("capta_filt_cnt", &self.capta_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptfilta {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptfilta {{ capta_filt_per: {=u8:?}, capta_filt_cnt: {=u8:?} }}",
            self.capta_filt_per(),
            self.capta_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_B Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptfiltb(pub u16);
impl Smcaptfiltb {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captb_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captb_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captb_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Smcaptfiltb {
    #[inline(always)]
    fn default() -> Smcaptfiltb {
        Smcaptfiltb(0)
    }
}
impl core::fmt::Debug for Smcaptfiltb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptfiltb")
            .field("captb_filt_per", &self.captb_filt_per())
            .field("captb_filt_cnt", &self.captb_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptfiltb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptfiltb {{ captb_filt_per: {=u8:?}, captb_filt_cnt: {=u8:?} }}",
            self.captb_filt_per(),
            self.captb_filt_cnt()
        )
    }
}
#[doc = "Capture PWM_X Input Filter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcaptfiltx(pub u16);
impl Smcaptfiltx {
    #[doc = "Input Capture Filter Period."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Input Capture Filter Period."]
    #[inline(always)]
    pub const fn set_captx_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Input Capture Filter Count."]
    #[must_use]
    #[inline(always)]
    pub const fn captx_filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Input Capture Filter Count."]
    #[inline(always)]
    pub const fn set_captx_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for Smcaptfiltx {
    #[inline(always)]
    fn default() -> Smcaptfiltx {
        Smcaptfiltx(0)
    }
}
impl core::fmt::Debug for Smcaptfiltx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcaptfiltx")
            .field("captx_filt_per", &self.captx_filt_per())
            .field("captx_filt_cnt", &self.captx_filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcaptfiltx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smcaptfiltx {{ captx_filt_per: {=u8:?}, captx_filt_cnt: {=u8:?} }}",
            self.captx_filt_per(),
            self.captx_filt_cnt()
        )
    }
}
#[doc = "Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcnt(pub u16);
impl Smcnt {
    #[doc = "Counter Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Register Bits."]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcnt {
    #[inline(always)]
    fn default() -> Smcnt {
        Smcnt(0)
    }
}
impl core::fmt::Debug for Smcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcnt").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcnt {{ cnt: {=u16:?} }}", self.cnt())
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smctrl(pub u16);
impl Smctrl {
    #[doc = "Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dblx(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Double Switching Enable."]
    #[inline(always)]
    pub const fn set_dblx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Load Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ldmod(&self) -> SmctrlLdmod {
        let val = (self.0 >> 2usize) & 0x01;
        SmctrlLdmod::from_bits(val as u8)
    }
    #[doc = "Load Mode Select."]
    #[inline(always)]
    pub const fn set_ldmod(&mut self, val: SmctrlLdmod) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn split(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Split the DBLPWM signal to PWM_A and PWM_B."]
    #[inline(always)]
    pub const fn set_split(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> SmctrlPrsc {
        let val = (self.0 >> 4usize) & 0x07;
        SmctrlPrsc::from_bits(val as u8)
    }
    #[doc = "Prescaler."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: SmctrlPrsc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u16) & 0x07) << 4usize);
    }
    #[doc = "Compare Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn compmode(&self) -> SmctrlCompmode {
        let val = (self.0 >> 7usize) & 0x01;
        SmctrlCompmode::from_bits(val as u8)
    }
    #[doc = "Compare Mode."]
    #[inline(always)]
    pub const fn set_compmode(&mut self, val: SmctrlCompmode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
    }
    #[doc = "Deadtime."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Deadtime."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
    }
    #[doc = "Full Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Full Cycle Reload."]
    #[inline(always)]
    pub const fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Half Cycle Reload."]
    #[must_use]
    #[inline(always)]
    pub const fn half(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Half Cycle Reload."]
    #[inline(always)]
    pub const fn set_half(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "Load Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn ldfq(&self) -> SmctrlLdfq {
        let val = (self.0 >> 12usize) & 0x0f;
        SmctrlLdfq::from_bits(val as u8)
    }
    #[doc = "Load Frequency."]
    #[inline(always)]
    pub const fn set_ldfq(&mut self, val: SmctrlLdfq) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for Smctrl {
    #[inline(always)]
    fn default() -> Smctrl {
        Smctrl(0)
    }
}
impl core::fmt::Debug for Smctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smctrl")
            .field("dblen", &self.dblen())
            .field("dblx", &self.dblx())
            .field("ldmod", &self.ldmod())
            .field("split", &self.split())
            .field("prsc", &self.prsc())
            .field("compmode", &self.compmode())
            .field("dt", &self.dt())
            .field("full", &self.full())
            .field("half", &self.half())
            .field("ldfq", &self.ldfq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smctrl {{ dblen: {=bool:?}, dblx: {=bool:?}, ldmod: {:?}, split: {=bool:?}, prsc: {:?}, compmode: {:?}, dt: {=u8:?}, full: {=bool:?}, half: {=bool:?}, ldfq: {:?} }}",
            self.dblen(),
            self.dblx(),
            self.ldmod(),
            self.split(),
            self.prsc(),
            self.compmode(),
            self.dt(),
            self.full(),
            self.half(),
            self.ldfq()
        )
    }
}
#[doc = "Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smctrl2(pub u16);
impl Smctrl2 {
    #[doc = "Clock Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> Smctrl2ClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        Smctrl2ClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: Smctrl2ClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> Smctrl2ReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        Smctrl2ReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: Smctrl2ReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> Smctrl2ForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        Smctrl2ForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: Smctrl2ForceSel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u16) & 0x07) << 3usize);
    }
    #[doc = "Force Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Initialization."]
    #[inline(always)]
    pub const fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Force Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frcen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force Enable."]
    #[inline(always)]
    pub const fn set_frcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Initialization Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn init_sel(&self) -> Smctrl2InitSel {
        let val = (self.0 >> 8usize) & 0x03;
        Smctrl2InitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: Smctrl2InitSel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "PWM_X Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_init(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Initial Value."]
    #[inline(always)]
    pub const fn set_pwmx_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM45 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm45_init(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "PWM45 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm45_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
    #[doc = "PWM23 Initial Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm23_init(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PWM23 Initial Value."]
    #[inline(always)]
    pub const fn set_pwm23_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn indep(&self) -> Smctrl2Indep {
        let val = (self.0 >> 13usize) & 0x01;
        Smctrl2Indep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: Smctrl2Indep) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u16) & 0x01) << 13usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Smctrl2 {
    #[inline(always)]
    fn default() -> Smctrl2 {
        Smctrl2(0)
    }
}
impl core::fmt::Debug for Smctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smctrl2")
            .field("clk_sel", &self.clk_sel())
            .field("reload_sel", &self.reload_sel())
            .field("force_sel", &self.force_sel())
            .field("force", &self.force())
            .field("frcen", &self.frcen())
            .field("init_sel", &self.init_sel())
            .field("pwmx_init", &self.pwmx_init())
            .field("pwm45_init", &self.pwm45_init())
            .field("pwm23_init", &self.pwm23_init())
            .field("indep", &self.indep())
            .field("dbgen", &self.dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smctrl2 {{ clk_sel: {:?}, reload_sel: {:?}, force_sel: {:?}, force: {=bool:?}, frcen: {=bool:?}, init_sel: {:?}, pwmx_init: {=bool:?}, pwm45_init: {=bool:?}, pwm23_init: {=bool:?}, indep: {:?}, dbgen: {=bool:?} }}",
            self.clk_sel(),
            self.reload_sel(),
            self.force_sel(),
            self.force(),
            self.frcen(),
            self.init_sel(),
            self.pwmx_init(),
            self.pwm45_init(),
            self.pwm23_init(),
            self.indep(),
            self.dbgen()
        )
    }
}
#[doc = "Capture Value 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval0(pub u16);
impl Smcval0 {
    #[doc = "Capture Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn captval0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 0."]
    #[inline(always)]
    pub const fn set_captval0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval0 {
    #[inline(always)]
    fn default() -> Smcval0 {
        Smcval0(0)
    }
}
impl core::fmt::Debug for Smcval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval0")
            .field("captval0", &self.captval0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval0 {{ captval0: {=u16:?} }}", self.captval0())
    }
}
#[doc = "Capture Value 0 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval0cyc(pub u16);
impl Smcval0cyc {
    #[doc = "Capture Value 0 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval0cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 0 Cycle."]
    #[inline(always)]
    pub const fn set_cval0cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval0cyc {
    #[inline(always)]
    fn default() -> Smcval0cyc {
        Smcval0cyc(0)
    }
}
impl core::fmt::Debug for Smcval0cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval0cyc")
            .field("cval0cyc", &self.cval0cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval0cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval0cyc {{ cval0cyc: {=u8:?} }}", self.cval0cyc())
    }
}
#[doc = "Capture Value 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval1(pub u16);
impl Smcval1 {
    #[doc = "Capture Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn captval1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 1."]
    #[inline(always)]
    pub const fn set_captval1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval1 {
    #[inline(always)]
    fn default() -> Smcval1 {
        Smcval1(0)
    }
}
impl core::fmt::Debug for Smcval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval1")
            .field("captval1", &self.captval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval1 {{ captval1: {=u16:?} }}", self.captval1())
    }
}
#[doc = "Capture Value 1 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval1cyc(pub u16);
impl Smcval1cyc {
    #[doc = "Capture Value 1 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval1cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 1 Cycle."]
    #[inline(always)]
    pub const fn set_cval1cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval1cyc {
    #[inline(always)]
    fn default() -> Smcval1cyc {
        Smcval1cyc(0)
    }
}
impl core::fmt::Debug for Smcval1cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval1cyc")
            .field("cval1cyc", &self.cval1cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval1cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval1cyc {{ cval1cyc: {=u8:?} }}", self.cval1cyc())
    }
}
#[doc = "Capture Value 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval2(pub u16);
impl Smcval2 {
    #[doc = "Capture Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn captval2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 2."]
    #[inline(always)]
    pub const fn set_captval2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval2 {
    #[inline(always)]
    fn default() -> Smcval2 {
        Smcval2(0)
    }
}
impl core::fmt::Debug for Smcval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval2")
            .field("captval2", &self.captval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval2 {{ captval2: {=u16:?} }}", self.captval2())
    }
}
#[doc = "Capture Value 2 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval2cyc(pub u16);
impl Smcval2cyc {
    #[doc = "Capture Value 2 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval2cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 2 Cycle."]
    #[inline(always)]
    pub const fn set_cval2cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval2cyc {
    #[inline(always)]
    fn default() -> Smcval2cyc {
        Smcval2cyc(0)
    }
}
impl core::fmt::Debug for Smcval2cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval2cyc")
            .field("cval2cyc", &self.cval2cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval2cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval2cyc {{ cval2cyc: {=u8:?} }}", self.cval2cyc())
    }
}
#[doc = "Capture Value 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval3(pub u16);
impl Smcval3 {
    #[doc = "Capture Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn captval3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 3."]
    #[inline(always)]
    pub const fn set_captval3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval3 {
    #[inline(always)]
    fn default() -> Smcval3 {
        Smcval3(0)
    }
}
impl core::fmt::Debug for Smcval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval3")
            .field("captval3", &self.captval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval3 {{ captval3: {=u16:?} }}", self.captval3())
    }
}
#[doc = "Capture Value 3 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval3cyc(pub u16);
impl Smcval3cyc {
    #[doc = "Capture Value 3 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval3cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 3 Cycle."]
    #[inline(always)]
    pub const fn set_cval3cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval3cyc {
    #[inline(always)]
    fn default() -> Smcval3cyc {
        Smcval3cyc(0)
    }
}
impl core::fmt::Debug for Smcval3cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval3cyc")
            .field("cval3cyc", &self.cval3cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval3cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval3cyc {{ cval3cyc: {=u8:?} }}", self.cval3cyc())
    }
}
#[doc = "Capture Value 4 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval4(pub u16);
impl Smcval4 {
    #[doc = "Capture Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn captval4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 4."]
    #[inline(always)]
    pub const fn set_captval4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval4 {
    #[inline(always)]
    fn default() -> Smcval4 {
        Smcval4(0)
    }
}
impl core::fmt::Debug for Smcval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval4")
            .field("captval4", &self.captval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval4 {{ captval4: {=u16:?} }}", self.captval4())
    }
}
#[doc = "Capture Value 4 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval4cyc(pub u16);
impl Smcval4cyc {
    #[doc = "Capture Value 4 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval4cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 4 Cycle."]
    #[inline(always)]
    pub const fn set_cval4cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval4cyc {
    #[inline(always)]
    fn default() -> Smcval4cyc {
        Smcval4cyc(0)
    }
}
impl core::fmt::Debug for Smcval4cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval4cyc")
            .field("cval4cyc", &self.cval4cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval4cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval4cyc {{ cval4cyc: {=u8:?} }}", self.cval4cyc())
    }
}
#[doc = "Capture Value 5 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval5(pub u16);
impl Smcval5 {
    #[doc = "Capture Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn captval5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Value 5."]
    #[inline(always)]
    pub const fn set_captval5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smcval5 {
    #[inline(always)]
    fn default() -> Smcval5 {
        Smcval5(0)
    }
}
impl core::fmt::Debug for Smcval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval5")
            .field("captval5", &self.captval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval5 {{ captval5: {=u16:?} }}", self.captval5())
    }
}
#[doc = "Capture Value 5 Cycle Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcval5cyc(pub u16);
impl Smcval5cyc {
    #[doc = "Capture Value 5 Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn cval5cyc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capture Value 5 Cycle."]
    #[inline(always)]
    pub const fn set_cval5cyc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
}
impl Default for Smcval5cyc {
    #[inline(always)]
    fn default() -> Smcval5cyc {
        Smcval5cyc(0)
    }
}
impl core::fmt::Debug for Smcval5cyc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcval5cyc")
            .field("cval5cyc", &self.cval5cyc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcval5cyc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smcval5cyc {{ cval5cyc: {=u8:?} }}", self.cval5cyc())
    }
}
#[doc = "Fault Disable Mapping Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdismap0(pub u16);
impl Smdismap0 {
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_A Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0b(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_B Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dis0x(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PWM_X Fault Disable Mask 0."]
    #[inline(always)]
    pub const fn set_dis0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
}
impl Default for Smdismap0 {
    #[inline(always)]
    fn default() -> Smdismap0 {
        Smdismap0(0)
    }
}
impl core::fmt::Debug for Smdismap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdismap0")
            .field("dis0a", &self.dis0a())
            .field("dis0b", &self.dis0b())
            .field("dis0x", &self.dis0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdismap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smdismap0 {{ dis0a: {=u8:?}, dis0b: {=u8:?}, dis0x: {=u8:?} }}",
            self.dis0a(),
            self.dis0b(),
            self.dis0x()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdmaen(pub u16);
impl Smdmaen {
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0de(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1de(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cx1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Capture B0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cb0de(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cb0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Capture B1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cb1de(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Capture B1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_cb1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capture A0 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ca0de(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A0 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_ca0de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Capture A1 FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ca1de(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Capture A1 FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_ca1de(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn captde(&self) -> SmdmaenCaptde {
        let val = (self.0 >> 6usize) & 0x03;
        SmdmaenCaptde::from_bits(val as u8)
    }
    #[doc = "Capture DMA Enable Source Select."]
    #[inline(always)]
    pub const fn set_captde(&mut self, val: SmdmaenCaptde) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "FIFO Watermark AND Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fand(&self) -> SmdmaenFand {
        let val = (self.0 >> 8usize) & 0x01;
        SmdmaenFand::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark AND Control."]
    #[inline(always)]
    pub const fn set_fand(&mut self, val: SmdmaenFand) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "Value Registers DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn valde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Value Registers DMA Enable."]
    #[inline(always)]
    pub const fn set_valde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Smdmaen {
    #[inline(always)]
    fn default() -> Smdmaen {
        Smdmaen(0)
    }
}
impl core::fmt::Debug for Smdmaen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdmaen")
            .field("cx0de", &self.cx0de())
            .field("cx1de", &self.cx1de())
            .field("cb0de", &self.cb0de())
            .field("cb1de", &self.cb1de())
            .field("ca0de", &self.ca0de())
            .field("ca1de", &self.ca1de())
            .field("captde", &self.captde())
            .field("fand", &self.fand())
            .field("valde", &self.valde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdmaen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smdmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, cb0de: {=bool:?}, cb1de: {=bool:?}, ca0de: {=bool:?}, ca1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
            self.cb0de(),
            self.cb1de(),
            self.ca0de(),
            self.ca1de(),
            self.captde(),
            self.fand(),
            self.valde()
        )
    }
}
#[doc = "Deadtime Count Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdtcnt0(pub u16);
impl Smdtcnt0 {
    #[doc = "Deadtime Count Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn set_dtcnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Smdtcnt0 {
    #[inline(always)]
    fn default() -> Smdtcnt0 {
        Smdtcnt0(0)
    }
}
impl core::fmt::Debug for Smdtcnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdtcnt0")
            .field("dtcnt0", &self.dtcnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdtcnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smdtcnt0 {{ dtcnt0: {=u16:?} }}", self.dtcnt0())
    }
}
#[doc = "Deadtime Count Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdtcnt1(pub u16);
impl Smdtcnt1 {
    #[doc = "Deadtime Count Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dtcnt1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn set_dtcnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
    }
}
impl Default for Smdtcnt1 {
    #[inline(always)]
    fn default() -> Smdtcnt1 {
        Smdtcnt1(0)
    }
}
impl core::fmt::Debug for Smdtcnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdtcnt1")
            .field("dtcnt1", &self.dtcnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdtcnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smdtcnt1 {{ dtcnt1: {=u16:?} }}", self.dtcnt1())
    }
}
#[doc = "Fractional Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfracval1(pub u16);
impl Smfracval1 {
    #[doc = "Fractional Value 1."]
    #[must_use]
    #[inline(always)]
    pub const fn fracval1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 1."]
    #[inline(always)]
    pub const fn set_fracval1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval1 {
    #[inline(always)]
    fn default() -> Smfracval1 {
        Smfracval1(0)
    }
}
impl core::fmt::Debug for Smfracval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfracval1")
            .field("fracval1", &self.fracval1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfracval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smfracval1 {{ fracval1: {=u8:?} }}", self.fracval1())
    }
}
#[doc = "Fractional Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfracval2(pub u16);
impl Smfracval2 {
    #[doc = "Fractional Value 2."]
    #[must_use]
    #[inline(always)]
    pub const fn fracval2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 2."]
    #[inline(always)]
    pub const fn set_fracval2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval2 {
    #[inline(always)]
    fn default() -> Smfracval2 {
        Smfracval2(0)
    }
}
impl core::fmt::Debug for Smfracval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfracval2")
            .field("fracval2", &self.fracval2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfracval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smfracval2 {{ fracval2: {=u8:?} }}", self.fracval2())
    }
}
#[doc = "Fractional Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfracval3(pub u16);
impl Smfracval3 {
    #[doc = "Fractional Value 3."]
    #[must_use]
    #[inline(always)]
    pub const fn fracval3(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 3."]
    #[inline(always)]
    pub const fn set_fracval3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval3 {
    #[inline(always)]
    fn default() -> Smfracval3 {
        Smfracval3(0)
    }
}
impl core::fmt::Debug for Smfracval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfracval3")
            .field("fracval3", &self.fracval3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfracval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smfracval3 {{ fracval3: {=u8:?} }}", self.fracval3())
    }
}
#[doc = "Fractional Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfracval4(pub u16);
impl Smfracval4 {
    #[doc = "Fractional Value 4."]
    #[must_use]
    #[inline(always)]
    pub const fn fracval4(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 4."]
    #[inline(always)]
    pub const fn set_fracval4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval4 {
    #[inline(always)]
    fn default() -> Smfracval4 {
        Smfracval4(0)
    }
}
impl core::fmt::Debug for Smfracval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfracval4")
            .field("fracval4", &self.fracval4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfracval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smfracval4 {{ fracval4: {=u8:?} }}", self.fracval4())
    }
}
#[doc = "Fractional Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfracval5(pub u16);
impl Smfracval5 {
    #[doc = "Fractional Value 5."]
    #[must_use]
    #[inline(always)]
    pub const fn fracval5(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Fractional Value 5."]
    #[inline(always)]
    pub const fn set_fracval5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Smfracval5 {
    #[inline(always)]
    fn default() -> Smfracval5 {
        Smfracval5(0)
    }
}
impl core::fmt::Debug for Smfracval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfracval5")
            .field("fracval5", &self.fracval5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfracval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smfracval5 {{ fracval5: {=u8:?} }}", self.fracval5())
    }
}
#[doc = "Fractional Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smfrctrl(pub u16);
impl Smfrctrl {
    #[doc = "Fractional Cycle PWM Period Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn frac1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle PWM Period Enable."]
    #[inline(always)]
    pub const fn set_frac1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A."]
    #[must_use]
    #[inline(always)]
    pub const fn frac23_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_A."]
    #[inline(always)]
    pub const fn set_frac23_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B."]
    #[must_use]
    #[inline(always)]
    pub const fn frac45_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Fractional Cycle Placement Enable for PWM_B."]
    #[inline(always)]
    pub const fn set_frac45_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Test Status Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn test(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Status Bit."]
    #[inline(always)]
    pub const fn set_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Smfrctrl {
    #[inline(always)]
    fn default() -> Smfrctrl {
        Smfrctrl(0)
    }
}
impl core::fmt::Debug for Smfrctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smfrctrl")
            .field("frac1_en", &self.frac1_en())
            .field("frac23_en", &self.frac23_en())
            .field("frac45_en", &self.frac45_en())
            .field("test", &self.test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smfrctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smfrctrl {{ frac1_en: {=bool:?}, frac23_en: {=bool:?}, frac45_en: {=bool:?}, test: {=bool:?} }}",
            self.frac1_en(),
            self.frac23_en(),
            self.frac45_en(),
            self.test()
        )
    }
}
#[doc = "Initial Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sminit(pub u16);
impl Sminit {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Sminit {
    #[inline(always)]
    fn default() -> Sminit {
        Sminit(0)
    }
}
impl core::fmt::Debug for Sminit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sminit")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sminit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sminit {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sminten(pub u16);
impl Sminten {
    #[doc = "Compare Interrupt Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpie(&self) -> SmintenCmpie {
        let val = (self.0 >> 0usize) & 0x3f;
        SmintenCmpie::from_bits(val as u8)
    }
    #[doc = "Compare Interrupt Enables."]
    #[inline(always)]
    pub const fn set_cmpie(&mut self, val: SmintenCmpie) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
}
impl Default for Sminten {
    #[inline(always)]
    fn default() -> Sminten {
        Sminten(0)
    }
}
impl core::fmt::Debug for Sminten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sminten")
            .field("cmpie", &self.cmpie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sminten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sminten {{ cmpie: {:?} }}", self.cmpie())
    }
}
#[doc = "Output Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smoctrl(pub u16);
impl Smoctrl {
    #[doc = "PWM_X Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmxfs(&self) -> SmoctrlPwmxfs {
        let val = (self.0 >> 0usize) & 0x03;
        SmoctrlPwmxfs::from_bits(val as u8)
    }
    #[doc = "PWM_X Fault State."]
    #[inline(always)]
    pub const fn set_pwmxfs(&mut self, val: SmoctrlPwmxfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "PWM_B Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmbfs(&self) -> SmoctrlPwmbfs {
        let val = (self.0 >> 2usize) & 0x03;
        SmoctrlPwmbfs::from_bits(val as u8)
    }
    #[doc = "PWM_B Fault State."]
    #[inline(always)]
    pub const fn set_pwmbfs(&mut self, val: SmoctrlPwmbfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "PWM_A Fault State."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmafs(&self) -> SmoctrlPwmafs {
        let val = (self.0 >> 4usize) & 0x03;
        SmoctrlPwmafs::from_bits(val as u8)
    }
    #[doc = "PWM_A Fault State."]
    #[inline(always)]
    pub const fn set_pwmafs(&mut self, val: SmoctrlPwmafs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "PWM_X Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Output Polarity."]
    #[inline(always)]
    pub const fn set_polx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "PWM_B Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn polb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Output Polarity."]
    #[inline(always)]
    pub const fn set_polb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "PWM_A Output Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pola(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Output Polarity."]
    #[inline(always)]
    pub const fn set_pola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "PWM_X Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmx_in(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_X Input."]
    #[inline(always)]
    pub const fn set_pwmx_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "PWM_B Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmb_in(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_B Input."]
    #[inline(always)]
    pub const fn set_pwmb_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
    }
    #[doc = "PWM_A Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pwma_in(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_A Input."]
    #[inline(always)]
    pub const fn set_pwma_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Smoctrl {
    #[inline(always)]
    fn default() -> Smoctrl {
        Smoctrl(0)
    }
}
impl core::fmt::Debug for Smoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smoctrl")
            .field("pwmxfs", &self.pwmxfs())
            .field("pwmbfs", &self.pwmbfs())
            .field("pwmafs", &self.pwmafs())
            .field("polx", &self.polx())
            .field("polb", &self.polb())
            .field("pola", &self.pola())
            .field("pwmx_in", &self.pwmx_in())
            .field("pwmb_in", &self.pwmb_in())
            .field("pwma_in", &self.pwma_in())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smoctrl {{ pwmxfs: {:?}, pwmbfs: {:?}, pwmafs: {:?}, polx: {=bool:?}, polb: {=bool:?}, pola: {=bool:?}, pwmx_in: {=bool:?}, pwmb_in: {=bool:?}, pwma_in: {=bool:?} }}",
            self.pwmxfs(),
            self.pwmbfs(),
            self.pwmafs(),
            self.polx(),
            self.polb(),
            self.pola(),
            self.pwmx_in(),
            self.pwmb_in(),
            self.pwma_in()
        )
    }
}
#[doc = "Phase Delay Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smphasedly(pub u16);
impl Smphasedly {
    #[doc = "Initial Count Register Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn phasedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Initial Count Register Bits."]
    #[inline(always)]
    pub const fn set_phasedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smphasedly {
    #[inline(always)]
    fn default() -> Smphasedly {
        Smphasedly(0)
    }
}
impl core::fmt::Debug for Smphasedly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smphasedly")
            .field("phasedly", &self.phasedly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smphasedly {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smphasedly {{ phasedly: {=u16:?} }}", self.phasedly())
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smsts(pub u16);
impl Smsts {
    #[doc = "Compare Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpf(&self) -> SmstsCmpf {
        let val = (self.0 >> 0usize) & 0x3f;
        SmstsCmpf::from_bits(val as u8)
    }
    #[doc = "Compare Flags."]
    #[inline(always)]
    pub const fn set_cmpf(&mut self, val: SmstsCmpf) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Capture Flag X."]
    #[must_use]
    #[inline(always)]
    pub const fn cfxf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X."]
    #[inline(always)]
    pub const fn set_cfxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag B."]
    #[must_use]
    #[inline(always)]
    pub const fn cfbf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag B."]
    #[inline(always)]
    pub const fn set_cfbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Capture Flag A."]
    #[must_use]
    #[inline(always)]
    pub const fn cfaf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag A."]
    #[inline(always)]
    pub const fn set_cfaf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
    }
}
impl Default for Smsts {
    #[inline(always)]
    fn default() -> Smsts {
        Smsts(0)
    }
}
impl core::fmt::Debug for Smsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smsts")
            .field("cmpf", &self.cmpf())
            .field("cfxf", &self.cfxf())
            .field("cfbf", &self.cfbf())
            .field("cfaf", &self.cfaf())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("rev", &self.rev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smsts {{ cmpf: {:?}, cfxf: {=bool:?}, cfbf: {=bool:?}, cfaf: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, rev: {=bool:?} }}",
            self.cmpf(),
            self.cfxf(),
            self.cfbf(),
            self.cfaf(),
            self.rf(),
            self.ref_(),
            self.rev()
        )
    }
}
#[doc = "Output Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smtctrl(pub u16);
impl Smtctrl {
    #[doc = "Output Trigger Enables."]
    #[must_use]
    #[inline(always)]
    pub const fn out_trig_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "PWM_IN Edge Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm_in_edge(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PWM_IN Edge Detection."]
    #[inline(always)]
    pub const fn set_pwm_in_edge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
    #[doc = "And Or Input Synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi_syn(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "And Or Input Synchronization."]
    #[inline(always)]
    pub const fn set_aoi_syn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
    }
}
impl Default for Smtctrl {
    #[inline(always)]
    fn default() -> Smtctrl {
        Smtctrl(0)
    }
}
impl core::fmt::Debug for Smtctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smtctrl")
            .field("out_trig_en", &self.out_trig_en())
            .field("pwm_in_edge", &self.pwm_in_edge())
            .field("aoi_syn", &self.aoi_syn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smtctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smtctrl {{ out_trig_en: {=u8:?}, pwm_in_edge: {=bool:?}, aoi_syn: {=bool:?} }}",
            self.out_trig_en(),
            self.pwm_in_edge(),
            self.aoi_syn()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval0(pub u16);
impl Smval0 {
    #[doc = "Value Register 0."]
    #[must_use]
    #[inline(always)]
    pub const fn init(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn set_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval0 {
    #[inline(always)]
    fn default() -> Smval0 {
        Smval0(0)
    }
}
impl core::fmt::Debug for Smval0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval0")
            .field("init", &self.init())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval0 {{ init: {=u16:?} }}", self.init())
    }
}
#[doc = "Value Register 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval1(pub u16);
impl Smval1 {
    #[doc = "Value Register 1."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval1 {
    #[inline(always)]
    fn default() -> Smval1 {
        Smval1(0)
    }
}
impl core::fmt::Debug for Smval1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval1").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval1 {{ val: {=u16:?} }}", self.val())
    }
}
#[doc = "Value Register 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval2(pub u16);
impl Smval2 {
    #[doc = "Value Register 2."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval2 {
    #[inline(always)]
    fn default() -> Smval2 {
        Smval2(0)
    }
}
impl core::fmt::Debug for Smval2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval2").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval2 {{ val: {=u16:?} }}", self.val())
    }
}
#[doc = "Value Register 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval3(pub u16);
impl Smval3 {
    #[doc = "Value Register 3."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval3 {
    #[inline(always)]
    fn default() -> Smval3 {
        Smval3(0)
    }
}
impl core::fmt::Debug for Smval3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval3").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval3 {{ val: {=u16:?} }}", self.val())
    }
}
#[doc = "Value Register 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval4(pub u16);
impl Smval4 {
    #[doc = "Value Register 4."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval4 {
    #[inline(always)]
    fn default() -> Smval4 {
        Smval4(0)
    }
}
impl core::fmt::Debug for Smval4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval4").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval4 {{ val: {=u16:?} }}", self.val())
    }
}
#[doc = "Value Register 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval5(pub u16);
impl Smval5 {
    #[doc = "Value Register 5."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval5 {
    #[inline(always)]
    fn default() -> Smval5 {
        Smval5(0)
    }
}
impl core::fmt::Debug for Smval5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval5").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval5 {{ val: {=u16:?} }}", self.val())
    }
}
#[doc = "Software Controlled Output Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swcout(pub u16);
impl Swcout {
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out45(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out45(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out23(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
}
impl Default for Swcout {
    #[inline(always)]
    fn default() -> Swcout {
        Swcout(0)
    }
}
impl core::fmt::Debug for Swcout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swcout")
            .field("sm0out45", &self.sm0out45())
            .field("sm0out23", &self.sm0out23())
            .field("sm1out45", &self.sm1out45())
            .field("sm1out23", &self.sm1out23())
            .field("sm2out45", &self.sm2out45())
            .field("sm2out23", &self.sm2out23())
            .field("sm3out45", &self.sm3out45())
            .field("sm3out23", &self.sm3out23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swcout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swcout {{ sm0out45: {=bool:?}, sm0out23: {=bool:?}, sm1out45: {=bool:?}, sm1out23: {=bool:?}, sm2out45: {=bool:?}, sm2out23: {=bool:?}, sm3out45: {=bool:?}, sm3out23: {=bool:?} }}",
            self.sm0out45(),
            self.sm0out23(),
            self.sm1out45(),
            self.sm1out23(),
            self.sm2out45(),
            self.sm2out23(),
            self.sm3out45(),
            self.sm3out23()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fauto {
    #[doc = "Automatic fault clearing is disabled."]
    NoFaultClear = 0x0,
    #[doc = "Automatic fault clearing is enabled."]
    FaultClearOnNextCycle = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Fauto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fauto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fauto {
    #[inline(always)]
    fn from(val: u8) -> Fauto {
        Fauto::from_bits(val)
    }
}
impl From<Fauto> for u8 {
    #[inline(always)]
    fn from(val: Fauto) -> u8 {
        Fauto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fflag {
    #[doc = "No fault."]
    NoFault = 0x0,
    #[doc = "Fault detected."]
    FaultDetected = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Fflag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fflag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fflag {
    #[inline(always)]
    fn from(val: u8) -> Fflag {
        Fflag::from_bits(val)
    }
}
impl From<Fflag> for u8 {
    #[inline(always)]
    fn from(val: Fflag) -> u8 {
        Fflag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ffull {
    #[doc = "Full cycle."]
    Full = 0x0,
    #[doc = "Half cycle."]
    Half = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Ffull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ffull {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ffull {
    #[inline(always)]
    fn from(val: u8) -> Ffull {
        Ffull::from_bits(val)
    }
}
impl From<Ffull> for u8 {
    #[inline(always)]
    fn from(val: Ffull) -> u8 {
        Ffull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fhalf {
    #[doc = "Full cycle recovery."]
    Full = 0x0,
    #[doc = "Half cycle recovery."]
    Half = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Fhalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fhalf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fhalf {
    #[inline(always)]
    fn from(val: u8) -> Fhalf {
        Fhalf::from_bits(val)
    }
}
impl From<Fhalf> for u8 {
    #[inline(always)]
    fn from(val: Fhalf) -> u8 {
        Fhalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fie {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Enabled."]
    Enabled = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Fie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fie {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fie {
    #[inline(always)]
    fn from(val: u8) -> Fie {
        Fie::from_bits(val)
    }
}
impl From<Fie> for u8 {
    #[inline(always)]
    fn from(val: Fie) -> u8 {
        Fie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flvl {
    #[doc = "Fault input is active high."]
    ActiveHigh = 0x0,
    #[doc = "Fault input is active low."]
    ActiveLow = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Flvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flvl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flvl {
    #[inline(always)]
    fn from(val: u8) -> Flvl {
        Flvl::from_bits(val)
    }
}
impl From<Flvl> for u8 {
    #[inline(always)]
    fn from(val: Flvl) -> u8 {
        Flvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fsafe {
    #[doc = "PWM output is forced to zero."]
    PwmOutputZero = 0x0,
    #[doc = "PWM output is forced to one."]
    PwmOutputOne = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Fsafe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fsafe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fsafe {
    #[inline(always)]
    fn from(val: u8) -> Fsafe {
        Fsafe::from_bits(val)
    }
}
impl From<Fsafe> for u8 {
    #[inline(always)]
    fn from(val: Fsafe) -> u8 {
        Fsafe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipol {
    #[doc = "Active high (normal)."]
    ActiveHigh = 0x0,
    #[doc = "Active low (inverted)."]
    ActiveLow = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Ipol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipol {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipol {
    #[inline(always)]
    fn from(val: u8) -> Ipol {
        Ipol::from_bits(val)
    }
}
impl From<Ipol> for u8 {
    #[inline(always)]
    fn from(val: Ipol) -> u8 {
        Ipol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldok {
    #[doc = "No load."]
    NoLoad = 0x0,
    #[doc = "Load."]
    Load = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Ldok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldok {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldok {
    #[inline(always)]
    fn from(val: u8) -> Ldok {
        Ldok::from_bits(val)
    }
}
impl From<Ldok> for u8 {
    #[inline(always)]
    fn from(val: Ldok) -> u8 {
        Ldok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nocomb {
    #[doc = "Combinational path."]
    Path = 0x0,
    #[doc = "No combinational path."]
    NoCombinationalPath = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Nocomb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nocomb {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nocomb {
    #[inline(always)]
    fn from(val: u8) -> Nocomb {
        Nocomb::from_bits(val)
    }
}
impl From<Nocomb> for u8 {
    #[inline(always)]
    fn from(val: Nocomb) -> u8 {
        Nocomb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Run {
    #[doc = "Stop."]
    Stop = 0x0,
    #[doc = "Run."]
    Run = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl Run {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Run {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Run {
    #[inline(always)]
    fn from(val: u8) -> Run {
        Run::from_bits(val)
    }
}
impl From<Run> for u8 {
    #[inline(always)]
    fn from(val: Run) -> u8 {
        Run::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlaEdga0 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlaEdga0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlaEdga0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlaEdga0 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlaEdga0 {
        SmcaptctrlaEdga0::from_bits(val)
    }
}
impl From<SmcaptctrlaEdga0> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlaEdga0) -> u8 {
        SmcaptctrlaEdga0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlaEdga1 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlaEdga1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlaEdga1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlaEdga1 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlaEdga1 {
        SmcaptctrlaEdga1::from_bits(val)
    }
}
impl From<SmcaptctrlaEdga1> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlaEdga1) -> u8 {
        SmcaptctrlaEdga1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlaInpSela {
    #[doc = "Input from PWM_A."]
    PwmA = 0x0,
    #[doc = "Input from PWM_X."]
    PwmX = 0x01,
}
impl SmcaptctrlaInpSela {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlaInpSela {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlaInpSela {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlaInpSela {
        SmcaptctrlaInpSela::from_bits(val)
    }
}
impl From<SmcaptctrlaInpSela> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlaInpSela) -> u8 {
        SmcaptctrlaInpSela::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlaOneshota {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Enabled."]
    Enabled = 0x01,
}
impl SmcaptctrlaOneshota {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlaOneshota {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlaOneshota {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlaOneshota {
        SmcaptctrlaOneshota::from_bits(val)
    }
}
impl From<SmcaptctrlaOneshota> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlaOneshota) -> u8 {
        SmcaptctrlaOneshota::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlbEdgb0 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlbEdgb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlbEdgb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlbEdgb0 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlbEdgb0 {
        SmcaptctrlbEdgb0::from_bits(val)
    }
}
impl From<SmcaptctrlbEdgb0> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlbEdgb0) -> u8 {
        SmcaptctrlbEdgb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlbEdgb1 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlbEdgb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlbEdgb1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlbEdgb1 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlbEdgb1 {
        SmcaptctrlbEdgb1::from_bits(val)
    }
}
impl From<SmcaptctrlbEdgb1> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlbEdgb1) -> u8 {
        SmcaptctrlbEdgb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlbInpSelb {
    #[doc = "Input from PWM_B."]
    PwmB = 0x0,
    #[doc = "Input from PWM_X."]
    PwmX = 0x01,
}
impl SmcaptctrlbInpSelb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlbInpSelb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlbInpSelb {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlbInpSelb {
        SmcaptctrlbInpSelb::from_bits(val)
    }
}
impl From<SmcaptctrlbInpSelb> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlbInpSelb) -> u8 {
        SmcaptctrlbInpSelb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlbOneshotb {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Enabled."]
    Enabled = 0x01,
}
impl SmcaptctrlbOneshotb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlbOneshotb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlbOneshotb {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlbOneshotb {
        SmcaptctrlbOneshotb::from_bits(val)
    }
}
impl From<SmcaptctrlbOneshotb> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlbOneshotb) -> u8 {
        SmcaptctrlbOneshotb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxEdgx0 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlxEdgx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxEdgx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxEdgx0 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxEdgx0 {
        SmcaptctrlxEdgx0::from_bits(val)
    }
}
impl From<SmcaptctrlxEdgx0> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxEdgx0) -> u8 {
        SmcaptctrlxEdgx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxEdgx1 {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Rising edge."]
    RisingEdge = 0x01,
    #[doc = "Falling edge."]
    FallingEdge = 0x02,
    #[doc = "Rising and falling edges."]
    RisingFallingEdge = 0x03,
}
impl SmcaptctrlxEdgx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxEdgx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxEdgx1 {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxEdgx1 {
        SmcaptctrlxEdgx1::from_bits(val)
    }
}
impl From<SmcaptctrlxEdgx1> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxEdgx1) -> u8 {
        SmcaptctrlxEdgx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxInpSelx {
    #[doc = "Input from PWM_X."]
    PwmX = 0x0,
    #[doc = "Input from PWM_B."]
    PwmB = 0x01,
}
impl SmcaptctrlxInpSelx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxInpSelx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxInpSelx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxInpSelx {
        SmcaptctrlxInpSelx::from_bits(val)
    }
}
impl From<SmcaptctrlxInpSelx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxInpSelx) -> u8 {
        SmcaptctrlxInpSelx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxOneshotx {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Enabled."]
    Enabled = 0x01,
}
impl SmcaptctrlxOneshotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxOneshotx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxOneshotx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxOneshotx {
        SmcaptctrlxOneshotx::from_bits(val)
    }
}
impl From<SmcaptctrlxOneshotx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxOneshotx) -> u8 {
        SmcaptctrlxOneshotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smctrl2ClkSel {
    #[doc = "IPBus clock."]
    IpBusClock = 0x0,
    #[doc = "External clock."]
    ExtClk = 0x01,
    #[doc = "Auxiliary clock."]
    AuxClk = 0x02,
    _RESERVED_3 = 0x03,
}
impl Smctrl2ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smctrl2ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smctrl2ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Smctrl2ClkSel {
        Smctrl2ClkSel::from_bits(val)
    }
}
impl From<Smctrl2ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Smctrl2ClkSel) -> u8 {
        Smctrl2ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smctrl2ForceSel {
    #[doc = "Software force is used."]
    Software = 0x0,
    #[doc = "Local reload signal is used."]
    LocalReloadSignal = 0x01,
    #[doc = "Local sync signal is used."]
    LocalSyncSignal = 0x02,
    #[doc = "System sync signal is used."]
    SystemSyncSignal = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Smctrl2ForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smctrl2ForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smctrl2ForceSel {
    #[inline(always)]
    fn from(val: u8) -> Smctrl2ForceSel {
        Smctrl2ForceSel::from_bits(val)
    }
}
impl From<Smctrl2ForceSel> for u8 {
    #[inline(always)]
    fn from(val: Smctrl2ForceSel) -> u8 {
        Smctrl2ForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smctrl2Indep {
    #[doc = "Complementary mode."]
    ComplementaryMode = 0x0,
    #[doc = "Independent mode."]
    IndependentMode = 0x01,
}
impl Smctrl2Indep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smctrl2Indep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smctrl2Indep {
    #[inline(always)]
    fn from(val: u8) -> Smctrl2Indep {
        Smctrl2Indep::from_bits(val)
    }
}
impl From<Smctrl2Indep> for u8 {
    #[inline(always)]
    fn from(val: Smctrl2Indep) -> u8 {
        Smctrl2Indep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smctrl2InitSel {
    #[doc = "Local sync signal."]
    LocalSync = 0x0,
    #[doc = "Master reload signal."]
    MasterReload = 0x01,
    #[doc = "Local reload signal."]
    LocalReload = 0x02,
    #[doc = "External sync signal."]
    ExternalSync = 0x03,
}
impl Smctrl2InitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smctrl2InitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smctrl2InitSel {
    #[inline(always)]
    fn from(val: u8) -> Smctrl2InitSel {
        Smctrl2InitSel::from_bits(val)
    }
}
impl From<Smctrl2InitSel> for u8 {
    #[inline(always)]
    fn from(val: Smctrl2InitSel) -> u8 {
        Smctrl2InitSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smctrl2ReloadSel {
    #[doc = "PWM reload signal."]
    PwmReloadSignal = 0x0,
    #[doc = "Local reload signal."]
    LocalReloadSignal = 0x01,
}
impl Smctrl2ReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smctrl2ReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smctrl2ReloadSel {
    #[inline(always)]
    fn from(val: u8) -> Smctrl2ReloadSel {
        Smctrl2ReloadSel::from_bits(val)
    }
}
impl From<Smctrl2ReloadSel> for u8 {
    #[inline(always)]
    fn from(val: Smctrl2ReloadSel) -> u8 {
        Smctrl2ReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlCompmode {
    #[doc = "Single edge."]
    SingleEdge = 0x0,
    #[doc = "Center aligned."]
    CenterAligned = 0x01,
}
impl SmctrlCompmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlCompmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlCompmode {
    #[inline(always)]
    fn from(val: u8) -> SmctrlCompmode {
        SmctrlCompmode::from_bits(val)
    }
}
impl From<SmctrlCompmode> for u8 {
    #[inline(always)]
    fn from(val: SmctrlCompmode) -> u8 {
        SmctrlCompmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlLdfq {
    #[doc = "Every PWM opportunity."]
    Everypwm = 0x0,
    #[doc = "Every 2 PWM opportunities."]
    Every2pwm = 0x01,
    #[doc = "Every 3 PWM opportunities."]
    Every3pwm = 0x02,
    #[doc = "Every 4 PWM opportunities."]
    Every4pwm = 0x03,
    #[doc = "Every 5 PWM opportunities."]
    Every5pwm = 0x04,
    #[doc = "Every 6 PWM opportunities."]
    Every6pwm = 0x05,
    #[doc = "Every 7 PWM opportunities."]
    Every7pwm = 0x06,
    #[doc = "Every 8 PWM opportunities."]
    Every8pwm = 0x07,
    #[doc = "Every 9 PWM opportunities."]
    Every9pwm = 0x08,
    #[doc = "Every 10 PWM opportunities."]
    Every10pwm = 0x09,
    #[doc = "Every 11 PWM opportunities."]
    Every11pwm = 0x0a,
    #[doc = "Every 12 PWM opportunities."]
    Every12pwm = 0x0b,
    #[doc = "Every 13 PWM opportunities."]
    Every13pwm = 0x0c,
    #[doc = "Every 14 PWM opportunities."]
    Every14pwm = 0x0d,
    #[doc = "Every 15 PWM opportunities."]
    Every15pwm = 0x0e,
    #[doc = "Every 16 PWM opportunities."]
    Every16pwm = 0x0f,
}
impl SmctrlLdfq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlLdfq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlLdfq {
    #[inline(always)]
    fn from(val: u8) -> SmctrlLdfq {
        SmctrlLdfq::from_bits(val)
    }
}
impl From<SmctrlLdfq> for u8 {
    #[inline(always)]
    fn from(val: SmctrlLdfq) -> u8 {
        SmctrlLdfq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlLdfreq {
    #[doc = "Load every half cycle period."]
    HalfCyclePeriod = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_f = 0x0f,
}
impl SmctrlLdfreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlLdfreq {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlLdfreq {
    #[inline(always)]
    fn from(val: u8) -> SmctrlLdfreq {
        SmctrlLdfreq::from_bits(val)
    }
}
impl From<SmctrlLdfreq> for u8 {
    #[inline(always)]
    fn from(val: SmctrlLdfreq) -> u8 {
        SmctrlLdfreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlLdmod {
    #[doc = "Reload next cycle."]
    ReloadNextCycle = 0x0,
    #[doc = "Reload next half cycle."]
    ReloadNextHalfCycle = 0x01,
}
impl SmctrlLdmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlLdmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlLdmod {
    #[inline(always)]
    fn from(val: u8) -> SmctrlLdmod {
        SmctrlLdmod::from_bits(val)
    }
}
impl From<SmctrlLdmod> for u8 {
    #[inline(always)]
    fn from(val: SmctrlLdmod) -> u8 {
        SmctrlLdmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlPrsc {
    #[doc = "1."]
    ClkDiv1 = 0x0,
    #[doc = "2."]
    ClkDiv2 = 0x01,
    #[doc = "4."]
    ClkDiv4 = 0x02,
    #[doc = "8."]
    ClkDiv8 = 0x03,
    #[doc = "16."]
    ClkDiv16 = 0x04,
    #[doc = "32."]
    ClkDiv32 = 0x05,
    #[doc = "64."]
    ClkDiv64 = 0x06,
    #[doc = "128."]
    ClkDiv128 = 0x07,
}
impl SmctrlPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlPrsc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlPrsc {
    #[inline(always)]
    fn from(val: u8) -> SmctrlPrsc {
        SmctrlPrsc::from_bits(val)
    }
}
impl From<SmctrlPrsc> for u8 {
    #[inline(always)]
    fn from(val: SmctrlPrsc) -> u8 {
        SmctrlPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmdmaenCaptde {
    #[doc = "Source is CMPF."]
    SourceIsCmpf = 0x0,
    #[doc = "Source is CFXF."]
    SourceIsCfxf = 0x01,
    #[doc = "Source is CFBF."]
    SourceIsCfbf = 0x02,
    #[doc = "Source is CFAF."]
    SourceIsCfaf = 0x03,
}
impl SmdmaenCaptde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmdmaenCaptde {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmdmaenCaptde {
    #[inline(always)]
    fn from(val: u8) -> SmdmaenCaptde {
        SmdmaenCaptde::from_bits(val)
    }
}
impl From<SmdmaenCaptde> for u8 {
    #[inline(always)]
    fn from(val: SmdmaenCaptde) -> u8 {
        SmdmaenCaptde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmdmaenFand {
    #[doc = "Disable AND."]
    DisableAnd = 0x0,
    #[doc = "Enable AND."]
    EnableAnd = 0x01,
}
impl SmdmaenFand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmdmaenFand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmdmaenFand {
    #[inline(always)]
    fn from(val: u8) -> SmdmaenFand {
        SmdmaenFand::from_bits(val)
    }
}
impl From<SmdmaenFand> for u8 {
    #[inline(always)]
    fn from(val: SmdmaenFand) -> u8 {
        SmdmaenFand::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmintenCmpie {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Enabled."]
    Enabled = 0x01,
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
impl SmintenCmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmintenCmpie {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmintenCmpie {
    #[inline(always)]
    fn from(val: u8) -> SmintenCmpie {
        SmintenCmpie::from_bits(val)
    }
}
impl From<SmintenCmpie> for u8 {
    #[inline(always)]
    fn from(val: SmintenCmpie) -> u8 {
        SmintenCmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmafs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    Logic0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    Logic1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    Tristated2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    Tristated3 = 0x03,
}
impl SmoctrlPwmafs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmafs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmafs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmafs {
        SmoctrlPwmafs::from_bits(val)
    }
}
impl From<SmoctrlPwmafs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmafs) -> u8 {
        SmoctrlPwmafs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmbfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    Logic0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    Logic1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    Tristated2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    Tristated3 = 0x03,
}
impl SmoctrlPwmbfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmbfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmbfs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmbfs {
        SmoctrlPwmbfs::from_bits(val)
    }
}
impl From<SmoctrlPwmbfs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmbfs) -> u8 {
        SmoctrlPwmbfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmoctrlPwmxfs {
    #[doc = "Output is forced to logic 0 state prior to consideration of output polarity control."]
    Logic0 = 0x0,
    #[doc = "Output is forced to logic 1 state prior to consideration of output polarity control."]
    Logic1 = 0x01,
    #[doc = "Output is put in a high-impedance state."]
    Tristated2 = 0x02,
    #[doc = "Output is put in a high-impedance state."]
    Tristated3 = 0x03,
}
impl SmoctrlPwmxfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmoctrlPwmxfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmoctrlPwmxfs {
    #[inline(always)]
    fn from(val: u8) -> SmoctrlPwmxfs {
        SmoctrlPwmxfs::from_bits(val)
    }
}
impl From<SmoctrlPwmxfs> for u8 {
    #[inline(always)]
    fn from(val: SmoctrlPwmxfs) -> u8 {
        SmoctrlPwmxfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smsel23 {
    #[doc = "Generated SMPWM23 signal used by the deadtime logic."]
    Smpwm23 = 0x0,
    #[doc = "Inverted generated SMPWM23 signal used by the deadtime logic."]
    InvertedSmpwm23 = 0x01,
    #[doc = "SWCOUT\\[SMOUT23\\] used by the deadtime logic."]
    Smout23 = 0x02,
    #[doc = "PWM_EXTA signal used by the deadtime logic."]
    PwmExta = 0x03,
}
impl Smsel23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smsel23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smsel23 {
    #[inline(always)]
    fn from(val: u8) -> Smsel23 {
        Smsel23::from_bits(val)
    }
}
impl From<Smsel23> for u8 {
    #[inline(always)]
    fn from(val: Smsel23) -> u8 {
        Smsel23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smsel45 {
    #[doc = "Generated SMPWM45 signal used by the deadtime logic."]
    Smpwm45 = 0x0,
    #[doc = "Inverted generated SMPWM45 signal used by the deadtime logic."]
    InvertedSmpwm45 = 0x01,
    #[doc = "SWCOUT\\[SMOUT45\\] used by the deadtime logic."]
    Smout45 = 0x02,
    #[doc = "Reserved."]
    Reserved3 = 0x03,
}
impl Smsel45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smsel45 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smsel45 {
    #[inline(always)]
    fn from(val: u8) -> Smsel45 {
        Smsel45::from_bits(val)
    }
}
impl From<Smsel45> for u8 {
    #[inline(always)]
    fn from(val: Smsel45) -> u8 {
        Smsel45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmstsCmpf {
    #[doc = "No match."]
    NoMatch = 0x0,
    #[doc = "Match."]
    Match = 0x01,
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
impl SmstsCmpf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmstsCmpf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmstsCmpf {
    #[inline(always)]
    fn from(val: u8) -> SmstsCmpf {
        SmstsCmpf::from_bits(val)
    }
}
impl From<SmstsCmpf> for u8 {
    #[inline(always)]
    fn from(val: SmstsCmpf) -> u8 {
        SmstsCmpf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StretchCntPrsc {
    #[doc = "Stretch prescaler divide by 1."]
    Div1 = 0x0,
    #[doc = "Stretch prescaler divide by 2."]
    Div2 = 0x01,
    #[doc = "Stretch prescaler divide by 4."]
    Div4 = 0x02,
    #[doc = "Stretch prescaler divide by 8."]
    Div8 = 0x03,
}
impl StretchCntPrsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StretchCntPrsc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StretchCntPrsc {
    #[inline(always)]
    fn from(val: u8) -> StretchCntPrsc {
        StretchCntPrsc::from_bits(val)
    }
}
impl From<StretchCntPrsc> for u8 {
    #[inline(always)]
    fn from(val: StretchCntPrsc) -> u8 {
        StretchCntPrsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrprot {
    #[doc = "Write protect disabled."]
    WriteProtectDisabled = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Write protect enabled for all submodules."]
    WriteProtectEnabledAllSubmodules = 0x02,
    _RESERVED_3 = 0x03,
}
impl Wrprot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrprot {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrprot {
    #[inline(always)]
    fn from(val: u8) -> Wrprot {
        Wrprot::from_bits(val)
    }
}
impl From<Wrprot> for u8 {
    #[inline(always)]
    fn from(val: Wrprot) -> u8 {
        Wrprot::to_bits(val)
    }
}
