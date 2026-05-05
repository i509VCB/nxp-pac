#[doc = "CRC mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODE(pub u32);
impl MODE {
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC_POLY(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial."]
    #[inline(always)]
    pub const fn set_CRC_POLY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)."]
    #[must_use]
    #[inline(always)]
    pub const fn BIT_RVS_WR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)."]
    #[inline(always)]
    pub const fn set_BIT_RVS_WR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPL_WR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA."]
    #[inline(always)]
    pub const fn set_CMPL_WR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM."]
    #[must_use]
    #[inline(always)]
    pub const fn BIT_RVS_SUM(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM."]
    #[inline(always)]
    pub const fn set_BIT_RVS_SUM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPL_SUM(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM."]
    #[inline(always)]
    pub const fn set_CMPL_SUM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for MODE {
    #[inline(always)]
    fn default() -> MODE {
        MODE(0)
    }
}
impl core::fmt::Debug for MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE")
            .field("CRC_POLY", &self.CRC_POLY())
            .field("BIT_RVS_WR", &self.BIT_RVS_WR())
            .field("CMPL_WR", &self.CMPL_WR())
            .field("BIT_RVS_SUM", &self.BIT_RVS_SUM())
            .field("CMPL_SUM", &self.CMPL_SUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODE {{ CRC_POLY: {=u8:?}, BIT_RVS_WR: {=bool:?}, CMPL_WR: {=bool:?}, BIT_RVS_SUM: {=bool:?}, CMPL_SUM: {=bool:?} }}",
            self.CRC_POLY(),
            self.BIT_RVS_WR(),
            self.CMPL_WR(),
            self.BIT_RVS_SUM(),
            self.CMPL_SUM()
        )
    }
}
#[doc = "CRC seed register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEED(pub u32);
impl SEED {
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC_SEED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub const fn set_CRC_SEED(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SEED {
    #[inline(always)]
    fn default() -> SEED {
        SEED(0)
    }
}
impl core::fmt::Debug for SEED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEED")
            .field("CRC_SEED", &self.CRC_SEED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SEED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SEED {{ CRC_SEED: {=u32:?} }}", self.CRC_SEED())
    }
}
#[doc = "CRC checksum register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SUM(pub u32);
impl SUM {
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC_SUM(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub const fn set_CRC_SUM(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SUM {
    #[inline(always)]
    fn default() -> SUM {
        SUM(0)
    }
}
impl core::fmt::Debug for SUM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUM")
            .field("CRC_SUM", &self.CRC_SUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SUM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SUM {{ CRC_SUM: {=u32:?} }}", self.CRC_SUM())
    }
}
#[doc = "CRC data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WR_DATA(pub u32);
impl WR_DATA {
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn CRC_WR_DATA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub const fn set_CRC_WR_DATA(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WR_DATA {
    #[inline(always)]
    fn default() -> WR_DATA {
        WR_DATA(0)
    }
}
impl core::fmt::Debug for WR_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_DATA")
            .field("CRC_WR_DATA", &self.CRC_WR_DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WR_DATA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WR_DATA {{ CRC_WR_DATA: {=u32:?} }}", self.CRC_WR_DATA())
    }
}
