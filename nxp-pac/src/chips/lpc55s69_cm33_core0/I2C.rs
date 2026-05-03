#[doc = "I2C-bus interfaces."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C {
    ptr: *mut u8,
}
unsafe impl Send for I2C {}
unsafe impl Sync for I2C {}
impl I2C {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration for shared functions."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Interrupt Enable Set and read register."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::common::Reg<regs::INTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Interrupt Enable Clear register."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::common::Reg<regs::INTENCLR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Time-out value register."]
    #[inline(always)]
    pub const fn TIMEOUT(self) -> crate::common::Reg<regs::TIMEOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    #[inline(always)]
    pub const fn CLKDIV(self) -> crate::common::Reg<regs::CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn INTSTAT(self) -> crate::common::Reg<regs::INTSTAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "Master control register."]
    #[inline(always)]
    pub const fn MSTCTL(self) -> crate::common::Reg<regs::MSTCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "Master timing configuration."]
    #[inline(always)]
    pub const fn MSTTIME(self) -> crate::common::Reg<regs::MSTTIME, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "Combined Master receiver and transmitter data register."]
    #[inline(always)]
    pub const fn MSTDAT(self) -> crate::common::Reg<regs::MSTDAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[doc = "Slave control register."]
    #[inline(always)]
    pub const fn SLVCTL(self) -> crate::common::Reg<regs::SLVCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "Combined Slave receiver and transmitter data register."]
    #[inline(always)]
    pub const fn SLVDAT(self) -> crate::common::Reg<regs::SLVDAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn SLVADR0(self) -> crate::common::Reg<regs::SLVADR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn SLVADR1(self) -> crate::common::Reg<regs::SLVADR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084cusize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn SLVADR2(self) -> crate::common::Reg<regs::SLVADR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn SLVADR3(self) -> crate::common::Reg<regs::SLVADR3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[doc = "Slave Qualification for address 0."]
    #[inline(always)]
    pub const fn SLVQUAL0(self) -> crate::common::Reg<regs::SLVQUAL0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[doc = "Monitor receiver data register."]
    #[inline(always)]
    pub const fn MONRXDAT(self) -> crate::common::Reg<regs::MONRXDAT, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<regs::ID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
