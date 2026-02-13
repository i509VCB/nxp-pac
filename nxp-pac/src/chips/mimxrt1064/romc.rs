#[doc = "ROMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Romc {
    ptr: *mut u8,
}
unsafe impl Send for Romc {}
unsafe impl Sync for Romc {}
impl Romc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ROMC Data Registers"]
    #[inline(always)]
    pub const fn rompatchd(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Rompatchd, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize + n * 4usize) as _) }
    }
    #[doc = "ROMC Control Register"]
    #[inline(always)]
    pub const fn rompatchcntl(self) -> crate::common::Reg<regs::Rompatchcntl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "ROMC Enable Register High"]
    #[inline(always)]
    pub const fn rompatchenh(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "ROMC Enable Register Low"]
    #[inline(always)]
    pub const fn rompatchenl(self) -> crate::common::Reg<regs::Rompatchenl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "ROMC Address Registers"]
    #[inline(always)]
    pub const fn rompatcha(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Rompatcha, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "ROMC Status Register"]
    #[inline(always)]
    pub const fn rompatchsr(self) -> crate::common::Reg<regs::Rompatchsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
}
pub mod regs;
pub mod vals;
