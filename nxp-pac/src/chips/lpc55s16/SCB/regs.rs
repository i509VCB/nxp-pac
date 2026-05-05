#[doc = "Application Interrupt and Reset Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AIRCR(pub u32);
impl AIRCR {
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTCLRACTIVE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_VECTCLRACTIVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRESETREQ(&self) -> super::vals::SYSRESETREQ {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SYSRESETREQ::from_bits(val as u8)
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[inline(always)]
    pub const fn set_SYSRESETREQ(&mut self, val: super::vals::SYSRESETREQ) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSRESETREQS(&self) -> super::vals::SYSRESETREQS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SYSRESETREQS::from_bits(val as u8)
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_SYSRESETREQS(&mut self, val: super::vals::SYSRESETREQS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIGROUP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_PRIGROUP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn BFHFNMINS(&self) -> super::vals::BFHFNMINS {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::BFHFNMINS::from_bits(val as u8)
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[inline(always)]
    pub const fn set_BFHFNMINS(&mut self, val: super::vals::BFHFNMINS) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn PRIS(&self) -> super::vals::PRIS {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PRIS::from_bits(val as u8)
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_PRIS(&mut self, val: super::vals::PRIS) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDIANNESS(&self) -> super::vals::ENDIANNESS {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::ENDIANNESS::from_bits(val as u8)
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_ENDIANNESS(&mut self, val: super::vals::ENDIANNESS) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn VECTKEY(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[inline(always)]
    pub const fn set_VECTKEY(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for AIRCR {
    #[inline(always)]
    fn default() -> AIRCR {
        AIRCR(0)
    }
}
impl core::fmt::Debug for AIRCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIRCR")
            .field("VECTCLRACTIVE", &self.VECTCLRACTIVE())
            .field("SYSRESETREQ", &self.SYSRESETREQ())
            .field("SYSRESETREQS", &self.SYSRESETREQS())
            .field("PRIGROUP", &self.PRIGROUP())
            .field("BFHFNMINS", &self.BFHFNMINS())
            .field("PRIS", &self.PRIS())
            .field("ENDIANNESS", &self.ENDIANNESS())
            .field("VECTKEY", &self.VECTKEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AIRCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AIRCR {{ VECTCLRACTIVE: {=bool:?}, SYSRESETREQ: {:?}, SYSRESETREQS: {:?}, PRIGROUP: {=u8:?}, BFHFNMINS: {:?}, PRIS: {:?}, ENDIANNESS: {:?}, VECTKEY: {=u16:?} }}",
            self.VECTCLRACTIVE(),
            self.SYSRESETREQ(),
            self.SYSRESETREQS(),
            self.PRIGROUP(),
            self.BFHFNMINS(),
            self.PRIS(),
            self.ENDIANNESS(),
            self.VECTKEY()
        )
    }
}
#[doc = "Non-secure Access Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NSACR(pub u32);
impl NSACR {
    #[doc = "CP0 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CP0 access."]
    #[inline(always)]
    pub const fn set_CP0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CP1 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CP1 access."]
    #[inline(always)]
    pub const fn set_CP1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "CP2 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "CP2 access."]
    #[inline(always)]
    pub const fn set_CP2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "CP3 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "CP3 access."]
    #[inline(always)]
    pub const fn set_CP3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CP4 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CP4 access."]
    #[inline(always)]
    pub const fn set_CP4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CP5 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CP5 access."]
    #[inline(always)]
    pub const fn set_CP5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "CP6 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "CP6 access."]
    #[inline(always)]
    pub const fn set_CP6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "CP7 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "CP7 access."]
    #[inline(always)]
    pub const fn set_CP7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CP10 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CP10 access."]
    #[inline(always)]
    pub const fn set_CP10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "CP11 access."]
    #[must_use]
    #[inline(always)]
    pub const fn CP11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CP11 access."]
    #[inline(always)]
    pub const fn set_CP11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for NSACR {
    #[inline(always)]
    fn default() -> NSACR {
        NSACR(0)
    }
}
impl core::fmt::Debug for NSACR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSACR")
            .field("CP0", &self.CP0())
            .field("CP1", &self.CP1())
            .field("CP2", &self.CP2())
            .field("CP3", &self.CP3())
            .field("CP4", &self.CP4())
            .field("CP5", &self.CP5())
            .field("CP6", &self.CP6())
            .field("CP7", &self.CP7())
            .field("CP10", &self.CP10())
            .field("CP11", &self.CP11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NSACR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NSACR {{ CP0: {=bool:?}, CP1: {=bool:?}, CP2: {=bool:?}, CP3: {=bool:?}, CP4: {=bool:?}, CP5: {=bool:?}, CP6: {=bool:?}, CP7: {=bool:?}, CP10: {=bool:?}, CP11: {=bool:?} }}",
            self.CP0(),
            self.CP1(),
            self.CP2(),
            self.CP3(),
            self.CP4(),
            self.CP5(),
            self.CP6(),
            self.CP7(),
            self.CP10(),
            self.CP11()
        )
    }
}
#[doc = "The SCR controls features of entry to and exit from low-power state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCR(pub u32);
impl SCR {
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPONEXIT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_SLEEPONEXIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPDEEP(&self) -> super::vals::SLEEPDEEP {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SLEEPDEEP::from_bits(val as u8)
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_SLEEPDEEP(&mut self, val: super::vals::SLEEPDEEP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEEPDEEPS(&self) -> super::vals::SLEEPDEEPS {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SLEEPDEEPS::from_bits(val as u8)
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_SLEEPDEEPS(&mut self, val: super::vals::SLEEPDEEPS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn SEVONPEND(&self) -> super::vals::SEVONPEND {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SEVONPEND::from_bits(val as u8)
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_SEVONPEND(&mut self, val: super::vals::SEVONPEND) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for SCR {
    #[inline(always)]
    fn default() -> SCR {
        SCR(0)
    }
}
impl core::fmt::Debug for SCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCR")
            .field("SLEEPONEXIT", &self.SLEEPONEXIT())
            .field("SLEEPDEEP", &self.SLEEPDEEP())
            .field("SLEEPDEEPS", &self.SLEEPDEEPS())
            .field("SEVONPEND", &self.SEVONPEND())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SCR {{ SLEEPONEXIT: {=bool:?}, SLEEPDEEP: {:?}, SLEEPDEEPS: {:?}, SEVONPEND: {:?} }}",
            self.SLEEPONEXIT(),
            self.SLEEPDEEP(),
            self.SLEEPDEEPS(),
            self.SEVONPEND()
        )
    }
}
#[doc = "System Handler Control and State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHCSR(pub u32);
impl SHCSR {
    #[doc = "MemManage exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTACT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception active."]
    #[inline(always)]
    pub const fn set_MEMFAULTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BusFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTACT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception active."]
    #[inline(always)]
    pub const fn set_BUSFAULTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "HardFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn HARDFAULTACT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "HardFault exception active."]
    #[inline(always)]
    pub const fn set_HARDFAULTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UsageFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTACT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception active."]
    #[inline(always)]
    pub const fn set_USGFAULTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SecureFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn SECUREFAULTACT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception active."]
    #[inline(always)]
    pub const fn set_SECUREFAULTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NMI exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn NMIACT(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NMI exception active."]
    #[inline(always)]
    pub const fn set_NMIACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SVCall active."]
    #[must_use]
    #[inline(always)]
    pub const fn SVCALLACT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall active."]
    #[inline(always)]
    pub const fn set_SVCALLACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Debug monitor active."]
    #[must_use]
    #[inline(always)]
    pub const fn MONITORACT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug monitor active."]
    #[inline(always)]
    pub const fn set_MONITORACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PendSV exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn PENDSVACT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PendSV exception active."]
    #[inline(always)]
    pub const fn set_PENDSVACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SysTick exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn SYSTICKACT(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception active."]
    #[inline(always)]
    pub const fn set_SYSTICKACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "UsageFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTPENDED(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault exception pending."]
    #[inline(always)]
    pub const fn set_USGFAULTPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "MemManage exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTPENDED(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage exception pending."]
    #[inline(always)]
    pub const fn set_MEMFAULTPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "BusFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTPENDED(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault exception pending."]
    #[inline(always)]
    pub const fn set_BUSFAULTPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SVCall pending."]
    #[must_use]
    #[inline(always)]
    pub const fn SVCALLPENDED(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SVCall pending."]
    #[inline(always)]
    pub const fn set_SVCALLPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "MemManage enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MEMFAULTENA(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MemManage enable."]
    #[inline(always)]
    pub const fn set_MEMFAULTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "BusFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSFAULTENA(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "BusFault enable."]
    #[inline(always)]
    pub const fn set_BUSFAULTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "UsageFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn USGFAULTENA(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "UsageFault enable."]
    #[inline(always)]
    pub const fn set_USGFAULTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SecureFault exception enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SECUREFAULTENA(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception enable."]
    #[inline(always)]
    pub const fn set_SECUREFAULTENA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SecureFault exception pended state bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SECUREFAULTPENDED(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SecureFault exception pended state bit."]
    #[inline(always)]
    pub const fn set_SECUREFAULTPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "HardFault exception pended state."]
    #[must_use]
    #[inline(always)]
    pub const fn HARDFAULTPENDED(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "HardFault exception pended state."]
    #[inline(always)]
    pub const fn set_HARDFAULTPENDED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for SHCSR {
    #[inline(always)]
    fn default() -> SHCSR {
        SHCSR(0)
    }
}
impl core::fmt::Debug for SHCSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHCSR")
            .field("MEMFAULTACT", &self.MEMFAULTACT())
            .field("BUSFAULTACT", &self.BUSFAULTACT())
            .field("HARDFAULTACT", &self.HARDFAULTACT())
            .field("USGFAULTACT", &self.USGFAULTACT())
            .field("SECUREFAULTACT", &self.SECUREFAULTACT())
            .field("NMIACT", &self.NMIACT())
            .field("SVCALLACT", &self.SVCALLACT())
            .field("MONITORACT", &self.MONITORACT())
            .field("PENDSVACT", &self.PENDSVACT())
            .field("SYSTICKACT", &self.SYSTICKACT())
            .field("USGFAULTPENDED", &self.USGFAULTPENDED())
            .field("MEMFAULTPENDED", &self.MEMFAULTPENDED())
            .field("BUSFAULTPENDED", &self.BUSFAULTPENDED())
            .field("SVCALLPENDED", &self.SVCALLPENDED())
            .field("MEMFAULTENA", &self.MEMFAULTENA())
            .field("BUSFAULTENA", &self.BUSFAULTENA())
            .field("USGFAULTENA", &self.USGFAULTENA())
            .field("SECUREFAULTENA", &self.SECUREFAULTENA())
            .field("SECUREFAULTPENDED", &self.SECUREFAULTPENDED())
            .field("HARDFAULTPENDED", &self.HARDFAULTPENDED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHCSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHCSR {{ MEMFAULTACT: {=bool:?}, BUSFAULTACT: {=bool:?}, HARDFAULTACT: {=bool:?}, USGFAULTACT: {=bool:?}, SECUREFAULTACT: {=bool:?}, NMIACT: {=bool:?}, SVCALLACT: {=bool:?}, MONITORACT: {=bool:?}, PENDSVACT: {=bool:?}, SYSTICKACT: {=bool:?}, USGFAULTPENDED: {=bool:?}, MEMFAULTPENDED: {=bool:?}, BUSFAULTPENDED: {=bool:?}, SVCALLPENDED: {=bool:?}, MEMFAULTENA: {=bool:?}, BUSFAULTENA: {=bool:?}, USGFAULTENA: {=bool:?}, SECUREFAULTENA: {=bool:?}, SECUREFAULTPENDED: {=bool:?}, HARDFAULTPENDED: {=bool:?} }}",
            self.MEMFAULTACT(),
            self.BUSFAULTACT(),
            self.HARDFAULTACT(),
            self.USGFAULTACT(),
            self.SECUREFAULTACT(),
            self.NMIACT(),
            self.SVCALLACT(),
            self.MONITORACT(),
            self.PENDSVACT(),
            self.SYSTICKACT(),
            self.USGFAULTPENDED(),
            self.MEMFAULTPENDED(),
            self.BUSFAULTPENDED(),
            self.SVCALLPENDED(),
            self.MEMFAULTENA(),
            self.BUSFAULTENA(),
            self.USGFAULTENA(),
            self.SECUREFAULTENA(),
            self.SECUREFAULTPENDED(),
            self.HARDFAULTPENDED()
        )
    }
}
