#[doc = "SCT capture register of capture channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[must_use]
    #[inline(always)]
    pub const fn capn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub const fn set_capn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        Cap(0)
    }
}
impl core::fmt::Debug for Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap")
            .field("capn_l", &self.capn_l())
            .field("capn_h", &self.capn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap {{ capn_l: {=u16:?}, capn_h: {=u16:?} }}",
            self.capn_l(),
            self.capn_h()
        )
    }
}
#[doc = "SCT capture control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl(pub u32);
impl Capctrl {
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn capconn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT."]
    #[inline(always)]
    pub const fn set_capconn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl {
    #[inline(always)]
    fn default() -> Capctrl {
        Capctrl(0)
    }
}
impl core::fmt::Debug for Capctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl")
            .field("capconn_l", &self.capconn_l())
            .field("capconn_h", &self.capconn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl {{ capconn_l: {=u16:?}, capconn_h: {=u16:?} }}",
            self.capconn_l(),
            self.capconn_h()
        )
    }
}
#[doc = "SCT conflict interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conen(pub u32);
impl Conen {
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ncen(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the SCT conflict flag register are both one (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_ncen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Conen {
    #[inline(always)]
    fn default() -> Conen {
        Conen(0)
    }
}
impl core::fmt::Debug for Conen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conen").field("ncen", &self.ncen()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Conen {{ ncen: {=u16:?} }}", self.ncen())
    }
}
#[doc = "SCT configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "SCT operation"]
    #[must_use]
    #[inline(always)]
    pub const fn unify(&self) -> super::vals::Unify {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Unify::from_bits(val as u8)
    }
    #[doc = "SCT operation"]
    #[inline(always)]
    pub const fn set_unify(&mut self, val: super::vals::Unify) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SCT clock mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clkmode(&self) -> super::vals::Clkmode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Clkmode::from_bits(val as u8)
    }
    #[doc = "SCT clock mode"]
    #[inline(always)]
    pub const fn set_clkmode(&mut self, val: super::vals::Clkmode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> super::vals::Cksel {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Cksel::from_bits(val as u8)
    }
    #[doc = "SCT clock select. The specific functionality of the designated input/edge is dependent on the CLKMODE bit selection in this register."]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: super::vals::Cksel) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the lower match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_noreload_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_h(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit prevents the higher match registers from being reloaded from their respective reload registers. Setting this bit eliminates the need to write to the reload registers MATCHREL if the match values are fixed. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_noreload_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[must_use]
    #[inline(always)]
    pub const fn insync(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Synchronization for input N (bit 9 = input 0, bit 10 = input 1,, bit 12 = input 3); all other bits are reserved. A 1 in one of these bits subjects the corresponding input to synchronization to the SCT clock, before it is used to create an event. If an input is known to already be synchronous to the SCT clock, this bit may be set to 0 for faster input response. (Note: The SCT clock is the system clock for CKMODEs 0-2. It is the selected, asynchronous SCT input clock for CKMODE3). Note that the INSYNC field only affects inputs used for event generation. It does not apply to the clock input specified in the CKSEL field."]
    #[inline(always)]
    pub const fn set_insync(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_l(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit causes a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit applies to both the higher and lower registers when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_autolimit_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A one in this bit will cause a match on match register 0 to be treated as a de-facto LIMIT condition without the need to define an associated event. As with any LIMIT event, this automatic limit causes the counter to be cleared to zero in unidirectional mode or to change the direction of count in bi-directional mode. Software can write to set or clear this bit at any time. This bit is not used when the UNIFY bit is set."]
    #[inline(always)]
    pub const fn set_autolimit_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("unify", &self.unify())
            .field("clkmode", &self.clkmode())
            .field("cksel", &self.cksel())
            .field("noreload_l", &self.noreload_l())
            .field("noreload_h", &self.noreload_h())
            .field("insync", &self.insync())
            .field("autolimit_l", &self.autolimit_l())
            .field("autolimit_h", &self.autolimit_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ unify: {:?}, clkmode: {:?}, cksel: {:?}, noreload_l: {=bool:?}, noreload_h: {=bool:?}, insync: {=u8:?}, autolimit_l: {=bool:?}, autolimit_h: {=bool:?} }}",
            self.unify(),
            self.clkmode(),
            self.cksel(),
            self.noreload_l(),
            self.noreload_h(),
            self.insync(),
            self.autolimit_l(),
            self.autolimit_h()
        )
    }
}
#[doc = "SCT conflict flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conflag(pub u32);
impl Conflag {
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if a no-change conflict event occurred on output n since reset or a 1 was last written to this bit (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_ncflag(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR L/Unified, STATE L/Unified, MATCH L/Unified, or the Output register when the L/U counter was not halted. A word write to certain L and H registers can be half successful and half unsuccessful."]
    #[inline(always)]
    pub const fn set_buserrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent bus error from this SCT involved writing CTR H, STATE H, MATCH H, or the Output register when the H counter was not halted."]
    #[inline(always)]
    pub const fn set_buserrh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Conflag {
    #[inline(always)]
    fn default() -> Conflag {
        Conflag(0)
    }
}
impl core::fmt::Debug for Conflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conflag")
            .field("ncflag", &self.ncflag())
            .field("buserrl", &self.buserrl())
            .field("buserrh", &self.buserrh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Conflag {{ ncflag: {=u16:?}, buserrl: {=bool:?}, buserrh: {=bool:?} }}",
            self.ncflag(),
            self.buserrl(),
            self.buserrh()
        )
    }
}
#[doc = "SCT counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit L counter value. When UNIFY = 1, read or write the lower 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_ctr_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit H counter value. When UNIFY = 1, read or write the upper 16 bits of the 32-bit unified counter."]
    #[inline(always)]
    pub const fn set_ctr_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("ctr_l", &self.ctr_l())
            .field("ctr_h", &self.ctr_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Count {{ ctr_l: {=u16:?}, ctr_h: {=u16:?} }}",
            self.ctr_l(),
            self.ctr_h()
        )
    }
}
#[doc = "SCT control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn down_l(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_down_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_stop_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_l(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_halt_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_l(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_clrctr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "L or unified counter direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_l(&self) -> super::vals::Bidir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bidir::from_bits(val as u8)
    }
    #[doc = "L or unified counter direction select"]
    #[inline(always)]
    pub const fn set_bidir_l(&mut self, val: super::vals::Bidir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_l(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_pre_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn down_h(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub const fn set_down_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub const fn set_stop_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub const fn set_halt_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_h(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub const fn set_clrctr_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_h(&self) -> super::vals::Bidir {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Bidir::from_bits(val as u8)
    }
    #[doc = "Direction select"]
    #[inline(always)]
    pub const fn set_bidir_h(&mut self, val: super::vals::Bidir) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_h(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub const fn set_pre_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
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
            .field("down_l", &self.down_l())
            .field("stop_l", &self.stop_l())
            .field("halt_l", &self.halt_l())
            .field("clrctr_l", &self.clrctr_l())
            .field("bidir_l", &self.bidir_l())
            .field("pre_l", &self.pre_l())
            .field("down_h", &self.down_h())
            .field("stop_h", &self.stop_h())
            .field("halt_h", &self.halt_h())
            .field("clrctr_h", &self.clrctr_h())
            .field("bidir_h", &self.bidir_h())
            .field("pre_h", &self.pre_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ down_l: {=bool:?}, stop_l: {=bool:?}, halt_l: {=bool:?}, clrctr_l: {=bool:?}, bidir_l: {:?}, pre_l: {=u8:?}, down_h: {=bool:?}, stop_h: {=bool:?}, halt_h: {=bool:?}, clrctr_h: {=bool:?}, bidir_h: {:?}, pre_h: {=u8:?} }}",
            self.down_l(),
            self.stop_l(),
            self.halt_l(),
            self.clrctr_l(),
            self.bidir_l(),
            self.pre_l(),
            self.down_h(),
            self.stop_h(),
            self.halt_h(),
            self.clrctr_h(),
            self.bidir_h(),
            self.pre_h()
        )
    }
}
#[doc = "SCT DMA request 0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq0(pub u32);
impl Dmareq0 {
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 0 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_dev_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn drl0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 0 when it loads the MATCH_L/Unified registers from the RELOAD_L/Unified registers."]
    #[inline(always)]
    pub const fn set_drl0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn drq0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 0. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_drq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq0 {
    #[inline(always)]
    fn default() -> Dmareq0 {
        Dmareq0(0)
    }
}
impl core::fmt::Debug for Dmareq0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq0")
            .field("dev_0", &self.dev_0())
            .field("drl0", &self.drl0())
            .field("drq0", &self.drq0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq0 {{ dev_0: {=u16:?}, drl0: {=bool:?}, drq0: {=bool:?} }}",
            self.dev_0(),
            self.drl0(),
            self.drq0()
        )
    }
}
#[doc = "SCT DMA request 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq1(pub u32);
impl Dmareq1 {
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n triggers DMA request 1 (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_dev_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[must_use]
    #[inline(always)]
    pub const fn drl1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "A 1 in this bit triggers DMA request 1 when it loads the Match L/Unified registers from the Reload L/Unified registers."]
    #[inline(always)]
    pub const fn set_drl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[must_use]
    #[inline(always)]
    pub const fn drq1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This read-only bit indicates the state of DMA Request 1. Note that if the related DMA channel is enabled and properly set up, it is unlikely that software will see this flag, it will be cleared rapidly by the DMA service. The flag remaining set could point to an issue with DMA setup."]
    #[inline(always)]
    pub const fn set_drq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq1 {
    #[inline(always)]
    fn default() -> Dmareq1 {
        Dmareq1(0)
    }
}
impl core::fmt::Debug for Dmareq1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq1")
            .field("dev_1", &self.dev_1())
            .field("drl1", &self.drl1())
            .field("drq1", &self.drq1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq1 {{ dev_1: {=u16:?}, drl1: {=bool:?}, drq1: {=bool:?} }}",
            self.dev_1(),
            self.drl1(),
            self.drq1()
        )
    }
}
#[doc = "SCT event control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvCtrl(pub u32);
impl EvCtrl {
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[must_use]
    #[inline(always)]
    pub const fn matchsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub const fn set_matchsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn hevent(&self) -> super::vals::Hevent {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hevent::from_bits(val as u8)
    }
    #[doc = "Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub const fn set_hevent(&mut self, val: super::vals::Hevent) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input/output select"]
    #[must_use]
    #[inline(always)]
    pub const fn outsel(&self) -> super::vals::Outsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Outsel::from_bits(val as u8)
    }
    #[doc = "Input/output select"]
    #[inline(always)]
    pub const fn set_outsel(&mut self, val: super::vals::Outsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[must_use]
    #[inline(always)]
    pub const fn iosel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub const fn set_iosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[must_use]
    #[inline(always)]
    pub const fn iocond(&self) -> super::vals::Iocond {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Iocond::from_bits(val as u8)
    }
    #[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub const fn set_iocond(&mut self, val: super::vals::Iocond) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[must_use]
    #[inline(always)]
    pub const fn combmode(&self) -> super::vals::Combmode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Combmode::from_bits(val as u8)
    }
    #[doc = "Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub const fn set_combmode(&mut self, val: super::vals::Combmode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[must_use]
    #[inline(always)]
    pub const fn stateld(&self) -> super::vals::Stateld {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Stateld::from_bits(val as u8)
    }
    #[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub const fn set_stateld(&mut self, val: super::vals::Stateld) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[must_use]
    #[inline(always)]
    pub const fn statev(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub const fn set_statev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[must_use]
    #[inline(always)]
    pub const fn matchmem(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub const fn set_matchmem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Direction {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::Direction::from_bits(val as u8)
    }
    #[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: super::vals::Direction) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for EvCtrl {
    #[inline(always)]
    fn default() -> EvCtrl {
        EvCtrl(0)
    }
}
impl core::fmt::Debug for EvCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvCtrl")
            .field("matchsel", &self.matchsel())
            .field("hevent", &self.hevent())
            .field("outsel", &self.outsel())
            .field("iosel", &self.iosel())
            .field("iocond", &self.iocond())
            .field("combmode", &self.combmode())
            .field("stateld", &self.stateld())
            .field("statev", &self.statev())
            .field("matchmem", &self.matchmem())
            .field("direction", &self.direction())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvCtrl {{ matchsel: {=u8:?}, hevent: {:?}, outsel: {:?}, iosel: {=u8:?}, iocond: {:?}, combmode: {:?}, stateld: {:?}, statev: {=u8:?}, matchmem: {=bool:?}, direction: {:?} }}",
            self.matchsel(),
            self.hevent(),
            self.outsel(),
            self.iosel(),
            self.iocond(),
            self.combmode(),
            self.stateld(),
            self.statev(),
            self.matchmem(),
            self.direction()
        )
    }
}
#[doc = "SCT event state register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvState(pub u32);
impl EvState {
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn statemskn(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub const fn set_statemskn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EvState {
    #[inline(always)]
    fn default() -> EvState {
        EvState(0)
    }
}
impl core::fmt::Debug for EvState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvState")
            .field("statemskn", &self.statemskn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EvState {{ statemskn: {=u16:?} }}", self.statemskn())
    }
}
#[doc = "SCT event interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Even(pub u32);
impl Even {
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn ien(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The SCT requests an interrupt when bit n of this register and the event flag register are both one (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_ien(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Even {
    #[inline(always)]
    fn default() -> Even {
        Even(0)
    }
}
impl core::fmt::Debug for Even {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Even").field("ien", &self.ien()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Even {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Even {{ ien: {=u16:?} }}", self.ien())
    }
}
#[doc = "SCT event flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflag(pub u32);
impl Evflag {
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn flag(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Bit n is one if event n has occurred since reset or a 1 was last written to this bit (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_flag(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Evflag {
    #[inline(always)]
    fn default() -> Evflag {
        Evflag(0)
    }
}
impl core::fmt::Debug for Evflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evflag")
            .field("flag", &self.flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Evflag {{ flag: {=u16:?} }}", self.flag())
    }
}
#[doc = "SCT halt event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halt(pub u32);
impl Halt {
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_haltmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the HALT_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_haltmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Halt {
    #[inline(always)]
    fn default() -> Halt {
        Halt(0)
    }
}
impl core::fmt::Debug for Halt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Halt")
            .field("haltmsk_l", &self.haltmsk_l())
            .field("haltmsk_h", &self.haltmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Halt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Halt {{ haltmsk_l: {=u16:?}, haltmsk_h: {=u16:?} }}",
            self.haltmsk_l(),
            self.haltmsk_h()
        )
    }
}
#[doc = "SCT input register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[must_use]
    #[inline(always)]
    pub const fn ain(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub const fn set_ain(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[must_use]
    #[inline(always)]
    pub const fn sin(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub const fn set_sin(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Input {
    #[inline(always)]
    fn default() -> Input {
        Input(0)
    }
}
impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Input")
            .field("ain[0]", &self.ain(0usize))
            .field("ain[1]", &self.ain(1usize))
            .field("ain[2]", &self.ain(2usize))
            .field("ain[3]", &self.ain(3usize))
            .field("ain[4]", &self.ain(4usize))
            .field("ain[5]", &self.ain(5usize))
            .field("ain[6]", &self.ain(6usize))
            .field("ain[7]", &self.ain(7usize))
            .field("ain[8]", &self.ain(8usize))
            .field("ain[9]", &self.ain(9usize))
            .field("ain[10]", &self.ain(10usize))
            .field("ain[11]", &self.ain(11usize))
            .field("ain[12]", &self.ain(12usize))
            .field("ain[13]", &self.ain(13usize))
            .field("ain[14]", &self.ain(14usize))
            .field("ain[15]", &self.ain(15usize))
            .field("sin[0]", &self.sin(0usize))
            .field("sin[1]", &self.sin(1usize))
            .field("sin[2]", &self.sin(2usize))
            .field("sin[3]", &self.sin(3usize))
            .field("sin[4]", &self.sin(4usize))
            .field("sin[5]", &self.sin(5usize))
            .field("sin[6]", &self.sin(6usize))
            .field("sin[7]", &self.sin(7usize))
            .field("sin[8]", &self.sin(8usize))
            .field("sin[9]", &self.sin(9usize))
            .field("sin[10]", &self.sin(10usize))
            .field("sin[11]", &self.sin(11usize))
            .field("sin[12]", &self.sin(12usize))
            .field("sin[13]", &self.sin(13usize))
            .field("sin[14]", &self.sin(14usize))
            .field("sin[15]", &self.sin(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ ain[0]: {=bool:?}, ain[1]: {=bool:?}, ain[2]: {=bool:?}, ain[3]: {=bool:?}, ain[4]: {=bool:?}, ain[5]: {=bool:?}, ain[6]: {=bool:?}, ain[7]: {=bool:?}, ain[8]: {=bool:?}, ain[9]: {=bool:?}, ain[10]: {=bool:?}, ain[11]: {=bool:?}, ain[12]: {=bool:?}, ain[13]: {=bool:?}, ain[14]: {=bool:?}, ain[15]: {=bool:?}, sin[0]: {=bool:?}, sin[1]: {=bool:?}, sin[2]: {=bool:?}, sin[3]: {=bool:?}, sin[4]: {=bool:?}, sin[5]: {=bool:?}, sin[6]: {=bool:?}, sin[7]: {=bool:?}, sin[8]: {=bool:?}, sin[9]: {=bool:?}, sin[10]: {=bool:?}, sin[11]: {=bool:?}, sin[12]: {=bool:?}, sin[13]: {=bool:?}, sin[14]: {=bool:?}, sin[15]: {=bool:?} }}",
            self.ain(0usize),
            self.ain(1usize),
            self.ain(2usize),
            self.ain(3usize),
            self.ain(4usize),
            self.ain(5usize),
            self.ain(6usize),
            self.ain(7usize),
            self.ain(8usize),
            self.ain(9usize),
            self.ain(10usize),
            self.ain(11usize),
            self.ain(12usize),
            self.ain(13usize),
            self.ain(14usize),
            self.ain(15usize),
            self.sin(0usize),
            self.sin(1usize),
            self.sin(2usize),
            self.sin(3usize),
            self.sin(4usize),
            self.sin(5usize),
            self.sin(6usize),
            self.sin(7usize),
            self.sin(8usize),
            self.sin(9usize),
            self.sin(10usize),
            self.sin(11usize),
            self.sin(12usize),
            self.sin(13usize),
            self.sin(14usize),
            self.sin(15usize)
        )
    }
}
#[doc = "SCT limit event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Limit(pub u32);
impl Limit {
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the L or unified counter (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_limmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n is used as a counter limit for the H counter (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_limmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Limit {
    #[inline(always)]
    fn default() -> Limit {
        Limit(0)
    }
}
impl core::fmt::Debug for Limit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Limit")
            .field("limmsk_l", &self.limmsk_l())
            .field("limmsk_h", &self.limmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Limit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Limit {{ limmsk_l: {=u16:?}, limmsk_h: {=u16:?} }}",
            self.limmsk_l(),
            self.limmsk_h()
        )
    }
}
#[doc = "SCT match value register of match channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[must_use]
    #[inline(always)]
    pub const fn matchn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub const fn set_matchn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
impl core::fmt::Debug for Match {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("matchn_l", &self.matchn_l())
            .field("matchn_h", &self.matchn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match {{ matchn_l: {=u16:?}, matchn_h: {=u16:?} }}",
            self.matchn_l(),
            self.matchn_h()
        )
    }
}
#[doc = "SCT match reload value register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel(pub u32);
impl Matchrel {
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[must_use]
    #[inline(always)]
    pub const fn reloadn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub const fn set_reloadn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel {
    #[inline(always)]
    fn default() -> Matchrel {
        Matchrel(0)
    }
}
impl core::fmt::Debug for Matchrel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel")
            .field("reloadn_l", &self.reloadn_l())
            .field("reloadn_h", &self.reloadn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel {{ reloadn_l: {=u16:?}, reloadn_h: {=u16:?} }}",
            self.reloadn_l(),
            self.reloadn_h()
        )
    }
}
#[doc = "SCT output 0 clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClr(pub u32);
impl OutClr {
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clr(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for OutClr {
    #[inline(always)]
    fn default() -> OutClr {
        OutClr(0)
    }
}
impl core::fmt::Debug for OutClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutClr")
            .field("clr[0]", &self.clr(0usize))
            .field("clr[1]", &self.clr(1usize))
            .field("clr[2]", &self.clr(2usize))
            .field("clr[3]", &self.clr(3usize))
            .field("clr[4]", &self.clr(4usize))
            .field("clr[5]", &self.clr(5usize))
            .field("clr[6]", &self.clr(6usize))
            .field("clr[7]", &self.clr(7usize))
            .field("clr[8]", &self.clr(8usize))
            .field("clr[9]", &self.clr(9usize))
            .field("clr[10]", &self.clr(10usize))
            .field("clr[11]", &self.clr(11usize))
            .field("clr[12]", &self.clr(12usize))
            .field("clr[13]", &self.clr(13usize))
            .field("clr[14]", &self.clr(14usize))
            .field("clr[15]", &self.clr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutClr {{ clr[0]: {=bool:?}, clr[1]: {=bool:?}, clr[2]: {=bool:?}, clr[3]: {=bool:?}, clr[4]: {=bool:?}, clr[5]: {=bool:?}, clr[6]: {=bool:?}, clr[7]: {=bool:?}, clr[8]: {=bool:?}, clr[9]: {=bool:?}, clr[10]: {=bool:?}, clr[11]: {=bool:?}, clr[12]: {=bool:?}, clr[13]: {=bool:?}, clr[14]: {=bool:?}, clr[15]: {=bool:?} }}",
            self.clr(0usize),
            self.clr(1usize),
            self.clr(2usize),
            self.clr(3usize),
            self.clr(4usize),
            self.clr(5usize),
            self.clr(6usize),
            self.clr(7usize),
            self.clr(8usize),
            self.clr(9usize),
            self.clr(10usize),
            self.clr(11usize),
            self.clr(12usize),
            self.clr(13usize),
            self.clr(14usize),
            self.clr(15usize)
        )
    }
}
#[doc = "SCT output 0 set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet(pub u32);
impl OutSet {
    #[must_use]
    #[inline(always)]
    pub const fn set(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_set(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        OutSet(0)
    }
}
impl core::fmt::Debug for OutSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSet")
            .field("set[0]", &self.set(0usize))
            .field("set[1]", &self.set(1usize))
            .field("set[2]", &self.set(2usize))
            .field("set[3]", &self.set(3usize))
            .field("set[4]", &self.set(4usize))
            .field("set[5]", &self.set(5usize))
            .field("set[6]", &self.set(6usize))
            .field("set[7]", &self.set(7usize))
            .field("set[8]", &self.set(8usize))
            .field("set[9]", &self.set(9usize))
            .field("set[10]", &self.set(10usize))
            .field("set[11]", &self.set(11usize))
            .field("set[12]", &self.set(12usize))
            .field("set[13]", &self.set(13usize))
            .field("set[14]", &self.set(14usize))
            .field("set[15]", &self.set(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OutSet {{ set[0]: {=bool:?}, set[1]: {=bool:?}, set[2]: {=bool:?}, set[3]: {=bool:?}, set[4]: {=bool:?}, set[5]: {=bool:?}, set[6]: {=bool:?}, set[7]: {=bool:?}, set[8]: {=bool:?}, set[9]: {=bool:?}, set[10]: {=bool:?}, set[11]: {=bool:?}, set[12]: {=bool:?}, set[13]: {=bool:?}, set[14]: {=bool:?}, set[15]: {=bool:?} }}",
            self.set(0usize),
            self.set(1usize),
            self.set(2usize),
            self.set(3usize),
            self.set(4usize),
            self.set(5usize),
            self.set(6usize),
            self.set(7usize),
            self.set(8usize),
            self.set(9usize),
            self.set(10usize),
            self.set(11usize),
            self.set(12usize),
            self.set(13usize),
            self.set(14usize),
            self.set(15usize)
        )
    }
}
#[doc = "SCT output register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Output(pub u32);
impl Output {
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Writing a 1 to bit n forces the corresponding output HIGH. Writing a 0 forces the corresponding output LOW (output 0 = bit 0, output 1 = bit 1, etc.). The number of bits = number of outputs in this SCT."]
    #[inline(always)]
    pub const fn set_out(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Output {
    #[inline(always)]
    fn default() -> Output {
        Output(0)
    }
}
impl core::fmt::Debug for Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Output").field("out", &self.out()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Output {{ out: {=u16:?} }}", self.out())
    }
}
#[doc = "SCT output counter direction control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputdirctrl(pub u32);
impl Outputdirctrl {
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr(&self, n: usize) -> super::vals::Setclr {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Setclr::from_bits(val as u8)
    }
    #[doc = "Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub const fn set_setclr(&mut self, n: usize, val: super::vals::Setclr) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Outputdirctrl {
    #[inline(always)]
    fn default() -> Outputdirctrl {
        Outputdirctrl(0)
    }
}
impl core::fmt::Debug for Outputdirctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outputdirctrl")
            .field("setclr[0]", &self.setclr(0usize))
            .field("setclr[1]", &self.setclr(1usize))
            .field("setclr[2]", &self.setclr(2usize))
            .field("setclr[3]", &self.setclr(3usize))
            .field("setclr[4]", &self.setclr(4usize))
            .field("setclr[5]", &self.setclr(5usize))
            .field("setclr[6]", &self.setclr(6usize))
            .field("setclr[7]", &self.setclr(7usize))
            .field("setclr[8]", &self.setclr(8usize))
            .field("setclr[9]", &self.setclr(9usize))
            .field("setclr[10]", &self.setclr(10usize))
            .field("setclr[11]", &self.setclr(11usize))
            .field("setclr[12]", &self.setclr(12usize))
            .field("setclr[13]", &self.setclr(13usize))
            .field("setclr[14]", &self.setclr(14usize))
            .field("setclr[15]", &self.setclr(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputdirctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputdirctrl {{ setclr[0]: {:?}, setclr[1]: {:?}, setclr[2]: {:?}, setclr[3]: {:?}, setclr[4]: {:?}, setclr[5]: {:?}, setclr[6]: {:?}, setclr[7]: {:?}, setclr[8]: {:?}, setclr[9]: {:?}, setclr[10]: {:?}, setclr[11]: {:?}, setclr[12]: {:?}, setclr[13]: {:?}, setclr[14]: {:?}, setclr[15]: {:?} }}",
            self.setclr(0usize),
            self.setclr(1usize),
            self.setclr(2usize),
            self.setclr(3usize),
            self.setclr(4usize),
            self.setclr(5usize),
            self.setclr(6usize),
            self.setclr(7usize),
            self.setclr(8usize),
            self.setclr(9usize),
            self.setclr(10usize),
            self.setclr(11usize),
            self.setclr(12usize),
            self.setclr(13usize),
            self.setclr(14usize),
            self.setclr(15usize)
        )
    }
}
#[doc = "SCT match/capture mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regmode(pub u32);
impl Regmode {
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub const fn set_regmod_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub const fn set_regmod_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Regmode {
    #[inline(always)]
    fn default() -> Regmode {
        Regmode(0)
    }
}
impl core::fmt::Debug for Regmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Regmode")
            .field("regmod_l", &self.regmod_l())
            .field("regmod_h", &self.regmod_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Regmode {{ regmod_l: {=u16:?}, regmod_h: {=u16:?} }}",
            self.regmod_l(),
            self.regmod_h()
        )
    }
}
#[doc = "SCT conflict resolution register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res(pub u32);
impl Res {
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ores(&self, n: usize) -> super::vals::Ores {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Ores::from_bits(val as u8)
    }
    #[doc = "Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub const fn set_ores(&mut self, n: usize, val: super::vals::Ores) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Res {
    #[inline(always)]
    fn default() -> Res {
        Res(0)
    }
}
impl core::fmt::Debug for Res {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res")
            .field("ores[0]", &self.ores(0usize))
            .field("ores[1]", &self.ores(1usize))
            .field("ores[2]", &self.ores(2usize))
            .field("ores[3]", &self.ores(3usize))
            .field("ores[4]", &self.ores(4usize))
            .field("ores[5]", &self.ores(5usize))
            .field("ores[6]", &self.ores(6usize))
            .field("ores[7]", &self.ores(7usize))
            .field("ores[8]", &self.ores(8usize))
            .field("ores[9]", &self.ores(9usize))
            .field("ores[10]", &self.ores(10usize))
            .field("ores[11]", &self.ores(11usize))
            .field("ores[12]", &self.ores(12usize))
            .field("ores[13]", &self.ores(13usize))
            .field("ores[14]", &self.ores(14usize))
            .field("ores[15]", &self.ores(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Res {{ ores[0]: {:?}, ores[1]: {:?}, ores[2]: {:?}, ores[3]: {:?}, ores[4]: {:?}, ores[5]: {:?}, ores[6]: {:?}, ores[7]: {:?}, ores[8]: {:?}, ores[9]: {:?}, ores[10]: {:?}, ores[11]: {:?}, ores[12]: {:?}, ores[13]: {:?}, ores[14]: {:?}, ores[15]: {:?} }}",
            self.ores(0usize),
            self.ores(1usize),
            self.ores(2usize),
            self.ores(3usize),
            self.ores(4usize),
            self.ores(5usize),
            self.ores(6usize),
            self.ores(7usize),
            self.ores(8usize),
            self.ores(9usize),
            self.ores(10usize),
            self.ores(11usize),
            self.ores(12usize),
            self.ores(13usize),
            self.ores(14usize),
            self.ores(15usize)
        )
    }
}
#[doc = "SCT start event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_startmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n clears the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_startmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start")
            .field("startmsk_l", &self.startmsk_l())
            .field("startmsk_h", &self.startmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Start {{ startmsk_l: {=u16:?}, startmsk_h: {=u16:?} }}",
            self.startmsk_l(),
            self.startmsk_h()
        )
    }
}
#[doc = "SCT state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn state_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_state_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "State variable."]
    #[must_use]
    #[inline(always)]
    pub const fn state_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "State variable."]
    #[inline(always)]
    pub const fn set_state_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("State")
            .field("state_l", &self.state_l())
            .field("state_h", &self.state_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for State {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "State {{ state_l: {=u8:?}, state_h: {=u8:?} }}",
            self.state_l(),
            self.state_h()
        )
    }
}
#[doc = "SCT stop event select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_L bit in the CTRL register (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_stopmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "If bit n is one, event n sets the STOP_H bit in the CTRL register (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of events in this SCT."]
    #[inline(always)]
    pub const fn set_stopmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop")
            .field("stopmsk_l", &self.stopmsk_l())
            .field("stopmsk_h", &self.stopmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stop {{ stopmsk_l: {=u16:?}, stopmsk_h: {=u16:?} }}",
            self.stopmsk_l(),
            self.stopmsk_h()
        )
    }
}
