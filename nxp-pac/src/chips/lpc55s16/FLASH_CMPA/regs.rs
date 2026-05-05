#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOOT_CFG(pub u32);
impl BOOT_CFG {
    #[doc = "Default ISP mode:."]
    #[must_use]
    #[inline(always)]
    pub const fn DEFAULT_ISP_MODE(&self) -> super::vals::DEFAULT_ISP_MODE {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::DEFAULT_ISP_MODE::from_bits(val as u8)
    }
    #[doc = "Default ISP mode:."]
    #[inline(always)]
    pub const fn set_DEFAULT_ISP_MODE(&mut self, val: super::vals::DEFAULT_ISP_MODE) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Core clock:."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SPEED(&self) -> super::vals::BOOT_SPEED {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::BOOT_SPEED::from_bits(val as u8)
    }
    #[doc = "Core clock:."]
    #[inline(always)]
    pub const fn set_BOOT_SPEED(&mut self, val: super::vals::BOOT_SPEED) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_FAILURE_PIN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin."]
    #[inline(always)]
    pub const fn set_BOOT_FAILURE_PIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for BOOT_CFG {
    #[inline(always)]
    fn default() -> BOOT_CFG {
        BOOT_CFG(0)
    }
}
impl core::fmt::Debug for BOOT_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_CFG")
            .field("DEFAULT_ISP_MODE", &self.DEFAULT_ISP_MODE())
            .field("BOOT_SPEED", &self.BOOT_SPEED())
            .field("BOOT_FAILURE_PIN", &self.BOOT_FAILURE_PIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOOT_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOOT_CFG {{ DEFAULT_ISP_MODE: {:?}, BOOT_SPEED: {:?}, BOOT_FAILURE_PIN: {=u8:?} }}",
            self.DEFAULT_ISP_MODE(),
            self.BOOT_SPEED(),
            self.BOOT_FAILURE_PIN()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CC_SOCU_DFLT(pub u32);
impl CC_SOCU_DFLT {
    #[doc = "Non Secure non-invasive debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn NIDEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Non Secure non-invasive debug fixed state."]
    #[inline(always)]
    pub const fn set_NIDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn DBGEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Non Secure debug fixed state."]
    #[inline(always)]
    pub const fn set_DBGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn SPNIDEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure non-invasive debug fixed state."]
    #[inline(always)]
    pub const fn set_SPNIDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn SPIDEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Secure invasive debug fixed state."]
    #[inline(always)]
    pub const fn set_SPIDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG TAP fixed state."]
    #[inline(always)]
    pub const fn set_TAPEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ISP Boot Command fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn ISP_CMD_EN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ISP Boot Command fixed state."]
    #[inline(always)]
    pub const fn set_ISP_CMD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn FA_ME_CMD_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FA Command fixed state."]
    #[inline(always)]
    pub const fn set_FA_ME_CMD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "inverse value of bits \\[15:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERSE_VALUE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "inverse value of bits \\[15:0\\]."]
    #[inline(always)]
    pub const fn set_INVERSE_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CC_SOCU_DFLT {
    #[inline(always)]
    fn default() -> CC_SOCU_DFLT {
        CC_SOCU_DFLT(0)
    }
}
impl core::fmt::Debug for CC_SOCU_DFLT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC_SOCU_DFLT")
            .field("NIDEN", &self.NIDEN())
            .field("DBGEN", &self.DBGEN())
            .field("SPNIDEN", &self.SPNIDEN())
            .field("SPIDEN", &self.SPIDEN())
            .field("TAPEN", &self.TAPEN())
            .field("ISP_CMD_EN", &self.ISP_CMD_EN())
            .field("FA_ME_CMD_EN", &self.FA_ME_CMD_EN())
            .field("INVERSE_VALUE", &self.INVERSE_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CC_SOCU_DFLT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CC_SOCU_DFLT {{ NIDEN: {=bool:?}, DBGEN: {=bool:?}, SPNIDEN: {=bool:?}, SPIDEN: {=bool:?}, TAPEN: {=bool:?}, ISP_CMD_EN: {=bool:?}, FA_ME_CMD_EN: {=bool:?}, INVERSE_VALUE: {=u16:?} }}",
            self.NIDEN(),
            self.DBGEN(),
            self.SPNIDEN(),
            self.SPIDEN(),
            self.TAPEN(),
            self.ISP_CMD_EN(),
            self.FA_ME_CMD_EN(),
            self.INVERSE_VALUE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CC_SOCU_PIN(pub u32);
impl CC_SOCU_PIN {
    #[doc = "Non Secure non-invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn NIDEN(&self) -> super::vals::CC_SOCU_PIN_NIDEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CC_SOCU_PIN_NIDEN::from_bits(val as u8)
    }
    #[doc = "Non Secure non-invasive debug enable."]
    #[inline(always)]
    pub const fn set_NIDEN(&mut self, val: super::vals::CC_SOCU_PIN_NIDEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DBGEN(&self) -> super::vals::CC_SOCU_PIN_DBGEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CC_SOCU_PIN_DBGEN::from_bits(val as u8)
    }
    #[doc = "Non Secure debug enable."]
    #[inline(always)]
    pub const fn set_DBGEN(&mut self, val: super::vals::CC_SOCU_PIN_DBGEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SPNIDEN(&self) -> super::vals::CC_SOCU_PIN_SPNIDEN {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CC_SOCU_PIN_SPNIDEN::from_bits(val as u8)
    }
    #[doc = "Secure non-invasive debug enable."]
    #[inline(always)]
    pub const fn set_SPNIDEN(&mut self, val: super::vals::CC_SOCU_PIN_SPNIDEN) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SPIDEN(&self) -> super::vals::CC_SOCU_PIN_SPIDEN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CC_SOCU_PIN_SPIDEN::from_bits(val as u8)
    }
    #[doc = "Secure invasive debug enable."]
    #[inline(always)]
    pub const fn set_SPIDEN(&mut self, val: super::vals::CC_SOCU_PIN_SPIDEN) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPEN(&self) -> super::vals::CC_SOCU_PIN_TAPEN {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CC_SOCU_PIN_TAPEN::from_bits(val as u8)
    }
    #[doc = "JTAG TAP enable."]
    #[inline(always)]
    pub const fn set_TAPEN(&mut self, val: super::vals::CC_SOCU_PIN_TAPEN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ISP Boot Command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ISP_CMD_EN(&self) -> super::vals::CC_SOCU_PIN_ISP_CMD_EN {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CC_SOCU_PIN_ISP_CMD_EN::from_bits(val as u8)
    }
    #[doc = "ISP Boot Command enable."]
    #[inline(always)]
    pub const fn set_ISP_CMD_EN(&mut self, val: super::vals::CC_SOCU_PIN_ISP_CMD_EN) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FA_ME_CMD_EN(&self) -> super::vals::CC_SOCU_PIN_FA_ME_CMD_EN {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CC_SOCU_PIN_FA_ME_CMD_EN::from_bits(val as u8)
    }
    #[doc = "FA Command enable."]
    #[inline(always)]
    pub const fn set_FA_ME_CMD_EN(&mut self, val: super::vals::CC_SOCU_PIN_FA_ME_CMD_EN) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Enforce UUID match during Debug authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn UUID_CHECK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub const fn set_UUID_CHECK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "inverse value of bits \\[15:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERSE_VALUE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "inverse value of bits \\[15:0\\]."]
    #[inline(always)]
    pub const fn set_INVERSE_VALUE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CC_SOCU_PIN {
    #[inline(always)]
    fn default() -> CC_SOCU_PIN {
        CC_SOCU_PIN(0)
    }
}
impl core::fmt::Debug for CC_SOCU_PIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC_SOCU_PIN")
            .field("NIDEN", &self.NIDEN())
            .field("DBGEN", &self.DBGEN())
            .field("SPNIDEN", &self.SPNIDEN())
            .field("SPIDEN", &self.SPIDEN())
            .field("TAPEN", &self.TAPEN())
            .field("ISP_CMD_EN", &self.ISP_CMD_EN())
            .field("FA_ME_CMD_EN", &self.FA_ME_CMD_EN())
            .field("UUID_CHECK", &self.UUID_CHECK())
            .field("INVERSE_VALUE", &self.INVERSE_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CC_SOCU_PIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CC_SOCU_PIN {{ NIDEN: {:?}, DBGEN: {:?}, SPNIDEN: {:?}, SPIDEN: {:?}, TAPEN: {:?}, ISP_CMD_EN: {:?}, FA_ME_CMD_EN: {:?}, UUID_CHECK: {=bool:?}, INVERSE_VALUE: {=u16:?} }}",
            self.NIDEN(),
            self.DBGEN(),
            self.SPNIDEN(),
            self.SPIDEN(),
            self.TAPEN(),
            self.ISP_CMD_EN(),
            self.FA_ME_CMD_EN(),
            self.UUID_CHECK(),
            self.INVERSE_VALUE()
        )
    }
}
#[doc = "Customer Defined (Programable through ROM API)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CUSTOMER_DEFINED(pub u32);
impl CUSTOMER_DEFINED {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CUSTOMER_DEFINED {
    #[inline(always)]
    fn default() -> CUSTOMER_DEFINED {
        CUSTOMER_DEFINED(0)
    }
}
impl core::fmt::Debug for CUSTOMER_DEFINED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CUSTOMER_DEFINED")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CUSTOMER_DEFINED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CUSTOMER_DEFINED {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "This 32-bit register contains the offset by which the image is to be remapped. The 12 LSBs are ignored, so the remap granularity is 4KB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_REMAP_OFFSET(pub u32);
impl FLASH_REMAP_OFFSET {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASH_REMAP_OFFSET {
    #[inline(always)]
    fn default() -> FLASH_REMAP_OFFSET {
        FLASH_REMAP_OFFSET(0)
    }
}
impl core::fmt::Debug for FLASH_REMAP_OFFSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_REMAP_OFFSET")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_REMAP_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLASH_REMAP_OFFSET {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "This 32-bit register contains the size of the image to remap, in bytes. The 12 LSBs are ignored, so the size granularity is 4KB."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_REMAP_SIZE(pub u32);
impl FLASH_REMAP_SIZE {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASH_REMAP_SIZE {
    #[inline(always)]
    fn default() -> FLASH_REMAP_SIZE {
        FLASH_REMAP_SIZE(0)
    }
}
impl core::fmt::Debug for FLASH_REMAP_SIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_REMAP_SIZE")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASH_REMAP_SIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FLASH_REMAP_SIZE {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_BASE_ADDR(pub u32);
impl PRINCE_BASE_ADDR {
    #[doc = "Programmable portion of the base address of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR0_PRG(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub const fn set_ADDR0_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR1_PRG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub const fn set_ADDR1_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ADDR2_PRG(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub const fn set_ADDR2_PRG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Lock PRINCE region0 settings."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_REG0(&self) -> super::vals::LOCK_REG0 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::LOCK_REG0::from_bits(val as u8)
    }
    #[doc = "Lock PRINCE region0 settings."]
    #[inline(always)]
    pub const fn set_LOCK_REG0(&mut self, val: super::vals::LOCK_REG0) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Lock PRINCE region1 settings."]
    #[must_use]
    #[inline(always)]
    pub const fn LOCK_REG1(&self) -> super::vals::LOCK_REG1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LOCK_REG1::from_bits(val as u8)
    }
    #[doc = "Lock PRINCE region1 settings."]
    #[inline(always)]
    pub const fn set_LOCK_REG1(&mut self, val: super::vals::LOCK_REG1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together."]
    #[must_use]
    #[inline(always)]
    pub const fn REG0_ERASE_CHECK_EN(&self) -> super::vals::REG0_ERASE_CHECK_EN {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::REG0_ERASE_CHECK_EN::from_bits(val as u8)
    }
    #[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub const fn set_REG0_ERASE_CHECK_EN(&mut self, val: super::vals::REG0_ERASE_CHECK_EN) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together."]
    #[must_use]
    #[inline(always)]
    pub const fn REG1_ERASE_CHECK_EN(&self) -> super::vals::REG1_ERASE_CHECK_EN {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::REG1_ERASE_CHECK_EN::from_bits(val as u8)
    }
    #[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub const fn set_REG1_ERASE_CHECK_EN(&mut self, val: super::vals::REG1_ERASE_CHECK_EN) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together."]
    #[must_use]
    #[inline(always)]
    pub const fn REG2_ERASE_CHECK_EN(&self) -> super::vals::REG2_ERASE_CHECK_EN {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::REG2_ERASE_CHECK_EN::from_bits(val as u8)
    }
    #[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together."]
    #[inline(always)]
    pub const fn set_REG2_ERASE_CHECK_EN(&mut self, val: super::vals::REG2_ERASE_CHECK_EN) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for PRINCE_BASE_ADDR {
    #[inline(always)]
    fn default() -> PRINCE_BASE_ADDR {
        PRINCE_BASE_ADDR(0)
    }
}
impl core::fmt::Debug for PRINCE_BASE_ADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_BASE_ADDR")
            .field("ADDR0_PRG", &self.ADDR0_PRG())
            .field("ADDR1_PRG", &self.ADDR1_PRG())
            .field("ADDR2_PRG", &self.ADDR2_PRG())
            .field("LOCK_REG0", &self.LOCK_REG0())
            .field("LOCK_REG1", &self.LOCK_REG1())
            .field("REG0_ERASE_CHECK_EN", &self.REG0_ERASE_CHECK_EN())
            .field("REG1_ERASE_CHECK_EN", &self.REG1_ERASE_CHECK_EN())
            .field("REG2_ERASE_CHECK_EN", &self.REG2_ERASE_CHECK_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_BASE_ADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_BASE_ADDR {{ ADDR0_PRG: {=u8:?}, ADDR1_PRG: {=u8:?}, ADDR2_PRG: {=u8:?}, LOCK_REG0: {:?}, LOCK_REG1: {:?}, REG0_ERASE_CHECK_EN: {:?}, REG1_ERASE_CHECK_EN: {:?}, REG2_ERASE_CHECK_EN: {:?} }}",
            self.ADDR0_PRG(),
            self.ADDR1_PRG(),
            self.ADDR2_PRG(),
            self.LOCK_REG0(),
            self.LOCK_REG1(),
            self.REG0_ERASE_CHECK_EN(),
            self.REG1_ERASE_CHECK_EN(),
            self.REG2_ERASE_CHECK_EN()
        )
    }
}
#[doc = "Region 0, sub-region enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_SR_0(pub u32);
impl PRINCE_SR_0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_SR_0 {
    #[inline(always)]
    fn default() -> PRINCE_SR_0 {
        PRINCE_SR_0(0)
    }
}
impl core::fmt::Debug for PRINCE_SR_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_SR_0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_SR_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRINCE_SR_0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Region 1, sub-region enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_SR_1(pub u32);
impl PRINCE_SR_1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_SR_1 {
    #[inline(always)]
    fn default() -> PRINCE_SR_1 {
        PRINCE_SR_1(0)
    }
}
impl core::fmt::Debug for PRINCE_SR_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_SR_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_SR_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRINCE_SR_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Region 2, sub-region enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_SR_2(pub u32);
impl PRINCE_SR_2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PRINCE_SR_2 {
    #[inline(always)]
    fn default() -> PRINCE_SR_2 {
        PRINCE_SR_2(0)
    }
}
impl core::fmt::Debug for PRINCE_SR_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_SR_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_SR_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PRINCE_SR_2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ROTKH(pub u32);
impl ROTKH {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ROTKH {
    #[inline(always)]
    fn default() -> ROTKH {
        ROTKH(0)
    }
}
impl core::fmt::Debug for ROTKH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROTKH")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ROTKH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ROTKH {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SDIO_CFG(pub u32);
impl SDIO_CFG {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SDIO_CFG {
    #[inline(always)]
    fn default() -> SDIO_CFG {
        SDIO_CFG(0)
    }
}
impl core::fmt::Debug for SDIO_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CFG")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SDIO_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SDIO_CFG {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Secure boot configuration flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SECURE_BOOT_CFG(pub u32);
impl SECURE_BOOT_CFG {
    #[doc = "Use RSA4096 keys only."]
    #[must_use]
    #[inline(always)]
    pub const fn RSA4K(&self) -> super::vals::RSA4K {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::RSA4K::from_bits(val as u8)
    }
    #[doc = "Use RSA4096 keys only."]
    #[inline(always)]
    pub const fn set_RSA4K(&mut self, val: super::vals::RSA4K) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Include NXP area in DICE computation."]
    #[must_use]
    #[inline(always)]
    pub const fn DICE_INC_NXP_CFG(&self) -> super::vals::DICE_INC_NXP_CFG {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DICE_INC_NXP_CFG::from_bits(val as u8)
    }
    #[doc = "Include NXP area in DICE computation."]
    #[inline(always)]
    pub const fn set_DICE_INC_NXP_CFG(&mut self, val: super::vals::DICE_INC_NXP_CFG) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Include Customer factory area (including keys) in DICE computation."]
    #[must_use]
    #[inline(always)]
    pub const fn DICE_CUST_CFG(&self) -> super::vals::DICE_CUST_CFG {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DICE_CUST_CFG::from_bits(val as u8)
    }
    #[doc = "Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub const fn set_DICE_CUST_CFG(&mut self, val: super::vals::DICE_CUST_CFG) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Skip DICE computation."]
    #[must_use]
    #[inline(always)]
    pub const fn SKIP_DICE(&self) -> super::vals::SKIP_DICE {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SKIP_DICE::from_bits(val as u8)
    }
    #[doc = "Skip DICE computation."]
    #[inline(always)]
    pub const fn set_SKIP_DICE(&mut self, val: super::vals::SKIP_DICE) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "TrustZone-M mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TZM_IMAGE_TYPE(&self) -> super::vals::TZM_IMAGE_TYPE {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::TZM_IMAGE_TYPE::from_bits(val as u8)
    }
    #[doc = "TrustZone-M mode."]
    #[inline(always)]
    pub const fn set_TZM_IMAGE_TYPE(&mut self, val: super::vals::TZM_IMAGE_TYPE) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Block PUF key code generation."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCK_SET_KEY(&self) -> super::vals::BLOCK_SET_KEY {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::BLOCK_SET_KEY::from_bits(val as u8)
    }
    #[doc = "Block PUF key code generation."]
    #[inline(always)]
    pub const fn set_BLOCK_SET_KEY(&mut self, val: super::vals::BLOCK_SET_KEY) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Block PUF enrollement."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCK_ENROLL(&self) -> super::vals::BLOCK_ENROLL {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::BLOCK_ENROLL::from_bits(val as u8)
    }
    #[doc = "Block PUF enrollement."]
    #[inline(always)]
    pub const fn set_BLOCK_ENROLL(&mut self, val: super::vals::BLOCK_ENROLL) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Include security EPOCH in DICE."]
    #[must_use]
    #[inline(always)]
    pub const fn DICE_INC_SEC_EPOCH(&self) -> super::vals::DICE_INC_SEC_EPOCH {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::DICE_INC_SEC_EPOCH::from_bits(val as u8)
    }
    #[doc = "Include security EPOCH in DICE."]
    #[inline(always)]
    pub const fn set_DICE_INC_SEC_EPOCH(&mut self, val: super::vals::DICE_INC_SEC_EPOCH) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Skip boot seed computation."]
    #[must_use]
    #[inline(always)]
    pub const fn SKIP_BOOT_SEED(&self) -> super::vals::SKIP_BOOT_SEED {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SKIP_BOOT_SEED::from_bits(val as u8)
    }
    #[doc = "Skip boot seed computation."]
    #[inline(always)]
    pub const fn set_SKIP_BOOT_SEED(&mut self, val: super::vals::SKIP_BOOT_SEED) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Include NXP area in BOOT SEED computation."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_INC_NXP_CFG(&self) -> super::vals::BOOT_SEED_INC_NXP_CFG {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::BOOT_SEED_INC_NXP_CFG::from_bits(val as u8)
    }
    #[doc = "Include NXP area in BOOT SEED computation."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_INC_NXP_CFG(&mut self, val: super::vals::BOOT_SEED_INC_NXP_CFG) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Include CMPA area in BOOT SEED computation."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_CUST_CFG(&self) -> super::vals::BOOT_SEED_CUST_CFG {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::BOOT_SEED_CUST_CFG::from_bits(val as u8)
    }
    #[doc = "Include CMPA area in BOOT SEED computation."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_CUST_CFG(&mut self, val: super::vals::BOOT_SEED_CUST_CFG) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Include security epoch area in BOOT_SEED computation."]
    #[must_use]
    #[inline(always)]
    pub const fn BOOT_SEED_INC_EPOCH(&self) -> super::vals::BOOT_SEED_INC_EPOCH {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::BOOT_SEED_INC_EPOCH::from_bits(val as u8)
    }
    #[doc = "Include security epoch area in BOOT_SEED computation."]
    #[inline(always)]
    pub const fn set_BOOT_SEED_INC_EPOCH(&mut self, val: super::vals::BOOT_SEED_INC_EPOCH) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Secure boot enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SEC_BOOT_EN(&self) -> super::vals::SEC_BOOT_EN {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::SEC_BOOT_EN::from_bits(val as u8)
    }
    #[doc = "Secure boot enable."]
    #[inline(always)]
    pub const fn set_SEC_BOOT_EN(&mut self, val: super::vals::SEC_BOOT_EN) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for SECURE_BOOT_CFG {
    #[inline(always)]
    fn default() -> SECURE_BOOT_CFG {
        SECURE_BOOT_CFG(0)
    }
}
impl core::fmt::Debug for SECURE_BOOT_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURE_BOOT_CFG")
            .field("RSA4K", &self.RSA4K())
            .field("DICE_INC_NXP_CFG", &self.DICE_INC_NXP_CFG())
            .field("DICE_CUST_CFG", &self.DICE_CUST_CFG())
            .field("SKIP_DICE", &self.SKIP_DICE())
            .field("TZM_IMAGE_TYPE", &self.TZM_IMAGE_TYPE())
            .field("BLOCK_SET_KEY", &self.BLOCK_SET_KEY())
            .field("BLOCK_ENROLL", &self.BLOCK_ENROLL())
            .field("DICE_INC_SEC_EPOCH", &self.DICE_INC_SEC_EPOCH())
            .field("SKIP_BOOT_SEED", &self.SKIP_BOOT_SEED())
            .field("BOOT_SEED_INC_NXP_CFG", &self.BOOT_SEED_INC_NXP_CFG())
            .field("BOOT_SEED_CUST_CFG", &self.BOOT_SEED_CUST_CFG())
            .field("BOOT_SEED_INC_EPOCH", &self.BOOT_SEED_INC_EPOCH())
            .field("SEC_BOOT_EN", &self.SEC_BOOT_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SECURE_BOOT_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SECURE_BOOT_CFG {{ RSA4K: {:?}, DICE_INC_NXP_CFG: {:?}, DICE_CUST_CFG: {:?}, SKIP_DICE: {:?}, TZM_IMAGE_TYPE: {:?}, BLOCK_SET_KEY: {:?}, BLOCK_ENROLL: {:?}, DICE_INC_SEC_EPOCH: {:?}, SKIP_BOOT_SEED: {:?}, BOOT_SEED_INC_NXP_CFG: {:?}, BOOT_SEED_CUST_CFG: {:?}, BOOT_SEED_INC_EPOCH: {:?}, SEC_BOOT_EN: {:?} }}",
            self.RSA4K(),
            self.DICE_INC_NXP_CFG(),
            self.DICE_CUST_CFG(),
            self.SKIP_DICE(),
            self.TZM_IMAGE_TYPE(),
            self.BLOCK_SET_KEY(),
            self.BLOCK_ENROLL(),
            self.DICE_INC_SEC_EPOCH(),
            self.SKIP_BOOT_SEED(),
            self.BOOT_SEED_INC_NXP_CFG(),
            self.BOOT_SEED_CUST_CFG(),
            self.BOOT_SEED_INC_EPOCH(),
            self.SEC_BOOT_EN()
        )
    }
}
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHA256_DIGEST(pub u32);
impl SHA256_DIGEST {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SHA256_DIGEST {
    #[inline(always)]
    fn default() -> SHA256_DIGEST {
        SHA256_DIGEST(0)
    }
}
impl core::fmt::Debug for SHA256_DIGEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA256_DIGEST")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHA256_DIGEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SHA256_DIGEST {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI_FLASH_CFG(pub u32);
impl SPI_FLASH_CFG {
    #[doc = "SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[must_use]
    #[inline(always)]
    pub const fn SPI_RECOVERY_BOOT_EN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub const fn set_SPI_RECOVERY_BOOT_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for SPI_FLASH_CFG {
    #[inline(always)]
    fn default() -> SPI_FLASH_CFG {
        SPI_FLASH_CFG(0)
    }
}
impl core::fmt::Debug for SPI_FLASH_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_FLASH_CFG")
            .field("SPI_RECOVERY_BOOT_EN", &self.SPI_RECOVERY_BOOT_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SPI_FLASH_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SPI_FLASH_CFG {{ SPI_RECOVERY_BOOT_EN: {=u8:?} }}",
            self.SPI_RECOVERY_BOOT_EN()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB_ID(pub u32);
impl USB_ID {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_VENDOR_ID(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_USB_VENDOR_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_PRODUCT_ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_USB_PRODUCT_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for USB_ID {
    #[inline(always)]
    fn default() -> USB_ID {
        USB_ID(0)
    }
}
impl core::fmt::Debug for USB_ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_ID")
            .field("USB_VENDOR_ID", &self.USB_VENDOR_ID())
            .field("USB_PRODUCT_ID", &self.USB_PRODUCT_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB_ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB_ID {{ USB_VENDOR_ID: {=u16:?}, USB_PRODUCT_ID: {=u16:?} }}",
            self.USB_VENDOR_ID(),
            self.USB_PRODUCT_ID()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VENDOR_USAGE(pub u32);
impl VENDOR_USAGE {
    #[doc = "Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[must_use]
    #[inline(always)]
    pub const fn VENDOR_USAGE(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub const fn set_VENDOR_USAGE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for VENDOR_USAGE {
    #[inline(always)]
    fn default() -> VENDOR_USAGE {
        VENDOR_USAGE(0)
    }
}
impl core::fmt::Debug for VENDOR_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENDOR_USAGE")
            .field("VENDOR_USAGE", &self.VENDOR_USAGE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VENDOR_USAGE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VENDOR_USAGE {{ VENDOR_USAGE: {=u16:?} }}",
            self.VENDOR_USAGE()
        )
    }
}
#[doc = "Xtal 16MHz capabank triming."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XTAL_16MHZ_CAPABANK_TRIM(pub u32);
impl XTAL_16MHZ_CAPABANK_TRIM {
    #[doc = "XTAL 16MHz capa bank trimmings."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_VALID(&self) -> super::vals::XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID::from_bits(val as u8)
    }
    #[doc = "XTAL 16MHz capa bank trimmings."]
    #[inline(always)]
    pub const fn set_TRIM_VALID(&mut self, val: super::vals::XTAL_16MHZ_CAPABANK_TRIM_TRIM_VALID) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL_LOAD_CAP_IEC_PF_X100(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_XTAL_LOAD_CAP_IEC_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn PCB_XIN_PARA_CAP_PF_X100(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_PCB_XIN_PARA_CAP_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn PCB_XOUT_PARA_CAP_PF_X100(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_PCB_XOUT_PARA_CAP_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for XTAL_16MHZ_CAPABANK_TRIM {
    #[inline(always)]
    fn default() -> XTAL_16MHZ_CAPABANK_TRIM {
        XTAL_16MHZ_CAPABANK_TRIM(0)
    }
}
impl core::fmt::Debug for XTAL_16MHZ_CAPABANK_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_16MHZ_CAPABANK_TRIM")
            .field("TRIM_VALID", &self.TRIM_VALID())
            .field(
                "XTAL_LOAD_CAP_IEC_PF_X100",
                &self.XTAL_LOAD_CAP_IEC_PF_X100(),
            )
            .field("PCB_XIN_PARA_CAP_PF_X100", &self.PCB_XIN_PARA_CAP_PF_X100())
            .field(
                "PCB_XOUT_PARA_CAP_PF_X100",
                &self.PCB_XOUT_PARA_CAP_PF_X100(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XTAL_16MHZ_CAPABANK_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XTAL_16MHZ_CAPABANK_TRIM {{ TRIM_VALID: {:?}, XTAL_LOAD_CAP_IEC_PF_X100: {=u16:?}, PCB_XIN_PARA_CAP_PF_X100: {=u16:?}, PCB_XOUT_PARA_CAP_PF_X100: {=u16:?} }}",
            self.TRIM_VALID(),
            self.XTAL_LOAD_CAP_IEC_PF_X100(),
            self.PCB_XIN_PARA_CAP_PF_X100(),
            self.PCB_XOUT_PARA_CAP_PF_X100()
        )
    }
}
#[doc = "Xtal 32kHz capabank triming."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XTAL_32KHZ_CAPABANK_TRIM(pub u32);
impl XTAL_32KHZ_CAPABANK_TRIM {
    #[doc = "XTAL 32kHz capa bank trimmings."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_VALID(&self) -> super::vals::XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID::from_bits(val as u8)
    }
    #[doc = "XTAL 32kHz capa bank trimmings."]
    #[inline(always)]
    pub const fn set_TRIM_VALID(&mut self, val: super::vals::XTAL_32KHZ_CAPABANK_TRIM_TRIM_VALID) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn XTAL_LOAD_CAP_IEC_PF_X100(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_XTAL_LOAD_CAP_IEC_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn PCB_XIN_PARA_CAP_PF_X100(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_PCB_XIN_PARA_CAP_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn PCB_XOUT_PARA_CAP_PF_X100(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_PCB_XOUT_PARA_CAP_PF_X100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for XTAL_32KHZ_CAPABANK_TRIM {
    #[inline(always)]
    fn default() -> XTAL_32KHZ_CAPABANK_TRIM {
        XTAL_32KHZ_CAPABANK_TRIM(0)
    }
}
impl core::fmt::Debug for XTAL_32KHZ_CAPABANK_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTAL_32KHZ_CAPABANK_TRIM")
            .field("TRIM_VALID", &self.TRIM_VALID())
            .field(
                "XTAL_LOAD_CAP_IEC_PF_X100",
                &self.XTAL_LOAD_CAP_IEC_PF_X100(),
            )
            .field("PCB_XIN_PARA_CAP_PF_X100", &self.PCB_XIN_PARA_CAP_PF_X100())
            .field(
                "PCB_XOUT_PARA_CAP_PF_X100",
                &self.PCB_XOUT_PARA_CAP_PF_X100(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XTAL_32KHZ_CAPABANK_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XTAL_32KHZ_CAPABANK_TRIM {{ TRIM_VALID: {:?}, XTAL_LOAD_CAP_IEC_PF_X100: {=u16:?}, PCB_XIN_PARA_CAP_PF_X100: {=u16:?}, PCB_XOUT_PARA_CAP_PF_X100: {=u16:?} }}",
            self.TRIM_VALID(),
            self.XTAL_LOAD_CAP_IEC_PF_X100(),
            self.PCB_XIN_PARA_CAP_PF_X100(),
            self.PCB_XOUT_PARA_CAP_PF_X100()
        )
    }
}
