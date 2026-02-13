#[doc = "Peripheral Additional Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addinfo(pub u8);
impl Addinfo {
    #[doc = "Host Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iehost(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Mode Enable"]
    #[inline(always)]
    pub const fn set_iehost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for Addinfo {
    #[inline(always)]
    fn default() -> Addinfo {
        Addinfo(0)
    }
}
impl core::fmt::Debug for Addinfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addinfo")
            .field("iehost", &self.iehost())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addinfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Addinfo {{ iehost: {=bool:?} }}", self.iehost())
    }
}
#[doc = "Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr(pub u8);
impl Addr {
    #[doc = "USB Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB Address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Low Speed Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lsen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Low Speed Enable"]
    #[inline(always)]
    pub const fn set_lsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        Addr(0)
    }
}
impl core::fmt::Debug for Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Addr")
            .field("addr", &self.addr())
            .field("lsen", &self.lsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Addr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Addr {{ addr: {=u8:?}, lsen: {=bool:?} }}",
            self.addr(),
            self.lsen()
        )
    }
}
#[doc = "BDT Page 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage1(pub u8);
impl Bdtpage1 {
    #[doc = "BDT Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "BDT Base Address"]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u8) & 0x7f) << 1usize);
    }
}
impl Default for Bdtpage1 {
    #[inline(always)]
    fn default() -> Bdtpage1 {
        Bdtpage1(0)
    }
}
impl core::fmt::Debug for Bdtpage1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage1")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage1 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "BDT Page 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage2(pub u8);
impl Bdtpage2 {
    #[doc = "BDT Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BDT Base Address"]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bdtpage2 {
    #[inline(always)]
    fn default() -> Bdtpage2 {
        Bdtpage2(0)
    }
}
impl core::fmt::Debug for Bdtpage2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage2")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage2 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "BDT Page 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bdtpage3(pub u8);
impl Bdtpage3 {
    #[doc = "BDT Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn bdtba(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "BDT Base Address"]
    #[inline(always)]
    pub const fn set_bdtba(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Bdtpage3 {
    #[inline(always)]
    fn default() -> Bdtpage3 {
        Bdtpage3(0)
    }
}
impl core::fmt::Debug for Bdtpage3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bdtpage3")
            .field("bdtba", &self.bdtba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bdtpage3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Bdtpage3 {{ bdtba: {=u8:?} }}", self.bdtba())
    }
}
#[doc = "USB Clock Recovery Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverCtrl(pub u8);
impl ClkRecoverCtrl {
    #[doc = "Selects the source for the initial FIRC trim fine value used after a reset."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_init_val_sel(&self) -> super::vals::TrimInitValSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TrimInitValSel::from_bits(val as u8)
    }
    #[doc = "Selects the source for the initial FIRC trim fine value used after a reset."]
    #[inline(always)]
    pub const fn set_trim_init_val_sel(&mut self, val: super::vals::TrimInitValSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Restart from IFR Trim Value"]
    #[must_use]
    #[inline(always)]
    pub const fn restart_ifrtrim_en(&self) -> super::vals::RestartIfrtrimEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::RestartIfrtrimEn::from_bits(val as u8)
    }
    #[doc = "Restart from IFR Trim Value"]
    #[inline(always)]
    pub const fn set_restart_ifrtrim_en(&mut self, val: super::vals::RestartIfrtrimEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reset or Resume to Rough Phase Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reset_resume_rough_en(&self) -> super::vals::ResetResumeRoughEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ResetResumeRoughEn::from_bits(val as u8)
    }
    #[doc = "Reset or Resume to Rough Phase Enable"]
    #[inline(always)]
    pub const fn set_reset_resume_rough_en(&mut self, val: super::vals::ResetResumeRoughEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Crystal-Less USB Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clock_recover_en(&self) -> super::vals::ClockRecoverEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ClockRecoverEn::from_bits(val as u8)
    }
    #[doc = "Crystal-Less USB Enable"]
    #[inline(always)]
    pub const fn set_clock_recover_en(&mut self, val: super::vals::ClockRecoverEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for ClkRecoverCtrl {
    #[inline(always)]
    fn default() -> ClkRecoverCtrl {
        ClkRecoverCtrl(0)
    }
}
impl core::fmt::Debug for ClkRecoverCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverCtrl")
            .field("trim_init_val_sel", &self.trim_init_val_sel())
            .field("restart_ifrtrim_en", &self.restart_ifrtrim_en())
            .field("reset_resume_rough_en", &self.reset_resume_rough_en())
            .field("clock_recover_en", &self.clock_recover_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverCtrl {{ trim_init_val_sel: {:?}, restart_ifrtrim_en: {:?}, reset_resume_rough_en: {:?}, clock_recover_en: {:?} }}",
            self.trim_init_val_sel(),
            self.restart_ifrtrim_en(),
            self.reset_resume_rough_en(),
            self.clock_recover_en()
        )
    }
}
#[doc = "Clock Recovery Combined Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntEn(pub u8);
impl ClkRecoverIntEn {
    #[doc = "Overflow error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error_en(&self) -> super::vals::OvfErrorEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OvfErrorEn::from_bits(val as u8)
    }
    #[doc = "Overflow error interrupt enable"]
    #[inline(always)]
    pub const fn set_ovf_error_en(&mut self, val: super::vals::OvfErrorEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntEn {
    #[inline(always)]
    fn default() -> ClkRecoverIntEn {
        ClkRecoverIntEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntEn")
            .field("ovf_error_en", &self.ovf_error_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntEn {{ ovf_error_en: {:?} }}",
            self.ovf_error_en()
        )
    }
}
#[doc = "Clock Recovery Separated Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIntStatus(pub u8);
impl ClkRecoverIntStatus {
    #[doc = "Overflow Error Interrupt Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ovf_error(&self) -> super::vals::OvfError {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::OvfError::from_bits(val as u8)
    }
    #[doc = "Overflow Error Interrupt Status Flag"]
    #[inline(always)]
    pub const fn set_ovf_error(&mut self, val: super::vals::OvfError) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for ClkRecoverIntStatus {
    #[inline(always)]
    fn default() -> ClkRecoverIntStatus {
        ClkRecoverIntStatus(0)
    }
}
impl core::fmt::Debug for ClkRecoverIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIntStatus")
            .field("ovf_error", &self.ovf_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkRecoverIntStatus {{ ovf_error: {:?} }}",
            self.ovf_error()
        )
    }
}
#[doc = "FIRC Oscillator Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRecoverIrcEn(pub u8);
impl ClkRecoverIrcEn {
    #[doc = "Fast IRC enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irc_en(&self) -> super::vals::IrcEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IrcEn::from_bits(val as u8)
    }
    #[doc = "Fast IRC enable"]
    #[inline(always)]
    pub const fn set_irc_en(&mut self, val: super::vals::IrcEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
}
impl Default for ClkRecoverIrcEn {
    #[inline(always)]
    fn default() -> ClkRecoverIrcEn {
        ClkRecoverIrcEn(0)
    }
}
impl core::fmt::Debug for ClkRecoverIrcEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRecoverIrcEn")
            .field("irc_en", &self.irc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRecoverIrcEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ClkRecoverIrcEn {{ irc_en: {:?} }}", self.irc_en())
    }
}
#[doc = "USB OTG Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u8);
impl Control {
    #[doc = "VBUS Monitoring Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Monitoring Source Select"]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "VBUS Session Valid status"]
    #[must_use]
    #[inline(always)]
    pub const fn sess_vld(&self) -> super::vals::SessVld {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SessVld::from_bits(val as u8)
    }
    #[doc = "VBUS Session Valid status"]
    #[inline(always)]
    pub const fn set_sess_vld(&mut self, val: super::vals::SessVld) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup in Non-OTG Device Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dppullupnonotg(&self) -> super::vals::Dppullupnonotg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dppullupnonotg::from_bits(val as u8)
    }
    #[doc = "DP Pullup in Non-OTG Device Mode"]
    #[inline(always)]
    pub const fn set_dppullupnonotg(&mut self, val: super::vals::Dppullupnonotg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("sess_vld", &self.sess_vld())
            .field("dppullupnonotg", &self.dppullupnonotg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ vbus_source_sel: {=bool:?}, sess_vld: {:?}, dppullupnonotg: {:?} }}",
            self.vbus_source_sel(),
            self.sess_vld(),
            self.dppullupnonotg()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u8);
