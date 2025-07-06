#[doc = "KPP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpp {
    ptr: *mut u8,
}
unsafe impl Send for Kpp {}
unsafe impl Sync for Kpp {}
impl Kpp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Keypad Control Register"]
    #[inline(always)]
    pub const fn kpcr(self) -> crate::common::Reg<regs::Kpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Keypad Status Register"]
    #[inline(always)]
    pub const fn kpsr(self) -> crate::common::Reg<regs::Kpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Keypad Data Direction Register"]
    #[inline(always)]
    pub const fn kddr(self) -> crate::common::Reg<regs::Kddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Keypad Data Register"]
    #[inline(always)]
    pub const fn kpdr(self) -> crate::common::Reg<regs::Kpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
}
pub mod regs;
pub mod vals;
