#[doc = "BootROM Lock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blr(pub u32);
impl Blr {
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::BlrLock {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::BlrLock::from_bits(val as u8)
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: super::vals::BlrLock) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Blr {
    #[inline(always)]
    fn default() -> Blr {
        Blr(0)
    }
}
impl core::fmt::Debug for Blr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blr").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Blr {{ lock: {:?} }}", self.lock())
    }
}
#[doc = "BootROM Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsr(pub u32);
impl Bsr {
    #[doc = "Provides status information written by the BootROM."]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Provides status information written by the BootROM."]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Bsr {
    #[inline(always)]
    fn default() -> Bsr {
        Bsr(0)
    }
}
impl core::fmt::Debug for Bsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bsr").field("stat", &self.stat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bsr {{ stat: {=u32:?} }}", self.stat())
    }
}
#[doc = "Clock Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckctrl(pub u32);
impl Ckctrl {
    #[doc = "Clocking Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ckmode(&self) -> super::vals::CkctrlCkmode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CkctrlCkmode::from_bits(val as u8)
    }
    #[doc = "Clocking Mode"]
    #[inline(always)]
    pub const fn set_ckmode(&mut self, val: super::vals::CkctrlCkmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ckctrl {
    #[inline(always)]
    fn default() -> Ckctrl {
        Ckctrl(0)
    }
}
impl core::fmt::Debug for Ckctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ckctrl")
            .field("ckmode", &self.ckmode())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ckctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ckctrl {{ ckmode: {:?}, lock: {=bool:?} }}",
            self.ckmode(),
            self.lock()
        )
    }
}
#[doc = "Clock Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckstat(pub u32);
impl Ckstat {
    #[doc = "Low Power Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ckmode(&self) -> super::vals::CkstatCkmode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::CkstatCkmode::from_bits(val as u8)
    }
    #[doc = "Low Power Status"]
    #[inline(always)]
    pub const fn set_ckmode(&mut self, val: super::vals::CkstatCkmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Wake-up Source"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Wake-up Source"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Clock Status Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Status Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ckstat {
    #[inline(always)]
    fn default() -> Ckstat {
        Ckstat(0)
    }
}
impl core::fmt::Debug for Ckstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ckstat")
            .field("ckmode", &self.ckmode())
            .field("wakeup", &self.wakeup())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ckstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ckstat {{ ckmode: {:?}, wakeup: {=u8:?}, valid: {=bool:?} }}",
            self.ckmode(),
            self.wakeup(),
            self.valid()
        )
    }
}
#[doc = "Core Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Corectl(pub u32);
impl Corectl {
    #[doc = "Non-maskable Pin Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn npie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Non-maskable Pin Interrupt Enable"]
    #[inline(always)]
    pub const fn set_npie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Corectl {
    #[inline(always)]
    fn default() -> Corectl {
        Corectl(0)
    }
}
impl core::fmt::Debug for Corectl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Corectl")
            .field("npie", &self.npie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Corectl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Corectl {{ npie: {=bool:?} }}", self.npie())
    }
}
#[doc = "Debug Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgctl(pub u32);
impl Dbgctl {
    #[doc = "Sleep Or Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Or Debug"]
    #[inline(always)]
    pub const fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Dbgctl {
    #[inline(always)]
    fn default() -> Dbgctl {
        Dbgctl(0)
    }
}
impl core::fmt::Debug for Dbgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgctl").field("sod", &self.sod()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dbgctl {{ sod: {=bool:?} }}", self.sod())
    }
}
#[doc = "Flash Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcr(pub u32);
impl Flashcr {
    #[doc = "Flash Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn flashdis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Disable"]
    #[inline(always)]
    pub const fn set_flashdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash Doze"]
    #[must_use]
    #[inline(always)]
    pub const fn flashdoze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Doze"]
    #[inline(always)]
    pub const fn set_flashdoze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Flashcr {
    #[inline(always)]
    fn default() -> Flashcr {
        Flashcr(0)
    }
}
impl core::fmt::Debug for Flashcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashcr")
            .field("flashdis", &self.flashdis())
            .field("flashdoze", &self.flashdoze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flashcr {{ flashdis: {=bool:?}, flashdoze: {=bool:?} }}",
            self.flashdis(),
            self.flashdoze()
        )
    }
}
#[doc = "Force Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fm0(pub u32);
impl Fm0 {
    #[doc = "Boot Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn forcecfg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Boot Configuration"]
    #[inline(always)]
    pub const fn set_forcecfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Fm0 {
    #[inline(always)]
    fn default() -> Fm0 {
        Fm0(0)
    }
}
impl core::fmt::Debug for Fm0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fm0")
            .field("forcecfg", &self.forcecfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fm0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fm0 {{ forcecfg: {=bool:?} }}", self.forcecfg())
    }
}
#[doc = "Global Power Mode Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpmctrl(pub u32);
impl Gpmctrl {
    #[doc = "Low-Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Low-Power Mode"]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Gpmctrl {
    #[inline(always)]
    fn default() -> Gpmctrl {
        Gpmctrl(0)
    }
}
impl core::fmt::Debug for Gpmctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpmctrl")
            .field("lpmode", &self.lpmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpmctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpmctrl {{ lpmode: {=u8:?} }}", self.lpmode())
    }
}
#[doc = "Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mr0(pub u32);
impl Mr0 {
    #[doc = "In System Programming Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ispmode_n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "In System Programming Mode"]
    #[inline(always)]
    pub const fn set_ispmode_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Mr0 {
    #[inline(always)]
    fn default() -> Mr0 {
        Mr0(0)
    }
}
impl core::fmt::Debug for Mr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mr0")
            .field("ispmode_n", &self.ispmode_n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mr0 {{ ispmode_n: {=bool:?} }}", self.ispmode_n())
    }
}
#[doc = "Power Mode Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrlmain(pub u32);
impl Pmctrlmain {
    #[doc = "Low-Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> super::vals::PmctrlmainLpmode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PmctrlmainLpmode::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode"]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: super::vals::PmctrlmainLpmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pmctrlmain {
    #[inline(always)]
    fn default() -> Pmctrlmain {
        Pmctrlmain(0)
    }
}
impl core::fmt::Debug for Pmctrlmain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmctrlmain")
            .field("lpmode", &self.lpmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmctrlmain {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmctrlmain {{ lpmode: {:?} }}", self.lpmode())
    }
}
#[doc = "Power Mode Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrlwake(pub u32);
impl Pmctrlwake {
    #[doc = "Low-Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> super::vals::PmctrlwakeLpmode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PmctrlwakeLpmode::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode"]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: super::vals::PmctrlwakeLpmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Pmctrlwake {
    #[inline(always)]
    fn default() -> Pmctrlwake {
        Pmctrlwake(0)
    }
}
impl core::fmt::Debug for Pmctrlwake {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmctrlwake")
            .field("lpmode", &self.lpmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmctrlwake {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pmctrlwake {{ lpmode: {:?} }}", self.lpmode())
    }
}
#[doc = "Power Mode Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmprot(pub u32);
impl Pmprot {
    #[doc = "Low-Power Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> super::vals::PmprotLpmode {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PmprotLpmode::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode"]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: super::vals::PmprotLpmode) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lock Register"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Lock Register"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pmprot {
    #[inline(always)]
    fn default() -> Pmprot {
        Pmprot(0)
    }
}
impl core::fmt::Debug for Pmprot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmprot")
            .field("lpmode", &self.lpmode())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmprot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmprot {{ lpmode: {:?}, lock: {=bool:?} }}",
            self.lpmode(),
            self.lock()
        )
    }
}
#[doc = "Reset Pin Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpc(pub u32);
impl Rpc {
    #[doc = "Reset Filter Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn filtcfg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Reset Filter Configuration"]
    #[inline(always)]
    pub const fn set_filtcfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn filten(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Filter Enable"]
    #[inline(always)]
    pub const fn set_filten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Low-Power Filter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpfen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Filter Enable"]
    #[inline(always)]
    pub const fn set_lpfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Rpc {
    #[inline(always)]
    fn default() -> Rpc {
        Rpc(0)
    }
}
impl core::fmt::Debug for Rpc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpc")
            .field("filtcfg", &self.filtcfg())
            .field("filten", &self.filten())
            .field("lpfen", &self.lpfen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rpc {{ filtcfg: {=u8:?}, filten: {=bool:?}, lpfen: {=bool:?} }}",
            self.filtcfg(),
            self.filten(),
            self.lpfen()
        )
    }
}
#[doc = "Reset Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcnt(pub u32);
impl Rstcnt {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rstcnt {
    #[inline(always)]
    fn default() -> Rstcnt {
        Rstcnt(0)
    }
}
impl core::fmt::Debug for Rstcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rstcnt")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rstcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rstcnt {{ count: {=u8:?} }}", self.count())
    }
}
#[doc = "SRAM Disable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramdis0(pub u32);
impl Sramdis0 {
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis0(&self) -> super::vals::Dis0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dis0::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis0(&mut self, val: super::vals::Dis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis1(&self) -> super::vals::Dis1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dis1::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis1(&mut self, val: super::vals::Dis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis2(&self) -> super::vals::Dis2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dis2::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis2(&mut self, val: super::vals::Dis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis3(&self) -> super::vals::Dis3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dis3::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis3(&mut self, val: super::vals::Dis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis4(&self) -> super::vals::Dis4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dis4::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis4(&mut self, val: super::vals::Dis4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis5(&self) -> super::vals::Dis5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dis5::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis5(&mut self, val: super::vals::Dis5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis6(&self) -> super::vals::Dis6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dis6::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis6(&mut self, val: super::vals::Dis6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis7(&self) -> super::vals::Dis7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dis7::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis7(&mut self, val: super::vals::Dis7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis8(&self) -> super::vals::Dis8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dis8::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis8(&mut self, val: super::vals::Dis8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis9(&self) -> super::vals::Dis9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dis9::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis9(&mut self, val: super::vals::Dis9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis10(&self) -> super::vals::Dis10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dis10::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis10(&mut self, val: super::vals::Dis10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis11(&self) -> super::vals::Dis11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dis11::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis11(&mut self, val: super::vals::Dis11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis12(&self) -> super::vals::Dis12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dis12::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis12(&mut self, val: super::vals::Dis12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis13(&self) -> super::vals::Dis13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dis13::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis13(&mut self, val: super::vals::Dis13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis14(&self) -> super::vals::Dis14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dis14::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis14(&mut self, val: super::vals::Dis14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis15(&self) -> super::vals::Dis15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dis15::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis15(&mut self, val: super::vals::Dis15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis16(&self) -> super::vals::Dis16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dis16::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis16(&mut self, val: super::vals::Dis16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis17(&self) -> super::vals::Dis17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dis17::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis17(&mut self, val: super::vals::Dis17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis18(&self) -> super::vals::Dis18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dis18::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis18(&mut self, val: super::vals::Dis18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis19(&self) -> super::vals::Dis19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dis19::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis19(&mut self, val: super::vals::Dis19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis20(&self) -> super::vals::Dis20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dis20::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis20(&mut self, val: super::vals::Dis20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis21(&self) -> super::vals::Dis21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dis21::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis21(&mut self, val: super::vals::Dis21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis22(&self) -> super::vals::Dis22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dis22::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis22(&mut self, val: super::vals::Dis22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis23(&self) -> super::vals::Dis23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dis23::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis23(&mut self, val: super::vals::Dis23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis24(&self) -> super::vals::Dis24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dis24::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis24(&mut self, val: super::vals::Dis24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis25(&self) -> super::vals::Dis25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dis25::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis25(&mut self, val: super::vals::Dis25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis26(&self) -> super::vals::Dis26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dis26::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis26(&mut self, val: super::vals::Dis26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis27(&self) -> super::vals::Dis27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dis27::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis27(&mut self, val: super::vals::Dis27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis28(&self) -> super::vals::Dis28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dis28::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis28(&mut self, val: super::vals::Dis28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis29(&self) -> super::vals::Dis29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dis29::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis29(&mut self, val: super::vals::Dis29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis30(&self) -> super::vals::Dis30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dis30::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis30(&mut self, val: super::vals::Dis30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SRAM Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn dis31(&self) -> super::vals::Dis31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dis31::from_bits(val as u8)
    }
    #[doc = "SRAM Disable"]
    #[inline(always)]
    pub const fn set_dis31(&mut self, val: super::vals::Dis31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sramdis0 {
    #[inline(always)]
    fn default() -> Sramdis0 {
        Sramdis0(0)
    }
}
impl core::fmt::Debug for Sramdis0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramdis0")
            .field("dis0", &self.dis0())
            .field("dis1", &self.dis1())
            .field("dis2", &self.dis2())
            .field("dis3", &self.dis3())
            .field("dis4", &self.dis4())
            .field("dis5", &self.dis5())
            .field("dis6", &self.dis6())
            .field("dis7", &self.dis7())
            .field("dis8", &self.dis8())
            .field("dis9", &self.dis9())
            .field("dis10", &self.dis10())
            .field("dis11", &self.dis11())
            .field("dis12", &self.dis12())
            .field("dis13", &self.dis13())
            .field("dis14", &self.dis14())
            .field("dis15", &self.dis15())
            .field("dis16", &self.dis16())
            .field("dis17", &self.dis17())
            .field("dis18", &self.dis18())
            .field("dis19", &self.dis19())
            .field("dis20", &self.dis20())
            .field("dis21", &self.dis21())
            .field("dis22", &self.dis22())
            .field("dis23", &self.dis23())
            .field("dis24", &self.dis24())
            .field("dis25", &self.dis25())
            .field("dis26", &self.dis26())
            .field("dis27", &self.dis27())
            .field("dis28", &self.dis28())
            .field("dis29", &self.dis29())
            .field("dis30", &self.dis30())
            .field("dis31", &self.dis31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramdis0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramdis0 {{ dis0: {:?}, dis1: {:?}, dis2: {:?}, dis3: {:?}, dis4: {:?}, dis5: {:?}, dis6: {:?}, dis7: {:?}, dis8: {:?}, dis9: {:?}, dis10: {:?}, dis11: {:?}, dis12: {:?}, dis13: {:?}, dis14: {:?}, dis15: {:?}, dis16: {:?}, dis17: {:?}, dis18: {:?}, dis19: {:?}, dis20: {:?}, dis21: {:?}, dis22: {:?}, dis23: {:?}, dis24: {:?}, dis25: {:?}, dis26: {:?}, dis27: {:?}, dis28: {:?}, dis29: {:?}, dis30: {:?}, dis31: {:?} }}",
            self.dis0(),
            self.dis1(),
            self.dis2(),
            self.dis3(),
            self.dis4(),
            self.dis5(),
            self.dis6(),
            self.dis7(),
            self.dis8(),
            self.dis9(),
            self.dis10(),
            self.dis11(),
            self.dis12(),
            self.dis13(),
            self.dis14(),
            self.dis15(),
            self.dis16(),
            self.dis17(),
            self.dis18(),
            self.dis19(),
            self.dis20(),
            self.dis21(),
            self.dis22(),
            self.dis23(),
            self.dis24(),
            self.dis25(),
            self.dis26(),
            self.dis27(),
            self.dis28(),
            self.dis29(),
            self.dis30(),
            self.dis31()
        )
    }
}
#[doc = "SRAM Retention"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramret0(pub u32);
impl Sramret0 {
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret0(&self) -> super::vals::Ret0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ret0::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret0(&mut self, val: super::vals::Ret0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret1(&self) -> super::vals::Ret1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ret1::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret1(&mut self, val: super::vals::Ret1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret2(&self) -> super::vals::Ret2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ret2::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret2(&mut self, val: super::vals::Ret2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret3(&self) -> super::vals::Ret3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ret3::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret3(&mut self, val: super::vals::Ret3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret4(&self) -> super::vals::Ret4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ret4::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret4(&mut self, val: super::vals::Ret4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret5(&self) -> super::vals::Ret5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ret5::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret5(&mut self, val: super::vals::Ret5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret6(&self) -> super::vals::Ret6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ret6::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret6(&mut self, val: super::vals::Ret6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret7(&self) -> super::vals::Ret7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ret7::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret7(&mut self, val: super::vals::Ret7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret8(&self) -> super::vals::Ret8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ret8::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret8(&mut self, val: super::vals::Ret8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret9(&self) -> super::vals::Ret9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ret9::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret9(&mut self, val: super::vals::Ret9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret10(&self) -> super::vals::Ret10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Ret10::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret10(&mut self, val: super::vals::Ret10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret11(&self) -> super::vals::Ret11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ret11::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret11(&mut self, val: super::vals::Ret11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret12(&self) -> super::vals::Ret12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ret12::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret12(&mut self, val: super::vals::Ret12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret13(&self) -> super::vals::Ret13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ret13::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret13(&mut self, val: super::vals::Ret13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret14(&self) -> super::vals::Ret14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Ret14::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret14(&mut self, val: super::vals::Ret14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret15(&self) -> super::vals::Ret15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Ret15::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret15(&mut self, val: super::vals::Ret15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret16(&self) -> super::vals::Ret16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ret16::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret16(&mut self, val: super::vals::Ret16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret17(&self) -> super::vals::Ret17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ret17::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret17(&mut self, val: super::vals::Ret17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret18(&self) -> super::vals::Ret18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Ret18::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret18(&mut self, val: super::vals::Ret18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret19(&self) -> super::vals::Ret19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ret19::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret19(&mut self, val: super::vals::Ret19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret20(&self) -> super::vals::Ret20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Ret20::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret20(&mut self, val: super::vals::Ret20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret21(&self) -> super::vals::Ret21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ret21::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret21(&mut self, val: super::vals::Ret21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret22(&self) -> super::vals::Ret22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ret22::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret22(&mut self, val: super::vals::Ret22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret23(&self) -> super::vals::Ret23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Ret23::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret23(&mut self, val: super::vals::Ret23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret24(&self) -> super::vals::Ret24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ret24::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret24(&mut self, val: super::vals::Ret24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret25(&self) -> super::vals::Ret25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Ret25::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret25(&mut self, val: super::vals::Ret25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret26(&self) -> super::vals::Ret26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Ret26::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret26(&mut self, val: super::vals::Ret26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret27(&self) -> super::vals::Ret27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Ret27::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret27(&mut self, val: super::vals::Ret27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret28(&self) -> super::vals::Ret28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ret28::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret28(&mut self, val: super::vals::Ret28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret29(&self) -> super::vals::Ret29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Ret29::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret29(&mut self, val: super::vals::Ret29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret30(&self) -> super::vals::Ret30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Ret30::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret30(&mut self, val: super::vals::Ret30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SRAM Retention"]
    #[must_use]
    #[inline(always)]
    pub const fn ret31(&self) -> super::vals::Ret31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ret31::from_bits(val as u8)
    }
    #[doc = "SRAM Retention"]
    #[inline(always)]
    pub const fn set_ret31(&mut self, val: super::vals::Ret31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sramret0 {
    #[inline(always)]
    fn default() -> Sramret0 {
        Sramret0(0)
    }
}
impl core::fmt::Debug for Sramret0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramret0")
            .field("ret0", &self.ret0())
            .field("ret1", &self.ret1())
            .field("ret2", &self.ret2())
            .field("ret3", &self.ret3())
            .field("ret4", &self.ret4())
            .field("ret5", &self.ret5())
            .field("ret6", &self.ret6())
            .field("ret7", &self.ret7())
            .field("ret8", &self.ret8())
            .field("ret9", &self.ret9())
            .field("ret10", &self.ret10())
            .field("ret11", &self.ret11())
            .field("ret12", &self.ret12())
            .field("ret13", &self.ret13())
            .field("ret14", &self.ret14())
            .field("ret15", &self.ret15())
            .field("ret16", &self.ret16())
            .field("ret17", &self.ret17())
            .field("ret18", &self.ret18())
            .field("ret19", &self.ret19())
            .field("ret20", &self.ret20())
            .field("ret21", &self.ret21())
            .field("ret22", &self.ret22())
            .field("ret23", &self.ret23())
            .field("ret24", &self.ret24())
            .field("ret25", &self.ret25())
            .field("ret26", &self.ret26())
            .field("ret27", &self.ret27())
            .field("ret28", &self.ret28())
            .field("ret29", &self.ret29())
            .field("ret30", &self.ret30())
            .field("ret31", &self.ret31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramret0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramret0 {{ ret0: {:?}, ret1: {:?}, ret2: {:?}, ret3: {:?}, ret4: {:?}, ret5: {:?}, ret6: {:?}, ret7: {:?}, ret8: {:?}, ret9: {:?}, ret10: {:?}, ret11: {:?}, ret12: {:?}, ret13: {:?}, ret14: {:?}, ret15: {:?}, ret16: {:?}, ret17: {:?}, ret18: {:?}, ret19: {:?}, ret20: {:?}, ret21: {:?}, ret22: {:?}, ret23: {:?}, ret24: {:?}, ret25: {:?}, ret26: {:?}, ret27: {:?}, ret28: {:?}, ret29: {:?}, ret30: {:?}, ret31: {:?} }}",
            self.ret0(),
            self.ret1(),
            self.ret2(),
            self.ret3(),
            self.ret4(),
            self.ret5(),
            self.ret6(),
            self.ret7(),
            self.ret8(),
            self.ret9(),
            self.ret10(),
            self.ret11(),
            self.ret12(),
            self.ret13(),
            self.ret14(),
            self.ret15(),
            self.ret16(),
            self.ret17(),
            self.ret18(),
            self.ret19(),
            self.ret20(),
            self.ret21(),
            self.ret22(),
            self.ret23(),
            self.ret24(),
            self.ret25(),
            self.ret26(),
            self.ret27(),
            self.ret28(),
            self.ret29(),
            self.ret30(),
            self.ret31()
        )
    }
}
#[doc = "System Reset Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srie(pub u32);
impl Srie {
    #[doc = "Pin Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset"]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset"]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset"]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CPU1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 Reset"]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VBAT System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT System Reset"]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Srie {
    #[inline(always)]
    fn default() -> Srie {
        Srie(0)
    }
}
impl core::fmt::Debug for Srie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srie")
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wwdt0", &self.wwdt0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("cpu1", &self.cpu1())
            .field("vbat", &self.vbat())
            .field("wwdt1", &self.wwdt1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srie {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srie {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cpu1: {=bool:?}, vbat: {=bool:?}, wwdt1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.scg(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
            self.cpu1(),
            self.vbat(),
            self.wwdt1(),
            self.cdog0(),
            self.cdog1()
        )
    }
}
#[doc = "System Reset Interrupt Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srif(pub u32);
impl Srif {
    #[doc = "Pin Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset"]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset"]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CPU1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 Reset"]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VBAT System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT System Reset"]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Srif {
    #[inline(always)]
    fn default() -> Srif {
        Srif(0)
    }
}
impl core::fmt::Debug for Srif {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srif")
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("lpack", &self.lpack())
            .field("wwdt0", &self.wwdt0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("cpu1", &self.cpu1())
            .field("vbat", &self.vbat())
            .field("wwdt1", &self.wwdt1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srif {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srif {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cpu1: {=bool:?}, vbat: {=bool:?}, wwdt1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
            self.cpu1(),
            self.vbat(),
            self.wwdt1(),
            self.cdog0(),
            self.cdog1()
        )
    }
}
#[doc = "System Reset Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srs(pub u32);
impl Srs {
    #[doc = "Wake-up Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Reset"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-on Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on Reset"]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Voltage Detect Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect Reset"]
    #[inline(always)]
    pub const fn set_vd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Warm Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn warm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Warm Reset"]
    #[inline(always)]
    pub const fn set_warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Fatal Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn fatal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Reset"]
    #[inline(always)]
    pub const fn set_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Debug Access Port Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Access Port Reset"]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn rstack(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timeout"]
    #[inline(always)]
    pub const fn set_rstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset"]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset"]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CPU1 System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 System Reset"]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VBAT System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT System Reset"]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "JTAG System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG System Reset"]
    #[inline(always)]
    pub const fn set_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Security Violation Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn secvio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation Reset"]
    #[inline(always)]
    pub const fn set_secvio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Tamper Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tamper(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Reset"]
    #[inline(always)]
    pub const fn set_tamper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Srs {
    #[inline(always)]
    fn default() -> Srs {
        Srs(0)
    }
}
impl core::fmt::Debug for Srs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srs")
            .field("wakeup", &self.wakeup())
            .field("por", &self.por())
            .field("vd", &self.vd())
            .field("warm", &self.warm())
            .field("fatal", &self.fatal())
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("rstack", &self.rstack())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wwdt0", &self.wwdt0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("cpu1", &self.cpu1())
            .field("vbat", &self.vbat())
            .field("wwdt1", &self.wwdt1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("secvio", &self.secvio())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srs {{ wakeup: {=bool:?}, por: {=bool:?}, vd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cpu1: {=bool:?}, vbat: {=bool:?}, wwdt1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, secvio: {=bool:?}, tamper: {=bool:?} }}",
            self.wakeup(),
            self.por(),
            self.vd(),
            self.warm(),
            self.fatal(),
            self.pin(),
            self.dap(),
            self.rstack(),
            self.lpack(),
            self.scg(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
            self.cpu1(),
            self.vbat(),
            self.wwdt1(),
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
            self.secvio(),
            self.tamper()
        )
    }
}
#[doc = "Sticky System Reset Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrs(pub u32);
impl Ssrs {
    #[doc = "Wake-up Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Reset"]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-on Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn por(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power-on Reset"]
    #[inline(always)]
    pub const fn set_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Voltage Detect Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Voltage Detect Reset"]
    #[inline(always)]
    pub const fn set_vd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Warm Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn warm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Warm Reset"]
    #[inline(always)]
    pub const fn set_warm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Fatal Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn fatal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Reset"]
    #[inline(always)]
    pub const fn set_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Pin Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Pin Reset"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DAP Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DAP Reset"]
    #[inline(always)]
    pub const fn set_dap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn rstack(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Timeout"]
    #[inline(always)]
    pub const fn set_rstack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lpack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    #[inline(always)]
    pub const fn set_lpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "System Clock Generation Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "System Clock Generation Reset"]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lockup Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Lockup Reset"]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CPU1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "CPU1 Reset"]
    #[inline(always)]
    pub const fn set_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VBAT System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn vbat(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "VBAT System Reset"]
    #[inline(always)]
    pub const fn set_vbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 0 Reset"]
    #[inline(always)]
    pub const fn set_cdog0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cdog1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Code Watchdog 1 Reset"]
    #[inline(always)]
    pub const fn set_cdog1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "JTAG System Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "JTAG System Reset"]
    #[inline(always)]
    pub const fn set_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Security Violation Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn secvio(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Security Violation Reset"]
    #[inline(always)]
    pub const fn set_secvio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Tamper Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tamper(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tamper Reset"]
    #[inline(always)]
    pub const fn set_tamper(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ssrs {
    #[inline(always)]
    fn default() -> Ssrs {
        Ssrs(0)
    }
}
impl core::fmt::Debug for Ssrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrs")
            .field("wakeup", &self.wakeup())
            .field("por", &self.por())
            .field("vd", &self.vd())
            .field("warm", &self.warm())
            .field("fatal", &self.fatal())
            .field("pin", &self.pin())
            .field("dap", &self.dap())
            .field("rstack", &self.rstack())
            .field("lpack", &self.lpack())
            .field("scg", &self.scg())
            .field("wwdt0", &self.wwdt0())
            .field("sw", &self.sw())
            .field("lockup", &self.lockup())
            .field("cpu1", &self.cpu1())
            .field("vbat", &self.vbat())
            .field("wwdt1", &self.wwdt1())
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("secvio", &self.secvio())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrs {{ wakeup: {=bool:?}, por: {=bool:?}, vd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cpu1: {=bool:?}, vbat: {=bool:?}, wwdt1: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, secvio: {=bool:?}, tamper: {=bool:?} }}",
            self.wakeup(),
            self.por(),
            self.vd(),
            self.warm(),
            self.fatal(),
            self.pin(),
            self.dap(),
            self.rstack(),
            self.lpack(),
            self.scg(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
            self.cpu1(),
            self.vbat(),
            self.wwdt1(),
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
            self.secvio(),
            self.tamper()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
