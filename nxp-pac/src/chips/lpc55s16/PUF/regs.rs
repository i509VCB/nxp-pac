#[doc = "PUF Allow register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ALLOW(pub u32);
impl ALLOW {
    #[doc = "Enroll operation is allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLOWENROLL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enroll operation is allowed."]
    #[inline(always)]
    pub const fn set_ALLOWENROLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start operation is allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLOWSTART(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Start operation is allowed."]
    #[inline(always)]
    pub const fn set_ALLOWSTART(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set Key operations are allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLOWSETKEY(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set Key operations are allowed."]
    #[inline(always)]
    pub const fn set_ALLOWSETKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Get Key operation is allowed."]
    #[must_use]
    #[inline(always)]
    pub const fn ALLOWGETKEY(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Get Key operation is allowed."]
    #[inline(always)]
    pub const fn set_ALLOWGETKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ALLOW {
    #[inline(always)]
    fn default() -> ALLOW {
        ALLOW(0)
    }
}
impl core::fmt::Debug for ALLOW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALLOW")
            .field("ALLOWENROLL", &self.ALLOWENROLL())
            .field("ALLOWSTART", &self.ALLOWSTART())
            .field("ALLOWSETKEY", &self.ALLOWSETKEY())
            .field("ALLOWGETKEY", &self.ALLOWGETKEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ALLOW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ALLOW {{ ALLOWENROLL: {=bool:?}, ALLOWSTART: {=bool:?}, ALLOWSETKEY: {=bool:?}, ALLOWGETKEY: {=bool:?} }}",
            self.ALLOWENROLL(),
            self.ALLOWSTART(),
            self.ALLOWSETKEY(),
            self.ALLOWGETKEY()
        )
    }
}
#[doc = "PUF config register for block bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "Block enroll operation. Write 1 to set, cleared on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCKENROLL_SETKEY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Block enroll operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub const fn set_BLOCKENROLL_SETKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Block set key operation. Write 1 to set, cleared on reset."]
    #[must_use]
    #[inline(always)]
    pub const fn BLOCKKEYOUTPUT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Block set key operation. Write 1 to set, cleared on reset."]
    #[inline(always)]
    pub const fn set_BLOCKKEYOUTPUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("BLOCKENROLL_SETKEY", &self.BLOCKENROLL_SETKEY())
            .field("BLOCKKEYOUTPUT", &self.BLOCKKEYOUTPUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ BLOCKENROLL_SETKEY: {=bool:?}, BLOCKKEYOUTPUT: {=bool:?} }}",
            self.BLOCKENROLL_SETKEY(),
            self.BLOCKKEYOUTPUT()
        )
    }
}
#[doc = "PUF Code Input register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CODEINPUT(pub u32);
impl CODEINPUT {
    #[doc = "AC/KC input data."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC input data."]
    #[inline(always)]
    pub const fn set_CODEIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CODEINPUT {
    #[inline(always)]
    fn default() -> CODEINPUT {
        CODEINPUT(0)
    }
}
impl core::fmt::Debug for CODEINPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODEINPUT")
            .field("CODEIN", &self.CODEIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CODEINPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CODEINPUT {{ CODEIN: {=u32:?} }}", self.CODEIN())
    }
}
#[doc = "PUF Code Output register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CODEOUTPUT(pub u32);
impl CODEOUTPUT {
    #[doc = "AC/KC output data."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEOUT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC output data."]
    #[inline(always)]
    pub const fn set_CODEOUT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CODEOUTPUT {
    #[inline(always)]
    fn default() -> CODEOUTPUT {
        CODEOUTPUT(0)
    }
}
impl core::fmt::Debug for CODEOUTPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CODEOUTPUT")
            .field("CODEOUT", &self.CODEOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CODEOUTPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CODEOUTPUT {{ CODEOUT: {=u32:?} }}", self.CODEOUT())
    }
}
#[doc = "PUF Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "Begin Zeroize operation for PUF and go to Error state."]
    #[must_use]
    #[inline(always)]
    pub const fn zeroize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Zeroize operation for PUF and go to Error state."]
    #[inline(always)]
    pub const fn set_zeroize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Begin Enroll operation."]
    #[must_use]
    #[inline(always)]
    pub const fn enroll(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Enroll operation."]
    #[inline(always)]
    pub const fn set_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Begin Start operation."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Start operation."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Begin Set Intrinsic Key operation."]
    #[must_use]
    #[inline(always)]
    pub const fn GENERATEKEY(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Set Intrinsic Key operation."]
    #[inline(always)]
    pub const fn set_GENERATEKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Begin Set User Key operation."]
    #[must_use]
    #[inline(always)]
    pub const fn SETKEY(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Set User Key operation."]
    #[inline(always)]
    pub const fn set_SETKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Begin Get Key operation."]
    #[must_use]
    #[inline(always)]
    pub const fn GETKEY(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Begin Get Key operation."]
    #[inline(always)]
    pub const fn set_GETKEY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("zeroize", &self.zeroize())
            .field("enroll", &self.enroll())
            .field("start", &self.start())
            .field("GENERATEKEY", &self.GENERATEKEY())
            .field("SETKEY", &self.SETKEY())
            .field("GETKEY", &self.GETKEY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ zeroize: {=bool:?}, enroll: {=bool:?}, start: {=bool:?}, GENERATEKEY: {=bool:?}, SETKEY: {=bool:?}, GETKEY: {=bool:?} }}",
            self.zeroize(),
            self.enroll(),
            self.start(),
            self.GENERATEKEY(),
            self.SETKEY(),
            self.GETKEY()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDXBLK(pub u32);
impl IDXBLK {
    #[doc = "Use to block PUF index 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 0."]
    #[inline(always)]
    pub const fn set_IDX0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Use to block PUF index 1."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 1."]
    #[inline(always)]
    pub const fn set_IDX1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Use to block PUF index 2."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 2."]
    #[inline(always)]
    pub const fn set_IDX2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Use to block PUF index 3."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 3."]
    #[inline(always)]
    pub const fn set_IDX3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Use to block PUF index 4."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 4."]
    #[inline(always)]
    pub const fn set_IDX4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Use to block PUF index 5."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 5."]
    #[inline(always)]
    pub const fn set_IDX5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Use to block PUF index 6."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 6."]
    #[inline(always)]
    pub const fn set_IDX6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Use to block PUF index 7."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 7."]
    #[inline(always)]
    pub const fn set_IDX7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Use to block PUF index 8."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 8."]
    #[inline(always)]
    pub const fn set_IDX8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Use to block PUF index 9."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 9."]
    #[inline(always)]
    pub const fn set_IDX9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Use to block PUF index 10."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 10."]
    #[inline(always)]
    pub const fn set_IDX10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Use to block PUF index 11."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 11."]
    #[inline(always)]
    pub const fn set_IDX11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Use to block PUF index 12."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 12."]
    #[inline(always)]
    pub const fn set_IDX12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Use to block PUF index 13."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 13."]
    #[inline(always)]
    pub const fn set_IDX13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Use to block PUF index 14."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 14."]
    #[inline(always)]
    pub const fn set_IDX14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Use to block PUF index 15."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 15."]
    #[inline(always)]
    pub const fn set_IDX15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IDXBLK {
    #[inline(always)]
    fn default() -> IDXBLK {
        IDXBLK(0)
    }
}
impl core::fmt::Debug for IDXBLK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDXBLK")
            .field("IDX0", &self.IDX0())
            .field("IDX1", &self.IDX1())
            .field("IDX2", &self.IDX2())
            .field("IDX3", &self.IDX3())
            .field("IDX4", &self.IDX4())
            .field("IDX5", &self.IDX5())
            .field("IDX6", &self.IDX6())
            .field("IDX7", &self.IDX7())
            .field("IDX8", &self.IDX8())
            .field("IDX9", &self.IDX9())
            .field("IDX10", &self.IDX10())
            .field("IDX11", &self.IDX11())
            .field("IDX12", &self.IDX12())
            .field("IDX13", &self.IDX13())
            .field("IDX14", &self.IDX14())
            .field("IDX15", &self.IDX15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDXBLK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IDXBLK {{ IDX0: {=u8:?}, IDX1: {=u8:?}, IDX2: {=u8:?}, IDX3: {=u8:?}, IDX4: {=u8:?}, IDX5: {=u8:?}, IDX6: {=u8:?}, IDX7: {=u8:?}, IDX8: {=u8:?}, IDX9: {=u8:?}, IDX10: {=u8:?}, IDX11: {=u8:?}, IDX12: {=u8:?}, IDX13: {=u8:?}, IDX14: {=u8:?}, IDX15: {=u8:?} }}",
            self.IDX0(),
            self.IDX1(),
            self.IDX2(),
            self.IDX3(),
            self.IDX4(),
            self.IDX5(),
            self.IDX6(),
            self.IDX7(),
            self.IDX8(),
            self.IDX9(),
            self.IDX10(),
            self.IDX11(),
            self.IDX12(),
            self.IDX13(),
            self.IDX14(),
            self.IDX15()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDXBLK_DP(pub u32);
impl IDXBLK_DP {
    #[doc = "Use to block PUF index 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 0."]
    #[inline(always)]
    pub const fn set_IDX0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Use to block PUF index 1."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 1."]
    #[inline(always)]
    pub const fn set_IDX1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Use to block PUF index 2."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 2."]
    #[inline(always)]
    pub const fn set_IDX2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Use to block PUF index 3."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 3."]
    #[inline(always)]
    pub const fn set_IDX3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Use to block PUF index 4."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 4."]
    #[inline(always)]
    pub const fn set_IDX4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Use to block PUF index 5."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 5."]
    #[inline(always)]
    pub const fn set_IDX5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Use to block PUF index 6."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 6."]
    #[inline(always)]
    pub const fn set_IDX6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Use to block PUF index 7."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 7."]
    #[inline(always)]
    pub const fn set_IDX7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Use to block PUF index 8."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 8."]
    #[inline(always)]
    pub const fn set_IDX8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Use to block PUF index 9."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 9."]
    #[inline(always)]
    pub const fn set_IDX9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Use to block PUF index 10."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 10."]
    #[inline(always)]
    pub const fn set_IDX10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Use to block PUF index 11."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 11."]
    #[inline(always)]
    pub const fn set_IDX11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Use to block PUF index 12."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 12."]
    #[inline(always)]
    pub const fn set_IDX12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Use to block PUF index 13."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 13."]
    #[inline(always)]
    pub const fn set_IDX13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Use to block PUF index 14."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 14."]
    #[inline(always)]
    pub const fn set_IDX14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Use to block PUF index 15."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Use to block PUF index 15."]
    #[inline(always)]
    pub const fn set_IDX15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IDXBLK_DP {
    #[inline(always)]
    fn default() -> IDXBLK_DP {
        IDXBLK_DP(0)
    }
}
impl core::fmt::Debug for IDXBLK_DP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDXBLK_DP")
            .field("IDX0", &self.IDX0())
            .field("IDX1", &self.IDX1())
            .field("IDX2", &self.IDX2())
            .field("IDX3", &self.IDX3())
            .field("IDX4", &self.IDX4())
            .field("IDX5", &self.IDX5())
            .field("IDX6", &self.IDX6())
            .field("IDX7", &self.IDX7())
            .field("IDX8", &self.IDX8())
            .field("IDX9", &self.IDX9())
            .field("IDX10", &self.IDX10())
            .field("IDX11", &self.IDX11())
            .field("IDX12", &self.IDX12())
            .field("IDX13", &self.IDX13())
            .field("IDX14", &self.IDX14())
            .field("IDX15", &self.IDX15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDXBLK_DP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IDXBLK_DP {{ IDX0: {=u8:?}, IDX1: {=u8:?}, IDX2: {=u8:?}, IDX3: {=u8:?}, IDX4: {=u8:?}, IDX5: {=u8:?}, IDX6: {=u8:?}, IDX7: {=u8:?}, IDX8: {=u8:?}, IDX9: {=u8:?}, IDX10: {=u8:?}, IDX11: {=u8:?}, IDX12: {=u8:?}, IDX13: {=u8:?}, IDX14: {=u8:?}, IDX15: {=u8:?} }}",
            self.IDX0(),
            self.IDX1(),
            self.IDX2(),
            self.IDX3(),
            self.IDX4(),
            self.IDX5(),
            self.IDX6(),
            self.IDX7(),
            self.IDX8(),
            self.IDX9(),
            self.IDX10(),
            self.IDX11(),
            self.IDX12(),
            self.IDX13(),
            self.IDX14(),
            self.IDX15()
        )
    }
}
#[doc = "Index block status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IDXBLK_STATUS(pub u32);
impl IDXBLK_STATUS {
    #[doc = "Status block index 0."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 0."]
    #[inline(always)]
    pub const fn set_IDX0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Status block index 1."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 1."]
    #[inline(always)]
    pub const fn set_IDX1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Status block index 2."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 2."]
    #[inline(always)]
    pub const fn set_IDX2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Status block index 3."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 3."]
    #[inline(always)]
    pub const fn set_IDX3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Status block index 4."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 4."]
    #[inline(always)]
    pub const fn set_IDX4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Status block index 5."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 5."]
    #[inline(always)]
    pub const fn set_IDX5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Status block index 6."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 6."]
    #[inline(always)]
    pub const fn set_IDX6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Status block index 7."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 7."]
    #[inline(always)]
    pub const fn set_IDX7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Status block index 8."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 8."]
    #[inline(always)]
    pub const fn set_IDX8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Status block index 9."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 9."]
    #[inline(always)]
    pub const fn set_IDX9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Status block index 10."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 10."]
    #[inline(always)]
    pub const fn set_IDX10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Status block index 11."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 11."]
    #[inline(always)]
    pub const fn set_IDX11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Status block index 12."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 12."]
    #[inline(always)]
    pub const fn set_IDX12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Status block index 13."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 13."]
    #[inline(always)]
    pub const fn set_IDX13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Status block index 14."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 14."]
    #[inline(always)]
    pub const fn set_IDX14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Status block index 15."]
    #[must_use]
    #[inline(always)]
    pub const fn IDX15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Status block index 15."]
    #[inline(always)]
    pub const fn set_IDX15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IDXBLK_STATUS {
    #[inline(always)]
    fn default() -> IDXBLK_STATUS {
        IDXBLK_STATUS(0)
    }
}
impl core::fmt::Debug for IDXBLK_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDXBLK_STATUS")
            .field("IDX0", &self.IDX0())
            .field("IDX1", &self.IDX1())
            .field("IDX2", &self.IDX2())
            .field("IDX3", &self.IDX3())
            .field("IDX4", &self.IDX4())
            .field("IDX5", &self.IDX5())
            .field("IDX6", &self.IDX6())
            .field("IDX7", &self.IDX7())
            .field("IDX8", &self.IDX8())
            .field("IDX9", &self.IDX9())
            .field("IDX10", &self.IDX10())
            .field("IDX11", &self.IDX11())
            .field("IDX12", &self.IDX12())
            .field("IDX13", &self.IDX13())
            .field("IDX14", &self.IDX14())
            .field("IDX15", &self.IDX15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IDXBLK_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IDXBLK_STATUS {{ IDX0: {=u8:?}, IDX1: {=u8:?}, IDX2: {=u8:?}, IDX3: {=u8:?}, IDX4: {=u8:?}, IDX5: {=u8:?}, IDX6: {=u8:?}, IDX7: {=u8:?}, IDX8: {=u8:?}, IDX9: {=u8:?}, IDX10: {=u8:?}, IDX11: {=u8:?}, IDX12: {=u8:?}, IDX13: {=u8:?}, IDX14: {=u8:?}, IDX15: {=u8:?} }}",
            self.IDX0(),
            self.IDX1(),
            self.IDX2(),
            self.IDX3(),
            self.IDX4(),
            self.IDX5(),
            self.IDX6(),
            self.IDX7(),
            self.IDX8(),
            self.IDX9(),
            self.IDX10(),
            self.IDX11(),
            self.IDX12(),
            self.IDX13(),
            self.IDX14(),
            self.IDX15()
        )
    }
}
#[doc = "PUF Interface Status and clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IFSTAT(pub u32);
impl IFSTAT {
    #[doc = "Indicates that an APB error has occurred,Writing logic1 clears the if_error bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that an APB error has occurred,Writing logic1 clears the if_error bit."]
    #[inline(always)]
    pub const fn set_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IFSTAT {
    #[inline(always)]
    fn default() -> IFSTAT {
        IFSTAT(0)
    }
}
impl core::fmt::Debug for IFSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFSTAT")
            .field("ERROR", &self.ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IFSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IFSTAT {{ ERROR: {=bool:?} }}", self.ERROR())
    }
}
#[doc = "PUF Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTEN(pub u32);
impl INTEN {
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn READYEN(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_READYEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn SUCCESEN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_SUCCESEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROREN(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_ERROREN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYINREQEN(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_KEYINREQEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYOUTAVAILEN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_KEYOUTAVAILEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEINREQEN(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_CODEINREQEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEOUTAVAILEN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt. Note that bit numbers match those assigned in QK_SR (Quiddikey Status Register)."]
    #[inline(always)]
    pub const fn set_CODEOUTAVAILEN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for INTEN {
    #[inline(always)]
    fn default() -> INTEN {
        INTEN(0)
    }
}
impl core::fmt::Debug for INTEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("READYEN", &self.READYEN())
            .field("SUCCESEN", &self.SUCCESEN())
            .field("ERROREN", &self.ERROREN())
            .field("KEYINREQEN", &self.KEYINREQEN())
            .field("KEYOUTAVAILEN", &self.KEYOUTAVAILEN())
            .field("CODEINREQEN", &self.CODEINREQEN())
            .field("CODEOUTAVAILEN", &self.CODEOUTAVAILEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTEN {{ READYEN: {=bool:?}, SUCCESEN: {=bool:?}, ERROREN: {=bool:?}, KEYINREQEN: {=bool:?}, KEYOUTAVAILEN: {=bool:?}, CODEINREQEN: {=bool:?}, CODEOUTAVAILEN: {=bool:?} }}",
            self.READYEN(),
            self.SUCCESEN(),
            self.ERROREN(),
            self.KEYINREQEN(),
            self.KEYOUTAVAILEN(),
            self.CODEINREQEN(),
            self.CODEOUTAVAILEN()
        )
    }
}
#[doc = "PUF interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Triggers on falling edge of busy, write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Triggers on falling edge of busy, write 1 to clear."]
    #[inline(always)]
    pub const fn set_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn SUCCESS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_SUCCESS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn ERROR(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYINREQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_KEYINREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYOUTAVAIL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_KEYOUTAVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEINREQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_CODEINREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEOUTAVAIL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Level sensitive interrupt, cleared when interrupt source clears."]
    #[inline(always)]
    pub const fn set_CODEOUTAVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("READY", &self.READY())
            .field("SUCCESS", &self.SUCCESS())
            .field("ERROR", &self.ERROR())
            .field("KEYINREQ", &self.KEYINREQ())
            .field("KEYOUTAVAIL", &self.KEYOUTAVAIL())
            .field("CODEINREQ", &self.CODEINREQ())
            .field("CODEOUTAVAIL", &self.CODEOUTAVAIL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ READY: {=bool:?}, SUCCESS: {=bool:?}, ERROR: {=bool:?}, KEYINREQ: {=bool:?}, KEYOUTAVAIL: {=bool:?}, CODEINREQ: {=bool:?}, CODEOUTAVAIL: {=bool:?} }}",
            self.READY(),
            self.SUCCESS(),
            self.ERROR(),
            self.KEYINREQ(),
            self.KEYOUTAVAIL(),
            self.CODEINREQ(),
            self.CODEOUTAVAIL()
        )
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYENABLE(pub u32);
impl KEYENABLE {
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY0 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY0 register.\"."]
    #[inline(always)]
    pub const fn set_KEY0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY1 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY1 register.\"."]
    #[inline(always)]
    pub const fn set_KEY1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY2 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY2 register.\"."]
    #[inline(always)]
    pub const fn set_KEY2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "\"10: Data coming out from PUF Index 0 interface are shifted in KEY3 register. 00, 01, 11 : Data coming out from PUF Index 0 interface are NOT shifted in KEY3 register.\"."]
    #[inline(always)]
    pub const fn set_KEY3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for KEYENABLE {
    #[inline(always)]
    fn default() -> KEYENABLE {
        KEYENABLE(0)
    }
}
impl core::fmt::Debug for KEYENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYENABLE")
            .field("KEY0", &self.KEY0())
            .field("KEY1", &self.KEY1())
            .field("KEY2", &self.KEY2())
            .field("KEY3", &self.KEY3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYENABLE {{ KEY0: {=u8:?}, KEY1: {=u8:?}, KEY2: {=u8:?}, KEY3: {=u8:?} }}",
            self.KEY0(),
            self.KEY1(),
            self.KEY2(),
            self.KEY3()
        )
    }
}
#[doc = "PUF Key Index register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYINDEX(pub u32);
impl KEYINDEX {
    #[doc = "Key index for Set Key operations."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYIDX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key index for Set Key operations."]
    #[inline(always)]
    pub const fn set_KEYIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for KEYINDEX {
    #[inline(always)]
    fn default() -> KEYINDEX {
        KEYINDEX(0)
    }
}
impl core::fmt::Debug for KEYINDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYINDEX")
            .field("KEYIDX", &self.KEYIDX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYINDEX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYINDEX {{ KEYIDX: {=u8:?} }}", self.KEYIDX())
    }
}
#[doc = "PUF Key Input register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYINPUT(pub u32);
impl KEYINPUT {
    #[doc = "Key input data."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYIN(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key input data."]
    #[inline(always)]
    pub const fn set_KEYIN(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KEYINPUT {
    #[inline(always)]
    fn default() -> KEYINPUT {
        KEYINPUT(0)
    }
}
impl core::fmt::Debug for KEYINPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYINPUT")
            .field("KEYIN", &self.KEYIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYINPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYINPUT {{ KEYIN: {=u32:?} }}", self.KEYIN())
    }
}
#[doc = "Only reset in case of full IC reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYLOCK(pub u32);
impl KEYLOCK {
    #[doc = "\"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "\"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[inline(always)]
    pub const fn set_KEY0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "\"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "\"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[inline(always)]
    pub const fn set_KEY1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "\"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "\"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[inline(always)]
    pub const fn set_KEY2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "\"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "\"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\"."]
    #[inline(always)]
    pub const fn set_KEY3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for KEYLOCK {
    #[inline(always)]
    fn default() -> KEYLOCK {
        KEYLOCK(0)
    }
}
impl core::fmt::Debug for KEYLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYLOCK")
            .field("KEY0", &self.KEY0())
            .field("KEY1", &self.KEY1())
            .field("KEY2", &self.KEY2())
            .field("KEY3", &self.KEY3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYLOCK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYLOCK {{ KEY0: {=u8:?}, KEY1: {=u8:?}, KEY2: {=u8:?}, KEY3: {=u8:?} }}",
            self.KEY0(),
            self.KEY1(),
            self.KEY2(),
            self.KEY3()
        )
    }
}
#[doc = "Only reset in case of full IC reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYMASK(pub u32);
impl KEYMASK {
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYMASK(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_KEYMASK(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KEYMASK {
    #[inline(always)]
    fn default() -> KEYMASK {
        KEYMASK(0)
    }
}
impl core::fmt::Debug for KEYMASK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYMASK")
            .field("KEYMASK", &self.KEYMASK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYMASK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYMASK {{ KEYMASK: {=u32:?} }}", self.KEYMASK())
    }
}
#[doc = "PUF Key Output Index register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYOUTINDEX(pub u32);
impl KEYOUTINDEX {
    #[doc = "Key index for the key that is currently output via the Key Output register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYOUTIDX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key index for the key that is currently output via the Key Output register."]
    #[inline(always)]
    pub const fn set_KEYOUTIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for KEYOUTINDEX {
    #[inline(always)]
    fn default() -> KEYOUTINDEX {
        KEYOUTINDEX(0)
    }
}
impl core::fmt::Debug for KEYOUTINDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYOUTINDEX")
            .field("KEYOUTIDX", &self.KEYOUTIDX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYOUTINDEX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYOUTINDEX {{ KEYOUTIDX: {=u8:?} }}", self.KEYOUTIDX())
    }
}
#[doc = "PUF Key Output register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYOUTPUT(pub u32);
impl KEYOUTPUT {
    #[doc = "Key output data."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYOUT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key output data."]
    #[inline(always)]
    pub const fn set_KEYOUT(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for KEYOUTPUT {
    #[inline(always)]
    fn default() -> KEYOUTPUT {
        KEYOUTPUT(0)
    }
}
impl core::fmt::Debug for KEYOUTPUT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYOUTPUT")
            .field("KEYOUT", &self.KEYOUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYOUTPUT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYOUTPUT {{ KEYOUT: {=u32:?} }}", self.KEYOUT())
    }
}
#[doc = "Reinitialize Keys shift registers counters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYRESET(pub u32);
impl KEYRESET {
    #[doc = "10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "10: Reset KEY0 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub const fn set_KEY0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "10: Reset KEY1 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub const fn set_KEY1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "10: Reset KEY2 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub const fn set_KEY2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "10: Reset KEY3 shift register. Self clearing. Must be done before loading any new key."]
    #[inline(always)]
    pub const fn set_KEY3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for KEYRESET {
    #[inline(always)]
    fn default() -> KEYRESET {
        KEYRESET(0)
    }
}
impl core::fmt::Debug for KEYRESET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYRESET")
            .field("KEY0", &self.KEY0())
            .field("KEY1", &self.KEY1())
            .field("KEY2", &self.KEY2())
            .field("KEY3", &self.KEY3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYRESET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KEYRESET {{ KEY0: {=u8:?}, KEY1: {=u8:?}, KEY2: {=u8:?}, KEY3: {=u8:?} }}",
            self.KEY0(),
            self.KEY1(),
            self.KEY2(),
            self.KEY3()
        )
    }
}
#[doc = "PUF Key Size register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KEYSIZE(pub u32);
impl KEYSIZE {
    #[doc = "Key size for Set Key operations."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYSIZE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Key size for Set Key operations."]
    #[inline(always)]
    pub const fn set_KEYSIZE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for KEYSIZE {
    #[inline(always)]
    fn default() -> KEYSIZE {
        KEYSIZE(0)
    }
}
impl core::fmt::Debug for KEYSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEYSIZE")
            .field("KEYSIZE", &self.KEYSIZE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KEYSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KEYSIZE {{ KEYSIZE: {=u8:?} }}", self.KEYSIZE())
    }
}
#[doc = "no description available."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SHIFT_STATUS(pub u32);
impl SHIFT_STATUS {
    #[doc = "Index counter from key 0 shift register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Index counter from key 0 shift register."]
    #[inline(always)]
    pub const fn set_KEY0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Index counter from key 1 shift register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Index counter from key 1 shift register."]
    #[inline(always)]
    pub const fn set_KEY1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Index counter from key 2 shift register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Index counter from key 2 shift register."]
    #[inline(always)]
    pub const fn set_KEY2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Index counter from key 3 shift register."]
    #[must_use]
    #[inline(always)]
    pub const fn KEY3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Index counter from key 3 shift register."]
    #[inline(always)]
    pub const fn set_KEY3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for SHIFT_STATUS {
    #[inline(always)]
    fn default() -> SHIFT_STATUS {
        SHIFT_STATUS(0)
    }
}
impl core::fmt::Debug for SHIFT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHIFT_STATUS")
            .field("KEY0", &self.KEY0())
            .field("KEY1", &self.KEY1())
            .field("KEY2", &self.KEY2())
            .field("KEY3", &self.KEY3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SHIFT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SHIFT_STATUS {{ KEY0: {=u8:?}, KEY1: {=u8:?}, KEY2: {=u8:?}, KEY3: {=u8:?} }}",
            self.KEY0(),
            self.KEY1(),
            self.KEY2(),
            self.KEY3()
        )
    }
}
#[doc = "PUF Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Indicates that operation is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates that operation is in progress."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Last operation was successful."]
    #[must_use]
    #[inline(always)]
    pub const fn SUCCESS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Last operation was successful."]
    #[inline(always)]
    pub const fn set_SUCCESS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PUF is in the Error state and no operations can be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PUF is in the Error state and no operations can be performed."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Request for next part of key."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYINREQ(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Request for next part of key."]
    #[inline(always)]
    pub const fn set_KEYINREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Next part of key is available."]
    #[must_use]
    #[inline(always)]
    pub const fn KEYOUTAVAIL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Next part of key is available."]
    #[inline(always)]
    pub const fn set_KEYOUTAVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Request for next part of AC/KC."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEINREQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Request for next part of AC/KC."]
    #[inline(always)]
    pub const fn set_CODEINREQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Next part of AC/KC is available."]
    #[must_use]
    #[inline(always)]
    pub const fn CODEOUTAVAIL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Next part of AC/KC is available."]
    #[inline(always)]
    pub const fn set_CODEOUTAVAIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("busy", &self.busy())
            .field("SUCCESS", &self.SUCCESS())
            .field("error", &self.error())
            .field("KEYINREQ", &self.KEYINREQ())
            .field("KEYOUTAVAIL", &self.KEYOUTAVAIL())
            .field("CODEINREQ", &self.CODEINREQ())
            .field("CODEOUTAVAIL", &self.CODEOUTAVAIL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ busy: {=bool:?}, SUCCESS: {=bool:?}, error: {=bool:?}, KEYINREQ: {=bool:?}, KEYOUTAVAIL: {=bool:?}, CODEINREQ: {=bool:?}, CODEOUTAVAIL: {=bool:?} }}",
            self.busy(),
            self.SUCCESS(),
            self.error(),
            self.KEYINREQ(),
            self.KEYOUTAVAIL(),
            self.CODEINREQ(),
            self.CODEOUTAVAIL()
        )
    }
}
