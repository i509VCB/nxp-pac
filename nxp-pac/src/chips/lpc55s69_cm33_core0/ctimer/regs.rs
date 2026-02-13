#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0re(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 0: a sequence of 0 then 1 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap0re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0fe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 0: a sequence of 1 then 0 causes CR0 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap0fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cap0i(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 0 capture event: a CR0 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_cap0i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1re(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 1: a sequence of 0 then 1 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap1re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1fe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 1: a sequence of 1 then 0 causes CR1 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap1fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cap1i(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 1 capture event: a CR1 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_cap1i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2re(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 2: a sequence of 0 then 1 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap2re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2fe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 2: a sequence of 1 then 0 causes CR2 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap2fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cap2i(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 2 capture event: a CR2 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_cap2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3re(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Rising edge of capture channel 3: a sequence of 0 then 1 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap3re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3fe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Falling edge of capture channel 3: a sequence of 1 then 0 causes CR3 to be loaded with the contents of TC. 0 = disabled. 1 = enabled."]
    #[inline(always)]
    pub const fn set_cap3fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cap3i(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Generate interrupt on channel 3 capture event: a CR3 load generates an interrupt."]
    #[inline(always)]
    pub const fn set_cap3i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("cap0re", &self.cap0re())
            .field("cap0fe", &self.cap0fe())
            .field("cap0i", &self.cap0i())
            .field("cap1re", &self.cap1re())
            .field("cap1fe", &self.cap1fe())
            .field("cap1i", &self.cap1i())
            .field("cap2re", &self.cap2re())
            .field("cap2fe", &self.cap2fe())
            .field("cap2i", &self.cap2i())
            .field("cap3re", &self.cap3re())
            .field("cap3fe", &self.cap3fe())
            .field("cap3i", &self.cap3i())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ cap0re: {=bool:?}, cap0fe: {=bool:?}, cap0i: {=bool:?}, cap1re: {=bool:?}, cap1fe: {=bool:?}, cap1i: {=bool:?}, cap2re: {=bool:?}, cap2fe: {=bool:?}, cap2i: {=bool:?}, cap3re: {=bool:?}, cap3fe: {=bool:?}, cap3i: {=bool:?} }}",
            self.cap0re(),
            self.cap0fe(),
            self.cap0i(),
            self.cap1re(),
            self.cap1fe(),
            self.cap1i(),
            self.cap2re(),
            self.cap2fe(),
            self.cap2i(),
            self.cap3re(),
            self.cap3fe(),
            self.cap3i()
        )
    }
}
#[doc = "Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Timer counter capture value."]
    #[must_use]
    #[inline(always)]
    pub const fn cap(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter capture value."]
    #[inline(always)]
    pub const fn set_cap(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr").field("cap", &self.cap()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cr {{ cap: {=u32:?} }}", self.cap())
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctcr(pub u32);
impl Ctcr {
    #[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[must_use]
    #[inline(always)]
    pub const fn ctmode(&self) -> super::vals::Ctmode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ctmode::from_bits(val as u8)
    }
    #[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub const fn set_ctmode(&mut self, val: super::vals::Ctmode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[must_use]
    #[inline(always)]
    pub const fn cinsel(&self) -> super::vals::Cinsel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Cinsel::from_bits(val as u8)
    }
    #[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub const fn set_cinsel(&mut self, val: super::vals::Cinsel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn encc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub const fn set_encc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn selcc(&self) -> super::vals::Selcc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Selcc::from_bits(val as u8)
    }
    #[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub const fn set_selcc(&mut self, val: super::vals::Selcc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for Ctcr {
    #[inline(always)]
    fn default() -> Ctcr {
        Ctcr(0)
    }
}
impl core::fmt::Debug for Ctcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctcr")
            .field("ctmode", &self.ctmode())
            .field("cinsel", &self.cinsel())
            .field("encc", &self.encc())
            .field("selcc", &self.selcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctcr {{ ctmode: {:?}, cinsel: {:?}, encc: {=bool:?}, selcc: {:?} }}",
            self.ctmode(),
            self.cinsel(),
            self.encc(),
            self.selcc()
        )
    }
}
#[doc = "External Match Register. The EMR controls the match function and the external match pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emr(pub u32);
impl Emr {
    #[doc = "External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn em0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_em0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn em1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_em1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn em2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_em2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn em3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub const fn set_em3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "External Match Control 0. Determines the functionality of External Match 0."]
    #[must_use]
    #[inline(always)]
    pub const fn emc0(&self) -> super::vals::Emc0 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Emc0::from_bits(val as u8)
    }
    #[doc = "External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub const fn set_emc0(&mut self, val: super::vals::Emc0) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "External Match Control 1. Determines the functionality of External Match 1."]
    #[must_use]
    #[inline(always)]
    pub const fn emc1(&self) -> super::vals::Emc1 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Emc1::from_bits(val as u8)
    }
    #[doc = "External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub const fn set_emc1(&mut self, val: super::vals::Emc1) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "External Match Control 2. Determines the functionality of External Match 2."]
    #[must_use]
    #[inline(always)]
    pub const fn emc2(&self) -> super::vals::Emc2 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Emc2::from_bits(val as u8)
    }
    #[doc = "External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub const fn set_emc2(&mut self, val: super::vals::Emc2) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "External Match Control 3. Determines the functionality of External Match 3."]
    #[must_use]
    #[inline(always)]
    pub const fn emc3(&self) -> super::vals::Emc3 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Emc3::from_bits(val as u8)
    }
    #[doc = "External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub const fn set_emc3(&mut self, val: super::vals::Emc3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for Emr {
    #[inline(always)]
    fn default() -> Emr {
        Emr(0)
    }
}
impl core::fmt::Debug for Emr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emr")
            .field("em0", &self.em0())
            .field("em1", &self.em1())
            .field("em2", &self.em2())
            .field("em3", &self.em3())
            .field("emc0", &self.emc0())
            .field("emc1", &self.emc1())
            .field("emc2", &self.emc2())
            .field("emc3", &self.emc3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emr {{ em0: {=bool:?}, em1: {=bool:?}, em2: {=bool:?}, em3: {=bool:?}, emc0: {:?}, emc1: {:?}, emc2: {:?}, emc3: {:?} }}",
            self.em0(),
            self.em1(),
            self.em2(),
            self.em3(),
            self.emc0(),
            self.emc1(),
            self.emc2(),
            self.emc3()
        )
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ir(pub u32);
impl Ir {
    #[doc = "Interrupt flag for match channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 0."]
    #[inline(always)]
    pub const fn set_mr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt flag for match channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1int(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 1."]
    #[inline(always)]
    pub const fn set_mr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt flag for match channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 2."]
    #[inline(always)]
    pub const fn set_mr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt flag for match channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3int(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for match channel 3."]
    #[inline(always)]
    pub const fn set_mr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt flag for capture channel 0 event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr0int(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub const fn set_cr0int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt flag for capture channel 1 event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr1int(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub const fn set_cr1int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt flag for capture channel 2 event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr2int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub const fn set_cr2int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt flag for capture channel 3 event."]
    #[must_use]
    #[inline(always)]
    pub const fn cr3int(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub const fn set_cr3int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ir {
    #[inline(always)]
    fn default() -> Ir {
        Ir(0)
    }
}
impl core::fmt::Debug for Ir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ir")
            .field("mr0int", &self.mr0int())
            .field("mr1int", &self.mr1int())
            .field("mr2int", &self.mr2int())
            .field("mr3int", &self.mr3int())
            .field("cr0int", &self.cr0int())
            .field("cr1int", &self.cr1int())
            .field("cr2int", &self.cr2int())
            .field("cr3int", &self.cr3int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ir {{ mr0int: {=bool:?}, mr1int: {=bool:?}, mr2int: {=bool:?}, mr3int: {=bool:?}, cr0int: {=bool:?}, cr1int: {=bool:?}, cr2int: {=bool:?}, cr3int: {=bool:?} }}",
            self.mr0int(),
            self.mr1int(),
            self.mr2int(),
            self.mr3int(),
            self.cr0int(),
            self.cr1int(),
            self.cr2int(),
            self.cr3int()
        )
    }
}
#[doc = "Match Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0i(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR0: an interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_mr0i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset on MR0: the TC will be reset if MR0 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0r(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR0: the TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub const fn set_mr0r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR0: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub const fn set_mr0s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1i(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR1: an interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_mr1i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset on MR1: the TC will be reset if MR1 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1r(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR1: the TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub const fn set_mr1r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1s(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR1: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub const fn set_mr1s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2i(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR2: an interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_mr2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset on MR2: the TC will be reset if MR2 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2r(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR2: the TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub const fn set_mr2r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2s(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR2: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC."]
    #[inline(always)]
    pub const fn set_mr2s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3i(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on MR3: an interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub const fn set_mr3i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset on MR3: the TC will be reset if MR3 matches it."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3r(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset on MR3: the TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub const fn set_mr3r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3s(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Stop on MR3: the TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub const fn set_mr3s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn mr0rl(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR0 with the contents of the Match 0 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_mr0rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn mr1rl(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR1 with the contents of the Match 1 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_mr1rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn mr2rl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR2 with the contents of the Match 2 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_mr2rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[must_use]
    #[inline(always)]
    pub const fn mr3rl(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Reload MR3 with the contents of the Match 3 Shadow Register when the TC is reset to zero (either via a match event or a write to bit 1 of the TCR)."]
    #[inline(always)]
    pub const fn set_mr3rl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("mr0i", &self.mr0i())
            .field("mr0r", &self.mr0r())
            .field("mr0s", &self.mr0s())
            .field("mr1i", &self.mr1i())
            .field("mr1r", &self.mr1r())
            .field("mr1s", &self.mr1s())
            .field("mr2i", &self.mr2i())
            .field("mr2r", &self.mr2r())
            .field("mr2s", &self.mr2s())
            .field("mr3i", &self.mr3i())
            .field("mr3r", &self.mr3r())
            .field("mr3s", &self.mr3s())
            .field("mr0rl", &self.mr0rl())
            .field("mr1rl", &self.mr1rl())
            .field("mr2rl", &self.mr2rl())
            .field("mr3rl", &self.mr3rl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ mr0i: {=bool:?}, mr0r: {=bool:?}, mr0s: {=bool:?}, mr1i: {=bool:?}, mr1r: {=bool:?}, mr1s: {=bool:?}, mr2i: {=bool:?}, mr2r: {=bool:?}, mr2s: {=bool:?}, mr3i: {=bool:?}, mr3r: {=bool:?}, mr3s: {=bool:?}, mr0rl: {=bool:?}, mr1rl: {=bool:?}, mr2rl: {=bool:?}, mr3rl: {=bool:?} }}",
            self.mr0i(),
            self.mr0r(),
            self.mr0s(),
            self.mr1i(),
            self.mr1r(),
            self.mr1s(),
            self.mr2i(),
            self.mr2r(),
            self.mr2s(),
            self.mr3i(),
            self.mr3r(),
            self.mr3s(),
            self.mr0rl(),
            self.mr1rl(),
            self.mr2rl(),
            self.mr3rl()
        )
    }
}
#[doc = "Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mr(pub u32);
impl Mr {
    #[doc = "Timer counter match value."]
    #[must_use]
    #[inline(always)]
    pub const fn match_(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter match value."]
    #[inline(always)]
    pub const fn set_match_(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mr {
    #[inline(always)]
    fn default() -> Mr {
        Mr(0)
    }
}
impl core::fmt::Debug for Mr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mr")
            .field("match_", &self.match_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mr {{ match_: {=u32:?} }}", self.match_())
    }
}
#[doc = "Match Shadow Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Timer counter match shadow value."]
    #[must_use]
    #[inline(always)]
    pub const fn shadow(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter match shadow value."]
    #[inline(always)]
    pub const fn set_shadow(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("shadow", &self.shadow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Msr {{ shadow: {=u32:?} }}", self.shadow())
    }
}
#[doc = "Prescale Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc(pub u32);
impl Pc {
    #[doc = "Prescale counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn pcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale counter value."]
    #[inline(always)]
    pub const fn set_pcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        Pc(0)
    }
}
impl core::fmt::Debug for Pc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pc").field("pcval", &self.pcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pc {{ pcval: {=u32:?} }}", self.pcval())
    }
}
#[doc = "Prescale Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc = "Prescale counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn prval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Prescale counter value."]
    #[inline(always)]
    pub const fn set_prval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pr {
    #[inline(always)]
    fn default() -> Pr {
        Pr(0)
    }
}
impl core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pr").field("prval", &self.prval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pr {{ prval: {=u32:?} }}", self.prval())
    }
}
#[doc = "PWM Control Register. This register enables PWM mode for the external match pins."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmc(pub u32);
impl Pwmc {
    #[doc = "PWM mode enable for channel0."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen0(&self) -> super::vals::Pwmen0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pwmen0::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel0."]
    #[inline(always)]
    pub const fn set_pwmen0(&mut self, val: super::vals::Pwmen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PWM mode enable for channel1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen1(&self) -> super::vals::Pwmen1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pwmen1::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel1."]
    #[inline(always)]
    pub const fn set_pwmen1(&mut self, val: super::vals::Pwmen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PWM mode enable for channel2."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen2(&self) -> super::vals::Pwmen2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pwmen2::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel2."]
    #[inline(always)]
    pub const fn set_pwmen2(&mut self, val: super::vals::Pwmen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn pwmen3(&self) -> super::vals::Pwmen3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pwmen3::from_bits(val as u8)
    }
    #[doc = "PWM mode enable for channel3. Note: It is recommended to use match channel 3 to set the PWM cycle."]
    #[inline(always)]
    pub const fn set_pwmen3(&mut self, val: super::vals::Pwmen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwmc {
    #[inline(always)]
    fn default() -> Pwmc {
        Pwmc(0)
    }
}
impl core::fmt::Debug for Pwmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwmc")
            .field("pwmen0", &self.pwmen0())
            .field("pwmen1", &self.pwmen1())
            .field("pwmen2", &self.pwmen2())
            .field("pwmen3", &self.pwmen3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwmc {{ pwmen0: {:?}, pwmen1: {:?}, pwmen2: {:?}, pwmen3: {:?} }}",
            self.pwmen0(),
            self.pwmen1(),
            self.pwmen2(),
            self.pwmen3()
        )
    }
}
#[doc = "Timer Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tc(pub u32);
impl Tc {
    #[doc = "Timer counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn tcval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timer counter value."]
    #[inline(always)]
    pub const fn set_tcval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tc {
    #[inline(always)]
    fn default() -> Tc {
        Tc(0)
    }
}
impl core::fmt::Debug for Tc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tc").field("tcval", &self.tcval()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tc {{ tcval: {=u32:?} }}", self.tcval())
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Counter enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Counter enable."]
    #[inline(always)]
    pub const fn set_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Counter reset."]
    #[must_use]
    #[inline(always)]
    pub const fn crst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Counter reset."]
    #[inline(always)]
    pub const fn set_crst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("cen", &self.cen())
            .field("crst", &self.crst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ cen: {=bool:?}, crst: {=bool:?} }}",
            self.cen(),
            self.crst()
        )
    }
}
