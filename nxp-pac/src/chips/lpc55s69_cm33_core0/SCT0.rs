#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EV {
    ptr: *mut u8,
}
unsafe impl Send for EV {}
unsafe impl Sync for EV {}
impl EV {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT event state register 0."]
    #[inline(always)]
    pub const fn EV_STATE(self) -> crate::common::Reg<regs::EV_STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT event control register 0."]
    #[inline(always)]
    pub const fn EV_CTRL(self) -> crate::common::Reg<regs::EV_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUT {
    ptr: *mut u8,
}
unsafe impl Send for OUT {}
unsafe impl Sync for OUT {}
impl OUT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT output 0 set register."]
    #[inline(always)]
    pub const fn OUT_SET(self) -> crate::common::Reg<regs::OUT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT output 0 clear register."]
    #[inline(always)]
    pub const fn OUT_CLR(self) -> crate::common::Reg<regs::OUT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "SCTimer/PWM (SCT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCT0 {
    ptr: *mut u8,
}
unsafe impl Send for SCT0 {}
unsafe impl Sync for SCT0 {}
impl SCT0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT configuration register."]
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT control register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SCT limit event select register."]
    #[inline(always)]
    pub const fn LIMIT(self) -> crate::common::Reg<regs::LIMIT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SCT halt event select register."]
    #[inline(always)]
    pub const fn HALT(self) -> crate::common::Reg<regs::HALT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SCT stop event select register."]
    #[inline(always)]
    pub const fn STOP(self) -> crate::common::Reg<regs::STOP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SCT start event select register."]
    #[inline(always)]
    pub const fn START(self) -> crate::common::Reg<regs::START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SCT counter register."]
    #[inline(always)]
    pub const fn COUNT(self) -> crate::common::Reg<regs::COUNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SCT state register."]
    #[inline(always)]
    pub const fn STATE(self) -> crate::common::Reg<regs::STATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SCT input register."]
    #[inline(always)]
    pub const fn INPUT(self) -> crate::common::Reg<regs::INPUT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SCT match/capture mode register."]
    #[inline(always)]
    pub const fn REGMODE(self) -> crate::common::Reg<regs::REGMODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SCT output register."]
    #[inline(always)]
    pub const fn OUTPUT(self) -> crate::common::Reg<regs::OUTPUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SCT output counter direction control register."]
    #[inline(always)]
    pub const fn OUTPUTDIRCTRL(self) -> crate::common::Reg<regs::OUTPUTDIRCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SCT conflict resolution register."]
    #[inline(always)]
    pub const fn RES(self) -> crate::common::Reg<regs::RES, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SCT DMA request 0 register."]
    #[inline(always)]
    pub const fn DMAREQ0(self) -> crate::common::Reg<regs::DMAREQ0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "SCT DMA request 1 register."]
    #[inline(always)]
    pub const fn DMAREQ1(self) -> crate::common::Reg<regs::DMAREQ1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SCT event interrupt enable register."]
    #[inline(always)]
    pub const fn EVEN(self) -> crate::common::Reg<regs::EVEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "SCT event flag register."]
    #[inline(always)]
    pub const fn EVFLAG(self) -> crate::common::Reg<regs::EVFLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "SCT conflict interrupt enable register."]
    #[inline(always)]
    pub const fn CONEN(self) -> crate::common::Reg<regs::CONEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "SCT conflict flag register."]
    #[inline(always)]
    pub const fn CONFLAG(self) -> crate::common::Reg<regs::CONFLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP0(self) -> crate::common::Reg<regs::CAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH0(self) -> crate::common::Reg<regs::MATCH0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP1(self) -> crate::common::Reg<regs::CAP1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH1(self) -> crate::common::Reg<regs::MATCH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP2(self) -> crate::common::Reg<regs::CAP2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH2(self) -> crate::common::Reg<regs::MATCH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP3(self) -> crate::common::Reg<regs::CAP3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH3(self) -> crate::common::Reg<regs::MATCH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP4(self) -> crate::common::Reg<regs::CAP4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH4(self) -> crate::common::Reg<regs::MATCH4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP5(self) -> crate::common::Reg<regs::CAP5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH5(self) -> crate::common::Reg<regs::MATCH5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP6(self) -> crate::common::Reg<regs::CAP6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH6(self) -> crate::common::Reg<regs::MATCH6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP7(self) -> crate::common::Reg<regs::CAP7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH7(self) -> crate::common::Reg<regs::MATCH7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP8(self) -> crate::common::Reg<regs::CAP8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH8(self) -> crate::common::Reg<regs::MATCH8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP9(self) -> crate::common::Reg<regs::CAP9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH9(self) -> crate::common::Reg<regs::MATCH9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP10(self) -> crate::common::Reg<regs::CAP10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH10(self) -> crate::common::Reg<regs::MATCH10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP11(self) -> crate::common::Reg<regs::CAP11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH11(self) -> crate::common::Reg<regs::MATCH11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP12(self) -> crate::common::Reg<regs::CAP12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH12(self) -> crate::common::Reg<regs::MATCH12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP13(self) -> crate::common::Reg<regs::CAP13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH13(self) -> crate::common::Reg<regs::MATCH13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP14(self) -> crate::common::Reg<regs::CAP14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH14(self) -> crate::common::Reg<regs::MATCH14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "SCT capture register of capture channel."]
    #[inline(always)]
    pub const fn CAP15(self) -> crate::common::Reg<regs::CAP15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "SCT match value register of match channels."]
    #[inline(always)]
    pub const fn MATCH15(self) -> crate::common::Reg<regs::MATCH15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL0(self) -> crate::common::Reg<regs::CAPCTRL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL0(self) -> crate::common::Reg<regs::MATCHREL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL1(self) -> crate::common::Reg<regs::CAPCTRL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL1(self) -> crate::common::Reg<regs::MATCHREL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL2(self) -> crate::common::Reg<regs::CAPCTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL2(self) -> crate::common::Reg<regs::MATCHREL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL3(self) -> crate::common::Reg<regs::CAPCTRL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL3(self) -> crate::common::Reg<regs::MATCHREL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL4(self) -> crate::common::Reg<regs::CAPCTRL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL4(self) -> crate::common::Reg<regs::MATCHREL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL5(self) -> crate::common::Reg<regs::CAPCTRL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL5(self) -> crate::common::Reg<regs::MATCHREL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL6(self) -> crate::common::Reg<regs::CAPCTRL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL6(self) -> crate::common::Reg<regs::MATCHREL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL7(self) -> crate::common::Reg<regs::CAPCTRL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL7(self) -> crate::common::Reg<regs::MATCHREL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL8(self) -> crate::common::Reg<regs::CAPCTRL8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL8(self) -> crate::common::Reg<regs::MATCHREL8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL9(self) -> crate::common::Reg<regs::CAPCTRL9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL9(self) -> crate::common::Reg<regs::MATCHREL9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL10(self) -> crate::common::Reg<regs::CAPCTRL10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL10(self) -> crate::common::Reg<regs::MATCHREL10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL11(self) -> crate::common::Reg<regs::CAPCTRL11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL11(self) -> crate::common::Reg<regs::MATCHREL11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL12(self) -> crate::common::Reg<regs::CAPCTRL12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL12(self) -> crate::common::Reg<regs::MATCHREL12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL13(self) -> crate::common::Reg<regs::CAPCTRL13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL13(self) -> crate::common::Reg<regs::MATCHREL13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL14(self) -> crate::common::Reg<regs::CAPCTRL14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL14(self) -> crate::common::Reg<regs::MATCHREL14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "SCT capture control register."]
    #[inline(always)]
    pub const fn CAPCTRL15(self) -> crate::common::Reg<regs::CAPCTRL15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "SCT match reload value register."]
    #[inline(always)]
    pub const fn MATCHREL15(self) -> crate::common::Reg<regs::MATCHREL15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn EV(self, n: usize) -> EV {
        assert!(n < 16usize);
        unsafe { EV::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 8usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn OUT(self, n: usize) -> OUT {
        assert!(n < 10usize);
        unsafe { OUT::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
