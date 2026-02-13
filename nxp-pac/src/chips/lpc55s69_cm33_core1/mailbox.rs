#[doc = "Mailbox"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mailbox {
    ptr: *mut u8,
}
unsafe impl Send for Mailbox {}
unsafe impl Sync for Mailbox {}
impl Mailbox {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn mboxirq(self, n: usize) -> Mboxirq {
        assert!(n < 2usize);
        unsafe { Mboxirq::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Mutual exclusion register\\[1\\]"]
    #[inline(always)]
    pub const fn mutex(self) -> crate::common::Reg<regs::Mutex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mboxirq {
    ptr: *mut u8,
}
unsafe impl Send for Mboxirq {}
unsafe impl Sync for Mboxirq {}
impl Mboxirq {
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
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Set bits in IRQ0"]
    #[inline(always)]
    pub const fn irqset(self) -> crate::common::Reg<regs::Irqset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clear bits in IRQ0"]
    #[inline(always)]
    pub const fn irqclr(self) -> crate::common::Reg<regs::Irqclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
