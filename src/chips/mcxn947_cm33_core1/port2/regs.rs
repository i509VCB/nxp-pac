#[doc = "Calibration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib0(pub u32);
impl Calib0 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib0 {
    #[inline(always)]
    fn default() -> Calib0 {
        Calib0(0)
    }
}
impl core::fmt::Debug for Calib0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib0")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib0 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Calibration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Calib1(pub u32);
impl Calib1 {
    #[doc = "Calibration of NMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn ncal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of NMOS Output Driver"]
    #[inline(always)]
    pub const fn set_ncal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[must_use]
    #[inline(always)]
    pub const fn pcal(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Calibration of PMOS Output Driver"]
    #[inline(always)]
    pub const fn set_pcal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Calib1 {
    #[inline(always)]
    fn default() -> Calib1 {
        Calib1(0)
    }
}
impl core::fmt::Debug for Calib1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Calib1")
            .field("ncal", &self.ncal())
            .field("pcal", &self.pcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Calib1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Calib1 {{ ncal: {=u8:?}, pcal: {=u8:?} }}",
            self.ncal(),
            self.pcal()
        )
    }
}
#[doc = "Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Port Voltage Range"]
    #[must_use]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "Port Voltage Range"]
    #[inline(always)]
    pub const fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Config {{ range: {:?} }}", self.range())
    }
}
#[doc = "EFT Detect Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edcr(pub u32);
impl Edcr {
    #[doc = "EFT Detect High Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn edhc(&self) -> super::vals::Edhc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Edhc::from_bits(val as u8)
    }
    #[doc = "EFT Detect High Clear"]
    #[inline(always)]
    pub const fn set_edhc(&mut self, val: super::vals::Edhc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "EFT Detect Low Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn edlc(&self) -> super::vals::Edlc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Edlc::from_bits(val as u8)
    }
    #[doc = "EFT Detect Low Clear"]
    #[inline(always)]
    pub const fn set_edlc(&mut self, val: super::vals::Edlc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Edcr {
    #[inline(always)]
    fn default() -> Edcr {
        Edcr(0)
    }
}
impl core::fmt::Debug for Edcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edcr")
            .field("edhc", &self.edhc())
            .field("edlc", &self.edlc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Edcr {{ edhc: {:?}, edlc: {:?} }}",
            self.edhc(),
            self.edlc()
        )
    }
}
#[doc = "EFT Detect Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edfr(pub u32);
impl Edfr {
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf0(&self) -> super::vals::Edf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Edf0::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf0(&mut self, val: super::vals::Edf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf1(&self) -> super::vals::Edf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Edf1::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf1(&mut self, val: super::vals::Edf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf2(&self) -> super::vals::Edf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Edf2::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf2(&mut self, val: super::vals::Edf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf3(&self) -> super::vals::Edf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Edf3::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf3(&mut self, val: super::vals::Edf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf4(&self) -> super::vals::Edf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Edf4::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf4(&mut self, val: super::vals::Edf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf5(&self) -> super::vals::Edf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Edf5::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf5(&mut self, val: super::vals::Edf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf6(&self) -> super::vals::Edf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Edf6::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf6(&mut self, val: super::vals::Edf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf7(&self) -> super::vals::Edf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Edf7::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf7(&mut self, val: super::vals::Edf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf8(&self) -> super::vals::Edf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Edf8::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf8(&mut self, val: super::vals::Edf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf9(&self) -> super::vals::Edf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Edf9::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf9(&mut self, val: super::vals::Edf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf10(&self) -> super::vals::Edf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Edf10::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf10(&mut self, val: super::vals::Edf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "EFT Detect Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn edf11(&self) -> super::vals::Edf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Edf11::from_bits(val as u8)
    }
    #[doc = "EFT Detect Flag"]
    #[inline(always)]
    pub const fn set_edf11(&mut self, val: super::vals::Edf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Edfr {
    #[inline(always)]
    fn default() -> Edfr {
        Edfr(0)
    }
}
impl core::fmt::Debug for Edfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edfr")
            .field("edf0", &self.edf0())
            .field("edf1", &self.edf1())
            .field("edf2", &self.edf2())
            .field("edf3", &self.edf3())
            .field("edf4", &self.edf4())
            .field("edf5", &self.edf5())
            .field("edf6", &self.edf6())
            .field("edf7", &self.edf7())
            .field("edf8", &self.edf8())
            .field("edf9", &self.edf9())
            .field("edf10", &self.edf10())
            .field("edf11", &self.edf11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Edfr {{ edf0: {:?}, edf1: {:?}, edf2: {:?}, edf3: {:?}, edf4: {:?}, edf5: {:?}, edf6: {:?}, edf7: {:?}, edf8: {:?}, edf9: {:?}, edf10: {:?}, edf11: {:?} }}",
            self.edf0(),
            self.edf1(),
            self.edf2(),
            self.edf3(),
            self.edf4(),
            self.edf5(),
            self.edf6(),
            self.edf7(),
            self.edf8(),
            self.edf9(),
            self.edf10(),
            self.edf11()
        )
    }
}
#[doc = "EFT Detect Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edier(pub u32);
impl Edier {
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie0(&self) -> super::vals::Edie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Edie0::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie0(&mut self, val: super::vals::Edie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie1(&self) -> super::vals::Edie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Edie1::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie1(&mut self, val: super::vals::Edie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie2(&self) -> super::vals::Edie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Edie2::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie2(&mut self, val: super::vals::Edie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie3(&self) -> super::vals::Edie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Edie3::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie3(&mut self, val: super::vals::Edie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie4(&self) -> super::vals::Edie4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Edie4::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie4(&mut self, val: super::vals::Edie4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie5(&self) -> super::vals::Edie5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Edie5::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie5(&mut self, val: super::vals::Edie5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie6(&self) -> super::vals::Edie6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Edie6::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie6(&mut self, val: super::vals::Edie6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie7(&self) -> super::vals::Edie7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Edie7::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie7(&mut self, val: super::vals::Edie7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie8(&self) -> super::vals::Edie8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Edie8::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie8(&mut self, val: super::vals::Edie8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie9(&self) -> super::vals::Edie9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Edie9::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie9(&mut self, val: super::vals::Edie9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie10(&self) -> super::vals::Edie10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Edie10::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie10(&mut self, val: super::vals::Edie10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn edie11(&self) -> super::vals::Edie11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Edie11::from_bits(val as u8)
    }
    #[doc = "EFT Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_edie11(&mut self, val: super::vals::Edie11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Edier {
    #[inline(always)]
    fn default() -> Edier {
        Edier(0)
    }
}
impl core::fmt::Debug for Edier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edier")
            .field("edie0", &self.edie0())
            .field("edie1", &self.edie1())
            .field("edie2", &self.edie2())
            .field("edie3", &self.edie3())
            .field("edie4", &self.edie4())
            .field("edie5", &self.edie5())
            .field("edie6", &self.edie6())
            .field("edie7", &self.edie7())
            .field("edie8", &self.edie8())
            .field("edie9", &self.edie9())
            .field("edie10", &self.edie10())
            .field("edie11", &self.edie11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Edier {{ edie0: {:?}, edie1: {:?}, edie2: {:?}, edie3: {:?}, edie4: {:?}, edie5: {:?}, edie6: {:?}, edie7: {:?}, edie8: {:?}, edie9: {:?}, edie10: {:?}, edie11: {:?} }}",
            self.edie0(),
            self.edie1(),
            self.edie2(),
            self.edie3(),
            self.edie4(),
            self.edie5(),
            self.edie6(),
            self.edie7(),
            self.edie8(),
            self.edie9(),
            self.edie10(),
            self.edie11()
        )
    }
}
#[doc = "Global Pin Control High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpchr(pub u32);
impl Gpchr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe16(&self) -> super::vals::Gpwe16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpwe16::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe16(&mut self, val: super::vals::Gpwe16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe17(&self) -> super::vals::Gpwe17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gpwe17::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe17(&mut self, val: super::vals::Gpwe17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe18(&self) -> super::vals::Gpwe18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gpwe18::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe18(&mut self, val: super::vals::Gpwe18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe19(&self) -> super::vals::Gpwe19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpwe19::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe19(&mut self, val: super::vals::Gpwe19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe20(&self) -> super::vals::Gpwe20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpwe20::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe20(&mut self, val: super::vals::Gpwe20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe21(&self) -> super::vals::Gpwe21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gpwe21::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe21(&mut self, val: super::vals::Gpwe21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe22(&self) -> super::vals::Gpwe22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gpwe22::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe22(&mut self, val: super::vals::Gpwe22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe23(&self) -> super::vals::Gpwe23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpwe23::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe23(&mut self, val: super::vals::Gpwe23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe24(&self) -> super::vals::Gpwe24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpwe24::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe24(&mut self, val: super::vals::Gpwe24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe25(&self) -> super::vals::Gpwe25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Gpwe25::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe25(&mut self, val: super::vals::Gpwe25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe26(&self) -> super::vals::Gpwe26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Gpwe26::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe26(&mut self, val: super::vals::Gpwe26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe27(&self) -> super::vals::Gpwe27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpwe27::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe27(&mut self, val: super::vals::Gpwe27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe28(&self) -> super::vals::Gpwe28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpwe28::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe28(&mut self, val: super::vals::Gpwe28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe29(&self) -> super::vals::Gpwe29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Gpwe29::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe29(&mut self, val: super::vals::Gpwe29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe30(&self) -> super::vals::Gpwe30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gpwe30::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe30(&mut self, val: super::vals::Gpwe30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe31(&self) -> super::vals::Gpwe31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpwe31::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe31(&mut self, val: super::vals::Gpwe31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpchr {
    #[inline(always)]
    fn default() -> Gpchr {
        Gpchr(0)
    }
}
impl core::fmt::Debug for Gpchr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpchr")
            .field("gpwd", &self.gpwd())
            .field("gpwe16", &self.gpwe16())
            .field("gpwe17", &self.gpwe17())
            .field("gpwe18", &self.gpwe18())
            .field("gpwe19", &self.gpwe19())
            .field("gpwe20", &self.gpwe20())
            .field("gpwe21", &self.gpwe21())
            .field("gpwe22", &self.gpwe22())
            .field("gpwe23", &self.gpwe23())
            .field("gpwe24", &self.gpwe24())
            .field("gpwe25", &self.gpwe25())
            .field("gpwe26", &self.gpwe26())
            .field("gpwe27", &self.gpwe27())
            .field("gpwe28", &self.gpwe28())
            .field("gpwe29", &self.gpwe29())
            .field("gpwe30", &self.gpwe30())
            .field("gpwe31", &self.gpwe31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpchr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpchr {{ gpwd: {=u16:?}, gpwe16: {:?}, gpwe17: {:?}, gpwe18: {:?}, gpwe19: {:?}, gpwe20: {:?}, gpwe21: {:?}, gpwe22: {:?}, gpwe23: {:?}, gpwe24: {:?}, gpwe25: {:?}, gpwe26: {:?}, gpwe27: {:?}, gpwe28: {:?}, gpwe29: {:?}, gpwe30: {:?}, gpwe31: {:?} }}",
            self.gpwd(),
            self.gpwe16(),
            self.gpwe17(),
            self.gpwe18(),
            self.gpwe19(),
            self.gpwe20(),
            self.gpwe21(),
            self.gpwe22(),
            self.gpwe23(),
            self.gpwe24(),
            self.gpwe25(),
            self.gpwe26(),
            self.gpwe27(),
            self.gpwe28(),
            self.gpwe29(),
            self.gpwe30(),
            self.gpwe31()
        )
    }
}
#[doc = "Global Pin Control Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpclr(pub u32);
impl Gpclr {
    #[doc = "Global Pin Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Pin Write Data"]
    #[inline(always)]
    pub const fn set_gpwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe0(&self) -> super::vals::Gpwe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Gpwe0::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe0(&mut self, val: super::vals::Gpwe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe1(&self) -> super::vals::Gpwe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Gpwe1::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe1(&mut self, val: super::vals::Gpwe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe2(&self) -> super::vals::Gpwe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Gpwe2::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe2(&mut self, val: super::vals::Gpwe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe3(&self) -> super::vals::Gpwe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Gpwe3::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe3(&mut self, val: super::vals::Gpwe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe4(&self) -> super::vals::Gpwe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Gpwe4::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe4(&mut self, val: super::vals::Gpwe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe5(&self) -> super::vals::Gpwe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Gpwe5::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe5(&mut self, val: super::vals::Gpwe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe6(&self) -> super::vals::Gpwe6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Gpwe6::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe6(&mut self, val: super::vals::Gpwe6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe7(&self) -> super::vals::Gpwe7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpwe7::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe7(&mut self, val: super::vals::Gpwe7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe8(&self) -> super::vals::Gpwe8 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpwe8::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe8(&mut self, val: super::vals::Gpwe8) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe9(&self) -> super::vals::Gpwe9 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Gpwe9::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe9(&mut self, val: super::vals::Gpwe9) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe10(&self) -> super::vals::Gpwe10 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Gpwe10::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe10(&mut self, val: super::vals::Gpwe10) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe11(&self) -> super::vals::Gpwe11 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Gpwe11::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe11(&mut self, val: super::vals::Gpwe11) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe12(&self) -> super::vals::Gpwe12 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Gpwe12::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe12(&mut self, val: super::vals::Gpwe12) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe13(&self) -> super::vals::Gpwe13 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Gpwe13::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe13(&mut self, val: super::vals::Gpwe13) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe14(&self) -> super::vals::Gpwe14 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gpwe14::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe14(&mut self, val: super::vals::Gpwe14) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Global Pin Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpwe15(&self) -> super::vals::Gpwe15 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gpwe15::from_bits(val as u8)
    }
    #[doc = "Global Pin Write Enable"]
    #[inline(always)]
    pub const fn set_gpwe15(&mut self, val: super::vals::Gpwe15) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpclr {
    #[inline(always)]
    fn default() -> Gpclr {
        Gpclr(0)
    }
}
impl core::fmt::Debug for Gpclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpclr")
            .field("gpwd", &self.gpwd())
            .field("gpwe0", &self.gpwe0())
            .field("gpwe1", &self.gpwe1())
            .field("gpwe2", &self.gpwe2())
            .field("gpwe3", &self.gpwe3())
            .field("gpwe4", &self.gpwe4())
            .field("gpwe5", &self.gpwe5())
            .field("gpwe6", &self.gpwe6())
            .field("gpwe7", &self.gpwe7())
            .field("gpwe8", &self.gpwe8())
            .field("gpwe9", &self.gpwe9())
            .field("gpwe10", &self.gpwe10())
            .field("gpwe11", &self.gpwe11())
            .field("gpwe12", &self.gpwe12())
            .field("gpwe13", &self.gpwe13())
            .field("gpwe14", &self.gpwe14())
            .field("gpwe15", &self.gpwe15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpclr {{ gpwd: {=u16:?}, gpwe0: {:?}, gpwe1: {:?}, gpwe2: {:?}, gpwe3: {:?}, gpwe4: {:?}, gpwe5: {:?}, gpwe6: {:?}, gpwe7: {:?}, gpwe8: {:?}, gpwe9: {:?}, gpwe10: {:?}, gpwe11: {:?}, gpwe12: {:?}, gpwe13: {:?}, gpwe14: {:?}, gpwe15: {:?} }}",
            self.gpwd(),
            self.gpwe0(),
            self.gpwe1(),
            self.gpwe2(),
            self.gpwe3(),
            self.gpwe4(),
            self.gpwe5(),
            self.gpwe6(),
            self.gpwe7(),
            self.gpwe8(),
            self.gpwe9(),
            self.gpwe10(),
            self.gpwe11(),
            self.gpwe12(),
            self.gpwe13(),
            self.gpwe14(),
            self.gpwe15()
        )
    }
}
#[doc = "Pin Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr0(pub u32);
impl Pcr0 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr0Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr0Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr0Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr0Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr0Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr0Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr0Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr0Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr0Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr0Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr0Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr0Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr0Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr0Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr0Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr0Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr0Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr0Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr0Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr0Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr0Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr0Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr0Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr0Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr0Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr0 {
    #[inline(always)]
    fn default() -> Pcr0 {
        Pcr0(0)
    }
}
impl core::fmt::Debug for Pcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr0")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr0 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr1(pub u32);
impl Pcr1 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr1Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr1Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr1Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr1Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr1Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr1Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr1Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr1Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr1Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr1Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr1Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr1Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr1Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr1Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr1Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr1Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr1Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr1Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr1Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr1Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr1Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr1Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr1Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr1Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr1Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr1Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr1Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr1 {
    #[inline(always)]
    fn default() -> Pcr1 {
        Pcr1(0)
    }
}
impl core::fmt::Debug for Pcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr1")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr1 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr10(pub u32);
impl Pcr10 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr10Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr10Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr10Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr10Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr10Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr10Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr10Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr10Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr10Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr10Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr10Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr10Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr10Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr10Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr10Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr10Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr10Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr10Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr10Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr10Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr10Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr10Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr10Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr10Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr10Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr10Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr10Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr10 {
    #[inline(always)]
    fn default() -> Pcr10 {
        Pcr10(0)
    }
}
impl core::fmt::Debug for Pcr10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr10")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr10 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr11(pub u32);
impl Pcr11 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr11Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr11Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr11Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr11Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr11Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr11Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr11Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr11Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr11Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr11Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr11Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr11Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr11Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr11Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr11Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr11Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr11Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr11Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr11Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr11Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr11Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr11Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr11Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr11Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr11Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr11Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr11Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr11 {
    #[inline(always)]
    fn default() -> Pcr11 {
        Pcr11(0)
    }
}
impl core::fmt::Debug for Pcr11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr11")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr11 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr2(pub u32);
impl Pcr2 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr2Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr2Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr2Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr2Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr2Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr2Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr2Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr2Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr2Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr2Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr2Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr2Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr2Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr2Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr2Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr2Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr2Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr2Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr2Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr2Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr2Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr2Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr2Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr2Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr2Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr2Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr2Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr2 {
    #[inline(always)]
    fn default() -> Pcr2 {
        Pcr2(0)
    }
}
impl core::fmt::Debug for Pcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr2")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr2 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr3(pub u32);
impl Pcr3 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr3Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr3Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr3Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr3Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr3Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr3Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr3Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr3Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr3Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr3Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr3Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr3Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr3Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr3Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr3Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr3Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr3Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr3Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr3Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr3Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr3Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr3Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr3Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr3Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr3Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr3Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr3Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr3 {
    #[inline(always)]
    fn default() -> Pcr3 {
        Pcr3(0)
    }
}
impl core::fmt::Debug for Pcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr3")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr3 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr4(pub u32);
impl Pcr4 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr4Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr4Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr4Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr4Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr4Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr4Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr4Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr4Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr4Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr4Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr4Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr4Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr4Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr4Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr4Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr4Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr4Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr4Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr4Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr4Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr4Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr4Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr4Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr4Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr4Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr4Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr4Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr4 {
    #[inline(always)]
    fn default() -> Pcr4 {
        Pcr4(0)
    }
}
impl core::fmt::Debug for Pcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr4")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr4 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr5(pub u32);
impl Pcr5 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr5Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr5Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr5Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr5Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr5Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr5Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr5Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr5Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr5Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr5Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr5Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr5Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr5Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr5Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr5Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr5Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr5Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr5Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr5Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr5Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr5Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr5Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr5Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr5Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr5Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr5Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr5Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr5 {
    #[inline(always)]
    fn default() -> Pcr5 {
        Pcr5(0)
    }
}
impl core::fmt::Debug for Pcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr5")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr5 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr6(pub u32);
impl Pcr6 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr6Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr6Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr6Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr6Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr6Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr6Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr6Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr6Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr6Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr6Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr6Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr6Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr6Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr6Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr6Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr6Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr6Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr6Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr6Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr6Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr6Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr6Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr6Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr6Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr6Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr6Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr6Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr6 {
    #[inline(always)]
    fn default() -> Pcr6 {
        Pcr6(0)
    }
}
impl core::fmt::Debug for Pcr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr6")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr6 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr7(pub u32);
impl Pcr7 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr7Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr7Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr7Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr7Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr7Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr7Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr7Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr7Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr7Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr7Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr7Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr7Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr7Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr7Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr7Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr7Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr7Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr7Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr7Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr7Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr7Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr7Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr7Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr7Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr7Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr7Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr7Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr7 {
    #[inline(always)]
    fn default() -> Pcr7 {
        Pcr7(0)
    }
}
impl core::fmt::Debug for Pcr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr7")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr7 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr8(pub u32);
impl Pcr8 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr8Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr8Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr8Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr8Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr8Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr8Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr8Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr8Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr8Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr8Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr8Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr8Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr8Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr8Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr8Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr8Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr8Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr8Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr8Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr8Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr8Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr8Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr8Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr8Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr8Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr8Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr8Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr8 {
    #[inline(always)]
    fn default() -> Pcr8 {
        Pcr8(0)
    }
}
impl core::fmt::Debug for Pcr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr8")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr8 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
        )
    }
}
#[doc = "Pin Control 9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr9(pub u32);
impl Pcr9 {
    #[doc = "Pull Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Pcr9Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pcr9Ps::from_bits(val as u8)
    }
    #[doc = "Pull Select"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Pcr9Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Pull Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> super::vals::Pcr9Pe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pcr9Pe::from_bits(val as u8)
    }
    #[doc = "Pull Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: super::vals::Pcr9Pe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slew Rate Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> super::vals::Pcr9Sre {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pcr9Sre::from_bits(val as u8)
    }
    #[doc = "Slew Rate Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: super::vals::Pcr9Sre) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Open Drain Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> super::vals::Pcr9Ode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pcr9Ode::from_bits(val as u8)
    }
    #[doc = "Open Drain Enable"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: super::vals::Pcr9Ode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Drive Strength Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Pcr9Dse {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pcr9Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Enable"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Pcr9Dse) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Pin Multiplex Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> super::vals::Pcr9Mux {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcr9Mux::from_bits(val as u8)
    }
    #[doc = "Pin Multiplex Control"]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: super::vals::Pcr9Mux) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Input Buffer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibe(&self) -> super::vals::Pcr9Ibe {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pcr9Ibe::from_bits(val as u8)
    }
    #[doc = "Input Buffer Enable"]
    #[inline(always)]
    pub const fn set_ibe(&mut self, val: super::vals::Pcr9Ibe) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert Input"]
    #[must_use]
    #[inline(always)]
    pub const fn inv(&self) -> super::vals::Pcr9Inv {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pcr9Inv::from_bits(val as u8)
    }
    #[doc = "Invert Input"]
    #[inline(always)]
    pub const fn set_inv(&mut self, val: super::vals::Pcr9Inv) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> super::vals::Pcr9Lk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pcr9Lk::from_bits(val as u8)
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: super::vals::Pcr9Lk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pcr9 {
    #[inline(always)]
    fn default() -> Pcr9 {
        Pcr9(0)
    }
}
impl core::fmt::Debug for Pcr9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr9")
            .field("ps", &self.ps())
            .field("pe", &self.pe())
            .field("sre", &self.sre())
            .field("ode", &self.ode())
            .field("dse", &self.dse())
            .field("mux", &self.mux())
            .field("ibe", &self.ibe())
            .field("inv", &self.inv())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcr9 {{ ps: {:?}, pe: {:?}, sre: {:?}, ode: {:?}, dse: {:?}, mux: {:?}, ibe: {:?}, inv: {:?}, lk: {:?} }}",
            self.ps(),
            self.pe(),
            self.sre(),
            self.ode(),
            self.dse(),
            self.mux(),
            self.ibe(),
            self.inv(),
            self.lk()
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
