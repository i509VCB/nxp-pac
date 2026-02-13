#[doc = "PWM"]
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
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm0cnt(self) -> crate::common::Reg<regs::Smcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm0init(self) -> crate::common::Reg<regs::Sminit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm0ctrl2(self) -> crate::common::Reg<regs::Smctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm0ctrl(self) -> crate::common::Reg<regs::Smctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm0val(self, n: usize) -> crate::common::Reg<regs::Smval, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize + n * 4usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm0octrl(self) -> crate::common::Reg<regs::Smoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm0sts(self) -> crate::common::Reg<regs::Smsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm0inten(self) -> crate::common::Reg<regs::Sminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm0dmaen(self) -> crate::common::Reg<regs::Smdmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm0tctrl(self) -> crate::common::Reg<regs::Smtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm0dismap0(self) -> crate::common::Reg<regs::Smdismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm0dtcnt0(self) -> crate::common::Reg<regs::Smdtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm0dtcnt1(self) -> crate::common::Reg<regs::Smdtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm0captctrlx(self) -> crate::common::Reg<regs::Smcaptctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm0captcompx(self) -> crate::common::Reg<regs::Smcaptcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm0cval0(self) -> crate::common::Reg<regs::Smcval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval0cyc(self) -> crate::common::Reg<regs::Smcval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm0cval1(self) -> crate::common::Reg<regs::Smcval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval1cyc(self) -> crate::common::Reg<regs::Smcval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm0captfiltx(self) -> crate::common::Reg<regs::Smcaptfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm1cnt(self) -> crate::common::Reg<regs::Smcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm1init(self) -> crate::common::Reg<regs::Sminit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm1ctrl2(self) -> crate::common::Reg<regs::Smctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm1ctrl(self) -> crate::common::Reg<regs::Smctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm1val(self, n: usize) -> crate::common::Reg<regs::Smval, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize + n * 4usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm1octrl(self) -> crate::common::Reg<regs::Smoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm1sts(self) -> crate::common::Reg<regs::Smsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm1inten(self) -> crate::common::Reg<regs::Sminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm1dmaen(self) -> crate::common::Reg<regs::Smdmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm1tctrl(self) -> crate::common::Reg<regs::Smtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm1dismap0(self) -> crate::common::Reg<regs::Smdismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm1dtcnt0(self) -> crate::common::Reg<regs::Smdtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm1dtcnt1(self) -> crate::common::Reg<regs::Smdtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm1captctrlx(self) -> crate::common::Reg<regs::Smcaptctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm1captcompx(self) -> crate::common::Reg<regs::Smcaptcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm1cval0(self) -> crate::common::Reg<regs::Smcval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval0cyc(self) -> crate::common::Reg<regs::Smcval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm1cval1(self) -> crate::common::Reg<regs::Smcval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval1cyc(self) -> crate::common::Reg<regs::Smcval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm1phasedly(self) -> crate::common::Reg<regs::Smphasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm1captfiltx(self) -> crate::common::Reg<regs::Smcaptfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm2cnt(self) -> crate::common::Reg<regs::Smcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm2init(self) -> crate::common::Reg<regs::Sminit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm2ctrl2(self) -> crate::common::Reg<regs::Smctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm2ctrl(self) -> crate::common::Reg<regs::Smctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm2val(self, n: usize) -> crate::common::Reg<regs::Smval, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize + n * 4usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm2octrl(self) -> crate::common::Reg<regs::Smoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm2sts(self) -> crate::common::Reg<regs::Smsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm2inten(self) -> crate::common::Reg<regs::Sminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm2dmaen(self) -> crate::common::Reg<regs::Smdmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm2tctrl(self) -> crate::common::Reg<regs::Smtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm2dismap0(self) -> crate::common::Reg<regs::Smdismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm2dtcnt0(self) -> crate::common::Reg<regs::Smdtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm2dtcnt1(self) -> crate::common::Reg<regs::Smdtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm2captctrlx(self) -> crate::common::Reg<regs::Smcaptctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm2captcompx(self) -> crate::common::Reg<regs::Smcaptcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfeusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm2cval0(self) -> crate::common::Reg<regs::Smcval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval0cyc(self) -> crate::common::Reg<regs::Smcval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm2cval1(self) -> crate::common::Reg<regs::Smcval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval1cyc(self) -> crate::common::Reg<regs::Smcval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm2phasedly(self) -> crate::common::Reg<regs::Smphasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm2captfiltx(self) -> crate::common::Reg<regs::Smcaptfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm3cnt(self) -> crate::common::Reg<regs::Smcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm3init(self) -> crate::common::Reg<regs::Sminit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm3ctrl2(self) -> crate::common::Reg<regs::Smctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm3ctrl(self) -> crate::common::Reg<regs::Smctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm3val(self, n: usize) -> crate::common::Reg<regs::Smval, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize + n * 4usize) as _)
        }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm3octrl(self) -> crate::common::Reg<regs::Smoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm3sts(self) -> crate::common::Reg<regs::Smsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm3inten(self) -> crate::common::Reg<regs::Sminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm3dmaen(self) -> crate::common::Reg<regs::Smdmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm3tctrl(self) -> crate::common::Reg<regs::Smtctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm3dismap0(self) -> crate::common::Reg<regs::Smdismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm3dtcnt0(self) -> crate::common::Reg<regs::Smdtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm3dtcnt1(self) -> crate::common::Reg<regs::Smdtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm3captctrlx(self) -> crate::common::Reg<regs::Smcaptctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm3captcompx(self) -> crate::common::Reg<regs::Smcaptcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm3cval0(self) -> crate::common::Reg<regs::Smcval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval0cyc(self) -> crate::common::Reg<regs::Smcval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm3cval1(self) -> crate::common::Reg<regs::Smcval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval1cyc(self) -> crate::common::Reg<regs::Smcval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm3phasedly(self) -> crate::common::Reg<regs::Smphasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm3captfiltx(self) -> crate::common::Reg<regs::Smcaptfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017eusize) as _) }
    }
    #[doc = "Output Enable Register"]
    #[inline(always)]
    pub const fn outen(self) -> crate::common::Reg<regs::Outen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Mask Register"]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "Software Controlled Output Register"]
    #[inline(always)]
    pub const fn swcout(self) -> crate::common::Reg<regs::Swcout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "PWM Source Select Register"]
    #[inline(always)]
    pub const fn dtsrcsel(self) -> crate::common::Reg<regs::Dtsrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0186usize) as _) }
    }
    #[doc = "Master Control Register"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Master Control 2 Register"]
    #[inline(always)]
    pub const fn mctrl2(self) -> crate::common::Reg<regs::Mctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018ausize) as _) }
    }
    #[doc = "Fault Control Register"]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::common::Reg<regs::Fctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Fault Status Register"]
    #[inline(always)]
    pub const fn fsts0(self) -> crate::common::Reg<regs::Fsts0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018eusize) as _) }
    }
    #[doc = "Fault Filter Register"]
    #[inline(always)]
    pub const fn ffilt0(self) -> crate::common::Reg<regs::Ffilt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Fault Test Register"]
    #[inline(always)]
    pub const fn ftst0(self) -> crate::common::Reg<regs::Ftst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0192usize) as _) }
    }
    #[doc = "Fault Control 2 Register"]
    #[inline(always)]
    pub const fn fctrl20(self) -> crate::common::Reg<regs::Fctrl20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
pub mod regs;
pub mod vals;
