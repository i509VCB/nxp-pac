#[doc = "Alpha Surface Buffer Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsBuf(pub u32);
impl AsBuf {
    #[doc = "Address pointer for the alpha surface 0 buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address pointer for the alpha surface 0 buffer."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AsBuf {
    #[inline(always)]
    fn default() -> AsBuf {
        AsBuf(0)
    }
}
impl core::fmt::Debug for AsBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsBuf").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsBuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AsBuf {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Overlay Color Key High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsClrkeyhigh(pub u32);
impl AsClrkeyhigh {
    #[doc = "High range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pixel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[inline(always)]
    pub const fn set_pixel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for AsClrkeyhigh {
    #[inline(always)]
    fn default() -> AsClrkeyhigh {
        AsClrkeyhigh(0)
    }
}
impl core::fmt::Debug for AsClrkeyhigh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsClrkeyhigh")
            .field("pixel", &self.pixel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsClrkeyhigh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AsClrkeyhigh {{ pixel: {=u32:?} }}", self.pixel())
    }
}
#[doc = "Overlay Color Key Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsClrkeylow(pub u32);
impl AsClrkeylow {
    #[doc = "Low range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pixel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Low range of RGB color key applied to AS buffer. Each overlay has an independent colorkey enable."]
    #[inline(always)]
    pub const fn set_pixel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for AsClrkeylow {
    #[inline(always)]
    fn default() -> AsClrkeylow {
        AsClrkeylow(0)
    }
}
impl core::fmt::Debug for AsClrkeylow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsClrkeylow")
            .field("pixel", &self.pixel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsClrkeylow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AsClrkeylow {{ pixel: {=u32:?} }}", self.pixel())
    }
}
#[doc = "Alpha Surface Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsCtrl(pub u32);
impl AsCtrl {
    #[doc = "Determines how the alpha value is constructed for this alpha surface"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_ctrl(&self) -> super::vals::AlphaCtrl {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::AlphaCtrl::from_bits(val as u8)
    }
    #[doc = "Determines how the alpha value is constructed for this alpha surface"]
    #[inline(always)]
    pub const fn set_alpha_ctrl(&mut self, val: super::vals::AlphaCtrl) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Indicates that colorkey functionality is enabled for this alpha surface"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_colorkey(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that colorkey functionality is enabled for this alpha surface"]
    #[inline(always)]
    pub const fn set_enable_colorkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the input buffer format for AS."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::AsCtrlFormat {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::AsCtrlFormat::from_bits(val as u8)
    }
    #[doc = "Indicates the input buffer format for AS."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::AsCtrlFormat) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Alpha modifier used when the ALPHA_MULTIPLY or ALPHA_OVERRIDE values are programmed in PXP_AS_CTRL\\[ALPHA_CTRL\\]"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates a raster operation to perform when enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rop(&self) -> super::vals::Rop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Rop::from_bits(val as u8)
    }
    #[doc = "Indicates a raster operation to perform when enabled"]
    #[inline(always)]
    pub const fn set_rop(&mut self, val: super::vals::Rop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Setting this bit to logic 0 will not alter the alpha value"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_invert(&self) -> super::vals::AlphaInvert {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::AlphaInvert::from_bits(val as u8)
    }
    #[doc = "Setting this bit to logic 0 will not alter the alpha value"]
    #[inline(always)]
    pub const fn set_alpha_invert(&mut self, val: super::vals::AlphaInvert) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for AsCtrl {
    #[inline(always)]
    fn default() -> AsCtrl {
        AsCtrl(0)
    }
}
impl core::fmt::Debug for AsCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsCtrl")
            .field("alpha_ctrl", &self.alpha_ctrl())
            .field("enable_colorkey", &self.enable_colorkey())
            .field("format", &self.format())
            .field("alpha", &self.alpha())
            .field("rop", &self.rop())
            .field("alpha_invert", &self.alpha_invert())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AsCtrl {{ alpha_ctrl: {:?}, enable_colorkey: {=bool:?}, format: {:?}, alpha: {=u8:?}, rop: {:?}, alpha_invert: {:?} }}",
            self.alpha_ctrl(),
            self.enable_colorkey(),
            self.format(),
            self.alpha(),
            self.rop(),
            self.alpha_invert()
        )
    }
}
#[doc = "Alpha Surface Pitch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AsPitch(pub u32);
impl AsPitch {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[must_use]
    #[inline(always)]
    pub const fn pitch(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub const fn set_pitch(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AsPitch {
    #[inline(always)]
    fn default() -> AsPitch {
        AsPitch(0)
    }
}
impl core::fmt::Debug for AsPitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AsPitch")
            .field("pitch", &self.pitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AsPitch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AsPitch {{ pitch: {=u16:?} }}", self.pitch())
    }
}
#[doc = "Color Space Conversion Coefficient Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csc1Coef0(pub u32);
impl Csc1Coef0 {
    #[doc = "Two's compliment amplitude offset implicit in the Y data"]
    #[must_use]
    #[inline(always)]
    pub const fn y_offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Two's compliment amplitude offset implicit in the Y data"]
    #[inline(always)]
    pub const fn set_y_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data"]
    #[must_use]
    #[inline(always)]
    pub const fn uv_offset(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[doc = "Two's compliment phase offset implicit for CbCr data"]
    #[inline(always)]
    pub const fn set_uv_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[must_use]
    #[inline(always)]
    pub const fn c0(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Two's compliment Y multiplier coefficient. YUV=0x100 (1.000) YCbCr=0x12A (1.164)"]
    #[inline(always)]
    pub const fn set_c0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Bypass the CSC unit in the scaling engine"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the CSC unit in the scaling engine"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Set to 1 when performing YCbCr conversion to RGB"]
    #[must_use]
    #[inline(always)]
    pub const fn ycbcr_mode(&self) -> super::vals::YcbcrMode {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::YcbcrMode::from_bits(val as u8)
    }
    #[doc = "Set to 1 when performing YCbCr conversion to RGB"]
    #[inline(always)]
    pub const fn set_ycbcr_mode(&mut self, val: super::vals::YcbcrMode) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Csc1Coef0 {
    #[inline(always)]
    fn default() -> Csc1Coef0 {
        Csc1Coef0(0)
    }
}
impl core::fmt::Debug for Csc1Coef0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csc1Coef0")
            .field("y_offset", &self.y_offset())
            .field("uv_offset", &self.uv_offset())
            .field("c0", &self.c0())
            .field("bypass", &self.bypass())
            .field("ycbcr_mode", &self.ycbcr_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csc1Coef0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csc1Coef0 {{ y_offset: {=u16:?}, uv_offset: {=u16:?}, c0: {=u16:?}, bypass: {=bool:?}, ycbcr_mode: {:?} }}",
            self.y_offset(),
            self.uv_offset(),
            self.c0(),
            self.bypass(),
            self.ycbcr_mode()
        )
    }
}
#[doc = "Color Space Conversion Coefficient Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csc1Coef1(pub u32);
impl Csc1Coef1 {
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    #[must_use]
    #[inline(always)]
    pub const fn c4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Two's compliment Blue U/Cb multiplier coefficient. YUV=0x208 (2.032) YCbCr=0x204 (2.017)"]
    #[inline(always)]
    pub const fn set_c4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    #[must_use]
    #[inline(always)]
    pub const fn c1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Two's compliment Red V/Cr multiplier coefficient. YUV=0x123 (1.140) YCbCr=0x198 (1.596)"]
    #[inline(always)]
    pub const fn set_c1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Csc1Coef1 {
    #[inline(always)]
    fn default() -> Csc1Coef1 {
        Csc1Coef1(0)
    }
}
impl core::fmt::Debug for Csc1Coef1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csc1Coef1")
            .field("c4", &self.c4())
            .field("c1", &self.c1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csc1Coef1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csc1Coef1 {{ c4: {=u16:?}, c1: {=u16:?} }}",
            self.c4(),
            self.c1()
        )
    }
}
#[doc = "Color Space Conversion Coefficient Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csc1Coef2(pub u32);
impl Csc1Coef2 {
    #[doc = "Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    #[must_use]
    #[inline(always)]
    pub const fn c3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Two's complement Green U/Cb multiplier coefficient. YUV=0x79C (-0.394) YCbCr=0x79C (-0.392)"]
    #[inline(always)]
    pub const fn set_c3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    #[must_use]
    #[inline(always)]
    pub const fn c2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Two's complement Green V/Cr multiplier coefficient. YUV=0x76B (-0.581) YCbCr=0x730 (-0.813)"]
    #[inline(always)]
    pub const fn set_c2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Csc1Coef2 {
    #[inline(always)]
    fn default() -> Csc1Coef2 {
        Csc1Coef2(0)
    }
}
impl core::fmt::Debug for Csc1Coef2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csc1Coef2")
            .field("c3", &self.c3())
            .field("c2", &self.c2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csc1Coef2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Csc1Coef2 {{ c3: {=u16:?}, c2: {=u16:?} }}",
            self.c3(),
            self.c2()
        )
    }
}
#[doc = "Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enables PXP operation with specified parameters"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[inline(always)]
    pub const fn set_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Next command interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Next command interrupt enable"]
    #[inline(always)]
    pub const fn set_next_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable handshake with LCD controller"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_lcd_handshake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable handshake with LCD controller"]
    #[inline(always)]
    pub const fn set_enable_lcd_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rotate(&self) -> super::vals::CtrlRotate {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlRotate::from_bits(val as u8)
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub const fn set_rotate(&mut self, val: super::vals::CtrlRotate) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn hflip(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_hflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn vflip(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_vflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[must_use]
    #[inline(always)]
    pub const fn rot_pos(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub const fn set_rot_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Select the block size to process."]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> super::vals::CtrlBlockSize {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CtrlBlockSize::from_bits(val as u8)
    }
    #[doc = "Select the block size to process."]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: super::vals::CtrlBlockSize) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable the PXP to run continuously"]
    #[must_use]
    #[inline(always)]
    pub const fn en_repeat(&self) -> super::vals::CtrlEnRepeat {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CtrlEnRepeat::from_bits(val as u8)
    }
    #[doc = "Enable the PXP to run continuously"]
    #[inline(always)]
    pub const fn set_en_repeat(&mut self, val: super::vals::CtrlEnRepeat) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> super::vals::CtrlClkgate {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CtrlClkgate::from_bits(val as u8)
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: super::vals::CtrlClkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> super::vals::CtrlSftrst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlSftrst::from_bits(val as u8)
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: super::vals::CtrlSftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("irq_enable", &self.irq_enable())
            .field("next_irq_enable", &self.next_irq_enable())
            .field("enable_lcd_handshake", &self.enable_lcd_handshake())
            .field("rotate", &self.rotate())
            .field("hflip", &self.hflip())
            .field("vflip", &self.vflip())
            .field("rot_pos", &self.rot_pos())
            .field("block_size", &self.block_size())
            .field("en_repeat", &self.en_repeat())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enable: {=bool:?}, irq_enable: {=bool:?}, next_irq_enable: {=bool:?}, enable_lcd_handshake: {=bool:?}, rotate: {:?}, hflip: {=bool:?}, vflip: {=bool:?}, rot_pos: {=bool:?}, block_size: {:?}, en_repeat: {:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enable(),
            self.irq_enable(),
            self.next_irq_enable(),
            self.enable_lcd_handshake(),
            self.rotate(),
            self.hflip(),
            self.vflip(),
            self.rot_pos(),
            self.block_size(),
            self.en_repeat(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "Enables PXP operation with specified parameters"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[inline(always)]
    pub const fn set_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Next command interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Next command interrupt enable"]
    #[inline(always)]
    pub const fn set_next_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable handshake with LCD controller"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_lcd_handshake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable handshake with LCD controller"]
    #[inline(always)]
    pub const fn set_enable_lcd_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rotate(&self) -> super::vals::CtrlClrRotate {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlClrRotate::from_bits(val as u8)
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub const fn set_rotate(&mut self, val: super::vals::CtrlClrRotate) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn hflip(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_hflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn vflip(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_vflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[must_use]
    #[inline(always)]
    pub const fn rot_pos(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub const fn set_rot_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Select the block size to process."]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> super::vals::CtrlClrBlockSize {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CtrlClrBlockSize::from_bits(val as u8)
    }
    #[doc = "Select the block size to process."]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: super::vals::CtrlClrBlockSize) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable the PXP to run continuously"]
    #[must_use]
    #[inline(always)]
    pub const fn en_repeat(&self) -> super::vals::CtrlClrEnRepeat {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CtrlClrEnRepeat::from_bits(val as u8)
    }
    #[doc = "Enable the PXP to run continuously"]
    #[inline(always)]
    pub const fn set_en_repeat(&mut self, val: super::vals::CtrlClrEnRepeat) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> super::vals::CtrlClrClkgate {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CtrlClrClkgate::from_bits(val as u8)
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: super::vals::CtrlClrClkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> super::vals::CtrlClrSftrst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlClrSftrst::from_bits(val as u8)
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: super::vals::CtrlClrSftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("enable", &self.enable())
            .field("irq_enable", &self.irq_enable())
            .field("next_irq_enable", &self.next_irq_enable())
            .field("enable_lcd_handshake", &self.enable_lcd_handshake())
            .field("rotate", &self.rotate())
            .field("hflip", &self.hflip())
            .field("vflip", &self.vflip())
            .field("rot_pos", &self.rot_pos())
            .field("block_size", &self.block_size())
            .field("en_repeat", &self.en_repeat())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlClr {{ enable: {=bool:?}, irq_enable: {=bool:?}, next_irq_enable: {=bool:?}, enable_lcd_handshake: {=bool:?}, rotate: {:?}, hflip: {=bool:?}, vflip: {=bool:?}, rot_pos: {=bool:?}, block_size: {:?}, en_repeat: {:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enable(),
            self.irq_enable(),
            self.next_irq_enable(),
            self.enable_lcd_handshake(),
            self.rotate(),
            self.hflip(),
            self.vflip(),
            self.rot_pos(),
            self.block_size(),
            self.en_repeat(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "Enables PXP operation with specified parameters"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[inline(always)]
    pub const fn set_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Next command interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Next command interrupt enable"]
    #[inline(always)]
    pub const fn set_next_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable handshake with LCD controller"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_lcd_handshake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable handshake with LCD controller"]
    #[inline(always)]
    pub const fn set_enable_lcd_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rotate(&self) -> super::vals::CtrlSetRotate {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlSetRotate::from_bits(val as u8)
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub const fn set_rotate(&mut self, val: super::vals::CtrlSetRotate) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn hflip(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_hflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn vflip(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_vflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[must_use]
    #[inline(always)]
    pub const fn rot_pos(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub const fn set_rot_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Select the block size to process."]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> super::vals::CtrlSetBlockSize {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CtrlSetBlockSize::from_bits(val as u8)
    }
    #[doc = "Select the block size to process."]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: super::vals::CtrlSetBlockSize) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable the PXP to run continuously"]
    #[must_use]
    #[inline(always)]
    pub const fn en_repeat(&self) -> super::vals::CtrlSetEnRepeat {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CtrlSetEnRepeat::from_bits(val as u8)
    }
    #[doc = "Enable the PXP to run continuously"]
    #[inline(always)]
    pub const fn set_en_repeat(&mut self, val: super::vals::CtrlSetEnRepeat) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> super::vals::CtrlSetClkgate {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CtrlSetClkgate::from_bits(val as u8)
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: super::vals::CtrlSetClkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> super::vals::CtrlSetSftrst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlSetSftrst::from_bits(val as u8)
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: super::vals::CtrlSetSftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("enable", &self.enable())
            .field("irq_enable", &self.irq_enable())
            .field("next_irq_enable", &self.next_irq_enable())
            .field("enable_lcd_handshake", &self.enable_lcd_handshake())
            .field("rotate", &self.rotate())
            .field("hflip", &self.hflip())
            .field("vflip", &self.vflip())
            .field("rot_pos", &self.rot_pos())
            .field("block_size", &self.block_size())
            .field("en_repeat", &self.en_repeat())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlSet {{ enable: {=bool:?}, irq_enable: {=bool:?}, next_irq_enable: {=bool:?}, enable_lcd_handshake: {=bool:?}, rotate: {:?}, hflip: {=bool:?}, vflip: {=bool:?}, rot_pos: {=bool:?}, block_size: {:?}, en_repeat: {:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enable(),
            self.irq_enable(),
            self.next_irq_enable(),
            self.enable_lcd_handshake(),
            self.rotate(),
            self.hflip(),
            self.vflip(),
            self.rot_pos(),
            self.block_size(),
            self.en_repeat(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "Enables PXP operation with specified parameters"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PXP operation with specified parameters"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable When using the PXP_NEXT functionality to reprogram the PXP, the new value of this bit will be used and may therefore enable or disable an interrupt unintentionally"]
    #[inline(always)]
    pub const fn set_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Next command interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Next command interrupt enable"]
    #[inline(always)]
    pub const fn set_next_irq_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable handshake with LCD controller"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_lcd_handshake(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable handshake with LCD controller"]
    #[inline(always)]
    pub const fn set_enable_lcd_handshake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn rotate(&self) -> super::vals::CtrlTogRotate {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CtrlTogRotate::from_bits(val as u8)
    }
    #[doc = "Indicates the clockwise rotation to be applied at the output buffer"]
    #[inline(always)]
    pub const fn set_rotate(&mut self, val: super::vals::CtrlTogRotate) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn hflip(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped horizontally (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_hflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[must_use]
    #[inline(always)]
    pub const fn vflip(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the output buffer should be flipped vertically (effect applied before rotation)."]
    #[inline(always)]
    pub const fn set_vflip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[must_use]
    #[inline(always)]
    pub const fn rot_pos(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls where rotation will occur in the PXP datapath"]
    #[inline(always)]
    pub const fn set_rot_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Select the block size to process."]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> super::vals::CtrlTogBlockSize {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CtrlTogBlockSize::from_bits(val as u8)
    }
    #[doc = "Select the block size to process."]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: super::vals::CtrlTogBlockSize) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable the PXP to run continuously"]
    #[must_use]
    #[inline(always)]
    pub const fn en_repeat(&self) -> super::vals::CtrlTogEnRepeat {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CtrlTogEnRepeat::from_bits(val as u8)
    }
    #[doc = "Enable the PXP to run continuously"]
    #[inline(always)]
    pub const fn set_en_repeat(&mut self, val: super::vals::CtrlTogEnRepeat) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> super::vals::CtrlTogClkgate {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::CtrlTogClkgate::from_bits(val as u8)
    }
    #[doc = "This bit must be set to zero for normal operation"]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: super::vals::CtrlTogClkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> super::vals::CtrlTogSftrst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CtrlTogSftrst::from_bits(val as u8)
    }
    #[doc = "This bit can be turned on and then off to reset the PXP block to its default state."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: super::vals::CtrlTogSftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("enable", &self.enable())
            .field("irq_enable", &self.irq_enable())
            .field("next_irq_enable", &self.next_irq_enable())
            .field("enable_lcd_handshake", &self.enable_lcd_handshake())
            .field("rotate", &self.rotate())
            .field("hflip", &self.hflip())
            .field("vflip", &self.vflip())
            .field("rot_pos", &self.rot_pos())
            .field("block_size", &self.block_size())
            .field("en_repeat", &self.en_repeat())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlTog {{ enable: {=bool:?}, irq_enable: {=bool:?}, next_irq_enable: {=bool:?}, enable_lcd_handshake: {=bool:?}, rotate: {:?}, hflip: {=bool:?}, vflip: {=bool:?}, rot_pos: {=bool:?}, block_size: {:?}, en_repeat: {:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enable(),
            self.irq_enable(),
            self.next_irq_enable(),
            self.enable_lcd_handshake(),
            self.rotate(),
            self.hflip(),
            self.vflip(),
            self.rot_pos(),
            self.block_size(),
            self.en_repeat(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Next Frame Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Next(pub u32);
impl Next {
    #[doc = "Indicates that the \"next frame\" functionality has been enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that the \"next frame\" functionality has been enabled"]
    #[inline(always)]
    pub const fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "A pointer to a data structure containing register values to be used when processing the next frame"]
    #[must_use]
    #[inline(always)]
    pub const fn pointer(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "A pointer to a data structure containing register values to be used when processing the next frame"]
    #[inline(always)]
    pub const fn set_pointer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for Next {
    #[inline(always)]
    fn default() -> Next {
        Next(0)
    }
}
impl core::fmt::Debug for Next {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Next")
            .field("enabled", &self.enabled())
            .field("pointer", &self.pointer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Next {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Next {{ enabled: {=bool:?}, pointer: {=u32:?} }}",
            self.enabled(),
            self.pointer()
        )
    }
}
#[doc = "Alpha Surface Lower Right Coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutAsLrc(pub u32);
impl OutAsLrc {
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    #[inline(always)]
    pub const fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for OutAsLrc {
    #[inline(always)]
    fn default() -> OutAsLrc {
        OutAsLrc(0)
    }
}
impl core::fmt::Debug for OutAsLrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutAsLrc")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutAsLrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutAsLrc {{ y: {=u16:?}, x: {=u16:?} }}",
            self.y(),
            self.x()
        )
    }
}
#[doc = "Alpha Surface Upper Left Coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutAsUlc(pub u32);
impl OutAsUlc {
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the alpha surface in the output frame buffer"]
    #[inline(always)]
    pub const fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the alpha surface (AS) in the output frame buffer"]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for OutAsUlc {
    #[inline(always)]
    fn default() -> OutAsUlc {
        OutAsUlc(0)
    }
}
impl core::fmt::Debug for OutAsUlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutAsUlc")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutAsUlc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutAsUlc {{ y: {=u16:?}, x: {=u16:?} }}",
            self.y(),
            self.x()
        )
    }
}
#[doc = "Output Frame Buffer Pointer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBuf(pub u32);
impl OutBuf {
    #[doc = "Current address pointer for the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address pointer for the output frame buffer"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OutBuf {
    #[inline(always)]
    fn default() -> OutBuf {
        OutBuf(0)
    }
}
impl core::fmt::Debug for OutBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutBuf")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutBuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutBuf {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Output Frame Buffer Pointer #2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBuf2(pub u32);
impl OutBuf2 {
    #[doc = "Current address pointer for the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Current address pointer for the output frame buffer"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OutBuf2 {
    #[inline(always)]
    fn default() -> OutBuf2 {
        OutBuf2(0)
    }
}
impl core::fmt::Debug for OutBuf2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutBuf2")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutBuf2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutBuf2 {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Output Buffer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutCtrl(pub u32);
impl OutCtrl {
    #[doc = "Output framebuffer format"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::OutCtrlFormat {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OutCtrlFormat::from_bits(val as u8)
    }
    #[doc = "Output framebuffer format"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::OutCtrlFormat) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[must_use]
    #[inline(always)]
    pub const fn interlaced_output(&self) -> super::vals::OutCtrlInterlacedOutput {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::OutCtrlInterlacedOutput::from_bits(val as u8)
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub const fn set_interlaced_output(&mut self, val: super::vals::OutCtrlInterlacedOutput) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_output(&self) -> super::vals::OutCtrlAlphaOutput {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutCtrlAlphaOutput::from_bits(val as u8)
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub const fn set_alpha_output(&mut self, val: super::vals::OutCtrlAlphaOutput) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OutCtrl {
    #[inline(always)]
    fn default() -> OutCtrl {
        OutCtrl(0)
    }
}
impl core::fmt::Debug for OutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutCtrl")
            .field("format", &self.format())
            .field("interlaced_output", &self.interlaced_output())
            .field("alpha_output", &self.alpha_output())
            .field("alpha", &self.alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutCtrl {{ format: {:?}, interlaced_output: {:?}, alpha_output: {:?}, alpha: {=u8:?} }}",
            self.format(),
            self.interlaced_output(),
            self.alpha_output(),
            self.alpha()
        )
    }
}
#[doc = "Output Buffer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutCtrlClr(pub u32);
impl OutCtrlClr {
    #[doc = "Output framebuffer format"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::OutCtrlClrFormat {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OutCtrlClrFormat::from_bits(val as u8)
    }
    #[doc = "Output framebuffer format"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::OutCtrlClrFormat) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[must_use]
    #[inline(always)]
    pub const fn interlaced_output(&self) -> super::vals::OutCtrlClrInterlacedOutput {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::OutCtrlClrInterlacedOutput::from_bits(val as u8)
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub const fn set_interlaced_output(&mut self, val: super::vals::OutCtrlClrInterlacedOutput) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_output(&self) -> super::vals::OutCtrlClrAlphaOutput {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutCtrlClrAlphaOutput::from_bits(val as u8)
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub const fn set_alpha_output(&mut self, val: super::vals::OutCtrlClrAlphaOutput) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OutCtrlClr {
    #[inline(always)]
    fn default() -> OutCtrlClr {
        OutCtrlClr(0)
    }
}
impl core::fmt::Debug for OutCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutCtrlClr")
            .field("format", &self.format())
            .field("interlaced_output", &self.interlaced_output())
            .field("alpha_output", &self.alpha_output())
            .field("alpha", &self.alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutCtrlClr {{ format: {:?}, interlaced_output: {:?}, alpha_output: {:?}, alpha: {=u8:?} }}",
            self.format(),
            self.interlaced_output(),
            self.alpha_output(),
            self.alpha()
        )
    }
}
#[doc = "Output Buffer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutCtrlSet(pub u32);
impl OutCtrlSet {
    #[doc = "Output framebuffer format"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::OutCtrlSetFormat {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OutCtrlSetFormat::from_bits(val as u8)
    }
    #[doc = "Output framebuffer format"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::OutCtrlSetFormat) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[must_use]
    #[inline(always)]
    pub const fn interlaced_output(&self) -> super::vals::OutCtrlSetInterlacedOutput {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::OutCtrlSetInterlacedOutput::from_bits(val as u8)
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub const fn set_interlaced_output(&mut self, val: super::vals::OutCtrlSetInterlacedOutput) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_output(&self) -> super::vals::OutCtrlSetAlphaOutput {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutCtrlSetAlphaOutput::from_bits(val as u8)
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub const fn set_alpha_output(&mut self, val: super::vals::OutCtrlSetAlphaOutput) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OutCtrlSet {
    #[inline(always)]
    fn default() -> OutCtrlSet {
        OutCtrlSet(0)
    }
}
impl core::fmt::Debug for OutCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutCtrlSet")
            .field("format", &self.format())
            .field("interlaced_output", &self.interlaced_output())
            .field("alpha_output", &self.alpha_output())
            .field("alpha", &self.alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutCtrlSet {{ format: {:?}, interlaced_output: {:?}, alpha_output: {:?}, alpha: {=u8:?} }}",
            self.format(),
            self.interlaced_output(),
            self.alpha_output(),
            self.alpha()
        )
    }
}
#[doc = "Output Buffer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutCtrlTog(pub u32);
impl OutCtrlTog {
    #[doc = "Output framebuffer format"]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::OutCtrlTogFormat {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::OutCtrlTogFormat::from_bits(val as u8)
    }
    #[doc = "Output framebuffer format"]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::OutCtrlTogFormat) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[must_use]
    #[inline(always)]
    pub const fn interlaced_output(&self) -> super::vals::OutCtrlTogInterlacedOutput {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::OutCtrlTogInterlacedOutput::from_bits(val as u8)
    }
    #[doc = "Determines how the PXP writes it's output data"]
    #[inline(always)]
    pub const fn set_interlaced_output(&mut self, val: super::vals::OutCtrlTogInterlacedOutput) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha_output(&self) -> super::vals::OutCtrlTogAlphaOutput {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::OutCtrlTogAlphaOutput::from_bits(val as u8)
    }
    #[doc = "Indicates that alpha component in output buffer pixels should be overwritten by PXP_OUT_CTRL\\[ALPHA\\]"]
    #[inline(always)]
    pub const fn set_alpha_output(&mut self, val: super::vals::OutCtrlTogAlphaOutput) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[must_use]
    #[inline(always)]
    pub const fn alpha(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "When generating an output buffer with an alpha component, the value in this field will be used when enabled to override the alpha passed through the pixel data pipeline"]
    #[inline(always)]
    pub const fn set_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OutCtrlTog {
    #[inline(always)]
    fn default() -> OutCtrlTog {
        OutCtrlTog(0)
    }
}
impl core::fmt::Debug for OutCtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutCtrlTog")
            .field("format", &self.format())
            .field("interlaced_output", &self.interlaced_output())
            .field("alpha_output", &self.alpha_output())
            .field("alpha", &self.alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutCtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutCtrlTog {{ format: {:?}, interlaced_output: {:?}, alpha_output: {:?}, alpha: {=u8:?} }}",
            self.format(),
            self.interlaced_output(),
            self.alpha_output(),
            self.alpha()
        )
    }
}
#[doc = "Output Surface Lower Right Coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutLrc(pub u32);
impl OutLrc {
    #[doc = "Indicates the number of vertical PIXELS in the output surface (non-rotated)"]
    #[must_use]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Indicates the number of vertical PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub const fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Indicates number of horizontal PIXELS in the output surface (non-rotated)"]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Indicates number of horizontal PIXELS in the output surface (non-rotated)"]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for OutLrc {
    #[inline(always)]
    fn default() -> OutLrc {
        OutLrc(0)
    }
}
impl core::fmt::Debug for OutLrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutLrc")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutLrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutLrc {{ y: {=u16:?}, x: {=u16:?} }}",
            self.y(),
            self.x()
        )
    }
}
#[doc = "Output Buffer Pitch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPitch(pub u32);
impl OutPitch {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[must_use]
    #[inline(always)]
    pub const fn pitch(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub const fn set_pitch(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OutPitch {
    #[inline(always)]
    fn default() -> OutPitch {
        OutPitch(0)
    }
}
impl core::fmt::Debug for OutPitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutPitch")
            .field("pitch", &self.pitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutPitch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutPitch {{ pitch: {=u16:?} }}", self.pitch())
    }
}
#[doc = "Processed Surface Lower Right Coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPsLrc(pub u32);
impl OutPsLrc {
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the lower right Y-coordinate (in pixels) of the processed surface in the output frame buffer"]
    #[inline(always)]
    pub const fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface (PS) in the output frame buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the lower right X-coordinate (in pixels) of the processed surface (PS) in the output frame buffer"]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for OutPsLrc {
    #[inline(always)]
    fn default() -> OutPsLrc {
        OutPsLrc(0)
    }
}
impl core::fmt::Debug for OutPsLrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutPsLrc")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutPsLrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutPsLrc {{ y: {=u16:?}, x: {=u16:?} }}",
            self.y(),
            self.x()
        )
    }
}
#[doc = "Processed Surface Upper Left Coordinate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPsUlc(pub u32);
impl OutPsUlc {
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the upper left Y-coordinate (in pixels) of the processed surface in the output buffer"]
    #[inline(always)]
    pub const fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface (PS) in the output buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "This field indicates the upper left X-coordinate (in pixels) of the processed surface (PS) in the output buffer"]
    #[inline(always)]
    pub const fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for OutPsUlc {
    #[inline(always)]
    fn default() -> OutPsUlc {
        OutPsUlc(0)
    }
}
impl core::fmt::Debug for OutPsUlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutPsUlc")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutPsUlc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutPsUlc {{ y: {=u16:?}, x: {=u16:?} }}",
            self.y(),
            self.x()
        )
    }
}
#[doc = "PXP Alpha Engine A Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PorterDuffCtrl(pub u32);
impl PorterDuffCtrl {
    #[doc = "Porter-Duff Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn porter_duff_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Porter-Duff Enable"]
    #[inline(always)]
    pub const fn set_porter_duff_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "s0 to s1 factor mode"]
    #[must_use]
    #[inline(always)]
    pub const fn s0_s1_factor_mode(&self) -> super::vals::S0S1FactorMode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::S0S1FactorMode::from_bits(val as u8)
    }
    #[doc = "s0 to s1 factor mode"]
    #[inline(always)]
    pub const fn set_s0_s1_factor_mode(&mut self, val: super::vals::S0S1FactorMode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "s0 global alpha mode"]
    #[must_use]
    #[inline(always)]
    pub const fn s0_global_alpha_mode(&self) -> super::vals::S0GlobalAlphaMode {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::S0GlobalAlphaMode::from_bits(val as u8)
    }
    #[doc = "s0 global alpha mode"]
    #[inline(always)]
    pub const fn set_s0_global_alpha_mode(&mut self, val: super::vals::S0GlobalAlphaMode) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "s0 alpha mode (Porter-Duff alpha mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn s0_alpha_mode(&self) -> super::vals::S0AlphaMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::S0AlphaMode::from_bits(val as u8)
    }
    #[doc = "s0 alpha mode (Porter-Duff alpha mode)"]
    #[inline(always)]
    pub const fn set_s0_alpha_mode(&mut self, val: super::vals::S0AlphaMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "s0 color mode (Porter-Duff color mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn s0_color_mode(&self) -> super::vals::S0ColorMode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::S0ColorMode::from_bits(val as u8)
    }
    #[doc = "s0 color mode (Porter-Duff color mode)"]
    #[inline(always)]
    pub const fn set_s0_color_mode(&mut self, val: super::vals::S0ColorMode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "s1 to s0 factor mode (Porter-Duff factor mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn s1_s0_factor_mode(&self) -> super::vals::S1S0FactorMode {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::S1S0FactorMode::from_bits(val as u8)
    }
    #[doc = "s1 to s0 factor mode (Porter-Duff factor mode)"]
    #[inline(always)]
    pub const fn set_s1_s0_factor_mode(&mut self, val: super::vals::S1S0FactorMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "s1 global alpha mode (Porter-Duff Global Alpha mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn s1_global_alpha_mode(&self) -> super::vals::S1GlobalAlphaMode {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::S1GlobalAlphaMode::from_bits(val as u8)
    }
    #[doc = "s1 global alpha mode (Porter-Duff Global Alpha mode)"]
    #[inline(always)]
    pub const fn set_s1_global_alpha_mode(&mut self, val: super::vals::S1GlobalAlphaMode) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "s1 alpha mode (Porter-Duff Alpha mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn s1_alpha_mode(&self) -> super::vals::S1AlphaMode {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::S1AlphaMode::from_bits(val as u8)
    }
    #[doc = "s1 alpha mode (Porter-Duff Alpha mode)"]
    #[inline(always)]
    pub const fn set_s1_alpha_mode(&mut self, val: super::vals::S1AlphaMode) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "s1 color mode"]
    #[must_use]
    #[inline(always)]
    pub const fn s1_color_mode(&self) -> super::vals::S1ColorMode {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::S1ColorMode::from_bits(val as u8)
    }
    #[doc = "s1 color mode"]
    #[inline(always)]
    pub const fn set_s1_color_mode(&mut self, val: super::vals::S1ColorMode) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "s0 global alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn s0_global_alpha(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "s0 global alpha"]
    #[inline(always)]
    pub const fn set_s0_global_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "s1 global alpha"]
    #[must_use]
    #[inline(always)]
    pub const fn s1_global_alpha(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "s1 global alpha"]
    #[inline(always)]
    pub const fn set_s1_global_alpha(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PorterDuffCtrl {
    #[inline(always)]
    fn default() -> PorterDuffCtrl {
        PorterDuffCtrl(0)
    }
}
impl core::fmt::Debug for PorterDuffCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PorterDuffCtrl")
            .field("porter_duff_enable", &self.porter_duff_enable())
            .field("s0_s1_factor_mode", &self.s0_s1_factor_mode())
            .field("s0_global_alpha_mode", &self.s0_global_alpha_mode())
            .field("s0_alpha_mode", &self.s0_alpha_mode())
            .field("s0_color_mode", &self.s0_color_mode())
            .field("s1_s0_factor_mode", &self.s1_s0_factor_mode())
            .field("s1_global_alpha_mode", &self.s1_global_alpha_mode())
            .field("s1_alpha_mode", &self.s1_alpha_mode())
            .field("s1_color_mode", &self.s1_color_mode())
            .field("s0_global_alpha", &self.s0_global_alpha())
            .field("s1_global_alpha", &self.s1_global_alpha())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PorterDuffCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PorterDuffCtrl {{ porter_duff_enable: {=bool:?}, s0_s1_factor_mode: {:?}, s0_global_alpha_mode: {:?}, s0_alpha_mode: {:?}, s0_color_mode: {:?}, s1_s0_factor_mode: {:?}, s1_global_alpha_mode: {:?}, s1_alpha_mode: {:?}, s1_color_mode: {:?}, s0_global_alpha: {=u8:?}, s1_global_alpha: {=u8:?} }}",
            self.porter_duff_enable(),
            self.s0_s1_factor_mode(),
            self.s0_global_alpha_mode(),
            self.s0_alpha_mode(),
            self.s0_color_mode(),
            self.s1_s0_factor_mode(),
            self.s1_global_alpha_mode(),
            self.s1_alpha_mode(),
            self.s1_color_mode(),
            self.s0_global_alpha(),
            self.s1_global_alpha()
        )
    }
}
#[doc = "PXP Power Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Power(pub u32);
impl Power {
    #[doc = "Select the low power state of the Rotation (ROT) memory."]
    #[must_use]
    #[inline(always)]
    pub const fn rot_mem_lp_state(&self) -> super::vals::RotMemLpState {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::RotMemLpState::from_bits(val as u8)
    }
    #[doc = "Select the low power state of the Rotation (ROT) memory."]
    #[inline(always)]
    pub const fn set_rot_mem_lp_state(&mut self, val: super::vals::RotMemLpState) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "Power control for the PXP."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Power control for the PXP."]
    #[inline(always)]
    pub const fn set_ctrl(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Power {
    #[inline(always)]
    fn default() -> Power {
        Power(0)
    }
}
impl core::fmt::Debug for Power {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Power")
            .field("rot_mem_lp_state", &self.rot_mem_lp_state())
            .field("ctrl", &self.ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Power {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Power {{ rot_mem_lp_state: {:?}, ctrl: {=u32:?} }}",
            self.rot_mem_lp_state(),
            self.ctrl()
        )
    }
}
#[doc = "PS Background Color"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsBackground(pub u32);
impl PsBackground {
    #[doc = "Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC"]
    #[must_use]
    #[inline(always)]
    pub const fn color(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Background color (in 24bpp format) for any pixels not within the buffer range specified by the PS ULC/LRC"]
    #[inline(always)]
    pub const fn set_color(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for PsBackground {
    #[inline(always)]
    fn default() -> PsBackground {
        PsBackground(0)
    }
}
impl core::fmt::Debug for PsBackground {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsBackground")
            .field("color", &self.color())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsBackground {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsBackground {{ color: {=u32:?} }}", self.color())
    }
}
#[doc = "PS Input Buffer Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsBuf(pub u32);
impl PsBuf {
    #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address pointer for the PS RGB or Y (luma) input buffer."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PsBuf {
    #[inline(always)]
    fn default() -> PsBuf {
        PsBuf(0)
    }
}
impl core::fmt::Debug for PsBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsBuf").field("addr", &self.addr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsBuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsBuf {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "PS Color Key High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsClrkeyhigh(pub u32);
impl PsClrkeyhigh {
    #[doc = "High range of color key applied to PS buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn pixel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High range of color key applied to PS buffer"]
    #[inline(always)]
    pub const fn set_pixel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for PsClrkeyhigh {
    #[inline(always)]
    fn default() -> PsClrkeyhigh {
        PsClrkeyhigh(0)
    }
}
impl core::fmt::Debug for PsClrkeyhigh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsClrkeyhigh")
            .field("pixel", &self.pixel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsClrkeyhigh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsClrkeyhigh {{ pixel: {=u32:?} }}", self.pixel())
    }
}
#[doc = "PS Color Key Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsClrkeylow(pub u32);
impl PsClrkeylow {
    #[doc = "Low range of color key applied to PS buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn pixel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Low range of color key applied to PS buffer"]
    #[inline(always)]
    pub const fn set_pixel(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for PsClrkeylow {
    #[inline(always)]
    fn default() -> PsClrkeylow {
        PsClrkeylow(0)
    }
}
impl core::fmt::Debug for PsClrkeylow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsClrkeylow")
            .field("pixel", &self.pixel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsClrkeylow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsClrkeylow {{ pixel: {=u32:?} }}", self.pixel())
    }
}
#[doc = "Processed Surface (PS) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsCtrl(pub u32);
impl PsCtrl {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::PsCtrlFormat {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PsCtrlFormat::from_bits(val as u8)
    }
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::PsCtrlFormat) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[must_use]
    #[inline(always)]
    pub const fn wb_swap(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub const fn set_wb_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Verticle pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decy(&self) -> super::vals::PsCtrlDecy {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::PsCtrlDecy::from_bits(val as u8)
    }
    #[doc = "Verticle pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decy(&mut self, val: super::vals::PsCtrlDecy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decx(&self) -> super::vals::PsCtrlDecx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::PsCtrlDecx::from_bits(val as u8)
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decx(&mut self, val: super::vals::PsCtrlDecx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for PsCtrl {
    #[inline(always)]
    fn default() -> PsCtrl {
        PsCtrl(0)
    }
}
impl core::fmt::Debug for PsCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsCtrl")
            .field("format", &self.format())
            .field("wb_swap", &self.wb_swap())
            .field("decy", &self.decy())
            .field("decx", &self.decx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsCtrl {{ format: {:?}, wb_swap: {=bool:?}, decy: {:?}, decx: {:?} }}",
            self.format(),
            self.wb_swap(),
            self.decy(),
            self.decx()
        )
    }
}
#[doc = "Processed Surface (PS) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsCtrlClr(pub u32);
impl PsCtrlClr {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::PsCtrlClrFormat {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PsCtrlClrFormat::from_bits(val as u8)
    }
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::PsCtrlClrFormat) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[must_use]
    #[inline(always)]
    pub const fn wb_swap(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub const fn set_wb_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Verticle pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decy(&self) -> super::vals::PsCtrlClrDecy {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::PsCtrlClrDecy::from_bits(val as u8)
    }
    #[doc = "Verticle pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decy(&mut self, val: super::vals::PsCtrlClrDecy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decx(&self) -> super::vals::PsCtrlClrDecx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::PsCtrlClrDecx::from_bits(val as u8)
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decx(&mut self, val: super::vals::PsCtrlClrDecx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for PsCtrlClr {
    #[inline(always)]
    fn default() -> PsCtrlClr {
        PsCtrlClr(0)
    }
}
impl core::fmt::Debug for PsCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsCtrlClr")
            .field("format", &self.format())
            .field("wb_swap", &self.wb_swap())
            .field("decy", &self.decy())
            .field("decx", &self.decx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsCtrlClr {{ format: {:?}, wb_swap: {=bool:?}, decy: {:?}, decx: {:?} }}",
            self.format(),
            self.wb_swap(),
            self.decy(),
            self.decx()
        )
    }
}
#[doc = "Processed Surface (PS) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsCtrlSet(pub u32);
impl PsCtrlSet {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::PsCtrlSetFormat {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PsCtrlSetFormat::from_bits(val as u8)
    }
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::PsCtrlSetFormat) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[must_use]
    #[inline(always)]
    pub const fn wb_swap(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub const fn set_wb_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Verticle pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decy(&self) -> super::vals::PsCtrlSetDecy {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::PsCtrlSetDecy::from_bits(val as u8)
    }
    #[doc = "Verticle pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decy(&mut self, val: super::vals::PsCtrlSetDecy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decx(&self) -> super::vals::PsCtrlSetDecx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::PsCtrlSetDecx::from_bits(val as u8)
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decx(&mut self, val: super::vals::PsCtrlSetDecx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for PsCtrlSet {
    #[inline(always)]
    fn default() -> PsCtrlSet {
        PsCtrlSet(0)
    }
}
impl core::fmt::Debug for PsCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsCtrlSet")
            .field("format", &self.format())
            .field("wb_swap", &self.wb_swap())
            .field("decy", &self.decy())
            .field("decx", &self.decx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsCtrlSet {{ format: {:?}, wb_swap: {=bool:?}, decy: {:?}, decx: {:?} }}",
            self.format(),
            self.wb_swap(),
            self.decy(),
            self.decx()
        )
    }
}
#[doc = "Processed Surface (PS) Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsCtrlTog(pub u32);
impl PsCtrlTog {
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn format(&self) -> super::vals::PsCtrlTogFormat {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::PsCtrlTogFormat::from_bits(val as u8)
    }
    #[doc = "PS buffer format. To select between YUV and YCbCr formats, see bit 31 of the CSC1_COEF0 register."]
    #[inline(always)]
    pub const fn set_format(&mut self, val: super::vals::PsCtrlTogFormat) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[must_use]
    #[inline(always)]
    pub const fn wb_swap(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Swap bytes in words. For each 16 bit word, the two bytes will be swapped."]
    #[inline(always)]
    pub const fn set_wb_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Verticle pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decy(&self) -> super::vals::PsCtrlTogDecy {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::PsCtrlTogDecy::from_bits(val as u8)
    }
    #[doc = "Verticle pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decy(&mut self, val: super::vals::PsCtrlTogDecy) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn decx(&self) -> super::vals::PsCtrlTogDecx {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::PsCtrlTogDecx::from_bits(val as u8)
    }
    #[doc = "Horizontal pre decimation filter control."]
    #[inline(always)]
    pub const fn set_decx(&mut self, val: super::vals::PsCtrlTogDecx) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for PsCtrlTog {
    #[inline(always)]
    fn default() -> PsCtrlTog {
        PsCtrlTog(0)
    }
}
impl core::fmt::Debug for PsCtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsCtrlTog")
            .field("format", &self.format())
            .field("wb_swap", &self.wb_swap())
            .field("decy", &self.decy())
            .field("decx", &self.decx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsCtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsCtrlTog {{ format: {:?}, wb_swap: {=bool:?}, decy: {:?}, decx: {:?} }}",
            self.format(),
            self.wb_swap(),
            self.decy(),
            self.decx()
        )
    }
}
#[doc = "PS Scale Offset Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsOffset(pub u32);
impl PsOffset {
    #[doc = "This is a 12 bit fractional representation (0"]
    #[must_use]
    #[inline(always)]
    pub const fn xoffset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub const fn set_xoffset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "This is a 12 bit fractional representation (0"]
    #[must_use]
    #[inline(always)]
    pub const fn yoffset(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "This is a 12 bit fractional representation (0"]
    #[inline(always)]
    pub const fn set_yoffset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for PsOffset {
    #[inline(always)]
    fn default() -> PsOffset {
        PsOffset(0)
    }
}
impl core::fmt::Debug for PsOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsOffset")
            .field("xoffset", &self.xoffset())
            .field("yoffset", &self.yoffset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsOffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsOffset {{ xoffset: {=u16:?}, yoffset: {=u16:?} }}",
            self.xoffset(),
            self.yoffset()
        )
    }
}
#[doc = "Processed Surface Pitch"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsPitch(pub u32);
impl PsPitch {
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[must_use]
    #[inline(always)]
    pub const fn pitch(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of bytes in memory between two vertically adjacent pixels."]
    #[inline(always)]
    pub const fn set_pitch(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PsPitch {
    #[inline(always)]
    fn default() -> PsPitch {
        PsPitch(0)
    }
}
impl core::fmt::Debug for PsPitch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsPitch")
            .field("pitch", &self.pitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsPitch {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsPitch {{ pitch: {=u16:?} }}", self.pitch())
    }
}
#[doc = "PS Scale Factor Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsScale(pub u32);
impl PsScale {
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    #[must_use]
    #[inline(always)]
    pub const fn xscale(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub const fn set_xscale(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    #[must_use]
    #[inline(always)]
    pub const fn yscale(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "This is a two bit integer and 12 bit fractional representation (##"]
    #[inline(always)]
    pub const fn set_yscale(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for PsScale {
    #[inline(always)]
    fn default() -> PsScale {
        PsScale(0)
    }
}
impl core::fmt::Debug for PsScale {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsScale")
            .field("xscale", &self.xscale())
            .field("yscale", &self.yscale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsScale {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PsScale {{ xscale: {=u16:?}, yscale: {=u16:?} }}",
            self.xscale(),
            self.yscale()
        )
    }
}
#[doc = "PS U/Cb or 2 Plane UV Input Buffer Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsUbuf(pub u32);
impl PsUbuf {
    #[doc = "Address pointer for the PS U/Cb or 2 plane UV Chroma input buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address pointer for the PS U/Cb or 2 plane UV Chroma input buffer."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PsUbuf {
    #[inline(always)]
    fn default() -> PsUbuf {
        PsUbuf(0)
    }
}
impl core::fmt::Debug for PsUbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsUbuf")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsUbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsUbuf {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "PS V/Cr Input Buffer Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PsVbuf(pub u32);
impl PsVbuf {
    #[doc = "Address pointer for the PS V/Cr Chroma input buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address pointer for the PS V/Cr Chroma input buffer."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PsVbuf {
    #[inline(always)]
    fn default() -> PsVbuf {
        PsVbuf(0)
    }
}
impl core::fmt::Debug for PsVbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PsVbuf")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PsVbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PsVbuf {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Indicates current PXP interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> super::vals::StatIrq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatIrq::from_bits(val as u8)
    }
    #[doc = "Indicates current PXP interrupt status"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: super::vals::StatIrq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_write_error(&self) -> super::vals::StatAxiWriteError {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatAxiWriteError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_write_error(&mut self, val: super::vals::StatAxiWriteError) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_read_error(&self) -> super::vals::StatAxiReadError {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatAxiReadError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_read_error(&mut self, val: super::vals::StatAxiReadError) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub const fn set_next_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_error_id(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[inline(always)]
    pub const fn set_axi_error_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn lut_dma_load_done_irq(&self) -> super::vals::StatLutDmaLoadDoneIrq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StatLutDmaLoadDoneIrq::from_bits(val as u8)
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub const fn set_lut_dma_load_done_irq(&mut self, val: super::vals::StatLutDmaLoadDoneIrq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blocky(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blocky(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blockx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blockx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("irq", &self.irq())
            .field("axi_write_error", &self.axi_write_error())
            .field("axi_read_error", &self.axi_read_error())
            .field("next_irq", &self.next_irq())
            .field("axi_error_id", &self.axi_error_id())
            .field("lut_dma_load_done_irq", &self.lut_dma_load_done_irq())
            .field("blocky", &self.blocky())
            .field("blockx", &self.blockx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ irq: {:?}, axi_write_error: {:?}, axi_read_error: {:?}, next_irq: {=bool:?}, axi_error_id: {=u8:?}, lut_dma_load_done_irq: {:?}, blocky: {=u8:?}, blockx: {=u8:?} }}",
            self.irq(),
            self.axi_write_error(),
            self.axi_read_error(),
            self.next_irq(),
            self.axi_error_id(),
            self.lut_dma_load_done_irq(),
            self.blocky(),
            self.blockx()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatClr(pub u32);
impl StatClr {
    #[doc = "Indicates current PXP interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> super::vals::StatClrIrq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatClrIrq::from_bits(val as u8)
    }
    #[doc = "Indicates current PXP interrupt status"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: super::vals::StatClrIrq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_write_error(&self) -> super::vals::StatClrAxiWriteError {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatClrAxiWriteError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_write_error(&mut self, val: super::vals::StatClrAxiWriteError) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_read_error(&self) -> super::vals::StatClrAxiReadError {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatClrAxiReadError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_read_error(&mut self, val: super::vals::StatClrAxiReadError) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub const fn set_next_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_error_id(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[inline(always)]
    pub const fn set_axi_error_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn lut_dma_load_done_irq(&self) -> super::vals::StatClrLutDmaLoadDoneIrq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StatClrLutDmaLoadDoneIrq::from_bits(val as u8)
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub const fn set_lut_dma_load_done_irq(&mut self, val: super::vals::StatClrLutDmaLoadDoneIrq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blocky(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blocky(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blockx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blockx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for StatClr {
    #[inline(always)]
    fn default() -> StatClr {
        StatClr(0)
    }
}
impl core::fmt::Debug for StatClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StatClr")
            .field("irq", &self.irq())
            .field("axi_write_error", &self.axi_write_error())
            .field("axi_read_error", &self.axi_read_error())
            .field("next_irq", &self.next_irq())
            .field("axi_error_id", &self.axi_error_id())
            .field("lut_dma_load_done_irq", &self.lut_dma_load_done_irq())
            .field("blocky", &self.blocky())
            .field("blockx", &self.blockx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StatClr {{ irq: {:?}, axi_write_error: {:?}, axi_read_error: {:?}, next_irq: {=bool:?}, axi_error_id: {=u8:?}, lut_dma_load_done_irq: {:?}, blocky: {=u8:?}, blockx: {=u8:?} }}",
            self.irq(),
            self.axi_write_error(),
            self.axi_read_error(),
            self.next_irq(),
            self.axi_error_id(),
            self.lut_dma_load_done_irq(),
            self.blocky(),
            self.blockx()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatSet(pub u32);
impl StatSet {
    #[doc = "Indicates current PXP interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> super::vals::StatSetIrq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatSetIrq::from_bits(val as u8)
    }
    #[doc = "Indicates current PXP interrupt status"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: super::vals::StatSetIrq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_write_error(&self) -> super::vals::StatSetAxiWriteError {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatSetAxiWriteError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_write_error(&mut self, val: super::vals::StatSetAxiWriteError) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_read_error(&self) -> super::vals::StatSetAxiReadError {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatSetAxiReadError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_read_error(&mut self, val: super::vals::StatSetAxiReadError) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub const fn set_next_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_error_id(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[inline(always)]
    pub const fn set_axi_error_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn lut_dma_load_done_irq(&self) -> super::vals::StatSetLutDmaLoadDoneIrq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StatSetLutDmaLoadDoneIrq::from_bits(val as u8)
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub const fn set_lut_dma_load_done_irq(&mut self, val: super::vals::StatSetLutDmaLoadDoneIrq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blocky(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blocky(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blockx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blockx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for StatSet {
    #[inline(always)]
    fn default() -> StatSet {
        StatSet(0)
    }
}
impl core::fmt::Debug for StatSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StatSet")
            .field("irq", &self.irq())
            .field("axi_write_error", &self.axi_write_error())
            .field("axi_read_error", &self.axi_read_error())
            .field("next_irq", &self.next_irq())
            .field("axi_error_id", &self.axi_error_id())
            .field("lut_dma_load_done_irq", &self.lut_dma_load_done_irq())
            .field("blocky", &self.blocky())
            .field("blockx", &self.blockx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StatSet {{ irq: {:?}, axi_write_error: {:?}, axi_read_error: {:?}, next_irq: {=bool:?}, axi_error_id: {=u8:?}, lut_dma_load_done_irq: {:?}, blocky: {=u8:?}, blockx: {=u8:?} }}",
            self.irq(),
            self.axi_write_error(),
            self.axi_read_error(),
            self.next_irq(),
            self.axi_error_id(),
            self.lut_dma_load_done_irq(),
            self.blocky(),
            self.blockx()
        )
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StatTog(pub u32);
impl StatTog {
    #[doc = "Indicates current PXP interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> super::vals::StatTogIrq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StatTogIrq::from_bits(val as u8)
    }
    #[doc = "Indicates current PXP interrupt status"]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: super::vals::StatTogIrq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_write_error(&self) -> super::vals::StatTogAxiWriteError {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StatTogAxiWriteError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI write error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_write_error(&mut self, val: super::vals::StatTogAxiWriteError) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_read_error(&self) -> super::vals::StatTogAxiReadError {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StatTogAxiReadError::from_bits(val as u8)
    }
    #[doc = "Indicates PXP encountered an AXI read error and processing has been terminated."]
    #[inline(always)]
    pub const fn set_axi_read_error(&mut self, val: super::vals::StatTogAxiReadError) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[must_use]
    #[inline(always)]
    pub const fn next_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that a command issued with the \"Next Command\" functionality has been issued and that a new command may be initiated with a write to the PXP_NEXT register"]
    #[inline(always)]
    pub const fn set_next_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[must_use]
    #[inline(always)]
    pub const fn axi_error_id(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the AXI ID of the failing bus operation."]
    #[inline(always)]
    pub const fn set_axi_error_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[must_use]
    #[inline(always)]
    pub const fn lut_dma_load_done_irq(&self) -> super::vals::StatTogLutDmaLoadDoneIrq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::StatTogLutDmaLoadDoneIrq::from_bits(val as u8)
    }
    #[doc = "Indicates that the LUT DMA transfer has completed."]
    #[inline(always)]
    pub const fn set_lut_dma_load_done_irq(&mut self, val: super::vals::StatTogLutDmaLoadDoneIrq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blocky(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blocky(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[must_use]
    #[inline(always)]
    pub const fn blockx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the X coordinate of the block currently being rendered."]
    #[inline(always)]
    pub const fn set_blockx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for StatTog {
    #[inline(always)]
    fn default() -> StatTog {
        StatTog(0)
    }
}
impl core::fmt::Debug for StatTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StatTog")
            .field("irq", &self.irq())
            .field("axi_write_error", &self.axi_write_error())
            .field("axi_read_error", &self.axi_read_error())
            .field("next_irq", &self.next_irq())
            .field("axi_error_id", &self.axi_error_id())
            .field("lut_dma_load_done_irq", &self.lut_dma_load_done_irq())
            .field("blocky", &self.blocky())
            .field("blockx", &self.blockx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StatTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StatTog {{ irq: {:?}, axi_write_error: {:?}, axi_read_error: {:?}, next_irq: {=bool:?}, axi_error_id: {=u8:?}, lut_dma_load_done_irq: {:?}, blocky: {=u8:?}, blockx: {=u8:?} }}",
            self.irq(),
            self.axi_write_error(),
            self.axi_read_error(),
            self.next_irq(),
            self.axi_error_id(),
            self.lut_dma_load_done_irq(),
            self.blocky(),
            self.blockx()
        )
    }
}
