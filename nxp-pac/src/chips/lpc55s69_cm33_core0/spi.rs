#[doc = "Serial Peripheral Interfaces (SPI)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPI Configuration register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SPI Delay register"]
    #[inline(always)]
    pub const fn dly(self) -> crate::common::Reg<regs::Dly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "SPI clock Divider"]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "SPI Interrupt Status"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "FIFO configuration and enable register."]
    #[inline(always)]
    pub const fn fifocfg(self) -> crate::common::Reg<regs::Fifocfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
    #[doc = "FIFO status register."]
    #[inline(always)]
    pub const fn fifostat(self) -> crate::common::Reg<regs::Fifostat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "FIFO trigger settings for interrupt and DMA request."]
    #[inline(always)]
    pub const fn fifotrig(self) -> crate::common::Reg<regs::Fifotrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e08usize) as _) }
    }
    #[doc = "FIFO interrupt enable set (enable) and read register."]
    #[inline(always)]
    pub const fn fifointenset(self) -> crate::common::Reg<regs::Fifointenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "FIFO interrupt enable clear (disable) and read register."]
    #[inline(always)]
    pub const fn fifointenclr(self) -> crate::common::Reg<regs::Fifointenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e14usize) as _) }
    }
    #[doc = "FIFO interrupt status register."]
    #[inline(always)]
    pub const fn fifointstat(self) -> crate::common::Reg<regs::Fifointstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e18usize) as _) }
    }
    #[doc = "FIFO write data."]
    #[inline(always)]
    pub const fn fifowr(self) -> crate::common::Reg<regs::Fifowr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize) as _) }
    }
    #[doc = "FIFO read data."]
    #[inline(always)]
    pub const fn fiford(self) -> crate::common::Reg<regs::Fiford, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e30usize) as _) }
    }
    #[doc = "FIFO data read with no FIFO pop."]
    #[inline(always)]
    pub const fn fifordnopop(self) -> crate::common::Reg<regs::Fifordnopop, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "FIFO size register"]
    #[inline(always)]
    pub const fn fifosize(self) -> crate::common::Reg<regs::Fifosize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e48usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
