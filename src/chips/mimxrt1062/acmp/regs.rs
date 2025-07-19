#[doc = "CMP Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u8);
impl Cr0 {
    #[doc = "Comparator hard block hysteresis control"]
    #[must_use]
    #[inline(always)]
    pub const fn hystctr(&self) -> super::vals::Hystctr {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Hystctr::from_bits(val as u8)
    }
    #[doc = "Comparator hard block hysteresis control"]
    #[inline(always)]
    pub const fn set_hystctr(&mut self, val: super::vals::Hystctr) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u8) & 0x03) << 0usize);
    }
    #[doc = "Filter Sample Count"]
    #[must_use]
    #[inline(always)]
    pub const fn filter_cnt(&self) -> super::vals::FilterCnt {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::FilterCnt::from_bits(val as u8)
    }
    #[doc = "Filter Sample Count"]
    #[inline(always)]
    pub const fn set_filter_cnt(&mut self, val: super::vals::FilterCnt) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u8) & 0x07) << 4usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0)
    }
}
impl core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr0")
            .field("hystctr", &self.hystctr())
            .field("filter_cnt", &self.filter_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr0 {{ hystctr: {:?}, filter_cnt: {:?} }}",
            self.hystctr(),
            self.filter_cnt()
        )
    }
}
#[doc = "CMP Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u8);
impl Cr1 {
    #[doc = "Comparator Module Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Module Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Comparator Output Pin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ope(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Output Pin Enable"]
    #[inline(always)]
    pub const fn set_ope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Comparator Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cos(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Output Select"]
    #[inline(always)]
    pub const fn set_cos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Comparator INVERT"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator INVERT"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Power Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pmode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Power Mode Select"]
    #[inline(always)]
    pub const fn set_pmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "Windowing Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Windowing Enable"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Sample Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn se(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Sample Enable"]
    #[inline(always)]
    pub const fn set_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("en", &self.en())
            .field("ope", &self.ope())
            .field("cos", &self.cos())
            .field("inv", &self.inv())
            .field("pmode", &self.pmode())
            .field("we", &self.we())
            .field("se", &self.se())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr1 {{ en: {=bool:?}, ope: {=bool:?}, cos: {=bool:?}, inv: {=bool:?}, pmode: {=bool:?}, we: {=bool:?}, se: {=bool:?} }}",
            self.en(),
            self.ope(),
            self.cos(),
            self.inv(),
            self.pmode(),
            self.we(),
            self.se()
        )
    }
}
#[doc = "DAC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Daccr(pub u8);
impl Daccr {
    #[doc = "DAC Output Voltage Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vosel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "DAC Output Voltage Select"]
    #[inline(always)]
    pub const fn set_vosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
    #[doc = "Supply Voltage Reference Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vrsel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Supply Voltage Reference Source Select"]
    #[inline(always)]
    pub const fn set_vrsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "DAC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dacen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DAC Enable"]
    #[inline(always)]
    pub const fn set_dacen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Daccr {
    #[inline(always)]
    fn default() -> Daccr {
        Daccr(0)
    }
}
impl core::fmt::Debug for Daccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Daccr")
            .field("vosel", &self.vosel())
            .field("vrsel", &self.vrsel())
            .field("dacen", &self.dacen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Daccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Daccr {{ vosel: {=u8:?}, vrsel: {=bool:?}, dacen: {=bool:?} }}",
            self.vosel(),
            self.vrsel(),
            self.dacen()
        )
    }
}
#[doc = "CMP Filter Period Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpr(pub u8);
impl Fpr {
    #[doc = "Filter Sample Period"]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Filter Sample Period"]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Fpr {
    #[inline(always)]
    fn default() -> Fpr {
        Fpr(0)
    }
}
impl core::fmt::Debug for Fpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fpr")
            .field("filt_per", &self.filt_per())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fpr {{ filt_per: {=u8:?} }}", self.filt_per())
    }
}
#[doc = "MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Muxcr(pub u8);
impl Muxcr {
    #[doc = "Minus Input Mux Control"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Msel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Msel::from_bits(val as u8)
    }
    #[doc = "Minus Input Mux Control"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Msel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u8) & 0x07) << 0usize);
    }
    #[doc = "Plus Input Mux Control"]
    #[must_use]
    #[inline(always)]
    pub const fn psel(&self) -> super::vals::Psel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Psel::from_bits(val as u8)
    }
    #[doc = "Plus Input Mux Control"]
    #[inline(always)]
    pub const fn set_psel(&mut self, val: super::vals::Psel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u8) & 0x07) << 3usize);
    }
}
impl Default for Muxcr {
    #[inline(always)]
    fn default() -> Muxcr {
        Muxcr(0)
    }
}
impl core::fmt::Debug for Muxcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Muxcr")
            .field("msel", &self.msel())
            .field("psel", &self.psel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Muxcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Muxcr {{ msel: {:?}, psel: {:?} }}",
            self.msel(),
            self.psel()
        )
    }
}
#[doc = "CMP Status and Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u8);
impl Scr {
    #[doc = "Analog Comparator Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cout(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Output"]
    #[inline(always)]
    pub const fn set_cout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn cff(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Falling"]
    #[inline(always)]
    pub const fn set_cff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Analog Comparator Flag Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Analog Comparator Flag Rising"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    #[must_use]
    #[inline(always)]
    pub const fn ief(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub const fn set_ief(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    #[must_use]
    #[inline(always)]
    pub const fn ier(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub const fn set_ier(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "DMA Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable Control"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("cout", &self.cout())
            .field("cff", &self.cff())
            .field("cfr", &self.cfr())
            .field("ief", &self.ief())
            .field("ier", &self.ier())
            .field("dmaen", &self.dmaen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ cout: {=bool:?}, cff: {=bool:?}, cfr: {=bool:?}, ief: {=bool:?}, ier: {=bool:?}, dmaen: {=bool:?} }}",
            self.cout(),
            self.cff(),
            self.cfr(),
            self.ief(),
            self.ier(),
            self.dmaen()
        )
    }
}
