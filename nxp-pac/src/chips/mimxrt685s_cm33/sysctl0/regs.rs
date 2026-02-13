#[doc = "AES key source selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AeskeySrcsel(pub u32);
impl AeskeySrcsel {
    #[doc = "AES Key Source Select:"]
    #[must_use]
    #[inline(always)]
    pub const fn aeskey_srcsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "AES Key Source Select:"]
    #[inline(always)]
    pub const fn set_aeskey_srcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for AeskeySrcsel {
    #[inline(always)]
    fn default() -> AeskeySrcsel {
        AeskeySrcsel(0)
    }
}
impl core::fmt::Debug for AeskeySrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AeskeySrcsel")
            .field("aeskey_srcsel", &self.aeskey_srcsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AeskeySrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AeskeySrcsel {{ aeskey_srcsel: {=u8:?} }}",
            self.aeskey_srcsel()
        )
    }
}
#[doc = "AHB Flexspi access control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbFlexspiAccessDisable(pub u32);
impl AhbFlexspiAccessDisable {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb_flexspi_access_disable(&self) -> super::vals::AhbFlexspiAccessDisable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AhbFlexspiAccessDisable::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ahb_flexspi_access_disable(
        &mut self,
        val: super::vals::AhbFlexspiAccessDisable,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for AhbFlexspiAccessDisable {
    #[inline(always)]
    fn default() -> AhbFlexspiAccessDisable {
        AhbFlexspiAccessDisable(0)
    }
}
impl core::fmt::Debug for AhbFlexspiAccessDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbFlexspiAccessDisable")
            .field(
                "ahb_flexspi_access_disable",
                &self.ahb_flexspi_access_disable(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbFlexspiAccessDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbFlexspiAccessDisable {{ ahb_flexspi_access_disable: {:?} }}",
            self.ahb_flexspi_access_disable()
        )
    }
}
#[doc = "AHB SRAM access disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AhbSramAccessDisable(pub u32);
impl AhbSramAccessDisable {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram00_if(&self) -> super::vals::AhbSramAccessDisableSram00If {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AhbSramAccessDisableSram00If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram00_if(&mut self, val: super::vals::AhbSramAccessDisableSram00If) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram01_if(&self) -> super::vals::AhbSramAccessDisableSram01If {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AhbSramAccessDisableSram01If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram01_if(&mut self, val: super::vals::AhbSramAccessDisableSram01If) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram02_if(&self) -> super::vals::AhbSramAccessDisableSram02If {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AhbSramAccessDisableSram02If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram02_if(&mut self, val: super::vals::AhbSramAccessDisableSram02If) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram03_if(&self) -> super::vals::AhbSramAccessDisableSram03If {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::AhbSramAccessDisableSram03If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram03_if(&mut self, val: super::vals::AhbSramAccessDisableSram03If) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram04_if(&self) -> super::vals::AhbSramAccessDisableSram04If {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AhbSramAccessDisableSram04If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram04_if(&mut self, val: super::vals::AhbSramAccessDisableSram04If) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram05_if(&self) -> super::vals::AhbSramAccessDisableSram05If {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AhbSramAccessDisableSram05If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram05_if(&mut self, val: super::vals::AhbSramAccessDisableSram05If) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram06_if(&self) -> super::vals::AhbSramAccessDisableSram06If {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::AhbSramAccessDisableSram06If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram06_if(&mut self, val: super::vals::AhbSramAccessDisableSram06If) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram07_if(&self) -> super::vals::AhbSramAccessDisableSram07If {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::AhbSramAccessDisableSram07If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram07_if(&mut self, val: super::vals::AhbSramAccessDisableSram07If) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram08_if(&self) -> super::vals::AhbSramAccessDisableSram08If {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::AhbSramAccessDisableSram08If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram08_if(&mut self, val: super::vals::AhbSramAccessDisableSram08If) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram09_if(&self) -> super::vals::AhbSramAccessDisableSram09If {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::AhbSramAccessDisableSram09If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram09_if(&mut self, val: super::vals::AhbSramAccessDisableSram09If) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram10_if(&self) -> super::vals::AhbSramAccessDisableSram10If {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::AhbSramAccessDisableSram10If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram10_if(&mut self, val: super::vals::AhbSramAccessDisableSram10If) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram11_if(&self) -> super::vals::AhbSramAccessDisableSram11If {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::AhbSramAccessDisableSram11If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram11_if(&mut self, val: super::vals::AhbSramAccessDisableSram11If) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram12_if(&self) -> super::vals::AhbSramAccessDisableSram12If {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::AhbSramAccessDisableSram12If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram12_if(&mut self, val: super::vals::AhbSramAccessDisableSram12If) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram13_if(&self) -> super::vals::AhbSramAccessDisableSram13If {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::AhbSramAccessDisableSram13If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram13_if(&mut self, val: super::vals::AhbSramAccessDisableSram13If) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram14_if(&self) -> super::vals::AhbSramAccessDisableSram14If {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::AhbSramAccessDisableSram14If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram14_if(&mut self, val: super::vals::AhbSramAccessDisableSram14If) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram15_if(&self) -> super::vals::AhbSramAccessDisableSram15If {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AhbSramAccessDisableSram15If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram15_if(&mut self, val: super::vals::AhbSramAccessDisableSram15If) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram16_if(&self) -> super::vals::AhbSramAccessDisableSram16If {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::AhbSramAccessDisableSram16If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram16_if(&mut self, val: super::vals::AhbSramAccessDisableSram16If) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram17_if(&self) -> super::vals::AhbSramAccessDisableSram17If {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::AhbSramAccessDisableSram17If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram17_if(&mut self, val: super::vals::AhbSramAccessDisableSram17If) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram18_if(&self) -> super::vals::AhbSramAccessDisableSram18If {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::AhbSramAccessDisableSram18If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram18_if(&mut self, val: super::vals::AhbSramAccessDisableSram18If) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram19_if(&self) -> super::vals::AhbSramAccessDisableSram19If {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::AhbSramAccessDisableSram19If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram19_if(&mut self, val: super::vals::AhbSramAccessDisableSram19If) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram20_if(&self) -> super::vals::AhbSramAccessDisableSram20If {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::AhbSramAccessDisableSram20If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram20_if(&mut self, val: super::vals::AhbSramAccessDisableSram20If) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram21_if(&self) -> super::vals::AhbSramAccessDisableSram21If {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::AhbSramAccessDisableSram21If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram21_if(&mut self, val: super::vals::AhbSramAccessDisableSram21If) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram22_if(&self) -> super::vals::AhbSramAccessDisableSram22If {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::AhbSramAccessDisableSram22If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram22_if(&mut self, val: super::vals::AhbSramAccessDisableSram22If) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram23_if(&self) -> super::vals::AhbSramAccessDisableSram23If {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::AhbSramAccessDisableSram23If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram23_if(&mut self, val: super::vals::AhbSramAccessDisableSram23If) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram24_if(&self) -> super::vals::AhbSramAccessDisableSram24If {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::AhbSramAccessDisableSram24If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram24_if(&mut self, val: super::vals::AhbSramAccessDisableSram24If) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram25_if(&self) -> super::vals::AhbSramAccessDisableSram25If {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::AhbSramAccessDisableSram25If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram25_if(&mut self, val: super::vals::AhbSramAccessDisableSram25If) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram26_if(&self) -> super::vals::AhbSramAccessDisableSram26If {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::AhbSramAccessDisableSram26If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram26_if(&mut self, val: super::vals::AhbSramAccessDisableSram26If) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram27_if(&self) -> super::vals::AhbSramAccessDisableSram27If {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::AhbSramAccessDisableSram27If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram27_if(&mut self, val: super::vals::AhbSramAccessDisableSram27If) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram28_if(&self) -> super::vals::AhbSramAccessDisableSram28If {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::AhbSramAccessDisableSram28If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram28_if(&mut self, val: super::vals::AhbSramAccessDisableSram28If) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram29_if(&self) -> super::vals::AhbSramAccessDisableSram29If {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::AhbSramAccessDisableSram29If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram29_if(&mut self, val: super::vals::AhbSramAccessDisableSram29If) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for AhbSramAccessDisable {
    #[inline(always)]
    fn default() -> AhbSramAccessDisable {
        AhbSramAccessDisable(0)
    }
}
impl core::fmt::Debug for AhbSramAccessDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AhbSramAccessDisable")
            .field("sram00_if", &self.sram00_if())
            .field("sram01_if", &self.sram01_if())
            .field("sram02_if", &self.sram02_if())
            .field("sram03_if", &self.sram03_if())
            .field("sram04_if", &self.sram04_if())
            .field("sram05_if", &self.sram05_if())
            .field("sram06_if", &self.sram06_if())
            .field("sram07_if", &self.sram07_if())
            .field("sram08_if", &self.sram08_if())
            .field("sram09_if", &self.sram09_if())
            .field("sram10_if", &self.sram10_if())
            .field("sram11_if", &self.sram11_if())
            .field("sram12_if", &self.sram12_if())
            .field("sram13_if", &self.sram13_if())
            .field("sram14_if", &self.sram14_if())
            .field("sram15_if", &self.sram15_if())
            .field("sram16_if", &self.sram16_if())
            .field("sram17_if", &self.sram17_if())
            .field("sram18_if", &self.sram18_if())
            .field("sram19_if", &self.sram19_if())
            .field("sram20_if", &self.sram20_if())
            .field("sram21_if", &self.sram21_if())
            .field("sram22_if", &self.sram22_if())
            .field("sram23_if", &self.sram23_if())
            .field("sram24_if", &self.sram24_if())
            .field("sram25_if", &self.sram25_if())
            .field("sram26_if", &self.sram26_if())
            .field("sram27_if", &self.sram27_if())
            .field("sram28_if", &self.sram28_if())
            .field("sram29_if", &self.sram29_if())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AhbSramAccessDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AhbSramAccessDisable {{ sram00_if: {:?}, sram01_if: {:?}, sram02_if: {:?}, sram03_if: {:?}, sram04_if: {:?}, sram05_if: {:?}, sram06_if: {:?}, sram07_if: {:?}, sram08_if: {:?}, sram09_if: {:?}, sram10_if: {:?}, sram11_if: {:?}, sram12_if: {:?}, sram13_if: {:?}, sram14_if: {:?}, sram15_if: {:?}, sram16_if: {:?}, sram17_if: {:?}, sram18_if: {:?}, sram19_if: {:?}, sram20_if: {:?}, sram21_if: {:?}, sram22_if: {:?}, sram23_if: {:?}, sram24_if: {:?}, sram25_if: {:?}, sram26_if: {:?}, sram27_if: {:?}, sram28_if: {:?}, sram29_if: {:?} }}",
            self.sram00_if(),
            self.sram01_if(),
            self.sram02_if(),
            self.sram03_if(),
            self.sram04_if(),
            self.sram05_if(),
            self.sram06_if(),
            self.sram07_if(),
            self.sram08_if(),
            self.sram09_if(),
            self.sram10_if(),
            self.sram11_if(),
            self.sram12_if(),
            self.sram13_if(),
            self.sram14_if(),
            self.sram15_if(),
            self.sram16_if(),
            self.sram17_if(),
            self.sram18_if(),
            self.sram19_if(),
            self.sram20_if(),
            self.sram21_if(),
            self.sram22_if(),
            self.sram23_if(),
            self.sram24_if(),
            self.sram25_if(),
            self.sram26_if(),
            self.sram27_if(),
            self.sram28_if(),
            self.sram29_if()
        )
    }
}
#[doc = "AHB matrix priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatrixprior(pub u32);
impl Ahbmatrixprior {
    #[doc = "Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Master 0 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3. (0 High)"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Master 1 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Master 2 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Master 3 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Master 4 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Master 5 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Master 6 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Master 7 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[must_use]
    #[inline(always)]
    pub const fn m8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Master 8 Priority. . . 0: 0, 1: 1, 2: 2, 3: 3."]
    #[inline(always)]
    pub const fn set_m8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Ahbmatrixprior {
    #[inline(always)]
    fn default() -> Ahbmatrixprior {
        Ahbmatrixprior(0)
    }
}
impl core::fmt::Debug for Ahbmatrixprior {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatrixprior")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .field("m6", &self.m6())
            .field("m7", &self.m7())
            .field("m8", &self.m8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatrixprior {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatrixprior {{ m0: {=u8:?}, m1: {=u8:?}, m2: {=u8:?}, m3: {=u8:?}, m4: {=u8:?}, m5: {=u8:?}, m6: {=u8:?}, m7: {=u8:?}, m8: {=u8:?} }}",
            self.m0(),
            self.m1(),
            self.m2(),
            self.m3(),
            self.m4(),
            self.m5(),
            self.m6(),
            self.m7(),
            self.m8()
        )
    }
}
#[doc = "auto clock gating override 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverride0(pub u32);
impl Autoclkgateoverride0 {
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb2apb0(&self) -> super::vals::Ahb2apb0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ahb2apb0::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_ahb2apb0(&mut self, val: super::vals::Ahb2apb0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb2apb1(&self) -> super::vals::Ahb2apb1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ahb2apb1::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_ahb2apb1(&mut self, val: super::vals::Ahb2apb1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_engine(&self) -> super::vals::CrcEngine {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CrcEngine::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_crc_engine(&mut self, val: super::vals::CrcEngine) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Casper {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Casper::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Casper) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0(&self) -> super::vals::Autoclkgateoverride0Dmac0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Autoclkgateoverride0Dmac0::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_dmac0(&mut self, val: super::vals::Autoclkgateoverride0Dmac0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1(&self) -> super::vals::Autoclkgateoverride0Dmac1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Autoclkgateoverride0Dmac1::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_dmac1(&mut self, val: super::vals::Autoclkgateoverride0Dmac1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Autoclkgateoverride0 {
    #[inline(always)]
    fn default() -> Autoclkgateoverride0 {
        Autoclkgateoverride0(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverride0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverride0")
            .field("ahb2apb0", &self.ahb2apb0())
            .field("ahb2apb1", &self.ahb2apb1())
            .field("crc_engine", &self.crc_engine())
            .field("casper", &self.casper())
            .field("dmac0", &self.dmac0())
            .field("dmac1", &self.dmac1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverride0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverride0 {{ ahb2apb0: {:?}, ahb2apb1: {:?}, crc_engine: {:?}, casper: {:?}, dmac0: {:?}, dmac1: {:?} }}",
            self.ahb2apb0(),
            self.ahb2apb1(),
            self.crc_engine(),
            self.casper(),
            self.dmac0(),
            self.dmac1()
        )
    }
}
#[doc = "auto clock gating override 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverride1(pub u32);
impl Autoclkgateoverride1 {
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0(&self) -> super::vals::SramIf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SramIf0::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if0(&mut self, val: super::vals::SramIf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1(&self) -> super::vals::SramIf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SramIf1::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if1(&mut self, val: super::vals::SramIf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2(&self) -> super::vals::SramIf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SramIf2::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if2(&mut self, val: super::vals::SramIf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3(&self) -> super::vals::SramIf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SramIf3::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if3(&mut self, val: super::vals::SramIf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4(&self) -> super::vals::SramIf4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SramIf4::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if4(&mut self, val: super::vals::SramIf4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5(&self) -> super::vals::SramIf5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SramIf5::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if5(&mut self, val: super::vals::SramIf5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6(&self) -> super::vals::SramIf6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SramIf6::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if6(&mut self, val: super::vals::SramIf6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7(&self) -> super::vals::SramIf7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SramIf7::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if7(&mut self, val: super::vals::SramIf7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8(&self) -> super::vals::SramIf8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SramIf8::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if8(&mut self, val: super::vals::SramIf8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9(&self) -> super::vals::SramIf9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SramIf9::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if9(&mut self, val: super::vals::SramIf9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10(&self) -> super::vals::SramIf10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SramIf10::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if10(&mut self, val: super::vals::SramIf10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11(&self) -> super::vals::SramIf11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SramIf11::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if11(&mut self, val: super::vals::SramIf11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12(&self) -> super::vals::SramIf12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SramIf12::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if12(&mut self, val: super::vals::SramIf12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13(&self) -> super::vals::SramIf13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SramIf13::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if13(&mut self, val: super::vals::SramIf13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14(&self) -> super::vals::SramIf14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SramIf14::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if14(&mut self, val: super::vals::SramIf14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15(&self) -> super::vals::SramIf15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SramIf15::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if15(&mut self, val: super::vals::SramIf15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16(&self) -> super::vals::SramIf16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::SramIf16::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if16(&mut self, val: super::vals::SramIf16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17(&self) -> super::vals::SramIf17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SramIf17::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if17(&mut self, val: super::vals::SramIf17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19(&self) -> super::vals::SramIf19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::SramIf19::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if19(&mut self, val: super::vals::SramIf19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20(&self) -> super::vals::SramIf20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::SramIf20::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if20(&mut self, val: super::vals::SramIf20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21(&self) -> super::vals::SramIf21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SramIf21::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if21(&mut self, val: super::vals::SramIf21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22(&self) -> super::vals::SramIf22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::SramIf22::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if22(&mut self, val: super::vals::SramIf22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23(&self) -> super::vals::SramIf23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::SramIf23::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if23(&mut self, val: super::vals::SramIf23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24(&self) -> super::vals::SramIf24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SramIf24::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if24(&mut self, val: super::vals::SramIf24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25(&self) -> super::vals::SramIf25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SramIf25::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if25(&mut self, val: super::vals::SramIf25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26(&self) -> super::vals::SramIf26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::SramIf26::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if26(&mut self, val: super::vals::SramIf26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27(&self) -> super::vals::SramIf27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SramIf27::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if27(&mut self, val: super::vals::SramIf27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28(&self) -> super::vals::SramIf28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::SramIf28::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if28(&mut self, val: super::vals::SramIf28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "auto clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29(&self) -> super::vals::SramIf29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SramIf29::from_bits(val as u8)
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn set_sram_if29(&mut self, val: super::vals::SramIf29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Autoclkgateoverride1 {
    #[inline(always)]
    fn default() -> Autoclkgateoverride1 {
        Autoclkgateoverride1(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverride1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverride1")
            .field("sram_if0", &self.sram_if0())
            .field("sram_if1", &self.sram_if1())
            .field("sram_if2", &self.sram_if2())
            .field("sram_if3", &self.sram_if3())
            .field("sram_if4", &self.sram_if4())
            .field("sram_if5", &self.sram_if5())
            .field("sram_if6", &self.sram_if6())
            .field("sram_if7", &self.sram_if7())
            .field("sram_if8", &self.sram_if8())
            .field("sram_if9", &self.sram_if9())
            .field("sram_if10", &self.sram_if10())
            .field("sram_if11", &self.sram_if11())
            .field("sram_if12", &self.sram_if12())
            .field("sram_if13", &self.sram_if13())
            .field("sram_if14", &self.sram_if14())
            .field("sram_if15", &self.sram_if15())
            .field("sram_if16", &self.sram_if16())
            .field("sram_if17", &self.sram_if17())
            .field("sram_if18", &self.sram_if18())
            .field("sram_if19", &self.sram_if19())
            .field("sram_if20", &self.sram_if20())
            .field("sram_if21", &self.sram_if21())
            .field("sram_if22", &self.sram_if22())
            .field("sram_if23", &self.sram_if23())
            .field("sram_if24", &self.sram_if24())
            .field("sram_if25", &self.sram_if25())
            .field("sram_if26", &self.sram_if26())
            .field("sram_if27", &self.sram_if27())
            .field("sram_if28", &self.sram_if28())
            .field("sram_if29", &self.sram_if29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverride1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverride1 {{ sram_if0: {:?}, sram_if1: {:?}, sram_if2: {:?}, sram_if3: {:?}, sram_if4: {:?}, sram_if5: {:?}, sram_if6: {:?}, sram_if7: {:?}, sram_if8: {:?}, sram_if9: {:?}, sram_if10: {:?}, sram_if11: {:?}, sram_if12: {:?}, sram_if13: {:?}, sram_if14: {:?}, sram_if15: {:?}, sram_if16: {:?}, sram_if17: {:?}, sram_if18: {=bool:?}, sram_if19: {:?}, sram_if20: {:?}, sram_if21: {:?}, sram_if22: {:?}, sram_if23: {:?}, sram_if24: {:?}, sram_if25: {:?}, sram_if26: {:?}, sram_if27: {:?}, sram_if28: {:?}, sram_if29: {:?} }}",
            self.sram_if0(),
            self.sram_if1(),
            self.sram_if2(),
            self.sram_if3(),
            self.sram_if4(),
            self.sram_if5(),
            self.sram_if6(),
            self.sram_if7(),
            self.sram_if8(),
            self.sram_if9(),
            self.sram_if10(),
            self.sram_if11(),
            self.sram_if12(),
            self.sram_if13(),
            self.sram_if14(),
            self.sram_if15(),
            self.sram_if16(),
            self.sram_if17(),
            self.sram_if18(),
            self.sram_if19(),
            self.sram_if20(),
            self.sram_if21(),
            self.sram_if22(),
            self.sram_if23(),
            self.sram_if24(),
            self.sram_if25(),
            self.sram_if26(),
            self.sram_if27(),
            self.sram_if28(),
            self.sram_if29()
        )
    }
}
#[doc = "boot state hmac register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootstatehmac(pub u32);
impl Bootstatehmac {
    #[doc = "HMAC of boot state used for attestation"]
    #[must_use]
    #[inline(always)]
    pub const fn bootstatehmac(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "HMAC of boot state used for attestation"]
    #[inline(always)]
    pub const fn set_bootstatehmac(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bootstatehmac {
    #[inline(always)]
    fn default() -> Bootstatehmac {
        Bootstatehmac(0)
    }
}
impl core::fmt::Debug for Bootstatehmac {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootstatehmac")
            .field("bootstatehmac", &self.bootstatehmac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootstatehmac {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bootstatehmac {{ bootstatehmac: {=u32:?} }}",
            self.bootstatehmac()
        )
    }
}
#[doc = "boot state seed register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootstateseed(pub u32);
impl Bootstateseed {
    #[doc = "A 256-bit random number set by boot ROM on each restart"]
    #[must_use]
    #[inline(always)]
    pub const fn bootstateseed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A 256-bit random number set by boot ROM on each restart"]
    #[inline(always)]
    pub const fn set_bootstateseed(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bootstateseed {
    #[inline(always)]
    fn default() -> Bootstateseed {
        Bootstateseed(0)
    }
}
impl core::fmt::Debug for Bootstateseed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootstateseed")
            .field("bootstateseed", &self.bootstateseed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootstateseed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bootstateseed {{ bootstateseed: {=u32:?} }}",
            self.bootstateseed()
        )
    }
}
#[doc = "Clock gate override 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkgateoverride0(pub u32);
impl Clkgateoverride0 {
    #[doc = "sdio 0 clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_0(&self) -> super::vals::Sdio0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sdio0::from_bits(val as u8)
    }
    #[doc = "sdio 0 clock override"]
    #[inline(always)]
    pub const fn set_sdio_0(&mut self, val: super::vals::Sdio0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "sdio 1 clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_1(&self) -> super::vals::Sdio1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sdio1::from_bits(val as u8)
    }
    #[doc = "sdio 1 clock override"]
    #[inline(always)]
    pub const fn set_sdio_1(&mut self, val: super::vals::Sdio1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "usbhsphy clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhsphy(&self) -> super::vals::Usbhsphy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Usbhsphy::from_bits(val as u8)
    }
    #[doc = "usbhsphy clock override"]
    #[inline(always)]
    pub const fn set_usbhsphy(&mut self, val: super::vals::Usbhsphy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "adc clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> super::vals::Adc {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Adc::from_bits(val as u8)
    }
    #[doc = "adc clock override"]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: super::vals::Adc) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "mu clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn mu(&self) -> super::vals::Clkgateoverride0Mu {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Clkgateoverride0Mu::from_bits(val as u8)
    }
    #[doc = "mu clock override"]
    #[inline(always)]
    pub const fn set_mu(&mut self, val: super::vals::Clkgateoverride0Mu) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "acomparator clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self) -> super::vals::Clkgateoverride0Acmp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Clkgateoverride0Acmp::from_bits(val as u8)
    }
    #[doc = "acomparator clock override"]
    #[inline(always)]
    pub const fn set_acmp(&mut self, val: super::vals::Clkgateoverride0Acmp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "pmc clock override"]
    #[must_use]
    #[inline(always)]
    pub const fn pmc(&self) -> super::vals::Pmc {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pmc::from_bits(val as u8)
    }
    #[doc = "pmc clock override"]
    #[inline(always)]
    pub const fn set_pmc(&mut self, val: super::vals::Pmc) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Clkgateoverride0 {
    #[inline(always)]
    fn default() -> Clkgateoverride0 {
        Clkgateoverride0(0)
    }
}
impl core::fmt::Debug for Clkgateoverride0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkgateoverride0")
            .field("sdio_0", &self.sdio_0())
            .field("sdio_1", &self.sdio_1())
            .field("usbhsphy", &self.usbhsphy())
            .field("adc", &self.adc())
            .field("mu", &self.mu())
            .field("acmp", &self.acmp())
            .field("pmc", &self.pmc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkgateoverride0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkgateoverride0 {{ sdio_0: {:?}, sdio_1: {:?}, usbhsphy: {:?}, adc: {:?}, mu: {:?}, acmp: {:?}, pmc: {:?} }}",
            self.sdio_0(),
            self.sdio_1(),
            self.usbhsphy(),
            self.adc(),
            self.mu(),
            self.acmp(),
            self.pmc()
        )
    }
}
#[doc = "Code Security for CPU0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsProtcpu0(pub u32);
impl CsProtcpu0 {
    #[doc = "Controls M33 AP Enable. Magic Value: 0x1234 5678"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_protcpu0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Controls M33 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    pub const fn set_cs_protcpu0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CsProtcpu0 {
    #[inline(always)]
    fn default() -> CsProtcpu0 {
        CsProtcpu0(0)
    }
}
impl core::fmt::Debug for CsProtcpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsProtcpu0")
            .field("cs_protcpu0", &self.cs_protcpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsProtcpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CsProtcpu0 {{ cs_protcpu0: {=u32:?} }}",
            self.cs_protcpu0()
        )
    }
}
#[doc = "Code Security for CPU1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsProtcpu1(pub u32);
impl CsProtcpu1 {
    #[doc = "Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
    #[must_use]
    #[inline(always)]
    pub const fn cs_protcpu1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Controls HIFI4 AP Enable. Magic Value: 0x1234 5678"]
    #[inline(always)]
    pub const fn set_cs_protcpu1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CsProtcpu1 {
    #[inline(always)]
    fn default() -> CsProtcpu1 {
        CsProtcpu1(0)
    }
}
impl core::fmt::Debug for CsProtcpu1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CsProtcpu1")
            .field("cs_protcpu1", &self.cs_protcpu1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CsProtcpu1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CsProtcpu1 {{ cs_protcpu1: {=u32:?} }}",
            self.cs_protcpu1()
        )
    }
}
#[doc = "Debug authorization scratch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgAuthScratch(pub u32);
impl DbgAuthScratch {
    #[doc = "Debug authorization scratch register for S/W."]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_auth_scratch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Debug authorization scratch register for S/W."]
    #[inline(always)]
    pub const fn set_dbg_auth_scratch(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DbgAuthScratch {
    #[inline(always)]
    fn default() -> DbgAuthScratch {
        DbgAuthScratch(0)
    }
}
impl core::fmt::Debug for DbgAuthScratch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgAuthScratch")
            .field("dbg_auth_scratch", &self.dbg_auth_scratch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgAuthScratch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DbgAuthScratch {{ dbg_auth_scratch: {=u32:?} }}",
            self.dbg_auth_scratch()
        )
    }
}
#[doc = "Debug features control for the CM33"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgFeatures(pub u32);
impl DbgFeatures {
    #[doc = "CM33 Debug Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::DbgFeaturesDbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DbgFeaturesDbgen::from_bits(val as u8)
    }
    #[doc = "CM33 Debug Enable Control"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::DbgFeaturesDbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CM33 NID Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> super::vals::DbgFeaturesNiden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DbgFeaturesNiden::from_bits(val as u8)
    }
    #[doc = "CM33 NID Enable Control"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: super::vals::DbgFeaturesNiden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CM33 SPID Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn spiden(&self) -> super::vals::DbgFeaturesSpiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DbgFeaturesSpiden::from_bits(val as u8)
    }
    #[doc = "CM33 SPID Enable Control"]
    #[inline(always)]
    pub const fn set_spiden(&mut self, val: super::vals::DbgFeaturesSpiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CM33 SPNIDEN Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn spniden(&self) -> super::vals::DbgFeaturesSpniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DbgFeaturesSpniden::from_bits(val as u8)
    }
    #[doc = "CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    pub const fn set_spniden(&mut self, val: super::vals::DbgFeaturesSpniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DbgFeatures {
    #[inline(always)]
    fn default() -> DbgFeatures {
        DbgFeatures(0)
    }
}
impl core::fmt::Debug for DbgFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgFeatures")
            .field("dbgen", &self.dbgen())
            .field("niden", &self.niden())
            .field("spiden", &self.spiden())
            .field("spniden", &self.spniden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DbgFeatures {{ dbgen: {:?}, niden: {:?}, spiden: {:?}, spniden: {:?} }}",
            self.dbgen(),
            self.niden(),
            self.spiden(),
            self.spniden()
        )
    }
}
#[doc = "Debug features duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgFeaturesDp(pub u32);
impl DbgFeaturesDp {
    #[doc = "CM33 Debug Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::DbgFeaturesDpDbgen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DbgFeaturesDpDbgen::from_bits(val as u8)
    }
    #[doc = "CM33 Debug Enable Control"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::DbgFeaturesDpDbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CM33 NID Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> super::vals::DbgFeaturesDpNiden {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DbgFeaturesDpNiden::from_bits(val as u8)
    }
    #[doc = "CM33 NID Enable Control"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: super::vals::DbgFeaturesDpNiden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CM33 SPID Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn spiden(&self) -> super::vals::DbgFeaturesDpSpiden {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DbgFeaturesDpSpiden::from_bits(val as u8)
    }
    #[doc = "CM33 SPID Enable Control"]
    #[inline(always)]
    pub const fn set_spiden(&mut self, val: super::vals::DbgFeaturesDpSpiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CM33 SPNIDEN Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn spniden(&self) -> super::vals::DbgFeaturesDpSpniden {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DbgFeaturesDpSpniden::from_bits(val as u8)
    }
    #[doc = "CM33 SPNIDEN Enable Control"]
    #[inline(always)]
    pub const fn set_spniden(&mut self, val: super::vals::DbgFeaturesDpSpniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DbgFeaturesDp {
    #[inline(always)]
    fn default() -> DbgFeaturesDp {
        DbgFeaturesDp(0)
    }
}
impl core::fmt::Debug for DbgFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgFeaturesDp")
            .field("dbgen", &self.dbgen())
            .field("niden", &self.niden())
            .field("spiden", &self.spiden())
            .field("spniden", &self.spniden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DbgFeaturesDp {{ dbgen: {:?}, niden: {:?}, spiden: {:?}, spniden: {:?} }}",
            self.dbgen(),
            self.niden(),
            self.spiden(),
            self.spniden()
        )
    }
}
#[doc = "Debug Write Lock registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgLocken(pub u32);
impl DbgLocken {
    #[doc = "Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_locken(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Debug Write Lock the following registers: DBG_FEATURES DBG_FEATURES_DP CS_PROTTEST CS_PROTCPU0 CS_PROTCPU1 DBG_AUTH_SCRATCH 1010: Write Enabled (Unlocked) Any other value other than 1010, Write Disabled (Locked)"]
    #[inline(always)]
    pub const fn set_dbg_locken(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for DbgLocken {
    #[inline(always)]
    fn default() -> DbgLocken {
        DbgLocken(0)
    }
}
impl core::fmt::Debug for DbgLocken {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgLocken")
            .field("dbg_locken", &self.dbg_locken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgLocken {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DbgLocken {{ dbg_locken: {=u8:?} }}", self.dbg_locken())
    }
}
#[doc = "DICE General Purpose 32-Bit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dicehwreg(pub u32);
impl Dicehwreg {
    #[doc = "DICE General Purpose 32-Bit Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dicehwreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub const fn set_dicehwreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dicehwreg {
    #[inline(always)]
    fn default() -> Dicehwreg {
        Dicehwreg(0)
    }
}
impl core::fmt::Debug for Dicehwreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dicehwreg")
            .field("dicehwreg", &self.dicehwreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dicehwreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dicehwreg {{ dicehwreg: {=u32:?} }}", self.dicehwreg())
    }
}
#[doc = "DSP Flexspi access control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DspFlexspiAccessDisable(pub u32);
impl DspFlexspiAccessDisable {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_flexspi_access_disable(&self) -> super::vals::DspFlexspiAccessDisable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DspFlexspiAccessDisable::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dsp_flexspi_access_disable(
        &mut self,
        val: super::vals::DspFlexspiAccessDisable,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for DspFlexspiAccessDisable {
    #[inline(always)]
    fn default() -> DspFlexspiAccessDisable {
        DspFlexspiAccessDisable(0)
    }
}
impl core::fmt::Debug for DspFlexspiAccessDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DspFlexspiAccessDisable")
            .field(
                "dsp_flexspi_access_disable",
                &self.dsp_flexspi_access_disable(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DspFlexspiAccessDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DspFlexspiAccessDisable {{ dsp_flexspi_access_disable: {:?} }}",
            self.dsp_flexspi_access_disable()
        )
    }
}
#[doc = "DSP SRAM access disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DspSramAccessDisable(pub u32);
impl DspSramAccessDisable {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram00_if(&self) -> super::vals::DspSramAccessDisableSram00If {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DspSramAccessDisableSram00If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram00_if(&mut self, val: super::vals::DspSramAccessDisableSram00If) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram01_if(&self) -> super::vals::DspSramAccessDisableSram01If {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DspSramAccessDisableSram01If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram01_if(&mut self, val: super::vals::DspSramAccessDisableSram01If) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram02_if(&self) -> super::vals::DspSramAccessDisableSram02If {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DspSramAccessDisableSram02If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram02_if(&mut self, val: super::vals::DspSramAccessDisableSram02If) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram03_if(&self) -> super::vals::DspSramAccessDisableSram03If {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DspSramAccessDisableSram03If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram03_if(&mut self, val: super::vals::DspSramAccessDisableSram03If) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram04_if(&self) -> super::vals::DspSramAccessDisableSram04If {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DspSramAccessDisableSram04If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram04_if(&mut self, val: super::vals::DspSramAccessDisableSram04If) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram05_if(&self) -> super::vals::DspSramAccessDisableSram05If {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DspSramAccessDisableSram05If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram05_if(&mut self, val: super::vals::DspSramAccessDisableSram05If) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram06_if(&self) -> super::vals::DspSramAccessDisableSram06If {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DspSramAccessDisableSram06If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram06_if(&mut self, val: super::vals::DspSramAccessDisableSram06If) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram07_if(&self) -> super::vals::DspSramAccessDisableSram07If {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DspSramAccessDisableSram07If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram07_if(&mut self, val: super::vals::DspSramAccessDisableSram07If) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram08_if(&self) -> super::vals::DspSramAccessDisableSram08If {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DspSramAccessDisableSram08If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram08_if(&mut self, val: super::vals::DspSramAccessDisableSram08If) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram09_if(&self) -> super::vals::DspSramAccessDisableSram09If {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DspSramAccessDisableSram09If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram09_if(&mut self, val: super::vals::DspSramAccessDisableSram09If) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram10_if(&self) -> super::vals::DspSramAccessDisableSram10If {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DspSramAccessDisableSram10If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram10_if(&mut self, val: super::vals::DspSramAccessDisableSram10If) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram11_if(&self) -> super::vals::DspSramAccessDisableSram11If {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DspSramAccessDisableSram11If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram11_if(&mut self, val: super::vals::DspSramAccessDisableSram11If) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram12_if(&self) -> super::vals::DspSramAccessDisableSram12If {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::DspSramAccessDisableSram12If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram12_if(&mut self, val: super::vals::DspSramAccessDisableSram12If) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram13_if(&self) -> super::vals::DspSramAccessDisableSram13If {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::DspSramAccessDisableSram13If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram13_if(&mut self, val: super::vals::DspSramAccessDisableSram13If) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram14_if(&self) -> super::vals::DspSramAccessDisableSram14If {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DspSramAccessDisableSram14If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram14_if(&mut self, val: super::vals::DspSramAccessDisableSram14If) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram15_if(&self) -> super::vals::DspSramAccessDisableSram15If {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DspSramAccessDisableSram15If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram15_if(&mut self, val: super::vals::DspSramAccessDisableSram15If) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram16_if(&self) -> super::vals::DspSramAccessDisableSram16If {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DspSramAccessDisableSram16If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram16_if(&mut self, val: super::vals::DspSramAccessDisableSram16If) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram17_if(&self) -> super::vals::DspSramAccessDisableSram17If {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DspSramAccessDisableSram17If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram17_if(&mut self, val: super::vals::DspSramAccessDisableSram17If) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram18_if(&self) -> super::vals::DspSramAccessDisableSram18If {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::DspSramAccessDisableSram18If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram18_if(&mut self, val: super::vals::DspSramAccessDisableSram18If) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram19_if(&self) -> super::vals::DspSramAccessDisableSram19If {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::DspSramAccessDisableSram19If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram19_if(&mut self, val: super::vals::DspSramAccessDisableSram19If) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram20_if(&self) -> super::vals::DspSramAccessDisableSram20If {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DspSramAccessDisableSram20If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram20_if(&mut self, val: super::vals::DspSramAccessDisableSram20If) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram21_if(&self) -> super::vals::DspSramAccessDisableSram21If {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::DspSramAccessDisableSram21If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram21_if(&mut self, val: super::vals::DspSramAccessDisableSram21If) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram22_if(&self) -> super::vals::DspSramAccessDisableSram22If {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DspSramAccessDisableSram22If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram22_if(&mut self, val: super::vals::DspSramAccessDisableSram22If) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram23_if(&self) -> super::vals::DspSramAccessDisableSram23If {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::DspSramAccessDisableSram23If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram23_if(&mut self, val: super::vals::DspSramAccessDisableSram23If) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram24_if(&self) -> super::vals::DspSramAccessDisableSram24If {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DspSramAccessDisableSram24If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram24_if(&mut self, val: super::vals::DspSramAccessDisableSram24If) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram25_if(&self) -> super::vals::DspSramAccessDisableSram25If {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DspSramAccessDisableSram25If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram25_if(&mut self, val: super::vals::DspSramAccessDisableSram25If) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram26_if(&self) -> super::vals::DspSramAccessDisableSram26If {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DspSramAccessDisableSram26If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram26_if(&mut self, val: super::vals::DspSramAccessDisableSram26If) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram27_if(&self) -> super::vals::DspSramAccessDisableSram27If {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::DspSramAccessDisableSram27If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram27_if(&mut self, val: super::vals::DspSramAccessDisableSram27If) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram28_if(&self) -> super::vals::DspSramAccessDisableSram28If {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DspSramAccessDisableSram28If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram28_if(&mut self, val: super::vals::DspSramAccessDisableSram28If) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram29_if(&self) -> super::vals::DspSramAccessDisableSram29If {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::DspSramAccessDisableSram29If::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram29_if(&mut self, val: super::vals::DspSramAccessDisableSram29If) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for DspSramAccessDisable {
    #[inline(always)]
    fn default() -> DspSramAccessDisable {
        DspSramAccessDisable(0)
    }
}
impl core::fmt::Debug for DspSramAccessDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DspSramAccessDisable")
            .field("sram00_if", &self.sram00_if())
            .field("sram01_if", &self.sram01_if())
            .field("sram02_if", &self.sram02_if())
            .field("sram03_if", &self.sram03_if())
            .field("sram04_if", &self.sram04_if())
            .field("sram05_if", &self.sram05_if())
            .field("sram06_if", &self.sram06_if())
            .field("sram07_if", &self.sram07_if())
            .field("sram08_if", &self.sram08_if())
            .field("sram09_if", &self.sram09_if())
            .field("sram10_if", &self.sram10_if())
            .field("sram11_if", &self.sram11_if())
            .field("sram12_if", &self.sram12_if())
            .field("sram13_if", &self.sram13_if())
            .field("sram14_if", &self.sram14_if())
            .field("sram15_if", &self.sram15_if())
            .field("sram16_if", &self.sram16_if())
            .field("sram17_if", &self.sram17_if())
            .field("sram18_if", &self.sram18_if())
            .field("sram19_if", &self.sram19_if())
            .field("sram20_if", &self.sram20_if())
            .field("sram21_if", &self.sram21_if())
            .field("sram22_if", &self.sram22_if())
            .field("sram23_if", &self.sram23_if())
            .field("sram24_if", &self.sram24_if())
            .field("sram25_if", &self.sram25_if())
            .field("sram26_if", &self.sram26_if())
            .field("sram27_if", &self.sram27_if())
            .field("sram28_if", &self.sram28_if())
            .field("sram29_if", &self.sram29_if())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DspSramAccessDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DspSramAccessDisable {{ sram00_if: {:?}, sram01_if: {:?}, sram02_if: {:?}, sram03_if: {:?}, sram04_if: {:?}, sram05_if: {:?}, sram06_if: {:?}, sram07_if: {:?}, sram08_if: {:?}, sram09_if: {:?}, sram10_if: {:?}, sram11_if: {:?}, sram12_if: {:?}, sram13_if: {:?}, sram14_if: {:?}, sram15_if: {:?}, sram16_if: {:?}, sram17_if: {:?}, sram18_if: {:?}, sram19_if: {:?}, sram20_if: {:?}, sram21_if: {:?}, sram22_if: {:?}, sram23_if: {:?}, sram24_if: {:?}, sram25_if: {:?}, sram26_if: {:?}, sram27_if: {:?}, sram28_if: {:?}, sram29_if: {:?} }}",
            self.sram00_if(),
            self.sram01_if(),
            self.sram02_if(),
            self.sram03_if(),
            self.sram04_if(),
            self.sram05_if(),
            self.sram06_if(),
            self.sram07_if(),
            self.sram08_if(),
            self.sram09_if(),
            self.sram10_if(),
            self.sram11_if(),
            self.sram12_if(),
            self.sram13_if(),
            self.sram14_if(),
            self.sram15_if(),
            self.sram16_if(),
            self.sram17_if(),
            self.sram18_if(),
            self.sram19_if(),
            self.sram20_if(),
            self.sram21_if(),
            self.sram22_if(),
            self.sram23_if(),
            self.sram24_if(),
            self.sram25_if(),
            self.sram26_if(),
            self.sram27_if(),
            self.sram28_if(),
            self.sram29_if()
        )
    }
}
#[doc = "DSP stall register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dspstall(pub u32);
impl Dspstall {
    #[doc = "Run / Stall Register. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn dspstall(&self) -> super::vals::Dspstall {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dspstall::from_bits(val as u8)
    }
    #[doc = "Run / Stall Register. . ."]
    #[inline(always)]
    pub const fn set_dspstall(&mut self, val: super::vals::Dspstall) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Dspstall {
    #[inline(always)]
    fn default() -> Dspstall {
        Dspstall(0)
    }
}
impl core::fmt::Debug for Dspstall {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dspstall")
            .field("dspstall", &self.dspstall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dspstall {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dspstall {{ dspstall: {:?} }}", self.dspstall())
    }
}
#[doc = "FLEXSPI NOR flash configure context register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiBootromScratch0(pub u32);
impl FlexspiBootromScratch0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn scratch(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_scratch(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlexspiBootromScratch0 {
    #[inline(always)]
    fn default() -> FlexspiBootromScratch0 {
        FlexspiBootromScratch0(0)
    }
}
impl core::fmt::Debug for FlexspiBootromScratch0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiBootromScratch0")
            .field("scratch", &self.scratch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiBootromScratch0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexspiBootromScratch0 {{ scratch: {=u32:?} }}",
            self.scratch()
        )
    }
}
#[doc = "FLEXSPI IO pads ctrl register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspipadctrl(pub u32);
impl Flexspipadctrl {
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn rasrcn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn fastfrz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn comptq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn compen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Drive FLEXSPI pad compensation circuit"]
    #[inline(always)]
    pub const fn set_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[must_use]
    #[inline(always)]
    pub const fn nasrcn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub const fn set_nasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[must_use]
    #[inline(always)]
    pub const fn nasrcp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub const fn set_nasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[must_use]
    #[inline(always)]
    pub const fn compok(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI pad compensation circuit status"]
    #[inline(always)]
    pub const fn set_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Flexspipadctrl {
    #[inline(always)]
    fn default() -> Flexspipadctrl {
        Flexspipadctrl(0)
    }
}
impl core::fmt::Debug for Flexspipadctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexspipadctrl")
            .field("rasrcn", &self.rasrcn())
            .field("rasrcp", &self.rasrcp())
            .field("fastfrz", &self.fastfrz())
            .field("freeze", &self.freeze())
            .field("comptq", &self.comptq())
            .field("compen", &self.compen())
            .field("nasrcn", &self.nasrcn())
            .field("nasrcp", &self.nasrcp())
            .field("compok", &self.compok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexspipadctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexspipadctrl {{ rasrcn: {=u8:?}, rasrcp: {=u8:?}, fastfrz: {=bool:?}, freeze: {=bool:?}, comptq: {=bool:?}, compen: {=bool:?}, nasrcn: {=u8:?}, nasrcp: {=u8:?}, compok: {=bool:?} }}",
            self.rasrcn(),
            self.rasrcp(),
            self.fastfrz(),
            self.freeze(),
            self.comptq(),
            self.compen(),
            self.nasrcn(),
            self.nasrcp(),
            self.compok()
        )
    }
}
#[doc = "Hash hardware key disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hashhwkeydisable(pub u32);
impl Hashhwkeydisable {
    #[doc = "This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
    #[must_use]
    #[inline(always)]
    pub const fn hashhwkeydisable(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This register control the access to AES keys delivered through secret HW bus from PUF and OTP to AES engine"]
    #[inline(always)]
    pub const fn set_hashhwkeydisable(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Hashhwkeydisable {
    #[inline(always)]
    fn default() -> Hashhwkeydisable {
        Hashhwkeydisable(0)
    }
}
impl core::fmt::Debug for Hashhwkeydisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hashhwkeydisable")
            .field("hashhwkeydisable", &self.hashhwkeydisable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hashhwkeydisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hashhwkeydisable {{ hashhwkeydisable: {=u32:?} }}",
            self.hashhwkeydisable()
        )
    }
}
#[doc = "HW unlock disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwunlockDisable(pub u32);
impl HwunlockDisable {
    #[doc = "HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
    #[must_use]
    #[inline(always)]
    pub const fn hwunlock_disable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HW Unlock / Disable: Disable the OTP unlock for test, Teal and DSP debug Write any value to disable the allow testmode signal/allow CPU0 debug signal/allow CPU1 debug signal"]
    #[inline(always)]
    pub const fn set_hwunlock_disable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for HwunlockDisable {
    #[inline(always)]
    fn default() -> HwunlockDisable {
        HwunlockDisable(0)
    }
}
impl core::fmt::Debug for HwunlockDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwunlockDisable")
            .field("hwunlock_disable", &self.hwunlock_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwunlockDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwunlockDisable {{ hwunlock_disable: {=u8:?} }}",
            self.hwunlock_disable()
        )
    }
}
#[doc = "Hardware Wake-up control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwwake(pub u32);
impl Hwwake {
    #[doc = "Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[must_use]
    #[inline(always)]
    pub const fn forcewake(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force peripheral clocking to stay on during deep-sleep mode. When 1, clocking to peripherals is prevented from being shut down when the CPU enters deep-sleep mode. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub const fn set_forcewake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn fcwake(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wake for Flexcomm Interfaces. When 1, any Flexcomm Interface FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub const fn set_fcwake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn dmicwake(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub const fn set_dmicwake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0wake(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Wake for DMAC0. When 1, DMAC0 being busy will cause peripheral clocking to remain running until DMAC0 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC0 has completed its related activity."]
    #[inline(always)]
    pub const fn set_dmac0wake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1wake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Wake for DMAC1. When 1, DMAC1 being busy will cause peripheral clocking to remain running until DMAC1 completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMAC1 has completed its related activity."]
    #[inline(always)]
    pub const fn set_dmac1wake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
    #[must_use]
    #[inline(always)]
    pub const fn espiwake(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Wake for ESPI. When 1, ESPI being busy will cause peripheral clocking to remain running until ESPI completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before ESPI has completed its related activity."]
    #[inline(always)]
    pub const fn set_espiwake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Hwwake {
    #[inline(always)]
    fn default() -> Hwwake {
        Hwwake(0)
    }
}
impl core::fmt::Debug for Hwwake {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwwake")
            .field("forcewake", &self.forcewake())
            .field("fcwake", &self.fcwake())
            .field("dmicwake", &self.dmicwake())
            .field("dmac0wake", &self.dmac0wake())
            .field("dmac1wake", &self.dmac1wake())
            .field("espiwake", &self.espiwake())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwwake {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwwake {{ forcewake: {=bool:?}, fcwake: {=bool:?}, dmicwake: {=bool:?}, dmac0wake: {=bool:?}, dmac1wake: {=bool:?}, espiwake: {=bool:?} }}",
            self.forcewake(),
            self.fcwake(),
            self.dmicwake(),
            self.dmac0wake(),
            self.dmac1wake(),
            self.espiwake()
        )
    }
}
#[doc = "jtag ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "JTAG IDCODE fix bit"]
    #[must_use]
    #[inline(always)]
    pub const fn fixbit(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG IDCODE fix bit"]
    #[inline(always)]
    pub const fn set_fixbit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "JTAG IDCODE manufacturer identity"]
    #[must_use]
    #[inline(always)]
    pub const fn manu(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x07ff;
        val as u16
    }
    #[doc = "JTAG IDCODE manufacturer identity"]
    #[inline(always)]
    pub const fn set_manu(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 1usize)) | (((val as u32) & 0x07ff) << 1usize);
    }
    #[doc = "JTAG IDCODE part number"]
    #[must_use]
    #[inline(always)]
    pub const fn partnum(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[doc = "JTAG IDCODE part number"]
    #[inline(always)]
    pub const fn set_partnum(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[doc = "JTAG IDCODE version number"]
    #[must_use]
    #[inline(always)]
    pub const fn vernum(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "JTAG IDCODE version number"]
    #[inline(always)]
    pub const fn set_vernum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("fixbit", &self.fixbit())
            .field("manu", &self.manu())
            .field("partnum", &self.partnum())
            .field("vernum", &self.vernum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "JtagId {{ fixbit: {=bool:?}, manu: {=u16:?}, partnum: {=u16:?}, vernum: {=u8:?} }}",
            self.fixbit(),
            self.manu(),
            self.partnum(),
            self.vernum()
        )
    }
}
#[doc = "Key block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyBlock(pub u32);
impl KeyBlock {
    #[doc = "key block register"]
    #[must_use]
    #[inline(always)]
    pub const fn key_block(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "key block register"]
    #[inline(always)]
    pub const fn set_key_block(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KeyBlock {
    #[inline(always)]
    fn default() -> KeyBlock {
        KeyBlock(0)
    }
}
impl core::fmt::Debug for KeyBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyBlock")
            .field("key_block", &self.key_block())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyBlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KeyBlock {{ key_block: {=u32:?} }}", self.key_block())
    }
}
#[doc = "M33 nmi source selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M33nmisrcsel(pub u32);
impl M33nmisrcsel {
    #[doc = "Selects one of the M33 interrupt sources as the NMI source. See M33 Interrupt Slot Table for Interrupt Slot Numers."]
    #[must_use]
    #[inline(always)]
    pub const fn nmisrcsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Selects one of the M33 interrupt sources as the NMI source. See M33 Interrupt Slot Table for Interrupt Slot Numers."]
    #[inline(always)]
    pub const fn set_nmisrcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "NMI interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nmien(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "NMI interrupt enable"]
    #[inline(always)]
    pub const fn set_nmien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for M33nmisrcsel {
    #[inline(always)]
    fn default() -> M33nmisrcsel {
        M33nmisrcsel(0)
    }
}
impl core::fmt::Debug for M33nmisrcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M33nmisrcsel")
            .field("nmisrcsel", &self.nmisrcsel())
            .field("nmien", &self.nmien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M33nmisrcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "M33nmisrcsel {{ nmisrcsel: {=u8:?}, nmien: {=bool:?} }}",
            self.nmisrcsel(),
            self.nmien()
        )
    }
}
#[doc = "Main Clock Safety"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclksafety(pub u32);
impl Mainclksafety {
    #[doc = "Main Clock turn on delay for Deep Sleep wake up"]
    #[must_use]
    #[inline(always)]
    pub const fn delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Main Clock turn on delay for Deep Sleep wake up"]
    #[inline(always)]
    pub const fn set_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Mainclksafety {
    #[inline(always)]
    fn default() -> Mainclksafety {
        Mainclksafety(0)
    }
}
impl core::fmt::Debug for Mainclksafety {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mainclksafety")
            .field("delay", &self.delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mainclksafety {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mainclksafety {{ delay: {=u16:?} }}", self.delay())
    }
}
#[doc = "Packer enable for DSP RAM packer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Packerenable(pub u32);
impl Packerenable {
    #[doc = "Write Packer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wrpenable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Packer Enable."]
    #[inline(always)]
    pub const fn set_wrpenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read Packer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdpenable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read Packer Enable"]
    #[inline(always)]
    pub const fn set_rdpenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Packerenable {
    #[inline(always)]
    fn default() -> Packerenable {
        Packerenable(0)
    }
}
impl core::fmt::Debug for Packerenable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Packerenable")
            .field("wrpenable", &self.wrpenable())
            .field("rdpenable", &self.rdpenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Packerenable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Packerenable {{ wrpenable: {=bool:?}, rdpenable: {=bool:?} }}",
            self.wrpenable(),
            self.rdpenable()
        )
    }
}
#[doc = "Run configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg0(pub u32);
impl Pdruncfg0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode0(&self) -> super::vals::Pdruncfg0PmicMode0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg0PmicMode0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode0(&mut self, val: super::vals::Pdruncfg0PmicMode0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode1(&self) -> super::vals::Pdruncfg0PmicMode1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg0PmicMode1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode1(&mut self, val: super::vals::Pdruncfg0PmicMode1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorereg_lp(&self) -> super::vals::Pdruncfg0VddcoreregLp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg0VddcoreregLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddcorereg_lp(&mut self, val: super::vals::Pdruncfg0VddcoreregLp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmcref_lp(&self) -> super::vals::Pdruncfg0PmcrefLp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg0PmcrefLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmcref_lp(&mut self, val: super::vals::Pdruncfg0PmcrefLp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8_pd(&self) -> super::vals::Pdruncfg0Hvd1v8Pd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg0Hvd1v8Pd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvd1v8_pd(&mut self, val: super::vals::Pdruncfg0Hvd1v8Pd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn porcore_lp(&self) -> super::vals::Pdruncfg0PorcoreLp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg0PorcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_porcore_lp(&mut self, val: super::vals::Pdruncfg0PorcoreLp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcore_lp(&self) -> super::vals::Pdruncfg0LvdcoreLp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg0LvdcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lvdcore_lp(&mut self, val: super::vals::Pdruncfg0LvdcoreLp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcore_pd(&self) -> super::vals::Pdruncfg0HvdcorePd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg0HvdcorePd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvdcore_pd(&mut self, val: super::vals::Pdruncfg0HvdcorePd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sysxtal_pd(&self) -> super::vals::Pdruncfg0SysxtalPd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg0SysxtalPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sysxtal_pd(&mut self, val: super::vals::Pdruncfg0SysxtalPd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lposc_pd(&self) -> super::vals::Pdruncfg0LposcPd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg0LposcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lposc_pd(&mut self, val: super::vals::Pdruncfg0LposcPd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sfro_pd(&self) -> super::vals::Pdruncfg0SfroPd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg0SfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sfro_pd(&mut self, val: super::vals::Pdruncfg0SfroPd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ffro_pd(&self) -> super::vals::Pdruncfg0FfroPd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg0FfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ffro_pd(&mut self, val: super::vals::Pdruncfg0FfroPd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllldo_pd(&self) -> super::vals::Pdruncfg0SyspllldoPd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg0SyspllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllldo_pd(&mut self, val: super::vals::Pdruncfg0SyspllldoPd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllana_pd(&self) -> super::vals::Pdruncfg0SyspllanaPd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg0SyspllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllana_pd(&mut self, val: super::vals::Pdruncfg0SyspllanaPd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllldo_pd(&self) -> super::vals::Pdruncfg0AudpllldoPd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg0AudpllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllldo_pd(&mut self, val: super::vals::Pdruncfg0AudpllldoPd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllana_pd(&self) -> super::vals::Pdruncfg0AudpllanaPd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg0AudpllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllana_pd(&mut self, val: super::vals::Pdruncfg0AudpllanaPd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_pd(&self) -> super::vals::Pdruncfg0AdcPd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg0AdcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_pd(&mut self, val: super::vals::Pdruncfg0AdcPd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_lp(&self) -> super::vals::Pdruncfg0AdcLp {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg0AdcLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_lp(&mut self, val: super::vals::Pdruncfg0AdcLp) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adctempsns_pd(&self) -> super::vals::Pdruncfg0AdctempsnsPd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg0AdctempsnsPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adctempsns_pd(&mut self, val: super::vals::Pdruncfg0AdctempsnsPd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_pd(&self) -> super::vals::Pdruncfg0AcmpPd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg0AcmpPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp_pd(&mut self, val: super::vals::Pdruncfg0AcmpPd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "High Speed Pad vdde0 voltage detect block"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_vdet_lp(&self) -> super::vals::Pdruncfg0Hspad0VdetLp {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg0Hspad0VdetLp::from_bits(val as u8)
    }
    #[doc = "High Speed Pad vdde0 voltage detect block"]
    #[inline(always)]
    pub const fn set_hspad0_vdet_lp(&mut self, val: super::vals::Pdruncfg0Hspad0VdetLp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "High speed Pad vdde0 reference blocks"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_ref_pd(&self) -> super::vals::Pdruncfg0Hspad0RefPd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg0Hspad0RefPd::from_bits(val as u8)
    }
    #[doc = "High speed Pad vdde0 reference blocks"]
    #[inline(always)]
    pub const fn set_hspad0_ref_pd(&mut self, val: super::vals::Pdruncfg0Hspad0RefPd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "High Speed Pad vdde2 voltage detect block"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_vdet_lp(&self) -> super::vals::Pdruncfg0Hspad2VdetLp {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg0Hspad2VdetLp::from_bits(val as u8)
    }
    #[doc = "High Speed Pad vdde2 voltage detect block"]
    #[inline(always)]
    pub const fn set_hspad2_vdet_lp(&mut self, val: super::vals::Pdruncfg0Hspad2VdetLp) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "High speed Pad vdde2 reference blocks"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_ref_pd(&self) -> super::vals::Pdruncfg0Hspad2RefPd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg0Hspad2RefPd::from_bits(val as u8)
    }
    #[doc = "High speed Pad vdde2 reference blocks"]
    #[inline(always)]
    pub const fn set_hspad2_ref_pd(&mut self, val: super::vals::Pdruncfg0Hspad2RefPd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg0 {
    #[inline(always)]
    fn default() -> Pdruncfg0 {
        Pdruncfg0(0)
    }
}
impl core::fmt::Debug for Pdruncfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg0")
            .field("pmic_mode0", &self.pmic_mode0())
            .field("pmic_mode1", &self.pmic_mode1())
            .field("vddcorereg_lp", &self.vddcorereg_lp())
            .field("pmcref_lp", &self.pmcref_lp())
            .field("hvd1v8_pd", &self.hvd1v8_pd())
            .field("porcore_lp", &self.porcore_lp())
            .field("lvdcore_lp", &self.lvdcore_lp())
            .field("hvdcore_pd", &self.hvdcore_pd())
            .field("sysxtal_pd", &self.sysxtal_pd())
            .field("lposc_pd", &self.lposc_pd())
            .field("sfro_pd", &self.sfro_pd())
            .field("ffro_pd", &self.ffro_pd())
            .field("syspllldo_pd", &self.syspllldo_pd())
            .field("syspllana_pd", &self.syspllana_pd())
            .field("audpllldo_pd", &self.audpllldo_pd())
            .field("audpllana_pd", &self.audpllana_pd())
            .field("adc_pd", &self.adc_pd())
            .field("adc_lp", &self.adc_lp())
            .field("adctempsns_pd", &self.adctempsns_pd())
            .field("acmp_pd", &self.acmp_pd())
            .field("hspad0_vdet_lp", &self.hspad0_vdet_lp())
            .field("hspad0_ref_pd", &self.hspad0_ref_pd())
            .field("hspad2_vdet_lp", &self.hspad2_vdet_lp())
            .field("hspad2_ref_pd", &self.hspad2_ref_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg0 {{ pmic_mode0: {:?}, pmic_mode1: {:?}, vddcorereg_lp: {:?}, pmcref_lp: {:?}, hvd1v8_pd: {:?}, porcore_lp: {:?}, lvdcore_lp: {:?}, hvdcore_pd: {:?}, sysxtal_pd: {:?}, lposc_pd: {:?}, sfro_pd: {:?}, ffro_pd: {:?}, syspllldo_pd: {:?}, syspllana_pd: {:?}, audpllldo_pd: {:?}, audpllana_pd: {:?}, adc_pd: {:?}, adc_lp: {:?}, adctempsns_pd: {:?}, acmp_pd: {:?}, hspad0_vdet_lp: {:?}, hspad0_ref_pd: {:?}, hspad2_vdet_lp: {:?}, hspad2_ref_pd: {:?} }}",
            self.pmic_mode0(),
            self.pmic_mode1(),
            self.vddcorereg_lp(),
            self.pmcref_lp(),
            self.hvd1v8_pd(),
            self.porcore_lp(),
            self.lvdcore_lp(),
            self.hvdcore_pd(),
            self.sysxtal_pd(),
            self.lposc_pd(),
            self.sfro_pd(),
            self.ffro_pd(),
            self.syspllldo_pd(),
            self.syspllana_pd(),
            self.audpllldo_pd(),
            self.audpllana_pd(),
            self.adc_pd(),
            self.adc_lp(),
            self.adctempsns_pd(),
            self.acmp_pd(),
            self.hspad0_vdet_lp(),
            self.hspad0_ref_pd(),
            self.hspad2_vdet_lp(),
            self.hspad2_ref_pd()
        )
    }
}
#[doc = "Run configuration 0 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg0Clr(pub u32);
impl Pdruncfg0Clr {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode0(&self) -> super::vals::Pdruncfg0ClrPmicMode0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg0ClrPmicMode0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode0(&mut self, val: super::vals::Pdruncfg0ClrPmicMode0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode1(&self) -> super::vals::Pdruncfg0ClrPmicMode1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg0ClrPmicMode1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode1(&mut self, val: super::vals::Pdruncfg0ClrPmicMode1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorereg_lp(&self) -> super::vals::Pdruncfg0ClrVddcoreregLp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg0ClrVddcoreregLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddcorereg_lp(&mut self, val: super::vals::Pdruncfg0ClrVddcoreregLp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmcref_lp(&self) -> super::vals::Pdruncfg0ClrPmcrefLp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg0ClrPmcrefLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmcref_lp(&mut self, val: super::vals::Pdruncfg0ClrPmcrefLp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8_pd(&self) -> super::vals::Pdruncfg0ClrHvd1v8Pd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg0ClrHvd1v8Pd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvd1v8_pd(&mut self, val: super::vals::Pdruncfg0ClrHvd1v8Pd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn porcore_lp(&self) -> super::vals::Pdruncfg0ClrPorcoreLp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg0ClrPorcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_porcore_lp(&mut self, val: super::vals::Pdruncfg0ClrPorcoreLp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcore_lp(&self) -> super::vals::Pdruncfg0ClrLvdcoreLp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg0ClrLvdcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lvdcore_lp(&mut self, val: super::vals::Pdruncfg0ClrLvdcoreLp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcore_pd(&self) -> super::vals::Pdruncfg0ClrHvdcorePd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg0ClrHvdcorePd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvdcore_pd(&mut self, val: super::vals::Pdruncfg0ClrHvdcorePd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rbb_pd(&self) -> super::vals::Pdruncfg0ClrRbbPd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg0ClrRbbPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rbb_pd(&mut self, val: super::vals::Pdruncfg0ClrRbbPd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fbb_pd(&self) -> super::vals::Pdruncfg0ClrFbbPd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg0ClrFbbPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fbb_pd(&mut self, val: super::vals::Pdruncfg0ClrFbbPd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sysxtal_pd(&self) -> super::vals::Pdruncfg0ClrSysxtalPd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg0ClrSysxtalPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sysxtal_pd(&mut self, val: super::vals::Pdruncfg0ClrSysxtalPd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lposc_pd(&self) -> super::vals::Pdruncfg0ClrLposcPd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg0ClrLposcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lposc_pd(&mut self, val: super::vals::Pdruncfg0ClrLposcPd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sfro_pd(&self) -> super::vals::Pdruncfg0ClrSfroPd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg0ClrSfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sfro_pd(&mut self, val: super::vals::Pdruncfg0ClrSfroPd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ffro_pd(&self) -> super::vals::Pdruncfg0ClrFfroPd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg0ClrFfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ffro_pd(&mut self, val: super::vals::Pdruncfg0ClrFfroPd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllldo_pd(&self) -> super::vals::Pdruncfg0ClrSyspllldoPd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg0ClrSyspllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllldo_pd(&mut self, val: super::vals::Pdruncfg0ClrSyspllldoPd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllana_pd(&self) -> super::vals::Pdruncfg0ClrSyspllanaPd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg0ClrSyspllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllana_pd(&mut self, val: super::vals::Pdruncfg0ClrSyspllanaPd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllldo_pd(&self) -> super::vals::Pdruncfg0ClrAudpllldoPd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg0ClrAudpllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllldo_pd(&mut self, val: super::vals::Pdruncfg0ClrAudpllldoPd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllana_pd(&self) -> super::vals::Pdruncfg0ClrAudpllanaPd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg0ClrAudpllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllana_pd(&mut self, val: super::vals::Pdruncfg0ClrAudpllanaPd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_pd(&self) -> super::vals::Pdruncfg0ClrAdcPd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg0ClrAdcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_pd(&mut self, val: super::vals::Pdruncfg0ClrAdcPd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_lp(&self) -> super::vals::Pdruncfg0ClrAdcLp {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg0ClrAdcLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_lp(&mut self, val: super::vals::Pdruncfg0ClrAdcLp) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adctempsns_pd(&self) -> super::vals::Pdruncfg0ClrAdctempsnsPd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg0ClrAdctempsnsPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adctempsns_pd(&mut self, val: super::vals::Pdruncfg0ClrAdctempsnsPd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_pd(&self) -> super::vals::Pdruncfg0ClrAcmpPd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg0ClrAcmpPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp_pd(&mut self, val: super::vals::Pdruncfg0ClrAcmpPd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_vdet_lp(&self) -> super::vals::Pdruncfg0ClrHspad0VdetLp {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg0ClrHspad0VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_vdet_lp(&mut self, val: super::vals::Pdruncfg0ClrHspad0VdetLp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_ref_pd(&self) -> super::vals::Pdruncfg0ClrHspad0RefPd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg0ClrHspad0RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_ref_pd(&mut self, val: super::vals::Pdruncfg0ClrHspad0RefPd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_vdet_lp(&self) -> super::vals::Pdruncfg0ClrHspad2VdetLp {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg0ClrHspad2VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_vdet_lp(&mut self, val: super::vals::Pdruncfg0ClrHspad2VdetLp) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_ref_pd(&self) -> super::vals::Pdruncfg0ClrHspad2RefPd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg0ClrHspad2RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_ref_pd(&mut self, val: super::vals::Pdruncfg0ClrHspad2RefPd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg0Clr {
    #[inline(always)]
    fn default() -> Pdruncfg0Clr {
        Pdruncfg0Clr(0)
    }
}
impl core::fmt::Debug for Pdruncfg0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg0Clr")
            .field("pmic_mode0", &self.pmic_mode0())
            .field("pmic_mode1", &self.pmic_mode1())
            .field("vddcorereg_lp", &self.vddcorereg_lp())
            .field("pmcref_lp", &self.pmcref_lp())
            .field("hvd1v8_pd", &self.hvd1v8_pd())
            .field("porcore_lp", &self.porcore_lp())
            .field("lvdcore_lp", &self.lvdcore_lp())
            .field("hvdcore_pd", &self.hvdcore_pd())
            .field("rbb_pd", &self.rbb_pd())
            .field("fbb_pd", &self.fbb_pd())
            .field("sysxtal_pd", &self.sysxtal_pd())
            .field("lposc_pd", &self.lposc_pd())
            .field("sfro_pd", &self.sfro_pd())
            .field("ffro_pd", &self.ffro_pd())
            .field("syspllldo_pd", &self.syspllldo_pd())
            .field("syspllana_pd", &self.syspllana_pd())
            .field("audpllldo_pd", &self.audpllldo_pd())
            .field("audpllana_pd", &self.audpllana_pd())
            .field("adc_pd", &self.adc_pd())
            .field("adc_lp", &self.adc_lp())
            .field("adctempsns_pd", &self.adctempsns_pd())
            .field("acmp_pd", &self.acmp_pd())
            .field("hspad0_vdet_lp", &self.hspad0_vdet_lp())
            .field("hspad0_ref_pd", &self.hspad0_ref_pd())
            .field("hspad2_vdet_lp", &self.hspad2_vdet_lp())
            .field("hspad2_ref_pd", &self.hspad2_ref_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg0Clr {{ pmic_mode0: {:?}, pmic_mode1: {:?}, vddcorereg_lp: {:?}, pmcref_lp: {:?}, hvd1v8_pd: {:?}, porcore_lp: {:?}, lvdcore_lp: {:?}, hvdcore_pd: {:?}, rbb_pd: {:?}, fbb_pd: {:?}, sysxtal_pd: {:?}, lposc_pd: {:?}, sfro_pd: {:?}, ffro_pd: {:?}, syspllldo_pd: {:?}, syspllana_pd: {:?}, audpllldo_pd: {:?}, audpllana_pd: {:?}, adc_pd: {:?}, adc_lp: {:?}, adctempsns_pd: {:?}, acmp_pd: {:?}, hspad0_vdet_lp: {:?}, hspad0_ref_pd: {:?}, hspad2_vdet_lp: {:?}, hspad2_ref_pd: {:?} }}",
            self.pmic_mode0(),
            self.pmic_mode1(),
            self.vddcorereg_lp(),
            self.pmcref_lp(),
            self.hvd1v8_pd(),
            self.porcore_lp(),
            self.lvdcore_lp(),
            self.hvdcore_pd(),
            self.rbb_pd(),
            self.fbb_pd(),
            self.sysxtal_pd(),
            self.lposc_pd(),
            self.sfro_pd(),
            self.ffro_pd(),
            self.syspllldo_pd(),
            self.syspllana_pd(),
            self.audpllldo_pd(),
            self.audpllana_pd(),
            self.adc_pd(),
            self.adc_lp(),
            self.adctempsns_pd(),
            self.acmp_pd(),
            self.hspad0_vdet_lp(),
            self.hspad0_ref_pd(),
            self.hspad2_vdet_lp(),
            self.hspad2_ref_pd()
        )
    }
}
#[doc = "Run configuration 0 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg0Set(pub u32);
impl Pdruncfg0Set {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode0(&self) -> super::vals::Pdruncfg0SetPmicMode0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg0SetPmicMode0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode0(&mut self, val: super::vals::Pdruncfg0SetPmicMode0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode1(&self) -> super::vals::Pdruncfg0SetPmicMode1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg0SetPmicMode1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode1(&mut self, val: super::vals::Pdruncfg0SetPmicMode1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorereg_lp(&self) -> super::vals::Pdruncfg0SetVddcoreregLp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg0SetVddcoreregLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddcorereg_lp(&mut self, val: super::vals::Pdruncfg0SetVddcoreregLp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmcref_lp(&self) -> super::vals::Pdruncfg0SetPmcrefLp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg0SetPmcrefLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmcref_lp(&mut self, val: super::vals::Pdruncfg0SetPmcrefLp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8_pd(&self) -> super::vals::Pdruncfg0SetHvd1v8Pd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg0SetHvd1v8Pd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvd1v8_pd(&mut self, val: super::vals::Pdruncfg0SetHvd1v8Pd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn porcore_lp(&self) -> super::vals::Pdruncfg0SetPorcoreLp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg0SetPorcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_porcore_lp(&mut self, val: super::vals::Pdruncfg0SetPorcoreLp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcore_lp(&self) -> super::vals::Pdruncfg0SetLvdcoreLp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg0SetLvdcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lvdcore_lp(&mut self, val: super::vals::Pdruncfg0SetLvdcoreLp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcore_pd(&self) -> super::vals::Pdruncfg0SetHvdcorePd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg0SetHvdcorePd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvdcore_pd(&mut self, val: super::vals::Pdruncfg0SetHvdcorePd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rbb_pd(&self) -> super::vals::Pdruncfg0SetRbbPd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg0SetRbbPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rbb_pd(&mut self, val: super::vals::Pdruncfg0SetRbbPd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn fbb_pd(&self) -> super::vals::Pdruncfg0SetFbbPd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg0SetFbbPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_fbb_pd(&mut self, val: super::vals::Pdruncfg0SetFbbPd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sysxtal_pd(&self) -> super::vals::Pdruncfg0SetSysxtalPd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg0SetSysxtalPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sysxtal_pd(&mut self, val: super::vals::Pdruncfg0SetSysxtalPd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lposc_pd(&self) -> super::vals::Pdruncfg0SetLposcPd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg0SetLposcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lposc_pd(&mut self, val: super::vals::Pdruncfg0SetLposcPd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sfro_pd(&self) -> super::vals::Pdruncfg0SetSfroPd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg0SetSfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sfro_pd(&mut self, val: super::vals::Pdruncfg0SetSfroPd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ffro_pd(&self) -> super::vals::Pdruncfg0SetFfroPd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg0SetFfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ffro_pd(&mut self, val: super::vals::Pdruncfg0SetFfroPd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllldo_pd(&self) -> super::vals::Pdruncfg0SetSyspllldoPd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg0SetSyspllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllldo_pd(&mut self, val: super::vals::Pdruncfg0SetSyspllldoPd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllana_pd(&self) -> super::vals::Pdruncfg0SetSyspllanaPd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg0SetSyspllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllana_pd(&mut self, val: super::vals::Pdruncfg0SetSyspllanaPd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllldo_pd(&self) -> super::vals::Pdruncfg0SetAudpllldoPd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg0SetAudpllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllldo_pd(&mut self, val: super::vals::Pdruncfg0SetAudpllldoPd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllana_pd(&self) -> super::vals::Pdruncfg0SetAudpllanaPd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg0SetAudpllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllana_pd(&mut self, val: super::vals::Pdruncfg0SetAudpllanaPd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_pd(&self) -> super::vals::Pdruncfg0SetAdcPd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg0SetAdcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_pd(&mut self, val: super::vals::Pdruncfg0SetAdcPd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_lp(&self) -> super::vals::Pdruncfg0SetAdcLp {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg0SetAdcLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_lp(&mut self, val: super::vals::Pdruncfg0SetAdcLp) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adctempsns_pd(&self) -> super::vals::Pdruncfg0SetAdctempsnsPd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg0SetAdctempsnsPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adctempsns_pd(&mut self, val: super::vals::Pdruncfg0SetAdctempsnsPd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_pd(&self) -> super::vals::Pdruncfg0SetAcmpPd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg0SetAcmpPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp_pd(&mut self, val: super::vals::Pdruncfg0SetAcmpPd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_vdet_lp(&self) -> super::vals::Pdruncfg0SetHspad0VdetLp {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg0SetHspad0VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_vdet_lp(&mut self, val: super::vals::Pdruncfg0SetHspad0VdetLp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_ref_pd(&self) -> super::vals::Pdruncfg0SetHspad0RefPd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg0SetHspad0RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_ref_pd(&mut self, val: super::vals::Pdruncfg0SetHspad0RefPd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_vdet_lp(&self) -> super::vals::Pdruncfg0SetHspad2VdetLp {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg0SetHspad2VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_vdet_lp(&mut self, val: super::vals::Pdruncfg0SetHspad2VdetLp) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_ref_pd(&self) -> super::vals::Pdruncfg0SetHspad2RefPd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg0SetHspad2RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_ref_pd(&mut self, val: super::vals::Pdruncfg0SetHspad2RefPd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg0Set {
    #[inline(always)]
    fn default() -> Pdruncfg0Set {
        Pdruncfg0Set(0)
    }
}
impl core::fmt::Debug for Pdruncfg0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg0Set")
            .field("pmic_mode0", &self.pmic_mode0())
            .field("pmic_mode1", &self.pmic_mode1())
            .field("vddcorereg_lp", &self.vddcorereg_lp())
            .field("pmcref_lp", &self.pmcref_lp())
            .field("hvd1v8_pd", &self.hvd1v8_pd())
            .field("porcore_lp", &self.porcore_lp())
            .field("lvdcore_lp", &self.lvdcore_lp())
            .field("hvdcore_pd", &self.hvdcore_pd())
            .field("rbb_pd", &self.rbb_pd())
            .field("fbb_pd", &self.fbb_pd())
            .field("sysxtal_pd", &self.sysxtal_pd())
            .field("lposc_pd", &self.lposc_pd())
            .field("sfro_pd", &self.sfro_pd())
            .field("ffro_pd", &self.ffro_pd())
            .field("syspllldo_pd", &self.syspllldo_pd())
            .field("syspllana_pd", &self.syspllana_pd())
            .field("audpllldo_pd", &self.audpllldo_pd())
            .field("audpllana_pd", &self.audpllana_pd())
            .field("adc_pd", &self.adc_pd())
            .field("adc_lp", &self.adc_lp())
            .field("adctempsns_pd", &self.adctempsns_pd())
            .field("acmp_pd", &self.acmp_pd())
            .field("hspad0_vdet_lp", &self.hspad0_vdet_lp())
            .field("hspad0_ref_pd", &self.hspad0_ref_pd())
            .field("hspad2_vdet_lp", &self.hspad2_vdet_lp())
            .field("hspad2_ref_pd", &self.hspad2_ref_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg0Set {{ pmic_mode0: {:?}, pmic_mode1: {:?}, vddcorereg_lp: {:?}, pmcref_lp: {:?}, hvd1v8_pd: {:?}, porcore_lp: {:?}, lvdcore_lp: {:?}, hvdcore_pd: {:?}, rbb_pd: {:?}, fbb_pd: {:?}, sysxtal_pd: {:?}, lposc_pd: {:?}, sfro_pd: {:?}, ffro_pd: {:?}, syspllldo_pd: {:?}, syspllana_pd: {:?}, audpllldo_pd: {:?}, audpllana_pd: {:?}, adc_pd: {:?}, adc_lp: {:?}, adctempsns_pd: {:?}, acmp_pd: {:?}, hspad0_vdet_lp: {:?}, hspad0_ref_pd: {:?}, hspad2_vdet_lp: {:?}, hspad2_ref_pd: {:?} }}",
            self.pmic_mode0(),
            self.pmic_mode1(),
            self.vddcorereg_lp(),
            self.pmcref_lp(),
            self.hvd1v8_pd(),
            self.porcore_lp(),
            self.lvdcore_lp(),
            self.hvdcore_pd(),
            self.rbb_pd(),
            self.fbb_pd(),
            self.sysxtal_pd(),
            self.lposc_pd(),
            self.sfro_pd(),
            self.ffro_pd(),
            self.syspllldo_pd(),
            self.syspllana_pd(),
            self.audpllldo_pd(),
            self.audpllana_pd(),
            self.adc_pd(),
            self.adc_lp(),
            self.adctempsns_pd(),
            self.acmp_pd(),
            self.hspad0_vdet_lp(),
            self.hspad0_ref_pd(),
            self.hspad2_vdet_lp(),
            self.hspad2_ref_pd()
        )
    }
}
#[doc = "Run configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg1(pub u32);
impl Pdruncfg1 {
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_apd(&self) -> super::vals::Pdruncfg1PqSramApd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg1PqSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_pq_sram_apd(&mut self, val: super::vals::Pdruncfg1PqSramApd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_ppd(&self) -> super::vals::Pdruncfg1PqSramPpd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg1PqSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_pq_sram_ppd(&mut self, val: super::vals::Pdruncfg1PqSramPpd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_apd(&self) -> super::vals::Pdruncfg1FlexspiSramApd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg1FlexspiSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_apd(&mut self, val: super::vals::Pdruncfg1FlexspiSramApd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_ppd(&self) -> super::vals::Pdruncfg1FlexspiSramPpd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg1FlexspiSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_ppd(&mut self, val: super::vals::Pdruncfg1FlexspiSramPpd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_apd(&self) -> super::vals::Pdruncfg1UsbhsSramApd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg1UsbhsSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_apd(&mut self, val: super::vals::Pdruncfg1UsbhsSramApd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_ppd(&self) -> super::vals::Pdruncfg1UsbhsSramPpd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg1UsbhsSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_ppd(&mut self, val: super::vals::Pdruncfg1UsbhsSramPpd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_apd(&self) -> super::vals::Pdruncfg1Usdhc0SramApd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg1Usdhc0SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_apd(&mut self, val: super::vals::Pdruncfg1Usdhc0SramApd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_ppd(&self) -> super::vals::Pdruncfg1Usdhc0SramPpd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg1Usdhc0SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_ppd(&mut self, val: super::vals::Pdruncfg1Usdhc0SramPpd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_apd(&self) -> super::vals::Pdruncfg1Usdhc1SramApd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg1Usdhc1SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_apd(&mut self, val: super::vals::Pdruncfg1Usdhc1SramApd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_ppd(&self) -> super::vals::Pdruncfg1Usdhc1SramPpd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg1Usdhc1SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_ppd(&mut self, val: super::vals::Pdruncfg1Usdhc1SramPpd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_apd(&self) -> super::vals::Pdruncfg1CasperSramApd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg1CasperSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_casper_sram_apd(&mut self, val: super::vals::Pdruncfg1CasperSramApd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_ppd(&self) -> super::vals::Pdruncfg1CasperSramPpd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg1CasperSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_casper_sram_ppd(&mut self, val: super::vals::Pdruncfg1CasperSramPpd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_apd(&self) -> super::vals::Pdruncfg1DspcacheRegfApd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg1DspcacheRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_apd(&mut self, val: super::vals::Pdruncfg1DspcacheRegfApd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_ppd(&self) -> super::vals::Pdruncfg1DspcacheRegfPpd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg1DspcacheRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_ppd(&mut self, val: super::vals::Pdruncfg1DspcacheRegfPpd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_apd(&self) -> super::vals::Pdruncfg1DsptcmRegfApd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg1DsptcmRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_apd(&mut self, val: super::vals::Pdruncfg1DsptcmRegfApd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_ppd(&self) -> super::vals::Pdruncfg1DsptcmRegfPpd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg1DsptcmRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_ppd(&mut self, val: super::vals::Pdruncfg1DsptcmRegfPpd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "array power and periph power"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_pd(&self) -> super::vals::Pdruncfg1RomPd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg1RomPd::from_bits(val as u8)
    }
    #[doc = "array power and periph power"]
    #[inline(always)]
    pub const fn set_rom_pd(&mut self, val: super::vals::Pdruncfg1RomPd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Needed when vddcore can be smaller than 0"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sleep(&self) -> super::vals::Pdruncfg1SramSleep {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdruncfg1SramSleep::from_bits(val as u8)
    }
    #[doc = "Needed when vddcore can be smaller than 0"]
    #[inline(always)]
    pub const fn set_sram_sleep(&mut self, val: super::vals::Pdruncfg1SramSleep) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdruncfg1 {
    #[inline(always)]
    fn default() -> Pdruncfg1 {
        Pdruncfg1(0)
    }
}
impl core::fmt::Debug for Pdruncfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg1")
            .field("pq_sram_apd", &self.pq_sram_apd())
            .field("pq_sram_ppd", &self.pq_sram_ppd())
            .field("flexspi_sram_apd", &self.flexspi_sram_apd())
            .field("flexspi_sram_ppd", &self.flexspi_sram_ppd())
            .field("usbhs_sram_apd", &self.usbhs_sram_apd())
            .field("usbhs_sram_ppd", &self.usbhs_sram_ppd())
            .field("usdhc0_sram_apd", &self.usdhc0_sram_apd())
            .field("usdhc0_sram_ppd", &self.usdhc0_sram_ppd())
            .field("usdhc1_sram_apd", &self.usdhc1_sram_apd())
            .field("usdhc1_sram_ppd", &self.usdhc1_sram_ppd())
            .field("casper_sram_apd", &self.casper_sram_apd())
            .field("casper_sram_ppd", &self.casper_sram_ppd())
            .field("dspcache_regf_apd", &self.dspcache_regf_apd())
            .field("dspcache_regf_ppd", &self.dspcache_regf_ppd())
            .field("dsptcm_regf_apd", &self.dsptcm_regf_apd())
            .field("dsptcm_regf_ppd", &self.dsptcm_regf_ppd())
            .field("rom_pd", &self.rom_pd())
            .field("sram_sleep", &self.sram_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg1 {{ pq_sram_apd: {:?}, pq_sram_ppd: {:?}, flexspi_sram_apd: {:?}, flexspi_sram_ppd: {:?}, usbhs_sram_apd: {:?}, usbhs_sram_ppd: {:?}, usdhc0_sram_apd: {:?}, usdhc0_sram_ppd: {:?}, usdhc1_sram_apd: {:?}, usdhc1_sram_ppd: {:?}, casper_sram_apd: {:?}, casper_sram_ppd: {:?}, dspcache_regf_apd: {:?}, dspcache_regf_ppd: {:?}, dsptcm_regf_apd: {:?}, dsptcm_regf_ppd: {:?}, rom_pd: {:?}, sram_sleep: {:?} }}",
            self.pq_sram_apd(),
            self.pq_sram_ppd(),
            self.flexspi_sram_apd(),
            self.flexspi_sram_ppd(),
            self.usbhs_sram_apd(),
            self.usbhs_sram_ppd(),
            self.usdhc0_sram_apd(),
            self.usdhc0_sram_ppd(),
            self.usdhc1_sram_apd(),
            self.usdhc1_sram_ppd(),
            self.casper_sram_apd(),
            self.casper_sram_ppd(),
            self.dspcache_regf_apd(),
            self.dspcache_regf_ppd(),
            self.dsptcm_regf_apd(),
            self.dsptcm_regf_ppd(),
            self.rom_pd(),
            self.sram_sleep()
        )
    }
}
#[doc = "Run configuration 1 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg1Clr(pub u32);
impl Pdruncfg1Clr {
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_apd(&self) -> super::vals::Pdruncfg1ClrPqSramApd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg1ClrPqSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_pq_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrPqSramApd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_ppd(&self) -> super::vals::Pdruncfg1ClrPqSramPpd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg1ClrPqSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_pq_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrPqSramPpd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_apd(&self) -> super::vals::Pdruncfg1ClrFlexspiSramApd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg1ClrFlexspiSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrFlexspiSramApd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_ppd(&self) -> super::vals::Pdruncfg1ClrFlexspiSramPpd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg1ClrFlexspiSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrFlexspiSramPpd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_apd(&self) -> super::vals::Pdruncfg1ClrUsbhsSramApd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg1ClrUsbhsSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrUsbhsSramApd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_ppd(&self) -> super::vals::Pdruncfg1ClrUsbhsSramPpd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg1ClrUsbhsSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrUsbhsSramPpd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_apd(&self) -> super::vals::Pdruncfg1ClrUsdhc0SramApd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg1ClrUsdhc0SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrUsdhc0SramApd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_ppd(&self) -> super::vals::Pdruncfg1ClrUsdhc0SramPpd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg1ClrUsdhc0SramPpd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrUsdhc0SramPpd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_apd(&self) -> super::vals::Pdruncfg1ClrUsdhc1SramApd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg1ClrUsdhc1SramApd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrUsdhc1SramApd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_ppd(&self) -> super::vals::Pdruncfg1ClrUsdhc1SramPpd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg1ClrUsdhc1SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrUsdhc1SramPpd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_apd(&self) -> super::vals::Pdruncfg1ClrCasperSramApd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg1ClrCasperSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_casper_sram_apd(&mut self, val: super::vals::Pdruncfg1ClrCasperSramApd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_ppd(&self) -> super::vals::Pdruncfg1ClrCasperSramPpd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg1ClrCasperSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_casper_sram_ppd(&mut self, val: super::vals::Pdruncfg1ClrCasperSramPpd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_apd(&self) -> super::vals::Pdruncfg1ClrDspcacheRegfApd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg1ClrDspcacheRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_apd(&mut self, val: super::vals::Pdruncfg1ClrDspcacheRegfApd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_ppd(&self) -> super::vals::Pdruncfg1ClrDspcacheRegfPpd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg1ClrDspcacheRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_ppd(&mut self, val: super::vals::Pdruncfg1ClrDspcacheRegfPpd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_apd(&self) -> super::vals::Pdruncfg1ClrDsptcmRegfApd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg1ClrDsptcmRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_apd(&mut self, val: super::vals::Pdruncfg1ClrDsptcmRegfApd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_ppd(&self) -> super::vals::Pdruncfg1ClrDsptcmRegfPpd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg1ClrDsptcmRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_ppd(&mut self, val: super::vals::Pdruncfg1ClrDsptcmRegfPpd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "array power and periph power"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_pd(&self) -> super::vals::Pdruncfg1ClrRomPd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg1ClrRomPd::from_bits(val as u8)
    }
    #[doc = "array power and periph power"]
    #[inline(always)]
    pub const fn set_rom_pd(&mut self, val: super::vals::Pdruncfg1ClrRomPd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sleep(&self) -> super::vals::Pdruncfg1ClrSramSleep {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdruncfg1ClrSramSleep::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram_sleep(&mut self, val: super::vals::Pdruncfg1ClrSramSleep) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdruncfg1Clr {
    #[inline(always)]
    fn default() -> Pdruncfg1Clr {
        Pdruncfg1Clr(0)
    }
}
impl core::fmt::Debug for Pdruncfg1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg1Clr")
            .field("pq_sram_apd", &self.pq_sram_apd())
            .field("pq_sram_ppd", &self.pq_sram_ppd())
            .field("flexspi_sram_apd", &self.flexspi_sram_apd())
            .field("flexspi_sram_ppd", &self.flexspi_sram_ppd())
            .field("usbhs_sram_apd", &self.usbhs_sram_apd())
            .field("usbhs_sram_ppd", &self.usbhs_sram_ppd())
            .field("usdhc0_sram_apd", &self.usdhc0_sram_apd())
            .field("usdhc0_sram_ppd", &self.usdhc0_sram_ppd())
            .field("usdhc1_sram_apd", &self.usdhc1_sram_apd())
            .field("usdhc1_sram_ppd", &self.usdhc1_sram_ppd())
            .field("casper_sram_apd", &self.casper_sram_apd())
            .field("casper_sram_ppd", &self.casper_sram_ppd())
            .field("dspcache_regf_apd", &self.dspcache_regf_apd())
            .field("dspcache_regf_ppd", &self.dspcache_regf_ppd())
            .field("dsptcm_regf_apd", &self.dsptcm_regf_apd())
            .field("dsptcm_regf_ppd", &self.dsptcm_regf_ppd())
            .field("rom_pd", &self.rom_pd())
            .field("sram_sleep", &self.sram_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg1Clr {{ pq_sram_apd: {:?}, pq_sram_ppd: {:?}, flexspi_sram_apd: {:?}, flexspi_sram_ppd: {:?}, usbhs_sram_apd: {:?}, usbhs_sram_ppd: {:?}, usdhc0_sram_apd: {:?}, usdhc0_sram_ppd: {:?}, usdhc1_sram_apd: {:?}, usdhc1_sram_ppd: {:?}, casper_sram_apd: {:?}, casper_sram_ppd: {:?}, dspcache_regf_apd: {:?}, dspcache_regf_ppd: {:?}, dsptcm_regf_apd: {:?}, dsptcm_regf_ppd: {:?}, rom_pd: {:?}, sram_sleep: {:?} }}",
            self.pq_sram_apd(),
            self.pq_sram_ppd(),
            self.flexspi_sram_apd(),
            self.flexspi_sram_ppd(),
            self.usbhs_sram_apd(),
            self.usbhs_sram_ppd(),
            self.usdhc0_sram_apd(),
            self.usdhc0_sram_ppd(),
            self.usdhc1_sram_apd(),
            self.usdhc1_sram_ppd(),
            self.casper_sram_apd(),
            self.casper_sram_ppd(),
            self.dspcache_regf_apd(),
            self.dspcache_regf_ppd(),
            self.dsptcm_regf_apd(),
            self.dsptcm_regf_ppd(),
            self.rom_pd(),
            self.sram_sleep()
        )
    }
}
#[doc = "Run configuration 1 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg1Set(pub u32);
impl Pdruncfg1Set {
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_apd(&self) -> super::vals::Pdruncfg1SetPqSramApd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg1SetPqSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_pq_sram_apd(&mut self, val: super::vals::Pdruncfg1SetPqSramApd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_ppd(&self) -> super::vals::Pdruncfg1SetPqSramPpd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg1SetPqSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_pq_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetPqSramPpd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_apd(&self) -> super::vals::Pdruncfg1SetFlexspiSramApd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg1SetFlexspiSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_apd(&mut self, val: super::vals::Pdruncfg1SetFlexspiSramApd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_ppd(&self) -> super::vals::Pdruncfg1SetFlexspiSramPpd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg1SetFlexspiSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetFlexspiSramPpd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_apd(&self) -> super::vals::Pdruncfg1SetUsbhsSramApd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg1SetUsbhsSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_apd(&mut self, val: super::vals::Pdruncfg1SetUsbhsSramApd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_ppd(&self) -> super::vals::Pdruncfg1SetUsbhsSramPpd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg1SetUsbhsSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetUsbhsSramPpd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_apd(&self) -> super::vals::Pdruncfg1SetUsdhc0SramApd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg1SetUsdhc0SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_apd(&mut self, val: super::vals::Pdruncfg1SetUsdhc0SramApd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_ppd(&self) -> super::vals::Pdruncfg1SetUsdhc0SramPpd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg1SetUsdhc0SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetUsdhc0SramPpd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_apd(&self) -> super::vals::Pdruncfg1SetUsdhc1SramApd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg1SetUsdhc1SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_apd(&mut self, val: super::vals::Pdruncfg1SetUsdhc1SramApd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_ppd(&self) -> super::vals::Pdruncfg1SetUsdhc1SramPpd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg1SetUsdhc1SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetUsdhc1SramPpd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_apd(&self) -> super::vals::Pdruncfg1SetCasperSramApd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg1SetCasperSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_casper_sram_apd(&mut self, val: super::vals::Pdruncfg1SetCasperSramApd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_ppd(&self) -> super::vals::Pdruncfg1SetCasperSramPpd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg1SetCasperSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_casper_sram_ppd(&mut self, val: super::vals::Pdruncfg1SetCasperSramPpd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_apd(&self) -> super::vals::Pdruncfg1SetDspcacheRegfApd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg1SetDspcacheRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_apd(&mut self, val: super::vals::Pdruncfg1SetDspcacheRegfApd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_ppd(&self) -> super::vals::Pdruncfg1SetDspcacheRegfPpd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg1SetDspcacheRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_ppd(&mut self, val: super::vals::Pdruncfg1SetDspcacheRegfPpd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_apd(&self) -> super::vals::Pdruncfg1SetDsptcmRegfApd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg1SetDsptcmRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_apd(&mut self, val: super::vals::Pdruncfg1SetDsptcmRegfApd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_ppd(&self) -> super::vals::Pdruncfg1SetDsptcmRegfPpd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg1SetDsptcmRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_ppd(&mut self, val: super::vals::Pdruncfg1SetDsptcmRegfPpd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "array power and periph power"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_pd(&self) -> super::vals::Pdruncfg1SetRomPd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg1SetRomPd::from_bits(val as u8)
    }
    #[doc = "array power and periph power"]
    #[inline(always)]
    pub const fn set_rom_pd(&mut self, val: super::vals::Pdruncfg1SetRomPd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sleep(&self) -> super::vals::Pdruncfg1SetSramSleep {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdruncfg1SetSramSleep::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sram_sleep(&mut self, val: super::vals::Pdruncfg1SetSramSleep) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdruncfg1Set {
    #[inline(always)]
    fn default() -> Pdruncfg1Set {
        Pdruncfg1Set(0)
    }
}
impl core::fmt::Debug for Pdruncfg1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg1Set")
            .field("pq_sram_apd", &self.pq_sram_apd())
            .field("pq_sram_ppd", &self.pq_sram_ppd())
            .field("flexspi_sram_apd", &self.flexspi_sram_apd())
            .field("flexspi_sram_ppd", &self.flexspi_sram_ppd())
            .field("usbhs_sram_apd", &self.usbhs_sram_apd())
            .field("usbhs_sram_ppd", &self.usbhs_sram_ppd())
            .field("usdhc0_sram_apd", &self.usdhc0_sram_apd())
            .field("usdhc0_sram_ppd", &self.usdhc0_sram_ppd())
            .field("usdhc1_sram_apd", &self.usdhc1_sram_apd())
            .field("usdhc1_sram_ppd", &self.usdhc1_sram_ppd())
            .field("casper_sram_apd", &self.casper_sram_apd())
            .field("casper_sram_ppd", &self.casper_sram_ppd())
            .field("dspcache_regf_apd", &self.dspcache_regf_apd())
            .field("dspcache_regf_ppd", &self.dspcache_regf_ppd())
            .field("dsptcm_regf_apd", &self.dsptcm_regf_apd())
            .field("dsptcm_regf_ppd", &self.dsptcm_regf_ppd())
            .field("rom_pd", &self.rom_pd())
            .field("sram_sleep", &self.sram_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg1Set {{ pq_sram_apd: {:?}, pq_sram_ppd: {:?}, flexspi_sram_apd: {:?}, flexspi_sram_ppd: {:?}, usbhs_sram_apd: {:?}, usbhs_sram_ppd: {:?}, usdhc0_sram_apd: {:?}, usdhc0_sram_ppd: {:?}, usdhc1_sram_apd: {:?}, usdhc1_sram_ppd: {:?}, casper_sram_apd: {:?}, casper_sram_ppd: {:?}, dspcache_regf_apd: {:?}, dspcache_regf_ppd: {:?}, dsptcm_regf_apd: {:?}, dsptcm_regf_ppd: {:?}, rom_pd: {:?}, sram_sleep: {:?} }}",
            self.pq_sram_apd(),
            self.pq_sram_ppd(),
            self.flexspi_sram_apd(),
            self.flexspi_sram_ppd(),
            self.usbhs_sram_apd(),
            self.usbhs_sram_ppd(),
            self.usdhc0_sram_apd(),
            self.usdhc0_sram_ppd(),
            self.usdhc1_sram_apd(),
            self.usdhc1_sram_ppd(),
            self.casper_sram_apd(),
            self.casper_sram_ppd(),
            self.dspcache_regf_apd(),
            self.dspcache_regf_ppd(),
            self.dsptcm_regf_apd(),
            self.dsptcm_regf_ppd(),
            self.rom_pd(),
            self.sram_sleep()
        )
    }
}
#[doc = "Run configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg2(pub u32);
impl Pdruncfg2 {
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_apd(&self) -> super::vals::Pdruncfg2SramIf0Apd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg2SramIf0Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if0_apd(&mut self, val: super::vals::Pdruncfg2SramIf0Apd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_apd(&self) -> super::vals::Pdruncfg2SramIf1Apd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg2SramIf1Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if1_apd(&mut self, val: super::vals::Pdruncfg2SramIf1Apd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_apd(&self) -> super::vals::Pdruncfg2SramIf2Apd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg2SramIf2Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if2_apd(&mut self, val: super::vals::Pdruncfg2SramIf2Apd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_apd(&self) -> super::vals::Pdruncfg2SramIf3Apd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg2SramIf3Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if3_apd(&mut self, val: super::vals::Pdruncfg2SramIf3Apd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_apd(&self) -> super::vals::Pdruncfg2SramIf4Apd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg2SramIf4Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if4_apd(&mut self, val: super::vals::Pdruncfg2SramIf4Apd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_apd(&self) -> super::vals::Pdruncfg2SramIf5Apd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg2SramIf5Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if5_apd(&mut self, val: super::vals::Pdruncfg2SramIf5Apd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_apd(&self) -> super::vals::Pdruncfg2SramIf6Apd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg2SramIf6Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if6_apd(&mut self, val: super::vals::Pdruncfg2SramIf6Apd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_apd(&self) -> super::vals::Pdruncfg2SramIf7Apd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg2SramIf7Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if7_apd(&mut self, val: super::vals::Pdruncfg2SramIf7Apd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_apd(&self) -> super::vals::Pdruncfg2SramIf8Apd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg2SramIf8Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if8_apd(&mut self, val: super::vals::Pdruncfg2SramIf8Apd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_apd(&self) -> super::vals::Pdruncfg2SramIf9Apd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg2SramIf9Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if9_apd(&mut self, val: super::vals::Pdruncfg2SramIf9Apd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_apd(&self) -> super::vals::Pdruncfg2SramIf10Apd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg2SramIf10Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if10_apd(&mut self, val: super::vals::Pdruncfg2SramIf10Apd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_apd(&self) -> super::vals::Pdruncfg2SramIf11Apd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg2SramIf11Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if11_apd(&mut self, val: super::vals::Pdruncfg2SramIf11Apd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_apd(&self) -> super::vals::Pdruncfg2SramIf12Apd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg2SramIf12Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if12_apd(&mut self, val: super::vals::Pdruncfg2SramIf12Apd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_apd(&self) -> super::vals::Pdruncfg2SramIf13Apd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg2SramIf13Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if13_apd(&mut self, val: super::vals::Pdruncfg2SramIf13Apd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_apd(&self) -> super::vals::Pdruncfg2SramIf14Apd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg2SramIf14Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if14_apd(&mut self, val: super::vals::Pdruncfg2SramIf14Apd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_apd(&self) -> super::vals::Pdruncfg2SramIf15Apd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg2SramIf15Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if15_apd(&mut self, val: super::vals::Pdruncfg2SramIf15Apd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_apd(&self) -> super::vals::Pdruncfg2SramIf16Apd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg2SramIf16Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if16_apd(&mut self, val: super::vals::Pdruncfg2SramIf16Apd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_apd(&self) -> super::vals::Pdruncfg2SramIf17Apd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg2SramIf17Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if17_apd(&mut self, val: super::vals::Pdruncfg2SramIf17Apd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_apd(&self) -> super::vals::Pdruncfg2SramIf18Apd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg2SramIf18Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if18_apd(&mut self, val: super::vals::Pdruncfg2SramIf18Apd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_apd(&self) -> super::vals::Pdruncfg2SramIf19Apd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg2SramIf19Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if19_apd(&mut self, val: super::vals::Pdruncfg2SramIf19Apd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_apd(&self) -> super::vals::Pdruncfg2SramIf20Apd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg2SramIf20Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if20_apd(&mut self, val: super::vals::Pdruncfg2SramIf20Apd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_apd(&self) -> super::vals::Pdruncfg2SramIf21Apd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg2SramIf21Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if21_apd(&mut self, val: super::vals::Pdruncfg2SramIf21Apd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_apd(&self) -> super::vals::Pdruncfg2SramIf22Apd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg2SramIf22Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if22_apd(&mut self, val: super::vals::Pdruncfg2SramIf22Apd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_apd(&self) -> super::vals::Pdruncfg2SramIf23Apd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg2SramIf23Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if23_apd(&mut self, val: super::vals::Pdruncfg2SramIf23Apd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_apd(&self) -> super::vals::Pdruncfg2SramIf24Apd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg2SramIf24Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if24_apd(&mut self, val: super::vals::Pdruncfg2SramIf24Apd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_apd(&self) -> super::vals::Pdruncfg2SramIf25Apd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg2SramIf25Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if25_apd(&mut self, val: super::vals::Pdruncfg2SramIf25Apd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_apd(&self) -> super::vals::Pdruncfg2SramIf26Apd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg2SramIf26Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if26_apd(&mut self, val: super::vals::Pdruncfg2SramIf26Apd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_apd(&self) -> super::vals::Pdruncfg2SramIf27Apd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg2SramIf27Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if27_apd(&mut self, val: super::vals::Pdruncfg2SramIf27Apd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_apd(&self) -> super::vals::Pdruncfg2SramIf28Apd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg2SramIf28Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if28_apd(&mut self, val: super::vals::Pdruncfg2SramIf28Apd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_apd(&self) -> super::vals::Pdruncfg2SramIf29Apd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg2SramIf29Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if29_apd(&mut self, val: super::vals::Pdruncfg2SramIf29Apd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg2 {
    #[inline(always)]
    fn default() -> Pdruncfg2 {
        Pdruncfg2(0)
    }
}
impl core::fmt::Debug for Pdruncfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg2")
            .field("sram_if0_apd", &self.sram_if0_apd())
            .field("sram_if1_apd", &self.sram_if1_apd())
            .field("sram_if2_apd", &self.sram_if2_apd())
            .field("sram_if3_apd", &self.sram_if3_apd())
            .field("sram_if4_apd", &self.sram_if4_apd())
            .field("sram_if5_apd", &self.sram_if5_apd())
            .field("sram_if6_apd", &self.sram_if6_apd())
            .field("sram_if7_apd", &self.sram_if7_apd())
            .field("sram_if8_apd", &self.sram_if8_apd())
            .field("sram_if9_apd", &self.sram_if9_apd())
            .field("sram_if10_apd", &self.sram_if10_apd())
            .field("sram_if11_apd", &self.sram_if11_apd())
            .field("sram_if12_apd", &self.sram_if12_apd())
            .field("sram_if13_apd", &self.sram_if13_apd())
            .field("sram_if14_apd", &self.sram_if14_apd())
            .field("sram_if15_apd", &self.sram_if15_apd())
            .field("sram_if16_apd", &self.sram_if16_apd())
            .field("sram_if17_apd", &self.sram_if17_apd())
            .field("sram_if18_apd", &self.sram_if18_apd())
            .field("sram_if19_apd", &self.sram_if19_apd())
            .field("sram_if20_apd", &self.sram_if20_apd())
            .field("sram_if21_apd", &self.sram_if21_apd())
            .field("sram_if22_apd", &self.sram_if22_apd())
            .field("sram_if23_apd", &self.sram_if23_apd())
            .field("sram_if24_apd", &self.sram_if24_apd())
            .field("sram_if25_apd", &self.sram_if25_apd())
            .field("sram_if26_apd", &self.sram_if26_apd())
            .field("sram_if27_apd", &self.sram_if27_apd())
            .field("sram_if28_apd", &self.sram_if28_apd())
            .field("sram_if29_apd", &self.sram_if29_apd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg2 {{ sram_if0_apd: {:?}, sram_if1_apd: {:?}, sram_if2_apd: {:?}, sram_if3_apd: {:?}, sram_if4_apd: {:?}, sram_if5_apd: {:?}, sram_if6_apd: {:?}, sram_if7_apd: {:?}, sram_if8_apd: {:?}, sram_if9_apd: {:?}, sram_if10_apd: {:?}, sram_if11_apd: {:?}, sram_if12_apd: {:?}, sram_if13_apd: {:?}, sram_if14_apd: {:?}, sram_if15_apd: {:?}, sram_if16_apd: {:?}, sram_if17_apd: {:?}, sram_if18_apd: {:?}, sram_if19_apd: {:?}, sram_if20_apd: {:?}, sram_if21_apd: {:?}, sram_if22_apd: {:?}, sram_if23_apd: {:?}, sram_if24_apd: {:?}, sram_if25_apd: {:?}, sram_if26_apd: {:?}, sram_if27_apd: {:?}, sram_if28_apd: {:?}, sram_if29_apd: {:?} }}",
            self.sram_if0_apd(),
            self.sram_if1_apd(),
            self.sram_if2_apd(),
            self.sram_if3_apd(),
            self.sram_if4_apd(),
            self.sram_if5_apd(),
            self.sram_if6_apd(),
            self.sram_if7_apd(),
            self.sram_if8_apd(),
            self.sram_if9_apd(),
            self.sram_if10_apd(),
            self.sram_if11_apd(),
            self.sram_if12_apd(),
            self.sram_if13_apd(),
            self.sram_if14_apd(),
            self.sram_if15_apd(),
            self.sram_if16_apd(),
            self.sram_if17_apd(),
            self.sram_if18_apd(),
            self.sram_if19_apd(),
            self.sram_if20_apd(),
            self.sram_if21_apd(),
            self.sram_if22_apd(),
            self.sram_if23_apd(),
            self.sram_if24_apd(),
            self.sram_if25_apd(),
            self.sram_if26_apd(),
            self.sram_if27_apd(),
            self.sram_if28_apd(),
            self.sram_if29_apd()
        )
    }
}
#[doc = "Run configuration 2 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg2Clr(pub u32);
impl Pdruncfg2Clr {
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_apd(&self) -> super::vals::Pdruncfg2ClrSramIf0Apd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf0Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if0_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf0Apd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_apd(&self) -> super::vals::Pdruncfg2ClrSramIf1Apd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf1Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if1_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf1Apd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_apd(&self) -> super::vals::Pdruncfg2ClrSramIf2Apd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf2Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if2_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf2Apd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_apd(&self) -> super::vals::Pdruncfg2ClrSramIf3Apd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf3Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if3_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf3Apd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_apd(&self) -> super::vals::Pdruncfg2ClrSramIf4Apd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf4Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if4_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf4Apd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_apd(&self) -> super::vals::Pdruncfg2ClrSramIf5Apd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf5Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if5_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf5Apd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_apd(&self) -> super::vals::Pdruncfg2ClrSramIf6Apd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf6Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if6_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf6Apd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_apd(&self) -> super::vals::Pdruncfg2ClrSramIf7Apd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf7Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if7_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf7Apd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_apd(&self) -> super::vals::Pdruncfg2ClrSramIf8Apd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf8Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if8_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf8Apd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_apd(&self) -> super::vals::Pdruncfg2ClrSramIf9Apd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf9Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if9_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf9Apd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_apd(&self) -> super::vals::Pdruncfg2ClrSramIf10Apd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf10Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if10_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf10Apd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_apd(&self) -> super::vals::Pdruncfg2ClrSramIf11Apd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf11Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if11_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf11Apd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_apd(&self) -> super::vals::Pdruncfg2ClrSramIf12Apd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf12Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if12_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf12Apd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_apd(&self) -> super::vals::Pdruncfg2ClrSramIf13Apd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf13Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if13_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf13Apd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_apd(&self) -> super::vals::Pdruncfg2ClrSramIf14Apd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf14Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if14_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf14Apd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_apd(&self) -> super::vals::Pdruncfg2ClrSramIf15Apd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf15Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if15_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf15Apd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_apd(&self) -> super::vals::Pdruncfg2ClrSramIf16Apd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf16Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if16_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf16Apd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_apd(&self) -> super::vals::Pdruncfg2ClrSramIf17Apd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf17Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if17_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf17Apd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_apd(&self) -> super::vals::Pdruncfg2ClrSramIf18Apd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf18Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if18_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf18Apd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_apd(&self) -> super::vals::Pdruncfg2ClrSramIf19Apd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf19Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if19_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf19Apd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_apd(&self) -> super::vals::Pdruncfg2ClrSramIf20Apd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf20Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if20_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf20Apd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_apd(&self) -> super::vals::Pdruncfg2ClrSramIf21Apd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf21Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if21_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf21Apd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_apd(&self) -> super::vals::Pdruncfg2ClrSramIf22Apd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf22Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if22_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf22Apd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_apd(&self) -> super::vals::Pdruncfg2ClrSramIf23Apd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf23Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if23_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf23Apd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_apd(&self) -> super::vals::Pdruncfg2ClrSramIf24Apd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf24Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if24_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf24Apd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_apd(&self) -> super::vals::Pdruncfg2ClrSramIf25Apd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf25Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if25_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf25Apd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_apd(&self) -> super::vals::Pdruncfg2ClrSramIf26Apd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf26Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if26_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf26Apd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_apd(&self) -> super::vals::Pdruncfg2ClrSramIf27Apd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf27Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if27_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf27Apd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_apd(&self) -> super::vals::Pdruncfg2ClrSramIf28Apd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf28Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if28_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf28Apd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_apd(&self) -> super::vals::Pdruncfg2ClrSramIf29Apd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg2ClrSramIf29Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if29_apd(&mut self, val: super::vals::Pdruncfg2ClrSramIf29Apd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg2Clr {
    #[inline(always)]
    fn default() -> Pdruncfg2Clr {
        Pdruncfg2Clr(0)
    }
}
impl core::fmt::Debug for Pdruncfg2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg2Clr")
            .field("sram_if0_apd", &self.sram_if0_apd())
            .field("sram_if1_apd", &self.sram_if1_apd())
            .field("sram_if2_apd", &self.sram_if2_apd())
            .field("sram_if3_apd", &self.sram_if3_apd())
            .field("sram_if4_apd", &self.sram_if4_apd())
            .field("sram_if5_apd", &self.sram_if5_apd())
            .field("sram_if6_apd", &self.sram_if6_apd())
            .field("sram_if7_apd", &self.sram_if7_apd())
            .field("sram_if8_apd", &self.sram_if8_apd())
            .field("sram_if9_apd", &self.sram_if9_apd())
            .field("sram_if10_apd", &self.sram_if10_apd())
            .field("sram_if11_apd", &self.sram_if11_apd())
            .field("sram_if12_apd", &self.sram_if12_apd())
            .field("sram_if13_apd", &self.sram_if13_apd())
            .field("sram_if14_apd", &self.sram_if14_apd())
            .field("sram_if15_apd", &self.sram_if15_apd())
            .field("sram_if16_apd", &self.sram_if16_apd())
            .field("sram_if17_apd", &self.sram_if17_apd())
            .field("sram_if18_apd", &self.sram_if18_apd())
            .field("sram_if19_apd", &self.sram_if19_apd())
            .field("sram_if20_apd", &self.sram_if20_apd())
            .field("sram_if21_apd", &self.sram_if21_apd())
            .field("sram_if22_apd", &self.sram_if22_apd())
            .field("sram_if23_apd", &self.sram_if23_apd())
            .field("sram_if24_apd", &self.sram_if24_apd())
            .field("sram_if25_apd", &self.sram_if25_apd())
            .field("sram_if26_apd", &self.sram_if26_apd())
            .field("sram_if27_apd", &self.sram_if27_apd())
            .field("sram_if28_apd", &self.sram_if28_apd())
            .field("sram_if29_apd", &self.sram_if29_apd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg2Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg2Clr {{ sram_if0_apd: {:?}, sram_if1_apd: {:?}, sram_if2_apd: {:?}, sram_if3_apd: {:?}, sram_if4_apd: {:?}, sram_if5_apd: {:?}, sram_if6_apd: {:?}, sram_if7_apd: {:?}, sram_if8_apd: {:?}, sram_if9_apd: {:?}, sram_if10_apd: {:?}, sram_if11_apd: {:?}, sram_if12_apd: {:?}, sram_if13_apd: {:?}, sram_if14_apd: {:?}, sram_if15_apd: {:?}, sram_if16_apd: {:?}, sram_if17_apd: {:?}, sram_if18_apd: {:?}, sram_if19_apd: {:?}, sram_if20_apd: {:?}, sram_if21_apd: {:?}, sram_if22_apd: {:?}, sram_if23_apd: {:?}, sram_if24_apd: {:?}, sram_if25_apd: {:?}, sram_if26_apd: {:?}, sram_if27_apd: {:?}, sram_if28_apd: {:?}, sram_if29_apd: {:?} }}",
            self.sram_if0_apd(),
            self.sram_if1_apd(),
            self.sram_if2_apd(),
            self.sram_if3_apd(),
            self.sram_if4_apd(),
            self.sram_if5_apd(),
            self.sram_if6_apd(),
            self.sram_if7_apd(),
            self.sram_if8_apd(),
            self.sram_if9_apd(),
            self.sram_if10_apd(),
            self.sram_if11_apd(),
            self.sram_if12_apd(),
            self.sram_if13_apd(),
            self.sram_if14_apd(),
            self.sram_if15_apd(),
            self.sram_if16_apd(),
            self.sram_if17_apd(),
            self.sram_if18_apd(),
            self.sram_if19_apd(),
            self.sram_if20_apd(),
            self.sram_if21_apd(),
            self.sram_if22_apd(),
            self.sram_if23_apd(),
            self.sram_if24_apd(),
            self.sram_if25_apd(),
            self.sram_if26_apd(),
            self.sram_if27_apd(),
            self.sram_if28_apd(),
            self.sram_if29_apd()
        )
    }
}
#[doc = "Run configuration 2 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg2Set(pub u32);
impl Pdruncfg2Set {
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_apd(&self) -> super::vals::Pdruncfg2SetSramIf0Apd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf0Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if0_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf0Apd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_apd(&self) -> super::vals::Pdruncfg2SetSramIf1Apd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf1Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if1_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf1Apd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_apd(&self) -> super::vals::Pdruncfg2SetSramIf2Apd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf2Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if2_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf2Apd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_apd(&self) -> super::vals::Pdruncfg2SetSramIf3Apd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf3Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if3_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf3Apd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_apd(&self) -> super::vals::Pdruncfg2SetSramIf4Apd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf4Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if4_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf4Apd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_apd(&self) -> super::vals::Pdruncfg2SetSramIf5Apd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf5Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if5_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf5Apd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_apd(&self) -> super::vals::Pdruncfg2SetSramIf6Apd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf6Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if6_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf6Apd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_apd(&self) -> super::vals::Pdruncfg2SetSramIf7Apd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf7Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if7_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf7Apd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_apd(&self) -> super::vals::Pdruncfg2SetSramIf8Apd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf8Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if8_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf8Apd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_apd(&self) -> super::vals::Pdruncfg2SetSramIf9Apd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf9Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if9_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf9Apd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_apd(&self) -> super::vals::Pdruncfg2SetSramIf10Apd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf10Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if10_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf10Apd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_apd(&self) -> super::vals::Pdruncfg2SetSramIf11Apd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf11Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if11_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf11Apd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_apd(&self) -> super::vals::Pdruncfg2SetSramIf12Apd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf12Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if12_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf12Apd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_apd(&self) -> super::vals::Pdruncfg2SetSramIf13Apd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf13Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if13_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf13Apd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_apd(&self) -> super::vals::Pdruncfg2SetSramIf14Apd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf14Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if14_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf14Apd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_apd(&self) -> super::vals::Pdruncfg2SetSramIf15Apd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf15Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if15_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf15Apd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_apd(&self) -> super::vals::Pdruncfg2SetSramIf16Apd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf16Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if16_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf16Apd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_apd(&self) -> super::vals::Pdruncfg2SetSramIf17Apd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf17Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if17_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf17Apd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_apd(&self) -> super::vals::Pdruncfg2SetSramIf18Apd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf18Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if18_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf18Apd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_apd(&self) -> super::vals::Pdruncfg2SetSramIf19Apd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf19Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if19_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf19Apd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_apd(&self) -> super::vals::Pdruncfg2SetSramIf20Apd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf20Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if20_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf20Apd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_apd(&self) -> super::vals::Pdruncfg2SetSramIf21Apd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf21Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if21_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf21Apd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_apd(&self) -> super::vals::Pdruncfg2SetSramIf22Apd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf22Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if22_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf22Apd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_apd(&self) -> super::vals::Pdruncfg2SetSramIf23Apd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf23Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if23_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf23Apd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_apd(&self) -> super::vals::Pdruncfg2SetSramIf24Apd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf24Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if24_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf24Apd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_apd(&self) -> super::vals::Pdruncfg2SetSramIf25Apd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf25Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if25_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf25Apd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_apd(&self) -> super::vals::Pdruncfg2SetSramIf26Apd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf26Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if26_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf26Apd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_apd(&self) -> super::vals::Pdruncfg2SetSramIf27Apd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf27Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if27_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf27Apd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_apd(&self) -> super::vals::Pdruncfg2SetSramIf28Apd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf28Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if28_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf28Apd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_apd(&self) -> super::vals::Pdruncfg2SetSramIf29Apd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg2SetSramIf29Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if29_apd(&mut self, val: super::vals::Pdruncfg2SetSramIf29Apd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg2Set {
    #[inline(always)]
    fn default() -> Pdruncfg2Set {
        Pdruncfg2Set(0)
    }
}
impl core::fmt::Debug for Pdruncfg2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg2Set")
            .field("sram_if0_apd", &self.sram_if0_apd())
            .field("sram_if1_apd", &self.sram_if1_apd())
            .field("sram_if2_apd", &self.sram_if2_apd())
            .field("sram_if3_apd", &self.sram_if3_apd())
            .field("sram_if4_apd", &self.sram_if4_apd())
            .field("sram_if5_apd", &self.sram_if5_apd())
            .field("sram_if6_apd", &self.sram_if6_apd())
            .field("sram_if7_apd", &self.sram_if7_apd())
            .field("sram_if8_apd", &self.sram_if8_apd())
            .field("sram_if9_apd", &self.sram_if9_apd())
            .field("sram_if10_apd", &self.sram_if10_apd())
            .field("sram_if11_apd", &self.sram_if11_apd())
            .field("sram_if12_apd", &self.sram_if12_apd())
            .field("sram_if13_apd", &self.sram_if13_apd())
            .field("sram_if14_apd", &self.sram_if14_apd())
            .field("sram_if15_apd", &self.sram_if15_apd())
            .field("sram_if16_apd", &self.sram_if16_apd())
            .field("sram_if17_apd", &self.sram_if17_apd())
            .field("sram_if18_apd", &self.sram_if18_apd())
            .field("sram_if19_apd", &self.sram_if19_apd())
            .field("sram_if20_apd", &self.sram_if20_apd())
            .field("sram_if21_apd", &self.sram_if21_apd())
            .field("sram_if22_apd", &self.sram_if22_apd())
            .field("sram_if23_apd", &self.sram_if23_apd())
            .field("sram_if24_apd", &self.sram_if24_apd())
            .field("sram_if25_apd", &self.sram_if25_apd())
            .field("sram_if26_apd", &self.sram_if26_apd())
            .field("sram_if27_apd", &self.sram_if27_apd())
            .field("sram_if28_apd", &self.sram_if28_apd())
            .field("sram_if29_apd", &self.sram_if29_apd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg2Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg2Set {{ sram_if0_apd: {:?}, sram_if1_apd: {:?}, sram_if2_apd: {:?}, sram_if3_apd: {:?}, sram_if4_apd: {:?}, sram_if5_apd: {:?}, sram_if6_apd: {:?}, sram_if7_apd: {:?}, sram_if8_apd: {:?}, sram_if9_apd: {:?}, sram_if10_apd: {:?}, sram_if11_apd: {:?}, sram_if12_apd: {:?}, sram_if13_apd: {:?}, sram_if14_apd: {:?}, sram_if15_apd: {:?}, sram_if16_apd: {:?}, sram_if17_apd: {:?}, sram_if18_apd: {:?}, sram_if19_apd: {:?}, sram_if20_apd: {:?}, sram_if21_apd: {:?}, sram_if22_apd: {:?}, sram_if23_apd: {:?}, sram_if24_apd: {:?}, sram_if25_apd: {:?}, sram_if26_apd: {:?}, sram_if27_apd: {:?}, sram_if28_apd: {:?}, sram_if29_apd: {:?} }}",
            self.sram_if0_apd(),
            self.sram_if1_apd(),
            self.sram_if2_apd(),
            self.sram_if3_apd(),
            self.sram_if4_apd(),
            self.sram_if5_apd(),
            self.sram_if6_apd(),
            self.sram_if7_apd(),
            self.sram_if8_apd(),
            self.sram_if9_apd(),
            self.sram_if10_apd(),
            self.sram_if11_apd(),
            self.sram_if12_apd(),
            self.sram_if13_apd(),
            self.sram_if14_apd(),
            self.sram_if15_apd(),
            self.sram_if16_apd(),
            self.sram_if17_apd(),
            self.sram_if18_apd(),
            self.sram_if19_apd(),
            self.sram_if20_apd(),
            self.sram_if21_apd(),
            self.sram_if22_apd(),
            self.sram_if23_apd(),
            self.sram_if24_apd(),
            self.sram_if25_apd(),
            self.sram_if26_apd(),
            self.sram_if27_apd(),
            self.sram_if28_apd(),
            self.sram_if29_apd()
        )
    }
}
#[doc = "Run configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg3(pub u32);
impl Pdruncfg3 {
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_ppd(&self) -> super::vals::Pdruncfg3SramIf0Ppd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg3SramIf0Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if0_ppd(&mut self, val: super::vals::Pdruncfg3SramIf0Ppd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_ppd(&self) -> super::vals::Pdruncfg3SramIf1Ppd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg3SramIf1Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if1_ppd(&mut self, val: super::vals::Pdruncfg3SramIf1Ppd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_ppd(&self) -> super::vals::Pdruncfg3SramIf2Ppd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg3SramIf2Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if2_ppd(&mut self, val: super::vals::Pdruncfg3SramIf2Ppd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_ppd(&self) -> super::vals::Pdruncfg3SramIf3Ppd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg3SramIf3Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if3_ppd(&mut self, val: super::vals::Pdruncfg3SramIf3Ppd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_ppd(&self) -> super::vals::Pdruncfg3SramIf4Ppd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg3SramIf4Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if4_ppd(&mut self, val: super::vals::Pdruncfg3SramIf4Ppd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_ppd(&self) -> super::vals::Pdruncfg3SramIf5Ppd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg3SramIf5Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if5_ppd(&mut self, val: super::vals::Pdruncfg3SramIf5Ppd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_ppd(&self) -> super::vals::Pdruncfg3SramIf6Ppd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg3SramIf6Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if6_ppd(&mut self, val: super::vals::Pdruncfg3SramIf6Ppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_ppd(&self) -> super::vals::Pdruncfg3SramIf7Ppd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg3SramIf7Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if7_ppd(&mut self, val: super::vals::Pdruncfg3SramIf7Ppd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_ppd(&self) -> super::vals::Pdruncfg3SramIf8Ppd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg3SramIf8Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if8_ppd(&mut self, val: super::vals::Pdruncfg3SramIf8Ppd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_ppd(&self) -> super::vals::Pdruncfg3SramIf9Ppd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg3SramIf9Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if9_ppd(&mut self, val: super::vals::Pdruncfg3SramIf9Ppd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_ppd(&self) -> super::vals::Pdruncfg3SramIf10Ppd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg3SramIf10Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if10_ppd(&mut self, val: super::vals::Pdruncfg3SramIf10Ppd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_ppd(&self) -> super::vals::Pdruncfg3SramIf11Ppd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg3SramIf11Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if11_ppd(&mut self, val: super::vals::Pdruncfg3SramIf11Ppd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_ppd(&self) -> super::vals::Pdruncfg3SramIf12Ppd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg3SramIf12Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if12_ppd(&mut self, val: super::vals::Pdruncfg3SramIf12Ppd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_ppd(&self) -> super::vals::Pdruncfg3SramIf13Ppd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg3SramIf13Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if13_ppd(&mut self, val: super::vals::Pdruncfg3SramIf13Ppd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_ppd(&self) -> super::vals::Pdruncfg3SramIf14Ppd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg3SramIf14Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if14_ppd(&mut self, val: super::vals::Pdruncfg3SramIf14Ppd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_ppd(&self) -> super::vals::Pdruncfg3SramIf15Ppd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg3SramIf15Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if15_ppd(&mut self, val: super::vals::Pdruncfg3SramIf15Ppd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_ppd(&self) -> super::vals::Pdruncfg3SramIf16Ppd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg3SramIf16Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if16_ppd(&mut self, val: super::vals::Pdruncfg3SramIf16Ppd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_ppd(&self) -> super::vals::Pdruncfg3SramIf17Ppd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg3SramIf17Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if17_ppd(&mut self, val: super::vals::Pdruncfg3SramIf17Ppd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_ppd(&self) -> super::vals::Pdruncfg3SramIf18Ppd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg3SramIf18Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if18_ppd(&mut self, val: super::vals::Pdruncfg3SramIf18Ppd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_ppd(&self) -> super::vals::Pdruncfg3SramIf19Ppd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg3SramIf19Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if19_ppd(&mut self, val: super::vals::Pdruncfg3SramIf19Ppd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_ppd(&self) -> super::vals::Pdruncfg3SramIf20Ppd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg3SramIf20Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if20_ppd(&mut self, val: super::vals::Pdruncfg3SramIf20Ppd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_ppd(&self) -> super::vals::Pdruncfg3SramIf21Ppd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg3SramIf21Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if21_ppd(&mut self, val: super::vals::Pdruncfg3SramIf21Ppd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_ppd(&self) -> super::vals::Pdruncfg3SramIf22Ppd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg3SramIf22Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if22_ppd(&mut self, val: super::vals::Pdruncfg3SramIf22Ppd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_ppd(&self) -> super::vals::Pdruncfg3SramIf23Ppd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg3SramIf23Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if23_ppd(&mut self, val: super::vals::Pdruncfg3SramIf23Ppd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_ppd(&self) -> super::vals::Pdruncfg3SramIf24Ppd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg3SramIf24Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if24_ppd(&mut self, val: super::vals::Pdruncfg3SramIf24Ppd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_ppd(&self) -> super::vals::Pdruncfg3SramIf25Ppd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg3SramIf25Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if25_ppd(&mut self, val: super::vals::Pdruncfg3SramIf25Ppd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_ppd(&self) -> super::vals::Pdruncfg3SramIf26Ppd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg3SramIf26Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if26_ppd(&mut self, val: super::vals::Pdruncfg3SramIf26Ppd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_ppd(&self) -> super::vals::Pdruncfg3SramIf27Ppd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg3SramIf27Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if27_ppd(&mut self, val: super::vals::Pdruncfg3SramIf27Ppd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_ppd(&self) -> super::vals::Pdruncfg3SramIf28Ppd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg3SramIf28Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if28_ppd(&mut self, val: super::vals::Pdruncfg3SramIf28Ppd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_ppd(&self) -> super::vals::Pdruncfg3SramIf29Ppd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg3SramIf29Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if29_ppd(&mut self, val: super::vals::Pdruncfg3SramIf29Ppd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg3 {
    #[inline(always)]
    fn default() -> Pdruncfg3 {
        Pdruncfg3(0)
    }
}
impl core::fmt::Debug for Pdruncfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg3")
            .field("sram_if0_ppd", &self.sram_if0_ppd())
            .field("sram_if1_ppd", &self.sram_if1_ppd())
            .field("sram_if2_ppd", &self.sram_if2_ppd())
            .field("sram_if3_ppd", &self.sram_if3_ppd())
            .field("sram_if4_ppd", &self.sram_if4_ppd())
            .field("sram_if5_ppd", &self.sram_if5_ppd())
            .field("sram_if6_ppd", &self.sram_if6_ppd())
            .field("sram_if7_ppd", &self.sram_if7_ppd())
            .field("sram_if8_ppd", &self.sram_if8_ppd())
            .field("sram_if9_ppd", &self.sram_if9_ppd())
            .field("sram_if10_ppd", &self.sram_if10_ppd())
            .field("sram_if11_ppd", &self.sram_if11_ppd())
            .field("sram_if12_ppd", &self.sram_if12_ppd())
            .field("sram_if13_ppd", &self.sram_if13_ppd())
            .field("sram_if14_ppd", &self.sram_if14_ppd())
            .field("sram_if15_ppd", &self.sram_if15_ppd())
            .field("sram_if16_ppd", &self.sram_if16_ppd())
            .field("sram_if17_ppd", &self.sram_if17_ppd())
            .field("sram_if18_ppd", &self.sram_if18_ppd())
            .field("sram_if19_ppd", &self.sram_if19_ppd())
            .field("sram_if20_ppd", &self.sram_if20_ppd())
            .field("sram_if21_ppd", &self.sram_if21_ppd())
            .field("sram_if22_ppd", &self.sram_if22_ppd())
            .field("sram_if23_ppd", &self.sram_if23_ppd())
            .field("sram_if24_ppd", &self.sram_if24_ppd())
            .field("sram_if25_ppd", &self.sram_if25_ppd())
            .field("sram_if26_ppd", &self.sram_if26_ppd())
            .field("sram_if27_ppd", &self.sram_if27_ppd())
            .field("sram_if28_ppd", &self.sram_if28_ppd())
            .field("sram_if29_ppd", &self.sram_if29_ppd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg3 {{ sram_if0_ppd: {:?}, sram_if1_ppd: {:?}, sram_if2_ppd: {:?}, sram_if3_ppd: {:?}, sram_if4_ppd: {:?}, sram_if5_ppd: {:?}, sram_if6_ppd: {:?}, sram_if7_ppd: {:?}, sram_if8_ppd: {:?}, sram_if9_ppd: {:?}, sram_if10_ppd: {:?}, sram_if11_ppd: {:?}, sram_if12_ppd: {:?}, sram_if13_ppd: {:?}, sram_if14_ppd: {:?}, sram_if15_ppd: {:?}, sram_if16_ppd: {:?}, sram_if17_ppd: {:?}, sram_if18_ppd: {:?}, sram_if19_ppd: {:?}, sram_if20_ppd: {:?}, sram_if21_ppd: {:?}, sram_if22_ppd: {:?}, sram_if23_ppd: {:?}, sram_if24_ppd: {:?}, sram_if25_ppd: {:?}, sram_if26_ppd: {:?}, sram_if27_ppd: {:?}, sram_if28_ppd: {:?}, sram_if29_ppd: {:?} }}",
            self.sram_if0_ppd(),
            self.sram_if1_ppd(),
            self.sram_if2_ppd(),
            self.sram_if3_ppd(),
            self.sram_if4_ppd(),
            self.sram_if5_ppd(),
            self.sram_if6_ppd(),
            self.sram_if7_ppd(),
            self.sram_if8_ppd(),
            self.sram_if9_ppd(),
            self.sram_if10_ppd(),
            self.sram_if11_ppd(),
            self.sram_if12_ppd(),
            self.sram_if13_ppd(),
            self.sram_if14_ppd(),
            self.sram_if15_ppd(),
            self.sram_if16_ppd(),
            self.sram_if17_ppd(),
            self.sram_if18_ppd(),
            self.sram_if19_ppd(),
            self.sram_if20_ppd(),
            self.sram_if21_ppd(),
            self.sram_if22_ppd(),
            self.sram_if23_ppd(),
            self.sram_if24_ppd(),
            self.sram_if25_ppd(),
            self.sram_if26_ppd(),
            self.sram_if27_ppd(),
            self.sram_if28_ppd(),
            self.sram_if29_ppd()
        )
    }
}
#[doc = "Run configuration 3 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg3Clr(pub u32);
impl Pdruncfg3Clr {
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf0Ppd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf0Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if0_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf0Ppd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf1Ppd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf1Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if1_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf1Ppd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf2Ppd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf2Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if2_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf2Ppd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf3Ppd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf3Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if3_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf3Ppd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf4Ppd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf4Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if4_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf4Ppd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf5Ppd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf5Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if5_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf5Ppd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf6Ppd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf6Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if6_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf6Ppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf7Ppd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf7Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if7_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf7Ppd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf8Ppd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf8Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if8_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf8Ppd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf9Ppd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf9Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if9_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf9Ppd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf10Ppd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf10Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if10_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf10Ppd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf11Ppd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf11Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if11_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf11Ppd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf12Ppd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf12Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if12_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf12Ppd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf13Ppd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf13Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if13_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf13Ppd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf14Ppd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf14Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if14_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf14Ppd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf15Ppd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf15Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if15_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf15Ppd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf16Ppd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf16Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if16_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf16Ppd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf17Ppd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf17Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if17_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf17Ppd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf18Ppd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf18Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if18_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf18Ppd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf19Ppd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf19Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if19_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf19Ppd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf20Ppd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf20Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if20_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf20Ppd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf21Ppd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf21Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if21_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf21Ppd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf22Ppd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf22Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if22_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf22Ppd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf23Ppd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf23Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if23_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf23Ppd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf24Ppd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf24Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if24_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf24Ppd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf25Ppd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf25Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if25_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf25Ppd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf26Ppd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf26Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if26_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf26Ppd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf27Ppd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf27Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if27_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf27Ppd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf28Ppd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf28Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if28_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf28Ppd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_ppd(&self) -> super::vals::Pdruncfg3ClrSramIf29Ppd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg3ClrSramIf29Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if29_ppd(&mut self, val: super::vals::Pdruncfg3ClrSramIf29Ppd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg3Clr {
    #[inline(always)]
    fn default() -> Pdruncfg3Clr {
        Pdruncfg3Clr(0)
    }
}
impl core::fmt::Debug for Pdruncfg3Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg3Clr")
            .field("sram_if0_ppd", &self.sram_if0_ppd())
            .field("sram_if1_ppd", &self.sram_if1_ppd())
            .field("sram_if2_ppd", &self.sram_if2_ppd())
            .field("sram_if3_ppd", &self.sram_if3_ppd())
            .field("sram_if4_ppd", &self.sram_if4_ppd())
            .field("sram_if5_ppd", &self.sram_if5_ppd())
            .field("sram_if6_ppd", &self.sram_if6_ppd())
            .field("sram_if7_ppd", &self.sram_if7_ppd())
            .field("sram_if8_ppd", &self.sram_if8_ppd())
            .field("sram_if9_ppd", &self.sram_if9_ppd())
            .field("sram_if10_ppd", &self.sram_if10_ppd())
            .field("sram_if11_ppd", &self.sram_if11_ppd())
            .field("sram_if12_ppd", &self.sram_if12_ppd())
            .field("sram_if13_ppd", &self.sram_if13_ppd())
            .field("sram_if14_ppd", &self.sram_if14_ppd())
            .field("sram_if15_ppd", &self.sram_if15_ppd())
            .field("sram_if16_ppd", &self.sram_if16_ppd())
            .field("sram_if17_ppd", &self.sram_if17_ppd())
            .field("sram_if18_ppd", &self.sram_if18_ppd())
            .field("sram_if19_ppd", &self.sram_if19_ppd())
            .field("sram_if20_ppd", &self.sram_if20_ppd())
            .field("sram_if21_ppd", &self.sram_if21_ppd())
            .field("sram_if22_ppd", &self.sram_if22_ppd())
            .field("sram_if23_ppd", &self.sram_if23_ppd())
            .field("sram_if24_ppd", &self.sram_if24_ppd())
            .field("sram_if25_ppd", &self.sram_if25_ppd())
            .field("sram_if26_ppd", &self.sram_if26_ppd())
            .field("sram_if27_ppd", &self.sram_if27_ppd())
            .field("sram_if28_ppd", &self.sram_if28_ppd())
            .field("sram_if29_ppd", &self.sram_if29_ppd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg3Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg3Clr {{ sram_if0_ppd: {:?}, sram_if1_ppd: {:?}, sram_if2_ppd: {:?}, sram_if3_ppd: {:?}, sram_if4_ppd: {:?}, sram_if5_ppd: {:?}, sram_if6_ppd: {:?}, sram_if7_ppd: {:?}, sram_if8_ppd: {:?}, sram_if9_ppd: {:?}, sram_if10_ppd: {:?}, sram_if11_ppd: {:?}, sram_if12_ppd: {:?}, sram_if13_ppd: {:?}, sram_if14_ppd: {:?}, sram_if15_ppd: {:?}, sram_if16_ppd: {:?}, sram_if17_ppd: {:?}, sram_if18_ppd: {:?}, sram_if19_ppd: {:?}, sram_if20_ppd: {:?}, sram_if21_ppd: {:?}, sram_if22_ppd: {:?}, sram_if23_ppd: {:?}, sram_if24_ppd: {:?}, sram_if25_ppd: {:?}, sram_if26_ppd: {:?}, sram_if27_ppd: {:?}, sram_if28_ppd: {:?}, sram_if29_ppd: {:?} }}",
            self.sram_if0_ppd(),
            self.sram_if1_ppd(),
            self.sram_if2_ppd(),
            self.sram_if3_ppd(),
            self.sram_if4_ppd(),
            self.sram_if5_ppd(),
            self.sram_if6_ppd(),
            self.sram_if7_ppd(),
            self.sram_if8_ppd(),
            self.sram_if9_ppd(),
            self.sram_if10_ppd(),
            self.sram_if11_ppd(),
            self.sram_if12_ppd(),
            self.sram_if13_ppd(),
            self.sram_if14_ppd(),
            self.sram_if15_ppd(),
            self.sram_if16_ppd(),
            self.sram_if17_ppd(),
            self.sram_if18_ppd(),
            self.sram_if19_ppd(),
            self.sram_if20_ppd(),
            self.sram_if21_ppd(),
            self.sram_if22_ppd(),
            self.sram_if23_ppd(),
            self.sram_if24_ppd(),
            self.sram_if25_ppd(),
            self.sram_if26_ppd(),
            self.sram_if27_ppd(),
            self.sram_if28_ppd(),
            self.sram_if29_ppd()
        )
    }
}
#[doc = "Run configuration 3 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdruncfg3Set(pub u32);
impl Pdruncfg3Set {
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_ppd(&self) -> super::vals::Pdruncfg3SetSramIf0Ppd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf0Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if0_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf0Ppd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_ppd(&self) -> super::vals::Pdruncfg3SetSramIf1Ppd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf1Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if1_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf1Ppd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_ppd(&self) -> super::vals::Pdruncfg3SetSramIf2Ppd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf2Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if2_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf2Ppd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_ppd(&self) -> super::vals::Pdruncfg3SetSramIf3Ppd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf3Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if3_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf3Ppd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_ppd(&self) -> super::vals::Pdruncfg3SetSramIf4Ppd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf4Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if4_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf4Ppd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_ppd(&self) -> super::vals::Pdruncfg3SetSramIf5Ppd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf5Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if5_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf5Ppd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_ppd(&self) -> super::vals::Pdruncfg3SetSramIf6Ppd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf6Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if6_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf6Ppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_ppd(&self) -> super::vals::Pdruncfg3SetSramIf7Ppd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf7Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if7_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf7Ppd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_ppd(&self) -> super::vals::Pdruncfg3SetSramIf8Ppd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf8Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if8_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf8Ppd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_ppd(&self) -> super::vals::Pdruncfg3SetSramIf9Ppd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf9Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if9_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf9Ppd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_ppd(&self) -> super::vals::Pdruncfg3SetSramIf10Ppd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf10Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if10_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf10Ppd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_ppd(&self) -> super::vals::Pdruncfg3SetSramIf11Ppd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf11Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if11_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf11Ppd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_ppd(&self) -> super::vals::Pdruncfg3SetSramIf12Ppd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf12Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if12_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf12Ppd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_ppd(&self) -> super::vals::Pdruncfg3SetSramIf13Ppd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf13Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if13_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf13Ppd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_ppd(&self) -> super::vals::Pdruncfg3SetSramIf14Ppd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf14Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if14_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf14Ppd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_ppd(&self) -> super::vals::Pdruncfg3SetSramIf15Ppd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf15Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if15_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf15Ppd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_ppd(&self) -> super::vals::Pdruncfg3SetSramIf16Ppd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf16Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if16_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf16Ppd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_ppd(&self) -> super::vals::Pdruncfg3SetSramIf17Ppd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf17Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if17_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf17Ppd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_ppd(&self) -> super::vals::Pdruncfg3SetSramIf18Ppd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf18Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if18_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf18Ppd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_ppd(&self) -> super::vals::Pdruncfg3SetSramIf19Ppd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf19Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if19_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf19Ppd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_ppd(&self) -> super::vals::Pdruncfg3SetSramIf20Ppd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf20Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if20_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf20Ppd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_ppd(&self) -> super::vals::Pdruncfg3SetSramIf21Ppd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf21Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if21_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf21Ppd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_ppd(&self) -> super::vals::Pdruncfg3SetSramIf22Ppd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf22Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if22_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf22Ppd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_ppd(&self) -> super::vals::Pdruncfg3SetSramIf23Ppd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf23Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if23_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf23Ppd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_ppd(&self) -> super::vals::Pdruncfg3SetSramIf24Ppd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf24Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if24_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf24Ppd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_ppd(&self) -> super::vals::Pdruncfg3SetSramIf25Ppd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf25Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if25_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf25Ppd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_ppd(&self) -> super::vals::Pdruncfg3SetSramIf26Ppd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf26Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if26_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf26Ppd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_ppd(&self) -> super::vals::Pdruncfg3SetSramIf27Ppd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf27Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if27_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf27Ppd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_ppd(&self) -> super::vals::Pdruncfg3SetSramIf28Ppd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf28Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if28_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf28Ppd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_ppd(&self) -> super::vals::Pdruncfg3SetSramIf29Ppd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdruncfg3SetSramIf29Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if29_ppd(&mut self, val: super::vals::Pdruncfg3SetSramIf29Ppd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdruncfg3Set {
    #[inline(always)]
    fn default() -> Pdruncfg3Set {
        Pdruncfg3Set(0)
    }
}
impl core::fmt::Debug for Pdruncfg3Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdruncfg3Set")
            .field("sram_if0_ppd", &self.sram_if0_ppd())
            .field("sram_if1_ppd", &self.sram_if1_ppd())
            .field("sram_if2_ppd", &self.sram_if2_ppd())
            .field("sram_if3_ppd", &self.sram_if3_ppd())
            .field("sram_if4_ppd", &self.sram_if4_ppd())
            .field("sram_if5_ppd", &self.sram_if5_ppd())
            .field("sram_if6_ppd", &self.sram_if6_ppd())
            .field("sram_if7_ppd", &self.sram_if7_ppd())
            .field("sram_if8_ppd", &self.sram_if8_ppd())
            .field("sram_if9_ppd", &self.sram_if9_ppd())
            .field("sram_if10_ppd", &self.sram_if10_ppd())
            .field("sram_if11_ppd", &self.sram_if11_ppd())
            .field("sram_if12_ppd", &self.sram_if12_ppd())
            .field("sram_if13_ppd", &self.sram_if13_ppd())
            .field("sram_if14_ppd", &self.sram_if14_ppd())
            .field("sram_if15_ppd", &self.sram_if15_ppd())
            .field("sram_if16_ppd", &self.sram_if16_ppd())
            .field("sram_if17_ppd", &self.sram_if17_ppd())
            .field("sram_if18_ppd", &self.sram_if18_ppd())
            .field("sram_if19_ppd", &self.sram_if19_ppd())
            .field("sram_if20_ppd", &self.sram_if20_ppd())
            .field("sram_if21_ppd", &self.sram_if21_ppd())
            .field("sram_if22_ppd", &self.sram_if22_ppd())
            .field("sram_if23_ppd", &self.sram_if23_ppd())
            .field("sram_if24_ppd", &self.sram_if24_ppd())
            .field("sram_if25_ppd", &self.sram_if25_ppd())
            .field("sram_if26_ppd", &self.sram_if26_ppd())
            .field("sram_if27_ppd", &self.sram_if27_ppd())
            .field("sram_if28_ppd", &self.sram_if28_ppd())
            .field("sram_if29_ppd", &self.sram_if29_ppd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdruncfg3Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdruncfg3Set {{ sram_if0_ppd: {:?}, sram_if1_ppd: {:?}, sram_if2_ppd: {:?}, sram_if3_ppd: {:?}, sram_if4_ppd: {:?}, sram_if5_ppd: {:?}, sram_if6_ppd: {:?}, sram_if7_ppd: {:?}, sram_if8_ppd: {:?}, sram_if9_ppd: {:?}, sram_if10_ppd: {:?}, sram_if11_ppd: {:?}, sram_if12_ppd: {:?}, sram_if13_ppd: {:?}, sram_if14_ppd: {:?}, sram_if15_ppd: {:?}, sram_if16_ppd: {:?}, sram_if17_ppd: {:?}, sram_if18_ppd: {:?}, sram_if19_ppd: {:?}, sram_if20_ppd: {:?}, sram_if21_ppd: {:?}, sram_if22_ppd: {:?}, sram_if23_ppd: {:?}, sram_if24_ppd: {:?}, sram_if25_ppd: {:?}, sram_if26_ppd: {:?}, sram_if27_ppd: {:?}, sram_if28_ppd: {:?}, sram_if29_ppd: {:?} }}",
            self.sram_if0_ppd(),
            self.sram_if1_ppd(),
            self.sram_if2_ppd(),
            self.sram_if3_ppd(),
            self.sram_if4_ppd(),
            self.sram_if5_ppd(),
            self.sram_if6_ppd(),
            self.sram_if7_ppd(),
            self.sram_if8_ppd(),
            self.sram_if9_ppd(),
            self.sram_if10_ppd(),
            self.sram_if11_ppd(),
            self.sram_if12_ppd(),
            self.sram_if13_ppd(),
            self.sram_if14_ppd(),
            self.sram_if15_ppd(),
            self.sram_if16_ppd(),
            self.sram_if17_ppd(),
            self.sram_if18_ppd(),
            self.sram_if19_ppd(),
            self.sram_if20_ppd(),
            self.sram_if21_ppd(),
            self.sram_if22_ppd(),
            self.sram_if23_ppd(),
            self.sram_if24_ppd(),
            self.sram_if25_ppd(),
            self.sram_if26_ppd(),
            self.sram_if27_ppd(),
            self.sram_if28_ppd(),
            self.sram_if29_ppd()
        )
    }
}
#[doc = "Sleep configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsleepcfg0(pub u32);
impl Pdsleepcfg0 {
    #[doc = "main clock shut off"]
    #[must_use]
    #[inline(always)]
    pub const fn mainclk_shutoff(&self) -> super::vals::MainclkShutoff {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MainclkShutoff::from_bits(val as u8)
    }
    #[doc = "main clock shut off"]
    #[inline(always)]
    pub const fn set_mainclk_shutoff(&mut self, val: super::vals::MainclkShutoff) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode0(&self) -> super::vals::Pdsleepcfg0PmicMode0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdsleepcfg0PmicMode0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode0(&mut self, val: super::vals::Pdsleepcfg0PmicMode0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_mode1(&self) -> super::vals::Pdsleepcfg0PmicMode1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdsleepcfg0PmicMode1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic_mode1(&mut self, val: super::vals::Pdsleepcfg0PmicMode1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn deep_pd(&self) -> super::vals::DeepPd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DeepPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_deep_pd(&mut self, val: super::vals::DeepPd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn vddcorereg_lp(&self) -> super::vals::Pdsleepcfg0VddcoreregLp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdsleepcfg0VddcoreregLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_vddcorereg_lp(&mut self, val: super::vals::Pdsleepcfg0VddcoreregLp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmcref_lp(&self) -> super::vals::Pdsleepcfg0PmcrefLp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdsleepcfg0PmcrefLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmcref_lp(&mut self, val: super::vals::Pdsleepcfg0PmcrefLp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvd1v8_pd(&self) -> super::vals::Pdsleepcfg0Hvd1v8Pd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdsleepcfg0Hvd1v8Pd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvd1v8_pd(&mut self, val: super::vals::Pdsleepcfg0Hvd1v8Pd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn porcore_lp(&self) -> super::vals::Pdsleepcfg0PorcoreLp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdsleepcfg0PorcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_porcore_lp(&mut self, val: super::vals::Pdsleepcfg0PorcoreLp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lvdcore_lp(&self) -> super::vals::Pdsleepcfg0LvdcoreLp {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdsleepcfg0LvdcoreLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lvdcore_lp(&mut self, val: super::vals::Pdsleepcfg0LvdcoreLp) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hvdcore_pd(&self) -> super::vals::Pdsleepcfg0HvdcorePd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdsleepcfg0HvdcorePd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hvdcore_pd(&mut self, val: super::vals::Pdsleepcfg0HvdcorePd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Writes to this bit in PDRUNCFG, but not PDSLEEPCFG, can be disabled by an OTP bit."]
    #[must_use]
    #[inline(always)]
    pub const fn rbb_pd(&self) -> super::vals::Pdsleepcfg0RbbPd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdsleepcfg0RbbPd::from_bits(val as u8)
    }
    #[doc = "Writes to this bit in PDRUNCFG, but not PDSLEEPCFG, can be disabled by an OTP bit."]
    #[inline(always)]
    pub const fn set_rbb_pd(&mut self, val: super::vals::Pdsleepcfg0RbbPd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Writes to this bit in PDRUNCFG, but not PDSLEEPCFG, can be disabled by an OTP bit."]
    #[must_use]
    #[inline(always)]
    pub const fn fbb_pd(&self) -> super::vals::Pdsleepcfg0FbbPd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdsleepcfg0FbbPd::from_bits(val as u8)
    }
    #[doc = "Writes to this bit in PDRUNCFG, but not PDSLEEPCFG, can be disabled by an OTP bit."]
    #[inline(always)]
    pub const fn set_fbb_pd(&mut self, val: super::vals::Pdsleepcfg0FbbPd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sysxtal_pd(&self) -> super::vals::Pdsleepcfg0SysxtalPd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdsleepcfg0SysxtalPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sysxtal_pd(&mut self, val: super::vals::Pdsleepcfg0SysxtalPd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn lposc_pd(&self) -> super::vals::Pdsleepcfg0LposcPd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdsleepcfg0LposcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_lposc_pd(&mut self, val: super::vals::Pdsleepcfg0LposcPd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sfro_pd(&self) -> super::vals::Pdsleepcfg0SfroPd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdsleepcfg0SfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sfro_pd(&mut self, val: super::vals::Pdsleepcfg0SfroPd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ffro_pd(&self) -> super::vals::Pdsleepcfg0FfroPd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdsleepcfg0FfroPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ffro_pd(&mut self, val: super::vals::Pdsleepcfg0FfroPd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllldo_pd(&self) -> super::vals::Pdsleepcfg0SyspllldoPd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdsleepcfg0SyspllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllldo_pd(&mut self, val: super::vals::Pdsleepcfg0SyspllldoPd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn syspllana_pd(&self) -> super::vals::Pdsleepcfg0SyspllanaPd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdsleepcfg0SyspllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_syspllana_pd(&mut self, val: super::vals::Pdsleepcfg0SyspllanaPd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllldo_pd(&self) -> super::vals::Pdsleepcfg0AudpllldoPd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdsleepcfg0AudpllldoPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllldo_pd(&mut self, val: super::vals::Pdsleepcfg0AudpllldoPd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn audpllana_pd(&self) -> super::vals::Pdsleepcfg0AudpllanaPd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdsleepcfg0AudpllanaPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_audpllana_pd(&mut self, val: super::vals::Pdsleepcfg0AudpllanaPd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_pd(&self) -> super::vals::Pdsleepcfg0AdcPd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdsleepcfg0AdcPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_pd(&mut self, val: super::vals::Pdsleepcfg0AdcPd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_lp(&self) -> super::vals::Pdsleepcfg0AdcLp {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdsleepcfg0AdcLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc_lp(&mut self, val: super::vals::Pdsleepcfg0AdcLp) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adctempsns_pd(&self) -> super::vals::Pdsleepcfg0AdctempsnsPd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdsleepcfg0AdctempsnsPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adctempsns_pd(&mut self, val: super::vals::Pdsleepcfg0AdctempsnsPd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_pd(&self) -> super::vals::Pdsleepcfg0AcmpPd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdsleepcfg0AcmpPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp_pd(&mut self, val: super::vals::Pdsleepcfg0AcmpPd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_vdet_lp(&self) -> super::vals::Pdsleepcfg0Hspad0VdetLp {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdsleepcfg0Hspad0VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_vdet_lp(&mut self, val: super::vals::Pdsleepcfg0Hspad0VdetLp) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad0_ref_pd(&self) -> super::vals::Pdsleepcfg0Hspad0RefPd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdsleepcfg0Hspad0RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad0_ref_pd(&mut self, val: super::vals::Pdsleepcfg0Hspad0RefPd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_vdet_lp(&self) -> super::vals::Pdsleepcfg0Hspad2VdetLp {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdsleepcfg0Hspad2VdetLp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_vdet_lp(&mut self, val: super::vals::Pdsleepcfg0Hspad2VdetLp) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hspad2_ref_pd(&self) -> super::vals::Pdsleepcfg0Hspad2RefPd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdsleepcfg0Hspad2RefPd::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hspad2_ref_pd(&mut self, val: super::vals::Pdsleepcfg0Hspad2RefPd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdsleepcfg0 {
    #[inline(always)]
    fn default() -> Pdsleepcfg0 {
        Pdsleepcfg0(0)
    }
}
impl core::fmt::Debug for Pdsleepcfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdsleepcfg0")
            .field("mainclk_shutoff", &self.mainclk_shutoff())
            .field("pmic_mode0", &self.pmic_mode0())
            .field("pmic_mode1", &self.pmic_mode1())
            .field("deep_pd", &self.deep_pd())
            .field("vddcorereg_lp", &self.vddcorereg_lp())
            .field("pmcref_lp", &self.pmcref_lp())
            .field("hvd1v8_pd", &self.hvd1v8_pd())
            .field("porcore_lp", &self.porcore_lp())
            .field("lvdcore_lp", &self.lvdcore_lp())
            .field("hvdcore_pd", &self.hvdcore_pd())
            .field("rbb_pd", &self.rbb_pd())
            .field("fbb_pd", &self.fbb_pd())
            .field("sysxtal_pd", &self.sysxtal_pd())
            .field("lposc_pd", &self.lposc_pd())
            .field("sfro_pd", &self.sfro_pd())
            .field("ffro_pd", &self.ffro_pd())
            .field("syspllldo_pd", &self.syspllldo_pd())
            .field("syspllana_pd", &self.syspllana_pd())
            .field("audpllldo_pd", &self.audpllldo_pd())
            .field("audpllana_pd", &self.audpllana_pd())
            .field("adc_pd", &self.adc_pd())
            .field("adc_lp", &self.adc_lp())
            .field("adctempsns_pd", &self.adctempsns_pd())
            .field("acmp_pd", &self.acmp_pd())
            .field("hspad0_vdet_lp", &self.hspad0_vdet_lp())
            .field("hspad0_ref_pd", &self.hspad0_ref_pd())
            .field("hspad2_vdet_lp", &self.hspad2_vdet_lp())
            .field("hspad2_ref_pd", &self.hspad2_ref_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdsleepcfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdsleepcfg0 {{ mainclk_shutoff: {:?}, pmic_mode0: {:?}, pmic_mode1: {:?}, deep_pd: {:?}, vddcorereg_lp: {:?}, pmcref_lp: {:?}, hvd1v8_pd: {:?}, porcore_lp: {:?}, lvdcore_lp: {:?}, hvdcore_pd: {:?}, rbb_pd: {:?}, fbb_pd: {:?}, sysxtal_pd: {:?}, lposc_pd: {:?}, sfro_pd: {:?}, ffro_pd: {:?}, syspllldo_pd: {:?}, syspllana_pd: {:?}, audpllldo_pd: {:?}, audpllana_pd: {:?}, adc_pd: {:?}, adc_lp: {:?}, adctempsns_pd: {:?}, acmp_pd: {:?}, hspad0_vdet_lp: {:?}, hspad0_ref_pd: {:?}, hspad2_vdet_lp: {:?}, hspad2_ref_pd: {:?} }}",
            self.mainclk_shutoff(),
            self.pmic_mode0(),
            self.pmic_mode1(),
            self.deep_pd(),
            self.vddcorereg_lp(),
            self.pmcref_lp(),
            self.hvd1v8_pd(),
            self.porcore_lp(),
            self.lvdcore_lp(),
            self.hvdcore_pd(),
            self.rbb_pd(),
            self.fbb_pd(),
            self.sysxtal_pd(),
            self.lposc_pd(),
            self.sfro_pd(),
            self.ffro_pd(),
            self.syspllldo_pd(),
            self.syspllana_pd(),
            self.audpllldo_pd(),
            self.audpllana_pd(),
            self.adc_pd(),
            self.adc_lp(),
            self.adctempsns_pd(),
            self.acmp_pd(),
            self.hspad0_vdet_lp(),
            self.hspad0_ref_pd(),
            self.hspad2_vdet_lp(),
            self.hspad2_ref_pd()
        )
    }
}
#[doc = "Sleep configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsleepcfg1(pub u32);
impl Pdsleepcfg1 {
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_apd(&self) -> super::vals::Pdsleepcfg1PqSramApd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdsleepcfg1PqSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_pq_sram_apd(&mut self, val: super::vals::Pdsleepcfg1PqSramApd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn pq_sram_ppd(&self) -> super::vals::Pdsleepcfg1PqSramPpd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdsleepcfg1PqSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_pq_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1PqSramPpd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_apd(&self) -> super::vals::Pdsleepcfg1FlexspiSramApd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdsleepcfg1FlexspiSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_apd(&mut self, val: super::vals::Pdsleepcfg1FlexspiSramApd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_sram_ppd(&self) -> super::vals::Pdsleepcfg1FlexspiSramPpd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdsleepcfg1FlexspiSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_flexspi_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1FlexspiSramPpd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_apd(&self) -> super::vals::Pdsleepcfg1UsbhsSramApd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdsleepcfg1UsbhsSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_apd(&mut self, val: super::vals::Pdsleepcfg1UsbhsSramApd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usbhs_sram_ppd(&self) -> super::vals::Pdsleepcfg1UsbhsSramPpd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdsleepcfg1UsbhsSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usbhs_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1UsbhsSramPpd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_apd(&self) -> super::vals::Pdsleepcfg1Usdhc0SramApd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdsleepcfg1Usdhc0SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_apd(&mut self, val: super::vals::Pdsleepcfg1Usdhc0SramApd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc0_sram_ppd(&self) -> super::vals::Pdsleepcfg1Usdhc0SramPpd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdsleepcfg1Usdhc0SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc0_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1Usdhc0SramPpd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_apd(&self) -> super::vals::Pdsleepcfg1Usdhc1SramApd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdsleepcfg1Usdhc1SramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_apd(&mut self, val: super::vals::Pdsleepcfg1Usdhc1SramApd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc1_sram_ppd(&self) -> super::vals::Pdsleepcfg1Usdhc1SramPpd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdsleepcfg1Usdhc1SramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_usdhc1_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1Usdhc1SramPpd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_apd(&self) -> super::vals::Pdsleepcfg1CasperSramApd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdsleepcfg1CasperSramApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_casper_sram_apd(&mut self, val: super::vals::Pdsleepcfg1CasperSramApd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn casper_sram_ppd(&self) -> super::vals::Pdsleepcfg1CasperSramPpd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdsleepcfg1CasperSramPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_casper_sram_ppd(&mut self, val: super::vals::Pdsleepcfg1CasperSramPpd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_apd(&self) -> super::vals::Pdsleepcfg1DspcacheRegfApd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdsleepcfg1DspcacheRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_apd(&mut self, val: super::vals::Pdsleepcfg1DspcacheRegfApd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dspcache_regf_ppd(&self) -> super::vals::Pdsleepcfg1DspcacheRegfPpd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdsleepcfg1DspcacheRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dspcache_regf_ppd(&mut self, val: super::vals::Pdsleepcfg1DspcacheRegfPpd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_apd(&self) -> super::vals::Pdsleepcfg1DsptcmRegfApd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdsleepcfg1DsptcmRegfApd::from_bits(val as u8)
    }
    #[doc = "Array power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_apd(&mut self, val: super::vals::Pdsleepcfg1DsptcmRegfApd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peiph power"]
    #[must_use]
    #[inline(always)]
    pub const fn dsptcm_regf_ppd(&self) -> super::vals::Pdsleepcfg1DsptcmRegfPpd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdsleepcfg1DsptcmRegfPpd::from_bits(val as u8)
    }
    #[doc = "Peiph power"]
    #[inline(always)]
    pub const fn set_dsptcm_regf_ppd(&mut self, val: super::vals::Pdsleepcfg1DsptcmRegfPpd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "array power and periph power"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_pd(&self) -> super::vals::Pdsleepcfg1RomPd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdsleepcfg1RomPd::from_bits(val as u8)
    }
    #[doc = "array power and periph power"]
    #[inline(always)]
    pub const fn set_rom_pd(&mut self, val: super::vals::Pdsleepcfg1RomPd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Needed when vddcore can be smaller than 0"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_sleep(&self) -> super::vals::Pdsleepcfg1SramSleep {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pdsleepcfg1SramSleep::from_bits(val as u8)
    }
    #[doc = "Needed when vddcore can be smaller than 0"]
    #[inline(always)]
    pub const fn set_sram_sleep(&mut self, val: super::vals::Pdsleepcfg1SramSleep) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pdsleepcfg1 {
    #[inline(always)]
    fn default() -> Pdsleepcfg1 {
        Pdsleepcfg1(0)
    }
}
impl core::fmt::Debug for Pdsleepcfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdsleepcfg1")
            .field("pq_sram_apd", &self.pq_sram_apd())
            .field("pq_sram_ppd", &self.pq_sram_ppd())
            .field("flexspi_sram_apd", &self.flexspi_sram_apd())
            .field("flexspi_sram_ppd", &self.flexspi_sram_ppd())
            .field("usbhs_sram_apd", &self.usbhs_sram_apd())
            .field("usbhs_sram_ppd", &self.usbhs_sram_ppd())
            .field("usdhc0_sram_apd", &self.usdhc0_sram_apd())
            .field("usdhc0_sram_ppd", &self.usdhc0_sram_ppd())
            .field("usdhc1_sram_apd", &self.usdhc1_sram_apd())
            .field("usdhc1_sram_ppd", &self.usdhc1_sram_ppd())
            .field("casper_sram_apd", &self.casper_sram_apd())
            .field("casper_sram_ppd", &self.casper_sram_ppd())
            .field("dspcache_regf_apd", &self.dspcache_regf_apd())
            .field("dspcache_regf_ppd", &self.dspcache_regf_ppd())
            .field("dsptcm_regf_apd", &self.dsptcm_regf_apd())
            .field("dsptcm_regf_ppd", &self.dsptcm_regf_ppd())
            .field("rom_pd", &self.rom_pd())
            .field("sram_sleep", &self.sram_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdsleepcfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdsleepcfg1 {{ pq_sram_apd: {:?}, pq_sram_ppd: {:?}, flexspi_sram_apd: {:?}, flexspi_sram_ppd: {:?}, usbhs_sram_apd: {:?}, usbhs_sram_ppd: {:?}, usdhc0_sram_apd: {:?}, usdhc0_sram_ppd: {:?}, usdhc1_sram_apd: {:?}, usdhc1_sram_ppd: {:?}, casper_sram_apd: {:?}, casper_sram_ppd: {:?}, dspcache_regf_apd: {:?}, dspcache_regf_ppd: {:?}, dsptcm_regf_apd: {:?}, dsptcm_regf_ppd: {:?}, rom_pd: {:?}, sram_sleep: {:?} }}",
            self.pq_sram_apd(),
            self.pq_sram_ppd(),
            self.flexspi_sram_apd(),
            self.flexspi_sram_ppd(),
            self.usbhs_sram_apd(),
            self.usbhs_sram_ppd(),
            self.usdhc0_sram_apd(),
            self.usdhc0_sram_ppd(),
            self.usdhc1_sram_apd(),
            self.usdhc1_sram_ppd(),
            self.casper_sram_apd(),
            self.casper_sram_ppd(),
            self.dspcache_regf_apd(),
            self.dspcache_regf_ppd(),
            self.dsptcm_regf_apd(),
            self.dsptcm_regf_ppd(),
            self.rom_pd(),
            self.sram_sleep()
        )
    }
}
#[doc = "Sleep configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsleepcfg2(pub u32);
impl Pdsleepcfg2 {
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_apd(&self) -> super::vals::Pdsleepcfg2SramIf0Apd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf0Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if0_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf0Apd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_apd(&self) -> super::vals::Pdsleepcfg2SramIf1Apd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf1Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if1_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf1Apd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_apd(&self) -> super::vals::Pdsleepcfg2SramIf2Apd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf2Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if2_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf2Apd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_apd(&self) -> super::vals::Pdsleepcfg2SramIf3Apd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf3Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if3_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf3Apd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_apd(&self) -> super::vals::Pdsleepcfg2SramIf4Apd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf4Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if4_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf4Apd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_apd(&self) -> super::vals::Pdsleepcfg2SramIf5Apd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf5Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if5_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf5Apd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_apd(&self) -> super::vals::Pdsleepcfg2SramIf6Apd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf6Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if6_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf6Apd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_apd(&self) -> super::vals::Pdsleepcfg2SramIf7Apd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf7Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if7_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf7Apd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_apd(&self) -> super::vals::Pdsleepcfg2SramIf8Apd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf8Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if8_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf8Apd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_apd(&self) -> super::vals::Pdsleepcfg2SramIf9Apd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf9Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if9_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf9Apd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_apd(&self) -> super::vals::Pdsleepcfg2SramIf10Apd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf10Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if10_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf10Apd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_apd(&self) -> super::vals::Pdsleepcfg2SramIf11Apd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf11Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if11_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf11Apd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_apd(&self) -> super::vals::Pdsleepcfg2SramIf12Apd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf12Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if12_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf12Apd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_apd(&self) -> super::vals::Pdsleepcfg2SramIf13Apd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf13Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if13_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf13Apd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_apd(&self) -> super::vals::Pdsleepcfg2SramIf14Apd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf14Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if14_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf14Apd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_apd(&self) -> super::vals::Pdsleepcfg2SramIf15Apd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf15Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if15_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf15Apd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_apd(&self) -> super::vals::Pdsleepcfg2SramIf16Apd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf16Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if16_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf16Apd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_apd(&self) -> super::vals::Pdsleepcfg2SramIf17Apd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf17Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if17_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf17Apd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_apd(&self) -> super::vals::Pdsleepcfg2SramIf18Apd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf18Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if18_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf18Apd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_apd(&self) -> super::vals::Pdsleepcfg2SramIf19Apd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf19Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if19_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf19Apd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_apd(&self) -> super::vals::Pdsleepcfg2SramIf20Apd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf20Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if20_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf20Apd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_apd(&self) -> super::vals::Pdsleepcfg2SramIf21Apd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf21Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if21_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf21Apd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_apd(&self) -> super::vals::Pdsleepcfg2SramIf22Apd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf22Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if22_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf22Apd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_apd(&self) -> super::vals::Pdsleepcfg2SramIf23Apd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf23Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if23_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf23Apd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_apd(&self) -> super::vals::Pdsleepcfg2SramIf24Apd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf24Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if24_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf24Apd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_apd(&self) -> super::vals::Pdsleepcfg2SramIf25Apd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf25Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if25_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf25Apd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_apd(&self) -> super::vals::Pdsleepcfg2SramIf26Apd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf26Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if26_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf26Apd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_apd(&self) -> super::vals::Pdsleepcfg2SramIf27Apd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf27Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if27_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf27Apd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_apd(&self) -> super::vals::Pdsleepcfg2SramIf28Apd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf28Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if28_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf28Apd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Array Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_apd(&self) -> super::vals::Pdsleepcfg2SramIf29Apd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdsleepcfg2SramIf29Apd::from_bits(val as u8)
    }
    #[doc = "Array Power"]
    #[inline(always)]
    pub const fn set_sram_if29_apd(&mut self, val: super::vals::Pdsleepcfg2SramIf29Apd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdsleepcfg2 {
    #[inline(always)]
    fn default() -> Pdsleepcfg2 {
        Pdsleepcfg2(0)
    }
}
impl core::fmt::Debug for Pdsleepcfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdsleepcfg2")
            .field("sram_if0_apd", &self.sram_if0_apd())
            .field("sram_if1_apd", &self.sram_if1_apd())
            .field("sram_if2_apd", &self.sram_if2_apd())
            .field("sram_if3_apd", &self.sram_if3_apd())
            .field("sram_if4_apd", &self.sram_if4_apd())
            .field("sram_if5_apd", &self.sram_if5_apd())
            .field("sram_if6_apd", &self.sram_if6_apd())
            .field("sram_if7_apd", &self.sram_if7_apd())
            .field("sram_if8_apd", &self.sram_if8_apd())
            .field("sram_if9_apd", &self.sram_if9_apd())
            .field("sram_if10_apd", &self.sram_if10_apd())
            .field("sram_if11_apd", &self.sram_if11_apd())
            .field("sram_if12_apd", &self.sram_if12_apd())
            .field("sram_if13_apd", &self.sram_if13_apd())
            .field("sram_if14_apd", &self.sram_if14_apd())
            .field("sram_if15_apd", &self.sram_if15_apd())
            .field("sram_if16_apd", &self.sram_if16_apd())
            .field("sram_if17_apd", &self.sram_if17_apd())
            .field("sram_if18_apd", &self.sram_if18_apd())
            .field("sram_if19_apd", &self.sram_if19_apd())
            .field("sram_if20_apd", &self.sram_if20_apd())
            .field("sram_if21_apd", &self.sram_if21_apd())
            .field("sram_if22_apd", &self.sram_if22_apd())
            .field("sram_if23_apd", &self.sram_if23_apd())
            .field("sram_if24_apd", &self.sram_if24_apd())
            .field("sram_if25_apd", &self.sram_if25_apd())
            .field("sram_if26_apd", &self.sram_if26_apd())
            .field("sram_if27_apd", &self.sram_if27_apd())
            .field("sram_if28_apd", &self.sram_if28_apd())
            .field("sram_if29_apd", &self.sram_if29_apd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdsleepcfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdsleepcfg2 {{ sram_if0_apd: {:?}, sram_if1_apd: {:?}, sram_if2_apd: {:?}, sram_if3_apd: {:?}, sram_if4_apd: {:?}, sram_if5_apd: {:?}, sram_if6_apd: {:?}, sram_if7_apd: {:?}, sram_if8_apd: {:?}, sram_if9_apd: {:?}, sram_if10_apd: {:?}, sram_if11_apd: {:?}, sram_if12_apd: {:?}, sram_if13_apd: {:?}, sram_if14_apd: {:?}, sram_if15_apd: {:?}, sram_if16_apd: {:?}, sram_if17_apd: {:?}, sram_if18_apd: {:?}, sram_if19_apd: {:?}, sram_if20_apd: {:?}, sram_if21_apd: {:?}, sram_if22_apd: {:?}, sram_if23_apd: {:?}, sram_if24_apd: {:?}, sram_if25_apd: {:?}, sram_if26_apd: {:?}, sram_if27_apd: {:?}, sram_if28_apd: {:?}, sram_if29_apd: {:?} }}",
            self.sram_if0_apd(),
            self.sram_if1_apd(),
            self.sram_if2_apd(),
            self.sram_if3_apd(),
            self.sram_if4_apd(),
            self.sram_if5_apd(),
            self.sram_if6_apd(),
            self.sram_if7_apd(),
            self.sram_if8_apd(),
            self.sram_if9_apd(),
            self.sram_if10_apd(),
            self.sram_if11_apd(),
            self.sram_if12_apd(),
            self.sram_if13_apd(),
            self.sram_if14_apd(),
            self.sram_if15_apd(),
            self.sram_if16_apd(),
            self.sram_if17_apd(),
            self.sram_if18_apd(),
            self.sram_if19_apd(),
            self.sram_if20_apd(),
            self.sram_if21_apd(),
            self.sram_if22_apd(),
            self.sram_if23_apd(),
            self.sram_if24_apd(),
            self.sram_if25_apd(),
            self.sram_if26_apd(),
            self.sram_if27_apd(),
            self.sram_if28_apd(),
            self.sram_if29_apd()
        )
    }
}
#[doc = "Sleep configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdsleepcfg3(pub u32);
impl Pdsleepcfg3 {
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if0_ppd(&self) -> super::vals::Pdsleepcfg3SramIf0Ppd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf0Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if0_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf0Ppd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if1_ppd(&self) -> super::vals::Pdsleepcfg3SramIf1Ppd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf1Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if1_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf1Ppd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if2_ppd(&self) -> super::vals::Pdsleepcfg3SramIf2Ppd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf2Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if2_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf2Ppd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if3_ppd(&self) -> super::vals::Pdsleepcfg3SramIf3Ppd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf3Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if3_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf3Ppd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if4_ppd(&self) -> super::vals::Pdsleepcfg3SramIf4Ppd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf4Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if4_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf4Ppd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if5_ppd(&self) -> super::vals::Pdsleepcfg3SramIf5Ppd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf5Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if5_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf5Ppd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if6_ppd(&self) -> super::vals::Pdsleepcfg3SramIf6Ppd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf6Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if6_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf6Ppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if7_ppd(&self) -> super::vals::Pdsleepcfg3SramIf7Ppd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf7Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if7_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf7Ppd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if8_ppd(&self) -> super::vals::Pdsleepcfg3SramIf8Ppd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf8Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if8_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf8Ppd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if9_ppd(&self) -> super::vals::Pdsleepcfg3SramIf9Ppd {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf9Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if9_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf9Ppd) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if10_ppd(&self) -> super::vals::Pdsleepcfg3SramIf10Ppd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf10Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if10_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf10Ppd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if11_ppd(&self) -> super::vals::Pdsleepcfg3SramIf11Ppd {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf11Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if11_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf11Ppd) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if12_ppd(&self) -> super::vals::Pdsleepcfg3SramIf12Ppd {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf12Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if12_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf12Ppd) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if13_ppd(&self) -> super::vals::Pdsleepcfg3SramIf13Ppd {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf13Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if13_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf13Ppd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if14_ppd(&self) -> super::vals::Pdsleepcfg3SramIf14Ppd {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf14Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if14_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf14Ppd) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if15_ppd(&self) -> super::vals::Pdsleepcfg3SramIf15Ppd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf15Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if15_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf15Ppd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if16_ppd(&self) -> super::vals::Pdsleepcfg3SramIf16Ppd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf16Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if16_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf16Ppd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if17_ppd(&self) -> super::vals::Pdsleepcfg3SramIf17Ppd {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf17Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if17_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf17Ppd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if18_ppd(&self) -> super::vals::Pdsleepcfg3SramIf18Ppd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf18Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if18_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf18Ppd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if19_ppd(&self) -> super::vals::Pdsleepcfg3SramIf19Ppd {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf19Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if19_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf19Ppd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if20_ppd(&self) -> super::vals::Pdsleepcfg3SramIf20Ppd {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf20Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if20_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf20Ppd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if21_ppd(&self) -> super::vals::Pdsleepcfg3SramIf21Ppd {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf21Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if21_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf21Ppd) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if22_ppd(&self) -> super::vals::Pdsleepcfg3SramIf22Ppd {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf22Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if22_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf22Ppd) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if23_ppd(&self) -> super::vals::Pdsleepcfg3SramIf23Ppd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf23Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if23_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf23Ppd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if24_ppd(&self) -> super::vals::Pdsleepcfg3SramIf24Ppd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf24Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if24_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf24Ppd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if25_ppd(&self) -> super::vals::Pdsleepcfg3SramIf25Ppd {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf25Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if25_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf25Ppd) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if26_ppd(&self) -> super::vals::Pdsleepcfg3SramIf26Ppd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf26Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if26_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf26Ppd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if27_ppd(&self) -> super::vals::Pdsleepcfg3SramIf27Ppd {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf27Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if27_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf27Ppd) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if28_ppd(&self) -> super::vals::Pdsleepcfg3SramIf28Ppd {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf28Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if28_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf28Ppd) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Periph Power"]
    #[must_use]
    #[inline(always)]
    pub const fn sram_if29_ppd(&self) -> super::vals::Pdsleepcfg3SramIf29Ppd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdsleepcfg3SramIf29Ppd::from_bits(val as u8)
    }
    #[doc = "Periph Power"]
    #[inline(always)]
    pub const fn set_sram_if29_ppd(&mut self, val: super::vals::Pdsleepcfg3SramIf29Ppd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Pdsleepcfg3 {
    #[inline(always)]
    fn default() -> Pdsleepcfg3 {
        Pdsleepcfg3(0)
    }
}
impl core::fmt::Debug for Pdsleepcfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdsleepcfg3")
            .field("sram_if0_ppd", &self.sram_if0_ppd())
            .field("sram_if1_ppd", &self.sram_if1_ppd())
            .field("sram_if2_ppd", &self.sram_if2_ppd())
            .field("sram_if3_ppd", &self.sram_if3_ppd())
            .field("sram_if4_ppd", &self.sram_if4_ppd())
            .field("sram_if5_ppd", &self.sram_if5_ppd())
            .field("sram_if6_ppd", &self.sram_if6_ppd())
            .field("sram_if7_ppd", &self.sram_if7_ppd())
            .field("sram_if8_ppd", &self.sram_if8_ppd())
            .field("sram_if9_ppd", &self.sram_if9_ppd())
            .field("sram_if10_ppd", &self.sram_if10_ppd())
            .field("sram_if11_ppd", &self.sram_if11_ppd())
            .field("sram_if12_ppd", &self.sram_if12_ppd())
            .field("sram_if13_ppd", &self.sram_if13_ppd())
            .field("sram_if14_ppd", &self.sram_if14_ppd())
            .field("sram_if15_ppd", &self.sram_if15_ppd())
            .field("sram_if16_ppd", &self.sram_if16_ppd())
            .field("sram_if17_ppd", &self.sram_if17_ppd())
            .field("sram_if18_ppd", &self.sram_if18_ppd())
            .field("sram_if19_ppd", &self.sram_if19_ppd())
            .field("sram_if20_ppd", &self.sram_if20_ppd())
            .field("sram_if21_ppd", &self.sram_if21_ppd())
            .field("sram_if22_ppd", &self.sram_if22_ppd())
            .field("sram_if23_ppd", &self.sram_if23_ppd())
            .field("sram_if24_ppd", &self.sram_if24_ppd())
            .field("sram_if25_ppd", &self.sram_if25_ppd())
            .field("sram_if26_ppd", &self.sram_if26_ppd())
            .field("sram_if27_ppd", &self.sram_if27_ppd())
            .field("sram_if28_ppd", &self.sram_if28_ppd())
            .field("sram_if29_ppd", &self.sram_if29_ppd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdsleepcfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdsleepcfg3 {{ sram_if0_ppd: {:?}, sram_if1_ppd: {:?}, sram_if2_ppd: {:?}, sram_if3_ppd: {:?}, sram_if4_ppd: {:?}, sram_if5_ppd: {:?}, sram_if6_ppd: {:?}, sram_if7_ppd: {:?}, sram_if8_ppd: {:?}, sram_if9_ppd: {:?}, sram_if10_ppd: {:?}, sram_if11_ppd: {:?}, sram_if12_ppd: {:?}, sram_if13_ppd: {:?}, sram_if14_ppd: {:?}, sram_if15_ppd: {:?}, sram_if16_ppd: {:?}, sram_if17_ppd: {:?}, sram_if18_ppd: {:?}, sram_if19_ppd: {:?}, sram_if20_ppd: {:?}, sram_if21_ppd: {:?}, sram_if22_ppd: {:?}, sram_if23_ppd: {:?}, sram_if24_ppd: {:?}, sram_if25_ppd: {:?}, sram_if26_ppd: {:?}, sram_if27_ppd: {:?}, sram_if28_ppd: {:?}, sram_if29_ppd: {:?} }}",
            self.sram_if0_ppd(),
            self.sram_if1_ppd(),
            self.sram_if2_ppd(),
            self.sram_if3_ppd(),
            self.sram_if4_ppd(),
            self.sram_if5_ppd(),
            self.sram_if6_ppd(),
            self.sram_if7_ppd(),
            self.sram_if8_ppd(),
            self.sram_if9_ppd(),
            self.sram_if10_ppd(),
            self.sram_if11_ppd(),
            self.sram_if12_ppd(),
            self.sram_if13_ppd(),
            self.sram_if14_ppd(),
            self.sram_if15_ppd(),
            self.sram_if16_ppd(),
            self.sram_if17_ppd(),
            self.sram_if18_ppd(),
            self.sram_if19_ppd(),
            self.sram_if20_ppd(),
            self.sram_if21_ppd(),
            self.sram_if22_ppd(),
            self.sram_if23_ppd(),
            self.sram_if24_ppd(),
            self.sram_if25_ppd(),
            self.sram_if26_ppd(),
            self.sram_if27_ppd(),
            self.sram_if28_ppd(),
            self.sram_if29_ppd()
        )
    }
}
#[doc = "PD Wake Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdwakecfg(pub u32);
impl Pdwakecfg {
    #[doc = "RBB mode on wakeup"]
    #[must_use]
    #[inline(always)]
    pub const fn rbbkeepst(&self) -> super::vals::Rbbkeepst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rbbkeepst::from_bits(val as u8)
    }
    #[doc = "RBB mode on wakeup"]
    #[inline(always)]
    pub const fn set_rbbkeepst(&mut self, val: super::vals::Rbbkeepst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FBB mode on wakeup"]
    #[must_use]
    #[inline(always)]
    pub const fn fbbkeepst(&self) -> super::vals::Fbbkeepst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fbbkeepst::from_bits(val as u8)
    }
    #[doc = "FBB mode on wakeup"]
    #[inline(always)]
    pub const fn set_fbbkeepst(&mut self, val: super::vals::Fbbkeepst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Pdwakecfg {
    #[inline(always)]
    fn default() -> Pdwakecfg {
        Pdwakecfg(0)
    }
}
impl core::fmt::Debug for Pdwakecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdwakecfg")
            .field("rbbkeepst", &self.rbbkeepst())
            .field("fbbkeepst", &self.fbbkeepst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdwakecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdwakecfg {{ rbbkeepst: {:?}, fbbkeepst: {:?} }}",
            self.rbbkeepst(),
            self.fbbkeepst()
        )
    }
}
#[doc = "product ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProductId(pub u32);
impl ProductId {
    #[doc = "This register contains the product ID which is unique for each part number."]
    #[must_use]
    #[inline(always)]
    pub const fn product_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register contains the product ID which is unique for each part number."]
    #[inline(always)]
    pub const fn set_product_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ProductId {
    #[inline(always)]
    fn default() -> ProductId {
        ProductId(0)
    }
}
impl core::fmt::Debug for ProductId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ProductId")
            .field("product_id", &self.product_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProductId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ProductId {{ product_id: {=u16:?} }}", self.product_id())
    }
}
#[doc = "sdio pad ctrl"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdiopadctl(pub u32);
impl Sdiopadctl {
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn rasrcn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn fastfrz(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn freeze(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn comptq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn compen(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Drives SDIO Pad Compensation Circuit"]
    #[inline(always)]
    pub const fn set_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[must_use]
    #[inline(always)]
    pub const fn nasrcn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub const fn set_nasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[must_use]
    #[inline(always)]
    pub const fn nasrcp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub const fn set_nasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[must_use]
    #[inline(always)]
    pub const fn compok(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "SDIO Pad Compensation Circuit Status"]
    #[inline(always)]
    pub const fn set_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Sdiopadctl {
    #[inline(always)]
    fn default() -> Sdiopadctl {
        Sdiopadctl(0)
    }
}
impl core::fmt::Debug for Sdiopadctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdiopadctl")
            .field("rasrcn", &self.rasrcn())
            .field("rasrcp", &self.rasrcp())
            .field("fastfrz", &self.fastfrz())
            .field("freeze", &self.freeze())
            .field("comptq", &self.comptq())
            .field("compen", &self.compen())
            .field("nasrcn", &self.nasrcn())
            .field("nasrcp", &self.nasrcp())
            .field("compok", &self.compok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdiopadctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdiopadctl {{ rasrcn: {=u8:?}, rasrcp: {=u8:?}, fastfrz: {=bool:?}, freeze: {=bool:?}, comptq: {=bool:?}, compen: {=bool:?}, nasrcn: {=u8:?}, nasrcp: {=u8:?}, compok: {=bool:?} }}",
            self.rasrcn(),
            self.rasrcp(),
            self.fastfrz(),
            self.freeze(),
            self.comptq(),
            self.compen(),
            self.nasrcn(),
            self.nasrcp(),
            self.compok()
        )
    }
}
#[doc = "SILICONREV ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SiliconrevId(pub u32);
impl SiliconrevId {
    #[doc = "Silicon revision minor tag. (IE, 0, 2, 3, etc)"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Silicon revision minor tag. (IE, 0, 2, 3, etc)"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Silicon revision major tag. (IE, A, B, C, etc)"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Silicon revision major tag. (IE, A, B, C, etc)"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for SiliconrevId {
    #[inline(always)]
    fn default() -> SiliconrevId {
        SiliconrevId(0)
    }
}
impl core::fmt::Debug for SiliconrevId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SiliconrevId")
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SiliconrevId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SiliconrevId {{ minor: {=u8:?}, major: {=u8:?} }}",
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Start enable 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten0(pub u32);
impl Starten0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_wdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mrt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sct0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sct0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm14(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm15(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmic0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hypervisor(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hypervisor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn secureviolation(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_secureviolation(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hwvad0(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hwvad0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Starten0 {
    #[inline(always)]
    fn default() -> Starten0 {
        Starten0(0)
    }
}
impl core::fmt::Debug for Starten0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten0")
            .field("wdt0", &self.wdt0())
            .field("dmac0", &self.dmac0())
            .field("nshsgpio_int0", &self.nshsgpio_int0())
            .field("nshsgpio_int1", &self.nshsgpio_int1())
            .field("gpio_int0_irq0", &self.gpio_int0_irq0())
            .field("gpio_int0_irq1", &self.gpio_int0_irq1())
            .field("gpio_int0_irq2", &self.gpio_int0_irq2())
            .field("gpio_int0_irq3", &self.gpio_int0_irq3())
            .field("utick0", &self.utick0())
            .field("mrt0", &self.mrt0())
            .field("ct32bit0", &self.ct32bit0())
            .field("ct32bit1", &self.ct32bit1())
            .field("sct0", &self.sct0())
            .field("ct32bit3", &self.ct32bit3())
            .field("flexcomm0", &self.flexcomm0())
            .field("flexcomm1", &self.flexcomm1())
            .field("flexcomm2", &self.flexcomm2())
            .field("flexcomm3", &self.flexcomm3())
            .field("flexcomm4", &self.flexcomm4())
            .field("flexcomm5", &self.flexcomm5())
            .field("flexcomm14", &self.flexcomm14())
            .field("flexcomm15", &self.flexcomm15())
            .field("adc0", &self.adc0())
            .field("acmp", &self.acmp())
            .field("dmic0", &self.dmic0())
            .field("hypervisor", &self.hypervisor())
            .field("secureviolation", &self.secureviolation())
            .field("hwvad0", &self.hwvad0())
            .field("rng", &self.rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten0 {{ wdt0: {=bool:?}, dmac0: {=bool:?}, nshsgpio_int0: {=bool:?}, nshsgpio_int1: {=bool:?}, gpio_int0_irq0: {=bool:?}, gpio_int0_irq1: {=bool:?}, gpio_int0_irq2: {=bool:?}, gpio_int0_irq3: {=bool:?}, utick0: {=bool:?}, mrt0: {=bool:?}, ct32bit0: {=bool:?}, ct32bit1: {=bool:?}, sct0: {=bool:?}, ct32bit3: {=bool:?}, flexcomm0: {=bool:?}, flexcomm1: {=bool:?}, flexcomm2: {=bool:?}, flexcomm3: {=bool:?}, flexcomm4: {=bool:?}, flexcomm5: {=bool:?}, flexcomm14: {=bool:?}, flexcomm15: {=bool:?}, adc0: {=bool:?}, acmp: {=bool:?}, dmic0: {=bool:?}, hypervisor: {=bool:?}, secureviolation: {=bool:?}, hwvad0: {=bool:?}, rng: {=bool:?} }}",
            self.wdt0(),
            self.dmac0(),
            self.nshsgpio_int0(),
            self.nshsgpio_int1(),
            self.gpio_int0_irq0(),
            self.gpio_int0_irq1(),
            self.gpio_int0_irq2(),
            self.gpio_int0_irq3(),
            self.utick0(),
            self.mrt0(),
            self.ct32bit0(),
            self.ct32bit1(),
            self.sct0(),
            self.ct32bit3(),
            self.flexcomm0(),
            self.flexcomm1(),
            self.flexcomm2(),
            self.flexcomm3(),
            self.flexcomm4(),
            self.flexcomm5(),
            self.flexcomm14(),
            self.flexcomm15(),
            self.adc0(),
            self.acmp(),
            self.dmic0(),
            self.hypervisor(),
            self.secureviolation(),
            self.hwvad0(),
            self.rng()
        )
    }
}
#[doc = "Start enable 0 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten0Clr(pub u32);
impl Starten0Clr {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt0(&self) -> super::vals::Starten0ClrWdt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Starten0ClrWdt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_wdt0(&mut self, val: super::vals::Starten0ClrWdt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0(&self) -> super::vals::Starten0ClrDmac0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Starten0ClrDmac0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac0(&mut self, val: super::vals::Starten0ClrDmac0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int0(&self) -> super::vals::Starten0ClrNshsgpioInt0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Starten0ClrNshsgpioInt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int0(&mut self, val: super::vals::Starten0ClrNshsgpioInt0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int1(&self) -> super::vals::Starten0ClrNshsgpioInt1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Starten0ClrNshsgpioInt1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int1(&mut self, val: super::vals::Starten0ClrNshsgpioInt1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq0(&self) -> super::vals::Starten0ClrGpioInt0Irq0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Starten0ClrGpioInt0Irq0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq0(&mut self, val: super::vals::Starten0ClrGpioInt0Irq0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq1(&self) -> super::vals::Starten0ClrGpioInt0Irq1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Starten0ClrGpioInt0Irq1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq1(&mut self, val: super::vals::Starten0ClrGpioInt0Irq1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq2(&self) -> super::vals::Starten0ClrGpioInt0Irq2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Starten0ClrGpioInt0Irq2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq2(&mut self, val: super::vals::Starten0ClrGpioInt0Irq2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq3(&self) -> super::vals::Starten0ClrGpioInt0Irq3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Starten0ClrGpioInt0Irq3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq3(&mut self, val: super::vals::Starten0ClrGpioInt0Irq3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> super::vals::Starten0ClrUtick0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Starten0ClrUtick0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: super::vals::Starten0ClrUtick0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0(&self) -> super::vals::Starten0ClrMrt0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Starten0ClrMrt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mrt0(&mut self, val: super::vals::Starten0ClrMrt0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0(&self) -> super::vals::Starten0ClrCt32bit0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Starten0ClrCt32bit0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit0(&mut self, val: super::vals::Starten0ClrCt32bit0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1(&self) -> super::vals::Starten0ClrCt32bit1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Starten0ClrCt32bit1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit1(&mut self, val: super::vals::Starten0ClrCt32bit1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sct0(&self) -> super::vals::Starten0ClrSct0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Starten0ClrSct0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sct0(&mut self, val: super::vals::Starten0ClrSct0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3(&self) -> super::vals::Starten0ClrCt32bit3 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Starten0ClrCt32bit3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit3(&mut self, val: super::vals::Starten0ClrCt32bit3) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0(&self) -> super::vals::Starten0ClrFlexcomm0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Starten0ClrFlexcomm0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm0(&mut self, val: super::vals::Starten0ClrFlexcomm0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1(&self) -> super::vals::Starten0ClrFlexcomm1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Starten0ClrFlexcomm1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm1(&mut self, val: super::vals::Starten0ClrFlexcomm1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2(&self) -> super::vals::Starten0ClrFlexcomm2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Starten0ClrFlexcomm2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm2(&mut self, val: super::vals::Starten0ClrFlexcomm2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3(&self) -> super::vals::Starten0ClrFlexcomm3 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Starten0ClrFlexcomm3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm3(&mut self, val: super::vals::Starten0ClrFlexcomm3) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4(&self) -> super::vals::Starten0ClrFlexcomm4 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Starten0ClrFlexcomm4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm4(&mut self, val: super::vals::Starten0ClrFlexcomm4) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5(&self) -> super::vals::Starten0ClrFlexcomm5 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Starten0ClrFlexcomm5::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm5(&mut self, val: super::vals::Starten0ClrFlexcomm5) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm14(&self) -> super::vals::Starten0ClrFlexcomm14 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Starten0ClrFlexcomm14::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm14(&mut self, val: super::vals::Starten0ClrFlexcomm14) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm15(&self) -> super::vals::Starten0ClrFlexcomm15 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Starten0ClrFlexcomm15::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm15(&mut self, val: super::vals::Starten0ClrFlexcomm15) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Starten0ClrAdc0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Starten0ClrAdc0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Starten0ClrAdc0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self) -> super::vals::Starten0ClrAcmp {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Starten0ClrAcmp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp(&mut self, val: super::vals::Starten0ClrAcmp) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0(&self) -> super::vals::Starten0ClrDmic0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Starten0ClrDmic0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmic0(&mut self, val: super::vals::Starten0ClrDmic0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hypervisor(&self) -> super::vals::Starten0ClrHypervisor {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Starten0ClrHypervisor::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hypervisor(&mut self, val: super::vals::Starten0ClrHypervisor) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn secureviolation(&self) -> super::vals::Starten0ClrSecureviolation {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Starten0ClrSecureviolation::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_secureviolation(&mut self, val: super::vals::Starten0ClrSecureviolation) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hwvad0(&self) -> super::vals::Starten0ClrHwvad0 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Starten0ClrHwvad0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hwvad0(&mut self, val: super::vals::Starten0ClrHwvad0) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> super::vals::Starten0ClrRng {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Starten0ClrRng::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: super::vals::Starten0ClrRng) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Starten0Clr {
    #[inline(always)]
    fn default() -> Starten0Clr {
        Starten0Clr(0)
    }
}
impl core::fmt::Debug for Starten0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten0Clr")
            .field("wdt0", &self.wdt0())
            .field("dmac0", &self.dmac0())
            .field("nshsgpio_int0", &self.nshsgpio_int0())
            .field("nshsgpio_int1", &self.nshsgpio_int1())
            .field("gpio_int0_irq0", &self.gpio_int0_irq0())
            .field("gpio_int0_irq1", &self.gpio_int0_irq1())
            .field("gpio_int0_irq2", &self.gpio_int0_irq2())
            .field("gpio_int0_irq3", &self.gpio_int0_irq3())
            .field("utick0", &self.utick0())
            .field("mrt0", &self.mrt0())
            .field("ct32bit0", &self.ct32bit0())
            .field("ct32bit1", &self.ct32bit1())
            .field("sct0", &self.sct0())
            .field("ct32bit3", &self.ct32bit3())
            .field("flexcomm0", &self.flexcomm0())
            .field("flexcomm1", &self.flexcomm1())
            .field("flexcomm2", &self.flexcomm2())
            .field("flexcomm3", &self.flexcomm3())
            .field("flexcomm4", &self.flexcomm4())
            .field("flexcomm5", &self.flexcomm5())
            .field("flexcomm14", &self.flexcomm14())
            .field("flexcomm15", &self.flexcomm15())
            .field("adc0", &self.adc0())
            .field("acmp", &self.acmp())
            .field("dmic0", &self.dmic0())
            .field("hypervisor", &self.hypervisor())
            .field("secureviolation", &self.secureviolation())
            .field("hwvad0", &self.hwvad0())
            .field("rng", &self.rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten0Clr {{ wdt0: {:?}, dmac0: {:?}, nshsgpio_int0: {:?}, nshsgpio_int1: {:?}, gpio_int0_irq0: {:?}, gpio_int0_irq1: {:?}, gpio_int0_irq2: {:?}, gpio_int0_irq3: {:?}, utick0: {:?}, mrt0: {:?}, ct32bit0: {:?}, ct32bit1: {:?}, sct0: {:?}, ct32bit3: {:?}, flexcomm0: {:?}, flexcomm1: {:?}, flexcomm2: {:?}, flexcomm3: {:?}, flexcomm4: {:?}, flexcomm5: {:?}, flexcomm14: {:?}, flexcomm15: {:?}, adc0: {:?}, acmp: {:?}, dmic0: {:?}, hypervisor: {:?}, secureviolation: {:?}, hwvad0: {:?}, rng: {:?} }}",
            self.wdt0(),
            self.dmac0(),
            self.nshsgpio_int0(),
            self.nshsgpio_int1(),
            self.gpio_int0_irq0(),
            self.gpio_int0_irq1(),
            self.gpio_int0_irq2(),
            self.gpio_int0_irq3(),
            self.utick0(),
            self.mrt0(),
            self.ct32bit0(),
            self.ct32bit1(),
            self.sct0(),
            self.ct32bit3(),
            self.flexcomm0(),
            self.flexcomm1(),
            self.flexcomm2(),
            self.flexcomm3(),
            self.flexcomm4(),
            self.flexcomm5(),
            self.flexcomm14(),
            self.flexcomm15(),
            self.adc0(),
            self.acmp(),
            self.dmic0(),
            self.hypervisor(),
            self.secureviolation(),
            self.hwvad0(),
            self.rng()
        )
    }
}
#[doc = "Start enable 0 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten0Set(pub u32);
impl Starten0Set {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt0(&self) -> super::vals::Starten0SetWdt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Starten0SetWdt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_wdt0(&mut self, val: super::vals::Starten0SetWdt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac0(&self) -> super::vals::Starten0SetDmac0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Starten0SetDmac0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac0(&mut self, val: super::vals::Starten0SetDmac0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int0(&self) -> super::vals::Starten0SetNshsgpioInt0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Starten0SetNshsgpioInt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int0(&mut self, val: super::vals::Starten0SetNshsgpioInt0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn nshsgpio_int1(&self) -> super::vals::Starten0SetNshsgpioInt1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Starten0SetNshsgpioInt1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_nshsgpio_int1(&mut self, val: super::vals::Starten0SetNshsgpioInt1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq0(&self) -> super::vals::Starten0SetGpioInt0Irq0 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Starten0SetGpioInt0Irq0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq0(&mut self, val: super::vals::Starten0SetGpioInt0Irq0) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq1(&self) -> super::vals::Starten0SetGpioInt0Irq1 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Starten0SetGpioInt0Irq1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq1(&mut self, val: super::vals::Starten0SetGpioInt0Irq1) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq2(&self) -> super::vals::Starten0SetGpioInt0Irq2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Starten0SetGpioInt0Irq2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq2(&mut self, val: super::vals::Starten0SetGpioInt0Irq2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq3(&self) -> super::vals::Starten0SetGpioInt0Irq3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Starten0SetGpioInt0Irq3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq3(&mut self, val: super::vals::Starten0SetGpioInt0Irq3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn utick0(&self) -> super::vals::Starten0SetUtick0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Starten0SetUtick0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_utick0(&mut self, val: super::vals::Starten0SetUtick0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mrt0(&self) -> super::vals::Starten0SetMrt0 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Starten0SetMrt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mrt0(&mut self, val: super::vals::Starten0SetMrt0) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit0(&self) -> super::vals::Starten0SetCt32bit0 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Starten0SetCt32bit0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit0(&mut self, val: super::vals::Starten0SetCt32bit0) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit1(&self) -> super::vals::Starten0SetCt32bit1 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Starten0SetCt32bit1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit1(&mut self, val: super::vals::Starten0SetCt32bit1) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sct0(&self) -> super::vals::Starten0SetSct0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Starten0SetSct0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sct0(&mut self, val: super::vals::Starten0SetSct0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit3(&self) -> super::vals::Starten0SetCt32bit3 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Starten0SetCt32bit3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit3(&mut self, val: super::vals::Starten0SetCt32bit3) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm0(&self) -> super::vals::Starten0SetFlexcomm0 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Starten0SetFlexcomm0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm0(&mut self, val: super::vals::Starten0SetFlexcomm0) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm1(&self) -> super::vals::Starten0SetFlexcomm1 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Starten0SetFlexcomm1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm1(&mut self, val: super::vals::Starten0SetFlexcomm1) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm2(&self) -> super::vals::Starten0SetFlexcomm2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Starten0SetFlexcomm2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm2(&mut self, val: super::vals::Starten0SetFlexcomm2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm3(&self) -> super::vals::Starten0SetFlexcomm3 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Starten0SetFlexcomm3::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm3(&mut self, val: super::vals::Starten0SetFlexcomm3) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm4(&self) -> super::vals::Starten0SetFlexcomm4 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Starten0SetFlexcomm4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm4(&mut self, val: super::vals::Starten0SetFlexcomm4) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm5(&self) -> super::vals::Starten0SetFlexcomm5 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Starten0SetFlexcomm5::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm5(&mut self, val: super::vals::Starten0SetFlexcomm5) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm14(&self) -> super::vals::Starten0SetFlexcomm14 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Starten0SetFlexcomm14::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm14(&mut self, val: super::vals::Starten0SetFlexcomm14) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm15(&self) -> super::vals::Starten0SetFlexcomm15 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Starten0SetFlexcomm15::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm15(&mut self, val: super::vals::Starten0SetFlexcomm15) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> super::vals::Starten0SetAdc0 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Starten0SetAdc0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: super::vals::Starten0SetAdc0) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp(&self) -> super::vals::Starten0SetAcmp {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Starten0SetAcmp::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_acmp(&mut self, val: super::vals::Starten0SetAcmp) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmic0(&self) -> super::vals::Starten0SetDmic0 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Starten0SetDmic0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmic0(&mut self, val: super::vals::Starten0SetDmic0) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hypervisor(&self) -> super::vals::Starten0SetHypervisor {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Starten0SetHypervisor::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hypervisor(&mut self, val: super::vals::Starten0SetHypervisor) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn secureviolation(&self) -> super::vals::Starten0SetSecureviolation {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Starten0SetSecureviolation::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_secureviolation(&mut self, val: super::vals::Starten0SetSecureviolation) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn hwvad0(&self) -> super::vals::Starten0SetHwvad0 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Starten0SetHwvad0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_hwvad0(&mut self, val: super::vals::Starten0SetHwvad0) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rng(&self) -> super::vals::Starten0SetRng {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Starten0SetRng::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rng(&mut self, val: super::vals::Starten0SetRng) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Starten0Set {
    #[inline(always)]
    fn default() -> Starten0Set {
        Starten0Set(0)
    }
}
impl core::fmt::Debug for Starten0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten0Set")
            .field("wdt0", &self.wdt0())
            .field("dmac0", &self.dmac0())
            .field("nshsgpio_int0", &self.nshsgpio_int0())
            .field("nshsgpio_int1", &self.nshsgpio_int1())
            .field("gpio_int0_irq0", &self.gpio_int0_irq0())
            .field("gpio_int0_irq1", &self.gpio_int0_irq1())
            .field("gpio_int0_irq2", &self.gpio_int0_irq2())
            .field("gpio_int0_irq3", &self.gpio_int0_irq3())
            .field("utick0", &self.utick0())
            .field("mrt0", &self.mrt0())
            .field("ct32bit0", &self.ct32bit0())
            .field("ct32bit1", &self.ct32bit1())
            .field("sct0", &self.sct0())
            .field("ct32bit3", &self.ct32bit3())
            .field("flexcomm0", &self.flexcomm0())
            .field("flexcomm1", &self.flexcomm1())
            .field("flexcomm2", &self.flexcomm2())
            .field("flexcomm3", &self.flexcomm3())
            .field("flexcomm4", &self.flexcomm4())
            .field("flexcomm5", &self.flexcomm5())
            .field("flexcomm14", &self.flexcomm14())
            .field("flexcomm15", &self.flexcomm15())
            .field("adc0", &self.adc0())
            .field("acmp", &self.acmp())
            .field("dmic0", &self.dmic0())
            .field("hypervisor", &self.hypervisor())
            .field("secureviolation", &self.secureviolation())
            .field("hwvad0", &self.hwvad0())
            .field("rng", &self.rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten0Set {{ wdt0: {:?}, dmac0: {:?}, nshsgpio_int0: {:?}, nshsgpio_int1: {:?}, gpio_int0_irq0: {:?}, gpio_int0_irq1: {:?}, gpio_int0_irq2: {:?}, gpio_int0_irq3: {:?}, utick0: {:?}, mrt0: {:?}, ct32bit0: {:?}, ct32bit1: {:?}, sct0: {:?}, ct32bit3: {:?}, flexcomm0: {:?}, flexcomm1: {:?}, flexcomm2: {:?}, flexcomm3: {:?}, flexcomm4: {:?}, flexcomm5: {:?}, flexcomm14: {:?}, flexcomm15: {:?}, adc0: {:?}, acmp: {:?}, dmic0: {:?}, hypervisor: {:?}, secureviolation: {:?}, hwvad0: {:?}, rng: {:?} }}",
            self.wdt0(),
            self.dmac0(),
            self.nshsgpio_int0(),
            self.nshsgpio_int1(),
            self.gpio_int0_irq0(),
            self.gpio_int0_irq1(),
            self.gpio_int0_irq2(),
            self.gpio_int0_irq3(),
            self.utick0(),
            self.mrt0(),
            self.ct32bit0(),
            self.ct32bit1(),
            self.sct0(),
            self.ct32bit3(),
            self.flexcomm0(),
            self.flexcomm1(),
            self.flexcomm2(),
            self.flexcomm3(),
            self.flexcomm4(),
            self.flexcomm5(),
            self.flexcomm14(),
            self.flexcomm15(),
            self.adc0(),
            self.acmp(),
            self.dmic0(),
            self.hypervisor(),
            self.secureviolation(),
            self.hwvad0(),
            self.rng()
        )
    }
}
#[doc = "Start enable 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten1(pub u32);
impl Starten1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite0_alarm_or_wakeup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rtc_lite0_alarm_or_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq4(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq5(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq6(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq7(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_wu(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_os_event_timer_wu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_irq(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_needclk(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_needclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sha(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sha(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Starten1 {
    #[inline(always)]
    fn default() -> Starten1 {
        Starten1(0)
    }
}
impl core::fmt::Debug for Starten1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten1")
            .field(
                "rtc_lite0_alarm_or_wakeup",
                &self.rtc_lite0_alarm_or_wakeup(),
            )
            .field("mu", &self.mu())
            .field("gpio_int0_irq4", &self.gpio_int0_irq4())
            .field("gpio_int0_irq5", &self.gpio_int0_irq5())
            .field("gpio_int0_irq6", &self.gpio_int0_irq6())
            .field("gpio_int0_irq7", &self.gpio_int0_irq7())
            .field("ct32bit2", &self.ct32bit2())
            .field("ct32bit4", &self.ct32bit4())
            .field("os_event_timer_wu", &self.os_event_timer_wu())
            .field("flexspi", &self.flexspi())
            .field("flexcomm6", &self.flexcomm6())
            .field("flexcomm7", &self.flexcomm7())
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("shsgpio_int0", &self.shsgpio_int0())
            .field("shsgpio_int1", &self.shsgpio_int1())
            .field("i3c0", &self.i3c0())
            .field("usb_irq", &self.usb_irq())
            .field("usb_needclk", &self.usb_needclk())
            .field("dmac1", &self.dmac1())
            .field("puf", &self.puf())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("pmic", &self.pmic())
            .field("sha", &self.sha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten1 {{ rtc_lite0_alarm_or_wakeup: {=bool:?}, mu: {=bool:?}, gpio_int0_irq4: {=bool:?}, gpio_int0_irq5: {=bool:?}, gpio_int0_irq6: {=bool:?}, gpio_int0_irq7: {=bool:?}, ct32bit2: {=bool:?}, ct32bit4: {=bool:?}, os_event_timer_wu: {=bool:?}, flexspi: {=bool:?}, flexcomm6: {=bool:?}, flexcomm7: {=bool:?}, sdio0: {=bool:?}, sdio1: {=bool:?}, shsgpio_int0: {=bool:?}, shsgpio_int1: {=bool:?}, i3c0: {=bool:?}, usb_irq: {=bool:?}, usb_needclk: {=bool:?}, dmac1: {=bool:?}, puf: {=bool:?}, powerquad: {=bool:?}, casper: {=bool:?}, pmic: {=bool:?}, sha: {=bool:?} }}",
            self.rtc_lite0_alarm_or_wakeup(),
            self.mu(),
            self.gpio_int0_irq4(),
            self.gpio_int0_irq5(),
            self.gpio_int0_irq6(),
            self.gpio_int0_irq7(),
            self.ct32bit2(),
            self.ct32bit4(),
            self.os_event_timer_wu(),
            self.flexspi(),
            self.flexcomm6(),
            self.flexcomm7(),
            self.sdio0(),
            self.sdio1(),
            self.shsgpio_int0(),
            self.shsgpio_int1(),
            self.i3c0(),
            self.usb_irq(),
            self.usb_needclk(),
            self.dmac1(),
            self.puf(),
            self.powerquad(),
            self.casper(),
            self.pmic(),
            self.sha()
        )
    }
}
#[doc = "Start enable 1 clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten1Clr(pub u32);
impl Starten1Clr {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite0_alarm_or_wakeup(&self) -> super::vals::Starten1ClrRtcLite0AlarmOrWakeup {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Starten1ClrRtcLite0AlarmOrWakeup::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rtc_lite0_alarm_or_wakeup(
        &mut self,
        val: super::vals::Starten1ClrRtcLite0AlarmOrWakeup,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mu(&self) -> super::vals::Starten1ClrMu {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Starten1ClrMu::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mu(&mut self, val: super::vals::Starten1ClrMu) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq4(&self) -> super::vals::Starten1ClrGpioInt0Irq4 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Starten1ClrGpioInt0Irq4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq4(&mut self, val: super::vals::Starten1ClrGpioInt0Irq4) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq5(&self) -> super::vals::Starten1ClrGpioInt0Irq5 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Starten1ClrGpioInt0Irq5::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq5(&mut self, val: super::vals::Starten1ClrGpioInt0Irq5) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq6(&self) -> super::vals::Starten1ClrGpioInt0Irq6 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Starten1ClrGpioInt0Irq6::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq6(&mut self, val: super::vals::Starten1ClrGpioInt0Irq6) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq7(&self) -> super::vals::Starten1ClrGpioInt0Irq7 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Starten1ClrGpioInt0Irq7::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq7(&mut self, val: super::vals::Starten1ClrGpioInt0Irq7) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2(&self) -> super::vals::Starten1ClrCt32bit2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Starten1ClrCt32bit2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit2(&mut self, val: super::vals::Starten1ClrCt32bit2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4(&self) -> super::vals::Starten1ClrCt32bit4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Starten1ClrCt32bit4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit4(&mut self, val: super::vals::Starten1ClrCt32bit4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_wu(&self) -> super::vals::Starten1ClrOsEventTimerWu {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Starten1ClrOsEventTimerWu::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_os_event_timer_wu(&mut self, val: super::vals::Starten1ClrOsEventTimerWu) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> super::vals::Starten1ClrFlexspi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Starten1ClrFlexspi::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: super::vals::Starten1ClrFlexspi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6(&self) -> super::vals::Starten1ClrFlexcomm6 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Starten1ClrFlexcomm6::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm6(&mut self, val: super::vals::Starten1ClrFlexcomm6) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7(&self) -> super::vals::Starten1ClrFlexcomm7 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Starten1ClrFlexcomm7::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm7(&mut self, val: super::vals::Starten1ClrFlexcomm7) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> super::vals::Starten1ClrSdio0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Starten1ClrSdio0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: super::vals::Starten1ClrSdio0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> super::vals::Starten1ClrSdio1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Starten1ClrSdio1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: super::vals::Starten1ClrSdio1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int0(&self) -> super::vals::Starten1ClrShsgpioInt0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Starten1ClrShsgpioInt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int0(&mut self, val: super::vals::Starten1ClrShsgpioInt0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int1(&self) -> super::vals::Starten1ClrShsgpioInt1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Starten1ClrShsgpioInt1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int1(&mut self, val: super::vals::Starten1ClrShsgpioInt1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> super::vals::Starten1ClrI3c0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Starten1ClrI3c0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: super::vals::Starten1ClrI3c0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_irq(&self) -> super::vals::Starten1ClrUsbIrq {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Starten1ClrUsbIrq::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_irq(&mut self, val: super::vals::Starten1ClrUsbIrq) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_needclk(&self) -> super::vals::Starten1ClrUsbNeedclk {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Starten1ClrUsbNeedclk::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_needclk(&mut self, val: super::vals::Starten1ClrUsbNeedclk) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1(&self) -> super::vals::Starten1ClrDmac1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Starten1ClrDmac1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac1(&mut self, val: super::vals::Starten1ClrDmac1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> super::vals::Starten1ClrPuf {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Starten1ClrPuf::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: super::vals::Starten1ClrPuf) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> super::vals::Starten1ClrPowerquad {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Starten1ClrPowerquad::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: super::vals::Starten1ClrPowerquad) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Starten1ClrCasper {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Starten1ClrCasper::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Starten1ClrCasper) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic(&self) -> super::vals::Starten1ClrPmic {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Starten1ClrPmic::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic(&mut self, val: super::vals::Starten1ClrPmic) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sha(&self) -> super::vals::Starten1ClrSha {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Starten1ClrSha::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sha(&mut self, val: super::vals::Starten1ClrSha) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Starten1Clr {
    #[inline(always)]
    fn default() -> Starten1Clr {
        Starten1Clr(0)
    }
}
impl core::fmt::Debug for Starten1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten1Clr")
            .field(
                "rtc_lite0_alarm_or_wakeup",
                &self.rtc_lite0_alarm_or_wakeup(),
            )
            .field("mu", &self.mu())
            .field("gpio_int0_irq4", &self.gpio_int0_irq4())
            .field("gpio_int0_irq5", &self.gpio_int0_irq5())
            .field("gpio_int0_irq6", &self.gpio_int0_irq6())
            .field("gpio_int0_irq7", &self.gpio_int0_irq7())
            .field("ct32bit2", &self.ct32bit2())
            .field("ct32bit4", &self.ct32bit4())
            .field("os_event_timer_wu", &self.os_event_timer_wu())
            .field("flexspi", &self.flexspi())
            .field("flexcomm6", &self.flexcomm6())
            .field("flexcomm7", &self.flexcomm7())
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("shsgpio_int0", &self.shsgpio_int0())
            .field("shsgpio_int1", &self.shsgpio_int1())
            .field("i3c0", &self.i3c0())
            .field("usb_irq", &self.usb_irq())
            .field("usb_needclk", &self.usb_needclk())
            .field("dmac1", &self.dmac1())
            .field("puf", &self.puf())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("pmic", &self.pmic())
            .field("sha", &self.sha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten1Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten1Clr {{ rtc_lite0_alarm_or_wakeup: {:?}, mu: {:?}, gpio_int0_irq4: {:?}, gpio_int0_irq5: {:?}, gpio_int0_irq6: {:?}, gpio_int0_irq7: {:?}, ct32bit2: {:?}, ct32bit4: {:?}, os_event_timer_wu: {:?}, flexspi: {:?}, flexcomm6: {:?}, flexcomm7: {:?}, sdio0: {:?}, sdio1: {:?}, shsgpio_int0: {:?}, shsgpio_int1: {:?}, i3c0: {:?}, usb_irq: {:?}, usb_needclk: {:?}, dmac1: {:?}, puf: {:?}, powerquad: {:?}, casper: {:?}, pmic: {:?}, sha: {:?} }}",
            self.rtc_lite0_alarm_or_wakeup(),
            self.mu(),
            self.gpio_int0_irq4(),
            self.gpio_int0_irq5(),
            self.gpio_int0_irq6(),
            self.gpio_int0_irq7(),
            self.ct32bit2(),
            self.ct32bit4(),
            self.os_event_timer_wu(),
            self.flexspi(),
            self.flexcomm6(),
            self.flexcomm7(),
            self.sdio0(),
            self.sdio1(),
            self.shsgpio_int0(),
            self.shsgpio_int1(),
            self.i3c0(),
            self.usb_irq(),
            self.usb_needclk(),
            self.dmac1(),
            self.puf(),
            self.powerquad(),
            self.casper(),
            self.pmic(),
            self.sha()
        )
    }
}
#[doc = "Start enable 1 set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Starten1Set(pub u32);
impl Starten1Set {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_lite0_alarm_or_wakeup(&self) -> super::vals::Starten1SetRtcLite0AlarmOrWakeup {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Starten1SetRtcLite0AlarmOrWakeup::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_rtc_lite0_alarm_or_wakeup(
        &mut self,
        val: super::vals::Starten1SetRtcLite0AlarmOrWakeup,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn mu(&self) -> super::vals::Starten1SetMu {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Starten1SetMu::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_mu(&mut self, val: super::vals::Starten1SetMu) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq4(&self) -> super::vals::Starten1SetGpioInt0Irq4 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Starten1SetGpioInt0Irq4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq4(&mut self, val: super::vals::Starten1SetGpioInt0Irq4) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq5(&self) -> super::vals::Starten1SetGpioInt0Irq5 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Starten1SetGpioInt0Irq5::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq5(&mut self, val: super::vals::Starten1SetGpioInt0Irq5) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq6(&self) -> super::vals::Starten1SetGpioInt0Irq6 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Starten1SetGpioInt0Irq6::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq6(&mut self, val: super::vals::Starten1SetGpioInt0Irq6) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_int0_irq7(&self) -> super::vals::Starten1SetGpioInt0Irq7 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Starten1SetGpioInt0Irq7::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_gpio_int0_irq7(&mut self, val: super::vals::Starten1SetGpioInt0Irq7) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit2(&self) -> super::vals::Starten1SetCt32bit2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Starten1SetCt32bit2::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit2(&mut self, val: super::vals::Starten1SetCt32bit2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn ct32bit4(&self) -> super::vals::Starten1SetCt32bit4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Starten1SetCt32bit4::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_ct32bit4(&mut self, val: super::vals::Starten1SetCt32bit4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn os_event_timer_wu(&self) -> super::vals::Starten1SetOsEventTimerWu {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Starten1SetOsEventTimerWu::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_os_event_timer_wu(&mut self, val: super::vals::Starten1SetOsEventTimerWu) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> super::vals::Starten1SetFlexspi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Starten1SetFlexspi::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: super::vals::Starten1SetFlexspi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm6(&self) -> super::vals::Starten1SetFlexcomm6 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Starten1SetFlexcomm6::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm6(&mut self, val: super::vals::Starten1SetFlexcomm6) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn flexcomm7(&self) -> super::vals::Starten1SetFlexcomm7 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Starten1SetFlexcomm7::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_flexcomm7(&mut self, val: super::vals::Starten1SetFlexcomm7) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio0(&self) -> super::vals::Starten1SetSdio0 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Starten1SetSdio0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio0(&mut self, val: super::vals::Starten1SetSdio0) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio1(&self) -> super::vals::Starten1SetSdio1 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Starten1SetSdio1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sdio1(&mut self, val: super::vals::Starten1SetSdio1) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int0(&self) -> super::vals::Starten1SetShsgpioInt0 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Starten1SetShsgpioInt0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int0(&mut self, val: super::vals::Starten1SetShsgpioInt0) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn shsgpio_int1(&self) -> super::vals::Starten1SetShsgpioInt1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Starten1SetShsgpioInt1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_shsgpio_int1(&mut self, val: super::vals::Starten1SetShsgpioInt1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> super::vals::Starten1SetI3c0 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Starten1SetI3c0::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: super::vals::Starten1SetI3c0) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_irq(&self) -> super::vals::Starten1SetUsbIrq {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Starten1SetUsbIrq::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_irq(&mut self, val: super::vals::Starten1SetUsbIrq) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_needclk(&self) -> super::vals::Starten1SetUsbNeedclk {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Starten1SetUsbNeedclk::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_needclk(&mut self, val: super::vals::Starten1SetUsbNeedclk) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn dmac1(&self) -> super::vals::Starten1SetDmac1 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Starten1SetDmac1::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_dmac1(&mut self, val: super::vals::Starten1SetDmac1) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> super::vals::Starten1SetPuf {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Starten1SetPuf::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: super::vals::Starten1SetPuf) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn powerquad(&self) -> super::vals::Starten1SetPowerquad {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Starten1SetPowerquad::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_powerquad(&mut self, val: super::vals::Starten1SetPowerquad) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn casper(&self) -> super::vals::Starten1SetCasper {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Starten1SetCasper::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_casper(&mut self, val: super::vals::Starten1SetCasper) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic(&self) -> super::vals::Starten1SetPmic {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Starten1SetPmic::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_pmic(&mut self, val: super::vals::Starten1SetPmic) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn sha(&self) -> super::vals::Starten1SetSha {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Starten1SetSha::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_sha(&mut self, val: super::vals::Starten1SetSha) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Starten1Set {
    #[inline(always)]
    fn default() -> Starten1Set {
        Starten1Set(0)
    }
}
impl core::fmt::Debug for Starten1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Starten1Set")
            .field(
                "rtc_lite0_alarm_or_wakeup",
                &self.rtc_lite0_alarm_or_wakeup(),
            )
            .field("mu", &self.mu())
            .field("gpio_int0_irq4", &self.gpio_int0_irq4())
            .field("gpio_int0_irq5", &self.gpio_int0_irq5())
            .field("gpio_int0_irq6", &self.gpio_int0_irq6())
            .field("gpio_int0_irq7", &self.gpio_int0_irq7())
            .field("ct32bit2", &self.ct32bit2())
            .field("ct32bit4", &self.ct32bit4())
            .field("os_event_timer_wu", &self.os_event_timer_wu())
            .field("flexspi", &self.flexspi())
            .field("flexcomm6", &self.flexcomm6())
            .field("flexcomm7", &self.flexcomm7())
            .field("sdio0", &self.sdio0())
            .field("sdio1", &self.sdio1())
            .field("shsgpio_int0", &self.shsgpio_int0())
            .field("shsgpio_int1", &self.shsgpio_int1())
            .field("i3c0", &self.i3c0())
            .field("usb_irq", &self.usb_irq())
            .field("usb_needclk", &self.usb_needclk())
            .field("dmac1", &self.dmac1())
            .field("puf", &self.puf())
            .field("powerquad", &self.powerquad())
            .field("casper", &self.casper())
            .field("pmic", &self.pmic())
            .field("sha", &self.sha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Starten1Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Starten1Set {{ rtc_lite0_alarm_or_wakeup: {:?}, mu: {:?}, gpio_int0_irq4: {:?}, gpio_int0_irq5: {:?}, gpio_int0_irq6: {:?}, gpio_int0_irq7: {:?}, ct32bit2: {:?}, ct32bit4: {:?}, os_event_timer_wu: {:?}, flexspi: {:?}, flexcomm6: {:?}, flexcomm7: {:?}, sdio0: {:?}, sdio1: {:?}, shsgpio_int0: {:?}, shsgpio_int1: {:?}, i3c0: {:?}, usb_irq: {:?}, usb_needclk: {:?}, dmac1: {:?}, puf: {:?}, powerquad: {:?}, casper: {:?}, pmic: {:?}, sha: {:?} }}",
            self.rtc_lite0_alarm_or_wakeup(),
            self.mu(),
            self.gpio_int0_irq4(),
            self.gpio_int0_irq5(),
            self.gpio_int0_irq6(),
            self.gpio_int0_irq7(),
            self.ct32bit2(),
            self.ct32bit4(),
            self.os_event_timer_wu(),
            self.flexspi(),
            self.flexcomm6(),
            self.flexcomm7(),
            self.sdio0(),
            self.sdio1(),
            self.shsgpio_int0(),
            self.shsgpio_int1(),
            self.i3c0(),
            self.usb_irq(),
            self.usb_needclk(),
            self.dmac1(),
            self.puf(),
            self.powerquad(),
            self.casper(),
            self.pmic(),
            self.sha()
        )
    }
}
#[doc = "system nstick calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemNstickCalib(pub u32);
impl SystemNstickCalib {
    #[doc = "Selects the system non-secure tick calibration value of the M33."]
    #[must_use]
    #[inline(always)]
    pub const fn system_nstick_calib(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Selects the system non-secure tick calibration value of the M33."]
    #[inline(always)]
    pub const fn set_system_nstick_calib(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for SystemNstickCalib {
    #[inline(always)]
    fn default() -> SystemNstickCalib {
        SystemNstickCalib(0)
    }
}
impl core::fmt::Debug for SystemNstickCalib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemNstickCalib")
            .field("system_nstick_calib", &self.system_nstick_calib())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemNstickCalib {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystemNstickCalib {{ system_nstick_calib: {=u32:?} }}",
            self.system_nstick_calib()
        )
    }
}
#[doc = "system stick calibration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemStickCalib(pub u32);
impl SystemStickCalib {
    #[doc = "Selects the system secure tick calibration value of the M33."]
    #[must_use]
    #[inline(always)]
    pub const fn system_stick_calib(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Selects the system secure tick calibration value of the M33."]
    #[inline(always)]
    pub const fn set_system_stick_calib(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
    }
}
impl Default for SystemStickCalib {
    #[inline(always)]
    fn default() -> SystemStickCalib {
        SystemStickCalib(0)
    }
}
impl core::fmt::Debug for SystemStickCalib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SystemStickCalib")
            .field("system_stick_calib", &self.system_stick_calib())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SystemStickCalib {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SystemStickCalib {{ system_stick_calib: {=u32:?} }}",
            self.system_stick_calib()
        )
    }
}
#[doc = "tempsensor ctrl"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tempsensorctl(pub u32);
impl Tempsensorctl {
    #[doc = "Temperature Sensor Source. . ."]
    #[must_use]
    #[inline(always)]
    pub const fn tssrc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Temperature Sensor Source. . ."]
    #[inline(always)]
    pub const fn set_tssrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Tempsensorctl {
    #[inline(always)]
    fn default() -> Tempsensorctl {
        Tempsensorctl(0)
    }
}
impl core::fmt::Debug for Tempsensorctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tempsensorctl")
            .field("tssrc", &self.tssrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsensorctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tempsensorctl {{ tssrc: {=bool:?} }}", self.tssrc())
    }
}
#[doc = "USB clock control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbclkctrl(pub u32);
impl Usbclkctrl {
    #[doc = "USB0 Device need clock signal control"]
    #[must_use]
    #[inline(always)]
    pub const fn ap_dev_clk(&self) -> super::vals::ApDevClk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ApDevClk::from_bits(val as u8)
    }
    #[doc = "USB0 Device need clock signal control"]
    #[inline(always)]
    pub const fn set_ap_dev_clk(&mut self, val: super::vals::ApDevClk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_dev_clk(&self) -> super::vals::PolDevClk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PolDevClk::from_bits(val as u8)
    }
    #[doc = "USB0 Device need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub const fn set_pol_dev_clk(&mut self, val: super::vals::PolDevClk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "USB0 Host need clock signal control"]
    #[must_use]
    #[inline(always)]
    pub const fn ap_host_clk(&self) -> super::vals::ApHostClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ApHostClk::from_bits(val as u8)
    }
    #[doc = "USB0 Host need clock signal control"]
    #[inline(always)]
    pub const fn set_ap_host_clk(&mut self, val: super::vals::ApHostClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_host_clk(&self) -> super::vals::PolHostClk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PolHostClk::from_bits(val as u8)
    }
    #[doc = "USB0 HOST need clock polarity for triggering the USB1 wake-up interrupt"]
    #[inline(always)]
    pub const fn set_pol_host_clk(&mut self, val: super::vals::PolHostClk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
    #[must_use]
    #[inline(always)]
    pub const fn hs_dev_wakeup_n(&self) -> super::vals::HsDevWakeupN {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::HsDevWakeupN::from_bits(val as u8)
    }
    #[doc = "External user wake-up signal for device mode; asserting this signal (active low) will result in exiting the low power mode; input to asynchronous control logic"]
    #[inline(always)]
    pub const fn set_hs_dev_wakeup_n(&mut self, val: super::vals::HsDevWakeupN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Usbclkctrl {
    #[inline(always)]
    fn default() -> Usbclkctrl {
        Usbclkctrl(0)
    }
}
impl core::fmt::Debug for Usbclkctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbclkctrl")
            .field("ap_dev_clk", &self.ap_dev_clk())
            .field("pol_dev_clk", &self.pol_dev_clk())
            .field("ap_host_clk", &self.ap_host_clk())
            .field("pol_host_clk", &self.pol_host_clk())
            .field("hs_dev_wakeup_n", &self.hs_dev_wakeup_n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbclkctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbclkctrl {{ ap_dev_clk: {:?}, pol_dev_clk: {:?}, ap_host_clk: {:?}, pol_host_clk: {:?}, hs_dev_wakeup_n: {:?} }}",
            self.ap_dev_clk(),
            self.pol_dev_clk(),
            self.ap_host_clk(),
            self.pol_host_clk(),
            self.hs_dev_wakeup_n()
        )
    }
}
#[doc = "USB clock status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbclkstat(pub u32);
impl Usbclkstat {
    #[doc = "USB Device USB_NEEDCLK signal status:"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_need_clkst(&self) -> super::vals::DevNeedClkst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DevNeedClkst::from_bits(val as u8)
    }
    #[doc = "USB Device USB_NEEDCLK signal status:"]
    #[inline(always)]
    pub const fn set_dev_need_clkst(&mut self, val: super::vals::DevNeedClkst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Device Host USB_NEEDCLK signal status:"]
    #[must_use]
    #[inline(always)]
    pub const fn host_need_clkst(&self) -> super::vals::HostNeedClkst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::HostNeedClkst::from_bits(val as u8)
    }
    #[doc = "USB Device Host USB_NEEDCLK signal status:"]
    #[inline(always)]
    pub const fn set_host_need_clkst(&mut self, val: super::vals::HostNeedClkst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Usbclkstat {
    #[inline(always)]
    fn default() -> Usbclkstat {
        Usbclkstat(0)
    }
}
impl core::fmt::Debug for Usbclkstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbclkstat")
            .field("dev_need_clkst", &self.dev_need_clkst())
            .field("host_need_clkst", &self.host_need_clkst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbclkstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbclkstat {{ dev_need_clkst: {:?}, host_need_clkst: {:?} }}",
            self.dev_need_clkst(),
            self.host_need_clkst()
        )
    }
}
#[doc = "USB PHY PLL0 lock time division 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphypll0locktimediv2(pub u32);
impl Usbphypll0locktimediv2 {
    #[doc = "USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
    #[must_use]
    #[inline(always)]
    pub const fn locktimediv2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "USBPHYPLL0 Lock Time: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value"]
    #[inline(always)]
    pub const fn set_locktimediv2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Usbphypll0locktimediv2 {
    #[inline(always)]
    fn default() -> Usbphypll0locktimediv2 {
        Usbphypll0locktimediv2(0)
    }
}
impl core::fmt::Debug for Usbphypll0locktimediv2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbphypll0locktimediv2")
            .field("locktimediv2", &self.locktimediv2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbphypll0locktimediv2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbphypll0locktimediv2 {{ locktimediv2: {=u16:?} }}",
            self.locktimediv2()
        )
    }
}
#[doc = "UUIDn 32-Bit Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uuid(pub u32);
impl Uuid {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn uuid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_uuid(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Uuid {
    #[inline(always)]
    fn default() -> Uuid {
        Uuid(0)
    }
}
impl core::fmt::Debug for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uuid").field("uuid", &self.uuid()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uuid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Uuid {{ uuid: {=u32:?} }}", self.uuid())
    }
}
