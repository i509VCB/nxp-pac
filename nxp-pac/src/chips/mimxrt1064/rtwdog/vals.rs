#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd32en {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    Cmd32en0 = 0x0,
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    Cmd32en1 = 0x01,
}
impl Cmd32en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd32en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd32en {
    #[inline(always)]
    fn from(val: u8) -> Cmd32en {
        Cmd32en::from_bits(val)
    }
}
impl From<Cmd32en> for u8 {
    #[inline(always)]
    fn from(val: Cmd32en) -> u8 {
        Cmd32en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbg {
    #[doc = "Watchdog disabled in chip debug mode."]
    Dbg0 = 0x0,
    #[doc = "Watchdog enabled in chip debug mode."]
    Dbg1 = 0x01,
}
impl Dbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbg {
    #[inline(always)]
    fn from(val: u8) -> Dbg {
        Dbg::from_bits(val)
    }
}
impl From<Dbg> for u8 {
    #[inline(always)]
    fn from(val: Dbg) -> u8 {
        Dbg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Watchdog disabled."]
    En0 = 0x0,
    #[doc = "Watchdog enabled."]
    En1 = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flg {
    #[doc = "No interrupt occurred."]
    Flg0 = 0x0,
    #[doc = "An interrupt occurred."]
    Flg1 = 0x01,
}
impl Flg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flg {
    #[inline(always)]
    fn from(val: u8) -> Flg {
        Flg::from_bits(val)
    }
}
impl From<Flg> for u8 {
    #[inline(always)]
    fn from(val: Flg) -> u8 {
        Flg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    Int0 = 0x0,
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    Int1 = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pres {
    #[doc = "256 prescaler disabled."]
    Pres0 = 0x0,
    #[doc = "256 prescaler enabled."]
    Pres1 = 0x01,
}
impl Pres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pres {
    #[inline(always)]
    fn from(val: u8) -> Pres {
        Pres::from_bits(val)
    }
}
impl From<Pres> for u8 {
    #[inline(always)]
    fn from(val: Pres) -> u8 {
        Pres::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcs {
    #[doc = "Reconfiguring WDOG."]
    Rcs0 = 0x0,
    #[doc = "Reconfiguration is successful."]
    Rcs1 = 0x01,
}
impl Rcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcs {
    #[inline(always)]
    fn from(val: u8) -> Rcs {
        Rcs::from_bits(val)
    }
}
impl From<Rcs> for u8 {
    #[inline(always)]
    fn from(val: Rcs) -> u8 {
        Rcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stop {
    #[doc = "Watchdog disabled in chip stop mode."]
    Stop0 = 0x0,
    #[doc = "Watchdog enabled in chip stop mode."]
    Stop1 = 0x01,
}
impl Stop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stop {
    #[inline(always)]
    fn from(val: u8) -> Stop {
        Stop::from_bits(val)
    }
}
impl From<Stop> for u8 {
    #[inline(always)]
    fn from(val: Stop) -> u8 {
        Stop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tst {
    #[doc = "Watchdog test mode disabled."]
    Tst0 = 0x0,
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    Tst1 = 0x01,
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\] is compared with TOVAL\\[TOVALLOW\\]."]
    Tst2 = 0x02,
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\] is compared with TOVAL\\[TOVALHIGH\\]."]
    Tst3 = 0x03,
}
impl Tst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tst {
    #[inline(always)]
    fn from(val: u8) -> Tst {
        Tst::from_bits(val)
    }
}
impl From<Tst> for u8 {
    #[inline(always)]
    fn from(val: Tst) -> u8 {
        Tst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ulk {
    #[doc = "WDOG is locked."]
    Ulk0 = 0x0,
    #[doc = "WDOG is unlocked."]
    Ulk1 = 0x01,
}
impl Ulk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ulk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ulk {
    #[inline(always)]
    fn from(val: u8) -> Ulk {
        Ulk::from_bits(val)
    }
}
impl From<Ulk> for u8 {
    #[inline(always)]
    fn from(val: Ulk) -> u8 {
        Ulk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Update {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    Update0 = 0x0,
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    Update1 = 0x01,
}
impl Update {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Update {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Update {
    #[inline(always)]
    fn from(val: u8) -> Update {
        Update::from_bits(val)
    }
}
impl From<Update> for u8 {
    #[inline(always)]
    fn from(val: Update) -> u8 {
        Update::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wait {
    #[doc = "Watchdog disabled in chip wait mode."]
    Wait0 = 0x0,
    #[doc = "Watchdog enabled in chip wait mode."]
    Wait1 = 0x01,
}
impl Wait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wait {
    #[inline(always)]
    fn from(val: u8) -> Wait {
        Wait::from_bits(val)
    }
}
impl From<Wait> for u8 {
    #[inline(always)]
    fn from(val: Wait) -> u8 {
        Wait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Win {
    #[doc = "Window mode disabled."]
    Win0 = 0x0,
    #[doc = "Window mode enabled."]
    Win1 = 0x01,
}
impl Win {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Win {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Win {
    #[inline(always)]
    fn from(val: u8) -> Win {
        Win::from_bits(val)
    }
}
impl From<Win> for u8 {
    #[inline(always)]
    fn from(val: Win) -> u8 {
        Win::to_bits(val)
    }
}
