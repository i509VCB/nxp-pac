#[doc = "CMPA Page programming on going. This field shall be set to 0x5CC55AA5 in the active CFPA page each time CMPA page programming is going on. It shall always be set to 0x00000000 in the CFPA scratch area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmpaProgInProgress(pub u32);
impl CmpaProgInProgress {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmpaProgInProgress {
    #[inline(always)]
    fn default() -> CmpaProgInProgress {
        CmpaProgInProgress(0)
    }
}
impl core::fmt::Debug for CmpaProgInProgress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmpaProgInProgress")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmpaProgInProgress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmpaProgInProgress {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Customer Defined (Programable through ROM API)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CustomerDefined(pub u32);
impl CustomerDefined {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CustomerDefined {
    #[inline(always)]
    fn default() -> CustomerDefined {
        CustomerDefined(0)
    }
}
impl core::fmt::Debug for CustomerDefined {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CustomerDefined")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CustomerDefined {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CustomerDefined {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcfgCcSocuDflt(pub u32);
impl DcfgCcSocuDflt {
    #[doc = "Non Secure non-invasive debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Non Secure debug fixed state"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn spniden(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub const fn set_spniden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn spiden(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Secure invasive debug fixed state"]
    #[inline(always)]
    pub const fn set_spiden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn tapen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG TAP fixed state"]
    #[inline(always)]
    pub const fn set_tapen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ISP Boot Command fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn isp_cmd_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ISP Boot Command fixed state"]
    #[inline(always)]
    pub const fn set_isp_cmd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn fa_cmd_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "FA Command fixed state"]
    #[inline(always)]
    pub const fn set_fa_cmd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flash Mass Erase Command fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn me_cmd_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Mass Erase Command fixed state"]
    #[inline(always)]
    pub const fn set_me_cmd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub const fn set_inverse_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DcfgCcSocuDflt {
    #[inline(always)]
    fn default() -> DcfgCcSocuDflt {
        DcfgCcSocuDflt(0)
    }
}
impl core::fmt::Debug for DcfgCcSocuDflt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcfgCcSocuDflt")
            .field("niden", &self.niden())
            .field("dbgen", &self.dbgen())
            .field("spniden", &self.spniden())
            .field("spiden", &self.spiden())
            .field("tapen", &self.tapen())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("isp_cmd_en", &self.isp_cmd_en())
            .field("fa_cmd_en", &self.fa_cmd_en())
            .field("me_cmd_en", &self.me_cmd_en())
            .field("cpu1_niden", &self.cpu1_niden())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcfgCcSocuDflt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcfgCcSocuDflt {{ niden: {=bool:?}, dbgen: {=bool:?}, spniden: {=bool:?}, spiden: {=bool:?}, tapen: {=bool:?}, cpu1_dbgen: {=bool:?}, isp_cmd_en: {=bool:?}, fa_cmd_en: {=bool:?}, me_cmd_en: {=bool:?}, cpu1_niden: {=bool:?}, inverse_value: {=u16:?} }}",
            self.niden(),
            self.dbgen(),
            self.spniden(),
            self.spiden(),
            self.tapen(),
            self.cpu1_dbgen(),
            self.isp_cmd_en(),
            self.fa_cmd_en(),
            self.me_cmd_en(),
            self.cpu1_niden(),
            self.inverse_value()
        )
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcfgCcSocuPin(pub u32);
impl DcfgCcSocuPin {
    #[doc = "Non Secure non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> super::vals::DcfgCcSocuPinNiden {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DcfgCcSocuPinNiden::from_bits(val as u8)
    }
    #[doc = "Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: super::vals::DcfgCcSocuPinNiden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::DcfgCcSocuPinDbgen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DcfgCcSocuPinDbgen::from_bits(val as u8)
    }
    #[doc = "Non Secure debug enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::DcfgCcSocuPinDbgen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spniden(&self) -> super::vals::DcfgCcSocuPinSpniden {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DcfgCcSocuPinSpniden::from_bits(val as u8)
    }
    #[doc = "Secure non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_spniden(&mut self, val: super::vals::DcfgCcSocuPinSpniden) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spiden(&self) -> super::vals::DcfgCcSocuPinSpiden {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DcfgCcSocuPinSpiden::from_bits(val as u8)
    }
    #[doc = "Secure invasive debug enable"]
    #[inline(always)]
    pub const fn set_spiden(&mut self, val: super::vals::DcfgCcSocuPinSpiden) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tapen(&self) -> super::vals::DcfgCcSocuPinTapen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DcfgCcSocuPinTapen::from_bits(val as u8)
    }
    #[doc = "JTAG TAP enable"]
    #[inline(always)]
    pub const fn set_tapen(&mut self, val: super::vals::DcfgCcSocuPinTapen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::DcfgCcSocuPinCpu1Dbgen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DcfgCcSocuPinCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable"]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::DcfgCcSocuPinCpu1Dbgen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "ISP Boot Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isp_cmd_en(&self) -> super::vals::DcfgCcSocuPinIspCmdEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DcfgCcSocuPinIspCmdEn::from_bits(val as u8)
    }
    #[doc = "ISP Boot Command enable"]
    #[inline(always)]
    pub const fn set_isp_cmd_en(&mut self, val: super::vals::DcfgCcSocuPinIspCmdEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fa_cmd_en(&self) -> super::vals::DcfgCcSocuPinFaCmdEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DcfgCcSocuPinFaCmdEn::from_bits(val as u8)
    }
    #[doc = "FA Command enable"]
    #[inline(always)]
    pub const fn set_fa_cmd_en(&mut self, val: super::vals::DcfgCcSocuPinFaCmdEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Flash Mass Erase Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn me_cmd_en(&self) -> super::vals::DcfgCcSocuPinMeCmdEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DcfgCcSocuPinMeCmdEn::from_bits(val as u8)
    }
    #[doc = "Flash Mass Erase Command enable"]
    #[inline(always)]
    pub const fn set_me_cmd_en(&mut self, val: super::vals::DcfgCcSocuPinMeCmdEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::DcfgCcSocuPinCpu1Niden {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DcfgCcSocuPinCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::DcfgCcSocuPinCpu1Niden) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enforce UUID match during Debug authentication."]
    #[must_use]
    #[inline(always)]
    pub const fn uuid_check(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enforce UUID match during Debug authentication."]
    #[inline(always)]
    pub const fn set_uuid_check(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub const fn set_inverse_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for DcfgCcSocuPin {
    #[inline(always)]
    fn default() -> DcfgCcSocuPin {
        DcfgCcSocuPin(0)
    }
}
impl core::fmt::Debug for DcfgCcSocuPin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcfgCcSocuPin")
            .field("niden", &self.niden())
            .field("dbgen", &self.dbgen())
            .field("spniden", &self.spniden())
            .field("spiden", &self.spiden())
            .field("tapen", &self.tapen())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("isp_cmd_en", &self.isp_cmd_en())
            .field("fa_cmd_en", &self.fa_cmd_en())
            .field("me_cmd_en", &self.me_cmd_en())
            .field("cpu1_niden", &self.cpu1_niden())
            .field("uuid_check", &self.uuid_check())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcfgCcSocuPin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcfgCcSocuPin {{ niden: {:?}, dbgen: {:?}, spniden: {:?}, spiden: {:?}, tapen: {:?}, cpu1_dbgen: {:?}, isp_cmd_en: {:?}, fa_cmd_en: {:?}, me_cmd_en: {:?}, cpu1_niden: {:?}, uuid_check: {=bool:?}, inverse_value: {=u16:?} }}",
            self.niden(),
            self.dbgen(),
            self.spniden(),
            self.spiden(),
            self.tapen(),
            self.cpu1_dbgen(),
            self.isp_cmd_en(),
            self.fa_cmd_en(),
            self.me_cmd_en(),
            self.cpu1_niden(),
            self.uuid_check(),
            self.inverse_value()
        )
    }
}
#[doc = "Enable FA mode. SET_FA_MODE Command should write 0xC33CA55A to this word to indicate boot ROM to enter FA mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnableFaMode(pub u32);
impl EnableFaMode {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EnableFaMode {
    #[inline(always)]
    fn default() -> EnableFaMode {
        EnableFaMode(0)
    }
}
impl core::fmt::Debug for EnableFaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnableFaMode")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnableFaMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnableFaMode {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Header(pub u32);
impl Header {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Header {
    #[inline(always)]
    fn default() -> Header {
        Header(0)
    }
}
impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Header {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Header {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Image key revocation ID (Monotonic counter)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ImageKeyRevoke(pub u32);
impl ImageKeyRevoke {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ImageKeyRevoke {
    #[inline(always)]
    fn default() -> ImageKeyRevoke {
        ImageKeyRevoke(0)
    }
}
impl core::fmt::Debug for ImageKeyRevoke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ImageKeyRevoke")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ImageKeyRevoke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ImageKeyRevoke {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Non-Secure firmware version (Monotonic counter)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NsFwVersion(pub u32);
impl NsFwVersion {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NsFwVersion {
    #[inline(always)]
    fn default() -> NsFwVersion {
        NsFwVersion(0)
    }
}
impl core::fmt::Debug for NsFwVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NsFwVersion")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NsFwVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NsFwVersion {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody0(pub u32);
impl PrinceRegion0IvBody0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody0 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody0 {
        PrinceRegion0IvBody0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody1(pub u32);
impl PrinceRegion0IvBody1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody1 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody1 {
        PrinceRegion0IvBody1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody10(pub u32);
impl PrinceRegion0IvBody10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody10 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody10 {
        PrinceRegion0IvBody10(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody11(pub u32);
impl PrinceRegion0IvBody11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody11 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody11 {
        PrinceRegion0IvBody11(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody2(pub u32);
impl PrinceRegion0IvBody2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody2 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody2 {
        PrinceRegion0IvBody2(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody3(pub u32);
impl PrinceRegion0IvBody3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody3 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody3 {
        PrinceRegion0IvBody3(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody4(pub u32);
impl PrinceRegion0IvBody4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody4 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody4 {
        PrinceRegion0IvBody4(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody5(pub u32);
impl PrinceRegion0IvBody5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody5 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody5 {
        PrinceRegion0IvBody5(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody6(pub u32);
impl PrinceRegion0IvBody6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody6 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody6 {
        PrinceRegion0IvBody6(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody7(pub u32);
impl PrinceRegion0IvBody7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody7 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody7 {
        PrinceRegion0IvBody7(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody8(pub u32);
impl PrinceRegion0IvBody8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody8 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody8 {
        PrinceRegion0IvBody8(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvBody9(pub u32);
impl PrinceRegion0IvBody9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvBody9 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvBody9 {
        PrinceRegion0IvBody9(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvBody9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode0(pub u32);
impl PrinceRegion0IvCode0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode0 {
        PrinceRegion0IvCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode1(pub u32);
impl PrinceRegion0IvCode1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode1 {
        PrinceRegion0IvCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode10(pub u32);
impl PrinceRegion0IvCode10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode10 {
        PrinceRegion0IvCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode11(pub u32);
impl PrinceRegion0IvCode11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode11 {
        PrinceRegion0IvCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode12(pub u32);
impl PrinceRegion0IvCode12 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode12 {
        PrinceRegion0IvCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode13(pub u32);
impl PrinceRegion0IvCode13 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode13 {
        PrinceRegion0IvCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode2(pub u32);
impl PrinceRegion0IvCode2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode2 {
        PrinceRegion0IvCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode3(pub u32);
impl PrinceRegion0IvCode3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode3 {
        PrinceRegion0IvCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode4(pub u32);
impl PrinceRegion0IvCode4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode4 {
        PrinceRegion0IvCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode5(pub u32);
impl PrinceRegion0IvCode5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode5 {
        PrinceRegion0IvCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode6(pub u32);
impl PrinceRegion0IvCode6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode6 {
        PrinceRegion0IvCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode7(pub u32);
impl PrinceRegion0IvCode7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode7 {
        PrinceRegion0IvCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode8(pub u32);
impl PrinceRegion0IvCode8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode8 {
        PrinceRegion0IvCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvCode9(pub u32);
impl PrinceRegion0IvCode9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvCode9 {
        PrinceRegion0IvCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvHeader0(pub u32);
impl PrinceRegion0IvHeader0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion0IvHeader0 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvHeader0 {
        PrinceRegion0IvHeader0(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvHeader0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion0IvHeader1(pub u32);
impl PrinceRegion0IvHeader1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion0IvHeader1 {
    #[inline(always)]
    fn default() -> PrinceRegion0IvHeader1 {
        PrinceRegion0IvHeader1(0)
    }
}
impl core::fmt::Debug for PrinceRegion0IvHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion0IvHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion0IvHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion0IvHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody0(pub u32);
impl PrinceRegion1IvBody0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody0 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody0 {
        PrinceRegion1IvBody0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody1(pub u32);
impl PrinceRegion1IvBody1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody1 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody1 {
        PrinceRegion1IvBody1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody10(pub u32);
impl PrinceRegion1IvBody10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody10 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody10 {
        PrinceRegion1IvBody10(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody11(pub u32);
impl PrinceRegion1IvBody11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody11 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody11 {
        PrinceRegion1IvBody11(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody2(pub u32);
impl PrinceRegion1IvBody2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody2 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody2 {
        PrinceRegion1IvBody2(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody3(pub u32);
impl PrinceRegion1IvBody3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody3 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody3 {
        PrinceRegion1IvBody3(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody4(pub u32);
impl PrinceRegion1IvBody4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody4 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody4 {
        PrinceRegion1IvBody4(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody5(pub u32);
impl PrinceRegion1IvBody5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody5 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody5 {
        PrinceRegion1IvBody5(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody6(pub u32);
impl PrinceRegion1IvBody6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody6 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody6 {
        PrinceRegion1IvBody6(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody7(pub u32);
impl PrinceRegion1IvBody7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody7 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody7 {
        PrinceRegion1IvBody7(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody8(pub u32);
impl PrinceRegion1IvBody8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody8 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody8 {
        PrinceRegion1IvBody8(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvBody9(pub u32);
impl PrinceRegion1IvBody9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvBody9 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvBody9 {
        PrinceRegion1IvBody9(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvBody9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode0(pub u32);
impl PrinceRegion1IvCode0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode0 {
        PrinceRegion1IvCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode1(pub u32);
impl PrinceRegion1IvCode1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode1 {
        PrinceRegion1IvCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode10(pub u32);
impl PrinceRegion1IvCode10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode10 {
        PrinceRegion1IvCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode11(pub u32);
impl PrinceRegion1IvCode11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode11 {
        PrinceRegion1IvCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode12(pub u32);
impl PrinceRegion1IvCode12 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode12 {
        PrinceRegion1IvCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode13(pub u32);
impl PrinceRegion1IvCode13 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode13 {
        PrinceRegion1IvCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode2(pub u32);
impl PrinceRegion1IvCode2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode2 {
        PrinceRegion1IvCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode3(pub u32);
impl PrinceRegion1IvCode3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode3 {
        PrinceRegion1IvCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode4(pub u32);
impl PrinceRegion1IvCode4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode4 {
        PrinceRegion1IvCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode5(pub u32);
impl PrinceRegion1IvCode5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode5 {
        PrinceRegion1IvCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode6(pub u32);
impl PrinceRegion1IvCode6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode6 {
        PrinceRegion1IvCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode7(pub u32);
impl PrinceRegion1IvCode7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode7 {
        PrinceRegion1IvCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode8(pub u32);
impl PrinceRegion1IvCode8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode8 {
        PrinceRegion1IvCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvCode9(pub u32);
impl PrinceRegion1IvCode9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvCode9 {
        PrinceRegion1IvCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvHeader0(pub u32);
impl PrinceRegion1IvHeader0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion1IvHeader0 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvHeader0 {
        PrinceRegion1IvHeader0(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvHeader0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion1IvHeader1(pub u32);
impl PrinceRegion1IvHeader1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion1IvHeader1 {
    #[inline(always)]
    fn default() -> PrinceRegion1IvHeader1 {
        PrinceRegion1IvHeader1(0)
    }
}
impl core::fmt::Debug for PrinceRegion1IvHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion1IvHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion1IvHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion1IvHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody0(pub u32);
impl PrinceRegion2IvBody0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody0 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody0 {
        PrinceRegion2IvBody0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody1(pub u32);
impl PrinceRegion2IvBody1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody1 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody1 {
        PrinceRegion2IvBody1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody10(pub u32);
impl PrinceRegion2IvBody10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody10 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody10 {
        PrinceRegion2IvBody10(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody11(pub u32);
impl PrinceRegion2IvBody11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody11 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody11 {
        PrinceRegion2IvBody11(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody2(pub u32);
impl PrinceRegion2IvBody2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody2 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody2 {
        PrinceRegion2IvBody2(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody3(pub u32);
impl PrinceRegion2IvBody3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody3 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody3 {
        PrinceRegion2IvBody3(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody4(pub u32);
impl PrinceRegion2IvBody4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody4 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody4 {
        PrinceRegion2IvBody4(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody5(pub u32);
impl PrinceRegion2IvBody5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody5 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody5 {
        PrinceRegion2IvBody5(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody6(pub u32);
impl PrinceRegion2IvBody6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody6 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody6 {
        PrinceRegion2IvBody6(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody7(pub u32);
impl PrinceRegion2IvBody7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody7 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody7 {
        PrinceRegion2IvBody7(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody8(pub u32);
impl PrinceRegion2IvBody8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody8 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody8 {
        PrinceRegion2IvBody8(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvBody9(pub u32);
impl PrinceRegion2IvBody9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvBody9 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvBody9 {
        PrinceRegion2IvBody9(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvBody9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvBody9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvBody9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvBody9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode0(pub u32);
impl PrinceRegion2IvCode0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode0 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode0 {
        PrinceRegion2IvCode0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode1(pub u32);
impl PrinceRegion2IvCode1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode1 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode1 {
        PrinceRegion2IvCode1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode1 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode10(pub u32);
impl PrinceRegion2IvCode10 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode10 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode10 {
        PrinceRegion2IvCode10(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode10")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode10 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode11(pub u32);
impl PrinceRegion2IvCode11 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode11 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode11 {
        PrinceRegion2IvCode11(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode11")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode11 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode12(pub u32);
impl PrinceRegion2IvCode12 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode12 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode12 {
        PrinceRegion2IvCode12(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode12")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode12 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode13(pub u32);
impl PrinceRegion2IvCode13 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode13 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode13 {
        PrinceRegion2IvCode13(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode13")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode13 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode2(pub u32);
impl PrinceRegion2IvCode2 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode2 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode2 {
        PrinceRegion2IvCode2(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode2 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode3(pub u32);
impl PrinceRegion2IvCode3 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode3 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode3 {
        PrinceRegion2IvCode3(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode3")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode3 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode4(pub u32);
impl PrinceRegion2IvCode4 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode4 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode4 {
        PrinceRegion2IvCode4(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode4")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode4 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode5(pub u32);
impl PrinceRegion2IvCode5 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode5 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode5 {
        PrinceRegion2IvCode5(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode5")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode5 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode6(pub u32);
impl PrinceRegion2IvCode6 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode6 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode6 {
        PrinceRegion2IvCode6(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode6")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode6 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode7(pub u32);
impl PrinceRegion2IvCode7 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode7 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode7 {
        PrinceRegion2IvCode7(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode7")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode7 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode8(pub u32);
impl PrinceRegion2IvCode8 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode8 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode8 {
        PrinceRegion2IvCode8(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode8")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode8 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvCode9(pub u32);
impl PrinceRegion2IvCode9 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvCode9 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvCode9 {
        PrinceRegion2IvCode9(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvCode9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvCode9")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvCode9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvCode9 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvHeader0(pub u32);
impl PrinceRegion2IvHeader0 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrinceRegion2IvHeader0 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvHeader0 {
        PrinceRegion2IvHeader0(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvHeader0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvHeader0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvHeader0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvHeader0 {{ field: {=u32:?} }}",
            self.field()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceRegion2IvHeader1(pub u32);
impl PrinceRegion2IvHeader1 {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for PrinceRegion2IvHeader1 {
    #[inline(always)]
    fn default() -> PrinceRegion2IvHeader1 {
        PrinceRegion2IvHeader1(0)
    }
}
impl core::fmt::Debug for PrinceRegion2IvHeader1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceRegion2IvHeader1")
            .field("type_", &self.type_())
            .field("index", &self.index())
            .field("size", &self.size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceRegion2IvHeader1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceRegion2IvHeader1 {{ type_: {=u8:?}, index: {=u8:?}, size: {=u8:?} }}",
            self.type_(),
            self.index(),
            self.size()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RotkhRevoke(pub u32);
impl RotkhRevoke {
    #[doc = "RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk0_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 0 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub const fn set_ro_tk0_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk1_en(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 1 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub const fn set_ro_tk1_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk2_en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 2 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub const fn set_ro_tk2_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk3_en(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "RoT Key 3 enable. 00 - Invalid 01 - Enabled 10, 11 - Key revoked"]
    #[inline(always)]
    pub const fn set_ro_tk3_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for RotkhRevoke {
    #[inline(always)]
    fn default() -> RotkhRevoke {
        RotkhRevoke(0)
    }
}
impl core::fmt::Debug for RotkhRevoke {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RotkhRevoke")
            .field("ro_tk0_en", &self.ro_tk0_en())
            .field("ro_tk1_en", &self.ro_tk1_en())
            .field("ro_tk2_en", &self.ro_tk2_en())
            .field("ro_tk3_en", &self.ro_tk3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RotkhRevoke {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RotkhRevoke {{ ro_tk0_en: {=u8:?}, ro_tk1_en: {=u8:?}, ro_tk2_en: {=u8:?}, ro_tk3_en: {=u8:?} }}",
            self.ro_tk0_en(),
            self.ro_tk1_en(),
            self.ro_tk2_en(),
            self.ro_tk3_en()
        )
    }
}
#[doc = "Secure firmware version (Monotonic counter)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SFwVersion(pub u32);
impl SFwVersion {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SFwVersion {
    #[inline(always)]
    fn default() -> SFwVersion {
        SFwVersion(0)
    }
}
impl core::fmt::Debug for SFwVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SFwVersion")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SFwVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SFwVersion {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sha256Digest(pub u32);
impl Sha256Digest {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sha256Digest {
    #[inline(always)]
    fn default() -> Sha256Digest {
        Sha256Digest(0)
    }
}
impl core::fmt::Debug for Sha256Digest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sha256Digest")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sha256Digest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sha256Digest {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendorUsage(pub u32);
impl VendorUsage {
    #[doc = "DBG_VENDOR_USAGE."]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_vendor_usage(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DBG_VENDOR_USAGE."]
    #[inline(always)]
    pub const fn set_dbg_vendor_usage(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn inverse_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub const fn set_inverse_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for VendorUsage {
    #[inline(always)]
    fn default() -> VendorUsage {
        VendorUsage(0)
    }
}
impl core::fmt::Debug for VendorUsage {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendorUsage")
            .field("dbg_vendor_usage", &self.dbg_vendor_usage())
            .field("inverse_value", &self.inverse_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendorUsage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendorUsage {{ dbg_vendor_usage: {=u16:?}, inverse_value: {=u16:?} }}",
            self.dbg_vendor_usage(),
            self.inverse_value()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn field(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_field(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Version {{ field: {=u32:?} }}", self.field())
    }
}
