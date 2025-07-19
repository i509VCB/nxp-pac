#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Afs {
    #[doc = "Abs Diff on consecutive green pixels"]
    AFS_0 = 0x0,
    #[doc = "Abs Diff on every third green pixels"]
    AFS_1 = 0x01,
    #[doc = "Abs Diff on every four green pixels"]
    AFS_2 = 0x02,
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
pub enum BaseaddrSwitchSel {
    #[doc = "Switching base address at the edge of the vsync"]
    BASEADDR_SWITCH_SEL_0 = 0x0,
    #[doc = "Switching base address at the edge of the first data of each frame"]
    BASEADDR_SWITCH_SEL_1 = 0x01,
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
    #[doc = "GR"]
    BTS_0 = 0x0,
    #[doc = "RG"]
    BTS_1 = 0x01,
    #[doc = "BG"]
    BTS_2 = 0x02,
    #[doc = "GB"]
    BTS_3 = 0x03,
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
pub enum CcirMode {
    #[doc = "Progressive mode is selected"]
    CCIR_MODE_0 = 0x0,
    #[doc = "Interlace mode is selected"]
    CCIR_MODE_1 = 0x01,
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
pub enum DmaBurstTypeRff {
    #[doc = "INCR8"]
    DMA_BURST_TYPE_RFF_0 = 0x0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_RFF_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_RFF_3 = 0x03,
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
    #[doc = "INCR8"]
    DMA_BURST_TYPE_SFF_0 = 0x0,
    #[doc = "INCR4"]
    DMA_BURST_TYPE_SFF_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "INCR16"]
    DMA_BURST_TYPE_SFF_3 = 0x03,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hsc(u8);
impl Hsc {
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_0: Self = Self(0x0);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_1: Self = Self(0x01);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_2: Self = Self(0x02);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_3: Self = Self(0x03);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_4: Self = Self(0x04);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_5: Self = Self(0x05);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_6: Self = Self(0x06);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_7: Self = Self(0x07);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_8: Self = Self(0x08);
    #[doc = "Number of pixels to skip minus 1"]
    pub const HSC_9: Self = Self(0x09);
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
            0x0 => f.write_str("HSC_0"),
            0x01 => f.write_str("HSC_1"),
            0x02 => f.write_str("HSC_2"),
            0x03 => f.write_str("HSC_3"),
            0x04 => f.write_str("HSC_4"),
            0x05 => f.write_str("HSC_5"),
            0x06 => f.write_str("HSC_6"),
            0x07 => f.write_str("HSC_7"),
            0x08 => f.write_str("HSC_8"),
            0x09 => f.write_str("HSC_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hsc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "HSC_0"),
            0x01 => defmt::write!(f, "HSC_1"),
            0x02 => defmt::write!(f, "HSC_2"),
            0x03 => defmt::write!(f, "HSC_3"),
            0x04 => defmt::write!(f, "HSC_4"),
            0x05 => defmt::write!(f, "HSC_5"),
            0x06 => defmt::write!(f, "HSC_6"),
            0x07 => defmt::write!(f, "HSC_7"),
            0x08 => defmt::write!(f, "HSC_8"),
            0x09 => defmt::write!(f, "HSC_9"),
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
pub enum LastDmaReqSel {
    #[doc = "fifo_full_level"]
    LAST_DMA_REQ_SEL_0 = 0x0,
    #[doc = "hburst_length"]
    LAST_DMA_REQ_SEL_1 = 0x01,
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
    #[doc = "512 x 384"]
    LVRM_0 = 0x0,
    #[doc = "448 x 336"]
    LVRM_1 = 0x01,
    #[doc = "384 x 288"]
    LVRM_2 = 0x02,
    #[doc = "384 x 256"]
    LVRM_3 = 0x03,
    #[doc = "320 x 240"]
    LVRM_4 = 0x04,
    #[doc = "288 x 216"]
    LVRM_5 = 0x05,
    #[doc = "400 x 300"]
    LVRM_6 = 0x06,
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
    MASK_OPTION_0 = 0x0,
    #[doc = "Writing to memory when CSI_ENABLE is 1."]
    MASK_OPTION_1 = 0x01,
    #[doc = "Writing to memory from second completely frame, when using this option, the CSI_ENABLE should be 1."]
    MASK_OPTION_2 = 0x02,
    #[doc = "Writing to memory when data comes in, not matter the CSI_ENABLE is 1 or 0."]
    MASK_OPTION_3 = 0x03,
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
    PACK_DIR_0 = 0x0,
    #[doc = "Pack from MSB first. For image data, 0x11, 0x22, 0x33, 0x44, it will appear as 0x11223344 in RX FIFO. For stat data, 0xAAAA, 0xBBBB, it will appear as 0xAAAABBBB in STAT FIFO."]
    PACK_DIR_1 = 0x01,
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
pub enum Rgb888aFormatSel {
    #[doc = "{8'h0, data\\[23:0\\]}"]
    RGB888A_FORMAT_SEL_0 = 0x0,
    #[doc = "{data\\[23:0\\], 8'h0}"]
    RGB888A_FORMAT_SEL_1 = 0x01,
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
pub enum RxFfLevel {
    #[doc = "4 Double words"]
    RX_FF_LEVEL_0 = 0x0,
    #[doc = "8 Double words"]
    RX_FF_LEVEL_1 = 0x01,
    #[doc = "16 Double words"]
    RX_FF_LEVEL_2 = 0x02,
    #[doc = "24 Double words"]
    RX_FF_LEVEL_3 = 0x03,
    #[doc = "32 Double words"]
    RX_FF_LEVEL_4 = 0x04,
    #[doc = "48 Double words"]
    RX_FF_LEVEL_5 = 0x05,
    #[doc = "64 Double words"]
    RX_FF_LEVEL_6 = 0x06,
    #[doc = "96 Double words"]
    RX_FF_LEVEL_7 = 0x07,
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
pub enum StatffLevel {
    #[doc = "4 Double words"]
    STATFF_LEVEL_0 = 0x0,
    #[doc = "8 Double words"]
    STATFF_LEVEL_1 = 0x01,
    #[doc = "12 Double words"]
    STATFF_LEVEL_2 = 0x02,
    #[doc = "16 Double words"]
    STATFF_LEVEL_3 = 0x03,
    #[doc = "24 Double words"]
    STATFF_LEVEL_4 = 0x04,
    #[doc = "32 Double words"]
    STATFF_LEVEL_5 = 0x05,
    #[doc = "48 Double words"]
    STATFF_LEVEL_6 = 0x06,
    #[doc = "64 Double words"]
    STATFF_LEVEL_7 = 0x07,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vsc(u8);
impl Vsc {
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_0: Self = Self(0x0);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_1: Self = Self(0x01);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_2: Self = Self(0x02);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_3: Self = Self(0x03);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_4: Self = Self(0x04);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_5: Self = Self(0x05);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_6: Self = Self(0x06);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_7: Self = Self(0x07);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_8: Self = Self(0x08);
    #[doc = "Number of rows to skip minus 1"]
    pub const VSC_9: Self = Self(0x09);
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
            0x0 => f.write_str("VSC_0"),
            0x01 => f.write_str("VSC_1"),
            0x02 => f.write_str("VSC_2"),
            0x03 => f.write_str("VSC_3"),
            0x04 => f.write_str("VSC_4"),
            0x05 => f.write_str("VSC_5"),
            0x06 => f.write_str("VSC_6"),
            0x07 => f.write_str("VSC_7"),
            0x08 => f.write_str("VSC_8"),
            0x09 => f.write_str("VSC_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vsc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "VSC_0"),
            0x01 => defmt::write!(f, "VSC_1"),
            0x02 => defmt::write!(f, "VSC_2"),
            0x03 => defmt::write!(f, "VSC_3"),
            0x04 => defmt::write!(f, "VSC_4"),
            0x05 => defmt::write!(f, "VSC_5"),
            0x06 => defmt::write!(f, "VSC_6"),
            0x07 => defmt::write!(f, "VSC_7"),
            0x08 => defmt::write!(f, "VSC_8"),
            0x09 => defmt::write!(f, "VSC_9"),
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
