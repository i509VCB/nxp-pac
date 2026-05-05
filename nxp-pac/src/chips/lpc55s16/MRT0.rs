#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHANNEL {
    ptr: *mut u8,
}
unsafe impl Send for CHANNEL {}
unsafe impl Sync for CHANNEL {}
impl CHANNEL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
    #[inline(always)]
    pub const fn INTVAL(self) -> crate::common::Reg<regs::INTVAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MRT Timer register. This register reads the value of the down-counter."]
    #[inline(always)]
    pub const fn TIMER(self) -> crate::common::Reg<regs::TIMER, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MRT Control register. This register controls the MRT modes."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MRT Status register."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Multi-Rate Timer (MRT)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MRT0 {
    ptr: *mut u8,
}
unsafe impl Send for MRT0 {}
unsafe impl Sync for MRT0 {}
impl MRT0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn CHANNEL(self, n: usize) -> CHANNEL {
        assert!(n < 4usize);
        unsafe { CHANNEL::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
    #[inline(always)]
    pub const fn MODCFG(self) -> crate::common::Reg<regs::MODCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Idle channel register. This register returns the number of the first idle channel."]
    #[inline(always)]
    pub const fn IDLE_CH(self) -> crate::common::Reg<regs::IDLE_CH, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Global interrupt flag register."]
    #[inline(always)]
    pub const fn IRQ_FLAG(self) -> crate::common::Reg<regs::IRQ_FLAG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
