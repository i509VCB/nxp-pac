#[doc = "USARTs."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USART {
    ptr: *mut u8,
}
unsafe impl Send for USART {}
unsafe impl Sync for USART {}
impl USART {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USART Control register. USART control settings that are more likely to change during operation."]
    #[inline(always)]
    pub const fn CTL(self) -> crate::common::Reg<regs::CTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USART Status register. The complete status value can be read here. Writing ones clears some bits in the register. Some bits can be cleared by writing a 1 to them."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Enable read and Set register for USART (not FIFO) status. Contains individual interrupt enable bits for each potential USART interrupt. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::common::Reg<regs::INTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable Clear register. Allows clearing any combination of bits in the INTENSET register. Writing a 1 to any implemented bit position causes the corresponding bit to be cleared."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::common::Reg<regs::INTENCLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Baud Rate Generator register. 16-bit integer baud rate divisor value."]
    #[inline(always)]
    pub const fn BRG(self) -> crate::common::Reg<regs::BRG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt status register. Reflects interrupts that are currently enabled."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Oversample selection register for asynchronous communication."]
    #[inline(always)]
    pub const fn OSR(self) -> crate::common::Reg<regs::OSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Address register for automatic address matching."]
    #[inline(always)]
    pub const fn ADDR(self) -> crate::common::Reg<regs::ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
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
