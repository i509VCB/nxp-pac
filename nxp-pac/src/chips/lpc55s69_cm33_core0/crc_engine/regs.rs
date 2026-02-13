#[doc = "CRC mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_poly(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub const fn set_crc_poly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[must_use]
    #[inline(always)]
    pub const fn bit_rvs_wr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub const fn set_bit_rvs_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_wr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub const fn set_cmpl_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[must_use]
    #[inline(always)]
    pub const fn bit_rvs_sum(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub const fn set_bit_rvs_sum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpl_sum(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub const fn set_cmpl_sum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
impl core::fmt::Debug for Mode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mode")
            .field("crc_poly", &self.crc_poly())
            .field("bit_rvs_wr", &self.bit_rvs_wr())
            .field("cmpl_wr", &self.cmpl_wr())
            .field("bit_rvs_sum", &self.bit_rvs_sum())
            .field("cmpl_sum", &self.cmpl_sum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mode {{ crc_poly: {=u8:?}, bit_rvs_wr: {=bool:?}, cmpl_wr: {=bool:?}, bit_rvs_sum: {=bool:?}, cmpl_sum: {=bool:?} }}",
            self.crc_poly(),
            self.bit_rvs_wr(),
            self.cmpl_wr(),
            self.bit_rvs_sum(),
            self.cmpl_sum()
        )
    }
}
#[doc = "CRC seed register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seed(pub u32);
impl Seed {
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_seed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub const fn set_crc_seed(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Seed {
    #[inline(always)]
    fn default() -> Seed {
        Seed(0)
    }
}
impl core::fmt::Debug for Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seed")
            .field("crc_seed", &self.crc_seed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Seed {{ crc_seed: {=u32:?} }}", self.crc_seed())
    }
}
#[doc = "CRC checksum register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sum(pub u32);
impl Sum {
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_sum(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub const fn set_crc_sum(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sum {
    #[inline(always)]
    fn default() -> Sum {
        Sum(0)
    }
}
impl core::fmt::Debug for Sum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sum")
            .field("crc_sum", &self.crc_sum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sum {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sum {{ crc_sum: {=u32:?} }}", self.crc_sum())
    }
}
#[doc = "CRC data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrData(pub u32);
impl WrData {
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_wr_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub const fn set_crc_wr_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WrData {
    #[inline(always)]
    fn default() -> WrData {
        WrData(0)
    }
}
impl core::fmt::Debug for WrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WrData")
            .field("crc_wr_data", &self.crc_wr_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WrData {{ crc_wr_data: {=u32:?} }}", self.crc_wr_data())
    }
}
