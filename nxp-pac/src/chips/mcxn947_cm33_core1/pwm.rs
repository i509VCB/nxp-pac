#[doc = "PWM"]
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
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm0cnt(self) -> crate::common::Reg<regs::Sm0cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm0init(self) -> crate::common::Reg<regs::Sm0init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm0ctrl2(self) -> crate::common::Reg<regs::Sm0ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm0ctrl(self) -> crate::common::Reg<regs::Sm0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm0val0(self) -> crate::common::Reg<regs::Sm0val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Fractional Value Register 1"]
    #[inline(always)]
    pub const fn sm0fracval1(self) -> crate::common::Reg<regs::Sm0fracval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn sm0val1(self) -> crate::common::Reg<regs::Sm0val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Fractional Value Register 2"]
    #[inline(always)]
    pub const fn sm0fracval2(self) -> crate::common::Reg<regs::Sm0fracval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn sm0val2(self) -> crate::common::Reg<regs::Sm0val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Fractional Value Register 3"]
    #[inline(always)]
    pub const fn sm0fracval3(self) -> crate::common::Reg<regs::Sm0fracval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn sm0val3(self) -> crate::common::Reg<regs::Sm0val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Fractional Value Register 4"]
    #[inline(always)]
    pub const fn sm0fracval4(self) -> crate::common::Reg<regs::Sm0fracval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn sm0val4(self) -> crate::common::Reg<regs::Sm0val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Fractional Value Register 5"]
    #[inline(always)]
    pub const fn sm0fracval5(self) -> crate::common::Reg<regs::Sm0fracval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn sm0val5(self) -> crate::common::Reg<regs::Sm0val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Fractional Control Register"]
    #[inline(always)]
    pub const fn sm0frctrl(self) -> crate::common::Reg<regs::Sm0frctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm0octrl(self) -> crate::common::Reg<regs::Sm0octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm0sts(self) -> crate::common::Reg<regs::Sm0sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm0inten(self) -> crate::common::Reg<regs::Sm0inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm0dmaen(self) -> crate::common::Reg<regs::Sm0dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm0tctrl(self) -> crate::common::Reg<regs::Sm0tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm0dismap0(self) -> crate::common::Reg<regs::Sm0dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm0dtcnt0(self) -> crate::common::Reg<regs::Sm0dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm0dtcnt1(self) -> crate::common::Reg<regs::Sm0dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Capture Control A Register"]
    #[inline(always)]
    pub const fn sm0captctrla(self) -> crate::common::Reg<regs::Sm0captctrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Capture Compare A Register"]
    #[inline(always)]
    pub const fn sm0captcompa(self) -> crate::common::Reg<regs::Sm0captcompa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Capture Control B Register"]
    #[inline(always)]
    pub const fn sm0captctrlb(self) -> crate::common::Reg<regs::Sm0captctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Capture Compare B Register"]
    #[inline(always)]
    pub const fn sm0captcompb(self) -> crate::common::Reg<regs::Sm0captcompb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm0captctrlx(self) -> crate::common::Reg<regs::Sm0captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm0captcompx(self) -> crate::common::Reg<regs::Sm0captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm0cval0(self) -> crate::common::Reg<regs::Sm0cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval0cyc(self) -> crate::common::Reg<regs::Sm0cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm0cval1(self) -> crate::common::Reg<regs::Sm0cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval1cyc(self) -> crate::common::Reg<regs::Sm0cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Capture Value 2 Register"]
    #[inline(always)]
    pub const fn sm0cval2(self) -> crate::common::Reg<regs::Sm0cval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval2cyc(self) -> crate::common::Reg<regs::Sm0cval2cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4ausize) as _) }
    }
    #[doc = "Capture Value 3 Register"]
    #[inline(always)]
    pub const fn sm0cval3(self) -> crate::common::Reg<regs::Sm0cval3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval3cyc(self) -> crate::common::Reg<regs::Sm0cval3cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4eusize) as _) }
    }
    #[doc = "Capture Value 4 Register"]
    #[inline(always)]
    pub const fn sm0cval4(self) -> crate::common::Reg<regs::Sm0cval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval4cyc(self) -> crate::common::Reg<regs::Sm0cval4cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
    #[doc = "Capture Value 5 Register"]
    #[inline(always)]
    pub const fn sm0cval5(self) -> crate::common::Reg<regs::Sm0cval5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[inline(always)]
    pub const fn sm0cval5cyc(self) -> crate::common::Reg<regs::Sm0cval5cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize) as _) }
    }
    #[doc = "Capture PWM_A Input Filter Register"]
    #[inline(always)]
    pub const fn sm0captfilta(self) -> crate::common::Reg<regs::Sm0captfilta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize) as _) }
    }
    #[doc = "Capture PWM_B Input Filter Register"]
    #[inline(always)]
    pub const fn sm0captfiltb(self) -> crate::common::Reg<regs::Sm0captfiltb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm0captfiltx(self) -> crate::common::Reg<regs::Sm0captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm1cnt(self) -> crate::common::Reg<regs::Sm1cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm1init(self) -> crate::common::Reg<regs::Sm1init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm1ctrl2(self) -> crate::common::Reg<regs::Sm1ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm1ctrl(self) -> crate::common::Reg<regs::Sm1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm1val0(self) -> crate::common::Reg<regs::Sm1val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "Fractional Value Register 1"]
    #[inline(always)]
    pub const fn sm1fracval1(self) -> crate::common::Reg<regs::Sm1fracval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn sm1val1(self) -> crate::common::Reg<regs::Sm1val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Fractional Value Register 2"]
    #[inline(always)]
    pub const fn sm1fracval2(self) -> crate::common::Reg<regs::Sm1fracval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn sm1val2(self) -> crate::common::Reg<regs::Sm1val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "Fractional Value Register 3"]
    #[inline(always)]
    pub const fn sm1fracval3(self) -> crate::common::Reg<regs::Sm1fracval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn sm1val3(self) -> crate::common::Reg<regs::Sm1val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "Fractional Value Register 4"]
    #[inline(always)]
    pub const fn sm1fracval4(self) -> crate::common::Reg<regs::Sm1fracval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn sm1val4(self) -> crate::common::Reg<regs::Sm1val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "Fractional Value Register 5"]
    #[inline(always)]
    pub const fn sm1fracval5(self) -> crate::common::Reg<regs::Sm1fracval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn sm1val5(self) -> crate::common::Reg<regs::Sm1val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "Fractional Control Register"]
    #[inline(always)]
    pub const fn sm1frctrl(self) -> crate::common::Reg<regs::Sm1frctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm1octrl(self) -> crate::common::Reg<regs::Sm1octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm1sts(self) -> crate::common::Reg<regs::Sm1sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm1inten(self) -> crate::common::Reg<regs::Sm1inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm1dmaen(self) -> crate::common::Reg<regs::Sm1dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm1tctrl(self) -> crate::common::Reg<regs::Sm1tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm1dismap0(self) -> crate::common::Reg<regs::Sm1dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm1dtcnt0(self) -> crate::common::Reg<regs::Sm1dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm1dtcnt1(self) -> crate::common::Reg<regs::Sm1dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x92usize) as _) }
    }
    #[doc = "Capture Control A Register"]
    #[inline(always)]
    pub const fn sm1captctrla(self) -> crate::common::Reg<regs::Sm1captctrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Capture Compare A Register"]
    #[inline(always)]
    pub const fn sm1captcompa(self) -> crate::common::Reg<regs::Sm1captcompa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x96usize) as _) }
    }
    #[doc = "Capture Control B Register"]
    #[inline(always)]
    pub const fn sm1captctrlb(self) -> crate::common::Reg<regs::Sm1captctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Capture Compare B Register"]
    #[inline(always)]
    pub const fn sm1captcompb(self) -> crate::common::Reg<regs::Sm1captcompb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9ausize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm1captctrlx(self) -> crate::common::Reg<regs::Sm1captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm1captcompx(self) -> crate::common::Reg<regs::Sm1captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm1cval0(self) -> crate::common::Reg<regs::Sm1cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval0cyc(self) -> crate::common::Reg<regs::Sm1cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm1cval1(self) -> crate::common::Reg<regs::Sm1cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval1cyc(self) -> crate::common::Reg<regs::Sm1cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa6usize) as _) }
    }
    #[doc = "Capture Value 2 Register"]
    #[inline(always)]
    pub const fn sm1cval2(self) -> crate::common::Reg<regs::Sm1cval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval2cyc(self) -> crate::common::Reg<regs::Sm1cval2cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaausize) as _) }
    }
    #[doc = "Capture Value 3 Register"]
    #[inline(always)]
    pub const fn sm1cval3(self) -> crate::common::Reg<regs::Sm1cval3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval3cyc(self) -> crate::common::Reg<regs::Sm1cval3cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xaeusize) as _) }
    }
    #[doc = "Capture Value 4 Register"]
    #[inline(always)]
    pub const fn sm1cval4(self) -> crate::common::Reg<regs::Sm1cval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval4cyc(self) -> crate::common::Reg<regs::Sm1cval4cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb2usize) as _) }
    }
    #[doc = "Capture Value 5 Register"]
    #[inline(always)]
    pub const fn sm1cval5(self) -> crate::common::Reg<regs::Sm1cval5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[inline(always)]
    pub const fn sm1cval5cyc(self) -> crate::common::Reg<regs::Sm1cval5cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb6usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm1phasedly(self) -> crate::common::Reg<regs::Sm1phasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Capture PWM_A Input Filter Register"]
    #[inline(always)]
    pub const fn sm1captfilta(self) -> crate::common::Reg<regs::Sm1captfilta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbausize) as _) }
    }
    #[doc = "Capture PWM_B Input Filter Register"]
    #[inline(always)]
    pub const fn sm1captfiltb(self) -> crate::common::Reg<regs::Sm1captfiltb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm1captfiltx(self) -> crate::common::Reg<regs::Sm1captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm2cnt(self) -> crate::common::Reg<regs::Sm2cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm2init(self) -> crate::common::Reg<regs::Sm2init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc2usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm2ctrl2(self) -> crate::common::Reg<regs::Sm2ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm2ctrl(self) -> crate::common::Reg<regs::Sm2ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc6usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm2val0(self) -> crate::common::Reg<regs::Sm2val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xcausize) as _) }
    }
    #[doc = "Fractional Value Register 1"]
    #[inline(always)]
    pub const fn sm2fracval1(self) -> crate::common::Reg<regs::Sm2fracval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn sm2val1(self) -> crate::common::Reg<regs::Sm2val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xceusize) as _) }
    }
    #[doc = "Fractional Value Register 2"]
    #[inline(always)]
    pub const fn sm2fracval2(self) -> crate::common::Reg<regs::Sm2fracval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn sm2val2(self) -> crate::common::Reg<regs::Sm2val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd2usize) as _) }
    }
    #[doc = "Fractional Value Register 3"]
    #[inline(always)]
    pub const fn sm2fracval3(self) -> crate::common::Reg<regs::Sm2fracval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn sm2val3(self) -> crate::common::Reg<regs::Sm2val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd6usize) as _) }
    }
    #[doc = "Fractional Value Register 4"]
    #[inline(always)]
    pub const fn sm2fracval4(self) -> crate::common::Reg<regs::Sm2fracval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn sm2val4(self) -> crate::common::Reg<regs::Sm2val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdausize) as _) }
    }
    #[doc = "Fractional Value Register 5"]
    #[inline(always)]
    pub const fn sm2fracval5(self) -> crate::common::Reg<regs::Sm2fracval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn sm2val5(self) -> crate::common::Reg<regs::Sm2val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdeusize) as _) }
    }
    #[doc = "Fractional Control Register"]
    #[inline(always)]
    pub const fn sm2frctrl(self) -> crate::common::Reg<regs::Sm2frctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm2octrl(self) -> crate::common::Reg<regs::Sm2octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe2usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm2sts(self) -> crate::common::Reg<regs::Sm2sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm2inten(self) -> crate::common::Reg<regs::Sm2inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe6usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm2dmaen(self) -> crate::common::Reg<regs::Sm2dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm2tctrl(self) -> crate::common::Reg<regs::Sm2tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xeausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm2dismap0(self) -> crate::common::Reg<regs::Sm2dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm2dtcnt0(self) -> crate::common::Reg<regs::Sm2dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm2dtcnt1(self) -> crate::common::Reg<regs::Sm2dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf2usize) as _) }
    }
    #[doc = "Capture Control A Register"]
    #[inline(always)]
    pub const fn sm2captctrla(self) -> crate::common::Reg<regs::Sm2captctrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Capture Compare A Register"]
    #[inline(always)]
    pub const fn sm2captcompa(self) -> crate::common::Reg<regs::Sm2captcompa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf6usize) as _) }
    }
    #[doc = "Capture Control B Register"]
    #[inline(always)]
    pub const fn sm2captctrlb(self) -> crate::common::Reg<regs::Sm2captctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Capture Compare B Register"]
    #[inline(always)]
    pub const fn sm2captcompb(self) -> crate::common::Reg<regs::Sm2captcompb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfausize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm2captctrlx(self) -> crate::common::Reg<regs::Sm2captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm2captcompx(self) -> crate::common::Reg<regs::Sm2captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfeusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm2cval0(self) -> crate::common::Reg<regs::Sm2cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval0cyc(self) -> crate::common::Reg<regs::Sm2cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm2cval1(self) -> crate::common::Reg<regs::Sm2cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval1cyc(self) -> crate::common::Reg<regs::Sm2cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Capture Value 2 Register"]
    #[inline(always)]
    pub const fn sm2cval2(self) -> crate::common::Reg<regs::Sm2cval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval2cyc(self) -> crate::common::Reg<regs::Sm2cval2cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "Capture Value 3 Register"]
    #[inline(always)]
    pub const fn sm2cval3(self) -> crate::common::Reg<regs::Sm2cval3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval3cyc(self) -> crate::common::Reg<regs::Sm2cval3cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010eusize) as _) }
    }
    #[doc = "Capture Value 4 Register"]
    #[inline(always)]
    pub const fn sm2cval4(self) -> crate::common::Reg<regs::Sm2cval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval4cyc(self) -> crate::common::Reg<regs::Sm2cval4cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0112usize) as _) }
    }
    #[doc = "Capture Value 5 Register"]
    #[inline(always)]
    pub const fn sm2cval5(self) -> crate::common::Reg<regs::Sm2cval5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[inline(always)]
    pub const fn sm2cval5cyc(self) -> crate::common::Reg<regs::Sm2cval5cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0116usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm2phasedly(self) -> crate::common::Reg<regs::Sm2phasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture PWM_A Input Filter Register"]
    #[inline(always)]
    pub const fn sm2captfilta(self) -> crate::common::Reg<regs::Sm2captfilta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011ausize) as _) }
    }
    #[doc = "Capture PWM_B Input Filter Register"]
    #[inline(always)]
    pub const fn sm2captfiltb(self) -> crate::common::Reg<regs::Sm2captfiltb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm2captfiltx(self) -> crate::common::Reg<regs::Sm2captfiltx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Counter Register"]
    #[inline(always)]
    pub const fn sm3cnt(self) -> crate::common::Reg<regs::Sm3cnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Initial Count Register"]
    #[inline(always)]
    pub const fn sm3init(self) -> crate::common::Reg<regs::Sm3init, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0122usize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn sm3ctrl2(self) -> crate::common::Reg<regs::Sm3ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn sm3ctrl(self) -> crate::common::Reg<regs::Sm3ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0126usize) as _) }
    }
    #[doc = "Value Register 0"]
    #[inline(always)]
    pub const fn sm3val0(self) -> crate::common::Reg<regs::Sm3val0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012ausize) as _) }
    }
    #[doc = "Fractional Value Register 1"]
    #[inline(always)]
    pub const fn sm3fracval1(self) -> crate::common::Reg<regs::Sm3fracval1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Value Register 1"]
    #[inline(always)]
    pub const fn sm3val1(self) -> crate::common::Reg<regs::Sm3val1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012eusize) as _) }
    }
    #[doc = "Fractional Value Register 2"]
    #[inline(always)]
    pub const fn sm3fracval2(self) -> crate::common::Reg<regs::Sm3fracval2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Value Register 2"]
    #[inline(always)]
    pub const fn sm3val2(self) -> crate::common::Reg<regs::Sm3val2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0132usize) as _) }
    }
    #[doc = "Fractional Value Register 3"]
    #[inline(always)]
    pub const fn sm3fracval3(self) -> crate::common::Reg<regs::Sm3fracval3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Value Register 3"]
    #[inline(always)]
    pub const fn sm3val3(self) -> crate::common::Reg<regs::Sm3val3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0136usize) as _) }
    }
    #[doc = "Fractional Value Register 4"]
    #[inline(always)]
    pub const fn sm3fracval4(self) -> crate::common::Reg<regs::Sm3fracval4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Value Register 4"]
    #[inline(always)]
    pub const fn sm3val4(self) -> crate::common::Reg<regs::Sm3val4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013ausize) as _) }
    }
    #[doc = "Fractional Value Register 5"]
    #[inline(always)]
    pub const fn sm3fracval5(self) -> crate::common::Reg<regs::Sm3fracval5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Value Register 5"]
    #[inline(always)]
    pub const fn sm3val5(self) -> crate::common::Reg<regs::Sm3val5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013eusize) as _) }
    }
    #[doc = "Fractional Control Register"]
    #[inline(always)]
    pub const fn sm3frctrl(self) -> crate::common::Reg<regs::Sm3frctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Output Control Register"]
    #[inline(always)]
    pub const fn sm3octrl(self) -> crate::common::Reg<regs::Sm3octrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0142usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sm3sts(self) -> crate::common::Reg<regs::Sm3sts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sm3inten(self) -> crate::common::Reg<regs::Sm3inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0146usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn sm3dmaen(self) -> crate::common::Reg<regs::Sm3dmaen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Output Trigger Control Register"]
    #[inline(always)]
    pub const fn sm3tctrl(self) -> crate::common::Reg<regs::Sm3tctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014ausize) as _) }
    }
    #[doc = "Fault Disable Mapping Register 0"]
    #[inline(always)]
    pub const fn sm3dismap0(self) -> crate::common::Reg<regs::Sm3dismap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Deadtime Count Register 0"]
    #[inline(always)]
    pub const fn sm3dtcnt0(self) -> crate::common::Reg<regs::Sm3dtcnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Deadtime Count Register 1"]
    #[inline(always)]
    pub const fn sm3dtcnt1(self) -> crate::common::Reg<regs::Sm3dtcnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0152usize) as _) }
    }
    #[doc = "Capture Control A Register"]
    #[inline(always)]
    pub const fn sm3captctrla(self) -> crate::common::Reg<regs::Sm3captctrla, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Capture Compare A Register"]
    #[inline(always)]
    pub const fn sm3captcompa(self) -> crate::common::Reg<regs::Sm3captcompa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0156usize) as _) }
    }
    #[doc = "Capture Control B Register"]
    #[inline(always)]
    pub const fn sm3captctrlb(self) -> crate::common::Reg<regs::Sm3captctrlb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Capture Compare B Register"]
    #[inline(always)]
    pub const fn sm3captcompb(self) -> crate::common::Reg<regs::Sm3captcompb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015ausize) as _) }
    }
    #[doc = "Capture Control X Register"]
    #[inline(always)]
    pub const fn sm3captctrlx(self) -> crate::common::Reg<regs::Sm3captctrlx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Capture Compare X Register"]
    #[inline(always)]
    pub const fn sm3captcompx(self) -> crate::common::Reg<regs::Sm3captcompx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015eusize) as _) }
    }
    #[doc = "Capture Value 0 Register"]
    #[inline(always)]
    pub const fn sm3cval0(self) -> crate::common::Reg<regs::Sm3cval0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Capture Value 0 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval0cyc(self) -> crate::common::Reg<regs::Sm3cval0cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0162usize) as _) }
    }
    #[doc = "Capture Value 1 Register"]
    #[inline(always)]
    pub const fn sm3cval1(self) -> crate::common::Reg<regs::Sm3cval1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Capture Value 1 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval1cyc(self) -> crate::common::Reg<regs::Sm3cval1cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0166usize) as _) }
    }
    #[doc = "Capture Value 2 Register"]
    #[inline(always)]
    pub const fn sm3cval2(self) -> crate::common::Reg<regs::Sm3cval2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Capture Value 2 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval2cyc(self) -> crate::common::Reg<regs::Sm3cval2cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016ausize) as _) }
    }
    #[doc = "Capture Value 3 Register"]
    #[inline(always)]
    pub const fn sm3cval3(self) -> crate::common::Reg<regs::Sm3cval3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Capture Value 3 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval3cyc(self) -> crate::common::Reg<regs::Sm3cval3cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016eusize) as _) }
    }
    #[doc = "Capture Value 4 Register"]
    #[inline(always)]
    pub const fn sm3cval4(self) -> crate::common::Reg<regs::Sm3cval4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Capture Value 4 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval4cyc(self) -> crate::common::Reg<regs::Sm3cval4cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0172usize) as _) }
    }
    #[doc = "Capture Value 5 Register"]
    #[inline(always)]
    pub const fn sm3cval5(self) -> crate::common::Reg<regs::Sm3cval5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Capture Value 5 Cycle Register"]
    #[inline(always)]
    pub const fn sm3cval5cyc(self) -> crate::common::Reg<regs::Sm3cval5cyc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0176usize) as _) }
    }
    #[doc = "Phase Delay Register"]
    #[inline(always)]
    pub const fn sm3phasedly(self) -> crate::common::Reg<regs::Sm3phasedly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Capture PWM_A Input Filter Register"]
    #[inline(always)]
    pub const fn sm3captfilta(self) -> crate::common::Reg<regs::Sm3captfilta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017ausize) as _) }
    }
    #[doc = "Capture PWM_B Input Filter Register"]
    #[inline(always)]
    pub const fn sm3captfiltb(self) -> crate::common::Reg<regs::Sm3captfiltb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Capture PWM_X Input Filter Register"]
    #[inline(always)]
    pub const fn sm3captfiltx(self) -> crate::common::Reg<regs::Sm3captfiltx, crate::common::RW> {
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
