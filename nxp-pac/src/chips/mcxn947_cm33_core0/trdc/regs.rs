#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW0(pub u32);
impl Mbc0Dom0Mem0BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW0 {
        Mbc0Dom0Mem0BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW1(pub u32);
impl Mbc0Dom0Mem0BlkCfgW1 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW1Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW1 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW1 {
        Mbc0Dom0Mem0BlkCfgW1(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW1")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW1 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW2(pub u32);
impl Mbc0Dom0Mem0BlkCfgW2 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW2Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW2 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW2 {
        Mbc0Dom0Mem0BlkCfgW2(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW2")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW2 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW3(pub u32);
impl Mbc0Dom0Mem0BlkCfgW3 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW3Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW3 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW3 {
        Mbc0Dom0Mem0BlkCfgW3(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW3")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW3 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW4(pub u32);
impl Mbc0Dom0Mem0BlkCfgW4 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW4Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW4 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW4 {
        Mbc0Dom0Mem0BlkCfgW4(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW4")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW4 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW5(pub u32);
impl Mbc0Dom0Mem0BlkCfgW5 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW5Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW5 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW5 {
        Mbc0Dom0Mem0BlkCfgW5(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW5")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW5 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW6(pub u32);
impl Mbc0Dom0Mem0BlkCfgW6 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW6Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW6 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW6 {
        Mbc0Dom0Mem0BlkCfgW6(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW6")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW6 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkCfgW7(pub u32);
impl Mbc0Dom0Mem0BlkCfgW7 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkCfgW7Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkCfgW7 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkCfgW7 {
        Mbc0Dom0Mem0BlkCfgW7(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkCfgW7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkCfgW7")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkCfgW7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkCfgW7 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkNseW0(pub u32);
impl Mbc0Dom0Mem0BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit0::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit1::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit2::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit3::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit4::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit5::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit6::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit7::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit8::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit9::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit10::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit11::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit12::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit13::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit14::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit15::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit16(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit16::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit16(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit17(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit17::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit17(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit18(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit18::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit18(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit19(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit19::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit19(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit20(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit20::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit20(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit21(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit21::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit21(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit22(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit22::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit22(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit23(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit23::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit23(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit24(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit24::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit24(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit25(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit25::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit25(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit26(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit26::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit26(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit27(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit27::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit27(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit28(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit28::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit28(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit29(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit29::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit29(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit30(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit30::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit30(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit31(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW0Bit31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW0Bit31::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit31(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW0Bit31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkNseW0 {
        Mbc0Dom0Mem0BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkNseW0")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .field("bit16", &self.bit16())
            .field("bit17", &self.bit17())
            .field("bit18", &self.bit18())
            .field("bit19", &self.bit19())
            .field("bit20", &self.bit20())
            .field("bit21", &self.bit21())
            .field("bit22", &self.bit22())
            .field("bit23", &self.bit23())
            .field("bit24", &self.bit24())
            .field("bit25", &self.bit25())
            .field("bit26", &self.bit26())
            .field("bit27", &self.bit27())
            .field("bit28", &self.bit28())
            .field("bit29", &self.bit29())
            .field("bit30", &self.bit30())
            .field("bit31", &self.bit31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkNseW0 {{ bit0: {:?}, bit1: {:?}, bit2: {:?}, bit3: {:?}, bit4: {:?}, bit5: {:?}, bit6: {:?}, bit7: {:?}, bit8: {:?}, bit9: {:?}, bit10: {:?}, bit11: {:?}, bit12: {:?}, bit13: {:?}, bit14: {:?}, bit15: {:?}, bit16: {:?}, bit17: {:?}, bit18: {:?}, bit19: {:?}, bit20: {:?}, bit21: {:?}, bit22: {:?}, bit23: {:?}, bit24: {:?}, bit25: {:?}, bit26: {:?}, bit27: {:?}, bit28: {:?}, bit29: {:?}, bit30: {:?}, bit31: {:?} }}",
            self.bit0(),
            self.bit1(),
            self.bit2(),
            self.bit3(),
            self.bit4(),
            self.bit5(),
            self.bit6(),
            self.bit7(),
            self.bit8(),
            self.bit9(),
            self.bit10(),
            self.bit11(),
            self.bit12(),
            self.bit13(),
            self.bit14(),
            self.bit15(),
            self.bit16(),
            self.bit17(),
            self.bit18(),
            self.bit19(),
            self.bit20(),
            self.bit21(),
            self.bit22(),
            self.bit23(),
            self.bit24(),
            self.bit25(),
            self.bit26(),
            self.bit27(),
            self.bit28(),
            self.bit29(),
            self.bit30(),
            self.bit31()
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem0BlkNseW1(pub u32);
impl Mbc0Dom0Mem0BlkNseW1 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit0::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit1::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit2::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit3::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit4::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit5::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit6::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit7::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit8::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit9::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit10::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit11::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit12::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit13::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit14::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit15::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit16(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit16::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit16(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit17(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit17::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit17(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit18(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit18::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit18(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit19(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit19::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit19(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit20(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit20::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit20(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit21(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit21::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit21(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit22(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit22::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit22(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit23(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit23::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit23(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit24(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit24::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit24(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit25(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit25::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit25(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit26(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit26::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit26(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit27(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit27::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit27(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit28(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit28::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit28(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit29(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit29::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit29(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit30(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit30::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit30(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit31(&self) -> super::vals::Mbc0Dom0Mem0BlkNseW1Bit31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem0BlkNseW1Bit31::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit31(&mut self, val: super::vals::Mbc0Dom0Mem0BlkNseW1Bit31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem0BlkNseW1 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem0BlkNseW1 {
        Mbc0Dom0Mem0BlkNseW1(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem0BlkNseW1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem0BlkNseW1")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .field("bit16", &self.bit16())
            .field("bit17", &self.bit17())
            .field("bit18", &self.bit18())
            .field("bit19", &self.bit19())
            .field("bit20", &self.bit20())
            .field("bit21", &self.bit21())
            .field("bit22", &self.bit22())
            .field("bit23", &self.bit23())
            .field("bit24", &self.bit24())
            .field("bit25", &self.bit25())
            .field("bit26", &self.bit26())
            .field("bit27", &self.bit27())
            .field("bit28", &self.bit28())
            .field("bit29", &self.bit29())
            .field("bit30", &self.bit30())
            .field("bit31", &self.bit31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem0BlkNseW1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem0BlkNseW1 {{ bit0: {:?}, bit1: {:?}, bit2: {:?}, bit3: {:?}, bit4: {:?}, bit5: {:?}, bit6: {:?}, bit7: {:?}, bit8: {:?}, bit9: {:?}, bit10: {:?}, bit11: {:?}, bit12: {:?}, bit13: {:?}, bit14: {:?}, bit15: {:?}, bit16: {:?}, bit17: {:?}, bit18: {:?}, bit19: {:?}, bit20: {:?}, bit21: {:?}, bit22: {:?}, bit23: {:?}, bit24: {:?}, bit25: {:?}, bit26: {:?}, bit27: {:?}, bit28: {:?}, bit29: {:?}, bit30: {:?}, bit31: {:?} }}",
            self.bit0(),
            self.bit1(),
            self.bit2(),
            self.bit3(),
            self.bit4(),
            self.bit5(),
            self.bit6(),
            self.bit7(),
            self.bit8(),
            self.bit9(),
            self.bit10(),
            self.bit11(),
            self.bit12(),
            self.bit13(),
            self.bit14(),
            self.bit15(),
            self.bit16(),
            self.bit17(),
            self.bit18(),
            self.bit19(),
            self.bit20(),
            self.bit21(),
            self.bit22(),
            self.bit23(),
            self.bit24(),
            self.bit25(),
            self.bit26(),
            self.bit27(),
            self.bit28(),
            self.bit29(),
            self.bit30(),
            self.bit31()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkCfgW0(pub u32);
impl Mbc0Dom0Mem1BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem1BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkCfgW0 {
        Mbc0Dom0Mem1BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem1BlkNseW0(pub u32);
impl Mbc0Dom0Mem1BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit0::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit1::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit2::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit3::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit4::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit5::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit6::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit7::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit8::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit9::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit10::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit11::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit12::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit13::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit14::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit15::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit16(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit16::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit16(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit17(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit17::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit17(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit18(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit18::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit18(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit19(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit19::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit19(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit20(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit20::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit20(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit21(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit21::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit21(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit22(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit22::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit22(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit23(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit23::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit23(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit24(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit24::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit24(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit25(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit25::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit25(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit26(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit26::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit26(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit27(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit27::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit27(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit28(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit28::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit28(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit29(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit29::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit29(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit30(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit30::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit30(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit31(&self) -> super::vals::Mbc0Dom0Mem1BlkNseW0Bit31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem1BlkNseW0Bit31::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit31(&mut self, val: super::vals::Mbc0Dom0Mem1BlkNseW0Bit31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem1BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem1BlkNseW0 {
        Mbc0Dom0Mem1BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem1BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem1BlkNseW0")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .field("bit16", &self.bit16())
            .field("bit17", &self.bit17())
            .field("bit18", &self.bit18())
            .field("bit19", &self.bit19())
            .field("bit20", &self.bit20())
            .field("bit21", &self.bit21())
            .field("bit22", &self.bit22())
            .field("bit23", &self.bit23())
            .field("bit24", &self.bit24())
            .field("bit25", &self.bit25())
            .field("bit26", &self.bit26())
            .field("bit27", &self.bit27())
            .field("bit28", &self.bit28())
            .field("bit29", &self.bit29())
            .field("bit30", &self.bit30())
            .field("bit31", &self.bit31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem1BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem1BlkNseW0 {{ bit0: {:?}, bit1: {:?}, bit2: {:?}, bit3: {:?}, bit4: {:?}, bit5: {:?}, bit6: {:?}, bit7: {:?}, bit8: {:?}, bit9: {:?}, bit10: {:?}, bit11: {:?}, bit12: {:?}, bit13: {:?}, bit14: {:?}, bit15: {:?}, bit16: {:?}, bit17: {:?}, bit18: {:?}, bit19: {:?}, bit20: {:?}, bit21: {:?}, bit22: {:?}, bit23: {:?}, bit24: {:?}, bit25: {:?}, bit26: {:?}, bit27: {:?}, bit28: {:?}, bit29: {:?}, bit30: {:?}, bit31: {:?} }}",
            self.bit0(),
            self.bit1(),
            self.bit2(),
            self.bit3(),
            self.bit4(),
            self.bit5(),
            self.bit6(),
            self.bit7(),
            self.bit8(),
            self.bit9(),
            self.bit10(),
            self.bit11(),
            self.bit12(),
            self.bit13(),
            self.bit14(),
            self.bit15(),
            self.bit16(),
            self.bit17(),
            self.bit18(),
            self.bit19(),
            self.bit20(),
            self.bit21(),
            self.bit22(),
            self.bit23(),
            self.bit24(),
            self.bit25(),
            self.bit26(),
            self.bit27(),
            self.bit28(),
            self.bit29(),
            self.bit30(),
            self.bit31()
        )
    }
}
#[doc = "MBC Memory Block Configuration Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem2BlkCfgW0(pub u32);
impl Mbc0Dom0Mem2BlkCfgW0 {
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel0(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel0(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse0) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel1(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel1(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel2(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel2(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse2(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse2(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel3(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel3(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse3(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse3(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel4(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel4(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse4(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse4(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse4) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel5(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel5(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse5(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse5(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel6(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel6(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse6(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse6(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse6) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn mbacsel7(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7 {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7::from_bits(val as u8)
    }
    #[doc = "Memory Block Access Control Select for block B"]
    #[inline(always)]
    pub const fn set_mbacsel7(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Mbacsel7) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "NonSecure Enable for block B"]
    #[must_use]
    #[inline(always)]
    pub const fn nse7(&self) -> super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7::from_bits(val as u8)
    }
    #[doc = "NonSecure Enable for block B"]
    #[inline(always)]
    pub const fn set_nse7(&mut self, val: super::vals::Mbc0Dom0Mem2BlkCfgW0Nse7) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem2BlkCfgW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem2BlkCfgW0 {
        Mbc0Dom0Mem2BlkCfgW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem2BlkCfgW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem2BlkCfgW0")
            .field("mbacsel0", &self.mbacsel0())
            .field("nse0", &self.nse0())
            .field("mbacsel1", &self.mbacsel1())
            .field("nse1", &self.nse1())
            .field("mbacsel2", &self.mbacsel2())
            .field("nse2", &self.nse2())
            .field("mbacsel3", &self.mbacsel3())
            .field("nse3", &self.nse3())
            .field("mbacsel4", &self.mbacsel4())
            .field("nse4", &self.nse4())
            .field("mbacsel5", &self.mbacsel5())
            .field("nse5", &self.nse5())
            .field("mbacsel6", &self.mbacsel6())
            .field("nse6", &self.nse6())
            .field("mbacsel7", &self.mbacsel7())
            .field("nse7", &self.nse7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem2BlkCfgW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem2BlkCfgW0 {{ mbacsel0: {:?}, nse0: {:?}, mbacsel1: {:?}, nse1: {:?}, mbacsel2: {:?}, nse2: {:?}, mbacsel3: {:?}, nse3: {:?}, mbacsel4: {:?}, nse4: {:?}, mbacsel5: {:?}, nse5: {:?}, mbacsel6: {:?}, nse6: {:?}, mbacsel7: {:?}, nse7: {:?} }}",
            self.mbacsel0(),
            self.nse0(),
            self.mbacsel1(),
            self.nse1(),
            self.mbacsel2(),
            self.nse2(),
            self.mbacsel3(),
            self.nse3(),
            self.mbacsel4(),
            self.nse4(),
            self.mbacsel5(),
            self.nse5(),
            self.mbacsel6(),
            self.nse6(),
            self.mbacsel7(),
            self.nse7()
        )
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Dom0Mem2BlkNseW0(pub u32);
impl Mbc0Dom0Mem2BlkNseW0 {
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit0::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit0(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit1::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit1(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit2(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit2::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit2(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit3(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit3::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit3(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit4(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit4::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit4(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit5(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit5::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit5(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit6(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit6::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit6(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit7(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit7::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit7(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit8(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit8::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit8(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit9(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit9::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit9(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit10(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit10::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit10(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit11(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit11::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit11(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit12(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit12::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit12(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit13(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit13::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit13(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit14(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit14::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit14(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit15(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit15::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit15(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit16(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit16::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit16(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit17(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit17::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit17(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit18(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit18::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit18(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit19(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit19::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit19(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit20(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit20::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit20(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit21(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit21::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit21(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit22(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit22::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit22(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit23(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit23::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit23(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit24(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit24::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit24(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit25(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit25::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit25(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit26(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit26::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit26(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit27(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit27::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit27(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit28(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit28::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit28(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit29(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit29::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit29(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit30(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit30::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit30(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn bit31(&self) -> super::vals::Mbc0Dom0Mem2BlkNseW0Bit31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mbc0Dom0Mem2BlkNseW0Bit31::from_bits(val as u8)
    }
    #[doc = "Bit b NonSecure Enable \\[b = 0 - 31\\]"]
    #[inline(always)]
    pub const fn set_bit31(&mut self, val: super::vals::Mbc0Dom0Mem2BlkNseW0Bit31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0Dom0Mem2BlkNseW0 {
    #[inline(always)]
    fn default() -> Mbc0Dom0Mem2BlkNseW0 {
        Mbc0Dom0Mem2BlkNseW0(0)
    }
}
impl core::fmt::Debug for Mbc0Dom0Mem2BlkNseW0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Dom0Mem2BlkNseW0")
            .field("bit0", &self.bit0())
            .field("bit1", &self.bit1())
            .field("bit2", &self.bit2())
            .field("bit3", &self.bit3())
            .field("bit4", &self.bit4())
            .field("bit5", &self.bit5())
            .field("bit6", &self.bit6())
            .field("bit7", &self.bit7())
            .field("bit8", &self.bit8())
            .field("bit9", &self.bit9())
            .field("bit10", &self.bit10())
            .field("bit11", &self.bit11())
            .field("bit12", &self.bit12())
            .field("bit13", &self.bit13())
            .field("bit14", &self.bit14())
            .field("bit15", &self.bit15())
            .field("bit16", &self.bit16())
            .field("bit17", &self.bit17())
            .field("bit18", &self.bit18())
            .field("bit19", &self.bit19())
            .field("bit20", &self.bit20())
            .field("bit21", &self.bit21())
            .field("bit22", &self.bit22())
            .field("bit23", &self.bit23())
            .field("bit24", &self.bit24())
            .field("bit25", &self.bit25())
            .field("bit26", &self.bit26())
            .field("bit27", &self.bit27())
            .field("bit28", &self.bit28())
            .field("bit29", &self.bit29())
            .field("bit30", &self.bit30())
            .field("bit31", &self.bit31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Dom0Mem2BlkNseW0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Dom0Mem2BlkNseW0 {{ bit0: {:?}, bit1: {:?}, bit2: {:?}, bit3: {:?}, bit4: {:?}, bit5: {:?}, bit6: {:?}, bit7: {:?}, bit8: {:?}, bit9: {:?}, bit10: {:?}, bit11: {:?}, bit12: {:?}, bit13: {:?}, bit14: {:?}, bit15: {:?}, bit16: {:?}, bit17: {:?}, bit18: {:?}, bit19: {:?}, bit20: {:?}, bit21: {:?}, bit22: {:?}, bit23: {:?}, bit24: {:?}, bit25: {:?}, bit26: {:?}, bit27: {:?}, bit28: {:?}, bit29: {:?}, bit30: {:?}, bit31: {:?} }}",
            self.bit0(),
            self.bit1(),
            self.bit2(),
            self.bit3(),
            self.bit4(),
            self.bit5(),
            self.bit6(),
            self.bit7(),
            self.bit8(),
            self.bit9(),
            self.bit10(),
            self.bit11(),
            self.bit12(),
            self.bit13(),
            self.bit14(),
            self.bit15(),
            self.bit16(),
            self.bit17(),
            self.bit18(),
            self.bit19(),
            self.bit20(),
            self.bit21(),
            self.bit22(),
            self.bit23(),
            self.bit24(),
            self.bit25(),
            self.bit26(),
            self.bit27(),
            self.bit28(),
            self.bit29(),
            self.bit30(),
            self.bit31()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem0Glbcfg(pub u32);
impl Mbc0Mem0Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem0Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem0Glbcfg {
        Mbc0Mem0Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem0Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem0Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem0Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem0Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem1Glbcfg(pub u32);
impl Mbc0Mem1Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem1Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem1Glbcfg {
        Mbc0Mem1Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem1Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem1Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem1Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem1Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem2Glbcfg(pub u32);
impl Mbc0Mem2Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Mbc0Mem2Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem2Glbcfg {
        Mbc0Mem2Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem2Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem2Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem2Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem2Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?} }}",
            self.nblks(),
            self.size_log2()
        )
    }
}
#[doc = "MBC Global Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0Mem3Glbcfg(pub u32);
impl Mbc0Mem3Glbcfg {
    #[doc = "Number of blocks in this memory"]
    #[must_use]
    #[inline(always)]
    pub const fn nblks(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of blocks in this memory"]
    #[inline(always)]
    pub const fn set_nblks(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Log2 size per block"]
    #[must_use]
    #[inline(always)]
    pub const fn size_log2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Log2 size per block"]
    #[inline(always)]
    pub const fn set_size_log2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Clear Error"]
    #[must_use]
    #[inline(always)]
    pub const fn clre(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Clear Error"]
    #[inline(always)]
    pub const fn set_clre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Mbc0Mem3Glbcfg {
    #[inline(always)]
    fn default() -> Mbc0Mem3Glbcfg {
        Mbc0Mem3Glbcfg(0)
    }
}
impl core::fmt::Debug for Mbc0Mem3Glbcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0Mem3Glbcfg")
            .field("nblks", &self.nblks())
            .field("size_log2", &self.size_log2())
            .field("clre", &self.clre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0Mem3Glbcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0Mem3Glbcfg {{ nblks: {=u16:?}, size_log2: {=u8:?}, clre: {=u8:?} }}",
            self.nblks(),
            self.size_log2(),
            self.clre()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac0(pub u32);
impl Mbc0MemnGlbac0 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for Mbc0MemnGlbac0 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac0 {
        Mbc0MemnGlbac0(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac0")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac0 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac1(pub u32);
impl Mbc0MemnGlbac1 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac1 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac1 {
        Mbc0MemnGlbac1(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac1")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac1 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac2(pub u32);
impl Mbc0MemnGlbac2 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac2 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac2 {
        Mbc0MemnGlbac2(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac2")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac2 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac3(pub u32);
impl Mbc0MemnGlbac3 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac3 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac3 {
        Mbc0MemnGlbac3(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac3")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac3 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac4(pub u32);
impl Mbc0MemnGlbac4 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac4 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac4 {
        Mbc0MemnGlbac4(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac4")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac4 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac5(pub u32);
impl Mbc0MemnGlbac5 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac5 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac5 {
        Mbc0MemnGlbac5(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac5")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac5 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac6(pub u32);
impl Mbc0MemnGlbac6 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac6 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac6 {
        Mbc0MemnGlbac6(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac6")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac6 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC Global Access Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0MemnGlbac7(pub u32);
impl Mbc0MemnGlbac7 {
    #[doc = "NonsecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn nux(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Execute"]
    #[inline(always)]
    pub const fn set_nux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NonsecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn nuw(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Write"]
    #[inline(always)]
    pub const fn set_nuw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NonsecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn nur(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecureUser Read"]
    #[inline(always)]
    pub const fn set_nur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NonsecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn npx(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Execute"]
    #[inline(always)]
    pub const fn set_npx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "NonsecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn npw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Write"]
    #[inline(always)]
    pub const fn set_npw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "NonsecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn npr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "NonsecurePriv Read"]
    #[inline(always)]
    pub const fn set_npr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SecureUser Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn sux(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Execute"]
    #[inline(always)]
    pub const fn set_sux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SecureUser Write"]
    #[must_use]
    #[inline(always)]
    pub const fn suw(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Write"]
    #[inline(always)]
    pub const fn set_suw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SecureUser Read"]
    #[must_use]
    #[inline(always)]
    pub const fn sur(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SecureUser Read"]
    #[inline(always)]
    pub const fn set_sur(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SecurePriv Execute"]
    #[must_use]
    #[inline(always)]
    pub const fn spx(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Execute"]
    #[inline(always)]
    pub const fn set_spx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SecurePriv Write"]
    #[must_use]
    #[inline(always)]
    pub const fn spw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Write"]
    #[inline(always)]
    pub const fn set_spw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SecurePriv Read"]
    #[must_use]
    #[inline(always)]
    pub const fn spr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SecurePriv Read"]
    #[inline(always)]
    pub const fn set_spr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LOCK"]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK"]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0MemnGlbac7 {
    #[inline(always)]
    fn default() -> Mbc0MemnGlbac7 {
        Mbc0MemnGlbac7(0)
    }
}
impl core::fmt::Debug for Mbc0MemnGlbac7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0MemnGlbac7")
            .field("nux", &self.nux())
            .field("nuw", &self.nuw())
            .field("nur", &self.nur())
            .field("npx", &self.npx())
            .field("npw", &self.npw())
            .field("npr", &self.npr())
            .field("sux", &self.sux())
            .field("suw", &self.suw())
            .field("sur", &self.sur())
            .field("spx", &self.spx())
            .field("spw", &self.spw())
            .field("spr", &self.spr())
            .field("lk", &self.lk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0MemnGlbac7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0MemnGlbac7 {{ nux: {=bool:?}, nuw: {=bool:?}, nur: {=bool:?}, npx: {=bool:?}, npw: {=bool:?}, npr: {=bool:?}, sux: {=bool:?}, suw: {=bool:?}, sur: {=bool:?}, spx: {=bool:?}, spw: {=bool:?}, spr: {=bool:?}, lk: {=bool:?} }}",
            self.nux(),
            self.nuw(),
            self.nur(),
            self.npx(),
            self.npw(),
            self.npr(),
            self.sux(),
            self.suw(),
            self.sur(),
            self.spx(),
            self.spw(),
            self.spr(),
            self.lk()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkClr(pub u32);
impl Mbc0NseBlkClr {
    #[doc = "Write-1 Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn w1clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write-1 Clear"]
    #[inline(always)]
    pub const fn set_w1clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mbc0NseBlkClr {
    #[inline(always)]
    fn default() -> Mbc0NseBlkClr {
        Mbc0NseBlkClr(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkClr")
            .field("w1clr", &self.w1clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mbc0NseBlkClr {{ w1clr: {=u32:?} }}", self.w1clr())
    }
}
#[doc = "MBC NonSecure Enable Block Clear All"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkClrAll(pub u32);
impl Mbc0NseBlkClrAll {
    #[doc = "Memory Select"]
    #[must_use]
    #[inline(always)]
    pub const fn memsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select"]
    #[inline(always)]
    pub const fn set_memsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> super::vals::Mbc0NseBlkClrAllDidSel0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0NseBlkClrAllDidSel0::from_bits(val as u8)
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: super::vals::Mbc0NseBlkClrAllDidSel0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Mbc0NseBlkClrAll {
    #[inline(always)]
    fn default() -> Mbc0NseBlkClrAll {
        Mbc0NseBlkClrAll(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkClrAll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkClrAll")
            .field("memsel", &self.memsel())
            .field("did_sel0", &self.did_sel0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkClrAll {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0NseBlkClrAll {{ memsel: {=u8:?}, did_sel0: {:?} }}",
            self.memsel(),
            self.did_sel0()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkIndex(pub u32);
impl Mbc0NseBlkIndex {
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[must_use]
    #[inline(always)]
    pub const fn wndx(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    #[inline(always)]
    pub const fn set_wndx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Memory Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Memory Select"]
    #[inline(always)]
    pub const fn set_mem_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DID Select"]
    #[must_use]
    #[inline(always)]
    pub const fn did_sel0(&self) -> super::vals::Mbc0NseBlkIndexDidSel0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Mbc0NseBlkIndexDidSel0::from_bits(val as u8)
    }
    #[doc = "DID Select"]
    #[inline(always)]
    pub const fn set_did_sel0(&mut self, val: super::vals::Mbc0NseBlkIndexDidSel0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Auto Increment"]
    #[must_use]
    #[inline(always)]
    pub const fn ai(&self) -> super::vals::Ai {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ai::from_bits(val as u8)
    }
    #[doc = "Auto Increment"]
    #[inline(always)]
    pub const fn set_ai(&mut self, val: super::vals::Ai) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mbc0NseBlkIndex {
    #[inline(always)]
    fn default() -> Mbc0NseBlkIndex {
        Mbc0NseBlkIndex(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkIndex")
            .field("wndx", &self.wndx())
            .field("mem_sel", &self.mem_sel())
            .field("did_sel0", &self.did_sel0())
            .field("ai", &self.ai())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkIndex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mbc0NseBlkIndex {{ wndx: {=u8:?}, mem_sel: {=u8:?}, did_sel0: {:?}, ai: {:?} }}",
            self.wndx(),
            self.mem_sel(),
            self.did_sel0(),
            self.ai()
        )
    }
}
#[doc = "MBC NonSecure Enable Block Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbc0NseBlkSet(pub u32);
impl Mbc0NseBlkSet {
    #[doc = "Write-1 Set"]
    #[must_use]
    #[inline(always)]
    pub const fn w1set(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Write-1 Set"]
    #[inline(always)]
    pub const fn set_w1set(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mbc0NseBlkSet {
    #[inline(always)]
    fn default() -> Mbc0NseBlkSet {
        Mbc0NseBlkSet(0)
    }
}
impl core::fmt::Debug for Mbc0NseBlkSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mbc0NseBlkSet")
            .field("w1set", &self.w1set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mbc0NseBlkSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mbc0NseBlkSet {{ w1set: {=u32:?} }}", self.w1set())
    }
}
