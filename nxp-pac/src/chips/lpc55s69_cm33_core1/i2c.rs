#[doc = "I2C-bus interfaces"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c {
    ptr: *mut u8,
}
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
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
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Interrupt Enable Set and read register."]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Interrupt Enable Clear register."]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "Time-out value register."]
    #[inline(always)]
    pub const fn timeout(self) -> crate::common::Reg<regs::Timeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0810usize) as _) }
    }
    #[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    #[inline(always)]
    pub const fn clkdiv(self) -> crate::common::Reg<regs::Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0814usize) as _) }
    }
    #[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
    #[doc = "Master control register."]
    #[inline(always)]
    pub const fn mstctl(self) -> crate::common::Reg<regs::Mstctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "Master timing configuration."]
    #[inline(always)]
    pub const fn msttime(self) -> crate::common::Reg<regs::Msttime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "Combined Master receiver and transmitter data register."]
    #[inline(always)]
    pub const fn mstdat(self) -> crate::common::Reg<regs::Mstdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0828usize) as _) }
    }
    #[doc = "Slave control register."]
    #[inline(always)]
    pub const fn slvctl(self) -> crate::common::Reg<regs::Slvctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "Combined Slave receiver and transmitter data register."]
    #[inline(always)]
    pub const fn slvdat(self) -> crate::common::Reg<regs::Slvdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0844usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn slvadr0(self) -> crate::common::Reg<regs::Slvadr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0848usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn slvadr1(self) -> crate::common::Reg<regs::Slvadr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x084cusize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn slvadr2(self) -> crate::common::Reg<regs::Slvadr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize) as _) }
    }
    #[doc = "Slave address register."]
    #[inline(always)]
    pub const fn slvadr3(self) -> crate::common::Reg<regs::Slvadr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0854usize) as _) }
    }
    #[doc = "Slave Qualification for address 0."]
    #[inline(always)]
    pub const fn slvqual0(self) -> crate::common::Reg<regs::Slvqual0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0858usize) as _) }
    }
    #[doc = "Monitor receiver data register."]
    #[inline(always)]
    pub const fn monrxdat(self) -> crate::common::Reg<regs::Monrxdat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
