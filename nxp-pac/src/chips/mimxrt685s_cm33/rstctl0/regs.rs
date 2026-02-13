#[doc = "peripheral reset control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0(pub u32);
impl Prstctl0 {
    #[doc = "HIFI DSP reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn hifi_dsp(&self) -> super::vals::Prstctl0HifiDsp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl0HifiDsp::from_bits(val as u8)
    }
    #[doc = "HIFI DSP reset control"]
    #[inline(always)]
    pub const fn set_hifi_dsp(&mut self, val: super::vals::Prstctl0HifiDsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "powerquad reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> super::vals::Prstctl0Powerquad {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Prstctl0Powerquad::from_bits(val as u8)
    }
    #[doc = "powerquad reset control"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: super::vals::Prstctl0Powerquad) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Prstctl0Casper {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Prstctl0Casper::from_bits(val as u8)
    }
    #[doc = "CAPSER reset control"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Prstctl0Casper) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Prstctl0Hashcrypt {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Prstctl0Hashcrypt::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT reset control"]
    #[inline(always)]
    pub const fn set_hashcrypt(&mut self, val: super::vals::Prstctl0Hashcrypt) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> super::vals::Prstctl0Puf {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Prstctl0Puf::from_bits(val as u8)
    }
    #[doc = "PUF reset control"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: super::vals::Prstctl0Puf) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> super::vals::Prstctl0Rng {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Prstctl0Rng::from_bits(val as u8)
    }
    #[doc = "RNG reset control"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: super::vals::Prstctl0Rng) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad(&self) -> super::vals::Prstctl0FlexspiOtfad {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl0FlexspiOtfad::from_bits(val as u8)
    }
    #[doc = "FLEXSPI reset control"]
    #[inline(always)]
    pub const fn set_flexspi_otfad(&mut self, val: super::vals::Prstctl0FlexspiOtfad) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB PHY reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy(&self) -> super::vals::Prstctl0UsbhsPhy {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Prstctl0UsbhsPhy::from_bits(val as u8)
    }
    #[doc = "USB PHY reset control"]
    #[inline(always)]
    pub const fn set_usbhs_phy(&mut self, val: super::vals::Prstctl0UsbhsPhy) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device(&self) -> super::vals::Prstctl0UsbhsDevice {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Prstctl0UsbhsDevice::from_bits(val as u8)
    }
    #[doc = "USB DEVICE reset control"]
    #[inline(always)]
    pub const fn set_usbhs_device(&mut self, val: super::vals::Prstctl0UsbhsDevice) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host(&self) -> super::vals::Prstctl0UsbhsHost {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Prstctl0UsbhsHost::from_bits(val as u8)
    }
    #[doc = "USB HOST reset control"]
    #[inline(always)]
    pub const fn set_usbhs_host(&mut self, val: super::vals::Prstctl0UsbhsHost) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram(&self) -> super::vals::Prstctl0UsbhsSram {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Prstctl0UsbhsSram::from_bits(val as u8)
    }
    #[doc = "USBHS RAM reset control"]
    #[inline(always)]
    pub const fn set_usbhs_sram(&mut self, val: super::vals::Prstctl0UsbhsSram) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> super::vals::Prstctl0Sct {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl0Sct::from_bits(val as u8)
    }
    #[doc = "SCT reset control"]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: super::vals::Prstctl0Sct) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl0 {
    #[inline(always)]
    fn default() -> Prstctl0 {
        Prstctl0(0)
    }
}
impl core::fmt::Debug for Prstctl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl0")
            .field("hifi_dsp", &self.hifi_dsp())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("hashcrypt", &self.hashcrypt())
            .field("puf", &self.puf())
            .field("rng", &self.rng())
            .field("flexspi_otfad", &self.flexspi_otfad())
            .field("usbhs_phy", &self.usbhs_phy())
            .field("usbhs_device", &self.usbhs_device())
            .field("usbhs_host", &self.usbhs_host())
            .field("usbhs_sram", &self.usbhs_sram())
            .field("sct", &self.sct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl0 {{ hifi_dsp: {:?}, powerquad: {:?}, casper: {:?}, hashcrypt: {:?}, puf: {:?}, rng: {:?}, flexspi_otfad: {:?}, usbhs_phy: {:?}, usbhs_device: {:?}, usbhs_host: {:?}, usbhs_sram: {:?}, sct: {:?} }}",
            self.hifi_dsp(),
            self.powerquad(),
            self.casper(),
            self.hashcrypt(),
            self.puf(),
            self.rng(),
            self.flexspi_otfad(),
            self.usbhs_phy(),
            self.usbhs_device(),
            self.usbhs_host(),
            self.usbhs_sram(),
            self.sct()
        )
    }
}
#[doc = "peripheral reset clear register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0Clr(pub u32);
impl Prstctl0Clr {
    #[doc = "HIFI DSP reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hifi_dsp(&self) -> super::vals::Prstctl0ClrHifiDsp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl0ClrHifiDsp::from_bits(val as u8)
    }
    #[doc = "HIFI DSP reset clear"]
    #[inline(always)]
    pub const fn set_hifi_dsp(&mut self, val: super::vals::Prstctl0ClrHifiDsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "powerquad reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> super::vals::Prstctl0ClrPowerquad {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Prstctl0ClrPowerquad::from_bits(val as u8)
    }
    #[doc = "powerquad reset clear"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: super::vals::Prstctl0ClrPowerquad) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Prstctl0ClrCasper {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Prstctl0ClrCasper::from_bits(val as u8)
    }
    #[doc = "CAPSER reset clear"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Prstctl0ClrCasper) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Prstctl0ClrHashcrypt {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Prstctl0ClrHashcrypt::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT reset clear"]
    #[inline(always)]
    pub const fn set_hashcrypt(&mut self, val: super::vals::Prstctl0ClrHashcrypt) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> super::vals::Prstctl0ClrPuf {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Prstctl0ClrPuf::from_bits(val as u8)
    }
    #[doc = "PUF reset clear"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: super::vals::Prstctl0ClrPuf) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> super::vals::Prstctl0ClrRng {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Prstctl0ClrRng::from_bits(val as u8)
    }
    #[doc = "RNG reset clear"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: super::vals::Prstctl0ClrRng) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad(&self) -> super::vals::Prstctl0ClrFlexspiOtfad {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl0ClrFlexspiOtfad::from_bits(val as u8)
    }
    #[doc = "FLEXSPI reset clear"]
    #[inline(always)]
    pub const fn set_flexspi_otfad(&mut self, val: super::vals::Prstctl0ClrFlexspiOtfad) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB PHY reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy(&self) -> super::vals::Prstctl0ClrUsbhsPhy {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Prstctl0ClrUsbhsPhy::from_bits(val as u8)
    }
    #[doc = "USB PHY reset clear"]
    #[inline(always)]
    pub const fn set_usbhs_phy(&mut self, val: super::vals::Prstctl0ClrUsbhsPhy) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device(&self) -> super::vals::Prstctl0ClrUsbhsDevice {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Prstctl0ClrUsbhsDevice::from_bits(val as u8)
    }
    #[doc = "USB DEVICE reset clear"]
    #[inline(always)]
    pub const fn set_usbhs_device(&mut self, val: super::vals::Prstctl0ClrUsbhsDevice) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host(&self) -> super::vals::Prstctl0ClrUsbhsHost {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Prstctl0ClrUsbhsHost::from_bits(val as u8)
    }
    #[doc = "USB HOST reset clear"]
    #[inline(always)]
    pub const fn set_usbhs_host(&mut self, val: super::vals::Prstctl0ClrUsbhsHost) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram(&self) -> super::vals::Prstctl0ClrUsbhsSram {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Prstctl0ClrUsbhsSram::from_bits(val as u8)
    }
    #[doc = "USBHS RAM reset clear"]
    #[inline(always)]
    pub const fn set_usbhs_sram(&mut self, val: super::vals::Prstctl0ClrUsbhsSram) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> super::vals::Prstctl0ClrSct {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl0ClrSct::from_bits(val as u8)
    }
    #[doc = "SCT reset clear"]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: super::vals::Prstctl0ClrSct) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl0Clr {
    #[inline(always)]
    fn default() -> Prstctl0Clr {
        Prstctl0Clr(0)
    }
}
impl core::fmt::Debug for Prstctl0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl0Clr")
            .field("hifi_dsp", &self.hifi_dsp())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("hashcrypt", &self.hashcrypt())
            .field("puf", &self.puf())
            .field("rng", &self.rng())
            .field("flexspi_otfad", &self.flexspi_otfad())
            .field("usbhs_phy", &self.usbhs_phy())
            .field("usbhs_device", &self.usbhs_device())
            .field("usbhs_host", &self.usbhs_host())
            .field("usbhs_sram", &self.usbhs_sram())
            .field("sct", &self.sct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl0Clr {{ hifi_dsp: {:?}, powerquad: {:?}, casper: {:?}, hashcrypt: {:?}, puf: {:?}, rng: {:?}, flexspi_otfad: {:?}, usbhs_phy: {:?}, usbhs_device: {:?}, usbhs_host: {:?}, usbhs_sram: {:?}, sct: {:?} }}",
            self.hifi_dsp(),
            self.powerquad(),
            self.casper(),
            self.hashcrypt(),
            self.puf(),
            self.rng(),
            self.flexspi_otfad(),
            self.usbhs_phy(),
            self.usbhs_device(),
            self.usbhs_host(),
            self.usbhs_sram(),
            self.sct()
        )
    }
}
#[doc = "peripheral reset set register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0Set(pub u32);
impl Prstctl0Set {
    #[doc = "HIFI DSP reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn hifi_dsp(&self) -> super::vals::Prstctl0SetHifiDsp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl0SetHifiDsp::from_bits(val as u8)
    }
    #[doc = "HIFI DSP reset set"]
    #[inline(always)]
    pub const fn set_hifi_dsp(&mut self, val: super::vals::Prstctl0SetHifiDsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "powerquad reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> super::vals::Prstctl0SetPowerquad {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Prstctl0SetPowerquad::from_bits(val as u8)
    }
    #[doc = "powerquad reset set"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: super::vals::Prstctl0SetPowerquad) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Prstctl0SetCasper {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Prstctl0SetCasper::from_bits(val as u8)
    }
    #[doc = "CAPSER reset set"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Prstctl0SetCasper) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Prstctl0SetHashcrypt {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Prstctl0SetHashcrypt::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT reset set"]
    #[inline(always)]
    pub const fn set_hashcrypt(&mut self, val: super::vals::Prstctl0SetHashcrypt) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> super::vals::Prstctl0SetPuf {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Prstctl0SetPuf::from_bits(val as u8)
    }
    #[doc = "PUF reset set"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: super::vals::Prstctl0SetPuf) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> super::vals::Prstctl0SetRng {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Prstctl0SetRng::from_bits(val as u8)
    }
    #[doc = "RNG reset set"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: super::vals::Prstctl0SetRng) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_otfad(&self) -> super::vals::Prstctl0SetFlexspiOtfad {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl0SetFlexspiOtfad::from_bits(val as u8)
    }
    #[doc = "FLEXSPI reset set"]
    #[inline(always)]
    pub const fn set_flexspi_otfad(&mut self, val: super::vals::Prstctl0SetFlexspiOtfad) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB PHY reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_phy(&self) -> super::vals::Prstctl0SetUsbhsPhy {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Prstctl0SetUsbhsPhy::from_bits(val as u8)
    }
    #[doc = "USB PHY reset set"]
    #[inline(always)]
    pub const fn set_usbhs_phy(&mut self, val: super::vals::Prstctl0SetUsbhsPhy) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_device(&self) -> super::vals::Prstctl0SetUsbhsDevice {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Prstctl0SetUsbhsDevice::from_bits(val as u8)
    }
    #[doc = "USB DEVICE reset set"]
    #[inline(always)]
    pub const fn set_usbhs_device(&mut self, val: super::vals::Prstctl0SetUsbhsDevice) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_host(&self) -> super::vals::Prstctl0SetUsbhsHost {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Prstctl0SetUsbhsHost::from_bits(val as u8)
    }
    #[doc = "USB HOST reset set"]
    #[inline(always)]
    pub const fn set_usbhs_host(&mut self, val: super::vals::Prstctl0SetUsbhsHost) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram(&self) -> super::vals::Prstctl0SetUsbhsSram {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Prstctl0SetUsbhsSram::from_bits(val as u8)
    }
    #[doc = "USBHS RAM reset set"]
    #[inline(always)]
    pub const fn set_usbhs_sram(&mut self, val: super::vals::Prstctl0SetUsbhsSram) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> super::vals::Prstctl0SetSct {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl0SetSct::from_bits(val as u8)
    }
    #[doc = "SCT reset set"]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: super::vals::Prstctl0SetSct) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl0Set {
    #[inline(always)]
    fn default() -> Prstctl0Set {
        Prstctl0Set(0)
    }
}
impl core::fmt::Debug for Prstctl0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl0Set")
            .field("hifi_dsp", &self.hifi_dsp())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("hashcrypt", &self.hashcrypt())
            .field("puf", &self.puf())
            .field("rng", &self.rng())
            .field("flexspi_otfad", &self.flexspi_otfad())
            .field("usbhs_phy", &self.usbhs_phy())
            .field("usbhs_device", &self.usbhs_device())
            .field("usbhs_host", &self.usbhs_host())
            .field("usbhs_sram", &self.usbhs_sram())
            .field("sct", &self.sct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl0Set {{ hifi_dsp: {:?}, powerquad: {:?}, casper: {:?}, hashcrypt: {:?}, puf: {:?}, rng: {:?}, flexspi_otfad: {:?}, usbhs_phy: {:?}, usbhs_device: {:?}, usbhs_host: {:?}, usbhs_sram: {:?}, sct: {:?} }}",
            self.hifi_dsp(),
            self.powerquad(),
            self.casper(),
            self.hashcrypt(),
            self.puf(),
            self.rng(),
            self.flexspi_otfad(),
            self.usbhs_phy(),
            self.usbhs_device(),
            self.usbhs_host(),
            self.usbhs_sram(),
            self.sct()
        )
    }
}
#[doc = "peripheral reset control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1(pub u32);
impl Prstctl1 {
    #[doc = "SDIO0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> super::vals::Prstctl1Sdio0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Prstctl1Sdio0::from_bits(val as u8)
    }
    #[doc = "SDIO0 reset control"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: super::vals::Prstctl1Sdio0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> super::vals::Prstctl1Sdio1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Prstctl1Sdio1::from_bits(val as u8)
    }
    #[doc = "SDIO1 reset control"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: super::vals::Prstctl1Sdio1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0(&self) -> super::vals::Prstctl1Acmp0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Prstctl1Acmp0::from_bits(val as u8)
    }
    #[doc = "Analog comparator reset control"]
    #[inline(always)]
    pub const fn set_acmp0(&mut self, val: super::vals::Prstctl1Acmp0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Prstctl1Adc0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl1Adc0::from_bits(val as u8)
    }
    #[doc = "ADC reset control"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Prstctl1Adc0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0(&self) -> super::vals::Prstctl1Shsgpio0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl1Shsgpio0::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 reset control"]
    #[inline(always)]
    pub const fn set_shsgpio0(&mut self, val: super::vals::Prstctl1Shsgpio0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl1 {
    #[inline(always)]
    fn default() -> Prstctl1 {
        Prstctl1(0)
    }
}
impl core::fmt::Debug for Prstctl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl1")
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("acmp0", &self.acmp0())
            .field("adc0", &self.adc0())
            .field("shsgpio0", &self.shsgpio0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl1 {{ sdio0: {:?}, sdio1: {:?}, acmp0: {:?}, adc0: {:?}, shsgpio0: {:?} }}",
            self.sdio0(),
            self.sdio1(),
            self.acmp0(),
            self.adc0(),
            self.shsgpio0()
        )
    }
}
#[doc = "peripheral reset clear register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1Clr(pub u32);
impl Prstctl1Clr {
    #[doc = "SDIO0 reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> super::vals::Prstctl1ClrSdio0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Prstctl1ClrSdio0::from_bits(val as u8)
    }
    #[doc = "SDIO0 reset clear"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: super::vals::Prstctl1ClrSdio0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> super::vals::Prstctl1ClrSdio1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Prstctl1ClrSdio1::from_bits(val as u8)
    }
    #[doc = "SDIO1 reset clear"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: super::vals::Prstctl1ClrSdio1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0(&self) -> super::vals::Prstctl1ClrAcmp0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Prstctl1ClrAcmp0::from_bits(val as u8)
    }
    #[doc = "Analog comparator reset clear"]
    #[inline(always)]
    pub const fn set_acmp0(&mut self, val: super::vals::Prstctl1ClrAcmp0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Prstctl1ClrAdc0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl1ClrAdc0::from_bits(val as u8)
    }
    #[doc = "ADC reset clear"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Prstctl1ClrAdc0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0(&self) -> super::vals::Prstctl1ClrShsgpio0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl1ClrShsgpio0::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 reset clear"]
    #[inline(always)]
    pub const fn set_shsgpio0(&mut self, val: super::vals::Prstctl1ClrShsgpio0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl1Clr {
    #[inline(always)]
    fn default() -> Prstctl1Clr {
        Prstctl1Clr(0)
    }
}
impl core::fmt::Debug for Prstctl1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl1Clr")
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("acmp0", &self.acmp0())
            .field("adc0", &self.adc0())
            .field("shsgpio0", &self.shsgpio0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl1Clr {{ sdio0: {:?}, sdio1: {:?}, acmp0: {:?}, adc0: {:?}, shsgpio0: {:?} }}",
            self.sdio0(),
            self.sdio1(),
            self.acmp0(),
            self.adc0(),
            self.shsgpio0()
        )
    }
}
#[doc = "peripheral reset set register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1Set(pub u32);
impl Prstctl1Set {
    #[doc = "SDIO0 reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> super::vals::Prstctl1SetSdio0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Prstctl1SetSdio0::from_bits(val as u8)
    }
    #[doc = "SDIO0 reset set"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: super::vals::Prstctl1SetSdio0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> super::vals::Prstctl1SetSdio1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Prstctl1SetSdio1::from_bits(val as u8)
    }
    #[doc = "SDIO1 reset set"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: super::vals::Prstctl1SetSdio1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp0(&self) -> super::vals::Prstctl1SetAcmp0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Prstctl1SetAcmp0::from_bits(val as u8)
    }
    #[doc = "Analog comparator reset set"]
    #[inline(always)]
    pub const fn set_acmp0(&mut self, val: super::vals::Prstctl1SetAcmp0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Prstctl1SetAdc0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Prstctl1SetAdc0::from_bits(val as u8)
    }
    #[doc = "ADC reset set"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Prstctl1SetAdc0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio0(&self) -> super::vals::Prstctl1SetShsgpio0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Prstctl1SetShsgpio0::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 reset set"]
    #[inline(always)]
    pub const fn set_shsgpio0(&mut self, val: super::vals::Prstctl1SetShsgpio0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Prstctl1Set {
    #[inline(always)]
    fn default() -> Prstctl1Set {
        Prstctl1Set(0)
    }
}
impl core::fmt::Debug for Prstctl1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl1Set")
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("acmp0", &self.acmp0())
            .field("adc0", &self.adc0())
            .field("shsgpio0", &self.shsgpio0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl1Set {{ sdio0: {:?}, sdio1: {:?}, acmp0: {:?}, adc0: {:?}, shsgpio0: {:?} }}",
            self.sdio0(),
            self.sdio1(),
            self.acmp0(),
            self.adc0(),
            self.shsgpio0()
        )
    }
}
#[doc = "peripheral reset control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2(pub u32);
impl Prstctl2 {
    #[doc = "utick reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> super::vals::Prstctl2Utick0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Prstctl2Utick0::from_bits(val as u8)
    }
    #[doc = "utick reset control"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: super::vals::Prstctl2Utick0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt reset control"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> super::vals::Prstctl2Wwdt0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl2Wwdt0::from_bits(val as u8)
    }
    #[doc = "wdt reset control"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: super::vals::Prstctl2Wwdt0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Prstctl2 {
    #[inline(always)]
    fn default() -> Prstctl2 {
        Prstctl2(0)
    }
}
impl core::fmt::Debug for Prstctl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl2")
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl2 {{ utick0: {:?}, wwdt0: {:?} }}",
            self.utick0(),
            self.wwdt0()
        )
    }
}
#[doc = "peripheral reset clear register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2Clr(pub u32);
impl Prstctl2Clr {
    #[doc = "utick reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> super::vals::Prstctl2ClrUtick0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Prstctl2ClrUtick0::from_bits(val as u8)
    }
    #[doc = "utick reset clear"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: super::vals::Prstctl2ClrUtick0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt reset clear"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> super::vals::Prstctl2ClrWwdt0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl2ClrWwdt0::from_bits(val as u8)
    }
    #[doc = "wdt reset clear"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: super::vals::Prstctl2ClrWwdt0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Prstctl2Clr {
    #[inline(always)]
    fn default() -> Prstctl2Clr {
        Prstctl2Clr(0)
    }
}
impl core::fmt::Debug for Prstctl2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl2Clr")
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl2Clr {{ utick0: {:?}, wwdt0: {:?} }}",
            self.utick0(),
            self.wwdt0()
        )
    }
}
#[doc = "peripheral reset set register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2Set(pub u32);
impl Prstctl2Set {
    #[doc = "utick reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> super::vals::Prstctl2SetUtick0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Prstctl2SetUtick0::from_bits(val as u8)
    }
    #[doc = "utick reset set"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: super::vals::Prstctl2SetUtick0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt reset set"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> super::vals::Prstctl2SetWwdt0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Prstctl2SetWwdt0::from_bits(val as u8)
    }
    #[doc = "wdt reset set"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: super::vals::Prstctl2SetWwdt0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Prstctl2Set {
    #[inline(always)]
    fn default() -> Prstctl2Set {
        Prstctl2Set(0)
    }
}
impl core::fmt::Debug for Prstctl2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prstctl2Set")
            .field("utick0", &self.utick0())
            .field("wwdt0", &self.wwdt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prstctl2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Prstctl2Set {{ utick0: {:?}, wwdt0: {:?} }}",
            self.utick0(),
            self.wwdt0()
        )
    }
}
#[doc = "system reset status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysrststat(pub u32);
impl Sysrststat {
    #[doc = "VDD POR Event Detected:"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd_por(&self) -> super::vals::VddPor {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VddPor::from_bits(val as u8)
    }
    #[doc = "VDD POR Event Detected:"]
    #[inline(always)]
    pub const fn set_vdd_por(&mut self, val: super::vals::VddPor) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PAD RESET Event Detected:"]
    #[must_use]
    #[inline(always)]
    pub const fn pad_reset(&self) -> super::vals::PadReset {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PadReset::from_bits(val as u8)
    }
    #[doc = "PAD RESET Event Detected:"]
    #[inline(always)]
    pub const fn set_pad_reset(&mut self, val: super::vals::PadReset) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ARM RESET Event Detected:"]
    #[must_use]
    #[inline(always)]
    pub const fn arm_apd_reset(&self) -> super::vals::ArmApdReset {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ArmApdReset::from_bits(val as u8)
    }
    #[doc = "ARM RESET Event Detected:"]
    #[inline(always)]
    pub const fn set_arm_apd_reset(&mut self, val: super::vals::ArmApdReset) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "WDT0 RESET Event Detected:"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt0_reset(&self) -> super::vals::Wdt0Reset {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wdt0Reset::from_bits(val as u8)
    }
    #[doc = "WDT0 RESET Event Detected:"]
    #[inline(always)]
    pub const fn set_wdt0_reset(&mut self, val: super::vals::Wdt0Reset) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "WDT1 RESET Event Detected:"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt1_reset(&self) -> super::vals::Wdt1Reset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wdt1Reset::from_bits(val as u8)
    }
    #[doc = "WDT1 RESET Event Detected:"]
    #[inline(always)]
    pub const fn set_wdt1_reset(&mut self, val: super::vals::Wdt1Reset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Sysrststat {
    #[inline(always)]
    fn default() -> Sysrststat {
        Sysrststat(0)
    }
}
impl core::fmt::Debug for Sysrststat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sysrststat")
            .field("vdd_por", &self.vdd_por())
            .field("pad_reset", &self.pad_reset())
            .field("arm_apd_reset", &self.arm_apd_reset())
            .field("wdt0_reset", &self.wdt0_reset())
            .field("wdt1_reset", &self.wdt1_reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sysrststat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sysrststat {{ vdd_por: {:?}, pad_reset: {:?}, arm_apd_reset: {:?}, wdt0_reset: {:?}, wdt1_reset: {:?} }}",
            self.vdd_por(),
            self.pad_reset(),
            self.arm_apd_reset(),
            self.wdt0_reset(),
            self.wdt1_reset()
        )
    }
}
