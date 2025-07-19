#[doc = "ADC_ETC Global Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_enable(&self) -> super::vals::TrigEnable {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::TrigEnable::from_bits(val as u8)
    }
    #[doc = "TRIG enable register."]
    #[inline(always)]
    pub const fn set_trig_enable(&mut self, val: super::vals::TrigEnable) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "TSC0 TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn ext0_trig_enable(&self) -> super::vals::Ext0TrigEnable {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ext0TrigEnable::from_bits(val as u8)
    }
    #[doc = "TSC0 TRIG enable register."]
    #[inline(always)]
    pub const fn set_ext0_trig_enable(&mut self, val: super::vals::Ext0TrigEnable) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "External TSC0 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn ext0_trig_priority(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "External TSC0 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[inline(always)]
    pub const fn set_ext0_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "TSC1 TRIG enable register."]
    #[must_use]
    #[inline(always)]
    pub const fn ext1_trig_enable(&self) -> super::vals::Ext1TrigEnable {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ext1TrigEnable::from_bits(val as u8)
    }
    #[doc = "TSC1 TRIG enable register."]
    #[inline(always)]
    pub const fn set_ext1_trig_enable(&mut self, val: super::vals::Ext1TrigEnable) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "External TSC1 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[must_use]
    #[inline(always)]
    pub const fn ext1_trig_priority(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "External TSC1 trigger priority, 7 is highest priority, while 0 is lowest."]
    #[inline(always)]
    pub const fn set_ext1_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Pre-divider for trig delay and interval"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_divider(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pre-divider for trig delay and interval"]
    #[inline(always)]
    pub const fn set_pre_divider(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Select the trigger type of the DMA_REQ."]
    #[must_use]
    #[inline(always)]
    pub const fn dma_mode_sel(&self) -> super::vals::DmaModeSel {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::DmaModeSel::from_bits(val as u8)
    }
    #[doc = "Select the trigger type of the DMA_REQ."]
    #[inline(always)]
    pub const fn set_dma_mode_sel(&mut self, val: super::vals::DmaModeSel) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "TSC Bypass To use ADC2, this bit should be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn tsc_bypass(&self) -> super::vals::TscBypass {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::TscBypass::from_bits(val as u8)
    }
    #[doc = "TSC Bypass To use ADC2, this bit should be cleared."]
    #[inline(always)]
    pub const fn set_tsc_bypass(&mut self, val: super::vals::TscBypass) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Software synchronous reset, active high."]
    #[must_use]
    #[inline(always)]
    pub const fn softrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software synchronous reset, active high."]
    #[inline(always)]
    pub const fn set_softrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("trig_enable", &self.trig_enable())
            .field("ext0_trig_enable", &self.ext0_trig_enable())
            .field("ext0_trig_priority", &self.ext0_trig_priority())
            .field("ext1_trig_enable", &self.ext1_trig_enable())
            .field("ext1_trig_priority", &self.ext1_trig_priority())
            .field("pre_divider", &self.pre_divider())
            .field("dma_mode_sel", &self.dma_mode_sel())
            .field("tsc_bypass", &self.tsc_bypass())
            .field("softrst", &self.softrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ trig_enable: {:?}, ext0_trig_enable: {:?}, ext0_trig_priority: {=u8:?}, ext1_trig_enable: {:?}, ext1_trig_priority: {=u8:?}, pre_divider: {=u8:?}, dma_mode_sel: {:?}, tsc_bypass: {:?}, softrst: {=bool:?} }}",
            self.trig_enable(),
            self.ext0_trig_enable(),
            self.ext0_trig_priority(),
            self.ext1_trig_enable(),
            self.ext1_trig_priority(),
            self.pre_divider(),
            self.dma_mode_sel(),
            self.tsc_bypass(),
            self.softrst()
        )
    }
}
#[doc = "ETC DMA control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCtrl(pub u32);
impl DmaCtrl {
    #[doc = "Enable DMA request when TRIG0 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG0 done."]
    #[inline(always)]
    pub const fn set_trig0_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable DMA request when TRIG1 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG1 done."]
    #[inline(always)]
    pub const fn set_trig1_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable DMA request when TRIG2 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG2 done."]
    #[inline(always)]
    pub const fn set_trig2_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable DMA request when TRIG3 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG3 done."]
    #[inline(always)]
    pub const fn set_trig3_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable DMA request when TRIG4 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG4 done."]
    #[inline(always)]
    pub const fn set_trig4_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable DMA request when TRIG5 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_enable(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG5 done."]
    #[inline(always)]
    pub const fn set_trig5_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable DMA request when TRIG6 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG6 done."]
    #[inline(always)]
    pub const fn set_trig6_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable DMA request when TRIG7 done."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_enable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA request when TRIG7 done."]
    #[inline(always)]
    pub const fn set_trig7_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_req(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig0_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_req(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig1_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_req(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig2_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_req(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig3_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_req(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig4_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_req(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig5_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_req(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig6_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Flag bit for DMA request"]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_req(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Flag bit for DMA request"]
    #[inline(always)]
    pub const fn set_trig7_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for DmaCtrl {
    #[inline(always)]
    fn default() -> DmaCtrl {
        DmaCtrl(0)
    }
}
impl core::fmt::Debug for DmaCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCtrl")
            .field("trig0_enable", &self.trig0_enable())
            .field("trig1_enable", &self.trig1_enable())
            .field("trig2_enable", &self.trig2_enable())
            .field("trig3_enable", &self.trig3_enable())
            .field("trig4_enable", &self.trig4_enable())
            .field("trig5_enable", &self.trig5_enable())
            .field("trig6_enable", &self.trig6_enable())
            .field("trig7_enable", &self.trig7_enable())
            .field("trig0_req", &self.trig0_req())
            .field("trig1_req", &self.trig1_req())
            .field("trig2_req", &self.trig2_req())
            .field("trig3_req", &self.trig3_req())
            .field("trig4_req", &self.trig4_req())
            .field("trig5_req", &self.trig5_req())
            .field("trig6_req", &self.trig6_req())
            .field("trig7_req", &self.trig7_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCtrl {{ trig0_enable: {=bool:?}, trig1_enable: {=bool:?}, trig2_enable: {=bool:?}, trig3_enable: {=bool:?}, trig4_enable: {=bool:?}, trig5_enable: {=bool:?}, trig6_enable: {=bool:?}, trig7_enable: {=bool:?}, trig0_req: {=bool:?}, trig1_req: {=bool:?}, trig2_req: {=bool:?}, trig3_req: {=bool:?}, trig4_req: {=bool:?}, trig5_req: {=bool:?}, trig6_req: {=bool:?}, trig7_req: {=bool:?} }}",
            self.trig0_enable(),
            self.trig1_enable(),
            self.trig2_enable(),
            self.trig3_enable(),
            self.trig4_enable(),
            self.trig5_enable(),
            self.trig6_enable(),
            self.trig7_enable(),
            self.trig0_req(),
            self.trig1_req(),
            self.trig2_req(),
            self.trig3_req(),
            self.trig4_req(),
            self.trig5_req(),
            self.trig6_req(),
            self.trig7_req()
        )
    }
}
#[doc = "ETC DONE0 and DONE1 IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done01Irq(pub u32);
impl Done01Irq {
    #[doc = "TRIG0 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG1 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG2 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG3 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG4 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG5 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG6 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG7 done0 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done0 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG0 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRIG1 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TRIG2 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TRIG3 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TRIG4 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRIG5 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TRIG6 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done1(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TRIG7 done1 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done1(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done1 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Done01Irq {
    #[inline(always)]
    fn default() -> Done01Irq {
        Done01Irq(0)
    }
}
impl core::fmt::Debug for Done01Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Done01Irq")
            .field("trig0_done0", &self.trig0_done0())
            .field("trig1_done0", &self.trig1_done0())
            .field("trig2_done0", &self.trig2_done0())
            .field("trig3_done0", &self.trig3_done0())
            .field("trig4_done0", &self.trig4_done0())
            .field("trig5_done0", &self.trig5_done0())
            .field("trig6_done0", &self.trig6_done0())
            .field("trig7_done0", &self.trig7_done0())
            .field("trig0_done1", &self.trig0_done1())
            .field("trig1_done1", &self.trig1_done1())
            .field("trig2_done1", &self.trig2_done1())
            .field("trig3_done1", &self.trig3_done1())
            .field("trig4_done1", &self.trig4_done1())
            .field("trig5_done1", &self.trig5_done1())
            .field("trig6_done1", &self.trig6_done1())
            .field("trig7_done1", &self.trig7_done1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Done01Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Done01Irq {{ trig0_done0: {=bool:?}, trig1_done0: {=bool:?}, trig2_done0: {=bool:?}, trig3_done0: {=bool:?}, trig4_done0: {=bool:?}, trig5_done0: {=bool:?}, trig6_done0: {=bool:?}, trig7_done0: {=bool:?}, trig0_done1: {=bool:?}, trig1_done1: {=bool:?}, trig2_done1: {=bool:?}, trig3_done1: {=bool:?}, trig4_done1: {=bool:?}, trig5_done1: {=bool:?}, trig6_done1: {=bool:?}, trig7_done1: {=bool:?} }}",
            self.trig0_done0(),
            self.trig1_done0(),
            self.trig2_done0(),
            self.trig3_done0(),
            self.trig4_done0(),
            self.trig5_done0(),
            self.trig6_done0(),
            self.trig7_done0(),
            self.trig0_done1(),
            self.trig1_done1(),
            self.trig2_done1(),
            self.trig3_done1(),
            self.trig4_done1(),
            self.trig5_done1(),
            self.trig6_done1(),
            self.trig7_done1()
        )
    }
}
#[doc = "ETC DONE_2 and DONE_ERR IRQ State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done23ErrIrq(pub u32);
impl Done23ErrIrq {
    #[doc = "TRIG0 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_done2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRIG1 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_done2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "TRIG2 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_done2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TRIG3 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_done2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TRIG4 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_done2(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "TRIG5 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_done2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TRIG6 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_done2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "TRIG7 done2 interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_done2(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 done2 interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_done2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TRIG0 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig0_err(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG0 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig0_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRIG1 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig1_err(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG1 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig1_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TRIG2 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig2_err(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG2 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig2_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "TRIG3 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig3_err(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG3 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig3_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TRIG4 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig4_err(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG4 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig4_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "TRIG5 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig5_err(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG5 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig5_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "TRIG6 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig6_err(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG6 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig6_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TRIG7 error interrupt detection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig7_err(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TRIG7 error interrupt detection."]
    #[inline(always)]
    pub const fn set_trig7_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Done23ErrIrq {
    #[inline(always)]
    fn default() -> Done23ErrIrq {
        Done23ErrIrq(0)
    }
}
impl core::fmt::Debug for Done23ErrIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Done23ErrIrq")
            .field("trig0_done2", &self.trig0_done2())
            .field("trig1_done2", &self.trig1_done2())
            .field("trig2_done2", &self.trig2_done2())
            .field("trig3_done2", &self.trig3_done2())
            .field("trig4_done2", &self.trig4_done2())
            .field("trig5_done2", &self.trig5_done2())
            .field("trig6_done2", &self.trig6_done2())
            .field("trig7_done2", &self.trig7_done2())
            .field("trig0_err", &self.trig0_err())
            .field("trig1_err", &self.trig1_err())
            .field("trig2_err", &self.trig2_err())
            .field("trig3_err", &self.trig3_err())
            .field("trig4_err", &self.trig4_err())
            .field("trig5_err", &self.trig5_err())
            .field("trig6_err", &self.trig6_err())
            .field("trig7_err", &self.trig7_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Done23ErrIrq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Done23ErrIrq {{ trig0_done2: {=bool:?}, trig1_done2: {=bool:?}, trig2_done2: {=bool:?}, trig3_done2: {=bool:?}, trig4_done2: {=bool:?}, trig5_done2: {=bool:?}, trig6_done2: {=bool:?}, trig7_done2: {=bool:?}, trig0_err: {=bool:?}, trig1_err: {=bool:?}, trig2_err: {=bool:?}, trig3_err: {=bool:?}, trig4_err: {=bool:?}, trig5_err: {=bool:?}, trig6_err: {=bool:?}, trig7_err: {=bool:?} }}",
            self.trig0_done2(),
            self.trig1_done2(),
            self.trig2_done2(),
            self.trig3_done2(),
            self.trig4_done2(),
            self.trig5_done2(),
            self.trig6_done2(),
            self.trig7_done2(),
            self.trig0_err(),
            self.trig1_err(),
            self.trig2_err(),
            self.trig3_err(),
            self.trig4_err(),
            self.trig5_err(),
            self.trig6_err(),
            self.trig7_err()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Chain10(pub u32);
impl Trig0Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig0Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig0Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig0Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig0Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig0Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig0Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig0Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig0Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig0Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig0Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig0Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig0Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0Chain10 {
    #[inline(always)]
    fn default() -> Trig0Chain10 {
        Trig0Chain10(0)
    }
}
impl core::fmt::Debug for Trig0Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Chain32(pub u32);
impl Trig0Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig0Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig0Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig0Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig0Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig0Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig0Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig0Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig0Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig0Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig0Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig0Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig0Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0Chain32 {
    #[inline(always)]
    fn default() -> Trig0Chain32 {
        Trig0Chain32(0)
    }
}
impl core::fmt::Debug for Trig0Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Chain54(pub u32);
impl Trig0Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig0Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig0Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig0Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig0Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig0Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig0Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig0Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig0Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig0Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig0Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig0Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig0Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0Chain54 {
    #[inline(always)]
    fn default() -> Trig0Chain54 {
        Trig0Chain54(0)
    }
}
impl core::fmt::Debug for Trig0Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Chain76(pub u32);
impl Trig0Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig0Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig0Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig0Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig0Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig0Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig0Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig0Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig0Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig0Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig0Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig0Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig0Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig0Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig0Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig0Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig0Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig0Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig0Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig0Chain76 {
    #[inline(always)]
    fn default() -> Trig0Chain76 {
        Trig0Chain76(0)
    }
}
impl core::fmt::Debug for Trig0Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Counter(pub u32);
impl Trig0Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig0Counter {
    #[inline(always)]
    fn default() -> Trig0Counter {
        Trig0Counter(0)
    }
}
impl core::fmt::Debug for Trig0Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Ctrl(pub u32);
impl Trig0Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig0CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig0CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig0CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig0CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig0CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig0CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig0CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig0CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig0CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig0Ctrl {
    #[inline(always)]
    fn default() -> Trig0Ctrl {
        Trig0Ctrl(0)
    }
}
impl core::fmt::Debug for Trig0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Ctrl {{ sw_trig: {=bool:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Result10(pub u32);
impl Trig0Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0Result10 {
    #[inline(always)]
    fn default() -> Trig0Result10 {
        Trig0Result10(0)
    }
}
impl core::fmt::Debug for Trig0Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Result32(pub u32);
impl Trig0Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0Result32 {
    #[inline(always)]
    fn default() -> Trig0Result32 {
        Trig0Result32(0)
    }
}
impl core::fmt::Debug for Trig0Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Result54(pub u32);
impl Trig0Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0Result54 {
    #[inline(always)]
    fn default() -> Trig0Result54 {
        Trig0Result54(0)
    }
}
impl core::fmt::Debug for Trig0Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig0Result76(pub u32);
impl Trig0Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig0Result76 {
    #[inline(always)]
    fn default() -> Trig0Result76 {
        Trig0Result76(0)
    }
}
impl core::fmt::Debug for Trig0Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig0Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig0Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig0Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Chain10(pub u32);
impl Trig1Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig1Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig1Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig1Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig1Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig1Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig1Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig1Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig1Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig1Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig1Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig1Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig1Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1Chain10 {
    #[inline(always)]
    fn default() -> Trig1Chain10 {
        Trig1Chain10(0)
    }
}
impl core::fmt::Debug for Trig1Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Chain32(pub u32);
impl Trig1Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig1Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig1Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig1Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig1Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig1Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig1Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig1Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig1Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig1Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig1Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig1Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig1Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1Chain32 {
    #[inline(always)]
    fn default() -> Trig1Chain32 {
        Trig1Chain32(0)
    }
}
impl core::fmt::Debug for Trig1Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Chain54(pub u32);
impl Trig1Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig1Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig1Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig1Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig1Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig1Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig1Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig1Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig1Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig1Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig1Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig1Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig1Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1Chain54 {
    #[inline(always)]
    fn default() -> Trig1Chain54 {
        Trig1Chain54(0)
    }
}
impl core::fmt::Debug for Trig1Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Chain76(pub u32);
impl Trig1Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig1Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig1Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig1Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig1Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig1Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig1Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig1Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig1Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig1Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig1Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig1Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig1Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig1Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig1Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig1Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig1Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig1Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig1Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig1Chain76 {
    #[inline(always)]
    fn default() -> Trig1Chain76 {
        Trig1Chain76(0)
    }
}
impl core::fmt::Debug for Trig1Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Counter(pub u32);
impl Trig1Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig1Counter {
    #[inline(always)]
    fn default() -> Trig1Counter {
        Trig1Counter(0)
    }
}
impl core::fmt::Debug for Trig1Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Ctrl(pub u32);
impl Trig1Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig1CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig1CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig1CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig1CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig1CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig1CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig1CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig1CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig1CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig1Ctrl {
    #[inline(always)]
    fn default() -> Trig1Ctrl {
        Trig1Ctrl(0)
    }
}
impl core::fmt::Debug for Trig1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Ctrl {{ sw_trig: {=bool:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Result10(pub u32);
impl Trig1Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1Result10 {
    #[inline(always)]
    fn default() -> Trig1Result10 {
        Trig1Result10(0)
    }
}
impl core::fmt::Debug for Trig1Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Result32(pub u32);
impl Trig1Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1Result32 {
    #[inline(always)]
    fn default() -> Trig1Result32 {
        Trig1Result32(0)
    }
}
impl core::fmt::Debug for Trig1Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Result54(pub u32);
impl Trig1Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1Result54 {
    #[inline(always)]
    fn default() -> Trig1Result54 {
        Trig1Result54(0)
    }
}
impl core::fmt::Debug for Trig1Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig1Result76(pub u32);
impl Trig1Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig1Result76 {
    #[inline(always)]
    fn default() -> Trig1Result76 {
        Trig1Result76(0)
    }
}
impl core::fmt::Debug for Trig1Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig1Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig1Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig1Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Chain10(pub u32);
impl Trig2Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig2Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig2Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig2Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig2Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig2Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig2Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig2Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig2Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig2Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig2Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig2Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig2Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2Chain10 {
    #[inline(always)]
    fn default() -> Trig2Chain10 {
        Trig2Chain10(0)
    }
}
impl core::fmt::Debug for Trig2Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Chain32(pub u32);
impl Trig2Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig2Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig2Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig2Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig2Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig2Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig2Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig2Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig2Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig2Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig2Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig2Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig2Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2Chain32 {
    #[inline(always)]
    fn default() -> Trig2Chain32 {
        Trig2Chain32(0)
    }
}
impl core::fmt::Debug for Trig2Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Chain54(pub u32);
impl Trig2Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig2Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig2Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig2Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig2Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig2Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig2Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig2Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig2Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig2Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig2Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig2Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig2Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2Chain54 {
    #[inline(always)]
    fn default() -> Trig2Chain54 {
        Trig2Chain54(0)
    }
}
impl core::fmt::Debug for Trig2Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Chain76(pub u32);
impl Trig2Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig2Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig2Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig2Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig2Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig2Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig2Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig2Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig2Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig2Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig2Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig2Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig2Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig2Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig2Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig2Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig2Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig2Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig2Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig2Chain76 {
    #[inline(always)]
    fn default() -> Trig2Chain76 {
        Trig2Chain76(0)
    }
}
impl core::fmt::Debug for Trig2Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Counter(pub u32);
impl Trig2Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig2Counter {
    #[inline(always)]
    fn default() -> Trig2Counter {
        Trig2Counter(0)
    }
}
impl core::fmt::Debug for Trig2Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Ctrl(pub u32);
impl Trig2Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig2CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig2CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig2CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig2CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig2CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig2CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig2CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig2CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig2CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig2Ctrl {
    #[inline(always)]
    fn default() -> Trig2Ctrl {
        Trig2Ctrl(0)
    }
}
impl core::fmt::Debug for Trig2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Ctrl {{ sw_trig: {=bool:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Result10(pub u32);
impl Trig2Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2Result10 {
    #[inline(always)]
    fn default() -> Trig2Result10 {
        Trig2Result10(0)
    }
}
impl core::fmt::Debug for Trig2Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Result32(pub u32);
impl Trig2Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2Result32 {
    #[inline(always)]
    fn default() -> Trig2Result32 {
        Trig2Result32(0)
    }
}
impl core::fmt::Debug for Trig2Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Result54(pub u32);
impl Trig2Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2Result54 {
    #[inline(always)]
    fn default() -> Trig2Result54 {
        Trig2Result54(0)
    }
}
impl core::fmt::Debug for Trig2Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig2Result76(pub u32);
impl Trig2Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig2Result76 {
    #[inline(always)]
    fn default() -> Trig2Result76 {
        Trig2Result76(0)
    }
}
impl core::fmt::Debug for Trig2Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig2Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig2Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig2Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Chain10(pub u32);
impl Trig3Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig3Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig3Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig3Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig3Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig3Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig3Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig3Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig3Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig3Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig3Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig3Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig3Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3Chain10 {
    #[inline(always)]
    fn default() -> Trig3Chain10 {
        Trig3Chain10(0)
    }
}
impl core::fmt::Debug for Trig3Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {=bool:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {=bool:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Chain32(pub u32);
impl Trig3Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig3Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig3Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig3Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig3Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig3Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig3Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig3Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig3Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig3Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig3Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig3Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig3Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3Chain32 {
    #[inline(always)]
    fn default() -> Trig3Chain32 {
        Trig3Chain32(0)
    }
}
impl core::fmt::Debug for Trig3Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {=bool:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {=bool:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Chain54(pub u32);
impl Trig3Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig3Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig3Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig3Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig3Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig3Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig3Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig3Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig3Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig3Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig3Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig3Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig3Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3Chain54 {
    #[inline(always)]
    fn default() -> Trig3Chain54 {
        Trig3Chain54(0)
    }
}
impl core::fmt::Debug for Trig3Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {=bool:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {=bool:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Chain76(pub u32);
impl Trig3Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig3Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig3Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig3Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig3Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig3Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig3Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig3Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig3Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig3Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig3Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig3Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig3Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig3Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig3Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig3Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig3Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig3Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig3Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig3Chain76 {
    #[inline(always)]
    fn default() -> Trig3Chain76 {
        Trig3Chain76(0)
    }
}
impl core::fmt::Debug for Trig3Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {=bool:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {=bool:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Counter(pub u32);
impl Trig3Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig3Counter {
    #[inline(always)]
    fn default() -> Trig3Counter {
        Trig3Counter(0)
    }
}
impl core::fmt::Debug for Trig3Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Ctrl(pub u32);
impl Trig3Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig3CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig3CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig3CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig3CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig3CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig3CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig3CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig3CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig3CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig3Ctrl {
    #[inline(always)]
    fn default() -> Trig3Ctrl {
        Trig3Ctrl(0)
    }
}
impl core::fmt::Debug for Trig3Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Ctrl {{ sw_trig: {=bool:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Result10(pub u32);
impl Trig3Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3Result10 {
    #[inline(always)]
    fn default() -> Trig3Result10 {
        Trig3Result10(0)
    }
}
impl core::fmt::Debug for Trig3Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Result32(pub u32);
impl Trig3Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3Result32 {
    #[inline(always)]
    fn default() -> Trig3Result32 {
        Trig3Result32(0)
    }
}
impl core::fmt::Debug for Trig3Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Result54(pub u32);
impl Trig3Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3Result54 {
    #[inline(always)]
    fn default() -> Trig3Result54 {
        Trig3Result54(0)
    }
}
impl core::fmt::Debug for Trig3Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig3Result76(pub u32);
impl Trig3Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig3Result76 {
    #[inline(always)]
    fn default() -> Trig3Result76 {
        Trig3Result76(0)
    }
}
impl core::fmt::Debug for Trig3Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig3Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig3Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig3Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Chain10(pub u32);
impl Trig4Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig4Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig4Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig4Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig4Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> super::vals::Trig4Chain10B2b0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig4Chain10B2b0::from_bits(val as u8)
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: super::vals::Trig4Chain10B2b0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig4Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig4Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig4Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig4Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig4Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig4Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> super::vals::Trig4Chain10B2b1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig4Chain10B2b1::from_bits(val as u8)
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: super::vals::Trig4Chain10B2b1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig4Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig4Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4Chain10 {
    #[inline(always)]
    fn default() -> Trig4Chain10 {
        Trig4Chain10(0)
    }
}
impl core::fmt::Debug for Trig4Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Chain32(pub u32);
impl Trig4Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig4Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig4Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig4Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig4Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> super::vals::Trig4Chain32B2b2 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig4Chain32B2b2::from_bits(val as u8)
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: super::vals::Trig4Chain32B2b2) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig4Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig4Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig4Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig4Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig4Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig4Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> super::vals::Trig4Chain32B2b3 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig4Chain32B2b3::from_bits(val as u8)
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: super::vals::Trig4Chain32B2b3) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig4Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig4Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4Chain32 {
    #[inline(always)]
    fn default() -> Trig4Chain32 {
        Trig4Chain32(0)
    }
}
impl core::fmt::Debug for Trig4Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Chain54(pub u32);
impl Trig4Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig4Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig4Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig4Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig4Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> super::vals::Trig4Chain54B2b4 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig4Chain54B2b4::from_bits(val as u8)
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: super::vals::Trig4Chain54B2b4) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig4Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig4Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig4Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig4Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig4Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig4Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> super::vals::Trig4Chain54B2b5 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig4Chain54B2b5::from_bits(val as u8)
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: super::vals::Trig4Chain54B2b5) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig4Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig4Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4Chain54 {
    #[inline(always)]
    fn default() -> Trig4Chain54 {
        Trig4Chain54(0)
    }
}
impl core::fmt::Debug for Trig4Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Chain76(pub u32);
impl Trig4Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig4Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig4Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig4Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig4Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig4Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig4Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> super::vals::Trig4Chain76B2b6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig4Chain76B2b6::from_bits(val as u8)
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: super::vals::Trig4Chain76B2b6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig4Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig4Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig4Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig4Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig4Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig4Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig4Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig4Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig4Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> super::vals::Trig4Chain76B2b7 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig4Chain76B2b7::from_bits(val as u8)
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: super::vals::Trig4Chain76B2b7) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig4Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig4Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig4Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig4Chain76 {
    #[inline(always)]
    fn default() -> Trig4Chain76 {
        Trig4Chain76(0)
    }
}
impl core::fmt::Debug for Trig4Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Counter(pub u32);
impl Trig4Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig4Counter {
    #[inline(always)]
    fn default() -> Trig4Counter {
        Trig4Counter(0)
    }
}
impl core::fmt::Debug for Trig4Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Ctrl(pub u32);
impl Trig4Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> super::vals::Trig4CtrlSwTrig {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trig4CtrlSwTrig::from_bits(val as u8)
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: super::vals::Trig4CtrlSwTrig) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig4CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig4CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig4CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig4CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig4CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig4CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig4CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig4CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig4CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig4Ctrl {
    #[inline(always)]
    fn default() -> Trig4Ctrl {
        Trig4Ctrl(0)
    }
}
impl core::fmt::Debug for Trig4Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Ctrl {{ sw_trig: {:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Result10(pub u32);
impl Trig4Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4Result10 {
    #[inline(always)]
    fn default() -> Trig4Result10 {
        Trig4Result10(0)
    }
}
impl core::fmt::Debug for Trig4Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Result32(pub u32);
impl Trig4Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4Result32 {
    #[inline(always)]
    fn default() -> Trig4Result32 {
        Trig4Result32(0)
    }
}
impl core::fmt::Debug for Trig4Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Result54(pub u32);
impl Trig4Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4Result54 {
    #[inline(always)]
    fn default() -> Trig4Result54 {
        Trig4Result54(0)
    }
}
impl core::fmt::Debug for Trig4Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig4Result76(pub u32);
impl Trig4Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig4Result76 {
    #[inline(always)]
    fn default() -> Trig4Result76 {
        Trig4Result76(0)
    }
}
impl core::fmt::Debug for Trig4Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig4Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig4Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig4Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Chain10(pub u32);
impl Trig5Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig5Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig5Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig5Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig5Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> super::vals::Trig5Chain10B2b0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig5Chain10B2b0::from_bits(val as u8)
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: super::vals::Trig5Chain10B2b0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig5Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig5Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig5Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig5Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig5Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig5Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> super::vals::Trig5Chain10B2b1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig5Chain10B2b1::from_bits(val as u8)
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: super::vals::Trig5Chain10B2b1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig5Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig5Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5Chain10 {
    #[inline(always)]
    fn default() -> Trig5Chain10 {
        Trig5Chain10(0)
    }
}
impl core::fmt::Debug for Trig5Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Chain32(pub u32);
impl Trig5Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig5Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig5Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig5Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig5Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> super::vals::Trig5Chain32B2b2 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig5Chain32B2b2::from_bits(val as u8)
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: super::vals::Trig5Chain32B2b2) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig5Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig5Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig5Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig5Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig5Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig5Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> super::vals::Trig5Chain32B2b3 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig5Chain32B2b3::from_bits(val as u8)
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: super::vals::Trig5Chain32B2b3) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig5Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig5Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5Chain32 {
    #[inline(always)]
    fn default() -> Trig5Chain32 {
        Trig5Chain32(0)
    }
}
impl core::fmt::Debug for Trig5Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Chain54(pub u32);
impl Trig5Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig5Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig5Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig5Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig5Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> super::vals::Trig5Chain54B2b4 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig5Chain54B2b4::from_bits(val as u8)
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: super::vals::Trig5Chain54B2b4) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig5Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig5Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig5Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig5Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig5Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig5Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> super::vals::Trig5Chain54B2b5 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig5Chain54B2b5::from_bits(val as u8)
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: super::vals::Trig5Chain54B2b5) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig5Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig5Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5Chain54 {
    #[inline(always)]
    fn default() -> Trig5Chain54 {
        Trig5Chain54(0)
    }
}
impl core::fmt::Debug for Trig5Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Chain76(pub u32);
impl Trig5Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig5Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig5Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig5Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig5Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig5Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig5Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> super::vals::Trig5Chain76B2b6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig5Chain76B2b6::from_bits(val as u8)
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: super::vals::Trig5Chain76B2b6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig5Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig5Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig5Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig5Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig5Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig5Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig5Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig5Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig5Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> super::vals::Trig5Chain76B2b7 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig5Chain76B2b7::from_bits(val as u8)
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: super::vals::Trig5Chain76B2b7) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig5Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig5Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig5Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig5Chain76 {
    #[inline(always)]
    fn default() -> Trig5Chain76 {
        Trig5Chain76(0)
    }
}
impl core::fmt::Debug for Trig5Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Counter(pub u32);
impl Trig5Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig5Counter {
    #[inline(always)]
    fn default() -> Trig5Counter {
        Trig5Counter(0)
    }
}
impl core::fmt::Debug for Trig5Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Ctrl(pub u32);
impl Trig5Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> super::vals::Trig5CtrlSwTrig {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trig5CtrlSwTrig::from_bits(val as u8)
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: super::vals::Trig5CtrlSwTrig) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig5CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig5CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig5CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig5CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig5CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig5CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig5CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig5CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig5CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig5Ctrl {
    #[inline(always)]
    fn default() -> Trig5Ctrl {
        Trig5Ctrl(0)
    }
}
impl core::fmt::Debug for Trig5Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Ctrl {{ sw_trig: {:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Result10(pub u32);
impl Trig5Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5Result10 {
    #[inline(always)]
    fn default() -> Trig5Result10 {
        Trig5Result10(0)
    }
}
impl core::fmt::Debug for Trig5Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Result32(pub u32);
impl Trig5Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5Result32 {
    #[inline(always)]
    fn default() -> Trig5Result32 {
        Trig5Result32(0)
    }
}
impl core::fmt::Debug for Trig5Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Result54(pub u32);
impl Trig5Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5Result54 {
    #[inline(always)]
    fn default() -> Trig5Result54 {
        Trig5Result54(0)
    }
}
impl core::fmt::Debug for Trig5Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig5Result76(pub u32);
impl Trig5Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig5Result76 {
    #[inline(always)]
    fn default() -> Trig5Result76 {
        Trig5Result76(0)
    }
}
impl core::fmt::Debug for Trig5Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig5Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig5Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig5Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Chain10(pub u32);
impl Trig6Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig6Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig6Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig6Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig6Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> super::vals::Trig6Chain10B2b0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig6Chain10B2b0::from_bits(val as u8)
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: super::vals::Trig6Chain10B2b0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig6Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig6Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig6Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig6Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig6Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig6Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> super::vals::Trig6Chain10B2b1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig6Chain10B2b1::from_bits(val as u8)
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: super::vals::Trig6Chain10B2b1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig6Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig6Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6Chain10 {
    #[inline(always)]
    fn default() -> Trig6Chain10 {
        Trig6Chain10(0)
    }
}
impl core::fmt::Debug for Trig6Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Chain32(pub u32);
impl Trig6Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig6Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig6Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig6Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig6Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> super::vals::Trig6Chain32B2b2 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig6Chain32B2b2::from_bits(val as u8)
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: super::vals::Trig6Chain32B2b2) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig6Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig6Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig6Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig6Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig6Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig6Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> super::vals::Trig6Chain32B2b3 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig6Chain32B2b3::from_bits(val as u8)
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: super::vals::Trig6Chain32B2b3) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig6Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig6Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6Chain32 {
    #[inline(always)]
    fn default() -> Trig6Chain32 {
        Trig6Chain32(0)
    }
}
impl core::fmt::Debug for Trig6Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Chain54(pub u32);
impl Trig6Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig6Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig6Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig6Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig6Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> super::vals::Trig6Chain54B2b4 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig6Chain54B2b4::from_bits(val as u8)
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: super::vals::Trig6Chain54B2b4) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig6Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig6Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig6Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig6Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig6Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig6Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> super::vals::Trig6Chain54B2b5 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig6Chain54B2b5::from_bits(val as u8)
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: super::vals::Trig6Chain54B2b5) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig6Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig6Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6Chain54 {
    #[inline(always)]
    fn default() -> Trig6Chain54 {
        Trig6Chain54(0)
    }
}
impl core::fmt::Debug for Trig6Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Chain76(pub u32);
impl Trig6Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig6Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig6Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig6Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig6Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig6Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig6Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> super::vals::Trig6Chain76B2b6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig6Chain76B2b6::from_bits(val as u8)
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: super::vals::Trig6Chain76B2b6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig6Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig6Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig6Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig6Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig6Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig6Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig6Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig6Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig6Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> super::vals::Trig6Chain76B2b7 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig6Chain76B2b7::from_bits(val as u8)
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: super::vals::Trig6Chain76B2b7) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig6Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig6Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig6Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig6Chain76 {
    #[inline(always)]
    fn default() -> Trig6Chain76 {
        Trig6Chain76(0)
    }
}
impl core::fmt::Debug for Trig6Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Counter(pub u32);
impl Trig6Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig6Counter {
    #[inline(always)]
    fn default() -> Trig6Counter {
        Trig6Counter(0)
    }
}
impl core::fmt::Debug for Trig6Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Ctrl(pub u32);
impl Trig6Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> super::vals::Trig6CtrlSwTrig {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trig6CtrlSwTrig::from_bits(val as u8)
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: super::vals::Trig6CtrlSwTrig) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig6CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig6CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig6CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig6CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig6CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig6CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig6CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig6CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig6CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig6Ctrl {
    #[inline(always)]
    fn default() -> Trig6Ctrl {
        Trig6Ctrl(0)
    }
}
impl core::fmt::Debug for Trig6Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Ctrl {{ sw_trig: {:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Result10(pub u32);
impl Trig6Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6Result10 {
    #[inline(always)]
    fn default() -> Trig6Result10 {
        Trig6Result10(0)
    }
}
impl core::fmt::Debug for Trig6Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Result32(pub u32);
impl Trig6Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6Result32 {
    #[inline(always)]
    fn default() -> Trig6Result32 {
        Trig6Result32(0)
    }
}
impl core::fmt::Debug for Trig6Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Result54(pub u32);
impl Trig6Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6Result54 {
    #[inline(always)]
    fn default() -> Trig6Result54 {
        Trig6Result54(0)
    }
}
impl core::fmt::Debug for Trig6Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig6Result76(pub u32);
impl Trig6Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig6Result76 {
    #[inline(always)]
    fn default() -> Trig6Result76 {
        Trig6Result76(0)
    }
}
impl core::fmt::Debug for Trig6Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig6Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig6Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig6Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
#[doc = "ETC_TRIG Chain 0/1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Chain10(pub u32);
impl Trig7Chain10 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel0(&self) -> super::vals::Trig7Chain10Csel0 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7Chain10Csel0::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel0(&mut self, val: super::vals::Trig7Chain10Csel0) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts0(&self) -> super::vals::Trig7Chain10Hwts0 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7Chain10Hwts0::from_bits(val as u8)
    }
    #[doc = "Segment 0 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts0(&mut self, val: super::vals::Trig7Chain10Hwts0) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 0 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b0(&self) -> super::vals::Trig7Chain10B2b0 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig7Chain10B2b0::from_bits(val as u8)
    }
    #[doc = "Segment 0 B2B"]
    #[inline(always)]
    pub const fn set_b2b0(&mut self, val: super::vals::Trig7Chain10B2b0) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie0(&self) -> super::vals::Trig7Chain10Ie0 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7Chain10Ie0::from_bits(val as u8)
    }
    #[doc = "Segment 0 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie0(&mut self, val: super::vals::Trig7Chain10Ie0) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel1(&self) -> super::vals::Trig7Chain10Csel1 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7Chain10Csel1::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel1(&mut self, val: super::vals::Trig7Chain10Csel1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts1(&self) -> super::vals::Trig7Chain10Hwts1 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7Chain10Hwts1::from_bits(val as u8)
    }
    #[doc = "Segment 1 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts1(&mut self, val: super::vals::Trig7Chain10Hwts1) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 1 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b1(&self) -> super::vals::Trig7Chain10B2b1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig7Chain10B2b1::from_bits(val as u8)
    }
    #[doc = "Segment 1 B2B"]
    #[inline(always)]
    pub const fn set_b2b1(&mut self, val: super::vals::Trig7Chain10B2b1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie1(&self) -> super::vals::Trig7Chain10Ie1 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7Chain10Ie1::from_bits(val as u8)
    }
    #[doc = "Segment 1 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie1(&mut self, val: super::vals::Trig7Chain10Ie1) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7Chain10 {
    #[inline(always)]
    fn default() -> Trig7Chain10 {
        Trig7Chain10(0)
    }
}
impl core::fmt::Debug for Trig7Chain10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Chain10")
            .field("csel0", &self.csel0())
            .field("hwts0", &self.hwts0())
            .field("b2b0", &self.b2b0())
            .field("ie0", &self.ie0())
            .field("csel1", &self.csel1())
            .field("hwts1", &self.hwts1())
            .field("b2b1", &self.b2b1())
            .field("ie1", &self.ie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Chain10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Chain10 {{ csel0: {:?}, hwts0: {:?}, b2b0: {:?}, ie0: {:?}, csel1: {:?}, hwts1: {:?}, b2b1: {:?}, ie1: {:?} }}",
            self.csel0(),
            self.hwts0(),
            self.b2b0(),
            self.ie0(),
            self.csel1(),
            self.hwts1(),
            self.b2b1(),
            self.ie1()
        )
    }
}
#[doc = "ETC_TRIG Chain 2/3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Chain32(pub u32);
impl Trig7Chain32 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel2(&self) -> super::vals::Trig7Chain32Csel2 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7Chain32Csel2::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel2(&mut self, val: super::vals::Trig7Chain32Csel2) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts2(&self) -> super::vals::Trig7Chain32Hwts2 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7Chain32Hwts2::from_bits(val as u8)
    }
    #[doc = "Segment 2 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts2(&mut self, val: super::vals::Trig7Chain32Hwts2) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 2 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b2(&self) -> super::vals::Trig7Chain32B2b2 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig7Chain32B2b2::from_bits(val as u8)
    }
    #[doc = "Segment 2 B2B"]
    #[inline(always)]
    pub const fn set_b2b2(&mut self, val: super::vals::Trig7Chain32B2b2) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie2(&self) -> super::vals::Trig7Chain32Ie2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7Chain32Ie2::from_bits(val as u8)
    }
    #[doc = "Segment 2 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie2(&mut self, val: super::vals::Trig7Chain32Ie2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel3(&self) -> super::vals::Trig7Chain32Csel3 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7Chain32Csel3::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel3(&mut self, val: super::vals::Trig7Chain32Csel3) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts3(&self) -> super::vals::Trig7Chain32Hwts3 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7Chain32Hwts3::from_bits(val as u8)
    }
    #[doc = "Segment 3 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts3(&mut self, val: super::vals::Trig7Chain32Hwts3) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 3 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b3(&self) -> super::vals::Trig7Chain32B2b3 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig7Chain32B2b3::from_bits(val as u8)
    }
    #[doc = "Segment 3 B2B"]
    #[inline(always)]
    pub const fn set_b2b3(&mut self, val: super::vals::Trig7Chain32B2b3) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie3(&self) -> super::vals::Trig7Chain32Ie3 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7Chain32Ie3::from_bits(val as u8)
    }
    #[doc = "Segment 3 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie3(&mut self, val: super::vals::Trig7Chain32Ie3) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7Chain32 {
    #[inline(always)]
    fn default() -> Trig7Chain32 {
        Trig7Chain32(0)
    }
}
impl core::fmt::Debug for Trig7Chain32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Chain32")
            .field("csel2", &self.csel2())
            .field("hwts2", &self.hwts2())
            .field("b2b2", &self.b2b2())
            .field("ie2", &self.ie2())
            .field("csel3", &self.csel3())
            .field("hwts3", &self.hwts3())
            .field("b2b3", &self.b2b3())
            .field("ie3", &self.ie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Chain32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Chain32 {{ csel2: {:?}, hwts2: {:?}, b2b2: {:?}, ie2: {:?}, csel3: {:?}, hwts3: {:?}, b2b3: {:?}, ie3: {:?} }}",
            self.csel2(),
            self.hwts2(),
            self.b2b2(),
            self.ie2(),
            self.csel3(),
            self.hwts3(),
            self.b2b3(),
            self.ie3()
        )
    }
}
#[doc = "ETC_TRIG Chain 4/5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Chain54(pub u32);
impl Trig7Chain54 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel4(&self) -> super::vals::Trig7Chain54Csel4 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7Chain54Csel4::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel4(&mut self, val: super::vals::Trig7Chain54Csel4) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts4(&self) -> super::vals::Trig7Chain54Hwts4 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7Chain54Hwts4::from_bits(val as u8)
    }
    #[doc = "Segment 4 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts4(&mut self, val: super::vals::Trig7Chain54Hwts4) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 4 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b4(&self) -> super::vals::Trig7Chain54B2b4 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig7Chain54B2b4::from_bits(val as u8)
    }
    #[doc = "Segment 4 B2B"]
    #[inline(always)]
    pub const fn set_b2b4(&mut self, val: super::vals::Trig7Chain54B2b4) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie4(&self) -> super::vals::Trig7Chain54Ie4 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7Chain54Ie4::from_bits(val as u8)
    }
    #[doc = "Segment 4 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie4(&mut self, val: super::vals::Trig7Chain54Ie4) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel5(&self) -> super::vals::Trig7Chain54Csel5 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7Chain54Csel5::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel5(&mut self, val: super::vals::Trig7Chain54Csel5) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts5(&self) -> super::vals::Trig7Chain54Hwts5 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7Chain54Hwts5::from_bits(val as u8)
    }
    #[doc = "Segment 5 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts5(&mut self, val: super::vals::Trig7Chain54Hwts5) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 5 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b5(&self) -> super::vals::Trig7Chain54B2b5 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig7Chain54B2b5::from_bits(val as u8)
    }
    #[doc = "Segment 5 B2B"]
    #[inline(always)]
    pub const fn set_b2b5(&mut self, val: super::vals::Trig7Chain54B2b5) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie5(&self) -> super::vals::Trig7Chain54Ie5 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7Chain54Ie5::from_bits(val as u8)
    }
    #[doc = "Segment 5 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie5(&mut self, val: super::vals::Trig7Chain54Ie5) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7Chain54 {
    #[inline(always)]
    fn default() -> Trig7Chain54 {
        Trig7Chain54(0)
    }
}
impl core::fmt::Debug for Trig7Chain54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Chain54")
            .field("csel4", &self.csel4())
            .field("hwts4", &self.hwts4())
            .field("b2b4", &self.b2b4())
            .field("ie4", &self.ie4())
            .field("csel5", &self.csel5())
            .field("hwts5", &self.hwts5())
            .field("b2b5", &self.b2b5())
            .field("ie5", &self.ie5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Chain54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Chain54 {{ csel4: {:?}, hwts4: {:?}, b2b4: {:?}, ie4: {:?}, csel5: {:?}, hwts5: {:?}, b2b5: {:?}, ie5: {:?} }}",
            self.csel4(),
            self.hwts4(),
            self.b2b4(),
            self.ie4(),
            self.csel5(),
            self.hwts5(),
            self.b2b5(),
            self.ie5()
        )
    }
}
#[doc = "ETC_TRIG Chain 6/7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Chain76(pub u32);
impl Trig7Chain76 {
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel6(&self) -> super::vals::Trig7Chain76Csel6 {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Trig7Chain76Csel6::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel6(&mut self, val: super::vals::Trig7Chain76Csel6) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts6(&self) -> super::vals::Trig7Chain76Hwts6 {
        let val = (self.0 >> 4usize) & 0xff;
        super::vals::Trig7Chain76Hwts6::from_bits(val as u8)
    }
    #[doc = "Segment 6 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts6(&mut self, val: super::vals::Trig7Chain76Hwts6) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val.to_bits() as u32) & 0xff) << 4usize);
    }
    #[doc = "Segment 6 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b6(&self) -> super::vals::Trig7Chain76B2b6 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Trig7Chain76B2b6::from_bits(val as u8)
    }
    #[doc = "Segment 6 B2B"]
    #[inline(always)]
    pub const fn set_b2b6(&mut self, val: super::vals::Trig7Chain76B2b6) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie6(&self) -> super::vals::Trig7Chain76Ie6 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Trig7Chain76Ie6::from_bits(val as u8)
    }
    #[doc = "Segment 6 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie6(&mut self, val: super::vals::Trig7Chain76Ie6) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "ADC channel selection"]
    #[must_use]
    #[inline(always)]
    pub const fn csel7(&self) -> super::vals::Trig7Chain76Csel7 {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trig7Chain76Csel7::from_bits(val as u8)
    }
    #[doc = "ADC channel selection"]
    #[inline(always)]
    pub const fn set_csel7(&mut self, val: super::vals::Trig7Chain76Csel7) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[must_use]
    #[inline(always)]
    pub const fn hwts7(&self) -> super::vals::Trig7Chain76Hwts7 {
        let val = (self.0 >> 20usize) & 0xff;
        super::vals::Trig7Chain76Hwts7::from_bits(val as u8)
    }
    #[doc = "Segment 7 HWTS ADC hardware trigger selection"]
    #[inline(always)]
    pub const fn set_hwts7(&mut self, val: super::vals::Trig7Chain76Hwts7) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
    }
    #[doc = "Segment 7 B2B"]
    #[must_use]
    #[inline(always)]
    pub const fn b2b7(&self) -> super::vals::Trig7Chain76B2b7 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Trig7Chain76B2b7::from_bits(val as u8)
    }
    #[doc = "Segment 7 B2B"]
    #[inline(always)]
    pub const fn set_b2b7(&mut self, val: super::vals::Trig7Chain76B2b7) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ie7(&self) -> super::vals::Trig7Chain76Ie7 {
        let val = (self.0 >> 29usize) & 0x03;
        super::vals::Trig7Chain76Ie7::from_bits(val as u8)
    }
    #[doc = "Segment 7 done interrupt selection"]
    #[inline(always)]
    pub const fn set_ie7(&mut self, val: super::vals::Trig7Chain76Ie7) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val.to_bits() as u32) & 0x03) << 29usize);
    }
}
impl Default for Trig7Chain76 {
    #[inline(always)]
    fn default() -> Trig7Chain76 {
        Trig7Chain76(0)
    }
}
impl core::fmt::Debug for Trig7Chain76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Chain76")
            .field("csel6", &self.csel6())
            .field("hwts6", &self.hwts6())
            .field("b2b6", &self.b2b6())
            .field("ie6", &self.ie6())
            .field("csel7", &self.csel7())
            .field("hwts7", &self.hwts7())
            .field("b2b7", &self.b2b7())
            .field("ie7", &self.ie7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Chain76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Chain76 {{ csel6: {:?}, hwts6: {:?}, b2b6: {:?}, ie6: {:?}, csel7: {:?}, hwts7: {:?}, b2b7: {:?}, ie7: {:?} }}",
            self.csel6(),
            self.hwts6(),
            self.b2b6(),
            self.ie6(),
            self.csel7(),
            self.hwts7(),
            self.b2b7(),
            self.ie7()
        )
    }
}
#[doc = "ETC_TRIG Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Counter(pub u32);
impl Trig7Counter {
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[must_use]
    #[inline(always)]
    pub const fn init_delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub const fn set_init_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "TRIGGER sampling interval counter"]
    #[inline(always)]
    pub const fn set_sample_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trig7Counter {
    #[inline(always)]
    fn default() -> Trig7Counter {
        Trig7Counter(0)
    }
}
impl core::fmt::Debug for Trig7Counter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Counter")
            .field("init_delay", &self.init_delay())
            .field("sample_interval", &self.sample_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Counter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Counter {{ init_delay: {=u16:?}, sample_interval: {=u16:?} }}",
            self.init_delay(),
            self.sample_interval()
        )
    }
}
#[doc = "ETC_TRIG Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Ctrl(pub u32);
impl Trig7Ctrl {
    #[doc = "Software trigger. This field is self-clearing."]
    #[must_use]
    #[inline(always)]
    pub const fn sw_trig(&self) -> super::vals::Trig7CtrlSwTrig {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trig7CtrlSwTrig::from_bits(val as u8)
    }
    #[doc = "Software trigger. This field is self-clearing."]
    #[inline(always)]
    pub const fn set_sw_trig(&mut self, val: super::vals::Trig7CtrlSwTrig) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_mode(&self) -> super::vals::Trig7CtrlTrigMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trig7CtrlTrigMode::from_bits(val as u8)
    }
    #[doc = "Trigger mode selection."]
    #[inline(always)]
    pub const fn set_trig_mode(&mut self, val: super::vals::Trig7CtrlTrigMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[must_use]
    #[inline(always)]
    pub const fn trig_chain(&self) -> super::vals::Trig7CtrlTrigChain {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Trig7CtrlTrigChain::from_bits(val as u8)
    }
    #[doc = "The number of segments inside the trigger chain of TRIGa."]
    #[inline(always)]
    pub const fn set_trig_chain(&mut self, val: super::vals::Trig7CtrlTrigChain) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[must_use]
    #[inline(always)]
    pub const fn trig_priority(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "External trigger priority, 7 is highest priority, while 0 is lowest"]
    #[inline(always)]
    pub const fn set_trig_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Trigger synchronization mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_mode(&self) -> super::vals::Trig7CtrlSyncMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Trig7CtrlSyncMode::from_bits(val as u8)
    }
    #[doc = "Trigger synchronization mode selection"]
    #[inline(always)]
    pub const fn set_sync_mode(&mut self, val: super::vals::Trig7CtrlSyncMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Trig7Ctrl {
    #[inline(always)]
    fn default() -> Trig7Ctrl {
        Trig7Ctrl(0)
    }
}
impl core::fmt::Debug for Trig7Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Ctrl")
            .field("sw_trig", &self.sw_trig())
            .field("trig_mode", &self.trig_mode())
            .field("trig_chain", &self.trig_chain())
            .field("trig_priority", &self.trig_priority())
            .field("sync_mode", &self.sync_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Ctrl {{ sw_trig: {:?}, trig_mode: {:?}, trig_chain: {:?}, trig_priority: {=u8:?}, sync_mode: {:?} }}",
            self.sw_trig(),
            self.trig_mode(),
            self.trig_chain(),
            self.trig_priority(),
            self.sync_mode()
        )
    }
}
#[doc = "ETC_TRIG Result Data 1/0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Result10(pub u32);
impl Trig7Result10 {
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA0The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA1The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7Result10 {
    #[inline(always)]
    fn default() -> Trig7Result10 {
        Trig7Result10(0)
    }
}
impl core::fmt::Debug for Trig7Result10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Result10")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Result10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Result10 {{ data0: {=u16:?}, data1: {=u16:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "ETC_TRIG Result Data 3/2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Result32(pub u32);
impl Trig7Result32 {
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA2The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA3The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7Result32 {
    #[inline(always)]
    fn default() -> Trig7Result32 {
        Trig7Result32(0)
    }
}
impl core::fmt::Debug for Trig7Result32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Result32")
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Result32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Result32 {{ data2: {=u16:?}, data3: {=u16:?} }}",
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "ETC_TRIG Result Data 5/4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Result54(pub u32);
impl Trig7Result54 {
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA4The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data5(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA5The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7Result54 {
    #[inline(always)]
    fn default() -> Trig7Result54 {
        Trig7Result54(0)
    }
}
impl core::fmt::Debug for Trig7Result54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Result54")
            .field("data4", &self.data4())
            .field("data5", &self.data5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Result54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Result54 {{ data4: {=u16:?}, data5: {=u16:?} }}",
            self.data4(),
            self.data5()
        )
    }
}
#[doc = "ETC_TRIG Result Data 7/6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig7Result76(pub u32);
impl Trig7Result76 {
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA6The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[must_use]
    #[inline(always)]
    pub const fn data7(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Result DATA7The sign bit from ADC result FIFO is ignored by ETC_TRIG result, so only 12-bit unsigned results is supported by ADC_ETC module"]
    #[inline(always)]
    pub const fn set_data7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Trig7Result76 {
    #[inline(always)]
    fn default() -> Trig7Result76 {
        Trig7Result76(0)
    }
}
impl core::fmt::Debug for Trig7Result76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig7Result76")
            .field("data6", &self.data6())
            .field("data7", &self.data7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig7Result76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Trig7Result76 {{ data6: {=u16:?}, data7: {=u16:?} }}",
            self.data6(),
            self.data7()
        )
    }
}
