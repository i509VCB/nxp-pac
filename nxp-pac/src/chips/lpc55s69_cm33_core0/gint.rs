#[doc = "Group GPIO input interrupt (GINT0/1)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gint {
    ptr: *mut u8,
}
unsafe impl Send for Gint {}
unsafe impl Sync for Gint {}
impl Gint {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO grouped interrupt control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPIO grouped interrupt port 0 polarity register"]
    #[inline(always)]
    pub const fn port_pol(self, n: usize) -> crate::common::Reg<regs::PortPol, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "GPIO grouped interrupt port 0 enable register"]
    #[inline(always)]
    pub const fn port_ena(self, n: usize) -> crate::common::Reg<regs::PortEna, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
