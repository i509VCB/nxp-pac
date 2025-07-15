#[doc = "Trigger Source IN0 to IN15 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel(pub u32);
impl OutSel {
    #[doc = "Selects digital glitch detector as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in0_seln(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Selects digital glitch detector as a trigger source."]
    #[inline(always)]
    pub const fn set_in0_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects TDET event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in1_seln(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Selects TDET event as a trigger source."]
    #[inline(always)]
    pub const fn set_in1_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects Code Watchdog 0 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in2_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Code Watchdog 0 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in2_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects VBAT voltage tamper event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in3_seln(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VBAT voltage tamper event as a trigger source."]
    #[inline(always)]
    pub const fn set_in3_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Selects low-voltage event on VDD_CORE rail as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in4_seln(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Selects low-voltage event on VDD_CORE rail as a trigger source."]
    #[inline(always)]
    pub const fn set_in4_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects Watchdog 0 timer event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in5_seln(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Watchdog 0 timer event as a trigger source."]
    #[inline(always)]
    pub const fn set_in5_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects Flash ECC mismatch event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in6_seln(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Flash ECC mismatch event as a trigger source."]
    #[inline(always)]
    pub const fn set_in6_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Selects AHB secure bus or MBC bus illegal access event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in7_seln(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Selects AHB secure bus or MBC bus illegal access event as a trigger source."]
    #[inline(always)]
    pub const fn set_in7_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Selects ELS error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in8_seln(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Selects ELS error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in8_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects SPC VDD_CORE glitch detector as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in9_seln(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SPC VDD_CORE glitch detector as a trigger source."]
    #[inline(always)]
    pub const fn set_in9_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Selects PKC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in10_seln(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Selects PKC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in10_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Selects Code Watchdog 1 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in11_seln(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Code Watchdog 1 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in11_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Selects Watchdog 1 timer event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in12_seln(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Selects Watchdog 1 timer event as a trigger source."]
    #[inline(always)]
    pub const fn set_in12_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Selects FREQME out of range status output as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in13_seln(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Selects FREQME out of range status output as a trigger source."]
    #[inline(always)]
    pub const fn set_in13_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Selects software event 0 as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in14_seln(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Selects software event 0 as a trigger source."]
    #[inline(always)]
    pub const fn set_in14_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Selects software event 1 as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in15_seln(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Selects software event 1 as a trigger source."]
    #[inline(always)]
    pub const fn set_in15_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OutSel {
    #[inline(always)]
    fn default() -> OutSel {
        OutSel(0)
    }
}
impl core::fmt::Debug for OutSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel")
            .field("in0_seln", &self.in0_seln())
            .field("in1_seln", &self.in1_seln())
            .field("in2_seln", &self.in2_seln())
            .field("in3_seln", &self.in3_seln())
            .field("in4_seln", &self.in4_seln())
            .field("in5_seln", &self.in5_seln())
            .field("in6_seln", &self.in6_seln())
            .field("in7_seln", &self.in7_seln())
            .field("in8_seln", &self.in8_seln())
            .field("in9_seln", &self.in9_seln())
            .field("in10_seln", &self.in10_seln())
            .field("in11_seln", &self.in11_seln())
            .field("in12_seln", &self.in12_seln())
            .field("in13_seln", &self.in13_seln())
            .field("in14_seln", &self.in14_seln())
            .field("in15_seln", &self.in15_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel {{ in0_seln: {=u8:?}, in1_seln: {=u8:?}, in2_seln: {=u8:?}, in3_seln: {=u8:?}, in4_seln: {=u8:?}, in5_seln: {=u8:?}, in6_seln: {=u8:?}, in7_seln: {=u8:?}, in8_seln: {=u8:?}, in9_seln: {=u8:?}, in10_seln: {=u8:?}, in11_seln: {=u8:?}, in12_seln: {=u8:?}, in13_seln: {=u8:?}, in14_seln: {=u8:?}, in15_seln: {=u8:?} }}",
            self.in0_seln(),
            self.in1_seln(),
            self.in2_seln(),
            self.in3_seln(),
            self.in4_seln(),
            self.in5_seln(),
            self.in6_seln(),
            self.in7_seln(),
            self.in8_seln(),
            self.in9_seln(),
            self.in10_seln(),
            self.in11_seln(),
            self.in12_seln(),
            self.in13_seln(),
            self.in14_seln(),
            self.in15_seln()
        )
    }
}
#[doc = "Trigger Source IN16 to IN31 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel1(pub u32);
impl OutSel1 {
    #[doc = "Selects SPC VDD_SYS_LVD detect as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in16_seln(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SPC VDD_SYS_LVD detect as a trigger source."]
    #[inline(always)]
    pub const fn set_in16_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects SPC VDD_IO_LVD detect as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in17_seln(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SPC VDD_IO_LVD detect as a trigger source."]
    #[inline(always)]
    pub const fn set_in17_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn in18_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_in18_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects VBAT temperature tamper output event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in19_seln(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VBAT temperature tamper output event as a trigger source."]
    #[inline(always)]
    pub const fn set_in19_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Selects VBAT clock tamper output event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in20_seln(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VBAT clock tamper output event as a trigger source."]
    #[inline(always)]
    pub const fn set_in20_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects INTM interrupt monitor error 0 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in21_seln(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Selects INTM interrupt monitor error 0 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in21_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects INTM interrupt monitor error 1 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in22_seln(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Selects INTM interrupt monitor error 1 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in22_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Selects INTM interrupt monitor error 2 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in23_seln(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Selects INTM interrupt monitor error 2 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in23_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Selects INTM interrupt monitor error 3 event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in24_seln(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Selects INTM interrupt monitor error 3 event as a trigger source."]
    #[inline(always)]
    pub const fn set_in24_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Selects MSF SOCTRIM 0 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in25_seln(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 0 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in25_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Selects MSF SOCTRIM 1 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in26_seln(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 1 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in26_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Selects MSF SOCTRIM 2 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in27_seln(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 2 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in27_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Selects MSF SOCTRIM 3 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in28_seln(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 3 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in28_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Selects MSF SOCTRIM 4 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in29_seln(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 4 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in29_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Selects MSF SOCTRIM 5 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in30_seln(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 5 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in30_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Selects MSF SOCTRIM 6 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in31_seln(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 6 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in31_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OutSel1 {
    #[inline(always)]
    fn default() -> OutSel1 {
        OutSel1(0)
    }
}
impl core::fmt::Debug for OutSel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel1")
            .field("in16_seln", &self.in16_seln())
            .field("in17_seln", &self.in17_seln())
            .field("in18_seln", &self.in18_seln())
            .field("in19_seln", &self.in19_seln())
            .field("in20_seln", &self.in20_seln())
            .field("in21_seln", &self.in21_seln())
            .field("in22_seln", &self.in22_seln())
            .field("in23_seln", &self.in23_seln())
            .field("in24_seln", &self.in24_seln())
            .field("in25_seln", &self.in25_seln())
            .field("in26_seln", &self.in26_seln())
            .field("in27_seln", &self.in27_seln())
            .field("in28_seln", &self.in28_seln())
            .field("in29_seln", &self.in29_seln())
            .field("in30_seln", &self.in30_seln())
            .field("in31_seln", &self.in31_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel1 {{ in16_seln: {=u8:?}, in17_seln: {=u8:?}, in18_seln: {=u8:?}, in19_seln: {=u8:?}, in20_seln: {=u8:?}, in21_seln: {=u8:?}, in22_seln: {=u8:?}, in23_seln: {=u8:?}, in24_seln: {=u8:?}, in25_seln: {=u8:?}, in26_seln: {=u8:?}, in27_seln: {=u8:?}, in28_seln: {=u8:?}, in29_seln: {=u8:?}, in30_seln: {=u8:?}, in31_seln: {=u8:?} }}",
            self.in16_seln(),
            self.in17_seln(),
            self.in18_seln(),
            self.in19_seln(),
            self.in20_seln(),
            self.in21_seln(),
            self.in22_seln(),
            self.in23_seln(),
            self.in24_seln(),
            self.in25_seln(),
            self.in26_seln(),
            self.in27_seln(),
            self.in28_seln(),
            self.in29_seln(),
            self.in30_seln(),
            self.in31_seln()
        )
    }
}
#[doc = "Trigger source IN32 to IN47 selector"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSel2(pub u32);
impl OutSel2 {
    #[doc = "Selects MSF SOCTRIM 7 ECC error event as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in32_seln(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Selects MSF SOCTRIM 7 ECC error event as a trigger source."]
    #[inline(always)]
    pub const fn set_in32_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects GDET0 & 1 SFR error detect as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in33_seln(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Selects GDET0 & 1 SFR error detect as a trigger source."]
    #[inline(always)]
    pub const fn set_in33_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Selects SPC VDD_CORE_HVD as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in34_seln(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SPC VDD_CORE_HVD as a trigger source."]
    #[inline(always)]
    pub const fn set_in34_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Selects VDD_SYS_HVD as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in35_seln(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VDD_SYS_HVD as a trigger source."]
    #[inline(always)]
    pub const fn set_in35_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Selects VDD_IO_HVD as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in36_seln(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Selects VDD_IO_HVD as a trigger source."]
    #[inline(always)]
    pub const fn set_in36_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Selects FLEXSPI GCM error as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in37_seln(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Selects FLEXSPI GCM error as a trigger source."]
    #[inline(always)]
    pub const fn set_in37_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects SM3 SGI error as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in46_seln(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Selects SM3 SGI error as a trigger source."]
    #[inline(always)]
    pub const fn set_in46_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Selects TRNG HW Error as a trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn in47_seln(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Selects TRNG HW Error as a trigger source."]
    #[inline(always)]
    pub const fn set_in47_seln(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for OutSel2 {
    #[inline(always)]
    fn default() -> OutSel2 {
        OutSel2(0)
    }
}
impl core::fmt::Debug for OutSel2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSel2")
            .field("in32_seln", &self.in32_seln())
            .field("in33_seln", &self.in33_seln())
            .field("in34_seln", &self.in34_seln())
            .field("in35_seln", &self.in35_seln())
            .field("in36_seln", &self.in36_seln())
            .field("in37_seln", &self.in37_seln())
            .field("in46_seln", &self.in46_seln())
            .field("in47_seln", &self.in47_seln())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSel2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSel2 {{ in32_seln: {=u8:?}, in33_seln: {=u8:?}, in34_seln: {=u8:?}, in35_seln: {=u8:?}, in36_seln: {=u8:?}, in37_seln: {=u8:?}, in46_seln: {=u8:?}, in47_seln: {=u8:?} }}",
            self.in32_seln(),
            self.in33_seln(),
            self.in34_seln(),
            self.in35_seln(),
            self.in36_seln(),
            self.in37_seln(),
            self.in46_seln(),
            self.in47_seln()
        )
    }
}
#[doc = "ITRC outputs and IN0 to IN15 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "GDET0 & 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in0_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET0 & 1 interrupt."]
    #[inline(always)]
    pub const fn set_in0_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TDET tamper output."]
    #[must_use]
    #[inline(always)]
    pub const fn in1_status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TDET tamper output."]
    #[inline(always)]
    pub const fn set_in1_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Code Watchdog 0 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in2_status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 interrupt."]
    #[inline(always)]
    pub const fn set_in2_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBAT volt tamper output."]
    #[must_use]
    #[inline(always)]
    pub const fn in3_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT volt tamper output."]
    #[inline(always)]
    pub const fn set_in3_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SPC VDD_CORE_LVD detect."]
    #[must_use]
    #[inline(always)]
    pub const fn in4_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE_LVD detect."]
    #[inline(always)]
    pub const fn set_in4_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Watch Dog timer event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in5_status(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Watch Dog timer event occurred."]
    #[inline(always)]
    pub const fn set_in5_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Flash ECC mismatch event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in6_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Flash ECC mismatch event occurred."]
    #[inline(always)]
    pub const fn set_in6_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB secure bus checkers detected illegal access."]
    #[must_use]
    #[inline(always)]
    pub const fn in7_status(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "AHB secure bus checkers detected illegal access."]
    #[inline(always)]
    pub const fn set_in7_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "ELS error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in8_status(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ELS error event occurred."]
    #[inline(always)]
    pub const fn set_in8_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SPC VDD_CORE glitch detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in9_status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE glitch detect event occurred."]
    #[inline(always)]
    pub const fn set_in9_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "PKC module detected an error event."]
    #[must_use]
    #[inline(always)]
    pub const fn in10_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PKC module detected an error event."]
    #[inline(always)]
    pub const fn set_in10_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Code Watchdog 1 interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in11_status(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 interrupt."]
    #[inline(always)]
    pub const fn set_in11_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog 1 timer event interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn in112_status(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog 1 timer event interrupt."]
    #[inline(always)]
    pub const fn set_in112_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FREQME out of range status output."]
    #[must_use]
    #[inline(always)]
    pub const fn in113_status(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "FREQME out of range status output."]
    #[inline(always)]
    pub const fn set_in113_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software event 0 occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in14_status(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software event 0 occurred."]
    #[inline(always)]
    pub const fn set_in14_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Software event 1 occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in15_status(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Software event 1 occurred."]
    #[inline(always)]
    pub const fn set_in15_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ITRC triggered ITRC_IRQ output."]
    #[must_use]
    #[inline(always)]
    pub const fn out0_status(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered ITRC_IRQ output."]
    #[inline(always)]
    pub const fn set_out0_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ITRC triggered ELS_RESET to clear ELS key store."]
    #[must_use]
    #[inline(always)]
    pub const fn out1_status(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered ELS_RESET to clear ELS key store."]
    #[inline(always)]
    pub const fn set_out1_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "ITRC triggered PUF_ZEROIZE to clear PUF key store and RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn out2_status(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered PUF_ZEROIZE to clear PUF key store and RAM."]
    #[inline(always)]
    pub const fn set_out2_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ITRC triggered RAM_ZEROIZE."]
    #[must_use]
    #[inline(always)]
    pub const fn out3_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered RAM_ZEROIZE."]
    #[inline(always)]
    pub const fn set_out3_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ITRC triggered CHIP_RESET to reset the chip after all other response process finished."]
    #[must_use]
    #[inline(always)]
    pub const fn out4_status(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered CHIP_RESET to reset the chip after all other response process finished."]
    #[inline(always)]
    pub const fn set_out4_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ITRC triggered TMPR_OUT0 internal signal connected to various on-chip multiplexers."]
    #[must_use]
    #[inline(always)]
    pub const fn out5_status(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered TMPR_OUT0 internal signal connected to various on-chip multiplexers."]
    #[inline(always)]
    pub const fn set_out5_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "ITRC triggered TMPR_OUT1 internal signal connected to various on-chip multiplexers."]
    #[must_use]
    #[inline(always)]
    pub const fn out6_status(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC triggered TMPR_OUT1 internal signal connected to various on-chip multiplexers."]
    #[inline(always)]
    pub const fn set_out6_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            .field("in0_status", &self.in0_status())
            .field("in1_status", &self.in1_status())
            .field("in2_status", &self.in2_status())
            .field("in3_status", &self.in3_status())
            .field("in4_status", &self.in4_status())
            .field("in5_status", &self.in5_status())
            .field("in6_status", &self.in6_status())
            .field("in7_status", &self.in7_status())
            .field("in8_status", &self.in8_status())
            .field("in9_status", &self.in9_status())
            .field("in10_status", &self.in10_status())
            .field("in11_status", &self.in11_status())
            .field("in112_status", &self.in112_status())
            .field("in113_status", &self.in113_status())
            .field("in14_status", &self.in14_status())
            .field("in15_status", &self.in15_status())
            .field("out0_status", &self.out0_status())
            .field("out1_status", &self.out1_status())
            .field("out2_status", &self.out2_status())
            .field("out3_status", &self.out3_status())
            .field("out4_status", &self.out4_status())
            .field("out5_status", &self.out5_status())
            .field("out6_status", &self.out6_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ in0_status: {=bool:?}, in1_status: {=bool:?}, in2_status: {=bool:?}, in3_status: {=bool:?}, in4_status: {=bool:?}, in5_status: {=bool:?}, in6_status: {=bool:?}, in7_status: {=bool:?}, in8_status: {=bool:?}, in9_status: {=bool:?}, in10_status: {=bool:?}, in11_status: {=bool:?}, in112_status: {=bool:?}, in113_status: {=bool:?}, in14_status: {=bool:?}, in15_status: {=bool:?}, out0_status: {=bool:?}, out1_status: {=bool:?}, out2_status: {=bool:?}, out3_status: {=bool:?}, out4_status: {=bool:?}, out5_status: {=bool:?}, out6_status: {=bool:?} }}",
            self.in0_status(),
            self.in1_status(),
            self.in2_status(),
            self.in3_status(),
            self.in4_status(),
            self.in5_status(),
            self.in6_status(),
            self.in7_status(),
            self.in8_status(),
            self.in9_status(),
            self.in10_status(),
            self.in11_status(),
            self.in112_status(),
            self.in113_status(),
            self.in14_status(),
            self.in15_status(),
            self.out0_status(),
            self.out1_status(),
            self.out2_status(),
            self.out3_status(),
            self.out4_status(),
            self.out5_status(),
            self.out6_status()
        )
    }
}
#[doc = "ITRC IN16 to IN47 Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1(pub u32);
impl Status1 {
    #[doc = "SSPC VDD_SYS_LVD detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in16_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSPC VDD_SYS_LVD detect event occurred."]
    #[inline(always)]
    pub const fn set_in16_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SPC VDD_IO_LVD detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in17_status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_IO_LVD detect event occurred."]
    #[inline(always)]
    pub const fn set_in17_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn in18_status(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_in18_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn in19_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_in19_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBAT clock tamper output event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in20_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT clock tamper output event occurred."]
    #[inline(always)]
    pub const fn set_in20_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "INTM interrupt monitor error 3~0 event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in24_21_status(&self) -> super::vals::In2421Status {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::In2421Status::from_bits(val as u8)
    }
    #[doc = "INTM interrupt monitor error 3~0 event occurred."]
    #[inline(always)]
    pub const fn set_in24_21_status(&mut self, val: super::vals::In2421Status) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "MSF SOCTRIM 7~0 ECC error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in32_25_status(&self) -> super::vals::In3225Status {
        let val = (self.0 >> 9usize) & 0xff;
        super::vals::In3225Status::from_bits(val as u8)
    }
    #[doc = "MSF SOCTRIM 7~0 ECC error event occurred."]
    #[inline(always)]
    pub const fn set_in32_25_status(&mut self, val: super::vals::In3225Status) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val.to_bits() as u32) & 0xff) << 9usize);
    }
    #[doc = "GDET0/1 SFR error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in33_status(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "GDET0/1 SFR error event occurred."]
    #[inline(always)]
    pub const fn set_in33_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SPC VDD_CORE high voltage detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in34_status(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_CORE high voltage detect event occurred."]
    #[inline(always)]
    pub const fn set_in34_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPC VDD_SYS_HVD high voltage detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in35_status(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_SYS_HVD high voltage detect event occurred."]
    #[inline(always)]
    pub const fn set_in35_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SPC VDD_IO high voltage detect event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in36_status(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SPC VDD_IO high voltage detect event occurred."]
    #[inline(always)]
    pub const fn set_in36_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "FLEXSPI GCM error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in37_status(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI GCM error event occurred."]
    #[inline(always)]
    pub const fn set_in37_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SM3 SGI error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in46_status(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "SM3 SGI error event occurred."]
    #[inline(always)]
    pub const fn set_in46_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "TRNG HW error event occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn in47_status(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG HW error event occurred."]
    #[inline(always)]
    pub const fn set_in47_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status1 {
    #[inline(always)]
    fn default() -> Status1 {
        Status1(0)
    }
}
impl core::fmt::Debug for Status1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status1")
            .field("in16_status", &self.in16_status())
            .field("in17_status", &self.in17_status())
            .field("in18_status", &self.in18_status())
            .field("in19_status", &self.in19_status())
            .field("in20_status", &self.in20_status())
            .field("in24_21_status", &self.in24_21_status())
            .field("in32_25_status", &self.in32_25_status())
            .field("in33_status", &self.in33_status())
            .field("in34_status", &self.in34_status())
            .field("in35_status", &self.in35_status())
            .field("in36_status", &self.in36_status())
            .field("in37_status", &self.in37_status())
            .field("in46_status", &self.in46_status())
            .field("in47_status", &self.in47_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status1 {{ in16_status: {=bool:?}, in17_status: {=bool:?}, in18_status: {=bool:?}, in19_status: {=bool:?}, in20_status: {=bool:?}, in24_21_status: {:?}, in32_25_status: {:?}, in33_status: {=bool:?}, in34_status: {=bool:?}, in35_status: {=bool:?}, in36_status: {=bool:?}, in37_status: {=bool:?}, in46_status: {=bool:?}, in47_status: {=bool:?} }}",
            self.in16_status(),
            self.in17_status(),
            self.in18_status(),
            self.in19_status(),
            self.in20_status(),
            self.in24_21_status(),
            self.in32_25_status(),
            self.in33_status(),
            self.in34_status(),
            self.in35_status(),
            self.in36_status(),
            self.in37_status(),
            self.in46_status(),
            self.in47_status()
        )
    }
}
#[doc = "Software event 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwEvent0(pub u32);
impl SwEvent0 {
    #[doc = "Trigger software event 0."]
    #[must_use]
    #[inline(always)]
    pub const fn trigger_sw_event_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Trigger software event 0."]
    #[inline(always)]
    pub const fn set_trigger_sw_event_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwEvent0 {
    #[inline(always)]
    fn default() -> SwEvent0 {
        SwEvent0(0)
    }
}
impl core::fmt::Debug for SwEvent0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwEvent0")
            .field("trigger_sw_event_0", &self.trigger_sw_event_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwEvent0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwEvent0 {{ trigger_sw_event_0: {=u32:?} }}",
            self.trigger_sw_event_0()
        )
    }
}
#[doc = "Software event 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwEvent1(pub u32);
impl SwEvent1 {
    #[doc = "Trigger software event 1."]
    #[must_use]
    #[inline(always)]
    pub const fn trigger_sw_event_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Trigger software event 1."]
    #[inline(always)]
    pub const fn set_trigger_sw_event_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwEvent1 {
    #[inline(always)]
    fn default() -> SwEvent1 {
        SwEvent1(0)
    }
}
impl core::fmt::Debug for SwEvent1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwEvent1")
            .field("trigger_sw_event_1", &self.trigger_sw_event_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwEvent1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SwEvent1 {{ trigger_sw_event_1: {=u32:?} }}",
            self.trigger_sw_event_1()
        )
    }
}
