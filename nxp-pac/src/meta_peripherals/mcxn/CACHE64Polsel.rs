#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CACHE64_POLSEL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache64Polsel {
    ptr: *mut u8,
}
unsafe impl Send for Cache64Polsel {}
unsafe impl Sync for Cache64Polsel {}
impl Cache64Polsel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Region 0 Top Boundary."]
    #[inline(always)]
    pub const fn reg0_top(self) -> crate::pac::common::Reg<Reg0Top, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Region 1 Top Boundary."]
    #[inline(always)]
    pub const fn reg1_top(self) -> crate::pac::common::Reg<Reg1Top, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Policy Select."]
    #[inline(always)]
    pub const fn polsel(self) -> crate::pac::common::Reg<Polsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "Policy Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polsel(pub u32);
impl Polsel {
    #[doc = "Policy Select For Region."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_policy(&self, n: usize) -> RegPolicy {
        assert!(n < 3usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        RegPolicy::from_bits(val as u8)
    }
    #[doc = "Policy Select For Region."]
    #[inline(always)]
    pub const fn set_reg_policy(&mut self, n: usize, val: RegPolicy) {
        assert!(n < 3usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Polsel {
    #[inline(always)]
    fn default() -> Polsel {
        Polsel(0)
    }
}
impl core::fmt::Debug for Polsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Polsel")
            .field("reg_policy[0]", &self.reg_policy(0usize))
            .field("reg_policy[1]", &self.reg_policy(1usize))
            .field("reg_policy[2]", &self.reg_policy(2usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Polsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Polsel {{ reg_policy[0]: {:?}, reg_policy[1]: {:?}, reg_policy[2]: {:?} }}",
            self.reg_policy(0usize),
            self.reg_policy(1usize),
            self.reg_policy(2usize)
        )
    }
}
#[doc = "Region 0 Top Boundary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg0Top(pub u32);
impl Reg0Top {
    #[doc = "Upper Limit Of Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn reg0_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Upper Limit Of Region 0."]
    #[inline(always)]
    pub const fn set_reg0_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 10usize)) | (((val as u32) & 0x0007_ffff) << 10usize);
    }
}
impl Default for Reg0Top {
    #[inline(always)]
    fn default() -> Reg0Top {
        Reg0Top(0)
    }
}
impl core::fmt::Debug for Reg0Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg0Top")
            .field("reg0_top", &self.reg0_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg0Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reg0Top {{ reg0_top: {=u32:?} }}", self.reg0_top())
    }
}
#[doc = "Region 1 Top Boundary."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1Top(pub u32);
impl Reg1Top {
    #[doc = "Upper Limit Of Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn reg1_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "Upper Limit Of Region 1."]
    #[inline(always)]
    pub const fn set_reg1_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 10usize)) | (((val as u32) & 0x0007_ffff) << 10usize);
    }
}
impl Default for Reg1Top {
    #[inline(always)]
    fn default() -> Reg1Top {
        Reg1Top(0)
    }
}
impl core::fmt::Debug for Reg1Top {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1Top")
            .field("reg1_top", &self.reg1_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1Top {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reg1Top {{ reg1_top: {=u32:?} }}", self.reg1_top())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegPolicy {
    #[doc = "Noncacheable."]
    Noncacheable = 0x0,
    #[doc = "Write-through."]
    WriteThrough = 0x01,
    #[doc = "Write-back."]
    WriteBack = 0x02,
    #[doc = "Invalid."]
    Invalid = 0x03,
}
impl RegPolicy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegPolicy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegPolicy {
    #[inline(always)]
    fn from(val: u8) -> RegPolicy {
        RegPolicy::from_bits(val)
    }
}
impl From<RegPolicy> for u8 {
    #[inline(always)]
    fn from(val: RegPolicy) -> u8 {
        RegPolicy::to_bits(val)
    }
}
