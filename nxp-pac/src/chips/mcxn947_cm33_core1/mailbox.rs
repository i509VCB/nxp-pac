#[doc = "Array of registers: IRQ, IRQSET, IRQCLR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq {
    ptr: *mut u8,
}
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "name (CPUn) Interrupt"]
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "name (CPUn) Interrupt Set"]
    #[inline(always)]
    pub const fn irqset(self) -> crate::common::Reg<regs::Irqset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "name (CPUn) Interrupt Clear"]
    #[inline(always)]
    pub const fn irqclr(self) -> crate::common::Reg<regs::Irqclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "MAILBOX"]
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
    #[doc = "Array of registers: IRQ, IRQSET, IRQCLR"]
    #[inline(always)]
    pub const fn irq(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Mutual Exclusion"]
    #[inline(always)]
    pub const fn mutex(self) -> crate::common::Reg<regs::Mutex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
pub mod regs;
