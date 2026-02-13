#[doc = "system ccontroller 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctl1 {
    ptr: *mut u8,
}
unsafe impl Send for Sysctl1 {}
unsafe impl Sync for Sysctl1 {}
impl Sysctl1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "mclk direction control"]
    #[inline(always)]
    pub const fn mclkpindir(self) -> crate::common::Reg<regs::Mclkpindir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DSP NMI source selection"]
    #[inline(always)]
    pub const fn dspnmisrcsel(self) -> crate::common::Reg<regs::Dspnmisrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "flexcomm control selection N"]
    #[inline(always)]
    pub const fn fcctrlsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Fcctrlsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "shared control set N"]
    #[inline(always)]
    pub const fn sharedctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sharedctrlset, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "RX Event Pulse Generator"]
    #[inline(always)]
    pub const fn rxevpulsegen(self) -> crate::common::Reg<regs::Rxevpulsegen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
}
pub mod regs;
pub mod vals;
