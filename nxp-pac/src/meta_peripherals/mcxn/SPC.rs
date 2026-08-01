#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "SPC."]
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
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status Control."]
    #[inline(always)]
    pub const fn sc(self) -> crate::pac::common::Reg<Sc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SPC Regulator Control."]
    #[inline(always)]
    pub const fn cntrl(self) -> crate::pac::common::Reg<Cntrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Low-Power Request Configuration."]
    #[inline(always)]
    pub const fn lpreq_cfg(self) -> crate::pac::common::Reg<LpreqCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SPC Power Domain Mode Status."]
    #[inline(always)]
    pub const fn pd_status(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<PdStatus, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _)
        }
    }
    #[doc = "SRAM Control."]
    #[inline(always)]
    pub const fn sramctl(self) -> crate::pac::common::Reg<Sramctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Active Power Mode Configuration."]
    #[inline(always)]
    pub const fn active_cfg(self) -> crate::pac::common::Reg<ActiveCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Active Power Mode Configuration 1."]
    #[inline(always)]
    pub const fn active_cfg1(self) -> crate::pac::common::Reg<ActiveCfg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Low-Power Mode Configuration."]
    #[inline(always)]
    pub const fn lp_cfg(self) -> crate::pac::common::Reg<LpCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Low Power Mode Configuration 1."]
    #[inline(always)]
    pub const fn lp_cfg1(self) -> crate::pac::common::Reg<LpCfg1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Low Power Wake-Up Delay."]
    #[inline(always)]
    pub const fn lpwkup_delay(
        self,
    ) -> crate::pac::common::Reg<LpwkupDelay, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Active Voltage Trim Delay."]
    #[inline(always)]
    pub const fn active_vdelay(
        self,
    ) -> crate::pac::common::Reg<ActiveVdelay, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Voltage Detect Status."]
    #[inline(always)]
    pub const fn vd_stat(self) -> crate::pac::common::Reg<VdStat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Core Voltage Detect Configuration."]
    #[inline(always)]
    pub const fn vd_core_cfg(self) -> crate::pac::common::Reg<VdCoreCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "System Voltage Detect Configuration."]
    #[inline(always)]
    pub const fn vd_sys_cfg(self) -> crate::pac::common::Reg<VdSysCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "IO Voltage Detect Configuration."]
    #[inline(always)]
    pub const fn vd_io_cfg(self) -> crate::pac::common::Reg<VdIoCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "External Voltage Domain Configuration."]
    #[inline(always)]
    pub const fn evd_cfg(self) -> crate::pac::common::Reg<EvdCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Glitch Detect Status Control."]
    #[inline(always)]
    pub const fn glitch_detect_sc(
        self,
    ) -> crate::pac::common::Reg<GlitchDetectSc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "LDO_CORE Configuration."]
    #[inline(always)]
    pub const fn coreldo_cfg(self) -> crate::pac::common::Reg<CoreldoCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "LDO_SYS Configuration."]
    #[inline(always)]
    pub const fn sysldo_cfg(self) -> crate::pac::common::Reg<SysldoCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "DCDC Configuration."]
    #[inline(always)]
    pub const fn dcdc_cfg(self) -> crate::pac::common::Reg<DcdcCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "DCDC Burst Configuration."]
    #[inline(always)]
    pub const fn dcdc_burst_cfg(
        self,
    ) -> crate::pac::common::Reg<DcdcBurstCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
}
#[doc = "Active Power Mode Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg(pub u32);
impl ActiveCfg {
    #[doc = "LDO_CORE VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> ActiveCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        ActiveCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: ActiveCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> ActiveCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        ActiveCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level."]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: ActiveCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LDO_SYS VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_ds(&self) -> ActiveCfgSysldoVddDs {
        let val = (self.0 >> 4usize) & 0x01;
        ActiveCfgSysldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_sysldo_vdd_ds(&mut self, val: ActiveCfgSysldoVddDs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO_SYS VDD Regulator Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_lvl(&self) -> SysldoVddLvl {
        let val = (self.0 >> 6usize) & 0x01;
        SysldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Regulator Voltage Level."]
    #[inline(always)]
    pub const fn set_sysldo_vdd_lvl(&mut self, val: SysldoVddLvl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DCDC VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_ds(&self) -> ActiveCfgDcdcVddDs {
        let val = (self.0 >> 8usize) & 0x03;
        ActiveCfgDcdcVddDs::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_dcdc_vdd_ds(&mut self, val: ActiveCfgDcdcVddDs) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DCDC VDD Regulator Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_lvl(&self) -> ActiveCfgDcdcVddLvl {
        let val = (self.0 >> 10usize) & 0x03;
        ActiveCfgDcdcVddLvl::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Regulator Voltage Level."]
    #[inline(always)]
    pub const fn set_dcdc_vdd_lvl(&mut self, val: ActiveCfgDcdcVddLvl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Glitch Detect Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Disable."]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMP Bandgap Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable."]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> ActiveCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        ActiveCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode."]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: ActiveCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "VDD Voltage Detect Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdd_vd_disable(&self) -> VddVdDisable {
        let val = (self.0 >> 23usize) & 0x01;
        VddVdDisable::from_bits(val as u8)
    }
    #[doc = "VDD Voltage Detect Disable."]
    #[inline(always)]
    pub const fn set_vdd_vd_disable(&mut self, val: VddVdDisable) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "IO Low-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn io_lvde(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "IO Low-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_io_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Core High-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "IO High-Voltage Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn io_hvde(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IO High-Voltage Detection Enable."]
    #[inline(always)]
    pub const fn set_io_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ActiveCfg {
    #[inline(always)]
    fn default() -> ActiveCfg {
        ActiveCfg(0)
    }
}
impl core::fmt::Debug for ActiveCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveCfg")
            .field("coreldo_vdd_ds", &self.coreldo_vdd_ds())
            .field("coreldo_vdd_lvl", &self.coreldo_vdd_lvl())
            .field("sysldo_vdd_ds", &self.sysldo_vdd_ds())
            .field("sysldo_vdd_lvl", &self.sysldo_vdd_lvl())
            .field("dcdc_vdd_ds", &self.dcdc_vdd_ds())
            .field("dcdc_vdd_lvl", &self.dcdc_vdd_lvl())
            .field("glitch_detect_disable", &self.glitch_detect_disable())
            .field("lpbuff_en", &self.lpbuff_en())
            .field("bgmode", &self.bgmode())
            .field("vdd_vd_disable", &self.vdd_vd_disable())
            .field("core_lvde", &self.core_lvde())
            .field("sys_lvde", &self.sys_lvde())
            .field("io_lvde", &self.io_lvde())
            .field("core_hvde", &self.core_hvde())
            .field("sys_hvde", &self.sys_hvde())
            .field("io_hvde", &self.io_hvde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ActiveCfg {{ coreldo_vdd_ds: {:?}, coreldo_vdd_lvl: {:?}, sysldo_vdd_ds: {:?}, sysldo_vdd_lvl: {:?}, dcdc_vdd_ds: {:?}, dcdc_vdd_lvl: {:?}, glitch_detect_disable: {=bool:?}, lpbuff_en: {=bool:?}, bgmode: {:?}, vdd_vd_disable: {:?}, core_lvde: {=bool:?}, sys_lvde: {=bool:?}, io_lvde: {=bool:?}, core_hvde: {=bool:?}, sys_hvde: {=bool:?}, io_hvde: {=bool:?} }}",
            self.coreldo_vdd_ds(),
            self.coreldo_vdd_lvl(),
            self.sysldo_vdd_ds(),
            self.sysldo_vdd_lvl(),
            self.dcdc_vdd_ds(),
            self.dcdc_vdd_lvl(),
            self.glitch_detect_disable(),
            self.lpbuff_en(),
            self.bgmode(),
            self.vdd_vd_disable(),
            self.core_lvde(),
            self.sys_lvde(),
            self.io_lvde(),
            self.core_hvde(),
            self.sys_hvde(),
            self.io_hvde()
        )
    }
}
#[doc = "Active Power Mode Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg1(pub u32);
impl ActiveCfg1 {
    #[doc = "Active Config Chip Control."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Active Config Chip Control."]
    #[inline(always)]
    pub const fn set_soc_cntrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ActiveCfg1 {
    #[inline(always)]
    fn default() -> ActiveCfg1 {
        ActiveCfg1(0)
    }
}
impl core::fmt::Debug for ActiveCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveCfg1")
            .field("soc_cntrl", &self.soc_cntrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ActiveCfg1 {{ soc_cntrl: {=u32:?} }}", self.soc_cntrl())
    }
}
#[doc = "Active Voltage Trim Delay."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveVdelay(pub u32);
impl ActiveVdelay {
    #[doc = "Active Voltage Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn active_vdelay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Active Voltage Delay."]
    #[inline(always)]
    pub const fn set_active_vdelay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ActiveVdelay {
    #[inline(always)]
    fn default() -> ActiveVdelay {
        ActiveVdelay(0)
    }
}
impl core::fmt::Debug for ActiveVdelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ActiveVdelay")
            .field("active_vdelay", &self.active_vdelay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ActiveVdelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ActiveVdelay {{ active_vdelay: {=u16:?} }}",
            self.active_vdelay()
        )
    }
}
#[doc = "SPC Regulator Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntrl(pub u32);
impl Cntrl {
    #[doc = "LDO_CORE Regulator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Regulator Enable."]
    #[inline(always)]
    pub const fn set_coreldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_SYS Regulator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_SYS Regulator Enable."]
    #[inline(always)]
    pub const fn set_sysldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DCDC_CORE Regulator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC_CORE Regulator Enable."]
    #[inline(always)]
    pub const fn set_dcdc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cntrl {
    #[inline(always)]
    fn default() -> Cntrl {
        Cntrl(0)
    }
}
impl core::fmt::Debug for Cntrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntrl")
            .field("coreldo_en", &self.coreldo_en())
            .field("sysldo_en", &self.sysldo_en())
            .field("dcdc_en", &self.dcdc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cntrl {{ coreldo_en: {=bool:?}, sysldo_en: {=bool:?}, dcdc_en: {=bool:?} }}",
            self.coreldo_en(),
            self.sysldo_en(),
            self.dcdc_en()
        )
    }
}
#[doc = "LDO_CORE Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreldoCfg(pub u32);
impl CoreldoCfg {
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn dpdown_pulldown_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable."]
    #[inline(always)]
    pub const fn set_dpdown_pulldown_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for CoreldoCfg {
    #[inline(always)]
    fn default() -> CoreldoCfg {
        CoreldoCfg(0)
    }
}
impl core::fmt::Debug for CoreldoCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CoreldoCfg")
            .field("dpdown_pulldown_disable", &self.dpdown_pulldown_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CoreldoCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CoreldoCfg {{ dpdown_pulldown_disable: {=bool:?} }}",
            self.dpdown_pulldown_disable()
        )
    }
}
#[doc = "DCDC Burst Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcBurstCfg(pub u32);
impl DcdcBurstCfg {
    #[doc = "Software Burst Request."]
    #[must_use]
    #[inline(always)]
    pub const fn burst_req(&self) -> BurstReq {
        let val = (self.0 >> 0usize) & 0x01;
        BurstReq::from_bits(val as u8)
    }
    #[doc = "Software Burst Request."]
    #[inline(always)]
    pub const fn set_burst_req(&mut self, val: BurstReq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "External Burst Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_burst_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Burst Request Enable."]
    #[inline(always)]
    pub const fn set_ext_burst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Acknowledge Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn burst_ack(&self) -> BurstAck {
        let val = (self.0 >> 3usize) & 0x01;
        BurstAck::from_bits(val as u8)
    }
    #[doc = "Burst Acknowledge Flag."]
    #[inline(always)]
    pub const fn set_burst_ack(&mut self, val: BurstAck) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Refresh Count Value."]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_refresh_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Refresh Count Value."]
    #[inline(always)]
    pub const fn set_pulse_refresh_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DcdcBurstCfg {
    #[inline(always)]
    fn default() -> DcdcBurstCfg {
        DcdcBurstCfg(0)
    }
}
impl core::fmt::Debug for DcdcBurstCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcBurstCfg")
            .field("burst_req", &self.burst_req())
            .field("ext_burst_en", &self.ext_burst_en())
            .field("burst_ack", &self.burst_ack())
            .field("pulse_refresh_cnt", &self.pulse_refresh_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcBurstCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcBurstCfg {{ burst_req: {:?}, ext_burst_en: {=bool:?}, burst_ack: {:?}, pulse_refresh_cnt: {=u16:?} }}",
            self.burst_req(),
            self.ext_burst_en(),
            self.burst_ack(),
            self.pulse_refresh_cnt()
        )
    }
}
#[doc = "DCDC Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCfg(pub u32);
impl DcdcCfg {
    #[doc = "DCDC Burst Frequency Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_cntrl_on(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Burst Frequency Control Enable."]
    #[inline(always)]
    pub const fn set_freq_cntrl_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DCDC Burst Frequency Control."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_cntrl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "DCDC Burst Frequency Control."]
    #[inline(always)]
    pub const fn set_freq_cntrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "DCDC Bleed Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bleed_en(&self) -> BleedEn {
        let val = (self.0 >> 19usize) & 0x01;
        BleedEn::from_bits(val as u8)
    }
    #[doc = "DCDC Bleed Enable."]
    #[inline(always)]
    pub const fn set_bleed_en(&mut self, val: BleedEn) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for DcdcCfg {
    #[inline(always)]
    fn default() -> DcdcCfg {
        DcdcCfg(0)
    }
}
impl core::fmt::Debug for DcdcCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcdcCfg")
            .field("freq_cntrl_on", &self.freq_cntrl_on())
            .field("freq_cntrl", &self.freq_cntrl())
            .field("bleed_en", &self.bleed_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcdcCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcdcCfg {{ freq_cntrl_on: {=bool:?}, freq_cntrl: {=u8:?}, bleed_en: {:?} }}",
            self.freq_cntrl_on(),
            self.freq_cntrl(),
            self.bleed_en()
        )
    }
}
#[doc = "External Voltage Domain Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvdCfg(pub u32);
impl EvdCfg {
    #[doc = "External Voltage Domain Isolation."]
    #[must_use]
    #[inline(always)]
    pub const fn evdiso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Isolation."]
    #[inline(always)]
    pub const fn set_evdiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "External Voltage Domain Low-Power Isolation."]
    #[must_use]
    #[inline(always)]
    pub const fn evdlpiso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Low-Power Isolation."]
    #[inline(always)]
    pub const fn set_evdlpiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "External Voltage Domain Status."]
    #[must_use]
    #[inline(always)]
    pub const fn evdstat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Status."]
    #[inline(always)]
    pub const fn set_evdstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for EvdCfg {
    #[inline(always)]
    fn default() -> EvdCfg {
        EvdCfg(0)
    }
}
impl core::fmt::Debug for EvdCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvdCfg")
            .field("evdiso", &self.evdiso())
            .field("evdlpiso", &self.evdlpiso())
            .field("evdstat", &self.evdstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvdCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvdCfg {{ evdiso: {=u8:?}, evdlpiso: {=u8:?}, evdstat: {=u8:?} }}",
            self.evdiso(),
            self.evdlpiso(),
            self.evdstat()
        )
    }
}
#[doc = "Glitch Detect Status Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlitchDetectSc(pub u32);
impl GlitchDetectSc {
    #[doc = "Counter Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_select(&self) -> CntSelect {
        let val = (self.0 >> 0usize) & 0x03;
        CntSelect::from_bits(val as u8)
    }
    #[doc = "Counter Select."]
    #[inline(always)]
    pub const fn set_cnt_select(&mut self, val: CntSelect) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout."]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Glitch Detect Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GLITCH_DETECT_FLAG."]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG."]
    #[inline(always)]
    pub const fn set_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for GlitchDetectSc {
    #[inline(always)]
    fn default() -> GlitchDetectSc {
        GlitchDetectSc(0)
    }
}
impl core::fmt::Debug for GlitchDetectSc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GlitchDetectSc")
            .field("cnt_select", &self.cnt_select())
            .field("timeout", &self.timeout())
            .field("re", &self.re())
            .field("ie", &self.ie())
            .field("glitch_detect_flag", &self.glitch_detect_flag())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GlitchDetectSc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GlitchDetectSc {{ cnt_select: {:?}, timeout: {=u8:?}, re: {=bool:?}, ie: {=bool:?}, glitch_detect_flag: {=u8:?}, lock: {=bool:?} }}",
            self.cnt_select(),
            self.timeout(),
            self.re(),
            self.ie(),
            self.glitch_detect_flag(),
            self.lock()
        )
    }
}
#[doc = "Low-Power Mode Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg(pub u32);
impl LpCfg {
    #[doc = "LDO_CORE VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> LpCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        LpCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: LpCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> LpCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        LpCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level."]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: LpCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LDO_SYS VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_ds(&self) -> LpCfgSysldoVddDs {
        let val = (self.0 >> 4usize) & 0x01;
        LpCfgSysldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_sysldo_vdd_ds(&mut self, val: LpCfgSysldoVddDs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DCDC VDD Drive Strength."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_ds(&self) -> LpCfgDcdcVddDs {
        let val = (self.0 >> 8usize) & 0x03;
        LpCfgDcdcVddDs::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Drive Strength."]
    #[inline(always)]
    pub const fn set_dcdc_vdd_ds(&mut self, val: LpCfgDcdcVddDs) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DCDC VDD Regulator Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_lvl(&self) -> LpCfgDcdcVddLvl {
        let val = (self.0 >> 10usize) & 0x03;
        LpCfgDcdcVddLvl::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Regulator Voltage Level."]
    #[inline(always)]
    pub const fn set_dcdc_vdd_lvl(&mut self, val: LpCfgDcdcVddLvl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Glitch Detect Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> LpCfgGlitchDetectDisable {
        let val = (self.0 >> 12usize) & 0x01;
        LpCfgGlitchDetectDisable::from_bits(val as u8)
    }
    #[doc = "Glitch Detect Disable."]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: LpCfgGlitchDetectDisable) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CORE VDD Internal Voltage Scaling (IVS) Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_ivs_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CORE VDD Internal Voltage Scaling (IVS) Enable."]
    #[inline(always)]
    pub const fn set_corevdd_ivs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CMP Bandgap Buffer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable."]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> LpCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        LpCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode."]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: LpCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Low-Power IREF Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_irefen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power IREF Enable."]
    #[inline(always)]
    pub const fn set_lp_irefen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "IO Low Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn io_lvde(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "IO Low Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_io_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Core High Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "IO High Voltage Detect Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn io_hvde(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IO High Voltage Detect Enable."]
    #[inline(always)]
    pub const fn set_io_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for LpCfg {
    #[inline(always)]
    fn default() -> LpCfg {
        LpCfg(0)
    }
}
impl core::fmt::Debug for LpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpCfg")
            .field("coreldo_vdd_ds", &self.coreldo_vdd_ds())
            .field("coreldo_vdd_lvl", &self.coreldo_vdd_lvl())
            .field("sysldo_vdd_ds", &self.sysldo_vdd_ds())
            .field("dcdc_vdd_ds", &self.dcdc_vdd_ds())
            .field("dcdc_vdd_lvl", &self.dcdc_vdd_lvl())
            .field("glitch_detect_disable", &self.glitch_detect_disable())
            .field("corevdd_ivs_en", &self.corevdd_ivs_en())
            .field("lpbuff_en", &self.lpbuff_en())
            .field("bgmode", &self.bgmode())
            .field("lp_irefen", &self.lp_irefen())
            .field("core_lvde", &self.core_lvde())
            .field("sys_lvde", &self.sys_lvde())
            .field("io_lvde", &self.io_lvde())
            .field("core_hvde", &self.core_hvde())
            .field("sys_hvde", &self.sys_hvde())
            .field("io_hvde", &self.io_hvde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpCfg {{ coreldo_vdd_ds: {:?}, coreldo_vdd_lvl: {:?}, sysldo_vdd_ds: {:?}, dcdc_vdd_ds: {:?}, dcdc_vdd_lvl: {:?}, glitch_detect_disable: {:?}, corevdd_ivs_en: {=bool:?}, lpbuff_en: {=bool:?}, bgmode: {:?}, lp_irefen: {=bool:?}, core_lvde: {=bool:?}, sys_lvde: {=bool:?}, io_lvde: {=bool:?}, core_hvde: {=bool:?}, sys_hvde: {=bool:?}, io_hvde: {=bool:?} }}",
            self.coreldo_vdd_ds(),
            self.coreldo_vdd_lvl(),
            self.sysldo_vdd_ds(),
            self.dcdc_vdd_ds(),
            self.dcdc_vdd_lvl(),
            self.glitch_detect_disable(),
            self.corevdd_ivs_en(),
            self.lpbuff_en(),
            self.bgmode(),
            self.lp_irefen(),
            self.core_lvde(),
            self.sys_lvde(),
            self.io_lvde(),
            self.core_hvde(),
            self.sys_hvde(),
            self.io_hvde()
        )
    }
}
#[doc = "Low Power Mode Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg1(pub u32);
impl LpCfg1 {
    #[doc = "Low-Power Configuration Chip Control."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Low-Power Configuration Chip Control."]
    #[inline(always)]
    pub const fn set_soc_cntrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LpCfg1 {
    #[inline(always)]
    fn default() -> LpCfg1 {
        LpCfg1(0)
    }
}
impl core::fmt::Debug for LpCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpCfg1")
            .field("soc_cntrl", &self.soc_cntrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpCfg1 {{ soc_cntrl: {=u32:?} }}", self.soc_cntrl())
    }
}
#[doc = "Low-Power Request Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpreqCfg(pub u32);
impl LpreqCfg {
    #[doc = "Low-Power Request Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqoe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Request Output Enable."]
    #[inline(always)]
    pub const fn set_lpreqoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low-Power Request Output Pin Polarity Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqpol(&self) -> Lpreqpol {
        let val = (self.0 >> 1usize) & 0x01;
        Lpreqpol::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Pin Polarity Control."]
    #[inline(always)]
    pub const fn set_lpreqpol(&mut self, val: Lpreqpol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-Power Request Output Override."]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqov(&self) -> Lpreqov {
        let val = (self.0 >> 2usize) & 0x03;
        Lpreqov::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Override."]
    #[inline(always)]
    pub const fn set_lpreqov(&mut self, val: Lpreqov) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for LpreqCfg {
    #[inline(always)]
    fn default() -> LpreqCfg {
        LpreqCfg(0)
    }
}
impl core::fmt::Debug for LpreqCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpreqCfg")
            .field("lpreqoe", &self.lpreqoe())
            .field("lpreqpol", &self.lpreqpol())
            .field("lpreqov", &self.lpreqov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpreqCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpreqCfg {{ lpreqoe: {=bool:?}, lpreqpol: {:?}, lpreqov: {:?} }}",
            self.lpreqoe(),
            self.lpreqpol(),
            self.lpreqov()
        )
    }
}
#[doc = "Low Power Wake-Up Delay."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpwkupDelay(pub u32);
impl LpwkupDelay {
    #[doc = "Low-Power Wake-Up Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn lpwkup_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low-Power Wake-Up Delay."]
    #[inline(always)]
    pub const fn set_lpwkup_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LpwkupDelay {
    #[inline(always)]
    fn default() -> LpwkupDelay {
        LpwkupDelay(0)
    }
}
impl core::fmt::Debug for LpwkupDelay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpwkupDelay")
            .field("lpwkup_delay", &self.lpwkup_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpwkupDelay {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpwkupDelay {{ lpwkup_delay: {=u16:?} }}",
            self.lpwkup_delay()
        )
    }
}
#[doc = "SPC Power Domain Mode Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdStatus(pub u32);
impl PdStatus {
    #[doc = "Power Request Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req_status(&self) -> PwrReqStatus {
        let val = (self.0 >> 0usize) & 0x01;
        PwrReqStatus::from_bits(val as u8)
    }
    #[doc = "Power Request Status Flag."]
    #[inline(always)]
    pub const fn set_pwr_req_status(&mut self, val: PwrReqStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power Domain Low Power Request Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pd_lp_req(&self) -> PdLpReq {
        let val = (self.0 >> 4usize) & 0x01;
        PdLpReq::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Request Flag."]
    #[inline(always)]
    pub const fn set_pd_lp_req(&mut self, val: PdLpReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Power Domain Low Power Mode Request."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> LpMode {
        let val = (self.0 >> 8usize) & 0x0f;
        LpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Mode Request."]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: LpMode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for PdStatus {
    #[inline(always)]
    fn default() -> PdStatus {
        PdStatus(0)
    }
}
impl core::fmt::Debug for PdStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PdStatus")
            .field("pwr_req_status", &self.pwr_req_status())
            .field("pd_lp_req", &self.pd_lp_req())
            .field("lp_mode", &self.lp_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PdStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PdStatus {{ pwr_req_status: {:?}, pd_lp_req: {:?}, lp_mode: {:?} }}",
            self.pwr_req_status(),
            self.pd_lp_req(),
            self.lp_mode()
        )
    }
}
#[doc = "Status Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc = "SPC Busy Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> Busy {
        let val = (self.0 >> 0usize) & 0x01;
        Busy::from_bits(val as u8)
    }
    #[doc = "SPC Busy Status Flag."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPC Power Mode Configuration Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_req(&self) -> SpcLpReq {
        let val = (self.0 >> 1usize) & 0x01;
        SpcLpReq::from_bits(val as u8)
    }
    #[doc = "SPC Power Mode Configuration Status Flag."]
    #[inline(always)]
    pub const fn set_spc_lp_req(&mut self, val: SpcLpReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Power Domain Low-Power Mode Request."]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_mode(&self) -> SpcLpMode {
        let val = (self.0 >> 4usize) & 0x0f;
        SpcLpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low-Power Mode Request."]
    #[inline(always)]
    pub const fn set_spc_lp_mode(&mut self, val: SpcLpMode) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Isolation Clear Flags."]
    #[must_use]
    #[inline(always)]
    pub const fn iso_clr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Isolation Clear Flags."]
    #[inline(always)]
    pub const fn set_iso_clr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Sc {
    #[inline(always)]
    fn default() -> Sc {
        Sc(0)
    }
}
impl core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sc")
            .field("busy", &self.busy())
            .field("spc_lp_req", &self.spc_lp_req())
            .field("spc_lp_mode", &self.spc_lp_mode())
            .field("iso_clr", &self.iso_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sc {{ busy: {:?}, spc_lp_req: {:?}, spc_lp_mode: {:?}, iso_clr: {=u8:?} }}",
            self.busy(),
            self.spc_lp_req(),
            self.spc_lp_mode(),
            self.iso_clr()
        )
    }
}
#[doc = "SRAM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramctl(pub u32);
impl Sramctl {
    #[doc = "Voltage Select Margin."]
    #[must_use]
    #[inline(always)]
    pub const fn vsm(&self) -> Vsm {
        let val = (self.0 >> 0usize) & 0x03;
        Vsm::from_bits(val as u8)
    }
    #[doc = "Voltage Select Margin."]
    #[inline(always)]
    pub const fn set_vsm(&mut self, val: Vsm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SRAM Voltage Update Request."]
    #[must_use]
    #[inline(always)]
    pub const fn req(&self) -> Req {
        let val = (self.0 >> 30usize) & 0x01;
        Req::from_bits(val as u8)
    }
    #[doc = "SRAM Voltage Update Request."]
    #[inline(always)]
    pub const fn set_req(&mut self, val: Req) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SRAM Voltage Update Request Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> Ack {
        let val = (self.0 >> 31usize) & 0x01;
        Ack::from_bits(val as u8)
    }
    #[doc = "SRAM Voltage Update Request Acknowledge."]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: Ack) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sramctl {
    #[inline(always)]
    fn default() -> Sramctl {
        Sramctl(0)
    }
}
impl core::fmt::Debug for Sramctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramctl")
            .field("vsm", &self.vsm())
            .field("req", &self.req())
            .field("ack", &self.ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramctl {{ vsm: {:?}, req: {:?}, ack: {:?} }}",
            self.vsm(),
            self.req(),
            self.ack()
        )
    }
}
#[doc = "LDO_SYS Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysldoCfg(pub u32);
impl SysldoCfg {
    #[doc = "Current Sink Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn isinken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Sink Enable."]
    #[inline(always)]
    pub const fn set_isinken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SysldoCfg {
    #[inline(always)]
    fn default() -> SysldoCfg {
        SysldoCfg(0)
    }
}
impl core::fmt::Debug for SysldoCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysldoCfg")
            .field("isinken", &self.isinken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysldoCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SysldoCfg {{ isinken: {=bool:?} }}", self.isinken())
    }
}
#[doc = "Core Voltage Detect Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdCoreCfg(pub u32);
impl VdCoreCfg {
    #[doc = "Core LVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Reset Enable."]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Core LVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Core VDD HVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Reset Enable."]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Core VDD HVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Core Voltage Detect Reset Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> VdCoreCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        VdCoreCfgLock::from_bits(val as u8)
    }
    #[doc = "Core Voltage Detect Reset Enable Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: VdCoreCfgLock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for VdCoreCfg {
    #[inline(always)]
    fn default() -> VdCoreCfg {
        VdCoreCfg(0)
    }
}
impl core::fmt::Debug for VdCoreCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdCoreCfg")
            .field("lvdre", &self.lvdre())
            .field("lvdie", &self.lvdie())
            .field("hvdre", &self.hvdre())
            .field("hvdie", &self.hvdie())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdCoreCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdCoreCfg {{ lvdre: {=bool:?}, lvdie: {=bool:?}, hvdre: {=bool:?}, hvdie: {=bool:?}, lock: {:?} }}",
            self.lvdre(),
            self.lvdie(),
            self.hvdre(),
            self.hvdie(),
            self.lock()
        )
    }
}
#[doc = "IO Voltage Detect Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdIoCfg(pub u32);
impl VdIoCfg {
    #[doc = "IO VDD LVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD LVD Reset Enable."]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO VDD LVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD LVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO VDD HVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD HVD Reset Enable."]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO VDD HVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD HVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO VDD Low-Voltage Level Select."]
    #[must_use]
    #[inline(always)]
    pub const fn lvsel(&self) -> Lvsel {
        let val = (self.0 >> 8usize) & 0x01;
        Lvsel::from_bits(val as u8)
    }
    #[doc = "IO VDD Low-Voltage Level Select."]
    #[inline(always)]
    pub const fn set_lvsel(&mut self, val: Lvsel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "IO Voltage Detect Reset Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> VdIoCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        VdIoCfgLock::from_bits(val as u8)
    }
    #[doc = "IO Voltage Detect Reset Enable Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: VdIoCfgLock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for VdIoCfg {
    #[inline(always)]
    fn default() -> VdIoCfg {
        VdIoCfg(0)
    }
}
impl core::fmt::Debug for VdIoCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdIoCfg")
            .field("lvdre", &self.lvdre())
            .field("lvdie", &self.lvdie())
            .field("hvdre", &self.hvdre())
            .field("hvdie", &self.hvdie())
            .field("lvsel", &self.lvsel())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdIoCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdIoCfg {{ lvdre: {=bool:?}, lvdie: {=bool:?}, hvdre: {=bool:?}, hvdie: {=bool:?}, lvsel: {:?}, lock: {:?} }}",
            self.lvdre(),
            self.lvdie(),
            self.hvdre(),
            self.hvdie(),
            self.lvsel(),
            self.lock()
        )
    }
}
#[doc = "Voltage Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdStat(pub u32);
impl VdStat {
    #[doc = "Core Low-Voltage Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_lvdf(&self) -> CorevddLvdf {
        let val = (self.0 >> 0usize) & 0x01;
        CorevddLvdf::from_bits(val as u8)
    }
    #[doc = "Core Low-Voltage Detect Flag."]
    #[inline(always)]
    pub const fn set_corevdd_lvdf(&mut self, val: CorevddLvdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "System Low-Voltage Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_lvdf(&self) -> SysvddLvdf {
        let val = (self.0 >> 1usize) & 0x01;
        SysvddLvdf::from_bits(val as u8)
    }
    #[doc = "System Low-Voltage Detect Flag."]
    #[inline(always)]
    pub const fn set_sysvdd_lvdf(&mut self, val: SysvddLvdf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IO VDD LVD Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn iovdd_lvdf(&self) -> IovddLvdf {
        let val = (self.0 >> 2usize) & 0x01;
        IovddLvdf::from_bits(val as u8)
    }
    #[doc = "IO VDD LVD Flag."]
    #[inline(always)]
    pub const fn set_iovdd_lvdf(&mut self, val: IovddLvdf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Core VDD HVD Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_hvdf(&self) -> CorevddHvdf {
        let val = (self.0 >> 4usize) & 0x01;
        CorevddHvdf::from_bits(val as u8)
    }
    #[doc = "Core VDD HVD Flag."]
    #[inline(always)]
    pub const fn set_corevdd_hvdf(&mut self, val: CorevddHvdf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "System HVD Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_hvdf(&self) -> SysvddHvdf {
        let val = (self.0 >> 5usize) & 0x01;
        SysvddHvdf::from_bits(val as u8)
    }
    #[doc = "System HVD Flag."]
    #[inline(always)]
    pub const fn set_sysvdd_hvdf(&mut self, val: SysvddHvdf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IO VDD HVD Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn iovdd_hvdf(&self) -> IovddHvdf {
        let val = (self.0 >> 6usize) & 0x01;
        IovddHvdf::from_bits(val as u8)
    }
    #[doc = "IO VDD HVD Flag."]
    #[inline(always)]
    pub const fn set_iovdd_hvdf(&mut self, val: IovddHvdf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for VdStat {
    #[inline(always)]
    fn default() -> VdStat {
        VdStat(0)
    }
}
impl core::fmt::Debug for VdStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdStat")
            .field("corevdd_lvdf", &self.corevdd_lvdf())
            .field("sysvdd_lvdf", &self.sysvdd_lvdf())
            .field("iovdd_lvdf", &self.iovdd_lvdf())
            .field("corevdd_hvdf", &self.corevdd_hvdf())
            .field("sysvdd_hvdf", &self.sysvdd_hvdf())
            .field("iovdd_hvdf", &self.iovdd_hvdf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdStat {{ corevdd_lvdf: {:?}, sysvdd_lvdf: {:?}, iovdd_lvdf: {:?}, corevdd_hvdf: {:?}, sysvdd_hvdf: {:?}, iovdd_hvdf: {:?} }}",
            self.corevdd_lvdf(),
            self.sysvdd_lvdf(),
            self.iovdd_lvdf(),
            self.corevdd_hvdf(),
            self.sysvdd_hvdf(),
            self.iovdd_hvdf()
        )
    }
}
#[doc = "System Voltage Detect Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdSysCfg(pub u32);
impl VdSysCfg {
    #[doc = "System LVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Reset Enable."]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System LVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System HVD Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Reset Enable."]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "System HVD Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Interrupt Enable."]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Voltage Detect Reset Enable Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> VdSysCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        VdSysCfgLock::from_bits(val as u8)
    }
    #[doc = "System Voltage Detect Reset Enable Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: VdSysCfgLock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for VdSysCfg {
    #[inline(always)]
    fn default() -> VdSysCfg {
        VdSysCfg(0)
    }
}
impl core::fmt::Debug for VdSysCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VdSysCfg")
            .field("lvdre", &self.lvdre())
            .field("lvdie", &self.lvdie())
            .field("hvdre", &self.hvdre())
            .field("hvdie", &self.hvdie())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VdSysCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VdSysCfg {{ lvdre: {=bool:?}, lvdie: {=bool:?}, hvdre: {=bool:?}, hvdie: {=bool:?}, lock: {:?} }}",
            self.lvdre(),
            self.lvdie(),
            self.hvdre(),
            self.hvdie(),
            self.lock()
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ack {
    #[doc = "Not acknowledged."]
    AckNo = 0x0,
    #[doc = "Acknowledged."]
    AckYes = 0x01,
}
impl Ack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ack {
    #[inline(always)]
    fn from(val: u8) -> Ack {
        Ack::from_bits(val)
    }
}
impl From<Ack> for u8 {
    #[inline(always)]
    fn from(val: Ack) -> u8 {
        Ack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgBgmode {
    #[doc = "Bandgap disabled."]
    Bgmode0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled."]
    Bgmode01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled."]
    Bgmode10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ActiveCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgBgmode {
        ActiveCfgBgmode::from_bits(val)
    }
}
impl From<ActiveCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgBgmode) -> u8 {
        ActiveCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddDs {
    #[doc = "Low."]
    Low = 0x0,
    #[doc = "Normal."]
    Normal = 0x01,
}
impl ActiveCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddDs {
        ActiveCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddDs) -> u8 {
        ActiveCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgCoreldoVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Regulate to mid voltage (1.0 V)."]
    Mid = 0x01,
    #[doc = "Regulate to normal voltage (1.1 V)."]
    Normal = 0x02,
    #[doc = "Regulate to overdrive voltage (1.2 V)."]
    Over = 0x03,
}
impl ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgCoreldoVddLvl {
        ActiveCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<ActiveCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgCoreldoVddLvl) -> u8 {
        ActiveCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgDcdcVddDs {
    _RESERVED_0 = 0x0,
    #[doc = "Low."]
    Low = 0x01,
    #[doc = "Normal."]
    Normal = 0x02,
    _RESERVED_3 = 0x03,
}
impl ActiveCfgDcdcVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgDcdcVddDs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgDcdcVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgDcdcVddDs {
        ActiveCfgDcdcVddDs::from_bits(val)
    }
}
impl From<ActiveCfgDcdcVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgDcdcVddDs) -> u8 {
        ActiveCfgDcdcVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgDcdcVddLvl {
    _RESERVED_0 = 0x0,
    #[doc = "Midvoltage (1.0 V)."]
    Dcdc01 = 0x01,
    #[doc = "Normal voltage (1.1 V)."]
    Dcdc10 = 0x02,
    #[doc = "Overdrive voltage (1.2 V)."]
    Dcdc11 = 0x03,
}
impl ActiveCfgDcdcVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgDcdcVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgDcdcVddLvl {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgDcdcVddLvl {
        ActiveCfgDcdcVddLvl::from_bits(val)
    }
}
impl From<ActiveCfgDcdcVddLvl> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgDcdcVddLvl) -> u8 {
        ActiveCfgDcdcVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ActiveCfgSysldoVddDs {
    #[doc = "Low."]
    Low = 0x0,
    #[doc = "Normal."]
    Normal = 0x01,
}
impl ActiveCfgSysldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ActiveCfgSysldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ActiveCfgSysldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> ActiveCfgSysldoVddDs {
        ActiveCfgSysldoVddDs::from_bits(val)
    }
}
impl From<ActiveCfgSysldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: ActiveCfgSysldoVddDs) -> u8 {
        ActiveCfgSysldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BleedEn {
    #[doc = "Do not add."]
    AddNo = 0x0,
    #[doc = "Add."]
    AddYes = 0x01,
}
impl BleedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BleedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BleedEn {
    #[inline(always)]
    fn from(val: u8) -> BleedEn {
        BleedEn::from_bits(val)
    }
}
impl From<BleedEn> for u8 {
    #[inline(always)]
    fn from(val: BleedEn) -> u8 {
        BleedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstAck {
    #[doc = "Did not complete."]
    ComplNo = 0x0,
    #[doc = "Completed."]
    ComplYes = 0x01,
}
impl BurstAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstAck {
    #[inline(always)]
    fn from(val: u8) -> BurstAck {
        BurstAck::from_bits(val)
    }
}
impl From<BurstAck> for u8 {
    #[inline(always)]
    fn from(val: BurstAck) -> u8 {
        BurstAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstReq {
    #[doc = "Do not generate."]
    GenNo = 0x0,
    #[doc = "Generate."]
    GenYes = 0x01,
}
impl BurstReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstReq {
    #[inline(always)]
    fn from(val: u8) -> BurstReq {
        BurstReq::from_bits(val)
    }
}
impl From<BurstReq> for u8 {
    #[inline(always)]
    fn from(val: BurstReq) -> u8 {
        BurstReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Not busy."]
    BusyNo = 0x0,
    #[doc = "Busy."]
    BusyYes = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CntSelect {
    #[doc = "0."]
    Bit0 = 0x0,
    #[doc = "1."]
    Bit1 = 0x01,
    #[doc = "2."]
    Bit2 = 0x02,
    #[doc = "3."]
    Bit3 = 0x03,
}
impl CntSelect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CntSelect {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CntSelect {
    #[inline(always)]
    fn from(val: u8) -> CntSelect {
        CntSelect::from_bits(val)
    }
}
impl From<CntSelect> for u8 {
    #[inline(always)]
    fn from(val: CntSelect) -> u8 {
        CntSelect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddHvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl CorevddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddHvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddHvdf {
        CorevddHvdf::from_bits(val)
    }
}
impl From<CorevddHvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddHvdf) -> u8 {
        CorevddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CorevddLvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl CorevddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CorevddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CorevddLvdf {
    #[inline(always)]
    fn from(val: u8) -> CorevddLvdf {
        CorevddLvdf::from_bits(val)
    }
}
impl From<CorevddLvdf> for u8 {
    #[inline(always)]
    fn from(val: CorevddLvdf) -> u8 {
        CorevddLvdf::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features."]
    pub const Standard: Self = Self(0x0);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Standard"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Standard"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IovddHvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl IovddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IovddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IovddHvdf {
    #[inline(always)]
    fn from(val: u8) -> IovddHvdf {
        IovddHvdf::from_bits(val)
    }
}
impl From<IovddHvdf> for u8 {
    #[inline(always)]
    fn from(val: IovddHvdf) -> u8 {
        IovddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IovddLvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl IovddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IovddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IovddLvdf {
    #[inline(always)]
    fn from(val: u8) -> IovddLvdf {
        IovddLvdf::from_bits(val)
    }
}
impl From<IovddLvdf> for u8 {
    #[inline(always)]
    fn from(val: IovddLvdf) -> u8 {
        IovddLvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgBgmode {
    #[doc = "Bandgap disabled."]
    Bgmode0 = 0x0,
    #[doc = "Bandgap enabled, buffer disabled."]
    Bgmode01 = 0x01,
    #[doc = "Bandgap enabled, buffer enabled."]
    Bgmode10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpCfgBgmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgBgmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgBgmode {
    #[inline(always)]
    fn from(val: u8) -> LpCfgBgmode {
        LpCfgBgmode::from_bits(val)
    }
}
impl From<LpCfgBgmode> for u8 {
    #[inline(always)]
    fn from(val: LpCfgBgmode) -> u8 {
        LpCfgBgmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddDs {
    #[doc = "Low."]
    Low = 0x0,
    #[doc = "Normal."]
    Normal = 0x01,
}
impl LpCfgCoreldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddDs {
        LpCfgCoreldoVddDs::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddDs) -> u8 {
        LpCfgCoreldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgCoreldoVddLvl {
    #[doc = "Retention voltage."]
    Under = 0x0,
    #[doc = "Mid voltage (1.0 V)."]
    Mid = 0x01,
    #[doc = "Normal voltage (1.1 V)."]
    Normal = 0x02,
    #[doc = "Overdrive voltage (1.2 V)."]
    Over = 0x03,
}
impl LpCfgCoreldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgCoreldoVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgCoreldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> LpCfgCoreldoVddLvl {
        LpCfgCoreldoVddLvl::from_bits(val)
    }
}
impl From<LpCfgCoreldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: LpCfgCoreldoVddLvl) -> u8 {
        LpCfgCoreldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgDcdcVddDs {
    #[doc = "Pulse refresh."]
    Pulse = 0x0,
    #[doc = "Low."]
    Low = 0x01,
    #[doc = "Normal."]
    Normal = 0x02,
    _RESERVED_3 = 0x03,
}
impl LpCfgDcdcVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgDcdcVddDs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgDcdcVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgDcdcVddDs {
        LpCfgDcdcVddDs::from_bits(val)
    }
}
impl From<LpCfgDcdcVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgDcdcVddDs) -> u8 {
        LpCfgDcdcVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgDcdcVddLvl {
    #[doc = "Retention voltage (0.7 V)."]
    Vdd00 = 0x0,
    #[doc = "Mid voltage (1.0 V)."]
    Vdd01 = 0x01,
    #[doc = "Normal voltage (1.1 V)."]
    Vdd10 = 0x02,
    #[doc = "Overdrive voltage (1.2 V)."]
    Vdd11 = 0x03,
}
impl LpCfgDcdcVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgDcdcVddLvl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgDcdcVddLvl {
    #[inline(always)]
    fn from(val: u8) -> LpCfgDcdcVddLvl {
        LpCfgDcdcVddLvl::from_bits(val)
    }
}
impl From<LpCfgDcdcVddLvl> for u8 {
    #[inline(always)]
    fn from(val: LpCfgDcdcVddLvl) -> u8 {
        LpCfgDcdcVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgGlitchDetectDisable {
    #[doc = "Enable."]
    Enable = 0x0,
    #[doc = "Disable."]
    Disable = 0x01,
}
impl LpCfgGlitchDetectDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgGlitchDetectDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgGlitchDetectDisable {
    #[inline(always)]
    fn from(val: u8) -> LpCfgGlitchDetectDisable {
        LpCfgGlitchDetectDisable::from_bits(val)
    }
}
impl From<LpCfgGlitchDetectDisable> for u8 {
    #[inline(always)]
    fn from(val: LpCfgGlitchDetectDisable) -> u8 {
        LpCfgGlitchDetectDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpCfgSysldoVddDs {
    #[doc = "Low."]
    Low = 0x0,
    #[doc = "Normal."]
    Normal = 0x01,
}
impl LpCfgSysldoVddDs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpCfgSysldoVddDs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpCfgSysldoVddDs {
    #[inline(always)]
    fn from(val: u8) -> LpCfgSysldoVddDs {
        LpCfgSysldoVddDs::from_bits(val)
    }
}
impl From<LpCfgSysldoVddDs> for u8 {
    #[inline(always)]
    fn from(val: LpCfgSysldoVddDs) -> u8 {
        LpCfgSysldoVddDs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpMode {
    #[doc = "SLEEP with system clock running."]
    Mode0 = 0x0,
    #[doc = "DSLEEP with system clock off."]
    Mode1 = 0x01,
    #[doc = "PDOWN with system clock off."]
    Mode2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off."]
    Mode8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpMode {
    #[inline(always)]
    fn from(val: u8) -> LpMode {
        LpMode::from_bits(val)
    }
}
impl From<LpMode> for u8 {
    #[inline(always)]
    fn from(val: LpMode) -> u8 {
        LpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqov {
    #[doc = "Not forced."]
    ForceNo = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Forced low (ignore LPREQPOL settings)."]
    ForceLow = 0x02,
    #[doc = "Forced high (ignore LPREQPOL settings)."]
    ForceHigh = 0x03,
}
impl Lpreqov {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqov {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqov {
    #[inline(always)]
    fn from(val: u8) -> Lpreqov {
        Lpreqov::from_bits(val)
    }
}
impl From<Lpreqov> for u8 {
    #[inline(always)]
    fn from(val: Lpreqov) -> u8 {
        Lpreqov::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpreqpol {
    #[doc = "High."]
    High = 0x0,
    #[doc = "Low."]
    Low = 0x01,
}
impl Lpreqpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpreqpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpreqpol {
    #[inline(always)]
    fn from(val: u8) -> Lpreqpol {
        Lpreqpol::from_bits(val)
    }
}
impl From<Lpreqpol> for u8 {
    #[inline(always)]
    fn from(val: Lpreqpol) -> u8 {
        Lpreqpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvsel {
    #[doc = "High range."]
    Normal = 0x0,
    #[doc = "Low range."]
    Safe = 0x01,
}
impl Lvsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvsel {
    #[inline(always)]
    fn from(val: u8) -> Lvsel {
        Lvsel::from_bits(val)
    }
}
impl From<Lvsel> for u8 {
    #[inline(always)]
    fn from(val: Lvsel) -> u8 {
        Lvsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdLpReq {
    #[doc = "Did not request."]
    ReqNo = 0x0,
    #[doc = "Requested."]
    ReqYes = 0x01,
}
impl PdLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdLpReq {
    #[inline(always)]
    fn from(val: u8) -> PdLpReq {
        PdLpReq::from_bits(val)
    }
}
impl From<PdLpReq> for u8 {
    #[inline(always)]
    fn from(val: PdLpReq) -> u8 {
        PdLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwrReqStatus {
    #[doc = "Did not request."]
    ReqNo = 0x0,
    #[doc = "Requested."]
    ReqYes = 0x01,
}
impl PwrReqStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwrReqStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwrReqStatus {
    #[inline(always)]
    fn from(val: u8) -> PwrReqStatus {
        PwrReqStatus::from_bits(val)
    }
}
impl From<PwrReqStatus> for u8 {
    #[inline(always)]
    fn from(val: PwrReqStatus) -> u8 {
        PwrReqStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Req {
    #[doc = "Do not request."]
    ReqNo = 0x0,
    #[doc = "Request."]
    ReqYes = 0x01,
}
impl Req {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Req {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Req {
    #[inline(always)]
    fn from(val: u8) -> Req {
        Req::from_bits(val)
    }
}
impl From<Req> for u8 {
    #[inline(always)]
    fn from(val: Req) -> u8 {
        Req::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpMode {
    #[doc = "Sleep mode with system clock running."]
    Mode0 = 0x0,
    #[doc = "DSLEEP with system clock off."]
    Mode1 = 0x01,
    #[doc = "PDOWN with system clock off."]
    Mode2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "DPDOWN with system clock off."]
    Mode8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SpcLpMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpMode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpMode {
    #[inline(always)]
    fn from(val: u8) -> SpcLpMode {
        SpcLpMode::from_bits(val)
    }
}
impl From<SpcLpMode> for u8 {
    #[inline(always)]
    fn from(val: SpcLpMode) -> u8 {
        SpcLpMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpcLpReq {
    #[doc = "SPC is in Active or Sleep mode; the ACTIVE_CFG register has control."]
    Active = 0x0,
    #[doc = "All power domains requested low-power mode; SPC entered a low-power state; power-mode configuration based on the LP_CFG register."]
    LowPower = 0x01,
}
impl SpcLpReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SpcLpReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SpcLpReq {
    #[inline(always)]
    fn from(val: u8) -> SpcLpReq {
        SpcLpReq::from_bits(val)
    }
}
impl From<SpcLpReq> for u8 {
    #[inline(always)]
    fn from(val: SpcLpReq) -> u8 {
        SpcLpReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysldoVddLvl {
    #[doc = "Normal voltage (1.8 V)."]
    Normal = 0x0,
    #[doc = "Overdrive voltage (2.5 V)."]
    Over = 0x01,
}
impl SysldoVddLvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysldoVddLvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysldoVddLvl {
    #[inline(always)]
    fn from(val: u8) -> SysldoVddLvl {
        SysldoVddLvl::from_bits(val)
    }
}
impl From<SysldoVddLvl> for u8 {
    #[inline(always)]
    fn from(val: SysldoVddLvl) -> u8 {
        SysldoVddLvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddHvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl SysvddHvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddHvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddHvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddHvdf {
        SysvddHvdf::from_bits(val)
    }
}
impl From<SysvddHvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddHvdf) -> u8 {
        SysvddHvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysvddLvdf {
    #[doc = "Event not detected."]
    EventNo = 0x0,
    #[doc = "Event detected."]
    EventYes = 0x01,
}
impl SysvddLvdf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysvddLvdf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysvddLvdf {
    #[inline(always)]
    fn from(val: u8) -> SysvddLvdf {
        SysvddLvdf::from_bits(val)
    }
}
impl From<SysvddLvdf> for u8 {
    #[inline(always)]
    fn from(val: SysvddLvdf) -> u8 {
        SysvddLvdf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdCoreCfgLock {
    #[doc = "Allow."]
    Allow = 0x0,
    #[doc = "Deny."]
    Deny = 0x01,
}
impl VdCoreCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdCoreCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdCoreCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdCoreCfgLock {
        VdCoreCfgLock::from_bits(val)
    }
}
impl From<VdCoreCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdCoreCfgLock) -> u8 {
        VdCoreCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdIoCfgLock {
    #[doc = "Allow."]
    Allow = 0x0,
    #[doc = "Deny."]
    Deny = 0x01,
}
impl VdIoCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdIoCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdIoCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdIoCfgLock {
        VdIoCfgLock::from_bits(val)
    }
}
impl From<VdIoCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdIoCfgLock) -> u8 {
        VdIoCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdSysCfgLock {
    #[doc = "Allow."]
    Allow = 0x0,
    #[doc = "Deny."]
    Deny = 0x01,
}
impl VdSysCfgLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdSysCfgLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdSysCfgLock {
    #[inline(always)]
    fn from(val: u8) -> VdSysCfgLock {
        VdSysCfgLock::from_bits(val)
    }
}
impl From<VdSysCfgLock> for u8 {
    #[inline(always)]
    fn from(val: VdSysCfgLock) -> u8 {
        VdSysCfgLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VddVdDisable {
    #[doc = "Enable."]
    Enable = 0x0,
    #[doc = "Disable."]
    Disable = 0x01,
}
impl VddVdDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VddVdDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VddVdDisable {
    #[inline(always)]
    fn from(val: u8) -> VddVdDisable {
        VddVdDisable::from_bits(val)
    }
}
impl From<VddVdDisable> for u8 {
    #[inline(always)]
    fn from(val: VddVdDisable) -> u8 {
        VddVdDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vsm {
    _RESERVED_0 = 0x0,
    #[doc = "1.0 V."]
    Vsm1 = 0x01,
    #[doc = "1.1 V."]
    Vsm2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Vsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vsm {
    #[inline(always)]
    fn from(val: u8) -> Vsm {
        Vsm::from_bits(val)
    }
}
impl From<Vsm> for u8 {
    #[inline(always)]
    fn from(val: Vsm) -> u8 {
        Vsm::to_bits(val)
    }
}
