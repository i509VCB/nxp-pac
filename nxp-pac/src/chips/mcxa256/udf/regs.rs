#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfCtrl(pub u32);
impl UdfCtrl {
    #[doc = "Bits are internally XORed with i_custom"]
    #[must_use]
    #[inline(always)]
    pub const fn salt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bits are internally XORed with i_custom"]
    #[inline(always)]
    pub const fn set_salt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Lock access to UDF"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Lock access to UDF"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Enable the UDF block"]
    #[must_use]
    #[inline(always)]
    pub const fn udf_en(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Enable the UDF block"]
    #[inline(always)]
    pub const fn set_udf_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_reserved25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved27(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_reserved27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Flush UDF and return to reset state"]
    #[must_use]
    #[inline(always)]
    pub const fn flush(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Flush UDF and return to reset state"]
    #[inline(always)]
    pub const fn set_flush(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub const fn set_reserved31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UdfCtrl {
    #[inline(always)]
    fn default() -> UdfCtrl {
        UdfCtrl(0)
    }
}
impl core::fmt::Debug for UdfCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfCtrl")
            .field("salt", &self.salt())
            .field("lock", &self.lock())
            .field("reserved21", &self.reserved21())
            .field("udf_en", &self.udf_en())
            .field("reserved25", &self.reserved25())
            .field("reserved27", &self.reserved27())
            .field("flush", &self.flush())
            .field("reserved31", &self.reserved31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UdfCtrl {{ salt: {=u16:?}, lock: {=u8:?}, reserved21: {=u8:?}, udf_en: {=u8:?}, reserved25: {=bool:?}, reserved27: {=u8:?}, flush: {=u8:?}, reserved31: {=bool:?} }}",
            self.salt(),
            self.lock(),
            self.reserved21(),
            self.udf_en(),
            self.reserved25(),
            self.reserved27(),
            self.flush(),
            self.reserved31()
        )
    }
}
#[doc = "Data Out Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfRdData(pub u32);
impl UdfRdData {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn o_dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_o_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdfRdData {
    #[inline(always)]
    fn default() -> UdfRdData {
        UdfRdData(0)
    }
}
impl core::fmt::Debug for UdfRdData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfRdData")
            .field("o_dat", &self.o_dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfRdData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdfRdData {{ o_dat: {=u32:?} }}", self.o_dat())
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfStatus(pub u32);
impl UdfStatus {
    #[doc = "Status bits"]
    #[must_use]
    #[inline(always)]
    pub const fn o_status(&self) -> super::vals::OStatus {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OStatus::from_bits(val as u8)
    }
    #[doc = "Status bits"]
    #[inline(always)]
    pub const fn set_o_status(&mut self, val: super::vals::OStatus) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn rsv(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_rsv(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 5usize)) | (((val as u32) & 0x03ff_ffff) << 5usize);
    }
    #[doc = "Indicates UDF is processing data"]
    #[must_use]
    #[inline(always)]
    pub const fn o_wait(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates UDF is processing data"]
    #[inline(always)]
    pub const fn set_o_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UdfStatus {
    #[inline(always)]
    fn default() -> UdfStatus {
        UdfStatus(0)
    }
}
impl core::fmt::Debug for UdfStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfStatus")
            .field("o_status", &self.o_status())
            .field("rsv", &self.rsv())
            .field("o_wait", &self.o_wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UdfStatus {{ o_status: {:?}, rsv: {=u32:?}, o_wait: {=bool:?} }}",
            self.o_status(),
            self.rsv(),
            self.o_wait()
        )
    }
}
#[doc = "Data In Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UdfWrData(pub u32);
impl UdfWrData {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn i_dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_i_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for UdfWrData {
    #[inline(always)]
    fn default() -> UdfWrData {
        UdfWrData(0)
    }
}
impl core::fmt::Debug for UdfWrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UdfWrData")
            .field("i_dat", &self.i_dat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UdfWrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UdfWrData {{ i_dat: {=u32:?} }}", self.i_dat())
    }
}
