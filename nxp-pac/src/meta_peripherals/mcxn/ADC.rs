#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status Register."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn ie(self) -> crate::pac::common::Reg<Ie, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn de(self) -> crate::pac::common::Reg<De, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::pac::common::Reg<Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pause Register."]
    #[inline(always)]
    pub const fn pause(self) -> crate::pac::common::Reg<Pause, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Software Trigger Register."]
    #[inline(always)]
    pub const fn swtrig(self) -> crate::pac::common::Reg<Swtrig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Trigger Status Register."]
    #[inline(always)]
    pub const fn tstat(self) -> crate::pac::common::Reg<Tstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Offset Trim Register."]
    #[inline(always)]
    pub const fn ofstrim(self) -> crate::pac::common::Reg<Ofstrim, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Trigger Control Register."]
    #[inline(always)]
    pub const fn tctrl(self, n: usize) -> crate::pac::common::Reg<Tctrl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _)
        }
    }
    #[doc = "FIFO Control Register."]
    #[inline(always)]
    pub const fn fctrl(self, n: usize) -> crate::pac::common::Reg<Fctrl, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _)
        }
    }
    #[doc = "Gain Calibration Control."]
    #[inline(always)]
    pub const fn gcc(self, n: usize) -> crate::pac::common::Reg<Gcc, crate::pac::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize + n * 4usize) as _)
        }
    }
    #[doc = "Gain Calculation Result."]
    #[inline(always)]
    pub const fn gcr(self, n: usize) -> crate::pac::common::Reg<Gcr, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize + n * 4usize) as _)
        }
    }
    #[doc = "Command Low Buffer Register."]
    #[inline(always)]
    pub const fn cmdl(self, n: usize) -> crate::pac::common::Reg<Cmdl, crate::pac::common::RW> {
        assert!(n < 15usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 8usize) as _)
        }
    }
    #[doc = "Command High Buffer Register."]
    #[inline(always)]
    pub const fn cmdh(self, n: usize) -> crate::pac::common::Reg<Cmdh, crate::pac::common::RW> {
        assert!(n < 15usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 8usize) as _)
        }
    }
    #[doc = "Compare Value Register."]
    #[inline(always)]
    pub const fn cv(self, n: usize) -> crate::pac::common::Reg<Cv, crate::pac::common::RW> {
        assert!(n < 15usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Data Result FIFO Register."]
    #[inline(always)]
    pub const fn resfifo(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Resfifo, crate::pac::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General A-Side Registers."]
    #[inline(always)]
    pub const fn cal_gar(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<CalGar, crate::pac::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General B-Side Registers."]
    #[inline(always)]
    pub const fn cal_gbr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<CalGbr, crate::pac::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
}
#[doc = "Calibration General A-Side Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalGar(pub u32);
impl CalGar {
    #[doc = "Calibration General A Side Register Element."]
    #[must_use]
    #[inline(always)]
    pub const fn cal_gar_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Calibration General A Side Register Element."]
    #[inline(always)]
    pub const fn set_cal_gar_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for CalGar {
    #[inline(always)]
    fn default() -> CalGar {
        CalGar(0)
    }
}
impl core::fmt::Debug for CalGar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalGar")
            .field("cal_gar_val", &self.cal_gar_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalGar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalGar {{ cal_gar_val: {=u16:?} }}", self.cal_gar_val())
    }
}
#[doc = "Calibration General B-Side Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalGbr(pub u32);
impl CalGbr {
    #[doc = "Calibration General B Side Register Element."]
    #[must_use]
    #[inline(always)]
    pub const fn cal_gbr_val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Calibration General B Side Register Element."]
    #[inline(always)]
    pub const fn set_cal_gbr_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for CalGbr {
    #[inline(always)]
    fn default() -> CalGbr {
        CalGbr(0)
    }
}
impl core::fmt::Debug for CalGbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalGbr")
            .field("cal_gbr_val", &self.cal_gbr_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalGbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CalGbr {{ cal_gbr_val: {=u16:?} }}", self.cal_gbr_val())
    }
}
#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "ADC Trigger Priority Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tprictrl(&self) -> Tprictrl {
        let val = (self.0 >> 0usize) & 0x03;
        Tprictrl::from_bits(val as u8)
    }
    #[doc = "ADC Trigger Priority Control."]
    #[inline(always)]
    pub const fn set_tprictrl(&mut self, val: Tprictrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Power Configuration Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsel(&self) -> Pwrsel {
        let val = (self.0 >> 4usize) & 0x03;
        Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select."]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: Pwrsel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Voltage Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> Refsel {
        let val = (self.0 >> 6usize) & 0x03;
        Refsel::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: Refsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Trigger Resume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tres(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Resume Enable."]
    #[inline(always)]
    pub const fn set_tres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Command Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmdres(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Command Resume."]
    #[inline(always)]
    pub const fn set_tcmdres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High-Priority Trigger Exception Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn hpt_exdi(&self) -> HptExdi {
        let val = (self.0 >> 10usize) & 0x01;
        HptExdi::from_bits(val as u8)
    }
    #[doc = "High-Priority Trigger Exception Disable."]
    #[inline(always)]
    pub const fn set_hpt_exdi(&mut self, val: HptExdi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power-up Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn pudly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power-up Delay."]
    #[inline(always)]
    pub const fn set_pudly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ADC Analog Pre-Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pwren(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Analog Pre-Enable."]
    #[inline(always)]
    pub const fn set_pwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("tprictrl", &self.tprictrl())
            .field("pwrsel", &self.pwrsel())
            .field("refsel", &self.refsel())
            .field("tres", &self.tres())
            .field("tcmdres", &self.tcmdres())
            .field("hpt_exdi", &self.hpt_exdi())
            .field("pudly", &self.pudly())
            .field("pwren", &self.pwren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ tprictrl: {:?}, pwrsel: {:?}, refsel: {:?}, tres: {=bool:?}, tcmdres: {=bool:?}, hpt_exdi: {:?}, pudly: {=u8:?}, pwren: {=bool:?} }}",
            self.tprictrl(),
            self.pwrsel(),
            self.refsel(),
            self.tres(),
            self.tcmdres(),
            self.hpt_exdi(),
            self.pudly(),
            self.pwren()
        )
    }
}
#[doc = "Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh(pub u32);
impl Cmdh {
    #[doc = "Compare Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> CmdhCmpen {
        let val = (self.0 >> 0usize) & 0x03;
        CmdhCmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable."]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: CmdhCmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for Trigger Assertion Before Execution."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_trig(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wait for Trigger Assertion Before Execution."]
    #[inline(always)]
    pub const fn set_wait_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> CmdhSts {
        let val = (self.0 >> 8usize) & 0x07;
        CmdhSts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: CmdhSts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> CmdhAvgs {
        let val = (self.0 >> 12usize) & 0x0f;
        CmdhAvgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: CmdhAvgs) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> CmdhLoop {
        let val = (self.0 >> 16usize) & 0x0f;
        CmdhLoop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: CmdhLoop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> CmdhNext {
        let val = (self.0 >> 24usize) & 0x0f;
        CmdhNext::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_next(&mut self, val: CmdhNext) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh {
    #[inline(always)]
    fn default() -> Cmdh {
        Cmdh(0)
    }
}
impl core::fmt::Debug for Cmdh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdh")
            .field("cmpen", &self.cmpen())
            .field("wait_trig", &self.wait_trig())
            .field("lwi", &self.lwi())
            .field("sts", &self.sts())
            .field("avgs", &self.avgs())
            .field("loop_", &self.loop_())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdh {{ cmpen: {:?}, wait_trig: {=bool:?}, lwi: {=bool:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.wait_trig(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl(pub u32);
impl Cmdl {
    #[doc = "Input Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> CmdlAdch {
        let val = (self.0 >> 0usize) & 0x1f;
        CmdlAdch::from_bits(val as u8)
    }
    #[doc = "Input Channel Select."]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: CmdlAdch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn ctype(&self) -> CmdlCtype {
        let val = (self.0 >> 5usize) & 0x03;
        CmdlCtype::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_ctype(&mut self, val: CmdlCtype) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select Resolution of Conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> CmdlMode {
        let val = (self.0 >> 7usize) & 0x01;
        CmdlMode::from_bits(val as u8)
    }
    #[doc = "Select Resolution of Conversions."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: CmdlMode) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Alternate Channel B Input Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn altb_adch(&self) -> CmdlAltbAdch {
        let val = (self.0 >> 16usize) & 0x1f;
        CmdlAltbAdch::from_bits(val as u8)
    }
    #[doc = "Alternate Channel B Input Channel Select."]
    #[inline(always)]
    pub const fn set_altb_adch(&mut self, val: CmdlAltbAdch) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Alternate Channel B Select Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn altben(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Alternate Channel B Select Enable."]
    #[inline(always)]
    pub const fn set_altben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Cmdl {
    #[inline(always)]
    fn default() -> Cmdl {
        Cmdl(0)
    }
}
impl core::fmt::Debug for Cmdl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdl")
            .field("adch", &self.adch())
            .field("ctype", &self.ctype())
            .field("mode", &self.mode())
            .field("altb_adch", &self.altb_adch())
            .field("altben", &self.altben())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl {{ adch: {:?}, ctype: {:?}, mode: {:?}, altb_adch: {:?}, altben: {=bool:?} }}",
            self.adch(),
            self.ctype(),
            self.mode(),
            self.altb_adch(),
            self.altben()
        )
    }
}
#[doc = "Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "ADC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn adcen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC Enable."]
    #[inline(always)]
    pub const fn set_adcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> Rst {
        let val = (self.0 >> 1usize) & 0x01;
        Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto-Calibration Request."]
    #[must_use]
    #[inline(always)]
    pub const fn cal_req(&self) -> CalReq {
        let val = (self.0 >> 3usize) & 0x01;
        CalReq::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Request."]
    #[inline(always)]
    pub const fn set_cal_req(&mut self, val: CalReq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Offset Calibration Request."]
    #[must_use]
    #[inline(always)]
    pub const fn calofs(&self) -> Calofs {
        let val = (self.0 >> 4usize) & 0x01;
        Calofs::from_bits(val as u8)
    }
    #[doc = "Offset Calibration Request."]
    #[inline(always)]
    pub const fn set_calofs(&mut self, val: Calofs) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reset FIFO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo0(&self) -> Rstfifo0 {
        let val = (self.0 >> 8usize) & 0x01;
        Rstfifo0::from_bits(val as u8)
    }
    #[doc = "Reset FIFO 0."]
    #[inline(always)]
    pub const fn set_rstfifo0(&mut self, val: Rstfifo0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset FIFO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo1(&self) -> Rstfifo1 {
        let val = (self.0 >> 9usize) & 0x01;
        Rstfifo1::from_bits(val as u8)
    }
    #[doc = "Reset FIFO 1."]
    #[inline(always)]
    pub const fn set_rstfifo1(&mut self, val: Rstfifo1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Auto-Calibration Averages."]
    #[must_use]
    #[inline(always)]
    pub const fn cal_avgs(&self) -> CalAvgs {
        let val = (self.0 >> 16usize) & 0x0f;
        CalAvgs::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Averages."]
    #[inline(always)]
    pub const fn set_cal_avgs(&mut self, val: CalAvgs) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("adcen", &self.adcen())
            .field("rst", &self.rst())
            .field("dozen", &self.dozen())
            .field("cal_req", &self.cal_req())
            .field("calofs", &self.calofs())
            .field("rstfifo0", &self.rstfifo0())
            .field("rstfifo1", &self.rstfifo1())
            .field("cal_avgs", &self.cal_avgs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ adcen: {=bool:?}, rst: {:?}, dozen: {:?}, cal_req: {:?}, calofs: {:?}, rstfifo0: {:?}, rstfifo1: {:?}, cal_avgs: {:?} }}",
            self.adcen(),
            self.rst(),
            self.dozen(),
            self.cal_req(),
            self.calofs(),
            self.rstfifo0(),
            self.rstfifo1(),
            self.cal_avgs()
        )
    }
}
#[doc = "Compare Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc = "Compare Value Low."]
    #[must_use]
    #[inline(always)]
    pub const fn cvl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value Low."]
    #[inline(always)]
    pub const fn set_cvl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Compare Value High."]
    #[must_use]
    #[inline(always)]
    pub const fn cvh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value High."]
    #[inline(always)]
    pub const fn set_cvh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cv {
    #[inline(always)]
    fn default() -> Cv {
        Cv(0)
    }
}
impl core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cv")
            .field("cvl", &self.cvl())
            .field("cvh", &self.cvh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cv {{ cvl: {=u16:?}, cvh: {=u16:?} }}",
            self.cvl(),
            self.cvh()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct De(pub u32);
impl De {
    #[doc = "FIFO 0 Watermark DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwmde0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Watermark DMA Enable."]
    #[inline(always)]
    pub const fn set_fwmde0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO1 Watermark DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwmde1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO1 Watermark DMA Enable."]
    #[inline(always)]
    pub const fn set_fwmde1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for De {
    #[inline(always)]
    fn default() -> De {
        De(0)
    }
}
impl core::fmt::Debug for De {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("De")
            .field("fwmde0", &self.fwmde0())
            .field("fwmde1", &self.fwmde1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "De {{ fwmde0: {=bool:?}, fwmde1: {=bool:?} }}",
            self.fwmde0(),
            self.fwmde1()
        )
    }
}
#[doc = "FIFO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Result FIFO Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn fcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Result FIFO Counter."]
    #[inline(always)]
    pub const fn set_fcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Watermark Level Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn fwmark(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark Level Selection."]
    #[inline(always)]
    pub const fn set_fwmark(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        Fctrl(0)
    }
}
impl core::fmt::Debug for Fctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl")
            .field("fcount", &self.fcount())
            .field("fwmark", &self.fwmark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl {{ fcount: {=u8:?}, fwmark: {=u8:?} }}",
            self.fcount(),
            self.fwmark()
        )
    }
}
#[doc = "Gain Calibration Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcc(pub u32);
impl Gcc {
    #[doc = "Gain Calibration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gain_cal(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calibration Value."]
    #[inline(always)]
    pub const fn set_gain_cal(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calibration Value Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> GccRdy {
        let val = (self.0 >> 24usize) & 0x01;
        GccRdy::from_bits(val as u8)
    }
    #[doc = "Gain Calibration Value Valid."]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: GccRdy) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Gcc {
    #[inline(always)]
    fn default() -> Gcc {
        Gcc(0)
    }
}
impl core::fmt::Debug for Gcc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcc")
            .field("gain_cal", &self.gain_cal())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcc {{ gain_cal: {=u16:?}, rdy: {:?} }}",
            self.gain_cal(),
            self.rdy()
        )
    }
}
#[doc = "Gain Calculation Result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr(pub u32);
impl Gcr {
    #[doc = "Gain Calculation Result."]
    #[must_use]
    #[inline(always)]
    pub const fn gcalr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calculation Result."]
    #[inline(always)]
    pub const fn set_gcalr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calculation Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Gain Calculation Ready."]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        Gcr(0)
    }
}
impl core::fmt::Debug for Gcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gcr")
            .field("gcalr", &self.gcalr())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gcr {{ gcalr: {=u16:?}, rdy: {=bool:?} }}",
            self.gcalr(),
            self.rdy()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "FIFO 0 Watermark Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwmie0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 Watermark Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fwmie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fofie0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fofie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO1 Watermark Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fwmie1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO1 Watermark Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fwmie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fofie1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fofie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger Exception Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn texc_ie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Exception Interrupt Enable."]
    #[inline(always)]
    pub const fn set_texc_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Completion Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_ie(&self) -> TcompIe {
        let val = (self.0 >> 16usize) & 0x0f;
        TcompIe::from_bits(val as u8)
    }
    #[doc = "Trigger Completion Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcomp_ie(&mut self, val: TcompIe) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        Ie(0)
    }
}
impl core::fmt::Debug for Ie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ie")
            .field("fwmie0", &self.fwmie0())
            .field("fofie0", &self.fofie0())
            .field("fwmie1", &self.fwmie1())
            .field("fofie1", &self.fofie1())
            .field("texc_ie", &self.texc_ie())
            .field("tcomp_ie", &self.tcomp_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ie {{ fwmie0: {=bool:?}, fofie0: {=bool:?}, fwmie1: {=bool:?}, fofie1: {=bool:?}, texc_ie: {=bool:?}, tcomp_ie: {:?} }}",
            self.fwmie0(),
            self.fofie0(),
            self.fwmie1(),
            self.fofie1(),
            self.texc_ie(),
            self.tcomp_ie()
        )
    }
}
#[doc = "Offset Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ofstrim(pub u32);
impl Ofstrim {
    #[doc = "Trim for Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn ofstrim_a(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for Offset."]
    #[inline(always)]
    pub const fn set_ofstrim_a(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trim for Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn ofstrim_b(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for Offset."]
    #[inline(always)]
    pub const fn set_ofstrim_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Ofstrim {
    #[inline(always)]
    fn default() -> Ofstrim {
        Ofstrim(0)
    }
}
impl core::fmt::Debug for Ofstrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ofstrim")
            .field("ofstrim_a", &self.ofstrim_a())
            .field("ofstrim_b", &self.ofstrim_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ofstrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ofstrim {{ ofstrim_a: {=u8:?}, ofstrim_b: {=u8:?} }}",
            self.ofstrim_a(),
            self.ofstrim_b()
        )
    }
}
#[doc = "Parameter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Trigger Number."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number."]
    #[inline(always)]
    pub const fn set_trig_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Result FIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn fifosize(&self) -> Fifosize {
        let val = (self.0 >> 8usize) & 0xff;
        Fifosize::from_bits(val as u8)
    }
    #[doc = "Result FIFO Depth."]
    #[inline(always)]
    pub const fn set_fifosize(&mut self, val: Fifosize) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Compare Value Number."]
    #[must_use]
    #[inline(always)]
    pub const fn cv_num(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Value Number."]
    #[inline(always)]
    pub const fn set_cv_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Command Buffer Number."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_num(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Buffer Number."]
    #[inline(always)]
    pub const fn set_cmd_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("trig_num", &self.trig_num())
            .field("fifosize", &self.fifosize())
            .field("cv_num", &self.cv_num())
            .field("cmd_num", &self.cmd_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ trig_num: {=u8:?}, fifosize: {:?}, cv_num: {=u8:?}, cmd_num: {=u8:?} }}",
            self.trig_num(),
            self.fifosize(),
            self.cv_num(),
            self.cmd_num()
        )
    }
}
#[doc = "Pause Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[doc = "Pause Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn pausedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Pause Delay."]
    #[inline(always)]
    pub const fn set_pausedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Pause Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pauseen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Pause Enable."]
    #[inline(always)]
    pub const fn set_pauseen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pause {
    #[inline(always)]
    fn default() -> Pause {
        Pause(0)
    }
}
impl core::fmt::Debug for Pause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pause")
            .field("pausedly", &self.pausedly())
            .field("pauseen", &self.pauseen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pause {{ pausedly: {=u16:?}, pauseen: {=bool:?} }}",
            self.pausedly(),
            self.pauseen()
        )
    }
}
#[doc = "Data Result FIFO Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resfifo(pub u32);
impl Resfifo {
    #[doc = "Data Result."]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Result."]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tsrc(&self) -> Tsrc {
        let val = (self.0 >> 16usize) & 0x03;
        Tsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source."]
    #[inline(always)]
    pub const fn set_tsrc(&mut self, val: Tsrc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Loop Count Value."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> Loopcnt {
        let val = (self.0 >> 20usize) & 0x0f;
        Loopcnt::from_bits(val as u8)
    }
    #[doc = "Loop Count Value."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: Loopcnt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Buffer Source."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsrc(&self) -> Cmdsrc {
        let val = (self.0 >> 24usize) & 0x0f;
        Cmdsrc::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source."]
    #[inline(always)]
    pub const fn set_cmdsrc(&mut self, val: Cmdsrc) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "FIFO Entry is Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Entry is Valid."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Resfifo {
    #[inline(always)]
    fn default() -> Resfifo {
        Resfifo(0)
    }
}
impl core::fmt::Debug for Resfifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Resfifo")
            .field("d", &self.d())
            .field("tsrc", &self.tsrc())
            .field("loopcnt", &self.loopcnt())
            .field("cmdsrc", &self.cmdsrc())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Resfifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Resfifo {{ d: {=u16:?}, tsrc: {:?}, loopcnt: {:?}, cmdsrc: {:?}, valid: {=bool:?} }}",
            self.d(),
            self.tsrc(),
            self.loopcnt(),
            self.cmdsrc(),
            self.valid()
        )
    }
}
#[doc = "Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Result FIFO 0 Ready Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdy0(&self) -> Rdy0 {
        let val = (self.0 >> 0usize) & 0x01;
        Rdy0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Ready Flag."]
    #[inline(always)]
    pub const fn set_rdy0(&mut self, val: Rdy0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fof0(&self) -> Fof0 {
        let val = (self.0 >> 1usize) & 0x01;
        Fof0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Overflow Flag."]
    #[inline(always)]
    pub const fn set_fof0(&mut self, val: Fof0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Result FIFO1 Ready Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdy1(&self) -> Rdy1 {
        let val = (self.0 >> 2usize) & 0x01;
        Rdy1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Ready Flag."]
    #[inline(always)]
    pub const fn set_rdy1(&mut self, val: Rdy1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fof1(&self) -> Fof1 {
        let val = (self.0 >> 3usize) & 0x01;
        Fof1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Overflow Flag."]
    #[inline(always)]
    pub const fn set_fof1(&mut self, val: Fof1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Flag For High-Priority Trigger Exception."]
    #[must_use]
    #[inline(always)]
    pub const fn texc_int(&self) -> TexcInt {
        let val = (self.0 >> 8usize) & 0x01;
        TexcInt::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For High-Priority Trigger Exception."]
    #[inline(always)]
    pub const fn set_texc_int(&mut self, val: TexcInt) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Flag For Trigger Completion."]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_int(&self) -> TcompInt {
        let val = (self.0 >> 9usize) & 0x01;
        TcompInt::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For Trigger Completion."]
    #[inline(always)]
    pub const fn set_tcomp_int(&mut self, val: TcompInt) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn cal_rdy(&self) -> CalRdy {
        let val = (self.0 >> 10usize) & 0x01;
        CalRdy::from_bits(val as u8)
    }
    #[doc = "Calibration Ready."]
    #[inline(always)]
    pub const fn set_cal_rdy(&mut self, val: CalRdy) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC Active."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_active(&self) -> AdcActive {
        let val = (self.0 >> 11usize) & 0x01;
        AdcActive::from_bits(val as u8)
    }
    #[doc = "ADC Active."]
    #[inline(always)]
    pub const fn set_adc_active(&mut self, val: AdcActive) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Trigger Active."]
    #[must_use]
    #[inline(always)]
    pub const fn trgact(&self) -> Trgact {
        let val = (self.0 >> 16usize) & 0x03;
        Trgact::from_bits(val as u8)
    }
    #[doc = "Trigger Active."]
    #[inline(always)]
    pub const fn set_trgact(&mut self, val: Trgact) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command Active."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdact(&self) -> Cmdact {
        let val = (self.0 >> 24usize) & 0x0f;
        Cmdact::from_bits(val as u8)
    }
    #[doc = "Command Active."]
    #[inline(always)]
    pub const fn set_cmdact(&mut self, val: Cmdact) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("rdy0", &self.rdy0())
            .field("fof0", &self.fof0())
            .field("rdy1", &self.rdy1())
            .field("fof1", &self.fof1())
            .field("texc_int", &self.texc_int())
            .field("tcomp_int", &self.tcomp_int())
            .field("cal_rdy", &self.cal_rdy())
            .field("adc_active", &self.adc_active())
            .field("trgact", &self.trgact())
            .field("cmdact", &self.cmdact())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ rdy0: {:?}, fof0: {:?}, rdy1: {:?}, fof1: {:?}, texc_int: {:?}, tcomp_int: {:?}, cal_rdy: {:?}, adc_active: {:?}, trgact: {:?}, cmdact: {:?} }}",
            self.rdy0(),
            self.fof0(),
            self.rdy1(),
            self.fof1(),
            self.texc_int(),
            self.tcomp_int(),
            self.cal_rdy(),
            self.adc_active(),
            self.trgact(),
            self.cmdact()
        )
    }
}
#[doc = "Software Trigger Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swtrig(pub u32);
impl Swtrig {
    #[doc = "Software Trigger 0."]
    #[must_use]
    #[inline(always)]
    pub const fn swt0(&self) -> Swt0 {
        let val = (self.0 >> 0usize) & 0x01;
        Swt0::from_bits(val as u8)
    }
    #[doc = "Software Trigger 0."]
    #[inline(always)]
    pub const fn set_swt0(&mut self, val: Swt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Trigger 1."]
    #[must_use]
    #[inline(always)]
    pub const fn swt1(&self) -> Swt1 {
        let val = (self.0 >> 1usize) & 0x01;
        Swt1::from_bits(val as u8)
    }
    #[doc = "Software Trigger 1."]
    #[inline(always)]
    pub const fn set_swt1(&mut self, val: Swt1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger 2."]
    #[must_use]
    #[inline(always)]
    pub const fn swt2(&self) -> Swt2 {
        let val = (self.0 >> 2usize) & 0x01;
        Swt2::from_bits(val as u8)
    }
    #[doc = "Software Trigger 2."]
    #[inline(always)]
    pub const fn set_swt2(&mut self, val: Swt2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software Trigger 3."]
    #[must_use]
    #[inline(always)]
    pub const fn swt3(&self) -> Swt3 {
        let val = (self.0 >> 3usize) & 0x01;
        Swt3::from_bits(val as u8)
    }
    #[doc = "Software Trigger 3."]
    #[inline(always)]
    pub const fn set_swt3(&mut self, val: Swt3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Swtrig {
    #[inline(always)]
    fn default() -> Swtrig {
        Swtrig(0)
    }
}
impl core::fmt::Debug for Swtrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swtrig")
            .field("swt0", &self.swt0())
            .field("swt1", &self.swt1())
            .field("swt2", &self.swt2())
            .field("swt3", &self.swt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swtrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swtrig {{ swt0: {:?}, swt1: {:?}, swt2: {:?}, swt3: {:?} }}",
            self.swt0(),
            self.swt1(),
            self.swt2(),
            self.swt3()
        )
    }
}
#[doc = "Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Trigger Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Enable."]
    #[inline(always)]
    pub const fn set_hten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SAR Result Destination for Channel A."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_sel_a(&self) -> FifoSelA {
        let val = (self.0 >> 1usize) & 0x01;
        FifoSelA::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination for Channel A."]
    #[inline(always)]
    pub const fn set_fifo_sel_a(&mut self, val: FifoSelA) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SAR Result Destination for Channel B."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_sel_b(&self) -> FifoSelB {
        let val = (self.0 >> 2usize) & 0x01;
        FifoSelB::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination for Channel B."]
    #[inline(always)]
    pub const fn set_fifo_sel_b(&mut self, val: FifoSelB) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Trigger Priority Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn tpri(&self) -> Tpri {
        let val = (self.0 >> 8usize) & 0x03;
        Tpri::from_bits(val as u8)
    }
    #[doc = "Trigger Priority Setting."]
    #[inline(always)]
    pub const fn set_tpri(&mut self, val: Tpri) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Trigger Resync."]
    #[must_use]
    #[inline(always)]
    pub const fn rsync(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Resync."]
    #[inline(always)]
    pub const fn set_rsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Trigger Delay Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tdly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger Delay Select."]
    #[inline(always)]
    pub const fn set_tdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn tcmd(&self) -> Tcmd {
        let val = (self.0 >> 24usize) & 0x0f;
        Tcmd::from_bits(val as u8)
    }
    #[doc = "Trigger Command Select."]
    #[inline(always)]
    pub const fn set_tcmd(&mut self, val: Tcmd) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tctrl {
    #[inline(always)]
    fn default() -> Tctrl {
        Tctrl(0)
    }
}
impl core::fmt::Debug for Tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tctrl")
            .field("hten", &self.hten())
            .field("fifo_sel_a", &self.fifo_sel_a())
            .field("fifo_sel_b", &self.fifo_sel_b())
            .field("tpri", &self.tpri())
            .field("rsync", &self.rsync())
            .field("tdly", &self.tdly())
            .field("tcmd", &self.tcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tctrl {{ hten: {=bool:?}, fifo_sel_a: {:?}, fifo_sel_b: {:?}, tpri: {:?}, rsync: {=bool:?}, tdly: {=u8:?}, tcmd: {:?} }}",
            self.hten(),
            self.fifo_sel_a(),
            self.fifo_sel_b(),
            self.tpri(),
            self.rsync(),
            self.tdly(),
            self.tcmd()
        )
    }
}
#[doc = "Trigger Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstat(pub u32);
impl Tstat {
    #[doc = "Trigger Exception Number."]
    #[must_use]
    #[inline(always)]
    pub const fn texc_num(&self) -> TexcNum {
        let val = (self.0 >> 0usize) & 0x0f;
        TexcNum::from_bits(val as u8)
    }
    #[doc = "Trigger Exception Number."]
    #[inline(always)]
    pub const fn set_texc_num(&mut self, val: TexcNum) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Trigger Completion Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tcomp_flag(&self) -> TcompFlag {
        let val = (self.0 >> 16usize) & 0x0f;
        TcompFlag::from_bits(val as u8)
    }
    #[doc = "Trigger Completion Flag."]
    #[inline(always)]
    pub const fn set_tcomp_flag(&mut self, val: TcompFlag) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tstat {
    #[inline(always)]
    fn default() -> Tstat {
        Tstat(0)
    }
}
impl core::fmt::Debug for Tstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tstat")
            .field("texc_num", &self.texc_num())
            .field("tcomp_flag", &self.tcomp_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tstat {{ texc_num: {:?}, tcomp_flag: {:?} }}",
            self.texc_num(),
            self.tcomp_flag()
        )
    }
}
#[doc = "Version ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn res(&self) -> Res {
        let val = (self.0 >> 0usize) & 0x01;
        Res::from_bits(val as u8)
    }
    #[doc = "Resolution."]
    #[inline(always)]
    pub const fn set_res(&mut self, val: Res) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn diffen(&self) -> Diffen {
        let val = (self.0 >> 1usize) & 0x01;
        Diffen::from_bits(val as u8)
    }
    #[doc = "Differential Supported."]
    #[inline(always)]
    pub const fn set_diffen(&mut self, val: Diffen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Multiple Vref Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn mvi(&self) -> Mvi {
        let val = (self.0 >> 3usize) & 0x01;
        Mvi::from_bits(val as u8)
    }
    #[doc = "Multiple Vref Implemented."]
    #[inline(always)]
    pub const fn set_mvi(&mut self, val: Mvi) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Scale Width."]
    #[must_use]
    #[inline(always)]
    pub const fn csw(&self) -> Csw {
        let val = (self.0 >> 4usize) & 0x07;
        Csw::from_bits(val as u8)
    }
    #[doc = "Channel Scale Width."]
    #[inline(always)]
    pub const fn set_csw(&mut self, val: Csw) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn vr1rngi(&self) -> Vr1rngi {
        let val = (self.0 >> 8usize) & 0x01;
        Vr1rngi::from_bits(val as u8)
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented."]
    #[inline(always)]
    pub const fn set_vr1rngi(&mut self, val: Vr1rngi) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Internal ADC Clock Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn iadcki(&self) -> Iadcki {
        let val = (self.0 >> 9usize) & 0x01;
        Iadcki::from_bits(val as u8)
    }
    #[doc = "Internal ADC Clock Implemented."]
    #[inline(always)]
    pub const fn set_iadcki(&mut self, val: Iadcki) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Function Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn calofsi(&self) -> Calofsi {
        let val = (self.0 >> 10usize) & 0x01;
        Calofsi::from_bits(val as u8)
    }
    #[doc = "Calibration Function Implemented."]
    #[inline(always)]
    pub const fn set_calofsi(&mut self, val: Calofsi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Number of Single-Ended Outputs Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn num_sec(&self) -> NumSec {
        let val = (self.0 >> 11usize) & 0x01;
        NumSec::from_bits(val as u8)
    }
    #[doc = "Number of Single-Ended Outputs Supported."]
    #[inline(always)]
    pub const fn set_num_sec(&mut self, val: NumSec) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Number of FIFOs."]
    #[must_use]
    #[inline(always)]
    pub const fn num_fifo(&self) -> NumFifo {
        let val = (self.0 >> 12usize) & 0x07;
        NumFifo::from_bits(val as u8)
    }
    #[doc = "Number of FIFOs."]
    #[inline(always)]
    pub const fn set_num_fifo(&mut self, val: NumFifo) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
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
            .field("res", &self.res())
            .field("diffen", &self.diffen())
            .field("mvi", &self.mvi())
            .field("csw", &self.csw())
            .field("vr1rngi", &self.vr1rngi())
            .field("iadcki", &self.iadcki())
            .field("calofsi", &self.calofsi())
            .field("num_sec", &self.num_sec())
            .field("num_fifo", &self.num_fifo())
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
            "Verid {{ res: {:?}, diffen: {:?}, mvi: {:?}, csw: {:?}, vr1rngi: {:?}, iadcki: {:?}, calofsi: {:?}, num_sec: {:?}, num_fifo: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.res(),
            self.diffen(),
            self.mvi(),
            self.csw(),
            self.vr1rngi(),
            self.iadcki(),
            self.calofsi(),
            self.num_sec(),
            self.num_fifo(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcActive {
    #[doc = "ADC is idle. There are no pending triggers to service and no active commands are being processed."]
    NotActive = 0x0,
    #[doc = "ADC is processing a conversion, running through the power-up delay, or servicing a trigger."]
    Busy = 0x01,
}
impl AdcActive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcActive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcActive {
    #[inline(always)]
    fn from(val: u8) -> AdcActive {
        AdcActive::from_bits(val)
    }
}
impl From<AdcActive> for u8 {
    #[inline(always)]
    fn from(val: AdcActive) -> u8 {
        AdcActive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalAvgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2 conversions averaged."]
    Average2 = 0x01,
    #[doc = "4 conversions averaged."]
    Average4 = 0x02,
    #[doc = "8 conversions averaged."]
    Average8 = 0x03,
    #[doc = "16 conversions averaged."]
    Average16 = 0x04,
    #[doc = "32 conversions averaged."]
    Average32 = 0x05,
    #[doc = "64 conversions averaged."]
    Average64 = 0x06,
    #[doc = "128 conversions averaged."]
    Average128 = 0x07,
    #[doc = "256 conversions averaged."]
    Average256 = 0x08,
    #[doc = "512 conversions averaged."]
    Average512 = 0x09,
    #[doc = "1024 conversions averaged."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CalAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalAvgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalAvgs {
    #[inline(always)]
    fn from(val: u8) -> CalAvgs {
        CalAvgs::from_bits(val)
    }
}
impl From<CalAvgs> for u8 {
    #[inline(always)]
    fn from(val: CalAvgs) -> u8 {
        CalAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalRdy {
    #[doc = "Calibration is incomplete or has not been run."]
    NotSet = 0x0,
    #[doc = "ADC is calibrated."]
    HardwareCalStepCompleted = 0x01,
}
impl CalRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalRdy {
    #[inline(always)]
    fn from(val: u8) -> CalRdy {
        CalRdy::from_bits(val)
    }
}
impl From<CalRdy> for u8 {
    #[inline(always)]
    fn from(val: CalRdy) -> u8 {
        CalRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CalReq {
    #[doc = "No request made."]
    NoCalibrationRequest = 0x0,
    #[doc = "Request has been made."]
    CalibrationRequestPending = 0x01,
}
impl CalReq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CalReq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CalReq {
    #[inline(always)]
    fn from(val: u8) -> CalReq {
        CalReq::from_bits(val)
    }
}
impl From<CalReq> for u8 {
    #[inline(always)]
    fn from(val: CalReq) -> u8 {
        CalReq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofs {
    #[doc = "Calibration function disabled."]
    NoActiveOffsetCalibrationRequest = 0x0,
    #[doc = "Request for offset calibration function."]
    OffsetCalibrationRequestPending = 0x01,
}
impl Calofs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofs {
    #[inline(always)]
    fn from(val: u8) -> Calofs {
        Calofs::from_bits(val)
    }
}
impl From<Calofs> for u8 {
    #[inline(always)]
    fn from(val: Calofs) -> u8 {
        Calofs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsi {
    #[doc = "Not implemented."]
    CalFunctionNotAvailable = 0x0,
    #[doc = "Implemented."]
    CalFunctionAvailable = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdact {
    #[doc = "No command currently in progress."]
    NoCommandActive = 0x0,
    #[doc = "Command 1 currently being executed."]
    Command1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    Command2 = 0x02,
    #[doc = "Associated command number currently being executed."]
    CommandX3 = 0x03,
    #[doc = "Associated command number currently being executed."]
    CommandX4 = 0x04,
    #[doc = "Associated command number currently being executed."]
    CommandX5 = 0x05,
    #[doc = "Associated command number currently being executed."]
    CommandX6 = 0x06,
    #[doc = "Associated command number currently being executed."]
    CommandX7 = 0x07,
    #[doc = "Associated command number currently being executed."]
    CommandX8 = 0x08,
    #[doc = "Associated command number currently being executed."]
    CommandX9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhAvgs {
    #[doc = "Single conversion."]
    NoAverage = 0x0,
    #[doc = "2."]
    Average2 = 0x01,
    #[doc = "4."]
    Average4 = 0x02,
    #[doc = "8."]
    Average8 = 0x03,
    #[doc = "16."]
    Average16 = 0x04,
    #[doc = "32."]
    Average32 = 0x05,
    #[doc = "64."]
    Average64 = 0x06,
    #[doc = "128."]
    Average128 = 0x07,
    #[doc = "256."]
    Average256 = 0x08,
    #[doc = "512."]
    Average512 = 0x09,
    #[doc = "1024."]
    Average1024 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl CmdhAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhAvgs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhAvgs {
    #[inline(always)]
    fn from(val: u8) -> CmdhAvgs {
        CmdhAvgs::from_bits(val)
    }
}
impl From<CmdhAvgs> for u8 {
    #[inline(always)]
    fn from(val: CmdhAvgs) -> u8 {
        CmdhAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhCmpen {
    #[doc = "Disabled."]
    DisabledAlwaysStoreResult = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Enabled. Store on true."]
    CompareResultStoreIfTrue = 0x02,
    #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
    CompareResultKeepConvertingUntilTrueStoreIfTrue = 0x03,
}
impl CmdhCmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhCmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhCmpen {
    #[inline(always)]
    fn from(val: u8) -> CmdhCmpen {
        CmdhCmpen::from_bits(val)
    }
}
impl From<CmdhCmpen> for u8 {
    #[inline(always)]
    fn from(val: CmdhCmpen) -> u8 {
        CmdhCmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhLoop {
    #[doc = "Looping not enabled. Command executes one time."]
    CmdExec1x = 0x0,
    #[doc = "Loop one time. Command executes two times."]
    CmdExec2x = 0x01,
    #[doc = "Loop two times. Command executes three times."]
    CmdExec3x = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
    CmdExecutesCorrespondingTimes9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    CmdExec15x = 0x0f,
}
impl CmdhLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhLoop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhLoop {
    #[inline(always)]
    fn from(val: u8) -> CmdhLoop {
        CmdhLoop::from_bits(val)
    }
}
impl From<CmdhLoop> for u8 {
    #[inline(always)]
    fn from(val: CmdhLoop) -> u8 {
        CmdhLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhNext {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NoNextCmdTerminateOnFinish = 0x0,
    #[doc = "CMD1."]
    DoCmd1Next = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command."]
    DoCorrespondingCmdNext9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    DoCmd15Next = 0x0f,
}
impl CmdhNext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhNext {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhNext {
    #[inline(always)]
    fn from(val: u8) -> CmdhNext {
        CmdhNext::from_bits(val)
    }
}
impl From<CmdhNext> for u8 {
    #[inline(always)]
    fn from(val: CmdhNext) -> u8 {
        CmdhNext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhSts {
    #[doc = "Minimum sample time of 3.5 ADCK cycles."]
    Sample3p5 = 0x0,
    #[doc = "5.5 ADCK cycles."]
    Sample5p5 = 0x01,
    #[doc = "7.5 ADCK cycles."]
    Sample7p5 = 0x02,
    #[doc = "11.5 ADCK cycles."]
    Sample11p5 = 0x03,
    #[doc = "19.5 ADCK cycles."]
    Sample19p5 = 0x04,
    #[doc = "35.5 ADCK cycles."]
    Sample35p5 = 0x05,
    #[doc = "67.5 ADCK cycles."]
    Sample67p5 = 0x06,
    #[doc = "131.5 ADCK cycles."]
    Sample131p5 = 0x07,
}
impl CmdhSts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhSts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhSts {
    #[inline(always)]
    fn from(val: u8) -> CmdhSts {
        CmdhSts::from_bits(val)
    }
}
impl From<CmdhSts> for u8 {
    #[inline(always)]
    fn from(val: CmdhSts) -> u8 {
        CmdhSts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlAdch {
    #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
    SelectCh0 = 0x0,
    #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
    SelectCh1 = 0x01,
    #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
    SelectCh2 = 0x02,
    #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
    SelectCh3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    SelectCorrespondingChannel9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
    SelectCh30 = 0x1e,
    #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
    SelectCh31 = 0x1f,
}
impl CmdlAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlAdch {
    #[inline(always)]
    fn from(val: u8) -> CmdlAdch {
        CmdlAdch::from_bits(val)
    }
}
impl From<CmdlAdch> for u8 {
    #[inline(always)]
    fn from(val: CmdlAdch) -> u8 {
        CmdlAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlAltbAdch {
    #[doc = "Select CH0B."]
    SelectCh0b = 0x0,
    #[doc = "Select CH1B."]
    SelectCh1b = 0x01,
    #[doc = "Select CH2B."]
    SelectCh2b = 0x02,
    #[doc = "Select CH3B."]
    SelectCh3b = 0x03,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB4 = 0x04,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB5 = 0x05,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB6 = 0x06,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB7 = 0x07,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB8 = 0x08,
    #[doc = "Select corresponding channel CHnB."]
    SelectCorrespondingCHnB9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "Select CH30B."]
    SelectCh30b = 0x1e,
    #[doc = "Select CH31B."]
    SelectCh31b = 0x1f,
}
impl CmdlAltbAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlAltbAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlAltbAdch {
    #[inline(always)]
    fn from(val: u8) -> CmdlAltbAdch {
        CmdlAltbAdch::from_bits(val)
    }
}
impl From<CmdlAltbAdch> for u8 {
    #[inline(always)]
    fn from(val: CmdlAltbAdch) -> u8 {
        CmdlAltbAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlCtype {
    #[doc = "Single-Ended mode. Only A-side channel is converted."]
    SingleEndedASideChannel = 0x0,
    #[doc = "Single-Ended mode. Only B-side channel is converted."]
    SingleEndedBSideChannel = 0x01,
    #[doc = "Differential mode. A-B."]
    DifferentialAMinusB = 0x02,
    #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
    DualAAndB = 0x03,
}
impl CmdlCtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlCtype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlCtype {
    #[inline(always)]
    fn from(val: u8) -> CmdlCtype {
        CmdlCtype::from_bits(val)
    }
}
impl From<CmdlCtype> for u8 {
    #[inline(always)]
    fn from(val: CmdlCtype) -> u8 {
        CmdlCtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlMode {
    #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2's complement output."]
    Data12Bits = 0x0,
    #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2's complement output."]
    Data16Bits = 0x01,
}
impl CmdlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlMode {
    #[inline(always)]
    fn from(val: u8) -> CmdlMode {
        CmdlMode::from_bits(val)
    }
}
impl From<CmdlMode> for u8 {
    #[inline(always)]
    fn from(val: CmdlMode) -> u8 {
        CmdlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a data word in RESFIFO. 0h is only found in the initial FIFO state, prior to the storage of an ADC conversion result into a RESFIFO buffer."]
    NotValid = 0x0,
    #[doc = "CMD1."]
    Cmd1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CorrespondingCmd9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    Cmd15 = 0x0f,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
    #[doc = "Not supported."]
    CscaleNotSupported = 0x0,
    #[doc = "Supported with one-bit CSCALE control field."]
    BitWidth1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Supported with six-bit CSCALE control field."]
    BitWidth6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Not supported."]
    DifferentialNotSupported = 0x0,
    #[doc = "Supported. CMDLn\\[CTYPE\\] controls fields implemented."]
    DifferentialSupported = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "ADC is enabled in low-power mode."]
    Enabled = 0x0,
    #[doc = "ADC is disabled in low-power mode."]
    Disabled = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelA {
    #[doc = "FIFO 0."]
    StoreToFifo0 = 0x0,
    #[doc = "FIFO 1."]
    StoreToFifo1 = 0x01,
}
impl FifoSelA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelA {
    #[inline(always)]
    fn from(val: u8) -> FifoSelA {
        FifoSelA::from_bits(val)
    }
}
impl From<FifoSelA> for u8 {
    #[inline(always)]
    fn from(val: FifoSelA) -> u8 {
        FifoSelA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoSelB {
    #[doc = "FIFO 0."]
    StoreToFifo0 = 0x0,
    #[doc = "FIFO 1."]
    StoreToFifo1 = 0x01,
}
impl FifoSelB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoSelB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoSelB {
    #[inline(always)]
    fn from(val: u8) -> FifoSelB {
        FifoSelB::from_bits(val)
    }
}
impl From<FifoSelB> for u8 {
    #[inline(always)]
    fn from(val: FifoSelB) -> u8 {
        FifoSelB::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fifosize(u8);
impl Fifosize {
    #[doc = "2."]
    pub const Entries2: Self = Self(0x01);
    #[doc = "4."]
    pub const Entries4: Self = Self(0x04);
    #[doc = "8."]
    pub const Entries8: Self = Self(0x08);
    #[doc = "16."]
    pub const Entries16: Self = Self(0x10);
    #[doc = "32."]
    pub const Entries32: Self = Self(0x20);
    #[doc = "64."]
    pub const Entries64: Self = Self(0x40);
}
impl Fifosize {
    pub const fn from_bits(val: u8) -> Fifosize {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fifosize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Entries2"),
            0x04 => f.write_str("Entries4"),
            0x08 => f.write_str("Entries8"),
            0x10 => f.write_str("Entries16"),
            0x20 => f.write_str("Entries32"),
            0x40 => f.write_str("Entries64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Entries2"),
            0x04 => defmt::write!(f, "Entries4"),
            0x08 => defmt::write!(f, "Entries8"),
            0x10 => defmt::write!(f, "Entries16"),
            0x20 => defmt::write!(f, "Entries32"),
            0x40 => defmt::write!(f, "Entries64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof0 {
    #[doc = "No result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    NoOverflow = 0x0,
    #[doc = "At least one result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
    OverflowDetected = 0x01,
}
impl Fof0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof0 {
    #[inline(always)]
    fn from(val: u8) -> Fof0 {
        Fof0::from_bits(val)
    }
}
impl From<Fof0> for u8 {
    #[inline(always)]
    fn from(val: Fof0) -> u8 {
        Fof0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof1 {
    #[doc = "No result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    NoOverflow = 0x0,
    #[doc = "At least one result FIFO1 overflow has occurred since the last time that the flag was cleared."]
    OverflowDetected = 0x01,
}
impl Fof1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof1 {
    #[inline(always)]
    fn from(val: u8) -> Fof1 {
        Fof1::from_bits(val)
    }
}
impl From<Fof1> for u8 {
    #[inline(always)]
    fn from(val: Fof1) -> u8 {
        Fof1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GccRdy {
    #[doc = "Invalid."]
    GainCalNotValid = 0x0,
    #[doc = "Valid."]
    HardwareCalRoutineCompleted = 0x01,
}
impl GccRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GccRdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GccRdy {
    #[inline(always)]
    fn from(val: u8) -> GccRdy {
        GccRdy::from_bits(val)
    }
}
impl From<GccRdy> for u8 {
    #[inline(always)]
    fn from(val: GccRdy) -> u8 {
        GccRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HptExdi {
    #[doc = "Enabled."]
    Enabled = 0x0,
    #[doc = "Disabled."]
    Disabled = 0x01,
}
impl HptExdi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HptExdi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HptExdi {
    #[inline(always)]
    fn from(val: u8) -> HptExdi {
        HptExdi::from_bits(val)
    }
}
impl From<HptExdi> for u8 {
    #[inline(always)]
    fn from(val: HptExdi) -> u8 {
        HptExdi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Not implemented."]
    InternalClkNotAvailable = 0x0,
    #[doc = "Implemented."]
    InternalClkAvailable = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    Result1 = 0x0,
    #[doc = "Result is from second conversion in command."]
    Result2 = 0x01,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult2 = 0x02,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult3 = 0x03,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult4 = 0x04,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult5 = 0x05,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult6 = 0x06,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult7 = 0x07,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult8 = 0x08,
    #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
    CorrespondingResult9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    Result16 = 0x0f,
}
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single VREFH input supported."]
    MultipleRefNotSupported = 0x0,
    #[doc = "Multiple VREFH inputs supported."]
    MultipleRefSupported = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumFifo {
    #[doc = "N/A."]
    NoFifoImplemented = 0x0,
    #[doc = "One."]
    Cnt1 = 0x01,
    #[doc = "Two."]
    Cnt2 = 0x02,
    #[doc = "Three."]
    Cnt3 = 0x03,
    #[doc = "Four."]
    Cnt4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl NumFifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumFifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumFifo {
    #[inline(always)]
    fn from(val: u8) -> NumFifo {
        NumFifo::from_bits(val)
    }
}
impl From<NumFifo> for u8 {
    #[inline(always)]
    fn from(val: NumFifo) -> u8 {
        NumFifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NumSec {
    #[doc = "One."]
    SingleConvertor = 0x0,
    #[doc = "Two."]
    DualConvertor = 0x01,
}
impl NumSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NumSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NumSec {
    #[inline(always)]
    fn from(val: u8) -> NumSec {
        NumSec::from_bits(val)
    }
}
impl From<NumSec> for u8 {
    #[inline(always)]
    fn from(val: NumSec) -> u8 {
        NumSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrsel {
    #[doc = "Low power."]
    Lowest = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "High power."]
    Highest = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy0 {
    #[doc = "Not above watermark."]
    BelowThreshold = 0x0,
    #[doc = "Above watermark."]
    AboveThreshold = 0x01,
}
impl Rdy0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy0 {
    #[inline(always)]
    fn from(val: u8) -> Rdy0 {
        Rdy0::from_bits(val)
    }
}
impl From<Rdy0> for u8 {
    #[inline(always)]
    fn from(val: Rdy0) -> u8 {
        Rdy0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy1 {
    #[doc = "Not above watermark."]
    BelowThreshold = 0x0,
    #[doc = "Above watermark."]
    AboveThreshold = 0x01,
}
impl Rdy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy1 {
    #[inline(always)]
    fn from(val: u8) -> Rdy1 {
        Rdy1::from_bits(val)
    }
}
impl From<Rdy1> for u8 {
    #[inline(always)]
    fn from(val: Rdy1) -> u8 {
        Rdy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "Option 1."]
    Option1 = 0x0,
    #[doc = "Option 2."]
    Option2 = 0x01,
    #[doc = "Option 3."]
    Option3 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res {
    #[doc = "Up to 13-bit differential or 12-bit single-ended resolution supported."]
    Max13Bit = 0x0,
    #[doc = "Up to 16-bit differential or 16-bit single-ended resolution supported. CMDLn\\[MODE\\] available for selecting the resolution of conversions for the associated command."]
    Max16Bit = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    ReleasedFromReset = 0x0,
    #[doc = "ADC logic is reset."]
    HeldInReset = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo0 {
    #[doc = "No effect."]
    NoAction = 0x0,
    #[doc = "FIFO 0 is reset."]
    TriggerReset = 0x01,
}
impl Rstfifo0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo0 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo0 {
        Rstfifo0::from_bits(val)
    }
}
impl From<Rstfifo0> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo0) -> u8 {
        Rstfifo0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo1 {
    #[doc = "No effect."]
    NoAction = 0x0,
    #[doc = "FIFO 1 is reset."]
    TriggerReset = 0x01,
}
impl Rstfifo1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo1 {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo1 {
        Rstfifo1::from_bits(val)
    }
}
impl From<Rstfifo1> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo1) -> u8 {
        Rstfifo1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 0 event generated."]
    InitiateTrigger0 = 0x01,
}
impl Swt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt0 {
    #[inline(always)]
    fn from(val: u8) -> Swt0 {
        Swt0::from_bits(val)
    }
}
impl From<Swt0> for u8 {
    #[inline(always)]
    fn from(val: Swt0) -> u8 {
        Swt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt1 {
    #[doc = "No trigger 1 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 1 event generated."]
    InitiateTrigger1 = 0x01,
}
impl Swt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt1 {
    #[inline(always)]
    fn from(val: u8) -> Swt1 {
        Swt1::from_bits(val)
    }
}
impl From<Swt1> for u8 {
    #[inline(always)]
    fn from(val: Swt1) -> u8 {
        Swt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 2 event generated."]
    InitiateTrigger2 = 0x01,
}
impl Swt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt2 {
    #[inline(always)]
    fn from(val: u8) -> Swt2 {
        Swt2::from_bits(val)
    }
}
impl From<Swt2> for u8 {
    #[inline(always)]
    fn from(val: Swt2) -> u8 {
        Swt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt3 {
    #[doc = "No trigger 3 event generated."]
    NoTrigger = 0x0,
    #[doc = "Trigger 3 event generated."]
    InitiateTrigger3 = 0x01,
}
impl Swt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt3 {
    #[inline(always)]
    fn from(val: u8) -> Swt3 {
        Swt3::from_bits(val)
    }
}
impl From<Swt3> for u8 {
    #[inline(always)]
    fn from(val: Swt3) -> u8 {
        Swt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    NotValid = 0x0,
    #[doc = "CMD1."]
    ExecuteCmd1 = 0x01,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd2 = 0x02,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd3 = 0x03,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd4 = 0x04,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd5 = 0x05,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd6 = 0x06,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd7 = 0x07,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd8 = 0x08,
    #[doc = "Corresponding CMD is executed."]
    ExecuteCorrespondingCmd9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15."]
    ExecuteCmd15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompFlag {
    #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
    NoTrigger = 0x0,
    #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
    Bit0MeansTrigger0Completed = 0x01,
    #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
    Bit1MeansTrigger1Completed = 0x02,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted3 = 0x03,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted4 = 0x04,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted5 = 0x05,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted6 = 0x06,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted7 = 0x07,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted8 = 0x08,
    #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
    SetBitsIndicateTriggerXCompleted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
    AllBitsSetIndicateAllTriggersCompleted = 0x0f,
}
impl TcompFlag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompFlag {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompFlag {
    #[inline(always)]
    fn from(val: u8) -> TcompFlag {
        TcompFlag::from_bits(val)
    }
}
impl From<TcompFlag> for u8 {
    #[inline(always)]
    fn from(val: TcompFlag) -> u8 {
        TcompFlag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompIe {
    #[doc = "All disabled."]
    Disabled = 0x0,
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    Trigger0CompleteEnabled = 0x01,
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    Trigger1CompleteEnabled = 0x02,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled3 = 0x03,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled4 = 0x04,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled5 = 0x05,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled6 = 0x06,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled7 = 0x07,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled8 = 0x08,
    #[doc = "Associated trigger completion interrupts are enabled."]
    TriggerXCompleteEnabled9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "All enabled."]
    AllTriggerCompletesEnabled = 0x0f,
}
impl TcompIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompIe {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompIe {
    #[inline(always)]
    fn from(val: u8) -> TcompIe {
        TcompIe::from_bits(val)
    }
}
impl From<TcompIe> for u8 {
    #[inline(always)]
    fn from(val: TcompIe) -> u8 {
        TcompIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcompInt {
    #[doc = "Either IE\\[TCOMP_IE\\] = 0, or no trigger sequences have run to completion."]
    FlagClear = 0x0,
    #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
    CompletionDetected = 0x01,
}
impl TcompInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcompInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcompInt {
    #[inline(always)]
    fn from(val: u8) -> TcompInt {
        TcompInt::from_bits(val)
    }
}
impl From<TcompInt> for u8 {
    #[inline(always)]
    fn from(val: TcompInt) -> u8 {
        TcompInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcInt {
    #[doc = "No trigger exceptions have occurred."]
    NoException = 0x0,
    #[doc = "A trigger exception has occurred and is pending acknowledgment."]
    ExceptionDetected = 0x01,
}
impl TexcInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcInt {
    #[inline(always)]
    fn from(val: u8) -> TexcInt {
        TexcInt::from_bits(val)
    }
}
impl From<TexcInt> for u8 {
    #[inline(always)]
    fn from(val: TexcInt) -> u8 {
        TexcInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TexcNum {
    #[doc = "No triggers have been interrupted by a high-priority exception."]
    NoExceptions = 0x0,
    #[doc = "Trigger 0 has been interrupted by a high-priority exception."]
    Bit0MeansTrigger0Interrupted = 0x01,
    #[doc = "Trigger 1 has been interrupted by a high-priority exception."]
    Bit1MeansTrigger1Interrupted = 0x02,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted3 = 0x03,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted4 = 0x04,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted5 = 0x05,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted6 = 0x06,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted7 = 0x07,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted8 = 0x08,
    #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
    SetBitsIndicateTriggerXInterrupted9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Every trigger sequence has been interrupted by a high-priority exception."]
    AllBitsSetIndicateAllTriggersInterrupted = 0x0f,
}
impl TexcNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TexcNum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TexcNum {
    #[inline(always)]
    fn from(val: u8) -> TexcNum {
        TexcNum::from_bits(val)
    }
}
impl From<TexcNum> for u8 {
    #[inline(always)]
    fn from(val: TexcNum) -> u8 {
        TexcNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Highest priority, Level 1."]
    HighestPriority = 0x0,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority1 = 0x01,
    #[doc = "Set to corresponding priority level."]
    CorrespondingLowerPriority2 = 0x02,
    #[doc = "Lowest priority, Level 4."]
    LowestPriority = 0x03,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tprictrl {
    #[doc = "Current conversion is aborted and the new command specified by the trigger is started."]
    AbortCurrentOnPriority = 0x0,
    #[doc = "Current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop is completed. CMDHn\\[LOOP\\] is ignored and the higher-priority trigger is serviced."]
    FinishCurrentOnPriority = 0x01,
    #[doc = "Current command is completed (averaging, looping, compare) before servicing the higher-priority trigger."]
    FinishSequenceOnPriority = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    Trig0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    Trig1 = 0x01,
    #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
    Trig2 = 0x02,
    #[doc = "Command (sequence) associated with Trigger 3 currently being executed."]
    Trig3 = 0x03,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
    #[doc = "Trigger source 0."]
    Trigger0 = 0x0,
    #[doc = "Trigger source 1."]
    Trigger1 = 0x01,
    #[doc = "Trigger source 2."]
    Trigger2 = 0x02,
    #[doc = "Trigger source 3."]
    Trigger3 = 0x03,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vr1rngi {
    #[doc = "Range control not required."]
    Ref1FixedVoltageRange = 0x0,
    #[doc = "Range control required."]
    Ref1SelectableVoltageRange = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
