#[doc = "GPT Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter Value. The COUNT bits show the current count value of the GPT counter."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Counter Value. The COUNT bits show the current count value of the GPT counter."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt").field("count", &self.count()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cnt {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "GPT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "GPT Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::En::from_bits(val as u8)
    }
    #[doc = "GPT Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: super::vals::En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPT Enable mode"]
    #[must_use]
    #[inline(always)]
    pub const fn enmod(&self) -> super::vals::Enmod {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Enmod::from_bits(val as u8)
    }
    #[doc = "GPT Enable mode"]
    #[inline(always)]
    pub const fn set_enmod(&mut self, val: super::vals::Enmod) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "GPT debug mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::Dbgen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dbgen::from_bits(val as u8)
    }
    #[doc = "GPT debug mode enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::Dbgen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "GPT Wait Mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn waiten(&self) -> super::vals::Waiten {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Waiten::from_bits(val as u8)
    }
    #[doc = "GPT Wait Mode enable"]
    #[inline(always)]
    pub const fn set_waiten(&mut self, val: super::vals::Waiten) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "GPT Doze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "GPT Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "GPT Stop Mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stopen(&self) -> super::vals::Stopen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Stopen::from_bits(val as u8)
    }
    #[doc = "GPT Stop Mode enable"]
    #[inline(always)]
    pub const fn set_stopen(&mut self, val: super::vals::Stopen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Clock Source select"]
    #[must_use]
    #[inline(always)]
    pub const fn clksrc(&self) -> super::vals::Clksrc {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Clksrc::from_bits(val as u8)
    }
    #[doc = "Clock Source select"]
    #[inline(always)]
    pub const fn set_clksrc(&mut self, val: super::vals::Clksrc) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Free-Run or Restart mode"]
    #[must_use]
    #[inline(always)]
    pub const fn frr(&self) -> super::vals::Frr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Frr::from_bits(val as u8)
    }
    #[doc = "Free-Run or Restart mode"]
    #[inline(always)]
    pub const fn set_frr(&mut self, val: super::vals::Frr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable 24 MHz clock input from crystal"]
    #[must_use]
    #[inline(always)]
    pub const fn en_24m(&self) -> super::vals::En24m {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::En24m::from_bits(val as u8)
    }
    #[doc = "Enable 24 MHz clock input from crystal"]
    #[inline(always)]
    pub const fn set_en_24m(&mut self, val: super::vals::En24m) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> super::vals::Swr {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Swr::from_bits(val as u8)
    }
    #[doc = "Software reset"]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: super::vals::Swr) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "See IM2"]
    #[must_use]
    #[inline(always)]
    pub const fn im1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "See IM2"]
    #[inline(always)]
    pub const fn set_im1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[must_use]
    #[inline(always)]
    pub const fn im2(&self) -> super::vals::Im2 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Im2::from_bits(val as u8)
    }
    #[doc = "IM2 (bits 19-18, Input Capture Channel 2 operating mode) IM1 (bits 17-16, Input Capture Channel 1 operating mode) The IMn bit field determines the transition on the input pin (for Input capture channel n), which will trigger a capture event"]
    #[inline(always)]
    pub const fn set_im2(&mut self, val: super::vals::Im2) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "See OM3"]
    #[must_use]
    #[inline(always)]
    pub const fn om1(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn set_om1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "See OM3"]
    #[must_use]
    #[inline(always)]
    pub const fn om2(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[doc = "See OM3"]
    #[inline(always)]
    pub const fn set_om2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[must_use]
    #[inline(always)]
    pub const fn om3(&self) -> super::vals::Om3 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Om3::from_bits(val as u8)
    }
    #[doc = "OM3 (bits 28-26) controls the Output Compare Channel 3 operating mode"]
    #[inline(always)]
    pub const fn set_om3(&mut self, val: super::vals::Om3) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "See F03"]
    #[must_use]
    #[inline(always)]
    pub const fn fo1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn set_fo1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "See F03"]
    #[must_use]
    #[inline(always)]
    pub const fn fo2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "See F03"]
    #[inline(always)]
    pub const fn set_fo2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[must_use]
    #[inline(always)]
    pub const fn fo3(&self) -> super::vals::Fo3 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Fo3::from_bits(val as u8)
    }
    #[doc = "FO3 Force Output Compare Channel 3 FO2 Force Output Compare Channel 2 FO1 Force Output Compare Channel 1 The FOn bit causes the pin action programmed for the timer Output Compare n pin (according to the OMn bits in this register)"]
    #[inline(always)]
    pub const fn set_fo3(&mut self, val: super::vals::Fo3) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("en", &self.en())
            .field("enmod", &self.enmod())
            .field("dbgen", &self.dbgen())
            .field("waiten", &self.waiten())
            .field("dozeen", &self.dozeen())
            .field("stopen", &self.stopen())
            .field("clksrc", &self.clksrc())
            .field("frr", &self.frr())
            .field("en_24m", &self.en_24m())
            .field("swr", &self.swr())
            .field("im1", &self.im1())
            .field("im2", &self.im2())
            .field("om1", &self.om1())
            .field("om2", &self.om2())
            .field("om3", &self.om3())
            .field("fo1", &self.fo1())
            .field("fo2", &self.fo2())
            .field("fo3", &self.fo3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ en: {:?}, enmod: {:?}, dbgen: {:?}, waiten: {:?}, dozeen: {:?}, stopen: {:?}, clksrc: {:?}, frr: {:?}, en_24m: {:?}, swr: {:?}, im1: {=u8:?}, im2: {:?}, om1: {=u8:?}, om2: {=u8:?}, om3: {:?}, fo1: {=bool:?}, fo2: {=bool:?}, fo3: {:?} }}",
            self.en(),
            self.enmod(),
            self.dbgen(),
            self.waiten(),
            self.dozeen(),
            self.stopen(),
            self.clksrc(),
            self.frr(),
            self.en_24m(),
            self.swr(),
            self.im1(),
            self.im2(),
            self.om1(),
            self.om2(),
            self.om3(),
            self.fo1(),
            self.fo2(),
            self.fo3()
        )
    }
}
#[doc = "GPT Input Capture Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr1(pub u32);
impl Icr1 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Icr1 {
    #[inline(always)]
    fn default() -> Icr1 {
        Icr1(0)
    }
}
impl core::fmt::Debug for Icr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr1").field("capt", &self.capt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icr1 {{ capt: {=u32:?} }}", self.capt())
    }
}
#[doc = "GPT Input Capture Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr2(pub u32);
impl Icr2 {
    #[doc = "Capture Value"]
    #[must_use]
    #[inline(always)]
    pub const fn capt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn set_capt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Icr2 {
    #[inline(always)]
    fn default() -> Icr2 {
        Icr2(0)
    }
}
impl core::fmt::Debug for Icr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr2").field("capt", &self.capt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Icr2 {{ capt: {=u32:?} }}", self.capt())
    }
}
#[doc = "GPT Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "See OF3IE"]
    #[must_use]
    #[inline(always)]
    pub const fn of1ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn set_of1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See OF3IE"]
    #[must_use]
    #[inline(always)]
    pub const fn of2ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3IE"]
    #[inline(always)]
    pub const fn set_of2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn of3ie(&self) -> super::vals::Of3ie {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Of3ie::from_bits(val as u8)
    }
    #[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub const fn set_of3ie(&mut self, val: super::vals::Of3ie) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "See IF2IE"]
    #[must_use]
    #[inline(always)]
    pub const fn if1ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See IF2IE"]
    #[inline(always)]
    pub const fn set_if1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn if2ie(&self) -> super::vals::If2ie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::If2ie::from_bits(val as u8)
    }
    #[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub const fn set_if2ie(&mut self, val: super::vals::If2ie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rovie(&self) -> super::vals::Rovie {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rovie::from_bits(val as u8)
    }
    #[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub const fn set_rovie(&mut self, val: super::vals::Rovie) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("of1ie", &self.of1ie())
            .field("of2ie", &self.of2ie())
            .field("of3ie", &self.of3ie())
            .field("if1ie", &self.if1ie())
            .field("if2ie", &self.if2ie())
            .field("rovie", &self.rovie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ir {{ of1ie: {=bool:?}, of2ie: {=bool:?}, of3ie: {:?}, if1ie: {=bool:?}, if2ie: {:?}, rovie: {:?} }}",
            self.of1ie(),
            self.of2ie(),
            self.of3ie(),
            self.if1ie(),
            self.if2ie(),
            self.rovie()
        )
    }
}
#[doc = "GPT Output Compare Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocr1(pub u32);
impl Ocr1 {
    #[doc = "Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Value"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ocr1 {
    #[inline(always)]
    fn default() -> Ocr1 {
        Ocr1(0)
    }
}
impl core::fmt::Debug for Ocr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ocr1").field("comp", &self.comp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ocr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ocr1 {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "GPT Output Compare Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocr2(pub u32);
impl Ocr2 {
    #[doc = "Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Value"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ocr2 {
    #[inline(always)]
    fn default() -> Ocr2 {
        Ocr2(0)
    }
}
impl core::fmt::Debug for Ocr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ocr2").field("comp", &self.comp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ocr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ocr2 {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "GPT Output Compare Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocr3(pub u32);
impl Ocr3 {
    #[doc = "Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn comp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compare Value"]
    #[inline(always)]
    pub const fn set_comp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ocr3 {
    #[inline(always)]
    fn default() -> Ocr3 {
        Ocr3(0)
    }
}
impl core::fmt::Debug for Ocr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ocr3").field("comp", &self.comp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ocr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ocr3 {{ comp: {=u32:?} }}", self.comp())
    }
}
#[doc = "GPT Prescaler Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescaler bits"]
    #[must_use]
    #[inline(always)]
    pub const fn prescaler(&self) -> super::vals::Prescaler {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::Prescaler::from_bits(val as u16)
    }
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn set_prescaler(&mut self, val: super::vals::Prescaler) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Prescaler bits"]
    #[must_use]
    #[inline(always)]
    pub const fn prescaler24m(&self) -> super::vals::Prescaler24m {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Prescaler24m::from_bits(val as u8)
    }
    #[doc = "Prescaler bits"]
    #[inline(always)]
    pub const fn set_prescaler24m(&mut self, val: super::vals::Prescaler24m) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr")
            .field("prescaler", &self.prescaler())
            .field("prescaler24m", &self.prescaler24m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pr {{ prescaler: {:?}, prescaler24m: {:?} }}",
            self.prescaler(),
            self.prescaler24m()
        )
    }
}
#[doc = "GPT Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "See OF3"]
    #[must_use]
    #[inline(always)]
    pub const fn of1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn set_of1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "See OF3"]
    #[must_use]
    #[inline(always)]
    pub const fn of2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "See OF3"]
    #[inline(always)]
    pub const fn set_of2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[must_use]
    #[inline(always)]
    pub const fn of3(&self) -> super::vals::Of3 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Of3::from_bits(val as u8)
    }
    #[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub const fn set_of3(&mut self, val: super::vals::Of3) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "See IF2"]
    #[must_use]
    #[inline(always)]
    pub const fn if1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "See IF2"]
    #[inline(always)]
    pub const fn set_if1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[must_use]
    #[inline(always)]
    pub const fn if2(&self) -> super::vals::If2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::If2::from_bits(val as u8)
    }
    #[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub const fn set_if2(&mut self, val: super::vals::If2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Rollover Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rov(&self) -> super::vals::Rov {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rov::from_bits(val as u8)
    }
    #[doc = "Rollover Flag"]
    #[inline(always)]
    pub const fn set_rov(&mut self, val: super::vals::Rov) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("of1", &self.of1())
            .field("of2", &self.of2())
            .field("of3", &self.of3())
            .field("if1", &self.if1())
            .field("if2", &self.if2())
            .field("rov", &self.rov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ of1: {=bool:?}, of2: {=bool:?}, of3: {:?}, if1: {=bool:?}, if2: {:?}, rov: {:?} }}",
            self.of1(),
            self.of2(),
            self.of3(),
            self.if1(),
            self.if2(),
            self.rov()
        )
    }
}
