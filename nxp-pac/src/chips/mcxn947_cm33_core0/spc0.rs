#[doc = "SPC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spc0 {
    ptr: *mut u8,
}
unsafe impl Send for Spc0 {}
unsafe impl Sync for Spc0 {}
impl Spc0 {
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
    #[doc = "SPC Regulator Control"]
    #[inline(always)]
    pub const fn cntrl(self) -> crate::common::Reg<regs::Cntrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Low-Power Request Configuration"]
    #[inline(always)]
    pub const fn lpreq_cfg(self) -> crate::common::Reg<regs::LpreqCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPC Power Domain Mode Status"]
    #[inline(always)]
    pub const fn pd_status(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PdStatus, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "SRAM Control"]
    #[inline(always)]
    pub const fn sramctl(self) -> crate::common::Reg<regs::Sramctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
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
    #[doc = "IO Voltage Detect Configuration"]
    #[inline(always)]
    pub const fn vd_io_cfg(self) -> crate::common::Reg<regs::VdIoCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "External Voltage Domain Configuration"]
    #[inline(always)]
    pub const fn evd_cfg(self) -> crate::common::Reg<regs::EvdCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Glitch Detect Status Control"]
    #[inline(always)]
    pub const fn glitch_detect_sc(
        self,
    ) -> crate::common::Reg<regs::GlitchDetectSc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "LDO_CORE Configuration"]
    #[inline(always)]
    pub const fn coreldo_cfg(self) -> crate::common::Reg<regs::CoreldoCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_SYS Configuration"]
    #[inline(always)]
    pub const fn sysldo_cfg(self) -> crate::common::Reg<regs::SysldoCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "DCDC Configuration"]
    #[inline(always)]
    pub const fn dcdc_cfg(self) -> crate::common::Reg<regs::DcdcCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "DCDC Burst Configuration"]
    #[inline(always)]
    pub const fn dcdc_burst_cfg(self) -> crate::common::Reg<regs::DcdcBurstCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
}
pub mod regs;
pub mod vals;
