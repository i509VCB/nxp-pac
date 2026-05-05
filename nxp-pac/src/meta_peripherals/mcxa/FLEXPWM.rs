#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "PWM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm {
    ptr: *mut u8,
}
unsafe impl Send for Flexpwm {}
unsafe impl Sync for Flexpwm {}
impl Flexpwm {
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
    pub const fn sm0cnt(self) -> crate::pac::common::Reg<Smcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm0init(self) -> crate::pac::common::Reg<Sminit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm0ctrl2(self) -> crate::pac::common::Reg<Smctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm0ctrl(self) -> crate::pac::common::Reg<Smctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm0val(self, n: usize) -> crate::pac::common::Reg<Smval, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize + n * 4usize) as _)
        }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm0octrl(self) -> crate::pac::common::Reg<Smoctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm0sts(self) -> crate::pac::common::Reg<Smsts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm0inten(self) -> crate::pac::common::Reg<Sminten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm0dmaen(self) -> crate::pac::common::Reg<Smdmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm0tctrl(self) -> crate::pac::common::Reg<Smtctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm0dismap0(self) -> crate::pac::common::Reg<Smdismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm0dtcnt0(self) -> crate::pac::common::Reg<Smdtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm0dtcnt1(self) -> crate::pac::common::Reg<Smdtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm0captctrlx(
        self,
    ) -> crate::pac::common::Reg<Smcaptctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm0captcompx(
        self,
    ) -> crate::pac::common::Reg<Smcaptcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm0cval0(self) -> crate::pac::common::Reg<Smcval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval0cyc(self) -> crate::pac::common::Reg<Smcval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm0cval1(self) -> crate::pac::common::Reg<Smcval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval1cyc(self) -> crate::pac::common::Reg<Smcval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm0captfiltx(
        self,
    ) -> crate::pac::common::Reg<Smcaptfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm1cnt(self) -> crate::pac::common::Reg<Smcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm1init(self) -> crate::pac::common::Reg<Sminit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm1ctrl2(self) -> crate::pac::common::Reg<Smctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm1ctrl(self) -> crate::pac::common::Reg<Smctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm1val(self, n: usize) -> crate::pac::common::Reg<Smval, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize + n * 4usize) as _)
        }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm1octrl(self) -> crate::pac::common::Reg<Smoctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm1sts(self) -> crate::pac::common::Reg<Smsts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm1inten(self) -> crate::pac::common::Reg<Sminten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm1dmaen(self) -> crate::pac::common::Reg<Smdmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm1tctrl(self) -> crate::pac::common::Reg<Smtctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm1dismap0(self) -> crate::pac::common::Reg<Smdismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm1dtcnt0(self) -> crate::pac::common::Reg<Smdtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm1dtcnt1(self) -> crate::pac::common::Reg<Smdtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm1captctrlx(
        self,
    ) -> crate::pac::common::Reg<Smcaptctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm1captcompx(
        self,
    ) -> crate::pac::common::Reg<Smcaptcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm1cval0(self) -> crate::pac::common::Reg<Smcval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval0cyc(self) -> crate::pac::common::Reg<Smcval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm1cval1(self) -> crate::pac::common::Reg<Smcval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval1cyc(self) -> crate::pac::common::Reg<Smcval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm1phasedly(self) -> crate::pac::common::Reg<Smphasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm1captfiltx(
        self,
    ) -> crate::pac::common::Reg<Smcaptfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm2cnt(self) -> crate::pac::common::Reg<Smcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm2init(self) -> crate::pac::common::Reg<Sminit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm2ctrl2(self) -> crate::pac::common::Reg<Smctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm2ctrl(self) -> crate::pac::common::Reg<Smctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm2val(self, n: usize) -> crate::pac::common::Reg<Smval, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize + n * 4usize) as _)
        }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm2octrl(self) -> crate::pac::common::Reg<Smoctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm2sts(self) -> crate::pac::common::Reg<Smsts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm2inten(self) -> crate::pac::common::Reg<Sminten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm2dmaen(self) -> crate::pac::common::Reg<Smdmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm2tctrl(self) -> crate::pac::common::Reg<Smtctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm2dismap0(self) -> crate::pac::common::Reg<Smdismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm2dtcnt0(self) -> crate::pac::common::Reg<Smdtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm2dtcnt1(self) -> crate::pac::common::Reg<Smdtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm2captctrlx(
        self,
    ) -> crate::pac::common::Reg<Smcaptctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm2captcompx(
        self,
    ) -> crate::pac::common::Reg<Smcaptcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfeusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm2cval0(self) -> crate::pac::common::Reg<Smcval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval0cyc(self) -> crate::pac::common::Reg<Smcval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm2cval1(self) -> crate::pac::common::Reg<Smcval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval1cyc(self) -> crate::pac::common::Reg<Smcval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm2phasedly(self) -> crate::pac::common::Reg<Smphasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm2captfiltx(
        self,
    ) -> crate::pac::common::Reg<Smcaptfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm3cnt(self) -> crate::pac::common::Reg<Smcnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm3init(self) -> crate::pac::common::Reg<Sminit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm3ctrl2(self) -> crate::pac::common::Reg<Smctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm3ctrl(self) -> crate::pac::common::Reg<Smctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm3val(self, n: usize) -> crate::pac::common::Reg<Smval, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize + n * 4usize) as _)
        }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm3octrl(self) -> crate::pac::common::Reg<Smoctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm3sts(self) -> crate::pac::common::Reg<Smsts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm3inten(self) -> crate::pac::common::Reg<Sminten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm3dmaen(self) -> crate::pac::common::Reg<Smdmaen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm3tctrl(self) -> crate::pac::common::Reg<Smtctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm3dismap0(self) -> crate::pac::common::Reg<Smdismap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm3dtcnt0(self) -> crate::pac::common::Reg<Smdtcnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm3dtcnt1(self) -> crate::pac::common::Reg<Smdtcnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm3captctrlx(
        self,
    ) -> crate::pac::common::Reg<Smcaptctrlx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm3captcompx(
        self,
    ) -> crate::pac::common::Reg<Smcaptcompx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm3cval0(self) -> crate::pac::common::Reg<Smcval0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm3cval0cyc(self) -> crate::pac::common::Reg<Smcval0cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm3cval1(self) -> crate::pac::common::Reg<Smcval1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm3cval1cyc(self) -> crate::pac::common::Reg<Smcval1cyc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm3phasedly(self) -> crate::pac::common::Reg<Smphasedly, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm3captfiltx(
        self,
    ) -> crate::pac::common::Reg<Smcaptfiltx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017eusize) as _) }
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
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel45(&self) -> Smsel45 {
        let val = (self.0 >> 0usize) & 0x03;
        Smsel45::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel45(&mut self, val: Smsel45) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0sel23(&self) -> Smsel23 {
        let val = (self.0 >> 2usize) & 0x03;
        Smsel23::from_bits(val as u8)
    }
    #[doc = "Submodule 0 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm0sel23(&mut self, val: Smsel23) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel45(&self) -> Smsel45 {
        let val = (self.0 >> 4usize) & 0x03;
        Smsel45::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel45(&mut self, val: Smsel45) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1sel23(&self) -> Smsel23 {
        let val = (self.0 >> 6usize) & 0x03;
        Smsel23::from_bits(val as u8)
    }
    #[doc = "Submodule 1 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm1sel23(&mut self, val: Smsel23) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel45(&self) -> Smsel45 {
        let val = (self.0 >> 8usize) & 0x03;
        Smsel45::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel45(&mut self, val: Smsel45) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2sel23(&self) -> Smsel23 {
        let val = (self.0 >> 10usize) & 0x03;
        Smsel23::from_bits(val as u8)
    }
    #[doc = "Submodule 2 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm2sel23(&mut self, val: Smsel23) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Submodule 3 PWM45 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel45(&self) -> Smsel45 {
        let val = (self.0 >> 12usize) & 0x03;
        Smsel45::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM45 Control Select."]
    #[inline(always)]
    pub const fn set_sm3sel45(&mut self, val: Smsel45) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Submodule 3 PWM23 Control Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3sel23(&self) -> Smsel23 {
        let val = (self.0 >> 14usize) & 0x03;
        Smsel23::from_bits(val as u8)
    }
    #[doc = "Submodule 3 PWM23 Control Select."]
    #[inline(always)]
    pub const fn set_sm3sel23(&mut self, val: Smsel23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
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
            .field("sm0sel45", &self.sm0sel45())
            .field("sm0sel23", &self.sm0sel23())
            .field("sm1sel45", &self.sm1sel45())
            .field("sm1sel23", &self.sm1sel23())
            .field("sm2sel45", &self.sm2sel45())
            .field("sm2sel23", &self.sm2sel23())
            .field("sm3sel45", &self.sm3sel45())
            .field("sm3sel23", &self.sm3sel23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dtsrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dtsrcsel {{ sm0sel45: {:?}, sm0sel23: {:?}, sm1sel45: {:?}, sm1sel23: {:?}, sm2sel45: {:?}, sm2sel23: {:?}, sm3sel45: {:?}, sm3sel23: {:?} }}",
            self.sm0sel45(),
            self.sm0sel23(),
            self.sm1sel45(),
            self.sm1sel23(),
            self.sm2sel45(),
            self.sm2sel23(),
            self.sm3sel45(),
            self.sm3sel23()
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
    pub const fn edgx0(&self) -> SmcaptctrlxEdgx {
        let val = (self.0 >> 2usize) & 0x03;
        SmcaptctrlxEdgx::from_bits(val as u8)
    }
    #[doc = "Edge X 0."]
    #[inline(always)]
    pub const fn set_edgx0(&mut self, val: SmcaptctrlxEdgx) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Edge X 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edgx1(&self) -> SmcaptctrlxEdgx {
        let val = (self.0 >> 4usize) & 0x03;
        SmcaptctrlxEdgx::from_bits(val as u8)
    }
    #[doc = "Edge X 1."]
    #[inline(always)]
    pub const fn set_edgx1(&mut self, val: SmcaptctrlxEdgx) {
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
    pub const fn clk_sel(&self) -> SmctrlClkSel {
        let val = (self.0 >> 0usize) & 0x03;
        SmctrlClkSel::from_bits(val as u8)
    }
    #[doc = "Clock Source Select."]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: SmctrlClkSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Reload Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn reload_sel(&self) -> SmctrlReloadSel {
        let val = (self.0 >> 2usize) & 0x01;
        SmctrlReloadSel::from_bits(val as u8)
    }
    #[doc = "Reload Source Select."]
    #[inline(always)]
    pub const fn set_reload_sel(&mut self, val: SmctrlReloadSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Force Select."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sel(&self) -> SmctrlForceSel {
        let val = (self.0 >> 3usize) & 0x07;
        SmctrlForceSel::from_bits(val as u8)
    }
    #[doc = "Force Select."]
    #[inline(always)]
    pub const fn set_force_sel(&mut self, val: SmctrlForceSel) {
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
    pub const fn init_sel(&self) -> SmctrlInitSel {
        let val = (self.0 >> 8usize) & 0x03;
        SmctrlInitSel::from_bits(val as u8)
    }
    #[doc = "Initialization Control Select."]
    #[inline(always)]
    pub const fn set_init_sel(&mut self, val: SmctrlInitSel) {
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
    pub const fn indep(&self) -> SmctrlIndep {
        let val = (self.0 >> 13usize) & 0x01;
        SmctrlIndep::from_bits(val as u8)
    }
    #[doc = "Independent or Complementary Pair Operation."]
    #[inline(always)]
    pub const fn set_indep(&mut self, val: SmctrlIndep) {
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
            "Smdmaen {{ cx0de: {=bool:?}, cx1de: {=bool:?}, captde: {:?}, fand: {:?}, valde: {=bool:?} }}",
            self.cx0de(),
            self.cx1de(),
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
    #[doc = "Capture X 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx0ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx0ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cx1ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture X 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_cx1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
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
            .field("cx0ie", &self.cx0ie())
            .field("cx1ie", &self.cx1ie())
            .field("rie", &self.rie())
            .field("reie", &self.reie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sminten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sminten {{ cmpie: {:?}, cx0ie: {=bool:?}, cx1ie: {=bool:?}, rie: {=bool:?}, reie: {=bool:?} }}",
            self.cmpie(),
            self.cx0ie(),
            self.cx1ie(),
            self.rie(),
            self.reie()
        )
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
    #[doc = "Capture Flag X0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X0."]
    #[inline(always)]
    pub const fn set_cfx0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Capture Flag X1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfx1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Flag X1."]
    #[inline(always)]
    pub const fn set_cfx1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Reload Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rf(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Flag."]
    #[inline(always)]
    pub const fn set_rf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
    }
    #[doc = "Reload Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reload Error Flag."]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
    }
    #[doc = "Registers Updated Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ruf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Registers Updated Flag."]
    #[inline(always)]
    pub const fn set_ruf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
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
            .field("cfx0", &self.cfx0())
            .field("cfx1", &self.cfx1())
            .field("rf", &self.rf())
            .field("ref_", &self.ref_())
            .field("ruf", &self.ruf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smsts {{ cmpf: {:?}, cfx0: {=bool:?}, cfx1: {=bool:?}, rf: {=bool:?}, ref_: {=bool:?}, ruf: {=bool:?} }}",
            self.cmpf(),
            self.cfx0(),
            self.cfx1(),
            self.rf(),
            self.ref_(),
            self.ruf()
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
    pub const fn out_trig_en(&self) -> SmtctrlOutTrigEn {
        let val = (self.0 >> 0usize) & 0x3f;
        SmtctrlOutTrigEn::from_bits(val as u8)
    }
    #[doc = "Output Trigger Enables."]
    #[inline(always)]
    pub const fn set_out_trig_en(&mut self, val: SmtctrlOutTrigEn) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u16) & 0x3f) << 0usize);
    }
    #[doc = "Trigger Frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn trgfrq(&self) -> SmtctrlTrgfrq {
        let val = (self.0 >> 12usize) & 0x01;
        SmtctrlTrgfrq::from_bits(val as u8)
    }
    #[doc = "Trigger Frequency."]
    #[inline(always)]
    pub const fn set_trgfrq(&mut self, val: SmtctrlTrgfrq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u16) & 0x01) << 12usize);
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwbot1(&self) -> SmtctrlPwbot {
        let val = (self.0 >> 14usize) & 0x01;
        SmtctrlPwbot::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 1 Source Select."]
    #[inline(always)]
    pub const fn set_pwbot1(&mut self, val: SmtctrlPwbot) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u16) & 0x01) << 14usize);
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwaot0(&self) -> SmtctrlPwaot {
        let val = (self.0 >> 15usize) & 0x01;
        SmtctrlPwaot::from_bits(val as u8)
    }
    #[doc = "Mux Output Trigger 0 Source Select."]
    #[inline(always)]
    pub const fn set_pwaot0(&mut self, val: SmtctrlPwaot) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u16) & 0x01) << 15usize);
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
            .field("trgfrq", &self.trgfrq())
            .field("pwbot1", &self.pwbot1())
            .field("pwaot0", &self.pwaot0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smtctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smtctrl {{ out_trig_en: {:?}, trgfrq: {:?}, pwbot1: {:?}, pwaot0: {:?} }}",
            self.out_trig_en(),
            self.trgfrq(),
            self.pwbot1(),
            self.pwaot0()
        )
    }
}
#[doc = "Value Register 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smval(pub u16);
impl Smval {
    #[doc = "Value 0."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value 0."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Smval {
    #[inline(always)]
    fn default() -> Smval {
        Smval(0)
    }
}
impl core::fmt::Debug for Smval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smval").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Smval {{ val: {=u16:?} }}", self.val())
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
    pub const fn sm0out45(&self) -> Smout {
        let val = (self.0 >> 0usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm0out45(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm0out23(&self) -> Smout {
        let val = (self.0 >> 1usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 0 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm0out23(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out45(&self) -> Smout {
        let val = (self.0 >> 2usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm1out45(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm1out23(&self) -> Smout {
        let val = (self.0 >> 3usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 1 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm1out23(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out45(&self) -> Smout {
        let val = (self.0 >> 4usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm2out45(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm2out23(&self) -> Smout {
        let val = (self.0 >> 5usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 2 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm2out23(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u16) & 0x01) << 5usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out45(&self) -> Smout {
        let val = (self.0 >> 6usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 45."]
    #[inline(always)]
    pub const fn set_sm3out45(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u16) & 0x01) << 6usize);
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3out23(&self) -> Smout {
        let val = (self.0 >> 7usize) & 0x01;
        Smout::from_bits(val as u8)
    }
    #[doc = "Submodule 3 Software Controlled Output 23."]
    #[inline(always)]
    pub const fn set_sm3out23(&mut self, val: Smout) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u16) & 0x01) << 7usize);
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
            "Swcout {{ sm0out45: {:?}, sm0out23: {:?}, sm1out45: {:?}, sm1out23: {:?}, sm2out45: {:?}, sm2out23: {:?}, sm3out45: {:?}, sm3out23: {:?} }}",
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
    #[doc = "Manual fault clearing. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared. This is further controlled by FCTRL\\[FSAFE\\]."]
    Manual = 0x0,
    #[doc = "Automatic fault clearing. PWM outputs disabled by this fault are enabled when FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFLAGx\\]. If neither FFULL nor FHALF is set, then the fault condition cannot be cleared."]
    Automatic = 0x01,
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
    #[doc = "No fault on the FAULTx pin."]
    NoFlag = 0x0,
    #[doc = "Fault on the FAULTx pin."]
    Flag = 0x01,
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
    #[doc = "PWM outputs are not re-enabled at the start of a full cycle."]
    PwmOutputsNotReenabled = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a full cycle."]
    PwmOutputsReenabled = 0x01,
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
    #[doc = "PWM outputs are not re-enabled at the start of a half cycle."]
    PwmOutputsNotReenabled = 0x0,
    #[doc = "PWM outputs are re-enabled at the start of a half cycle (as defined by VAL0)."]
    PwmOutputsReenabled = 0x01,
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
    #[doc = "FAULTx CPU interrupt requests disabled."]
    Disabled = 0x0,
    #[doc = "FAULTx CPU interrupt requests enabled."]
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
    #[doc = "A logic 0 on the fault input indicates a fault condition."]
    Logic0 = 0x0,
    #[doc = "A logic 1 on the fault input indicates a fault condition."]
    Logic1 = 0x01,
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
    #[doc = "Normal mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\] without regard to the state of FSTS\\[FFPINx\\]. If neither FHALF nor FFULL is set, then the fault condition cannot be cleared. The PWM outputs disabled by this fault input will not be re-enabled until the actual FAULTx input signal de-asserts since the fault input will combinationally disable the PWM outputs (as programmed in DISMAPn)."]
    Normal = 0x0,
    #[doc = "Safe mode. PWM outputs disabled by this fault are not enabled until FSTS\\[FFLAGx\\] is clear and FSTS\\[FFPINx\\] is clear at the start of a half cycle or full cycle depending on the states of FSTS\\[FHALF\\] and FSTS\\[FFULL\\]. If neither FHLAF nor FFULL is set, then the fault condition cannot be cleared."]
    Safe = 0x01,
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
    #[doc = "PWM23 is used to generate complementary PWM pair in the corresponding submodule."]
    Pwm23 = 0x0,
    #[doc = "PWM45 is used to generate complementary PWM pair in the corresponding submodule."]
    Pwm45 = 0x01,
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
    #[doc = "Do not load new values."]
    Disabled = 0x0,
    #[doc = "Load prescaler, modulus, and PWM values of the corresponding submodule."]
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
    #[doc = "There is a combinational link from the fault inputs to the PWM outputs. The fault inputs are combined with the filtered and latched fault signals to disable the PWM outputs."]
    Enabled = 0x0,
    #[doc = "The direct combinational path from the fault inputs to the PWM outputs is disabled and the filtered and latched fault signals are used to disable the PWM outputs."]
    Disabled = 0x01,
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
    #[doc = "PWM counter is stopped, but PWM outputs hold the current state."]
    Disabled = 0x0,
    #[doc = "PWM counter is started in the corresponding submodule."]
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
pub enum SmcaptctrlxEdgx {
    #[doc = "Disabled."]
    Disabled = 0x0,
    #[doc = "Capture falling edges."]
    FallingEdge = 0x01,
    #[doc = "Capture rising edges."]
    RisingEdge = 0x02,
    #[doc = "Capture any edge."]
    AnyEdge = 0x03,
}
impl SmcaptctrlxEdgx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmcaptctrlxEdgx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmcaptctrlxEdgx {
    #[inline(always)]
    fn from(val: u8) -> SmcaptctrlxEdgx {
        SmcaptctrlxEdgx::from_bits(val)
    }
}
impl From<SmcaptctrlxEdgx> for u8 {
    #[inline(always)]
    fn from(val: SmcaptctrlxEdgx) -> u8 {
        SmcaptctrlxEdgx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcaptctrlxInpSelx {
    #[doc = "Raw PWM_X input signal selected as source."]
    PwmX = 0x0,
    #[doc = "Edge Counter."]
    EdgeCounter = 0x01,
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
    #[doc = "Free Running."]
    FreeRunning = 0x0,
    #[doc = "One Shot."]
    OneShot = 0x01,
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
pub enum SmctrlClkSel {
    #[doc = "The IPBus clock is used as the clock for the local prescaler and counter."]
    Ipbus = 0x0,
    #[doc = "EXT_CLK is used as the clock for the local prescaler and counter."]
    ExtClk = 0x01,
    #[doc = "Submodule 0's clock (AUX_CLK) is used as the source clock for the local prescaler and counter. This setting should not be used in submodule 0 as it forces the clock to logic 0."]
    AuxClk = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmctrlClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlClkSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlClkSel {
        SmctrlClkSel::from_bits(val)
    }
}
impl From<SmctrlClkSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlClkSel) -> u8 {
        SmctrlClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlCompmode {
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to\" method. This means that PWM edges are only produced when the counter is equal to one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period maintains this state until a match with VAL3 clears the output in the following period."]
    EqualTo = 0x0,
    #[doc = "The VAL* registers and the PWM counter are compared using an \"equal to or greater than\" method. This means that PWM edges are produced when the counter is equal to or greater than one of the VAL* register values. This implies that a PWM_A output that is high at the end of a period could go low at the start of the next period if the starting counter value is greater than (but not necessarily equal to) the new VAL3 value."]
    EqualToOrGreaterThan = 0x01,
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
pub enum SmctrlForceSel {
    #[doc = "The local force signal, CTRL2\\[FORCE\\], from this submodule is used to force updates."]
    Local = 0x0,
    #[doc = "The master force signal from submodule 0 is used to force updates. This setting should not be used in submodule 0 as it holds the FORCE OUTPUT signal to logic 0."]
    Master = 0x01,
    #[doc = "The local reload signal from this submodule is used to force updates without regard to the state of LDOK."]
    LocalReload = 0x02,
    #[doc = "The master reload signal from submodule0 is used to force updates if LDOK is set. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MasterReload = 0x03,
    #[doc = "The local sync signal from this submodule is used to force updates."]
    LocalSync = 0x04,
    #[doc = "The master sync signal from submodule0 is used to force updates. This setting should not be used in submodule0 as it holds the FORCE OUTPUT signal to logic 0."]
    MasterSync = 0x05,
    #[doc = "The external force signal, EXT_FORCE, from outside the PWM module causes updates."]
    ExtForce = 0x06,
    #[doc = "The external sync signal, EXT_SYNC, from outside the PWM module causes updates."]
    ExtSync = 0x07,
}
impl SmctrlForceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlForceSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlForceSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlForceSel {
        SmctrlForceSel::from_bits(val)
    }
}
impl From<SmctrlForceSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlForceSel) -> u8 {
        SmctrlForceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlIndep {
    #[doc = "PWM_A and PWM_B form a complementary PWM pair."]
    Complementary = 0x0,
    #[doc = "PWM_A and PWM_B outputs are independent PWMs."]
    Independent = 0x01,
}
impl SmctrlIndep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlIndep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlIndep {
    #[inline(always)]
    fn from(val: u8) -> SmctrlIndep {
        SmctrlIndep::from_bits(val)
    }
}
impl From<SmctrlIndep> for u8 {
    #[inline(always)]
    fn from(val: SmctrlIndep) -> u8 {
        SmctrlIndep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmctrlInitSel {
    #[doc = "Local sync (PWM_X) causes initialization."]
    PwmX = 0x0,
    #[doc = "Master reload from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0. The submodule counter will only re-initialize when a master reload occurs."]
    MasterReload = 0x01,
    #[doc = "Master sync from submodule 0 causes initialization. This setting should not be used in submodule 0 as it forces the INIT signal to logic 0."]
    MasterSync = 0x02,
    #[doc = "EXT_SYNC causes initialization."]
    ExtSync = 0x03,
}
impl SmctrlInitSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlInitSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlInitSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlInitSel {
        SmctrlInitSel::from_bits(val)
    }
}
impl From<SmctrlInitSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlInitSel) -> u8 {
        SmctrlInitSel::to_bits(val)
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
pub enum SmctrlLdmod {
    #[doc = "Buffered registers of this submodule are loaded and take effect at the next PWM reload if MCTRL\\[LDOK\\] is set."]
    NextPwmReload = 0x0,
    #[doc = "Buffered registers of this submodule are loaded and take effect immediately upon MCTRL\\[LDOK\\] being set. In this case, it is not necessary to set CTRL\\[FULL\\] or CTRL\\[HALF\\]."]
    MtctrlLdokSet = 0x01,
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
    #[doc = "Prescaler 1."]
    One = 0x0,
    #[doc = "Prescaler 2."]
    Two = 0x01,
    #[doc = "Prescaler 4."]
    Four = 0x02,
    #[doc = "Prescaler 8."]
    Eight = 0x03,
    #[doc = "Prescaler 16."]
    Sixteen = 0x04,
    #[doc = "Prescaler 32."]
    Thirtytwo = 0x05,
    #[doc = "Prescaler 64."]
    Sixtyfour = 0x06,
    #[doc = "Prescaler 128."]
    Hundredtwentyeight = 0x07,
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
pub enum SmctrlReloadSel {
    #[doc = "The local RELOAD signal is used to reload registers."]
    Local = 0x0,
    #[doc = "The master RELOAD signal (from submodule 0) is used to reload registers. This setting should not be used in submodule 0 as it forces the RELOAD signal to logic 0."]
    Master = 0x01,
}
impl SmctrlReloadSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmctrlReloadSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmctrlReloadSel {
    #[inline(always)]
    fn from(val: u8) -> SmctrlReloadSel {
        SmctrlReloadSel::from_bits(val)
    }
}
impl From<SmctrlReloadSel> for u8 {
    #[inline(always)]
    fn from(val: SmctrlReloadSel) -> u8 {
        SmctrlReloadSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmdmaenCaptde {
    #[doc = "Read DMA requests disabled."]
    Disabled = 0x0,
    #[doc = "Exceeding a FIFO watermark sets the DMA read request. This requires at least one of DMAEN\\[CA1DE\\], DMAEN\\[CA0DE\\], DMAEN\\[CB1DE\\], DMAEN\\[CB0DE\\], DMAEN\\[CX1DE\\], or DMAEN\\[CX0DE\\] to be set to determine which watermark(s) the DMA request is sensitive."]
    Exceedfifo = 0x01,
    #[doc = "A local synchronization (VAL1 matches counter) sets the read DMA request."]
    LocalSync = 0x02,
    #[doc = "A local reload (STS\\[RF\\] being set) sets the read DMA request."]
    LocalReload = 0x03,
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
    #[doc = "Selected FIFO watermarks are OR'ed together."]
    Or = 0x0,
    #[doc = "Selected FIFO watermarks are AND'ed together."]
    And = 0x01,
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
    #[doc = "The corresponding STS\\[CMPF\\] bit will not cause an interrupt request."]
    Disabled = 0x0,
    #[doc = "The corresponding STS\\[CMPF\\] bit will cause an interrupt request."]
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
pub enum Smout {
    #[doc = "A logic 0 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    Logic0 = 0x0,
    #[doc = "A logic 1 is supplied to the deadtime generator of submodule 0 instead of PWM23."]
    Logic1 = 0x01,
}
impl Smout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smout {
    #[inline(always)]
    fn from(val: u8) -> Smout {
        Smout::from_bits(val)
    }
}
impl From<Smout> for u8 {
    #[inline(always)]
    fn from(val: Smout) -> u8 {
        Smout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smsel23 {
    #[doc = "Generated SM0PWM23 signal used by the deadtime logic."]
    pwm = 0x0,
    #[doc = "Inverted generated SM0PWM23 signal used by the deadtime logic."]
    Invertedpwm = 0x01,
    #[doc = "SWCOUT\\[SM0OUT23\\] used by the deadtime logic."]
    out = 0x02,
    #[doc = "PWM0_EXTA signal used by the deadtime logic."]
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
    #[doc = "Generated SM0PWM45 signal used by the deadtime logic."]
    pwm = 0x0,
    #[doc = "Inverted generated SM0PWM45 signal used by the deadtime logic."]
    Invertedpwm = 0x01,
    #[doc = "SWCOUT\\[SM0OUT45\\] used by the deadtime logic."]
    out = 0x02,
    _RESERVED_3 = 0x03,
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
    #[doc = "No compare event has occurred for a particular VALx value."]
    NoEvent = 0x0,
    #[doc = "A compare event has occurred for a particular VALx value."]
    Event = 0x01,
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
pub enum SmtctrlOutTrigEn {
    _RESERVED_0 = 0x0,
    #[doc = "PWM_OUT_TRIG0 will set when the counter value matches the VAL0 value."]
    Val0 = 0x01,
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
impl SmtctrlOutTrigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlOutTrigEn {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlOutTrigEn {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlOutTrigEn {
        SmtctrlOutTrigEn::from_bits(val)
    }
}
impl From<SmtctrlOutTrigEn> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlOutTrigEn) -> u8 {
        SmtctrlOutTrigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlPwaot {
    #[doc = "Route the PWM_OUT_TRIG0 signal to PWM_MUX_TRIG0 port."]
    PwmOutTrig0Signal = 0x0,
    #[doc = "Route the PWM_A output to the PWM_MUX_TRIG0 port."]
    PwmaOutput = 0x01,
}
impl SmtctrlPwaot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlPwaot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlPwaot {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlPwaot {
        SmtctrlPwaot::from_bits(val)
    }
}
impl From<SmtctrlPwaot> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlPwaot) -> u8 {
        SmtctrlPwaot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlPwbot {
    #[doc = "Route the PWM_OUT_TRIG1 signal to PWM_MUX_TRIG1 port."]
    PwmOutTrig1Signal = 0x0,
    #[doc = "Route the PWM_B output to the PWM_MUX_TRIG1 port."]
    PwmbOutput = 0x01,
}
impl SmtctrlPwbot {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlPwbot {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlPwbot {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlPwbot {
        SmtctrlPwbot::from_bits(val)
    }
}
impl From<SmtctrlPwbot> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlPwbot) -> u8 {
        SmtctrlPwbot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmtctrlTrgfrq {
    #[doc = "Trigger outputs are generated during every PWM period even if the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    Everypwm = 0x0,
    #[doc = "Trigger outputs are generated only during the final PWM period prior to a reload opportunity when the PWM is not reloaded every period due to CTRL\\[LDFQ\\] being non-zero."]
    Finalpwm = 0x01,
}
impl SmtctrlTrgfrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmtctrlTrgfrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmtctrlTrgfrq {
    #[inline(always)]
    fn from(val: u8) -> SmtctrlTrgfrq {
        SmtctrlTrgfrq::from_bits(val)
    }
}
impl From<SmtctrlTrgfrq> for u8 {
    #[inline(always)]
    fn from(val: SmtctrlTrgfrq) -> u8 {
        SmtctrlTrgfrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StretchCntPrsc {
    #[doc = "Stretch count is zero, no stretch."]
    Disabled = 0x0,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 2 IPBus clock period."]
    Enabled = 0x01,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 4 IPBus clock period."]
    DisabledLocked = 0x02,
    #[doc = "Stretch mux0_trig/mux1_trig/out0_trig/out1_trig/pwma_trig/pwmb_trig for 8 IPBus clock period."]
    EnabledLocked = 0x03,
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
    #[doc = "Write protection off (default)."]
    Disabled = 0x0,
    #[doc = "Write protection on."]
    Enabled = 0x01,
    #[doc = "Write protection off and locked until chip reset."]
    DisabledLocked = 0x02,
    #[doc = "Write protection on and locked until chip reset."]
    EnabledLocked = 0x03,
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
