#[doc = "Clock Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::CsrScs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::CsrScs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::CsrScs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Csr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "FIRC Auto-trimming Counter 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc1(pub u32);
impl Fircatc1 {
    #[doc = "Ideal Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn idealc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ideal Counter."]
    #[inline(always)]
    pub const fn set_idealc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Fircatc1 {
    #[inline(always)]
    fn default() -> Fircatc1 {
        Fircatc1(0)
    }
}
impl core::fmt::Debug for Fircatc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc1")
            .field("idealc", &self.idealc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fircatc1 {{ idealc: {=u16:?} }}", self.idealc())
    }
}
#[doc = "FIRC Auto-trimming Counter 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc2(pub u32);
impl Fircatc2 {
    #[doc = "Coarse Trim Minimum Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn coarminc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Minimum Counter."]
    #[inline(always)]
    pub const fn set_coarminc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Coarse Trim Maximum Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn coarmaxc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Coarse Trim Maximum Counter."]
    #[inline(always)]
    pub const fn set_coarmaxc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc2 {
    #[inline(always)]
    fn default() -> Fircatc2 {
        Fircatc2(0)
    }
}
impl core::fmt::Debug for Fircatc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc2")
            .field("coarminc", &self.coarminc())
            .field("coarmaxc", &self.coarmaxc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc2 {{ coarminc: {=u16:?}, coarmaxc: {=u16:?} }}",
            self.coarminc(),
            self.coarmaxc()
        )
    }
}
#[doc = "FIRC Auto-trimming Counter 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircatc3(pub u32);
impl Fircatc3 {
    #[doc = "Fine Trim Minimum Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn fineminc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Minimum Counter."]
    #[inline(always)]
    pub const fn set_fineminc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Fine Trim Maximum Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn finemaxc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Fine Trim Maximum Counter."]
    #[inline(always)]
    pub const fn set_finemaxc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Fircatc3 {
    #[inline(always)]
    fn default() -> Fircatc3 {
        Fircatc3(0)
    }
}
impl core::fmt::Debug for Fircatc3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircatc3")
            .field("fineminc", &self.fineminc())
            .field("finemaxc", &self.finemaxc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircatc3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircatc3 {{ fineminc: {=u16:?}, finemaxc: {=u16:?} }}",
            self.fineminc(),
            self.finemaxc()
        )
    }
}
#[doc = "FIRC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccfg(pub u32);
impl Firccfg {
    #[doc = "Frequency select."]
    #[must_use]
    #[inline(always)]
    pub const fn freq_sel(&self) -> super::vals::FreqSel {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::FreqSel::from_bits(val as u8)
    }
    #[doc = "Frequency select."]
    #[inline(always)]
    pub const fn set_freq_sel(&mut self, val: super::vals::FreqSel) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
}
impl Default for Firccfg {
    #[inline(always)]
    fn default() -> Firccfg {
        Firccfg(0)
    }
}
impl core::fmt::Debug for Firccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccfg")
            .field("freq_sel", &self.freq_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Firccfg {{ freq_sel: {:?} }}", self.freq_sel())
    }
}
#[doc = "FIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firccsr(pub u32);
impl Firccsr {
    #[doc = "FIRC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Enable."]
    #[inline(always)]
    pub const fn set_fircen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsten(&self) -> super::vals::Fircsten {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fircsten::from_bits(val as u8)
    }
    #[doc = "FIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_fircsten(&mut self, val: super::vals::Fircsten) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_sclk_periph_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_sclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firc_fclk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Clock to peripherals Enable."]
    #[inline(always)]
    pub const fn set_firc_fclk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "FRO_HF Trim Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn firctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FRO_HF Trim Enable."]
    #[inline(always)]
    pub const fn set_firctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIRC Trim Update."]
    #[must_use]
    #[inline(always)]
    pub const fn firctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Trim Update."]
    #[inline(always)]
    pub const fn set_firctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIRC TRIM LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::FirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "FIRC TRIM LOCK."]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::FirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::FirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::FirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::FirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FIRC Valid status."]
    #[must_use]
    #[inline(always)]
    pub const fn fircvld(&self) -> super::vals::Fircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Fircvld::from_bits(val as u8)
    }
    #[doc = "FIRC Valid status."]
    #[inline(always)]
    pub const fn set_fircvld(&mut self, val: super::vals::Fircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn fircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Selected."]
    #[inline(always)]
    pub const fn set_fircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "FIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr(&self) -> super::vals::Fircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Fircerr::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error."]
    #[inline(always)]
    pub const fn set_fircerr(&mut self, val: super::vals::Fircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircerr_ie(&self) -> super::vals::FircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::FircerrIe::from_bits(val as u8)
    }
    #[doc = "FIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircerr_ie(&mut self, val: super::vals::FircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc_ie(&self) -> super::vals::FircaccIe {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FircaccIe::from_bits(val as u8)
    }
    #[doc = "FIRC Accurate Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fircacc_ie(&mut self, val: super::vals::FircaccIe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FIRC Frequency Accurate."]
    #[must_use]
    #[inline(always)]
    pub const fn fircacc(&self) -> super::vals::Fircacc {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Fircacc::from_bits(val as u8)
    }
    #[doc = "FIRC Frequency Accurate."]
    #[inline(always)]
    pub const fn set_fircacc(&mut self, val: super::vals::Fircacc) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Firccsr {
    #[inline(always)]
    fn default() -> Firccsr {
        Firccsr(0)
    }
}
impl core::fmt::Debug for Firccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firccsr")
            .field("fircen", &self.fircen())
            .field("fircsten", &self.fircsten())
            .field("firc_sclk_periph_en", &self.firc_sclk_periph_en())
            .field("firc_fclk_periph_en", &self.firc_fclk_periph_en())
            .field("firctren", &self.firctren())
            .field("firctrup", &self.firctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("fircvld", &self.fircvld())
            .field("fircsel", &self.fircsel())
            .field("fircerr", &self.fircerr())
            .field("fircerr_ie", &self.fircerr_ie())
            .field("fircacc_ie", &self.fircacc_ie())
            .field("fircacc", &self.fircacc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firccsr {{ fircen: {=bool:?}, fircsten: {:?}, firc_sclk_periph_en: {=bool:?}, firc_fclk_periph_en: {=bool:?}, firctren: {=bool:?}, firctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, fircvld: {:?}, fircsel: {=bool:?}, fircerr: {:?}, fircerr_ie: {:?}, fircacc_ie: {:?}, fircacc: {:?} }}",
            self.fircen(),
            self.fircsten(),
            self.firc_sclk_periph_en(),
            self.firc_fclk_periph_en(),
            self.firctren(),
            self.firctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.fircvld(),
            self.fircsel(),
            self.fircerr(),
            self.fircerr_ie(),
            self.fircacc_ie(),
            self.fircacc()
        )
    }
}
#[doc = "FIRC Auto-trimming Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fircstat(pub u32);
impl Fircstat {
    #[doc = "Trim Fine."]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine."]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse."]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse."]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Fircstat {
    #[inline(always)]
    fn default() -> Fircstat {
        Fircstat(0)
    }
}
impl core::fmt::Debug for Fircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fircstat")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fircstat {{ trimfine: {=u8:?}, trimcoar: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar()
        )
    }
}
#[doc = "FIRC Trim Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctcfg(pub u32);
impl Firctcfg {
    #[doc = "Trim Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::FirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::FirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source."]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::FirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "FIRC Trim Pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "FIRC Trim Pre-divider."]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Firctcfg {
    #[inline(always)]
    fn default() -> Firctcfg {
        Firctcfg(0)
    }
}
impl core::fmt::Debug for Firctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "FIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Firctrim(pub u32);
impl Firctrim {
    #[doc = "Trim Fine."]
    #[must_use]
    #[inline(always)]
    pub const fn trimfine(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trim Fine."]
    #[inline(always)]
    pub const fn set_trimfine(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trim Coarse."]
    #[must_use]
    #[inline(always)]
    pub const fn trimcoar(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Coarse."]
    #[inline(always)]
    pub const fn set_trimcoar(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temperature2."]
    #[must_use]
    #[inline(always)]
    pub const fn trimtemp2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Trim Temperature2."]
    #[inline(always)]
    pub const fn set_trimtemp2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Trim Start."]
    #[must_use]
    #[inline(always)]
    pub const fn trimstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trim Start."]
    #[inline(always)]
    pub const fn set_trimstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Firctrim {
    #[inline(always)]
    fn default() -> Firctrim {
        Firctrim(0)
    }
}
impl core::fmt::Debug for Firctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Firctrim")
            .field("trimfine", &self.trimfine())
            .field("trimcoar", &self.trimcoar())
            .field("trimtemp2", &self.trimtemp2())
            .field("trimstart", &self.trimstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Firctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Firctrim {{ trimfine: {=u8:?}, trimcoar: {=u8:?}, trimtemp2: {=u8:?}, trimstart: {=u8:?} }}",
            self.trimfine(),
            self.trimcoar(),
            self.trimtemp2(),
            self.trimstart()
        )
    }
}
#[doc = "Parameter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "SOSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn soscclkpres(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Present."]
    #[inline(always)]
    pub const fn set_soscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sircclkpres(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock Present."]
    #[inline(always)]
    pub const fn set_sircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIRC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn fircclkpres(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIRC Clock Present."]
    #[inline(always)]
    pub const fn set_fircclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ROSC Clock Present."]
    #[must_use]
    #[inline(always)]
    pub const fn roscclkpres(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Clock Present."]
    #[inline(always)]
    pub const fn set_roscclkpres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("soscclkpres", &self.soscclkpres())
            .field("sircclkpres", &self.sircclkpres())
            .field("fircclkpres", &self.fircclkpres())
            .field("roscclkpres", &self.roscclkpres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ soscclkpres: {=bool:?}, sircclkpres: {=bool:?}, fircclkpres: {=bool:?}, roscclkpres: {=bool:?} }}",
            self.soscclkpres(),
            self.sircclkpres(),
            self.fircclkpres(),
            self.roscclkpres()
        )
    }
}
#[doc = "Run Clock Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rccr(pub u32);
impl Rccr {
    #[doc = "System Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn scs(&self) -> super::vals::RccrScs {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::RccrScs::from_bits(val as u8)
    }
    #[doc = "System Clock Source."]
    #[inline(always)]
    pub const fn set_scs(&mut self, val: super::vals::RccrScs) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Rccr {
    #[inline(always)]
    fn default() -> Rccr {
        Rccr(0)
    }
}
impl core::fmt::Debug for Rccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rccr").field("scs", &self.scs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rccr {{ scs: {:?} }}", self.scs())
    }
}
#[doc = "ROSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosccsr(pub u32);
impl Rosccsr {
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::RosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::RosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::RosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ROSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn roscvld(&self) -> super::vals::Roscvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Roscvld::from_bits(val as u8)
    }
    #[doc = "ROSC Valid."]
    #[inline(always)]
    pub const fn set_roscvld(&mut self, val: super::vals::Roscvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ROSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn roscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "ROSC Selected."]
    #[inline(always)]
    pub const fn set_roscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "ROSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn roscerr(&self) -> super::vals::Roscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Roscerr::from_bits(val as u8)
    }
    #[doc = "ROSC Clock Error."]
    #[inline(always)]
    pub const fn set_roscerr(&mut self, val: super::vals::Roscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Rosccsr {
    #[inline(always)]
    fn default() -> Rosccsr {
        Rosccsr(0)
    }
}
impl core::fmt::Debug for Rosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rosccsr")
            .field("lk", &self.lk())
            .field("roscvld", &self.roscvld())
            .field("roscsel", &self.roscsel())
            .field("roscerr", &self.roscerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rosccsr {{ lk: {:?}, roscvld: {:?}, roscsel: {=bool:?}, roscerr: {:?} }}",
            self.lk(),
            self.roscvld(),
            self.roscsel(),
            self.roscerr()
        )
    }
}
#[doc = "SIRC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirccsr(pub u32);
impl Sirccsr {
    #[doc = "SIRC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Stop Enable."]
    #[inline(always)]
    pub const fn set_sircsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sirc_clk_periph_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Clock to Peripherals Enable."]
    #[inline(always)]
    pub const fn set_sirc_clk_periph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctren(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG\\[RANGE\\]=1)."]
    #[inline(always)]
    pub const fn set_sirctren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SIRC Trim Update."]
    #[must_use]
    #[inline(always)]
    pub const fn sirctrup(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Trim Update."]
    #[inline(always)]
    pub const fn set_sirctrup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SIRC TRIM LOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock(&self) -> super::vals::SirccsrTrimLock {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SirccsrTrimLock::from_bits(val as u8)
    }
    #[doc = "SIRC TRIM LOCK."]
    #[inline(always)]
    pub const fn set_trim_lock(&mut self, val: super::vals::SirccsrTrimLock) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn coarse_trim_bypass(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Coarse Auto Trim Bypass."]
    #[inline(always)]
    pub const fn set_coarse_trim_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SirccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SirccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SirccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SIRC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn sircvld(&self) -> super::vals::Sircvld {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Sircvld::from_bits(val as u8)
    }
    #[doc = "SIRC Valid."]
    #[inline(always)]
    pub const fn set_sircvld(&mut self, val: super::vals::Sircvld) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SIRC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn sircsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SIRC Selected."]
    #[inline(always)]
    pub const fn set_sircsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SIRC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr(&self) -> super::vals::Sircerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Sircerr::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error."]
    #[inline(always)]
    pub const fn set_sircerr(&mut self, val: super::vals::Sircerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sircerr_ie(&self) -> super::vals::SircerrIe {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SircerrIe::from_bits(val as u8)
    }
    #[doc = "SIRC Clock Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sircerr_ie(&mut self, val: super::vals::SircerrIe) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Sirccsr {
    #[inline(always)]
    fn default() -> Sirccsr {
        Sirccsr(0)
    }
}
impl core::fmt::Debug for Sirccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirccsr")
            .field("sircsten", &self.sircsten())
            .field("sirc_clk_periph_en", &self.sirc_clk_periph_en())
            .field("sirctren", &self.sirctren())
            .field("sirctrup", &self.sirctrup())
            .field("trim_lock", &self.trim_lock())
            .field("coarse_trim_bypass", &self.coarse_trim_bypass())
            .field("lk", &self.lk())
            .field("sircvld", &self.sircvld())
            .field("sircsel", &self.sircsel())
            .field("sircerr", &self.sircerr())
            .field("sircerr_ie", &self.sircerr_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirccsr {{ sircsten: {=bool:?}, sirc_clk_periph_en: {=bool:?}, sirctren: {=bool:?}, sirctrup: {=bool:?}, trim_lock: {:?}, coarse_trim_bypass: {=bool:?}, lk: {:?}, sircvld: {:?}, sircsel: {=bool:?}, sircerr: {:?}, sircerr_ie: {:?} }}",
            self.sircsten(),
            self.sirc_clk_periph_en(),
            self.sirctren(),
            self.sirctrup(),
            self.trim_lock(),
            self.coarse_trim_bypass(),
            self.lk(),
            self.sircvld(),
            self.sircsel(),
            self.sircerr(),
            self.sircerr_ie()
        )
    }
}
#[doc = "SIRC Auto-trimming Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sircstat(pub u32);
impl Sircstat {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sircstat {
    #[inline(always)]
    fn default() -> Sircstat {
        Sircstat(0)
    }
}
impl core::fmt::Debug for Sircstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sircstat")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sircstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sircstat {{ ccotrim: {=u8:?}, cltrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim()
        )
    }
}
#[doc = "SIRC Trim Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctcfg(pub u32);
impl Sirctcfg {
    #[doc = "Trim Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trimsrc(&self) -> super::vals::SirctcfgTrimsrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SirctcfgTrimsrc::from_bits(val as u8)
    }
    #[doc = "Trim Source."]
    #[inline(always)]
    pub const fn set_trimsrc(&mut self, val: super::vals::SirctcfgTrimsrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SIRC Trim Pre-divider."]
    #[must_use]
    #[inline(always)]
    pub const fn trimdiv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "SIRC Trim Pre-divider."]
    #[inline(always)]
    pub const fn set_trimdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Sirctcfg {
    #[inline(always)]
    fn default() -> Sirctcfg {
        Sirctcfg(0)
    }
}
impl core::fmt::Debug for Sirctcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctcfg")
            .field("trimsrc", &self.trimsrc())
            .field("trimdiv", &self.trimdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctcfg {{ trimsrc: {:?}, trimdiv: {=u8:?} }}",
            self.trimsrc(),
            self.trimdiv()
        )
    }
}
#[doc = "SIRC Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sirctrim(pub u32);
impl Sirctrim {
    #[doc = "CCO Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn ccotrim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CCO Trim."]
    #[inline(always)]
    pub const fn set_ccotrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CL Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn cltrim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "CL Trim."]
    #[inline(always)]
    pub const fn set_cltrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Trim Temp."]
    #[must_use]
    #[inline(always)]
    pub const fn tctrim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Temp."]
    #[inline(always)]
    pub const fn set_tctrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[must_use]
    #[inline(always)]
    pub const fn fvchtrim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Calibrates the replica voltage in FSU for CCO to get well frequency at initial period."]
    #[inline(always)]
    pub const fn set_fvchtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Sirctrim {
    #[inline(always)]
    fn default() -> Sirctrim {
        Sirctrim(0)
    }
}
impl core::fmt::Debug for Sirctrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sirctrim")
            .field("ccotrim", &self.ccotrim())
            .field("cltrim", &self.cltrim())
            .field("tctrim", &self.tctrim())
            .field("fvchtrim", &self.fvchtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sirctrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sirctrim {{ ccotrim: {=u8:?}, cltrim: {=u8:?}, tctrim: {=u8:?}, fvchtrim: {=u8:?} }}",
            self.ccotrim(),
            self.cltrim(),
            self.tctrim(),
            self.fvchtrim()
        )
    }
}
#[doc = "SOSC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccfg(pub u32);
impl Sosccfg {
    #[doc = "External Reference Select."]
    #[must_use]
    #[inline(always)]
    pub const fn erefs(&self) -> super::vals::Erefs {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Erefs::from_bits(val as u8)
    }
    #[doc = "External Reference Select."]
    #[inline(always)]
    pub const fn set_erefs(&mut self, val: super::vals::Erefs) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SOSC Range Select."]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "SOSC Range Select."]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sosccfg {
    #[inline(always)]
    fn default() -> Sosccfg {
        Sosccfg(0)
    }
}
impl core::fmt::Debug for Sosccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccfg")
            .field("erefs", &self.erefs())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccfg {{ erefs: {:?}, range: {:?} }}",
            self.erefs(),
            self.range()
        )
    }
}
#[doc = "SOSC Control Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccsr(pub u32);
impl Sosccsr {
    #[doc = "SOSC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Enable."]
    #[inline(always)]
    pub const fn set_soscen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SOSC Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Stop Enable."]
    #[inline(always)]
    pub const fn set_soscsten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Clock Monitor Enable."]
    #[inline(always)]
    pub const fn set_sosccm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sosccmre(&self) -> super::vals::Sosccmre {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Sosccmre::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Monitor Reset Enable."]
    #[inline(always)]
    pub const fn set_sosccmre(&mut self, val: super::vals::Sosccmre) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock Register."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::SosccsrLk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SosccsrLk::from_bits(val as u8)
    }
    #[doc = "Lock Register."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::SosccsrLk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SOSC Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid."]
    #[inline(always)]
    pub const fn set_soscvld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SOSC Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn soscsel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Selected."]
    #[inline(always)]
    pub const fn set_soscsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "SOSC Clock Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soscerr(&self) -> super::vals::Soscerr {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Soscerr::from_bits(val as u8)
    }
    #[doc = "SOSC Clock Error."]
    #[inline(always)]
    pub const fn set_soscerr(&mut self, val: super::vals::Soscerr) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn soscvld_ie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SOSC Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_soscvld_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Sosccsr {
    #[inline(always)]
    fn default() -> Sosccsr {
        Sosccsr(0)
    }
}
impl core::fmt::Debug for Sosccsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sosccsr")
            .field("soscen", &self.soscen())
            .field("soscsten", &self.soscsten())
            .field("sosccm", &self.sosccm())
            .field("sosccmre", &self.sosccmre())
            .field("lk", &self.lk())
            .field("soscvld", &self.soscvld())
            .field("soscsel", &self.soscsel())
            .field("soscerr", &self.soscerr())
            .field("soscvld_ie", &self.soscvld_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sosccsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sosccsr {{ soscen: {=bool:?}, soscsten: {=bool:?}, sosccm: {=bool:?}, sosccmre: {:?}, lk: {:?}, soscvld: {=bool:?}, soscsel: {=bool:?}, soscerr: {:?}, soscvld_ie: {=bool:?} }}",
            self.soscen(),
            self.soscsten(),
            self.sosccm(),
            self.sosccmre(),
            self.lk(),
            self.soscvld(),
            self.soscsel(),
            self.soscerr(),
            self.soscvld_ie()
        )
    }
}
#[doc = "Trim Lock register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimLock(pub u32);
impl TrimLock {
    #[doc = "TRIM_UNLOCK."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_unlock(&self) -> super::vals::TrimUnlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TrimUnlock::from_bits(val as u8)
    }
    #[doc = "TRIM_UNLOCK."]
    #[inline(always)]
    pub const fn set_trim_unlock(&mut self, val: super::vals::TrimUnlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR_DISABLE."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_disable(&self) -> super::vals::IfrDisable {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IfrDisable::from_bits(val as u8)
    }
    #[doc = "IFR_DISABLE."]
    #[inline(always)]
    pub const fn set_ifr_disable(&mut self, val: super::vals::IfrDisable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_lock_key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIM_LOCK_KEY."]
    #[inline(always)]
    pub const fn set_trim_lock_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TrimLock {
    #[inline(always)]
    fn default() -> TrimLock {
        TrimLock(0)
    }
}
impl core::fmt::Debug for TrimLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimLock")
            .field("trim_unlock", &self.trim_unlock())
            .field("ifr_disable", &self.ifr_disable())
            .field("trim_lock_key", &self.trim_lock_key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimLock {{ trim_unlock: {:?}, ifr_disable: {:?}, trim_lock_key: {=u16:?} }}",
            self.trim_unlock(),
            self.ifr_disable(),
            self.trim_lock_key()
        )
    }
}
#[doc = "Version ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "SCG Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SCG Version Number."]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Verid {{ version: {=u32:?} }}", self.version())
    }
}
