#[doc = "Base Address for region 0 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BASE_ADDR0(pub u32);
impl BASE_ADDR0 {
    #[doc = "Fixed portion of the base address of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_FIXED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub const fn set_ADDR_FIXED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_PRG(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub const fn set_ADDR_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BASE_ADDR0 {
    #[inline(always)]
    fn default() -> BASE_ADDR0 {
        BASE_ADDR0(0)
    }
}
impl core::fmt::Debug for BASE_ADDR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASE_ADDR0")
            .field("ADDR_FIXED", &self.ADDR_FIXED())
            .field("ADDR_PRG", &self.ADDR_PRG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BASE_ADDR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BASE_ADDR0 {{ ADDR_FIXED: {=u32:?}, ADDR_PRG: {=u8:?} }}",
            self.ADDR_FIXED(),
            self.ADDR_PRG()
        )
    }
}
#[doc = "Base Address for region 1 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BASE_ADDR1(pub u32);
impl BASE_ADDR1 {
    #[doc = "Fixed portion of the base address of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_FIXED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 1."]
    #[inline(always)]
    pub const fn set_ADDR_FIXED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_PRG(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub const fn set_ADDR_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BASE_ADDR1 {
    #[inline(always)]
    fn default() -> BASE_ADDR1 {
        BASE_ADDR1(0)
    }
}
impl core::fmt::Debug for BASE_ADDR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASE_ADDR1")
            .field("ADDR_FIXED", &self.ADDR_FIXED())
            .field("ADDR_PRG", &self.ADDR_PRG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BASE_ADDR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BASE_ADDR1 {{ ADDR_FIXED: {=u32:?}, ADDR_PRG: {=u8:?} }}",
            self.ADDR_FIXED(),
            self.ADDR_PRG()
        )
    }
}
#[doc = "Base Address for region 2 register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BASE_ADDR2(pub u32);
impl BASE_ADDR2 {
    #[doc = "Fixed portion of the base address of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_FIXED(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 2."]
    #[inline(always)]
    pub const fn set_ADDR_FIXED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR_PRG(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub const fn set_ADDR_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BASE_ADDR2 {
    #[inline(always)]
    fn default() -> BASE_ADDR2 {
        BASE_ADDR2(0)
    }
}
impl core::fmt::Debug for BASE_ADDR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BASE_ADDR2")
            .field("ADDR_FIXED", &self.ADDR_FIXED())
            .field("ADDR_PRG", &self.ADDR_PRG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BASE_ADDR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BASE_ADDR2 {{ ADDR_FIXED: {=u32:?}, ADDR_PRG: {=u8:?} }}",
            self.ADDR_FIXED(),
            self.ADDR_PRG()
        )
    }
}
#[doc = "Encryption Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENC_ENABLE(pub u32);
impl ENC_ENABLE {
    #[doc = "Enables PRINCE encryption for flash programming."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PRINCE encryption for flash programming."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ENC_ENABLE {
    #[inline(always)]
    fn default() -> ENC_ENABLE {
        ENC_ENABLE(0)
    }
}
impl core::fmt::Debug for ENC_ENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENC_ENABLE")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENC_ENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENC_ENABLE {{ EN: {=bool:?} }}", self.EN())
    }
}
#[doc = "Error status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERR(pub u32);
impl ERR {
    #[doc = "PRINCE Error Status. This bit is write-1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ERRSTAT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PRINCE Error Status. This bit is write-1 to clear."]
    #[inline(always)]
    pub const fn set_ERRSTAT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ERR {
    #[inline(always)]
    fn default() -> ERR {
        ERR(0)
    }
}
impl core::fmt::Debug for ERR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERR")
            .field("ERRSTAT", &self.ERRSTAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ERR {{ ERRSTAT: {=bool:?} }}", self.ERRSTAT())
    }
}
#[doc = "Initial Vector register for region 0, Least Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_LSB0(pub u32);
impl IV_LSB0 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_LSB0 {
    #[inline(always)]
    fn default() -> IV_LSB0 {
        IV_LSB0(0)
    }
}
impl core::fmt::Debug for IV_LSB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_LSB0")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_LSB0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_LSB0 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Initial Vector register for region 1, Least Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_LSB1(pub u32);
impl IV_LSB1 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_LSB1 {
    #[inline(always)]
    fn default() -> IV_LSB1 {
        IV_LSB1(0)
    }
}
impl core::fmt::Debug for IV_LSB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_LSB1")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_LSB1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_LSB1 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Initial Vector register for region 2, Least Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_LSB2(pub u32);
impl IV_LSB2 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_LSB2 {
    #[inline(always)]
    fn default() -> IV_LSB2 {
        IV_LSB2(0)
    }
}
impl core::fmt::Debug for IV_LSB2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_LSB2")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_LSB2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_LSB2 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Initial Vector register for region 0, Most Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_MSB0(pub u32);
impl IV_MSB0 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_MSB0 {
    #[inline(always)]
    fn default() -> IV_MSB0 {
        IV_MSB0(0)
    }
}
impl core::fmt::Debug for IV_MSB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_MSB0")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_MSB0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_MSB0 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Initial Vector register for region 1, Most Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_MSB1(pub u32);
impl IV_MSB1 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_MSB1 {
    #[inline(always)]
    fn default() -> IV_MSB1 {
        IV_MSB1(0)
    }
}
impl core::fmt::Debug for IV_MSB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_MSB1")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_MSB1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_MSB1 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Initial Vector register for region 2, Most Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IV_MSB2(pub u32);
impl IV_MSB2 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn IVVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_IVVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IV_MSB2 {
    #[inline(always)]
    fn default() -> IV_MSB2 {
        IV_MSB2(0)
    }
}
impl core::fmt::Debug for IV_MSB2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IV_MSB2")
            .field("IVVAL", &self.IVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IV_MSB2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IV_MSB2 {{ IVVAL: {=u32:?} }}", self.IVVAL())
    }
}
#[doc = "Lock register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LOCK(pub u32);
impl LOCK {
    #[doc = "Lock Region 0 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKREG0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 0 registers."]
    #[inline(always)]
    pub const fn set_LOCKREG0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock Region 1 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKREG1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 1 registers."]
    #[inline(always)]
    pub const fn set_LOCKREG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Lock Region 2 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKREG2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 2 registers."]
    #[inline(always)]
    pub const fn set_LOCKREG2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock the Mask registers."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCKMASK(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Lock the Mask registers."]
    #[inline(always)]
    pub const fn set_LOCKMASK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LOCK {
    #[inline(always)]
    fn default() -> LOCK {
        LOCK(0)
    }
}
impl core::fmt::Debug for LOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("LOCKREG0", &self.LOCKREG0())
            .field("LOCKREG1", &self.LOCKREG1())
            .field("LOCKREG2", &self.LOCKREG2())
            .field("LOCKMASK", &self.LOCKMASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LOCK {{ LOCKREG0: {=bool:?}, LOCKREG1: {=bool:?}, LOCKREG2: {=bool:?}, LOCKMASK: {=bool:?} }}",
            self.LOCKREG0(),
            self.LOCKREG1(),
            self.LOCKREG2(),
            self.LOCKMASK()
        )
    }
}
#[doc = "Data Mask register, 32 Least Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK_LSB(pub u32);
impl MASK_LSB {
    #[doc = "Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[must_use]
    #[inline(always)]
    pub const fn MASKVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub const fn set_MASKVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MASK_LSB {
    #[inline(always)]
    fn default() -> MASK_LSB {
        MASK_LSB(0)
    }
}
impl core::fmt::Debug for MASK_LSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_LSB")
            .field("MASKVAL", &self.MASKVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK_LSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK_LSB {{ MASKVAL: {=u32:?} }}", self.MASKVAL())
    }
}
#[doc = "Data Mask register, 32 Most Significant Bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MASK_MSB(pub u32);
impl MASK_MSB {
    #[doc = "Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[must_use]
    #[inline(always)]
    pub const fn MASKVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub const fn set_MASKVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MASK_MSB {
    #[inline(always)]
    fn default() -> MASK_MSB {
        MASK_MSB(0)
    }
}
impl core::fmt::Debug for MASK_MSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASK_MSB")
            .field("MASKVAL", &self.MASKVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MASK_MSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MASK_MSB {{ MASKVAL: {=u32:?} }}", self.MASKVAL())
    }
}
#[doc = "Sub-Region Enable register for region 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SR_ENABLE0(pub u32);
impl SR_ENABLE0 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SR_ENABLE0 {
    #[inline(always)]
    fn default() -> SR_ENABLE0 {
        SR_ENABLE0(0)
    }
}
impl core::fmt::Debug for SR_ENABLE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_ENABLE0")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SR_ENABLE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SR_ENABLE0 {{ EN: {=u32:?} }}", self.EN())
    }
}
#[doc = "Sub-Region Enable register for region 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SR_ENABLE1(pub u32);
impl SR_ENABLE1 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SR_ENABLE1 {
    #[inline(always)]
    fn default() -> SR_ENABLE1 {
        SR_ENABLE1(0)
    }
}
impl core::fmt::Debug for SR_ENABLE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_ENABLE1")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SR_ENABLE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SR_ENABLE1 {{ EN: {=u32:?} }}", self.EN())
    }
}
#[doc = "Sub-Region Enable register for region 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SR_ENABLE2(pub u32);
impl SR_ENABLE2 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SR_ENABLE2 {
    #[inline(always)]
    fn default() -> SR_ENABLE2 {
        SR_ENABLE2(0)
    }
}
impl core::fmt::Debug for SR_ENABLE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_ENABLE2")
            .field("EN", &self.EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SR_ENABLE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SR_ENABLE2 {{ EN: {=u32:?} }}", self.EN())
    }
}