impl Ctl {
    #[doc = "USB Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbensofen(&self) -> super::vals::Usbensofen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbensofen::from_bits(val as u8)
    }
    #[doc = "USB Enable"]
    #[inline(always)]
    pub const fn set_usbensofen(&mut self, val: super::vals::Usbensofen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Odd Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn oddrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Odd Reset"]
    #[inline(always)]
    pub const fn set_oddrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Resume"]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Resume"]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Host Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hostmodeen(&self) -> super::vals::Hostmodeen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hostmodeen::from_bits(val as u8)
    }
    #[doc = "Host Mode Enable"]
    #[inline(always)]
    pub const fn set_hostmodeen(&mut self, val: super::vals::Hostmodeen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Reset Signaling Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Signaling Enable"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
    }
    #[doc = "TXD Suspend And Token Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn txsuspendtokenbusy(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TXD Suspend And Token Busy"]
    #[inline(always)]
    pub const fn set_txsuspendtokenbusy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
    }
    #[doc = "Live USB Single-Ended Zero signal"]
    #[must_use]
    #[inline(always)]
    pub const fn se0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Live USB Single-Ended Zero signal"]
    #[inline(always)]
    pub const fn set_se0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "Live USB Differential Receiver JSTATE Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn jstate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Live USB Differential Receiver JSTATE Signal"]
    #[inline(always)]
    pub const fn set_jstate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
