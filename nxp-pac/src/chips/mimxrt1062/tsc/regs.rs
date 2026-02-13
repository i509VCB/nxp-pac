#[doc = "Basic Setting"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BasicSetting(pub u32);
impl BasicSetting {
    #[doc = "Auto Measure"]
    #[must_use]
    #[inline(always)]
    pub const fn auto_measure(&self) -> super::vals::AutoMeasure {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AutoMeasure::from_bits(val as u8)
    }
    #[doc = "Auto Measure"]
    #[inline(always)]
    pub const fn set_auto_measure(&mut self, val: super::vals::AutoMeasure) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "4/5 Wire detection"]
    #[must_use]
    #[inline(always)]
    pub const fn wire_4_5(&self) -> super::vals::Wire45 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Wire45::from_bits(val as u8)
    }
    #[doc = "4/5 Wire detection"]
    #[inline(always)]
    pub const fn set_wire_4_5(&mut self, val: super::vals::Wire45) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Measure Delay Time"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_delay_time(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Measure Delay Time"]
    #[inline(always)]
    pub const fn set_measure_delay_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for BasicSetting {
    #[inline(always)]
    fn default() -> BasicSetting {
        BasicSetting(0)
    }
}
impl core::fmt::Debug for BasicSetting {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BasicSetting")
            .field("auto_measure", &self.auto_measure())
            .field("wire_4_5", &self.wire_4_5())
            .field("measure_delay_time", &self.measure_delay_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BasicSetting {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BasicSetting {{ auto_measure: {:?}, wire_4_5: {:?}, measure_delay_time: {=u32:?} }}",
            self.auto_measure(),
            self.wire_4_5(),
            self.measure_delay_time()
        )
    }
}
#[doc = "Debug Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugMode(pub u32);
impl DebugMode {
    #[doc = "ADC Conversion Value"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_conv_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ADC Conversion Value"]
    #[inline(always)]
    pub const fn set_adc_conv_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "ADC COCO Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_coco(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ADC COCO Signal"]
    #[inline(always)]
    pub const fn set_adc_coco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Hardware Trigger Select Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_hwts(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Hardware Trigger Select Signal"]
    #[inline(always)]
    pub const fn set_ext_hwts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trigger(&self) -> super::vals::Trigger {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Trigger::from_bits(val as u8)
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub const fn set_trigger(&mut self, val: super::vals::Trigger) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ADC Coco Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_coco_clear(&self) -> super::vals::AdcCocoClear {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::AdcCocoClear::from_bits(val as u8)
    }
    #[doc = "ADC Coco Clear"]
    #[inline(always)]
    pub const fn set_adc_coco_clear(&mut self, val: super::vals::AdcCocoClear) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "ADC COCO Clear Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_coco_clear_disable(&self) -> super::vals::AdcCocoClearDisable {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::AdcCocoClearDisable::from_bits(val as u8)
    }
    #[doc = "ADC COCO Clear Disable"]
    #[inline(always)]
    pub const fn set_adc_coco_clear_disable(&mut self, val: super::vals::AdcCocoClearDisable) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_en(&self) -> super::vals::DebugEn {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DebugEn::from_bits(val as u8)
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_debug_en(&mut self, val: super::vals::DebugEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for DebugMode {
    #[inline(always)]
    fn default() -> DebugMode {
        DebugMode(0)
    }
}
impl core::fmt::Debug for DebugMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugMode")
            .field("adc_conv_value", &self.adc_conv_value())
            .field("adc_coco", &self.adc_coco())
            .field("ext_hwts", &self.ext_hwts())
            .field("trigger", &self.trigger())
            .field("adc_coco_clear", &self.adc_coco_clear())
            .field("adc_coco_clear_disable", &self.adc_coco_clear_disable())
            .field("debug_en", &self.debug_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugMode {{ adc_conv_value: {=u16:?}, adc_coco: {=bool:?}, ext_hwts: {=u8:?}, trigger: {:?}, adc_coco_clear: {:?}, adc_coco_clear_disable: {:?}, debug_en: {:?} }}",
            self.adc_conv_value(),
            self.adc_coco(),
            self.ext_hwts(),
            self.trigger(),
            self.adc_coco_clear(),
            self.adc_coco_clear_disable(),
            self.debug_en()
        )
    }
}
#[doc = "Debug Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugMode2(pub u32);
impl DebugMode2 {
    #[doc = "XPUL Wire Pull Down Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xpul_pull_down(&self) -> super::vals::XpulPullDown {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XpulPullDown::from_bits(val as u8)
    }
    #[doc = "XPUL Wire Pull Down Switch"]
    #[inline(always)]
    pub const fn set_xpul_pull_down(&mut self, val: super::vals::XpulPullDown) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XPUL Wire Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xpul_pull_up(&self) -> super::vals::XpulPullUp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::XpulPullUp::from_bits(val as u8)
    }
    #[doc = "XPUL Wire Pull Up Switch"]
    #[inline(always)]
    pub const fn set_xpul_pull_up(&mut self, val: super::vals::XpulPullUp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "XPUL Wire 200K Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xpul_200k_pull_up(&self) -> super::vals::Xpul200kPullUp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Xpul200kPullUp::from_bits(val as u8)
    }
    #[doc = "XPUL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub const fn set_xpul_200k_pull_up(&mut self, val: super::vals::Xpul200kPullUp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "XNUR Wire Pull Down Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xnur_pull_down(&self) -> super::vals::XnurPullDown {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::XnurPullDown::from_bits(val as u8)
    }
    #[doc = "XNUR Wire Pull Down Switch"]
    #[inline(always)]
    pub const fn set_xnur_pull_down(&mut self, val: super::vals::XnurPullDown) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "XNUR Wire Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xnur_pull_up(&self) -> super::vals::XnurPullUp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::XnurPullUp::from_bits(val as u8)
    }
    #[doc = "XNUR Wire Pull Up Switch"]
    #[inline(always)]
    pub const fn set_xnur_pull_up(&mut self, val: super::vals::XnurPullUp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "XNUR Wire 200K Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn xnur_200k_pull_up(&self) -> super::vals::Xnur200kPullUp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Xnur200kPullUp::from_bits(val as u8)
    }
    #[doc = "XNUR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub const fn set_xnur_200k_pull_up(&mut self, val: super::vals::Xnur200kPullUp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "YPLL Wire Pull Down Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ypll_pull_down(&self) -> super::vals::YpllPullDown {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::YpllPullDown::from_bits(val as u8)
    }
    #[doc = "YPLL Wire Pull Down Switch"]
    #[inline(always)]
    pub const fn set_ypll_pull_down(&mut self, val: super::vals::YpllPullDown) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "YPLL Wire Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ypll_pull_up(&self) -> super::vals::YpllPullUp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::YpllPullUp::from_bits(val as u8)
    }
    #[doc = "YPLL Wire Pull Up Switch"]
    #[inline(always)]
    pub const fn set_ypll_pull_up(&mut self, val: super::vals::YpllPullUp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "YPLL Wire 200K Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ypll_200k_pull_up(&self) -> super::vals::Ypll200kPullUp {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ypll200kPullUp::from_bits(val as u8)
    }
    #[doc = "YPLL Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub const fn set_ypll_200k_pull_up(&mut self, val: super::vals::Ypll200kPullUp) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "YNLR Wire Pull Down Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ynlr_pull_down(&self) -> super::vals::YnlrPullDown {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::YnlrPullDown::from_bits(val as u8)
    }
    #[doc = "YNLR Wire Pull Down Switch"]
    #[inline(always)]
    pub const fn set_ynlr_pull_down(&mut self, val: super::vals::YnlrPullDown) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "YNLR Wire Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ynlr_pull_up(&self) -> super::vals::YnlrPullUp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::YnlrPullUp::from_bits(val as u8)
    }
    #[doc = "YNLR Wire Pull Up Switch"]
    #[inline(always)]
    pub const fn set_ynlr_pull_up(&mut self, val: super::vals::YnlrPullUp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "YNLR Wire 200K Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn ynlr_200k_pull_up(&self) -> super::vals::Ynlr200kPullUp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ynlr200kPullUp::from_bits(val as u8)
    }
    #[doc = "YNLR Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub const fn set_ynlr_200k_pull_up(&mut self, val: super::vals::Ynlr200kPullUp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Wiper Wire Pull Down Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wiper_pull_down(&self) -> super::vals::WiperPullDown {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::WiperPullDown::from_bits(val as u8)
    }
    #[doc = "Wiper Wire Pull Down Switch"]
    #[inline(always)]
    pub const fn set_wiper_pull_down(&mut self, val: super::vals::WiperPullDown) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Wiper Wire Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wiper_pull_up(&self) -> super::vals::WiperPullUp {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::WiperPullUp::from_bits(val as u8)
    }
    #[doc = "Wiper Wire Pull Up Switch"]
    #[inline(always)]
    pub const fn set_wiper_pull_up(&mut self, val: super::vals::WiperPullUp) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Wiper Wire 200K Pull Up Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wiper_200k_pull_up(&self) -> super::vals::Wiper200kPullUp {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Wiper200kPullUp::from_bits(val as u8)
    }
    #[doc = "Wiper Wire 200K Pull Up Switch"]
    #[inline(always)]
    pub const fn set_wiper_200k_pull_up(&mut self, val: super::vals::Wiper200kPullUp) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Detect Four Wire"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_four_wire(&self) -> super::vals::DetectFourWire {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DetectFourWire::from_bits(val as u8)
    }
    #[doc = "Detect Four Wire"]
    #[inline(always)]
    pub const fn set_detect_four_wire(&mut self, val: super::vals::DetectFourWire) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Detect Five Wire"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_five_wire(&self) -> super::vals::DetectFiveWire {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::DetectFiveWire::from_bits(val as u8)
    }
    #[doc = "Detect Five Wire"]
    #[inline(always)]
    pub const fn set_detect_five_wire(&mut self, val: super::vals::DetectFiveWire) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn state_machine(&self) -> super::vals::StateMachine {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::StateMachine::from_bits(val as u8)
    }
    #[doc = "State Machine"]
    #[inline(always)]
    pub const fn set_state_machine(&mut self, val: super::vals::StateMachine) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Intermediate State"]
    #[must_use]
    #[inline(always)]
    pub const fn intermediate(&self) -> super::vals::Intermediate {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Intermediate::from_bits(val as u8)
    }
    #[doc = "Intermediate State"]
    #[inline(always)]
    pub const fn set_intermediate(&mut self, val: super::vals::Intermediate) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Detect Enable Four Wire"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_enable_four_wire(&self) -> super::vals::DetectEnableFourWire {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::DetectEnableFourWire::from_bits(val as u8)
    }
    #[doc = "Detect Enable Four Wire"]
    #[inline(always)]
    pub const fn set_detect_enable_four_wire(&mut self, val: super::vals::DetectEnableFourWire) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Detect Enable Five Wire"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_enable_five_wire(&self) -> super::vals::DetectEnableFiveWire {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DetectEnableFiveWire::from_bits(val as u8)
    }
    #[doc = "Detect Enable Five Wire"]
    #[inline(always)]
    pub const fn set_detect_enable_five_wire(&mut self, val: super::vals::DetectEnableFiveWire) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This field indicates glitch threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn de_glitch(&self) -> super::vals::DeGlitch {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::DeGlitch::from_bits(val as u8)
    }
    #[doc = "This field indicates glitch threshold"]
    #[inline(always)]
    pub const fn set_de_glitch(&mut self, val: super::vals::DeGlitch) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for DebugMode2 {
    #[inline(always)]
    fn default() -> DebugMode2 {
        DebugMode2(0)
    }
}
impl core::fmt::Debug for DebugMode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugMode2")
            .field("xpul_pull_down", &self.xpul_pull_down())
            .field("xpul_pull_up", &self.xpul_pull_up())
            .field("xpul_200k_pull_up", &self.xpul_200k_pull_up())
            .field("xnur_pull_down", &self.xnur_pull_down())
            .field("xnur_pull_up", &self.xnur_pull_up())
            .field("xnur_200k_pull_up", &self.xnur_200k_pull_up())
            .field("ypll_pull_down", &self.ypll_pull_down())
            .field("ypll_pull_up", &self.ypll_pull_up())
            .field("ypll_200k_pull_up", &self.ypll_200k_pull_up())
            .field("ynlr_pull_down", &self.ynlr_pull_down())
            .field("ynlr_pull_up", &self.ynlr_pull_up())
            .field("ynlr_200k_pull_up", &self.ynlr_200k_pull_up())
            .field("wiper_pull_down", &self.wiper_pull_down())
            .field("wiper_pull_up", &self.wiper_pull_up())
            .field("wiper_200k_pull_up", &self.wiper_200k_pull_up())
            .field("detect_four_wire", &self.detect_four_wire())
            .field("detect_five_wire", &self.detect_five_wire())
            .field("state_machine", &self.state_machine())
            .field("intermediate", &self.intermediate())
            .field("detect_enable_four_wire", &self.detect_enable_four_wire())
            .field("detect_enable_five_wire", &self.detect_enable_five_wire())
            .field("de_glitch", &self.de_glitch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugMode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugMode2 {{ xpul_pull_down: {:?}, xpul_pull_up: {:?}, xpul_200k_pull_up: {:?}, xnur_pull_down: {:?}, xnur_pull_up: {:?}, xnur_200k_pull_up: {:?}, ypll_pull_down: {:?}, ypll_pull_up: {:?}, ypll_200k_pull_up: {:?}, ynlr_pull_down: {:?}, ynlr_pull_up: {:?}, ynlr_200k_pull_up: {:?}, wiper_pull_down: {:?}, wiper_pull_up: {:?}, wiper_200k_pull_up: {:?}, detect_four_wire: {:?}, detect_five_wire: {:?}, state_machine: {:?}, intermediate: {:?}, detect_enable_four_wire: {:?}, detect_enable_five_wire: {:?}, de_glitch: {:?} }}",
            self.xpul_pull_down(),
            self.xpul_pull_up(),
            self.xpul_200k_pull_up(),
            self.xnur_pull_down(),
            self.xnur_pull_up(),
            self.xnur_200k_pull_up(),
            self.ypll_pull_down(),
            self.ypll_pull_up(),
            self.ypll_200k_pull_up(),
            self.ynlr_pull_down(),
            self.ynlr_pull_up(),
            self.ynlr_200k_pull_up(),
            self.wiper_pull_down(),
            self.wiper_pull_up(),
            self.wiper_200k_pull_up(),
            self.detect_four_wire(),
            self.detect_five_wire(),
            self.state_machine(),
            self.intermediate(),
            self.detect_enable_four_wire(),
            self.detect_enable_five_wire(),
            self.de_glitch()
        )
    }
}
#[doc = "Flow Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlowControl(pub u32);
impl FlowControl {
    #[doc = "Soft Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_rst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Soft Reset"]
    #[inline(always)]
    pub const fn set_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start Measure"]
    #[must_use]
    #[inline(always)]
    pub const fn start_measure(&self) -> super::vals::StartMeasure {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StartMeasure::from_bits(val as u8)
    }
    #[doc = "Start Measure"]
    #[inline(always)]
    pub const fn set_start_measure(&mut self, val: super::vals::StartMeasure) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Drop Measure"]
    #[must_use]
    #[inline(always)]
    pub const fn drop_measure(&self) -> super::vals::DropMeasure {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DropMeasure::from_bits(val as u8)
    }
    #[doc = "Drop Measure"]
    #[inline(always)]
    pub const fn set_drop_measure(&mut self, val: super::vals::DropMeasure) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Start Sense"]
    #[must_use]
    #[inline(always)]
    pub const fn start_sense(&self) -> super::vals::StartSense {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::StartSense::from_bits(val as u8)
    }
    #[doc = "Start Sense"]
    #[inline(always)]
    pub const fn set_start_sense(&mut self, val: super::vals::StartSense) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is for SW disable registers"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::Disable {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Disable::from_bits(val as u8)
    }
    #[doc = "This bit is for SW disable registers"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::Disable) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for FlowControl {
    #[inline(always)]
    fn default() -> FlowControl {
        FlowControl(0)
    }
}
impl core::fmt::Debug for FlowControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlowControl")
            .field("sw_rst", &self.sw_rst())
            .field("start_measure", &self.start_measure())
            .field("drop_measure", &self.drop_measure())
            .field("start_sense", &self.start_sense())
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlowControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlowControl {{ sw_rst: {=bool:?}, start_measure: {:?}, drop_measure: {:?}, start_sense: {:?}, disable: {:?} }}",
            self.sw_rst(),
            self.start_measure(),
            self.drop_measure(),
            self.start_sense(),
            self.disable()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEn(pub u32);
