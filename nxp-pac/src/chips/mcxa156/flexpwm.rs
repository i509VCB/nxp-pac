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
    pub const fn sm0cnt(self) -> crate::common::Reg<regs::Sm0cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm0init(self) -> crate::common::Reg<regs::Sm0init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm0ctrl2(self) -> crate::common::Reg<regs::Sm0ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm0ctrl(self) -> crate::common::Reg<regs::Sm0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm0val0(self) -> crate::common::Reg<regs::Sm0val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm0val1(self) -> crate::common::Reg<regs::Sm0val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm0val2(self) -> crate::common::Reg<regs::Sm0val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm0val3(self) -> crate::common::Reg<regs::Sm0val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm0val4(self) -> crate::common::Reg<regs::Sm0val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm0val5(self) -> crate::common::Reg<regs::Sm0val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm0octrl(self) -> crate::common::Reg<regs::Sm0octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm0sts(self) -> crate::common::Reg<regs::Sm0sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm0inten(self) -> crate::common::Reg<regs::Sm0inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm0dmaen(self) -> crate::common::Reg<regs::Sm0dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm0tctrl(self) -> crate::common::Reg<regs::Sm0tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm0dismap0(self) -> crate::common::Reg<regs::Sm0dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm0dtcnt0(self) -> crate::common::Reg<regs::Sm0dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm0dtcnt1(self) -> crate::common::Reg<regs::Sm0dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm0captctrlx(self) -> crate::common::Reg<regs::Sm0captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm0captcompx(self) -> crate::common::Reg<regs::Sm0captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm0cval0(self) -> crate::common::Reg<regs::Sm0cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval0cyc(self) -> crate::common::Reg<regs::Sm0cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm0cval1(self) -> crate::common::Reg<regs::Sm0cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm0cval1cyc(self) -> crate::common::Reg<regs::Sm0cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm0captfiltx(self) -> crate::common::Reg<regs::Sm0captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm1cnt(self) -> crate::common::Reg<regs::Sm1cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm1init(self) -> crate::common::Reg<regs::Sm1init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm1ctrl2(self) -> crate::common::Reg<regs::Sm1ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm1ctrl(self) -> crate::common::Reg<regs::Sm1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm1val0(self) -> crate::common::Reg<regs::Sm1val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm1val1(self) -> crate::common::Reg<regs::Sm1val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm1val2(self) -> crate::common::Reg<regs::Sm1val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm1val3(self) -> crate::common::Reg<regs::Sm1val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm1val4(self) -> crate::common::Reg<regs::Sm1val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm1val5(self) -> crate::common::Reg<regs::Sm1val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm1octrl(self) -> crate::common::Reg<regs::Sm1octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm1sts(self) -> crate::common::Reg<regs::Sm1sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm1inten(self) -> crate::common::Reg<regs::Sm1inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm1dmaen(self) -> crate::common::Reg<regs::Sm1dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm1tctrl(self) -> crate::common::Reg<regs::Sm1tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm1dismap0(self) -> crate::common::Reg<regs::Sm1dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm1dtcnt0(self) -> crate::common::Reg<regs::Sm1dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm1dtcnt1(self) -> crate::common::Reg<regs::Sm1dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm1captctrlx(self) -> crate::common::Reg<regs::Sm1captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm1captcompx(self) -> crate::common::Reg<regs::Sm1captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm1cval0(self) -> crate::common::Reg<regs::Sm1cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval0cyc(self) -> crate::common::Reg<regs::Sm1cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm1cval1(self) -> crate::common::Reg<regs::Sm1cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm1cval1cyc(self) -> crate::common::Reg<regs::Sm1cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm1phasedly(self) -> crate::common::Reg<regs::Sm1phasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm1captfiltx(self) -> crate::common::Reg<regs::Sm1captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Counter Register."]
    #[inline(always)]
    pub const fn sm2cnt(self) -> crate::common::Reg<regs::Sm2cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initial Count Register."]
    #[inline(always)]
    pub const fn sm2init(self) -> crate::common::Reg<regs::Sm2init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Control 2 Register."]
    #[inline(always)]
    pub const fn sm2ctrl2(self) -> crate::common::Reg<regs::Sm2ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn sm2ctrl(self) -> crate::common::Reg<regs::Sm2ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Value Register 0."]
    #[inline(always)]
    pub const fn sm2val0(self) -> crate::common::Reg<regs::Sm2val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize) as _) }
    }
    #[doc = "Value Register 1."]
    #[inline(always)]
    pub const fn sm2val1(self) -> crate::common::Reg<regs::Sm2val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xceusize) as _) }
    }
    #[doc = "Value Register 2."]
    #[inline(always)]
    pub const fn sm2val2(self) -> crate::common::Reg<regs::Sm2val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd2usize) as _) }
    }
    #[doc = "Value Register 3."]
    #[inline(always)]
    pub const fn sm2val3(self) -> crate::common::Reg<regs::Sm2val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd6usize) as _) }
    }
    #[doc = "Value Register 4."]
    #[inline(always)]
    pub const fn sm2val4(self) -> crate::common::Reg<regs::Sm2val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdausize) as _) }
    }
    #[doc = "Value Register 5."]
    #[inline(always)]
    pub const fn sm2val5(self) -> crate::common::Reg<regs::Sm2val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdeusize) as _) }
    }
    #[doc = "Output Control Register."]
    #[inline(always)]
    pub const fn sm2octrl(self) -> crate::common::Reg<regs::Sm2octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn sm2sts(self) -> crate::common::Reg<regs::Sm2sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn sm2inten(self) -> crate::common::Reg<regs::Sm2inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn sm2dmaen(self) -> crate::common::Reg<regs::Sm2dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Output Trigger Control Register."]
    #[inline(always)]
    pub const fn sm2tctrl(self) -> crate::common::Reg<regs::Sm2tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0."]
    #[inline(always)]
    pub const fn sm2dismap0(self) -> crate::common::Reg<regs::Sm2dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Deadtime Count Register 0."]
    #[inline(always)]
    pub const fn sm2dtcnt0(self) -> crate::common::Reg<regs::Sm2dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Deadtime Count Register 1."]
    #[inline(always)]
    pub const fn sm2dtcnt1(self) -> crate::common::Reg<regs::Sm2dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Capture Control X Register."]
    #[inline(always)]
    pub const fn sm2captctrlx(self) -> crate::common::Reg<regs::Sm2captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Compare X Register."]
    #[inline(always)]
    pub const fn sm2captcompx(self) -> crate::common::Reg<regs::Sm2captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfeusize) as _) }
    }
    #[doc = "Capture Value 0 Register."]
    #[inline(always)]
    pub const fn sm2cval0(self) -> crate::common::Reg<regs::Sm2cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval0cyc(self) -> crate::common::Reg<regs::Sm2cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Capture Value 1 Register."]
    #[inline(always)]
    pub const fn sm2cval1(self) -> crate::common::Reg<regs::Sm2cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register."]
    #[inline(always)]
    pub const fn sm2cval1cyc(self) -> crate::common::Reg<regs::Sm2cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Phase Delay Register."]
    #[inline(always)]
    pub const fn sm2phasedly(self) -> crate::common::Reg<regs::Sm2phasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register."]
    #[inline(always)]
    pub const fn sm2captfiltx(self) -> crate::common::Reg<regs::Sm2captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Output Enable Register."]
    #[inline(always)]
    pub const fn outen(self) -> crate::common::Reg<regs::Outen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Mask Register."]
    #[inline(always)]
    pub const fn mask(self) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0182usize) as _) }
    }
    #[doc = "Software Controlled Output Register."]
    #[inline(always)]
    pub const fn swcout(self) -> crate::common::Reg<regs::Swcout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "PWM Source Select Register."]
    #[inline(always)]
    pub const fn dtsrcsel(self) -> crate::common::Reg<regs::Dtsrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0186usize) as _) }
    }
    #[doc = "Master Control Register."]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Master Control 2 Register."]
    #[inline(always)]
    pub const fn mctrl2(self) -> crate::common::Reg<regs::Mctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018ausize) as _) }
    }
    #[doc = "Fault Control Register."]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::common::Reg<regs::Fctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Fault Status Register."]
    #[inline(always)]
    pub const fn fsts0(self) -> crate::common::Reg<regs::Fsts0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018eusize) as _) }
    }
    #[doc = "Fault Filter Register."]
    #[inline(always)]
    pub const fn ffilt0(self) -> crate::common::Reg<regs::Ffilt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Fault Test Register."]
    #[inline(always)]
    pub const fn ftst0(self) -> crate::common::Reg<regs::Ftst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0192usize) as _) }
    }
    #[doc = "Fault Control 2 Register."]
    #[inline(always)]
    pub const fn fctrl20(self) -> crate::common::Reg<regs::Fctrl20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
pub mod regs;
pub mod vals;
