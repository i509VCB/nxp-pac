#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcbulkcurrented(pub u32);
impl Hcbulkcurrented {
    #[doc = "BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[must_use]
    #[inline(always)]
    pub const fn bced(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub const fn set_bced(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hcbulkcurrented {
    #[inline(always)]
    fn default() -> Hcbulkcurrented {
        Hcbulkcurrented(0)
    }
}
impl core::fmt::Debug for Hcbulkcurrented {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcbulkcurrented")
            .field("bced", &self.bced())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcbulkcurrented {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcbulkcurrented {{ bced: {=u32:?} }}", self.bced())
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcbulkheaded(pub u32);
impl Hcbulkheaded {
    #[doc = "BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn bhed(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub const fn set_bhed(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hcbulkheaded {
    #[inline(always)]
    fn default() -> Hcbulkheaded {
        Hcbulkheaded(0)
    }
}
impl core::fmt::Debug for Hcbulkheaded {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcbulkheaded")
            .field("bhed", &self.bhed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcbulkheaded {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcbulkheaded {{ bhed: {=u32:?} }}", self.bhed())
    }
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccommandstatus(pub u32);
impl Hccommandstatus {
    #[doc = "HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[must_use]
    #[inline(always)]
    pub const fn hcr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub const fn set_hcr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[must_use]
    #[inline(always)]
    pub const fn clf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub const fn set_clf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[must_use]
    #[inline(always)]
    pub const fn blf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub const fn set_blf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[must_use]
    #[inline(always)]
    pub const fn ocr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub const fn set_ocr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn soc(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub const fn set_soc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for Hccommandstatus {
    #[inline(always)]
    fn default() -> Hccommandstatus {
        Hccommandstatus(0)
    }
}
impl core::fmt::Debug for Hccommandstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hccommandstatus")
            .field("hcr", &self.hcr())
            .field("clf", &self.clf())
            .field("blf", &self.blf())
            .field("ocr", &self.ocr())
            .field("soc", &self.soc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hccommandstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hccommandstatus {{ hcr: {=bool:?}, clf: {=bool:?}, blf: {=bool:?}, ocr: {=bool:?}, soc: {=u8:?} }}",
            self.hcr(),
            self.clf(),
            self.blf(),
            self.ocr(),
            self.soc()
        )
    }
}
#[doc = "Defines the operating modes of the HC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccontrol(pub u32);
impl Hccontrol {
    #[doc = "ControlBulkServiceRatio."]
    #[must_use]
    #[inline(always)]
    pub const fn cbsr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ControlBulkServiceRatio."]
    #[inline(always)]
    pub const fn set_cbsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PeriodicListEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn ple(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PeriodicListEnable."]
    #[inline(always)]
    pub const fn set_ple(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IsochronousEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IsochronousEnable."]
    #[inline(always)]
    pub const fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ControlListEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn cle(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ControlListEnable."]
    #[inline(always)]
    pub const fn set_cle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ble(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub const fn set_ble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[must_use]
    #[inline(always)]
    pub const fn hcfs(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub const fn set_hcfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[must_use]
    #[inline(always)]
    pub const fn ir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub const fn set_ir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn rwc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub const fn set_rwc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn rwe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub const fn set_rwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Hccontrol {
    #[inline(always)]
    fn default() -> Hccontrol {
        Hccontrol(0)
    }
}
impl core::fmt::Debug for Hccontrol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hccontrol")
            .field("cbsr", &self.cbsr())
            .field("ple", &self.ple())
            .field("ie", &self.ie())
            .field("cle", &self.cle())
            .field("ble", &self.ble())
            .field("hcfs", &self.hcfs())
            .field("ir", &self.ir())
            .field("rwc", &self.rwc())
            .field("rwe", &self.rwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hccontrol {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hccontrol {{ cbsr: {=u8:?}, ple: {=bool:?}, ie: {=bool:?}, cle: {=bool:?}, ble: {=bool:?}, hcfs: {=u8:?}, ir: {=bool:?}, rwc: {=bool:?}, rwe: {=bool:?} }}",
            self.cbsr(),
            self.ple(),
            self.ie(),
            self.cle(),
            self.ble(),
            self.hcfs(),
            self.ir(),
            self.rwc(),
            self.rwe()
        )
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccontrolcurrented(pub u32);
impl Hccontrolcurrented {
    #[doc = "ControlCurrentED."]
    #[must_use]
    #[inline(always)]
    pub const fn cced(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "ControlCurrentED."]
    #[inline(always)]
    pub const fn set_cced(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hccontrolcurrented {
    #[inline(always)]
    fn default() -> Hccontrolcurrented {
        Hccontrolcurrented(0)
    }
}
impl core::fmt::Debug for Hccontrolcurrented {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hccontrolcurrented")
            .field("cced", &self.cced())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hccontrolcurrented {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hccontrolcurrented {{ cced: {=u32:?} }}", self.cced())
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccontrolheaded(pub u32);
impl Hccontrolheaded {
    #[doc = "HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn ched(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub const fn set_ched(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hccontrolheaded {
    #[inline(always)]
    fn default() -> Hccontrolheaded {
        Hccontrolheaded(0)
    }
}
impl core::fmt::Debug for Hccontrolheaded {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hccontrolheaded")
            .field("ched", &self.ched())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hccontrolheaded {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hccontrolheaded {{ ched: {=u32:?} }}", self.ched())
    }
}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcdonehead(pub u32);
impl Hcdonehead {
    #[doc = "DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[must_use]
    #[inline(always)]
    pub const fn dh(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub const fn set_dh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hcdonehead {
    #[inline(always)]
    fn default() -> Hcdonehead {
        Hcdonehead(0)
    }
}
impl core::fmt::Debug for Hcdonehead {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcdonehead")
            .field("dh", &self.dh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcdonehead {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcdonehead {{ dh: {=u32:?} }}", self.dh())
    }
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfminterval(pub u32);
impl Hcfminterval {
    #[doc = "FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[must_use]
    #[inline(always)]
    pub const fn fi(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub const fn set_fi(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[must_use]
    #[inline(always)]
    pub const fn fsmps(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub const fn set_fsmps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
    #[doc = "FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[must_use]
    #[inline(always)]
    pub const fn fit(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub const fn set_fit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcfminterval {
    #[inline(always)]
    fn default() -> Hcfminterval {
        Hcfminterval(0)
    }
}
impl core::fmt::Debug for Hcfminterval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcfminterval")
            .field("fi", &self.fi())
            .field("fsmps", &self.fsmps())
            .field("fit", &self.fit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcfminterval {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcfminterval {{ fi: {=u16:?}, fsmps: {=u16:?}, fit: {=bool:?} }}",
            self.fi(),
            self.fsmps(),
            self.fit()
        )
    }
}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfmnumber(pub u32);
impl Hcfmnumber {
    #[doc = "FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Hcfmnumber {
    #[inline(always)]
    fn default() -> Hcfmnumber {
        Hcfmnumber(0)
    }
}
impl core::fmt::Debug for Hcfmnumber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcfmnumber")
            .field("fn_", &self.fn_())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcfmnumber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcfmnumber {{ fn_: {=u16:?} }}", self.fn_())
    }
}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcfmremaining(pub u32);
impl Hcfmremaining {
    #[doc = "FrameRemaining This counter is decremented at each bit time."]
    #[must_use]
    #[inline(always)]
    pub const fn fr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn frt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub const fn set_frt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcfmremaining {
    #[inline(always)]
    fn default() -> Hcfmremaining {
        Hcfmremaining(0)
    }
}
impl core::fmt::Debug for Hcfmremaining {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcfmremaining")
            .field("fr", &self.fr())
            .field("frt", &self.frt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcfmremaining {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcfmremaining {{ fr: {=u16:?}, frt: {=bool:?} }}",
            self.fr(),
            self.frt()
        )
    }
}
#[doc = "Contains the physical address of the host controller communication area"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hchcca(pub u32);
impl Hchcca {
    #[doc = "Base address of the Host Controller Communication Area."]
    #[must_use]
    #[inline(always)]
    pub const fn hcca(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub const fn set_hcca(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Hchcca {
    #[inline(always)]
    fn default() -> Hchcca {
        Hchcca(0)
    }
}
impl core::fmt::Debug for Hchcca {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hchcca")
            .field("hcca", &self.hcca())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hchcca {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hchcca {{ hcca: {=u32:?} }}", self.hcca())
    }
}
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcinterruptdisable(pub u32);
impl Hcinterruptdisable {
    #[doc = "Scheduling Overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn so(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Overrun interrupt."]
    #[inline(always)]
    pub const fn set_so(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start of Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame interrupt."]
    #[inline(always)]
    pub const fn set_sf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Resume Detect interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detect interrupt."]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fno(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fno(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rhsc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[inline(always)]
    pub const fn set_rhsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ownership Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn oc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Ownership Change interrupt."]
    #[inline(always)]
    pub const fn set_oc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "A 0 written to this field is ignored by HC."]
    #[must_use]
    #[inline(always)]
    pub const fn mie(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "A 0 written to this field is ignored by HC."]
    #[inline(always)]
    pub const fn set_mie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcinterruptdisable {
    #[inline(always)]
    fn default() -> Hcinterruptdisable {
        Hcinterruptdisable(0)
    }
}
impl core::fmt::Debug for Hcinterruptdisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcinterruptdisable")
            .field("so", &self.so())
            .field("wdh", &self.wdh())
            .field("sf", &self.sf())
            .field("rd", &self.rd())
            .field("ue", &self.ue())
            .field("fno", &self.fno())
            .field("rhsc", &self.rhsc())
            .field("oc", &self.oc())
            .field("mie", &self.mie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcinterruptdisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcinterruptdisable {{ so: {=bool:?}, wdh: {=bool:?}, sf: {=bool:?}, rd: {=bool:?}, ue: {=bool:?}, fno: {=bool:?}, rhsc: {=bool:?}, oc: {=bool:?}, mie: {=bool:?} }}",
            self.so(),
            self.wdh(),
            self.sf(),
            self.rd(),
            self.ue(),
            self.fno(),
            self.rhsc(),
            self.oc(),
            self.mie()
        )
    }
}
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcinterruptenable(pub u32);
impl Hcinterruptenable {
    #[doc = "Scheduling Overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn so(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Overrun interrupt."]
    #[inline(always)]
    pub const fn set_so(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start of Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn sf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame interrupt."]
    #[inline(always)]
    pub const fn set_sf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Resume Detect interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detect interrupt."]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fno(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fno(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rhsc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[inline(always)]
    pub const fn set_rhsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ownership Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn oc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Ownership Change interrupt."]
    #[inline(always)]
    pub const fn set_oc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mie(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcinterruptenable {
    #[inline(always)]
    fn default() -> Hcinterruptenable {
        Hcinterruptenable(0)
    }
}
impl core::fmt::Debug for Hcinterruptenable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcinterruptenable")
            .field("so", &self.so())
            .field("wdh", &self.wdh())
            .field("sf", &self.sf())
            .field("rd", &self.rd())
            .field("ue", &self.ue())
            .field("fno", &self.fno())
            .field("rhsc", &self.rhsc())
            .field("oc", &self.oc())
            .field("mie", &self.mie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcinterruptenable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcinterruptenable {{ so: {=bool:?}, wdh: {=bool:?}, sf: {=bool:?}, rd: {=bool:?}, ue: {=bool:?}, fno: {=bool:?}, rhsc: {=bool:?}, oc: {=bool:?}, mie: {=bool:?} }}",
            self.so(),
            self.wdh(),
            self.sf(),
            self.rd(),
            self.ue(),
            self.fno(),
            self.rhsc(),
            self.oc(),
            self.mie()
        )
    }
}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcinterruptstatus(pub u32);
impl Hcinterruptstatus {
    #[doc = "SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[must_use]
    #[inline(always)]
    pub const fn so(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub const fn set_so(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[must_use]
    #[inline(always)]
    pub const fn sf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub const fn set_sf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[must_use]
    #[inline(always)]
    pub const fn fno(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub const fn set_fno(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn rhsc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub const fn set_rhsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[must_use]
    #[inline(always)]
    pub const fn oc(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub const fn set_oc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for Hcinterruptstatus {
    #[inline(always)]
    fn default() -> Hcinterruptstatus {
        Hcinterruptstatus(0)
    }
}
impl core::fmt::Debug for Hcinterruptstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcinterruptstatus")
            .field("so", &self.so())
            .field("wdh", &self.wdh())
            .field("sf", &self.sf())
            .field("rd", &self.rd())
            .field("ue", &self.ue())
            .field("fno", &self.fno())
            .field("rhsc", &self.rhsc())
            .field("oc", &self.oc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcinterruptstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcinterruptstatus {{ so: {=bool:?}, wdh: {=bool:?}, sf: {=bool:?}, rd: {=bool:?}, ue: {=bool:?}, fno: {=bool:?}, rhsc: {=bool:?}, oc: {=u32:?} }}",
            self.so(),
            self.wdh(),
            self.sf(),
            self.rd(),
            self.ue(),
            self.fno(),
            self.rhsc(),
            self.oc()
        )
    }
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hclsthreshold(pub u32);
impl Hclsthreshold {
    #[doc = "LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[must_use]
    #[inline(always)]
    pub const fn lst(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub const fn set_lst(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Hclsthreshold {
    #[inline(always)]
    fn default() -> Hclsthreshold {
        Hclsthreshold(0)
    }
}
impl core::fmt::Debug for Hclsthreshold {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hclsthreshold")
            .field("lst", &self.lst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hclsthreshold {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hclsthreshold {{ lst: {=u16:?} }}", self.lst())
    }
}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcperiodcurrented(pub u32);
impl Hcperiodcurrented {
    #[doc = "The content of this register is updated by HC after a periodic ED is processed."]
    #[must_use]
    #[inline(always)]
    pub const fn pced(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub const fn set_pced(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Hcperiodcurrented {
    #[inline(always)]
    fn default() -> Hcperiodcurrented {
        Hcperiodcurrented(0)
    }
}
impl core::fmt::Debug for Hcperiodcurrented {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcperiodcurrented")
            .field("pced", &self.pced())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcperiodcurrented {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcperiodcurrented {{ pced: {=u32:?} }}", self.pced())
    }
}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcperiodicstart(pub u32);
impl Hcperiodicstart {
    #[doc = "PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Hcperiodicstart {
    #[inline(always)]
    fn default() -> Hcperiodicstart {
        Hcperiodicstart(0)
    }
}
impl core::fmt::Debug for Hcperiodicstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcperiodicstart")
            .field("ps", &self.ps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcperiodicstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcperiodicstart {{ ps: {=u16:?} }}", self.ps())
    }
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcrevision(pub u32);
impl Hcrevision {
    #[doc = "Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rev(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision."]
    #[inline(always)]
    pub const fn set_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Hcrevision {
    #[inline(always)]
    fn default() -> Hcrevision {
        Hcrevision(0)
    }
}
impl core::fmt::Debug for Hcrevision {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcrevision")
            .field("rev", &self.rev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcrevision {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Hcrevision {{ rev: {=u8:?} }}", self.rev())
    }
}
#[doc = "First of the two registers which describes the characteristics of the root hub"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcrhdescriptora(pub u32);
impl Hcrhdescriptora {
    #[doc = "NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[must_use]
    #[inline(always)]
    pub const fn ndp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub const fn set_ndp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[must_use]
    #[inline(always)]
    pub const fn psm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub const fn set_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[must_use]
    #[inline(always)]
    pub const fn nps(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub const fn set_nps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DeviceType This bit specifies that the root hub is not a compound device."]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[must_use]
    #[inline(always)]
    pub const fn ocpm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub const fn set_ocpm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[must_use]
    #[inline(always)]
    pub const fn nocp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub const fn set_nocp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[must_use]
    #[inline(always)]
    pub const fn potpgt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub const fn set_potpgt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hcrhdescriptora {
    #[inline(always)]
    fn default() -> Hcrhdescriptora {
        Hcrhdescriptora(0)
    }
}
impl core::fmt::Debug for Hcrhdescriptora {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcrhdescriptora")
            .field("ndp", &self.ndp())
            .field("psm", &self.psm())
            .field("nps", &self.nps())
            .field("dt", &self.dt())
            .field("ocpm", &self.ocpm())
            .field("nocp", &self.nocp())
            .field("potpgt", &self.potpgt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcrhdescriptora {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcrhdescriptora {{ ndp: {=u8:?}, psm: {=bool:?}, nps: {=bool:?}, dt: {=bool:?}, ocpm: {=bool:?}, nocp: {=bool:?}, potpgt: {=u8:?} }}",
            self.ndp(),
            self.psm(),
            self.nps(),
            self.dt(),
            self.ocpm(),
            self.nocp(),
            self.potpgt()
        )
    }
}
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcrhdescriptorb(pub u32);
impl Hcrhdescriptorb {
    #[doc = "DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ppcm(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub const fn set_ppcm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hcrhdescriptorb {
    #[inline(always)]
    fn default() -> Hcrhdescriptorb {
        Hcrhdescriptorb(0)
    }
}
impl core::fmt::Debug for Hcrhdescriptorb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcrhdescriptorb")
            .field("dr", &self.dr())
            .field("ppcm", &self.ppcm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcrhdescriptorb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcrhdescriptorb {{ dr: {=u16:?}, ppcm: {=u16:?} }}",
            self.dr(),
            self.ppcm()
        )
    }
}
#[doc = "Controls and reports the port events on a per-port basis"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcrhportstatus(pub u32);
impl Hcrhportstatus {
    #[doc = "(read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "(read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub const fn set_ccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "(read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn pes(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub const fn set_pes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "(read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn pss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub const fn set_pss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "(read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[must_use]
    #[inline(always)]
    pub const fn poci(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub const fn set_poci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "(read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn prs(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub const fn set_prs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "(read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn pps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub const fn set_pps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "(read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[must_use]
    #[inline(always)]
    pub const fn lsda(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub const fn set_lsda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn csc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub const fn set_csc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn pesc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub const fn set_pesc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn pssc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub const fn set_pssc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[must_use]
    #[inline(always)]
    pub const fn ocic(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub const fn set_ocic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[must_use]
    #[inline(always)]
    pub const fn prsc(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub const fn set_prsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Hcrhportstatus {
    #[inline(always)]
    fn default() -> Hcrhportstatus {
        Hcrhportstatus(0)
    }
}
impl core::fmt::Debug for Hcrhportstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcrhportstatus")
            .field("ccs", &self.ccs())
            .field("pes", &self.pes())
            .field("pss", &self.pss())
            .field("poci", &self.poci())
            .field("prs", &self.prs())
            .field("pps", &self.pps())
            .field("lsda", &self.lsda())
            .field("csc", &self.csc())
            .field("pesc", &self.pesc())
            .field("pssc", &self.pssc())
            .field("ocic", &self.ocic())
            .field("prsc", &self.prsc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcrhportstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcrhportstatus {{ ccs: {=bool:?}, pes: {=bool:?}, pss: {=bool:?}, poci: {=bool:?}, prs: {=bool:?}, pps: {=bool:?}, lsda: {=bool:?}, csc: {=bool:?}, pesc: {=bool:?}, pssc: {=bool:?}, ocic: {=bool:?}, prsc: {=bool:?} }}",
            self.ccs(),
            self.pes(),
            self.pss(),
            self.poci(),
            self.prs(),
            self.pps(),
            self.lsda(),
            self.csc(),
            self.pesc(),
            self.pssc(),
            self.ocic(),
            self.prsc()
        )
    }
}
#[doc = "This register is divided into two parts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcrhstatus(pub u32);
impl Hcrhstatus {
    #[doc = "(read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn lps(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub const fn set_lps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn oci(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub const fn set_oci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "(read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn drwe(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "(read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub const fn set_drwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "(read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[must_use]
    #[inline(always)]
    pub const fn lpsc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub const fn set_lpsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[must_use]
    #[inline(always)]
    pub const fn ocic(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub const fn set_ocic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "(write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn crwe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "(write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub const fn set_crwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Hcrhstatus {
    #[inline(always)]
    fn default() -> Hcrhstatus {
        Hcrhstatus(0)
    }
}
impl core::fmt::Debug for Hcrhstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcrhstatus")
            .field("lps", &self.lps())
            .field("oci", &self.oci())
            .field("drwe", &self.drwe())
            .field("lpsc", &self.lpsc())
            .field("ocic", &self.ocic())
            .field("crwe", &self.crwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcrhstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcrhstatus {{ lps: {=bool:?}, oci: {=bool:?}, drwe: {=bool:?}, lpsc: {=bool:?}, ocic: {=bool:?}, crwe: {=bool:?} }}",
            self.lps(),
            self.oci(),
            self.drwe(),
            self.lpsc(),
            self.ocic(),
            self.crwe()
        )
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portmode(pub u32);
impl Portmode {
    #[doc = "Port ID pin value."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port ID pin value."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port ID pin pull-up enable."]
    #[must_use]
    #[inline(always)]
    pub const fn id_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port ID pin pull-up enable."]
    #[inline(always)]
    pub const fn set_id_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1: device 0: host."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_enable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1: device 0: host."]
    #[inline(always)]
    pub const fn set_dev_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Portmode {
    #[inline(always)]
    fn default() -> Portmode {
        Portmode(0)
    }
}
impl core::fmt::Debug for Portmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Portmode")
            .field("id", &self.id())
            .field("id_en", &self.id_en())
            .field("dev_enable", &self.dev_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Portmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Portmode {{ id: {=bool:?}, id_en: {=bool:?}, dev_enable: {=bool:?} }}",
            self.id(),
            self.id_en(),
            self.dev_enable()
        )
    }
}
