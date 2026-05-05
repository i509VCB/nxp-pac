#[doc = "MCU Debugger Mailbox."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DBGMAILBOX {
    ptr: *mut u8,
}
unsafe impl Send for DBGMAILBOX {}
unsafe impl Sync for DBGMAILBOX {}
impl DBGMAILBOX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CRC mode register."]
    #[inline(always)]
    pub const fn CSW(self) -> crate::common::Reg<regs::CSW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CRC seed register."]
    #[inline(always)]
    pub const fn REQUEST(self) -> crate::common::Reg<regs::REQUEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Return value from ROM."]
    #[inline(always)]
    pub const fn RETURN(self) -> crate::common::Reg<regs::RETURN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Identification register."]
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs;
