#[doc = "CRC engine"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcEngine {
    ptr: *mut u8,
}
unsafe impl Send for CrcEngine {}
unsafe impl Sync for CrcEngine {}
impl CrcEngine {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CRC mode register"]
    #[inline(always)]
    pub const fn mode(self) -> crate::common::Reg<regs::Mode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CRC seed register"]
    #[inline(always)]
    pub const fn seed(self) -> crate::common::Reg<regs::Seed, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CRC checksum register"]
    #[inline(always)]
    pub const fn sum(self) -> crate::common::Reg<regs::Sum, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC data register"]
    #[inline(always)]
    pub const fn wr_data(self) -> crate::common::Reg<regs::WrData, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
