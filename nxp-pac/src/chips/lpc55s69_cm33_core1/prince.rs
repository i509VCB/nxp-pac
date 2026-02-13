#[doc = "PRINCE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prince {
    ptr: *mut u8,
}
unsafe impl Send for Prince {}
unsafe impl Sync for Prince {}
impl Prince {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Encryption Enable register"]
    #[inline(always)]
    pub const fn enc_enable(self) -> crate::common::Reg<regs::EncEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Data Mask register, 32 Least Significant Bits"]
    #[inline(always)]
    pub const fn mask_lsb(self) -> crate::common::Reg<regs::MaskLsb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Data Mask register, 32 Most Significant Bits"]
    #[inline(always)]
    pub const fn mask_msb(self) -> crate::common::Reg<regs::MaskMsb, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Lock register"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Initial Vector register for region 0, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb0(self) -> crate::common::Reg<regs::IvLsb0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Initial Vector register for region 0, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb0(self) -> crate::common::Reg<regs::IvMsb0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Base Address for region 0 register"]
    #[inline(always)]
    pub const fn base_addr0(self) -> crate::common::Reg<regs::BaseAddr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 0"]
    #[inline(always)]
    pub const fn sr_enable0(self) -> crate::common::Reg<regs::SrEnable0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Initial Vector register for region 1, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb1(self) -> crate::common::Reg<regs::IvLsb1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Initial Vector register for region 1, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb1(self) -> crate::common::Reg<regs::IvMsb1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Base Address for region 1 register"]
    #[inline(always)]
    pub const fn base_addr1(self) -> crate::common::Reg<regs::BaseAddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 1"]
    #[inline(always)]
    pub const fn sr_enable1(self) -> crate::common::Reg<regs::SrEnable1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Initial Vector register for region 2, Least Significant Bits"]
    #[inline(always)]
    pub const fn iv_lsb2(self) -> crate::common::Reg<regs::IvLsb2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Initial Vector register for region 2, Most Significant Bits"]
    #[inline(always)]
    pub const fn iv_msb2(self) -> crate::common::Reg<regs::IvMsb2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Base Address for region 2 register"]
    #[inline(always)]
    pub const fn base_addr2(self) -> crate::common::Reg<regs::BaseAddr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Sub-Region Enable register for region 2"]
    #[inline(always)]
    pub const fn sr_enable2(self) -> crate::common::Reg<regs::SrEnable2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs;
