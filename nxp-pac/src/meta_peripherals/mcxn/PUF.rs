#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "PUF."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Puf {
    ptr: *mut u8,
}
unsafe impl Send for Puf {}
unsafe impl Sync for Puf {}
impl Puf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn cr(self) -> crate::pac::common::Reg<Cr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Operation Result."]
    #[inline(always)]
    pub const fn orr(self) -> crate::pac::common::Reg<Orr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sr(self) -> crate::pac::common::Reg<Sr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Allow."]
    #[inline(always)]
    pub const fn ar(self) -> crate::pac::common::Reg<Ar, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn ier(self) -> crate::pac::common::Reg<Ier, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Mask."]
    #[inline(always)]
    pub const fn imr(self) -> crate::pac::common::Reg<Imr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn isr(self) -> crate::pac::common::Reg<Isr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Data Destination."]
    #[inline(always)]
    pub const fn data_dest(self) -> crate::pac::common::Reg<DataDest, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Data Source."]
    #[inline(always)]
    pub const fn data_src(self) -> crate::pac::common::Reg<DataSrc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data Input."]
    #[inline(always)]
    pub const fn dir(self) -> crate::pac::common::Reg<Dir, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Data Output."]
    #[inline(always)]
    pub const fn dor(self) -> crate::pac::common::Reg<Dor, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Miscellaneous."]
    #[inline(always)]
    pub const fn misc(self) -> crate::pac::common::Reg<Misc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Interface Status."]
    #[inline(always)]
    pub const fn if_sr(self) -> crate::pac::common::Reg<IfSr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "PUF Score."]
    #[inline(always)]
    pub const fn psr(self) -> crate::pac::common::Reg<Psr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Hardware Restrict User Context 0."]
    #[inline(always)]
    pub const fn hw_ruc0(self) -> crate::pac::common::Reg<HwRuc0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Hardware Restrict User Context 1."]
    #[inline(always)]
    pub const fn hw_ruc1(self) -> crate::pac::common::Reg<HwRuc1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Hardware Information."]
    #[inline(always)]
    pub const fn hw_info(self) -> crate::pac::common::Reg<HwInfo, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Hardware Identifier."]
    #[inline(always)]
    pub const fn hw_id(self) -> crate::pac::common::Reg<HwId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Hardware Version."]
    #[inline(always)]
    pub const fn hw_ver(self) -> crate::pac::common::Reg<HwVer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SRAM Configuration."]
    #[inline(always)]
    pub const fn sram_cfg(self) -> crate::pac::common::Reg<SramCfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn sram_status(self) -> crate::pac::common::Reg<SramStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Enable Clear."]
    #[inline(always)]
    pub const fn sram_int_clr_enable(
        self,
    ) -> crate::pac::common::Reg<SramIntClrEnable, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Interrupt Enable Set."]
    #[inline(always)]
    pub const fn sram_int_set_enable(
        self,
    ) -> crate::pac::common::Reg<SramIntSetEnable, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn sram_int_status(
        self,
    ) -> crate::pac::common::Reg<SramIntStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn sram_int_enable(
        self,
    ) -> crate::pac::common::Reg<SramIntEnable, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Interrupt Status Clear."]
    #[inline(always)]
    pub const fn sram_int_clr_status(
        self,
    ) -> crate::pac::common::Reg<SramIntClrStatus, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Interrupt Status set."]
    #[inline(always)]
    pub const fn sram_int_set_status(
        self,
    ) -> crate::pac::common::Reg<SramIntSetStatus, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
}
#[doc = "Allow."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ar(pub u32);
impl Ar {
    #[doc = "Enroll operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_enroll(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enroll operation."]
    #[inline(always)]
    pub const fn set_allow_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub const fn set_allow_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reconstruct operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_reconstruct(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reconstruct operation."]
    #[inline(always)]
    pub const fn set_allow_reconstruct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Stop operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_stop(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop operation."]
    #[inline(always)]
    pub const fn set_allow_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Get Key operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_get_key(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Get Key operation."]
    #[inline(always)]
    pub const fn set_allow_get_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Unwrap operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_unwrap(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Unwrap operation."]
    #[inline(always)]
    pub const fn set_allow_unwrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wrap Generated Random operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_wrap_generated_random(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap Generated Random operation."]
    #[inline(always)]
    pub const fn set_allow_wrap_generated_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Wrap operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_wrap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap operation."]
    #[inline(always)]
    pub const fn set_allow_wrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Generate Random operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_generate_random(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Generate Random operation."]
    #[inline(always)]
    pub const fn set_allow_generate_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_test_memory(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_allow_test_memory(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Test PUF operation."]
    #[must_use]
    #[inline(always)]
    pub const fn allow_test_puf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Test PUF operation."]
    #[inline(always)]
    pub const fn set_allow_test_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ar {
    #[inline(always)]
    fn default() -> Ar {
        Ar(0)
    }
}
impl core::fmt::Debug for Ar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ar")
            .field("allow_enroll", &self.allow_enroll())
            .field("allow_start", &self.allow_start())
            .field("allow_reconstruct", &self.allow_reconstruct())
            .field("allow_stop", &self.allow_stop())
            .field("allow_get_key", &self.allow_get_key())
            .field("allow_unwrap", &self.allow_unwrap())
            .field(
                "allow_wrap_generated_random",
                &self.allow_wrap_generated_random(),
            )
            .field("allow_wrap", &self.allow_wrap())
            .field("allow_generate_random", &self.allow_generate_random())
            .field("allow_test_memory", &self.allow_test_memory())
            .field("allow_test_puf", &self.allow_test_puf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ar {{ allow_enroll: {=bool:?}, allow_start: {=bool:?}, allow_reconstruct: {=bool:?}, allow_stop: {=bool:?}, allow_get_key: {=bool:?}, allow_unwrap: {=bool:?}, allow_wrap_generated_random: {=bool:?}, allow_wrap: {=bool:?}, allow_generate_random: {=bool:?}, allow_test_memory: {=bool:?}, allow_test_puf: {=bool:?} }}",
            self.allow_enroll(),
            self.allow_start(),
            self.allow_reconstruct(),
            self.allow_stop(),
            self.allow_get_key(),
            self.allow_unwrap(),
            self.allow_wrap_generated_random(),
            self.allow_wrap(),
            self.allow_generate_random(),
            self.allow_test_memory(),
            self.allow_test_puf()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Zeroize operation."]
    #[must_use]
    #[inline(always)]
    pub const fn zeroize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroize operation."]
    #[inline(always)]
    pub const fn set_zeroize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enroll operation."]
    #[must_use]
    #[inline(always)]
    pub const fn enroll(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enroll operation."]
    #[inline(always)]
    pub const fn set_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start operation."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start operation."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reconstruct operation."]
    #[must_use]
    #[inline(always)]
    pub const fn reconstruct(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reconstruct operation."]
    #[inline(always)]
    pub const fn set_reconstruct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Stop operation."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop operation."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Get Key operation."]
    #[must_use]
    #[inline(always)]
    pub const fn get_key(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Get Key operation."]
    #[inline(always)]
    pub const fn set_get_key(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Unwrap operation."]
    #[must_use]
    #[inline(always)]
    pub const fn unwrap(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Unwrap operation."]
    #[inline(always)]
    pub const fn set_unwrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wrap Generated Random operation."]
    #[must_use]
    #[inline(always)]
    pub const fn wrap_generated_random(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap Generated Random operation."]
    #[inline(always)]
    pub const fn set_wrap_generated_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Wrap operation."]
    #[must_use]
    #[inline(always)]
    pub const fn wrap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap operation."]
    #[inline(always)]
    pub const fn set_wrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Generate Random operation."]
    #[must_use]
    #[inline(always)]
    pub const fn generate_random(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Generate Random operation."]
    #[inline(always)]
    pub const fn set_generate_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Test memory operation."]
    #[must_use]
    #[inline(always)]
    pub const fn test_memory(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Test memory operation."]
    #[inline(always)]
    pub const fn set_test_memory(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Test PUF operation."]
    #[must_use]
    #[inline(always)]
    pub const fn test_puf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Test PUF operation."]
    #[inline(always)]
    pub const fn set_test_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("zeroize", &self.zeroize())
            .field("enroll", &self.enroll())
            .field("start", &self.start())
            .field("reconstruct", &self.reconstruct())
            .field("stop", &self.stop())
            .field("get_key", &self.get_key())
            .field("unwrap", &self.unwrap())
            .field("wrap_generated_random", &self.wrap_generated_random())
            .field("wrap", &self.wrap())
            .field("generate_random", &self.generate_random())
            .field("test_memory", &self.test_memory())
            .field("test_puf", &self.test_puf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ zeroize: {=bool:?}, enroll: {=bool:?}, start: {=bool:?}, reconstruct: {=bool:?}, stop: {=bool:?}, get_key: {=bool:?}, unwrap: {=bool:?}, wrap_generated_random: {=bool:?}, wrap: {=bool:?}, generate_random: {=bool:?}, test_memory: {=bool:?}, test_puf: {=bool:?} }}",
            self.zeroize(),
            self.enroll(),
            self.start(),
            self.reconstruct(),
            self.stop(),
            self.get_key(),
            self.unwrap(),
            self.wrap_generated_random(),
            self.wrap(),
            self.generate_random(),
            self.test_memory(),
            self.test_puf()
        )
    }
}
#[doc = "Data Destination."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataDest(pub u32);
impl DataDest {
    #[doc = "Key available via the DOR register."]
    #[must_use]
    #[inline(always)]
    pub const fn dest_dor(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Key available via the DOR register."]
    #[inline(always)]
    pub const fn set_dest_dor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Key available to ELS."]
    #[must_use]
    #[inline(always)]
    pub const fn dest_so(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Key available to ELS."]
    #[inline(always)]
    pub const fn set_dest_so(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for DataDest {
    #[inline(always)]
    fn default() -> DataDest {
        DataDest(0)
    }
}
impl core::fmt::Debug for DataDest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DataDest")
            .field("dest_dor", &self.dest_dor())
            .field("dest_so", &self.dest_so())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DataDest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DataDest {{ dest_dor: {=bool:?}, dest_so: {=bool:?} }}",
            self.dest_dor(),
            self.dest_so()
        )
    }
}
#[doc = "Data Source."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataSrc(pub u32);
impl DataSrc {
    #[doc = "Data provided via the DIR register."]
    #[must_use]
    #[inline(always)]
    pub const fn src_dir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Data provided via the DIR register."]
    #[inline(always)]
    pub const fn set_src_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data provided via the SI interface."]
    #[must_use]
    #[inline(always)]
    pub const fn src_si(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data provided via the SI interface."]
    #[inline(always)]
    pub const fn set_src_si(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for DataSrc {
    #[inline(always)]
    fn default() -> DataSrc {
        DataSrc(0)
    }
}
impl core::fmt::Debug for DataSrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DataSrc")
            .field("src_dir", &self.src_dir())
            .field("src_si", &self.src_si())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DataSrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DataSrc {{ src_dir: {=bool:?}, src_si: {=bool:?} }}",
            self.src_dir(),
            self.src_si()
        )
    }
}
#[doc = "Data Input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc = "Input data."]
    #[must_use]
    #[inline(always)]
    pub const fn di(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input data."]
    #[inline(always)]
    pub const fn set_di(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dir {
    #[inline(always)]
    fn default() -> Dir {
        Dir(0)
    }
}
impl core::fmt::Debug for Dir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dir").field("di", &self.di()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dir {{ di: {=u32:?} }}", self.di())
    }
}
#[doc = "Data Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dor(pub u32);
impl Dor {
    #[doc = "Output data."]
    #[must_use]
    #[inline(always)]
    pub const fn do_(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output data."]
    #[inline(always)]
    pub const fn set_do_(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dor {
    #[inline(always)]
    fn default() -> Dor {
        Dor(0)
    }
}
impl core::fmt::Debug for Dor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dor").field("do_", &self.do_()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dor {{ do_: {=u32:?} }}", self.do_())
    }
}
#[doc = "Hardware Identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwId(pub u32);
impl HwId {
    #[doc = "Provides the hardware identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Provides the hardware identifier."]
    #[inline(always)]
    pub const fn set_hw_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwId {
    #[inline(always)]
    fn default() -> HwId {
        HwId(0)
    }
}
impl core::fmt::Debug for HwId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwId")
            .field("hw_id", &self.hw_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwId {{ hw_id: {=u32:?} }}", self.hw_id())
    }
}
#[doc = "Hardware Information."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwInfo(pub u32);
impl HwInfo {
    #[doc = "Wrap configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn config_wrap(&self) -> ConfigWrap {
        let val = (self.0 >> 24usize) & 0x01;
        ConfigWrap::from_bits(val as u8)
    }
    #[doc = "Wrap configuration."]
    #[inline(always)]
    pub const fn set_config_wrap(&mut self, val: ConfigWrap) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "PUF configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn config_type(&self) -> ConfigType {
        let val = (self.0 >> 28usize) & 0x0f;
        ConfigType::from_bits(val as u8)
    }
    #[doc = "PUF configuration."]
    #[inline(always)]
    pub const fn set_config_type(&mut self, val: ConfigType) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for HwInfo {
    #[inline(always)]
    fn default() -> HwInfo {
        HwInfo(0)
    }
}
impl core::fmt::Debug for HwInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwInfo")
            .field("config_wrap", &self.config_wrap())
            .field("config_type", &self.config_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwInfo {{ config_wrap: {:?}, config_type: {:?} }}",
            self.config_wrap(),
            self.config_type()
        )
    }
}
#[doc = "Hardware Restrict User Context 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwRuc0(pub u32);
impl HwRuc0 {
    #[doc = "Life cycle state based restrictions."]
    #[must_use]
    #[inline(always)]
    pub const fn lc_state(&self) -> LcState {
        let val = (self.0 >> 0usize) & 0xff;
        LcState::from_bits(val as u8)
    }
    #[doc = "Life cycle state based restrictions."]
    #[inline(always)]
    pub const fn set_lc_state(&mut self, val: LcState) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Temporal boot state."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_state(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Temporal boot state."]
    #[inline(always)]
    pub const fn set_boot_state(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
    #[doc = "Disable key access when debugger is attached to CPU0 after power-up."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_debug(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Disable key access when debugger is attached to CPU0 after power-up."]
    #[inline(always)]
    pub const fn set_cpu0_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Disable key access when debugger is attached to COOLFLUX after power-up."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_debug(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Disable key access when debugger is attached to COOLFLUX after power-up."]
    #[inline(always)]
    pub const fn set_coolflux_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "DSP debug status."]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_debug(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "DSP debug status."]
    #[inline(always)]
    pub const fn set_dsp_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Restrict the key access based on TrustZone security level."]
    #[must_use]
    #[inline(always)]
    pub const fn access_level(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Restrict the key access based on TrustZone security level."]
    #[inline(always)]
    pub const fn set_access_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for HwRuc0 {
    #[inline(always)]
    fn default() -> HwRuc0 {
        HwRuc0(0)
    }
}
impl core::fmt::Debug for HwRuc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwRuc0")
            .field("lc_state", &self.lc_state())
            .field("boot_state", &self.boot_state())
            .field("cpu0_debug", &self.cpu0_debug())
            .field("coolflux_debug", &self.coolflux_debug())
            .field("dsp_debug", &self.dsp_debug())
            .field("access_level", &self.access_level())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwRuc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwRuc0 {{ lc_state: {:?}, boot_state: {=u16:?}, cpu0_debug: {=bool:?}, coolflux_debug: {=bool:?}, dsp_debug: {=bool:?}, access_level: {=u8:?} }}",
            self.lc_state(),
            self.boot_state(),
            self.cpu0_debug(),
            self.coolflux_debug(),
            self.dsp_debug(),
            self.access_level()
        )
    }
}
#[doc = "Hardware Restrict User Context 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwRuc1(pub u32);
impl HwRuc1 {
    #[doc = "Application customizable context."]
    #[must_use]
    #[inline(always)]
    pub const fn app_ctx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application customizable context."]
    #[inline(always)]
    pub const fn set_app_ctx(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for HwRuc1 {
    #[inline(always)]
    fn default() -> HwRuc1 {
        HwRuc1(0)
    }
}
impl core::fmt::Debug for HwRuc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwRuc1")
            .field("app_ctx", &self.app_ctx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwRuc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HwRuc1 {{ app_ctx: {=u32:?} }}", self.app_ctx())
    }
}
#[doc = "Hardware Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HwVer(pub u32);
impl HwVer {
    #[doc = "Provides the hardware version, patch part."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Provides the hardware version, patch part."]
    #[inline(always)]
    pub const fn set_hw_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Provides the hardware version, minor part."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_version_minor(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Provides the hardware version, minor part."]
    #[inline(always)]
    pub const fn set_hw_version_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Provides the hardware version, major part."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_version_major(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Provides the hardware version, major part."]
    #[inline(always)]
    pub const fn set_hw_version_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for HwVer {
    #[inline(always)]
    fn default() -> HwVer {
        HwVer(0)
    }
}
impl core::fmt::Debug for HwVer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HwVer")
            .field("hw_rev", &self.hw_rev())
            .field("hw_version_minor", &self.hw_version_minor())
            .field("hw_version_major", &self.hw_version_major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HwVer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HwVer {{ hw_rev: {=u8:?}, hw_version_minor: {=u8:?}, hw_version_major: {=u8:?} }}",
            self.hw_rev(),
            self.hw_version_minor(),
            self.hw_version_major()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("int_en", &self.int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ier {{ int_en: {=bool:?} }}", self.int_en())
    }
}
#[doc = "Interface Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IfSr(pub u32);
impl IfSr {
    #[doc = "APB error."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "APB error."]
    #[inline(always)]
    pub const fn set_apb_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IfSr {
    #[inline(always)]
    fn default() -> IfSr {
        IfSr(0)
    }
}
impl core::fmt::Debug for IfSr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IfSr")
            .field("apb_error", &self.apb_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IfSr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IfSr {{ apb_error: {=bool:?} }}", self.apb_error())
    }
}
#[doc = "Interrupt Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Busy interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy interrupt."]
    #[inline(always)]
    pub const fn set_int_en_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ok interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_ok(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ok interrupt."]
    #[inline(always)]
    pub const fn set_int_en_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error interrupt."]
    #[inline(always)]
    pub const fn set_int_en_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Zeroized interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_zeroized(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroized interrupt."]
    #[inline(always)]
    pub const fn set_int_en_zeroized(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rejected interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_rejected(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rejected interrupt."]
    #[inline(always)]
    pub const fn set_int_en_rejected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Data in request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_di_request(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Data in request interrupt."]
    #[inline(always)]
    pub const fn set_int_en_di_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Data out request interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en_do_request(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Data out request interrupt."]
    #[inline(always)]
    pub const fn set_int_en_do_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("int_en_busy", &self.int_en_busy())
            .field("int_en_ok", &self.int_en_ok())
            .field("int_en_error", &self.int_en_error())
            .field("int_en_zeroized", &self.int_en_zeroized())
            .field("int_en_rejected", &self.int_en_rejected())
            .field("int_en_di_request", &self.int_en_di_request())
            .field("int_en_do_request", &self.int_en_do_request())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ int_en_busy: {=bool:?}, int_en_ok: {=bool:?}, int_en_error: {=bool:?}, int_en_zeroized: {=bool:?}, int_en_rejected: {=bool:?}, int_en_di_request: {=bool:?}, int_en_do_request: {=bool:?} }}",
            self.int_en_busy(),
            self.int_en_ok(),
            self.int_en_error(),
            self.int_en_zeroized(),
            self.int_en_rejected(),
            self.int_en_di_request(),
            self.int_en_do_request()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Negative edge occurred on Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn int_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Negative edge occurred on Busy."]
    #[inline(always)]
    pub const fn set_int_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Positive edge occurred on Ok."]
    #[must_use]
    #[inline(always)]
    pub const fn int_ok(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on Ok."]
    #[inline(always)]
    pub const fn set_int_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Positive edge occurred on Error."]
    #[must_use]
    #[inline(always)]
    pub const fn int_error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on Error."]
    #[inline(always)]
    pub const fn set_int_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Positive edge occurred on Zeroized."]
    #[must_use]
    #[inline(always)]
    pub const fn int_zeroized(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on Zeroized."]
    #[inline(always)]
    pub const fn set_int_zeroized(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Positive edge occurred on Rejected."]
    #[must_use]
    #[inline(always)]
    pub const fn int_rejected(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on Rejected."]
    #[inline(always)]
    pub const fn set_int_rejected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Positive edge occurred on di_request."]
    #[must_use]
    #[inline(always)]
    pub const fn int_di_request(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on di_request."]
    #[inline(always)]
    pub const fn set_int_di_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Positive edge occurred on do_request."]
    #[must_use]
    #[inline(always)]
    pub const fn int_do_request(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Positive edge occurred on do_request."]
    #[inline(always)]
    pub const fn set_int_do_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("int_busy", &self.int_busy())
            .field("int_ok", &self.int_ok())
            .field("int_error", &self.int_error())
            .field("int_zeroized", &self.int_zeroized())
            .field("int_rejected", &self.int_rejected())
            .field("int_di_request", &self.int_di_request())
            .field("int_do_request", &self.int_do_request())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ int_busy: {=bool:?}, int_ok: {=bool:?}, int_error: {=bool:?}, int_zeroized: {=bool:?}, int_rejected: {=bool:?}, int_di_request: {=bool:?}, int_do_request: {=bool:?} }}",
            self.int_busy(),
            self.int_ok(),
            self.int_error(),
            self.int_zeroized(),
            self.int_rejected(),
            self.int_di_request(),
            self.int_do_request()
        )
    }
}
#[doc = "Miscellaneous."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Defines the endianness of data in DIR and DOR:."]
    #[must_use]
    #[inline(always)]
    pub const fn data_endianness(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Defines the endianness of data in DIR and DOR:."]
    #[inline(always)]
    pub const fn set_data_endianness(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field("data_endianness", &self.data_endianness())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misc {{ data_endianness: {=bool:?} }}",
            self.data_endianness()
        )
    }
}
#[doc = "Operation Result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Orr(pub u32);
impl Orr {
    #[doc = "Result code of last operation."]
    #[must_use]
    #[inline(always)]
    pub const fn result_code(&self) -> ResultCode {
        let val = (self.0 >> 0usize) & 0xff;
        ResultCode::from_bits(val as u8)
    }
    #[doc = "Result code of last operation."]
    #[inline(always)]
    pub const fn set_result_code(&mut self, val: ResultCode) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Last operation type."]
    #[must_use]
    #[inline(always)]
    pub const fn last_operation(&self) -> LastOperation {
        let val = (self.0 >> 24usize) & 0xff;
        LastOperation::from_bits(val as u8)
    }
    #[doc = "Last operation type."]
    #[inline(always)]
    pub const fn set_last_operation(&mut self, val: LastOperation) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Orr {
    #[inline(always)]
    fn default() -> Orr {
        Orr(0)
    }
}
impl core::fmt::Debug for Orr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Orr")
            .field("result_code", &self.result_code())
            .field("last_operation", &self.last_operation())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Orr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Orr {{ result_code: {:?}, last_operation: {:?} }}",
            self.result_code(),
            self.last_operation()
        )
    }
}
#[doc = "PUF Score."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Provides the PUF score obtained during the last Test PUF, Enroll or Start operation."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_score(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Provides the PUF score obtained during the last Test PUF, Enroll or Start operation."]
    #[inline(always)]
    pub const fn set_puf_score(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("puf_score", &self.puf_score())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Psr {{ puf_score: {=u8:?} }}", self.puf_score())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Operation in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operation in progress."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Last operation successful."]
    #[must_use]
    #[inline(always)]
    pub const fn ok(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Last operation successful."]
    #[inline(always)]
    pub const fn set_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Last operation failed."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Last operation failed."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Zeroized or Locked state."]
    #[must_use]
    #[inline(always)]
    pub const fn zeroized(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroized or Locked state."]
    #[inline(always)]
    pub const fn set_zeroized(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Operation rejected."]
    #[must_use]
    #[inline(always)]
    pub const fn rejected(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Operation rejected."]
    #[inline(always)]
    pub const fn set_rejected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the request for data in transfer via the DIR register."]
    #[must_use]
    #[inline(always)]
    pub const fn di_request(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the request for data in transfer via the DIR register."]
    #[inline(always)]
    pub const fn set_di_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the request for data out transfer via the DOR register."]
    #[must_use]
    #[inline(always)]
    pub const fn do_request(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the request for data out transfer via the DOR register."]
    #[inline(always)]
    pub const fn set_do_request(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("busy", &self.busy())
            .field("ok", &self.ok())
            .field("error", &self.error())
            .field("zeroized", &self.zeroized())
            .field("rejected", &self.rejected())
            .field("di_request", &self.di_request())
            .field("do_request", &self.do_request())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ busy: {=bool:?}, ok: {=bool:?}, error: {=bool:?}, zeroized: {=bool:?}, rejected: {=bool:?}, di_request: {=bool:?}, do_request: {=bool:?} }}",
            self.busy(),
            self.ok(),
            self.error(),
            self.zeroized(),
            self.rejected(),
            self.di_request(),
            self.do_request()
        )
    }
}
#[doc = "SRAM Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramCfg(pub u32);
impl SramCfg {
    #[doc = "PUF SRAM Controller activation."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller activation."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF SRAM Clock Gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ckgating(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Clock Gating control."]
    #[inline(always)]
    pub const fn set_ckgating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SramCfg {
    #[inline(always)]
    fn default() -> SramCfg {
        SramCfg(0)
    }
}
impl core::fmt::Debug for SramCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramCfg")
            .field("enable", &self.enable())
            .field("ckgating", &self.ckgating())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramCfg {{ enable: {=bool:?}, ckgating: {=bool:?} }}",
            self.enable(),
            self.ckgating()
        )
    }
}
#[doc = "Interrupt Enable Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntClrEnable(pub u32);
impl SramIntClrEnable {
    #[doc = "READY Interrupt Enable clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable clear."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable clear."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable clear."]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntClrEnable {
    #[inline(always)]
    fn default() -> SramIntClrEnable {
        SramIntClrEnable(0)
    }
}
impl core::fmt::Debug for SramIntClrEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntClrEnable")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntClrEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntClrEnable {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntClrStatus(pub u32);
impl SramIntClrStatus {
    #[doc = "READY Interrupt Status clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status clear."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> SramIntClrStatusApbErr {
        let val = (self.0 >> 1usize) & 0x01;
        SramIntClrStatusApbErr::from_bits(val as u8)
    }
    #[doc = "APB_ERR Interrupt Status Clear."]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: SramIntClrStatusApbErr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntClrStatus {
    #[inline(always)]
    fn default() -> SramIntClrStatus {
        SramIntClrStatus(0)
    }
}
impl core::fmt::Debug for SramIntClrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntClrStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntClrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntClrStatus {{ ready: {=bool:?}, apb_err: {:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntEnable(pub u32);
impl SramIntEnable {
    #[doc = "READY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sram_apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sram_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntEnable {
    #[inline(always)]
    fn default() -> SramIntEnable {
        SramIntEnable(0)
    }
}
impl core::fmt::Debug for SramIntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntEnable")
            .field("ready", &self.ready())
            .field("sram_apb_err", &self.sram_apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntEnable {{ ready: {=bool:?}, sram_apb_err: {=bool:?} }}",
            self.ready(),
            self.sram_apb_err()
        )
    }
}
#[doc = "Interrupt Enable Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntSetEnable(pub u32);
impl SramIntSetEnable {
    #[doc = "READY Interrupt Enable set."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Enable set."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Enable set."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Enable set."]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntSetEnable {
    #[inline(always)]
    fn default() -> SramIntSetEnable {
        SramIntSetEnable(0)
    }
}
impl core::fmt::Debug for SramIntSetEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntSetEnable")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntSetEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntSetEnable {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntSetStatus(pub u32);
impl SramIntSetStatus {
    #[doc = "READY Interrupt Status set."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status set."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status Set."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> SramIntSetStatusApbErr {
        let val = (self.0 >> 1usize) & 0x01;
        SramIntSetStatusApbErr::from_bits(val as u8)
    }
    #[doc = "APB_ERR Interrupt Status Set."]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: SramIntSetStatusApbErr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntSetStatus {
    #[inline(always)]
    fn default() -> SramIntSetStatus {
        SramIntSetStatus(0)
    }
}
impl core::fmt::Debug for SramIntSetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntSetStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntSetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntSetStatus {{ ready: {=bool:?}, apb_err: {:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramIntStatus(pub u32);
impl SramIntStatus {
    #[doc = "READY Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "READY Interrupt Status."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB_ERR Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn apb_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB_ERR Interrupt Status."]
    #[inline(always)]
    pub const fn set_apb_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SramIntStatus {
    #[inline(always)]
    fn default() -> SramIntStatus {
        SramIntStatus(0)
    }
}
impl core::fmt::Debug for SramIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramIntStatus")
            .field("ready", &self.ready())
            .field("apb_err", &self.apb_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramIntStatus {{ ready: {=bool:?}, apb_err: {=bool:?} }}",
            self.ready(),
            self.apb_err()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramStatus(pub u32);
impl SramStatus {
    #[doc = "PUF SRAM Controller State."]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller State."]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SramStatus {
    #[inline(always)]
    fn default() -> SramStatus {
        SramStatus(0)
    }
}
impl core::fmt::Debug for SramStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramStatus")
            .field("ready", &self.ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SramStatus {{ ready: {=bool:?} }}", self.ready())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigType {
    _RESERVED_0 = 0x0,
    #[doc = "Indicates that PUF configuration is Safe."]
    Safe = 0x01,
    #[doc = "Indicates that PUF configuration is Plus."]
    Plus = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl ConfigType {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigType {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigType {
    #[inline(always)]
    fn from(val: u8) -> ConfigType {
        ConfigType::from_bits(val)
    }
}
impl From<ConfigType> for u8 {
    #[inline(always)]
    fn from(val: ConfigType) -> u8 {
        ConfigType::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigWrap {
    #[doc = "Indicates that Wrap is not included."]
    Enable = 0x0,
    #[doc = "Indicates that Wrap is included."]
    Disable = 0x01,
}
impl ConfigWrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ConfigWrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ConfigWrap {
    #[inline(always)]
    fn from(val: u8) -> ConfigWrap {
        ConfigWrap::from_bits(val)
    }
}
impl From<ConfigWrap> for u8 {
    #[inline(always)]
    fn from(val: ConfigWrap) -> u8 {
        ConfigWrap::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LastOperation(u8);
impl LastOperation {
    #[doc = "Indicates that the operation is in progress."]
    pub const LoProgress: Self = Self(0x0);
    #[doc = "Indicates that the last operation was Enroll."]
    pub const LoEnroll: Self = Self(0x01);
    #[doc = "Indicates that the last operation was Start."]
    pub const LoStart: Self = Self(0x02);
    #[doc = "Indicates that the last operation was Reconstruct."]
    pub const LoReconstruct: Self = Self(0x03);
    #[doc = "Indicates that the last operation was Stop."]
    pub const LoStop: Self = Self(0x05);
    #[doc = "Indicates that the last operation was Get Key."]
    pub const LoGetKey: Self = Self(0x06);
    #[doc = "Indicates that the last operation was Unwrap."]
    pub const LoUnwrap: Self = Self(0x07);
    #[doc = "Indicates that the last operation was Wrap Generated Random."]
    pub const LoWrapGenRnd: Self = Self(0x08);
    #[doc = "Indicates that the last operation was Wrap."]
    pub const LoWrap: Self = Self(0x09);
    #[doc = "Indicates that the last operation was Generate Random."]
    pub const LoGenRnd: Self = Self(0x0f);
    #[doc = "Indicates that the last operation was Test Memory."]
    pub const LoTestMemory: Self = Self(0x1e);
    #[doc = "Indicates that the last operation was Test PUF."]
    pub const LoTestPuf: Self = Self(0x1f);
    #[doc = "Indicates that the last operation was Initialization."]
    pub const LoInitialization: Self = Self(0x20);
    #[doc = "Indicates that the last operation was Zeroize."]
    pub const LoZeroize: Self = Self(0x2f);
}
impl LastOperation {
    pub const fn from_bits(val: u8) -> LastOperation {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LastOperation {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("LoProgress"),
            0x01 => f.write_str("LoEnroll"),
            0x02 => f.write_str("LoStart"),
            0x03 => f.write_str("LoReconstruct"),
            0x05 => f.write_str("LoStop"),
            0x06 => f.write_str("LoGetKey"),
            0x07 => f.write_str("LoUnwrap"),
            0x08 => f.write_str("LoWrapGenRnd"),
            0x09 => f.write_str("LoWrap"),
            0x0f => f.write_str("LoGenRnd"),
            0x1e => f.write_str("LoTestMemory"),
            0x1f => f.write_str("LoTestPuf"),
            0x20 => f.write_str("LoInitialization"),
            0x2f => f.write_str("LoZeroize"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LastOperation {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "LoProgress"),
            0x01 => defmt::write!(f, "LoEnroll"),
            0x02 => defmt::write!(f, "LoStart"),
            0x03 => defmt::write!(f, "LoReconstruct"),
            0x05 => defmt::write!(f, "LoStop"),
            0x06 => defmt::write!(f, "LoGetKey"),
            0x07 => defmt::write!(f, "LoUnwrap"),
            0x08 => defmt::write!(f, "LoWrapGenRnd"),
            0x09 => defmt::write!(f, "LoWrap"),
            0x0f => defmt::write!(f, "LoGenRnd"),
            0x1e => defmt::write!(f, "LoTestMemory"),
            0x1f => defmt::write!(f, "LoTestPuf"),
            0x20 => defmt::write!(f, "LoInitialization"),
            0x2f => defmt::write!(f, "LoZeroize"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LastOperation {
    #[inline(always)]
    fn from(val: u8) -> LastOperation {
        LastOperation::from_bits(val)
    }
}
impl From<LastOperation> for u8 {
    #[inline(always)]
    fn from(val: LastOperation) -> u8 {
        LastOperation::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct LcState(u8);
impl LcState {
    #[doc = "OEM Develop."]
    pub const OemOpen: Self = Self(0x03);
    #[doc = "OEM Develop 2."]
    pub const OemSecureWorld: Self = Self(0x07);
    #[doc = "OEM In-field."]
    pub const OemClosed: Self = Self(0x0f);
    #[doc = "OEM Field return."]
    pub const OemFieldReturn: Self = Self(0x1f);
    #[doc = "NXP Field Return/Failure Analysis."]
    pub const FieldReturnNxp: Self = Self(0x3f);
    #[doc = "In-field Locked."]
    pub const OemLocked: Self = Self(0xcf);
    #[doc = "Bricked."]
    pub const OemShredded: Self = Self(0xff);
}
impl LcState {
    pub const fn from_bits(val: u8) -> LcState {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for LcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x03 => f.write_str("OemOpen"),
            0x07 => f.write_str("OemSecureWorld"),
            0x0f => f.write_str("OemClosed"),
            0x1f => f.write_str("OemFieldReturn"),
            0x3f => f.write_str("FieldReturnNxp"),
            0xcf => f.write_str("OemLocked"),
            0xff => f.write_str("OemShredded"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LcState {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x03 => defmt::write!(f, "OemOpen"),
            0x07 => defmt::write!(f, "OemSecureWorld"),
            0x0f => defmt::write!(f, "OemClosed"),
            0x1f => defmt::write!(f, "OemFieldReturn"),
            0x3f => defmt::write!(f, "FieldReturnNxp"),
            0xcf => defmt::write!(f, "OemLocked"),
            0xff => defmt::write!(f, "OemShredded"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for LcState {
    #[inline(always)]
    fn from(val: u8) -> LcState {
        LcState::from_bits(val)
    }
}
impl From<LcState> for u8 {
    #[inline(always)]
    fn from(val: LcState) -> u8 {
        LcState::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResultCode(u8);
impl ResultCode {
    #[doc = "Indicates that the last operation was successful or operation is in progress."]
    pub const Ok: Self = Self(0x0);
    #[doc = "Indicates that the AC is not for the current product/version."]
    pub const ErrProduct: Self = Self(0xf0);
    #[doc = "Indicates that the AC in the second phase is not for the current product/version."]
    pub const ErrProductPh2: Self = Self(0xf1);
    #[doc = "Indicates that the AC is corrupted."]
    pub const ErrTransfer: Self = Self(0xf2);
    #[doc = "Indicates that the AC in the second phase is corrupted."]
    pub const ErrTransferPh2: Self = Self(0xf3);
    #[doc = "Indicates that the authentication of the provided AC failed."]
    pub const ErrAuth: Self = Self(0xf4);
    #[doc = "Indicates that the authentication of the provided AC failed in the second phase."]
    pub const ErrAuthPh2: Self = Self(0xf5);
    #[doc = "Indicates that the SRAM PUF quality verification fails."]
    pub const ErrPufQuality: Self = Self(0xf6);
    #[doc = "Indicates that the incorrect or unsupported context is provided."]
    pub const ErrContext: Self = Self(0xf7);
    #[doc = "Indicates that a data destination was set that is not allowed according to other settings and the current PUF state."]
    pub const ErrDestination: Self = Self(0xf8);
    #[doc = "Indicates that the PUF SRAM access has failed."]
    pub const Failure: Self = Self(0xff);
}
impl ResultCode {
    pub const fn from_bits(val: u8) -> ResultCode {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ResultCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Ok"),
            0xf0 => f.write_str("ErrProduct"),
            0xf1 => f.write_str("ErrProductPh2"),
            0xf2 => f.write_str("ErrTransfer"),
            0xf3 => f.write_str("ErrTransferPh2"),
            0xf4 => f.write_str("ErrAuth"),
            0xf5 => f.write_str("ErrAuthPh2"),
            0xf6 => f.write_str("ErrPufQuality"),
            0xf7 => f.write_str("ErrContext"),
            0xf8 => f.write_str("ErrDestination"),
            0xff => f.write_str("Failure"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResultCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ok"),
            0xf0 => defmt::write!(f, "ErrProduct"),
            0xf1 => defmt::write!(f, "ErrProductPh2"),
            0xf2 => defmt::write!(f, "ErrTransfer"),
            0xf3 => defmt::write!(f, "ErrTransferPh2"),
            0xf4 => defmt::write!(f, "ErrAuth"),
            0xf5 => defmt::write!(f, "ErrAuthPh2"),
            0xf6 => defmt::write!(f, "ErrPufQuality"),
            0xf7 => defmt::write!(f, "ErrContext"),
            0xf8 => defmt::write!(f, "ErrDestination"),
            0xff => defmt::write!(f, "Failure"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ResultCode {
    #[inline(always)]
    fn from(val: u8) -> ResultCode {
        ResultCode::from_bits(val)
    }
}
impl From<ResultCode> for u8 {
    #[inline(always)]
    fn from(val: ResultCode) -> u8 {
        ResultCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIntClrStatusApbErr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware."]
    Enable = 0x01,
}
impl SramIntClrStatusApbErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIntClrStatusApbErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIntClrStatusApbErr {
    #[inline(always)]
    fn from(val: u8) -> SramIntClrStatusApbErr {
        SramIntClrStatusApbErr::from_bits(val)
    }
}
impl From<SramIntClrStatusApbErr> for u8 {
    #[inline(always)]
    fn from(val: SramIntClrStatusApbErr) -> u8 {
        SramIntClrStatusApbErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIntSetStatusApbErr {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware."]
    Enable = 0x01,
}
impl SramIntSetStatusApbErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIntSetStatusApbErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIntSetStatusApbErr {
    #[inline(always)]
    fn from(val: u8) -> SramIntSetStatusApbErr {
        SramIntSetStatusApbErr::from_bits(val)
    }
}
impl From<SramIntSetStatusApbErr> for u8 {
    #[inline(always)]
    fn from(val: SramIntSetStatusApbErr) -> u8 {
        SramIntSetStatusApbErr::to_bits(val)
    }
}
