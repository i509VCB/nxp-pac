#[doc = "system controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctl {
    ptr: *mut u8,
}
unsafe impl Send for Sysctl {}
unsafe impl Sync for Sysctl {}
impl Sysctl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "update lock out control"]
    #[inline(always)]
    pub const fn updatelckout(self) -> crate::common::Reg<regs::Updatelckout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Selects the source for SCK going into Flexcomm index"]
    #[inline(always)]
    pub const fn fcctrlsel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Fcctrlsel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub const fn sharedctrlset(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sharedctrlset, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Status register for USB HS"]
    #[inline(always)]
    pub const fn usb_hs_status(self) -> crate::common::Reg<regs::UsbHsStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
}
pub mod regs;
pub mod vals;
