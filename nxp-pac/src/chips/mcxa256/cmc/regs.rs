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
    #[doc = "Flash Wake"]
    #[must_use]
    #[inline(always)]
    pub const fn flashwake(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Wake"]
    #[inline(always)]
    pub const fn set_flashwake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("flashwake", &self.flashwake())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flashcr {{ flashdis: {=bool:?}, flashdoze: {=bool:?}, flashwake: {=bool:?} }}",
            self.flashdis(),
            self.flashdoze(),
            self.flashwake()
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
            "Srie {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.scg(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
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
            "Srif {{ pin: {=bool:?}, dap: {=bool:?}, lpack: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?} }}",
            self.pin(),
            self.dap(),
            self.lpack(),
            self.wwdt0(),
            self.sw(),
            self.lockup(),
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
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srs {{ wakeup: {=bool:?}, por: {=bool:?}, vd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, tamper: {=bool:?} }}",
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
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
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
            .field("cdog0", &self.cdog0())
            .field("cdog1", &self.cdog1())
            .field("jtag", &self.jtag())
            .field("tamper", &self.tamper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrs {{ wakeup: {=bool:?}, por: {=bool:?}, vd: {=bool:?}, warm: {=bool:?}, fatal: {=bool:?}, pin: {=bool:?}, dap: {=bool:?}, rstack: {=bool:?}, lpack: {=bool:?}, scg: {=bool:?}, wwdt0: {=bool:?}, sw: {=bool:?}, lockup: {=bool:?}, cdog0: {=bool:?}, cdog1: {=bool:?}, jtag: {=bool:?}, tamper: {=bool:?} }}",
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
            self.cdog0(),
            self.cdog1(),
            self.jtag(),
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
