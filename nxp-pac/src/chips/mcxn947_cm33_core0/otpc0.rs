#[doc = "OTPC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otpc0 {
    ptr: *mut u8,
}
unsafe impl Send for Otpc0 {}
unsafe impl Sync for Otpc0 {}
impl Otpc0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameters"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Read and Write Control"]
    #[inline(always)]
    pub const fn rwc(self) -> crate::common::Reg<regs::Rwc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Reload Control"]
    #[inline(always)]
    pub const fn rlc(self) -> crate::common::Reg<regs::Rlc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Power Control"]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Write Data"]
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<regs::Wdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Read Data"]
    #[inline(always)]
    pub const fn rdata(self) -> crate::common::Reg<regs::Rdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timing1"]
    #[inline(always)]
    pub const fn timing1(self) -> crate::common::Reg<regs::Timing1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Timing2"]
    #[inline(always)]
    pub const fn timing2(self) -> crate::common::Reg<regs::Timing2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Secure"]
    #[inline(always)]
    pub const fn secure(self) -> crate::common::Reg<regs::Secure, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Inverted Secure"]
    #[inline(always)]
    pub const fn secure_inv(self) -> crate::common::Reg<regs::SecureInv, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Debug and Key"]
    #[inline(always)]
    pub const fn dbg_key(self) -> crate::common::Reg<regs::DbgKey, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "MISC Config"]
    #[inline(always)]
    pub const fn misc_cfg(self) -> crate::common::Reg<regs::MiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "PHANTOM Config"]
    #[inline(always)]
    pub const fn phantom_cfg(self) -> crate::common::Reg<regs::PhantomCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Flexible Config 0"]
    #[inline(always)]
    pub const fn flex_cfg0(self) -> crate::common::Reg<regs::FlexCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Flexible Config 1"]
    #[inline(always)]
    pub const fn flex_cfg1(self) -> crate::common::Reg<regs::FlexCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
