#[doc = "Interrupt request register for the Cortex-M0+ CPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[must_use]
    #[inline(always)]
    pub const fn intreq(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "If any bit is set, an interrupt request is sent to the Cortex-M0+ interrupt controller."]
    #[inline(always)]
    pub const fn set_intreq(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        Irq(0)
    }
}
impl core::fmt::Debug for Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irq")
            .field("intreq", &self.intreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irq {{ intreq: {=u32:?} }}", self.intreq())
    }
}
#[doc = "Clear bits in IRQ0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqclr(pub u32);
impl Irqclr {
    #[doc = "Writing 1 clears the corresponding bit in the IRQ0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn intreqclr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing 1 clears the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub const fn set_intreqclr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Irqclr {
    #[inline(always)]
    fn default() -> Irqclr {
        Irqclr(0)
    }
}
impl core::fmt::Debug for Irqclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqclr")
            .field("intreqclr", &self.intreqclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqclr {{ intreqclr: {=u32:?} }}", self.intreqclr())
    }
}
#[doc = "Set bits in IRQ0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqset(pub u32);
impl Irqset {
    #[doc = "Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[must_use]
    #[inline(always)]
    pub const fn intreqset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing 1 sets the corresponding bit in the IRQ0 register."]
    #[inline(always)]
    pub const fn set_intreqset(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Irqset {
    #[inline(always)]
    fn default() -> Irqset {
        Irqset(0)
    }
}
impl core::fmt::Debug for Irqset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqset")
            .field("intreqset", &self.intreqset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqset {{ intreqset: {=u32:?} }}", self.intreqset())
    }
}
#[doc = "Mutual exclusion register\\[1\\]"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mutex(pub u32);
impl Mutex {
    #[doc = "Cleared when read, set when written. See usage description above."]
    #[must_use]
    #[inline(always)]
    pub const fn ex(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cleared when read, set when written. See usage description above."]
    #[inline(always)]
    pub const fn set_ex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Mutex {
    #[inline(always)]
    fn default() -> Mutex {
        Mutex(0)
    }
}
impl core::fmt::Debug for Mutex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mutex").field("ex", &self.ex()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mutex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mutex {{ ex: {=bool:?} }}", self.ex())
    }
}
