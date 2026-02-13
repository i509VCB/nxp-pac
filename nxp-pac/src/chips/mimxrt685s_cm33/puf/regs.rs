#[doc = "PUF Allow"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Allow(pub u32);
impl Allow {
    #[doc = "Allow Enroll"]
    #[must_use]
    #[inline(always)]
    pub const fn allowenroll(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Enroll"]
    #[inline(always)]
    pub const fn set_allowenroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow Start"]
    #[must_use]
    #[inline(always)]
    pub const fn allowstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Start"]
    #[inline(always)]
    pub const fn set_allowstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Set Key"]
    #[must_use]
    #[inline(always)]
    pub const fn allowsetkey(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Set Key"]
    #[inline(always)]
    pub const fn set_allowsetkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allow Get Key"]
    #[must_use]
    #[inline(always)]
    pub const fn allowgetkey(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Get Key"]
    #[inline(always)]
    pub const fn set_allowgetkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Allow {
    #[inline(always)]
    fn default() -> Allow {
        Allow(0)
    }
}
impl core::fmt::Debug for Allow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Allow")
            .field("allowenroll", &self.allowenroll())
            .field("allowstart", &self.allowstart())
            .field("allowsetkey", &self.allowsetkey())
            .field("allowgetkey", &self.allowgetkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Allow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Allow {{ allowenroll: {=bool:?}, allowstart: {=bool:?}, allowsetkey: {=bool:?}, allowgetkey: {=bool:?} }}",
            self.allowenroll(),
            self.allowstart(),
            self.allowsetkey(),
            self.allowgetkey()
        )
    }
}
#[doc = "PUF Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Block Enroll and Set Key Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn blockenroll_setkey(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub const fn set_blockenroll_setkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Block Key Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn blockkeyoutput(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Block Key Output Data"]
    #[inline(always)]
    pub const fn set_blockkeyoutput(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("blockenroll_setkey", &self.blockenroll_setkey())
            .field("blockkeyoutput", &self.blockkeyoutput())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ blockenroll_setkey: {=bool:?}, blockkeyoutput: {=bool:?} }}",
            self.blockenroll_setkey(),
            self.blockkeyoutput()
        )
    }
}
#[doc = "PUF Code Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codeinput(pub u32);
impl Codeinput {
    #[doc = "AC/KC Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn codein(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC Input Data"]
    #[inline(always)]
    pub const fn set_codein(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codeinput {
    #[inline(always)]
    fn default() -> Codeinput {
        Codeinput(0)
    }
}
impl core::fmt::Debug for Codeinput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Codeinput")
            .field("codein", &self.codein())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Codeinput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Codeinput {{ codein: {=u32:?} }}", self.codein())
    }
}
#[doc = "PUF Code Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codeoutput(pub u32);
impl Codeoutput {
    #[doc = "AC/KC Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn codeout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC Output Data"]
    #[inline(always)]
    pub const fn set_codeout(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codeoutput {
    #[inline(always)]
    fn default() -> Codeoutput {
        Codeoutput(0)
    }
}
impl core::fmt::Debug for Codeoutput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Codeoutput")
            .field("codeout", &self.codeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Codeoutput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Codeoutput {{ codeout: {=u32:?} }}", self.codeout())
    }
}
#[doc = "PUF Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Zeroize. Begin Zeroize operation for Quiddikey and go to Error state"]
    #[must_use]
    #[inline(always)]
    pub const fn zeroize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroize. Begin Zeroize operation for Quiddikey and go to Error state"]
    #[inline(always)]
    pub const fn set_zeroize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enroll. Begin Enroll operation"]
    #[must_use]
    #[inline(always)]
    pub const fn enroll(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enroll. Begin Enroll operation"]
    #[inline(always)]
    pub const fn set_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start. Begin Start operation"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start. Begin Start operation"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set Intrinsic Key. Begin Set Intrinsic Key operation"]
    #[must_use]
    #[inline(always)]
    pub const fn generatekey(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set Intrinsic Key. Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub const fn set_generatekey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set Key. Begin Set User Key operation"]
    #[must_use]
    #[inline(always)]
    pub const fn setkey(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set Key. Begin Set User Key operation"]
    #[inline(always)]
    pub const fn set_setkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Get Key. Begin Get Key operation"]
    #[must_use]
    #[inline(always)]
    pub const fn getkey(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Get Key. Begin Get Key operation"]
    #[inline(always)]
    pub const fn set_getkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("zeroize", &self.zeroize())
            .field("enroll", &self.enroll())
            .field("start", &self.start())
            .field("generatekey", &self.generatekey())
            .field("setkey", &self.setkey())
            .field("getkey", &self.getkey())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ zeroize: {=bool:?}, enroll: {=bool:?}, start: {=bool:?}, generatekey: {=bool:?}, setkey: {=bool:?}, getkey: {=bool:?} }}",
            self.zeroize(),
            self.enroll(),
            self.start(),
            self.generatekey(),
            self.setkey(),
            self.getkey()
        )
    }
}
#[doc = "Index Block High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkH(pub u32);
impl IdxblkH {
    #[doc = "Index 8"]
    #[must_use]
    #[inline(always)]
    pub const fn idx8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 8"]
    #[inline(always)]
    pub const fn set_idx8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 9"]
    #[must_use]
    #[inline(always)]
    pub const fn idx9(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub const fn set_idx9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 10"]
    #[must_use]
    #[inline(always)]
    pub const fn idx10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub const fn set_idx10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 11"]
    #[must_use]
    #[inline(always)]
    pub const fn idx11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub const fn set_idx11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 12"]
    #[must_use]
    #[inline(always)]
    pub const fn idx12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub const fn set_idx12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 13"]
    #[must_use]
    #[inline(always)]
    pub const fn idx13(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub const fn set_idx13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 14"]
    #[must_use]
    #[inline(always)]
    pub const fn idx14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub const fn set_idx14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 15"]
    #[must_use]
    #[inline(always)]
    pub const fn idx15(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub const fn set_idx15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Lock Index"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_idx(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub const fn set_lock_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IdxblkH {
    #[inline(always)]
    fn default() -> IdxblkH {
        IdxblkH(0)
    }
}
impl core::fmt::Debug for IdxblkH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IdxblkH")
            .field("idx8", &self.idx8())
            .field("idx9", &self.idx9())
            .field("idx10", &self.idx10())
            .field("idx11", &self.idx11())
            .field("idx12", &self.idx12())
            .field("idx13", &self.idx13())
            .field("idx14", &self.idx14())
            .field("idx15", &self.idx15())
            .field("lock_idx", &self.lock_idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IdxblkH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IdxblkH {{ idx8: {=u8:?}, idx9: {=u8:?}, idx10: {=u8:?}, idx11: {=u8:?}, idx12: {=u8:?}, idx13: {=u8:?}, idx14: {=u8:?}, idx15: {=u8:?}, lock_idx: {=u8:?} }}",
            self.idx8(),
            self.idx9(),
            self.idx10(),
            self.idx11(),
            self.idx12(),
            self.idx13(),
            self.idx14(),
            self.idx15(),
            self.lock_idx()
        )
    }
}
#[doc = "Index Block High Duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkHDp(pub u32);
impl IdxblkHDp {
    #[doc = "Index 8"]
    #[must_use]
    #[inline(always)]
    pub const fn idx8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 8"]
    #[inline(always)]
    pub const fn set_idx8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 9"]
    #[must_use]
    #[inline(always)]
    pub const fn idx9(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub const fn set_idx9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 10"]
    #[must_use]
    #[inline(always)]
    pub const fn idx10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub const fn set_idx10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 11"]
    #[must_use]
    #[inline(always)]
    pub const fn idx11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub const fn set_idx11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 12"]
    #[must_use]
    #[inline(always)]
    pub const fn idx12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub const fn set_idx12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 13"]
    #[must_use]
    #[inline(always)]
    pub const fn idx13(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub const fn set_idx13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 14"]
    #[must_use]
    #[inline(always)]
    pub const fn idx14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub const fn set_idx14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 15"]
    #[must_use]
    #[inline(always)]
    pub const fn idx15(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub const fn set_idx15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for IdxblkHDp {
    #[inline(always)]
    fn default() -> IdxblkHDp {
        IdxblkHDp(0)
    }
}
impl core::fmt::Debug for IdxblkHDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IdxblkHDp")
            .field("idx8", &self.idx8())
            .field("idx9", &self.idx9())
            .field("idx10", &self.idx10())
            .field("idx11", &self.idx11())
            .field("idx12", &self.idx12())
            .field("idx13", &self.idx13())
            .field("idx14", &self.idx14())
            .field("idx15", &self.idx15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IdxblkHDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IdxblkHDp {{ idx8: {=u8:?}, idx9: {=u8:?}, idx10: {=u8:?}, idx11: {=u8:?}, idx12: {=u8:?}, idx13: {=u8:?}, idx14: {=u8:?}, idx15: {=u8:?} }}",
            self.idx8(),
            self.idx9(),
            self.idx10(),
            self.idx11(),
            self.idx12(),
            self.idx13(),
            self.idx14(),
            self.idx15()
        )
    }
}
#[doc = "Index Block Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkL(pub u32);
impl IdxblkL {
    #[doc = "Index 1"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 1"]
    #[inline(always)]
    pub const fn set_idx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 2"]
    #[must_use]
    #[inline(always)]
    pub const fn idx2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub const fn set_idx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 3"]
    #[must_use]
    #[inline(always)]
    pub const fn idx3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub const fn set_idx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 4"]
    #[must_use]
    #[inline(always)]
    pub const fn idx4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub const fn set_idx4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 5"]
    #[must_use]
    #[inline(always)]
    pub const fn idx5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub const fn set_idx5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 6"]
    #[must_use]
    #[inline(always)]
    pub const fn idx6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub const fn set_idx6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 7"]
    #[must_use]
    #[inline(always)]
    pub const fn idx7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub const fn set_idx7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Lock Index"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_idx(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub const fn set_lock_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IdxblkL {
    #[inline(always)]
    fn default() -> IdxblkL {
        IdxblkL(0)
    }
}
impl core::fmt::Debug for IdxblkL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IdxblkL")
            .field("idx1", &self.idx1())
            .field("idx2", &self.idx2())
            .field("idx3", &self.idx3())
            .field("idx4", &self.idx4())
            .field("idx5", &self.idx5())
            .field("idx6", &self.idx6())
            .field("idx7", &self.idx7())
            .field("lock_idx", &self.lock_idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IdxblkL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IdxblkL {{ idx1: {=u8:?}, idx2: {=u8:?}, idx3: {=u8:?}, idx4: {=u8:?}, idx5: {=u8:?}, idx6: {=u8:?}, idx7: {=u8:?}, lock_idx: {=u8:?} }}",
            self.idx1(),
            self.idx2(),
            self.idx3(),
            self.idx4(),
            self.idx5(),
            self.idx6(),
            self.idx7(),
            self.lock_idx()
        )
    }
}
#[doc = "Index Block Low Duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkLDp(pub u32);
impl IdxblkLDp {
    #[doc = "Index 0"]
    #[must_use]
    #[inline(always)]
    pub const fn idx0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 0"]
    #[inline(always)]
    pub const fn set_idx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 1"]
    #[must_use]
    #[inline(always)]
    pub const fn idx1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 1"]
    #[inline(always)]
    pub const fn set_idx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 2"]
    #[must_use]
    #[inline(always)]
    pub const fn idx2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub const fn set_idx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 3"]
    #[must_use]
    #[inline(always)]
    pub const fn idx3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub const fn set_idx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 4"]
    #[must_use]
    #[inline(always)]
    pub const fn idx4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub const fn set_idx4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 5"]
    #[must_use]
    #[inline(always)]
    pub const fn idx5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub const fn set_idx5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 6"]
    #[must_use]
    #[inline(always)]
    pub const fn idx6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub const fn set_idx6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 7"]
    #[must_use]
    #[inline(always)]
    pub const fn idx7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub const fn set_idx7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for IdxblkLDp {
    #[inline(always)]
    fn default() -> IdxblkLDp {
        IdxblkLDp(0)
    }
}
impl core::fmt::Debug for IdxblkLDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IdxblkLDp")
            .field("idx0", &self.idx0())
            .field("idx1", &self.idx1())
            .field("idx2", &self.idx2())
            .field("idx3", &self.idx3())
            .field("idx4", &self.idx4())
            .field("idx5", &self.idx5())
            .field("idx6", &self.idx6())
            .field("idx7", &self.idx7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IdxblkLDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IdxblkLDp {{ idx0: {=u8:?}, idx1: {=u8:?}, idx2: {=u8:?}, idx3: {=u8:?}, idx4: {=u8:?}, idx5: {=u8:?}, idx6: {=u8:?}, idx7: {=u8:?} }}",
            self.idx0(),
            self.idx1(),
            self.idx2(),
            self.idx3(),
            self.idx4(),
            self.idx5(),
            self.idx6(),
            self.idx7()
        )
    }
}
#[doc = "PUF Interface Status and Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifstat(pub u32);
impl Ifstat {
    #[doc = "Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ifstat {
    #[inline(always)]
    fn default() -> Ifstat {
        Ifstat(0)
    }
}
impl core::fmt::Debug for Ifstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ifstat")
            .field("error", &self.error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ifstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ifstat {{ error: {=bool:?} }}", self.error())
    }
}
#[doc = "PUF Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn readyen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[inline(always)]
    pub const fn set_readyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[must_use]
    #[inline(always)]
    pub const fn succesen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[inline(always)]
    pub const fn set_succesen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn erroren(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[inline(always)]
    pub const fn set_erroren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[must_use]
    #[inline(always)]
    pub const fn keyinreqen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[inline(always)]
    pub const fn set_keyinreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[must_use]
    #[inline(always)]
    pub const fn keyoutavailen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[inline(always)]
    pub const fn set_keyoutavailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[must_use]
    #[inline(always)]
    pub const fn codeinreqen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[inline(always)]
    pub const fn set_codeinreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[must_use]
    #[inline(always)]
    pub const fn codeoutavailen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[inline(always)]
    pub const fn set_codeoutavailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("readyen", &self.readyen())
            .field("succesen", &self.succesen())
            .field("erroren", &self.erroren())
            .field("keyinreqen", &self.keyinreqen())
            .field("keyoutavailen", &self.keyoutavailen())
            .field("codeinreqen", &self.codeinreqen())
            .field("codeoutavailen", &self.codeoutavailen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ readyen: {=bool:?}, succesen: {=bool:?}, erroren: {=bool:?}, keyinreqen: {=bool:?}, keyoutavailen: {=bool:?}, codeinreqen: {=bool:?}, codeoutavailen: {=bool:?} }}",
            self.readyen(),
            self.succesen(),
            self.erroren(),
            self.keyinreqen(),
            self.keyoutavailen(),
            self.codeinreqen(),
            self.codeoutavailen()
        )
    }
}
#[doc = "PUF Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Success"]
    #[must_use]
    #[inline(always)]
    pub const fn success(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Success"]
    #[inline(always)]
    pub const fn set_success(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Key In Request"]
    #[must_use]
    #[inline(always)]
    pub const fn keyinreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub const fn set_keyinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Key Out Available"]
    #[must_use]
    #[inline(always)]
    pub const fn keyoutavail(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub const fn set_keyoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Code In Request"]
    #[must_use]
    #[inline(always)]
    pub const fn codeinreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub const fn set_codeinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Code Out Available"]
    #[must_use]
    #[inline(always)]
    pub const fn codeoutavail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub const fn set_codeoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("ready", &self.ready())
            .field("success", &self.success())
            .field("error", &self.error())
            .field("keyinreq", &self.keyinreq())
            .field("keyoutavail", &self.keyoutavail())
            .field("codeinreq", &self.codeinreq())
            .field("codeoutavail", &self.codeoutavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ ready: {=bool:?}, success: {=bool:?}, error: {=bool:?}, keyinreq: {=bool:?}, keyoutavail: {=bool:?}, codeinreq: {=bool:?}, codeoutavail: {=bool:?} }}",
            self.ready(),
            self.success(),
            self.error(),
            self.keyinreq(),
            self.keyoutavail(),
            self.codeinreq(),
            self.codeoutavail()
        )
    }
}
#[doc = "Key Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyenable(pub u32);
impl Keyenable {
    #[doc = "Key 0"]
    #[must_use]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeyenableKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeyenableKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn set_key0(&mut self, val: super::vals::KeyenableKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[must_use]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeyenableKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeyenableKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn set_key1(&mut self, val: super::vals::KeyenableKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[must_use]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeyenableKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeyenableKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn set_key2(&mut self, val: super::vals::KeyenableKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[must_use]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeyenableKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeyenableKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn set_key3(&mut self, val: super::vals::KeyenableKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keyenable {
    #[inline(always)]
    fn default() -> Keyenable {
        Keyenable(0)
    }
}
impl core::fmt::Debug for Keyenable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyenable")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyenable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Keyenable {{ key0: {:?}, key1: {:?}, key2: {:?}, key3: {:?} }}",
            self.key0(),
            self.key1(),
            self.key2(),
            self.key3()
        )
    }
}
#[doc = "PUF Key Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyindex(pub u32);
impl Keyindex {
    #[doc = "Key index for Set Key operations"]
    #[must_use]
    #[inline(always)]
    pub const fn keyidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key index for Set Key operations"]
    #[inline(always)]
    pub const fn set_keyidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Keyindex {
    #[inline(always)]
    fn default() -> Keyindex {
        Keyindex(0)
    }
}
impl core::fmt::Debug for Keyindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyindex")
            .field("keyidx", &self.keyidx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyindex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keyindex {{ keyidx: {=u8:?} }}", self.keyidx())
    }
}
#[doc = "PUF Key Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyinput(pub u32);
impl Keyinput {
    #[doc = "Key Input Data"]
    #[must_use]
    #[inline(always)]
    pub const fn keyin(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key Input Data"]
    #[inline(always)]
    pub const fn set_keyin(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyinput {
    #[inline(always)]
    fn default() -> Keyinput {
        Keyinput(0)
    }
}
impl core::fmt::Debug for Keyinput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyinput")
            .field("keyin", &self.keyin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyinput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keyinput {{ keyin: {=u32:?} }}", self.keyin())
    }
}
#[doc = "Key Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keylock(pub u32);
impl Keylock {
    #[doc = "Key 0"]
    #[must_use]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeylockKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeylockKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn set_key0(&mut self, val: super::vals::KeylockKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[must_use]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeylockKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeylockKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn set_key1(&mut self, val: super::vals::KeylockKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[must_use]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeylockKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeylockKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn set_key2(&mut self, val: super::vals::KeylockKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[must_use]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeylockKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeylockKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn set_key3(&mut self, val: super::vals::KeylockKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keylock {
    #[inline(always)]
    fn default() -> Keylock {
        Keylock(0)
    }
}
impl core::fmt::Debug for Keylock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keylock")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keylock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Keylock {{ key0: {:?}, key1: {:?}, key2: {:?}, key3: {:?} }}",
            self.key0(),
            self.key1(),
            self.key2(),
            self.key3()
        )
    }
}
#[doc = "Key Mask x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keymask(pub u32);
impl Keymask {
    #[doc = "Key a Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn keymask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key a Mask"]
    #[inline(always)]
    pub const fn set_keymask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keymask {
    #[inline(always)]
    fn default() -> Keymask {
        Keymask(0)
    }
}
impl core::fmt::Debug for Keymask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keymask")
            .field("keymask", &self.keymask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keymask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keymask {{ keymask: {=u32:?} }}", self.keymask())
    }
}
#[doc = "PUF Key Output Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyoutindex(pub u32);
impl Keyoutindex {
    #[doc = "Key Output Index"]
    #[must_use]
    #[inline(always)]
    pub const fn keyoutidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key Output Index"]
    #[inline(always)]
    pub const fn set_keyoutidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Keyoutindex {
    #[inline(always)]
    fn default() -> Keyoutindex {
        Keyoutindex(0)
    }
}
impl core::fmt::Debug for Keyoutindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyoutindex")
            .field("keyoutidx", &self.keyoutidx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyoutindex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keyoutindex {{ keyoutidx: {=u8:?} }}", self.keyoutidx())
    }
}
#[doc = "PUF Key Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyoutput(pub u32);
impl Keyoutput {
    #[doc = "Key Output Data"]
    #[must_use]
    #[inline(always)]
    pub const fn keyout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key Output Data"]
    #[inline(always)]
    pub const fn set_keyout(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyoutput {
    #[inline(always)]
    fn default() -> Keyoutput {
        Keyoutput(0)
    }
}
impl core::fmt::Debug for Keyoutput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyoutput")
            .field("keyout", &self.keyout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyoutput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keyoutput {{ keyout: {=u32:?} }}", self.keyout())
    }
}
#[doc = "Key Reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyreset(pub u32);
impl Keyreset {
    #[doc = "Key 0"]
    #[must_use]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeyresetKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeyresetKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn set_key0(&mut self, val: super::vals::KeyresetKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[must_use]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeyresetKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeyresetKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn set_key1(&mut self, val: super::vals::KeyresetKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[must_use]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeyresetKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeyresetKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn set_key2(&mut self, val: super::vals::KeyresetKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[must_use]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeyresetKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeyresetKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn set_key3(&mut self, val: super::vals::KeyresetKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keyreset {
    #[inline(always)]
    fn default() -> Keyreset {
        Keyreset(0)
    }
}
impl core::fmt::Debug for Keyreset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keyreset")
            .field("key0", &self.key0())
            .field("key1", &self.key1())
            .field("key2", &self.key2())
            .field("key3", &self.key3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keyreset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Keyreset {{ key0: {:?}, key1: {:?}, key2: {:?}, key3: {:?} }}",
            self.key0(),
            self.key1(),
            self.key2(),
            self.key3()
        )
    }
}
#[doc = "PUF Key Size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keysize(pub u32);
impl Keysize {
    #[doc = "Key Size for Set Key operations"]
    #[must_use]
    #[inline(always)]
    pub const fn keysize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Key Size for Set Key operations"]
    #[inline(always)]
    pub const fn set_keysize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Keysize {
    #[inline(always)]
    fn default() -> Keysize {
        Keysize(0)
    }
}
impl core::fmt::Debug for Keysize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Keysize")
            .field("keysize", &self.keysize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Keysize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Keysize {{ keysize: {=u8:?} }}", self.keysize())
    }
}
#[doc = "PUF Power Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrctrl(pub u32);
impl Pwrctrl {
    #[doc = "RAM Power On"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_on(&self) -> super::vals::RamOn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RamOn::from_bits(val as u8)
    }
    #[doc = "RAM Power On"]
    #[inline(always)]
    pub const fn set_ram_on(&mut self, val: super::vals::RamOn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF Clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ck_dis(&self) -> super::vals::CkDis {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CkDis::from_bits(val as u8)
    }
    #[doc = "PUF Clock control."]
    #[inline(always)]
    pub const fn set_ck_dis(&mut self, val: super::vals::CkDis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Pwrctrl {
    #[inline(always)]
    fn default() -> Pwrctrl {
        Pwrctrl(0)
    }
}
impl core::fmt::Debug for Pwrctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwrctrl")
            .field("ram_on", &self.ram_on())
            .field("ck_dis", &self.ck_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwrctrl {{ ram_on: {:?}, ck_dis: {:?} }}",
            self.ram_on(),
            self.ck_dis()
        )
    }
}
#[doc = "PUF Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Success"]
    #[must_use]
    #[inline(always)]
    pub const fn success(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Success"]
    #[inline(always)]
    pub const fn set_success(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Key In Request"]
    #[must_use]
    #[inline(always)]
    pub const fn keyinreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub const fn set_keyinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Key Out Available"]
    #[must_use]
    #[inline(always)]
    pub const fn keyoutavail(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub const fn set_keyoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Code In Request"]
    #[must_use]
    #[inline(always)]
    pub const fn codeinreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub const fn set_codeinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Code Out Available"]
    #[must_use]
    #[inline(always)]
    pub const fn codeoutavail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub const fn set_codeoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("busy", &self.busy())
            .field("success", &self.success())
            .field("error", &self.error())
            .field("keyinreq", &self.keyinreq())
            .field("keyoutavail", &self.keyoutavail())
            .field("codeinreq", &self.codeinreq())
            .field("codeoutavail", &self.codeoutavail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ busy: {=bool:?}, success: {=bool:?}, error: {=bool:?}, keyinreq: {=bool:?}, keyoutavail: {=bool:?}, codeinreq: {=bool:?}, codeoutavail: {=bool:?} }}",
            self.busy(),
            self.success(),
            self.error(),
            self.keyinreq(),
            self.keyoutavail(),
            self.codeinreq(),
            self.codeoutavail()
        )
    }
}
#[doc = "PUF Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Version"]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Version"]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Version {{ version: {=u32:?} }}", self.version())
    }
}
