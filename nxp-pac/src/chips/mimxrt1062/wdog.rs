#[doc = "WDOG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdog {
    ptr: *mut u8,
}
unsafe impl Send for Wdog {}
unsafe impl Sync for Wdog {}
impl Wdog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Watchdog Control Register"]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Watchdog Service Register"]
    #[inline(always)]
    pub const fn wsr(self) -> crate::common::Reg<regs::Wsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Watchdog Reset Status Register"]
    #[inline(always)]
    pub const fn wrsr(self) -> crate::common::Reg<regs::Wrsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Watchdog Interrupt Control Register"]
    #[inline(always)]
    pub const fn wicr(self) -> crate::common::Reg<regs::Wicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Watchdog Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn wmcr(self) -> crate::common::Reg<regs::Wmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
