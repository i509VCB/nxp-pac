#[doc = "Watchdog Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Low byte of the Watchdog Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cntlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn set_cntlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of the Watchdog Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn cnthigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of the Watchdog Counter"]
    #[inline(always)]
    pub const fn set_cnthigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt")
            .field("cntlow", &self.cntlow())
            .field("cnthigh", &self.cnthigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cnt {{ cntlow: {=u8:?}, cnthigh: {=u8:?} }}",
            self.cntlow(),
            self.cnthigh()
        )
    }
}
#[doc = "Watchdog Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Watchdog Test"]
    #[must_use]
    #[inline(always)]
    pub const fn tst(&self) -> super::vals::Tst {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Tst::from_bits(val as u8)
    }
    #[doc = "Watchdog Test"]
    #[inline(always)]
    pub const fn set_tst(&mut self, val: super::vals::Tst) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Allow updates"]
    #[must_use]
    #[inline(always)]
    pub const fn update(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow updates"]
    #[inline(always)]
    pub const fn set_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Watchdog Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt"]
    #[inline(always)]
    pub const fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Watchdog Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Watchdog Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Watchdog Clock"]
    #[inline(always)]
    pub const fn set_clk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reconfiguration Success"]
    #[must_use]
    #[inline(always)]
    pub const fn rcs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reconfiguration Success"]
    #[inline(always)]
    pub const fn set_rcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Unlock status"]
    #[must_use]
    #[inline(always)]
    pub const fn ulk(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock status"]
    #[inline(always)]
    pub const fn set_ulk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Watchdog prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn pres(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog prescaler"]
    #[inline(always)]
    pub const fn set_pres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd32en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub const fn set_cmd32en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Watchdog Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Interrupt Flag"]
    #[inline(always)]
    pub const fn set_flg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Watchdog Window"]
    #[must_use]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Window"]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs")
            .field("stop", &self.stop())
            .field("wait", &self.wait())
            .field("dbg", &self.dbg())
            .field("tst", &self.tst())
            .field("update", &self.update())
            .field("int", &self.int())
            .field("en", &self.en())
            .field("clk", &self.clk())
            .field("rcs", &self.rcs())
            .field("ulk", &self.ulk())
            .field("pres", &self.pres())
            .field("cmd32en", &self.cmd32en())
            .field("flg", &self.flg())
            .field("win", &self.win())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cs {{ stop: {=bool:?}, wait: {=bool:?}, dbg: {=bool:?}, tst: {:?}, update: {=bool:?}, int: {=bool:?}, en: {=bool:?}, clk: {=u8:?}, rcs: {=bool:?}, ulk: {=bool:?}, pres: {=bool:?}, cmd32en: {=bool:?}, flg: {=bool:?}, win: {=bool:?} }}",
            self.stop(),
            self.wait(),
            self.dbg(),
            self.tst(),
            self.update(),
            self.int(),
            self.en(),
            self.clk(),
            self.rcs(),
            self.ulk(),
            self.pres(),
            self.cmd32en(),
            self.flg(),
            self.win()
        )
    }
}
#[doc = "Watchdog Timeout Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Toval(pub u32);
impl Toval {
    #[doc = "Low byte of the timeout value"]
    #[must_use]
    #[inline(always)]
    pub const fn tovallow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of the timeout value"]
    #[inline(always)]
    pub const fn set_tovallow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of the timeout value"]
    #[must_use]
    #[inline(always)]
    pub const fn tovalhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of the timeout value"]
    #[inline(always)]
    pub const fn set_tovalhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Toval {
    #[inline(always)]
    fn default() -> Toval {
        Toval(0)
    }
}
impl core::fmt::Debug for Toval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Toval")
            .field("tovallow", &self.tovallow())
            .field("tovalhigh", &self.tovalhigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Toval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Toval {{ tovallow: {=u8:?}, tovalhigh: {=u8:?} }}",
            self.tovallow(),
            self.tovalhigh()
        )
    }
}
#[doc = "Watchdog Window Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Win(pub u32);
impl Win {
    #[doc = "Low byte of Watchdog Window"]
    #[must_use]
    #[inline(always)]
    pub const fn winlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low byte of Watchdog Window"]
    #[inline(always)]
    pub const fn set_winlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High byte of Watchdog Window"]
    #[must_use]
    #[inline(always)]
    pub const fn winhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High byte of Watchdog Window"]
    #[inline(always)]
    pub const fn set_winhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Win {
    #[inline(always)]
    fn default() -> Win {
        Win(0)
    }
}
impl core::fmt::Debug for Win {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Win")
            .field("winlow", &self.winlow())
            .field("winhigh", &self.winhigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Win {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Win {{ winlow: {=u8:?}, winhigh: {=u8:?} }}",
            self.winlow(),
            self.winhigh()
        )
    }
}
