#[doc = "CRC engine."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CRC_ENGINE {
    ptr: *mut u8,
}
unsafe impl Send for CRC_ENGINE {}
unsafe impl Sync for CRC_ENGINE {}
impl CRC_ENGINE {
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
    pub const fn MODE(self) -> crate::common::Reg<regs::MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CRC seed register."]
    #[inline(always)]
    pub const fn SEED(self) -> crate::common::Reg<regs::SEED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CRC checksum register."]
    #[inline(always)]
    pub const fn SUM(self) -> crate::common::Reg<regs::SUM, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CRC data register."]
    #[inline(always)]
    pub const fn WR_DATA(self) -> crate::common::Reg<regs::WR_DATA, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
