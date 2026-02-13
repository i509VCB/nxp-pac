#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ev {
    ptr: *mut u8,
}
unsafe impl Send for Ev {}
unsafe impl Sync for Ev {}
impl Ev {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT event state register 0"]
    #[inline(always)]
    pub const fn ev_state(self) -> crate::common::Reg<regs::EvState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT event control register 0"]
    #[inline(always)]
    pub const fn ev_ctrl(self) -> crate::common::Reg<regs::EvCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out {
    ptr: *mut u8,
}
unsafe impl Send for Out {}
unsafe impl Sync for Out {}
impl Out {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT output 0 set register"]
    #[inline(always)]
    pub const fn out_set(self) -> crate::common::Reg<regs::OutSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT output 0 clear register"]
    #[inline(always)]
    pub const fn out_clr(self) -> crate::common::Reg<regs::OutClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "SCTimer/PWM (SCT)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0 {
    ptr: *mut u8,
}
unsafe impl Send for Sct0 {}
unsafe impl Sync for Sct0 {}
impl Sct0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT configuration register"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SCT limit event select register"]
    #[inline(always)]
    pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SCT halt event select register"]
    #[inline(always)]
    pub const fn halt(self) -> crate::common::Reg<regs::Halt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SCT stop event select register"]
    #[inline(always)]
    pub const fn stop(self) -> crate::common::Reg<regs::Stop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SCT start event select register"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SCT counter register"]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SCT state register"]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SCT input register"]
    #[inline(always)]
    pub const fn input(self) -> crate::common::Reg<regs::Input, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SCT match/capture mode register"]
    #[inline(always)]
    pub const fn regmode(self) -> crate::common::Reg<regs::Regmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SCT output register"]
    #[inline(always)]
    pub const fn output(self) -> crate::common::Reg<regs::Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SCT output counter direction control register"]
    #[inline(always)]
    pub const fn outputdirctrl(self) -> crate::common::Reg<regs::Outputdirctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SCT conflict resolution register"]
    #[inline(always)]
    pub const fn res(self) -> crate::common::Reg<regs::Res, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SCT DMA request 0 register"]
    #[inline(always)]
    pub const fn dmareq0(self) -> crate::common::Reg<regs::Dmareq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "SCT DMA request 1 register"]
    #[inline(always)]
    pub const fn dmareq1(self) -> crate::common::Reg<regs::Dmareq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SCT event interrupt enable register"]
    #[inline(always)]
    pub const fn even(self) -> crate::common::Reg<regs::Even, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "SCT event flag register"]
    #[inline(always)]
    pub const fn evflag(self) -> crate::common::Reg<regs::Evflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "SCT conflict interrupt enable register"]
    #[inline(always)]
    pub const fn conen(self) -> crate::common::Reg<regs::Conen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "SCT conflict flag register"]
    #[inline(always)]
    pub const fn conflag(self) -> crate::common::Reg<regs::Conflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap(self, n: usize) -> crate::common::Reg<regs::Cap, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SCT match value register of match channels"]
    #[inline(always)]
    pub const fn match_(self, n: usize) -> crate::common::Reg<regs::Match, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl(self, n: usize) -> crate::common::Reg<regs::Capctrl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "SCT match reload value register"]
    #[inline(always)]
    pub const fn matchrel(self, n: usize) -> crate::common::Reg<regs::Matchrel, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn ev(self, n: usize) -> Ev {
        assert!(n < 16usize);
        unsafe { Ev::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 8usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn out(self, n: usize) -> Out {
        assert!(n < 10usize);
        unsafe { Out::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
