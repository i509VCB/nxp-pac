#[doc = "Configuration for shared functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub const fn set_MSTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub const fn set_SLVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[must_use]
    #[inline(always)]
    pub const fn MONEN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub const fn set_MONEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TIMEOUTEN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub const fn set_TIMEOUTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Monitor function Clock Stretching."]
    #[must_use]
    #[inline(always)]
    pub const fn MONCLKSTR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor function Clock Stretching."]
    #[inline(always)]
    pub const fn set_MONCLKSTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[must_use]
    #[inline(always)]
    pub const fn HSCAPABLE(&self) -> super::vals::HSCAPABLE {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::HSCAPABLE::from_bits(val as u8)
    }
    #[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub const fn set_HSCAPABLE(&mut self, val: super::vals::HSCAPABLE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("MSTEN", &self.MSTEN())
            .field("SLVEN", &self.SLVEN())
            .field("MONEN", &self.MONEN())
            .field("TIMEOUTEN", &self.TIMEOUTEN())
            .field("MONCLKSTR", &self.MONCLKSTR())
            .field("HSCAPABLE", &self.HSCAPABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ MSTEN: {=bool:?}, SLVEN: {=bool:?}, MONEN: {=bool:?}, TIMEOUTEN: {=bool:?}, MONCLKSTR: {=bool:?}, HSCAPABLE: {:?} }}",
            self.MSTEN(),
            self.SLVEN(),
            self.MONEN(),
            self.TIMEOUTEN(),
            self.MONCLKSTR(),
            self.HSCAPABLE()
        )
    }
}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CLKDIV(pub u32);
impl CLKDIV {
    #[doc = "This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVVAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub const fn set_DIVVAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CLKDIV {
    #[inline(always)]
    fn default() -> CLKDIV {
        CLKDIV(0)
    }
}
impl core::fmt::Debug for CLKDIV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("DIVVAL", &self.DIVVAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CLKDIV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CLKDIV {{ DIVVAL: {=u16:?} }}", self.DIVVAL())
    }
}
#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID(pub u32);
impl ID {
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[must_use]
    #[inline(always)]
    pub const fn APERTURE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub const fn set_APERTURE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR_REV(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn set_MINOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR_REV(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn set_MAJOR_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ID {
    #[inline(always)]
    fn default() -> ID {
        ID(0)
    }
}
impl core::fmt::Debug for ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("APERTURE", &self.APERTURE())
            .field("MINOR_REV", &self.MINOR_REV())
            .field("MAJOR_REV", &self.MAJOR_REV())
            .field("ID", &self.ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID {{ APERTURE: {=u8:?}, MINOR_REV: {=u8:?}, MAJOR_REV: {=u8:?}, ID: {=u16:?} }}",
            self.APERTURE(),
            self.MINOR_REV(),
            self.MAJOR_REV(),
            self.ID()
        )
    }
}
#[doc = "Interrupt Enable Clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENCLR(pub u32);
impl INTENCLR {
    #[doc = "Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTPENDINGCLR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub const fn set_MSTPENDINGCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTARBLOSSCLR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub const fn set_MSTARBLOSSCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTSTPERRCLR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub const fn set_MSTSTSTPERRCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVPENDINGCLR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Pending interrupt clear."]
    #[inline(always)]
    pub const fn set_SLVPENDINGCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVNOTSTRCLR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub const fn set_SLVNOTSTRCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselect interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDESELCLR(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselect interrupt clear."]
    #[inline(always)]
    pub const fn set_SLVDESELCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor data Ready interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRDYCLR(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub const fn set_MONRDYCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overrun interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn MONOVCLR(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub const fn set_MONOVCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn MONIDLECLR(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle interrupt clear."]
    #[inline(always)]
    pub const fn set_MONIDLECLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn EVENTTIMEOUTCLR(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event time-out interrupt clear."]
    #[inline(always)]
    pub const fn set_EVENTTIMEOUTCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out interrupt clear."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLTIMEOUTCLR(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL time-out interrupt clear."]
    #[inline(always)]
    pub const fn set_SCLTIMEOUTCLR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for INTENCLR {
    #[inline(always)]
    fn default() -> INTENCLR {
        INTENCLR(0)
    }
}
impl core::fmt::Debug for INTENCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENCLR")
            .field("MSTPENDINGCLR", &self.MSTPENDINGCLR())
            .field("MSTARBLOSSCLR", &self.MSTARBLOSSCLR())
            .field("MSTSTSTPERRCLR", &self.MSTSTSTPERRCLR())
            .field("SLVPENDINGCLR", &self.SLVPENDINGCLR())
            .field("SLVNOTSTRCLR", &self.SLVNOTSTRCLR())
            .field("SLVDESELCLR", &self.SLVDESELCLR())
            .field("MONRDYCLR", &self.MONRDYCLR())
            .field("MONOVCLR", &self.MONOVCLR())
            .field("MONIDLECLR", &self.MONIDLECLR())
            .field("EVENTTIMEOUTCLR", &self.EVENTTIMEOUTCLR())
            .field("SCLTIMEOUTCLR", &self.SCLTIMEOUTCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENCLR {{ MSTPENDINGCLR: {=bool:?}, MSTARBLOSSCLR: {=bool:?}, MSTSTSTPERRCLR: {=bool:?}, SLVPENDINGCLR: {=bool:?}, SLVNOTSTRCLR: {=bool:?}, SLVDESELCLR: {=bool:?}, MONRDYCLR: {=bool:?}, MONOVCLR: {=bool:?}, MONIDLECLR: {=bool:?}, EVENTTIMEOUTCLR: {=bool:?}, SCLTIMEOUTCLR: {=bool:?} }}",
            self.MSTPENDINGCLR(),
            self.MSTARBLOSSCLR(),
            self.MSTSTSTPERRCLR(),
            self.SLVPENDINGCLR(),
            self.SLVNOTSTRCLR(),
            self.SLVDESELCLR(),
            self.MONRDYCLR(),
            self.MONOVCLR(),
            self.MONIDLECLR(),
            self.EVENTTIMEOUTCLR(),
            self.SCLTIMEOUTCLR()
        )
    }
}
#[doc = "Interrupt Enable Set and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTENSET(pub u32);
impl INTENSET {
    #[doc = "Master Pending interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTPENDINGEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending interrupt Enable."]
    #[inline(always)]
    pub const fn set_MSTPENDINGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTARBLOSSEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub const fn set_MSTARBLOSSEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTSTPERREN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub const fn set_MSTSTSTPERREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVPENDINGEN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Pending interrupt Enable."]
    #[inline(always)]
    pub const fn set_SLVPENDINGEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVNOTSTREN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub const fn set_SLVNOTSTREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselect interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDESELEN(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub const fn set_SLVDESELEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor data Ready interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRDYEN(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub const fn set_MONRDYEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overrun interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MONOVEN(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub const fn set_MONOVEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MONIDLEEN(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub const fn set_MONIDLEEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EVENTTIMEOUTEN(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event time-out interrupt Enable."]
    #[inline(always)]
    pub const fn set_EVENTTIMEOUTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLTIMEOUTEN(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL time-out interrupt Enable."]
    #[inline(always)]
    pub const fn set_SCLTIMEOUTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for INTENSET {
    #[inline(always)]
    fn default() -> INTENSET {
        INTENSET(0)
    }
}
impl core::fmt::Debug for INTENSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTENSET")
            .field("MSTPENDINGEN", &self.MSTPENDINGEN())
            .field("MSTARBLOSSEN", &self.MSTARBLOSSEN())
            .field("MSTSTSTPERREN", &self.MSTSTSTPERREN())
            .field("SLVPENDINGEN", &self.SLVPENDINGEN())
            .field("SLVNOTSTREN", &self.SLVNOTSTREN())
            .field("SLVDESELEN", &self.SLVDESELEN())
            .field("MONRDYEN", &self.MONRDYEN())
            .field("MONOVEN", &self.MONOVEN())
            .field("MONIDLEEN", &self.MONIDLEEN())
            .field("EVENTTIMEOUTEN", &self.EVENTTIMEOUTEN())
            .field("SCLTIMEOUTEN", &self.SCLTIMEOUTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTENSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTENSET {{ MSTPENDINGEN: {=bool:?}, MSTARBLOSSEN: {=bool:?}, MSTSTSTPERREN: {=bool:?}, SLVPENDINGEN: {=bool:?}, SLVNOTSTREN: {=bool:?}, SLVDESELEN: {=bool:?}, MONRDYEN: {=bool:?}, MONOVEN: {=bool:?}, MONIDLEEN: {=bool:?}, EVENTTIMEOUTEN: {=bool:?}, SCLTIMEOUTEN: {=bool:?} }}",
            self.MSTPENDINGEN(),
            self.MSTARBLOSSEN(),
            self.MSTSTSTPERREN(),
            self.SLVPENDINGEN(),
            self.SLVNOTSTREN(),
            self.SLVDESELEN(),
            self.MONRDYEN(),
            self.MONOVEN(),
            self.MONIDLEEN(),
            self.EVENTTIMEOUTEN(),
            self.SCLTIMEOUTEN()
        )
    }
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Master Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTPENDING(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending."]
    #[inline(always)]
    pub const fn set_MSTPENDING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTARBLOSS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration Loss flag."]
    #[inline(always)]
    pub const fn set_MSTARBLOSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTSTPERR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error flag."]
    #[inline(always)]
    pub const fn set_MSTSTSTPERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVPENDING(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Pending."]
    #[inline(always)]
    pub const fn set_SLVPENDING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching status."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVNOTSTR(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Not Stretching status."]
    #[inline(always)]
    pub const fn set_SLVNOTSTR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselected flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDESEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselected flag."]
    #[inline(always)]
    pub const fn set_SLVDESEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRDY(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Ready."]
    #[inline(always)]
    pub const fn set_MONRDY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overflow flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MONOV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub const fn set_MONOV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MONIDLE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle flag."]
    #[inline(always)]
    pub const fn set_MONIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn EVENTTIMEOUT(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event time-out Interrupt flag."]
    #[inline(always)]
    pub const fn set_EVENTTIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out Interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLTIMEOUT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL time-out Interrupt flag."]
    #[inline(always)]
    pub const fn set_SCLTIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("MSTPENDING", &self.MSTPENDING())
            .field("MSTARBLOSS", &self.MSTARBLOSS())
            .field("MSTSTSTPERR", &self.MSTSTSTPERR())
            .field("SLVPENDING", &self.SLVPENDING())
            .field("SLVNOTSTR", &self.SLVNOTSTR())
            .field("SLVDESEL", &self.SLVDESEL())
            .field("MONRDY", &self.MONRDY())
            .field("MONOV", &self.MONOV())
            .field("MONIDLE", &self.MONIDLE())
            .field("EVENTTIMEOUT", &self.EVENTTIMEOUT())
            .field("SCLTIMEOUT", &self.SCLTIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ MSTPENDING: {=bool:?}, MSTARBLOSS: {=bool:?}, MSTSTSTPERR: {=bool:?}, SLVPENDING: {=bool:?}, SLVNOTSTR: {=bool:?}, SLVDESEL: {=bool:?}, MONRDY: {=bool:?}, MONOV: {=bool:?}, MONIDLE: {=bool:?}, EVENTTIMEOUT: {=bool:?}, SCLTIMEOUT: {=bool:?} }}",
            self.MSTPENDING(),
            self.MSTARBLOSS(),
            self.MSTSTSTPERR(),
            self.SLVPENDING(),
            self.SLVNOTSTR(),
            self.SLVDESEL(),
            self.MONRDY(),
            self.MONOV(),
            self.MONIDLE(),
            self.EVENTTIMEOUT(),
            self.SCLTIMEOUT()
        )
    }
}
#[doc = "Monitor receiver data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MONRXDAT(pub u32);
impl MONRXDAT {
    #[doc = "Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRXDAT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub const fn set_MONRXDAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Monitor Received Start."]
    #[must_use]
    #[inline(always)]
    pub const fn MONSTART(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Received Start."]
    #[inline(always)]
    pub const fn set_MONSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Monitor Received Repeated Start."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRESTART(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Received Repeated Start."]
    #[inline(always)]
    pub const fn set_MONRESTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Monitor Received NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn MONNACK(&self) -> super::vals::MONNACK {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::MONNACK::from_bits(val as u8)
    }
    #[doc = "Monitor Received NACK."]
    #[inline(always)]
    pub const fn set_MONNACK(&mut self, val: super::vals::MONNACK) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for MONRXDAT {
    #[inline(always)]
    fn default() -> MONRXDAT {
        MONRXDAT(0)
    }
}
impl core::fmt::Debug for MONRXDAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MONRXDAT")
            .field("MONRXDAT", &self.MONRXDAT())
            .field("MONSTART", &self.MONSTART())
            .field("MONRESTART", &self.MONRESTART())
            .field("MONNACK", &self.MONNACK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MONRXDAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MONRXDAT {{ MONRXDAT: {=u8:?}, MONSTART: {=bool:?}, MONRESTART: {=bool:?}, MONNACK: {:?} }}",
            self.MONRXDAT(),
            self.MONSTART(),
            self.MONRESTART(),
            self.MONNACK()
        )
    }
}
#[doc = "Master control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSTCTL(pub u32);
impl MSTCTL {
    #[doc = "Master Continue. This bit is write-only."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTCONTINUE(&self) -> super::vals::MSTCONTINUE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MSTCONTINUE::from_bits(val as u8)
    }
    #[doc = "Master Continue. This bit is write-only."]
    #[inline(always)]
    pub const fn set_MSTCONTINUE(&mut self, val: super::vals::MSTCONTINUE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Start control. This bit is write-only."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTART(&self) -> super::vals::MSTSTART {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MSTSTART::from_bits(val as u8)
    }
    #[doc = "Master Start control. This bit is write-only."]
    #[inline(always)]
    pub const fn set_MSTSTART(&mut self, val: super::vals::MSTSTART) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Master Stop control. This bit is write-only."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTOP(&self) -> super::vals::MSTSTOP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::MSTSTOP::from_bits(val as u8)
    }
    #[doc = "Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub const fn set_MSTSTOP(&mut self, val: super::vals::MSTSTOP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTDMA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub const fn set_MSTDMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MSTCTL {
    #[inline(always)]
    fn default() -> MSTCTL {
        MSTCTL(0)
    }
}
impl core::fmt::Debug for MSTCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTCTL")
            .field("MSTCONTINUE", &self.MSTCONTINUE())
            .field("MSTSTART", &self.MSTSTART())
            .field("MSTSTOP", &self.MSTSTOP())
            .field("MSTDMA", &self.MSTDMA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSTCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MSTCTL {{ MSTCONTINUE: {:?}, MSTSTART: {:?}, MSTSTOP: {:?}, MSTDMA: {=bool:?} }}",
            self.MSTCONTINUE(),
            self.MSTSTART(),
            self.MSTSTOP(),
            self.MSTDMA()
        )
    }
}
#[doc = "Combined Master receiver and transmitter data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSTDAT(pub u32);
impl MSTDAT {
    #[doc = "Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for MSTDAT {
    #[inline(always)]
    fn default() -> MSTDAT {
        MSTDAT(0)
    }
}
impl core::fmt::Debug for MSTDAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTDAT")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSTDAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MSTDAT {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = "Master timing configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MSTTIME(pub u32);
impl MSTTIME {
    #[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSCLLOW(&self) -> super::vals::MSTSCLLOW {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MSTSCLLOW::from_bits(val as u8)
    }
    #[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub const fn set_MSTSCLLOW(&mut self, val: super::vals::MSTSCLLOW) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSCLHIGH(&self) -> super::vals::MSTSCLHIGH {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::MSTSCLHIGH::from_bits(val as u8)
    }
    #[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub const fn set_MSTSCLHIGH(&mut self, val: super::vals::MSTSCLHIGH) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for MSTTIME {
    #[inline(always)]
    fn default() -> MSTTIME {
        MSTTIME(0)
    }
}
impl core::fmt::Debug for MSTTIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTTIME")
            .field("MSTSCLLOW", &self.MSTSCLLOW())
            .field("MSTSCLHIGH", &self.MSTSCLHIGH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MSTTIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MSTTIME {{ MSTSCLLOW: {:?}, MSTSCLHIGH: {:?} }}",
            self.MSTSCLLOW(),
            self.MSTSCLHIGH()
        )
    }
}
#[doc = "Slave address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVADR0(pub u32);
impl SLVADR0 {
    #[doc = "Slave Address n Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SADISABLE(&self) -> super::vals::SLVADR0_SADISABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SLVADR0_SADISABLE::from_bits(val as u8)
    }
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub const fn set_SADISABLE(&mut self, val: super::vals::SLVADR0_SADISABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVADR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub const fn set_SLVADR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTONACK(&self) -> super::vals::AUTONACK {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AUTONACK::from_bits(val as u8)
    }
    #[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub const fn set_AUTONACK(&mut self, val: super::vals::AUTONACK) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for SLVADR0 {
    #[inline(always)]
    fn default() -> SLVADR0 {
        SLVADR0(0)
    }
}
impl core::fmt::Debug for SLVADR0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADR0")
            .field("SADISABLE", &self.SADISABLE())
            .field("SLVADR", &self.SLVADR())
            .field("AUTONACK", &self.AUTONACK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVADR0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVADR0 {{ SADISABLE: {:?}, SLVADR: {=u8:?}, AUTONACK: {:?} }}",
            self.SADISABLE(),
            self.SLVADR(),
            self.AUTONACK()
        )
    }
}
#[doc = "Slave address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVADR1(pub u32);
impl SLVADR1 {
    #[doc = "Slave Address n Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SADISABLE(&self) -> super::vals::SLVADR1_SADISABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SLVADR1_SADISABLE::from_bits(val as u8)
    }
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub const fn set_SADISABLE(&mut self, val: super::vals::SLVADR1_SADISABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVADR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub const fn set_SLVADR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for SLVADR1 {
    #[inline(always)]
    fn default() -> SLVADR1 {
        SLVADR1(0)
    }
}
impl core::fmt::Debug for SLVADR1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADR1")
            .field("SADISABLE", &self.SADISABLE())
            .field("SLVADR", &self.SLVADR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVADR1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVADR1 {{ SADISABLE: {:?}, SLVADR: {=u8:?} }}",
            self.SADISABLE(),
            self.SLVADR()
        )
    }
}
#[doc = "Slave address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVADR2(pub u32);
impl SLVADR2 {
    #[doc = "Slave Address n Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SADISABLE(&self) -> super::vals::SLVADR2_SADISABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SLVADR2_SADISABLE::from_bits(val as u8)
    }
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub const fn set_SADISABLE(&mut self, val: super::vals::SLVADR2_SADISABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVADR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub const fn set_SLVADR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for SLVADR2 {
    #[inline(always)]
    fn default() -> SLVADR2 {
        SLVADR2(0)
    }
}
impl core::fmt::Debug for SLVADR2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADR2")
            .field("SADISABLE", &self.SADISABLE())
            .field("SLVADR", &self.SLVADR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVADR2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVADR2 {{ SADISABLE: {:?}, SLVADR: {=u8:?} }}",
            self.SADISABLE(),
            self.SLVADR()
        )
    }
}
#[doc = "Slave address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVADR3(pub u32);
impl SLVADR3 {
    #[doc = "Slave Address n Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn SADISABLE(&self) -> super::vals::SLVADR3_SADISABLE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SLVADR3_SADISABLE::from_bits(val as u8)
    }
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub const fn set_SADISABLE(&mut self, val: super::vals::SLVADR3_SADISABLE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVADR(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub const fn set_SLVADR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for SLVADR3 {
    #[inline(always)]
    fn default() -> SLVADR3 {
        SLVADR3(0)
    }
}
impl core::fmt::Debug for SLVADR3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVADR3")
            .field("SADISABLE", &self.SADISABLE())
            .field("SLVADR", &self.SLVADR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVADR3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVADR3 {{ SADISABLE: {:?}, SLVADR: {=u8:?} }}",
            self.SADISABLE(),
            self.SLVADR()
        )
    }
}
#[doc = "Slave control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVCTL(pub u32);
impl SLVCTL {
    #[doc = "Slave Continue."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVCONTINUE(&self) -> super::vals::SLVCONTINUE {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SLVCONTINUE::from_bits(val as u8)
    }
    #[doc = "Slave Continue."]
    #[inline(always)]
    pub const fn set_SLVCONTINUE(&mut self, val: super::vals::SLVCONTINUE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave NACK."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVNACK(&self) -> super::vals::SLVNACK {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SLVNACK::from_bits(val as u8)
    }
    #[doc = "Slave NACK."]
    #[inline(always)]
    pub const fn set_SLVNACK(&mut self, val: super::vals::SLVNACK) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slave DMA enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDMA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Slave DMA enable."]
    #[inline(always)]
    pub const fn set_SLVDMA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOACK(&self) -> super::vals::AUTOACK {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::AUTOACK::from_bits(val as u8)
    }
    #[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub const fn set_AUTOACK(&mut self, val: super::vals::AUTOACK) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTOMATCHREAD(&self) -> super::vals::AUTOMATCHREAD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::AUTOMATCHREAD::from_bits(val as u8)
    }
    #[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub const fn set_AUTOMATCHREAD(&mut self, val: super::vals::AUTOMATCHREAD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for SLVCTL {
    #[inline(always)]
    fn default() -> SLVCTL {
        SLVCTL(0)
    }
}
impl core::fmt::Debug for SLVCTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVCTL")
            .field("SLVCONTINUE", &self.SLVCONTINUE())
            .field("SLVNACK", &self.SLVNACK())
            .field("SLVDMA", &self.SLVDMA())
            .field("AUTOACK", &self.AUTOACK())
            .field("AUTOMATCHREAD", &self.AUTOMATCHREAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVCTL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVCTL {{ SLVCONTINUE: {:?}, SLVNACK: {:?}, SLVDMA: {=bool:?}, AUTOACK: {:?}, AUTOMATCHREAD: {:?} }}",
            self.SLVCONTINUE(),
            self.SLVNACK(),
            self.SLVDMA(),
            self.AUTOACK(),
            self.AUTOMATCHREAD()
        )
    }
}
#[doc = "Combined Slave receiver and transmitter data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVDAT(pub u32);
impl SLVDAT {
    #[doc = "Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[inline(always)]
    pub const fn set_DATA(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SLVDAT {
    #[inline(always)]
    fn default() -> SLVDAT {
        SLVDAT(0)
    }
}
impl core::fmt::Debug for SLVDAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVDAT")
            .field("DATA", &self.DATA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVDAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SLVDAT {{ DATA: {=u8:?} }}", self.DATA())
    }
}
#[doc = "Slave Qualification for address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SLVQUAL0(pub u32);
impl SLVQUAL0 {
    #[doc = "Qualify mode for slave address 0."]
    #[must_use]
    #[inline(always)]
    pub const fn QUALMODE0(&self) -> super::vals::QUALMODE0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::QUALMODE0::from_bits(val as u8)
    }
    #[doc = "Qualify mode for slave address 0."]
    #[inline(always)]
    pub const fn set_QUALMODE0(&mut self, val: super::vals::QUALMODE0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVQUAL0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub const fn set_SLVQUAL0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for SLVQUAL0 {
    #[inline(always)]
    fn default() -> SLVQUAL0 {
        SLVQUAL0(0)
    }
}
impl core::fmt::Debug for SLVQUAL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLVQUAL0")
            .field("QUALMODE0", &self.QUALMODE0())
            .field("SLVQUAL0", &self.SLVQUAL0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SLVQUAL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SLVQUAL0 {{ QUALMODE0: {:?}, SLVQUAL0: {=u8:?} }}",
            self.QUALMODE0(),
            self.SLVQUAL0()
        )
    }
}
#[doc = "Status register for Master, Slave, and Monitor functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTPENDING(&self) -> super::vals::MSTPENDING {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MSTPENDING::from_bits(val as u8)
    }
    #[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub const fn set_MSTPENDING(&mut self, val: super::vals::MSTPENDING) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTATE(&self) -> super::vals::MSTSTATE {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::MSTSTATE::from_bits(val as u8)
    }
    #[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub const fn set_MSTSTATE(&mut self, val: super::vals::MSTSTATE) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTARBLOSS(&self) -> super::vals::MSTARBLOSS {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MSTARBLOSS::from_bits(val as u8)
    }
    #[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub const fn set_MSTARBLOSS(&mut self, val: super::vals::MSTARBLOSS) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[must_use]
    #[inline(always)]
    pub const fn MSTSTSTPERR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub const fn set_MSTSTSTPERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVPENDING(&self) -> super::vals::SLVPENDING {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SLVPENDING::from_bits(val as u8)
    }
    #[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub const fn set_SLVPENDING(&mut self, val: super::vals::SLVPENDING) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVSTATE(&self) -> super::vals::SLVSTATE {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::SLVSTATE::from_bits(val as u8)
    }
    #[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub const fn set_SLVSTATE(&mut self, val: super::vals::SLVSTATE) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVNOTSTR(&self) -> super::vals::SLVNOTSTR {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SLVNOTSTR::from_bits(val as u8)
    }
    #[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub const fn set_SLVNOTSTR(&mut self, val: super::vals::SLVNOTSTR) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVIDX(&self) -> super::vals::SLVIDX {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::SLVIDX::from_bits(val as u8)
    }
    #[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub const fn set_SLVIDX(&mut self, val: super::vals::SLVIDX) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVSEL(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub const fn set_SLVSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SLVDESEL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_SLVDESEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[must_use]
    #[inline(always)]
    pub const fn MONRDY(&self) -> super::vals::MONRDY {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::MONRDY::from_bits(val as u8)
    }
    #[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub const fn set_MONRDY(&mut self, val: super::vals::MONRDY) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overflow flag."]
    #[must_use]
    #[inline(always)]
    pub const fn MONOV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub const fn set_MONOV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn MONACTIVE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub const fn set_MONACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn MONIDLE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_MONIDLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[must_use]
    #[inline(always)]
    pub const fn EVENTTIMEOUT(&self) -> super::vals::EVENTTIMEOUT {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::EVENTTIMEOUT::from_bits(val as u8)
    }
    #[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub const fn set_EVENTTIMEOUT(&mut self, val: super::vals::EVENTTIMEOUT) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SCLTIMEOUT(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn set_SCLTIMEOUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("MSTPENDING", &self.MSTPENDING())
            .field("MSTSTATE", &self.MSTSTATE())
            .field("MSTARBLOSS", &self.MSTARBLOSS())
            .field("MSTSTSTPERR", &self.MSTSTSTPERR())
            .field("SLVPENDING", &self.SLVPENDING())
            .field("SLVSTATE", &self.SLVSTATE())
            .field("SLVNOTSTR", &self.SLVNOTSTR())
            .field("SLVIDX", &self.SLVIDX())
            .field("SLVSEL", &self.SLVSEL())
            .field("SLVDESEL", &self.SLVDESEL())
            .field("MONRDY", &self.MONRDY())
            .field("MONOV", &self.MONOV())
            .field("MONACTIVE", &self.MONACTIVE())
            .field("MONIDLE", &self.MONIDLE())
            .field("EVENTTIMEOUT", &self.EVENTTIMEOUT())
            .field("SCLTIMEOUT", &self.SCLTIMEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ MSTPENDING: {:?}, MSTSTATE: {:?}, MSTARBLOSS: {:?}, MSTSTSTPERR: {=bool:?}, SLVPENDING: {:?}, SLVSTATE: {:?}, SLVNOTSTR: {:?}, SLVIDX: {:?}, SLVSEL: {=bool:?}, SLVDESEL: {=bool:?}, MONRDY: {:?}, MONOV: {=bool:?}, MONACTIVE: {=bool:?}, MONIDLE: {=bool:?}, EVENTTIMEOUT: {:?}, SCLTIMEOUT: {=bool:?} }}",
            self.MSTPENDING(),
            self.MSTSTATE(),
            self.MSTARBLOSS(),
            self.MSTSTSTPERR(),
            self.SLVPENDING(),
            self.SLVSTATE(),
            self.SLVNOTSTR(),
            self.SLVIDX(),
            self.SLVSEL(),
            self.SLVDESEL(),
            self.MONRDY(),
            self.MONOV(),
            self.MONACTIVE(),
            self.MONIDLE(),
            self.EVENTTIMEOUT(),
            self.SCLTIMEOUT()
        )
    }
}
#[doc = "Time-out value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMEOUT(pub u32);
impl TIMEOUT {
    #[doc = "Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[must_use]
    #[inline(always)]
    pub const fn TOMIN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub const fn set_TOMIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn TO(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub const fn set_TO(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for TIMEOUT {
    #[inline(always)]
    fn default() -> TIMEOUT {
        TIMEOUT(0)
    }
}
impl core::fmt::Debug for TIMEOUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("TOMIN", &self.TOMIN())
            .field("TO", &self.TO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMEOUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TIMEOUT {{ TOMIN: {=u8:?}, TO: {=u16:?} }}",
            self.TOMIN(),
            self.TO()
        )
    }
}
