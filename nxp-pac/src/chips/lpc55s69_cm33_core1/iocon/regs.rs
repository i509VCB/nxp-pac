#[doc = "Digital I/O control for port 0 pins PIO0_13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio(pub u32);
impl Pio {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> super::vals::PioFunc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PioFunc::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_func(&mut self, val: super::vals::PioFunc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::PioMode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PioMode::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::PioMode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn slew(&self) -> super::vals::PioSlew {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PioSlew::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_slew(&mut self, val: super::vals::PioSlew) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn invert(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_invert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn digimode(&self) -> super::vals::PioDigimode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PioDigimode::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_digimode(&mut self, val: super::vals::PioDigimode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn od(&self) -> super::vals::PioOd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PioOd::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_od(&mut self, val: super::vals::PioOd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn asw(&self) -> super::vals::PioAsw {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PioAsw::from_bits(val as u8)
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_asw(&mut self, val: super::vals::PioAsw) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ssel(&self) -> super::vals::PioSsel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PioSsel::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_ssel(&mut self, val: super::vals::PioSsel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn filteroff(&self) -> super::vals::PioFilteroff {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PioFilteroff::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_filteroff(&mut self, val: super::vals::PioFilteroff) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecs(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ecs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn egp(&self) -> super::vals::PioEgp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PioEgp::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_egp(&mut self, val: super::vals::PioEgp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cfilter(&self) -> super::vals::PioI2cfilter {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PioI2cfilter::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_i2cfilter(&mut self, val: super::vals::PioI2cfilter) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Pio {
    #[inline(always)]
    fn default() -> Pio {
        Pio(0)
    }
}
impl core::fmt::Debug for Pio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pio")
            .field("func", &self.func())
            .field("mode", &self.mode())
            .field("slew", &self.slew())
            .field("invert", &self.invert())
            .field("digimode", &self.digimode())
            .field("od", &self.od())
            .field("asw", &self.asw())
            .field("ssel", &self.ssel())
            .field("filteroff", &self.filteroff())
            .field("ecs", &self.ecs())
            .field("egp", &self.egp())
            .field("i2cfilter", &self.i2cfilter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pio {{ func: {:?}, mode: {:?}, slew: {:?}, invert: {=bool:?}, digimode: {:?}, od: {:?}, asw: {:?}, ssel: {:?}, filteroff: {:?}, ecs: {=bool:?}, egp: {:?}, i2cfilter: {:?} }}",
            self.func(),
            self.mode(),
            self.slew(),
            self.invert(),
            self.digimode(),
            self.od(),
            self.asw(),
            self.ssel(),
            self.filteroff(),
            self.ecs(),
            self.egp(),
            self.i2cfilter()
        )
    }
}
