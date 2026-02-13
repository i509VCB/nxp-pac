#[doc = "Automatic wakeup from deepsleep / deep powerdown modes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autowkup(pub u32);
impl Autowkup {
    #[doc = "Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
    #[must_use]
    #[inline(always)]
    pub const fn autowktime(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Auto wake up delay timer. Added delay after sequencer delay value: delay time = value/16MHz"]
    #[inline(always)]
    pub const fn set_autowktime(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Autowkup {
    #[inline(always)]
    fn default() -> Autowkup {
        Autowkup(0)
    }
}
impl core::fmt::Debug for Autowkup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autowkup")
            .field("autowktime", &self.autowktime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autowkup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Autowkup {{ autowktime: {=u16:?} }}", self.autowktime())
    }
}
#[doc = "PMC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
    #[must_use]
    #[inline(always)]
    pub const fn applycfg(&self) -> super::vals::Applycfg {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Applycfg::from_bits(val as u8)
    }
    #[doc = "Apply updated PMC PDRUNCFG bits (SRAM power gates, RBB, FBB, LVD, and HVD control bits) and/or RUNCTRL setting"]
    #[inline(always)]
    pub const fn set_applycfg(&mut self, val: super::vals::Applycfg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable analog buffer for references or ATX2"]
    #[must_use]
    #[inline(always)]
    pub const fn bufen(&self) -> super::vals::Bufen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bufen::from_bits(val as u8)
    }
    #[doc = "Enable analog buffer for references or ATX2"]
    #[inline(always)]
    pub const fn set_bufen(&mut self, val: super::vals::Bufen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "vddcore Low-Voltage Detector Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcoreie(&self) -> super::vals::Lvdcoreie {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lvdcoreie::from_bits(val as u8)
    }
    #[doc = "vddcore Low-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lvdcoreie(&mut self, val: super::vals::Lvdcoreie) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "vddcore Low-Voltage Detector Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcorere(&self) -> super::vals::Lvdcorere {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Lvdcorere::from_bits(val as u8)
    }
    #[doc = "vddcore Low-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub const fn set_lvdcorere(&mut self, val: super::vals::Lvdcorere) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "vddcore High-Voltage Detector Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcoreie(&self) -> super::vals::Hvdcoreie {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Hvdcoreie::from_bits(val as u8)
    }
    #[doc = "vddcore High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvdcoreie(&mut self, val: super::vals::Hvdcoreie) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "vddcore High-Voltage Detector Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcorere(&self) -> super::vals::Hvdcorere {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Hvdcorere::from_bits(val as u8)
    }
    #[doc = "vddcore High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub const fn set_hvdcorere(&mut self, val: super::vals::Hvdcorere) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8ie(&self) -> super::vals::Hvd1v8ie {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Hvd1v8ie::from_bits(val as u8)
    }
    #[doc = "vdd1v8 High-Voltage Detector Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hvd1v8ie(&mut self, val: super::vals::Hvd1v8ie) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "vdd1v8 High-Voltage Detector Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8re(&self) -> super::vals::Hvd1v8re {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Hvd1v8re::from_bits(val as u8)
    }
    #[doc = "vdd1v8 High-Voltage Detector Reset Enable"]
    #[inline(always)]
    pub const fn set_hvd1v8re(&mut self, val: super::vals::Hvd1v8re) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "PMC automatic wakeup enable and interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn autowken(&self) -> super::vals::Autowken {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Autowken::from_bits(val as u8)
    }
    #[doc = "PMC automatic wakeup enable and interrupt enable"]
    #[inline(always)]
    pub const fn set_autowken(&mut self, val: super::vals::Autowken) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "PMIC interrupt pin enable"]
    #[must_use]
    #[inline(always)]
    pub const fn intrpaden(&self) -> super::vals::Intrpaden {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Intrpaden::from_bits(val as u8)
    }
    #[doc = "PMIC interrupt pin enable"]
    #[inline(always)]
    pub const fn set_intrpaden(&mut self, val: super::vals::Intrpaden) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
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
            .field("applycfg", &self.applycfg())
            .field("bufen", &self.bufen())
            .field("lvdcoreie", &self.lvdcoreie())
            .field("lvdcorere", &self.lvdcorere())
            .field("hvdcoreie", &self.hvdcoreie())
            .field("hvdcorere", &self.hvdcorere())
            .field("hvd1v8ie", &self.hvd1v8ie())
            .field("hvd1v8re", &self.hvd1v8re())
            .field("autowken", &self.autowken())
            .field("intrpaden", &self.intrpaden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ applycfg: {:?}, bufen: {:?}, lvdcoreie: {:?}, lvdcorere: {:?}, hvdcoreie: {:?}, hvdcorere: {:?}, hvd1v8ie: {:?}, hvd1v8re: {:?}, autowken: {:?}, intrpaden: {:?} }}",
            self.applycfg(),
            self.bufen(),
            self.lvdcoreie(),
            self.lvdcorere(),
            self.hvdcoreie(),
            self.hvdcorere(),
            self.hvd1v8ie(),
            self.hvd1v8re(),
            self.autowken(),
            self.intrpaden()
        )
    }
}
#[doc = "Wakeup, interrupt, and reset flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags(pub u32);
impl Flags {
    #[doc = "vddcore POR Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn porcoref(&self) -> super::vals::Porcoref {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Porcoref::from_bits(val as u8)
    }
    #[doc = "vddcore POR Flag"]
    #[inline(always)]
    pub const fn set_porcoref(&mut self, val: super::vals::Porcoref) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "vdd1v8 power on reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn por1v8f(&self) -> super::vals::Por1v8f {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Por1v8f::from_bits(val as u8)
    }
    #[doc = "vdd1v8 power on reset flag"]
    #[inline(always)]
    pub const fn set_por1v8f(&mut self, val: super::vals::Por1v8f) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "vdd_ao18 power on reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn porao18f(&self) -> super::vals::Porao18f {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Porao18f::from_bits(val as u8)
    }
    #[doc = "vdd_ao18 power on reset flag"]
    #[inline(always)]
    pub const fn set_porao18f(&mut self, val: super::vals::Porao18f) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcoref(&self) -> super::vals::Lvdcoref {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lvdcoref::from_bits(val as u8)
    }
    #[doc = "vddcore Low-Voltage Detector Flag This flag is set when a low voltage event was detected by the vddcore LVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub const fn set_lvdcoref(&mut self, val: super::vals::Lvdcoref) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcoref(&self) -> super::vals::Hvdcoref {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Hvdcoref::from_bits(val as u8)
    }
    #[doc = "vddcore High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vddcore HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub const fn set_hvdcoref(&mut self, val: super::vals::Hvdcoref) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8f(&self) -> super::vals::Hvd1v8f {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Hvd1v8f::from_bits(val as u8)
    }
    #[doc = "vdd1v8 High-Voltage Detector Flag This flag is set when a high-voltage event was detected by the vdd1v8 HVD monitor and it is enabled for interrupt or reset"]
    #[inline(always)]
    pub const fn set_hvd1v8f(&mut self, val: super::vals::Hvd1v8f) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RTC Wakeup from deep powerdown mode flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rtcf(&self) -> super::vals::Rtcf {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Rtcf::from_bits(val as u8)
    }
    #[doc = "RTC Wakeup from deep powerdown mode flag."]
    #[inline(always)]
    pub const fn set_rtcf(&mut self, val: super::vals::Rtcf) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "PMC Auto Wakeup Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn autowkf(&self) -> super::vals::Autowkf {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Autowkf::from_bits(val as u8)
    }
    #[doc = "PMC Auto Wakeup Interrupt flag."]
    #[inline(always)]
    pub const fn set_autowkf(&mut self, val: super::vals::Autowkf) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "PMIC interrupt pin flag"]
    #[must_use]
    #[inline(always)]
    pub const fn intnpadf(&self) -> super::vals::Intnpadf {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Intnpadf::from_bits(val as u8)
    }
    #[doc = "PMIC interrupt pin flag"]
    #[inline(always)]
    pub const fn set_intnpadf(&mut self, val: super::vals::Intnpadf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Reset pad flag"]
    #[must_use]
    #[inline(always)]
    pub const fn resetnpadf(&self) -> super::vals::Resetnpadf {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Resetnpadf::from_bits(val as u8)
    }
    #[doc = "Reset pad flag"]
    #[inline(always)]
    pub const fn set_resetnpadf(&mut self, val: super::vals::Resetnpadf) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Deep powerdown wakeup flag"]
    #[must_use]
    #[inline(always)]
    pub const fn deeppdf(&self) -> super::vals::Deeppdf {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Deeppdf::from_bits(val as u8)
    }
    #[doc = "Deep powerdown wakeup flag"]
    #[inline(always)]
    pub const fn set_deeppdf(&mut self, val: super::vals::Deeppdf) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        Flags(0)
    }
}
impl core::fmt::Debug for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flags")
            .field("porcoref", &self.porcoref())
            .field("por1v8f", &self.por1v8f())
            .field("porao18f", &self.porao18f())
            .field("lvdcoref", &self.lvdcoref())
            .field("hvdcoref", &self.hvdcoref())
            .field("hvd1v8f", &self.hvd1v8f())
            .field("rtcf", &self.rtcf())
            .field("autowkf", &self.autowkf())
            .field("intnpadf", &self.intnpadf())
            .field("resetnpadf", &self.resetnpadf())
            .field("deeppdf", &self.deeppdf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flags {{ porcoref: {:?}, por1v8f: {:?}, porao18f: {:?}, lvdcoref: {:?}, hvdcoref: {:?}, hvd1v8f: {:?}, rtcf: {:?}, autowkf: {:?}, intnpadf: {:?}, resetnpadf: {:?}, deeppdf: {:?} }}",
            self.porcoref(),
            self.por1v8f(),
            self.porao18f(),
            self.lvdcoref(),
            self.hvdcoref(),
            self.hvd1v8f(),
            self.rtcf(),
            self.autowkf(),
            self.intnpadf(),
            self.resetnpadf(),
            self.deeppdf()
        )
    }
}
#[doc = "Active vddcore LVD monitor trip adjust"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcorectrl(pub u32);
impl Lvdcorectrl {
    #[doc = "Vddcore LVD falling trip voltage"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcorelvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Vddcore LVD falling trip voltage"]
    #[inline(always)]
    pub const fn set_lvdcorelvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Lvdcorectrl {
    #[inline(always)]
    fn default() -> Lvdcorectrl {
        Lvdcorectrl(0)
    }
}
impl core::fmt::Debug for Lvdcorectrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lvdcorectrl")
            .field("lvdcorelvl", &self.lvdcorelvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lvdcorectrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lvdcorectrl {{ lvdcorelvl: {=u8:?} }}",
            self.lvdcorelvl()
        )
    }
}
#[doc = "Memory Sequencer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memseqctrl(pub u32);
impl Memseqctrl {
    #[doc = "Number of memories to turn on/off at a time."]
    #[must_use]
    #[inline(always)]
    pub const fn memseqnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of memories to turn on/off at a time."]
    #[inline(always)]
    pub const fn set_memseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Memseqctrl {
    #[inline(always)]
    fn default() -> Memseqctrl {
        Memseqctrl(0)
    }
}
impl core::fmt::Debug for Memseqctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Memseqctrl")
            .field("memseqnum", &self.memseqnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Memseqctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Memseqctrl {{ memseqnum: {=u8:?} }}", self.memseqnum())
    }
}
#[doc = "GPIO vdde range selection control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padvrange(pub u32);
impl Padvrange {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddio_0range(&self) -> super::vals::Vddio0range {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Vddio0range::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddio_0range(&mut self, val: super::vals::Vddio0range) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddio_1range(&self) -> super::vals::Vddio1range {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Vddio1range::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddio_1range(&mut self, val: super::vals::Vddio1range) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddio_2range(&self) -> super::vals::Vddio2range {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Vddio2range::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddio_2range(&mut self, val: super::vals::Vddio2range) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Padvrange {
    #[inline(always)]
    fn default() -> Padvrange {
        Padvrange(0)
    }
}
impl core::fmt::Debug for Padvrange {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padvrange")
            .field("vddio_0range", &self.vddio_0range())
            .field("vddio_1range", &self.vddio_1range())
            .field("vddio_2range", &self.vddio_2range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padvrange {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Padvrange {{ vddio_0range: {:?}, vddio_1range: {:?}, vddio_2range: {:?} }}",
            self.vddio_0range(),
            self.vddio_1range(),
            self.vddio_2range()
        )
    }
}
#[doc = "PMIC power mode select control configuration to let PMC know when vddcore or vdd1v8 will power off/on"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmiccfg(pub u32);
impl Pmiccfg {
    #[doc = "vddcore state in PMIC mode 0"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorem0(&self) -> super::vals::Vddcorem0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vddcorem0::from_bits(val as u8)
    }
    #[doc = "vddcore state in PMIC mode 0"]
    #[inline(always)]
    pub const fn set_vddcorem0(&mut self, val: super::vals::Vddcorem0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "vddcore state in PMIC mode 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorem1(&self) -> super::vals::Vddcorem1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Vddcorem1::from_bits(val as u8)
    }
    #[doc = "vddcore state in PMIC mode 1"]
    #[inline(always)]
    pub const fn set_vddcorem1(&mut self, val: super::vals::Vddcorem1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "vddcore state in PMIC mode 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorem2(&self) -> super::vals::Vddcorem2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Vddcorem2::from_bits(val as u8)
    }
    #[doc = "vddcore state in PMIC mode 2"]
    #[inline(always)]
    pub const fn set_vddcorem2(&mut self, val: super::vals::Vddcorem2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "vddcore state in PMIC mode 3"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorem3(&self) -> super::vals::Vddcorem3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Vddcorem3::from_bits(val as u8)
    }
    #[doc = "vddcore state in PMIC mode 3"]
    #[inline(always)]
    pub const fn set_vddcorem3(&mut self, val: super::vals::Vddcorem3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "vdd1v8 state in PMIC mode 0"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1v8m0(&self) -> super::vals::Vdd1v8m0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Vdd1v8m0::from_bits(val as u8)
    }
    #[doc = "vdd1v8 state in PMIC mode 0"]
    #[inline(always)]
    pub const fn set_vdd1v8m0(&mut self, val: super::vals::Vdd1v8m0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "vdd1v8 state in PMIC mode 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1v8m1(&self) -> super::vals::Vdd1v8m1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Vdd1v8m1::from_bits(val as u8)
    }
    #[doc = "vdd1v8 state in PMIC mode 1"]
    #[inline(always)]
    pub const fn set_vdd1v8m1(&mut self, val: super::vals::Vdd1v8m1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "vdd1v8 state in PMIC mode 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1v8m2(&self) -> super::vals::Vdd1v8m2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Vdd1v8m2::from_bits(val as u8)
    }
    #[doc = "vdd1v8 state in PMIC mode 2"]
    #[inline(always)]
    pub const fn set_vdd1v8m2(&mut self, val: super::vals::Vdd1v8m2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "vdd1v8 state in PMIC mode 3"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1v8m3(&self) -> super::vals::Vdd1v8m3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Vdd1v8m3::from_bits(val as u8)
    }
    #[doc = "vdd1v8 state in PMIC mode 3"]
    #[inline(always)]
    pub const fn set_vdd1v8m3(&mut self, val: super::vals::Vdd1v8m3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Pmiccfg {
    #[inline(always)]
    fn default() -> Pmiccfg {
        Pmiccfg(0)
    }
}
impl core::fmt::Debug for Pmiccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmiccfg")
            .field("vddcorem0", &self.vddcorem0())
            .field("vddcorem1", &self.vddcorem1())
            .field("vddcorem2", &self.vddcorem2())
            .field("vddcorem3", &self.vddcorem3())
            .field("vdd1v8m0", &self.vdd1v8m0())
            .field("vdd1v8m1", &self.vdd1v8m1())
            .field("vdd1v8m2", &self.vdd1v8m2())
            .field("vdd1v8m3", &self.vdd1v8m3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmiccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmiccfg {{ vddcorem0: {:?}, vddcorem1: {:?}, vddcorem2: {:?}, vddcorem3: {:?}, vdd1v8m0: {:?}, vdd1v8m1: {:?}, vdd1v8m2: {:?}, vdd1v8m3: {:?} }}",
            self.vddcorem0(),
            self.vddcorem1(),
            self.vddcorem2(),
            self.vddcorem3(),
            self.vdd1v8m0(),
            self.vdd1v8m1(),
            self.vdd1v8m2(),
            self.vdd1v8m3()
        )
    }
}
#[doc = "PMC controls used during run mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Runctrl(pub u32);
impl Runctrl {
    #[doc = "Vddcore voltage value when SYSCTL is in run mode"]
    #[must_use]
    #[inline(always)]
    pub const fn corelvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vddcore voltage value when SYSCTL is in run mode"]
    #[inline(always)]
    pub const fn set_corelvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Runctrl {
    #[inline(always)]
    fn default() -> Runctrl {
        Runctrl(0)
    }
}
impl core::fmt::Debug for Runctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Runctrl")
            .field("corelvl", &self.corelvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Runctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Runctrl {{ corelvl: {=u8:?} }}", self.corelvl())
    }
}
#[doc = "PMC controls used during deep sleep mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sleepctrl(pub u32);
impl Sleepctrl {
    #[doc = "Vddcore voltage value when SYSCTL is in sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn corelvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Vddcore voltage value when SYSCTL is in sleep mode"]
    #[inline(always)]
    pub const fn set_corelvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Sleepctrl {
    #[inline(always)]
    fn default() -> Sleepctrl {
        Sleepctrl(0)
    }
}
impl core::fmt::Debug for Sleepctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sleepctrl")
            .field("corelvl", &self.corelvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sleepctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sleepctrl {{ corelvl: {=u8:?} }}", self.corelvl())
    }
}
#[doc = "PMC status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "General sequencer and finite state machine status"]
    #[must_use]
    #[inline(always)]
    pub const fn activefsm(&self) -> super::vals::Activefsm {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Activefsm::from_bits(val as u8)
    }
    #[doc = "General sequencer and finite state machine status"]
    #[inline(always)]
    pub const fn set_activefsm(&mut self, val: super::vals::Activefsm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("activefsm", &self.activefsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Status {{ activefsm: {:?} }}", self.activefsm())
    }
}
