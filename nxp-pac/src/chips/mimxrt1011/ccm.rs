#[doc = "CCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccm {
    ptr: *mut u8,
}
unsafe impl Send for Ccm {}
unsafe impl Sync for Ccm {}
impl Ccm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CCM Control Register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CCM Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CCM Clock Switcher Register"]
    #[inline(always)]
    pub const fn ccsr(self) -> crate::common::Reg<regs::Ccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CCM Bus Clock Divider Register"]
    #[inline(always)]
    pub const fn cbcdr(self) -> crate::common::Reg<regs::Cbcdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CCM Bus Clock Multiplexer Register"]
    #[inline(always)]
    pub const fn cbcmr(self) -> crate::common::Reg<regs::Cbcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CCM Serial Clock Multiplexer Register 1"]
    #[inline(always)]
    pub const fn cscmr1(self) -> crate::common::Reg<regs::Cscmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CCM Serial Clock Multiplexer Register 2"]
    #[inline(always)]
    pub const fn cscmr2(self) -> crate::common::Reg<regs::Cscmr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CCM Serial Clock Divider Register 1"]
    #[inline(always)]
    pub const fn cscdr1(self) -> crate::common::Reg<regs::Cscdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CCM Clock Divider Register"]
    #[inline(always)]
    pub const fn cs1cdr(self) -> crate::common::Reg<regs::Cs1cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CCM D1 Clock Divider Register"]
    #[inline(always)]
    pub const fn cdcdr(self) -> crate::common::Reg<regs::Cdcdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "CCM Serial Clock Divider Register 2"]
    #[inline(always)]
    pub const fn cscdr2(self) -> crate::common::Reg<regs::Cscdr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "CCM Divider Handshake In-Process Register"]
    #[inline(always)]
    pub const fn cdhipr(self) -> crate::common::Reg<regs::Cdhipr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "CCM Low Power Control Register"]
    #[inline(always)]
    pub const fn clpcr(self) -> crate::common::Reg<regs::Clpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "CCM Interrupt Status Register"]
    #[inline(always)]
    pub const fn cisr(self) -> crate::common::Reg<regs::Cisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "CCM Interrupt Mask Register"]
    #[inline(always)]
    pub const fn cimr(self) -> crate::common::Reg<regs::Cimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CCM Clock Output Source Register"]
    #[inline(always)]
    pub const fn ccosr(self) -> crate::common::Reg<regs::Ccosr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "CCM General Purpose Register"]
    #[inline(always)]
    pub const fn cgpr(self) -> crate::common::Reg<regs::Cgpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "CCM Clock Gating Register 0"]
    #[inline(always)]
    pub const fn ccgr0(self) -> crate::common::Reg<regs::Ccgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "CCM Clock Gating Register 1"]
    #[inline(always)]
    pub const fn ccgr1(self) -> crate::common::Reg<regs::Ccgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "CCM Clock Gating Register 2"]
    #[inline(always)]
    pub const fn ccgr2(self) -> crate::common::Reg<regs::Ccgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "CCM Clock Gating Register 3"]
    #[inline(always)]
    pub const fn ccgr3(self) -> crate::common::Reg<regs::Ccgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "CCM Clock Gating Register 4"]
    #[inline(always)]
    pub const fn ccgr4(self) -> crate::common::Reg<regs::Ccgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "CCM Clock Gating Register 5"]
    #[inline(always)]
    pub const fn ccgr5(self) -> crate::common::Reg<regs::Ccgr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "CCM Clock Gating Register 6"]
    #[inline(always)]
    pub const fn ccgr6(self) -> crate::common::Reg<regs::Ccgr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "CCM Module Enable Overide Register"]
    #[inline(always)]
    pub const fn cmeor(self) -> crate::common::Reg<regs::Cmeor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
}
pub mod regs;
pub mod vals;