impl IntEn {
    #[doc = "Measure Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_int_en(&self) -> super::vals::MeasureIntEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MeasureIntEn::from_bits(val as u8)
    }
    #[doc = "Measure Interrupt Enable"]
    #[inline(always)]
    pub const fn set_measure_int_en(&mut self, val: super::vals::MeasureIntEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_int_en(&self) -> super::vals::DetectIntEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DetectIntEn::from_bits(val as u8)
    }
    #[doc = "Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_detect_int_en(&mut self, val: super::vals::DetectIntEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Idle Software Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn idle_sw_int_en(&self) -> super::vals::IdleSwIntEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IdleSwIntEn::from_bits(val as u8)
    }
    #[doc = "Idle Software Interrupt Enable"]
    #[inline(always)]
    pub const fn set_idle_sw_int_en(&mut self, val: super::vals::IdleSwIntEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for IntEn {
    #[inline(always)]
    fn default() -> IntEn {
        IntEn(0)
    }
}
impl core::fmt::Debug for IntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEn")
            .field("measure_int_en", &self.measure_int_en())
            .field("detect_int_en", &self.detect_int_en())
            .field("idle_sw_int_en", &self.idle_sw_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntEn {{ measure_int_en: {:?}, detect_int_en: {:?}, idle_sw_int_en: {:?} }}",
            self.measure_int_en(),
            self.detect_int_en(),
            self.idle_sw_int_en()
        )
    }
}
#[doc = "Interrupt Signal Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSigEn(pub u32);
impl IntSigEn {
    #[doc = "Measure Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_sig_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Measure Signal Enable"]
    #[inline(always)]
    pub const fn set_measure_sig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Detect Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sig_en(&self) -> super::vals::DetectSigEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DetectSigEn::from_bits(val as u8)
    }
    #[doc = "Detect Signal Enable"]
    #[inline(always)]
    pub const fn set_detect_sig_en(&mut self, val: super::vals::DetectSigEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Valid Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn valid_sig_en(&self) -> super::vals::ValidSigEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ValidSigEn::from_bits(val as u8)
    }
    #[doc = "Valid Signal Enable"]
    #[inline(always)]
    pub const fn set_valid_sig_en(&mut self, val: super::vals::ValidSigEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Idle Software Signal Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn idle_sw_sig_en(&self) -> super::vals::IdleSwSigEn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IdleSwSigEn::from_bits(val as u8)
    }
    #[doc = "Idle Software Signal Enable"]
    #[inline(always)]
    pub const fn set_idle_sw_sig_en(&mut self, val: super::vals::IdleSwSigEn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for IntSigEn {
    #[inline(always)]
    fn default() -> IntSigEn {
        IntSigEn(0)
    }
}
impl core::fmt::Debug for IntSigEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSigEn")
            .field("measure_sig_en", &self.measure_sig_en())
            .field("detect_sig_en", &self.detect_sig_en())
            .field("valid_sig_en", &self.valid_sig_en())
            .field("idle_sw_sig_en", &self.idle_sw_sig_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSigEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSigEn {{ measure_sig_en: {=bool:?}, detect_sig_en: {:?}, valid_sig_en: {:?}, idle_sw_sig_en: {:?} }}",
            self.measure_sig_en(),
            self.detect_sig_en(),
            self.valid_sig_en(),
            self.idle_sw_sig_en()
        )
    }
}
#[doc = "Intterrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Measure Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn measure(&self) -> super::vals::Measure {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Measure::from_bits(val as u8)
    }
    #[doc = "Measure Signal"]
    #[inline(always)]
    pub const fn set_measure(&mut self, val: super::vals::Measure) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Detect Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn detect(&self) -> super::vals::Detect {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Detect::from_bits(val as u8)
    }
    #[doc = "Detect Signal"]
    #[inline(always)]
    pub const fn set_detect(&mut self, val: super::vals::Detect) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Valid Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> super::vals::Valid {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Valid::from_bits(val as u8)
    }
    #[doc = "Valid Signal"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: super::vals::Valid) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Idle Software"]
    #[must_use]
    #[inline(always)]
    pub const fn idle_sw(&self) -> super::vals::IdleSw {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IdleSw::from_bits(val as u8)
    }
    #[doc = "Idle Software"]
    #[inline(always)]
    pub const fn set_idle_sw(&mut self, val: super::vals::IdleSw) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("measure", &self.measure())
            .field("detect", &self.detect())
            .field("valid", &self.valid())
            .field("idle_sw", &self.idle_sw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ measure: {:?}, detect: {:?}, valid: {:?}, idle_sw: {:?} }}",
            self.measure(),
            self.detect(),
            self.valid(),
            self.idle_sw()
        )
    }
}
#[doc = "Measure Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MeaseureValue(pub u32);
impl MeaseureValue {
    #[doc = "Y Value"]
    #[must_use]
    #[inline(always)]
    pub const fn y_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Y Value"]
    #[inline(always)]
    pub const fn set_y_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "X Value"]
    #[must_use]
    #[inline(always)]
    pub const fn x_value(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "X Value"]
    #[inline(always)]
    pub const fn set_x_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for MeaseureValue {
    #[inline(always)]
    fn default() -> MeaseureValue {
        MeaseureValue(0)
    }
}
impl core::fmt::Debug for MeaseureValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MeaseureValue")
            .field("y_value", &self.y_value())
            .field("x_value", &self.x_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MeaseureValue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MeaseureValue {{ y_value: {=u16:?}, x_value: {=u16:?} }}",
            self.y_value(),
            self.x_value()
        )
    }
}
#[doc = "Pre-charge Time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PreChargeTime(pub u32);
impl PreChargeTime {
    #[doc = "Before detection, the top screen needs some time before being pulled up to a high voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_charge_time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Before detection, the top screen needs some time before being pulled up to a high voltage."]
    #[inline(always)]
    pub const fn set_pre_charge_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PreChargeTime {
    #[inline(always)]
    fn default() -> PreChargeTime {
        PreChargeTime(0)
    }
}
impl core::fmt::Debug for PreChargeTime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PreChargeTime")
            .field("pre_charge_time", &self.pre_charge_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PreChargeTime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PreChargeTime {{ pre_charge_time: {=u32:?} }}",
            self.pre_charge_time()
        )
    }
}
