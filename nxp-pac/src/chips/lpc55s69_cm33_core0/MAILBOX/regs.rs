#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQ(pub u32);
impl IRQ {
    #[doc = "If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[must_use]
    #[inline(always)]
    pub const fn INTREQ(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub const fn set_INTREQ(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQ {
    #[inline(always)]
    fn default() -> IRQ {
        IRQ(0)
    }
}
impl core::fmt::Debug for IRQ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQ")
            .field("INTREQ", &self.INTREQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IRQ {{ INTREQ: {=u32:?} }}", self.INTREQ())
    }
}
#[doc = "Clear bits in IRQ0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQCLR(pub u32);
impl IRQCLR {
    #[doc = "Writing 1 clears the corresponding bit in the IRQ0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn INTREQCLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing 1 clears the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub const fn set_INTREQCLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQCLR {
    #[inline(always)]
    fn default() -> IRQCLR {
        IRQCLR(0)
    }
}
impl core::fmt::Debug for IRQCLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQCLR")
            .field("INTREQCLR", &self.INTREQCLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQCLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IRQCLR {{ INTREQCLR: {=u32:?} }}", self.INTREQCLR())
    }
}
#[doc = "Set bits in IRQ0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IRQSET(pub u32);
impl IRQSET {
    #[doc = "Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn INTREQSET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub const fn set_INTREQSET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IRQSET {
    #[inline(always)]
    fn default() -> IRQSET {
        IRQSET(0)
    }
}
impl core::fmt::Debug for IRQSET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQSET")
            .field("INTREQSET", &self.INTREQSET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IRQSET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IRQSET {{ INTREQSET: {=u32:?} }}", self.INTREQSET())
    }
}
#[doc = "Mutual exclusion register\\[1\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MUTEX(pub u32);
impl MUTEX {
    #[doc = "Cleared when read, set when written. See usage description above."]
    #[must_use]
    #[inline(always)]
    pub const fn EX(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub const fn set_EX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MUTEX {
    #[inline(always)]
    fn default() -> MUTEX {
        MUTEX(0)
    }
}
impl core::fmt::Debug for MUTEX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUTEX").field("EX", &self.EX()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MUTEX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MUTEX {{ EX: {=bool:?} }}", self.EX())
    }
}
