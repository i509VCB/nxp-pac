#[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Time Interval Value"]
    #[inline(always)]
    pub const fn intval(self) -> crate::common::Reg<regs::Intval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Multi-Rate Timer (MRT)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrt0 {
    ptr: *mut u8,
}
unsafe impl Send for Mrt0 {}
unsafe impl Sync for Mrt0 {}
impl Mrt0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: INTVAL, TIMER, CTRL, STAT"]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 4usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Module Configuration"]
    #[inline(always)]
    pub const fn modcfg(self) -> crate::common::Reg<regs::Modcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Idle Channel"]
    #[inline(always)]
    pub const fn idle_ch(self) -> crate::common::Reg<regs::IdleCh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Global Interrupt Flag"]
    #[inline(always)]
    pub const fn irq_flag(self) -> crate::common::Reg<regs::IrqFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
