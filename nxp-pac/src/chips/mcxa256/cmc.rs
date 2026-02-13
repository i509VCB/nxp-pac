#[doc = "CMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmc {
    ptr: *mut u8,
}
unsafe impl Send for Cmc {}
unsafe impl Sync for Cmc {}
impl Cmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Clock Control"]
    #[inline(always)]
    pub const fn ckctrl(self) -> crate::common::Reg<regs::Ckctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Clock Status"]
    #[inline(always)]
    pub const fn ckstat(self) -> crate::common::Reg<regs::Ckstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Power Mode Protection"]
    #[inline(always)]
    pub const fn pmprot(self) -> crate::common::Reg<regs::Pmprot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Global Power Mode Control"]
    #[inline(always)]
    pub const fn gpmctrl(self) -> crate::common::Reg<regs::Gpmctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Power Mode Control"]
    #[inline(always)]
    pub const fn pmctrlmain(self) -> crate::common::Reg<regs::Pmctrlmain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "System Reset Status"]
    #[inline(always)]
    pub const fn srs(self) -> crate::common::Reg<regs::Srs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Reset Pin Control"]
    #[inline(always)]
    pub const fn rpc(self) -> crate::common::Reg<regs::Rpc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Sticky System Reset Status"]
    #[inline(always)]
    pub const fn ssrs(self) -> crate::common::Reg<regs::Ssrs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "System Reset Interrupt Enable"]
    #[inline(always)]
    pub const fn srie(self) -> crate::common::Reg<regs::Srie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "System Reset Interrupt Flag"]
    #[inline(always)]
    pub const fn srif(self) -> crate::common::Reg<regs::Srif, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Mode"]
    #[inline(always)]
    pub const fn mr0(self) -> crate::common::Reg<regs::Mr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Force Mode"]
    #[inline(always)]
    pub const fn fm0(self) -> crate::common::Reg<regs::Fm0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Flash Control"]
    #[inline(always)]
    pub const fn flashcr(self) -> crate::common::Reg<regs::Flashcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Core Control"]
    #[inline(always)]
    pub const fn corectl(self) -> crate::common::Reg<regs::Corectl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Debug Control"]
    #[inline(always)]
    pub const fn dbgctl(self) -> crate::common::Reg<regs::Dbgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
}
pub mod regs;
pub mod vals;
