#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCR(pub u32);
impl CCR {
    #[doc = "Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP0RE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP0RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP0FE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP0FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP0I(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_CAP0I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP1RE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP1RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP1FE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP1FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP1I(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_CAP1I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP2RE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP2RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP2FE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP2FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP2I(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_CAP2I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP3RE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP3RE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP3FE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_CAP3FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP3I(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_CAP3I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for CCR {
    #[inline(always)]
    fn default() -> CCR {
        CCR(0)
    }
}
impl core::fmt::Debug for CCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("CAP0RE", &self.CAP0RE())
            .field("CAP0FE", &self.CAP0FE())
            .field("CAP0I", &self.CAP0I())
            .field("CAP1RE", &self.CAP1RE())
            .field("CAP1FE", &self.CAP1FE())
            .field("CAP1I", &self.CAP1I())
            .field("CAP2RE", &self.CAP2RE())
            .field("CAP2FE", &self.CAP2FE())
            .field("CAP2I", &self.CAP2I())
            .field("CAP3RE", &self.CAP3RE())
            .field("CAP3FE", &self.CAP3FE())
            .field("CAP3I", &self.CAP3I())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCR {{ CAP0RE: {=bool:?}, CAP0FE: {=bool:?}, CAP0I: {=bool:?}, CAP1RE: {=bool:?}, CAP1FE: {=bool:?}, CAP1I: {=bool:?}, CAP2RE: {=bool:?}, CAP2FE: {=bool:?}, CAP2I: {=bool:?}, CAP3RE: {=bool:?}, CAP3FE: {=bool:?}, CAP3I: {=bool:?} }}",
            self.CAP0RE(),
            self.CAP0FE(),
            self.CAP0I(),
            self.CAP1RE(),
            self.CAP1FE(),
            self.CAP1I(),
            self.CAP2RE(),
            self.CAP2FE(),
            self.CAP2I(),
            self.CAP3RE(),
            self.CAP3FE(),
            self.CAP3I()
        )
    }
}
#[doc = "Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CR(pub u32);
impl CR {
    #[doc = "Timer counter capture value."]
    #[must_use]
    #[inline(always)]
    pub const fn CAP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter capture value."]
    #[inline(always)]
    pub const fn set_CAP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CR {
    #[inline(always)]
    fn default() -> CR {
        CR(0)
    }
}
impl core::fmt::Debug for CR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR").field("CAP", &self.CAP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CR {{ CAP: {=u32:?} }}", self.CAP())
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTCR(pub u32);
impl CTCR {
    #[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[must_use]
    #[inline(always)]
    pub const fn CTMODE(&self) -> super::vals::CTMODE {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CTMODE::from_bits(val as u8)
    }
    #[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub const fn set_CTMODE(&mut self, val: super::vals::CTMODE) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[must_use]
    #[inline(always)]
    pub const fn CINSEL(&self) -> super::vals::CINSEL {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CINSEL::from_bits(val as u8)
    }
    #[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub const fn set_CINSEL(&mut self, val: super::vals::CINSEL) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn ENCC(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub const fn set_ENCC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn SELCC(&self) -> super::vals::SELCC {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::SELCC::from_bits(val as u8)
    }
    #[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub const fn set_SELCC(&mut self, val: super::vals::SELCC) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for CTCR {
    #[inline(always)]
    fn default() -> CTCR {
        CTCR(0)
    }
}
impl core::fmt::Debug for CTCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTCR")
            .field("CTMODE", &self.CTMODE())
            .field("CINSEL", &self.CINSEL())
            .field("ENCC", &self.ENCC())
            .field("SELCC", &self.SELCC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTCR {{ CTMODE: {:?}, CINSEL: {:?}, ENCC: {=bool:?}, SELCC: {:?} }}",
            self.CTMODE(),
            self.CINSEL(),
            self.ENCC(),
            self.SELCC()
        )
    }
}
#[doc = "External Match Register. The EMR controls the match function and the external match pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EMR(pub u32);
impl EMR {
    #[doc = "External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn EM0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_EM0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn EM1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_EM1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn EM2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_EM2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn EM3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_EM3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "External Match Control 0. Determines the functionality of External Match 0."]
    #[must_use]
    #[inline(always)]
    pub const fn EMC0(&self) -> super::vals::EMC0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::EMC0::from_bits(val as u8)
    }
    #[doc = "External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub const fn set_EMC0(&mut self, val: super::vals::EMC0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Match Control 1. Determines the functionality of External Match 1."]
    #[must_use]
    #[inline(always)]
    pub const fn EMC1(&self) -> super::vals::EMC1 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::EMC1::from_bits(val as u8)
    }
    #[doc = "External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub const fn set_EMC1(&mut self, val: super::vals::EMC1) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "External Match Control 2. Determines the functionality of External Match 2."]
    #[must_use]
    #[inline(always)]
    pub const fn EMC2(&self) -> super::vals::EMC2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::EMC2::from_bits(val as u8)
    }
    #[doc = "External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub const fn set_EMC2(&mut self, val: super::vals::EMC2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Match Control 3. Determines the functionality of External Match 3."]
    #[must_use]
    #[inline(always)]
    pub const fn EMC3(&self) -> super::vals::EMC3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::EMC3::from_bits(val as u8)
    }
    #[doc = "External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub const fn set_EMC3(&mut self, val: super::vals::EMC3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for EMR {
    #[inline(always)]
    fn default() -> EMR {
        EMR(0)
    }
}
impl core::fmt::Debug for EMR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMR")
            .field("EM0", &self.EM0())
            .field("EM1", &self.EM1())
            .field("EM2", &self.EM2())
            .field("EM3", &self.EM3())
            .field("EMC0", &self.EMC0())
            .field("EMC1", &self.EMC1())
            .field("EMC2", &self.EMC2())
            .field("EMC3", &self.EMC3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EMR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EMR {{ EM0: {=bool:?}, EM1: {=bool:?}, EM2: {=bool:?}, EM3: {=bool:?}, EMC0: {:?}, EMC1: {:?}, EMC2: {:?}, EMC3: {:?} }}",
            self.EM0(),
            self.EM1(),
            self.EM2(),
            self.EM3(),
            self.EMC0(),
            self.EMC1(),
            self.EMC2(),
            self.EMC3()
        )
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IR(pub u32);
impl IR {
    #[doc = "Interrupt flag for match channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn MR0INT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 0."]
    #[inline(always)]
    pub const fn set_MR0INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt flag for match channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn MR1INT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 1."]
    #[inline(always)]
    pub const fn set_MR1INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt flag for match channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn MR2INT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 2."]
    #[inline(always)]
    pub const fn set_MR2INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt flag for match channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn MR3INT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 3."]
    #[inline(always)]
    pub const fn set_MR3INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt flag for capture channel 0 event."]
    #[must_use]
    #[inline(always)]
    pub const fn CR0INT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub const fn set_CR0INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt flag for capture channel 1 event."]
    #[must_use]
    #[inline(always)]
    pub const fn CR1INT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub const fn set_CR1INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt flag for capture channel 2 event."]
    #[must_use]
    #[inline(always)]
    pub const fn CR2INT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub const fn set_CR2INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt flag for capture channel 3 event."]
    #[must_use]
    #[inline(always)]
    pub const fn CR3INT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub const fn set_CR3INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IR {
    #[inline(always)]
    fn default() -> IR {
        IR(0)
    }
}
impl core::fmt::Debug for IR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("MR0INT", &self.MR0INT())
            .field("MR1INT", &self.MR1INT())
            .field("MR2INT", &self.MR2INT())
            .field("MR3INT", &self.MR3INT())
            .field("CR0INT", &self.CR0INT())
            .field("CR1INT", &self.CR1INT())
            .field("CR2INT", &self.CR2INT())
            .field("CR3INT", &self.CR3INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IR {{ MR0INT: {=bool:?}, MR1INT: {=bool:?}, MR2INT: {=bool:?}, MR3INT: {=bool:?}, CR0INT: {=bool:?}, CR1INT: {=bool:?}, CR2INT: {=bool:?}, CR3INT: {=bool:?} }}",
            self.MR0INT(),
            self.MR1INT(),
            self.MR2INT(),
            self.MR3INT(),
            self.CR0INT(),
            self.CR1INT(),
            self.CR2INT(),
            self.CR3INT()
        )
    }
}
#[doc = "Match Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCR(pub u32);
impl MCR {
    #[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR0I(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_MR0I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset on MR0: the TC will be reset if MR0 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn MR0R(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub const fn set_MR0R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR0S(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub const fn set_MR0S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR1I(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_MR1I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset on MR1: the TC will be reset if MR1 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn MR1R(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub const fn set_MR1R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR1S(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub const fn set_MR1S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR2I(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_MR2I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset on MR2: the TC will be reset if MR2 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn MR2R(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub const fn set_MR2R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR2S(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub const fn set_MR2S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR3I(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_MR3I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset on MR3: the TC will be reset if MR3 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn MR3R(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub const fn set_MR3R(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn MR3S(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub const fn set_MR3S(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn MR0RL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_MR0RL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn MR1RL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_MR1RL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn MR2RL(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_MR2RL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn MR3RL(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_MR3RL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MCR {
    #[inline(always)]
    fn default() -> MCR {
        MCR(0)
    }
}
impl core::fmt::Debug for MCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCR")
            .field("MR0I", &self.MR0I())
            .field("MR0R", &self.MR0R())
            .field("MR0S", &self.MR0S())
            .field("MR1I", &self.MR1I())
            .field("MR1R", &self.MR1R())
            .field("MR1S", &self.MR1S())
            .field("MR2I", &self.MR2I())
            .field("MR2R", &self.MR2R())
            .field("MR2S", &self.MR2S())
            .field("MR3I", &self.MR3I())
            .field("MR3R", &self.MR3R())
            .field("MR3S", &self.MR3S())
            .field("MR0RL", &self.MR0RL())
            .field("MR1RL", &self.MR1RL())
            .field("MR2RL", &self.MR2RL())
            .field("MR3RL", &self.MR3RL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MCR {{ MR0I: {=bool:?}, MR0R: {=bool:?}, MR0S: {=bool:?}, MR1I: {=bool:?}, MR1R: {=bool:?}, MR1S: {=bool:?}, MR2I: {=bool:?}, MR2R: {=bool:?}, MR2S: {=bool:?}, MR3I: {=bool:?}, MR3R: {=bool:?}, MR3S: {=bool:?}, MR0RL: {=bool:?}, MR1RL: {=bool:?}, MR2RL: {=bool:?}, MR3RL: {=bool:?} }}",
            self.MR0I(),
            self.MR0R(),
            self.MR0S(),
            self.MR1I(),
            self.MR1R(),
            self.MR1S(),
            self.MR2I(),
            self.MR2R(),
            self.MR2S(),
            self.MR3I(),
            self.MR3R(),
            self.MR3S(),
            self.MR0RL(),
            self.MR1RL(),
            self.MR2RL(),
            self.MR3RL()
        )
    }
}
#[doc = "Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MR(pub u32);
impl MR {
    #[doc = "Timer counter match value."]
    #[must_use]
    #[inline(always)]
    pub const fn MATCH(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter match value."]
    #[inline(always)]
    pub const fn set_MATCH(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MR {
    #[inline(always)]
    fn default() -> MR {
        MR(0)
    }
}
impl core::fmt::Debug for MR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MR").field("MATCH", &self.MATCH()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MR {{ MATCH: {=u32:?} }}", self.MATCH())
    }
}
#[doc = "Match Shadow Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSR(pub u32);
impl MSR {
    #[doc = "Timer counter match shadow value."]
    #[must_use]
    #[inline(always)]
    pub const fn SHADOW(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter match shadow value."]
    #[inline(always)]
    pub const fn set_SHADOW(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MSR {
    #[inline(always)]
    fn default() -> MSR {
        MSR(0)
    }
}
impl core::fmt::Debug for MSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSR")
            .field("SHADOW", &self.SHADOW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MSR {{ SHADOW: {=u32:?} }}", self.SHADOW())
    }
}
#[doc = "Prescale Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PC(pub u32);
impl PC {
    #[doc = "Prescale counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn PCVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale counter value."]
    #[inline(always)]
    pub const fn set_PCVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PC {
    #[inline(always)]
    fn default() -> PC {
        PC(0)
    }
}
impl core::fmt::Debug for PC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PC").field("PCVAL", &self.PCVAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PC {{ PCVAL: {=u32:?} }}", self.PCVAL())
    }
}
#[doc = "Prescale Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PR(pub u32);
impl PR {
    #[doc = "Prescale counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn PRVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale counter value."]
    #[inline(always)]
    pub const fn set_PRVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PR {
    #[inline(always)]
    fn default() -> PR {
        PR(0)
    }
}
impl core::fmt::Debug for PR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR").field("PRVAL", &self.PRVAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PR {{ PRVAL: {=u32:?} }}", self.PRVAL())
    }
}
#[doc = "PWM Control Register. This register enables PWM mode for the external match pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWMC(pub u32);
impl PWMC {
    #[doc = "PWM mode enable for channel0."]
    #[must_use]
    #[inline(always)]
    pub const fn PWMEN0(&self) -> super::vals::PWMEN0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PWMEN0::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel0."]
    #[inline(always)]
    pub const fn set_PWMEN0(&mut self, val: super::vals::PWMEN0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PWM mode enable for channel1."]
    #[must_use]
    #[inline(always)]
    pub const fn PWMEN1(&self) -> super::vals::PWMEN1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PWMEN1::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel1."]
    #[inline(always)]
    pub const fn set_PWMEN1(&mut self, val: super::vals::PWMEN1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PWM mode enable for channel2."]
    #[must_use]
    #[inline(always)]
    pub const fn PWMEN2(&self) -> super::vals::PWMEN2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PWMEN2::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel2."]
    #[inline(always)]
    pub const fn set_PWMEN2(&mut self, val: super::vals::PWMEN2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn PWMEN3(&self) -> super::vals::PWMEN3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PWMEN3::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub const fn set_PWMEN3(&mut self, val: super::vals::PWMEN3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for PWMC {
    #[inline(always)]
    fn default() -> PWMC {
        PWMC(0)
    }
}
impl core::fmt::Debug for PWMC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMC")
            .field("PWMEN0", &self.PWMEN0())
            .field("PWMEN1", &self.PWMEN1())
            .field("PWMEN2", &self.PWMEN2())
            .field("PWMEN3", &self.PWMEN3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWMC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWMC {{ PWMEN0: {:?}, PWMEN1: {:?}, PWMEN2: {:?}, PWMEN3: {:?} }}",
            self.PWMEN0(),
            self.PWMEN1(),
            self.PWMEN2(),
            self.PWMEN3()
        )
    }
}
#[doc = "Timer Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TC(pub u32);
impl TC {
    #[doc = "Timer counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn TCVAL(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter value."]
    #[inline(always)]
    pub const fn set_TCVAL(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TC {
    #[inline(always)]
    fn default() -> TC {
        TC(0)
    }
}
impl core::fmt::Debug for TC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TC").field("TCVAL", &self.TCVAL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TC {{ TCVAL: {=u32:?} }}", self.TCVAL())
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TCR(pub u32);
impl TCR {
    #[doc = "Counter enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable."]
    #[inline(always)]
    pub const fn set_CEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter reset."]
    #[must_use]
    #[inline(always)]
    pub const fn CRST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter reset."]
    #[inline(always)]
    pub const fn set_CRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for TCR {
    #[inline(always)]
    fn default() -> TCR {
        TCR(0)
    }
}
impl core::fmt::Debug for TCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCR")
            .field("CEN", &self.CEN())
            .field("CRST", &self.CRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TCR {{ CEN: {=bool:?}, CRST: {=bool:?} }}",
            self.CEN(),
            self.CRST()
        )
    }
}
