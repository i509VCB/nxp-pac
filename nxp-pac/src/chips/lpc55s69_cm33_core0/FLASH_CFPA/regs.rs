#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMPA_PROG_IN_PROGRESS(pub u32);
impl CMPA_PROG_IN_PROGRESS {
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
impl Default for CMPA_PROG_IN_PROGRESS {
    #[inline(always)]
    fn default() -> CMPA_PROG_IN_PROGRESS {
        CMPA_PROG_IN_PROGRESS(0)
    }
}
impl core::fmt::Debug for CMPA_PROG_IN_PROGRESS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPA_PROG_IN_PROGRESS")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMPA_PROG_IN_PROGRESS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMPA_PROG_IN_PROGRESS {{ FIELD: {=u32:?} }}",
            self.FIELD()
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
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCFG_CC_SOCU_DFLT(pub u32);
impl DCFG_CC_SOCU_DFLT {
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
    #[doc = "CPU1 (Micro cortex M33) invasive debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_DBGEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug fixed state."]
    #[inline(always)]
    pub const fn set_CPU1_DBGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
    pub const fn FA_CMD_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FA Command fixed state."]
    #[inline(always)]
    pub const fn set_FA_CMD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flash Mass Erase Command fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn ME_CMD_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Mass Erase Command fixed state."]
    #[inline(always)]
    pub const fn set_ME_CMD_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_NIDEN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state."]
    #[inline(always)]
    pub const fn set_CPU1_NIDEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
impl Default for DCFG_CC_SOCU_DFLT {
    #[inline(always)]
    fn default() -> DCFG_CC_SOCU_DFLT {
        DCFG_CC_SOCU_DFLT(0)
    }
}
impl core::fmt::Debug for DCFG_CC_SOCU_DFLT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG_CC_SOCU_DFLT")
            .field("NIDEN", &self.NIDEN())
            .field("DBGEN", &self.DBGEN())
            .field("SPNIDEN", &self.SPNIDEN())
            .field("SPIDEN", &self.SPIDEN())
            .field("TAPEN", &self.TAPEN())
            .field("CPU1_DBGEN", &self.CPU1_DBGEN())
            .field("ISP_CMD_EN", &self.ISP_CMD_EN())
            .field("FA_CMD_EN", &self.FA_CMD_EN())
            .field("ME_CMD_EN", &self.ME_CMD_EN())
            .field("CPU1_NIDEN", &self.CPU1_NIDEN())
            .field("INVERSE_VALUE", &self.INVERSE_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCFG_CC_SOCU_DFLT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCFG_CC_SOCU_DFLT {{ NIDEN: {=bool:?}, DBGEN: {=bool:?}, SPNIDEN: {=bool:?}, SPIDEN: {=bool:?}, TAPEN: {=bool:?}, CPU1_DBGEN: {=bool:?}, ISP_CMD_EN: {=bool:?}, FA_CMD_EN: {=bool:?}, ME_CMD_EN: {=bool:?}, CPU1_NIDEN: {=bool:?}, INVERSE_VALUE: {=u16:?} }}",
            self.NIDEN(),
            self.DBGEN(),
            self.SPNIDEN(),
            self.SPIDEN(),
            self.TAPEN(),
            self.CPU1_DBGEN(),
            self.ISP_CMD_EN(),
            self.FA_CMD_EN(),
            self.ME_CMD_EN(),
            self.CPU1_NIDEN(),
            self.INVERSE_VALUE()
        )
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCFG_CC_SOCU_PIN(pub u32);
impl DCFG_CC_SOCU_PIN {
    #[doc = "Non Secure non-invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn NIDEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_NIDEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_NIDEN::from_bits(val as u8)
    }
    #[doc = "Non Secure non-invasive debug enable."]
    #[inline(always)]
    pub const fn set_NIDEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_NIDEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DBGEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_DBGEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_DBGEN::from_bits(val as u8)
    }
    #[doc = "Non Secure debug enable."]
    #[inline(always)]
    pub const fn set_DBGEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_DBGEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SPNIDEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_SPNIDEN {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_SPNIDEN::from_bits(val as u8)
    }
    #[doc = "Secure non-invasive debug enable."]
    #[inline(always)]
    pub const fn set_SPNIDEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_SPNIDEN) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SPIDEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_SPIDEN {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_SPIDEN::from_bits(val as u8)
    }
    #[doc = "Secure invasive debug enable."]
    #[inline(always)]
    pub const fn set_SPIDEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_SPIDEN) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TAPEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_TAPEN {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_TAPEN::from_bits(val as u8)
    }
    #[doc = "JTAG TAP enable."]
    #[inline(always)]
    pub const fn set_TAPEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_TAPEN) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_DBGEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_CPU1_DBGEN {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_CPU1_DBGEN::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable."]
    #[inline(always)]
    pub const fn set_CPU1_DBGEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_CPU1_DBGEN) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "ISP Boot Command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ISP_CMD_EN(&self) -> super::vals::DCFG_CC_SOCU_PIN_ISP_CMD_EN {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_ISP_CMD_EN::from_bits(val as u8)
    }
    #[doc = "ISP Boot Command enable."]
    #[inline(always)]
    pub const fn set_ISP_CMD_EN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_ISP_CMD_EN) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FA_CMD_EN(&self) -> super::vals::DCFG_CC_SOCU_PIN_FA_CMD_EN {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_FA_CMD_EN::from_bits(val as u8)
    }
    #[doc = "FA Command enable."]
    #[inline(always)]
    pub const fn set_FA_CMD_EN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_FA_CMD_EN) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Flash Mass Erase Command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ME_CMD_EN(&self) -> super::vals::DCFG_CC_SOCU_PIN_ME_CMD_EN {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_ME_CMD_EN::from_bits(val as u8)
    }
    #[doc = "Flash Mass Erase Command enable."]
    #[inline(always)]
    pub const fn set_ME_CMD_EN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_ME_CMD_EN) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_NIDEN(&self) -> super::vals::DCFG_CC_SOCU_PIN_CPU1_NIDEN {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DCFG_CC_SOCU_PIN_CPU1_NIDEN::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable."]
    #[inline(always)]
    pub const fn set_CPU1_NIDEN(&mut self, val: super::vals::DCFG_CC_SOCU_PIN_CPU1_NIDEN) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
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
impl Default for DCFG_CC_SOCU_PIN {
    #[inline(always)]
    fn default() -> DCFG_CC_SOCU_PIN {
        DCFG_CC_SOCU_PIN(0)
    }
}
impl core::fmt::Debug for DCFG_CC_SOCU_PIN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCFG_CC_SOCU_PIN")
            .field("NIDEN", &self.NIDEN())
            .field("DBGEN", &self.DBGEN())
            .field("SPNIDEN", &self.SPNIDEN())
            .field("SPIDEN", &self.SPIDEN())
            .field("TAPEN", &self.TAPEN())
            .field("CPU1_DBGEN", &self.CPU1_DBGEN())
            .field("ISP_CMD_EN", &self.ISP_CMD_EN())
            .field("FA_CMD_EN", &self.FA_CMD_EN())
            .field("ME_CMD_EN", &self.ME_CMD_EN())
            .field("CPU1_NIDEN", &self.CPU1_NIDEN())
            .field("UUID_CHECK", &self.UUID_CHECK())
            .field("INVERSE_VALUE", &self.INVERSE_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCFG_CC_SOCU_PIN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCFG_CC_SOCU_PIN {{ NIDEN: {:?}, DBGEN: {:?}, SPNIDEN: {:?}, SPIDEN: {:?}, TAPEN: {:?}, CPU1_DBGEN: {:?}, ISP_CMD_EN: {:?}, FA_CMD_EN: {:?}, ME_CMD_EN: {:?}, CPU1_NIDEN: {:?}, UUID_CHECK: {=bool:?}, INVERSE_VALUE: {=u16:?} }}",
            self.NIDEN(),
            self.DBGEN(),
            self.SPNIDEN(),
            self.SPIDEN(),
            self.TAPEN(),
            self.CPU1_DBGEN(),
            self.ISP_CMD_EN(),
            self.FA_CMD_EN(),
            self.ME_CMD_EN(),
            self.CPU1_NIDEN(),
            self.UUID_CHECK(),
            self.INVERSE_VALUE()
        )
    }
}
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENABLE_FA_MODE(pub u32);
impl ENABLE_FA_MODE {
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
impl Default for ENABLE_FA_MODE {
    #[inline(always)]
    fn default() -> ENABLE_FA_MODE {
        ENABLE_FA_MODE(0)
    }
}
impl core::fmt::Debug for ENABLE_FA_MODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE_FA_MODE")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENABLE_FA_MODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENABLE_FA_MODE {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HEADER(pub u32);
impl HEADER {
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
impl Default for HEADER {
    #[inline(always)]
    fn default() -> HEADER {
        HEADER(0)
    }
}
impl core::fmt::Debug for HEADER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HEADER")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HEADER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HEADER {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Image key revocation ID (Monotonic counter)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IMAGE_KEY_REVOKE(pub u32);
impl IMAGE_KEY_REVOKE {
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
impl Default for IMAGE_KEY_REVOKE {
    #[inline(always)]
    fn default() -> IMAGE_KEY_REVOKE {
        IMAGE_KEY_REVOKE(0)
    }
}
impl core::fmt::Debug for IMAGE_KEY_REVOKE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMAGE_KEY_REVOKE")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IMAGE_KEY_REVOKE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IMAGE_KEY_REVOKE {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "Non-Secure firmware version (Monotonic counter)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NS_FW_Version(pub u32);
impl NS_FW_Version {
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
impl Default for NS_FW_Version {
    #[inline(always)]
    fn default() -> NS_FW_Version {
        NS_FW_Version(0)
    }
}
impl core::fmt::Debug for NS_FW_Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NS_FW_Version")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NS_FW_Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NS_FW_Version {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY0(pub u32);
impl PRINCE_REGION0_IV_BODY0 {
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
impl Default for PRINCE_REGION0_IV_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY0 {
        PRINCE_REGION0_IV_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY1(pub u32);
impl PRINCE_REGION0_IV_BODY1 {
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
impl Default for PRINCE_REGION0_IV_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY1 {
        PRINCE_REGION0_IV_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY10(pub u32);
impl PRINCE_REGION0_IV_BODY10 {
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
impl Default for PRINCE_REGION0_IV_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY10 {
        PRINCE_REGION0_IV_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY11(pub u32);
impl PRINCE_REGION0_IV_BODY11 {
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
impl Default for PRINCE_REGION0_IV_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY11 {
        PRINCE_REGION0_IV_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY2(pub u32);
impl PRINCE_REGION0_IV_BODY2 {
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
impl Default for PRINCE_REGION0_IV_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY2 {
        PRINCE_REGION0_IV_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY3(pub u32);
impl PRINCE_REGION0_IV_BODY3 {
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
impl Default for PRINCE_REGION0_IV_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY3 {
        PRINCE_REGION0_IV_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY4(pub u32);
impl PRINCE_REGION0_IV_BODY4 {
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
impl Default for PRINCE_REGION0_IV_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY4 {
        PRINCE_REGION0_IV_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY5(pub u32);
impl PRINCE_REGION0_IV_BODY5 {
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
impl Default for PRINCE_REGION0_IV_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY5 {
        PRINCE_REGION0_IV_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY6(pub u32);
impl PRINCE_REGION0_IV_BODY6 {
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
impl Default for PRINCE_REGION0_IV_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY6 {
        PRINCE_REGION0_IV_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY7(pub u32);
impl PRINCE_REGION0_IV_BODY7 {
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
impl Default for PRINCE_REGION0_IV_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY7 {
        PRINCE_REGION0_IV_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY8(pub u32);
impl PRINCE_REGION0_IV_BODY8 {
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
impl Default for PRINCE_REGION0_IV_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY8 {
        PRINCE_REGION0_IV_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_BODY9(pub u32);
impl PRINCE_REGION0_IV_BODY9 {
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
impl Default for PRINCE_REGION0_IV_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_BODY9 {
        PRINCE_REGION0_IV_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE0(pub u32);
impl PRINCE_REGION0_IV_CODE0 {
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
impl Default for PRINCE_REGION0_IV_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE0 {
        PRINCE_REGION0_IV_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE1(pub u32);
impl PRINCE_REGION0_IV_CODE1 {
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
impl Default for PRINCE_REGION0_IV_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE1 {
        PRINCE_REGION0_IV_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE10(pub u32);
impl PRINCE_REGION0_IV_CODE10 {
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
impl Default for PRINCE_REGION0_IV_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE10 {
        PRINCE_REGION0_IV_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE11(pub u32);
impl PRINCE_REGION0_IV_CODE11 {
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
impl Default for PRINCE_REGION0_IV_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE11 {
        PRINCE_REGION0_IV_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE12(pub u32);
impl PRINCE_REGION0_IV_CODE12 {
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
impl Default for PRINCE_REGION0_IV_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE12 {
        PRINCE_REGION0_IV_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE13(pub u32);
impl PRINCE_REGION0_IV_CODE13 {
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
impl Default for PRINCE_REGION0_IV_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE13 {
        PRINCE_REGION0_IV_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE2(pub u32);
impl PRINCE_REGION0_IV_CODE2 {
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
impl Default for PRINCE_REGION0_IV_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE2 {
        PRINCE_REGION0_IV_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE3(pub u32);
impl PRINCE_REGION0_IV_CODE3 {
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
impl Default for PRINCE_REGION0_IV_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE3 {
        PRINCE_REGION0_IV_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE4(pub u32);
impl PRINCE_REGION0_IV_CODE4 {
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
impl Default for PRINCE_REGION0_IV_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE4 {
        PRINCE_REGION0_IV_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE5(pub u32);
impl PRINCE_REGION0_IV_CODE5 {
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
impl Default for PRINCE_REGION0_IV_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE5 {
        PRINCE_REGION0_IV_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE6(pub u32);
impl PRINCE_REGION0_IV_CODE6 {
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
impl Default for PRINCE_REGION0_IV_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE6 {
        PRINCE_REGION0_IV_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE7(pub u32);
impl PRINCE_REGION0_IV_CODE7 {
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
impl Default for PRINCE_REGION0_IV_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE7 {
        PRINCE_REGION0_IV_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE8(pub u32);
impl PRINCE_REGION0_IV_CODE8 {
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
impl Default for PRINCE_REGION0_IV_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE8 {
        PRINCE_REGION0_IV_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_CODE9(pub u32);
impl PRINCE_REGION0_IV_CODE9 {
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
impl Default for PRINCE_REGION0_IV_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_CODE9 {
        PRINCE_REGION0_IV_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_HEADER0(pub u32);
impl PRINCE_REGION0_IV_HEADER0 {
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
impl Default for PRINCE_REGION0_IV_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_HEADER0 {
        PRINCE_REGION0_IV_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION0_IV_HEADER1(pub u32);
impl PRINCE_REGION0_IV_HEADER1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION0_IV_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION0_IV_HEADER1 {
        PRINCE_REGION0_IV_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION0_IV_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION0_IV_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION0_IV_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION0_IV_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY0(pub u32);
impl PRINCE_REGION1_IV_BODY0 {
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
impl Default for PRINCE_REGION1_IV_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY0 {
        PRINCE_REGION1_IV_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY1(pub u32);
impl PRINCE_REGION1_IV_BODY1 {
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
impl Default for PRINCE_REGION1_IV_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY1 {
        PRINCE_REGION1_IV_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY10(pub u32);
impl PRINCE_REGION1_IV_BODY10 {
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
impl Default for PRINCE_REGION1_IV_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY10 {
        PRINCE_REGION1_IV_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY11(pub u32);
impl PRINCE_REGION1_IV_BODY11 {
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
impl Default for PRINCE_REGION1_IV_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY11 {
        PRINCE_REGION1_IV_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY2(pub u32);
impl PRINCE_REGION1_IV_BODY2 {
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
impl Default for PRINCE_REGION1_IV_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY2 {
        PRINCE_REGION1_IV_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY3(pub u32);
impl PRINCE_REGION1_IV_BODY3 {
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
impl Default for PRINCE_REGION1_IV_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY3 {
        PRINCE_REGION1_IV_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY4(pub u32);
impl PRINCE_REGION1_IV_BODY4 {
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
impl Default for PRINCE_REGION1_IV_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY4 {
        PRINCE_REGION1_IV_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY5(pub u32);
impl PRINCE_REGION1_IV_BODY5 {
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
impl Default for PRINCE_REGION1_IV_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY5 {
        PRINCE_REGION1_IV_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY6(pub u32);
impl PRINCE_REGION1_IV_BODY6 {
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
impl Default for PRINCE_REGION1_IV_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY6 {
        PRINCE_REGION1_IV_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY7(pub u32);
impl PRINCE_REGION1_IV_BODY7 {
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
impl Default for PRINCE_REGION1_IV_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY7 {
        PRINCE_REGION1_IV_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY8(pub u32);
impl PRINCE_REGION1_IV_BODY8 {
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
impl Default for PRINCE_REGION1_IV_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY8 {
        PRINCE_REGION1_IV_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_BODY9(pub u32);
impl PRINCE_REGION1_IV_BODY9 {
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
impl Default for PRINCE_REGION1_IV_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_BODY9 {
        PRINCE_REGION1_IV_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE0(pub u32);
impl PRINCE_REGION1_IV_CODE0 {
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
impl Default for PRINCE_REGION1_IV_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE0 {
        PRINCE_REGION1_IV_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE1(pub u32);
impl PRINCE_REGION1_IV_CODE1 {
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
impl Default for PRINCE_REGION1_IV_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE1 {
        PRINCE_REGION1_IV_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE10(pub u32);
impl PRINCE_REGION1_IV_CODE10 {
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
impl Default for PRINCE_REGION1_IV_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE10 {
        PRINCE_REGION1_IV_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE11(pub u32);
impl PRINCE_REGION1_IV_CODE11 {
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
impl Default for PRINCE_REGION1_IV_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE11 {
        PRINCE_REGION1_IV_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE12(pub u32);
impl PRINCE_REGION1_IV_CODE12 {
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
impl Default for PRINCE_REGION1_IV_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE12 {
        PRINCE_REGION1_IV_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE13(pub u32);
impl PRINCE_REGION1_IV_CODE13 {
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
impl Default for PRINCE_REGION1_IV_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE13 {
        PRINCE_REGION1_IV_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE2(pub u32);
impl PRINCE_REGION1_IV_CODE2 {
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
impl Default for PRINCE_REGION1_IV_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE2 {
        PRINCE_REGION1_IV_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE3(pub u32);
impl PRINCE_REGION1_IV_CODE3 {
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
impl Default for PRINCE_REGION1_IV_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE3 {
        PRINCE_REGION1_IV_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE4(pub u32);
impl PRINCE_REGION1_IV_CODE4 {
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
impl Default for PRINCE_REGION1_IV_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE4 {
        PRINCE_REGION1_IV_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE5(pub u32);
impl PRINCE_REGION1_IV_CODE5 {
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
impl Default for PRINCE_REGION1_IV_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE5 {
        PRINCE_REGION1_IV_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE6(pub u32);
impl PRINCE_REGION1_IV_CODE6 {
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
impl Default for PRINCE_REGION1_IV_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE6 {
        PRINCE_REGION1_IV_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE7(pub u32);
impl PRINCE_REGION1_IV_CODE7 {
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
impl Default for PRINCE_REGION1_IV_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE7 {
        PRINCE_REGION1_IV_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE8(pub u32);
impl PRINCE_REGION1_IV_CODE8 {
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
impl Default for PRINCE_REGION1_IV_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE8 {
        PRINCE_REGION1_IV_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_CODE9(pub u32);
impl PRINCE_REGION1_IV_CODE9 {
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
impl Default for PRINCE_REGION1_IV_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_CODE9 {
        PRINCE_REGION1_IV_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_HEADER0(pub u32);
impl PRINCE_REGION1_IV_HEADER0 {
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
impl Default for PRINCE_REGION1_IV_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_HEADER0 {
        PRINCE_REGION1_IV_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION1_IV_HEADER1(pub u32);
impl PRINCE_REGION1_IV_HEADER1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION1_IV_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION1_IV_HEADER1 {
        PRINCE_REGION1_IV_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION1_IV_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION1_IV_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION1_IV_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION1_IV_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY0(pub u32);
impl PRINCE_REGION2_IV_BODY0 {
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
impl Default for PRINCE_REGION2_IV_BODY0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY0 {
        PRINCE_REGION2_IV_BODY0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY1(pub u32);
impl PRINCE_REGION2_IV_BODY1 {
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
impl Default for PRINCE_REGION2_IV_BODY1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY1 {
        PRINCE_REGION2_IV_BODY1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY10(pub u32);
impl PRINCE_REGION2_IV_BODY10 {
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
impl Default for PRINCE_REGION2_IV_BODY10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY10 {
        PRINCE_REGION2_IV_BODY10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY11(pub u32);
impl PRINCE_REGION2_IV_BODY11 {
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
impl Default for PRINCE_REGION2_IV_BODY11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY11 {
        PRINCE_REGION2_IV_BODY11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY2(pub u32);
impl PRINCE_REGION2_IV_BODY2 {
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
impl Default for PRINCE_REGION2_IV_BODY2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY2 {
        PRINCE_REGION2_IV_BODY2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY3(pub u32);
impl PRINCE_REGION2_IV_BODY3 {
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
impl Default for PRINCE_REGION2_IV_BODY3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY3 {
        PRINCE_REGION2_IV_BODY3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY4(pub u32);
impl PRINCE_REGION2_IV_BODY4 {
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
impl Default for PRINCE_REGION2_IV_BODY4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY4 {
        PRINCE_REGION2_IV_BODY4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY5(pub u32);
impl PRINCE_REGION2_IV_BODY5 {
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
impl Default for PRINCE_REGION2_IV_BODY5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY5 {
        PRINCE_REGION2_IV_BODY5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY6(pub u32);
impl PRINCE_REGION2_IV_BODY6 {
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
impl Default for PRINCE_REGION2_IV_BODY6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY6 {
        PRINCE_REGION2_IV_BODY6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY7(pub u32);
impl PRINCE_REGION2_IV_BODY7 {
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
impl Default for PRINCE_REGION2_IV_BODY7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY7 {
        PRINCE_REGION2_IV_BODY7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY8(pub u32);
impl PRINCE_REGION2_IV_BODY8 {
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
impl Default for PRINCE_REGION2_IV_BODY8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY8 {
        PRINCE_REGION2_IV_BODY8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_BODY9(pub u32);
impl PRINCE_REGION2_IV_BODY9 {
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
impl Default for PRINCE_REGION2_IV_BODY9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_BODY9 {
        PRINCE_REGION2_IV_BODY9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_BODY9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_BODY9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_BODY9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_BODY9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE0(pub u32);
impl PRINCE_REGION2_IV_CODE0 {
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
impl Default for PRINCE_REGION2_IV_CODE0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE0 {
        PRINCE_REGION2_IV_CODE0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE1(pub u32);
impl PRINCE_REGION2_IV_CODE1 {
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
impl Default for PRINCE_REGION2_IV_CODE1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE1 {
        PRINCE_REGION2_IV_CODE1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE10(pub u32);
impl PRINCE_REGION2_IV_CODE10 {
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
impl Default for PRINCE_REGION2_IV_CODE10 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE10 {
        PRINCE_REGION2_IV_CODE10(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE10")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE10 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE11(pub u32);
impl PRINCE_REGION2_IV_CODE11 {
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
impl Default for PRINCE_REGION2_IV_CODE11 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE11 {
        PRINCE_REGION2_IV_CODE11(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE11")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE11 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE12(pub u32);
impl PRINCE_REGION2_IV_CODE12 {
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
impl Default for PRINCE_REGION2_IV_CODE12 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE12 {
        PRINCE_REGION2_IV_CODE12(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE12")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE12 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE13(pub u32);
impl PRINCE_REGION2_IV_CODE13 {
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
impl Default for PRINCE_REGION2_IV_CODE13 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE13 {
        PRINCE_REGION2_IV_CODE13(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE13")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE13 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE2(pub u32);
impl PRINCE_REGION2_IV_CODE2 {
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
impl Default for PRINCE_REGION2_IV_CODE2 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE2 {
        PRINCE_REGION2_IV_CODE2(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE3(pub u32);
impl PRINCE_REGION2_IV_CODE3 {
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
impl Default for PRINCE_REGION2_IV_CODE3 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE3 {
        PRINCE_REGION2_IV_CODE3(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE4(pub u32);
impl PRINCE_REGION2_IV_CODE4 {
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
impl Default for PRINCE_REGION2_IV_CODE4 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE4 {
        PRINCE_REGION2_IV_CODE4(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE4")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE4 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE5(pub u32);
impl PRINCE_REGION2_IV_CODE5 {
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
impl Default for PRINCE_REGION2_IV_CODE5 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE5 {
        PRINCE_REGION2_IV_CODE5(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE5")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE5 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE6(pub u32);
impl PRINCE_REGION2_IV_CODE6 {
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
impl Default for PRINCE_REGION2_IV_CODE6 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE6 {
        PRINCE_REGION2_IV_CODE6(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE6")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE6 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE7(pub u32);
impl PRINCE_REGION2_IV_CODE7 {
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
impl Default for PRINCE_REGION2_IV_CODE7 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE7 {
        PRINCE_REGION2_IV_CODE7(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE7")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE7 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE8(pub u32);
impl PRINCE_REGION2_IV_CODE8 {
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
impl Default for PRINCE_REGION2_IV_CODE8 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE8 {
        PRINCE_REGION2_IV_CODE8(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE8")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE8 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_CODE9(pub u32);
impl PRINCE_REGION2_IV_CODE9 {
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
impl Default for PRINCE_REGION2_IV_CODE9 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_CODE9 {
        PRINCE_REGION2_IV_CODE9(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_CODE9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_CODE9")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_CODE9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_CODE9 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_HEADER0(pub u32);
impl PRINCE_REGION2_IV_HEADER0 {
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
impl Default for PRINCE_REGION2_IV_HEADER0 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_HEADER0 {
        PRINCE_REGION2_IV_HEADER0(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_HEADER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_HEADER0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_HEADER0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_HEADER0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PRINCE_REGION2_IV_HEADER1(pub u32);
impl PRINCE_REGION2_IV_HEADER1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TYPE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TYPE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn INDEX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_INDEX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn SIZE(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PRINCE_REGION2_IV_HEADER1 {
    #[inline(always)]
    fn default() -> PRINCE_REGION2_IV_HEADER1 {
        PRINCE_REGION2_IV_HEADER1(0)
    }
}
impl core::fmt::Debug for PRINCE_REGION2_IV_HEADER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRINCE_REGION2_IV_HEADER1")
            .field("TYPE", &self.TYPE())
            .field("INDEX", &self.INDEX())
            .field("SIZE", &self.SIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PRINCE_REGION2_IV_HEADER1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PRINCE_REGION2_IV_HEADER1 {{ TYPE: {=u8:?}, INDEX: {=u8:?}, SIZE: {=u8:?} }}",
            self.TYPE(),
            self.INDEX(),
            self.SIZE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ROTKH_REVOKE(pub u32);
impl ROTKH_REVOKE {
    #[doc = "RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[must_use]
    #[inline(always)]
    pub const fn RoTK0_EN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[inline(always)]
    pub const fn set_RoTK0_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[must_use]
    #[inline(always)]
    pub const fn RoTK1_EN(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[inline(always)]
    pub const fn set_RoTK1_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[must_use]
    #[inline(always)]
    pub const fn RoTK2_EN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[inline(always)]
    pub const fn set_RoTK2_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[must_use]
    #[inline(always)]
    pub const fn RoTK3_EN(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked."]
    #[inline(always)]
    pub const fn set_RoTK3_EN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for ROTKH_REVOKE {
    #[inline(always)]
    fn default() -> ROTKH_REVOKE {
        ROTKH_REVOKE(0)
    }
}
impl core::fmt::Debug for ROTKH_REVOKE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROTKH_REVOKE")
            .field("RoTK0_EN", &self.RoTK0_EN())
            .field("RoTK1_EN", &self.RoTK1_EN())
            .field("RoTK2_EN", &self.RoTK2_EN())
            .field("RoTK3_EN", &self.RoTK3_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ROTKH_REVOKE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ROTKH_REVOKE {{ RoTK0_EN: {=u8:?}, RoTK1_EN: {=u8:?}, RoTK2_EN: {=u8:?}, RoTK3_EN: {=u8:?} }}",
            self.RoTK0_EN(),
            self.RoTK1_EN(),
            self.RoTK2_EN(),
            self.RoTK3_EN()
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
#[doc = "Secure firmware version (Monotonic counter)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S_FW_Version(pub u32);
impl S_FW_Version {
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
impl Default for S_FW_Version {
    #[inline(always)]
    fn default() -> S_FW_Version {
        S_FW_Version(0)
    }
}
impl core::fmt::Debug for S_FW_Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S_FW_Version")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S_FW_Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "S_FW_Version {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VENDOR_USAGE(pub u32);
impl VENDOR_USAGE {
    #[doc = "DBG_VENDOR_USAGE."]
    #[must_use]
    #[inline(always)]
    pub const fn DBG_VENDOR_USAGE(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub const fn set_DBG_VENDOR_USAGE(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
impl Default for VENDOR_USAGE {
    #[inline(always)]
    fn default() -> VENDOR_USAGE {
        VENDOR_USAGE(0)
    }
}
impl core::fmt::Debug for VENDOR_USAGE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VENDOR_USAGE")
            .field("DBG_VENDOR_USAGE", &self.DBG_VENDOR_USAGE())
            .field("INVERSE_VALUE", &self.INVERSE_VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VENDOR_USAGE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VENDOR_USAGE {{ DBG_VENDOR_USAGE: {=u16:?}, INVERSE_VALUE: {=u16:?} }}",
            self.DBG_VENDOR_USAGE(),
            self.INVERSE_VALUE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VERSION(pub u32);
impl VERSION {
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
impl Default for VERSION {
    #[inline(always)]
    fn default() -> VERSION {
        VERSION(0)
    }
}
impl core::fmt::Debug for VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "VERSION {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