impl core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl")
            .field("usbensofen", &self.usbensofen())
            .field("oddrst", &self.oddrst())
            .field("resume", &self.resume())
            .field("hostmodeen", &self.hostmodeen())
            .field("reset", &self.reset())
            .field("txsuspendtokenbusy", &self.txsuspendtokenbusy())
            .field("se0", &self.se0())
            .field("jstate", &self.jstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl {{ usbensofen: {:?}, oddrst: {=bool:?}, resume: {=bool:?}, hostmodeen: {:?}, reset: {=bool:?}, txsuspendtokenbusy: {=bool:?}, se0: {=bool:?}, jstate: {=bool:?} }}",
            self.usbensofen(),
            self.oddrst(),
            self.resume(),
            self.hostmodeen(),
            self.reset(),
            self.txsuspendtokenbusy(),
            self.se0(),
            self.jstate()
        )
    }
}
#[doc = "Endpoint Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endpt(pub u8);
impl Endpt {
    #[doc = "Endpoint Handshaking Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ephshk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Handshaking Enable"]
    #[inline(always)]
    pub const fn set_ephshk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
    #[doc = "Endpoint Stalled"]
    #[must_use]
    #[inline(always)]
    pub const fn epstall(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint Stalled"]
    #[inline(always)]
    pub const fn set_epstall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
    }
    #[doc = "Endpoint for TX transfers enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eptxen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint for TX transfers enable"]
    #[inline(always)]
    pub const fn set_eptxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "Endpoint for RX transfers enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eprxen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Endpoint for RX transfers enable"]
    #[inline(always)]
    pub const fn set_eprxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
    }
    #[doc = "Control Transfer Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn epctldis(&self) -> super::vals::Epctldis {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Epctldis::from_bits(val as u8)
    }
    #[doc = "Control Transfer Disable"]
    #[inline(always)]
    pub const fn set_epctldis(&mut self, val: super::vals::Epctldis) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Retry Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn retrydis(&self) -> super::vals::Retrydis {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Retrydis::from_bits(val as u8)
    }
    #[doc = "Retry Disable"]
    #[inline(always)]
    pub const fn set_retrydis(&mut self, val: super::vals::Retrydis) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Host Without A Hub"]
    #[must_use]
    #[inline(always)]
    pub const fn hostwohub(&self) -> super::vals::Hostwohub {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hostwohub::from_bits(val as u8)
    }
    #[doc = "Host Without A Hub"]
    #[inline(always)]
    pub const fn set_hostwohub(&mut self, val: super::vals::Hostwohub) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Endpt {
    #[inline(always)]
    fn default() -> Endpt {
        Endpt(0)
    }
}
impl core::fmt::Debug for Endpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endpt")
            .field("ephshk", &self.ephshk())
            .field("epstall", &self.epstall())
            .field("eptxen", &self.eptxen())
            .field("eprxen", &self.eprxen())
            .field("epctldis", &self.epctldis())
            .field("retrydis", &self.retrydis())
            .field("hostwohub", &self.hostwohub())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endpt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endpt {{ ephshk: {=bool:?}, epstall: {=bool:?}, eptxen: {=bool:?}, eprxen: {=bool:?}, epctldis: {:?}, retrydis: {:?}, hostwohub: {:?} }}",
            self.ephshk(),
            self.epstall(),
            self.eptxen(),
            self.eprxen(),
            self.epctldis(),
            self.retrydis(),
            self.hostwohub()
        )
    }
}
#[doc = "Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erren(pub u8);
impl Erren {
    #[doc = "PIDERR Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn piderren(&self) -> super::vals::Piderren {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Piderren::from_bits(val as u8)
    }
    #[doc = "PIDERR Interrupt Enable"]
    #[inline(always)]
    pub const fn set_piderren(&mut self, val: super::vals::Piderren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CRC5/EOF Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc5eofen(&self) -> super::vals::Crc5eofen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Crc5eofen::from_bits(val as u8)
    }
    #[doc = "CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    pub const fn set_crc5eofen(&mut self, val: super::vals::Crc5eofen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CRC16 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc16en(&self) -> super::vals::Crc16en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Crc16en::from_bits(val as u8)
    }
    #[doc = "CRC16 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_crc16en(&mut self, val: super::vals::Crc16en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "DFN8 (Data Field Not Integer Number of Bytes) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dfn8en(&self) -> super::vals::Dfn8en {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dfn8en::from_bits(val as u8)
    }
    #[doc = "DFN8 (Data Field Not Integer Number of Bytes) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dfn8en(&mut self, val: super::vals::Dfn8en) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "BTOERR (Bus Timeout Error) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn btoerren(&self) -> super::vals::Btoerren {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Btoerren::from_bits(val as u8)
    }
    #[doc = "BTOERR (Bus Timeout Error) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_btoerren(&mut self, val: super::vals::Btoerren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "DMAERR Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerren(&self) -> super::vals::Dmaerren {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmaerren::from_bits(val as u8)
    }
    #[doc = "DMAERR Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dmaerren(&mut self, val: super::vals::Dmaerren) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "OWNERR Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ownerren(&self) -> super::vals::Ownerren {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ownerren::from_bits(val as u8)
    }
    #[doc = "OWNERR Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ownerren(&mut self, val: super::vals::Ownerren) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "BTSERR (Bit Stuff Error) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn btserren(&self) -> super::vals::Btserren {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Btserren::from_bits(val as u8)
    }
    #[doc = "BTSERR (Bit Stuff Error) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_btserren(&mut self, val: super::vals::Btserren) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Erren {
    #[inline(always)]
    fn default() -> Erren {
        Erren(0)
    }
}
impl core::fmt::Debug for Erren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erren")
            .field("piderren", &self.piderren())
            .field("crc5eofen", &self.crc5eofen())
            .field("crc16en", &self.crc16en())
            .field("dfn8en", &self.dfn8en())
            .field("btoerren", &self.btoerren())
            .field("dmaerren", &self.dmaerren())
            .field("ownerren", &self.ownerren())
            .field("btserren", &self.btserren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erren {{ piderren: {:?}, crc5eofen: {:?}, crc16en: {:?}, dfn8en: {:?}, btoerren: {:?}, dmaerren: {:?}, ownerren: {:?}, btserren: {:?} }}",
            self.piderren(),
            self.crc5eofen(),
            self.crc16en(),
            self.dfn8en(),
            self.btoerren(),
            self.dmaerren(),
            self.ownerren(),
            self.btserren()
        )
    }
}
#[doc = "Error Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u8);
impl Errstat {
    #[doc = "PID Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn piderr(&self) -> super::vals::Piderr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Piderr::from_bits(val as u8)
    }
    #[doc = "PID Error Flag"]
    #[inline(always)]
    pub const fn set_piderr(&mut self, val: super::vals::Piderr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "CRC5 Error or End of Frame Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn crc5eof(&self) -> super::vals::Crc5eof {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Crc5eof::from_bits(val as u8)
    }
    #[doc = "CRC5 Error or End of Frame Error Flag"]
    #[inline(always)]
    pub const fn set_crc5eof(&mut self, val: super::vals::Crc5eof) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "CRC16 Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn crc16(&self) -> super::vals::Crc16 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Crc16::from_bits(val as u8)
    }
    #[doc = "CRC16 Error Flag"]
    #[inline(always)]
    pub const fn set_crc16(&mut self, val: super::vals::Crc16) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Data Field Not 8 Bits Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dfn8(&self) -> super::vals::Dfn8 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dfn8::from_bits(val as u8)
    }
    #[doc = "Data Field Not 8 Bits Flag"]
    #[inline(always)]
    pub const fn set_dfn8(&mut self, val: super::vals::Dfn8) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Bus Turnaround Timeout Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn btoerr(&self) -> super::vals::Btoerr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Btoerr::from_bits(val as u8)
    }
    #[doc = "Bus Turnaround Timeout Error Flag"]
    #[inline(always)]
    pub const fn set_btoerr(&mut self, val: super::vals::Btoerr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "DMA Access Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerr(&self) -> super::vals::Dmaerr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmaerr::from_bits(val as u8)
    }
    #[doc = "DMA Access Error Flag"]
    #[inline(always)]
    pub const fn set_dmaerr(&mut self, val: super::vals::Dmaerr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "BD Unavailable Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ownerr(&self) -> super::vals::Ownerr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Ownerr::from_bits(val as u8)
    }
    #[doc = "BD Unavailable Error Flag"]
    #[inline(always)]
    pub const fn set_ownerr(&mut self, val: super::vals::Ownerr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Bit Stuff Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn btserr(&self) -> super::vals::Btserr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Btserr::from_bits(val as u8)
    }
    #[doc = "Bit Stuff Error Flag"]
    #[inline(always)]
    pub const fn set_btserr(&mut self, val: super::vals::Btserr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Errstat {
    #[inline(always)]
    fn default() -> Errstat {
        Errstat(0)
    }
}
impl core::fmt::Debug for Errstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Errstat")
            .field("piderr", &self.piderr())
            .field("crc5eof", &self.crc5eof())
            .field("crc16", &self.crc16())
            .field("dfn8", &self.dfn8())
            .field("btoerr", &self.btoerr())
            .field("dmaerr", &self.dmaerr())
            .field("ownerr", &self.ownerr())
            .field("btserr", &self.btserr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Errstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Errstat {{ piderr: {:?}, crc5eof: {:?}, crc16: {:?}, dfn8: {:?}, btoerr: {:?}, dmaerr: {:?}, ownerr: {:?}, btserr: {:?} }}",
            self.piderr(),
            self.crc5eof(),
            self.crc16(),
            self.dfn8(),
            self.btoerr(),
            self.dmaerr(),
            self.ownerr(),
            self.btserr()
        )
    }
}
#[doc = "Frame Number Register High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnumh(pub u8);
impl Frmnumh {
    #[doc = "Frame Number, Bits 8-10"]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Frame Number, Bits 8-10"]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
    }
}
impl Default for Frmnumh {
    #[inline(always)]
    fn default() -> Frmnumh {
        Frmnumh(0)
    }
}
impl core::fmt::Debug for Frmnumh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frmnumh").field("frm", &self.frm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frmnumh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frmnumh {{ frm: {=u8:?} }}", self.frm())
    }
}
#[doc = "Frame Number Register Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnuml(pub u8);
impl Frmnuml {
    #[doc = "Frame Number, Bits 0-7"]
    #[must_use]
    #[inline(always)]
    pub const fn frm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame Number, Bits 0-7"]
    #[inline(always)]
    pub const fn set_frm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Frmnuml {
    #[inline(always)]
    fn default() -> Frmnuml {
        Frmnuml(0)
    }
}
impl core::fmt::Debug for Frmnuml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frmnuml").field("frm", &self.frm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frmnuml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frmnuml {{ frm: {=u8:?} }}", self.frm())
    }
}
#[doc = "Peripheral ID Complement"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idcomp(pub u8);
impl Idcomp {
    #[doc = "Negative Peripheral ID"]
    #[must_use]
    #[inline(always)]
    pub const fn nid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Negative Peripheral ID"]
    #[inline(always)]
    pub const fn set_nid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
}
impl Default for Idcomp {
    #[inline(always)]
    fn default() -> Idcomp {
        Idcomp(0)
    }
}
impl core::fmt::Debug for Idcomp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Idcomp").field("nid", &self.nid()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Idcomp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Idcomp {{ nid: {=u8:?} }}", self.nid())
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u8);
impl Inten {
    #[doc = "USBRST Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbrsten(&self) -> super::vals::Usbrsten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbrsten::from_bits(val as u8)
    }
    #[doc = "USBRST Interrupt Enable"]
    #[inline(always)]
    pub const fn set_usbrsten(&mut self, val: super::vals::Usbrsten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "ERROR Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erroren(&self) -> super::vals::Erroren {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Erroren::from_bits(val as u8)
    }
    #[doc = "ERROR Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erroren(&mut self, val: super::vals::Erroren) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "SOFTOK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn softoken(&self) -> super::vals::Softoken {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Softoken::from_bits(val as u8)
    }
    #[doc = "SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_softoken(&mut self, val: super::vals::Softoken) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "TOKDNE Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tokdneen(&self) -> super::vals::Tokdneen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tokdneen::from_bits(val as u8)
    }
    #[doc = "TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tokdneen(&mut self, val: super::vals::Tokdneen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "SLEEP Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sleepen(&self) -> super::vals::Sleepen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sleepen::from_bits(val as u8)
    }
    #[doc = "SLEEP Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sleepen(&mut self, val: super::vals::Sleepen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "RESUME Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn resumeen(&self) -> super::vals::Resumeen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Resumeen::from_bits(val as u8)
    }
    #[doc = "RESUME Interrupt Enable"]
    #[inline(always)]
    pub const fn set_resumeen(&mut self, val: super::vals::Resumeen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "ATTACH Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn attachen(&self) -> super::vals::Attachen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Attachen::from_bits(val as u8)
    }
    #[doc = "ATTACH Interrupt Enable"]
    #[inline(always)]
    pub const fn set_attachen(&mut self, val: super::vals::Attachen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "STALL Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stallen(&self) -> super::vals::Stallen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Stallen::from_bits(val as u8)
    }
    #[doc = "STALL Interrupt Enable"]
    #[inline(always)]
    pub const fn set_stallen(&mut self, val: super::vals::Stallen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
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
            .field("usbrsten", &self.usbrsten())
            .field("erroren", &self.erroren())
            .field("softoken", &self.softoken())
            .field("tokdneen", &self.tokdneen())
            .field("sleepen", &self.sleepen())
            .field("resumeen", &self.resumeen())
            .field("attachen", &self.attachen())
            .field("stallen", &self.stallen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ usbrsten: {:?}, erroren: {:?}, softoken: {:?}, tokdneen: {:?}, sleepen: {:?}, resumeen: {:?}, attachen: {:?}, stallen: {:?} }}",
            self.usbrsten(),
            self.erroren(),
            self.softoken(),
            self.tokdneen(),
            self.sleepen(),
            self.resumeen(),
            self.attachen(),
            self.stallen()
        )
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Istat(pub u8);
impl Istat {
    #[doc = "USB Reset Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn usbrst(&self) -> super::vals::Usbrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Usbrst::from_bits(val as u8)
    }
    #[doc = "USB Reset Flag"]
    #[inline(always)]
    pub const fn set_usbrst(&mut self, val: super::vals::Usbrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> super::vals::Error {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Error::from_bits(val as u8)
    }
    #[doc = "Error Flag"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: super::vals::Error) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Start Of Frame (SOF) Token Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn softok(&self) -> super::vals::Softok {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Softok::from_bits(val as u8)
    }
    #[doc = "Start Of Frame (SOF) Token Flag"]
    #[inline(always)]
    pub const fn set_softok(&mut self, val: super::vals::Softok) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Current Token Processing Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tokdne(&self) -> super::vals::Tokdne {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tokdne::from_bits(val as u8)
    }
    #[doc = "Current Token Processing Flag"]
    #[inline(always)]
    pub const fn set_tokdne(&mut self, val: super::vals::Tokdne) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Sleep Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep(&self) -> super::vals::Sleep {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Sleep::from_bits(val as u8)
    }
    #[doc = "Sleep Flag"]
    #[inline(always)]
    pub const fn set_sleep(&mut self, val: super::vals::Sleep) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Resume Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn resume(&self) -> super::vals::Resume {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Resume::from_bits(val as u8)
    }
    #[doc = "Resume Flag"]
    #[inline(always)]
    pub const fn set_resume(&mut self, val: super::vals::Resume) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Attach Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn attach(&self) -> super::vals::Attach {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Attach::from_bits(val as u8)
    }
    #[doc = "Attach Interrupt Flag"]
    #[inline(always)]
    pub const fn set_attach(&mut self, val: super::vals::Attach) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Stall Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn stall(&self) -> super::vals::Stall {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Stall::from_bits(val as u8)
    }
    #[doc = "Stall Interrupt Flag"]
    #[inline(always)]
    pub const fn set_stall(&mut self, val: super::vals::Stall) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Istat {
    #[inline(always)]
    fn default() -> Istat {
        Istat(0)
    }
}
impl core::fmt::Debug for Istat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Istat")
            .field("usbrst", &self.usbrst())
            .field("error", &self.error())
            .field("softok", &self.softok())
            .field("tokdne", &self.tokdne())
            .field("sleep", &self.sleep())
            .field("resume", &self.resume())
            .field("attach", &self.attach())
            .field("stall", &self.stall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Istat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Istat {{ usbrst: {:?}, error: {:?}, softok: {:?}, tokdne: {:?}, sleep: {:?}, resume: {:?}, attach: {:?}, stall: {:?} }}",
            self.usbrst(),
            self.error(),
            self.softok(),
            self.tokdne(),
            self.sleep(),
            self.resume(),
            self.attach(),
            self.stall()
        )
    }
}
#[doc = "Miscellaneous Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Miscctrl(pub u8);
impl Miscctrl {
    #[doc = "Dynamic SOF Threshold Compare mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sofdynthld(&self) -> super::vals::Sofdynthld {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sofdynthld::from_bits(val as u8)
    }
    #[doc = "Dynamic SOF Threshold Compare mode"]
    #[inline(always)]
    pub const fn set_sofdynthld(&mut self, val: super::vals::Sofdynthld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "SOF_TOK Interrupt Generation Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sofbusset(&self) -> super::vals::Sofbusset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sofbusset::from_bits(val as u8)
    }
    #[doc = "SOF_TOK Interrupt Generation Mode Select"]
    #[inline(always)]
    pub const fn set_sofbusset(&mut self, val: super::vals::Sofbusset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "OWN Error Detect for ISO IN and ISO OUT Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ownerrisodis(&self) -> super::vals::Ownerrisodis {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ownerrisodis::from_bits(val as u8)
    }
    #[doc = "OWN Error Detect for ISO IN and ISO OUT Disable"]
    #[inline(always)]
    pub const fn set_ownerrisodis(&mut self, val: super::vals::Ownerrisodis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "VREGIN Rising Edge Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vredg_en(&self) -> super::vals::VredgEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VredgEn::from_bits(val as u8)
    }
    #[doc = "VREGIN Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub const fn set_vredg_en(&mut self, val: super::vals::VredgEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "VREGIN Falling Edge Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vfedg_en(&self) -> super::vals::VfedgEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VfedgEn::from_bits(val as u8)
    }
    #[doc = "VREGIN Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub const fn set_vfedg_en(&mut self, val: super::vals::VfedgEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "USB Peripheral Mode Stall Adjust Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stl_adj_en(&self) -> super::vals::StlAdjEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StlAdjEn::from_bits(val as u8)
    }
    #[doc = "USB Peripheral Mode Stall Adjust Enable"]
    #[inline(always)]
    pub const fn set_stl_adj_en(&mut self, val: super::vals::StlAdjEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Miscctrl {
    #[inline(always)]
    fn default() -> Miscctrl {
        Miscctrl(0)
    }
}
impl core::fmt::Debug for Miscctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Miscctrl")
            .field("sofdynthld", &self.sofdynthld())
            .field("sofbusset", &self.sofbusset())
            .field("ownerrisodis", &self.ownerrisodis())
            .field("vredg_en", &self.vredg_en())
            .field("vfedg_en", &self.vfedg_en())
            .field("stl_adj_en", &self.stl_adj_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Miscctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Miscctrl {{ sofdynthld: {:?}, sofbusset: {:?}, ownerrisodis: {:?}, vredg_en: {:?}, vfedg_en: {:?}, stl_adj_en: {:?} }}",
            self.sofdynthld(),
            self.sofbusset(),
            self.ownerrisodis(),
            self.vredg_en(),
            self.vfedg_en(),
            self.stl_adj_en()
        )
    }
}
#[doc = "USB OTG Observe"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Observe(pub u8);
impl Observe {
    #[doc = "D- Pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn dmpd(&self) -> super::vals::Dmpd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmpd::from_bits(val as u8)
    }
    #[doc = "D- Pulldown"]
    #[inline(always)]
    pub const fn set_dmpd(&mut self, val: super::vals::Dmpd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "D+ Pulldown"]
    #[must_use]
    #[inline(always)]
    pub const fn dppd(&self) -> super::vals::Dppd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dppd::from_bits(val as u8)
    }
    #[doc = "D+ Pulldown"]
    #[inline(always)]
    pub const fn set_dppd(&mut self, val: super::vals::Dppd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "D+ Pullup"]
    #[must_use]
    #[inline(always)]
    pub const fn dppu(&self) -> super::vals::Dppu {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dppu::from_bits(val as u8)
    }
    #[doc = "D+ Pullup"]
    #[inline(always)]
    pub const fn set_dppu(&mut self, val: super::vals::Dppu) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Observe {
    #[inline(always)]
    fn default() -> Observe {
        Observe(0)
    }
}
impl core::fmt::Debug for Observe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Observe")
            .field("dmpd", &self.dmpd())
            .field("dppd", &self.dppd())
            .field("dppu", &self.dppu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Observe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Observe {{ dmpd: {:?}, dppd: {:?}, dppu: {:?} }}",
            self.dmpd(),
            self.dppd(),
            self.dppu()
        )
    }
}
#[doc = "OTG Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgctl(pub u8);
impl Otgctl {
    #[doc = "On-The-Go Pullup and Pulldown Resistor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn otgen(&self) -> super::vals::Otgen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Otgen::from_bits(val as u8)
    }
    #[doc = "On-The-Go Pullup and Pulldown Resistor Enable"]
    #[inline(always)]
    pub const fn set_otgen(&mut self, val: super::vals::Otgen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "D- Data Line Pulldown Resistor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmlow(&self) -> super::vals::Dmlow {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmlow::from_bits(val as u8)
    }
    #[doc = "D- Data Line Pulldown Resistor Enable"]
    #[inline(always)]
    pub const fn set_dmlow(&mut self, val: super::vals::Dmlow) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "D+ Data Line pulldown Resistor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dplow(&self) -> super::vals::Dplow {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dplow::from_bits(val as u8)
    }
    #[doc = "D+ Data Line pulldown Resistor Enable"]
    #[inline(always)]
    pub const fn set_dplow(&mut self, val: super::vals::Dplow) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "D+ Data Line Pullup Resistor Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dphigh(&self) -> super::vals::Dphigh {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dphigh::from_bits(val as u8)
    }
    #[doc = "D+ Data Line Pullup Resistor Enable"]
    #[inline(always)]
    pub const fn set_dphigh(&mut self, val: super::vals::Dphigh) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Otgctl {
    #[inline(always)]
    fn default() -> Otgctl {
        Otgctl(0)
    }
}
impl core::fmt::Debug for Otgctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgctl")
            .field("otgen", &self.otgen())
            .field("dmlow", &self.dmlow())
            .field("dplow", &self.dplow())
            .field("dphigh", &self.dphigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgctl {{ otgen: {:?}, dmlow: {:?}, dplow: {:?}, dphigh: {:?} }}",
            self.otgen(),
            self.dmlow(),
            self.dplow(),
            self.dphigh()
        )
    }
}
#[doc = "OTG Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgicr(pub u8);
impl Otgicr {
    #[doc = "Line State Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linestateen(&self) -> super::vals::Linestateen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Linestateen::from_bits(val as u8)
    }
    #[doc = "Line State Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_linestateen(&mut self, val: super::vals::Linestateen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "1-Millisecond Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn onemsecen(&self) -> super::vals::Onemsecen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Onemsecen::from_bits(val as u8)
    }
    #[doc = "1-Millisecond Interrupt Enable"]
    #[inline(always)]
    pub const fn set_onemsecen(&mut self, val: super::vals::Onemsecen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgicr {
    #[inline(always)]
    fn default() -> Otgicr {
        Otgicr(0)
    }
}
impl core::fmt::Debug for Otgicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgicr")
            .field("linestateen", &self.linestateen())
            .field("onemsecen", &self.onemsecen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgicr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgicr {{ linestateen: {:?}, onemsecen: {:?} }}",
            self.linestateen(),
            self.onemsecen()
        )
    }
}
#[doc = "OTG Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgistat(pub u8);
impl Otgistat {
    #[doc = "Line State Change Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn line_state_chg(&self) -> super::vals::LineStateChg {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::LineStateChg::from_bits(val as u8)
    }
    #[doc = "Line State Change Interrupt Flag"]
    #[inline(always)]
    pub const fn set_line_state_chg(&mut self, val: super::vals::LineStateChg) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "One Millisecond Timer Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn onemsec(&self) -> super::vals::Onemsec {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Onemsec::from_bits(val as u8)
    }
    #[doc = "One Millisecond Timer Timeout Flag"]
    #[inline(always)]
    pub const fn set_onemsec(&mut self, val: super::vals::Onemsec) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgistat {
    #[inline(always)]
    fn default() -> Otgistat {
        Otgistat(0)
    }
}
impl core::fmt::Debug for Otgistat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgistat")
            .field("line_state_chg", &self.line_state_chg())
            .field("onemsec", &self.onemsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgistat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgistat {{ line_state_chg: {:?}, onemsec: {:?} }}",
            self.line_state_chg(),
            self.onemsec()
        )
    }
}
#[doc = "OTG Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgstat(pub u8);
impl Otgstat {
    #[doc = "Line State Stable"]
    #[must_use]
    #[inline(always)]
    pub const fn linestatestable(&self) -> super::vals::Linestatestable {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Linestatestable::from_bits(val as u8)
    }
    #[doc = "Line State Stable"]
    #[inline(always)]
    pub const fn set_linestatestable(&mut self, val: super::vals::Linestatestable) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Reserved for 1 ms count"]
    #[must_use]
    #[inline(always)]
    pub const fn onemsec(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved for 1 ms count"]
    #[inline(always)]
    pub const fn set_onemsec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
}
impl Default for Otgstat {
    #[inline(always)]
    fn default() -> Otgstat {
        Otgstat(0)
    }
}
impl core::fmt::Debug for Otgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgstat")
            .field("linestatestable", &self.linestatestable())
            .field("onemsec", &self.onemsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgstat {{ linestatestable: {:?}, onemsec: {=bool:?} }}",
            self.linestatestable(),
            self.onemsec()
        )
    }
}
#[doc = "Peripheral ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perid(pub u8);
impl Perid {
    #[doc = "Peripheral Identification"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Peripheral Identification"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u8) & 0x3f) << 0usize);
    }
}
impl Default for Perid {
    #[inline(always)]
    fn default() -> Perid {
        Perid(0)
    }
}
impl core::fmt::Debug for Perid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Perid").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Perid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Perid {{ id: {=u8:?} }}", self.id())
    }
}
#[doc = "Peripheral Revision"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rev(pub u8);
impl Rev {
    #[doc = "Revision"]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision"]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Rev {
    #[inline(always)]
    fn default() -> Rev {
        Rev(0)
    }
}
impl core::fmt::Debug for Rev {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rev").field("rev", &self.rev()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rev {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rev {{ rev: {=u8:?} }}", self.rev())
    }
}
#[doc = "SOF Threshold"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Softhld(pub u8);
impl Softhld {
    #[doc = "SOF Count Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SOF Count Threshold"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Softhld {
    #[inline(always)]
    fn default() -> Softhld {
        Softhld(0)
    }
}
impl core::fmt::Debug for Softhld {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Softhld").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Softhld {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Softhld {{ cnt: {=u8:?} }}", self.cnt())
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallIhDis(pub u8);
impl StallIhDis {
    #[doc = "Disable Endpoint 8 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis8(&self) -> super::vals::StallIDis8 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StallIDis8::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 8 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis8(&mut self, val: super::vals::StallIDis8) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Disable Endpoint 9 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis9(&self) -> super::vals::StallIDis9 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StallIDis9::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 9 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis9(&mut self, val: super::vals::StallIDis9) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Disable Endpoint 10 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis10(&self) -> super::vals::StallIDis10 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StallIDis10::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 10 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis10(&mut self, val: super::vals::StallIDis10) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Disable Endpoint 11 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis11(&self) -> super::vals::StallIDis11 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StallIDis11::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 11 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis11(&mut self, val: super::vals::StallIDis11) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Disable Endpoint 12 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis12(&self) -> super::vals::StallIDis12 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StallIDis12::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 12 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis12(&mut self, val: super::vals::StallIDis12) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Disable Endpoint 13 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis13(&self) -> super::vals::StallIDis13 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StallIDis13::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 13 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis13(&mut self, val: super::vals::StallIDis13) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Disable Endpoint 14 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis14(&self) -> super::vals::StallIDis14 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StallIDis14::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 14 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis14(&mut self, val: super::vals::StallIDis14) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Disable Endpoint 15 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis15(&self) -> super::vals::StallIDis15 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StallIDis15::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 15 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis15(&mut self, val: super::vals::StallIDis15) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for StallIhDis {
    #[inline(always)]
    fn default() -> StallIhDis {
        StallIhDis(0)
    }
}
impl core::fmt::Debug for StallIhDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallIhDis")
            .field("stall_i_dis8", &self.stall_i_dis8())
            .field("stall_i_dis9", &self.stall_i_dis9())
            .field("stall_i_dis10", &self.stall_i_dis10())
            .field("stall_i_dis11", &self.stall_i_dis11())
            .field("stall_i_dis12", &self.stall_i_dis12())
            .field("stall_i_dis13", &self.stall_i_dis13())
            .field("stall_i_dis14", &self.stall_i_dis14())
            .field("stall_i_dis15", &self.stall_i_dis15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallIhDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallIhDis {{ stall_i_dis8: {:?}, stall_i_dis9: {:?}, stall_i_dis10: {:?}, stall_i_dis11: {:?}, stall_i_dis12: {:?}, stall_i_dis13: {:?}, stall_i_dis14: {:?}, stall_i_dis15: {:?} }}",
            self.stall_i_dis8(),
            self.stall_i_dis9(),
            self.stall_i_dis10(),
            self.stall_i_dis11(),
            self.stall_i_dis12(),
            self.stall_i_dis13(),
            self.stall_i_dis14(),
            self.stall_i_dis15()
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallIlDis(pub u8);
impl StallIlDis {
    #[doc = "Disable Endpoint 0 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis0(&self) -> super::vals::StallIDis0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StallIDis0::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 0 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis0(&mut self, val: super::vals::StallIDis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Disable Endpoint 1 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis1(&self) -> super::vals::StallIDis1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StallIDis1::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 1 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis1(&mut self, val: super::vals::StallIDis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Disable Endpoint 2 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis2(&self) -> super::vals::StallIDis2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StallIDis2::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 2 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis2(&mut self, val: super::vals::StallIDis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Disable Endpoint 3 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis3(&self) -> super::vals::StallIDis3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StallIDis3::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 3 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis3(&mut self, val: super::vals::StallIDis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Disable Endpoint 4 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis4(&self) -> super::vals::StallIDis4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StallIDis4::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 4 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis4(&mut self, val: super::vals::StallIDis4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Disable Endpoint 5 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis5(&self) -> super::vals::StallIDis5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StallIDis5::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 5 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis5(&mut self, val: super::vals::StallIDis5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Disable Endpoint 6 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis6(&self) -> super::vals::StallIDis6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StallIDis6::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 6 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis6(&mut self, val: super::vals::StallIDis6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Disable Endpoint 7 IN Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_i_dis7(&self) -> super::vals::StallIDis7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StallIDis7::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 7 IN Direction"]
    #[inline(always)]
    pub const fn set_stall_i_dis7(&mut self, val: super::vals::StallIDis7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for StallIlDis {
    #[inline(always)]
    fn default() -> StallIlDis {
        StallIlDis(0)
    }
}
impl core::fmt::Debug for StallIlDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallIlDis")
            .field("stall_i_dis0", &self.stall_i_dis0())
            .field("stall_i_dis1", &self.stall_i_dis1())
            .field("stall_i_dis2", &self.stall_i_dis2())
            .field("stall_i_dis3", &self.stall_i_dis3())
            .field("stall_i_dis4", &self.stall_i_dis4())
            .field("stall_i_dis5", &self.stall_i_dis5())
            .field("stall_i_dis6", &self.stall_i_dis6())
            .field("stall_i_dis7", &self.stall_i_dis7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallIlDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallIlDis {{ stall_i_dis0: {:?}, stall_i_dis1: {:?}, stall_i_dis2: {:?}, stall_i_dis3: {:?}, stall_i_dis4: {:?}, stall_i_dis5: {:?}, stall_i_dis6: {:?}, stall_i_dis7: {:?} }}",
            self.stall_i_dis0(),
            self.stall_i_dis1(),
            self.stall_i_dis2(),
            self.stall_i_dis3(),
            self.stall_i_dis4(),
            self.stall_i_dis5(),
            self.stall_i_dis6(),
            self.stall_i_dis7()
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallOhDis(pub u8);
impl StallOhDis {
    #[doc = "Disable Endpoint 8 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis8(&self) -> super::vals::StallODis8 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StallODis8::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 8 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis8(&mut self, val: super::vals::StallODis8) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Disable Endpoint 9 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis9(&self) -> super::vals::StallODis9 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StallODis9::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 9 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis9(&mut self, val: super::vals::StallODis9) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Disable Endpoint 10 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis10(&self) -> super::vals::StallODis10 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StallODis10::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 10 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis10(&mut self, val: super::vals::StallODis10) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Disable Endpoint 11 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis11(&self) -> super::vals::StallODis11 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StallODis11::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 11 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis11(&mut self, val: super::vals::StallODis11) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Disable endpoint 12 OUT direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis12(&self) -> super::vals::StallODis12 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StallODis12::from_bits(val as u8)
    }
    #[doc = "Disable endpoint 12 OUT direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis12(&mut self, val: super::vals::StallODis12) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Disable Endpoint 13 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis13(&self) -> super::vals::StallODis13 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StallODis13::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 13 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis13(&mut self, val: super::vals::StallODis13) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Disable Endpoint 14 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis14(&self) -> super::vals::StallODis14 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StallODis14::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 14 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis14(&mut self, val: super::vals::StallODis14) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Disable Endpoint 15 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis15(&self) -> super::vals::StallODis15 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StallODis15::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 15 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis15(&mut self, val: super::vals::StallODis15) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for StallOhDis {
    #[inline(always)]
    fn default() -> StallOhDis {
        StallOhDis(0)
    }
}
impl core::fmt::Debug for StallOhDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallOhDis")
            .field("stall_o_dis8", &self.stall_o_dis8())
            .field("stall_o_dis9", &self.stall_o_dis9())
            .field("stall_o_dis10", &self.stall_o_dis10())
            .field("stall_o_dis11", &self.stall_o_dis11())
            .field("stall_o_dis12", &self.stall_o_dis12())
            .field("stall_o_dis13", &self.stall_o_dis13())
            .field("stall_o_dis14", &self.stall_o_dis14())
            .field("stall_o_dis15", &self.stall_o_dis15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallOhDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallOhDis {{ stall_o_dis8: {:?}, stall_o_dis9: {:?}, stall_o_dis10: {:?}, stall_o_dis11: {:?}, stall_o_dis12: {:?}, stall_o_dis13: {:?}, stall_o_dis14: {:?}, stall_o_dis15: {:?} }}",
            self.stall_o_dis8(),
            self.stall_o_dis9(),
            self.stall_o_dis10(),
            self.stall_o_dis11(),
            self.stall_o_dis12(),
            self.stall_o_dis13(),
            self.stall_o_dis14(),
            self.stall_o_dis15()
        )
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StallOlDis(pub u8);
impl StallOlDis {
    #[doc = "Disable Endpoint 0 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis0(&self) -> super::vals::StallODis0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::StallODis0::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 0 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis0(&mut self, val: super::vals::StallODis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Disable Endpoint 1 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis1(&self) -> super::vals::StallODis1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::StallODis1::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 1 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis1(&mut self, val: super::vals::StallODis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Disable Endpoint 2 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis2(&self) -> super::vals::StallODis2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::StallODis2::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 2 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis2(&mut self, val: super::vals::StallODis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Disable Endpoint 3 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis3(&self) -> super::vals::StallODis3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::StallODis3::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 3 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis3(&mut self, val: super::vals::StallODis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Disable Endpoint 4 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis4(&self) -> super::vals::StallODis4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::StallODis4::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 4 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis4(&mut self, val: super::vals::StallODis4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Disable Endpoint 5 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis5(&self) -> super::vals::StallODis5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::StallODis5::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 5 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis5(&mut self, val: super::vals::StallODis5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Disable Endpoint 6 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis6(&self) -> super::vals::StallODis6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::StallODis6::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 6 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis6(&mut self, val: super::vals::StallODis6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Disable Endpoint 7 OUT Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn stall_o_dis7(&self) -> super::vals::StallODis7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::StallODis7::from_bits(val as u8)
    }
    #[doc = "Disable Endpoint 7 OUT Direction"]
    #[inline(always)]
    pub const fn set_stall_o_dis7(&mut self, val: super::vals::StallODis7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for StallOlDis {
    #[inline(always)]
    fn default() -> StallOlDis {
        StallOlDis(0)
    }
}
impl core::fmt::Debug for StallOlDis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StallOlDis")
            .field("stall_o_dis0", &self.stall_o_dis0())
            .field("stall_o_dis1", &self.stall_o_dis1())
            .field("stall_o_dis2", &self.stall_o_dis2())
            .field("stall_o_dis3", &self.stall_o_dis3())
            .field("stall_o_dis4", &self.stall_o_dis4())
            .field("stall_o_dis5", &self.stall_o_dis5())
            .field("stall_o_dis6", &self.stall_o_dis6())
            .field("stall_o_dis7", &self.stall_o_dis7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StallOlDis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "StallOlDis {{ stall_o_dis0: {:?}, stall_o_dis1: {:?}, stall_o_dis2: {:?}, stall_o_dis3: {:?}, stall_o_dis4: {:?}, stall_o_dis5: {:?}, stall_o_dis6: {:?}, stall_o_dis7: {:?} }}",
            self.stall_o_dis0(),
            self.stall_o_dis1(),
            self.stall_o_dis2(),
            self.stall_o_dis3(),
            self.stall_o_dis4(),
            self.stall_o_dis5(),
            self.stall_o_dis6(),
            self.stall_o_dis7()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u8);
impl Stat {
    #[doc = "Odd Bank"]
    #[must_use]
    #[inline(always)]
    pub const fn odd(&self) -> super::vals::Odd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Odd::from_bits(val as u8)
    }
    #[doc = "Odd Bank"]
    #[inline(always)]
    pub const fn set_odd(&mut self, val: super::vals::Odd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Transmit Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tx::from_bits(val as u8)
    }
    #[doc = "Transmit Indicator"]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "Endpoint address"]
    #[must_use]
    #[inline(always)]
    pub const fn endp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint address"]
    #[inline(always)]
    pub const fn set_endp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
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
            .field("odd", &self.odd())
            .field("tx", &self.tx())
            .field("endp", &self.endp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ odd: {:?}, tx: {:?}, endp: {=u8:?} }}",
            self.odd(),
            self.tx(),
            self.endp()
        )
    }
}
#[doc = "Token"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Token(pub u8);
impl Token {
    #[doc = "Token Endpoint Address"]
    #[must_use]
    #[inline(always)]
    pub const fn tokenendpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Token Endpoint Address"]
    #[inline(always)]
    pub const fn set_tokenendpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Token Type"]
    #[must_use]
    #[inline(always)]
    pub const fn tokenpid(&self) -> super::vals::Tokenpid {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Tokenpid::from_bits(val as u8)
    }
    #[doc = "Token Type"]
    #[inline(always)]
    pub const fn set_tokenpid(&mut self, val: super::vals::Tokenpid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u8) & 0x0f) << 4usize);
    }
}
impl Default for Token {
    #[inline(always)]
    fn default() -> Token {
        Token(0)
    }
}
impl core::fmt::Debug for Token {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Token")
            .field("tokenendpt", &self.tokenendpt())
            .field("tokenpid", &self.tokenpid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Token {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Token {{ tokenendpt: {=u8:?}, tokenpid: {:?} }}",
            self.tokenendpt(),
            self.tokenpid()
        )
    }
}
#[doc = "USB Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbctrl(pub u8);
impl Usbctrl {
    #[doc = "DP and DM Lane Reversal Control"]
    #[must_use]
    #[inline(always)]
    pub const fn dpdm_lane_reverse(&self) -> super::vals::DpdmLaneReverse {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DpdmLaneReverse::from_bits(val as u8)
    }
    #[doc = "DP and DM Lane Reversal Control"]
    #[inline(always)]
    pub const fn set_dpdm_lane_reverse(&mut self, val: super::vals::DpdmLaneReverse) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u8) & 0x01) << 2usize);
    }
    #[doc = "Host-Mode-Only Low-Speed Device EOP Signaling"]
    #[must_use]
    #[inline(always)]
    pub const fn host_ls_eop(&self) -> super::vals::HostLsEop {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::HostLsEop::from_bits(val as u8)
    }
    #[doc = "Host-Mode-Only Low-Speed Device EOP Signaling"]
    #[inline(always)]
    pub const fn set_host_ls_eop(&mut self, val: super::vals::HostLsEop) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "UART Select"]
    #[must_use]
    #[inline(always)]
    pub const fn uartsel(&self) -> super::vals::Uartsel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Uartsel::from_bits(val as u8)
    }
    #[doc = "UART Select"]
    #[inline(always)]
    pub const fn set_uartsel(&mut self, val: super::vals::Uartsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "UART Signal Channel Select"]
    #[must_use]
    #[inline(always)]
    pub const fn uartchls(&self) -> super::vals::Uartchls {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Uartchls::from_bits(val as u8)
    }
    #[doc = "UART Signal Channel Select"]
    #[inline(always)]
    pub const fn set_uartchls(&mut self, val: super::vals::Uartchls) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "Pulldown Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pde(&self) -> super::vals::Pde {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pde::from_bits(val as u8)
    }
    #[doc = "Pulldown Enable"]
    #[inline(always)]
    pub const fn set_pde(&mut self, val: super::vals::Pde) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> super::vals::Susp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Susp::from_bits(val as u8)
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: super::vals::Susp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Usbctrl {
    #[inline(always)]
    fn default() -> Usbctrl {
        Usbctrl(0)
    }
}
impl core::fmt::Debug for Usbctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbctrl")
            .field("dpdm_lane_reverse", &self.dpdm_lane_reverse())
            .field("host_ls_eop", &self.host_ls_eop())
            .field("uartsel", &self.uartsel())
            .field("uartchls", &self.uartchls())
            .field("pde", &self.pde())
            .field("susp", &self.susp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbctrl {{ dpdm_lane_reverse: {:?}, host_ls_eop: {:?}, uartsel: {:?}, uartchls: {:?}, pde: {:?}, susp: {:?} }}",
            self.dpdm_lane_reverse(),
            self.host_ls_eop(),
            self.uartsel(),
            self.uartchls(),
            self.pde(),
            self.susp()
        )
    }
}
#[doc = "Frame Adjust"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfrmadjust(pub u8);
impl Usbfrmadjust {
    #[doc = "Frame Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn adj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Frame Adjustment"]
    #[inline(always)]
    pub const fn set_adj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Usbfrmadjust {
    #[inline(always)]
    fn default() -> Usbfrmadjust {
        Usbfrmadjust(0)
    }
}
impl core::fmt::Debug for Usbfrmadjust {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbfrmadjust")
            .field("adj", &self.adj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbfrmadjust {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usbfrmadjust {{ adj: {=u8:?} }}", self.adj())
    }
}
#[doc = "USB Transceiver Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbtrc0(pub u8);
impl Usbtrc0 {
    #[doc = "USB Asynchronous Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_resume_int(&self) -> super::vals::UsbResumeInt {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbResumeInt::from_bits(val as u8)
    }
    #[doc = "USB Asynchronous Interrupt"]
    #[inline(always)]
    pub const fn set_usb_resume_int(&mut self, val: super::vals::UsbResumeInt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
    #[doc = "Synchronous USB Interrupt Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_det(&self) -> super::vals::SyncDet {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SyncDet::from_bits(val as u8)
    }
    #[doc = "Synchronous USB Interrupt Detect"]
    #[inline(always)]
    pub const fn set_sync_det(&mut self, val: super::vals::SyncDet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u8) & 0x01) << 1usize);
    }
    #[doc = "Combined USB Clock Recovery interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn usb_clk_recovery_int(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Combined USB Clock Recovery interrupt status"]
    #[inline(always)]
    pub const fn set_usb_clk_recovery_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
    }
    #[doc = "VREGIN Rising Edge Interrupt Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn vredg_det(&self) -> super::vals::VredgDet {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::VredgDet::from_bits(val as u8)
    }
    #[doc = "VREGIN Rising Edge Interrupt Detect"]
    #[inline(always)]
    pub const fn set_vredg_det(&mut self, val: super::vals::VredgDet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u8) & 0x01) << 3usize);
    }
    #[doc = "VREGIN Falling Edge Interrupt Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn vfedg_det(&self) -> super::vals::VfedgDet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::VfedgDet::from_bits(val as u8)
    }
    #[doc = "VREGIN Falling Edge Interrupt Detect"]
    #[inline(always)]
    pub const fn set_vfedg_det(&mut self, val: super::vals::VfedgDet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u8) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Resume Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbresmen(&self) -> super::vals::Usbresmen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Usbresmen::from_bits(val as u8)
    }
    #[doc = "Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub const fn set_usbresmen(&mut self, val: super::vals::Usbresmen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u8) & 0x01) << 5usize);
    }
    #[doc = "VREGIN Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vregin_sts(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "VREGIN Status"]
    #[inline(always)]
    pub const fn set_vregin_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
    }
    #[doc = "USB Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn usbreset(&self) -> super::vals::Usbreset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Usbreset::from_bits(val as u8)
    }
    #[doc = "USB Reset"]
    #[inline(always)]
    pub const fn set_usbreset(&mut self, val: super::vals::Usbreset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Usbtrc0 {
    #[inline(always)]
    fn default() -> Usbtrc0 {
        Usbtrc0(0)
    }
}
impl core::fmt::Debug for Usbtrc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbtrc0")
            .field("usb_resume_int", &self.usb_resume_int())
            .field("sync_det", &self.sync_det())
            .field("usb_clk_recovery_int", &self.usb_clk_recovery_int())
            .field("vredg_det", &self.vredg_det())
            .field("vfedg_det", &self.vfedg_det())
            .field("usbresmen", &self.usbresmen())
            .field("vregin_sts", &self.vregin_sts())
            .field("usbreset", &self.usbreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbtrc0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbtrc0 {{ usb_resume_int: {:?}, sync_det: {:?}, usb_clk_recovery_int: {=bool:?}, vredg_det: {:?}, vfedg_det: {:?}, usbresmen: {:?}, vregin_sts: {=bool:?}, usbreset: {:?} }}",
            self.usb_resume_int(),
            self.sync_det(),
            self.usb_clk_recovery_int(),
            self.vredg_det(),
            self.vfedg_det(),
            self.usbresmen(),
            self.vregin_sts(),
            self.usbreset()
        )
    }
}
