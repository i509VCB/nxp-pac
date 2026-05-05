#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_0(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_0 {
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_AMBIENT_0 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_0 {
        AUX_BIAS_CURVE_AMBIENT_0(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_0")
            .field("VREF1VCURVETRIM_0", &self.VREF1VCURVETRIM_0())
            .field("VREF1VCURVETRIM_1", &self.VREF1VCURVETRIM_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_0 {{ VREF1VCURVETRIM_0: {=u16:?}, VREF1VCURVETRIM_1: {=u16:?} }}",
            self.VREF1VCURVETRIM_0(),
            self.VREF1VCURVETRIM_1()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_1(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_1 {
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_AMBIENT_1 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_1 {
        AUX_BIAS_CURVE_AMBIENT_1(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_1")
            .field("VREF1VCURVETRIM_2", &self.VREF1VCURVETRIM_2())
            .field("VREF1VCURVETRIM_3", &self.VREF1VCURVETRIM_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_1 {{ VREF1VCURVETRIM_2: {=u16:?}, VREF1VCURVETRIM_3: {=u16:?} }}",
            self.VREF1VCURVETRIM_2(),
            self.VREF1VCURVETRIM_3()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_2(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_2 {
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_AMBIENT_2 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_2 {
        AUX_BIAS_CURVE_AMBIENT_2(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_2")
            .field("VREF1VCURVETRIM_4", &self.VREF1VCURVETRIM_4())
            .field("VREF1VCURVETRIM_5", &self.VREF1VCURVETRIM_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_2 {{ VREF1VCURVETRIM_4: {=u16:?}, VREF1VCURVETRIM_5: {=u16:?} }}",
            self.VREF1VCURVETRIM_4(),
            self.VREF1VCURVETRIM_5()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_3(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_3 {
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_AMBIENT_3 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_3 {
        AUX_BIAS_CURVE_AMBIENT_3(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_3")
            .field("VREF1VCURVETRIM_6", &self.VREF1VCURVETRIM_6())
            .field("VREF1VCURVETRIM_7", &self.VREF1VCURVETRIM_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_3 {{ VREF1VCURVETRIM_6: {=u16:?}, VREF1VCURVETRIM_7: {=u16:?} }}",
            self.VREF1VCURVETRIM_6(),
            self.VREF1VCURVETRIM_7()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_ARRAY0(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
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
impl Default for AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
        AUX_BIAS_CURVE_AMBIENT_ARRAY0(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_ARRAY1(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
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
impl Default for AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
        AUX_BIAS_CURVE_AMBIENT_ARRAY1(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_ARRAY2(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
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
impl Default for AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
        AUX_BIAS_CURVE_AMBIENT_ARRAY2(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_ARRAY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve Ambient (30degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_AMBIENT_ARRAY3(pub u32);
impl AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
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
impl Default for AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
        AUX_BIAS_CURVE_AMBIENT_ARRAY3(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_AMBIENT_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_AMBIENT_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_AMBIENT_ARRAY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_0(pub u32);
impl AUX_BIAS_CURVE_TEMP_0 {
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_0 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_1 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_TEMP_0 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_0 {
        AUX_BIAS_CURVE_TEMP_0(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_0")
            .field("VREF1VCURVETRIM_0", &self.VREF1VCURVETRIM_0())
            .field("VREF1VCURVETRIM_1", &self.VREF1VCURVETRIM_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_0 {{ VREF1VCURVETRIM_0: {=u16:?}, VREF1VCURVETRIM_1: {=u16:?} }}",
            self.VREF1VCURVETRIM_0(),
            self.VREF1VCURVETRIM_1()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_1(pub u32);
impl AUX_BIAS_CURVE_TEMP_1 {
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_2 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_3 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_TEMP_1 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_1 {
        AUX_BIAS_CURVE_TEMP_1(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_1")
            .field("VREF1VCURVETRIM_2", &self.VREF1VCURVETRIM_2())
            .field("VREF1VCURVETRIM_3", &self.VREF1VCURVETRIM_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_1 {{ VREF1VCURVETRIM_2: {=u16:?}, VREF1VCURVETRIM_3: {=u16:?} }}",
            self.VREF1VCURVETRIM_2(),
            self.VREF1VCURVETRIM_3()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_2(pub u32);
impl AUX_BIAS_CURVE_TEMP_2 {
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_4 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_5 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_TEMP_2 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_2 {
        AUX_BIAS_CURVE_TEMP_2(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_2")
            .field("VREF1VCURVETRIM_4", &self.VREF1VCURVETRIM_4())
            .field("VREF1VCURVETRIM_5", &self.VREF1VCURVETRIM_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_2 {{ VREF1VCURVETRIM_4: {=u16:?}, VREF1VCURVETRIM_5: {=u16:?} }}",
            self.VREF1VCURVETRIM_4(),
            self.VREF1VCURVETRIM_5()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_3(pub u32);
impl AUX_BIAS_CURVE_TEMP_3 {
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_6 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM_7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VREF1VCURVETRIM_7 (unit: 100uV)."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AUX_BIAS_CURVE_TEMP_3 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_3 {
        AUX_BIAS_CURVE_TEMP_3(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_3")
            .field("VREF1VCURVETRIM_6", &self.VREF1VCURVETRIM_6())
            .field("VREF1VCURVETRIM_7", &self.VREF1VCURVETRIM_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_3 {{ VREF1VCURVETRIM_6: {=u16:?}, VREF1VCURVETRIM_7: {=u16:?} }}",
            self.VREF1VCURVETRIM_6(),
            self.VREF1VCURVETRIM_7()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_ARRAY0(pub u32);
impl AUX_BIAS_CURVE_TEMP_ARRAY0 {
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
impl Default for AUX_BIAS_CURVE_TEMP_ARRAY0 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_ARRAY0 {
        AUX_BIAS_CURVE_TEMP_ARRAY0(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_ARRAY1(pub u32);
impl AUX_BIAS_CURVE_TEMP_ARRAY1 {
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
impl Default for AUX_BIAS_CURVE_TEMP_ARRAY1 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_ARRAY1 {
        AUX_BIAS_CURVE_TEMP_ARRAY1(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_ARRAY2(pub u32);
impl AUX_BIAS_CURVE_TEMP_ARRAY2 {
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
impl Default for AUX_BIAS_CURVE_TEMP_ARRAY2 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_ARRAY2 {
        AUX_BIAS_CURVE_TEMP_ARRAY2(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_ARRAY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "Aux Bias Curve TEMP (105degC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS_CURVE_TEMP_ARRAY3(pub u32);
impl AUX_BIAS_CURVE_TEMP_ARRAY3 {
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
impl Default for AUX_BIAS_CURVE_TEMP_ARRAY3 {
    #[inline(always)]
    fn default() -> AUX_BIAS_CURVE_TEMP_ARRAY3 {
        AUX_BIAS_CURVE_TEMP_ARRAY3(0)
    }
}
impl core::fmt::Debug for AUX_BIAS_CURVE_TEMP_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS_CURVE_TEMP_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS_CURVE_TEMP_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS_CURVE_TEMP_ARRAY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOD(pub u32);
impl BOD {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_VBAT_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_VBAT_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_VBAT_TRIGLVL(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_VBAT_TRIGLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_VBAT_HYST(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_VBAT_HYST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_CORE_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_CORE_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_CORE_TRIGLVL(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_CORE_TRIGLVL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn BOD_CORE_HYST(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_BOD_CORE_HYST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
}
impl Default for BOD {
    #[inline(always)]
    fn default() -> BOD {
        BOD(0)
    }
}
impl core::fmt::Debug for BOD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD")
            .field("BOD_VBAT_TRIM_VALID", &self.BOD_VBAT_TRIM_VALID())
            .field("BOD_VBAT_TRIGLVL", &self.BOD_VBAT_TRIGLVL())
            .field("BOD_VBAT_HYST", &self.BOD_VBAT_HYST())
            .field("BOD_CORE_TRIM_VALID", &self.BOD_CORE_TRIM_VALID())
            .field("BOD_CORE_TRIGLVL", &self.BOD_CORE_TRIGLVL())
            .field("BOD_CORE_HYST", &self.BOD_CORE_HYST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOD {{ BOD_VBAT_TRIM_VALID: {=bool:?}, BOD_VBAT_TRIGLVL: {=u8:?}, BOD_VBAT_HYST: {=u8:?}, BOD_CORE_TRIM_VALID: {=bool:?}, BOD_CORE_TRIGLVL: {=u8:?}, BOD_CORE_HYST: {=u8:?} }}",
            self.BOD_VBAT_TRIM_VALID(),
            self.BOD_VBAT_TRIGLVL(),
            self.BOD_VBAT_HYST(),
            self.BOD_CORE_TRIM_VALID(),
            self.BOD_CORE_TRIGLVL(),
            self.BOD_CORE_HYST()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_HIGH_0(pub u32);
impl DCDC_POWER_PROFILE_HIGH_0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_DCDC_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn RC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_RC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn ICOMP(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_ICOMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn ISEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_ISEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn ICENABLE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_ICENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TMOS(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_TMOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLEISENSE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_DISABLEISENSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_VOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn SLICINGENABLE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_SLICINGENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn INDUCTORCLAMPENABLE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_INDUCTORCLAMPENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT_PWD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_VOUT_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DCDC_POWER_PROFILE_HIGH_0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_HIGH_0 {
        DCDC_POWER_PROFILE_HIGH_0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_HIGH_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_HIGH_0")
            .field("DCDC_TRIM_VALID", &self.DCDC_TRIM_VALID())
            .field("RC", &self.RC())
            .field("ICOMP", &self.ICOMP())
            .field("ISEL", &self.ISEL())
            .field("ICENABLE", &self.ICENABLE())
            .field("TMOS", &self.TMOS())
            .field("DISABLEISENSE", &self.DISABLEISENSE())
            .field("VOUT", &self.VOUT())
            .field("SLICINGENABLE", &self.SLICINGENABLE())
            .field("INDUCTORCLAMPENABLE", &self.INDUCTORCLAMPENABLE())
            .field("VOUT_PWD", &self.VOUT_PWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_HIGH_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_HIGH_0 {{ DCDC_TRIM_VALID: {=bool:?}, RC: {=u8:?}, ICOMP: {=u8:?}, ISEL: {=u8:?}, ICENABLE: {=bool:?}, TMOS: {=u8:?}, DISABLEISENSE: {=bool:?}, VOUT: {=u8:?}, SLICINGENABLE: {=bool:?}, INDUCTORCLAMPENABLE: {=bool:?}, VOUT_PWD: {=u8:?} }}",
            self.DCDC_TRIM_VALID(),
            self.RC(),
            self.ICOMP(),
            self.ISEL(),
            self.ICENABLE(),
            self.TMOS(),
            self.DISABLEISENSE(),
            self.VOUT(),
            self.SLICINGENABLE(),
            self.INDUCTORCLAMPENABLE(),
            self.VOUT_PWD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_HIGH_1(pub u32);
impl DCDC_POWER_PROFILE_HIGH_1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn RTRIMOFFET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_RTRIMOFFET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn RSENSETRIM(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_RSENSETRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTENABLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_DTESTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCURVE(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETCURVE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETDC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETDC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTSEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_DTESTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn ISCALEENABLE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_ISCALEENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEBYPASS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_FORCEBYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMAUTOCOT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_TRIMAUTOCOT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEFULLCYCLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_FORCEFULLCYCLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn LCENABLE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_LCENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFF(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_TOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFFENABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_TOFFENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DCDC_POWER_PROFILE_HIGH_1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_HIGH_1 {
        DCDC_POWER_PROFILE_HIGH_1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_HIGH_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_HIGH_1")
            .field("RTRIMOFFET", &self.RTRIMOFFET())
            .field("RSENSETRIM", &self.RSENSETRIM())
            .field("DTESTENABLE", &self.DTESTENABLE())
            .field("SETCURVE", &self.SETCURVE())
            .field("SETDC", &self.SETDC())
            .field("DTESTSEL", &self.DTESTSEL())
            .field("ISCALEENABLE", &self.ISCALEENABLE())
            .field("FORCEBYPASS", &self.FORCEBYPASS())
            .field("TRIMAUTOCOT", &self.TRIMAUTOCOT())
            .field("FORCEFULLCYCLE", &self.FORCEFULLCYCLE())
            .field("LCENABLE", &self.LCENABLE())
            .field("TOFF", &self.TOFF())
            .field("TOFFENABLE", &self.TOFFENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_HIGH_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_HIGH_1 {{ RTRIMOFFET: {=u8:?}, RSENSETRIM: {=u8:?}, DTESTENABLE: {=bool:?}, SETCURVE: {=u8:?}, SETDC: {=u8:?}, DTESTSEL: {=u8:?}, ISCALEENABLE: {=bool:?}, FORCEBYPASS: {=bool:?}, TRIMAUTOCOT: {=u8:?}, FORCEFULLCYCLE: {=bool:?}, LCENABLE: {=bool:?}, TOFF: {=u8:?}, TOFFENABLE: {=bool:?} }}",
            self.RTRIMOFFET(),
            self.RSENSETRIM(),
            self.DTESTENABLE(),
            self.SETCURVE(),
            self.SETDC(),
            self.DTESTSEL(),
            self.ISCALEENABLE(),
            self.FORCEBYPASS(),
            self.TRIMAUTOCOT(),
            self.FORCEFULLCYCLE(),
            self.LCENABLE(),
            self.TOFF(),
            self.TOFFENABLE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_HIGH_ARRAY0(pub u32);
impl DCDC_POWER_PROFILE_HIGH_ARRAY0 {
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
impl Default for DCDC_POWER_PROFILE_HIGH_ARRAY0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_HIGH_ARRAY0 {
        DCDC_POWER_PROFILE_HIGH_ARRAY0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_HIGH_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_HIGH_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_HIGH_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_HIGH_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_HIGH_ARRAY1(pub u32);
impl DCDC_POWER_PROFILE_HIGH_ARRAY1 {
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
impl Default for DCDC_POWER_PROFILE_HIGH_ARRAY1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_HIGH_ARRAY1 {
        DCDC_POWER_PROFILE_HIGH_ARRAY1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_HIGH_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_HIGH_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_HIGH_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_HIGH_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_LOW_0(pub u32);
impl DCDC_POWER_PROFILE_LOW_0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_DCDC_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn RC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_RC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn ICOMP(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_ICOMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn ISEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_ISEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn ICENABLE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_ICENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TMOS(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_TMOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLEISENSE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_DISABLEISENSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_VOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn SLICINGENABLE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_SLICINGENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn INDUCTORCLAMPENABLE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_INDUCTORCLAMPENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT_PWD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_VOUT_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DCDC_POWER_PROFILE_LOW_0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_LOW_0 {
        DCDC_POWER_PROFILE_LOW_0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_LOW_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_LOW_0")
            .field("DCDC_TRIM_VALID", &self.DCDC_TRIM_VALID())
            .field("RC", &self.RC())
            .field("ICOMP", &self.ICOMP())
            .field("ISEL", &self.ISEL())
            .field("ICENABLE", &self.ICENABLE())
            .field("TMOS", &self.TMOS())
            .field("DISABLEISENSE", &self.DISABLEISENSE())
            .field("VOUT", &self.VOUT())
            .field("SLICINGENABLE", &self.SLICINGENABLE())
            .field("INDUCTORCLAMPENABLE", &self.INDUCTORCLAMPENABLE())
            .field("VOUT_PWD", &self.VOUT_PWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_LOW_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_LOW_0 {{ DCDC_TRIM_VALID: {=bool:?}, RC: {=u8:?}, ICOMP: {=u8:?}, ISEL: {=u8:?}, ICENABLE: {=bool:?}, TMOS: {=u8:?}, DISABLEISENSE: {=bool:?}, VOUT: {=u8:?}, SLICINGENABLE: {=bool:?}, INDUCTORCLAMPENABLE: {=bool:?}, VOUT_PWD: {=u8:?} }}",
            self.DCDC_TRIM_VALID(),
            self.RC(),
            self.ICOMP(),
            self.ISEL(),
            self.ICENABLE(),
            self.TMOS(),
            self.DISABLEISENSE(),
            self.VOUT(),
            self.SLICINGENABLE(),
            self.INDUCTORCLAMPENABLE(),
            self.VOUT_PWD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_LOW_1(pub u32);
impl DCDC_POWER_PROFILE_LOW_1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn RTRIMOFFET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_RTRIMOFFET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn RSENSETRIM(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_RSENSETRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTENABLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_DTESTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCURVE(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETCURVE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETDC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETDC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTSEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_DTESTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn ISCALEENABLE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_ISCALEENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEBYPASS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_FORCEBYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMAUTOCOT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_TRIMAUTOCOT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEFULLCYCLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_FORCEFULLCYCLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn LCENABLE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_LCENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFF(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_TOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFFENABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_TOFFENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DCDC_POWER_PROFILE_LOW_1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_LOW_1 {
        DCDC_POWER_PROFILE_LOW_1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_LOW_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_LOW_1")
            .field("RTRIMOFFET", &self.RTRIMOFFET())
            .field("RSENSETRIM", &self.RSENSETRIM())
            .field("DTESTENABLE", &self.DTESTENABLE())
            .field("SETCURVE", &self.SETCURVE())
            .field("SETDC", &self.SETDC())
            .field("DTESTSEL", &self.DTESTSEL())
            .field("ISCALEENABLE", &self.ISCALEENABLE())
            .field("FORCEBYPASS", &self.FORCEBYPASS())
            .field("TRIMAUTOCOT", &self.TRIMAUTOCOT())
            .field("FORCEFULLCYCLE", &self.FORCEFULLCYCLE())
            .field("LCENABLE", &self.LCENABLE())
            .field("TOFF", &self.TOFF())
            .field("TOFFENABLE", &self.TOFFENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_LOW_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_LOW_1 {{ RTRIMOFFET: {=u8:?}, RSENSETRIM: {=u8:?}, DTESTENABLE: {=bool:?}, SETCURVE: {=u8:?}, SETDC: {=u8:?}, DTESTSEL: {=u8:?}, ISCALEENABLE: {=bool:?}, FORCEBYPASS: {=bool:?}, TRIMAUTOCOT: {=u8:?}, FORCEFULLCYCLE: {=bool:?}, LCENABLE: {=bool:?}, TOFF: {=u8:?}, TOFFENABLE: {=bool:?} }}",
            self.RTRIMOFFET(),
            self.RSENSETRIM(),
            self.DTESTENABLE(),
            self.SETCURVE(),
            self.SETDC(),
            self.DTESTSEL(),
            self.ISCALEENABLE(),
            self.FORCEBYPASS(),
            self.TRIMAUTOCOT(),
            self.FORCEFULLCYCLE(),
            self.LCENABLE(),
            self.TOFF(),
            self.TOFFENABLE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_LOW_ARRAY0(pub u32);
impl DCDC_POWER_PROFILE_LOW_ARRAY0 {
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
impl Default for DCDC_POWER_PROFILE_LOW_ARRAY0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_LOW_ARRAY0 {
        DCDC_POWER_PROFILE_LOW_ARRAY0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_LOW_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_LOW_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_LOW_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_LOW_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_LOW_ARRAY1(pub u32);
impl DCDC_POWER_PROFILE_LOW_ARRAY1 {
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
impl Default for DCDC_POWER_PROFILE_LOW_ARRAY1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_LOW_ARRAY1 {
        DCDC_POWER_PROFILE_LOW_ARRAY1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_LOW_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_LOW_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_LOW_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_LOW_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_MEDIUM_0(pub u32);
impl DCDC_POWER_PROFILE_MEDIUM_0 {
    #[doc = "DCDC is trimed."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC is trimed."]
    #[inline(always)]
    pub const fn set_DCDC_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Constant On-Time calibration."]
    #[must_use]
    #[inline(always)]
    pub const fn RC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "Constant On-Time calibration."]
    #[inline(always)]
    pub const fn set_RC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "Select the type of ZCD comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn ICOMP(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Select the type of ZCD comparator."]
    #[inline(always)]
    pub const fn set_ICOMP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "Alter Internal biasing currents."]
    #[must_use]
    #[inline(always)]
    pub const fn ISEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Alter Internal biasing currents."]
    #[inline(always)]
    pub const fn set_ISEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[must_use]
    #[inline(always)]
    pub const fn ICENABLE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of auto scaling of COT period with variations in VDD."]
    #[inline(always)]
    pub const fn set_ICENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[must_use]
    #[inline(always)]
    pub const fn TMOS(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "One-shot generator reference current trimming signal."]
    #[inline(always)]
    pub const fn set_TMOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Disable Current sensing."]
    #[must_use]
    #[inline(always)]
    pub const fn DISABLEISENSE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Current sensing."]
    #[inline(always)]
    pub const fn set_DISABLEISENSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Set output regulation voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage."]
    #[inline(always)]
    pub const fn set_VOUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Enable staggered switching of power switches."]
    #[must_use]
    #[inline(always)]
    pub const fn SLICINGENABLE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enable staggered switching of power switches."]
    #[inline(always)]
    pub const fn set_SLICINGENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[must_use]
    #[inline(always)]
    pub const fn INDUCTORCLAMPENABLE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable shorting of Inductor during PFM idle time."]
    #[inline(always)]
    pub const fn set_INDUCTORCLAMPENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT_PWD(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set output regulation voltage during Deep Sleep."]
    #[inline(always)]
    pub const fn set_VOUT_PWD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for DCDC_POWER_PROFILE_MEDIUM_0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_MEDIUM_0 {
        DCDC_POWER_PROFILE_MEDIUM_0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_MEDIUM_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_MEDIUM_0")
            .field("DCDC_TRIM_VALID", &self.DCDC_TRIM_VALID())
            .field("RC", &self.RC())
            .field("ICOMP", &self.ICOMP())
            .field("ISEL", &self.ISEL())
            .field("ICENABLE", &self.ICENABLE())
            .field("TMOS", &self.TMOS())
            .field("DISABLEISENSE", &self.DISABLEISENSE())
            .field("VOUT", &self.VOUT())
            .field("SLICINGENABLE", &self.SLICINGENABLE())
            .field("INDUCTORCLAMPENABLE", &self.INDUCTORCLAMPENABLE())
            .field("VOUT_PWD", &self.VOUT_PWD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_MEDIUM_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_MEDIUM_0 {{ DCDC_TRIM_VALID: {=bool:?}, RC: {=u8:?}, ICOMP: {=u8:?}, ISEL: {=u8:?}, ICENABLE: {=bool:?}, TMOS: {=u8:?}, DISABLEISENSE: {=bool:?}, VOUT: {=u8:?}, SLICINGENABLE: {=bool:?}, INDUCTORCLAMPENABLE: {=bool:?}, VOUT_PWD: {=u8:?} }}",
            self.DCDC_TRIM_VALID(),
            self.RC(),
            self.ICOMP(),
            self.ISEL(),
            self.ICENABLE(),
            self.TMOS(),
            self.DISABLEISENSE(),
            self.VOUT(),
            self.SLICINGENABLE(),
            self.INDUCTORCLAMPENABLE(),
            self.VOUT_PWD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_MEDIUM_1(pub u32);
impl DCDC_POWER_PROFILE_MEDIUM_1 {
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn RTRIMOFFET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust the offset voltage of BJT based comparator."]
    #[inline(always)]
    pub const fn set_RTRIMOFFET(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[must_use]
    #[inline(always)]
    pub const fn RSENSETRIM(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Adjust Max inductor peak current limiting."]
    #[inline(always)]
    pub const fn set_RSENSETRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Digital test signals."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTENABLE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Digital test signals."]
    #[inline(always)]
    pub const fn set_DTESTENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETCURVE(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETCURVE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Bandgap calibration parameter."]
    #[must_use]
    #[inline(always)]
    pub const fn SETDC(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Bandgap calibration parameter."]
    #[inline(always)]
    pub const fn set_SETDC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select the output signal for test."]
    #[must_use]
    #[inline(always)]
    pub const fn DTESTSEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Select the output signal for test."]
    #[inline(always)]
    pub const fn set_DTESTSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Modify COT behavior."]
    #[must_use]
    #[inline(always)]
    pub const fn ISCALEENABLE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Modify COT behavior."]
    #[inline(always)]
    pub const fn set_ISCALEENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force bypass mode."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEBYPASS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force bypass mode."]
    #[inline(always)]
    pub const fn set_FORCEBYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIMAUTOCOT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Change the scaling ratio of the feedforward compensation."]
    #[inline(always)]
    pub const fn set_TRIMAUTOCOT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCEFULLCYCLE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force full PFM PMOS and NMOS cycle."]
    #[inline(always)]
    pub const fn set_FORCEFULLCYCLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[must_use]
    #[inline(always)]
    pub const fn LCENABLE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Change the range of the peak detector of current inside the inductor."]
    #[inline(always)]
    pub const fn set_LCENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Constant Off-Time calibration input."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFF(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "Constant Off-Time calibration input."]
    #[inline(always)]
    pub const fn set_TOFF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[must_use]
    #[inline(always)]
    pub const fn TOFFENABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Constant Off-Time feature."]
    #[inline(always)]
    pub const fn set_TOFFENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DCDC_POWER_PROFILE_MEDIUM_1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_MEDIUM_1 {
        DCDC_POWER_PROFILE_MEDIUM_1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_MEDIUM_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_MEDIUM_1")
            .field("RTRIMOFFET", &self.RTRIMOFFET())
            .field("RSENSETRIM", &self.RSENSETRIM())
            .field("DTESTENABLE", &self.DTESTENABLE())
            .field("SETCURVE", &self.SETCURVE())
            .field("SETDC", &self.SETDC())
            .field("DTESTSEL", &self.DTESTSEL())
            .field("ISCALEENABLE", &self.ISCALEENABLE())
            .field("FORCEBYPASS", &self.FORCEBYPASS())
            .field("TRIMAUTOCOT", &self.TRIMAUTOCOT())
            .field("FORCEFULLCYCLE", &self.FORCEFULLCYCLE())
            .field("LCENABLE", &self.LCENABLE())
            .field("TOFF", &self.TOFF())
            .field("TOFFENABLE", &self.TOFFENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_MEDIUM_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_MEDIUM_1 {{ RTRIMOFFET: {=u8:?}, RSENSETRIM: {=u8:?}, DTESTENABLE: {=bool:?}, SETCURVE: {=u8:?}, SETDC: {=u8:?}, DTESTSEL: {=u8:?}, ISCALEENABLE: {=bool:?}, FORCEBYPASS: {=bool:?}, TRIMAUTOCOT: {=u8:?}, FORCEFULLCYCLE: {=bool:?}, LCENABLE: {=bool:?}, TOFF: {=u8:?}, TOFFENABLE: {=bool:?} }}",
            self.RTRIMOFFET(),
            self.RSENSETRIM(),
            self.DTESTENABLE(),
            self.SETCURVE(),
            self.SETDC(),
            self.DTESTSEL(),
            self.ISCALEENABLE(),
            self.FORCEBYPASS(),
            self.TRIMAUTOCOT(),
            self.FORCEFULLCYCLE(),
            self.LCENABLE(),
            self.TOFF(),
            self.TOFFENABLE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_MEDIUM_ARRAY0(pub u32);
impl DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
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
impl Default for DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
        DCDC_POWER_PROFILE_MEDIUM_ARRAY0(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_MEDIUM_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_MEDIUM_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCDC_POWER_PROFILE_MEDIUM_ARRAY1(pub u32);
impl DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
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
impl Default for DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
    #[inline(always)]
    fn default() -> DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
        DCDC_POWER_PROFILE_MEDIUM_ARRAY1(0)
    }
}
impl core::fmt::Debug for DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_POWER_PROFILE_MEDIUM_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCDC_POWER_PROFILE_MEDIUM_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEVICE_TYPE(pub u32);
impl DEVICE_TYPE {
    #[doc = "Device type number. (E.g : LPC5569 stored as 5569 decimal)."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_TYPE_NUM(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 5569 decimal)."]
    #[inline(always)]
    pub const fn set_DEVICE_TYPE_NUM(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_TYPE_SEC(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)."]
    #[inline(always)]
    pub const fn set_DEVICE_TYPE_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_TYPE_PKG(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Device package type: 0000 : HLQFP 0001 : HTQFP 0010 : HVQFN 0100 : VFBGA 1000 : WLCSP."]
    #[inline(always)]
    pub const fn set_DEVICE_TYPE_PKG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of pins on the package."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_TYPE_PIN(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Number of pins on the package."]
    #[inline(always)]
    pub const fn set_DEVICE_TYPE_PIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DEVICE_TYPE {
    #[inline(always)]
    fn default() -> DEVICE_TYPE {
        DEVICE_TYPE(0)
    }
}
impl core::fmt::Debug for DEVICE_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_TYPE")
            .field("DEVICE_TYPE_NUM", &self.DEVICE_TYPE_NUM())
            .field("DEVICE_TYPE_SEC", &self.DEVICE_TYPE_SEC())
            .field("DEVICE_TYPE_PKG", &self.DEVICE_TYPE_PKG())
            .field("DEVICE_TYPE_PIN", &self.DEVICE_TYPE_PIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEVICE_TYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEVICE_TYPE {{ DEVICE_TYPE_NUM: {=u16:?}, DEVICE_TYPE_SEC: {=bool:?}, DEVICE_TYPE_PKG: {=u8:?}, DEVICE_TYPE_PIN: {=u8:?} }}",
            self.DEVICE_TYPE_NUM(),
            self.DEVICE_TYPE_SEC(),
            self.DEVICE_TYPE_PKG(),
            self.DEVICE_TYPE_PIN()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DIS_ROM_HIDING(pub u32);
impl DIS_ROM_HIDING {
    #[doc = "When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[must_use]
    #[inline(always)]
    pub const fn DIS_ROM_HIDING(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "When 0x3CC35AA5 ROM hiding feture is disabled. All other values critical ROM is hidden."]
    #[inline(always)]
    pub const fn set_DIS_ROM_HIDING(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DIS_ROM_HIDING {
    #[inline(always)]
    fn default() -> DIS_ROM_HIDING {
        DIS_ROM_HIDING(0)
    }
}
impl core::fmt::Debug for DIS_ROM_HIDING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIS_ROM_HIDING")
            .field("DIS_ROM_HIDING", &self.DIS_ROM_HIDING())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DIS_ROM_HIDING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DIS_ROM_HIDING {{ DIS_ROM_HIDING: {=u32:?} }}",
            self.DIS_ROM_HIDING()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_0(pub u32);
impl ECID_BACKUP_0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn COORD_Y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_COORD_Y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn COORD_X(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_COORD_X(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ECID_BACKUP_0 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_0 {
        ECID_BACKUP_0(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_0")
            .field("COORD_Y", &self.COORD_Y())
            .field("COORD_X", &self.COORD_X())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ECID_BACKUP_0 {{ COORD_Y: {=u16:?}, COORD_X: {=u16:?} }}",
            self.COORD_Y(),
            self.COORD_X()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_1(pub u32);
impl ECID_BACKUP_1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn WAFER(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_WAFER(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ECID_BACKUP_1 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_1 {
        ECID_BACKUP_1(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_1")
            .field("WAFER", &self.WAFER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ECID_BACKUP_1 {{ WAFER: {=u8:?} }}", self.WAFER())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_2(pub u32);
impl ECID_BACKUP_2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn LOTID_LSB(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_LOTID_LSB(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ECID_BACKUP_2 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_2 {
        ECID_BACKUP_2(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_2")
            .field("LOTID_LSB", &self.LOTID_LSB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ECID_BACKUP_2 {{ LOTID_LSB: {=u32:?} }}",
            self.LOTID_LSB()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_3(pub u32);
impl ECID_BACKUP_3 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn LOTID_MSB(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_LOTID_MSB(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ECID_BACKUP_3 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_3 {
        ECID_BACKUP_3(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_3")
            .field("LOTID_MSB", &self.LOTID_MSB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ECID_BACKUP_3 {{ LOTID_MSB: {=u32:?} }}",
            self.LOTID_MSB()
        )
    }
}
#[doc = "ECID backup (the original is in page n-1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_ARRAY0(pub u32);
impl ECID_BACKUP_ARRAY0 {
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
impl Default for ECID_BACKUP_ARRAY0 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_ARRAY0 {
        ECID_BACKUP_ARRAY0(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ECID_BACKUP_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "ECID backup (the original is in page n-1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_ARRAY1(pub u32);
impl ECID_BACKUP_ARRAY1 {
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
impl Default for ECID_BACKUP_ARRAY1 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_ARRAY1 {
        ECID_BACKUP_ARRAY1(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ECID_BACKUP_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "ECID backup (the original is in page n-1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_ARRAY2(pub u32);
impl ECID_BACKUP_ARRAY2 {
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
impl Default for ECID_BACKUP_ARRAY2 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_ARRAY2 {
        ECID_BACKUP_ARRAY2(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ECID_BACKUP_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "ECID backup (the original is in page n-1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECID_BACKUP_ARRAY3(pub u32);
impl ECID_BACKUP_ARRAY3 {
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
impl Default for ECID_BACKUP_ARRAY3 {
    #[inline(always)]
    fn default() -> ECID_BACKUP_ARRAY3 {
        ECID_BACKUP_ARRAY3(0)
    }
}
impl core::fmt::Debug for ECID_BACKUP_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECID_BACKUP_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECID_BACKUP_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ECID_BACKUP_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_0(pub u32);
impl FINAL_TEST_BATCH_ID_0 {
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
impl Default for FINAL_TEST_BATCH_ID_0 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_0 {
        FINAL_TEST_BATCH_ID_0(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_1(pub u32);
impl FINAL_TEST_BATCH_ID_1 {
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
impl Default for FINAL_TEST_BATCH_ID_1 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_1 {
        FINAL_TEST_BATCH_ID_1(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_2(pub u32);
impl FINAL_TEST_BATCH_ID_2 {
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
impl Default for FINAL_TEST_BATCH_ID_2 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_2 {
        FINAL_TEST_BATCH_ID_2(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_3(pub u32);
impl FINAL_TEST_BATCH_ID_3 {
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
impl Default for FINAL_TEST_BATCH_ID_3 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_3 {
        FINAL_TEST_BATCH_ID_3(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_ARRAY0(pub u32);
impl FINAL_TEST_BATCH_ID_ARRAY0 {
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
impl Default for FINAL_TEST_BATCH_ID_ARRAY0 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_ARRAY0 {
        FINAL_TEST_BATCH_ID_ARRAY0(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_ARRAY1(pub u32);
impl FINAL_TEST_BATCH_ID_ARRAY1 {
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
impl Default for FINAL_TEST_BATCH_ID_ARRAY1 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_ARRAY1 {
        FINAL_TEST_BATCH_ID_ARRAY1(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_ARRAY2(pub u32);
impl FINAL_TEST_BATCH_ID_ARRAY2 {
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
impl Default for FINAL_TEST_BATCH_ID_ARRAY2 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_ARRAY2 {
        FINAL_TEST_BATCH_ID_ARRAY2(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_ARRAY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_BATCH_ID_ARRAY3(pub u32);
impl FINAL_TEST_BATCH_ID_ARRAY3 {
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
impl Default for FINAL_TEST_BATCH_ID_ARRAY3 {
    #[inline(always)]
    fn default() -> FINAL_TEST_BATCH_ID_ARRAY3 {
        FINAL_TEST_BATCH_ID_ARRAY3(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_BATCH_ID_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_BATCH_ID_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_BATCH_ID_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_BATCH_ID_ARRAY3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_DATE(pub u32);
impl FINAL_TEST_DATE {
    #[doc = "DATE \\[stored as : year*10000+month*100+day\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn DATE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATE \\[stored as : year*10000+month*100+day\\]."]
    #[inline(always)]
    pub const fn set_DATE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FINAL_TEST_DATE {
    #[inline(always)]
    fn default() -> FINAL_TEST_DATE {
        FINAL_TEST_DATE(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_DATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_DATE")
            .field("DATE", &self.DATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_DATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FINAL_TEST_DATE {{ DATE: {=u32:?} }}", self.DATE())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_PROGRAM_VERSION(pub u32);
impl FINAL_TEST_PROGRAM_VERSION {
    #[doc = "PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn PROGRAM_VERSION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[inline(always)]
    pub const fn set_PROGRAM_VERSION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FINAL_TEST_PROGRAM_VERSION {
    #[inline(always)]
    fn default() -> FINAL_TEST_PROGRAM_VERSION {
        FINAL_TEST_PROGRAM_VERSION(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_PROGRAM_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_PROGRAM_VERSION")
            .field("PROGRAM_VERSION", &self.PROGRAM_VERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_PROGRAM_VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FINAL_TEST_PROGRAM_VERSION {{ PROGRAM_VERSION: {=u32:?} }}",
            self.PROGRAM_VERSION()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FINAL_TEST_TIME(pub u32);
impl FINAL_TEST_TIME {
    #[doc = "TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn TIME(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[inline(always)]
    pub const fn set_TIME(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FINAL_TEST_TIME {
    #[inline(always)]
    fn default() -> FINAL_TEST_TIME {
        FINAL_TEST_TIME(0)
    }
}
impl core::fmt::Debug for FINAL_TEST_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FINAL_TEST_TIME")
            .field("TIME", &self.TIME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FINAL_TEST_TIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FINAL_TEST_TIME {{ TIME: {=u32:?} }}", self.TIME())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASHSIZECFG(pub u32);
impl FLASHSIZECFG {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_CONFIGURATION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FLASH_CONFIGURATION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FLASHSIZECFG {
    #[inline(always)]
    fn default() -> FLASHSIZECFG {
        FLASHSIZECFG(0)
    }
}
impl core::fmt::Debug for FLASHSIZECFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASHSIZECFG")
            .field("FLASH_CONFIGURATION", &self.FLASH_CONFIGURATION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FLASHSIZECFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FLASHSIZECFG {{ FLASH_CONFIGURATION: {=u32:?} }}",
            self.FLASH_CONFIGURATION()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRO_192MHZ(pub u32);
impl FRO_192MHZ {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO192M_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO192M_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FRO192M_BIASTRIM\\[5:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO192M_BIASTRIM(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "FRO192M_BIASTRIM\\[5:0\\]."]
    #[inline(always)]
    pub const fn set_FRO192M_BIASTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "FRO192M_TEMPTRIM\\[6:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO192M_TEMPTRIM(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "FRO192M_TEMPTRIM\\[6:0\\]."]
    #[inline(always)]
    pub const fn set_FRO192M_TEMPTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "FRO192M_DACTRIM\\[7:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO192M_DACTRIM(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0xff;
        val as u8
    }
    #[doc = "FRO192M_DACTRIM\\[7:0\\]."]
    #[inline(always)]
    pub const fn set_FRO192M_DACTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 17usize)) | (((val as u32) & 0xff) << 17usize);
    }
}
impl Default for FRO_192MHZ {
    #[inline(always)]
    fn default() -> FRO_192MHZ {
        FRO_192MHZ(0)
    }
}
impl core::fmt::Debug for FRO_192MHZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO_192MHZ")
            .field("FRO192M_TRIM_VALID", &self.FRO192M_TRIM_VALID())
            .field("FRO192M_BIASTRIM", &self.FRO192M_BIASTRIM())
            .field("FRO192M_TEMPTRIM", &self.FRO192M_TEMPTRIM())
            .field("FRO192M_DACTRIM", &self.FRO192M_DACTRIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRO_192MHZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRO_192MHZ {{ FRO192M_TRIM_VALID: {=bool:?}, FRO192M_BIASTRIM: {=u8:?}, FRO192M_TEMPTRIM: {=u8:?}, FRO192M_DACTRIM: {=u8:?} }}",
            self.FRO192M_TRIM_VALID(),
            self.FRO192M_BIASTRIM(),
            self.FRO192M_TEMPTRIM(),
            self.FRO192M_DACTRIM()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRO_1MHZ(pub u32);
impl FRO_1MHZ {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO1M_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO1M_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frequency trimming bits."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO1M_FREQSEL(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Frequency trimming bits."]
    #[inline(always)]
    pub const fn set_FRO1M_FREQSEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for FRO_1MHZ {
    #[inline(always)]
    fn default() -> FRO_1MHZ {
        FRO_1MHZ(0)
    }
}
impl core::fmt::Debug for FRO_1MHZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO_1MHZ")
            .field("FRO1M_TRIM_VALID", &self.FRO1M_TRIM_VALID())
            .field("FRO1M_FREQSEL", &self.FRO1M_FREQSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRO_1MHZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRO_1MHZ {{ FRO1M_TRIM_VALID: {=bool:?}, FRO1M_FREQSEL: {=u8:?} }}",
            self.FRO1M_TRIM_VALID(),
            self.FRO1M_FREQSEL()
        )
    }
}
#[doc = "GPO0 register 0 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_0(pub u32);
impl GPO0_0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO32K_NTAT(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO32K_NTAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO32K_PTAT(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO32K_PTAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO32K_CAPCAL(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FRO32K_CAPCAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for GPO0_0 {
    #[inline(always)]
    fn default() -> GPO0_0 {
        GPO0_0(0)
    }
}
impl core::fmt::Debug for GPO0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_0")
            .field("FRO_TRIM_VALID", &self.FRO_TRIM_VALID())
            .field("FRO32K_NTAT", &self.FRO32K_NTAT())
            .field("FRO32K_PTAT", &self.FRO32K_PTAT())
            .field("FRO32K_CAPCAL", &self.FRO32K_CAPCAL())
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO0_0 {{ FRO_TRIM_VALID: {=bool:?}, FRO32K_NTAT: {=u8:?}, FRO32K_PTAT: {=u8:?}, FRO32K_CAPCAL: {=u16:?}, FIELD: {=u16:?} }}",
            self.FRO_TRIM_VALID(),
            self.FRO32K_NTAT(),
            self.FRO32K_PTAT(),
            self.FRO32K_CAPCAL(),
            self.FIELD()
        )
    }
}
#[doc = "GPO0 register 1 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_1(pub u32);
impl GPO0_1 {
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
impl Default for GPO0_1 {
    #[inline(always)]
    fn default() -> GPO0_1 {
        GPO0_1(0)
    }
}
impl core::fmt::Debug for GPO0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO0 register 2 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_2(pub u32);
impl GPO0_2 {
    #[doc = "00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSTEM_SPEED_CODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00 : FRO12MHz 01 : FRO24MHz 10 : FRO48MHz 11 : FRO96MHz."]
    #[inline(always)]
    pub const fn set_SYSTEM_SPEED_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_CTRL_OPMODE(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "00 : Delay Line 01 : RCLK (back up clock) 10 : PCLK (back up clock)."]
    #[inline(always)]
    pub const fn set_FLASH_CTRL_OPMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for GPO0_2 {
    #[inline(always)]
    fn default() -> GPO0_2 {
        GPO0_2(0)
    }
}
impl core::fmt::Debug for GPO0_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_2")
            .field("SYSTEM_SPEED_CODE", &self.SYSTEM_SPEED_CODE())
            .field("FLASH_CTRL_OPMODE", &self.FLASH_CTRL_OPMODE())
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO0_2 {{ SYSTEM_SPEED_CODE: {=u8:?}, FLASH_CTRL_OPMODE: {=u8:?}, FIELD: {=u32:?} }}",
            self.SYSTEM_SPEED_CODE(),
            self.FLASH_CTRL_OPMODE(),
            self.FIELD()
        )
    }
}
#[doc = "GPO0 register 3 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_3(pub u32);
impl GPO0_3 {
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
impl Default for GPO0_3 {
    #[inline(always)]
    fn default() -> GPO0_3 {
        GPO0_3(0)
    }
}
impl core::fmt::Debug for GPO0_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO0 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_ARRAY0(pub u32);
impl GPO0_ARRAY0 {
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
impl Default for GPO0_ARRAY0 {
    #[inline(always)]
    fn default() -> GPO0_ARRAY0 {
        GPO0_ARRAY0(0)
    }
}
impl core::fmt::Debug for GPO0_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO0 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_ARRAY1(pub u32);
impl GPO0_ARRAY1 {
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
impl Default for GPO0_ARRAY1 {
    #[inline(always)]
    fn default() -> GPO0_ARRAY1 {
        GPO0_ARRAY1(0)
    }
}
impl core::fmt::Debug for GPO0_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO0 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_ARRAY2(pub u32);
impl GPO0_ARRAY2 {
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
impl Default for GPO0_ARRAY2 {
    #[inline(always)]
    fn default() -> GPO0_ARRAY2 {
        GPO0_ARRAY2(0)
    }
}
impl core::fmt::Debug for GPO0_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO0 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO0_ARRAY3(pub u32);
impl GPO0_ARRAY3 {
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
impl Default for GPO0_ARRAY3 {
    #[inline(always)]
    fn default() -> GPO0_ARRAY3 {
        GPO0_ARRAY3(0)
    }
}
impl core::fmt::Debug for GPO0_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO0_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO0_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO0_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO1 register 0 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_0(pub u32);
impl GPO1_0 {
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[must_use]
    #[inline(always)]
    pub const fn FINAL_TEST_NOT_DONE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub const fn set_FINAL_TEST_NOT_DONE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 69 decimal)."]
    #[must_use]
    #[inline(always)]
    pub const fn PARTCONFIG(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x7f;
        val as u8
    }
    #[doc = "Device type number. (E.g : LPC5569 stored as 69 decimal)."]
    #[inline(always)]
    pub const fn set_PARTCONFIG(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVICE_TYPE_SEC(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Security device type: 0: LPC55xxx (Non Secure Familly) 1: LPC55Sxxx (Secure Familly)."]
    #[inline(always)]
    pub const fn set_DEVICE_TYPE_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_SIZE(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "SRAM_SIZE\\[3:0\\]: (For Niobe4) 0000 : 320 KB 0001 : 256 KB 0010 : 144 KB 0011 : 80 KB (For Niobe4 Mini) 0100 : 96 KB 0101 : 80 KB 0110 : 64 KB 0111 : 48 KB All others : RESERVED."]
    #[inline(always)]
    pub const fn set_SRAM_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SECURITY_EXTENSION_DISABLE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub const fn set_CPU0_SECURITY_EXTENSION_DISABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "ROM Revision-Minor \\[3:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_REVISION_MINOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Revision-Minor \\[3:0\\]."]
    #[inline(always)]
    pub const fn set_ROM_REVISION_MINOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "METAL REVISION ID\\[3:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn METAL_REVISION_ID(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "METAL REVISION ID\\[3:0\\]."]
    #[inline(always)]
    pub const fn set_METAL_REVISION_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPO1_0 {
    #[inline(always)]
    fn default() -> GPO1_0 {
        GPO1_0(0)
    }
}
impl core::fmt::Debug for GPO1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_0")
            .field("FINAL_TEST_NOT_DONE", &self.FINAL_TEST_NOT_DONE())
            .field("PARTCONFIG", &self.PARTCONFIG())
            .field("DEVICE_TYPE_SEC", &self.DEVICE_TYPE_SEC())
            .field("SRAM_SIZE", &self.SRAM_SIZE())
            .field(
                "CPU0_SECURITY_EXTENSION_DISABLE",
                &self.CPU0_SECURITY_EXTENSION_DISABLE(),
            )
            .field("FIELD", &self.FIELD())
            .field("ROM_REVISION_MINOR", &self.ROM_REVISION_MINOR())
            .field("METAL_REVISION_ID", &self.METAL_REVISION_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO1_0 {{ FINAL_TEST_NOT_DONE: {=u8:?}, PARTCONFIG: {=u8:?}, DEVICE_TYPE_SEC: {=bool:?}, SRAM_SIZE: {=u8:?}, CPU0_SECURITY_EXTENSION_DISABLE: {=u8:?}, FIELD: {=u8:?}, ROM_REVISION_MINOR: {=u8:?}, METAL_REVISION_ID: {=u8:?} }}",
            self.FINAL_TEST_NOT_DONE(),
            self.PARTCONFIG(),
            self.DEVICE_TYPE_SEC(),
            self.SRAM_SIZE(),
            self.CPU0_SECURITY_EXTENSION_DISABLE(),
            self.FIELD(),
            self.ROM_REVISION_MINOR(),
            self.METAL_REVISION_ID()
        )
    }
}
#[doc = "GPO1 register 1 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_1(pub u32);
impl GPO1_1 {
    #[doc = "ROM Patch Version \\[3:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn ROM_PATCH_VERSION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM Patch Version \\[3:0\\]."]
    #[inline(always)]
    pub const fn set_ROM_PATCH_VERSION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CUSTOMER REVISION ID\\[3:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn CUSTOMER_REVISION_ID(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CUSTOMER REVISION ID\\[3:0\\]."]
    #[inline(always)]
    pub const fn set_CUSTOMER_REVISION_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for GPO1_1 {
    #[inline(always)]
    fn default() -> GPO1_1 {
        GPO1_1(0)
    }
}
impl core::fmt::Debug for GPO1_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_1")
            .field("ROM_PATCH_VERSION", &self.ROM_PATCH_VERSION())
            .field("CUSTOMER_REVISION_ID", &self.CUSTOMER_REVISION_ID())
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO1_1 {{ ROM_PATCH_VERSION: {=u8:?}, CUSTOMER_REVISION_ID: {=u8:?}, FIELD: {=u32:?} }}",
            self.ROM_PATCH_VERSION(),
            self.CUSTOMER_REVISION_ID(),
            self.FIELD()
        )
    }
}
#[doc = "GPO1 register 2 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_2(pub u32);
impl GPO1_2 {
    #[doc = "High Voltage Stress: 0=not done; 1=done."]
    #[must_use]
    #[inline(always)]
    pub const fn HVST(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "High Voltage Stress: 0=not done; 1=done."]
    #[inline(always)]
    pub const fn set_HVST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for GPO1_2 {
    #[inline(always)]
    fn default() -> GPO1_2 {
        GPO1_2(0)
    }
}
impl core::fmt::Debug for GPO1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_2")
            .field("HVST", &self.HVST())
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO1_2 {{ HVST: {=bool:?}, FIELD: {=u32:?} }}",
            self.HVST(),
            self.FIELD()
        )
    }
}
#[doc = "GPO1 register 3 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_3(pub u32);
impl GPO1_3 {
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
impl Default for GPO1_3 {
    #[inline(always)]
    fn default() -> GPO1_3 {
        GPO1_3(0)
    }
}
impl core::fmt::Debug for GPO1_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO1_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO1 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_ARRAY0(pub u32);
impl GPO1_ARRAY0 {
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
impl Default for GPO1_ARRAY0 {
    #[inline(always)]
    fn default() -> GPO1_ARRAY0 {
        GPO1_ARRAY0(0)
    }
}
impl core::fmt::Debug for GPO1_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO1_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO1 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_ARRAY1(pub u32);
impl GPO1_ARRAY1 {
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
impl Default for GPO1_ARRAY1 {
    #[inline(always)]
    fn default() -> GPO1_ARRAY1 {
        GPO1_ARRAY1(0)
    }
}
impl core::fmt::Debug for GPO1_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO1_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO1 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_ARRAY2(pub u32);
impl GPO1_ARRAY2 {
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
impl Default for GPO1_ARRAY2 {
    #[inline(always)]
    fn default() -> GPO1_ARRAY2 {
        GPO1_ARRAY2(0)
    }
}
impl core::fmt::Debug for GPO1_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO1_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO1 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO1_ARRAY3(pub u32);
impl GPO1_ARRAY3 {
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
impl Default for GPO1_ARRAY3 {
    #[inline(always)]
    fn default() -> GPO1_ARRAY3 {
        GPO1_ARRAY3(0)
    }
}
impl core::fmt::Debug for GPO1_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO1_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO1_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO1_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 register 0 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_0(pub u32);
impl GPO2_0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn USBHS_PHY_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_USBHS_PHY_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USB_REG_ENV_TAIL_ADJ_VD(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USB_REG_ENV_TAIL_ADJ_VD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USBPHY_TX_D_CAL(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USBPHY_TX_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USBPHY_TX_CAL45DP(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USBPHY_TX_CAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USBPHY_TX_CAL45DN(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USBPHY_TX_CAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USB2_REFBIAS_TST(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USB2_REFBIAS_TST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_USB2_REFBIAS_VBGADJ(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_USB2_REFBIAS_VBGADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIM_PLL_CTRL0_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_TRIM_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "(For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_SIZE(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "(For Niobe4) 000 : 640 KB 001 : 512 KB 010 : 256 KB 011 : 128 KB 100 : 0 KB All others : RESERVED (For Niobe4 Mini) FLASH_SIZE\\[2:0\\] 000 : 256 KB 001 : 128 KB 010 : 80 KB (reserved) 011 : 64 KB 100 : 0 kB (reserved) All others : RESERVED."]
    #[inline(always)]
    pub const fn set_FLASH_SIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU0_SECURITY_EXTENSION_DISABLE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "CPU0_SECURITY_EXTENSION_DISABLE\\[3:0\\]: 1010 : CPU0 Security Extension is disabled. All Other values: CPU0 Security Extension is enabled."]
    #[inline(always)]
    pub const fn set_CPU0_SECURITY_EXTENSION_DISABLE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPO2_0 {
    #[inline(always)]
    fn default() -> GPO2_0 {
        GPO2_0(0)
    }
}
impl core::fmt::Debug for GPO2_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_0")
            .field("USBHS_PHY_TRIM_VALID", &self.USBHS_PHY_TRIM_VALID())
            .field(
                "TRIM_USB_REG_ENV_TAIL_ADJ_VD",
                &self.TRIM_USB_REG_ENV_TAIL_ADJ_VD(),
            )
            .field("TRIM_USBPHY_TX_D_CAL", &self.TRIM_USBPHY_TX_D_CAL())
            .field("TRIM_USBPHY_TX_CAL45DP", &self.TRIM_USBPHY_TX_CAL45DP())
            .field("TRIM_USBPHY_TX_CAL45DN", &self.TRIM_USBPHY_TX_CAL45DN())
            .field("TRIM_USB2_REFBIAS_TST", &self.TRIM_USB2_REFBIAS_TST())
            .field("TRIM_USB2_REFBIAS_VBGADJ", &self.TRIM_USB2_REFBIAS_VBGADJ())
            .field("TRIM_PLL_CTRL0_DIV_SEL", &self.TRIM_PLL_CTRL0_DIV_SEL())
            .field("FLASH_SIZE", &self.FLASH_SIZE())
            .field(
                "CPU0_SECURITY_EXTENSION_DISABLE",
                &self.CPU0_SECURITY_EXTENSION_DISABLE(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO2_0 {{ USBHS_PHY_TRIM_VALID: {=bool:?}, TRIM_USB_REG_ENV_TAIL_ADJ_VD: {=u8:?}, TRIM_USBPHY_TX_D_CAL: {=u8:?}, TRIM_USBPHY_TX_CAL45DP: {=u8:?}, TRIM_USBPHY_TX_CAL45DN: {=u8:?}, TRIM_USB2_REFBIAS_TST: {=u8:?}, TRIM_USB2_REFBIAS_VBGADJ: {=u8:?}, TRIM_PLL_CTRL0_DIV_SEL: {=u8:?}, FLASH_SIZE: {=u8:?}, CPU0_SECURITY_EXTENSION_DISABLE: {=u8:?} }}",
            self.USBHS_PHY_TRIM_VALID(),
            self.TRIM_USB_REG_ENV_TAIL_ADJ_VD(),
            self.TRIM_USBPHY_TX_D_CAL(),
            self.TRIM_USBPHY_TX_CAL45DP(),
            self.TRIM_USBPHY_TX_CAL45DN(),
            self.TRIM_USB2_REFBIAS_TST(),
            self.TRIM_USB2_REFBIAS_VBGADJ(),
            self.TRIM_PLL_CTRL0_DIV_SEL(),
            self.FLASH_SIZE(),
            self.CPU0_SECURITY_EXTENSION_DISABLE()
        )
    }
}
#[doc = "GPO2 register 1 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_1(pub u32);
impl GPO2_1 {
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
impl Default for GPO2_1 {
    #[inline(always)]
    fn default() -> GPO2_1 {
        GPO2_1(0)
    }
}
impl core::fmt::Debug for GPO2_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 register 2 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_2(pub u32);
impl GPO2_2 {
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
impl Default for GPO2_2 {
    #[inline(always)]
    fn default() -> GPO2_2 {
        GPO2_2(0)
    }
}
impl core::fmt::Debug for GPO2_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 register 3 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_3(pub u32);
impl GPO2_3 {
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
impl Default for GPO2_3 {
    #[inline(always)]
    fn default() -> GPO2_3 {
        GPO2_3(0)
    }
}
impl core::fmt::Debug for GPO2_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_ARRAY0(pub u32);
impl GPO2_ARRAY0 {
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
impl Default for GPO2_ARRAY0 {
    #[inline(always)]
    fn default() -> GPO2_ARRAY0 {
        GPO2_ARRAY0(0)
    }
}
impl core::fmt::Debug for GPO2_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_ARRAY1(pub u32);
impl GPO2_ARRAY1 {
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
impl Default for GPO2_ARRAY1 {
    #[inline(always)]
    fn default() -> GPO2_ARRAY1 {
        GPO2_ARRAY1(0)
    }
}
impl core::fmt::Debug for GPO2_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_ARRAY2(pub u32);
impl GPO2_ARRAY2 {
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
impl Default for GPO2_ARRAY2 {
    #[inline(always)]
    fn default() -> GPO2_ARRAY2 {
        GPO2_ARRAY2(0)
    }
}
impl core::fmt::Debug for GPO2_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO2 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO2_ARRAY3(pub u32);
impl GPO2_ARRAY3 {
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
impl Default for GPO2_ARRAY3 {
    #[inline(always)]
    fn default() -> GPO2_ARRAY3 {
        GPO2_ARRAY3(0)
    }
}
impl core::fmt::Debug for GPO2_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO2_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO2_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO2_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 register 0 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_0(pub u32);
impl GPO3_0 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BIAS_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_AUX_BIAS_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BIAS_ITRIM(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_AUX_BIAS_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BIAS_PTAT_ITRIM(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_AUX_BIAS_PTAT_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BIAS_VREF1_VTRIM(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_AUX_BIAS_VREF1_VTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn AUX_BIAS_VREF1_VCURVE_TRIM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_AUX_BIAS_VREF1_VCURVE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn FIELD(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x3f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_FIELD(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 19usize)) | (((val as u32) & 0x3f) << 19usize);
    }
    #[doc = "ModelNumber extension\\[2:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn MODELNUM_EXTENSION(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "ModelNumber extension\\[2:0\\]."]
    #[inline(always)]
    pub const fn set_MODELNUM_EXTENSION(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[must_use]
    #[inline(always)]
    pub const fn FINAL_TEST_NOT_DONE(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "FINAL_TEST_NOT_DONE\\[3:0\\]: 1010 : Final Test Not Done. All Other values: Final Test Done."]
    #[inline(always)]
    pub const fn set_FINAL_TEST_NOT_DONE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GPO3_0 {
    #[inline(always)]
    fn default() -> GPO3_0 {
        GPO3_0(0)
    }
}
impl core::fmt::Debug for GPO3_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_0")
            .field("AUX_BIAS_TRIM_VALID", &self.AUX_BIAS_TRIM_VALID())
            .field("AUX_BIAS_ITRIM", &self.AUX_BIAS_ITRIM())
            .field("AUX_BIAS_PTAT_ITRIM", &self.AUX_BIAS_PTAT_ITRIM())
            .field("AUX_BIAS_VREF1_VTRIM", &self.AUX_BIAS_VREF1_VTRIM())
            .field(
                "AUX_BIAS_VREF1_VCURVE_TRIM",
                &self.AUX_BIAS_VREF1_VCURVE_TRIM(),
            )
            .field("FIELD", &self.FIELD())
            .field("MODELNUM_EXTENSION", &self.MODELNUM_EXTENSION())
            .field("FINAL_TEST_NOT_DONE", &self.FINAL_TEST_NOT_DONE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPO3_0 {{ AUX_BIAS_TRIM_VALID: {=bool:?}, AUX_BIAS_ITRIM: {=u8:?}, AUX_BIAS_PTAT_ITRIM: {=u8:?}, AUX_BIAS_VREF1_VTRIM: {=u8:?}, AUX_BIAS_VREF1_VCURVE_TRIM: {=u8:?}, FIELD: {=u8:?}, MODELNUM_EXTENSION: {=u8:?}, FINAL_TEST_NOT_DONE: {=u8:?} }}",
            self.AUX_BIAS_TRIM_VALID(),
            self.AUX_BIAS_ITRIM(),
            self.AUX_BIAS_PTAT_ITRIM(),
            self.AUX_BIAS_VREF1_VTRIM(),
            self.AUX_BIAS_VREF1_VCURVE_TRIM(),
            self.FIELD(),
            self.MODELNUM_EXTENSION(),
            self.FINAL_TEST_NOT_DONE()
        )
    }
}
#[doc = "GPO3 register 1 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_1(pub u32);
impl GPO3_1 {
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
impl Default for GPO3_1 {
    #[inline(always)]
    fn default() -> GPO3_1 {
        GPO3_1(0)
    }
}
impl core::fmt::Debug for GPO3_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 register 2 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_2(pub u32);
impl GPO3_2 {
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
impl Default for GPO3_2 {
    #[inline(always)]
    fn default() -> GPO3_2 {
        GPO3_2(0)
    }
}
impl core::fmt::Debug for GPO3_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 register 3 description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_3(pub u32);
impl GPO3_3 {
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
impl Default for GPO3_3 {
    #[inline(always)]
    fn default() -> GPO3_3 {
        GPO3_3(0)
    }
}
impl core::fmt::Debug for GPO3_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_ARRAY0(pub u32);
impl GPO3_ARRAY0 {
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
impl Default for GPO3_ARRAY0 {
    #[inline(always)]
    fn default() -> GPO3_ARRAY0 {
        GPO3_ARRAY0(0)
    }
}
impl core::fmt::Debug for GPO3_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_ARRAY1(pub u32);
impl GPO3_ARRAY1 {
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
impl Default for GPO3_ARRAY1 {
    #[inline(always)]
    fn default() -> GPO3_ARRAY1 {
        GPO3_ARRAY1(0)
    }
}
impl core::fmt::Debug for GPO3_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_ARRAY2(pub u32);
impl GPO3_ARRAY2 {
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
impl Default for GPO3_ARRAY2 {
    #[inline(always)]
    fn default() -> GPO3_ARRAY2 {
        GPO3_ARRAY2(0)
    }
}
impl core::fmt::Debug for GPO3_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "GPO3 array description."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO3_ARRAY3(pub u32);
impl GPO3_ARRAY3 {
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
impl Default for GPO3_ARRAY3 {
    #[inline(always)]
    fn default() -> GPO3_ARRAY3 {
        GPO3_ARRAY3(0)
    }
}
impl core::fmt::Debug for GPO3_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO3_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO3_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO3_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_0(pub u32);
impl GPO_CHECKSUM_0 {
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
impl Default for GPO_CHECKSUM_0 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_0 {
        GPO_CHECKSUM_0(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_1(pub u32);
impl GPO_CHECKSUM_1 {
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
impl Default for GPO_CHECKSUM_1 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_1 {
        GPO_CHECKSUM_1(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_2(pub u32);
impl GPO_CHECKSUM_2 {
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
impl Default for GPO_CHECKSUM_2 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_2 {
        GPO_CHECKSUM_2(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_3(pub u32);
impl GPO_CHECKSUM_3 {
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
impl Default for GPO_CHECKSUM_3 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_3 {
        GPO_CHECKSUM_3(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_ARRAY0(pub u32);
impl GPO_CHECKSUM_ARRAY0 {
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
impl Default for GPO_CHECKSUM_ARRAY0 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_ARRAY0 {
        GPO_CHECKSUM_ARRAY0(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_ARRAY1(pub u32);
impl GPO_CHECKSUM_ARRAY1 {
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
impl Default for GPO_CHECKSUM_ARRAY1 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_ARRAY1 {
        GPO_CHECKSUM_ARRAY1(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_ARRAY2(pub u32);
impl GPO_CHECKSUM_ARRAY2 {
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
impl Default for GPO_CHECKSUM_ARRAY2 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_ARRAY2 {
        GPO_CHECKSUM_ARRAY2(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "checksum of the GPO data in words \\[3:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPO_CHECKSUM_ARRAY3(pub u32);
impl GPO_CHECKSUM_ARRAY3 {
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
impl Default for GPO_CHECKSUM_ARRAY3 {
    #[inline(always)]
    fn default() -> GPO_CHECKSUM_ARRAY3 {
        GPO_CHECKSUM_ARRAY3(0)
    }
}
impl core::fmt::Debug for GPO_CHECKSUM_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPO_CHECKSUM_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPO_CHECKSUM_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPO_CHECKSUM_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LDO_AO(pub u32);
impl LDO_AO {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_ACTIVE_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVE_TRIM(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_ACTIVE_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DSLP_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DSLP_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DSLP_TRIM(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DSLP_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PDWN_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PDWN_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PDWN_TRIM(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PDWN_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDW_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DPDW_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DPDW_TRIM(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DPDW_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for LDO_AO {
    #[inline(always)]
    fn default() -> LDO_AO {
        LDO_AO(0)
    }
}
impl core::fmt::Debug for LDO_AO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_AO")
            .field("ACTIVE_TRIM_VALID", &self.ACTIVE_TRIM_VALID())
            .field("ACTIVE_TRIM", &self.ACTIVE_TRIM())
            .field("DSLP_TRIM_VALID", &self.DSLP_TRIM_VALID())
            .field("DSLP_TRIM", &self.DSLP_TRIM())
            .field("PDWN_TRIM_VALID", &self.PDWN_TRIM_VALID())
            .field("PDWN_TRIM", &self.PDWN_TRIM())
            .field("DPDW_TRIM_VALID", &self.DPDW_TRIM_VALID())
            .field("DPDW_TRIM", &self.DPDW_TRIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LDO_AO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LDO_AO {{ ACTIVE_TRIM_VALID: {=bool:?}, ACTIVE_TRIM: {=u8:?}, DSLP_TRIM_VALID: {=bool:?}, DSLP_TRIM: {=u8:?}, PDWN_TRIM_VALID: {=bool:?}, PDWN_TRIM: {=u8:?}, DPDW_TRIM_VALID: {=bool:?}, DPDW_TRIM: {=u8:?} }}",
            self.ACTIVE_TRIM_VALID(),
            self.ACTIVE_TRIM(),
            self.DSLP_TRIM_VALID(),
            self.DSLP_TRIM(),
            self.PDWN_TRIM_VALID(),
            self.PDWN_TRIM(),
            self.DPDW_TRIM_VALID(),
            self.DPDW_TRIM()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[255:128\\])."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NXP_DEVICE_CERTIFICATE_0(pub u32);
impl NXP_DEVICE_CERTIFICATE_0 {
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
impl Default for NXP_DEVICE_CERTIFICATE_0 {
    #[inline(always)]
    fn default() -> NXP_DEVICE_CERTIFICATE_0 {
        NXP_DEVICE_CERTIFICATE_0(0)
    }
}
impl core::fmt::Debug for NXP_DEVICE_CERTIFICATE_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NXP_DEVICE_CERTIFICATE_0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NXP_DEVICE_CERTIFICATE_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NXP_DEVICE_CERTIFICATE_0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - r\\[127:0\\])."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NXP_DEVICE_CERTIFICATE_1(pub u32);
impl NXP_DEVICE_CERTIFICATE_1 {
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
impl Default for NXP_DEVICE_CERTIFICATE_1 {
    #[inline(always)]
    fn default() -> NXP_DEVICE_CERTIFICATE_1 {
        NXP_DEVICE_CERTIFICATE_1(0)
    }
}
impl core::fmt::Debug for NXP_DEVICE_CERTIFICATE_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NXP_DEVICE_CERTIFICATE_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NXP_DEVICE_CERTIFICATE_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NXP_DEVICE_CERTIFICATE_1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[255:128\\])."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NXP_DEVICE_CERTIFICATE_2(pub u32);
impl NXP_DEVICE_CERTIFICATE_2 {
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
impl Default for NXP_DEVICE_CERTIFICATE_2 {
    #[inline(always)]
    fn default() -> NXP_DEVICE_CERTIFICATE_2 {
        NXP_DEVICE_CERTIFICATE_2(0)
    }
}
impl core::fmt::Debug for NXP_DEVICE_CERTIFICATE_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NXP_DEVICE_CERTIFICATE_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NXP_DEVICE_CERTIFICATE_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NXP_DEVICE_CERTIFICATE_2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "NXP Device Certificate (ECDSA_sign - s\\[127:0\\])."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NXP_DEVICE_CERTIFICATE_3(pub u32);
impl NXP_DEVICE_CERTIFICATE_3 {
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
impl Default for NXP_DEVICE_CERTIFICATE_3 {
    #[inline(always)]
    fn default() -> NXP_DEVICE_CERTIFICATE_3 {
        NXP_DEVICE_CERTIFICATE_3(0)
    }
}
impl core::fmt::Debug for NXP_DEVICE_CERTIFICATE_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NXP_DEVICE_CERTIFICATE_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NXP_DEVICE_CERTIFICATE_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NXP_DEVICE_CERTIFICATE_3 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NXP_DEVICE_PRIVATE_KEY(pub u32);
impl NXP_DEVICE_PRIVATE_KEY {
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
impl Default for NXP_DEVICE_PRIVATE_KEY {
    #[inline(always)]
    fn default() -> NXP_DEVICE_PRIVATE_KEY {
        NXP_DEVICE_PRIVATE_KEY(0)
    }
}
impl core::fmt::Debug for NXP_DEVICE_PRIVATE_KEY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NXP_DEVICE_PRIVATE_KEY")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NXP_DEVICE_PRIVATE_KEY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NXP_DEVICE_PRIVATE_KEY {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PERIPHENCFG(pub u32);
impl PERIPHENCFG {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PERIPHERAL_CONFIGURATION(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PERIPHERAL_CONFIGURATION(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn CPU1_ENABLE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_CPU1_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PERIPHENCFG {
    #[inline(always)]
    fn default() -> PERIPHENCFG {
        PERIPHENCFG(0)
    }
}
impl core::fmt::Debug for PERIPHENCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIPHENCFG")
            .field("PERIPHERAL_CONFIGURATION", &self.PERIPHERAL_CONFIGURATION())
            .field("CPU1_ENABLE", &self.CPU1_ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PERIPHENCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PERIPHENCFG {{ PERIPHERAL_CONFIGURATION: {=u16:?}, CPU1_ENABLE: {=bool:?} }}",
            self.PERIPHERAL_CONFIGURATION(),
            self.CPU1_ENABLE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PUF_SRAM(pub u32);
impl PUF_SRAM {
    #[doc = "1: PUF_SRAM is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn PUF_SRAM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: PUF_SRAM is valid."]
    #[inline(always)]
    pub const fn set_PUF_SRAM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF SRAM Controller operating mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PUF SRAM Controller operating mode."]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
    #[doc = "Source Biasing voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn SMB(&self) -> super::vals::SMB {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::SMB::from_bits(val as u8)
    }
    #[doc = "Source Biasing voltage."]
    #[inline(always)]
    pub const fn set_SMB(&mut self, val: super::vals::SMB) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Read Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn RM(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "Read Margin control settings."]
    #[inline(always)]
    pub const fn set_RM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "Write Margin control settings."]
    #[must_use]
    #[inline(always)]
    pub const fn WM(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Write Margin control settings."]
    #[inline(always)]
    pub const fn set_WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Write read margin enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WRME(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Write read margin enable."]
    #[inline(always)]
    pub const fn set_WRME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SRAM Read Assist Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RAEN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Read Assist Enable."]
    #[inline(always)]
    pub const fn set_RAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SRAM Read Assist settings."]
    #[must_use]
    #[inline(always)]
    pub const fn RAM(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "SRAM Read Assist settings."]
    #[inline(always)]
    pub const fn set_RAM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "SRAM Write Assist Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WAEN(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SRAM Write Assist Enable."]
    #[inline(always)]
    pub const fn set_WAEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SRAM Write Assist settings."]
    #[must_use]
    #[inline(always)]
    pub const fn WAM(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "SRAM Write Assist settings."]
    #[inline(always)]
    pub const fn set_WAM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "STBP."]
    #[must_use]
    #[inline(always)]
    pub const fn STBP(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "STBP."]
    #[inline(always)]
    pub const fn set_STBP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for PUF_SRAM {
    #[inline(always)]
    fn default() -> PUF_SRAM {
        PUF_SRAM(0)
    }
}
impl core::fmt::Debug for PUF_SRAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUF_SRAM")
            .field("PUF_SRAM_VALID", &self.PUF_SRAM_VALID())
            .field("mode", &self.mode())
            .field("ckgating", &self.ckgating())
            .field("SMB", &self.SMB())
            .field("RM", &self.RM())
            .field("WM", &self.WM())
            .field("WRME", &self.WRME())
            .field("RAEN", &self.RAEN())
            .field("RAM", &self.RAM())
            .field("WAEN", &self.WAEN())
            .field("WAM", &self.WAM())
            .field("STBP", &self.STBP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PUF_SRAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PUF_SRAM {{ PUF_SRAM_VALID: {=bool:?}, mode: {=bool:?}, ckgating: {=bool:?}, SMB: {:?}, RM: {=u8:?}, WM: {=u8:?}, WRME: {=bool:?}, RAEN: {=bool:?}, RAM: {=u8:?}, WAEN: {=bool:?}, WAM: {=u8:?}, STBP: {=bool:?} }}",
            self.PUF_SRAM_VALID(),
            self.mode(),
            self.ckgating(),
            self.SMB(),
            self.RM(),
            self.WM(),
            self.WRME(),
            self.RAEN(),
            self.RAM(),
            self.WAEN(),
            self.WAM(),
            self.STBP()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_ARRAY0(pub u32);
impl PVT_MONITOR_0_ARRAY0 {
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
impl Default for PVT_MONITOR_0_ARRAY0 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_ARRAY0 {
        PVT_MONITOR_0_ARRAY0(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_ARRAY1(pub u32);
impl PVT_MONITOR_0_ARRAY1 {
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
impl Default for PVT_MONITOR_0_ARRAY1 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_ARRAY1 {
        PVT_MONITOR_0_ARRAY1(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_ARRAY2(pub u32);
impl PVT_MONITOR_0_ARRAY2 {
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
impl Default for PVT_MONITOR_0_ARRAY2 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_ARRAY2 {
        PVT_MONITOR_0_ARRAY2(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_ARRAY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_DELAYS_LSB(pub u32);
impl PVT_MONITOR_0_DELAYS_LSB {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DELAY_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_1(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for PVT_MONITOR_0_DELAYS_LSB {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_DELAYS_LSB {
        PVT_MONITOR_0_DELAYS_LSB(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_DELAYS_LSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_DELAYS_LSB")
            .field("DELAY_VALID", &self.DELAY_VALID())
            .field("DELAY_0", &self.DELAY_0())
            .field("DELAY_1", &self.DELAY_1())
            .field("DELAY_2", &self.DELAY_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_DELAYS_LSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_DELAYS_LSB {{ DELAY_VALID: {=bool:?}, DELAY_0: {=u16:?}, DELAY_1: {=u16:?}, DELAY_2: {=u16:?} }}",
            self.DELAY_VALID(),
            self.DELAY_0(),
            self.DELAY_1(),
            self.DELAY_2()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_DELAYS_MSB(pub u32);
impl PVT_MONITOR_0_DELAYS_MSB {
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_4(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_5(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for PVT_MONITOR_0_DELAYS_MSB {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_DELAYS_MSB {
        PVT_MONITOR_0_DELAYS_MSB(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_DELAYS_MSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_DELAYS_MSB")
            .field("DELAY_3", &self.DELAY_3())
            .field("DELAY_4", &self.DELAY_4())
            .field("DELAY_5", &self.DELAY_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_DELAYS_MSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_DELAYS_MSB {{ DELAY_3: {=u16:?}, DELAY_4: {=u16:?}, DELAY_5: {=u16:?} }}",
            self.DELAY_3(),
            self.DELAY_4(),
            self.DELAY_5()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_0_RINGO(pub u32);
impl PVT_MONITOR_0_RINGO {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_RINGO_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_FREQ_HZ(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_RINGO_FREQ_HZ(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for PVT_MONITOR_0_RINGO {
    #[inline(always)]
    fn default() -> PVT_MONITOR_0_RINGO {
        PVT_MONITOR_0_RINGO(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_0_RINGO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_0_RINGO")
            .field("RINGO_VALID", &self.RINGO_VALID())
            .field("RINGO_FREQ_HZ", &self.RINGO_FREQ_HZ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_0_RINGO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_0_RINGO {{ RINGO_VALID: {=bool:?}, RINGO_FREQ_HZ: {=u32:?} }}",
            self.RINGO_VALID(),
            self.RINGO_FREQ_HZ()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_ARRAY0(pub u32);
impl PVT_MONITOR_1_ARRAY0 {
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
impl Default for PVT_MONITOR_1_ARRAY0 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_ARRAY0 {
        PVT_MONITOR_1_ARRAY0(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_ARRAY0 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_ARRAY1(pub u32);
impl PVT_MONITOR_1_ARRAY1 {
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
impl Default for PVT_MONITOR_1_ARRAY1 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_ARRAY1 {
        PVT_MONITOR_1_ARRAY1(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_ARRAY1 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_ARRAY2(pub u32);
impl PVT_MONITOR_1_ARRAY2 {
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
impl Default for PVT_MONITOR_1_ARRAY2 {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_ARRAY2 {
        PVT_MONITOR_1_ARRAY2(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_ARRAY2 {{ FIELD: {=u32:?} }}",
            self.FIELD()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_DELAYS_LSB(pub u32);
impl PVT_MONITOR_1_DELAYS_LSB {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_DELAY_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_0(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_1(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 11usize)) | (((val as u32) & 0x03ff) << 11usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_2(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
}
impl Default for PVT_MONITOR_1_DELAYS_LSB {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_DELAYS_LSB {
        PVT_MONITOR_1_DELAYS_LSB(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_DELAYS_LSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_DELAYS_LSB")
            .field("DELAY_VALID", &self.DELAY_VALID())
            .field("DELAY_0", &self.DELAY_0())
            .field("DELAY_1", &self.DELAY_1())
            .field("DELAY_2", &self.DELAY_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_DELAYS_LSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_DELAYS_LSB {{ DELAY_VALID: {=bool:?}, DELAY_0: {=u16:?}, DELAY_1: {=u16:?}, DELAY_2: {=u16:?} }}",
            self.DELAY_VALID(),
            self.DELAY_0(),
            self.DELAY_1(),
            self.DELAY_2()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_DELAYS_MSB(pub u32);
impl PVT_MONITOR_1_DELAYS_MSB {
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_4(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[doc = "Delay in us."]
    #[must_use]
    #[inline(always)]
    pub const fn DELAY_5(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Delay in us."]
    #[inline(always)]
    pub const fn set_DELAY_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for PVT_MONITOR_1_DELAYS_MSB {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_DELAYS_MSB {
        PVT_MONITOR_1_DELAYS_MSB(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_DELAYS_MSB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_DELAYS_MSB")
            .field("DELAY_3", &self.DELAY_3())
            .field("DELAY_4", &self.DELAY_4())
            .field("DELAY_5", &self.DELAY_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_DELAYS_MSB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_DELAYS_MSB {{ DELAY_3: {=u16:?}, DELAY_4: {=u16:?}, DELAY_5: {=u16:?} }}",
            self.DELAY_3(),
            self.DELAY_4(),
            self.DELAY_5()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVT_MONITOR_1_RINGO(pub u32);
impl PVT_MONITOR_1_RINGO {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_RINGO_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_FREQ_HZ(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_RINGO_FREQ_HZ(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for PVT_MONITOR_1_RINGO {
    #[inline(always)]
    fn default() -> PVT_MONITOR_1_RINGO {
        PVT_MONITOR_1_RINGO(0)
    }
}
impl core::fmt::Debug for PVT_MONITOR_1_RINGO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_1_RINGO")
            .field("RINGO_VALID", &self.RINGO_VALID())
            .field("RINGO_FREQ_HZ", &self.RINGO_FREQ_HZ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PVT_MONITOR_1_RINGO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PVT_MONITOR_1_RINGO {{ RINGO_VALID: {=bool:?}, RINGO_FREQ_HZ: {=u32:?} }}",
            self.RINGO_VALID(),
            self.RINGO_FREQ_HZ()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMSIZECFG(pub u32);
impl RAMSIZECFG {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn SRAM_CONFIGURATION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_SRAM_CONFIGURATION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RAMSIZECFG {
    #[inline(always)]
    fn default() -> RAMSIZECFG {
        RAMSIZECFG(0)
    }
}
impl core::fmt::Debug for RAMSIZECFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMSIZECFG")
            .field("SRAM_CONFIGURATION", &self.SRAM_CONFIGURATION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMSIZECFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAMSIZECFG {{ SRAM_CONFIGURATION: {=u32:?} }}",
            self.SRAM_CONFIGURATION()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO_0(pub u32);
impl RINGO_0 {
    #[doc = "1: RINGO_0_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_0_CTRL_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_0_CTRL is valid."]
    #[inline(always)]
    pub const fn set_RINGO_0_CTRL_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_0_CTRL(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_0_CTRL = ANACTRL->RINGO0_CTRL\\[30:0\\]."]
    #[inline(always)]
    pub const fn set_RINGO_0_CTRL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RINGO_0 {
    #[inline(always)]
    fn default() -> RINGO_0 {
        RINGO_0(0)
    }
}
impl core::fmt::Debug for RINGO_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO_0")
            .field("RINGO_0_CTRL_VALID", &self.RINGO_0_CTRL_VALID())
            .field("RINGO_0_CTRL", &self.RINGO_0_CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO_0 {{ RINGO_0_CTRL_VALID: {=bool:?}, RINGO_0_CTRL: {=u32:?} }}",
            self.RINGO_0_CTRL_VALID(),
            self.RINGO_0_CTRL()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO_1(pub u32);
impl RINGO_1 {
    #[doc = "1: RINGO_1_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_1_CTRL_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_1_CTRL is valid."]
    #[inline(always)]
    pub const fn set_RINGO_1_CTRL_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_1_CTRL(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_1_CTRL = ANACTRL->RINGO1_CTRL\\[30:0\\]."]
    #[inline(always)]
    pub const fn set_RINGO_1_CTRL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RINGO_1 {
    #[inline(always)]
    fn default() -> RINGO_1 {
        RINGO_1(0)
    }
}
impl core::fmt::Debug for RINGO_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO_1")
            .field("RINGO_1_CTRL_VALID", &self.RINGO_1_CTRL_VALID())
            .field("RINGO_1_CTRL", &self.RINGO_1_CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO_1 {{ RINGO_1_CTRL_VALID: {=bool:?}, RINGO_1_CTRL: {=u32:?} }}",
            self.RINGO_1_CTRL_VALID(),
            self.RINGO_1_CTRL()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO_2(pub u32);
impl RINGO_2 {
    #[doc = "1: RINGO_2_CTRL is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_2_CTRL_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: RINGO_2_CTRL is valid."]
    #[inline(always)]
    pub const fn set_RINGO_2_CTRL_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn RINGO_2_CTRL(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "To copy RINGO_2_CTRL = ANACTRL->RINGO2_CTRL\\[30:0\\]."]
    #[inline(always)]
    pub const fn set_RINGO_2_CTRL(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RINGO_2 {
    #[inline(always)]
    fn default() -> RINGO_2 {
        RINGO_2(0)
    }
}
impl core::fmt::Debug for RINGO_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO_2")
            .field("RINGO_2_CTRL_VALID", &self.RINGO_2_CTRL_VALID())
            .field("RINGO_2_CTRL", &self.RINGO_2_CTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO_2 {{ RINGO_2_CTRL_VALID: {=bool:?}, RINGO_2_CTRL: {=u32:?} }}",
            self.RINGO_2_CTRL_VALID(),
            self.RINGO_2_CTRL()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SDIO_DELAY(pub u32);
impl SDIO_DELAY {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_0_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_SDIO_0_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDIO_0_DELAY (unit: 100 ps)."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIO_0_DELAY(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[doc = "SDIO_0_DELAY (unit: 100 ps)."]
    #[inline(always)]
    pub const fn set_SDIO_0_DELAY(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
}
impl Default for SDIO_DELAY {
    #[inline(always)]
    fn default() -> SDIO_DELAY {
        SDIO_DELAY(0)
    }
}
impl core::fmt::Debug for SDIO_DELAY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_DELAY")
            .field("SDIO_0_VALID", &self.SDIO_0_VALID())
            .field("SDIO_0_DELAY", &self.SDIO_0_DELAY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SDIO_DELAY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SDIO_DELAY {{ SDIO_0_VALID: {=bool:?}, SDIO_0_DELAY: {=u16:?} }}",
            self.SDIO_0_VALID(),
            self.SDIO_0_DELAY()
        )
    }
}
#[doc = "SHA-256 DIGEST (9EC00 - 9FDBC) ROM Patch Area + NXP Area (IMPORTANT NOTE: Pages used for Repair (N-8 to N-3) are excluded from the computation) SHA256_DIGESTindex for DIGEST\\[((index * 32) + 31):(index * 32)\\]."]
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
pub struct TEMP_SENS_OFFSET(pub u32);
impl TEMP_SENS_OFFSET {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OFFSET_x1024\\[30:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn OFFSET_x1024(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "OFFSET_x1024\\[30:0\\]."]
    #[inline(always)]
    pub const fn set_OFFSET_x1024(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TEMP_SENS_OFFSET {
    #[inline(always)]
    fn default() -> TEMP_SENS_OFFSET {
        TEMP_SENS_OFFSET(0)
    }
}
impl core::fmt::Debug for TEMP_SENS_OFFSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP_SENS_OFFSET")
            .field("VALID", &self.VALID())
            .field("OFFSET_x1024", &self.OFFSET_x1024())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMP_SENS_OFFSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMP_SENS_OFFSET {{ VALID: {=bool:?}, OFFSET_x1024: {=u32:?} }}",
            self.VALID(),
            self.OFFSET_x1024()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMP_SENS_SLOPE(pub u32);
impl TEMP_SENS_SLOPE {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SLOPE_x1024\\[30:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn SLOPE_x1024(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "SLOPE_x1024\\[30:0\\]."]
    #[inline(always)]
    pub const fn set_SLOPE_x1024(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TEMP_SENS_SLOPE {
    #[inline(always)]
    fn default() -> TEMP_SENS_SLOPE {
        TEMP_SENS_SLOPE(0)
    }
}
impl core::fmt::Debug for TEMP_SENS_SLOPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP_SENS_SLOPE")
            .field("VALID", &self.VALID())
            .field("SLOPE_x1024", &self.SLOPE_x1024())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMP_SENS_SLOPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMP_SENS_SLOPE {{ VALID: {=bool:?}, SLOPE_x1024: {=u32:?} }}",
            self.VALID(),
            self.SLOPE_x1024()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMP_SENS_VBE1VBE8_REF_1(pub u32);
impl TEMP_SENS_VBE1VBE8_REF_1 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VBE1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VBE1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VBE8(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VBE8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TEMP_SENS_VBE1VBE8_REF_1 {
    #[inline(always)]
    fn default() -> TEMP_SENS_VBE1VBE8_REF_1 {
        TEMP_SENS_VBE1VBE8_REF_1(0)
    }
}
impl core::fmt::Debug for TEMP_SENS_VBE1VBE8_REF_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP_SENS_VBE1VBE8_REF_1")
            .field("VBE1", &self.VBE1())
            .field("VBE8", &self.VBE8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMP_SENS_VBE1VBE8_REF_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMP_SENS_VBE1VBE8_REF_1 {{ VBE1: {=u16:?}, VBE8: {=u16:?} }}",
            self.VBE1(),
            self.VBE8()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEMP_SENS_VBE1VBE8_REF_2(pub u32);
impl TEMP_SENS_VBE1VBE8_REF_2 {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VBE1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VBE1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn VBE8(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_VBE8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TEMP_SENS_VBE1VBE8_REF_2 {
    #[inline(always)]
    fn default() -> TEMP_SENS_VBE1VBE8_REF_2 {
        TEMP_SENS_VBE1VBE8_REF_2(0)
    }
}
impl core::fmt::Debug for TEMP_SENS_VBE1VBE8_REF_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEMP_SENS_VBE1VBE8_REF_2")
            .field("VBE1", &self.VBE1())
            .field("VBE8", &self.VBE8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEMP_SENS_VBE1VBE8_REF_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEMP_SENS_VBE1VBE8_REF_2 {{ VBE1: {=u16:?}, VBE8: {=u16:?} }}",
            self.VBE1(),
            self.VBE8()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBCFG(pub u32);
impl USBCFG {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_READY_TIME_OUT_MS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_READY_TIME_OUT_MS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_SPEED(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "USB_SPEED\\[7:0\\]= 0x00 : USB High Speed Module used for ISP 0x01 : USB Full SPeed Module used for ISP 0x02 : Neither USB High Speed module nor USB Full Speed module used for ISP 0x03 - 0xFF : RESERVED."]
    #[inline(always)]
    pub const fn set_USB_SPEED(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[must_use]
    #[inline(always)]
    pub const fn USB_USE_XO32M_CAPA_BANKS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the use of Crystal 32 MHz internal Capa Banks during the configuration of the High Speed USB for ISP: 0: Disable Crystal 32 MHz CapaBanks. 1: Enable Crystal 32 MHz CapaBanks."]
    #[inline(always)]
    pub const fn set_USB_USE_XO32M_CAPA_BANKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for USBCFG {
    #[inline(always)]
    fn default() -> USBCFG {
        USBCFG(0)
    }
}
impl core::fmt::Debug for USBCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCFG")
            .field("XO32M_READY_TIME_OUT_MS", &self.XO32M_READY_TIME_OUT_MS())
            .field("USB_SPEED", &self.USB_SPEED())
            .field("USB_USE_XO32M_CAPA_BANKS", &self.USB_USE_XO32M_CAPA_BANKS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBCFG {{ XO32M_READY_TIME_OUT_MS: {=u8:?}, USB_SPEED: {=u8:?}, USB_USE_XO32M_CAPA_BANKS: {=bool:?} }}",
            self.XO32M_READY_TIME_OUT_MS(),
            self.USB_SPEED(),
            self.USB_USE_XO32M_CAPA_BANKS()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_0(pub u32);
impl UUID_0 {
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
impl Default for UUID_0 {
    #[inline(always)]
    fn default() -> UUID_0 {
        UUID_0(0)
    }
}
impl core::fmt::Debug for UUID_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_1(pub u32);
impl UUID_1 {
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
impl Default for UUID_1 {
    #[inline(always)]
    fn default() -> UUID_1 {
        UUID_1(0)
    }
}
impl core::fmt::Debug for UUID_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_2(pub u32);
impl UUID_2 {
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
impl Default for UUID_2 {
    #[inline(always)]
    fn default() -> UUID_2 {
        UUID_2(0)
    }
}
impl core::fmt::Debug for UUID_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_3(pub u32);
impl UUID_3 {
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
impl Default for UUID_3 {
    #[inline(always)]
    fn default() -> UUID_3 {
        UUID_3(0)
    }
}
impl core::fmt::Debug for UUID_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_ARRAY0(pub u32);
impl UUID_ARRAY0 {
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
impl Default for UUID_ARRAY0 {
    #[inline(always)]
    fn default() -> UUID_ARRAY0 {
        UUID_ARRAY0(0)
    }
}
impl core::fmt::Debug for UUID_ARRAY0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_ARRAY0")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_ARRAY0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_ARRAY0 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_ARRAY1(pub u32);
impl UUID_ARRAY1 {
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
impl Default for UUID_ARRAY1 {
    #[inline(always)]
    fn default() -> UUID_ARRAY1 {
        UUID_ARRAY1(0)
    }
}
impl core::fmt::Debug for UUID_ARRAY1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_ARRAY1")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_ARRAY1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_ARRAY1 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_ARRAY2(pub u32);
impl UUID_ARRAY2 {
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
impl Default for UUID_ARRAY2 {
    #[inline(always)]
    fn default() -> UUID_ARRAY2 {
        UUID_ARRAY2(0)
    }
}
impl core::fmt::Debug for UUID_ARRAY2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_ARRAY2")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_ARRAY2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_ARRAY2 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UUID_ARRAY3(pub u32);
impl UUID_ARRAY3 {
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
impl Default for UUID_ARRAY3 {
    #[inline(always)]
    fn default() -> UUID_ARRAY3 {
        UUID_ARRAY3(0)
    }
}
impl core::fmt::Debug for UUID_ARRAY3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UUID_ARRAY3")
            .field("FIELD", &self.FIELD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UUID_ARRAY3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UUID_ARRAY3 {{ FIELD: {=u32:?} }}", self.FIELD())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST1_DATE(pub u32);
impl WAFER_TEST1_DATE {
    #[doc = "WT1_DATE \\[stored as : year*10000+month*100+day\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT1_DATE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_DATE \\[stored as : year*10000+month*100+day\\]."]
    #[inline(always)]
    pub const fn set_WT1_DATE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST1_DATE {
    #[inline(always)]
    fn default() -> WAFER_TEST1_DATE {
        WAFER_TEST1_DATE(0)
    }
}
impl core::fmt::Debug for WAFER_TEST1_DATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST1_DATE")
            .field("WT1_DATE", &self.WT1_DATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST1_DATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST1_DATE {{ WT1_DATE: {=u32:?} }}",
            self.WT1_DATE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST1_PROGRAM_VERSION(pub u32);
impl WAFER_TEST1_PROGRAM_VERSION {
    #[doc = "WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT1_PROGRAM_VERSION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[inline(always)]
    pub const fn set_WT1_PROGRAM_VERSION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST1_PROGRAM_VERSION {
    #[inline(always)]
    fn default() -> WAFER_TEST1_PROGRAM_VERSION {
        WAFER_TEST1_PROGRAM_VERSION(0)
    }
}
impl core::fmt::Debug for WAFER_TEST1_PROGRAM_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST1_PROGRAM_VERSION")
            .field("WT1_PROGRAM_VERSION", &self.WT1_PROGRAM_VERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST1_PROGRAM_VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST1_PROGRAM_VERSION {{ WT1_PROGRAM_VERSION: {=u32:?} }}",
            self.WT1_PROGRAM_VERSION()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST1_TIME(pub u32);
impl WAFER_TEST1_TIME {
    #[doc = "WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT1_TIME(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT1_TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[inline(always)]
    pub const fn set_WT1_TIME(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST1_TIME {
    #[inline(always)]
    fn default() -> WAFER_TEST1_TIME {
        WAFER_TEST1_TIME(0)
    }
}
impl core::fmt::Debug for WAFER_TEST1_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST1_TIME")
            .field("WT1_TIME", &self.WT1_TIME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST1_TIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST1_TIME {{ WT1_TIME: {=u32:?} }}",
            self.WT1_TIME()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST2_DATE(pub u32);
impl WAFER_TEST2_DATE {
    #[doc = "WT2_DATE \\[stored as : year*10000+month*100+day\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT2_DATE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_DATE \\[stored as : year*10000+month*100+day\\]."]
    #[inline(always)]
    pub const fn set_WT2_DATE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST2_DATE {
    #[inline(always)]
    fn default() -> WAFER_TEST2_DATE {
        WAFER_TEST2_DATE(0)
    }
}
impl core::fmt::Debug for WAFER_TEST2_DATE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST2_DATE")
            .field("WT2_DATE", &self.WT2_DATE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST2_DATE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST2_DATE {{ WT2_DATE: {=u32:?} }}",
            self.WT2_DATE()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST2_PROGRAM_VERSION(pub u32);
impl WAFER_TEST2_PROGRAM_VERSION {
    #[doc = "WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT2_PROGRAM_VERSION(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_PROGRAM_VERSION \\[xx.yy stored as : 100*x+y\\]."]
    #[inline(always)]
    pub const fn set_WT2_PROGRAM_VERSION(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST2_PROGRAM_VERSION {
    #[inline(always)]
    fn default() -> WAFER_TEST2_PROGRAM_VERSION {
        WAFER_TEST2_PROGRAM_VERSION(0)
    }
}
impl core::fmt::Debug for WAFER_TEST2_PROGRAM_VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST2_PROGRAM_VERSION")
            .field("WT2_PROGRAM_VERSION", &self.WT2_PROGRAM_VERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST2_PROGRAM_VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST2_PROGRAM_VERSION {{ WT2_PROGRAM_VERSION: {=u32:?} }}",
            self.WT2_PROGRAM_VERSION()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WAFER_TEST2_TIME(pub u32);
impl WAFER_TEST2_TIME {
    #[doc = "WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn WT2_TIME(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "WT2_TIME \\[stored as : hour*10000+minute*100+seconde\\]."]
    #[inline(always)]
    pub const fn set_WT2_TIME(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WAFER_TEST2_TIME {
    #[inline(always)]
    fn default() -> WAFER_TEST2_TIME {
        WAFER_TEST2_TIME(0)
    }
}
impl core::fmt::Debug for WAFER_TEST2_TIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAFER_TEST2_TIME")
            .field("WT2_TIME", &self.WT2_TIME())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WAFER_TEST2_TIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WAFER_TEST2_TIME {{ WT2_TIME: {=u32:?} }}",
            self.WT2_TIME()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XO_32KHZ(pub u32);
impl XO_32KHZ {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XIN_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XIN_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XIN_CAPCAL_6PF(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XIN_CAPCAL_6PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XIN_CAPCAL_8PF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XIN_CAPCAL_8PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XOUT_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XOUT_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XOUT_CAPCAL_6PF(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XOUT_CAPCAL_6PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32K_XOUT_CAPCAL_8PF(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32K_XOUT_CAPCAL_8PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
}
impl Default for XO_32KHZ {
    #[inline(always)]
    fn default() -> XO_32KHZ {
        XO_32KHZ(0)
    }
}
impl core::fmt::Debug for XO_32KHZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XO_32KHZ")
            .field("XO32K_XIN_TRIM_VALID", &self.XO32K_XIN_TRIM_VALID())
            .field("XO32K_XIN_CAPCAL_6PF", &self.XO32K_XIN_CAPCAL_6PF())
            .field("XO32K_XIN_CAPCAL_8PF", &self.XO32K_XIN_CAPCAL_8PF())
            .field("XO32K_XOUT_TRIM_VALID", &self.XO32K_XOUT_TRIM_VALID())
            .field("XO32K_XOUT_CAPCAL_6PF", &self.XO32K_XOUT_CAPCAL_6PF())
            .field("XO32K_XOUT_CAPCAL_8PF", &self.XO32K_XOUT_CAPCAL_8PF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XO_32KHZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XO_32KHZ {{ XO32K_XIN_TRIM_VALID: {=bool:?}, XO32K_XIN_CAPCAL_6PF: {=u8:?}, XO32K_XIN_CAPCAL_8PF: {=u8:?}, XO32K_XOUT_TRIM_VALID: {=bool:?}, XO32K_XOUT_CAPCAL_6PF: {=u8:?}, XO32K_XOUT_CAPCAL_8PF: {=u8:?} }}",
            self.XO32K_XIN_TRIM_VALID(),
            self.XO32K_XIN_CAPCAL_6PF(),
            self.XO32K_XIN_CAPCAL_8PF(),
            self.XO32K_XOUT_TRIM_VALID(),
            self.XO32K_XOUT_CAPCAL_6PF(),
            self.XO32K_XOUT_CAPCAL_8PF()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XO_32MHZ(pub u32);
impl XO_32MHZ {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XIN_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XIN_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XIN_CAPCAL_6PF(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XIN_CAPCAL_6PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XIN_CAPCAL_8PF(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XIN_CAPCAL_8PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XOUT_TRIM_VALID(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XOUT_TRIM_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XOUT_CAPCAL_6PF(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XOUT_CAPCAL_6PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XOUT_CAPCAL_8PF(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XOUT_CAPCAL_8PF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XO_SLAVE_STATUS(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XO_SLAVE_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn XO32M_XO_AC_BUF_STATUS(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_XO32M_XO_AC_BUF_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for XO_32MHZ {
    #[inline(always)]
    fn default() -> XO_32MHZ {
        XO_32MHZ(0)
    }
}
impl core::fmt::Debug for XO_32MHZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XO_32MHZ")
            .field("XO32M_XIN_TRIM_VALID", &self.XO32M_XIN_TRIM_VALID())
            .field("XO32M_XIN_CAPCAL_6PF", &self.XO32M_XIN_CAPCAL_6PF())
            .field("XO32M_XIN_CAPCAL_8PF", &self.XO32M_XIN_CAPCAL_8PF())
            .field("XO32M_XOUT_TRIM_VALID", &self.XO32M_XOUT_TRIM_VALID())
            .field("XO32M_XOUT_CAPCAL_6PF", &self.XO32M_XOUT_CAPCAL_6PF())
            .field("XO32M_XOUT_CAPCAL_8PF", &self.XO32M_XOUT_CAPCAL_8PF())
            .field("XO32M_XO_SLAVE_STATUS", &self.XO32M_XO_SLAVE_STATUS())
            .field("XO32M_XO_AC_BUF_STATUS", &self.XO32M_XO_AC_BUF_STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XO_32MHZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XO_32MHZ {{ XO32M_XIN_TRIM_VALID: {=bool:?}, XO32M_XIN_CAPCAL_6PF: {=u8:?}, XO32M_XIN_CAPCAL_8PF: {=u8:?}, XO32M_XOUT_TRIM_VALID: {=bool:?}, XO32M_XOUT_CAPCAL_6PF: {=u8:?}, XO32M_XOUT_CAPCAL_8PF: {=u8:?}, XO32M_XO_SLAVE_STATUS: {=bool:?}, XO32M_XO_AC_BUF_STATUS: {=bool:?} }}",
            self.XO32M_XIN_TRIM_VALID(),
            self.XO32M_XIN_CAPCAL_6PF(),
            self.XO32M_XIN_CAPCAL_8PF(),
            self.XO32M_XOUT_TRIM_VALID(),
            self.XO32M_XOUT_CAPCAL_6PF(),
            self.XO32M_XOUT_CAPCAL_8PF(),
            self.XO32M_XO_SLAVE_STATUS(),
            self.XO32M_XO_AC_BUF_STATUS()
        )
    }
}
