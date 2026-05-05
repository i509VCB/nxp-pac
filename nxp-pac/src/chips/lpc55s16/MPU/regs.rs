#[doc = "MPU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Enables the MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE(&self) -> super::vals::ENABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ENABLE::from_bits(val as u8)
    }
    #[doc = "Enables the MPU."]
    #[inline(always)]
    pub const fn set_ENABLE(&mut self, val: super::vals::ENABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers."]
    #[must_use]
    #[inline(always)]
    pub const fn HFNMIENA(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers."]
    #[inline(always)]
    pub const fn set_HFNMIENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIVDEFENA(&self) -> super::vals::PRIVDEFENA {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PRIVDEFENA::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_PRIVDEFENA(&mut self, val: super::vals::PRIVDEFENA) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ENABLE", &self.ENABLE())
            .field("HFNMIENA", &self.HFNMIENA())
            .field("PRIVDEFENA", &self.PRIVDEFENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ENABLE: {:?}, HFNMIENA: {=bool:?}, PRIVDEFENA: {:?} }}",
            self.ENABLE(),
            self.HFNMIENA(),
            self.PRIVDEFENA()
        )
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAIR0(pub u32);
impl MAIR0 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[inline(always)]
    pub const fn set_ATTR0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[inline(always)]
    pub const fn set_ATTR1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[inline(always)]
    pub const fn set_ATTR2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[inline(always)]
    pub const fn set_ATTR3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for MAIR0 {
    #[inline(always)]
    fn default() -> MAIR0 {
        MAIR0(0)
    }
}
impl core::fmt::Debug for MAIR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAIR0")
            .field("ATTR0", &self.ATTR0())
            .field("ATTR1", &self.ATTR1())
            .field("ATTR2", &self.ATTR2())
            .field("ATTR3", &self.ATTR3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAIR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MAIR0 {{ ATTR0: {=u8:?}, ATTR1: {=u8:?}, ATTR2: {=u8:?}, ATTR3: {=u8:?} }}",
            self.ATTR0(),
            self.ATTR1(),
            self.ATTR2(),
            self.ATTR3()
        )
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MAIR1(pub u32);
impl MAIR1 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[inline(always)]
    pub const fn set_ATTR4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[inline(always)]
    pub const fn set_ATTR5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[inline(always)]
    pub const fn set_ATTR6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTR7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[inline(always)]
    pub const fn set_ATTR7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for MAIR1 {
    #[inline(always)]
    fn default() -> MAIR1 {
        MAIR1(0)
    }
}
impl core::fmt::Debug for MAIR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAIR1")
            .field("ATTR4", &self.ATTR4())
            .field("ATTR5", &self.ATTR5())
            .field("ATTR6", &self.ATTR6())
            .field("ATTR7", &self.ATTR7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MAIR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MAIR1 {{ ATTR4: {=u8:?}, ATTR5: {=u8:?}, ATTR6: {=u8:?}, ATTR7: {=u8:?} }}",
            self.ATTR4(),
            self.ATTR5(),
            self.ATTR6(),
            self.ATTR7()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RBAR(pub u32);
impl RBAR {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn XN(&self) -> super::vals::RBAR_XN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RBAR_XN::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_XN(&mut self, val: super::vals::RBAR_XN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn AP(&self) -> super::vals::RBAR_AP {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RBAR_AP::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_AP(&mut self, val: super::vals::RBAR_AP) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn SH(&self) -> super::vals::RBAR_SH {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RBAR_SH::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_SH(&mut self, val: super::vals::RBAR_SH) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RBAR {
    #[inline(always)]
    fn default() -> RBAR {
        RBAR(0)
    }
}
impl core::fmt::Debug for RBAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR")
            .field("XN", &self.XN())
            .field("AP", &self.AP())
            .field("SH", &self.SH())
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RBAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RBAR {{ XN: {:?}, AP: {:?}, SH: {:?}, BASE: {=u32:?} }}",
            self.XN(),
            self.AP(),
            self.SH(),
            self.BASE()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RBAR_A1(pub u32);
impl RBAR_A1 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn XN(&self) -> super::vals::RBAR_A1_XN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RBAR_A1_XN::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_XN(&mut self, val: super::vals::RBAR_A1_XN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn AP(&self) -> super::vals::RBAR_A1_AP {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RBAR_A1_AP::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_AP(&mut self, val: super::vals::RBAR_A1_AP) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn SH(&self) -> super::vals::RBAR_A1_SH {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RBAR_A1_SH::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_SH(&mut self, val: super::vals::RBAR_A1_SH) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RBAR_A1 {
    #[inline(always)]
    fn default() -> RBAR_A1 {
        RBAR_A1(0)
    }
}
impl core::fmt::Debug for RBAR_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR_A1")
            .field("XN", &self.XN())
            .field("AP", &self.AP())
            .field("SH", &self.SH())
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RBAR_A1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RBAR_A1 {{ XN: {:?}, AP: {:?}, SH: {:?}, BASE: {=u32:?} }}",
            self.XN(),
            self.AP(),
            self.SH(),
            self.BASE()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RBAR_A2(pub u32);
impl RBAR_A2 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn XN(&self) -> super::vals::RBAR_A2_XN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RBAR_A2_XN::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_XN(&mut self, val: super::vals::RBAR_A2_XN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn AP(&self) -> super::vals::RBAR_A2_AP {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RBAR_A2_AP::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_AP(&mut self, val: super::vals::RBAR_A2_AP) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn SH(&self) -> super::vals::RBAR_A2_SH {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RBAR_A2_SH::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_SH(&mut self, val: super::vals::RBAR_A2_SH) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RBAR_A2 {
    #[inline(always)]
    fn default() -> RBAR_A2 {
        RBAR_A2(0)
    }
}
impl core::fmt::Debug for RBAR_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR_A2")
            .field("XN", &self.XN())
            .field("AP", &self.AP())
            .field("SH", &self.SH())
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RBAR_A2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RBAR_A2 {{ XN: {:?}, AP: {:?}, SH: {:?}, BASE: {=u32:?} }}",
            self.XN(),
            self.AP(),
            self.SH(),
            self.BASE()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RBAR_A3(pub u32);
impl RBAR_A3 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn XN(&self) -> super::vals::RBAR_A3_XN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RBAR_A3_XN::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_XN(&mut self, val: super::vals::RBAR_A3_XN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn AP(&self) -> super::vals::RBAR_A3_AP {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RBAR_A3_AP::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_AP(&mut self, val: super::vals::RBAR_A3_AP) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn SH(&self) -> super::vals::RBAR_A3_SH {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RBAR_A3_SH::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_SH(&mut self, val: super::vals::RBAR_A3_SH) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn BASE(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_BASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RBAR_A3 {
    #[inline(always)]
    fn default() -> RBAR_A3 {
        RBAR_A3(0)
    }
}
impl core::fmt::Debug for RBAR_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RBAR_A3")
            .field("XN", &self.XN())
            .field("AP", &self.AP())
            .field("SH", &self.SH())
            .field("BASE", &self.BASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RBAR_A3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RBAR_A3 {{ XN: {:?}, AP: {:?}, SH: {:?}, BASE: {=u32:?} }}",
            self.XN(),
            self.AP(),
            self.SH(),
            self.BASE()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RLAR(pub u32);
impl RLAR {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTRINDX(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_ATTRINDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMIT(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_LIMIT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RLAR {
    #[inline(always)]
    fn default() -> RLAR {
        RLAR(0)
    }
}
impl core::fmt::Debug for RLAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLAR")
            .field("EN", &self.EN())
            .field("ATTRINDX", &self.ATTRINDX())
            .field("LIMIT", &self.LIMIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RLAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RLAR {{ EN: {=bool:?}, ATTRINDX: {=u8:?}, LIMIT: {=u32:?} }}",
            self.EN(),
            self.ATTRINDX(),
            self.LIMIT()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RLAR_A1(pub u32);
impl RLAR_A1 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTRINDX(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_ATTRINDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMIT(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_LIMIT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RLAR_A1 {
    #[inline(always)]
    fn default() -> RLAR_A1 {
        RLAR_A1(0)
    }
}
impl core::fmt::Debug for RLAR_A1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLAR_A1")
            .field("EN", &self.EN())
            .field("ATTRINDX", &self.ATTRINDX())
            .field("LIMIT", &self.LIMIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RLAR_A1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RLAR_A1 {{ EN: {=bool:?}, ATTRINDX: {=u8:?}, LIMIT: {=u32:?} }}",
            self.EN(),
            self.ATTRINDX(),
            self.LIMIT()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RLAR_A2(pub u32);
impl RLAR_A2 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTRINDX(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_ATTRINDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMIT(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_LIMIT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RLAR_A2 {
    #[inline(always)]
    fn default() -> RLAR_A2 {
        RLAR_A2(0)
    }
}
impl core::fmt::Debug for RLAR_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLAR_A2")
            .field("EN", &self.EN())
            .field("ATTRINDX", &self.ATTRINDX())
            .field("LIMIT", &self.LIMIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RLAR_A2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RLAR_A2 {{ EN: {=bool:?}, ATTRINDX: {=u8:?}, LIMIT: {=u32:?} }}",
            self.EN(),
            self.ATTRINDX(),
            self.LIMIT()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RLAR_A3(pub u32);
impl RLAR_A3 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn EN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn ATTRINDX(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_ATTRINDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn LIMIT(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_LIMIT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RLAR_A3 {
    #[inline(always)]
    fn default() -> RLAR_A3 {
        RLAR_A3(0)
    }
}
impl core::fmt::Debug for RLAR_A3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLAR_A3")
            .field("EN", &self.EN())
            .field("ATTRINDX", &self.ATTRINDX())
            .field("LIMIT", &self.LIMIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RLAR_A3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RLAR_A3 {{ EN: {=bool:?}, ATTRINDX: {=u8:?}, LIMIT: {=u32:?} }}",
            self.EN(),
            self.ATTRINDX(),
            self.LIMIT()
        )
    }
}
#[doc = "MPU Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RNR(pub u32);
impl RNR {
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[must_use]
    #[inline(always)]
    pub const fn REGION(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub const fn set_REGION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RNR {
    #[inline(always)]
    fn default() -> RNR {
        RNR(0)
    }
}
impl core::fmt::Debug for RNR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNR")
            .field("REGION", &self.REGION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RNR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RNR {{ REGION: {=u8:?} }}", self.REGION())
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU support. Software can use it to determine if the processor implements an MPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TYPE(pub u32);
impl TYPE {
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn SEPARATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU."]
    #[inline(always)]
    pub const fn set_SEPARATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn DREGION(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[inline(always)]
    pub const fn set_DREGION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for TYPE {
    #[inline(always)]
    fn default() -> TYPE {
        TYPE(0)
    }
}
impl core::fmt::Debug for TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TYPE")
            .field("SEPARATE", &self.SEPARATE())
            .field("DREGION", &self.DREGION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TYPE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TYPE {{ SEPARATE: {=bool:?}, DREGION: {=u8:?} }}",
            self.SEPARATE(),
            self.DREGION()
        )
    }
}
