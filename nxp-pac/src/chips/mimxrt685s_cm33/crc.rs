#[doc = "LPC_Next0 CRC engine"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    ptr: *mut u8,
}
unsafe impl Send for Crc {}
unsafe impl Sync for Crc {}
impl Crc {
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
    #[doc = "16-bit data written to this register will be taken to perform CRC calculation with selected bit order and 1’s complement pre-process."]
    #[inline(always)]
    pub const fn wr_data16(self) -> crate::common::Reg<regs::WrData16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "32-bit data written to this register will be taken to perform CRC calculation with selected bit order and 1’s complement pre-process."]
    #[inline(always)]
    pub const fn wr_data32(self) -> crate::common::Reg<regs::WrData32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "8-bit data written to this register will be taken to perform CRC calculation with selected bit order and 1’s complement pre-process."]
    #[inline(always)]
    pub const fn wr_data8(self) -> crate::common::Reg<regs::WrData8, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
