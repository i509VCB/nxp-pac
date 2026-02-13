#[doc = "Application defined context mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppCtxMask(pub u32);
impl AppCtxMask {
    #[doc = "Application defined context"]
    #[must_use]
    #[inline(always)]
    pub const fn app_ctx_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application defined context"]
    #[inline(always)]
    pub const fn set_app_ctx_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AppCtxMask {
    #[inline(always)]
    fn default() -> AppCtxMask {
        AppCtxMask(0)
    }
}
impl core::fmt::Debug for AppCtxMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AppCtxMask")
            .field("app_ctx_mask", &self.app_ctx_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AppCtxMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AppCtxMask {{ app_ctx_mask: {=u32:?} }}",
            self.app_ctx_mask()
        )
    }
}
#[doc = "PUF command blocking configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Disable PUF enroll command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_enroll(&self) -> super::vals::DisPufEnroll {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DisPufEnroll::from_bits(val as u8)
    }
    #[doc = "Disable PUF enroll command"]
    #[inline(always)]
    pub const fn set_dis_puf_enroll(&mut self, val: super::vals::DisPufEnroll) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable PUF start command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_start(&self) -> super::vals::DisPufStart {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DisPufStart::from_bits(val as u8)
    }
    #[doc = "Disable PUF start command"]
    #[inline(always)]
    pub const fn set_dis_puf_start(&mut self, val: super::vals::DisPufStart) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable PUF stop command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_stop(&self) -> super::vals::DisPufStop {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DisPufStop::from_bits(val as u8)
    }
    #[doc = "Disable PUF stop command"]
    #[inline(always)]
    pub const fn set_dis_puf_stop(&mut self, val: super::vals::DisPufStop) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable PUF get key command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_get_key(&self) -> super::vals::DisPufGetKey {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DisPufGetKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF get key command"]
    #[inline(always)]
    pub const fn set_dis_puf_get_key(&mut self, val: super::vals::DisPufGetKey) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Disable PUF unwrap key command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_unwrap_key(&self) -> super::vals::DisPufUnwrapKey {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DisPufUnwrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF unwrap key command"]
    #[inline(always)]
    pub const fn set_dis_puf_unwrap_key(&mut self, val: super::vals::DisPufUnwrapKey) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Disable PUF generate and wrap key command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_gen_wrap_key(&self) -> super::vals::DisPufGenWrapKey {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DisPufGenWrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF generate and wrap key command"]
    #[inline(always)]
    pub const fn set_dis_puf_gen_wrap_key(&mut self, val: super::vals::DisPufGenWrapKey) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Disable PUF wrap key command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_wrap_key(&self) -> super::vals::DisPufWrapKey {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::DisPufWrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF wrap key command"]
    #[inline(always)]
    pub const fn set_dis_puf_wrap_key(&mut self, val: super::vals::DisPufWrapKey) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Disable PUF generate and wrap key command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_gen_random_number(&self) -> super::vals::DisPufGenRandomNumber {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::DisPufGenRandomNumber::from_bits(val as u8)
    }
    #[doc = "Disable PUF generate and wrap key command"]
    #[inline(always)]
    pub const fn set_dis_puf_gen_random_number(&mut self, val: super::vals::DisPufGenRandomNumber) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Disable PUF test command"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_test(&self) -> super::vals::DisPufTest {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::DisPufTest::from_bits(val as u8)
    }
    #[doc = "Disable PUF test command"]
    #[inline(always)]
    pub const fn set_dis_puf_test(&mut self, val: super::vals::DisPufTest) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("dis_puf_enroll", &self.dis_puf_enroll())
            .field("dis_puf_start", &self.dis_puf_start())
            .field("dis_puf_stop", &self.dis_puf_stop())
            .field("dis_puf_get_key", &self.dis_puf_get_key())
            .field("dis_puf_unwrap_key", &self.dis_puf_unwrap_key())
            .field("dis_puf_gen_wrap_key", &self.dis_puf_gen_wrap_key())
            .field("dis_puf_wrap_key", &self.dis_puf_wrap_key())
            .field(
                "dis_puf_gen_random_number",
                &self.dis_puf_gen_random_number(),
            )
            .field("dis_puf_test", &self.dis_puf_test())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ dis_puf_enroll: {:?}, dis_puf_start: {:?}, dis_puf_stop: {:?}, dis_puf_get_key: {:?}, dis_puf_unwrap_key: {:?}, dis_puf_gen_wrap_key: {:?}, dis_puf_wrap_key: {:?}, dis_puf_gen_random_number: {:?}, dis_puf_test: {:?} }}",
            self.dis_puf_enroll(),
            self.dis_puf_start(),
            self.dis_puf_stop(),
            self.dis_puf_get_key(),
            self.dis_puf_unwrap_key(),
            self.dis_puf_gen_wrap_key(),
            self.dis_puf_wrap_key(),
            self.dis_puf_gen_random_number(),
            self.dis_puf_test()
        )
    }
}
#[doc = "Security level lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecLock(pub u32);
impl SecLock {
    #[doc = "Security Level"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_level(&self) -> super::vals::SecLevel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SecLevel::from_bits(val as u8)
    }
    #[doc = "Security Level"]
    #[inline(always)]
    pub const fn set_sec_level(&mut self, val: super::vals::SecLevel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Anti-pole of security level"]
    #[must_use]
    #[inline(always)]
    pub const fn anti_pole_sec_level(&self) -> super::vals::AntiPoleSecLevel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AntiPoleSecLevel::from_bits(val as u8)
    }
    #[doc = "Anti-pole of security level"]
    #[inline(always)]
    pub const fn set_anti_pole_sec_level(&mut self, val: super::vals::AntiPoleSecLevel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn pattern(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pattern"]
    #[inline(always)]
    pub const fn set_pattern(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for SecLock {
    #[inline(always)]
    fn default() -> SecLock {
        SecLock(0)
    }
}
impl core::fmt::Debug for SecLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecLock")
            .field("sec_level", &self.sec_level())
            .field("anti_pole_sec_level", &self.anti_pole_sec_level())
            .field("pattern", &self.pattern())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecLock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SecLock {{ sec_level: {:?}, anti_pole_sec_level: {:?}, pattern: {=u16:?} }}",
            self.sec_level(),
            self.anti_pole_sec_level(),
            self.pattern()
        )
    }
}
