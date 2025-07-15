#[doc = "Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Controls the maximum value of a variable delay that will be applied before any ELS AES operation is started"]
    #[must_use]
    #[inline(always)]
    pub const fn adctrl(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Controls the maximum value of a variable delay that will be applied before any ELS AES operation is started"]
    #[inline(always)]
    pub const fn set_adctrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("adctrl", &self.adctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cfg {{ adctrl: {=u16:?} }}", self.adctrl())
    }
}
#[doc = "Command Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdcfg0(pub u32);
impl Cmdcfg0 {
    #[doc = "See"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcfg0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "See"]
    #[inline(always)]
    pub const fn set_cmdcfg0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmdcfg0 {
    #[inline(always)]
    fn default() -> Cmdcfg0 {
        Cmdcfg0(0)
    }
}
impl core::fmt::Debug for Cmdcfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdcfg0")
            .field("cmdcfg0", &self.cmdcfg0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdcfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdcfg0 {{ cmdcfg0: {=u32:?} }}", self.cmdcfg0())
    }
}
#[doc = "Command CRC Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdcrc(pub u32);
impl Cmdcrc {
    #[doc = "Indicates the current CRC value."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the current CRC value."]
    #[inline(always)]
    pub const fn set_cmdcrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cmdcrc {
    #[inline(always)]
    fn default() -> Cmdcrc {
        Cmdcrc(0)
    }
}
impl core::fmt::Debug for Cmdcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdcrc")
            .field("cmdcrc", &self.cmdcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdcrc {{ cmdcrc: {=u32:?} }}", self.cmdcrc())
    }
}
#[doc = "CRC Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdcrcCtrl(pub u32);
impl CmdcrcCtrl {
    #[doc = "CRC reset to initial valueCMDCRC_EN and CMDCRC_RST fields act independently"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrc_rst(&self) -> super::vals::CmdcrcRst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CmdcrcRst::from_bits(val as u8)
    }
    #[doc = "CRC reset to initial valueCMDCRC_EN and CMDCRC_RST fields act independently"]
    #[inline(always)]
    pub const fn set_cmdcrc_rst(&mut self, val: super::vals::CmdcrcRst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CRC enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcrc_en(&self) -> super::vals::CmdcrcEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::CmdcrcEn::from_bits(val as u8)
    }
    #[doc = "CRC enable bit"]
    #[inline(always)]
    pub const fn set_cmdcrc_en(&mut self, val: super::vals::CmdcrcEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for CmdcrcCtrl {
    #[inline(always)]
    fn default() -> CmdcrcCtrl {
        CmdcrcCtrl(0)
    }
}
impl core::fmt::Debug for CmdcrcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdcrcCtrl")
            .field("cmdcrc_rst", &self.cmdcrc_rst())
            .field("cmdcrc_en", &self.cmdcrc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdcrcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmdcrcCtrl {{ cmdcrc_rst: {:?}, cmdcrc_en: {:?} }}",
            self.cmdcrc_rst(),
            self.cmdcrc_en()
        )
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "ELS enable"]
    #[must_use]
    #[inline(always)]
    pub const fn els_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ELS enable"]
    #[inline(always)]
    pub const fn set_els_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write to 1 to start an ELS operation. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn els_start(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to start an ELS operation. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_els_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write to 1 to perform an ELS synchronous reset. Writing 0 has no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn els_reset(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to perform an ELS synchronous reset. Writing 0 has no effect."]
    #[inline(always)]
    pub const fn set_els_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ELS Command ID"]
    #[must_use]
    #[inline(always)]
    pub const fn els_cmd(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "ELS Command ID"]
    #[inline(always)]
    pub const fn set_els_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
    }
    #[doc = "Defines endianness"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_order(&self) -> super::vals::ByteOrder {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ByteOrder::from_bits(val as u8)
    }
    #[doc = "Defines endianness"]
    #[inline(always)]
    pub const fn set_byte_order(&mut self, val: super::vals::ByteOrder) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("els_en", &self.els_en())
            .field("els_start", &self.els_start())
            .field("els_reset", &self.els_reset())
            .field("els_cmd", &self.els_cmd())
            .field("byte_order", &self.byte_order())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ els_en: {=bool:?}, els_start: {=bool:?}, els_reset: {=bool:?}, els_cmd: {=u8:?}, byte_order: {:?} }}",
            self.els_en(),
            self.els_start(),
            self.els_reset(),
            self.els_cmd(),
            self.byte_order()
        )
    }
}
#[doc = "Final DMA Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaFinAddr(pub u32);
impl DmaFinAddr {
    #[doc = "Indicates the final address of system memory that was accessed by ELS during the last command."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_fin_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the final address of system memory that was accessed by ELS during the last command."]
    #[inline(always)]
    pub const fn set_dma_fin_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaFinAddr {
    #[inline(always)]
    fn default() -> DmaFinAddr {
        DmaFinAddr(0)
    }
}
impl core::fmt::Debug for DmaFinAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaFinAddr")
            .field("dma_fin_addr", &self.dma_fin_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaFinAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaFinAddr {{ dma_fin_addr: {=u32:?} }}",
            self.dma_fin_addr()
        )
    }
}
#[doc = "DMA Result 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaRes0(pub u32);
impl DmaRes0 {
    #[doc = "Defines the system start address where the result of the ELS operation is transferred via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_res0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system start address where the result of the ELS operation is transferred via DMA."]
    #[inline(always)]
    pub const fn set_addr_res0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaRes0 {
    #[inline(always)]
    fn default() -> DmaRes0 {
        DmaRes0(0)
    }
}
impl core::fmt::Debug for DmaRes0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaRes0")
            .field("addr_res0", &self.addr_res0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaRes0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaRes0 {{ addr_res0: {=u32:?} }}", self.addr_res0())
    }
}
#[doc = "DMA Result 0 Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaRes0Len(pub u32);
impl DmaRes0Len {
    #[doc = "Size in bytes of the data to be transferred"]
    #[must_use]
    #[inline(always)]
    pub const fn size_res0_len(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred"]
    #[inline(always)]
    pub const fn set_size_res0_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaRes0Len {
    #[inline(always)]
    fn default() -> DmaRes0Len {
        DmaRes0Len(0)
    }
}
impl core::fmt::Debug for DmaRes0Len {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaRes0Len")
            .field("size_res0_len", &self.size_res0_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaRes0Len {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaRes0Len {{ size_res0_len: {=u32:?} }}",
            self.size_res0_len()
        )
    }
}
#[doc = "DMA Source 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSrc0(pub u32);
impl DmaSrc0 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_src0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[inline(always)]
    pub const fn set_addr_src0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaSrc0 {
    #[inline(always)]
    fn default() -> DmaSrc0 {
        DmaSrc0(0)
    }
}
impl core::fmt::Debug for DmaSrc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSrc0")
            .field("addr_src0", &self.addr_src0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSrc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaSrc0 {{ addr_src0: {=u32:?} }}", self.addr_src0())
    }
}
#[doc = "DMA Source 0 Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSrc0Len(pub u32);
impl DmaSrc0Len {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC0."]
    #[must_use]
    #[inline(always)]
    pub const fn size_src0_len(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC0."]
    #[inline(always)]
    pub const fn set_size_src0_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaSrc0Len {
    #[inline(always)]
    fn default() -> DmaSrc0Len {
        DmaSrc0Len(0)
    }
}
impl core::fmt::Debug for DmaSrc0Len {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSrc0Len")
            .field("size_src0_len", &self.size_src0_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSrc0Len {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaSrc0Len {{ size_src0_len: {=u32:?} }}",
            self.size_src0_len()
        )
    }
}
#[doc = "DMA Source 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSrc1(pub u32);
impl DmaSrc1 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_src1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[inline(always)]
    pub const fn set_addr_src1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaSrc1 {
    #[inline(always)]
    fn default() -> DmaSrc1 {
        DmaSrc1(0)
    }
}
impl core::fmt::Debug for DmaSrc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSrc1")
            .field("addr_src1", &self.addr_src1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSrc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaSrc1 {{ addr_src1: {=u32:?} }}", self.addr_src1())
    }
}
#[doc = "DMA Source 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSrc2(pub u32);
impl DmaSrc2 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_src2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA."]
    #[inline(always)]
    pub const fn set_addr_src2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaSrc2 {
    #[inline(always)]
    fn default() -> DmaSrc2 {
        DmaSrc2(0)
    }
}
impl core::fmt::Debug for DmaSrc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSrc2")
            .field("addr_src2", &self.addr_src2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSrc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaSrc2 {{ addr_src2: {=u32:?} }}", self.addr_src2())
    }
}
#[doc = "DMA Source 2 Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSrc2Len(pub u32);
impl DmaSrc2Len {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC2."]
    #[must_use]
    #[inline(always)]
    pub const fn size_src2_len(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC2."]
    #[inline(always)]
    pub const fn set_size_src2_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaSrc2Len {
    #[inline(always)]
    fn default() -> DmaSrc2Len {
        DmaSrc2Len(0)
    }
}
impl core::fmt::Debug for DmaSrc2Len {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSrc2Len")
            .field("size_src2_len", &self.size_src2_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSrc2Len {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaSrc2Len {{ size_src2_len: {=u32:?} }}",
            self.size_src2_len()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs0(pub u32);
impl ElsKs0 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ksize(&self) -> super::vals::Ks0Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks0Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks0_ksize(&mut self, val: super::vals::Ks0Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks0_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks0_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks0_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks0_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks0_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks0_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks0_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks0_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks0_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks0_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks0_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks0_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks0_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks0_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks0_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks0_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks0_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks0_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks0_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks0_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks0_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks0_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Wrap key"]
    #[inline(always)]
    pub const fn set_ks0_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks0_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks0_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks0_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs0 {
    #[inline(always)]
    fn default() -> ElsKs0 {
        ElsKs0(0)
    }
}
impl core::fmt::Debug for ElsKs0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs0")
            .field("ks0_ksize", &self.ks0_ksize())
            .field("ks0_kact", &self.ks0_kact())
            .field("ks0_kbase", &self.ks0_kbase())
            .field("ks0_fgp", &self.ks0_fgp())
            .field("ks0_frtn", &self.ks0_frtn())
            .field("ks0_fhwo", &self.ks0_fhwo())
            .field("ks0_ukpuk", &self.ks0_ukpuk())
            .field("ks0_utecdh", &self.ks0_utecdh())
            .field("ks0_ucmac", &self.ks0_ucmac())
            .field("ks0_uksk", &self.ks0_uksk())
            .field("ks0_urtf", &self.ks0_urtf())
            .field("ks0_uckdf", &self.ks0_uckdf())
            .field("ks0_uhkdf", &self.ks0_uhkdf())
            .field("ks0_uecsg", &self.ks0_uecsg())
            .field("ks0_uecdh", &self.ks0_uecdh())
            .field("ks0_uaes", &self.ks0_uaes())
            .field("ks0_uhmac", &self.ks0_uhmac())
            .field("ks0_ukwk", &self.ks0_ukwk())
            .field("ks0_ukuok", &self.ks0_ukuok())
            .field("ks0_utlspms", &self.ks0_utlspms())
            .field("ks0_utlsms", &self.ks0_utlsms())
            .field("ks0_ukgsrc", &self.ks0_ukgsrc())
            .field("ks0_uhwo", &self.ks0_uhwo())
            .field("ks0_uwrpok", &self.ks0_uwrpok())
            .field("ks0_uduk", &self.ks0_uduk())
            .field("ks0_upprot", &self.ks0_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs0 {{ ks0_ksize: {:?}, ks0_kact: {=bool:?}, ks0_kbase: {=bool:?}, ks0_fgp: {=bool:?}, ks0_frtn: {=bool:?}, ks0_fhwo: {=bool:?}, ks0_ukpuk: {=bool:?}, ks0_utecdh: {=bool:?}, ks0_ucmac: {=bool:?}, ks0_uksk: {=bool:?}, ks0_urtf: {=bool:?}, ks0_uckdf: {=bool:?}, ks0_uhkdf: {=bool:?}, ks0_uecsg: {=bool:?}, ks0_uecdh: {=bool:?}, ks0_uaes: {=bool:?}, ks0_uhmac: {=bool:?}, ks0_ukwk: {=bool:?}, ks0_ukuok: {=bool:?}, ks0_utlspms: {=bool:?}, ks0_utlsms: {=bool:?}, ks0_ukgsrc: {=bool:?}, ks0_uhwo: {=bool:?}, ks0_uwrpok: {=bool:?}, ks0_uduk: {=bool:?}, ks0_upprot: {=u8:?} }}",
            self.ks0_ksize(),
            self.ks0_kact(),
            self.ks0_kbase(),
            self.ks0_fgp(),
            self.ks0_frtn(),
            self.ks0_fhwo(),
            self.ks0_ukpuk(),
            self.ks0_utecdh(),
            self.ks0_ucmac(),
            self.ks0_uksk(),
            self.ks0_urtf(),
            self.ks0_uckdf(),
            self.ks0_uhkdf(),
            self.ks0_uecsg(),
            self.ks0_uecdh(),
            self.ks0_uaes(),
            self.ks0_uhmac(),
            self.ks0_ukwk(),
            self.ks0_ukuok(),
            self.ks0_utlspms(),
            self.ks0_utlsms(),
            self.ks0_ukgsrc(),
            self.ks0_uhwo(),
            self.ks0_uwrpok(),
            self.ks0_uduk(),
            self.ks0_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs1(pub u32);
impl ElsKs1 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ksize(&self) -> super::vals::Ks1Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks1Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks1_ksize(&mut self, val: super::vals::Ks1Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks1_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks1_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks1_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks1_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks1_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks1_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks1_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks1_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks1_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks1_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks1_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks1_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks1_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks1_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks1_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks1_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks1_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks1_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks1_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks1_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks1_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks1_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks1_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks1_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks1_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks1_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs1 {
    #[inline(always)]
    fn default() -> ElsKs1 {
        ElsKs1(0)
    }
}
impl core::fmt::Debug for ElsKs1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs1")
            .field("ks1_ksize", &self.ks1_ksize())
            .field("ks1_kact", &self.ks1_kact())
            .field("ks1_kbase", &self.ks1_kbase())
            .field("ks1_fgp", &self.ks1_fgp())
            .field("ks1_frtn", &self.ks1_frtn())
            .field("ks1_fhwo", &self.ks1_fhwo())
            .field("ks1_ukpuk", &self.ks1_ukpuk())
            .field("ks1_utecdh", &self.ks1_utecdh())
            .field("ks1_ucmac", &self.ks1_ucmac())
            .field("ks1_uksk", &self.ks1_uksk())
            .field("ks1_urtf", &self.ks1_urtf())
            .field("ks1_uckdf", &self.ks1_uckdf())
            .field("ks1_uhkdf", &self.ks1_uhkdf())
            .field("ks1_uecsg", &self.ks1_uecsg())
            .field("ks1_uecdh", &self.ks1_uecdh())
            .field("ks1_uaes", &self.ks1_uaes())
            .field("ks1_uhmac", &self.ks1_uhmac())
            .field("ks1_ukwk", &self.ks1_ukwk())
            .field("ks1_ukuok", &self.ks1_ukuok())
            .field("ks1_utlspms", &self.ks1_utlspms())
            .field("ks1_utlsms", &self.ks1_utlsms())
            .field("ks1_ukgsrc", &self.ks1_ukgsrc())
            .field("ks1_uhwo", &self.ks1_uhwo())
            .field("ks1_uwrpok", &self.ks1_uwrpok())
            .field("ks1_uduk", &self.ks1_uduk())
            .field("ks1_upprot", &self.ks1_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs1 {{ ks1_ksize: {:?}, ks1_kact: {=bool:?}, ks1_kbase: {=bool:?}, ks1_fgp: {=bool:?}, ks1_frtn: {=bool:?}, ks1_fhwo: {=bool:?}, ks1_ukpuk: {=bool:?}, ks1_utecdh: {=bool:?}, ks1_ucmac: {=bool:?}, ks1_uksk: {=bool:?}, ks1_urtf: {=bool:?}, ks1_uckdf: {=bool:?}, ks1_uhkdf: {=bool:?}, ks1_uecsg: {=bool:?}, ks1_uecdh: {=bool:?}, ks1_uaes: {=bool:?}, ks1_uhmac: {=bool:?}, ks1_ukwk: {=bool:?}, ks1_ukuok: {=bool:?}, ks1_utlspms: {=bool:?}, ks1_utlsms: {=bool:?}, ks1_ukgsrc: {=bool:?}, ks1_uhwo: {=bool:?}, ks1_uwrpok: {=bool:?}, ks1_uduk: {=bool:?}, ks1_upprot: {=u8:?} }}",
            self.ks1_ksize(),
            self.ks1_kact(),
            self.ks1_kbase(),
            self.ks1_fgp(),
            self.ks1_frtn(),
            self.ks1_fhwo(),
            self.ks1_ukpuk(),
            self.ks1_utecdh(),
            self.ks1_ucmac(),
            self.ks1_uksk(),
            self.ks1_urtf(),
            self.ks1_uckdf(),
            self.ks1_uhkdf(),
            self.ks1_uecsg(),
            self.ks1_uecdh(),
            self.ks1_uaes(),
            self.ks1_uhmac(),
            self.ks1_ukwk(),
            self.ks1_ukuok(),
            self.ks1_utlspms(),
            self.ks1_utlsms(),
            self.ks1_ukgsrc(),
            self.ks1_uhwo(),
            self.ks1_uwrpok(),
            self.ks1_uduk(),
            self.ks1_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs10(pub u32);
impl ElsKs10 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ksize(&self) -> super::vals::Ks10Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks10Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks10_ksize(&mut self, val: super::vals::Ks10Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks10_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks10_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks10_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks10_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks10_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks10_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks10_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks10_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks10_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks10_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks10_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks10_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks10_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks10_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks10_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks10_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks10_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks10_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks10_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks10_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks10_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks10_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks10_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks10_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks10_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks10_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs10 {
    #[inline(always)]
    fn default() -> ElsKs10 {
        ElsKs10(0)
    }
}
impl core::fmt::Debug for ElsKs10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs10")
            .field("ks10_ksize", &self.ks10_ksize())
            .field("ks10_kact", &self.ks10_kact())
            .field("ks10_kbase", &self.ks10_kbase())
            .field("ks10_fgp", &self.ks10_fgp())
            .field("ks10_frtn", &self.ks10_frtn())
            .field("ks10_fhwo", &self.ks10_fhwo())
            .field("ks10_ukpuk", &self.ks10_ukpuk())
            .field("ks10_utecdh", &self.ks10_utecdh())
            .field("ks10_ucmac", &self.ks10_ucmac())
            .field("ks10_uksk", &self.ks10_uksk())
            .field("ks10_urtf", &self.ks10_urtf())
            .field("ks10_uckdf", &self.ks10_uckdf())
            .field("ks10_uhkdf", &self.ks10_uhkdf())
            .field("ks10_uecsg", &self.ks10_uecsg())
            .field("ks10_uecdh", &self.ks10_uecdh())
            .field("ks10_uaes", &self.ks10_uaes())
            .field("ks10_uhmac", &self.ks10_uhmac())
            .field("ks10_ukwk", &self.ks10_ukwk())
            .field("ks10_ukuok", &self.ks10_ukuok())
            .field("ks10_utlspms", &self.ks10_utlspms())
            .field("ks10_utlsms", &self.ks10_utlsms())
            .field("ks10_ukgsrc", &self.ks10_ukgsrc())
            .field("ks10_uhwo", &self.ks10_uhwo())
            .field("ks10_uwrpok", &self.ks10_uwrpok())
            .field("ks10_uduk", &self.ks10_uduk())
            .field("ks10_upprot", &self.ks10_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs10 {{ ks10_ksize: {:?}, ks10_kact: {=bool:?}, ks10_kbase: {=bool:?}, ks10_fgp: {=bool:?}, ks10_frtn: {=bool:?}, ks10_fhwo: {=bool:?}, ks10_ukpuk: {=bool:?}, ks10_utecdh: {=bool:?}, ks10_ucmac: {=bool:?}, ks10_uksk: {=bool:?}, ks10_urtf: {=bool:?}, ks10_uckdf: {=bool:?}, ks10_uhkdf: {=bool:?}, ks10_uecsg: {=bool:?}, ks10_uecdh: {=bool:?}, ks10_uaes: {=bool:?}, ks10_uhmac: {=bool:?}, ks10_ukwk: {=bool:?}, ks10_ukuok: {=bool:?}, ks10_utlspms: {=bool:?}, ks10_utlsms: {=bool:?}, ks10_ukgsrc: {=bool:?}, ks10_uhwo: {=bool:?}, ks10_uwrpok: {=bool:?}, ks10_uduk: {=bool:?}, ks10_upprot: {=u8:?} }}",
            self.ks10_ksize(),
            self.ks10_kact(),
            self.ks10_kbase(),
            self.ks10_fgp(),
            self.ks10_frtn(),
            self.ks10_fhwo(),
            self.ks10_ukpuk(),
            self.ks10_utecdh(),
            self.ks10_ucmac(),
            self.ks10_uksk(),
            self.ks10_urtf(),
            self.ks10_uckdf(),
            self.ks10_uhkdf(),
            self.ks10_uecsg(),
            self.ks10_uecdh(),
            self.ks10_uaes(),
            self.ks10_uhmac(),
            self.ks10_ukwk(),
            self.ks10_ukuok(),
            self.ks10_utlspms(),
            self.ks10_utlsms(),
            self.ks10_ukgsrc(),
            self.ks10_uhwo(),
            self.ks10_uwrpok(),
            self.ks10_uduk(),
            self.ks10_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs11(pub u32);
impl ElsKs11 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ksize(&self) -> super::vals::Ks11Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks11Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks11_ksize(&mut self, val: super::vals::Ks11Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks11_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks11_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks11_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks11_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks11_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks11_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks11_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks11_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks11_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks11_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks11_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks11_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks11_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks11_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks11_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks11_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks11_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks11_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks11_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks11_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks11_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks11_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks11_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks11_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks11_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks11_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs11 {
    #[inline(always)]
    fn default() -> ElsKs11 {
        ElsKs11(0)
    }
}
impl core::fmt::Debug for ElsKs11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs11")
            .field("ks11_ksize", &self.ks11_ksize())
            .field("ks11_kact", &self.ks11_kact())
            .field("ks11_kbase", &self.ks11_kbase())
            .field("ks11_fgp", &self.ks11_fgp())
            .field("ks11_frtn", &self.ks11_frtn())
            .field("ks11_fhwo", &self.ks11_fhwo())
            .field("ks11_ukpuk", &self.ks11_ukpuk())
            .field("ks11_utecdh", &self.ks11_utecdh())
            .field("ks11_ucmac", &self.ks11_ucmac())
            .field("ks11_uksk", &self.ks11_uksk())
            .field("ks11_urtf", &self.ks11_urtf())
            .field("ks11_uckdf", &self.ks11_uckdf())
            .field("ks11_uhkdf", &self.ks11_uhkdf())
            .field("ks11_uecsg", &self.ks11_uecsg())
            .field("ks11_uecdh", &self.ks11_uecdh())
            .field("ks11_uaes", &self.ks11_uaes())
            .field("ks11_uhmac", &self.ks11_uhmac())
            .field("ks11_ukwk", &self.ks11_ukwk())
            .field("ks11_ukuok", &self.ks11_ukuok())
            .field("ks11_utlspms", &self.ks11_utlspms())
            .field("ks11_utlsms", &self.ks11_utlsms())
            .field("ks11_ukgsrc", &self.ks11_ukgsrc())
            .field("ks11_uhwo", &self.ks11_uhwo())
            .field("ks11_uwrpok", &self.ks11_uwrpok())
            .field("ks11_uduk", &self.ks11_uduk())
            .field("ks11_upprot", &self.ks11_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs11 {{ ks11_ksize: {:?}, ks11_kact: {=bool:?}, ks11_kbase: {=bool:?}, ks11_fgp: {=bool:?}, ks11_frtn: {=bool:?}, ks11_fhwo: {=bool:?}, ks11_ukpuk: {=bool:?}, ks11_utecdh: {=bool:?}, ks11_ucmac: {=bool:?}, ks11_uksk: {=bool:?}, ks11_urtf: {=bool:?}, ks11_uckdf: {=bool:?}, ks11_uhkdf: {=bool:?}, ks11_uecsg: {=bool:?}, ks11_uecdh: {=bool:?}, ks11_uaes: {=bool:?}, ks11_uhmac: {=bool:?}, ks11_ukwk: {=bool:?}, ks11_ukuok: {=bool:?}, ks11_utlspms: {=bool:?}, ks11_utlsms: {=bool:?}, ks11_ukgsrc: {=bool:?}, ks11_uhwo: {=bool:?}, ks11_uwrpok: {=bool:?}, ks11_uduk: {=bool:?}, ks11_upprot: {=u8:?} }}",
            self.ks11_ksize(),
            self.ks11_kact(),
            self.ks11_kbase(),
            self.ks11_fgp(),
            self.ks11_frtn(),
            self.ks11_fhwo(),
            self.ks11_ukpuk(),
            self.ks11_utecdh(),
            self.ks11_ucmac(),
            self.ks11_uksk(),
            self.ks11_urtf(),
            self.ks11_uckdf(),
            self.ks11_uhkdf(),
            self.ks11_uecsg(),
            self.ks11_uecdh(),
            self.ks11_uaes(),
            self.ks11_uhmac(),
            self.ks11_ukwk(),
            self.ks11_ukuok(),
            self.ks11_utlspms(),
            self.ks11_utlsms(),
            self.ks11_ukgsrc(),
            self.ks11_uhwo(),
            self.ks11_uwrpok(),
            self.ks11_uduk(),
            self.ks11_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs12(pub u32);
impl ElsKs12 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ksize(&self) -> super::vals::Ks12Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks12Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks12_ksize(&mut self, val: super::vals::Ks12Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks12_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks12_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks12_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks12_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks12_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks12_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks12_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks12_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks12_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks12_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks12_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks12_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks12_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks12_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks12_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks12_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks12_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks12_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks12_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks12_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks12_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks12_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks12_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks12_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks12_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks12_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs12 {
    #[inline(always)]
    fn default() -> ElsKs12 {
        ElsKs12(0)
    }
}
impl core::fmt::Debug for ElsKs12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs12")
            .field("ks12_ksize", &self.ks12_ksize())
            .field("ks12_kact", &self.ks12_kact())
            .field("ks12_kbase", &self.ks12_kbase())
            .field("ks12_fgp", &self.ks12_fgp())
            .field("ks12_frtn", &self.ks12_frtn())
            .field("ks12_fhwo", &self.ks12_fhwo())
            .field("ks12_ukpuk", &self.ks12_ukpuk())
            .field("ks12_utecdh", &self.ks12_utecdh())
            .field("ks12_ucmac", &self.ks12_ucmac())
            .field("ks12_uksk", &self.ks12_uksk())
            .field("ks12_urtf", &self.ks12_urtf())
            .field("ks12_uckdf", &self.ks12_uckdf())
            .field("ks12_uhkdf", &self.ks12_uhkdf())
            .field("ks12_uecsg", &self.ks12_uecsg())
            .field("ks12_uecdh", &self.ks12_uecdh())
            .field("ks12_uaes", &self.ks12_uaes())
            .field("ks12_uhmac", &self.ks12_uhmac())
            .field("ks12_ukwk", &self.ks12_ukwk())
            .field("ks12_ukuok", &self.ks12_ukuok())
            .field("ks12_utlspms", &self.ks12_utlspms())
            .field("ks12_utlsms", &self.ks12_utlsms())
            .field("ks12_ukgsrc", &self.ks12_ukgsrc())
            .field("ks12_uhwo", &self.ks12_uhwo())
            .field("ks12_uwrpok", &self.ks12_uwrpok())
            .field("ks12_uduk", &self.ks12_uduk())
            .field("ks12_upprot", &self.ks12_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs12 {{ ks12_ksize: {:?}, ks12_kact: {=bool:?}, ks12_kbase: {=bool:?}, ks12_fgp: {=bool:?}, ks12_frtn: {=bool:?}, ks12_fhwo: {=bool:?}, ks12_ukpuk: {=bool:?}, ks12_utecdh: {=bool:?}, ks12_ucmac: {=bool:?}, ks12_uksk: {=bool:?}, ks12_urtf: {=bool:?}, ks12_uckdf: {=bool:?}, ks12_uhkdf: {=bool:?}, ks12_uecsg: {=bool:?}, ks12_uecdh: {=bool:?}, ks12_uaes: {=bool:?}, ks12_uhmac: {=bool:?}, ks12_ukwk: {=bool:?}, ks12_ukuok: {=bool:?}, ks12_utlspms: {=bool:?}, ks12_utlsms: {=bool:?}, ks12_ukgsrc: {=bool:?}, ks12_uhwo: {=bool:?}, ks12_uwrpok: {=bool:?}, ks12_uduk: {=bool:?}, ks12_upprot: {=u8:?} }}",
            self.ks12_ksize(),
            self.ks12_kact(),
            self.ks12_kbase(),
            self.ks12_fgp(),
            self.ks12_frtn(),
            self.ks12_fhwo(),
            self.ks12_ukpuk(),
            self.ks12_utecdh(),
            self.ks12_ucmac(),
            self.ks12_uksk(),
            self.ks12_urtf(),
            self.ks12_uckdf(),
            self.ks12_uhkdf(),
            self.ks12_uecsg(),
            self.ks12_uecdh(),
            self.ks12_uaes(),
            self.ks12_uhmac(),
            self.ks12_ukwk(),
            self.ks12_ukuok(),
            self.ks12_utlspms(),
            self.ks12_utlsms(),
            self.ks12_ukgsrc(),
            self.ks12_uhwo(),
            self.ks12_uwrpok(),
            self.ks12_uduk(),
            self.ks12_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs13(pub u32);
impl ElsKs13 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ksize(&self) -> super::vals::Ks13Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks13Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks13_ksize(&mut self, val: super::vals::Ks13Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks13_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks13_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks13_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks13_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks13_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks13_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks13_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks13_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks13_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks13_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks13_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks13_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks13_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks13_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks13_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks13_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks13_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks13_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks13_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks13_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks13_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks13_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks13_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks13_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks13_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks13_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs13 {
    #[inline(always)]
    fn default() -> ElsKs13 {
        ElsKs13(0)
    }
}
impl core::fmt::Debug for ElsKs13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs13")
            .field("ks13_ksize", &self.ks13_ksize())
            .field("ks13_kact", &self.ks13_kact())
            .field("ks13_kbase", &self.ks13_kbase())
            .field("ks13_fgp", &self.ks13_fgp())
            .field("ks13_frtn", &self.ks13_frtn())
            .field("ks13_fhwo", &self.ks13_fhwo())
            .field("ks13_ukpuk", &self.ks13_ukpuk())
            .field("ks13_utecdh", &self.ks13_utecdh())
            .field("ks13_ucmac", &self.ks13_ucmac())
            .field("ks13_uksk", &self.ks13_uksk())
            .field("ks13_urtf", &self.ks13_urtf())
            .field("ks13_uckdf", &self.ks13_uckdf())
            .field("ks13_uhkdf", &self.ks13_uhkdf())
            .field("ks13_uecsg", &self.ks13_uecsg())
            .field("ks13_uecdh", &self.ks13_uecdh())
            .field("ks13_uaes", &self.ks13_uaes())
            .field("ks13_uhmac", &self.ks13_uhmac())
            .field("ks13_ukwk", &self.ks13_ukwk())
            .field("ks13_ukuok", &self.ks13_ukuok())
            .field("ks13_utlspms", &self.ks13_utlspms())
            .field("ks13_utlsms", &self.ks13_utlsms())
            .field("ks13_ukgsrc", &self.ks13_ukgsrc())
            .field("ks13_uhwo", &self.ks13_uhwo())
            .field("ks13_uwrpok", &self.ks13_uwrpok())
            .field("ks13_uduk", &self.ks13_uduk())
            .field("ks13_upprot", &self.ks13_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs13 {{ ks13_ksize: {:?}, ks13_kact: {=bool:?}, ks13_kbase: {=bool:?}, ks13_fgp: {=bool:?}, ks13_frtn: {=bool:?}, ks13_fhwo: {=bool:?}, ks13_ukpuk: {=bool:?}, ks13_utecdh: {=bool:?}, ks13_ucmac: {=bool:?}, ks13_uksk: {=bool:?}, ks13_urtf: {=bool:?}, ks13_uckdf: {=bool:?}, ks13_uhkdf: {=bool:?}, ks13_uecsg: {=bool:?}, ks13_uecdh: {=bool:?}, ks13_uaes: {=bool:?}, ks13_uhmac: {=bool:?}, ks13_ukwk: {=bool:?}, ks13_ukuok: {=bool:?}, ks13_utlspms: {=bool:?}, ks13_utlsms: {=bool:?}, ks13_ukgsrc: {=bool:?}, ks13_uhwo: {=bool:?}, ks13_uwrpok: {=bool:?}, ks13_uduk: {=bool:?}, ks13_upprot: {=u8:?} }}",
            self.ks13_ksize(),
            self.ks13_kact(),
            self.ks13_kbase(),
            self.ks13_fgp(),
            self.ks13_frtn(),
            self.ks13_fhwo(),
            self.ks13_ukpuk(),
            self.ks13_utecdh(),
            self.ks13_ucmac(),
            self.ks13_uksk(),
            self.ks13_urtf(),
            self.ks13_uckdf(),
            self.ks13_uhkdf(),
            self.ks13_uecsg(),
            self.ks13_uecdh(),
            self.ks13_uaes(),
            self.ks13_uhmac(),
            self.ks13_ukwk(),
            self.ks13_ukuok(),
            self.ks13_utlspms(),
            self.ks13_utlsms(),
            self.ks13_ukgsrc(),
            self.ks13_uhwo(),
            self.ks13_uwrpok(),
            self.ks13_uduk(),
            self.ks13_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs14(pub u32);
impl ElsKs14 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ksize(&self) -> super::vals::Ks14Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks14Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks14_ksize(&mut self, val: super::vals::Ks14Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks14_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks14_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks14_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks14_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks14_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks14_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks14_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks14_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks14_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks14_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks14_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks14_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks14_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks14_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks14_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks14_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks14_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks14_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks14_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks14_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks14_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks14_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks14_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks14_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks14_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks14_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs14 {
    #[inline(always)]
    fn default() -> ElsKs14 {
        ElsKs14(0)
    }
}
impl core::fmt::Debug for ElsKs14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs14")
            .field("ks14_ksize", &self.ks14_ksize())
            .field("ks14_kact", &self.ks14_kact())
            .field("ks14_kbase", &self.ks14_kbase())
            .field("ks14_fgp", &self.ks14_fgp())
            .field("ks14_frtn", &self.ks14_frtn())
            .field("ks14_fhwo", &self.ks14_fhwo())
            .field("ks14_ukpuk", &self.ks14_ukpuk())
            .field("ks14_utecdh", &self.ks14_utecdh())
            .field("ks14_ucmac", &self.ks14_ucmac())
            .field("ks14_uksk", &self.ks14_uksk())
            .field("ks14_urtf", &self.ks14_urtf())
            .field("ks14_uckdf", &self.ks14_uckdf())
            .field("ks14_uhkdf", &self.ks14_uhkdf())
            .field("ks14_uecsg", &self.ks14_uecsg())
            .field("ks14_uecdh", &self.ks14_uecdh())
            .field("ks14_uaes", &self.ks14_uaes())
            .field("ks14_uhmac", &self.ks14_uhmac())
            .field("ks14_ukwk", &self.ks14_ukwk())
            .field("ks14_ukuok", &self.ks14_ukuok())
            .field("ks14_utlspms", &self.ks14_utlspms())
            .field("ks14_utlsms", &self.ks14_utlsms())
            .field("ks14_ukgsrc", &self.ks14_ukgsrc())
            .field("ks14_uhwo", &self.ks14_uhwo())
            .field("ks14_uwrpok", &self.ks14_uwrpok())
            .field("ks14_uduk", &self.ks14_uduk())
            .field("ks14_upprot", &self.ks14_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs14 {{ ks14_ksize: {:?}, ks14_kact: {=bool:?}, ks14_kbase: {=bool:?}, ks14_fgp: {=bool:?}, ks14_frtn: {=bool:?}, ks14_fhwo: {=bool:?}, ks14_ukpuk: {=bool:?}, ks14_utecdh: {=bool:?}, ks14_ucmac: {=bool:?}, ks14_uksk: {=bool:?}, ks14_urtf: {=bool:?}, ks14_uckdf: {=bool:?}, ks14_uhkdf: {=bool:?}, ks14_uecsg: {=bool:?}, ks14_uecdh: {=bool:?}, ks14_uaes: {=bool:?}, ks14_uhmac: {=bool:?}, ks14_ukwk: {=bool:?}, ks14_ukuok: {=bool:?}, ks14_utlspms: {=bool:?}, ks14_utlsms: {=bool:?}, ks14_ukgsrc: {=bool:?}, ks14_uhwo: {=bool:?}, ks14_uwrpok: {=bool:?}, ks14_uduk: {=bool:?}, ks14_upprot: {=u8:?} }}",
            self.ks14_ksize(),
            self.ks14_kact(),
            self.ks14_kbase(),
            self.ks14_fgp(),
            self.ks14_frtn(),
            self.ks14_fhwo(),
            self.ks14_ukpuk(),
            self.ks14_utecdh(),
            self.ks14_ucmac(),
            self.ks14_uksk(),
            self.ks14_urtf(),
            self.ks14_uckdf(),
            self.ks14_uhkdf(),
            self.ks14_uecsg(),
            self.ks14_uecdh(),
            self.ks14_uaes(),
            self.ks14_uhmac(),
            self.ks14_ukwk(),
            self.ks14_ukuok(),
            self.ks14_utlspms(),
            self.ks14_utlsms(),
            self.ks14_ukgsrc(),
            self.ks14_uhwo(),
            self.ks14_uwrpok(),
            self.ks14_uduk(),
            self.ks14_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs15(pub u32);
impl ElsKs15 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ksize(&self) -> super::vals::Ks15Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks15Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks15_ksize(&mut self, val: super::vals::Ks15Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks15_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks15_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks15_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks15_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks15_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks15_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks15_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks15_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks15_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks15_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks15_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks15_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks15_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks15_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks15_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks15_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks15_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks15_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks15_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks15_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks15_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks15_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks15_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks15_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks15_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks15_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs15 {
    #[inline(always)]
    fn default() -> ElsKs15 {
        ElsKs15(0)
    }
}
impl core::fmt::Debug for ElsKs15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs15")
            .field("ks15_ksize", &self.ks15_ksize())
            .field("ks15_kact", &self.ks15_kact())
            .field("ks15_kbase", &self.ks15_kbase())
            .field("ks15_fgp", &self.ks15_fgp())
            .field("ks15_frtn", &self.ks15_frtn())
            .field("ks15_fhwo", &self.ks15_fhwo())
            .field("ks15_ukpuk", &self.ks15_ukpuk())
            .field("ks15_utecdh", &self.ks15_utecdh())
            .field("ks15_ucmac", &self.ks15_ucmac())
            .field("ks15_uksk", &self.ks15_uksk())
            .field("ks15_urtf", &self.ks15_urtf())
            .field("ks15_uckdf", &self.ks15_uckdf())
            .field("ks15_uhkdf", &self.ks15_uhkdf())
            .field("ks15_uecsg", &self.ks15_uecsg())
            .field("ks15_uecdh", &self.ks15_uecdh())
            .field("ks15_uaes", &self.ks15_uaes())
            .field("ks15_uhmac", &self.ks15_uhmac())
            .field("ks15_ukwk", &self.ks15_ukwk())
            .field("ks15_ukuok", &self.ks15_ukuok())
            .field("ks15_utlspms", &self.ks15_utlspms())
            .field("ks15_utlsms", &self.ks15_utlsms())
            .field("ks15_ukgsrc", &self.ks15_ukgsrc())
            .field("ks15_uhwo", &self.ks15_uhwo())
            .field("ks15_uwrpok", &self.ks15_uwrpok())
            .field("ks15_uduk", &self.ks15_uduk())
            .field("ks15_upprot", &self.ks15_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs15 {{ ks15_ksize: {:?}, ks15_kact: {=bool:?}, ks15_kbase: {=bool:?}, ks15_fgp: {=bool:?}, ks15_frtn: {=bool:?}, ks15_fhwo: {=bool:?}, ks15_ukpuk: {=bool:?}, ks15_utecdh: {=bool:?}, ks15_ucmac: {=bool:?}, ks15_uksk: {=bool:?}, ks15_urtf: {=bool:?}, ks15_uckdf: {=bool:?}, ks15_uhkdf: {=bool:?}, ks15_uecsg: {=bool:?}, ks15_uecdh: {=bool:?}, ks15_uaes: {=bool:?}, ks15_uhmac: {=bool:?}, ks15_ukwk: {=bool:?}, ks15_ukuok: {=bool:?}, ks15_utlspms: {=bool:?}, ks15_utlsms: {=bool:?}, ks15_ukgsrc: {=bool:?}, ks15_uhwo: {=bool:?}, ks15_uwrpok: {=bool:?}, ks15_uduk: {=bool:?}, ks15_upprot: {=u8:?} }}",
            self.ks15_ksize(),
            self.ks15_kact(),
            self.ks15_kbase(),
            self.ks15_fgp(),
            self.ks15_frtn(),
            self.ks15_fhwo(),
            self.ks15_ukpuk(),
            self.ks15_utecdh(),
            self.ks15_ucmac(),
            self.ks15_uksk(),
            self.ks15_urtf(),
            self.ks15_uckdf(),
            self.ks15_uhkdf(),
            self.ks15_uecsg(),
            self.ks15_uecdh(),
            self.ks15_uaes(),
            self.ks15_uhmac(),
            self.ks15_ukwk(),
            self.ks15_ukuok(),
            self.ks15_utlspms(),
            self.ks15_utlsms(),
            self.ks15_ukgsrc(),
            self.ks15_uhwo(),
            self.ks15_uwrpok(),
            self.ks15_uduk(),
            self.ks15_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs16(pub u32);
impl ElsKs16 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ksize(&self) -> super::vals::Ks16Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks16Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks16_ksize(&mut self, val: super::vals::Ks16Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks16_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks16_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks16_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks16_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks16_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks16_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks16_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks16_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks16_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks16_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks16_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks16_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks16_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks16_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks16_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks16_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks16_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks16_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks16_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks16_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks16_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks16_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks16_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks16_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks16_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks16_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs16 {
    #[inline(always)]
    fn default() -> ElsKs16 {
        ElsKs16(0)
    }
}
impl core::fmt::Debug for ElsKs16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs16")
            .field("ks16_ksize", &self.ks16_ksize())
            .field("ks16_kact", &self.ks16_kact())
            .field("ks16_kbase", &self.ks16_kbase())
            .field("ks16_fgp", &self.ks16_fgp())
            .field("ks16_frtn", &self.ks16_frtn())
            .field("ks16_fhwo", &self.ks16_fhwo())
            .field("ks16_ukpuk", &self.ks16_ukpuk())
            .field("ks16_utecdh", &self.ks16_utecdh())
            .field("ks16_ucmac", &self.ks16_ucmac())
            .field("ks16_uksk", &self.ks16_uksk())
            .field("ks16_urtf", &self.ks16_urtf())
            .field("ks16_uckdf", &self.ks16_uckdf())
            .field("ks16_uhkdf", &self.ks16_uhkdf())
            .field("ks16_uecsg", &self.ks16_uecsg())
            .field("ks16_uecdh", &self.ks16_uecdh())
            .field("ks16_uaes", &self.ks16_uaes())
            .field("ks16_uhmac", &self.ks16_uhmac())
            .field("ks16_ukwk", &self.ks16_ukwk())
            .field("ks16_ukuok", &self.ks16_ukuok())
            .field("ks16_utlspms", &self.ks16_utlspms())
            .field("ks16_utlsms", &self.ks16_utlsms())
            .field("ks16_ukgsrc", &self.ks16_ukgsrc())
            .field("ks16_uhwo", &self.ks16_uhwo())
            .field("ks16_uwrpok", &self.ks16_uwrpok())
            .field("ks16_uduk", &self.ks16_uduk())
            .field("ks16_upprot", &self.ks16_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs16 {{ ks16_ksize: {:?}, ks16_kact: {=bool:?}, ks16_kbase: {=bool:?}, ks16_fgp: {=bool:?}, ks16_frtn: {=bool:?}, ks16_fhwo: {=bool:?}, ks16_ukpuk: {=bool:?}, ks16_utecdh: {=bool:?}, ks16_ucmac: {=bool:?}, ks16_uksk: {=bool:?}, ks16_urtf: {=bool:?}, ks16_uckdf: {=bool:?}, ks16_uhkdf: {=bool:?}, ks16_uecsg: {=bool:?}, ks16_uecdh: {=bool:?}, ks16_uaes: {=bool:?}, ks16_uhmac: {=bool:?}, ks16_ukwk: {=bool:?}, ks16_ukuok: {=bool:?}, ks16_utlspms: {=bool:?}, ks16_utlsms: {=bool:?}, ks16_ukgsrc: {=bool:?}, ks16_uhwo: {=bool:?}, ks16_uwrpok: {=bool:?}, ks16_uduk: {=bool:?}, ks16_upprot: {=u8:?} }}",
            self.ks16_ksize(),
            self.ks16_kact(),
            self.ks16_kbase(),
            self.ks16_fgp(),
            self.ks16_frtn(),
            self.ks16_fhwo(),
            self.ks16_ukpuk(),
            self.ks16_utecdh(),
            self.ks16_ucmac(),
            self.ks16_uksk(),
            self.ks16_urtf(),
            self.ks16_uckdf(),
            self.ks16_uhkdf(),
            self.ks16_uecsg(),
            self.ks16_uecdh(),
            self.ks16_uaes(),
            self.ks16_uhmac(),
            self.ks16_ukwk(),
            self.ks16_ukuok(),
            self.ks16_utlspms(),
            self.ks16_utlsms(),
            self.ks16_ukgsrc(),
            self.ks16_uhwo(),
            self.ks16_uwrpok(),
            self.ks16_uduk(),
            self.ks16_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs17(pub u32);
impl ElsKs17 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ksize(&self) -> super::vals::Ks17Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks17Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks17_ksize(&mut self, val: super::vals::Ks17Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks17_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks17_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks17_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks17_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks17_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks17_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks17_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks17_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks17_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks17_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks17_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks17_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks17_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks17_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks17_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks17_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks17_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks17_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks17_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks17_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks17_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks17_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks17_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks17_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks17_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks17_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs17 {
    #[inline(always)]
    fn default() -> ElsKs17 {
        ElsKs17(0)
    }
}
impl core::fmt::Debug for ElsKs17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs17")
            .field("ks17_ksize", &self.ks17_ksize())
            .field("ks17_kact", &self.ks17_kact())
            .field("ks17_kbase", &self.ks17_kbase())
            .field("ks17_fgp", &self.ks17_fgp())
            .field("ks17_frtn", &self.ks17_frtn())
            .field("ks17_fhwo", &self.ks17_fhwo())
            .field("ks17_ukpuk", &self.ks17_ukpuk())
            .field("ks17_utecdh", &self.ks17_utecdh())
            .field("ks17_ucmac", &self.ks17_ucmac())
            .field("ks17_uksk", &self.ks17_uksk())
            .field("ks17_urtf", &self.ks17_urtf())
            .field("ks17_uckdf", &self.ks17_uckdf())
            .field("ks17_uhkdf", &self.ks17_uhkdf())
            .field("ks17_uecsg", &self.ks17_uecsg())
            .field("ks17_uecdh", &self.ks17_uecdh())
            .field("ks17_uaes", &self.ks17_uaes())
            .field("ks17_uhmac", &self.ks17_uhmac())
            .field("ks17_ukwk", &self.ks17_ukwk())
            .field("ks17_ukuok", &self.ks17_ukuok())
            .field("ks17_utlspms", &self.ks17_utlspms())
            .field("ks17_utlsms", &self.ks17_utlsms())
            .field("ks17_ukgsrc", &self.ks17_ukgsrc())
            .field("ks17_uhwo", &self.ks17_uhwo())
            .field("ks17_uwrpok", &self.ks17_uwrpok())
            .field("ks17_uduk", &self.ks17_uduk())
            .field("ks17_upprot", &self.ks17_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs17 {{ ks17_ksize: {:?}, ks17_kact: {=bool:?}, ks17_kbase: {=bool:?}, ks17_fgp: {=bool:?}, ks17_frtn: {=bool:?}, ks17_fhwo: {=bool:?}, ks17_ukpuk: {=bool:?}, ks17_utecdh: {=bool:?}, ks17_ucmac: {=bool:?}, ks17_uksk: {=bool:?}, ks17_urtf: {=bool:?}, ks17_uckdf: {=bool:?}, ks17_uhkdf: {=bool:?}, ks17_uecsg: {=bool:?}, ks17_uecdh: {=bool:?}, ks17_uaes: {=bool:?}, ks17_uhmac: {=bool:?}, ks17_ukwk: {=bool:?}, ks17_ukuok: {=bool:?}, ks17_utlspms: {=bool:?}, ks17_utlsms: {=bool:?}, ks17_ukgsrc: {=bool:?}, ks17_uhwo: {=bool:?}, ks17_uwrpok: {=bool:?}, ks17_uduk: {=bool:?}, ks17_upprot: {=u8:?} }}",
            self.ks17_ksize(),
            self.ks17_kact(),
            self.ks17_kbase(),
            self.ks17_fgp(),
            self.ks17_frtn(),
            self.ks17_fhwo(),
            self.ks17_ukpuk(),
            self.ks17_utecdh(),
            self.ks17_ucmac(),
            self.ks17_uksk(),
            self.ks17_urtf(),
            self.ks17_uckdf(),
            self.ks17_uhkdf(),
            self.ks17_uecsg(),
            self.ks17_uecdh(),
            self.ks17_uaes(),
            self.ks17_uhmac(),
            self.ks17_ukwk(),
            self.ks17_ukuok(),
            self.ks17_utlspms(),
            self.ks17_utlsms(),
            self.ks17_ukgsrc(),
            self.ks17_uhwo(),
            self.ks17_uwrpok(),
            self.ks17_uduk(),
            self.ks17_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs18(pub u32);
impl ElsKs18 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ksize(&self) -> super::vals::Ks18Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks18Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks18_ksize(&mut self, val: super::vals::Ks18Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks18_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks18_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks18_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks18_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks18_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks18_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks18_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks18_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks18_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks18_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks18_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks18_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks18_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks18_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks18_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks18_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks18_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks18_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks18_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks18_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks18_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks18_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks18_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks18_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks18_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks18_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs18 {
    #[inline(always)]
    fn default() -> ElsKs18 {
        ElsKs18(0)
    }
}
impl core::fmt::Debug for ElsKs18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs18")
            .field("ks18_ksize", &self.ks18_ksize())
            .field("ks18_kact", &self.ks18_kact())
            .field("ks18_kbase", &self.ks18_kbase())
            .field("ks18_fgp", &self.ks18_fgp())
            .field("ks18_frtn", &self.ks18_frtn())
            .field("ks18_fhwo", &self.ks18_fhwo())
            .field("ks18_ukpuk", &self.ks18_ukpuk())
            .field("ks18_utecdh", &self.ks18_utecdh())
            .field("ks18_ucmac", &self.ks18_ucmac())
            .field("ks18_uksk", &self.ks18_uksk())
            .field("ks18_urtf", &self.ks18_urtf())
            .field("ks18_uckdf", &self.ks18_uckdf())
            .field("ks18_uhkdf", &self.ks18_uhkdf())
            .field("ks18_uecsg", &self.ks18_uecsg())
            .field("ks18_uecdh", &self.ks18_uecdh())
            .field("ks18_uaes", &self.ks18_uaes())
            .field("ks18_uhmac", &self.ks18_uhmac())
            .field("ks18_ukwk", &self.ks18_ukwk())
            .field("ks18_ukuok", &self.ks18_ukuok())
            .field("ks18_utlspms", &self.ks18_utlspms())
            .field("ks18_utlsms", &self.ks18_utlsms())
            .field("ks18_ukgsrc", &self.ks18_ukgsrc())
            .field("ks18_uhwo", &self.ks18_uhwo())
            .field("ks18_uwrpok", &self.ks18_uwrpok())
            .field("ks18_uduk", &self.ks18_uduk())
            .field("ks18_upprot", &self.ks18_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs18 {{ ks18_ksize: {:?}, ks18_kact: {=bool:?}, ks18_kbase: {=bool:?}, ks18_fgp: {=bool:?}, ks18_frtn: {=bool:?}, ks18_fhwo: {=bool:?}, ks18_ukpuk: {=bool:?}, ks18_utecdh: {=bool:?}, ks18_ucmac: {=bool:?}, ks18_uksk: {=bool:?}, ks18_urtf: {=bool:?}, ks18_uckdf: {=bool:?}, ks18_uhkdf: {=bool:?}, ks18_uecsg: {=bool:?}, ks18_uecdh: {=bool:?}, ks18_uaes: {=bool:?}, ks18_uhmac: {=bool:?}, ks18_ukwk: {=bool:?}, ks18_ukuok: {=bool:?}, ks18_utlspms: {=bool:?}, ks18_utlsms: {=bool:?}, ks18_ukgsrc: {=bool:?}, ks18_uhwo: {=bool:?}, ks18_uwrpok: {=bool:?}, ks18_uduk: {=bool:?}, ks18_upprot: {=u8:?} }}",
            self.ks18_ksize(),
            self.ks18_kact(),
            self.ks18_kbase(),
            self.ks18_fgp(),
            self.ks18_frtn(),
            self.ks18_fhwo(),
            self.ks18_ukpuk(),
            self.ks18_utecdh(),
            self.ks18_ucmac(),
            self.ks18_uksk(),
            self.ks18_urtf(),
            self.ks18_uckdf(),
            self.ks18_uhkdf(),
            self.ks18_uecsg(),
            self.ks18_uecdh(),
            self.ks18_uaes(),
            self.ks18_uhmac(),
            self.ks18_ukwk(),
            self.ks18_ukuok(),
            self.ks18_utlspms(),
            self.ks18_utlsms(),
            self.ks18_ukgsrc(),
            self.ks18_uhwo(),
            self.ks18_uwrpok(),
            self.ks18_uduk(),
            self.ks18_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs19(pub u32);
impl ElsKs19 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ksize(&self) -> super::vals::Ks19Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks19Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks19_ksize(&mut self, val: super::vals::Ks19Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks19_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks19_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks19_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks19_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks19_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks19_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks19_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks19_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks19_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks19_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks19_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks19_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks19_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks19_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks19_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks19_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks19_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks19_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks19_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks19_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks19_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks19_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks19_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks19_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks19_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks19_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs19 {
    #[inline(always)]
    fn default() -> ElsKs19 {
        ElsKs19(0)
    }
}
impl core::fmt::Debug for ElsKs19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs19")
            .field("ks19_ksize", &self.ks19_ksize())
            .field("ks19_kact", &self.ks19_kact())
            .field("ks19_kbase", &self.ks19_kbase())
            .field("ks19_fgp", &self.ks19_fgp())
            .field("ks19_frtn", &self.ks19_frtn())
            .field("ks19_fhwo", &self.ks19_fhwo())
            .field("ks19_ukpuk", &self.ks19_ukpuk())
            .field("ks19_utecdh", &self.ks19_utecdh())
            .field("ks19_ucmac", &self.ks19_ucmac())
            .field("ks19_uksk", &self.ks19_uksk())
            .field("ks19_urtf", &self.ks19_urtf())
            .field("ks19_uckdf", &self.ks19_uckdf())
            .field("ks19_uhkdf", &self.ks19_uhkdf())
            .field("ks19_uecsg", &self.ks19_uecsg())
            .field("ks19_uecdh", &self.ks19_uecdh())
            .field("ks19_uaes", &self.ks19_uaes())
            .field("ks19_uhmac", &self.ks19_uhmac())
            .field("ks19_ukwk", &self.ks19_ukwk())
            .field("ks19_ukuok", &self.ks19_ukuok())
            .field("ks19_utlspms", &self.ks19_utlspms())
            .field("ks19_utlsms", &self.ks19_utlsms())
            .field("ks19_ukgsrc", &self.ks19_ukgsrc())
            .field("ks19_uhwo", &self.ks19_uhwo())
            .field("ks19_uwrpok", &self.ks19_uwrpok())
            .field("ks19_uduk", &self.ks19_uduk())
            .field("ks19_upprot", &self.ks19_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs19 {{ ks19_ksize: {:?}, ks19_kact: {=bool:?}, ks19_kbase: {=bool:?}, ks19_fgp: {=bool:?}, ks19_frtn: {=bool:?}, ks19_fhwo: {=bool:?}, ks19_ukpuk: {=bool:?}, ks19_utecdh: {=bool:?}, ks19_ucmac: {=bool:?}, ks19_uksk: {=bool:?}, ks19_urtf: {=bool:?}, ks19_uckdf: {=bool:?}, ks19_uhkdf: {=bool:?}, ks19_uecsg: {=bool:?}, ks19_uecdh: {=bool:?}, ks19_uaes: {=bool:?}, ks19_uhmac: {=bool:?}, ks19_ukwk: {=bool:?}, ks19_ukuok: {=bool:?}, ks19_utlspms: {=bool:?}, ks19_utlsms: {=bool:?}, ks19_ukgsrc: {=bool:?}, ks19_uhwo: {=bool:?}, ks19_uwrpok: {=bool:?}, ks19_uduk: {=bool:?}, ks19_upprot: {=u8:?} }}",
            self.ks19_ksize(),
            self.ks19_kact(),
            self.ks19_kbase(),
            self.ks19_fgp(),
            self.ks19_frtn(),
            self.ks19_fhwo(),
            self.ks19_ukpuk(),
            self.ks19_utecdh(),
            self.ks19_ucmac(),
            self.ks19_uksk(),
            self.ks19_urtf(),
            self.ks19_uckdf(),
            self.ks19_uhkdf(),
            self.ks19_uecsg(),
            self.ks19_uecdh(),
            self.ks19_uaes(),
            self.ks19_uhmac(),
            self.ks19_ukwk(),
            self.ks19_ukuok(),
            self.ks19_utlspms(),
            self.ks19_utlsms(),
            self.ks19_ukgsrc(),
            self.ks19_uhwo(),
            self.ks19_uwrpok(),
            self.ks19_uduk(),
            self.ks19_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs2(pub u32);
impl ElsKs2 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ksize(&self) -> super::vals::Ks2Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks2Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks2_ksize(&mut self, val: super::vals::Ks2Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks2_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks2_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks2_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks2_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks2_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks2_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks2_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks2_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks2_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks2_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks2_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks2_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks2_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks2_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks2_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks2_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks2_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks2_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks2_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks2_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks2_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks2_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks2_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks2_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks2_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks2_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs2 {
    #[inline(always)]
    fn default() -> ElsKs2 {
        ElsKs2(0)
    }
}
impl core::fmt::Debug for ElsKs2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs2")
            .field("ks2_ksize", &self.ks2_ksize())
            .field("ks2_kact", &self.ks2_kact())
            .field("ks2_kbase", &self.ks2_kbase())
            .field("ks2_fgp", &self.ks2_fgp())
            .field("ks2_frtn", &self.ks2_frtn())
            .field("ks2_fhwo", &self.ks2_fhwo())
            .field("ks2_ukpuk", &self.ks2_ukpuk())
            .field("ks2_utecdh", &self.ks2_utecdh())
            .field("ks2_ucmac", &self.ks2_ucmac())
            .field("ks2_uksk", &self.ks2_uksk())
            .field("ks2_urtf", &self.ks2_urtf())
            .field("ks2_uckdf", &self.ks2_uckdf())
            .field("ks2_uhkdf", &self.ks2_uhkdf())
            .field("ks2_uecsg", &self.ks2_uecsg())
            .field("ks2_uecdh", &self.ks2_uecdh())
            .field("ks2_uaes", &self.ks2_uaes())
            .field("ks2_uhmac", &self.ks2_uhmac())
            .field("ks2_ukwk", &self.ks2_ukwk())
            .field("ks2_ukuok", &self.ks2_ukuok())
            .field("ks2_utlspms", &self.ks2_utlspms())
            .field("ks2_utlsms", &self.ks2_utlsms())
            .field("ks2_ukgsrc", &self.ks2_ukgsrc())
            .field("ks2_uhwo", &self.ks2_uhwo())
            .field("ks2_uwrpok", &self.ks2_uwrpok())
            .field("ks2_uduk", &self.ks2_uduk())
            .field("ks2_upprot", &self.ks2_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs2 {{ ks2_ksize: {:?}, ks2_kact: {=bool:?}, ks2_kbase: {=bool:?}, ks2_fgp: {=bool:?}, ks2_frtn: {=bool:?}, ks2_fhwo: {=bool:?}, ks2_ukpuk: {=bool:?}, ks2_utecdh: {=bool:?}, ks2_ucmac: {=bool:?}, ks2_uksk: {=bool:?}, ks2_urtf: {=bool:?}, ks2_uckdf: {=bool:?}, ks2_uhkdf: {=bool:?}, ks2_uecsg: {=bool:?}, ks2_uecdh: {=bool:?}, ks2_uaes: {=bool:?}, ks2_uhmac: {=bool:?}, ks2_ukwk: {=bool:?}, ks2_ukuok: {=bool:?}, ks2_utlspms: {=bool:?}, ks2_utlsms: {=bool:?}, ks2_ukgsrc: {=bool:?}, ks2_uhwo: {=bool:?}, ks2_uwrpok: {=bool:?}, ks2_uduk: {=bool:?}, ks2_upprot: {=u8:?} }}",
            self.ks2_ksize(),
            self.ks2_kact(),
            self.ks2_kbase(),
            self.ks2_fgp(),
            self.ks2_frtn(),
            self.ks2_fhwo(),
            self.ks2_ukpuk(),
            self.ks2_utecdh(),
            self.ks2_ucmac(),
            self.ks2_uksk(),
            self.ks2_urtf(),
            self.ks2_uckdf(),
            self.ks2_uhkdf(),
            self.ks2_uecsg(),
            self.ks2_uecdh(),
            self.ks2_uaes(),
            self.ks2_uhmac(),
            self.ks2_ukwk(),
            self.ks2_ukuok(),
            self.ks2_utlspms(),
            self.ks2_utlsms(),
            self.ks2_ukgsrc(),
            self.ks2_uhwo(),
            self.ks2_uwrpok(),
            self.ks2_uduk(),
            self.ks2_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs3(pub u32);
impl ElsKs3 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ksize(&self) -> super::vals::Ks3Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks3Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks3_ksize(&mut self, val: super::vals::Ks3Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks3_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks3_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks3_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks3_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks3_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks3_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks3_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks3_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks3_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks3_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks3_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks3_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks3_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks3_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks3_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks3_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks3_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks3_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks3_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks3_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks3_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks3_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks3_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks3_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks3_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks3_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs3 {
    #[inline(always)]
    fn default() -> ElsKs3 {
        ElsKs3(0)
    }
}
impl core::fmt::Debug for ElsKs3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs3")
            .field("ks3_ksize", &self.ks3_ksize())
            .field("ks3_kact", &self.ks3_kact())
            .field("ks3_kbase", &self.ks3_kbase())
            .field("ks3_fgp", &self.ks3_fgp())
            .field("ks3_frtn", &self.ks3_frtn())
            .field("ks3_fhwo", &self.ks3_fhwo())
            .field("ks3_ukpuk", &self.ks3_ukpuk())
            .field("ks3_utecdh", &self.ks3_utecdh())
            .field("ks3_ucmac", &self.ks3_ucmac())
            .field("ks3_uksk", &self.ks3_uksk())
            .field("ks3_urtf", &self.ks3_urtf())
            .field("ks3_uckdf", &self.ks3_uckdf())
            .field("ks3_uhkdf", &self.ks3_uhkdf())
            .field("ks3_uecsg", &self.ks3_uecsg())
            .field("ks3_uecdh", &self.ks3_uecdh())
            .field("ks3_uaes", &self.ks3_uaes())
            .field("ks3_uhmac", &self.ks3_uhmac())
            .field("ks3_ukwk", &self.ks3_ukwk())
            .field("ks3_ukuok", &self.ks3_ukuok())
            .field("ks3_utlspms", &self.ks3_utlspms())
            .field("ks3_utlsms", &self.ks3_utlsms())
            .field("ks3_ukgsrc", &self.ks3_ukgsrc())
            .field("ks3_uhwo", &self.ks3_uhwo())
            .field("ks3_uwrpok", &self.ks3_uwrpok())
            .field("ks3_uduk", &self.ks3_uduk())
            .field("ks3_upprot", &self.ks3_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs3 {{ ks3_ksize: {:?}, ks3_kact: {=bool:?}, ks3_kbase: {=bool:?}, ks3_fgp: {=bool:?}, ks3_frtn: {=bool:?}, ks3_fhwo: {=bool:?}, ks3_ukpuk: {=bool:?}, ks3_utecdh: {=bool:?}, ks3_ucmac: {=bool:?}, ks3_uksk: {=bool:?}, ks3_urtf: {=bool:?}, ks3_uckdf: {=bool:?}, ks3_uhkdf: {=bool:?}, ks3_uecsg: {=bool:?}, ks3_uecdh: {=bool:?}, ks3_uaes: {=bool:?}, ks3_uhmac: {=bool:?}, ks3_ukwk: {=bool:?}, ks3_ukuok: {=bool:?}, ks3_utlspms: {=bool:?}, ks3_utlsms: {=bool:?}, ks3_ukgsrc: {=bool:?}, ks3_uhwo: {=bool:?}, ks3_uwrpok: {=bool:?}, ks3_uduk: {=bool:?}, ks3_upprot: {=u8:?} }}",
            self.ks3_ksize(),
            self.ks3_kact(),
            self.ks3_kbase(),
            self.ks3_fgp(),
            self.ks3_frtn(),
            self.ks3_fhwo(),
            self.ks3_ukpuk(),
            self.ks3_utecdh(),
            self.ks3_ucmac(),
            self.ks3_uksk(),
            self.ks3_urtf(),
            self.ks3_uckdf(),
            self.ks3_uhkdf(),
            self.ks3_uecsg(),
            self.ks3_uecdh(),
            self.ks3_uaes(),
            self.ks3_uhmac(),
            self.ks3_ukwk(),
            self.ks3_ukuok(),
            self.ks3_utlspms(),
            self.ks3_utlsms(),
            self.ks3_ukgsrc(),
            self.ks3_uhwo(),
            self.ks3_uwrpok(),
            self.ks3_uduk(),
            self.ks3_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs4(pub u32);
impl ElsKs4 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ksize(&self) -> super::vals::Ks4Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks4Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks4_ksize(&mut self, val: super::vals::Ks4Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks4_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks4_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks4_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks4_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks4_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks4_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks4_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks4_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks4_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks4_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks4_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks4_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks4_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks4_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks4_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks4_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks4_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks4_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks4_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks4_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks4_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks4_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks4_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks4_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks4_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks4_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs4 {
    #[inline(always)]
    fn default() -> ElsKs4 {
        ElsKs4(0)
    }
}
impl core::fmt::Debug for ElsKs4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs4")
            .field("ks4_ksize", &self.ks4_ksize())
            .field("ks4_kact", &self.ks4_kact())
            .field("ks4_kbase", &self.ks4_kbase())
            .field("ks4_fgp", &self.ks4_fgp())
            .field("ks4_frtn", &self.ks4_frtn())
            .field("ks4_fhwo", &self.ks4_fhwo())
            .field("ks4_ukpuk", &self.ks4_ukpuk())
            .field("ks4_utecdh", &self.ks4_utecdh())
            .field("ks4_ucmac", &self.ks4_ucmac())
            .field("ks4_uksk", &self.ks4_uksk())
            .field("ks4_urtf", &self.ks4_urtf())
            .field("ks4_uckdf", &self.ks4_uckdf())
            .field("ks4_uhkdf", &self.ks4_uhkdf())
            .field("ks4_uecsg", &self.ks4_uecsg())
            .field("ks4_uecdh", &self.ks4_uecdh())
            .field("ks4_uaes", &self.ks4_uaes())
            .field("ks4_uhmac", &self.ks4_uhmac())
            .field("ks4_ukwk", &self.ks4_ukwk())
            .field("ks4_ukuok", &self.ks4_ukuok())
            .field("ks4_utlspms", &self.ks4_utlspms())
            .field("ks4_utlsms", &self.ks4_utlsms())
            .field("ks4_ukgsrc", &self.ks4_ukgsrc())
            .field("ks4_uhwo", &self.ks4_uhwo())
            .field("ks4_uwrpok", &self.ks4_uwrpok())
            .field("ks4_uduk", &self.ks4_uduk())
            .field("ks4_upprot", &self.ks4_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs4 {{ ks4_ksize: {:?}, ks4_kact: {=bool:?}, ks4_kbase: {=bool:?}, ks4_fgp: {=bool:?}, ks4_frtn: {=bool:?}, ks4_fhwo: {=bool:?}, ks4_ukpuk: {=bool:?}, ks4_utecdh: {=bool:?}, ks4_ucmac: {=bool:?}, ks4_uksk: {=bool:?}, ks4_urtf: {=bool:?}, ks4_uckdf: {=bool:?}, ks4_uhkdf: {=bool:?}, ks4_uecsg: {=bool:?}, ks4_uecdh: {=bool:?}, ks4_uaes: {=bool:?}, ks4_uhmac: {=bool:?}, ks4_ukwk: {=bool:?}, ks4_ukuok: {=bool:?}, ks4_utlspms: {=bool:?}, ks4_utlsms: {=bool:?}, ks4_ukgsrc: {=bool:?}, ks4_uhwo: {=bool:?}, ks4_uwrpok: {=bool:?}, ks4_uduk: {=bool:?}, ks4_upprot: {=u8:?} }}",
            self.ks4_ksize(),
            self.ks4_kact(),
            self.ks4_kbase(),
            self.ks4_fgp(),
            self.ks4_frtn(),
            self.ks4_fhwo(),
            self.ks4_ukpuk(),
            self.ks4_utecdh(),
            self.ks4_ucmac(),
            self.ks4_uksk(),
            self.ks4_urtf(),
            self.ks4_uckdf(),
            self.ks4_uhkdf(),
            self.ks4_uecsg(),
            self.ks4_uecdh(),
            self.ks4_uaes(),
            self.ks4_uhmac(),
            self.ks4_ukwk(),
            self.ks4_ukuok(),
            self.ks4_utlspms(),
            self.ks4_utlsms(),
            self.ks4_ukgsrc(),
            self.ks4_uhwo(),
            self.ks4_uwrpok(),
            self.ks4_uduk(),
            self.ks4_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs5(pub u32);
impl ElsKs5 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ksize(&self) -> super::vals::Ks5Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks5Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks5_ksize(&mut self, val: super::vals::Ks5Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks5_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks5_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks5_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks5_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks5_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks5_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks5_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks5_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks5_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks5_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks5_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks5_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks5_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks5_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks5_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks5_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks5_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks5_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks5_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks5_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks5_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks5_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks5_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks5_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks5_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks5_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs5 {
    #[inline(always)]
    fn default() -> ElsKs5 {
        ElsKs5(0)
    }
}
impl core::fmt::Debug for ElsKs5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs5")
            .field("ks5_ksize", &self.ks5_ksize())
            .field("ks5_kact", &self.ks5_kact())
            .field("ks5_kbase", &self.ks5_kbase())
            .field("ks5_fgp", &self.ks5_fgp())
            .field("ks5_frtn", &self.ks5_frtn())
            .field("ks5_fhwo", &self.ks5_fhwo())
            .field("ks5_ukpuk", &self.ks5_ukpuk())
            .field("ks5_utecdh", &self.ks5_utecdh())
            .field("ks5_ucmac", &self.ks5_ucmac())
            .field("ks5_uksk", &self.ks5_uksk())
            .field("ks5_urtf", &self.ks5_urtf())
            .field("ks5_uckdf", &self.ks5_uckdf())
            .field("ks5_uhkdf", &self.ks5_uhkdf())
            .field("ks5_uecsg", &self.ks5_uecsg())
            .field("ks5_uecdh", &self.ks5_uecdh())
            .field("ks5_uaes", &self.ks5_uaes())
            .field("ks5_uhmac", &self.ks5_uhmac())
            .field("ks5_ukwk", &self.ks5_ukwk())
            .field("ks5_ukuok", &self.ks5_ukuok())
            .field("ks5_utlspms", &self.ks5_utlspms())
            .field("ks5_utlsms", &self.ks5_utlsms())
            .field("ks5_ukgsrc", &self.ks5_ukgsrc())
            .field("ks5_uhwo", &self.ks5_uhwo())
            .field("ks5_uwrpok", &self.ks5_uwrpok())
            .field("ks5_uduk", &self.ks5_uduk())
            .field("ks5_upprot", &self.ks5_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs5 {{ ks5_ksize: {:?}, ks5_kact: {=bool:?}, ks5_kbase: {=bool:?}, ks5_fgp: {=bool:?}, ks5_frtn: {=bool:?}, ks5_fhwo: {=bool:?}, ks5_ukpuk: {=bool:?}, ks5_utecdh: {=bool:?}, ks5_ucmac: {=bool:?}, ks5_uksk: {=bool:?}, ks5_urtf: {=bool:?}, ks5_uckdf: {=bool:?}, ks5_uhkdf: {=bool:?}, ks5_uecsg: {=bool:?}, ks5_uecdh: {=bool:?}, ks5_uaes: {=bool:?}, ks5_uhmac: {=bool:?}, ks5_ukwk: {=bool:?}, ks5_ukuok: {=bool:?}, ks5_utlspms: {=bool:?}, ks5_utlsms: {=bool:?}, ks5_ukgsrc: {=bool:?}, ks5_uhwo: {=bool:?}, ks5_uwrpok: {=bool:?}, ks5_uduk: {=bool:?}, ks5_upprot: {=u8:?} }}",
            self.ks5_ksize(),
            self.ks5_kact(),
            self.ks5_kbase(),
            self.ks5_fgp(),
            self.ks5_frtn(),
            self.ks5_fhwo(),
            self.ks5_ukpuk(),
            self.ks5_utecdh(),
            self.ks5_ucmac(),
            self.ks5_uksk(),
            self.ks5_urtf(),
            self.ks5_uckdf(),
            self.ks5_uhkdf(),
            self.ks5_uecsg(),
            self.ks5_uecdh(),
            self.ks5_uaes(),
            self.ks5_uhmac(),
            self.ks5_ukwk(),
            self.ks5_ukuok(),
            self.ks5_utlspms(),
            self.ks5_utlsms(),
            self.ks5_ukgsrc(),
            self.ks5_uhwo(),
            self.ks5_uwrpok(),
            self.ks5_uduk(),
            self.ks5_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs6(pub u32);
impl ElsKs6 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ksize(&self) -> super::vals::Ks6Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks6Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks6_ksize(&mut self, val: super::vals::Ks6Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks6_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks6_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks6_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks6_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks6_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks6_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks6_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks6_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks6_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks6_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks6_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks6_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks6_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks6_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks6_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks6_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks6_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks6_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks6_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks6_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks6_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks6_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks6_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks6_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks6_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks6_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs6 {
    #[inline(always)]
    fn default() -> ElsKs6 {
        ElsKs6(0)
    }
}
impl core::fmt::Debug for ElsKs6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs6")
            .field("ks6_ksize", &self.ks6_ksize())
            .field("ks6_kact", &self.ks6_kact())
            .field("ks6_kbase", &self.ks6_kbase())
            .field("ks6_fgp", &self.ks6_fgp())
            .field("ks6_frtn", &self.ks6_frtn())
            .field("ks6_fhwo", &self.ks6_fhwo())
            .field("ks6_ukpuk", &self.ks6_ukpuk())
            .field("ks6_utecdh", &self.ks6_utecdh())
            .field("ks6_ucmac", &self.ks6_ucmac())
            .field("ks6_uksk", &self.ks6_uksk())
            .field("ks6_urtf", &self.ks6_urtf())
            .field("ks6_uckdf", &self.ks6_uckdf())
            .field("ks6_uhkdf", &self.ks6_uhkdf())
            .field("ks6_uecsg", &self.ks6_uecsg())
            .field("ks6_uecdh", &self.ks6_uecdh())
            .field("ks6_uaes", &self.ks6_uaes())
            .field("ks6_uhmac", &self.ks6_uhmac())
            .field("ks6_ukwk", &self.ks6_ukwk())
            .field("ks6_ukuok", &self.ks6_ukuok())
            .field("ks6_utlspms", &self.ks6_utlspms())
            .field("ks6_utlsms", &self.ks6_utlsms())
            .field("ks6_ukgsrc", &self.ks6_ukgsrc())
            .field("ks6_uhwo", &self.ks6_uhwo())
            .field("ks6_uwrpok", &self.ks6_uwrpok())
            .field("ks6_uduk", &self.ks6_uduk())
            .field("ks6_upprot", &self.ks6_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs6 {{ ks6_ksize: {:?}, ks6_kact: {=bool:?}, ks6_kbase: {=bool:?}, ks6_fgp: {=bool:?}, ks6_frtn: {=bool:?}, ks6_fhwo: {=bool:?}, ks6_ukpuk: {=bool:?}, ks6_utecdh: {=bool:?}, ks6_ucmac: {=bool:?}, ks6_uksk: {=bool:?}, ks6_urtf: {=bool:?}, ks6_uckdf: {=bool:?}, ks6_uhkdf: {=bool:?}, ks6_uecsg: {=bool:?}, ks6_uecdh: {=bool:?}, ks6_uaes: {=bool:?}, ks6_uhmac: {=bool:?}, ks6_ukwk: {=bool:?}, ks6_ukuok: {=bool:?}, ks6_utlspms: {=bool:?}, ks6_utlsms: {=bool:?}, ks6_ukgsrc: {=bool:?}, ks6_uhwo: {=bool:?}, ks6_uwrpok: {=bool:?}, ks6_uduk: {=bool:?}, ks6_upprot: {=u8:?} }}",
            self.ks6_ksize(),
            self.ks6_kact(),
            self.ks6_kbase(),
            self.ks6_fgp(),
            self.ks6_frtn(),
            self.ks6_fhwo(),
            self.ks6_ukpuk(),
            self.ks6_utecdh(),
            self.ks6_ucmac(),
            self.ks6_uksk(),
            self.ks6_urtf(),
            self.ks6_uckdf(),
            self.ks6_uhkdf(),
            self.ks6_uecsg(),
            self.ks6_uecdh(),
            self.ks6_uaes(),
            self.ks6_uhmac(),
            self.ks6_ukwk(),
            self.ks6_ukuok(),
            self.ks6_utlspms(),
            self.ks6_utlsms(),
            self.ks6_ukgsrc(),
            self.ks6_uhwo(),
            self.ks6_uwrpok(),
            self.ks6_uduk(),
            self.ks6_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs7(pub u32);
impl ElsKs7 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ksize(&self) -> super::vals::Ks7Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks7Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks7_ksize(&mut self, val: super::vals::Ks7Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks7_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks7_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks7_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks7_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks7_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks7_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks7_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks7_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks7_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks7_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks7_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks7_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks7_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks7_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks7_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks7_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks7_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks7_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks7_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks7_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks7_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks7_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks7_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks7_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks7_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks7_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs7 {
    #[inline(always)]
    fn default() -> ElsKs7 {
        ElsKs7(0)
    }
}
impl core::fmt::Debug for ElsKs7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs7")
            .field("ks7_ksize", &self.ks7_ksize())
            .field("ks7_kact", &self.ks7_kact())
            .field("ks7_kbase", &self.ks7_kbase())
            .field("ks7_fgp", &self.ks7_fgp())
            .field("ks7_frtn", &self.ks7_frtn())
            .field("ks7_fhwo", &self.ks7_fhwo())
            .field("ks7_ukpuk", &self.ks7_ukpuk())
            .field("ks7_utecdh", &self.ks7_utecdh())
            .field("ks7_ucmac", &self.ks7_ucmac())
            .field("ks7_uksk", &self.ks7_uksk())
            .field("ks7_urtf", &self.ks7_urtf())
            .field("ks7_uckdf", &self.ks7_uckdf())
            .field("ks7_uhkdf", &self.ks7_uhkdf())
            .field("ks7_uecsg", &self.ks7_uecsg())
            .field("ks7_uecdh", &self.ks7_uecdh())
            .field("ks7_uaes", &self.ks7_uaes())
            .field("ks7_uhmac", &self.ks7_uhmac())
            .field("ks7_ukwk", &self.ks7_ukwk())
            .field("ks7_ukuok", &self.ks7_ukuok())
            .field("ks7_utlspms", &self.ks7_utlspms())
            .field("ks7_utlsms", &self.ks7_utlsms())
            .field("ks7_ukgsrc", &self.ks7_ukgsrc())
            .field("ks7_uhwo", &self.ks7_uhwo())
            .field("ks7_uwrpok", &self.ks7_uwrpok())
            .field("ks7_uduk", &self.ks7_uduk())
            .field("ks7_upprot", &self.ks7_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs7 {{ ks7_ksize: {:?}, ks7_kact: {=bool:?}, ks7_kbase: {=bool:?}, ks7_fgp: {=bool:?}, ks7_frtn: {=bool:?}, ks7_fhwo: {=bool:?}, ks7_ukpuk: {=bool:?}, ks7_utecdh: {=bool:?}, ks7_ucmac: {=bool:?}, ks7_uksk: {=bool:?}, ks7_urtf: {=bool:?}, ks7_uckdf: {=bool:?}, ks7_uhkdf: {=bool:?}, ks7_uecsg: {=bool:?}, ks7_uecdh: {=bool:?}, ks7_uaes: {=bool:?}, ks7_uhmac: {=bool:?}, ks7_ukwk: {=bool:?}, ks7_ukuok: {=bool:?}, ks7_utlspms: {=bool:?}, ks7_utlsms: {=bool:?}, ks7_ukgsrc: {=bool:?}, ks7_uhwo: {=bool:?}, ks7_uwrpok: {=bool:?}, ks7_uduk: {=bool:?}, ks7_upprot: {=u8:?} }}",
            self.ks7_ksize(),
            self.ks7_kact(),
            self.ks7_kbase(),
            self.ks7_fgp(),
            self.ks7_frtn(),
            self.ks7_fhwo(),
            self.ks7_ukpuk(),
            self.ks7_utecdh(),
            self.ks7_ucmac(),
            self.ks7_uksk(),
            self.ks7_urtf(),
            self.ks7_uckdf(),
            self.ks7_uhkdf(),
            self.ks7_uecsg(),
            self.ks7_uecdh(),
            self.ks7_uaes(),
            self.ks7_uhmac(),
            self.ks7_ukwk(),
            self.ks7_ukuok(),
            self.ks7_utlspms(),
            self.ks7_utlsms(),
            self.ks7_ukgsrc(),
            self.ks7_uhwo(),
            self.ks7_uwrpok(),
            self.ks7_uduk(),
            self.ks7_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs8(pub u32);
impl ElsKs8 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ksize(&self) -> super::vals::Ks8Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks8Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks8_ksize(&mut self, val: super::vals::Ks8Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks8_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks8_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks8_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks8_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks8_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks8_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks8_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks8_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks8_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks8_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks8_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks8_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks8_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks8_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks8_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks8_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks8_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks8_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks8_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks8_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks8_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks8_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks8_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks8_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks8_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks8_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs8 {
    #[inline(always)]
    fn default() -> ElsKs8 {
        ElsKs8(0)
    }
}
impl core::fmt::Debug for ElsKs8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs8")
            .field("ks8_ksize", &self.ks8_ksize())
            .field("ks8_kact", &self.ks8_kact())
            .field("ks8_kbase", &self.ks8_kbase())
            .field("ks8_fgp", &self.ks8_fgp())
            .field("ks8_frtn", &self.ks8_frtn())
            .field("ks8_fhwo", &self.ks8_fhwo())
            .field("ks8_ukpuk", &self.ks8_ukpuk())
            .field("ks8_utecdh", &self.ks8_utecdh())
            .field("ks8_ucmac", &self.ks8_ucmac())
            .field("ks8_uksk", &self.ks8_uksk())
            .field("ks8_urtf", &self.ks8_urtf())
            .field("ks8_uckdf", &self.ks8_uckdf())
            .field("ks8_uhkdf", &self.ks8_uhkdf())
            .field("ks8_uecsg", &self.ks8_uecsg())
            .field("ks8_uecdh", &self.ks8_uecdh())
            .field("ks8_uaes", &self.ks8_uaes())
            .field("ks8_uhmac", &self.ks8_uhmac())
            .field("ks8_ukwk", &self.ks8_ukwk())
            .field("ks8_ukuok", &self.ks8_ukuok())
            .field("ks8_utlspms", &self.ks8_utlspms())
            .field("ks8_utlsms", &self.ks8_utlsms())
            .field("ks8_ukgsrc", &self.ks8_ukgsrc())
            .field("ks8_uhwo", &self.ks8_uhwo())
            .field("ks8_uwrpok", &self.ks8_uwrpok())
            .field("ks8_uduk", &self.ks8_uduk())
            .field("ks8_upprot", &self.ks8_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs8 {{ ks8_ksize: {:?}, ks8_kact: {=bool:?}, ks8_kbase: {=bool:?}, ks8_fgp: {=bool:?}, ks8_frtn: {=bool:?}, ks8_fhwo: {=bool:?}, ks8_ukpuk: {=bool:?}, ks8_utecdh: {=bool:?}, ks8_ucmac: {=bool:?}, ks8_uksk: {=bool:?}, ks8_urtf: {=bool:?}, ks8_uckdf: {=bool:?}, ks8_uhkdf: {=bool:?}, ks8_uecsg: {=bool:?}, ks8_uecdh: {=bool:?}, ks8_uaes: {=bool:?}, ks8_uhmac: {=bool:?}, ks8_ukwk: {=bool:?}, ks8_ukuok: {=bool:?}, ks8_utlspms: {=bool:?}, ks8_utlsms: {=bool:?}, ks8_ukgsrc: {=bool:?}, ks8_uhwo: {=bool:?}, ks8_uwrpok: {=bool:?}, ks8_uduk: {=bool:?}, ks8_upprot: {=u8:?} }}",
            self.ks8_ksize(),
            self.ks8_kact(),
            self.ks8_kbase(),
            self.ks8_fgp(),
            self.ks8_frtn(),
            self.ks8_fhwo(),
            self.ks8_ukpuk(),
            self.ks8_utecdh(),
            self.ks8_ucmac(),
            self.ks8_uksk(),
            self.ks8_urtf(),
            self.ks8_uckdf(),
            self.ks8_uhkdf(),
            self.ks8_uecsg(),
            self.ks8_uecdh(),
            self.ks8_uaes(),
            self.ks8_uhmac(),
            self.ks8_ukwk(),
            self.ks8_ukuok(),
            self.ks8_utlspms(),
            self.ks8_utlsms(),
            self.ks8_ukgsrc(),
            self.ks8_uhwo(),
            self.ks8_uwrpok(),
            self.ks8_uduk(),
            self.ks8_upprot()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKs9(pub u32);
impl ElsKs9 {
    #[doc = "Key size"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ksize(&self) -> super::vals::Ks9Ksize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ks9Ksize::from_bits(val as u8)
    }
    #[doc = "Key size"]
    #[inline(always)]
    pub const fn set_ks9_ksize(&mut self, val: super::vals::Ks9Ksize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key is active"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_kact(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key is active"]
    #[inline(always)]
    pub const fn set_ks9_kact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "First slot in a multislot key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_kbase(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "First slot in a multislot key"]
    #[inline(always)]
    pub const fn set_ks9_kbase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Feature General Purpose"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_fgp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature General Purpose"]
    #[inline(always)]
    pub const fn set_ks9_fgp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Feature Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_frtn(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Retention"]
    #[inline(always)]
    pub const fn set_ks9_frtn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Feature Output"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_fhwo(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Feature Output"]
    #[inline(always)]
    pub const fn set_ks9_fhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ukpuk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks9_ukpuk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_utecdh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks9_utecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CMAC key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ucmac(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC key"]
    #[inline(always)]
    pub const fn set_ks9_ucmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "KSK key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uksk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "KSK key"]
    #[inline(always)]
    pub const fn set_ks9_uksk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Real Time Fingerprint key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_urtf(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Real Time Fingerprint key"]
    #[inline(always)]
    pub const fn set_ks9_urtf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Derivation key for CKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uckdf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for CKDF command"]
    #[inline(always)]
    pub const fn set_ks9_uckdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Derivation key for HKDF command"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uhkdf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Derivation key for HKDF command"]
    #[inline(always)]
    pub const fn set_ks9_uhkdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Ecc signing key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uecsg(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc signing key"]
    #[inline(always)]
    pub const fn set_ks9_uecsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Ecc diffie hellman key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uecdh(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Ecc diffie hellman key"]
    #[inline(always)]
    pub const fn set_ks9_uecdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Aes key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uaes(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Aes key"]
    #[inline(always)]
    pub const fn set_ks9_uaes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Hmac key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uhmac(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Hmac key"]
    #[inline(always)]
    pub const fn set_ks9_uhmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Key wrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ukwk(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Key wrapping key"]
    #[inline(always)]
    pub const fn set_ks9_ukwk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Key unwrapping key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ukuok(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Key unwrapping key"]
    #[inline(always)]
    pub const fn set_ks9_ukuok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "TLS Pre Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_utlspms(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Pre Master Secret"]
    #[inline(always)]
    pub const fn set_ks9_utlspms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "TLS Master Secret"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_utlsms(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "TLS Master Secret"]
    #[inline(always)]
    pub const fn set_ks9_utlsms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Supply KEYGEN source"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_ukgsrc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Supply KEYGEN source"]
    #[inline(always)]
    pub const fn set_ks9_ukgsrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Hardware out key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uhwo(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware out key"]
    #[inline(always)]
    pub const fn set_ks9_uhwo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Ok to wrap key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uwrpok(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ok to wrap key"]
    #[inline(always)]
    pub const fn set_ks9_uwrpok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device Unique Key"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_uduk(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device Unique Key"]
    #[inline(always)]
    pub const fn set_ks9_uduk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Priviledge level"]
    #[must_use]
    #[inline(always)]
    pub const fn ks9_upprot(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Priviledge level"]
    #[inline(always)]
    pub const fn set_ks9_upprot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for ElsKs9 {
    #[inline(always)]
    fn default() -> ElsKs9 {
        ElsKs9(0)
    }
}
impl core::fmt::Debug for ElsKs9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKs9")
            .field("ks9_ksize", &self.ks9_ksize())
            .field("ks9_kact", &self.ks9_kact())
            .field("ks9_kbase", &self.ks9_kbase())
            .field("ks9_fgp", &self.ks9_fgp())
            .field("ks9_frtn", &self.ks9_frtn())
            .field("ks9_fhwo", &self.ks9_fhwo())
            .field("ks9_ukpuk", &self.ks9_ukpuk())
            .field("ks9_utecdh", &self.ks9_utecdh())
            .field("ks9_ucmac", &self.ks9_ucmac())
            .field("ks9_uksk", &self.ks9_uksk())
            .field("ks9_urtf", &self.ks9_urtf())
            .field("ks9_uckdf", &self.ks9_uckdf())
            .field("ks9_uhkdf", &self.ks9_uhkdf())
            .field("ks9_uecsg", &self.ks9_uecsg())
            .field("ks9_uecdh", &self.ks9_uecdh())
            .field("ks9_uaes", &self.ks9_uaes())
            .field("ks9_uhmac", &self.ks9_uhmac())
            .field("ks9_ukwk", &self.ks9_ukwk())
            .field("ks9_ukuok", &self.ks9_ukuok())
            .field("ks9_utlspms", &self.ks9_utlspms())
            .field("ks9_utlsms", &self.ks9_utlsms())
            .field("ks9_ukgsrc", &self.ks9_ukgsrc())
            .field("ks9_uhwo", &self.ks9_uhwo())
            .field("ks9_uwrpok", &self.ks9_uwrpok())
            .field("ks9_uduk", &self.ks9_uduk())
            .field("ks9_upprot", &self.ks9_upprot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKs9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsKs9 {{ ks9_ksize: {:?}, ks9_kact: {=bool:?}, ks9_kbase: {=bool:?}, ks9_fgp: {=bool:?}, ks9_frtn: {=bool:?}, ks9_fhwo: {=bool:?}, ks9_ukpuk: {=bool:?}, ks9_utecdh: {=bool:?}, ks9_ucmac: {=bool:?}, ks9_uksk: {=bool:?}, ks9_urtf: {=bool:?}, ks9_uckdf: {=bool:?}, ks9_uhkdf: {=bool:?}, ks9_uecsg: {=bool:?}, ks9_uecdh: {=bool:?}, ks9_uaes: {=bool:?}, ks9_uhmac: {=bool:?}, ks9_ukwk: {=bool:?}, ks9_ukuok: {=bool:?}, ks9_utlspms: {=bool:?}, ks9_utlsms: {=bool:?}, ks9_ukgsrc: {=bool:?}, ks9_uhwo: {=bool:?}, ks9_uwrpok: {=bool:?}, ks9_uduk: {=bool:?}, ks9_upprot: {=u8:?} }}",
            self.ks9_ksize(),
            self.ks9_kact(),
            self.ks9_kbase(),
            self.ks9_fgp(),
            self.ks9_frtn(),
            self.ks9_fhwo(),
            self.ks9_ukpuk(),
            self.ks9_utecdh(),
            self.ks9_ucmac(),
            self.ks9_uksk(),
            self.ks9_urtf(),
            self.ks9_uckdf(),
            self.ks9_uhkdf(),
            self.ks9_uecsg(),
            self.ks9_uecdh(),
            self.ks9_uaes(),
            self.ks9_uhmac(),
            self.ks9_ukwk(),
            self.ks9_ukuok(),
            self.ks9_utlspms(),
            self.ks9_utlsms(),
            self.ks9_ukgsrc(),
            self.ks9_uhwo(),
            self.ks9_uwrpok(),
            self.ks9_uduk(),
            self.ks9_upprot()
        )
    }
}
#[doc = "Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrStatus(pub u32);
impl ErrStatus {
    #[doc = "Indicates public or private bus access error."]
    #[must_use]
    #[inline(always)]
    pub const fn bus_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates public or private bus access error."]
    #[inline(always)]
    pub const fn set_bus_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates operational error, that is, ELS has been incorrectly operated."]
    #[must_use]
    #[inline(always)]
    pub const fn opn_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates operational error, that is, ELS has been incorrectly operated."]
    #[inline(always)]
    pub const fn set_opn_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates algorithm error; an internal algorithm has produced an unexpected result."]
    #[must_use]
    #[inline(always)]
    pub const fn alg_err(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates algorithm error; an internal algorithm has produced an unexpected result."]
    #[inline(always)]
    pub const fn set_alg_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates data integrity error, that is, internal data integrity check has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn itg_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates data integrity error, that is, internal data integrity check has failed."]
    #[inline(always)]
    pub const fn set_itg_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates hardware fault error; an attempt to change the value of an internal register."]
    #[must_use]
    #[inline(always)]
    pub const fn flt_err(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates hardware fault error; an attempt to change the value of an internal register."]
    #[inline(always)]
    pub const fn set_flt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates user read of PRNG_DATOUT when STATUS\\[PRNG_RDY\\] is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn prng_err(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates user read of PRNG_DATOUT when STATUS\\[PRNG_RDY\\] is 0."]
    #[inline(always)]
    pub const fn set_prng_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the triggered error level: 0, 1 ,2."]
    #[must_use]
    #[inline(always)]
    pub const fn err_lvl(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates the triggered error level: 0, 1 ,2."]
    #[inline(always)]
    pub const fn set_err_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "TRNG unable to gather entropy with the current configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dtrng_err(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG unable to gather entropy with the current configuration"]
    #[inline(always)]
    pub const fn set_dtrng_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for ErrStatus {
    #[inline(always)]
    fn default() -> ErrStatus {
        ErrStatus(0)
    }
}
impl core::fmt::Debug for ErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrStatus")
            .field("bus_err", &self.bus_err())
            .field("opn_err", &self.opn_err())
            .field("alg_err", &self.alg_err())
            .field("itg_err", &self.itg_err())
            .field("flt_err", &self.flt_err())
            .field("prng_err", &self.prng_err())
            .field("err_lvl", &self.err_lvl())
            .field("dtrng_err", &self.dtrng_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ErrStatus {{ bus_err: {=bool:?}, opn_err: {=bool:?}, alg_err: {=bool:?}, itg_err: {=bool:?}, flt_err: {=bool:?}, prng_err: {=bool:?}, err_lvl: {=u8:?}, dtrng_err: {=bool:?} }}",
            self.bus_err(),
            self.opn_err(),
            self.alg_err(),
            self.itg_err(),
            self.flt_err(),
            self.prng_err(),
            self.err_lvl(),
            self.dtrng_err()
        )
    }
}
#[doc = "Error Status Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrStatusClr(pub u32);
impl ErrStatusClr {
    #[doc = "ELS error status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn err_clr(&self) -> super::vals::ErrClr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ErrClr::from_bits(val as u8)
    }
    #[doc = "ELS error status bit"]
    #[inline(always)]
    pub const fn set_err_clr(&mut self, val: super::vals::ErrClr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ErrStatusClr {
    #[inline(always)]
    fn default() -> ErrStatusClr {
        ErrStatusClr(0)
    }
}
impl core::fmt::Debug for ErrStatusClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErrStatusClr")
            .field("err_clr", &self.err_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErrStatusClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ErrStatusClr {{ err_clr: {:?} }}", self.err_clr())
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEnable(pub u32);
impl IntEnable {
    #[doc = "Enables or disables the operation of the ELS interrupt output port."]
    #[must_use]
    #[inline(always)]
    pub const fn int_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables or disables the operation of the ELS interrupt output port."]
    #[inline(always)]
    pub const fn set_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntEnable {
    #[inline(always)]
    fn default() -> IntEnable {
        IntEnable(0)
    }
}
impl core::fmt::Debug for IntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEnable")
            .field("int_en", &self.int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntEnable {{ int_en: {=bool:?} }}", self.int_en())
    }
}
#[doc = "Interrupt Status Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatusClr(pub u32);
impl IntStatusClr {
    #[doc = "Interrupt status clear bit"]
    #[must_use]
    #[inline(always)]
    pub const fn int_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status clear bit"]
    #[inline(always)]
    pub const fn set_int_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntStatusClr {
    #[inline(always)]
    fn default() -> IntStatusClr {
        IntStatusClr(0)
    }
}
impl core::fmt::Debug for IntStatusClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatusClr")
            .field("int_clr", &self.int_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatusClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntStatusClr {{ int_clr: {=bool:?} }}", self.int_clr())
    }
}
#[doc = "Interrupt Status Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatusSet(pub u32);
impl IntStatusSet {
    #[doc = "Software triggered interrupt bit"]
    #[must_use]
    #[inline(always)]
    pub const fn int_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software triggered interrupt bit"]
    #[inline(always)]
    pub const fn set_int_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntStatusSet {
    #[inline(always)]
    fn default() -> IntStatusSet {
        IntStatusSet(0)
    }
}
impl core::fmt::Debug for IntStatusSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatusSet")
            .field("int_set", &self.int_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatusSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntStatusSet {{ int_set: {=bool:?} }}", self.int_set())
    }
}
#[doc = "Keystore Index 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kidx0(pub u32);
impl Kidx0 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[must_use]
    #[inline(always)]
    pub const fn kidx0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[inline(always)]
    pub const fn set_kidx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Kidx0 {
    #[inline(always)]
    fn default() -> Kidx0 {
        Kidx0(0)
    }
}
impl core::fmt::Debug for Kidx0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kidx0")
            .field("kidx0", &self.kidx0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kidx0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kidx0 {{ kidx0: {=u8:?} }}", self.kidx0())
    }
}
#[doc = "Keystore Index 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kidx1(pub u32);
impl Kidx1 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[must_use]
    #[inline(always)]
    pub const fn kidx1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[inline(always)]
    pub const fn set_kidx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Kidx1 {
    #[inline(always)]
    fn default() -> Kidx1 {
        Kidx1(0)
    }
}
impl core::fmt::Debug for Kidx1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kidx1")
            .field("kidx1", &self.kidx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kidx1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kidx1 {{ kidx1: {=u8:?} }}", self.kidx1())
    }
}
#[doc = "Keystore Index 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kidx2(pub u32);
impl Kidx2 {
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[must_use]
    #[inline(always)]
    pub const fn kidx2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Selects the base 128-bit section of a key in ELS keystore"]
    #[inline(always)]
    pub const fn set_kidx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Kidx2 {
    #[inline(always)]
    fn default() -> Kidx2 {
        Kidx2(0)
    }
}
impl core::fmt::Debug for Kidx2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kidx2")
            .field("kidx2", &self.kidx2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kidx2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kidx2 {{ kidx2: {=u8:?} }}", self.kidx2())
    }
}
#[doc = "Key Properties Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpropin(pub u32);
impl Kpropin {
    #[doc = "Specifies requested properties of the key created by ELS command."]
    #[must_use]
    #[inline(always)]
    pub const fn kpropin(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Specifies requested properties of the key created by ELS command."]
    #[inline(always)]
    pub const fn set_kpropin(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Kpropin {
    #[inline(always)]
    fn default() -> Kpropin {
        Kpropin(0)
    }
}
impl core::fmt::Debug for Kpropin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpropin")
            .field("kpropin", &self.kpropin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpropin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Kpropin {{ kpropin: {=u32:?} }}", self.kpropin())
    }
}
#[doc = "Master ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MasterId(pub u32);
impl MasterId {
    #[doc = "Sets the privileged master ID."]
    #[must_use]
    #[inline(always)]
    pub const fn master_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Sets the privileged master ID."]
    #[inline(always)]
    pub const fn set_master_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for MasterId {
    #[inline(always)]
    fn default() -> MasterId {
        MasterId(0)
    }
}
impl core::fmt::Debug for MasterId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MasterId")
            .field("master_id", &self.master_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MasterId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MasterId {{ master_id: {=u8:?} }}", self.master_id())
    }
}
#[doc = "PRNG SW Read Out"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PrngDatout(pub u32);
impl PrngDatout {
    #[doc = "32-bit wide pseudo-random number"]
    #[must_use]
    #[inline(always)]
    pub const fn prng_datout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32-bit wide pseudo-random number"]
    #[inline(always)]
    pub const fn set_prng_datout(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PrngDatout {
    #[inline(always)]
    fn default() -> PrngDatout {
        PrngDatout(0)
    }
}
impl core::fmt::Debug for PrngDatout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PrngDatout")
            .field("prng_datout", &self.prng_datout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PrngDatout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PrngDatout {{ prng_datout: {=u32:?} }}",
            self.prng_datout()
        )
    }
}
#[doc = "Session ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SessionId(pub u32);
impl SessionId {
    #[doc = "Indicates the current value of the session ID."]
    #[must_use]
    #[inline(always)]
    pub const fn session_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the current value of the session ID."]
    #[inline(always)]
    pub const fn set_session_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SessionId {
    #[inline(always)]
    fn default() -> SessionId {
        SessionId(0)
    }
}
impl core::fmt::Debug for SessionId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SessionId")
            .field("session_id", &self.session_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SessionId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SessionId {{ session_id: {=u32:?} }}", self.session_id())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "When set, indicates the ELS is executing a crypto sequence"]
    #[must_use]
    #[inline(always)]
    pub const fn els_busy(&self) -> super::vals::ElsBusy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ElsBusy::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS is executing a crypto sequence"]
    #[inline(always)]
    pub const fn set_els_busy(&mut self, val: super::vals::ElsBusy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "When set, indicates the ELS has an active interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn els_irq(&self) -> super::vals::ElsIrq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ElsIrq::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS has an active interrupt"]
    #[inline(always)]
    pub const fn set_els_irq(&mut self, val: super::vals::ElsIrq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "When set, indicates the ELS has detected an internal error"]
    #[must_use]
    #[inline(always)]
    pub const fn els_err(&self) -> super::vals::ElsErr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ElsErr::from_bits(val as u8)
    }
    #[doc = "When set, indicates the ELS has detected an internal error"]
    #[inline(always)]
    pub const fn set_els_err(&mut self, val: super::vals::ElsErr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "When set, indicates the internal PRNG is ready"]
    #[must_use]
    #[inline(always)]
    pub const fn prng_rdy(&self) -> super::vals::PrngRdy {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PrngRdy::from_bits(val as u8)
    }
    #[doc = "When set, indicates the internal PRNG is ready"]
    #[inline(always)]
    pub const fn set_prng_rdy(&mut self, val: super::vals::PrngRdy) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Signature verify result status"]
    #[must_use]
    #[inline(always)]
    pub const fn ecdsa_vfy_status(&self) -> super::vals::EcdsaVfyStatus {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EcdsaVfyStatus::from_bits(val as u8)
    }
    #[doc = "Signature verify result status"]
    #[inline(always)]
    pub const fn set_ecdsa_vfy_status(&mut self, val: super::vals::EcdsaVfyStatus) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Current command privilege level"]
    #[must_use]
    #[inline(always)]
    pub const fn pprot(&self) -> super::vals::Pprot {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pprot::from_bits(val as u8)
    }
    #[doc = "Current command privilege level"]
    #[inline(always)]
    pub const fn set_pprot(&mut self, val: super::vals::Pprot) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Entropy quality of the current DRBG instance"]
    #[must_use]
    #[inline(always)]
    pub const fn drbg_ent_lvl(&self) -> super::vals::DrbgEntLvl {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DrbgEntLvl::from_bits(val as u8)
    }
    #[doc = "Entropy quality of the current DRBG instance"]
    #[inline(always)]
    pub const fn set_drbg_ent_lvl(&mut self, val: super::vals::DrbgEntLvl) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "When set, it indicates TRNG is gathering entropy"]
    #[must_use]
    #[inline(always)]
    pub const fn dtrng_busy(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When set, it indicates TRNG is gathering entropy"]
    #[inline(always)]
    pub const fn set_dtrng_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When set, indicates that ELS is locked by a master"]
    #[must_use]
    #[inline(always)]
    pub const fn els_locked(&self) -> super::vals::ElsLocked {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::ElsLocked::from_bits(val as u8)
    }
    #[doc = "When set, indicates that ELS is locked by a master"]
    #[inline(always)]
    pub const fn set_els_locked(&mut self, val: super::vals::ElsLocked) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("els_busy", &self.els_busy())
            .field("els_irq", &self.els_irq())
            .field("els_err", &self.els_err())
            .field("prng_rdy", &self.prng_rdy())
            .field("ecdsa_vfy_status", &self.ecdsa_vfy_status())
            .field("pprot", &self.pprot())
            .field("drbg_ent_lvl", &self.drbg_ent_lvl())
            .field("dtrng_busy", &self.dtrng_busy())
            .field("els_locked", &self.els_locked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ els_busy: {:?}, els_irq: {:?}, els_err: {:?}, prng_rdy: {:?}, ecdsa_vfy_status: {:?}, pprot: {:?}, drbg_ent_lvl: {:?}, dtrng_busy: {=bool:?}, els_locked: {:?} }}",
            self.els_busy(),
            self.els_irq(),
            self.els_err(),
            self.prng_rdy(),
            self.ecdsa_vfy_status(),
            self.pprot(),
            self.drbg_ent_lvl(),
            self.dtrng_busy(),
            self.els_locked()
        )
    }
}
#[doc = "Version Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Specifies the extended release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn z(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the extended release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_z(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Specifies the minor release version digit0; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn y2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the minor release version digit0; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_y2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Specifies the minor release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn y1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the minor release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_y1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Specifies the major release version; possible values are from 1-9."]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the major release version; possible values are from 1-9."]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Specifies the software extended revision version; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_z(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software extended revision version; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_sw_z(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Specifies the software minor release version digit0; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_y2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software minor release version digit0; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_sw_y2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Specifies the software minor release version digit1; possible values are from 0-9."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_y1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software minor release version digit1; possible values are from 0-9."]
    #[inline(always)]
    pub const fn set_sw_y1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Specifies the software major release version; possible values are from 1-9."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_x(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the software major release version; possible values are from 1-9."]
    #[inline(always)]
    pub const fn set_sw_x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("z", &self.z())
            .field("y2", &self.y2())
            .field("y1", &self.y1())
            .field("x", &self.x())
            .field("sw_z", &self.sw_z())
            .field("sw_y2", &self.sw_y2())
            .field("sw_y1", &self.sw_y1())
            .field("sw_x", &self.sw_x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ z: {=u8:?}, y2: {=u8:?}, y1: {=u8:?}, x: {=u8:?}, sw_z: {=u8:?}, sw_y2: {=u8:?}, sw_y1: {=u8:?}, sw_x: {=u8:?} }}",
            self.z(),
            self.y2(),
            self.y1(),
            self.x(),
            self.sw_z(),
            self.sw_y2(),
            self.sw_y1(),
            self.sw_x()
        )
    }
}
