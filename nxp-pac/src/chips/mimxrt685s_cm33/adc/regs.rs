#[doc = "ADC Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "ADC trigger priority control"]
    #[must_use]
    #[inline(always)]
    pub const fn tprictrl(&self) -> super::vals::Tprictrl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tprictrl::from_bits(val as u8)
    }
    #[doc = "ADC trigger priority control"]
    #[inline(always)]
    pub const fn set_tprictrl(&mut self, val: super::vals::Tprictrl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power Configuration Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsel(&self) -> super::vals::Pwrsel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select"]
    #[inline(always)]
    pub const fn set_pwrsel(&mut self, val: super::vals::Pwrsel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Voltage Reference Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection"]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Power Up Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pudly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power Up Delay"]
    #[inline(always)]
    pub const fn set_pudly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pwren(&self) -> super::vals::Pwren {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pwren::from_bits(val as u8)
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[inline(always)]
    pub const fn set_pwren(&mut self, val: super::vals::Pwren) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
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
            "Cfg {{ tprictrl: {:?}, pwrsel: {:?}, refsel: {:?}, pudly: {=u8:?}, pwren: {:?} }}",
            self.tprictrl(),
            self.pwrsel(),
            self.refsel(),
            self.pudly(),
            self.pwren()
        )
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh(pub u32);
impl Cmdh {
    #[doc = "Compare Function Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::CmdhCmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CmdhCmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn set_cmpen(&mut self, val: super::vals::CmdhCmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Loop with Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::CmdhLwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CmdhLwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn set_lwi(&mut self, val: super::vals::CmdhLwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::CmdhSts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CmdhSts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: super::vals::CmdhSts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[must_use]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::CmdhAvgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CmdhAvgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn set_avgs(&mut self, val: super::vals::CmdhAvgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::CmdhLoop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CmdhLoop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn set_loop_(&mut self, val: super::vals::CmdhLoop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[must_use]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::CmdhNext {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CmdhNext::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn set_next(&mut self, val: super::vals::CmdhNext) {
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
            "Cmdh {{ cmpen: {:?}, lwi: {:?}, sts: {:?}, avgs: {:?}, loop_: {:?}, next: {:?} }}",
            self.cmpen(),
            self.lwi(),
            self.sts(),
            self.avgs(),
            self.loop_(),
            self.next()
        )
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl(pub u32);
impl Cmdl {
    #[doc = "Input channel select"]
    #[must_use]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::CmdlAdch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CmdlAdch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn set_adch(&mut self, val: super::vals::CmdlAdch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[must_use]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::CmdlAbsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CmdlAbsel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn set_absel(&mut self, val: super::vals::CmdlAbsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::CmdlDiff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CmdlDiff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn set_diff(&mut self, val: super::vals::CmdlDiff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[must_use]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::CmdlCscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::CmdlCscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn set_cscale(&mut self, val: super::vals::CmdlCscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
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
            .field("absel", &self.absel())
            .field("diff", &self.diff())
            .field("cscale", &self.cscale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdl {{ adch: {:?}, absel: {:?}, diff: {:?}, cscale: {:?} }}",
            self.adch(),
            self.absel(),
            self.diff(),
            self.cscale()
        )
    }
}
#[doc = "ADC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "ADC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adcen(&self) -> super::vals::Adcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Adcen::from_bits(val as u8)
    }
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub const fn set_adcen(&mut self, val: super::vals::Adcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rstfifo(&self) -> super::vals::Rstfifo {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rstfifo::from_bits(val as u8)
    }
    #[doc = "Reset FIFO"]
    #[inline(always)]
    pub const fn set_rstfifo(&mut self, val: super::vals::Rstfifo) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("rstfifo", &self.rstfifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ adcen: {:?}, rst: {:?}, dozen: {:?}, rstfifo: {:?} }}",
            self.adcen(),
            self.rst(),
            self.dozen(),
            self.rstfifo()
        )
    }
}
#[doc = "Compare Value Register"]
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
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct De(pub u32);
impl De {
    #[doc = "FIFO Watermark DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmde(&self) -> super::vals::Fwmde {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fwmde::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark DMA Enable"]
    #[inline(always)]
    pub const fn set_fwmde(&mut self, val: super::vals::Fwmde) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
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
        f.debug_struct("De").field("fwmde", &self.fwmde()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "De {{ fwmde: {:?} }}", self.fwmde())
    }
}
#[doc = "ADC FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Result FIFO counter"]
    #[must_use]
    #[inline(always)]
    pub const fn fcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Result FIFO counter"]
    #[inline(always)]
    pub const fn set_fcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Watermark level selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmark(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark level selection"]
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
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwmie(&self) -> super::vals::Fwmie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fwmie::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwmie(&mut self, val: super::vals::Fwmie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fofie(&self) -> super::vals::Fofie {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fofie::from_bits(val as u8)
    }
    #[doc = "Result FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fofie(&mut self, val: super::vals::Fofie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            .field("fwmie", &self.fwmie())
            .field("fofie", &self.fofie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ie {{ fwmie: {:?}, fofie: {:?} }}",
            self.fwmie(),
            self.fofie()
        )
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Trigger Number"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn set_trig_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Result FIFO Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn fifosize(&self) -> super::vals::Fifosize {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Fifosize::from_bits(val as u8)
    }
    #[doc = "Result FIFO Depth"]
    #[inline(always)]
    pub const fn set_fifosize(&mut self, val: super::vals::Fifosize) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Compare Value Number"]
    #[must_use]
    #[inline(always)]
    pub const fn cv_num(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Value Number"]
    #[inline(always)]
    pub const fn set_cv_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Command Buffer Number"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_num(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Buffer Number"]
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
#[doc = "ADC Pause Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[doc = "Pause Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pausedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Pause Delay"]
    #[inline(always)]
    pub const fn set_pausedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "PAUSE Option Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pauseen(&self) -> super::vals::Pauseen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pauseen::from_bits(val as u8)
    }
    #[doc = "PAUSE Option Enable"]
    #[inline(always)]
    pub const fn set_pauseen(&mut self, val: super::vals::Pauseen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Pause {{ pausedly: {=u16:?}, pauseen: {:?} }}",
            self.pausedly(),
            self.pauseen()
        )
    }
}
#[doc = "ADC Data Result FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resfifo(pub u32);
impl Resfifo {
    #[doc = "Data result"]
    #[must_use]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data result"]
    #[inline(always)]
    pub const fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn tsrc(&self) -> super::vals::Tsrc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Tsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_tsrc(&mut self, val: super::vals::Tsrc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Loop count value"]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> super::vals::Loopcnt {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Loopcnt::from_bits(val as u8)
    }
    #[doc = "Loop count value"]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: super::vals::Loopcnt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Buffer Source"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsrc(&self) -> super::vals::Cmdsrc {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdsrc::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source"]
    #[inline(always)]
    pub const fn set_cmdsrc(&mut self, val: super::vals::Cmdsrc) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "FIFO entry is valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> super::vals::Valid {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Valid::from_bits(val as u8)
    }
    #[doc = "FIFO entry is valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: super::vals::Valid) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            "Resfifo {{ d: {=u16:?}, tsrc: {:?}, loopcnt: {:?}, cmdsrc: {:?}, valid: {:?} }}",
            self.d(),
            self.tsrc(),
            self.loopcnt(),
            self.cmdsrc(),
            self.valid()
        )
    }
}
#[doc = "ADC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Result FIFO Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> super::vals::Rdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rdy::from_bits(val as u8)
    }
    #[doc = "Result FIFO Ready Flag"]
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: super::vals::Rdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fof(&self) -> super::vals::Fof {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fof::from_bits(val as u8)
    }
    #[doc = "Result FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_fof(&mut self, val: super::vals::Fof) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Trigger Active"]
    #[must_use]
    #[inline(always)]
    pub const fn trgact(&self) -> super::vals::Trgact {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trgact::from_bits(val as u8)
    }
    #[doc = "Trigger Active"]
    #[inline(always)]
    pub const fn set_trgact(&mut self, val: super::vals::Trgact) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command Active"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdact(&self) -> super::vals::Cmdact {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdact::from_bits(val as u8)
    }
    #[doc = "Command Active"]
    #[inline(always)]
    pub const fn set_cmdact(&mut self, val: super::vals::Cmdact) {
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
            .field("rdy", &self.rdy())
            .field("fof", &self.fof())
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
            "Stat {{ rdy: {:?}, fof: {:?}, trgact: {:?}, cmdact: {:?} }}",
            self.rdy(),
            self.fof(),
            self.trgact(),
            self.cmdact()
        )
    }
}
#[doc = "Software Trigger Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swtrig(pub u32);
impl Swtrig {
    #[doc = "Software trigger 0 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt0(&self) -> super::vals::Swt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swt0::from_bits(val as u8)
    }
    #[doc = "Software trigger 0 event"]
    #[inline(always)]
    pub const fn set_swt0(&mut self, val: super::vals::Swt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software trigger 1 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt1(&self) -> super::vals::Swt1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Swt1::from_bits(val as u8)
    }
    #[doc = "Software trigger 1 event"]
    #[inline(always)]
    pub const fn set_swt1(&mut self, val: super::vals::Swt1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software trigger 2 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt2(&self) -> super::vals::Swt2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Swt2::from_bits(val as u8)
    }
    #[doc = "Software trigger 2 event"]
    #[inline(always)]
    pub const fn set_swt2(&mut self, val: super::vals::Swt2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software trigger 3 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt3(&self) -> super::vals::Swt3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Swt3::from_bits(val as u8)
    }
    #[doc = "Software trigger 3 event"]
    #[inline(always)]
    pub const fn set_swt3(&mut self, val: super::vals::Swt3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Software trigger 4 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt4(&self) -> super::vals::Swt4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Swt4::from_bits(val as u8)
    }
    #[doc = "Software trigger 4 event"]
    #[inline(always)]
    pub const fn set_swt4(&mut self, val: super::vals::Swt4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Software trigger 5 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt5(&self) -> super::vals::Swt5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Swt5::from_bits(val as u8)
    }
    #[doc = "Software trigger 5 event"]
    #[inline(always)]
    pub const fn set_swt5(&mut self, val: super::vals::Swt5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Software trigger 6 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt6(&self) -> super::vals::Swt6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Swt6::from_bits(val as u8)
    }
    #[doc = "Software trigger 6 event"]
    #[inline(always)]
    pub const fn set_swt6(&mut self, val: super::vals::Swt6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Software trigger 7 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt7(&self) -> super::vals::Swt7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Swt7::from_bits(val as u8)
    }
    #[doc = "Software trigger 7 event"]
    #[inline(always)]
    pub const fn set_swt7(&mut self, val: super::vals::Swt7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Software trigger 8 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt8(&self) -> super::vals::Swt8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Swt8::from_bits(val as u8)
    }
    #[doc = "Software trigger 8 event"]
    #[inline(always)]
    pub const fn set_swt8(&mut self, val: super::vals::Swt8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Software trigger 9 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt9(&self) -> super::vals::Swt9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Swt9::from_bits(val as u8)
    }
    #[doc = "Software trigger 9 event"]
    #[inline(always)]
    pub const fn set_swt9(&mut self, val: super::vals::Swt9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Software trigger 10 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt10(&self) -> super::vals::Swt10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Swt10::from_bits(val as u8)
    }
    #[doc = "Software trigger 10 event"]
    #[inline(always)]
    pub const fn set_swt10(&mut self, val: super::vals::Swt10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software trigger 11 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt11(&self) -> super::vals::Swt11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Swt11::from_bits(val as u8)
    }
    #[doc = "Software trigger 11 event"]
    #[inline(always)]
    pub const fn set_swt11(&mut self, val: super::vals::Swt11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Software trigger 12 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt12(&self) -> super::vals::Swt12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Swt12::from_bits(val as u8)
    }
    #[doc = "Software trigger 12 event"]
    #[inline(always)]
    pub const fn set_swt12(&mut self, val: super::vals::Swt12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Software trigger 13 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt13(&self) -> super::vals::Swt13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Swt13::from_bits(val as u8)
    }
    #[doc = "Software trigger 13 event"]
    #[inline(always)]
    pub const fn set_swt13(&mut self, val: super::vals::Swt13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Software trigger 14 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt14(&self) -> super::vals::Swt14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Swt14::from_bits(val as u8)
    }
    #[doc = "Software trigger 14 event"]
    #[inline(always)]
    pub const fn set_swt14(&mut self, val: super::vals::Swt14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software trigger 15 event"]
    #[must_use]
    #[inline(always)]
    pub const fn swt15(&self) -> super::vals::Swt15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Swt15::from_bits(val as u8)
    }
    #[doc = "Software trigger 15 event"]
    #[inline(always)]
    pub const fn set_swt15(&mut self, val: super::vals::Swt15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
            .field("swt4", &self.swt4())
            .field("swt5", &self.swt5())
            .field("swt6", &self.swt6())
            .field("swt7", &self.swt7())
            .field("swt8", &self.swt8())
            .field("swt9", &self.swt9())
            .field("swt10", &self.swt10())
            .field("swt11", &self.swt11())
            .field("swt12", &self.swt12())
            .field("swt13", &self.swt13())
            .field("swt14", &self.swt14())
            .field("swt15", &self.swt15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swtrig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swtrig {{ swt0: {:?}, swt1: {:?}, swt2: {:?}, swt3: {:?}, swt4: {:?}, swt5: {:?}, swt6: {:?}, swt7: {:?}, swt8: {:?}, swt9: {:?}, swt10: {:?}, swt11: {:?}, swt12: {:?}, swt13: {:?}, swt14: {:?}, swt15: {:?} }}",
            self.swt0(),
            self.swt1(),
            self.swt2(),
            self.swt3(),
            self.swt4(),
            self.swt5(),
            self.swt6(),
            self.swt7(),
            self.swt8(),
            self.swt9(),
            self.swt10(),
            self.swt11(),
            self.swt12(),
            self.swt13(),
            self.swt14(),
            self.swt15()
        )
    }
}
#[doc = "Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Trigger enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hten(&self) -> super::vals::Hten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hten::from_bits(val as u8)
    }
    #[doc = "Trigger enable"]
    #[inline(always)]
    pub const fn set_hten(&mut self, val: super::vals::Hten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger priority setting"]
    #[must_use]
    #[inline(always)]
    pub const fn tpri(&self) -> super::vals::Tpri {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Tpri::from_bits(val as u8)
    }
    #[doc = "Trigger priority setting"]
    #[inline(always)]
    pub const fn set_tpri(&mut self, val: super::vals::Tpri) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Trigger delay select"]
    #[must_use]
    #[inline(always)]
    pub const fn tdly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger delay select"]
    #[inline(always)]
    pub const fn set_tdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger command select"]
    #[must_use]
    #[inline(always)]
    pub const fn tcmd(&self) -> super::vals::Tcmd {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Tcmd::from_bits(val as u8)
    }
    #[doc = "Trigger command select"]
    #[inline(always)]
    pub const fn set_tcmd(&mut self, val: super::vals::Tcmd) {
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
            .field("tpri", &self.tpri())
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
            "Tctrl {{ hten: {:?}, tpri: {:?}, tdly: {=u8:?}, tcmd: {:?} }}",
            self.hten(),
            self.tpri(),
            self.tdly(),
            self.tcmd()
        )
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Resolution"]
    #[must_use]
    #[inline(always)]
    pub const fn res(&self) -> super::vals::Res {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Res::from_bits(val as u8)
    }
    #[doc = "Resolution"]
    #[inline(always)]
    pub const fn set_res(&mut self, val: super::vals::Res) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn diffen(&self) -> super::vals::Diffen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Diffen::from_bits(val as u8)
    }
    #[doc = "Differential Supported"]
    #[inline(always)]
    pub const fn set_diffen(&mut self, val: super::vals::Diffen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Multi Vref Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn mvi(&self) -> super::vals::Mvi {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mvi::from_bits(val as u8)
    }
    #[doc = "Multi Vref Implemented"]
    #[inline(always)]
    pub const fn set_mvi(&mut self, val: super::vals::Mvi) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Scale Width"]
    #[must_use]
    #[inline(always)]
    pub const fn csw(&self) -> super::vals::Csw {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Csw::from_bits(val as u8)
    }
    #[doc = "Channel Scale Width"]
    #[inline(always)]
    pub const fn set_csw(&mut self, val: super::vals::Csw) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn vr1rngi(&self) -> super::vals::Vr1rngi {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vr1rngi::from_bits(val as u8)
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub const fn set_vr1rngi(&mut self, val: super::vals::Vr1rngi) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Internal ADC Clock implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn iadcki(&self) -> super::vals::Iadcki {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Iadcki::from_bits(val as u8)
    }
    #[doc = "Internal ADC Clock implemented"]
    #[inline(always)]
    pub const fn set_iadcki(&mut self, val: super::vals::Iadcki) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Offset Function Implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn calofsi(&self) -> super::vals::Calofsi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Calofsi::from_bits(val as u8)
    }
    #[doc = "Calibration Offset Function Implemented"]
    #[inline(always)]
    pub const fn set_calofsi(&mut self, val: super::vals::Calofsi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
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
            .field("res", &self.res())
            .field("diffen", &self.diffen())
            .field("mvi", &self.mvi())
            .field("csw", &self.csw())
            .field("vr1rngi", &self.vr1rngi())
            .field("iadcki", &self.iadcki())
            .field("calofsi", &self.calofsi())
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
            "Verid {{ res: {:?}, diffen: {:?}, mvi: {:?}, csw: {:?}, vr1rngi: {:?}, iadcki: {:?}, calofsi: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.res(),
            self.diffen(),
            self.mvi(),
            self.csw(),
            self.vr1rngi(),
            self.iadcki(),
            self.calofsi(),
            self.minor(),
            self.major()
        )
    }
}
