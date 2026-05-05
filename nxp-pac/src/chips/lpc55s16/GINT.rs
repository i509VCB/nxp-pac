#[doc = "Group GPIO input interrupt (GINT0/1)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GINT {
    ptr: *mut u8,
}
unsafe impl Send for GINT {}
unsafe impl Sync for GINT {}
impl GINT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO grouped interrupt control register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPIO grouped interrupt port 0 polarity register."]
    #[inline(always)]
    pub const fn PORT_POL(self, n: usize) -> crate::common::Reg<regs::PORT_POL, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "GPIO grouped interrupt port 0 enable register."]
    #[inline(always)]
    pub const fn PORT_ENA(self, n: usize) -> crate::common::Reg<regs::PORT_ENA, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
