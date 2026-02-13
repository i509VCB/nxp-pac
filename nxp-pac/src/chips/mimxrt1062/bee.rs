#[doc = "Bus Encryption Engine"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bee {
    ptr: *mut u8,
}
unsafe impl Send for Bee {}
unsafe impl Sync for Bee {}
impl Bee {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Offset region 0 Register"]
    #[inline(always)]
    pub const fn addr_offset0(self) -> crate::common::Reg<regs::AddrOffset0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Offset region 1 Register"]
    #[inline(always)]
    pub const fn addr_offset1(self) -> crate::common::Reg<regs::AddrOffset1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AES Key 0 Register"]
    #[inline(always)]
    pub const fn aes_key0_w0(self) -> crate::common::Reg<regs::AesKey0W0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AES Key 1 Register"]
    #[inline(always)]
    pub const fn aes_key0_w1(self) -> crate::common::Reg<regs::AesKey0W1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "AES Key 2 Register"]
    #[inline(always)]
    pub const fn aes_key0_w2(self) -> crate::common::Reg<regs::AesKey0W2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "AES Key 3 Register"]
    #[inline(always)]
    pub const fn aes_key0_w3(self) -> crate::common::Reg<regs::AesKey0W3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "NONCE00 Register"]
    #[inline(always)]
    pub const fn ctr_nonce0_w0(self) -> crate::common::Reg<regs::CtrNonce0W0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "NONCE01 Register"]
    #[inline(always)]
    pub const fn ctr_nonce0_w1(self) -> crate::common::Reg<regs::CtrNonce0W1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "NONCE02 Register"]
    #[inline(always)]
    pub const fn ctr_nonce0_w2(self) -> crate::common::Reg<regs::CtrNonce0W2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "NONCE03 Register"]
    #[inline(always)]
    pub const fn ctr_nonce0_w3(self) -> crate::common::Reg<regs::CtrNonce0W3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "NONCE10 Register"]
    #[inline(always)]
    pub const fn ctr_nonce1_w0(self) -> crate::common::Reg<regs::CtrNonce1W0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "NONCE11 Register"]
    #[inline(always)]
    pub const fn ctr_nonce1_w1(self) -> crate::common::Reg<regs::CtrNonce1W1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "NONCE12 Register"]
    #[inline(always)]
    pub const fn ctr_nonce1_w2(self) -> crate::common::Reg<regs::CtrNonce1W2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "NONCE13 Register"]
    #[inline(always)]
    pub const fn ctr_nonce1_w3(self) -> crate::common::Reg<regs::CtrNonce1W3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Region1 Top Address Register"]
    #[inline(always)]
    pub const fn region1_top(self) -> crate::common::Reg<regs::Region1Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Region1 Bottom Address Register"]
    #[inline(always)]
    pub const fn region1_bot(self) -> crate::common::Reg<regs::Region1Bot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
pub mod vals;
