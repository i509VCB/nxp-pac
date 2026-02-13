#[doc = "WDOG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtwdog {
    ptr: *mut u8,
}
unsafe impl Send for Rtwdog {}
unsafe impl Sync for Rtwdog {}
impl Rtwdog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Watchdog Control and Status Register"]
    #[inline(always)]
    pub const fn cs(self) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Watchdog Counter Register"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Watchdog Timeout Value Register"]
    #[inline(always)]
    pub const fn toval(self) -> crate::common::Reg<regs::Toval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog Window Register"]
    #[inline(always)]
    pub const fn win(self) -> crate::common::Reg<regs::Win, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
