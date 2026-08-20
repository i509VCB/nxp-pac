#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "PUF Key Context Management."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PufCtrl {
    ptr: *mut u8,
}
unsafe impl Send for PufCtrl {}
unsafe impl Sync for PufCtrl {}
impl PufCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PUF command blocking configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::pac::common::Reg<Config, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Security level lock."]
    #[inline(always)]
    pub const fn sec_lock(self) -> crate::pac::common::Reg<SecLock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Application defined context mask."]
    #[inline(always)]
    pub const fn app_ctx_mask(self) -> crate::pac::common::Reg<AppCtxMask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
}
#[doc = "Application defined context mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AppCtxMask(pub u32);
impl AppCtxMask {
    #[doc = "Application defined context."]
    #[must_use]
    #[inline(always)]
    pub const fn app_ctx_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application defined context."]
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
#[doc = "PUF command blocking configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "Disable PUF enroll command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_enroll(&self) -> DisPufEnroll {
        let val = (self.0 >> 1usize) & 0x01;
        DisPufEnroll::from_bits(val as u8)
    }
    #[doc = "Disable PUF enroll command."]
    #[inline(always)]
    pub const fn set_dis_puf_enroll(&mut self, val: DisPufEnroll) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable PUF start command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_start(&self) -> DisPufStart {
        let val = (self.0 >> 2usize) & 0x01;
        DisPufStart::from_bits(val as u8)
    }
    #[doc = "Disable PUF start command."]
    #[inline(always)]
    pub const fn set_dis_puf_start(&mut self, val: DisPufStart) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable PUF stop command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_stop(&self) -> DisPufStop {
        let val = (self.0 >> 5usize) & 0x01;
        DisPufStop::from_bits(val as u8)
    }
    #[doc = "Disable PUF stop command."]
    #[inline(always)]
    pub const fn set_dis_puf_stop(&mut self, val: DisPufStop) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable PUF get key command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_get_key(&self) -> DisPufGetKey {
        let val = (self.0 >> 6usize) & 0x01;
        DisPufGetKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF get key command."]
    #[inline(always)]
    pub const fn set_dis_puf_get_key(&mut self, val: DisPufGetKey) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Disable PUF unwrap key command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_unwrap_key(&self) -> DisPufUnwrapKey {
        let val = (self.0 >> 7usize) & 0x01;
        DisPufUnwrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF unwrap key command."]
    #[inline(always)]
    pub const fn set_dis_puf_unwrap_key(&mut self, val: DisPufUnwrapKey) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Disable PUF generate and wrap key command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_gen_wrap_key(&self) -> DisPufGenWrapKey {
        let val = (self.0 >> 8usize) & 0x01;
        DisPufGenWrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF generate and wrap key command."]
    #[inline(always)]
    pub const fn set_dis_puf_gen_wrap_key(&mut self, val: DisPufGenWrapKey) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Disable PUF wrap key command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_wrap_key(&self) -> DisPufWrapKey {
        let val = (self.0 >> 9usize) & 0x01;
        DisPufWrapKey::from_bits(val as u8)
    }
    #[doc = "Disable PUF wrap key command."]
    #[inline(always)]
    pub const fn set_dis_puf_wrap_key(&mut self, val: DisPufWrapKey) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Disable PUF generate and wrap key command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_gen_random_number(&self) -> DisPufGenRandomNumber {
        let val = (self.0 >> 15usize) & 0x01;
        DisPufGenRandomNumber::from_bits(val as u8)
    }
    #[doc = "Disable PUF generate and wrap key command."]
    #[inline(always)]
    pub const fn set_dis_puf_gen_random_number(&mut self, val: DisPufGenRandomNumber) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Disable PUF test command."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_puf_test(&self) -> DisPufTest {
        let val = (self.0 >> 31usize) & 0x01;
        DisPufTest::from_bits(val as u8)
    }
    #[doc = "Disable PUF test command."]
    #[inline(always)]
    pub const fn set_dis_puf_test(&mut self, val: DisPufTest) {
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
#[doc = "Security level lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecLock(pub u32);
impl SecLock {
    #[doc = "Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_level(&self) -> SecLevel {
        let val = (self.0 >> 0usize) & 0x03;
        SecLevel::from_bits(val as u8)
    }
    #[doc = "Security Level."]
    #[inline(always)]
    pub const fn set_sec_level(&mut self, val: SecLevel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Anti-pole of security level."]
    #[must_use]
    #[inline(always)]
    pub const fn anti_pole_sec_level(&self) -> AntiPoleSecLevel {
        let val = (self.0 >> 2usize) & 0x03;
        AntiPoleSecLevel::from_bits(val as u8)
    }
    #[doc = "Anti-pole of security level."]
    #[inline(always)]
    pub const fn set_anti_pole_sec_level(&mut self, val: AntiPoleSecLevel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn pattern(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Pattern."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AntiPoleSecLevel {
    #[doc = "Secure and privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Secure and non-privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Non-secure and privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Non-secure and non-privileged Master."]
    SecurePrivMaster = 0x03,
}
impl AntiPoleSecLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AntiPoleSecLevel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AntiPoleSecLevel {
    #[inline(always)]
    fn from(val: u8) -> AntiPoleSecLevel {
        AntiPoleSecLevel::from_bits(val)
    }
}
impl From<AntiPoleSecLevel> for u8 {
    #[inline(always)]
    fn from(val: AntiPoleSecLevel) -> u8 {
        AntiPoleSecLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufEnroll {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufEnroll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufEnroll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufEnroll {
    #[inline(always)]
    fn from(val: u8) -> DisPufEnroll {
        DisPufEnroll::from_bits(val)
    }
}
impl From<DisPufEnroll> for u8 {
    #[inline(always)]
    fn from(val: DisPufEnroll) -> u8 {
        DisPufEnroll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGenRandomNumber {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufGenRandomNumber {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGenRandomNumber {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGenRandomNumber {
    #[inline(always)]
    fn from(val: u8) -> DisPufGenRandomNumber {
        DisPufGenRandomNumber::from_bits(val)
    }
}
impl From<DisPufGenRandomNumber> for u8 {
    #[inline(always)]
    fn from(val: DisPufGenRandomNumber) -> u8 {
        DisPufGenRandomNumber::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGenWrapKey {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufGenWrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGenWrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGenWrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufGenWrapKey {
        DisPufGenWrapKey::from_bits(val)
    }
}
impl From<DisPufGenWrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufGenWrapKey) -> u8 {
        DisPufGenWrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufGetKey {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufGetKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufGetKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufGetKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufGetKey {
        DisPufGetKey::from_bits(val)
    }
}
impl From<DisPufGetKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufGetKey) -> u8 {
        DisPufGetKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufStart {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufStart {
    #[inline(always)]
    fn from(val: u8) -> DisPufStart {
        DisPufStart::from_bits(val)
    }
}
impl From<DisPufStart> for u8 {
    #[inline(always)]
    fn from(val: DisPufStart) -> u8 {
        DisPufStart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufStop {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufStop {
    #[inline(always)]
    fn from(val: u8) -> DisPufStop {
        DisPufStop::from_bits(val)
    }
}
impl From<DisPufStop> for u8 {
    #[inline(always)]
    fn from(val: DisPufStop) -> u8 {
        DisPufStop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufTest {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufTest {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufTest {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufTest {
    #[inline(always)]
    fn from(val: u8) -> DisPufTest {
        DisPufTest::from_bits(val)
    }
}
impl From<DisPufTest> for u8 {
    #[inline(always)]
    fn from(val: DisPufTest) -> u8 {
        DisPufTest::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufUnwrapKey {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufUnwrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufUnwrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufUnwrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufUnwrapKey {
        DisPufUnwrapKey::from_bits(val)
    }
}
impl From<DisPufUnwrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufUnwrapKey) -> u8 {
        DisPufUnwrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPufWrapKey {
    #[doc = "Command enabled."]
    Enable = 0x0,
    #[doc = "Command disabled."]
    Disable = 0x01,
}
impl DisPufWrapKey {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPufWrapKey {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPufWrapKey {
    #[inline(always)]
    fn from(val: u8) -> DisPufWrapKey {
        DisPufWrapKey::from_bits(val)
    }
}
impl From<DisPufWrapKey> for u8 {
    #[inline(always)]
    fn from(val: DisPufWrapKey) -> u8 {
        DisPufWrapKey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecLevel {
    #[doc = "Non-secure and non-privileged Master."]
    NonsecureNonprivMaster = 0x0,
    #[doc = "Non-secure and privileged Master."]
    NonsecurePrivMaster = 0x01,
    #[doc = "Secure and non-privileged Master."]
    SecureNonprivMaster = 0x02,
    #[doc = "Secure and privileged Master."]
    SecurePrivMaster = 0x03,
}
impl SecLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecLevel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecLevel {
    #[inline(always)]
    fn from(val: u8) -> SecLevel {
        SecLevel::from_bits(val)
    }
}
impl From<SecLevel> for u8 {
    #[inline(always)]
    fn from(val: SecLevel) -> u8 {
        SecLevel::to_bits(val)
    }
}
