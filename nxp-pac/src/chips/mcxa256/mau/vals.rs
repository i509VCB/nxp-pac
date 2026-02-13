#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvdtRes0 {
    #[doc = "UINT"]
    UINT = 0x0,
    #[doc = "INT"]
    INT = 0x01,
    #[doc = "Q1.X"]
    Q1_X = 0x02,
    #[doc = "FLOAT"]
    FLOAT = 0x03,
}
impl OvdtRes0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvdtRes0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvdtRes0 {
    #[inline(always)]
    fn from(val: u8) -> OvdtRes0 {
        OvdtRes0::from_bits(val)
    }
}
impl From<OvdtRes0> for u8 {
    #[inline(always)]
    fn from(val: OvdtRes0) -> u8 {
        OvdtRes0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvdtRes1 {
    #[doc = "UINT"]
    UINT = 0x0,
    #[doc = "INT"]
    INT = 0x01,
    #[doc = "Q1.X"]
    Q1_X = 0x02,
    #[doc = "FLOAT"]
    FLOAT = 0x03,
}
impl OvdtRes1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvdtRes1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvdtRes1 {
    #[inline(always)]
    fn from(val: u8) -> OvdtRes1 {
        OvdtRes1::from_bits(val)
    }
}
impl From<OvdtRes1> for u8 {
    #[inline(always)]
    fn from(val: OvdtRes1) -> u8 {
        OvdtRes1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvdtRes2 {
    #[doc = "UINT"]
    UINT = 0x0,
    #[doc = "INT"]
    INT = 0x01,
    #[doc = "Q1.X"]
    Q1_X = 0x02,
    #[doc = "FLOAT"]
    FLOAT = 0x03,
}
impl OvdtRes2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvdtRes2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvdtRes2 {
    #[inline(always)]
    fn from(val: u8) -> OvdtRes2 {
        OvdtRes2::from_bits(val)
    }
}
impl From<OvdtRes2> for u8 {
    #[inline(always)]
    fn from(val: OvdtRes2) -> u8 {
        OvdtRes2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvdtRes3 {
    #[doc = "UINT"]
    UINT = 0x0,
    #[doc = "INT"]
    INT = 0x01,
    #[doc = "Q1.X"]
    Q1_X = 0x02,
    #[doc = "FLOAT"]
    FLOAT = 0x03,
}
impl OvdtRes3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvdtRes3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvdtRes3 {
    #[inline(always)]
    fn from(val: u8) -> OvdtRes3 {
        OvdtRes3::from_bits(val)
    }
}
impl From<OvdtRes3> for u8 {
    #[inline(always)]
    fn from(val: OvdtRes3) -> u8 {
        OvdtRes3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Dz {
    #[doc = "No exact infinite result is defined for an operation on finite operands."]
    RES0_DZ_NO = 0x0,
    #[doc = "An exact infinite result is defined for an operation on finite operands."]
    RES0_DZ_YES = 0x01,
}
impl Res0Dz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Dz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Dz {
    #[inline(always)]
    fn from(val: u8) -> Res0Dz {
        Res0Dz::from_bits(val)
    }
}
impl From<Res0Dz> for u8 {
    #[inline(always)]
    fn from(val: Res0Dz) -> u8 {
        Res0Dz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Err {
    #[doc = "No invalid indirect operation is detected."]
    RES0_ERR_NO = 0x0,
    #[doc = "An invalid indirect operation error is detected."]
    RES0_ERR_YES = 0x01,
}
impl Res0Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Err {
    #[inline(always)]
    fn from(val: u8) -> Res0Err {
        Res0Err::from_bits(val)
    }
}
impl From<Res0Err> for u8 {
    #[inline(always)]
    fn from(val: Res0Err) -> u8 {
        Res0Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Full {
    #[doc = "RES0 has not updated and cannot be read."]
    RES0_FULL_NO = 0x0,
    #[doc = "RES0 has updated and can be read."]
    RES0_FULL_YES = 0x01,
}
impl Res0Full {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Full {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Full {
    #[inline(always)]
    fn from(val: u8) -> Res0Full {
        Res0Full::from_bits(val)
    }
}
impl From<Res0Full> for u8 {
    #[inline(always)]
    fn from(val: Res0Full) -> u8 {
        Res0Full::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Nv {
    #[doc = "There is usefully definable result."]
    RES0_NV_NO = 0x0,
    #[doc = "There is no usefully definable result."]
    RES0_NV_YES = 0x01,
}
impl Res0Nv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Nv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Nv {
    #[inline(always)]
    fn from(val: u8) -> Res0Nv {
        Res0Nv::from_bits(val)
    }
}
impl From<Res0Nv> for u8 {
    #[inline(always)]
    fn from(val: Res0Nv) -> u8 {
        Res0Nv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Nx {
    #[doc = "The result is not rounded."]
    RES0_NX_NO = 0x0,
    #[doc = "The result is rounded, and as a result some digits lost."]
    RES0_NX_YES = 0x01,
}
impl Res0Nx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Nx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Nx {
    #[inline(always)]
    fn from(val: u8) -> Res0Nx {
        Res0Nx::from_bits(val)
    }
}
impl From<Res0Nx> for u8 {
    #[inline(always)]
    fn from(val: Res0Nx) -> u8 {
        Res0Nx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Of {
    #[doc = "The result format's largest finite number is not exceeded."]
    RES0_OF_NO = 0x0,
    #[doc = "The result format's largest finite number is exceeded."]
    RES0_OF_YES = 0x01,
}
impl Res0Of {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Of {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Of {
    #[inline(always)]
    fn from(val: u8) -> Res0Of {
        Res0Of::from_bits(val)
    }
}
impl From<Res0Of> for u8 {
    #[inline(always)]
    fn from(val: Res0Of) -> u8 {
        Res0Of::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Ovwr {
    #[doc = "The value of RES0 has been read."]
    RES0_OVWR_NO = 0x0,
    #[doc = "The value of RES0 has not been read yet and is overwritten by a new MAUWRAP result."]
    RES0_OVWR_YES = 0x01,
}
impl Res0Ovwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Ovwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Ovwr {
    #[inline(always)]
    fn from(val: u8) -> Res0Ovwr {
        Res0Ovwr::from_bits(val)
    }
}
impl From<Res0Ovwr> for u8 {
    #[inline(always)]
    fn from(val: Res0Ovwr) -> u8 {
        Res0Ovwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res0Uf {
    #[doc = "No tiny non-zero result is detected."]
    RES0_UF_NO = 0x0,
    #[doc = "A tiny non-zero result is detected."]
    RES0_UF_YES = 0x01,
}
impl Res0Uf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res0Uf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res0Uf {
    #[inline(always)]
    fn from(val: u8) -> Res0Uf {
        Res0Uf::from_bits(val)
    }
}
impl From<Res0Uf> for u8 {
    #[inline(always)]
    fn from(val: Res0Uf) -> u8 {
        Res0Uf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Dz {
    #[doc = "No exact infinite result is defined for an operation on finite operands."]
    RES1_DZ_NO = 0x0,
    #[doc = "An exact infinite result is defined for an operation on finite operands."]
    RES1_DZ_YES = 0x01,
}
impl Res1Dz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Dz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Dz {
    #[inline(always)]
    fn from(val: u8) -> Res1Dz {
        Res1Dz::from_bits(val)
    }
}
impl From<Res1Dz> for u8 {
    #[inline(always)]
    fn from(val: Res1Dz) -> u8 {
        Res1Dz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Err {
    #[doc = "No invalid indirect operation is detected."]
    RES1_ERR_NO = 0x0,
    #[doc = "An invalid indirect operation error is detected."]
    RES1_ERR_YES = 0x01,
}
impl Res1Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Err {
    #[inline(always)]
    fn from(val: u8) -> Res1Err {
        Res1Err::from_bits(val)
    }
}
impl From<Res1Err> for u8 {
    #[inline(always)]
    fn from(val: Res1Err) -> u8 {
        Res1Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Full {
    #[doc = "RES1 has not updated and cannot be read."]
    RES1_FULL_NO = 0x0,
    #[doc = "RES1 has updated and can be read."]
    RES1_FULL_YES = 0x01,
}
impl Res1Full {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Full {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Full {
    #[inline(always)]
    fn from(val: u8) -> Res1Full {
        Res1Full::from_bits(val)
    }
}
impl From<Res1Full> for u8 {
    #[inline(always)]
    fn from(val: Res1Full) -> u8 {
        Res1Full::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Nv {
    #[doc = "There is usefully definable result."]
    RES1_NV_NO = 0x0,
    #[doc = "There is no usefully definable result."]
    RES1_NV_YES = 0x01,
}
impl Res1Nv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Nv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Nv {
    #[inline(always)]
    fn from(val: u8) -> Res1Nv {
        Res1Nv::from_bits(val)
    }
}
impl From<Res1Nv> for u8 {
    #[inline(always)]
    fn from(val: Res1Nv) -> u8 {
        Res1Nv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Nx {
    #[doc = "The result is not rounded."]
    RES1_NX_NO = 0x0,
    #[doc = "The result is rounded, and as a result some digits lost."]
    RES1_NX_YES = 0x01,
}
impl Res1Nx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Nx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Nx {
    #[inline(always)]
    fn from(val: u8) -> Res1Nx {
        Res1Nx::from_bits(val)
    }
}
impl From<Res1Nx> for u8 {
    #[inline(always)]
    fn from(val: Res1Nx) -> u8 {
        Res1Nx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Of {
    #[doc = "The result format's largest finite number is not exceeded."]
    RES1_OF_NO = 0x0,
    #[doc = "The result format's largest finite number is exceeded."]
    RES1_OF_YES = 0x01,
}
impl Res1Of {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Of {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Of {
    #[inline(always)]
    fn from(val: u8) -> Res1Of {
        Res1Of::from_bits(val)
    }
}
impl From<Res1Of> for u8 {
    #[inline(always)]
    fn from(val: Res1Of) -> u8 {
        Res1Of::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Ovwr {
    #[doc = "The value of RES1 has been read."]
    RES1_OVWR_NO = 0x0,
    #[doc = "The value of RES1 has not been read yet and is overwritten by a new MAUWRAP result."]
    RES1_OVWR_YES = 0x01,
}
impl Res1Ovwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Ovwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Ovwr {
    #[inline(always)]
    fn from(val: u8) -> Res1Ovwr {
        Res1Ovwr::from_bits(val)
    }
}
impl From<Res1Ovwr> for u8 {
    #[inline(always)]
    fn from(val: Res1Ovwr) -> u8 {
        Res1Ovwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res1Uf {
    #[doc = "No tiny non-zero result is detected."]
    RES1_UF_NO = 0x0,
    #[doc = "A tiny non-zero result is detected."]
    RES1_UF_YES = 0x01,
}
impl Res1Uf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res1Uf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res1Uf {
    #[inline(always)]
    fn from(val: u8) -> Res1Uf {
        Res1Uf::from_bits(val)
    }
}
impl From<Res1Uf> for u8 {
    #[inline(always)]
    fn from(val: Res1Uf) -> u8 {
        Res1Uf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Dz {
    #[doc = "No exact infinite result is defined for an operation on finite operands."]
    RES2_DZ_NO = 0x0,
    #[doc = "An exact infinite result is defined for an operation on finite operands."]
    RES2_DZ_YES = 0x01,
}
impl Res2Dz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Dz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Dz {
    #[inline(always)]
    fn from(val: u8) -> Res2Dz {
        Res2Dz::from_bits(val)
    }
}
impl From<Res2Dz> for u8 {
    #[inline(always)]
    fn from(val: Res2Dz) -> u8 {
        Res2Dz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Err {
    #[doc = "No invalid indirect operation is detected."]
    RES2_ERR_NO = 0x0,
    #[doc = "An invalid indirect operation error is detected."]
    RES2_ERR_YES = 0x01,
}
impl Res2Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Err {
    #[inline(always)]
    fn from(val: u8) -> Res2Err {
        Res2Err::from_bits(val)
    }
}
impl From<Res2Err> for u8 {
    #[inline(always)]
    fn from(val: Res2Err) -> u8 {
        Res2Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Full {
    #[doc = "RES2 has not updated and cannot be read."]
    RES2_FULL_NO = 0x0,
    #[doc = "RES2 has updated and can be read."]
    RES2_FULL_YES = 0x01,
}
impl Res2Full {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Full {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Full {
    #[inline(always)]
    fn from(val: u8) -> Res2Full {
        Res2Full::from_bits(val)
    }
}
impl From<Res2Full> for u8 {
    #[inline(always)]
    fn from(val: Res2Full) -> u8 {
        Res2Full::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Nv {
    #[doc = "There is usefully definable result."]
    RES2_NV_NO = 0x0,
    #[doc = "There is no usefully definable result."]
    RES2_NV_YES = 0x01,
}
impl Res2Nv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Nv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Nv {
    #[inline(always)]
    fn from(val: u8) -> Res2Nv {
        Res2Nv::from_bits(val)
    }
}
impl From<Res2Nv> for u8 {
    #[inline(always)]
    fn from(val: Res2Nv) -> u8 {
        Res2Nv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Nx {
    #[doc = "The result is not rounded."]
    RES2_NX_NO = 0x0,
    #[doc = "The result is rounded, and as a result some digits lost."]
    RES2_NX_YES = 0x01,
}
impl Res2Nx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Nx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Nx {
    #[inline(always)]
    fn from(val: u8) -> Res2Nx {
        Res2Nx::from_bits(val)
    }
}
impl From<Res2Nx> for u8 {
    #[inline(always)]
    fn from(val: Res2Nx) -> u8 {
        Res2Nx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Of {
    #[doc = "The result format's largest finite number is not exceeded."]
    RES2_OF_NO = 0x0,
    #[doc = "The result format's largest finite number is exceeded."]
    RES2_OF_YES = 0x01,
}
impl Res2Of {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Of {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Of {
    #[inline(always)]
    fn from(val: u8) -> Res2Of {
        Res2Of::from_bits(val)
    }
}
impl From<Res2Of> for u8 {
    #[inline(always)]
    fn from(val: Res2Of) -> u8 {
        Res2Of::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Ovwr {
    #[doc = "The value of RES2 has been read."]
    RES2_OVWR_NO = 0x0,
    #[doc = "The value of RES2 has not been read yet and is overwritten by a new MAUWRAP result."]
    RES2_OVWR_YES = 0x01,
}
impl Res2Ovwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Ovwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Ovwr {
    #[inline(always)]
    fn from(val: u8) -> Res2Ovwr {
        Res2Ovwr::from_bits(val)
    }
}
impl From<Res2Ovwr> for u8 {
    #[inline(always)]
    fn from(val: Res2Ovwr) -> u8 {
        Res2Ovwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res2Uf {
    #[doc = "No tiny non-zero result is detected."]
    RES2_UF_NO = 0x0,
    #[doc = "A tiny non-zero result is detected."]
    RES2_UF_YES = 0x01,
}
impl Res2Uf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res2Uf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res2Uf {
    #[inline(always)]
    fn from(val: u8) -> Res2Uf {
        Res2Uf::from_bits(val)
    }
}
impl From<Res2Uf> for u8 {
    #[inline(always)]
    fn from(val: Res2Uf) -> u8 {
        Res2Uf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Dz {
    #[doc = "No exact infinite result is defined for an operation on finite operands."]
    RES3_DZ_NO = 0x0,
    #[doc = "An exact infinite result is defined for an operation on finite operands."]
    RES3_DZ_YES = 0x01,
}
impl Res3Dz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Dz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Dz {
    #[inline(always)]
    fn from(val: u8) -> Res3Dz {
        Res3Dz::from_bits(val)
    }
}
impl From<Res3Dz> for u8 {
    #[inline(always)]
    fn from(val: Res3Dz) -> u8 {
        Res3Dz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Err {
    #[doc = "No invalid indirect operation is detected."]
    RES3_ERR_NO = 0x0,
    #[doc = "An invalid indirect operation error is detected."]
    RES3_ERR_YES = 0x01,
}
impl Res3Err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Err {
    #[inline(always)]
    fn from(val: u8) -> Res3Err {
        Res3Err::from_bits(val)
    }
}
impl From<Res3Err> for u8 {
    #[inline(always)]
    fn from(val: Res3Err) -> u8 {
        Res3Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Full {
    #[doc = "RES3 has not updated and cannot be read."]
    RES3_FULL_NO = 0x0,
    #[doc = "RES3 has updated and can be read."]
    RES3_FULL_YES = 0x01,
}
impl Res3Full {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Full {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Full {
    #[inline(always)]
    fn from(val: u8) -> Res3Full {
        Res3Full::from_bits(val)
    }
}
impl From<Res3Full> for u8 {
    #[inline(always)]
    fn from(val: Res3Full) -> u8 {
        Res3Full::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Nv {
    #[doc = "There is usefully definable result."]
    RES3_NV_NO = 0x0,
    #[doc = "There is no usefully definable result."]
    RES3_NV_YES = 0x01,
}
impl Res3Nv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Nv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Nv {
    #[inline(always)]
    fn from(val: u8) -> Res3Nv {
        Res3Nv::from_bits(val)
    }
}
impl From<Res3Nv> for u8 {
    #[inline(always)]
    fn from(val: Res3Nv) -> u8 {
        Res3Nv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Nx {
    #[doc = "The result is not rounded."]
    RES3_NX_NO = 0x0,
    #[doc = "The result is rounded, and as a result some digits lost."]
    RES3_NX_YES = 0x01,
}
impl Res3Nx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Nx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Nx {
    #[inline(always)]
    fn from(val: u8) -> Res3Nx {
        Res3Nx::from_bits(val)
    }
}
impl From<Res3Nx> for u8 {
    #[inline(always)]
    fn from(val: Res3Nx) -> u8 {
        Res3Nx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Of {
    #[doc = "The result format's largest finite number is not exceeded."]
    RES3_OF_NO = 0x0,
    #[doc = "The result format's largest finite number is exceeded."]
    RES3_OF_YES = 0x01,
}
impl Res3Of {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Of {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Of {
    #[inline(always)]
    fn from(val: u8) -> Res3Of {
        Res3Of::from_bits(val)
    }
}
impl From<Res3Of> for u8 {
    #[inline(always)]
    fn from(val: Res3Of) -> u8 {
        Res3Of::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Ovwr {
    #[doc = "The value of RES3 has been read."]
    RES3_OVWR_NO = 0x0,
    #[doc = "The value of RES3 has not been read yet and is overwritten by a new MAUWRAP result."]
    RES3_OVWR_YES = 0x01,
}
impl Res3Ovwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Ovwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Ovwr {
    #[inline(always)]
    fn from(val: u8) -> Res3Ovwr {
        Res3Ovwr::from_bits(val)
    }
}
impl From<Res3Ovwr> for u8 {
    #[inline(always)]
    fn from(val: Res3Ovwr) -> u8 {
        Res3Ovwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res3Uf {
    #[doc = "No tiny non-zero result is detected."]
    RES3_UF_NO = 0x0,
    #[doc = "A tiny non-zero result is detected."]
    RES3_UF_YES = 0x01,
}
impl Res3Uf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res3Uf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res3Uf {
    #[inline(always)]
    fn from(val: u8) -> Res3Uf {
        Res3Uf::from_bits(val)
    }
}
impl From<Res3Uf> for u8 {
    #[inline(always)]
    fn from(val: Res3Uf) -> u8 {
        Res3Uf::to_bits(val)
    }
}
