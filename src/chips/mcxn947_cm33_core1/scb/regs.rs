#[doc = "Application Interrupt and Reset Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aircr(pub u32);
impl Aircr {
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn vectclractive(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for Debug use. This bit reads as 0. When writing to the register you must write 0 to this bit, otherwise behavior is UNPREDICTABLE. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_vectclractive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[must_use]
    #[inline(always)]
    pub const fn sysresetreq(&self) -> super::vals::Sysresetreq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sysresetreq::from_bits(val as u8)
    }
    #[doc = "System reset request. This bit allows software or a debugger to request a system reset. This bit is not banked between Security states. RW if SYSRESETREQS is 0. When SYSRESETREQS is set to 1, from Non-secure state this bit acts as RAZ/WI."]
    #[inline(always)]
    pub const fn set_sysresetreq(&mut self, val: super::vals::Sysresetreq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn sysresetreqs(&self) -> super::vals::Sysresetreqs {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sysresetreqs::from_bits(val as u8)
    }
    #[doc = "System reset request, Secure state only. The value of this bit defines whether the SYSRESETREQ bit is functional for Non-secure use. This bit is not banked between Security states. RW from Secure State and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_sysresetreqs(&mut self, val: super::vals::Sysresetreqs) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states"]
    #[must_use]
    #[inline(always)]
    pub const fn prigroup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Interrupt priority grouping field. This field determines the split of group priority from subpriority. This bit is banked between Security states"]
    #[inline(always)]
    pub const fn set_prigroup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn bfhfnmins(&self) -> super::vals::Bfhfnmins {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Bfhfnmins::from_bits(val as u8)
    }
    #[doc = "BusFault, HardFault, and NMI Non-secure enable. The value of this bit defines whether BusFault and NMI exceptions are Non-secure, and whether exceptions target the Non-secure HardFault exception. This bit is not banked between Security states. RW from Secure-state and RO from Non-secure state."]
    #[inline(always)]
    pub const fn set_bfhfnmins(&mut self, val: super::vals::Bfhfnmins) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[must_use]
    #[inline(always)]
    pub const fn pris(&self) -> super::vals::Pris {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Pris::from_bits(val as u8)
    }
    #[doc = "Prioritize Secure exceptions. The value of this bit defines whether Secure exception priority boosting is enabled. This bit is not banked between Security states. RW from Secure state and RAZ/WI from Non-secure state."]
    #[inline(always)]
    pub const fn set_pris(&mut self, val: super::vals::Pris) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn endianness(&self) -> super::vals::Endianness {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Endianness::from_bits(val as u8)
    }
    #[doc = "Data endianness bit. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_endianness(&mut self, val: super::vals::Endianness) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn vectkey(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register key: Reads as 0xFA05. On writes, write 0x5FA to VECTKEY, otherwise the write is ignored. This Field is not banked between Security states."]
    #[inline(always)]
    pub const fn set_vectkey(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Aircr {
    #[inline(always)]
    fn default() -> Aircr {
        Aircr(0)
    }
}
impl core::fmt::Debug for Aircr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aircr")
            .field("vectclractive", &self.vectclractive())
            .field("sysresetreq", &self.sysresetreq())
            .field("sysresetreqs", &self.sysresetreqs())
            .field("prigroup", &self.prigroup())
            .field("bfhfnmins", &self.bfhfnmins())
            .field("pris", &self.pris())
            .field("endianness", &self.endianness())
            .field("vectkey", &self.vectkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Aircr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Aircr {{ vectclractive: {=bool:?}, sysresetreq: {:?}, sysresetreqs: {:?}, prigroup: {=u8:?}, bfhfnmins: {:?}, pris: {:?}, endianness: {:?}, vectkey: {=u16:?} }}",
            self.vectclractive(),
            self.sysresetreq(),
            self.sysresetreqs(),
            self.prigroup(),
            self.bfhfnmins(),
            self.pris(),
            self.endianness(),
            self.vectkey()
        )
    }
}
#[doc = "Coprocessor Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpacr(pub u32);
impl Cpacr {
    #[doc = "CP0 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp0(&self) -> super::vals::CpacrCp0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CpacrCp0::from_bits(val as u8)
    }
    #[doc = "CP0 Privilege."]
    #[inline(always)]
    pub const fn set_cp0(&mut self, val: super::vals::CpacrCp0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CP1 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp1(&self) -> super::vals::CpacrCp1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CpacrCp1::from_bits(val as u8)
    }
    #[doc = "CP1 Privilege."]
    #[inline(always)]
    pub const fn set_cp1(&mut self, val: super::vals::CpacrCp1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CP2 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp2(&self) -> super::vals::CpacrCp2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::CpacrCp2::from_bits(val as u8)
    }
    #[doc = "CP2 Privilege."]
    #[inline(always)]
    pub const fn set_cp2(&mut self, val: super::vals::CpacrCp2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CP3 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp3(&self) -> super::vals::CpacrCp3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::CpacrCp3::from_bits(val as u8)
    }
    #[doc = "CP3 Privilege."]
    #[inline(always)]
    pub const fn set_cp3(&mut self, val: super::vals::CpacrCp3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CP4 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp4(&self) -> super::vals::CpacrCp4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::CpacrCp4::from_bits(val as u8)
    }
    #[doc = "CP4 Privilege."]
    #[inline(always)]
    pub const fn set_cp4(&mut self, val: super::vals::CpacrCp4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CP5 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp5(&self) -> super::vals::CpacrCp5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::CpacrCp5::from_bits(val as u8)
    }
    #[doc = "CP5 Privilege."]
    #[inline(always)]
    pub const fn set_cp5(&mut self, val: super::vals::CpacrCp5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "CP6 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp6(&self) -> super::vals::CpacrCp6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CpacrCp6::from_bits(val as u8)
    }
    #[doc = "CP6 Privilege."]
    #[inline(always)]
    pub const fn set_cp6(&mut self, val: super::vals::CpacrCp6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "CP7 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp7(&self) -> super::vals::CpacrCp7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::CpacrCp7::from_bits(val as u8)
    }
    #[doc = "CP7 Privilege."]
    #[inline(always)]
    pub const fn set_cp7(&mut self, val: super::vals::CpacrCp7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CP10 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp10(&self) -> super::vals::CpacrCp10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::CpacrCp10::from_bits(val as u8)
    }
    #[doc = "CP10 Privilege."]
    #[inline(always)]
    pub const fn set_cp10(&mut self, val: super::vals::CpacrCp10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "CP11 Privilege."]
    #[must_use]
    #[inline(always)]
    pub const fn cp11(&self) -> super::vals::Cp11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Cp11::from_bits(val as u8)
    }
    #[doc = "CP11 Privilege."]
    #[inline(always)]
    pub const fn set_cp11(&mut self, val: super::vals::Cp11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
}
impl Default for Cpacr {
    #[inline(always)]
    fn default() -> Cpacr {
        Cpacr(0)
    }
}
impl core::fmt::Debug for Cpacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpacr")
            .field("cp0", &self.cp0())
            .field("cp1", &self.cp1())
            .field("cp2", &self.cp2())
            .field("cp3", &self.cp3())
            .field("cp4", &self.cp4())
            .field("cp5", &self.cp5())
            .field("cp6", &self.cp6())
            .field("cp7", &self.cp7())
            .field("cp10", &self.cp10())
            .field("cp11", &self.cp11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpacr {{ cp0: {:?}, cp1: {:?}, cp2: {:?}, cp3: {:?}, cp4: {:?}, cp5: {:?}, cp6: {:?}, cp7: {:?}, cp10: {:?}, cp11: {:?} }}",
            self.cp0(),
            self.cp1(),
            self.cp2(),
            self.cp3(),
            self.cp4(),
            self.cp5(),
            self.cp6(),
            self.cp7(),
            self.cp10(),
            self.cp11()
        )
    }
}
#[doc = "Non-secure Access Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nsacr(pub u32);
impl Nsacr {
    #[doc = "CP0 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp0(&self) -> super::vals::NsacrCp0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NsacrCp0::from_bits(val as u8)
    }
    #[doc = "CP0 access."]
    #[inline(always)]
    pub const fn set_cp0(&mut self, val: super::vals::NsacrCp0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CP1 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp1(&self) -> super::vals::NsacrCp1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::NsacrCp1::from_bits(val as u8)
    }
    #[doc = "CP1 access."]
    #[inline(always)]
    pub const fn set_cp1(&mut self, val: super::vals::NsacrCp1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CP2 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp2(&self) -> super::vals::NsacrCp2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::NsacrCp2::from_bits(val as u8)
    }
    #[doc = "CP2 access."]
    #[inline(always)]
    pub const fn set_cp2(&mut self, val: super::vals::NsacrCp2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CP3 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp3(&self) -> super::vals::NsacrCp3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::NsacrCp3::from_bits(val as u8)
    }
    #[doc = "CP3 access."]
    #[inline(always)]
    pub const fn set_cp3(&mut self, val: super::vals::NsacrCp3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CP4 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp4(&self) -> super::vals::NsacrCp4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::NsacrCp4::from_bits(val as u8)
    }
    #[doc = "CP4 access."]
    #[inline(always)]
    pub const fn set_cp4(&mut self, val: super::vals::NsacrCp4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "CP5 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp5(&self) -> super::vals::NsacrCp5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::NsacrCp5::from_bits(val as u8)
    }
    #[doc = "CP5 access."]
    #[inline(always)]
    pub const fn set_cp5(&mut self, val: super::vals::NsacrCp5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "CP6 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp6(&self) -> super::vals::NsacrCp6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::NsacrCp6::from_bits(val as u8)
    }
    #[doc = "CP6 access."]
    #[inline(always)]
    pub const fn set_cp6(&mut self, val: super::vals::NsacrCp6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CP7 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp7(&self) -> super::vals::NsacrCp7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::NsacrCp7::from_bits(val as u8)
    }
    #[doc = "CP7 access."]
    #[inline(always)]
    pub const fn set_cp7(&mut self, val: super::vals::NsacrCp7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CP10 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp10(&self) -> super::vals::NsacrCp10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::NsacrCp10::from_bits(val as u8)
    }
    #[doc = "CP10 access."]
    #[inline(always)]
    pub const fn set_cp10(&mut self, val: super::vals::NsacrCp10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "CP11 access."]
    #[must_use]
    #[inline(always)]
    pub const fn cp11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "CP11 access."]
    #[inline(always)]
    pub const fn set_cp11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Nsacr {
    #[inline(always)]
    fn default() -> Nsacr {
        Nsacr(0)
    }
}
impl core::fmt::Debug for Nsacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nsacr")
            .field("cp0", &self.cp0())
            .field("cp1", &self.cp1())
            .field("cp2", &self.cp2())
            .field("cp3", &self.cp3())
            .field("cp4", &self.cp4())
            .field("cp5", &self.cp5())
            .field("cp6", &self.cp6())
            .field("cp7", &self.cp7())
            .field("cp10", &self.cp10())
            .field("cp11", &self.cp11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nsacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nsacr {{ cp0: {:?}, cp1: {:?}, cp2: {:?}, cp3: {:?}, cp4: {:?}, cp5: {:?}, cp6: {:?}, cp7: {:?}, cp10: {:?}, cp11: {=bool:?} }}",
            self.cp0(),
            self.cp1(),
            self.cp2(),
            self.cp3(),
            self.cp4(),
            self.cp5(),
            self.cp6(),
            self.cp7(),
            self.cp10(),
            self.cp11()
        )
    }
}
#[doc = "The SCR controls features of entry to and exit from low-power state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleeponexit(&self) -> super::vals::Sleeponexit {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sleeponexit::from_bits(val as u8)
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_sleeponexit(&mut self, val: super::vals::Sleeponexit) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleepdeep(&self) -> super::vals::Sleepdeep {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sleepdeep::from_bits(val as u8)
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low-power mode. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_sleepdeep(&mut self, val: super::vals::Sleepdeep) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sleepdeeps(&self) -> super::vals::Sleepdeeps {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sleepdeeps::from_bits(val as u8)
    }
    #[doc = "Controls whether the SLEEPDEEP bit is only accessible from the Secure state. This bit in only accessible from the Secure state, and behaves as RAZ/WI when accessed from the Nonsecure state. This bit is not banked between Security states."]
    #[inline(always)]
    pub const fn set_sleepdeeps(&mut self, val: super::vals::Sleepdeeps) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[must_use]
    #[inline(always)]
    pub const fn sevonpend(&self) -> super::vals::Sevonpend {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sevonpend::from_bits(val as u8)
    }
    #[doc = "Send Event on Pending bit. When an event or interrupt enters pending state, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event. This bit is banked between Security states."]
    #[inline(always)]
    pub const fn set_sevonpend(&mut self, val: super::vals::Sevonpend) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("sleeponexit", &self.sleeponexit())
            .field("sleepdeep", &self.sleepdeep())
            .field("sleepdeeps", &self.sleepdeeps())
            .field("sevonpend", &self.sevonpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ sleeponexit: {:?}, sleepdeep: {:?}, sleepdeeps: {:?}, sevonpend: {:?} }}",
            self.sleeponexit(),
            self.sleepdeep(),
            self.sleepdeeps(),
            self.sevonpend()
        )
    }
}
#[doc = "System Handler Control and State Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shcsr(pub u32);
impl Shcsr {
    #[doc = "MemManage exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultact(&self) -> super::vals::Memfaultact {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Memfaultact::from_bits(val as u8)
    }
    #[doc = "MemManage exception active."]
    #[inline(always)]
    pub const fn set_memfaultact(&mut self, val: super::vals::Memfaultact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BusFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultact(&self) -> super::vals::Busfaultact {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Busfaultact::from_bits(val as u8)
    }
    #[doc = "BusFault exception active."]
    #[inline(always)]
    pub const fn set_busfaultact(&mut self, val: super::vals::Busfaultact) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HardFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn hardfaultact(&self) -> super::vals::Hardfaultact {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hardfaultact::from_bits(val as u8)
    }
    #[doc = "HardFault exception active."]
    #[inline(always)]
    pub const fn set_hardfaultact(&mut self, val: super::vals::Hardfaultact) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "UsageFault exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultact(&self) -> super::vals::Usgfaultact {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Usgfaultact::from_bits(val as u8)
    }
    #[doc = "UsageFault exception active."]
    #[inline(always)]
    pub const fn set_usgfaultact(&mut self, val: super::vals::Usgfaultact) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SecureFault exception active"]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultact(&self) -> super::vals::Securefaultact {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Securefaultact::from_bits(val as u8)
    }
    #[doc = "SecureFault exception active"]
    #[inline(always)]
    pub const fn set_securefaultact(&mut self, val: super::vals::Securefaultact) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "NMI exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiact(&self) -> super::vals::Nmiact {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Nmiact::from_bits(val as u8)
    }
    #[doc = "NMI exception active."]
    #[inline(always)]
    pub const fn set_nmiact(&mut self, val: super::vals::Nmiact) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SVCall active."]
    #[must_use]
    #[inline(always)]
    pub const fn svcallact(&self) -> super::vals::Svcallact {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Svcallact::from_bits(val as u8)
    }
    #[doc = "SVCall active."]
    #[inline(always)]
    pub const fn set_svcallact(&mut self, val: super::vals::Svcallact) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Debug monitor active."]
    #[must_use]
    #[inline(always)]
    pub const fn monitoract(&self) -> super::vals::Monitoract {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Monitoract::from_bits(val as u8)
    }
    #[doc = "Debug monitor active."]
    #[inline(always)]
    pub const fn set_monitoract(&mut self, val: super::vals::Monitoract) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "PendSV exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn pendsvact(&self) -> super::vals::Pendsvact {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pendsvact::from_bits(val as u8)
    }
    #[doc = "PendSV exception active."]
    #[inline(always)]
    pub const fn set_pendsvact(&mut self, val: super::vals::Pendsvact) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SysTick exception active."]
    #[must_use]
    #[inline(always)]
    pub const fn systickact(&self) -> super::vals::Systickact {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Systickact::from_bits(val as u8)
    }
    #[doc = "SysTick exception active."]
    #[inline(always)]
    pub const fn set_systickact(&mut self, val: super::vals::Systickact) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "UsageFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultpended(&self) -> super::vals::Usgfaultpended {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Usgfaultpended::from_bits(val as u8)
    }
    #[doc = "UsageFault exception pending."]
    #[inline(always)]
    pub const fn set_usgfaultpended(&mut self, val: super::vals::Usgfaultpended) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "MemManage exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultpended(&self) -> super::vals::Memfaultpended {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Memfaultpended::from_bits(val as u8)
    }
    #[doc = "MemManage exception pending."]
    #[inline(always)]
    pub const fn set_memfaultpended(&mut self, val: super::vals::Memfaultpended) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "BusFault exception pending."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultpended(&self) -> super::vals::Busfaultpended {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Busfaultpended::from_bits(val as u8)
    }
    #[doc = "BusFault exception pending."]
    #[inline(always)]
    pub const fn set_busfaultpended(&mut self, val: super::vals::Busfaultpended) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SVCall pending."]
    #[must_use]
    #[inline(always)]
    pub const fn svcallpended(&self) -> super::vals::Svcallpended {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Svcallpended::from_bits(val as u8)
    }
    #[doc = "SVCall pending."]
    #[inline(always)]
    pub const fn set_svcallpended(&mut self, val: super::vals::Svcallpended) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "MemManage enable."]
    #[must_use]
    #[inline(always)]
    pub const fn memfaultena(&self) -> super::vals::Memfaultena {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Memfaultena::from_bits(val as u8)
    }
    #[doc = "MemManage enable."]
    #[inline(always)]
    pub const fn set_memfaultena(&mut self, val: super::vals::Memfaultena) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "BusFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn busfaultena(&self) -> super::vals::Busfaultena {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Busfaultena::from_bits(val as u8)
    }
    #[doc = "BusFault enable."]
    #[inline(always)]
    pub const fn set_busfaultena(&mut self, val: super::vals::Busfaultena) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "UsageFault enable."]
    #[must_use]
    #[inline(always)]
    pub const fn usgfaultena(&self) -> super::vals::Usgfaultena {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Usgfaultena::from_bits(val as u8)
    }
    #[doc = "UsageFault enable."]
    #[inline(always)]
    pub const fn set_usgfaultena(&mut self, val: super::vals::Usgfaultena) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "SecureFault exception enable."]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultena(&self) -> super::vals::Securefaultena {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Securefaultena::from_bits(val as u8)
    }
    #[doc = "SecureFault exception enable."]
    #[inline(always)]
    pub const fn set_securefaultena(&mut self, val: super::vals::Securefaultena) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "SecureFault exception pended state bit."]
    #[must_use]
    #[inline(always)]
    pub const fn securefaultpended(&self) -> super::vals::Securefaultpended {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Securefaultpended::from_bits(val as u8)
    }
    #[doc = "SecureFault exception pended state bit."]
    #[inline(always)]
    pub const fn set_securefaultpended(&mut self, val: super::vals::Securefaultpended) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "HardFault exception pended state"]
    #[must_use]
    #[inline(always)]
    pub const fn hardfaultpended(&self) -> super::vals::Hardfaultpended {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Hardfaultpended::from_bits(val as u8)
    }
    #[doc = "HardFault exception pended state"]
    #[inline(always)]
    pub const fn set_hardfaultpended(&mut self, val: super::vals::Hardfaultpended) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
}
impl Default for Shcsr {
    #[inline(always)]
    fn default() -> Shcsr {
        Shcsr(0)
    }
}
impl core::fmt::Debug for Shcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shcsr")
            .field("memfaultact", &self.memfaultact())
            .field("busfaultact", &self.busfaultact())
            .field("hardfaultact", &self.hardfaultact())
            .field("usgfaultact", &self.usgfaultact())
            .field("securefaultact", &self.securefaultact())
            .field("nmiact", &self.nmiact())
            .field("svcallact", &self.svcallact())
            .field("monitoract", &self.monitoract())
            .field("pendsvact", &self.pendsvact())
            .field("systickact", &self.systickact())
            .field("usgfaultpended", &self.usgfaultpended())
            .field("memfaultpended", &self.memfaultpended())
            .field("busfaultpended", &self.busfaultpended())
            .field("svcallpended", &self.svcallpended())
            .field("memfaultena", &self.memfaultena())
            .field("busfaultena", &self.busfaultena())
            .field("usgfaultena", &self.usgfaultena())
            .field("securefaultena", &self.securefaultena())
            .field("securefaultpended", &self.securefaultpended())
            .field("hardfaultpended", &self.hardfaultpended())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shcsr {{ memfaultact: {:?}, busfaultact: {:?}, hardfaultact: {:?}, usgfaultact: {:?}, securefaultact: {:?}, nmiact: {:?}, svcallact: {:?}, monitoract: {:?}, pendsvact: {:?}, systickact: {:?}, usgfaultpended: {:?}, memfaultpended: {:?}, busfaultpended: {:?}, svcallpended: {:?}, memfaultena: {:?}, busfaultena: {:?}, usgfaultena: {:?}, securefaultena: {:?}, securefaultpended: {:?}, hardfaultpended: {:?} }}",
            self.memfaultact(),
            self.busfaultact(),
            self.hardfaultact(),
            self.usgfaultact(),
            self.securefaultact(),
            self.nmiact(),
            self.svcallact(),
            self.monitoract(),
            self.pendsvact(),
            self.systickact(),
            self.usgfaultpended(),
            self.memfaultpended(),
            self.busfaultpended(),
            self.svcallpended(),
            self.memfaultena(),
            self.busfaultena(),
            self.usgfaultena(),
            self.securefaultena(),
            self.securefaultpended(),
            self.hardfaultpended()
        )
    }
}
