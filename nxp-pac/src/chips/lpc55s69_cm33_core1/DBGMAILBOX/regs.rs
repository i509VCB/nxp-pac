#[doc = "CRC mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CSW(pub u32);
impl CSW {
    #[doc = "Debugger will set this bit to 1 to request a resynchronrisation."]
    #[must_use]
    #[inline(always)]
    pub const fn RESYNCH_REQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debugger will set this bit to 1 to request a resynchronrisation."]
    #[inline(always)]
    pub const fn set_RESYNCH_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request is pending from debugger (i.e unread value in REQUEST)."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ_PENDING(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request is pending from debugger (i.e unread value in REQUEST)."]
    #[inline(always)]
    pub const fn set_REQ_PENDING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)."]
    #[must_use]
    #[inline(always)]
    pub const fn DBG_OR_ERR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debugger overrun error (previous REQUEST overwritten before being picked up by ROM)."]
    #[inline(always)]
    pub const fn set_DBG_OR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "AHB overrun Error (Return value overwritten by ROM)."]
    #[must_use]
    #[inline(always)]
    pub const fn AHB_OR_ERR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AHB overrun Error (Return value overwritten by ROM)."]
    #[inline(always)]
    pub const fn set_AHB_OR_ERR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[must_use]
    #[inline(always)]
    pub const fn SOFT_RESET(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Soft Reset for DM (write-only from AHB, not readable and selfclearing). A write to this bit will cause a soft reset for DM."]
    #[inline(always)]
    pub const fn set_SOFT_RESET(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)."]
    #[must_use]
    #[inline(always)]
    pub const fn CHIP_RESET_REQ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Write only bit. Once written will cause the chip to reset (note that the DM is not reset by this reset as it is only resettable by a SOFT reset or a POR/BOD event)."]
    #[inline(always)]
    pub const fn set_CHIP_RESET_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for CSW {
    #[inline(always)]
    fn default() -> CSW {
        CSW(0)
    }
}
impl core::fmt::Debug for CSW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSW")
            .field("RESYNCH_REQ", &self.RESYNCH_REQ())
            .field("REQ_PENDING", &self.REQ_PENDING())
            .field("DBG_OR_ERR", &self.DBG_OR_ERR())
            .field("AHB_OR_ERR", &self.AHB_OR_ERR())
            .field("SOFT_RESET", &self.SOFT_RESET())
            .field("CHIP_RESET_REQ", &self.CHIP_RESET_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CSW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CSW {{ RESYNCH_REQ: {=bool:?}, REQ_PENDING: {=bool:?}, DBG_OR_ERR: {=bool:?}, AHB_OR_ERR: {=bool:?}, SOFT_RESET: {=bool:?}, CHIP_RESET_REQ: {=bool:?} }}",
            self.RESYNCH_REQ(),
            self.REQ_PENDING(),
            self.DBG_OR_ERR(),
            self.AHB_OR_ERR(),
            self.SOFT_RESET(),
            self.CHIP_RESET_REQ()
        )
    }
}
#[doc = "Identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID(pub u32);
impl ID {
    #[doc = "Identification value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Identification value."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ID {
    #[inline(always)]
    fn default() -> ID {
        ID(0)
    }
}
impl core::fmt::Debug for ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID").field("ID", &self.ID()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ID {{ ID: {=u32:?} }}", self.ID())
    }
}
#[doc = "CRC seed register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct REQUEST(pub u32);
impl REQUEST {
    #[doc = "Request Value."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Request Value."]
    #[inline(always)]
    pub const fn set_REQ(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for REQUEST {
    #[inline(always)]
    fn default() -> REQUEST {
        REQUEST(0)
    }
}
impl core::fmt::Debug for REQUEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REQUEST").field("REQ", &self.REQ()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for REQUEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "REQUEST {{ REQ: {=u32:?} }}", self.REQ())
    }
}
#[doc = "Return value from ROM."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RETURN(pub u32);
impl RETURN {
    #[doc = "The Return value from ROM."]
    #[must_use]
    #[inline(always)]
    pub const fn RET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The Return value from ROM."]
    #[inline(always)]
    pub const fn set_RET(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RETURN {
    #[inline(always)]
    fn default() -> RETURN {
        RETURN(0)
    }
}
impl core::fmt::Debug for RETURN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETURN").field("RET", &self.RET()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RETURN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RETURN {{ RET: {=u32:?} }}", self.RET())
    }
}
