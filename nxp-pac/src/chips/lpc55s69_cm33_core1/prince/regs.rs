#[doc = "Base Address for region 0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BaseAddr0(pub u32);
impl BaseAddr0 {
    #[doc = "Fixed portion of the base address of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_fixed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 0."]
    #[inline(always)]
    pub const fn set_addr_fixed(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_prg(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 0."]
    #[inline(always)]
    pub const fn set_addr_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BaseAddr0 {
    #[inline(always)]
    fn default() -> BaseAddr0 {
        BaseAddr0(0)
    }
}
impl core::fmt::Debug for BaseAddr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BaseAddr0")
            .field("addr_fixed", &self.addr_fixed())
            .field("addr_prg", &self.addr_prg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BaseAddr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BaseAddr0 {{ addr_fixed: {=u32:?}, addr_prg: {=u8:?} }}",
            self.addr_fixed(),
            self.addr_prg()
        )
    }
}
#[doc = "Base Address for region 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BaseAddr1(pub u32);
impl BaseAddr1 {
    #[doc = "Fixed portion of the base address of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_fixed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 1."]
    #[inline(always)]
    pub const fn set_addr_fixed(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_prg(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 1."]
    #[inline(always)]
    pub const fn set_addr_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BaseAddr1 {
    #[inline(always)]
    fn default() -> BaseAddr1 {
        BaseAddr1(0)
    }
}
impl core::fmt::Debug for BaseAddr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BaseAddr1")
            .field("addr_fixed", &self.addr_fixed())
            .field("addr_prg", &self.addr_prg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BaseAddr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BaseAddr1 {{ addr_fixed: {=u32:?}, addr_prg: {=u8:?} }}",
            self.addr_fixed(),
            self.addr_prg()
        )
    }
}
#[doc = "Base Address for region 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BaseAddr2(pub u32);
impl BaseAddr2 {
    #[doc = "Fixed portion of the base address of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_fixed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Fixed portion of the base address of region 2."]
    #[inline(always)]
    pub const fn set_addr_fixed(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_prg(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Programmable portion of the base address of region 2."]
    #[inline(always)]
    pub const fn set_addr_prg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for BaseAddr2 {
    #[inline(always)]
    fn default() -> BaseAddr2 {
        BaseAddr2(0)
    }
}
impl core::fmt::Debug for BaseAddr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BaseAddr2")
            .field("addr_fixed", &self.addr_fixed())
            .field("addr_prg", &self.addr_prg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BaseAddr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BaseAddr2 {{ addr_fixed: {=u32:?}, addr_prg: {=u8:?} }}",
            self.addr_fixed(),
            self.addr_prg()
        )
    }
}
#[doc = "Encryption Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EncEnable(pub u32);
impl EncEnable {
    #[doc = "Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Encryption Enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for EncEnable {
    #[inline(always)]
    fn default() -> EncEnable {
        EncEnable(0)
    }
}
impl core::fmt::Debug for EncEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EncEnable").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EncEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EncEnable {{ en: {=bool:?} }}", self.en())
    }
}
#[doc = "Initial Vector register for region 0, Least Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvLsb0(pub u32);
impl IvLsb0 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvLsb0 {
    #[inline(always)]
    fn default() -> IvLsb0 {
        IvLsb0(0)
    }
}
impl core::fmt::Debug for IvLsb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvLsb0")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvLsb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvLsb0 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Initial Vector register for region 1, Least Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvLsb1(pub u32);
impl IvLsb1 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvLsb1 {
    #[inline(always)]
    fn default() -> IvLsb1 {
        IvLsb1(0)
    }
}
impl core::fmt::Debug for IvLsb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvLsb1")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvLsb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvLsb1 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Initial Vector register for region 2, Least Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvLsb2(pub u32);
impl IvLsb2 {
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Least Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvLsb2 {
    #[inline(always)]
    fn default() -> IvLsb2 {
        IvLsb2(0)
    }
}
impl core::fmt::Debug for IvLsb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvLsb2")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvLsb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvLsb2 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Initial Vector register for region 0, Most Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvMsb0(pub u32);
impl IvMsb0 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvMsb0 {
    #[inline(always)]
    fn default() -> IvMsb0 {
        IvMsb0(0)
    }
}
impl core::fmt::Debug for IvMsb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvMsb0")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvMsb0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvMsb0 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Initial Vector register for region 1, Most Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvMsb1(pub u32);
impl IvMsb1 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvMsb1 {
    #[inline(always)]
    fn default() -> IvMsb1 {
        IvMsb1(0)
    }
}
impl core::fmt::Debug for IvMsb1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvMsb1")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvMsb1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvMsb1 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Initial Vector register for region 2, Most Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvMsb2(pub u32);
impl IvMsb2 {
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[must_use]
    #[inline(always)]
    pub const fn ivval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Initial Vector value for the 32 Most Significant Bits of the 64-bit Initial Vector."]
    #[inline(always)]
    pub const fn set_ivval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IvMsb2 {
    #[inline(always)]
    fn default() -> IvMsb2 {
        IvMsb2(0)
    }
}
impl core::fmt::Debug for IvMsb2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvMsb2")
            .field("ivval", &self.ivval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvMsb2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvMsb2 {{ ivval: {=u32:?} }}", self.ivval())
    }
}
#[doc = "Lock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Lock Region 0 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lockreg0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 0 registers."]
    #[inline(always)]
    pub const fn set_lockreg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock Region 1 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lockreg1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 1 registers."]
    #[inline(always)]
    pub const fn set_lockreg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Lock Region 2 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lockreg2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Region 2 registers."]
    #[inline(always)]
    pub const fn set_lockreg2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock the Mask registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lockmask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Lock the Mask registers."]
    #[inline(always)]
    pub const fn set_lockmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("lockreg0", &self.lockreg0())
            .field("lockreg1", &self.lockreg1())
            .field("lockreg2", &self.lockreg2())
            .field("lockmask", &self.lockmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ lockreg0: {=bool:?}, lockreg1: {=bool:?}, lockreg2: {=bool:?}, lockmask: {=bool:?} }}",
            self.lockreg0(),
            self.lockreg1(),
            self.lockreg2(),
            self.lockmask()
        )
    }
}
#[doc = "Data Mask register, 32 Least Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskLsb(pub u32);
impl MaskLsb {
    #[doc = "Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[must_use]
    #[inline(always)]
    pub const fn maskval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value of the 32 Least Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub const fn set_maskval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MaskLsb {
    #[inline(always)]
    fn default() -> MaskLsb {
        MaskLsb(0)
    }
}
impl core::fmt::Debug for MaskLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskLsb")
            .field("maskval", &self.maskval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MaskLsb {{ maskval: {=u32:?} }}", self.maskval())
    }
}
#[doc = "Data Mask register, 32 Most Significant Bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaskMsb(pub u32);
impl MaskMsb {
    #[doc = "Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[must_use]
    #[inline(always)]
    pub const fn maskval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Value of the 32 Most Significant Bits of the 64-bit data mask."]
    #[inline(always)]
    pub const fn set_maskval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MaskMsb {
    #[inline(always)]
    fn default() -> MaskMsb {
        MaskMsb(0)
    }
}
impl core::fmt::Debug for MaskMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaskMsb")
            .field("maskval", &self.maskval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaskMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MaskMsb {{ maskval: {=u32:?} }}", self.maskval())
    }
}
#[doc = "Sub-Region Enable register for region 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrEnable0(pub u32);
impl SrEnable0 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 0."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SrEnable0 {
    #[inline(always)]
    fn default() -> SrEnable0 {
        SrEnable0(0)
    }
}
impl core::fmt::Debug for SrEnable0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrEnable0").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrEnable0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SrEnable0 {{ en: {=u32:?} }}", self.en())
    }
}
#[doc = "Sub-Region Enable register for region 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrEnable1(pub u32);
impl SrEnable1 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 1."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SrEnable1 {
    #[inline(always)]
    fn default() -> SrEnable1 {
        SrEnable1(0)
    }
}
impl core::fmt::Debug for SrEnable1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrEnable1").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrEnable1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SrEnable1 {{ en: {=u32:?} }}", self.en())
    }
}
#[doc = "Sub-Region Enable register for region 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrEnable2(pub u32);
impl SrEnable2 {
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Each bit in this field enables an 8KB subregion for encryption at offset 8KB*bitnum of region 2."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SrEnable2 {
    #[inline(always)]
    fn default() -> SrEnable2 {
        SrEnable2(0)
    }
}
impl core::fmt::Debug for SrEnable2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SrEnable2").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SrEnable2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SrEnable2 {{ en: {=u32:?} }}", self.en())
    }
}
