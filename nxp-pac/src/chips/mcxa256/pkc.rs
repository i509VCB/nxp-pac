#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkc {
    ptr: *mut u8,
}
unsafe impl Send for Pkc {}
unsafe impl Sync for Pkc {}
impl Pkc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn pkc_status(self) -> crate::common::Reg<regs::PkcStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn pkc_ctrl(self) -> crate::common::Reg<regs::PkcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn pkc_cfg(self) -> crate::common::Reg<regs::PkcCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Mode register, parameter set 1"]
    #[inline(always)]
    pub const fn pkc_mode1(self) -> crate::common::Reg<regs::PkcMode1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "X+Y pointer register, parameter set 1"]
    #[inline(always)]
    pub const fn pkc_xyptr1(self) -> crate::common::Reg<regs::PkcXyptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Z+R pointer register, parameter set 1"]
    #[inline(always)]
    pub const fn pkc_zrptr1(self) -> crate::common::Reg<regs::PkcZrptr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Length register, parameter set 1"]
    #[inline(always)]
    pub const fn pkc_len1(self) -> crate::common::Reg<regs::PkcLen1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Mode register, parameter set 2"]
    #[inline(always)]
    pub const fn pkc_mode2(self) -> crate::common::Reg<regs::PkcMode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "X+Y pointer register, parameter set 2"]
    #[inline(always)]
    pub const fn pkc_xyptr2(self) -> crate::common::Reg<regs::PkcXyptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Z+R pointer register, parameter set 2"]
    #[inline(always)]
    pub const fn pkc_zrptr2(self) -> crate::common::Reg<regs::PkcZrptr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Length register, parameter set 2"]
    #[inline(always)]
    pub const fn pkc_len2(self) -> crate::common::Reg<regs::PkcLen2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Universal pointer FUP program"]
    #[inline(always)]
    pub const fn pkc_uptr(self) -> crate::common::Reg<regs::PkcUptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Universal pointer FUP table"]
    #[inline(always)]
    pub const fn pkc_uptrt(self) -> crate::common::Reg<regs::PkcUptrt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Universal pointer length"]
    #[inline(always)]
    pub const fn pkc_ulen(self) -> crate::common::Reg<regs::PkcUlen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "MC pattern data interface"]
    #[inline(always)]
    pub const fn pkc_mcdata(self) -> crate::common::Reg<regs::PkcMcdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PKC version register"]
    #[inline(always)]
    pub const fn pkc_version(self) -> crate::common::Reg<regs::PkcVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub const fn pkc_soft_rst(self) -> crate::common::Reg<regs::PkcSoftRst, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb0usize) as _) }
    }
    #[doc = "Access Error"]
    #[inline(always)]
    pub const fn pkc_access_err(self) -> crate::common::Reg<regs::PkcAccessErr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "Clear Access Error"]
    #[inline(always)]
    pub const fn pkc_access_err_clr(
        self,
    ) -> crate::common::Reg<regs::PkcAccessErrClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "Interrupt enable clear"]
    #[inline(always)]
    pub const fn pkc_int_clr_enable(
        self,
    ) -> crate::common::Reg<regs::PkcIntClrEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "Interrupt enable set"]
    #[inline(always)]
    pub const fn pkc_int_set_enable(
        self,
    ) -> crate::common::Reg<regs::PkcIntSetEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "Interrupt status"]
    #[inline(always)]
    pub const fn pkc_int_status(self) -> crate::common::Reg<regs::PkcIntStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Interrupt enable"]
    #[inline(always)]
    pub const fn pkc_int_enable(self) -> crate::common::Reg<regs::PkcIntEnable, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Interrupt status clear"]
    #[inline(always)]
    pub const fn pkc_int_clr_status(
        self,
    ) -> crate::common::Reg<regs::PkcIntClrStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Interrupt status set"]
    #[inline(always)]
    pub const fn pkc_int_set_status(
        self,
    ) -> crate::common::Reg<regs::PkcIntSetStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Module ID"]
    #[inline(always)]
    pub const fn pkc_module_id(self) -> crate::common::Reg<regs::PkcModuleId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
