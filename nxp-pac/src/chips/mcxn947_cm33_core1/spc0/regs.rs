#[doc = "Active Power Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg(pub u32);
impl ActiveCfg {
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> super::vals::ActiveCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ActiveCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: super::vals::ActiveCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> super::vals::ActiveCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::ActiveCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: super::vals::ActiveCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_ds(&self) -> super::vals::ActiveCfgSysldoVddDs {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ActiveCfgSysldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_sysldo_vdd_ds(&mut self, val: super::vals::ActiveCfgSysldoVddDs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LDO_SYS VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_lvl(&self) -> super::vals::SysldoVddLvl {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SysldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_sysldo_vdd_lvl(&mut self, val: super::vals::SysldoVddLvl) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DCDC VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_ds(&self) -> super::vals::ActiveCfgDcdcVddDs {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::ActiveCfgDcdcVddDs::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_dcdc_vdd_ds(&mut self, val: super::vals::ActiveCfgDcdcVddDs) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_lvl(&self) -> super::vals::ActiveCfgDcdcVddLvl {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::ActiveCfgDcdcVddLvl::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_dcdc_vdd_lvl(&mut self, val: super::vals::ActiveCfgDcdcVddLvl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Glitch Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Disable"]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> super::vals::ActiveCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::ActiveCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode"]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: super::vals::ActiveCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "VDD Voltage Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd_vd_disable(&self) -> super::vals::VddVdDisable {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::VddVdDisable::from_bits(val as u8)
    }
    #[doc = "VDD Voltage Detect Disable"]
    #[inline(always)]
    pub const fn set_vdd_vd_disable(&mut self, val: super::vals::VddVdDisable) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "IO Low-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn io_lvde(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "IO Low-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_io_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Core High-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High-Voltage Detection Enable"]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "IO High-Voltage Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn io_hvde(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IO High-Voltage Detection Enable"]
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
#[doc = "Active Power Mode Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveCfg1(pub u32);
impl ActiveCfg1 {
    #[doc = "Active Config Chip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Active Config Chip Control"]
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
#[doc = "Active Voltage Trim Delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ActiveVdelay(pub u32);
impl ActiveVdelay {
    #[doc = "Active Voltage Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn active_vdelay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Active Voltage Delay"]
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
#[doc = "SPC Regulator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntrl(pub u32);
impl Cntrl {
    #[doc = "LDO_CORE Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Regulator Enable"]
    #[inline(always)]
    pub const fn set_coreldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_SYS Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_SYS Regulator Enable"]
    #[inline(always)]
    pub const fn set_sysldo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DCDC_CORE Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC_CORE Regulator Enable"]
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
#[doc = "LDO_CORE Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreldoCfg(pub u32);
impl CoreldoCfg {
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dpdown_pulldown_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LDO_CORE Deep Power Down Pulldown Disable"]
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
#[doc = "DCDC Burst Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcBurstCfg(pub u32);
impl DcdcBurstCfg {
    #[doc = "Software Burst Request"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_req(&self) -> super::vals::BurstReq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::BurstReq::from_bits(val as u8)
    }
    #[doc = "Software Burst Request"]
    #[inline(always)]
    pub const fn set_burst_req(&mut self, val: super::vals::BurstReq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "External Burst Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_burst_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Burst Request Enable"]
    #[inline(always)]
    pub const fn set_ext_burst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Acknowledge Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_ack(&self) -> super::vals::BurstAck {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::BurstAck::from_bits(val as u8)
    }
    #[doc = "Burst Acknowledge Flag"]
    #[inline(always)]
    pub const fn set_burst_ack(&mut self, val: super::vals::BurstAck) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Refresh Count Value"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_refresh_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Refresh Count Value"]
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
#[doc = "DCDC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcdcCfg(pub u32);
impl DcdcCfg {
    #[doc = "DCDC Burst Frequency Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_cntrl_on(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Burst Frequency Control Enable"]
    #[inline(always)]
    pub const fn set_freq_cntrl_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DCDC Burst Frequency Control"]
    #[must_use]
    #[inline(always)]
    pub const fn freq_cntrl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "DCDC Burst Frequency Control"]
    #[inline(always)]
    pub const fn set_freq_cntrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "DCDC Bleed Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bleed_en(&self) -> super::vals::BleedEn {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::BleedEn::from_bits(val as u8)
    }
    #[doc = "DCDC Bleed Enable"]
    #[inline(always)]
    pub const fn set_bleed_en(&mut self, val: super::vals::BleedEn) {
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
#[doc = "External Voltage Domain Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvdCfg(pub u32);
impl EvdCfg {
    #[doc = "External Voltage Domain Isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn evdiso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Isolation"]
    #[inline(always)]
    pub const fn set_evdiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "External Voltage Domain Low-Power Isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn evdlpiso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Low-Power Isolation"]
    #[inline(always)]
    pub const fn set_evdlpiso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "External Voltage Domain Status"]
    #[must_use]
    #[inline(always)]
    pub const fn evdstat(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "External Voltage Domain Status"]
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
#[doc = "Glitch Detect Status Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlitchDetectSc(pub u32);
impl GlitchDetectSc {
    #[doc = "Counter Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_select(&self) -> super::vals::CntSelect {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CntSelect::from_bits(val as u8)
    }
    #[doc = "Counter Select"]
    #[inline(always)]
    pub const fn set_cnt_select(&mut self, val: super::vals::CntSelect) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG"]
    #[inline(always)]
    pub const fn set_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Glitch Detect Reset Enable Lock Bit"]
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
#[doc = "Low-Power Mode Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg(pub u32);
impl LpCfg {
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_ds(&self) -> super::vals::LpCfgCoreldoVddDs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpCfgCoreldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_ds(&mut self, val: super::vals::LpCfgCoreldoVddDs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn coreldo_vdd_lvl(&self) -> super::vals::LpCfgCoreldoVddLvl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::LpCfgCoreldoVddLvl::from_bits(val as u8)
    }
    #[doc = "LDO_CORE VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_coreldo_vdd_lvl(&mut self, val: super::vals::LpCfgCoreldoVddLvl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn sysldo_vdd_ds(&self) -> super::vals::LpCfgSysldoVddDs {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LpCfgSysldoVddDs::from_bits(val as u8)
    }
    #[doc = "LDO_SYS VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_sysldo_vdd_ds(&mut self, val: super::vals::LpCfgSysldoVddDs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DCDC VDD Drive Strength"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_ds(&self) -> super::vals::LpCfgDcdcVddDs {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::LpCfgDcdcVddDs::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Drive Strength"]
    #[inline(always)]
    pub const fn set_dcdc_vdd_ds(&mut self, val: super::vals::LpCfgDcdcVddDs) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_vdd_lvl(&self) -> super::vals::LpCfgDcdcVddLvl {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::LpCfgDcdcVddLvl::from_bits(val as u8)
    }
    #[doc = "DCDC VDD Regulator Voltage Level"]
    #[inline(always)]
    pub const fn set_dcdc_vdd_lvl(&mut self, val: super::vals::LpCfgDcdcVddLvl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Glitch Detect Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn glitch_detect_disable(&self) -> super::vals::LpCfgGlitchDetectDisable {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::LpCfgGlitchDetectDisable::from_bits(val as u8)
    }
    #[doc = "Glitch Detect Disable"]
    #[inline(always)]
    pub const fn set_glitch_detect_disable(&mut self, val: super::vals::LpCfgGlitchDetectDisable) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "CORE VDD Internal Voltage Scaling (IVS) Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_ivs_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CORE VDD Internal Voltage Scaling (IVS) Enable"]
    #[inline(always)]
    pub const fn set_corevdd_ivs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbuff_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "CMP Bandgap Buffer Enable"]
    #[inline(always)]
    pub const fn set_lpbuff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Bandgap Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bgmode(&self) -> super::vals::LpCfgBgmode {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LpCfgBgmode::from_bits(val as u8)
    }
    #[doc = "Bandgap Mode"]
    #[inline(always)]
    pub const fn set_bgmode(&mut self, val: super::vals::LpCfgBgmode) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Low-Power IREF Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_irefen(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power IREF Enable"]
    #[inline(always)]
    pub const fn set_lp_irefen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Core Low Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_lvde(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Core Low Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_core_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "System Low Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_lvde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "System Low Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_sys_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "IO Low Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn io_lvde(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "IO Low Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_io_lvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Core High Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn core_hvde(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Core High Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_core_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "System High Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sys_hvde(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "System High Voltage Detect Enable"]
    #[inline(always)]
    pub const fn set_sys_hvde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "IO High Voltage Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn io_hvde(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IO High Voltage Detect Enable"]
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
#[doc = "Low Power Mode Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpCfg1(pub u32);
impl LpCfg1 {
    #[doc = "Low-Power Configuration Chip Control"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_cntrl(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Low-Power Configuration Chip Control"]
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
#[doc = "Low-Power Request Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpreqCfg(pub u32);
impl LpreqCfg {
    #[doc = "Low-Power Request Output Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqoe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Request Output Enable"]
    #[inline(always)]
    pub const fn set_lpreqoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Low-Power Request Output Pin Polarity Control"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqpol(&self) -> super::vals::Lpreqpol {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpreqpol::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Pin Polarity Control"]
    #[inline(always)]
    pub const fn set_lpreqpol(&mut self, val: super::vals::Lpreqpol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Low-Power Request Output Override"]
    #[must_use]
    #[inline(always)]
    pub const fn lpreqov(&self) -> super::vals::Lpreqov {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Lpreqov::from_bits(val as u8)
    }
    #[doc = "Low-Power Request Output Override"]
    #[inline(always)]
    pub const fn set_lpreqov(&mut self, val: super::vals::Lpreqov) {
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
#[doc = "Low Power Wake-Up Delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpwkupDelay(pub u32);
impl LpwkupDelay {
    #[doc = "Low-Power Wake-Up Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn lpwkup_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Low-Power Wake-Up Delay"]
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
#[doc = "SPC Power Domain Mode Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PdStatus(pub u32);
impl PdStatus {
    #[doc = "Power Request Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req_status(&self) -> super::vals::PwrReqStatus {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PwrReqStatus::from_bits(val as u8)
    }
    #[doc = "Power Request Status Flag"]
    #[inline(always)]
    pub const fn set_pwr_req_status(&mut self, val: super::vals::PwrReqStatus) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power Domain Low Power Request Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_lp_req(&self) -> super::vals::PdLpReq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PdLpReq::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Request Flag"]
    #[inline(always)]
    pub const fn set_pd_lp_req(&mut self, val: super::vals::PdLpReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Power Domain Low Power Mode Request"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> super::vals::LpMode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::LpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low Power Mode Request"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: super::vals::LpMode) {
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
#[doc = "Status Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc = "SPC Busy Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> super::vals::Busy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Busy::from_bits(val as u8)
    }
    #[doc = "SPC Busy Status Flag"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: super::vals::Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SPC Power Mode Configuration Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_req(&self) -> super::vals::SpcLpReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SpcLpReq::from_bits(val as u8)
    }
    #[doc = "SPC Power Mode Configuration Status Flag"]
    #[inline(always)]
    pub const fn set_spc_lp_req(&mut self, val: super::vals::SpcLpReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Power Domain Low-Power Mode Request"]
    #[must_use]
    #[inline(always)]
    pub const fn spc_lp_mode(&self) -> super::vals::SpcLpMode {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::SpcLpMode::from_bits(val as u8)
    }
    #[doc = "Power Domain Low-Power Mode Request"]
    #[inline(always)]
    pub const fn set_spc_lp_mode(&mut self, val: super::vals::SpcLpMode) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Isolation Clear Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn iso_clr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Isolation Clear Flags"]
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
#[doc = "SRAM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramctl(pub u32);
impl Sramctl {
    #[doc = "Voltage Select Margin"]
    #[must_use]
    #[inline(always)]
    pub const fn vsm(&self) -> super::vals::Vsm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Vsm::from_bits(val as u8)
    }
    #[doc = "Voltage Select Margin"]
    #[inline(always)]
    pub const fn set_vsm(&mut self, val: super::vals::Vsm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SRAM Voltage Update Request"]
    #[must_use]
    #[inline(always)]
    pub const fn req(&self) -> super::vals::Req {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Req::from_bits(val as u8)
    }
    #[doc = "SRAM Voltage Update Request"]
    #[inline(always)]
    pub const fn set_req(&mut self, val: super::vals::Req) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SRAM Voltage Update Request Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> super::vals::Ack {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ack::from_bits(val as u8)
    }
    #[doc = "SRAM Voltage Update Request Acknowledge"]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: super::vals::Ack) {
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
#[doc = "LDO_SYS Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysldoCfg(pub u32);
impl SysldoCfg {
    #[doc = "Current Sink Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isinken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Sink Enable"]
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
#[doc = "Core Voltage Detect Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdCoreCfg(pub u32);
impl VdCoreCfg {
    #[doc = "Core LVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Core LVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Core LVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Core VDD HVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Core VDD HVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Core VDD HVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Core Voltage Detect Reset Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::VdCoreCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::VdCoreCfgLock::from_bits(val as u8)
    }
    #[doc = "Core Voltage Detect Reset Enable Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::VdCoreCfgLock) {
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
#[doc = "IO Voltage Detect Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdIoCfg(pub u32);
impl VdIoCfg {
    #[doc = "IO VDD LVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD LVD Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IO VDD LVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD LVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IO VDD HVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD HVD Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IO VDD HVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IO VDD HVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IO VDD Low-Voltage Level Select"]
    #[must_use]
    #[inline(always)]
    pub const fn lvsel(&self) -> super::vals::Lvsel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lvsel::from_bits(val as u8)
    }
    #[doc = "IO VDD Low-Voltage Level Select"]
    #[inline(always)]
    pub const fn set_lvsel(&mut self, val: super::vals::Lvsel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "IO Voltage Detect Reset Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::VdIoCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::VdIoCfgLock::from_bits(val as u8)
    }
    #[doc = "IO Voltage Detect Reset Enable Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::VdIoCfgLock) {
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
#[doc = "Voltage Detect Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdStat(pub u32);
impl VdStat {
    #[doc = "Core Low-Voltage Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_lvdf(&self) -> super::vals::CorevddLvdf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CorevddLvdf::from_bits(val as u8)
    }
    #[doc = "Core Low-Voltage Detect Flag"]
    #[inline(always)]
    pub const fn set_corevdd_lvdf(&mut self, val: super::vals::CorevddLvdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "System Low-Voltage Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_lvdf(&self) -> super::vals::SysvddLvdf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SysvddLvdf::from_bits(val as u8)
    }
    #[doc = "System Low-Voltage Detect Flag"]
    #[inline(always)]
    pub const fn set_sysvdd_lvdf(&mut self, val: super::vals::SysvddLvdf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IO VDD LVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iovdd_lvdf(&self) -> super::vals::IovddLvdf {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IovddLvdf::from_bits(val as u8)
    }
    #[doc = "IO VDD LVD Flag"]
    #[inline(always)]
    pub const fn set_iovdd_lvdf(&mut self, val: super::vals::IovddLvdf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Core VDD HVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn corevdd_hvdf(&self) -> super::vals::CorevddHvdf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CorevddHvdf::from_bits(val as u8)
    }
    #[doc = "Core VDD HVD Flag"]
    #[inline(always)]
    pub const fn set_corevdd_hvdf(&mut self, val: super::vals::CorevddHvdf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "System HVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sysvdd_hvdf(&self) -> super::vals::SysvddHvdf {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SysvddHvdf::from_bits(val as u8)
    }
    #[doc = "System HVD Flag"]
    #[inline(always)]
    pub const fn set_sysvdd_hvdf(&mut self, val: super::vals::SysvddHvdf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IO VDD HVD Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn iovdd_hvdf(&self) -> super::vals::IovddHvdf {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::IovddHvdf::from_bits(val as u8)
    }
    #[doc = "IO VDD HVD Flag"]
    #[inline(always)]
    pub const fn set_iovdd_hvdf(&mut self, val: super::vals::IovddHvdf) {
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
#[doc = "System Voltage Detect Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VdSysCfg(pub u32);
impl VdSysCfg {
    #[doc = "System LVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "System LVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "System LVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System HVD Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdre(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "System HVD Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "System HVD Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Voltage Detect Reset Enable Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::VdSysCfgLock {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::VdSysCfgLock::from_bits(val as u8)
    }
    #[doc = "System Voltage Detect Reset Enable Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::VdSysCfgLock) {
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
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
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
