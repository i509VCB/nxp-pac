#[doc = "Mailbox."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAILBOX {
    ptr: *mut u8,
}
unsafe impl Send for MAILBOX {}
unsafe impl Sync for MAILBOX {}
impl MAILBOX {
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
    pub const fn MBOXIRQ(self, n: usize) -> MBOXIRQ {
        assert!(n < 2usize);
        unsafe { MBOXIRQ::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Mutual exclusion register\\[1\\]."]
    #[inline(always)]
    pub const fn MUTEX(self) -> crate::common::Reg<regs::MUTEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MBOXIRQ {
    ptr: *mut u8,
}
unsafe impl Send for MBOXIRQ {}
unsafe impl Sync for MBOXIRQ {}
impl MBOXIRQ {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt request register for the Cortex-M0+ CPU."]
    #[inline(always)]
    pub const fn IRQ(self) -> crate::common::Reg<regs::IRQ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Set bits in IRQ0."]
    #[inline(always)]
    pub const fn IRQSET(self) -> crate::common::Reg<regs::IRQSET, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clear bits in IRQ0."]
    #[inline(always)]
    pub const fn IRQCLR(self) -> crate::common::Reg<regs::IRQCLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
