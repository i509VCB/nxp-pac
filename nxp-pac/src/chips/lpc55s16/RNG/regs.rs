#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COUNTER_CFG(pub u32);
impl COUNTER_CFG {
    #[doc = "00: disabled 01: update once."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00: disabled 01: update once."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects the internal clock on which to compute statistics."]
    #[must_use]
    #[inline(always)]
    pub const fn CLOCK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Selects the internal clock on which to compute statistics."]
    #[inline(always)]
    pub const fn set_CLOCK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[must_use]
    #[inline(always)]
    pub const fn SHIFT4X(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "To be used to add precision to clock_ratio and determine 'entropy refill'."]
    #[inline(always)]
    pub const fn set_SHIFT4X(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
}
impl Default for COUNTER_CFG {
    #[inline(always)]
    fn default() -> COUNTER_CFG {
        COUNTER_CFG(0)
    }
}
impl core::fmt::Debug for COUNTER_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNTER_CFG")
            .field("MODE", &self.MODE())
            .field("CLOCK_SEL", &self.CLOCK_SEL())
            .field("SHIFT4X", &self.SHIFT4X())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COUNTER_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COUNTER_CFG {{ MODE: {=u8:?}, CLOCK_SEL: {=u8:?}, SHIFT4X: {=u8:?} }}",
            self.MODE(),
            self.CLOCK_SEL(),
            self.SHIFT4X()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct COUNTER_VAL(pub u32);
impl COUNTER_VAL {
    #[doc = "Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_RATIO(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Gives the ratio between the internal clocks frequencies and the register clock frequency for evaluation and certification purposes."]
    #[inline(always)]
    pub const fn set_CLK_RATIO(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[must_use]
    #[inline(always)]
    pub const fn REFRESH_CNT(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x3fff;
        val as u16
    }
    #[doc = "Incremented (till max possible value) each time COUNTER was updated since last reading to any *_NUMBER."]
    #[inline(always)]
    pub const fn set_REFRESH_CNT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
    }
}
impl Default for COUNTER_VAL {
    #[inline(always)]
    fn default() -> COUNTER_VAL {
        COUNTER_VAL(0)
    }
}
impl core::fmt::Debug for COUNTER_VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNTER_VAL")
            .field("CLK_RATIO", &self.CLK_RATIO())
            .field("REFRESH_CNT", &self.REFRESH_CNT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for COUNTER_VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "COUNTER_VAL {{ CLK_RATIO: {=u8:?}, REFRESH_CNT: {=u16:?} }}",
            self.CLK_RATIO(),
            self.REFRESH_CNT()
        )
    }
}
#[doc = "This register contains a random 32 bit number which is pre-computed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENCRYPTED_NUMBER(pub u32);
impl ENCRYPTED_NUMBER {
    #[doc = "This register contains a random 32 bit number which is pre-computed."]
    #[must_use]
    #[inline(always)]
    pub const fn ENCRYPTED_NUMBER(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This register contains a random 32 bit number which is pre-computed."]
    #[inline(always)]
    pub const fn set_ENCRYPTED_NUMBER(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ENCRYPTED_NUMBER {
    #[inline(always)]
    fn default() -> ENCRYPTED_NUMBER {
        ENCRYPTED_NUMBER(0)
    }
}
impl core::fmt::Debug for ENCRYPTED_NUMBER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENCRYPTED_NUMBER")
            .field("ENCRYPTED_NUMBER", &self.ENCRYPTED_NUMBER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENCRYPTED_NUMBER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENCRYPTED_NUMBER {{ ENCRYPTED_NUMBER: {=u32:?} }}",
            self.ENCRYPTED_NUMBER()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENTROPY_INJECT(pub u32);
impl ENTROPY_INJECT {
    #[doc = "Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[must_use]
    #[inline(always)]
    pub const fn ENTROPY(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub const fn set_ENTROPY(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ENTROPY_INJECT {
    #[inline(always)]
    fn default() -> ENTROPY_INJECT {
        ENTROPY_INJECT(0)
    }
}
impl core::fmt::Debug for ENTROPY_INJECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTROPY_INJECT")
            .field("ENTROPY", &self.ENTROPY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENTROPY_INJECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENTROPY_INJECT {{ ENTROPY: {=u32:?} }}", self.ENTROPY())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC_CFG(pub u32);
impl MISC_CFG {
    #[doc = "If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[must_use]
    #[inline(always)]
    pub const fn AES_RESEED(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set, ENCRYPTED_NUMBER generation becomes predictable, provided all secrets and current internal state are known: independant from entropy source."]
    #[inline(always)]
    pub const fn set_AES_RESEED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set this bit to re-seed AES."]
    #[must_use]
    #[inline(always)]
    pub const fn AES_DT_CFG(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to re-seed AES."]
    #[inline(always)]
    pub const fn set_AES_DT_CFG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MISC_CFG {
    #[inline(always)]
    fn default() -> MISC_CFG {
        MISC_CFG(0)
    }
}
impl core::fmt::Debug for MISC_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CFG")
            .field("AES_RESEED", &self.AES_RESEED())
            .field("AES_DT_CFG", &self.AES_DT_CFG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MISC_CFG {{ AES_RESEED: {=bool:?}, AES_DT_CFG: {=bool:?} }}",
            self.AES_RESEED(),
            self.AES_DT_CFG()
        )
    }
}
#[doc = "IP identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODULEID(pub u32);
impl MODULEID {
    #[doc = "Aperture i."]
    #[must_use]
    #[inline(always)]
    pub const fn APERTURE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture i."]
    #[inline(always)]
    pub const fn set_APERTURE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn MIN_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision i."]
    #[inline(always)]
    pub const fn set_MIN_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision i."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJ_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision i."]
    #[inline(always)]
    pub const fn set_MAJ_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Identifier."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Identifier."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MODULEID {
    #[inline(always)]
    fn default() -> MODULEID {
        MODULEID(0)
    }
}
impl core::fmt::Debug for MODULEID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODULEID")
            .field("APERTURE", &self.APERTURE())
            .field("MIN_REV", &self.MIN_REV())
            .field("MAJ_REV", &self.MAJ_REV())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODULEID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODULEID {{ APERTURE: {=u8:?}, MIN_REV: {=u8:?}, MAJ_REV: {=u8:?}, ID: {=u16:?} }}",
            self.APERTURE(),
            self.MIN_REV(),
            self.MAJ_REV(),
            self.ID()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ONLINE_TEST_CFG(pub u32);
impl ONLINE_TEST_CFG {
    #[doc = "0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[must_use]
    #[inline(always)]
    pub const fn ACTIVATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub const fn set_ACTIVATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_SEL(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub const fn set_DATA_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
}
impl Default for ONLINE_TEST_CFG {
    #[inline(always)]
    fn default() -> ONLINE_TEST_CFG {
        ONLINE_TEST_CFG(0)
    }
}
impl core::fmt::Debug for ONLINE_TEST_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ONLINE_TEST_CFG")
            .field("ACTIVATE", &self.ACTIVATE())
            .field("DATA_SEL", &self.DATA_SEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ONLINE_TEST_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ONLINE_TEST_CFG {{ ACTIVATE: {=bool:?}, DATA_SEL: {=u8:?} }}",
            self.ACTIVATE(),
            self.DATA_SEL()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ONLINE_TEST_VAL(pub u32);
impl ONLINE_TEST_VAL {
    #[doc = "This value is updated as described in field 'activate'."]
    #[must_use]
    #[inline(always)]
    pub const fn LIVE_CHI_SQUARED(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "This value is updated as described in field 'activate'."]
    #[inline(always)]
    pub const fn set_LIVE_CHI_SQUARED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[must_use]
    #[inline(always)]
    pub const fn MIN_CHI_SQUARED(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[inline(always)]
    pub const fn set_MIN_CHI_SQUARED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[must_use]
    #[inline(always)]
    pub const fn MAX_CHI_SQUARED(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "This field is reset when 'activate'==0."]
    #[inline(always)]
    pub const fn set_MAX_CHI_SQUARED(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for ONLINE_TEST_VAL {
    #[inline(always)]
    fn default() -> ONLINE_TEST_VAL {
        ONLINE_TEST_VAL(0)
    }
}
impl core::fmt::Debug for ONLINE_TEST_VAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ONLINE_TEST_VAL")
            .field("LIVE_CHI_SQUARED", &self.LIVE_CHI_SQUARED())
            .field("MIN_CHI_SQUARED", &self.MIN_CHI_SQUARED())
            .field("MAX_CHI_SQUARED", &self.MAX_CHI_SQUARED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ONLINE_TEST_VAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ONLINE_TEST_VAL {{ LIVE_CHI_SQUARED: {=u8:?}, MIN_CHI_SQUARED: {=u8:?}, MAX_CHI_SQUARED: {=u8:?} }}",
            self.LIVE_CHI_SQUARED(),
            self.MIN_CHI_SQUARED(),
            self.MAX_CHI_SQUARED()
        )
    }
}
#[doc = "Powerdown mode (standard but certainly useless here)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct POWERDOWN(pub u32);
impl POWERDOWN {
    #[doc = "Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[must_use]
    #[inline(always)]
    pub const fn SOFT_RESET(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request softreset that will go low automaticaly after acknowledge from CORE."]
    #[inline(always)]
    pub const fn set_SOFT_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_SOFT_RESET(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When used with softreset it forces CORE_RESETN to low on acknowledge from CORE."]
    #[inline(always)]
    pub const fn set_FORCE_SOFT_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When set all accesses to standard registers are blocked."]
    #[must_use]
    #[inline(always)]
    pub const fn POWERDOWN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When set all accesses to standard registers are blocked."]
    #[inline(always)]
    pub const fn set_POWERDOWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for POWERDOWN {
    #[inline(always)]
    fn default() -> POWERDOWN {
        POWERDOWN(0)
    }
}
impl core::fmt::Debug for POWERDOWN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWERDOWN")
            .field("SOFT_RESET", &self.SOFT_RESET())
            .field("FORCE_SOFT_RESET", &self.FORCE_SOFT_RESET())
            .field("POWERDOWN", &self.POWERDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for POWERDOWN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "POWERDOWN {{ SOFT_RESET: {=bool:?}, FORCE_SOFT_RESET: {=bool:?}, POWERDOWN: {=bool:?} }}",
            self.SOFT_RESET(),
            self.FORCE_SOFT_RESET(),
            self.POWERDOWN()
        )
    }
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RANDOM_NUMBER(pub u32);
impl RANDOM_NUMBER {
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[must_use]
    #[inline(always)]
    pub const fn RANDOM_NUMBER(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read."]
    #[inline(always)]
    pub const fn set_RANDOM_NUMBER(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RANDOM_NUMBER {
    #[inline(always)]
    fn default() -> RANDOM_NUMBER {
        RANDOM_NUMBER(0)
    }
}
impl core::fmt::Debug for RANDOM_NUMBER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RANDOM_NUMBER")
            .field("RANDOM_NUMBER", &self.RANDOM_NUMBER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RANDOM_NUMBER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RANDOM_NUMBER {{ RANDOM_NUMBER: {=u32:?} }}",
            self.RANDOM_NUMBER()
        )
    }
}
