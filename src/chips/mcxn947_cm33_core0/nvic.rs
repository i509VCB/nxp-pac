#[doc = "Nested Vectored Interrupt Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nvic {
    ptr: *mut u8,
}
unsafe impl Send for Nvic {}
unsafe impl Sync for Nvic {}
impl Nvic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Set Enable Register"]
    #[inline(always)]
    pub const fn iser(self, n: usize) -> crate::common::Reg<regs::Iser, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register"]
    #[inline(always)]
    pub const fn icer(self, n: usize) -> crate::common::Reg<regs::Icer, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register"]
    #[inline(always)]
    pub const fn ispr(self, n: usize) -> crate::common::Reg<regs::Ispr, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Clear Pending Register"]
    #[inline(always)]
    pub const fn icpr(self, n: usize) -> crate::common::Reg<regs::Icpr, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Active Bit Register"]
    #[inline(always)]
    pub const fn iabr(self, n: usize) -> crate::common::Reg<regs::Iabr, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Target Non-secure Register"]
    #[inline(always)]
    pub const fn itns(self, n: usize) -> crate::common::Reg<regs::Itns, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Priority Register"]
    #[inline(always)]
    pub const fn ipr(self, n: usize) -> crate::common::Reg<regs::Ipr, crate::common::RW> {
        assert!(n < 120usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
    #[doc = "Software Trigger Interrupt Register"]
    #[inline(always)]
    pub const fn stir(self) -> crate::common::Reg<regs::Stir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
}
pub mod regs;
pub mod vals;
