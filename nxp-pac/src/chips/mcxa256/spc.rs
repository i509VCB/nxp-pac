#[doc = "SPC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spc {
    ptr: *mut u8,
}
unsafe impl Send for Spc {}
unsafe impl Sync for Spc {}
impl Spc {
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
    #[doc = "Status Control"]
    #[inline(always)]
    pub const fn sc(self) -> crate::common::Reg<regs::Sc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Low-Power Request Configuration"]
    #[inline(always)]
    pub const fn lpreq_cfg(self) -> crate::common::Reg<regs::LpreqCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPC Power Domain Mode Status"]
    #[inline(always)]
    pub const fn pd_status0(self) -> crate::common::Reg<regs::PdStatus0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SRAM Control"]
    #[inline(always)]
    pub const fn sramctl(self) -> crate::common::Reg<regs::Sramctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SRAM Retention Reference Trim"]
    #[inline(always)]
    pub const fn sramretldo_reftrim(
        self,
    ) -> crate::common::Reg<regs::SramretldoReftrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SRAM Retention LDO Control"]
    #[inline(always)]
    pub const fn sramretldo_cntrl(
        self,
    ) -> crate::common::Reg<regs::SramretldoCntrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Active Power Mode Configuration"]
    #[inline(always)]
    pub const fn active_cfg(self) -> crate::common::Reg<regs::ActiveCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Active Power Mode Configuration 1"]
    #[inline(always)]
    pub const fn active_cfg1(self) -> crate::common::Reg<regs::ActiveCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Low-Power Mode Configuration"]
    #[inline(always)]
    pub const fn lp_cfg(self) -> crate::common::Reg<regs::LpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Low Power Mode Configuration 1"]
    #[inline(always)]
    pub const fn lp_cfg1(self) -> crate::common::Reg<regs::LpCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Low Power Wake-Up Delay"]
    #[inline(always)]
    pub const fn lpwkup_delay(self) -> crate::common::Reg<regs::LpwkupDelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Active Voltage Trim Delay"]
    #[inline(always)]
    pub const fn active_vdelay(self) -> crate::common::Reg<regs::ActiveVdelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Voltage Detect Status"]
    #[inline(always)]
    pub const fn vd_stat(self) -> crate::common::Reg<regs::VdStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Core Voltage Detect Configuration"]
    #[inline(always)]
    pub const fn vd_core_cfg(self) -> crate::common::Reg<regs::VdCoreCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "System Voltage Detect Configuration"]
    #[inline(always)]
    pub const fn vd_sys_cfg(self) -> crate::common::Reg<regs::VdSysCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "External Voltage Domain Configuration"]
    #[inline(always)]
    pub const fn evd_cfg(self) -> crate::common::Reg<regs::EvdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "LDO_CORE Configuration"]
    #[inline(always)]
    pub const fn coreldo_cfg(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
}
pub mod regs;
pub mod vals;
