#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Afs {
    #[doc = "Abs Diff on consecutive green pixels."]
    Afs0 = 0x0,
    #[doc = "Abs Diff on every third green pixels."]
    Afs1 = 0x01,
    #[doc = "Abs Diff on every four green pixels."]
    Afs2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Afs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Afs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Afs {
    #[inline(always)]
    fn from(val: u8) -> Afs {
        Afs::from_bits(val)
    }
}
impl From<Afs> for u8 {
    #[inline(always)]
    fn from(val: Afs) -> u8 {
        Afs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseaddrChangeErrorIe {
    #[doc = "Interrupt disabled."]
    BaseaddrChangeErrorIe0 = 0x0,
    #[doc = "Interrupt enabled."]
    BaseaddrChangeErrorIe1 = 0x01,
}
impl BaseaddrChangeErrorIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseaddrChangeErrorIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseaddrChangeErrorIe {
    #[inline(always)]
    fn from(val: u8) -> BaseaddrChangeErrorIe {
        BaseaddrChangeErrorIe::from_bits(val)
    }
}
impl From<BaseaddrChangeErrorIe> for u8 {
    #[inline(always)]
    fn from(val: BaseaddrChangeErrorIe) -> u8 {
        BaseaddrChangeErrorIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BaseaddrSwitchSel {
    #[doc = "Switching base address at the edge of the vsync."]
    BaseaddrSwitchSel0 = 0x0,
    #[doc = "Switching base address at the edge of the first data of each frame."]
    BaseaddrSwitchSel1 = 0x01,
}
impl BaseaddrSwitchSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BaseaddrSwitchSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BaseaddrSwitchSel {
    #[inline(always)]
    fn from(val: u8) -> BaseaddrSwitchSel {
        BaseaddrSwitchSel::from_bits(val)
    }
}
impl From<BaseaddrSwitchSel> for u8 {
    #[inline(always)]
    fn from(val: BaseaddrSwitchSel) -> u8 {
        BaseaddrSwitchSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bts {
    #[doc = "GR."]
    Bts0 = 0x0,
    #[doc = "RG."]
    Bts1 = 0x01,
    #[doc = "BG."]
    Bts2 = 0x02,
    #[doc = "GB."]
    Bts3 = 0x03,
}
impl Bts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bts {
    #[inline(always)]
    fn from(val: u8) -> Bts {
        Bts::from_bits(val)
    }
}
impl From<Bts> for u8 {
    #[inline(always)]
    fn from(val: Bts) -> u8 {
        Bts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcirEn {
    #[doc = "Traditional interface is selected."]
    CcirEn0 = 0x0,
    #[doc = "BT.656 interface is selected."]
    CcirEn1 = 0x01,
}
impl CcirEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcirEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcirEn {
    #[inline(always)]
    fn from(val: u8) -> CcirEn {
        CcirEn::from_bits(val)
    }
}
impl From<CcirEn> for u8 {
    #[inline(always)]
    fn from(val: CcirEn) -> u8 {
        CcirEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CcirMode {
    #[doc = "Progressive mode is selected."]
    CcirMode0 = 0x0,
    #[doc = "Interlace mode is selected."]
    CcirMode1 = 0x01,
}
impl CcirMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CcirMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CcirMode {
    #[inline(always)]
    fn from(val: u8) -> CcirMode {
        CcirMode::from_bits(val)
    }
}
impl From<CcirMode> for u8 {
    #[inline(always)]
    fn from(val: CcirMode) -> u8 {
        CcirMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CofInt {
    #[doc = "Video field has no change."]
    CofInt0 = 0x0,
    #[doc = "Change of video field is detected."]
    CofInt1 = 0x01,
}
impl CofInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CofInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CofInt {
    #[inline(always)]
    fn from(val: u8) -> CofInt {
        CofInt::from_bits(val)
    }
}
impl From<CofInt> for u8 {
    #[inline(always)]
    fn from(val: CofInt) -> u8 {
        CofInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CofIntEn {
    #[doc = "COF interrupt is disabled."]
    CofIntEn0 = 0x0,
    #[doc = "COF interrupt is enabled."]
    CofIntEn1 = 0x01,
}
impl CofIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CofIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CofIntEn {
    #[inline(always)]
    fn from(val: u8) -> CofIntEn {
        CofIntEn::from_bits(val)
    }
}
impl From<CofIntEn> for u8 {
    #[inline(always)]
    fn from(val: CofIntEn) -> u8 {
        CofIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeinterlaceEn {
    #[doc = "Deinterlace disabled."]
    DeinterlaceEn0 = 0x0,
    #[doc = "Deinterlace enabled."]
    DeinterlaceEn1 = 0x01,
}
impl DeinterlaceEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeinterlaceEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeinterlaceEn {
    #[inline(always)]
    fn from(val: u8) -> DeinterlaceEn {
        DeinterlaceEn::from_bits(val)
    }
}
impl From<DeinterlaceEn> for u8 {
    #[inline(always)]
    fn from(val: DeinterlaceEn) -> u8 {
        DeinterlaceEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaBurstTypeRff {
    #[doc = "INCR8."]
    DmaBurstTypeRff0 = 0x0,
    #[doc = "INCR4."]
    DmaBurstTypeRff1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "INCR16."]
    DmaBurstTypeRff3 = 0x03,
}
impl DmaBurstTypeRff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaBurstTypeRff {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaBurstTypeRff {
    #[inline(always)]
    fn from(val: u8) -> DmaBurstTypeRff {
        DmaBurstTypeRff::from_bits(val)
    }
}
impl From<DmaBurstTypeRff> for u8 {
    #[inline(always)]
    fn from(val: DmaBurstTypeRff) -> u8 {
        DmaBurstTypeRff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaBurstTypeSff {
    #[doc = "INCR8."]
    DmaBurstTypeSff0 = 0x0,
    #[doc = "INCR4."]
    DmaBurstTypeSff1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "INCR16."]
    DmaBurstTypeSff3 = 0x03,
}
impl DmaBurstTypeSff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaBurstTypeSff {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaBurstTypeSff {
    #[inline(always)]
    fn from(val: u8) -> DmaBurstTypeSff {
        DmaBurstTypeSff::from_bits(val)
    }
}
impl From<DmaBurstTypeSff> for u8 {
    #[inline(always)]
    fn from(val: DmaBurstTypeSff) -> u8 {
        DmaBurstTypeSff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaField1DoneIe {
    #[doc = "Interrupt disabled."]
    DmaField1DoneIe0 = 0x0,
    #[doc = "Interrupt enabled."]
    DmaField1DoneIe1 = 0x01,
}
impl DmaField1DoneIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaField1DoneIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaField1DoneIe {
    #[inline(always)]
    fn from(val: u8) -> DmaField1DoneIe {
        DmaField1DoneIe::from_bits(val)
    }
}
impl From<DmaField1DoneIe> for u8 {
    #[inline(always)]
    fn from(val: DmaField1DoneIe) -> u8 {
        DmaField1DoneIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaReflashRff {
    #[doc = "No reflashing."]
    DmaReflashRff0 = 0x0,
    #[doc = "Reflash the embedded DMA controller."]
    DmaReflashRff1 = 0x01,
}
impl DmaReflashRff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaReflashRff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaReflashRff {
    #[inline(always)]
    fn from(val: u8) -> DmaReflashRff {
        DmaReflashRff::from_bits(val)
    }
}
impl From<DmaReflashRff> for u8 {
    #[inline(always)]
    fn from(val: DmaReflashRff) -> u8 {
        DmaReflashRff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaReflashSff {
    #[doc = "No reflashing."]
    DmaReflashSff0 = 0x0,
    #[doc = "Reflash the embedded DMA controller."]
    DmaReflashSff1 = 0x01,
}
impl DmaReflashSff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaReflashSff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaReflashSff {
    #[inline(always)]
    fn from(val: u8) -> DmaReflashSff {
        DmaReflashSff::from_bits(val)
    }
}
impl From<DmaReflashSff> for u8 {
    #[inline(always)]
    fn from(val: DmaReflashSff) -> u8 {
        DmaReflashSff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaReqEnRff {
    #[doc = "Disable the dma request."]
    DmaReqEnRff0 = 0x0,
    #[doc = "Enable the dma request."]
    DmaReqEnRff1 = 0x01,
}
impl DmaReqEnRff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaReqEnRff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaReqEnRff {
    #[inline(always)]
    fn from(val: u8) -> DmaReqEnRff {
        DmaReqEnRff::from_bits(val)
    }
}
impl From<DmaReqEnRff> for u8 {
    #[inline(always)]
    fn from(val: DmaReqEnRff) -> u8 {
        DmaReqEnRff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaReqEnSff {
    #[doc = "Disable the dma request."]
    DmaReqEnSff0 = 0x0,
    #[doc = "Enable the dma request."]
    DmaReqEnSff1 = 0x01,
}
impl DmaReqEnSff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaReqEnSff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaReqEnSff {
    #[inline(always)]
    fn from(val: u8) -> DmaReqEnSff {
        DmaReqEnSff::from_bits(val)
    }
}
impl From<DmaReqEnSff> for u8 {
    #[inline(always)]
    fn from(val: DmaReqEnSff) -> u8 {
        DmaReqEnSff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaTsfDoneFb1 {
    #[doc = "DMA transfer is not completed."]
    DmaTsfDoneFb10 = 0x0,
    #[doc = "DMA transfer is completed."]
    DmaTsfDoneFb11 = 0x01,
}
impl DmaTsfDoneFb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaTsfDoneFb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaTsfDoneFb1 {
    #[inline(always)]
    fn from(val: u8) -> DmaTsfDoneFb1 {
        DmaTsfDoneFb1::from_bits(val)
    }
}
impl From<DmaTsfDoneFb1> for u8 {
    #[inline(always)]
    fn from(val: DmaTsfDoneFb1) -> u8 {
        DmaTsfDoneFb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaTsfDoneFb2 {
    #[doc = "DMA transfer is not completed."]
    DmaTsfDoneFb20 = 0x0,
    #[doc = "DMA transfer is completed."]
    DmaTsfDoneFb21 = 0x01,
}
impl DmaTsfDoneFb2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaTsfDoneFb2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaTsfDoneFb2 {
    #[inline(always)]
    fn from(val: u8) -> DmaTsfDoneFb2 {
        DmaTsfDoneFb2::from_bits(val)
    }
}
impl From<DmaTsfDoneFb2> for u8 {
    #[inline(always)]
    fn from(val: DmaTsfDoneFb2) -> u8 {
        DmaTsfDoneFb2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaTsfDoneSff {
    #[doc = "DMA transfer is not completed."]
    DmaTsfDoneSff0 = 0x0,
    #[doc = "DMA transfer is completed."]
    DmaTsfDoneSff1 = 0x01,
}
impl DmaTsfDoneSff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaTsfDoneSff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaTsfDoneSff {
    #[inline(always)]
    fn from(val: u8) -> DmaTsfDoneSff {
        DmaTsfDoneSff::from_bits(val)
    }
}
impl From<DmaTsfDoneSff> for u8 {
    #[inline(always)]
    fn from(val: DmaTsfDoneSff) -> u8 {
        DmaTsfDoneSff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drdy {
    #[doc = "No data (word) is ready."]
    Drdy0 = 0x0,
    #[doc = "At least 1 datum (word) is ready in RXFIFO."]
    Drdy1 = 0x01,
}
impl Drdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drdy {
    #[inline(always)]
    fn from(val: u8) -> Drdy {
        Drdy::from_bits(val)
    }
}
impl From<Drdy> for u8 {
    #[inline(always)]
    fn from(val: Drdy) -> u8 {
        Drdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Drm {
    #[doc = "Stats grid of 8 x 6."]
    Drm0 = 0x0,
    #[doc = "Stats grid of 8 x 12."]
    Drm1 = 0x01,
}
impl Drm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Drm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Drm {
    #[inline(always)]
    fn from(val: u8) -> Drm {
        Drm::from_bits(val)
    }
}
impl From<Drm> for u8 {
    #[inline(always)]
    fn from(val: Drm) -> u8 {
        Drm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccAutoEn {
    #[doc = "Auto Error correction is disabled."]
    EccAutoEn0 = 0x0,
    #[doc = "Auto Error correction is enabled."]
    EccAutoEn1 = 0x01,
}
impl EccAutoEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccAutoEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccAutoEn {
    #[inline(always)]
    fn from(val: u8) -> EccAutoEn {
        EccAutoEn::from_bits(val)
    }
}
impl From<EccAutoEn> for u8 {
    #[inline(always)]
    fn from(val: EccAutoEn) -> u8 {
        EccAutoEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccInt {
    #[doc = "No error detected."]
    EccInt0 = 0x0,
    #[doc = "Error is detected in BT.656 coding."]
    EccInt1 = 0x01,
}
impl EccInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccInt {
    #[inline(always)]
    fn from(val: u8) -> EccInt {
        EccInt::from_bits(val)
    }
}
impl From<EccInt> for u8 {
    #[inline(always)]
    fn from(val: EccInt) -> u8 {
        EccInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccIntEn {
    #[doc = "No interrupt is generated when error is detected. Only the status bit ECC_INT is set."]
    EccIntEn0 = 0x0,
    #[doc = "Interrupt is generated when error is detected."]
    EccIntEn1 = 0x01,
}
impl EccIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccIntEn {
    #[inline(always)]
    fn from(val: u8) -> EccIntEn {
        EccIntEn::from_bits(val)
    }
}
impl From<EccIntEn> for u8 {
    #[inline(always)]
    fn from(val: EccIntEn) -> u8 {
        EccIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EofInt {
    #[doc = "EOF is not detected."]
    EofInt0 = 0x0,
    #[doc = "EOF is detected."]
    EofInt1 = 0x01,
}
impl EofInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EofInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EofInt {
    #[inline(always)]
    fn from(val: u8) -> EofInt {
        EofInt::from_bits(val)
    }
}
impl From<EofInt> for u8 {
    #[inline(always)]
    fn from(val: EofInt) -> u8 {
        EofInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EofIntEn {
    #[doc = "EOF interrupt is disabled."]
    EofIntEn0 = 0x0,
    #[doc = "EOF interrupt is generated when RX count value is reached."]
    EofIntEn1 = 0x01,
}
impl EofIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EofIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EofIntEn {
    #[inline(always)]
    fn from(val: u8) -> EofIntEn {
        EofIntEn::from_bits(val)
    }
}
impl From<EofIntEn> for u8 {
    #[inline(always)]
    fn from(val: EofIntEn) -> u8 {
        EofIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtVsync {
    #[doc = "Internal VSYNC mode."]
    ExtVsync0 = 0x0,
    #[doc = "External VSYNC mode."]
    ExtVsync1 = 0x01,
}
impl ExtVsync {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtVsync {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtVsync {
    #[inline(always)]
    fn from(val: u8) -> ExtVsync {
        ExtVsync::from_bits(val)
    }
}
impl From<ExtVsync> for u8 {
    #[inline(always)]
    fn from(val: ExtVsync) -> u8 {
        ExtVsync::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum F1Int {
    #[doc = "Field 1 of video is not detected."]
    F1Int0 = 0x0,
    #[doc = "Field 1 of video is about to start."]
    F1Int1 = 0x01,
}
impl F1Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> F1Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for F1Int {
    #[inline(always)]
    fn from(val: u8) -> F1Int {
        F1Int::from_bits(val)
    }
}
impl From<F1Int> for u8 {
    #[inline(always)]
    fn from(val: F1Int) -> u8 {
        F1Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum F2Int {
    #[doc = "Field 2 of video is not detected."]
    F2Int0 = 0x0,
    #[doc = "Field 2 of video is about to start."]
    F2Int1 = 0x01,
}
impl F2Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> F2Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for F2Int {
    #[inline(always)]
    fn from(val: u8) -> F2Int {
        F2Int::from_bits(val)
    }
}
impl From<F2Int> for u8 {
    #[inline(always)]
    fn from(val: F2Int) -> u8 {
        F2Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fb1DmaDoneInten {
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt disable."]
    Fb1DmaDoneInten0 = 0x0,
    #[doc = "Frame Buffer1 DMA Transfer Done interrupt enable."]
    Fb1DmaDoneInten1 = 0x01,
}
impl Fb1DmaDoneInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fb1DmaDoneInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fb1DmaDoneInten {
    #[inline(always)]
    fn from(val: u8) -> Fb1DmaDoneInten {
        Fb1DmaDoneInten::from_bits(val)
    }
}
impl From<Fb1DmaDoneInten> for u8 {
    #[inline(always)]
    fn from(val: Fb1DmaDoneInten) -> u8 {
        Fb1DmaDoneInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fb2DmaDoneInten {
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt disable."]
    Fb2DmaDoneInten0 = 0x0,
    #[doc = "Frame Buffer2 DMA Transfer Done interrupt enable."]
    Fb2DmaDoneInten1 = 0x01,
}
impl Fb2DmaDoneInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fb2DmaDoneInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fb2DmaDoneInten {
    #[inline(always)]
    fn from(val: u8) -> Fb2DmaDoneInten {
        Fb2DmaDoneInten::from_bits(val)
    }
}
impl From<Fb2DmaDoneInten> for u8 {
    #[inline(always)]
    fn from(val: Fb2DmaDoneInten) -> u8 {
        Fb2DmaDoneInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fcc {
    #[doc = "Asynchronous FIFO clear is selected."]
    Fcc0 = 0x0,
    #[doc = "Synchronous FIFO clear is selected."]
    Fcc1 = 0x01,
}
impl Fcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fcc {
    #[inline(always)]
    fn from(val: u8) -> Fcc {
        Fcc::from_bits(val)
    }
}
impl From<Fcc> for u8 {
    #[inline(always)]
    fn from(val: Fcc) -> u8 {
        Fcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Field0DoneIe {
    #[doc = "Interrupt disabled."]
    Field0DoneIe0 = 0x0,
    #[doc = "Interrupt enabled."]
    Field0DoneIe1 = 0x01,
}
impl Field0DoneIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Field0DoneIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Field0DoneIe {
    #[inline(always)]
    fn from(val: u8) -> Field0DoneIe {
        Field0DoneIe::from_bits(val)
    }
}
impl From<Field0DoneIe> for u8 {
    #[inline(always)]
    fn from(val: Field0DoneIe) -> u8 {
        Field0DoneIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrmcntRst {
    #[doc = "Do not reset."]
    FrmcntRst0 = 0x0,
    #[doc = "Reset frame counter immediately."]
    FrmcntRst1 = 0x01,
}
impl FrmcntRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrmcntRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrmcntRst {
    #[inline(always)]
    fn from(val: u8) -> FrmcntRst {
        FrmcntRst::from_bits(val)
    }
}
impl From<FrmcntRst> for u8 {
    #[inline(always)]
    fn from(val: FrmcntRst) -> u8 {
        FrmcntRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GclkMode {
    #[doc = "Non-gated clock mode. All incoming pixel clocks are valid. HSYNC is ignored."]
    GclkMode0 = 0x0,
    #[doc = "Gated clock mode. Pixel clock signal is valid only when HSYNC is active."]
    GclkMode1 = 0x01,
}
impl GclkMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GclkMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GclkMode {
    #[inline(always)]
    fn from(val: u8) -> GclkMode {
        GclkMode::from_bits(val)
    }
}
impl From<GclkMode> for u8 {
    #[inline(always)]
    fn from(val: GclkMode) -> u8 {
        GclkMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HrespErrEn {
    #[doc = "Disable hresponse error interrupt."]
    HrespErrEn0 = 0x0,
    #[doc = "Enable hresponse error interrupt."]
    HrespErrEn1 = 0x01,
}
impl HrespErrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HrespErrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HrespErrEn {
    #[inline(always)]
    fn from(val: u8) -> HrespErrEn {
        HrespErrEn::from_bits(val)
    }
}
impl From<HrespErrEn> for u8 {
    #[inline(always)]
    fn from(val: HrespErrEn) -> u8 {
        HrespErrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HrespErrInt {
    #[doc = "No hresponse error."]
    HrespErrInt0 = 0x0,
    #[doc = "Hresponse error is detected."]
    HrespErrInt1 = 0x01,
}
impl HrespErrInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HrespErrInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HrespErrInt {
    #[inline(always)]
    fn from(val: u8) -> HrespErrInt {
        HrespErrInt::from_bits(val)
    }
}
impl From<HrespErrInt> for u8 {
    #[inline(always)]
    fn from(val: HrespErrInt) -> u8 {
        HrespErrInt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hsc(u8);
impl Hsc {
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc0: Self = Self(0x0);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc1: Self = Self(0x01);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc2: Self = Self(0x02);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc3: Self = Self(0x03);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc4: Self = Self(0x04);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc5: Self = Self(0x05);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc6: Self = Self(0x06);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc7: Self = Self(0x07);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc8: Self = Self(0x08);
    #[doc = "Number of pixels to skip minus 1."]
    pub const Hsc9: Self = Self(0x09);
}
impl Hsc {
    pub const fn from_bits(val: u8) -> Hsc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Hsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hsc0"),
            0x01 => f.write_str("Hsc1"),
            0x02 => f.write_str("Hsc2"),
            0x03 => f.write_str("Hsc3"),
            0x04 => f.write_str("Hsc4"),
            0x05 => f.write_str("Hsc5"),
            0x06 => f.write_str("Hsc6"),
            0x07 => f.write_str("Hsc7"),
            0x08 => f.write_str("Hsc8"),
            0x09 => f.write_str("Hsc9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hsc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hsc0"),
            0x01 => defmt::write!(f, "Hsc1"),
            0x02 => defmt::write!(f, "Hsc2"),
            0x03 => defmt::write!(f, "Hsc3"),
            0x04 => defmt::write!(f, "Hsc4"),
            0x05 => defmt::write!(f, "Hsc5"),
            0x06 => defmt::write!(f, "Hsc6"),
            0x07 => defmt::write!(f, "Hsc7"),
            0x08 => defmt::write!(f, "Hsc8"),
            0x09 => defmt::write!(f, "Hsc9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Hsc {
    #[inline(always)]
    fn from(val: u8) -> Hsc {
        Hsc::from_bits(val)
    }
}
impl From<Hsc> for u8 {
    #[inline(always)]
    fn from(val: Hsc) -> u8 {
        Hsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsyncPol {
    #[doc = "HSYNC is active low."]
    HsyncPol0 = 0x0,
    #[doc = "HSYNC is active high."]
    HsyncPol1 = 0x01,
}
impl HsyncPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsyncPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsyncPol {
    #[inline(always)]
    fn from(val: u8) -> HsyncPol {
        HsyncPol::from_bits(val)
    }
}
impl From<HsyncPol> for u8 {
    #[inline(always)]
    fn from(val: HsyncPol) -> u8 {
        HsyncPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvData {
    #[doc = "CSI_D\\[7:0\\] data lines are directly applied to internal circuitry."]
    InvData0 = 0x0,
    #[doc = "CSI_D\\[7:0\\] data lines are inverted before applied to internal circuitry."]
    InvData1 = 0x01,
}
impl InvData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InvData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InvData {
    #[inline(always)]
    fn from(val: u8) -> InvData {
        InvData::from_bits(val)
    }
}
impl From<InvData> for u8 {
    #[inline(always)]
    fn from(val: InvData) -> u8 {
        InvData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvPclk {
    #[doc = "CSI_PIXCLK is directly applied to internal circuitry."]
    InvPclk0 = 0x0,
    #[doc = "CSI_PIXCLK is inverted before applied to internal circuitry."]
    InvPclk1 = 0x01,
}
impl InvPclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InvPclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InvPclk {
    #[inline(always)]
    fn from(val: u8) -> InvPclk {
        InvPclk::from_bits(val)
    }
}
impl From<InvPclk> for u8 {
    #[inline(always)]
    fn from(val: InvPclk) -> u8 {
        InvPclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LastDmaReqSel {
    #[doc = "fifo_full_level."]
    LastDmaReqSel0 = 0x0,
    #[doc = "hburst_length."]
    LastDmaReqSel1 = 0x01,
}
impl LastDmaReqSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LastDmaReqSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LastDmaReqSel {
    #[inline(always)]
    fn from(val: u8) -> LastDmaReqSel {
        LastDmaReqSel::from_bits(val)
    }
}
impl From<LastDmaReqSel> for u8 {
    #[inline(always)]
    fn from(val: LastDmaReqSel) -> u8 {
        LastDmaReqSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lvrm {
    #[doc = "512 x 384."]
    Lvrm0 = 0x0,
    #[doc = "448 x 336."]
    Lvrm1 = 0x01,
    #[doc = "384 x 288."]
    Lvrm2 = 0x02,
    #[doc = "384 x 256."]
    Lvrm3 = 0x03,
    #[doc = "320 x 240."]
    Lvrm4 = 0x04,
    #[doc = "288 x 216."]
    Lvrm5 = 0x05,
    #[doc = "400 x 300."]
    Lvrm6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Lvrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lvrm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lvrm {
    #[inline(always)]
    fn from(val: u8) -> Lvrm {
        Lvrm::from_bits(val)
    }
}
impl From<Lvrm> for u8 {
    #[inline(always)]
    fn from(val: Lvrm) -> u8 {
        Lvrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskOption {
    #[doc = "Writing to memory (OCRAM or external DDR) from first completely frame, when using this option, the CSI_ENABLE should be 1."]
    MaskOption0 = 0x0,
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    MaskOption1 = 0x01,
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    MaskOption2 = 0x02,
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    MaskOption3 = 0x03,
}
impl MaskOption {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskOption {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskOption {
    #[inline(always)]
    fn from(val: u8) -> MaskOption {
        MaskOption::from_bits(val)
    }
}
impl From<MaskOption> for u8 {
    #[inline(always)]
    fn from(val: MaskOption) -> u8 {
        MaskOption::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PackDir {
    #[doc = "Pack from LSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x44332211 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xBBBBAAAA in STAT FIFO."]
    PackDir0 = 0x0,
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    PackDir1 = 0x01,
}
impl PackDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PackDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PackDir {
    #[inline(always)]
    fn from(val: u8) -> PackDir {
        PackDir::from_bits(val)
    }
}
impl From<PackDir> for u8 {
    #[inline(always)]
    fn from(val: PackDir) -> u8 {
        PackDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parallel24En {
    #[doc = "Input is disabled."]
    Parallel24En0 = 0x0,
    #[doc = "Input is enabled."]
    Parallel24En1 = 0x01,
}
impl Parallel24En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Parallel24En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Parallel24En {
    #[inline(always)]
    fn from(val: u8) -> Parallel24En {
        Parallel24En::from_bits(val)
    }
}
impl From<Parallel24En> for u8 {
    #[inline(always)]
    fn from(val: Parallel24En) -> u8 {
        Parallel24En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PixelBit {
    #[doc = "8-bit data for each pixel."]
    PixelBit0 = 0x0,
    #[doc = "10-bit data for each pixel."]
    PixelBit1 = 0x01,
}
impl PixelBit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PixelBit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PixelBit {
    #[inline(always)]
    fn from(val: u8) -> PixelBit {
        PixelBit::from_bits(val)
    }
}
impl From<PixelBit> for u8 {
    #[inline(always)]
    fn from(val: PixelBit) -> u8 {
        PixelBit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrPIfEn {
    #[doc = "CSI to PrP bus is disabled."]
    PrPIfEn0 = 0x0,
    #[doc = "CSI to PrP bus is enabled."]
    PrPIfEn1 = 0x01,
}
impl PrPIfEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrPIfEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrPIfEn {
    #[inline(always)]
    fn from(val: u8) -> PrPIfEn {
        PrPIfEn::from_bits(val)
    }
}
impl From<PrPIfEn> for u8 {
    #[inline(always)]
    fn from(val: PrPIfEn) -> u8 {
        PrPIfEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Redge {
    #[doc = "Pixel data is latched at the falling edge of CSI_PIXCLK."]
    Redge0 = 0x0,
    #[doc = "Pixel data is latched at the rising edge of CSI_PIXCLK."]
    Redge1 = 0x01,
}
impl Redge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Redge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Redge {
    #[inline(always)]
    fn from(val: u8) -> Redge {
        Redge::from_bits(val)
    }
}
impl From<Redge> for u8 {
    #[inline(always)]
    fn from(val: Redge) -> u8 {
        Redge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RfOrInt {
    #[doc = "RXFIFO has not overflowed."]
    RfOrInt0 = 0x0,
    #[doc = "RXFIFO has overflowed."]
    RfOrInt1 = 0x01,
}
impl RfOrInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RfOrInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RfOrInt {
    #[inline(always)]
    fn from(val: u8) -> RfOrInt {
        RfOrInt::from_bits(val)
    }
}
impl From<RfOrInt> for u8 {
    #[inline(always)]
    fn from(val: RfOrInt) -> u8 {
        RfOrInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RfOrInten {
    #[doc = "RxFIFO overrun interrupt is disabled."]
    RfOrInten0 = 0x0,
    #[doc = "RxFIFO overrun interrupt is enabled."]
    RfOrInten1 = 0x01,
}
impl RfOrInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RfOrInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RfOrInten {
    #[inline(always)]
    fn from(val: u8) -> RfOrInten {
        RfOrInten::from_bits(val)
    }
}
impl From<RfOrInten> for u8 {
    #[inline(always)]
    fn from(val: RfOrInten) -> u8 {
        RfOrInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rgb888aFormatSel {
    #[doc = "{8'h0, data\\[23:0\\]}."]
    Rgb888aFormatSel0 = 0x0,
    #[doc = "{data\\[23:0\\], 8'h0}."]
    Rgb888aFormatSel1 = 0x01,
}
impl Rgb888aFormatSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rgb888aFormatSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rgb888aFormatSel {
    #[inline(always)]
    fn from(val: u8) -> Rgb888aFormatSel {
        Rgb888aFormatSel::from_bits(val)
    }
}
impl From<Rgb888aFormatSel> for u8 {
    #[inline(always)]
    fn from(val: Rgb888aFormatSel) -> u8 {
        Rgb888aFormatSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFfInt {
    #[doc = "RxFIFO is not full."]
    RxFfInt0 = 0x0,
    #[doc = "RxFIFO is full."]
    RxFfInt1 = 0x01,
}
impl RxFfInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFfInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFfInt {
    #[inline(always)]
    fn from(val: u8) -> RxFfInt {
        RxFfInt::from_bits(val)
    }
}
impl From<RxFfInt> for u8 {
    #[inline(always)]
    fn from(val: RxFfInt) -> u8 {
        RxFfInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxFfLevel {
    #[doc = "4 Double words."]
    RxFfLevel0 = 0x0,
    #[doc = "8 Double words."]
    RxFfLevel1 = 0x01,
    #[doc = "16 Double words."]
    RxFfLevel2 = 0x02,
    #[doc = "24 Double words."]
    RxFfLevel3 = 0x03,
    #[doc = "32 Double words."]
    RxFfLevel4 = 0x04,
    #[doc = "48 Double words."]
    RxFfLevel5 = 0x05,
    #[doc = "64 Double words."]
    RxFfLevel6 = 0x06,
    #[doc = "96 Double words."]
    RxFfLevel7 = 0x07,
}
impl RxFfLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxFfLevel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxFfLevel {
    #[inline(always)]
    fn from(val: u8) -> RxFfLevel {
        RxFfLevel::from_bits(val)
    }
}
impl From<RxFfLevel> for u8 {
    #[inline(always)]
    fn from(val: RxFfLevel) -> u8 {
        RxFfLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxffInten {
    #[doc = "RxFIFO full interrupt disable."]
    RxffInten0 = 0x0,
    #[doc = "RxFIFO full interrupt enable."]
    RxffInten1 = 0x01,
}
impl RxffInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxffInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxffInten {
    #[inline(always)]
    fn from(val: u8) -> RxffInten {
        RxffInten::from_bits(val)
    }
}
impl From<RxffInten> for u8 {
    #[inline(always)]
    fn from(val: RxffInten) -> u8 {
        RxffInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sce {
    #[doc = "Skip count disable."]
    Sce0 = 0x0,
    #[doc = "Skip count enable."]
    Sce1 = 0x01,
}
impl Sce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sce {
    #[inline(always)]
    fn from(val: u8) -> Sce {
        Sce::from_bits(val)
    }
}
impl From<Sce> for u8 {
    #[inline(always)]
    fn from(val: Sce) -> u8 {
        Sce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sensor16bits {
    #[doc = "Only one 8-bit sensor is connected."]
    Sensor16bits0 = 0x0,
    #[doc = "One 16-bit sensor is connected."]
    Sensor16bits1 = 0x01,
}
impl Sensor16bits {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sensor16bits {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sensor16bits {
    #[inline(always)]
    fn from(val: u8) -> Sensor16bits {
        Sensor16bits::from_bits(val)
    }
}
impl From<Sensor16bits> for u8 {
    #[inline(always)]
    fn from(val: Sensor16bits) -> u8 {
        Sensor16bits::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SfOrInt {
    #[doc = "STATFIFO has not overflowed."]
    SfOrInt0 = 0x0,
    #[doc = "STATFIFO has overflowed."]
    SfOrInt1 = 0x01,
}
impl SfOrInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SfOrInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SfOrInt {
    #[inline(always)]
    fn from(val: u8) -> SfOrInt {
        SfOrInt::from_bits(val)
    }
}
impl From<SfOrInt> for u8 {
    #[inline(always)]
    fn from(val: SfOrInt) -> u8 {
        SfOrInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SfOrInten {
    #[doc = "STATFIFO overrun interrupt is disabled."]
    SfOrInten0 = 0x0,
    #[doc = "STATFIFO overrun interrupt is enabled."]
    SfOrInten1 = 0x01,
}
impl SfOrInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SfOrInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SfOrInten {
    #[inline(always)]
    fn from(val: u8) -> SfOrInten {
        SfOrInten::from_bits(val)
    }
}
impl From<SfOrInten> for u8 {
    #[inline(always)]
    fn from(val: SfOrInten) -> u8 {
        SfOrInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SffDmaDoneInten {
    #[doc = "STATFIFO DMA Transfer Done interrupt disable."]
    SffDmaDoneInten0 = 0x0,
    #[doc = "STATFIFO DMA Transfer Done interrupt enable."]
    SffDmaDoneInten1 = 0x01,
}
impl SffDmaDoneInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SffDmaDoneInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SffDmaDoneInten {
    #[inline(always)]
    fn from(val: u8) -> SffDmaDoneInten {
        SffDmaDoneInten::from_bits(val)
    }
}
impl From<SffDmaDoneInten> for u8 {
    #[inline(always)]
    fn from(val: SffDmaDoneInten) -> u8 {
        SffDmaDoneInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SofInt {
    #[doc = "SOF is not detected."]
    SofInt0 = 0x0,
    #[doc = "SOF is detected."]
    SofInt1 = 0x01,
}
impl SofInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SofInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SofInt {
    #[inline(always)]
    fn from(val: u8) -> SofInt {
        SofInt::from_bits(val)
    }
}
impl From<SofInt> for u8 {
    #[inline(always)]
    fn from(val: SofInt) -> u8 {
        SofInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SofInten {
    #[doc = "SOF interrupt disable."]
    SofInten0 = 0x0,
    #[doc = "SOF interrupt enable."]
    SofInten1 = 0x01,
}
impl SofInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SofInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SofInten {
    #[inline(always)]
    fn from(val: u8) -> SofInten {
        SofInten::from_bits(val)
    }
}
impl From<SofInten> for u8 {
    #[inline(always)]
    fn from(val: SofInten) -> u8 {
        SofInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SofPol {
    #[doc = "SOF interrupt is generated on SOF falling edge."]
    SofPol0 = 0x0,
    #[doc = "SOF interrupt is generated on SOF rising edge."]
    SofPol1 = 0x01,
}
impl SofPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SofPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SofPol {
    #[inline(always)]
    fn from(val: u8) -> SofPol {
        SofPol::from_bits(val)
    }
}
impl From<SofPol> for u8 {
    #[inline(always)]
    fn from(val: SofPol) -> u8 {
        SofPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatffInt {
    #[doc = "STATFIFO is not full."]
    StatffInt0 = 0x0,
    #[doc = "STATFIFO is full."]
    StatffInt1 = 0x01,
}
impl StatffInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatffInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatffInt {
    #[inline(always)]
    fn from(val: u8) -> StatffInt {
        StatffInt::from_bits(val)
    }
}
impl From<StatffInt> for u8 {
    #[inline(always)]
    fn from(val: StatffInt) -> u8 {
        StatffInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatffInten {
    #[doc = "STATFIFO full interrupt disable."]
    StatffInten0 = 0x0,
    #[doc = "STATFIFO full interrupt enable."]
    StatffInten1 = 0x01,
}
impl StatffInten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatffInten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatffInten {
    #[inline(always)]
    fn from(val: u8) -> StatffInten {
        StatffInten::from_bits(val)
    }
}
impl From<StatffInten> for u8 {
    #[inline(always)]
    fn from(val: StatffInten) -> u8 {
        StatffInten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatffLevel {
    #[doc = "4 Double words."]
    StatffLevel0 = 0x0,
    #[doc = "8 Double words."]
    StatffLevel1 = 0x01,
    #[doc = "12 Double words."]
    StatffLevel2 = 0x02,
    #[doc = "16 Double words."]
    StatffLevel3 = 0x03,
    #[doc = "24 Double words."]
    StatffLevel4 = 0x04,
    #[doc = "32 Double words."]
    StatffLevel5 = 0x05,
    #[doc = "48 Double words."]
    StatffLevel6 = 0x06,
    #[doc = "64 Double words."]
    StatffLevel7 = 0x07,
}
impl StatffLevel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatffLevel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatffLevel {
    #[inline(always)]
    fn from(val: u8) -> StatffLevel {
        StatffLevel::from_bits(val)
    }
}
impl From<StatffLevel> for u8 {
    #[inline(always)]
    fn from(val: StatffLevel) -> u8 {
        StatffLevel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swap16En {
    #[doc = "Disable swapping."]
    Swap16En0 = 0x0,
    #[doc = "Enable swapping."]
    Swap16En1 = 0x01,
}
impl Swap16En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swap16En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swap16En {
    #[inline(always)]
    fn from(val: u8) -> Swap16En {
        Swap16En::from_bits(val)
    }
}
impl From<Swap16En> for u8 {
    #[inline(always)]
    fn from(val: Swap16En) -> u8 {
        Swap16En::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vsc(u8);
impl Vsc {
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc0: Self = Self(0x0);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc1: Self = Self(0x01);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc2: Self = Self(0x02);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc3: Self = Self(0x03);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc4: Self = Self(0x04);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc5: Self = Self(0x05);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc6: Self = Self(0x06);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc7: Self = Self(0x07);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc8: Self = Self(0x08);
    #[doc = "Number of rows to skip minus 1."]
    pub const Vsc9: Self = Self(0x09);
}
impl Vsc {
    pub const fn from_bits(val: u8) -> Vsc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Vsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Vsc0"),
            0x01 => f.write_str("Vsc1"),
            0x02 => f.write_str("Vsc2"),
            0x03 => f.write_str("Vsc3"),
            0x04 => f.write_str("Vsc4"),
            0x05 => f.write_str("Vsc5"),
            0x06 => f.write_str("Vsc6"),
            0x07 => f.write_str("Vsc7"),
            0x08 => f.write_str("Vsc8"),
            0x09 => f.write_str("Vsc9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vsc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Vsc0"),
            0x01 => defmt::write!(f, "Vsc1"),
            0x02 => defmt::write!(f, "Vsc2"),
            0x03 => defmt::write!(f, "Vsc3"),
            0x04 => defmt::write!(f, "Vsc4"),
            0x05 => defmt::write!(f, "Vsc5"),
            0x06 => defmt::write!(f, "Vsc6"),
            0x07 => defmt::write!(f, "Vsc7"),
            0x08 => defmt::write!(f, "Vsc8"),
            0x09 => defmt::write!(f, "Vsc9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Vsc {
    #[inline(always)]
    fn from(val: u8) -> Vsc {
        Vsc::from_bits(val)
    }
}
impl From<Vsc> for u8 {
    #[inline(always)]
    fn from(val: Vsc) -> u8 {
        Vsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ZeroPackEn {
    #[doc = "Zero packing disabled."]
    ZeroPackEn0 = 0x0,
    #[doc = "Zero packing enabled."]
    ZeroPackEn1 = 0x01,
}
impl ZeroPackEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ZeroPackEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ZeroPackEn {
    #[inline(always)]
    fn from(val: u8) -> ZeroPackEn {
        ZeroPackEn::from_bits(val)
    }
}
impl From<ZeroPackEn> for u8 {
    #[inline(always)]
    fn from(val: ZeroPackEn) -> u8 {
        ZeroPackEn::to_bits(val)
    }
}
