#[doc = "Serial Peripheral Interfaces (SPI)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SPI {
    ptr: *mut u8,
}
unsafe impl Send for SPI {}
unsafe impl Sync for SPI {}
impl SPI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPI Configuration register."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SPI Delay register."]
    #[inline(always)]
    pub const fn DLY(self) -> crate::common::Reg<regs::DLY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::common::Reg<regs::INTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::common::Reg<regs::INTENCLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "SPI clock Divider."]
    #[inline(always)]
    pub const fn DIV(self) -> crate::common::Reg<regs::DIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "SPI Interrupt Status."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn FIFOCFG(self) -> crate::common::Reg<regs::FIFOCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "FIFO status register."]
    #[inline(always)]
    pub const fn FIFOSTAT(self) -> crate::common::Reg<regs::FIFOSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn FIFOTRIG(self) -> crate::common::Reg<regs::FIFOTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn FIFOINTENSET(self) -> crate::common::Reg<regs::FIFOINTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn FIFOINTENCLR(self) -> crate::common::Reg<regs::FIFOINTENCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e14usize) as _) }
    }
    #[doc = "FIFO interrupt status register."]
    #[inline(always)]
    pub const fn FIFOINTSTAT(self) -> crate::common::Reg<regs::FIFOINTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e18usize) as _) }
    }
    #[doc = "FIFO write data."]
    #[inline(always)]
    pub const fn FIFOWR(self) -> crate::common::Reg<regs::FIFOWR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize) as _) }
    }
    #[doc = "FIFO read data."]
    #[inline(always)]
    pub const fn FIFORD(self) -> crate::common::Reg<regs::FIFORD, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e30usize) as _) }
    }
    #[doc = "FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn FIFORDNOPOP(self) -> crate::common::Reg<regs::FIFORDNOPOP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "FIFO size register."]
    #[inline(always)]
    pub const fn FIFOSIZE(self) -> crate::common::Reg<regs::FIFOSIZE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e48usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
