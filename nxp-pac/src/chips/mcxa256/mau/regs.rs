#[doc = "General Exception Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GexpStatus(pub u32);
impl GexpStatus {
    #[doc = "Direct operation Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Direct operation Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GexpStatus {
    #[inline(always)]
    fn default() -> GexpStatus {
        GexpStatus(0)
    }
}
impl core::fmt::Debug for GexpStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GexpStatus")
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GexpStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GexpStatus {{ error: {=bool:?} }}", self.error())
    }
}
#[doc = "General Exception Status Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GexpStatusIe(pub u32);
impl GexpStatusIe {
    #[doc = "Direct operation Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn error_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Direct operation Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_error_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for GexpStatusIe {
    #[inline(always)]
    fn default() -> GexpStatusIe {
        GexpStatusIe(0)
    }
}
impl core::fmt::Debug for GexpStatusIe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GexpStatusIe")
            .field("error_ie", &self.error_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GexpStatusIe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GexpStatusIe {{ error_ie: {=bool:?} }}", self.error_ie())
    }
}
#[doc = "Operation Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpCtrl(pub u32);
impl OpCtrl {
    #[doc = "Override RES0 Data Type Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_en_res0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override RES0 Data Type Enable"]
    #[inline(always)]
    pub const fn set_ovdt_en_res0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override RES0 Data Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_res0(&self) -> super::vals::OvdtRes0 {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::OvdtRes0::from_bits(val as u8)
    }
    #[doc = "Override RES0 Data Type"]
    #[inline(always)]
    pub const fn set_ovdt_res0(&mut self, val: super::vals::OvdtRes0) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Override RES1 Data Type Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_en_res1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Override RES1 Data Type Enable"]
    #[inline(always)]
    pub const fn set_ovdt_en_res1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Override RES1 Data Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_res1(&self) -> super::vals::OvdtRes1 {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::OvdtRes1::from_bits(val as u8)
    }
    #[doc = "Override RES1 Data Type"]
    #[inline(always)]
    pub const fn set_ovdt_res1(&mut self, val: super::vals::OvdtRes1) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Override RES2 Data Type Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_en_res2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Override RES2 Data Type Enable"]
    #[inline(always)]
    pub const fn set_ovdt_en_res2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Override RES2 Data Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_res2(&self) -> super::vals::OvdtRes2 {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::OvdtRes2::from_bits(val as u8)
    }
    #[doc = "Override RES2 Data Type"]
    #[inline(always)]
    pub const fn set_ovdt_res2(&mut self, val: super::vals::OvdtRes2) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "Override RES3 Data Type Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_en_res3(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Override RES3 Data Type Enable"]
    #[inline(always)]
    pub const fn set_ovdt_en_res3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Override RES3 Data Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ovdt_res3(&self) -> super::vals::OvdtRes3 {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::OvdtRes3::from_bits(val as u8)
    }
    #[doc = "Override RES3 Data Type"]
    #[inline(always)]
    pub const fn set_ovdt_res3(&mut self, val: super::vals::OvdtRes3) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
}
impl Default for OpCtrl {
    #[inline(always)]
    fn default() -> OpCtrl {
        OpCtrl(0)
    }
}
impl core::fmt::Debug for OpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OpCtrl")
            .field("ovdt_en_res0", &self.ovdt_en_res0())
            .field("ovdt_res0", &self.ovdt_res0())
            .field("ovdt_en_res1", &self.ovdt_en_res1())
            .field("ovdt_res1", &self.ovdt_res1())
            .field("ovdt_en_res2", &self.ovdt_en_res2())
            .field("ovdt_res2", &self.ovdt_res2())
            .field("ovdt_en_res3", &self.ovdt_en_res3())
            .field("ovdt_res3", &self.ovdt_res3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OpCtrl {{ ovdt_en_res0: {=bool:?}, ovdt_res0: {:?}, ovdt_en_res1: {=bool:?}, ovdt_res1: {:?}, ovdt_en_res2: {=bool:?}, ovdt_res2: {:?}, ovdt_en_res3: {=bool:?}, ovdt_res3: {:?} }}",
            self.ovdt_en_res0(),
            self.ovdt_res0(),
            self.ovdt_en_res1(),
            self.ovdt_res1(),
            self.ovdt_en_res2(),
            self.ovdt_res2(),
            self.ovdt_en_res3(),
            self.ovdt_res3()
        )
    }
}
#[doc = "Result Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res0(pub u32);
impl Res0 {
    #[doc = "MAUWRAP Result Register 0"]
    #[must_use]
    #[inline(always)]
    pub const fn mau_res0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAUWRAP Result Register 0"]
    #[inline(always)]
    pub const fn set_mau_res0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res0 {
    #[inline(always)]
    fn default() -> Res0 {
        Res0(0)
    }
}
impl core::fmt::Debug for Res0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res0")
            .field("mau_res0", &self.mau_res0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res0 {{ mau_res0: {=u32:?} }}", self.mau_res0())
    }
}
#[doc = "Result Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res1(pub u32);
impl Res1 {
    #[doc = "MAUWRAP Result Register 1"]
    #[must_use]
    #[inline(always)]
    pub const fn mau_res1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAUWRAP Result Register 1"]
    #[inline(always)]
    pub const fn set_mau_res1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res1 {
    #[inline(always)]
    fn default() -> Res1 {
        Res1(0)
    }
}
impl core::fmt::Debug for Res1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res1")
            .field("mau_res1", &self.mau_res1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res1 {{ mau_res1: {=u32:?} }}", self.mau_res1())
    }
}
#[doc = "Result Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res2(pub u32);
impl Res2 {
    #[doc = "MAUWRAP Result Register 2"]
    #[must_use]
    #[inline(always)]
    pub const fn mau_res2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAUWRAP Result Register 2"]
    #[inline(always)]
    pub const fn set_mau_res2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res2 {
    #[inline(always)]
    fn default() -> Res2 {
        Res2(0)
    }
}
impl core::fmt::Debug for Res2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res2")
            .field("mau_res2", &self.mau_res2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res2 {{ mau_res2: {=u32:?} }}", self.mau_res2())
    }
}
#[doc = "Result Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res3(pub u32);
impl Res3 {
    #[doc = "MAUWRAP Result Register 3"]
    #[must_use]
    #[inline(always)]
    pub const fn mau_res3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAUWRAP Result Register 3"]
    #[inline(always)]
    pub const fn set_mau_res3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Res3 {
    #[inline(always)]
    fn default() -> Res3 {
        Res3(0)
    }
}
impl core::fmt::Debug for Res3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res3")
            .field("mau_res3", &self.mau_res3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Res3 {{ mau_res3: {=u32:?} }}", self.mau_res3())
    }
}
#[doc = "Result Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResStatus(pub u32);
impl ResStatus {
    #[doc = "RES0 IEEE Inexact Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_nx(&self) -> super::vals::Res0Nx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Res0Nx::from_bits(val as u8)
    }
    #[doc = "RES0 IEEE Inexact Flag"]
    #[inline(always)]
    pub const fn set_res0_nx(&mut self, val: super::vals::Res0Nx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RES0 IEEE Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_uf(&self) -> super::vals::Res0Uf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Res0Uf::from_bits(val as u8)
    }
    #[doc = "RES0 IEEE Underflow Flag"]
    #[inline(always)]
    pub const fn set_res0_uf(&mut self, val: super::vals::Res0Uf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "RES0 IEEE Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_of(&self) -> super::vals::Res0Of {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Res0Of::from_bits(val as u8)
    }
    #[doc = "RES0 IEEE Overflow Flag"]
    #[inline(always)]
    pub const fn set_res0_of(&mut self, val: super::vals::Res0Of) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "RES0 IEEE Divide by Zero Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_dz(&self) -> super::vals::Res0Dz {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Res0Dz::from_bits(val as u8)
    }
    #[doc = "RES0 IEEE Divide by Zero Flag"]
    #[inline(always)]
    pub const fn set_res0_dz(&mut self, val: super::vals::Res0Dz) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RES0 IEEE Invalid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_nv(&self) -> super::vals::Res0Nv {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Res0Nv::from_bits(val as u8)
    }
    #[doc = "RES0 IEEE Invalid Flag"]
    #[inline(always)]
    pub const fn set_res0_nv(&mut self, val: super::vals::Res0Nv) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "RES0 Indirect Operation Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_err(&self) -> super::vals::Res0Err {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Res0Err::from_bits(val as u8)
    }
    #[doc = "RES0 Indirect Operation Error Flag"]
    #[inline(always)]
    pub const fn set_res0_err(&mut self, val: super::vals::Res0Err) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RES0 Overwrite Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_ovwr(&self) -> super::vals::Res0Ovwr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Res0Ovwr::from_bits(val as u8)
    }
    #[doc = "RES0 Overwrite Flag"]
    #[inline(always)]
    pub const fn set_res0_ovwr(&mut self, val: super::vals::Res0Ovwr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RES0 Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_full(&self) -> super::vals::Res0Full {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Res0Full::from_bits(val as u8)
    }
    #[doc = "RES0 Full Flag"]
    #[inline(always)]
    pub const fn set_res0_full(&mut self, val: super::vals::Res0Full) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "RES1 IEEE Inexact Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_nx(&self) -> super::vals::Res1Nx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Res1Nx::from_bits(val as u8)
    }
    #[doc = "RES1 IEEE Inexact Flag"]
    #[inline(always)]
    pub const fn set_res1_nx(&mut self, val: super::vals::Res1Nx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RES1 IEEE Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_uf(&self) -> super::vals::Res1Uf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Res1Uf::from_bits(val as u8)
    }
    #[doc = "RES1 IEEE Underflow Flag"]
    #[inline(always)]
    pub const fn set_res1_uf(&mut self, val: super::vals::Res1Uf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "RES1 IEEE Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_of(&self) -> super::vals::Res1Of {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Res1Of::from_bits(val as u8)
    }
    #[doc = "RES1 IEEE Overflow Flag"]
    #[inline(always)]
    pub const fn set_res1_of(&mut self, val: super::vals::Res1Of) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "RES1 IEEE Divide by Zero Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_dz(&self) -> super::vals::Res1Dz {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Res1Dz::from_bits(val as u8)
    }
    #[doc = "RES1 IEEE Divide by Zero Flag"]
    #[inline(always)]
    pub const fn set_res1_dz(&mut self, val: super::vals::Res1Dz) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RES1 IEEE Invalid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_nv(&self) -> super::vals::Res1Nv {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Res1Nv::from_bits(val as u8)
    }
    #[doc = "RES1 IEEE Invalid Flag"]
    #[inline(always)]
    pub const fn set_res1_nv(&mut self, val: super::vals::Res1Nv) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "RES1 Indirect Operation Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_err(&self) -> super::vals::Res1Err {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Res1Err::from_bits(val as u8)
    }
    #[doc = "RES1 Indirect Operation Error Flag"]
    #[inline(always)]
    pub const fn set_res1_err(&mut self, val: super::vals::Res1Err) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "RES1 Overwrite Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_ovwr(&self) -> super::vals::Res1Ovwr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Res1Ovwr::from_bits(val as u8)
    }
    #[doc = "RES1 Overwrite Flag"]
    #[inline(always)]
    pub const fn set_res1_ovwr(&mut self, val: super::vals::Res1Ovwr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "RES1 Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_full(&self) -> super::vals::Res1Full {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Res1Full::from_bits(val as u8)
    }
    #[doc = "RES1 Full Flag"]
    #[inline(always)]
    pub const fn set_res1_full(&mut self, val: super::vals::Res1Full) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "RES2 IEEE Inexact Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_nx(&self) -> super::vals::Res2Nx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Res2Nx::from_bits(val as u8)
    }
    #[doc = "RES2 IEEE Inexact Flag"]
    #[inline(always)]
    pub const fn set_res2_nx(&mut self, val: super::vals::Res2Nx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "RES2 IEEE Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_uf(&self) -> super::vals::Res2Uf {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Res2Uf::from_bits(val as u8)
    }
    #[doc = "RES2 IEEE Underflow Flag"]
    #[inline(always)]
    pub const fn set_res2_uf(&mut self, val: super::vals::Res2Uf) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "RES2 IEEE Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_of(&self) -> super::vals::Res2Of {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Res2Of::from_bits(val as u8)
    }
    #[doc = "RES2 IEEE Overflow Flag"]
    #[inline(always)]
    pub const fn set_res2_of(&mut self, val: super::vals::Res2Of) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "RES2 IEEE Divide by Zero Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_dz(&self) -> super::vals::Res2Dz {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Res2Dz::from_bits(val as u8)
    }
    #[doc = "RES2 IEEE Divide by Zero Flag"]
    #[inline(always)]
    pub const fn set_res2_dz(&mut self, val: super::vals::Res2Dz) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "RES2 IEEE Invalid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_nv(&self) -> super::vals::Res2Nv {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Res2Nv::from_bits(val as u8)
    }
    #[doc = "RES2 IEEE Invalid Flag"]
    #[inline(always)]
    pub const fn set_res2_nv(&mut self, val: super::vals::Res2Nv) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "RES2 Indirect Operation Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_err(&self) -> super::vals::Res2Err {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Res2Err::from_bits(val as u8)
    }
    #[doc = "RES2 Indirect Operation Error Flag"]
    #[inline(always)]
    pub const fn set_res2_err(&mut self, val: super::vals::Res2Err) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "RES2 Overwrite Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_ovwr(&self) -> super::vals::Res2Ovwr {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Res2Ovwr::from_bits(val as u8)
    }
    #[doc = "RES2 Overwrite Flag"]
    #[inline(always)]
    pub const fn set_res2_ovwr(&mut self, val: super::vals::Res2Ovwr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "RES2 Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_full(&self) -> super::vals::Res2Full {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Res2Full::from_bits(val as u8)
    }
    #[doc = "RES2 Full Flag"]
    #[inline(always)]
    pub const fn set_res2_full(&mut self, val: super::vals::Res2Full) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "RES3 IEEE Inexact Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_nx(&self) -> super::vals::Res3Nx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Res3Nx::from_bits(val as u8)
    }
    #[doc = "RES3 IEEE Inexact Flag"]
    #[inline(always)]
    pub const fn set_res3_nx(&mut self, val: super::vals::Res3Nx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RES3 IEEE Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_uf(&self) -> super::vals::Res3Uf {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Res3Uf::from_bits(val as u8)
    }
    #[doc = "RES3 IEEE Underflow Flag"]
    #[inline(always)]
    pub const fn set_res3_uf(&mut self, val: super::vals::Res3Uf) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "RES3 IEEE Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_of(&self) -> super::vals::Res3Of {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Res3Of::from_bits(val as u8)
    }
    #[doc = "RES3 IEEE Overflow Flag"]
    #[inline(always)]
    pub const fn set_res3_of(&mut self, val: super::vals::Res3Of) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "RES3 IEEE Divide by Zero Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_dz(&self) -> super::vals::Res3Dz {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Res3Dz::from_bits(val as u8)
    }
    #[doc = "RES3 IEEE Divide by Zero Flag"]
    #[inline(always)]
    pub const fn set_res3_dz(&mut self, val: super::vals::Res3Dz) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "RES3 IEEE Invalid Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_nv(&self) -> super::vals::Res3Nv {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Res3Nv::from_bits(val as u8)
    }
    #[doc = "RES3 IEEE Invalid Flag"]
    #[inline(always)]
    pub const fn set_res3_nv(&mut self, val: super::vals::Res3Nv) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "RES3 Indirect Operation Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_err(&self) -> super::vals::Res3Err {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Res3Err::from_bits(val as u8)
    }
    #[doc = "RES3 Indirect Operation Error Flag"]
    #[inline(always)]
    pub const fn set_res3_err(&mut self, val: super::vals::Res3Err) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "RES3 Overwrite Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_ovwr(&self) -> super::vals::Res3Ovwr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Res3Ovwr::from_bits(val as u8)
    }
    #[doc = "RES3 Overwrite Flag"]
    #[inline(always)]
    pub const fn set_res3_ovwr(&mut self, val: super::vals::Res3Ovwr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "RES3 Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_full(&self) -> super::vals::Res3Full {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Res3Full::from_bits(val as u8)
    }
    #[doc = "RES3 Full Flag"]
    #[inline(always)]
    pub const fn set_res3_full(&mut self, val: super::vals::Res3Full) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ResStatus {
    #[inline(always)]
    fn default() -> ResStatus {
        ResStatus(0)
    }
}
impl core::fmt::Debug for ResStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ResStatus")
            .field("res0_nx", &self.res0_nx())
            .field("res0_uf", &self.res0_uf())
            .field("res0_of", &self.res0_of())
            .field("res0_dz", &self.res0_dz())
            .field("res0_nv", &self.res0_nv())
            .field("res0_err", &self.res0_err())
            .field("res0_ovwr", &self.res0_ovwr())
            .field("res0_full", &self.res0_full())
            .field("res1_nx", &self.res1_nx())
            .field("res1_uf", &self.res1_uf())
            .field("res1_of", &self.res1_of())
            .field("res1_dz", &self.res1_dz())
            .field("res1_nv", &self.res1_nv())
            .field("res1_err", &self.res1_err())
            .field("res1_ovwr", &self.res1_ovwr())
            .field("res1_full", &self.res1_full())
            .field("res2_nx", &self.res2_nx())
            .field("res2_uf", &self.res2_uf())
            .field("res2_of", &self.res2_of())
            .field("res2_dz", &self.res2_dz())
            .field("res2_nv", &self.res2_nv())
            .field("res2_err", &self.res2_err())
            .field("res2_ovwr", &self.res2_ovwr())
            .field("res2_full", &self.res2_full())
            .field("res3_nx", &self.res3_nx())
            .field("res3_uf", &self.res3_uf())
            .field("res3_of", &self.res3_of())
            .field("res3_dz", &self.res3_dz())
            .field("res3_nv", &self.res3_nv())
            .field("res3_err", &self.res3_err())
            .field("res3_ovwr", &self.res3_ovwr())
            .field("res3_full", &self.res3_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ResStatus {{ res0_nx: {:?}, res0_uf: {:?}, res0_of: {:?}, res0_dz: {:?}, res0_nv: {:?}, res0_err: {:?}, res0_ovwr: {:?}, res0_full: {:?}, res1_nx: {:?}, res1_uf: {:?}, res1_of: {:?}, res1_dz: {:?}, res1_nv: {:?}, res1_err: {:?}, res1_ovwr: {:?}, res1_full: {:?}, res2_nx: {:?}, res2_uf: {:?}, res2_of: {:?}, res2_dz: {:?}, res2_nv: {:?}, res2_err: {:?}, res2_ovwr: {:?}, res2_full: {:?}, res3_nx: {:?}, res3_uf: {:?}, res3_of: {:?}, res3_dz: {:?}, res3_nv: {:?}, res3_err: {:?}, res3_ovwr: {:?}, res3_full: {:?} }}",
            self.res0_nx(),
            self.res0_uf(),
            self.res0_of(),
            self.res0_dz(),
            self.res0_nv(),
            self.res0_err(),
            self.res0_ovwr(),
            self.res0_full(),
            self.res1_nx(),
            self.res1_uf(),
            self.res1_of(),
            self.res1_dz(),
            self.res1_nv(),
            self.res1_err(),
            self.res1_ovwr(),
            self.res1_full(),
            self.res2_nx(),
            self.res2_uf(),
            self.res2_of(),
            self.res2_dz(),
            self.res2_nv(),
            self.res2_err(),
            self.res2_ovwr(),
            self.res2_full(),
            self.res3_nx(),
            self.res3_uf(),
            self.res3_of(),
            self.res3_dz(),
            self.res3_nv(),
            self.res3_err(),
            self.res3_ovwr(),
            self.res3_full()
        )
    }
}
#[doc = "Result Status Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResStatusIe(pub u32);
impl ResStatusIe {
    #[doc = "RES0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn res0_ie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RES0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_res0_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RES1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn res1_ie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RES1 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_res1_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RES2 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn res2_ie(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RES2 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_res2_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RES3 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn res3_ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RES3 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_res3_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ResStatusIe {
    #[inline(always)]
    fn default() -> ResStatusIe {
        ResStatusIe(0)
    }
}
impl core::fmt::Debug for ResStatusIe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ResStatusIe")
            .field("res0_ie", &self.res0_ie())
            .field("res1_ie", &self.res1_ie())
            .field("res2_ie", &self.res2_ie())
            .field("res3_ie", &self.res3_ie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResStatusIe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ResStatusIe {{ res0_ie: {=bool:?}, res1_ie: {=bool:?}, res2_ie: {=bool:?}, res3_ie: {=bool:?} }}",
            self.res0_ie(),
            self.res1_ie(),
            self.res2_ie(),
            self.res3_ie()
        )
    }
}
#[doc = "System Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtlr(pub u32);
impl SysCtlr {
    #[doc = "Automatic Clock Gating Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn acg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Clock Gating Enable"]
    #[inline(always)]
    pub const fn set_acg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SysCtlr {
    #[inline(always)]
    fn default() -> SysCtlr {
        SysCtlr(0)
    }
}
impl core::fmt::Debug for SysCtlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysCtlr")
            .field("acg_en", &self.acg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysCtlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SysCtlr {{ acg_en: {=bool:?} }}", self.acg_en())
    }
}
