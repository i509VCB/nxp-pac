#[doc = "FLEXRAM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexram {
    ptr: *mut u8,
}
unsafe impl Send for Flexram {}
unsafe impl Sync for Flexram {}
impl Flexram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TCM CRTL Register"]
    #[inline(always)]
    pub const fn tcm_ctrl(self) -> crate::common::Reg<regs::TcmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OCRAM Magic Address Register"]
    #[inline(always)]
    pub const fn ocram_magic_addr(
        self,
    ) -> crate::common::Reg<regs::OcramMagicAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DTCM Magic Address Register"]
    #[inline(always)]
    pub const fn dtcm_magic_addr(
        self,
    ) -> crate::common::Reg<regs::DtcmMagicAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ITCM Magic Address Register"]
    #[inline(always)]
    pub const fn itcm_magic_addr(
        self,
    ) -> crate::common::Reg<regs::ItcmMagicAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn int_stat_en(self) -> crate::common::Reg<regs::IntStatEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_sig_en(self) -> crate::common::Reg<regs::IntSigEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
