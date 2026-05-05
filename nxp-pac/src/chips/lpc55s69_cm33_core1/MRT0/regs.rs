#[doc = "MRT Control register. This register controls the MRT modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Enable the TIMERn interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn INTEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the TIMERn interrupt."]
    #[inline(always)]
    pub const fn set_INTEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects timer mode."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::MODE {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::MODE::from_bits(val as u8)
    }
    #[doc = "Selects timer mode."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::MODE) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("INTEN", &self.INTEN())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ INTEN: {=bool:?}, MODE: {:?} }}",
            self.INTEN(),
            self.MODE()
        )
    }
}
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDLE_CH(pub u32);
impl IDLE_CH {
    #[doc = "Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn CHAN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub const fn set_CHAN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IDLE_CH {
    #[inline(always)]
    fn default() -> IDLE_CH {
        IDLE_CH(0)
    }
}
impl core::fmt::Debug for IDLE_CH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CH")
            .field("CHAN", &self.CHAN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDLE_CH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IDLE_CH {{ CHAN: {=u8:?} }}", self.CHAN())
    }
}
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTVAL(pub u32);
impl INTVAL {
    #[doc = "Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[must_use]
    #[inline(always)]
    pub const fn IVALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub const fn set_IVALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[must_use]
    #[inline(always)]
    pub const fn LOAD(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub const fn set_LOAD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for INTVAL {
    #[inline(always)]
    fn default() -> INTVAL {
        INTVAL(0)
    }
}
impl core::fmt::Debug for INTVAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTVAL")
            .field("IVALUE", &self.IVALUE())
            .field("LOAD", &self.LOAD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTVAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTVAL {{ IVALUE: {=u32:?}, LOAD: {=bool:?} }}",
            self.IVALUE(),
            self.LOAD()
        )
    }
}
#[doc = "Global interrupt flag register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQ_FLAG(pub u32);
impl IRQ_FLAG {
    #[doc = "Monitors the interrupt flag of TIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn GFLAG0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub const fn set_GFLAG0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn GFLAG1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub const fn set_GFLAG1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn GFLAG2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub const fn set_GFLAG2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn GFLAG3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub const fn set_GFLAG3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IRQ_FLAG {
    #[inline(always)]
    fn default() -> IRQ_FLAG {
        IRQ_FLAG(0)
    }
}
impl core::fmt::Debug for IRQ_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ_FLAG")
            .field("GFLAG0", &self.GFLAG0())
            .field("GFLAG1", &self.GFLAG1())
            .field("GFLAG2", &self.GFLAG2())
            .field("GFLAG3", &self.GFLAG3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQ_FLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IRQ_FLAG {{ GFLAG0: {=bool:?}, GFLAG1: {=bool:?}, GFLAG2: {=bool:?}, GFLAG3: {=bool:?} }}",
            self.GFLAG0(),
            self.GFLAG1(),
            self.GFLAG2(),
            self.GFLAG3()
        )
    }
}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MODCFG(pub u32);
impl MODCFG {
    #[doc = "Identifies the number of channels in this MRT.(4 channels on this device.)."]
    #[must_use]
    #[inline(always)]
    pub const fn NOC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Identifies the number of channels in this MRT.(4 channels on this device.)."]
    #[inline(always)]
    pub const fn set_NOC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Identifies the number of timer bits in this MRT. (24 bits wide on this device.)."]
    #[must_use]
    #[inline(always)]
    pub const fn NOB(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "Identifies the number of timer bits in this MRT. (24 bits wide on this device.)."]
    #[inline(always)]
    pub const fn set_NOB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[must_use]
    #[inline(always)]
    pub const fn MULTITASK(&self) -> super::vals::MULTITASK {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::MULTITASK::from_bits(val as u8)
    }
    #[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub const fn set_MULTITASK(&mut self, val: super::vals::MULTITASK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MODCFG {
    #[inline(always)]
    fn default() -> MODCFG {
        MODCFG(0)
    }
}
impl core::fmt::Debug for MODCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODCFG")
            .field("NOC", &self.NOC())
            .field("NOB", &self.NOB())
            .field("MULTITASK", &self.MULTITASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MODCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MODCFG {{ NOC: {=u8:?}, NOB: {=u8:?}, MULTITASK: {:?} }}",
            self.NOC(),
            self.NOB(),
            self.MULTITASK()
        )
    }
}
#[doc = "MRT Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Monitors the interrupt flag."]
    #[must_use]
    #[inline(always)]
    pub const fn INTFLAG(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag."]
    #[inline(always)]
    pub const fn set_INTFLAG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the state of TIMERn. This bit is read-only."]
    #[must_use]
    #[inline(always)]
    pub const fn RUN(&self) -> super::vals::RUN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RUN::from_bits(val as u8)
    }
    #[doc = "Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub const fn set_RUN(&mut self, val: super::vals::RUN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[must_use]
    #[inline(always)]
    pub const fn INUSE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub const fn set_INUSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("INTFLAG", &self.INTFLAG())
            .field("RUN", &self.RUN())
            .field("INUSE", &self.INUSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ INTFLAG: {=bool:?}, RUN: {:?}, INUSE: {=bool:?} }}",
            self.INTFLAG(),
            self.RUN(),
            self.INUSE()
        )
    }
}
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER(pub u32);
impl TIMER {
    #[doc = "Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
    #[must_use]
    #[inline(always)]
    pub const fn VALUE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
    #[inline(always)]
    pub const fn set_VALUE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for TIMER {
    #[inline(always)]
    fn default() -> TIMER {
        TIMER(0)
    }
}
impl core::fmt::Debug for TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER")
            .field("VALUE", &self.VALUE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER {{ VALUE: {=u32:?} }}", self.VALUE())
    }
}
