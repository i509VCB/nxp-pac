#[doc = "GDET Configuration 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf0(pub u32);
impl GdetConf0 {
    #[doc = "GDET Configuration 0 Field 3_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_3_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "GDET Configuration 0 Field 3_0"]
    #[inline(always)]
    pub const fn set_field_3_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for GdetConf0 {
    #[inline(always)]
    fn default() -> GdetConf0 {
        GdetConf0(0)
    }
}
impl core::fmt::Debug for GdetConf0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf0")
            .field("field_3_0", &self.field_3_0())
            .field("sbz", &self.sbz())
            .field("rfu", &self.rfu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf0 {{ field_3_0: {=u8:?}, sbz: {=bool:?}, rfu: {=u32:?} }}",
            self.field_3_0(),
            self.sbz(),
            self.rfu()
        )
    }
}
#[doc = "GDET Configuration 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf1(pub u32);
impl GdetConf1 {
    #[doc = "GDET Configuration 1 Field 1_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_1_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "GDET Configuration 1 Field 1_0"]
    #[inline(always)]
    pub const fn set_field_1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "GDET Configuration 1 Field 3_2"]
    #[must_use]
    #[inline(always)]
    pub const fn field_3_2(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "GDET Configuration 1 Field 3_2"]
    #[inline(always)]
    pub const fn set_field_3_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "GDET Configuration 1 Field 7"]
    #[must_use]
    #[inline(always)]
    pub const fn field_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "GDET Configuration 1 Field 7"]
    #[inline(always)]
    pub const fn set_field_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GDET Configuration 1 Field 8"]
    #[must_use]
    #[inline(always)]
    pub const fn field_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GDET Configuration 1 Field 8"]
    #[inline(always)]
    pub const fn set_field_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz4(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz5(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for GdetConf1 {
    #[inline(always)]
    fn default() -> GdetConf1 {
        GdetConf1(0)
    }
}
impl core::fmt::Debug for GdetConf1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf1")
            .field("field_1_0", &self.field_1_0())
            .field("field_3_2", &self.field_3_2())
            .field("sbz1", &self.sbz1())
            .field("sbz2", &self.sbz2())
            .field("sbz3", &self.sbz3())
            .field("field_7", &self.field_7())
            .field("field_8", &self.field_8())
            .field("sbz4", &self.sbz4())
            .field("sbz5", &self.sbz5())
            .field("rfu", &self.rfu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf1 {{ field_1_0: {=u8:?}, field_3_2: {=u8:?}, sbz1: {=bool:?}, sbz2: {=bool:?}, sbz3: {=bool:?}, field_7: {=bool:?}, field_8: {=bool:?}, sbz4: {=bool:?}, sbz5: {=bool:?}, rfu: {=u32:?} }}",
            self.field_1_0(),
            self.field_3_2(),
            self.sbz1(),
            self.sbz2(),
            self.sbz3(),
            self.field_7(),
            self.field_8(),
            self.sbz4(),
            self.sbz5(),
            self.rfu()
        )
    }
}
#[doc = "GDET Configuration 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf2(pub u32);
impl GdetConf2 {
    #[doc = "GDET Configuration 2 Field 6_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_6_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "GDET Configuration 2 Field 6_0"]
    #[inline(always)]
    pub const fn set_field_6_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "GDET Configuration 2 Field 21_16"]
    #[must_use]
    #[inline(always)]
    pub const fn field_21_16(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "GDET Configuration 2 Field 21_16"]
    #[inline(always)]
    pub const fn set_field_21_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu2(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "GDET Configuration 2 Field 29_24"]
    #[must_use]
    #[inline(always)]
    pub const fn field_29_24(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "GDET Configuration 2 Field 29_24"]
    #[inline(always)]
    pub const fn set_field_29_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu3(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for GdetConf2 {
    #[inline(always)]
    fn default() -> GdetConf2 {
        GdetConf2(0)
    }
}
impl core::fmt::Debug for GdetConf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf2")
            .field("field_6_0", &self.field_6_0())
            .field("rfu1", &self.rfu1())
            .field("field_21_16", &self.field_21_16())
            .field("rfu2", &self.rfu2())
            .field("field_29_24", &self.field_29_24())
            .field("rfu3", &self.rfu3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf2 {{ field_6_0: {=u8:?}, rfu1: {=u16:?}, field_21_16: {=u8:?}, rfu2: {=u8:?}, field_29_24: {=u8:?}, rfu3: {=u8:?} }}",
            self.field_6_0(),
            self.rfu1(),
            self.field_21_16(),
            self.rfu2(),
            self.field_29_24(),
            self.rfu3()
        )
    }
}
#[doc = "GDET Configuration 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf3(pub u32);
impl GdetConf3 {
    #[doc = "GDET Configuration 3 Field 6_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_6_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "GDET Configuration 3 Field 6_0"]
    #[inline(always)]
    pub const fn set_field_6_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GdetConf3 {
    #[inline(always)]
    fn default() -> GdetConf3 {
        GdetConf3(0)
    }
}
impl core::fmt::Debug for GdetConf3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf3")
            .field("field_6_0", &self.field_6_0())
            .field("rfu1", &self.rfu1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf3 {{ field_6_0: {=u8:?}, rfu1: {=u32:?} }}",
            self.field_6_0(),
            self.rfu1()
        )
    }
}
#[doc = "GDET Configuration 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf4(pub u32);
impl GdetConf4 {
    #[doc = "GDET Configuration 4 Field 6_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_6_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "GDET Configuration 4 Field 6_0"]
    #[inline(always)]
    pub const fn set_field_6_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for GdetConf4 {
    #[inline(always)]
    fn default() -> GdetConf4 {
        GdetConf4(0)
    }
}
impl core::fmt::Debug for GdetConf4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf4")
            .field("field_6_0", &self.field_6_0())
            .field("rfu1", &self.rfu1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf4 {{ field_6_0: {=u8:?}, rfu1: {=u32:?} }}",
            self.field_6_0(),
            self.rfu1()
        )
    }
}
#[doc = "GDET Configuration 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetConf5(pub u32);
impl GdetConf5 {
    #[doc = "GDET Configuration 5 Field 5_0"]
    #[must_use]
    #[inline(always)]
    pub const fn field_5_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "GDET Configuration 5 Field 5_0"]
    #[inline(always)]
    pub const fn set_field_5_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "GDET Configuration 5 Field 11_6"]
    #[must_use]
    #[inline(always)]
    pub const fn field_11_6(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "GDET Configuration 5 Field 11_6"]
    #[inline(always)]
    pub const fn set_field_11_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for GdetConf5 {
    #[inline(always)]
    fn default() -> GdetConf5 {
        GdetConf5(0)
    }
}
impl core::fmt::Debug for GdetConf5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetConf5")
            .field("field_5_0", &self.field_5_0())
            .field("field_11_6", &self.field_11_6())
            .field("rfu1", &self.rfu1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetConf5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetConf5 {{ field_5_0: {=u8:?}, field_11_6: {=u8:?}, rfu1: {=u32:?} }}",
            self.field_5_0(),
            self.field_11_6(),
            self.rfu1()
        )
    }
}
#[doc = "GDET Delay Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetDlyCtrl(pub u32);
impl GdetDlyCtrl {
    #[doc = "GDET Delay Control of the Voltage Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vol_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "GDET Delay Control of the Voltage Mode"]
    #[inline(always)]
    pub const fn set_vol_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Select the Control of the Trim Code to the Delay Line"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_vol_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Select the Control of the Trim Code to the Delay Line"]
    #[inline(always)]
    pub const fn set_sw_vol_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for GdetDlyCtrl {
    #[inline(always)]
    fn default() -> GdetDlyCtrl {
        GdetDlyCtrl(0)
    }
}
impl core::fmt::Debug for GdetDlyCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetDlyCtrl")
            .field("vol_sel", &self.vol_sel())
            .field("sw_vol_ctrl", &self.sw_vol_ctrl())
            .field("rfu", &self.rfu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetDlyCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetDlyCtrl {{ vol_sel: {=u8:?}, sw_vol_ctrl: {=bool:?}, rfu: {=u32:?} }}",
            self.vol_sel(),
            self.sw_vol_ctrl(),
            self.rfu()
        )
    }
}
#[doc = "GDET Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetEnable1(pub u32);
impl GdetEnable1 {
    #[doc = "If set, the detector will be clock gated"]
    #[must_use]
    #[inline(always)]
    pub const fn en1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set, the detector will be clock gated"]
    #[inline(always)]
    pub const fn set_en1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for GdetEnable1 {
    #[inline(always)]
    fn default() -> GdetEnable1 {
        GdetEnable1(0)
    }
}
impl core::fmt::Debug for GdetEnable1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetEnable1")
            .field("en1", &self.en1())
            .field("rfu", &self.rfu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetEnable1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetEnable1 {{ en1: {=bool:?}, rfu: {=u32:?} }}",
            self.en1(),
            self.rfu()
        )
    }
}
#[doc = "GDET Reset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetReset(pub u32);
impl GdetReset {
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Soft Reset for the Core Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sft_rst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Soft Reset for the Core Reset"]
    #[inline(always)]
    pub const fn set_sft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu2(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for GdetReset {
    #[inline(always)]
    fn default() -> GdetReset {
        GdetReset(0)
    }
}
impl core::fmt::Debug for GdetReset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetReset")
            .field("rfu1", &self.rfu1())
            .field("sft_rst", &self.sft_rst())
            .field("rfu2", &self.rfu2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetReset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetReset {{ rfu1: {=u8:?}, sft_rst: {=bool:?}, rfu2: {=u32:?} }}",
            self.rfu1(),
            self.sft_rst(),
            self.rfu2()
        )
    }
}
#[doc = "GDET Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetTest(pub u32);
impl GdetTest {
    #[doc = "Should Be Left to Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn sbz(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Should Be Left to Zero"]
    #[inline(always)]
    pub const fn set_sbz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved for Future Use"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Reserved for Future Use"]
    #[inline(always)]
    pub const fn set_rfu(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for GdetTest {
    #[inline(always)]
    fn default() -> GdetTest {
        GdetTest(0)
    }
}
impl core::fmt::Debug for GdetTest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetTest")
            .field("sbz", &self.sbz())
            .field("rfu", &self.rfu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetTest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetTest {{ sbz: {=bool:?}, rfu: {=u32:?} }}",
            self.sbz(),
            self.rfu()
        )
    }
}
