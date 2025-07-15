#[doc = "MPU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enables the MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enables the MPU."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers"]
    #[must_use]
    #[inline(always)]
    pub const fn hfnmiena(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers"]
    #[inline(always)]
    pub const fn set_hfnmiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn privdefena(&self) -> super::vals::Privdefena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Privdefena::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_privdefena(&mut self, val: super::vals::Privdefena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enable", &self.enable())
            .field("hfnmiena", &self.hfnmiena())
            .field("privdefena", &self.privdefena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {:?}, hfnmiena: {=bool:?}, privdefena: {:?} }}",
            self.enable(),
            self.hfnmiena(),
            self.privdefena()
        )
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mair0(pub u32);
impl Mair0 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[must_use]
    #[inline(always)]
    pub const fn attr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[inline(always)]
    pub const fn set_attr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[must_use]
    #[inline(always)]
    pub const fn attr1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[inline(always)]
    pub const fn set_attr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[must_use]
    #[inline(always)]
    pub const fn attr2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[inline(always)]
    pub const fn set_attr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[must_use]
    #[inline(always)]
    pub const fn attr3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[inline(always)]
    pub const fn set_attr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mair0 {
    #[inline(always)]
    fn default() -> Mair0 {
        Mair0(0)
    }
}
impl core::fmt::Debug for Mair0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mair0")
            .field("attr0", &self.attr0())
            .field("attr1", &self.attr1())
            .field("attr2", &self.attr2())
            .field("attr3", &self.attr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mair0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mair0 {{ attr0: {=u8:?}, attr1: {=u8:?}, attr2: {=u8:?}, attr3: {=u8:?} }}",
            self.attr0(),
            self.attr1(),
            self.attr2(),
            self.attr3()
        )
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mair1(pub u32);
impl Mair1 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[must_use]
    #[inline(always)]
    pub const fn attr4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[inline(always)]
    pub const fn set_attr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[must_use]
    #[inline(always)]
    pub const fn attr5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[inline(always)]
    pub const fn set_attr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[must_use]
    #[inline(always)]
    pub const fn attr6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[inline(always)]
    pub const fn set_attr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[must_use]
    #[inline(always)]
    pub const fn attr7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[inline(always)]
    pub const fn set_attr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mair1 {
    #[inline(always)]
    fn default() -> Mair1 {
        Mair1(0)
    }
}
impl core::fmt::Debug for Mair1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mair1")
            .field("attr4", &self.attr4())
            .field("attr5", &self.attr5())
            .field("attr6", &self.attr6())
            .field("attr7", &self.attr7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mair1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mair1 {{ attr4: {=u8:?}, attr5: {=u8:?}, attr6: {=u8:?}, attr7: {=u8:?} }}",
            self.attr4(),
            self.attr5(),
            self.attr6(),
            self.attr7()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar(pub u32);
impl Rbar {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarXn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarXn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: super::vals::RbarXn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarAp {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarAp::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RbarAp) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarSh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarSh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_sh(&mut self, val: super::vals::RbarSh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        Rbar(0)
    }
}
impl core::fmt::Debug for Rbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rbar")
            .field("xn", &self.xn())
            .field("ap", &self.ap())
            .field("sh", &self.sh())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rbar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rbar {{ xn: {:?}, ap: {:?}, sh: {:?}, base: {=u32:?} }}",
            self.xn(),
            self.ap(),
            self.sh(),
            self.base()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA1(pub u32);
impl RbarA1 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA1Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA1Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: super::vals::RbarA1Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA1Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA1Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RbarA1Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA1Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA1Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_sh(&mut self, val: super::vals::RbarA1Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA1 {
    #[inline(always)]
    fn default() -> RbarA1 {
        RbarA1(0)
    }
}
impl core::fmt::Debug for RbarA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA1")
            .field("xn", &self.xn())
            .field("ap", &self.ap())
            .field("sh", &self.sh())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA1 {{ xn: {:?}, ap: {:?}, sh: {:?}, base: {=u32:?} }}",
            self.xn(),
            self.ap(),
            self.sh(),
            self.base()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA2(pub u32);
impl RbarA2 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA2Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA2Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: super::vals::RbarA2Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA2Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA2Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RbarA2Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA2Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA2Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_sh(&mut self, val: super::vals::RbarA2Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA2 {
    #[inline(always)]
    fn default() -> RbarA2 {
        RbarA2(0)
    }
}
impl core::fmt::Debug for RbarA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA2")
            .field("xn", &self.xn())
            .field("ap", &self.ap())
            .field("sh", &self.sh())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA2 {{ xn: {:?}, ap: {:?}, sh: {:?}, base: {=u32:?} }}",
            self.xn(),
            self.ap(),
            self.sh(),
            self.base()
        )
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA3(pub u32);
impl RbarA3 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[must_use]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA3Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA3Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn set_xn(&mut self, val: super::vals::RbarA3Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[must_use]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA3Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA3Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn set_ap(&mut self, val: super::vals::RbarA3Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[must_use]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA3Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA3Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn set_sh(&mut self, val: super::vals::RbarA3Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA3 {
    #[inline(always)]
    fn default() -> RbarA3 {
        RbarA3(0)
    }
}
impl core::fmt::Debug for RbarA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbarA3")
            .field("xn", &self.xn())
            .field("ap", &self.ap())
            .field("sh", &self.sh())
            .field("base", &self.base())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbarA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RbarA3 {{ xn: {:?}, ap: {:?}, sh: {:?}, base: {=u32:?} }}",
            self.xn(),
            self.ap(),
            self.sh(),
            self.base()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlar(pub u32);
impl Rlar {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rlar {
    #[inline(always)]
    fn default() -> Rlar {
        Rlar(0)
    }
}
impl core::fmt::Debug for Rlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rlar")
            .field("en", &self.en())
            .field("attrindx", &self.attrindx())
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rlar {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rlar {{ en: {=bool:?}, attrindx: {=u8:?}, limit: {=u32:?} }}",
            self.en(),
            self.attrindx(),
            self.limit()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA1(pub u32);
impl RlarA1 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA1 {
    #[inline(always)]
    fn default() -> RlarA1 {
        RlarA1(0)
    }
}
impl core::fmt::Debug for RlarA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RlarA1")
            .field("en", &self.en())
            .field("attrindx", &self.attrindx())
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RlarA1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RlarA1 {{ en: {=bool:?}, attrindx: {=u8:?}, limit: {=u32:?} }}",
            self.en(),
            self.attrindx(),
            self.limit()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA2(pub u32);
impl RlarA2 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA2 {
    #[inline(always)]
    fn default() -> RlarA2 {
        RlarA2(0)
    }
}
impl core::fmt::Debug for RlarA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RlarA2")
            .field("en", &self.en())
            .field("attrindx", &self.attrindx())
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RlarA2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RlarA2 {{ en: {=bool:?}, attrindx: {=u8:?}, limit: {=u32:?} }}",
            self.en(),
            self.attrindx(),
            self.limit()
        )
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA3(pub u32);
impl RlarA3 {
    #[doc = "Enables this region."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[must_use]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA3 {
    #[inline(always)]
    fn default() -> RlarA3 {
        RlarA3(0)
    }
}
impl core::fmt::Debug for RlarA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RlarA3")
            .field("en", &self.en())
            .field("attrindx", &self.attrindx())
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RlarA3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RlarA3 {{ en: {=bool:?}, attrindx: {=u8:?}, limit: {=u32:?} }}",
            self.en(),
            self.attrindx(),
            self.limit()
        )
    }
}
#[doc = "MPU Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr(pub u32);
impl Rnr {
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[must_use]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub const fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        Rnr(0)
    }
}
impl core::fmt::Debug for Rnr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rnr")
            .field("region", &self.region())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rnr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rnr {{ region: {=u8:?} }}", self.region())
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU support. Software can use it to determine if the processor implements an MPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u32);
impl Type {
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU"]
    #[must_use]
    #[inline(always)]
    pub const fn separate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU"]
    #[inline(always)]
    pub const fn set_separate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[must_use]
    #[inline(always)]
    pub const fn dregion(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[inline(always)]
    pub const fn set_dregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Type {
    #[inline(always)]
    fn default() -> Type {
        Type(0)
    }
}
impl core::fmt::Debug for Type {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Type")
            .field("separate", &self.separate())
            .field("dregion", &self.dregion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Type {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Type {{ separate: {=bool:?}, dregion: {=u8:?} }}",
            self.separate(),
            self.dregion()
        )
    }
}
