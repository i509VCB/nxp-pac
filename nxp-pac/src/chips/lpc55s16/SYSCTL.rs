#[doc = "system controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SYSCTL {
    ptr: *mut u8,
}
unsafe impl Send for SYSCTL {}
unsafe impl Sync for SYSCTL {}
impl SYSCTL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "update lock out control."]
    #[inline(always)]
    pub const fn UPDATELCKOUT(self) -> crate::common::Reg<regs::UPDATELCKOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Selects the source for SCK going into Flexcomm index."]
    #[inline(always)]
    pub const fn FCCTRLSEL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FCCTRLSEL, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Selects sources and data combinations for shared signal set index."]
    #[inline(always)]
    pub const fn SHAREDCTRLSET(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SHAREDCTRLSET, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Status register for USB HS."]
    #[inline(always)]
    pub const fn USB_HS_STATUS(self) -> crate::common::Reg<regs::USB_HS_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "CODE_GRAY LSB input Register."]
    #[inline(always)]
    pub const fn CODE_GRAY_LSB(self) -> crate::common::Reg<regs::CODE_GRAY_LSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "CODE_GRAY MSB input Register."]
    #[inline(always)]
    pub const fn CODE_GRAY_MSB(self) -> crate::common::Reg<regs::CODE_GRAY_MSB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "CODE_BIN LSB output Register."]
    #[inline(always)]
    pub const fn CODE_BIN_LSB(self) -> crate::common::Reg<regs::CODE_BIN_LSB, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "CODE_BIN MSB output Register."]
    #[inline(always)]
    pub const fn CODE_BIN_MSB(self) -> crate::common::Reg<regs::CODE_BIN_MSB, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
