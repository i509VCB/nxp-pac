#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootCfg(pub u32);
impl BootCfg {
    #[doc = "Default ISP mode:"]
    #[must_use]
    #[inline(always)]
    pub const fn default_isp_mode(&self) -> super::vals::DefaultIspMode {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::DefaultIspMode::from_bits(val as u8)
    }
    #[doc = "Default ISP mode:"]
    #[inline(always)]
    pub const fn set_default_isp_mode(&mut self, val: super::vals::DefaultIspMode) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Core clock:"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_speed(&self) -> super::vals::BootSpeed {
        let val = (self.0 >> 7usize) & 0x03;
        super::vals::BootSpeed::from_bits(val as u8)
    }
    #[doc = "Core clock:"]
    #[inline(always)]
    pub const fn set_boot_speed(&mut self, val: super::vals::BootSpeed) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
    }
    #[doc = "GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_failure_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\] - Defines GPIO port \\[7:3\\] - Defines GPIO pin"]
    #[inline(always)]
    pub const fn set_boot_failure_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for BootCfg {
    #[inline(always)]
    fn default() -> BootCfg {
        BootCfg(0)
    }
}
impl core::fmt::Debug for BootCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootCfg")
            .field("default_isp_mode", &self.default_isp_mode())
            .field("boot_speed", &self.boot_speed())
            .field("boot_failure_pin", &self.boot_failure_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BootCfg {{ default_isp_mode: {:?}, boot_speed: {:?}, boot_failure_pin: {=u8:?} }}",
            self.default_isp_mode(),
            self.boot_speed(),
            self.boot_failure_pin()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcSocuDflt(pub u32);
impl CcSocuDflt {
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
impl Default for CcSocuDflt {
    #[inline(always)]
    fn default() -> CcSocuDflt {
        CcSocuDflt(0)
    }
}
impl core::fmt::Debug for CcSocuDflt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcSocuDflt")
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
impl defmt::Format for CcSocuDflt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CcSocuDflt {{ niden: {=bool:?}, dbgen: {=bool:?}, spniden: {=bool:?}, spiden: {=bool:?}, tapen: {=bool:?}, cpu1_dbgen: {=bool:?}, isp_cmd_en: {=bool:?}, fa_cmd_en: {=bool:?}, me_cmd_en: {=bool:?}, cpu1_niden: {=bool:?}, inverse_value: {=u16:?} }}",
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
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CcSocuPin(pub u32);
impl CcSocuPin {
    #[doc = "Non Secure non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> super::vals::CcSocuPinNiden {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CcSocuPinNiden::from_bits(val as u8)
    }
    #[doc = "Non Secure non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: super::vals::CcSocuPinNiden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Non Secure debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> super::vals::CcSocuPinDbgen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CcSocuPinDbgen::from_bits(val as u8)
    }
    #[doc = "Non Secure debug enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: super::vals::CcSocuPinDbgen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Secure non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spniden(&self) -> super::vals::CcSocuPinSpniden {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CcSocuPinSpniden::from_bits(val as u8)
    }
    #[doc = "Secure non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_spniden(&mut self, val: super::vals::CcSocuPinSpniden) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Secure invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spiden(&self) -> super::vals::CcSocuPinSpiden {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CcSocuPinSpiden::from_bits(val as u8)
    }
    #[doc = "Secure invasive debug enable"]
    #[inline(always)]
    pub const fn set_spiden(&mut self, val: super::vals::CcSocuPinSpiden) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "JTAG TAP enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tapen(&self) -> super::vals::CcSocuPinTapen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CcSocuPinTapen::from_bits(val as u8)
    }
    #[doc = "JTAG TAP enable"]
    #[inline(always)]
    pub const fn set_tapen(&mut self, val: super::vals::CcSocuPinTapen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> super::vals::CcSocuPinCpu1Dbgen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::CcSocuPinCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) invasive debug enable"]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: super::vals::CcSocuPinCpu1Dbgen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "ISP Boot Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isp_cmd_en(&self) -> super::vals::CcSocuPinIspCmdEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CcSocuPinIspCmdEn::from_bits(val as u8)
    }
    #[doc = "ISP Boot Command enable"]
    #[inline(always)]
    pub const fn set_isp_cmd_en(&mut self, val: super::vals::CcSocuPinIspCmdEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FA Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fa_cmd_en(&self) -> super::vals::CcSocuPinFaCmdEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CcSocuPinFaCmdEn::from_bits(val as u8)
    }
    #[doc = "FA Command enable"]
    #[inline(always)]
    pub const fn set_fa_cmd_en(&mut self, val: super::vals::CcSocuPinFaCmdEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Flash Mass Erase Command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn me_cmd_en(&self) -> super::vals::CcSocuPinMeCmdEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::CcSocuPinMeCmdEn::from_bits(val as u8)
    }
    #[doc = "Flash Mass Erase Command enable"]
    #[inline(always)]
    pub const fn set_me_cmd_en(&mut self, val: super::vals::CcSocuPinMeCmdEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> super::vals::CcSocuPinCpu1Niden {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::CcSocuPinCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 (Micro cortex M33) non-invasive debug enable"]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: super::vals::CcSocuPinCpu1Niden) {
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
impl Default for CcSocuPin {
    #[inline(always)]
    fn default() -> CcSocuPin {
        CcSocuPin(0)
    }
}
impl core::fmt::Debug for CcSocuPin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CcSocuPin")
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
impl defmt::Format for CcSocuPin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CcSocuPin {{ niden: {:?}, dbgen: {:?}, spniden: {:?}, spiden: {:?}, tapen: {:?}, cpu1_dbgen: {:?}, isp_cmd_en: {:?}, fa_cmd_en: {:?}, me_cmd_en: {:?}, cpu1_niden: {:?}, uuid_check: {=bool:?}, inverse_value: {=u16:?} }}",
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
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceBaseAddr(pub u32);
impl PrinceBaseAddr {
    #[doc = "Programmable portion of the base address of region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr0_prg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 0"]
    #[inline(always)]
    pub const fn set_addr0_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr1_prg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 1"]
    #[inline(always)]
    pub const fn set_addr1_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Programmable portion of the base address of region 2"]
    #[must_use]
    #[inline(always)]
    pub const fn addr2_prg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 2"]
    #[inline(always)]
    pub const fn set_addr2_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Lock PRINCE region0 settings"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_reg0(&self) -> super::vals::LockReg0 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::LockReg0::from_bits(val as u8)
    }
    #[doc = "Lock PRINCE region0 settings"]
    #[inline(always)]
    pub const fn set_lock_reg0(&mut self, val: super::vals::LockReg0) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Lock PRINCE region1 settings"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_reg1(&self) -> super::vals::LockReg1 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::LockReg1::from_bits(val as u8)
    }
    #[doc = "Lock PRINCE region1 settings"]
    #[inline(always)]
    pub const fn set_lock_reg1(&mut self, val: super::vals::LockReg1) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together"]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_erase_check_en(&self) -> super::vals::Reg0EraseCheckEn {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Reg0EraseCheckEn::from_bits(val as u8)
    }
    #[doc = "For PRINCE region0 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub const fn set_reg0_erase_check_en(&mut self, val: super::vals::Reg0EraseCheckEn) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together"]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_erase_check_en(&self) -> super::vals::Reg1EraseCheckEn {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Reg1EraseCheckEn::from_bits(val as u8)
    }
    #[doc = "For PRINCE region1 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub const fn set_reg1_erase_check_en(&mut self, val: super::vals::Reg1EraseCheckEn) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together"]
    #[must_use]
    #[inline(always)]
    pub const fn reg2_erase_check_en(&self) -> super::vals::Reg2EraseCheckEn {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Reg2EraseCheckEn::from_bits(val as u8)
    }
    #[doc = "For PRINCE region2 enable checking whether all encrypted pages are erased together"]
    #[inline(always)]
    pub const fn set_reg2_erase_check_en(&mut self, val: super::vals::Reg2EraseCheckEn) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for PrinceBaseAddr {
    #[inline(always)]
    fn default() -> PrinceBaseAddr {
        PrinceBaseAddr(0)
    }
}
impl core::fmt::Debug for PrinceBaseAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceBaseAddr")
            .field("addr0_prg", &self.addr0_prg())
            .field("addr1_prg", &self.addr1_prg())
            .field("addr2_prg", &self.addr2_prg())
            .field("lock_reg0", &self.lock_reg0())
            .field("lock_reg1", &self.lock_reg1())
            .field("reg0_erase_check_en", &self.reg0_erase_check_en())
            .field("reg1_erase_check_en", &self.reg1_erase_check_en())
            .field("reg2_erase_check_en", &self.reg2_erase_check_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceBaseAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrinceBaseAddr {{ addr0_prg: {=u8:?}, addr1_prg: {=u8:?}, addr2_prg: {=u8:?}, lock_reg0: {:?}, lock_reg1: {:?}, reg0_erase_check_en: {:?}, reg1_erase_check_en: {:?}, reg2_erase_check_en: {:?} }}",
            self.addr0_prg(),
            self.addr1_prg(),
            self.addr2_prg(),
            self.lock_reg0(),
            self.lock_reg1(),
            self.reg0_erase_check_en(),
            self.reg1_erase_check_en(),
            self.reg2_erase_check_en()
        )
    }
}
#[doc = "Region 0, sub-region enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceSr0(pub u32);
impl PrinceSr0 {
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
impl Default for PrinceSr0 {
    #[inline(always)]
    fn default() -> PrinceSr0 {
        PrinceSr0(0)
    }
}
impl core::fmt::Debug for PrinceSr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceSr0")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceSr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceSr0 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Region 1, sub-region enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceSr1(pub u32);
impl PrinceSr1 {
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
impl Default for PrinceSr1 {
    #[inline(always)]
    fn default() -> PrinceSr1 {
        PrinceSr1(0)
    }
}
impl core::fmt::Debug for PrinceSr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceSr1")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceSr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceSr1 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Region 2, sub-region enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrinceSr2(pub u32);
impl PrinceSr2 {
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
impl Default for PrinceSr2 {
    #[inline(always)]
    fn default() -> PrinceSr2 {
        PrinceSr2(0)
    }
}
impl core::fmt::Debug for PrinceSr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrinceSr2")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrinceSr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PrinceSr2 {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "ROTKHindex for Root of Trust Keys Table hash\\[(((7 - index) * 32) + 31):((7 - index) * 32)\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rotkh(pub u32);
impl Rotkh {
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
impl Default for Rotkh {
    #[inline(always)]
    fn default() -> Rotkh {
        Rotkh(0)
    }
}
impl core::fmt::Debug for Rotkh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rotkh")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rotkh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rotkh {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdioCfg(pub u32);
impl SdioCfg {
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
impl Default for SdioCfg {
    #[inline(always)]
    fn default() -> SdioCfg {
        SdioCfg(0)
    }
}
impl core::fmt::Debug for SdioCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SdioCfg")
            .field("field", &self.field())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SdioCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SdioCfg {{ field: {=u32:?} }}", self.field())
    }
}
#[doc = "Secure boot configuration flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecureBootCfg(pub u32);
impl SecureBootCfg {
    #[doc = "Use RSA4096 keys only."]
    #[must_use]
    #[inline(always)]
    pub const fn rsa4k(&self) -> super::vals::Rsa4k {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Rsa4k::from_bits(val as u8)
    }
    #[doc = "Use RSA4096 keys only."]
    #[inline(always)]
    pub const fn set_rsa4k(&mut self, val: super::vals::Rsa4k) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Include NXP area in DICE computation."]
    #[must_use]
    #[inline(always)]
    pub const fn dice_inc_nxp_cfg(&self) -> super::vals::DiceIncNxpCfg {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DiceIncNxpCfg::from_bits(val as u8)
    }
    #[doc = "Include NXP area in DICE computation."]
    #[inline(always)]
    pub const fn set_dice_inc_nxp_cfg(&mut self, val: super::vals::DiceIncNxpCfg) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Include Customer factory area (including keys) in DICE computation."]
    #[must_use]
    #[inline(always)]
    pub const fn dice_cust_cfg(&self) -> super::vals::DiceCustCfg {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DiceCustCfg::from_bits(val as u8)
    }
    #[doc = "Include Customer factory area (including keys) in DICE computation."]
    #[inline(always)]
    pub const fn set_dice_cust_cfg(&mut self, val: super::vals::DiceCustCfg) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Skip DICE computation"]
    #[must_use]
    #[inline(always)]
    pub const fn skip_dice(&self) -> super::vals::SkipDice {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SkipDice::from_bits(val as u8)
    }
    #[doc = "Skip DICE computation"]
    #[inline(always)]
    pub const fn set_skip_dice(&mut self, val: super::vals::SkipDice) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "TrustZone-M mode"]
    #[must_use]
    #[inline(always)]
    pub const fn tzm_image_type(&self) -> super::vals::TzmImageType {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::TzmImageType::from_bits(val as u8)
    }
    #[doc = "TrustZone-M mode"]
    #[inline(always)]
    pub const fn set_tzm_image_type(&mut self, val: super::vals::TzmImageType) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Block PUF key code generation"]
    #[must_use]
    #[inline(always)]
    pub const fn block_set_key(&self) -> super::vals::BlockSetKey {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::BlockSetKey::from_bits(val as u8)
    }
    #[doc = "Block PUF key code generation"]
    #[inline(always)]
    pub const fn set_block_set_key(&mut self, val: super::vals::BlockSetKey) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Block PUF enrollement"]
    #[must_use]
    #[inline(always)]
    pub const fn block_enroll(&self) -> super::vals::BlockEnroll {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::BlockEnroll::from_bits(val as u8)
    }
    #[doc = "Block PUF enrollement"]
    #[inline(always)]
    pub const fn set_block_enroll(&mut self, val: super::vals::BlockEnroll) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Include security EPOCH in DICE"]
    #[must_use]
    #[inline(always)]
    pub const fn dice_inc_sec_epoch(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Include security EPOCH in DICE"]
    #[inline(always)]
    pub const fn set_dice_inc_sec_epoch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Secure boot enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_boot_en(&self) -> super::vals::SecBootEn {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::SecBootEn::from_bits(val as u8)
    }
    #[doc = "Secure boot enable"]
    #[inline(always)]
    pub const fn set_sec_boot_en(&mut self, val: super::vals::SecBootEn) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for SecureBootCfg {
    #[inline(always)]
    fn default() -> SecureBootCfg {
        SecureBootCfg(0)
    }
}
impl core::fmt::Debug for SecureBootCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecureBootCfg")
            .field("rsa4k", &self.rsa4k())
            .field("dice_inc_nxp_cfg", &self.dice_inc_nxp_cfg())
            .field("dice_cust_cfg", &self.dice_cust_cfg())
            .field("skip_dice", &self.skip_dice())
            .field("tzm_image_type", &self.tzm_image_type())
            .field("block_set_key", &self.block_set_key())
            .field("block_enroll", &self.block_enroll())
            .field("dice_inc_sec_epoch", &self.dice_inc_sec_epoch())
            .field("sec_boot_en", &self.sec_boot_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecureBootCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecureBootCfg {{ rsa4k: {:?}, dice_inc_nxp_cfg: {:?}, dice_cust_cfg: {:?}, skip_dice: {:?}, tzm_image_type: {:?}, block_set_key: {:?}, block_enroll: {:?}, dice_inc_sec_epoch: {=u8:?}, sec_boot_en: {:?} }}",
            self.rsa4k(),
            self.dice_inc_nxp_cfg(),
            self.dice_cust_cfg(),
            self.skip_dice(),
            self.tzm_image_type(),
            self.block_set_key(),
            self.block_enroll(),
            self.dice_inc_sec_epoch(),
            self.sec_boot_en()
        )
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
pub struct SpiFlashCfg(pub u32);
impl SpiFlashCfg {
    #[doc = "SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[must_use]
    #[inline(always)]
    pub const fn spi_recovery_boot_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "SPI flash recovery boot is enabled, if non-zero value is written to this field."]
    #[inline(always)]
    pub const fn set_spi_recovery_boot_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for SpiFlashCfg {
    #[inline(always)]
    fn default() -> SpiFlashCfg {
        SpiFlashCfg(0)
    }
}
impl core::fmt::Debug for SpiFlashCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpiFlashCfg")
            .field("spi_recovery_boot_en", &self.spi_recovery_boot_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpiFlashCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SpiFlashCfg {{ spi_recovery_boot_en: {=u8:?} }}",
            self.spi_recovery_boot_en()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbId(pub u32);
impl UsbId {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_product_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_usb_product_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for UsbId {
    #[inline(always)]
    fn default() -> UsbId {
        UsbId(0)
    }
}
impl core::fmt::Debug for UsbId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbId")
            .field("usb_vendor_id", &self.usb_vendor_id())
            .field("usb_product_id", &self.usb_product_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbId {{ usb_vendor_id: {=u16:?}, usb_product_id: {=u16:?} }}",
            self.usb_vendor_id(),
            self.usb_product_id()
        )
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendorUsage(pub u32);
impl VendorUsage {
    #[doc = "Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_usage(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Upper 16 bits of vendor usage field defined in DAP. Lower 16-bits come from customer field area."]
    #[inline(always)]
    pub const fn set_vendor_usage(&mut self, val: u16) {
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
            .field("vendor_usage", &self.vendor_usage())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendorUsage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendorUsage {{ vendor_usage: {=u16:?} }}",
            self.vendor_usage()
        )
    }
}
#[doc = "Xtal 16MHz capabank triming."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal16mhzCapabankTrim(pub u32);
impl Xtal16mhzCapabankTrim {
    #[doc = "XTAL 16MHz capa bank trimmings"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_valid(&self) -> super::vals::Xtal16mhzCapabankTrimTrimValid {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xtal16mhzCapabankTrimTrimValid::from_bits(val as u8)
    }
    #[doc = "XTAL 16MHz capa bank trimmings"]
    #[inline(always)]
    pub const fn set_trim_valid(&mut self, val: super::vals::Xtal16mhzCapabankTrimTrimValid) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_load_cap_iec_pf_x100(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_xtal_load_cap_iec_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn pcb_xin_para_cap_pf_x100(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_pcb_xin_para_cap_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn pcb_xout_para_cap_pf_x100(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_pcb_xout_para_cap_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for Xtal16mhzCapabankTrim {
    #[inline(always)]
    fn default() -> Xtal16mhzCapabankTrim {
        Xtal16mhzCapabankTrim(0)
    }
}
impl core::fmt::Debug for Xtal16mhzCapabankTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xtal16mhzCapabankTrim")
            .field("trim_valid", &self.trim_valid())
            .field(
                "xtal_load_cap_iec_pf_x100",
                &self.xtal_load_cap_iec_pf_x100(),
            )
            .field("pcb_xin_para_cap_pf_x100", &self.pcb_xin_para_cap_pf_x100())
            .field(
                "pcb_xout_para_cap_pf_x100",
                &self.pcb_xout_para_cap_pf_x100(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xtal16mhzCapabankTrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xtal16mhzCapabankTrim {{ trim_valid: {:?}, xtal_load_cap_iec_pf_x100: {=u16:?}, pcb_xin_para_cap_pf_x100: {=u16:?}, pcb_xout_para_cap_pf_x100: {=u16:?} }}",
            self.trim_valid(),
            self.xtal_load_cap_iec_pf_x100(),
            self.pcb_xin_para_cap_pf_x100(),
            self.pcb_xout_para_cap_pf_x100()
        )
    }
}
#[doc = "Xtal 32kHz capabank triming."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32khzCapabankTrim(pub u32);
impl Xtal32khzCapabankTrim {
    #[doc = "XTAL 32kHz capa bank trimmings"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_valid(&self) -> super::vals::Xtal32khzCapabankTrimTrimValid {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Xtal32khzCapabankTrimTrimValid::from_bits(val as u8)
    }
    #[doc = "XTAL 32kHz capa bank trimmings"]
    #[inline(always)]
    pub const fn set_trim_valid(&mut self, val: super::vals::Xtal32khzCapabankTrimTrimValid) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_load_cap_iec_pf_x100(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_xtal_load_cap_iec_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn pcb_xin_para_cap_pf_x100(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_pcb_xin_para_cap_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[must_use]
    #[inline(always)]
    pub const fn pcb_xout_para_cap_pf_x100(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub const fn set_pcb_xout_para_cap_pf_x100(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for Xtal32khzCapabankTrim {
    #[inline(always)]
    fn default() -> Xtal32khzCapabankTrim {
        Xtal32khzCapabankTrim(0)
    }
}
impl core::fmt::Debug for Xtal32khzCapabankTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xtal32khzCapabankTrim")
            .field("trim_valid", &self.trim_valid())
            .field(
                "xtal_load_cap_iec_pf_x100",
                &self.xtal_load_cap_iec_pf_x100(),
            )
            .field("pcb_xin_para_cap_pf_x100", &self.pcb_xin_para_cap_pf_x100())
            .field(
                "pcb_xout_para_cap_pf_x100",
                &self.pcb_xout_para_cap_pf_x100(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xtal32khzCapabankTrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xtal32khzCapabankTrim {{ trim_valid: {:?}, xtal_load_cap_iec_pf_x100: {=u16:?}, pcb_xin_para_cap_pf_x100: {=u16:?}, pcb_xout_para_cap_pf_x100: {=u16:?} }}",
            self.trim_valid(),
            self.xtal_load_cap_iec_pf_x100(),
            self.pcb_xin_para_cap_pf_x100(),
            self.pcb_xout_para_cap_pf_x100()
        )
    }
}
