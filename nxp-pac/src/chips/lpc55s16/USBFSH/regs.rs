#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCBULKCURRENTED(pub u32);
impl HCBULKCURRENTED {
    #[doc = "BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[must_use]
    #[inline(always)]
    pub const fn BCED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub const fn set_BCED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCBULKCURRENTED {
    #[inline(always)]
    fn default() -> HCBULKCURRENTED {
        HCBULKCURRENTED(0)
    }
}
impl core::fmt::Debug for HCBULKCURRENTED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCBULKCURRENTED")
            .field("BCED", &self.BCED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCBULKCURRENTED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCBULKCURRENTED {{ BCED: {=u32:?} }}", self.BCED())
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCBULKHEADED(pub u32);
impl HCBULKHEADED {
    #[doc = "BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn BHED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub const fn set_BHED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCBULKHEADED {
    #[inline(always)]
    fn default() -> HCBULKHEADED {
        HCBULKHEADED(0)
    }
}
impl core::fmt::Debug for HCBULKHEADED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCBULKHEADED")
            .field("BHED", &self.BHED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCBULKHEADED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCBULKHEADED {{ BHED: {=u32:?} }}", self.BHED())
    }
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCOMMANDSTATUS(pub u32);
impl HCCOMMANDSTATUS {
    #[doc = "HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[must_use]
    #[inline(always)]
    pub const fn HCR(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub const fn set_HCR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[must_use]
    #[inline(always)]
    pub const fn CLF(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub const fn set_CLF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[must_use]
    #[inline(always)]
    pub const fn BLF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub const fn set_BLF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[must_use]
    #[inline(always)]
    pub const fn OCR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub const fn set_OCR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[must_use]
    #[inline(always)]
    pub const fn SOC(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub const fn set_SOC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
}
impl Default for HCCOMMANDSTATUS {
    #[inline(always)]
    fn default() -> HCCOMMANDSTATUS {
        HCCOMMANDSTATUS(0)
    }
}
impl core::fmt::Debug for HCCOMMANDSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCOMMANDSTATUS")
            .field("HCR", &self.HCR())
            .field("CLF", &self.CLF())
            .field("BLF", &self.BLF())
            .field("OCR", &self.OCR())
            .field("SOC", &self.SOC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCOMMANDSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCCOMMANDSTATUS {{ HCR: {=bool:?}, CLF: {=bool:?}, BLF: {=bool:?}, OCR: {=bool:?}, SOC: {=u8:?} }}",
            self.HCR(),
            self.CLF(),
            self.BLF(),
            self.OCR(),
            self.SOC()
        )
    }
}
#[doc = "Defines the operating modes of the HC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCONTROL(pub u32);
impl HCCONTROL {
    #[doc = "ControlBulkServiceRatio."]
    #[must_use]
    #[inline(always)]
    pub const fn CBSR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ControlBulkServiceRatio."]
    #[inline(always)]
    pub const fn set_CBSR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PeriodicListEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PeriodicListEnable."]
    #[inline(always)]
    pub const fn set_PLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IsochronousEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn IE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IsochronousEnable."]
    #[inline(always)]
    pub const fn set_IE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "ControlListEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn CLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "ControlListEnable."]
    #[inline(always)]
    pub const fn set_CLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[must_use]
    #[inline(always)]
    pub const fn BLE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "BulkListEnable This bit is set to enable the processing of the Bulk list in the next Frame."]
    #[inline(always)]
    pub const fn set_BLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[must_use]
    #[inline(always)]
    pub const fn HCFS(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "HostControllerFunctionalState for USB 00b: USBRESET 01b: USBRESUME 10b: USBOPERATIONAL 11b: USBSUSPEND A transition to USBOPERATIONAL from another state causes SOFgeneration to begin 1 ms later."]
    #[inline(always)]
    pub const fn set_HCFS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[must_use]
    #[inline(always)]
    pub const fn IR(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "InterruptRouting This bit determines the routing of interrupts generated by events registered in HcInterruptStatus."]
    #[inline(always)]
    pub const fn set_IR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn RWC(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "RemoteWakeupConnected This bit indicates whether HC supports remote wake-up signaling."]
    #[inline(always)]
    pub const fn set_RWC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn RWE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "RemoteWakeupEnable This bit is used by HCD to enable or disable the remote wake-up feature upon the detection of upstream resume signaling."]
    #[inline(always)]
    pub const fn set_RWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for HCCONTROL {
    #[inline(always)]
    fn default() -> HCCONTROL {
        HCCONTROL(0)
    }
}
impl core::fmt::Debug for HCCONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCONTROL")
            .field("CBSR", &self.CBSR())
            .field("PLE", &self.PLE())
            .field("IE", &self.IE())
            .field("CLE", &self.CLE())
            .field("BLE", &self.BLE())
            .field("HCFS", &self.HCFS())
            .field("IR", &self.IR())
            .field("RWC", &self.RWC())
            .field("RWE", &self.RWE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCCONTROL {{ CBSR: {=u8:?}, PLE: {=bool:?}, IE: {=bool:?}, CLE: {=bool:?}, BLE: {=bool:?}, HCFS: {=u8:?}, IR: {=bool:?}, RWC: {=bool:?}, RWE: {=bool:?} }}",
            self.CBSR(),
            self.PLE(),
            self.IE(),
            self.CLE(),
            self.BLE(),
            self.HCFS(),
            self.IR(),
            self.RWC(),
            self.RWE()
        )
    }
}
#[doc = "Contains the physical address of the current endpoint descriptor of the control list."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCONTROLCURRENTED(pub u32);
impl HCCONTROLCURRENTED {
    #[doc = "ControlCurrentED."]
    #[must_use]
    #[inline(always)]
    pub const fn CCED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "ControlCurrentED."]
    #[inline(always)]
    pub const fn set_CCED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCCONTROLCURRENTED {
    #[inline(always)]
    fn default() -> HCCONTROLCURRENTED {
        HCCONTROLCURRENTED(0)
    }
}
impl core::fmt::Debug for HCCONTROLCURRENTED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCONTROLCURRENTED")
            .field("CCED", &self.CCED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCONTROLCURRENTED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCCONTROLCURRENTED {{ CCED: {=u32:?} }}", self.CCED())
    }
}
#[doc = "Contains the physical address of the first endpoint descriptor of the control list."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCONTROLHEADED(pub u32);
impl HCCONTROLHEADED {
    #[doc = "HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn CHED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub const fn set_CHED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCCONTROLHEADED {
    #[inline(always)]
    fn default() -> HCCONTROLHEADED {
        HCCONTROLHEADED(0)
    }
}
impl core::fmt::Debug for HCCONTROLHEADED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCONTROLHEADED")
            .field("CHED", &self.CHED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCONTROLHEADED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCCONTROLHEADED {{ CHED: {=u32:?} }}", self.CHED())
    }
}
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCDONEHEAD(pub u32);
impl HCDONEHEAD {
    #[doc = "DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[must_use]
    #[inline(always)]
    pub const fn DH(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub const fn set_DH(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCDONEHEAD {
    #[inline(always)]
    fn default() -> HCDONEHEAD {
        HCDONEHEAD(0)
    }
}
impl core::fmt::Debug for HCDONEHEAD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDONEHEAD")
            .field("DH", &self.DH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCDONEHEAD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCDONEHEAD {{ DH: {=u32:?} }}", self.DH())
    }
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCFMINTERVAL(pub u32);
impl HCFMINTERVAL {
    #[doc = "FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[must_use]
    #[inline(always)]
    pub const fn FI(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub const fn set_FI(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[must_use]
    #[inline(always)]
    pub const fn FSMPS(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub const fn set_FSMPS(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
    #[doc = "FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[must_use]
    #[inline(always)]
    pub const fn FIT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub const fn set_FIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HCFMINTERVAL {
    #[inline(always)]
    fn default() -> HCFMINTERVAL {
        HCFMINTERVAL(0)
    }
}
impl core::fmt::Debug for HCFMINTERVAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFMINTERVAL")
            .field("FI", &self.FI())
            .field("FSMPS", &self.FSMPS())
            .field("FIT", &self.FIT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCFMINTERVAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCFMINTERVAL {{ FI: {=u16:?}, FSMPS: {=u16:?}, FIT: {=bool:?} }}",
            self.FI(),
            self.FSMPS(),
            self.FIT()
        )
    }
}
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCFMNUMBER(pub u32);
impl HCFMNUMBER {
    #[doc = "FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[must_use]
    #[inline(always)]
    pub const fn FN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FrameNumber This is incremented when HcFmRemaining is re-loaded."]
    #[inline(always)]
    pub const fn set_FN(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for HCFMNUMBER {
    #[inline(always)]
    fn default() -> HCFMNUMBER {
        HCFMNUMBER(0)
    }
}
impl core::fmt::Debug for HCFMNUMBER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFMNUMBER")
            .field("FN", &self.FN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCFMNUMBER {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCFMNUMBER {{ FN: {=u16:?} }}", self.FN())
    }
}
#[doc = "A 14-bit counter showing the bit time remaining in the current frame."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCFMREMAINING(pub u32);
impl HCFMREMAINING {
    #[doc = "FrameRemaining This counter is decremented at each bit time."]
    #[must_use]
    #[inline(always)]
    pub const fn FR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "FrameRemaining This counter is decremented at each bit time."]
    #[inline(always)]
    pub const fn set_FR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[must_use]
    #[inline(always)]
    pub const fn FRT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FrameRemainingToggle This bit is loaded from the FrameIntervalToggle field of HcFmInterval whenever FrameRemaining reaches 0."]
    #[inline(always)]
    pub const fn set_FRT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HCFMREMAINING {
    #[inline(always)]
    fn default() -> HCFMREMAINING {
        HCFMREMAINING(0)
    }
}
impl core::fmt::Debug for HCFMREMAINING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCFMREMAINING")
            .field("FR", &self.FR())
            .field("FRT", &self.FRT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCFMREMAINING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCFMREMAINING {{ FR: {=u16:?}, FRT: {=bool:?} }}",
            self.FR(),
            self.FRT()
        )
    }
}
#[doc = "Contains the physical address of the host controller communication area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCHCCA(pub u32);
impl HCHCCA {
    #[doc = "Base address of the Host Controller Communication Area."]
    #[must_use]
    #[inline(always)]
    pub const fn HCCA(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub const fn set_HCCA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for HCHCCA {
    #[inline(always)]
    fn default() -> HCHCCA {
        HCHCCA(0)
    }
}
impl core::fmt::Debug for HCHCCA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCHCCA")
            .field("HCCA", &self.HCCA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCHCCA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCHCCA {{ HCCA: {=u32:?} }}", self.HCCA())
    }
}
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCINTERRUPTDISABLE(pub u32);
impl HCINTERRUPTDISABLE {
    #[doc = "Scheduling Overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SO(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Overrun interrupt."]
    #[inline(always)]
    pub const fn set_SO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WDH(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub const fn set_WDH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start of Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame interrupt."]
    #[inline(always)]
    pub const fn set_SF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Resume Detect interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detect interrupt."]
    #[inline(always)]
    pub const fn set_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn FNO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[inline(always)]
    pub const fn set_FNO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RHSC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[inline(always)]
    pub const fn set_RHSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ownership Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OC(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Ownership Change interrupt."]
    #[inline(always)]
    pub const fn set_OC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "A 0 written to this field is ignored by HC."]
    #[must_use]
    #[inline(always)]
    pub const fn MIE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "A 0 written to this field is ignored by HC."]
    #[inline(always)]
    pub const fn set_MIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HCINTERRUPTDISABLE {
    #[inline(always)]
    fn default() -> HCINTERRUPTDISABLE {
        HCINTERRUPTDISABLE(0)
    }
}
impl core::fmt::Debug for HCINTERRUPTDISABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTERRUPTDISABLE")
            .field("SO", &self.SO())
            .field("WDH", &self.WDH())
            .field("SF", &self.SF())
            .field("RD", &self.RD())
            .field("UE", &self.UE())
            .field("FNO", &self.FNO())
            .field("RHSC", &self.RHSC())
            .field("OC", &self.OC())
            .field("MIE", &self.MIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCINTERRUPTDISABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCINTERRUPTDISABLE {{ SO: {=bool:?}, WDH: {=bool:?}, SF: {=bool:?}, RD: {=bool:?}, UE: {=bool:?}, FNO: {=bool:?}, RHSC: {=bool:?}, OC: {=bool:?}, MIE: {=bool:?} }}",
            self.SO(),
            self.WDH(),
            self.SF(),
            self.RD(),
            self.UE(),
            self.FNO(),
            self.RHSC(),
            self.OC(),
            self.MIE()
        )
    }
}
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCINTERRUPTENABLE(pub u32);
impl HCINTERRUPTENABLE {
    #[doc = "Scheduling Overrun interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SO(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scheduling Overrun interrupt."]
    #[inline(always)]
    pub const fn set_SO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WDH(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub const fn set_WDH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start of Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Frame interrupt."]
    #[inline(always)]
    pub const fn set_SF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Resume Detect interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detect interrupt."]
    #[inline(always)]
    pub const fn set_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Unrecoverable Error interrupt."]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn FNO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Number Overflow interrupt."]
    #[inline(always)]
    pub const fn set_FNO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RHSC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Root Hub Status Change interrupt."]
    #[inline(always)]
    pub const fn set_RHSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Ownership Change interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OC(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Ownership Change interrupt."]
    #[inline(always)]
    pub const fn set_OC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Master Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MIE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master Interrupt Enable."]
    #[inline(always)]
    pub const fn set_MIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HCINTERRUPTENABLE {
    #[inline(always)]
    fn default() -> HCINTERRUPTENABLE {
        HCINTERRUPTENABLE(0)
    }
}
impl core::fmt::Debug for HCINTERRUPTENABLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTERRUPTENABLE")
            .field("SO", &self.SO())
            .field("WDH", &self.WDH())
            .field("SF", &self.SF())
            .field("RD", &self.RD())
            .field("UE", &self.UE())
            .field("FNO", &self.FNO())
            .field("RHSC", &self.RHSC())
            .field("OC", &self.OC())
            .field("MIE", &self.MIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCINTERRUPTENABLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCINTERRUPTENABLE {{ SO: {=bool:?}, WDH: {=bool:?}, SF: {=bool:?}, RD: {=bool:?}, UE: {=bool:?}, FNO: {=bool:?}, RHSC: {=bool:?}, OC: {=bool:?}, MIE: {=bool:?} }}",
            self.SO(),
            self.WDH(),
            self.SF(),
            self.RD(),
            self.UE(),
            self.FNO(),
            self.RHSC(),
            self.OC(),
            self.MIE()
        )
    }
}
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCINTERRUPTSTATUS(pub u32);
impl HCINTERRUPTSTATUS {
    #[doc = "SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[must_use]
    #[inline(always)]
    pub const fn SO(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SchedulingOverrun This bit is set when the USB schedule for the current Frame overruns and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub const fn set_SO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[must_use]
    #[inline(always)]
    pub const fn WDH(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "WritebackDoneHead This bit is set immediately after HC has written HcDoneHead to HccaDoneHead."]
    #[inline(always)]
    pub const fn set_WDH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[must_use]
    #[inline(always)]
    pub const fn SF(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "StartofFrame This bit is set by HC at each start of a frame and after the update of HccaFrameNumber."]
    #[inline(always)]
    pub const fn set_SF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[must_use]
    #[inline(always)]
    pub const fn RD(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "ResumeDetected This bit is set when HC detects that a device on the USB is asserting resume signaling."]
    #[inline(always)]
    pub const fn set_RD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "UnrecoverableError This bit is set when HC detects a system error not related to USB."]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[must_use]
    #[inline(always)]
    pub const fn FNO(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "FrameNumberOverflow This bit is set when the MSb of HcFmNumber (bit 15) changes value, from 0 to 1 or from 1 to 0, and after HccaFrameNumber has been updated."]
    #[inline(always)]
    pub const fn set_FNO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[must_use]
    #[inline(always)]
    pub const fn RHSC(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RootHubStatusChange This bit is set when the content of HcRhStatus or the content of any of HcRhPortStatus\\[NumberofDownstreamPort\\] has changed."]
    #[inline(always)]
    pub const fn set_RHSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[must_use]
    #[inline(always)]
    pub const fn OC(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "OwnershipChange This bit is set by HC when HCD sets the OwnershipChangeRequest field in HcCommandStatus."]
    #[inline(always)]
    pub const fn set_OC(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for HCINTERRUPTSTATUS {
    #[inline(always)]
    fn default() -> HCINTERRUPTSTATUS {
        HCINTERRUPTSTATUS(0)
    }
}
impl core::fmt::Debug for HCINTERRUPTSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTERRUPTSTATUS")
            .field("SO", &self.SO())
            .field("WDH", &self.WDH())
            .field("SF", &self.SF())
            .field("RD", &self.RD())
            .field("UE", &self.UE())
            .field("FNO", &self.FNO())
            .field("RHSC", &self.RHSC())
            .field("OC", &self.OC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCINTERRUPTSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCINTERRUPTSTATUS {{ SO: {=bool:?}, WDH: {=bool:?}, SF: {=bool:?}, RD: {=bool:?}, UE: {=bool:?}, FNO: {=bool:?}, RHSC: {=bool:?}, OC: {=u32:?} }}",
            self.SO(),
            self.WDH(),
            self.SF(),
            self.RD(),
            self.UE(),
            self.FNO(),
            self.RHSC(),
            self.OC()
        )
    }
}
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCLSTHRESHOLD(pub u32);
impl HCLSTHRESHOLD {
    #[doc = "LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[must_use]
    #[inline(always)]
    pub const fn LST(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub const fn set_LST(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for HCLSTHRESHOLD {
    #[inline(always)]
    fn default() -> HCLSTHRESHOLD {
        HCLSTHRESHOLD(0)
    }
}
impl core::fmt::Debug for HCLSTHRESHOLD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCLSTHRESHOLD")
            .field("LST", &self.LST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCLSTHRESHOLD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCLSTHRESHOLD {{ LST: {=u16:?} }}", self.LST())
    }
}
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCPERIODCURRENTED(pub u32);
impl HCPERIODCURRENTED {
    #[doc = "The content of this register is updated by HC after a periodic ED is processed."]
    #[must_use]
    #[inline(always)]
    pub const fn PCED(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub const fn set_PCED(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for HCPERIODCURRENTED {
    #[inline(always)]
    fn default() -> HCPERIODCURRENTED {
        HCPERIODCURRENTED(0)
    }
}
impl core::fmt::Debug for HCPERIODCURRENTED {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCPERIODCURRENTED")
            .field("PCED", &self.PCED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCPERIODCURRENTED {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCPERIODCURRENTED {{ PCED: {=u32:?} }}", self.PCED())
    }
}
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCPERIODICSTART(pub u32);
impl HCPERIODICSTART {
    #[doc = "PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn PS(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "PeriodicStart After a hardware reset, this field is cleared and then set by HCD during the HC initialization."]
    #[inline(always)]
    pub const fn set_PS(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for HCPERIODICSTART {
    #[inline(always)]
    fn default() -> HCPERIODICSTART {
        HCPERIODICSTART(0)
    }
}
impl core::fmt::Debug for HCPERIODICSTART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCPERIODICSTART")
            .field("PS", &self.PS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCPERIODICSTART {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCPERIODICSTART {{ PS: {=u16:?} }}", self.PS())
    }
}
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCREVISION(pub u32);
impl HCREVISION {
    #[doc = "Revision."]
    #[must_use]
    #[inline(always)]
    pub const fn REV(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision."]
    #[inline(always)]
    pub const fn set_REV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for HCREVISION {
    #[inline(always)]
    fn default() -> HCREVISION {
        HCREVISION(0)
    }
}
impl core::fmt::Debug for HCREVISION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCREVISION")
            .field("REV", &self.REV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCREVISION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HCREVISION {{ REV: {=u8:?} }}", self.REV())
    }
}
#[doc = "First of the two registers which describes the characteristics of the root hub."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCRHDESCRIPTORA(pub u32);
impl HCRHDESCRIPTORA {
    #[doc = "NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[must_use]
    #[inline(always)]
    pub const fn NDP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "NumberDownstreamPorts These bits specify the number of downstream ports supported by the root hub."]
    #[inline(always)]
    pub const fn set_NDP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[must_use]
    #[inline(always)]
    pub const fn PSM(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PowerSwitchingMode This bit is used to specify how the power switching of the root hub ports is controlled."]
    #[inline(always)]
    pub const fn set_PSM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[must_use]
    #[inline(always)]
    pub const fn NPS(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "NoPowerSwitching These bits are used to specify whether power switching is supported or port are always powered."]
    #[inline(always)]
    pub const fn set_NPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DeviceType This bit specifies that the root hub is not a compound device."]
    #[must_use]
    #[inline(always)]
    pub const fn DT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DeviceType This bit specifies that the root hub is not a compound device."]
    #[inline(always)]
    pub const fn set_DT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[must_use]
    #[inline(always)]
    pub const fn OCPM(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentProtectionMode This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub const fn set_OCPM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[must_use]
    #[inline(always)]
    pub const fn NOCP(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "NoOverCurrentProtection This bit describes how the overcurrent status for the root hub ports are reported."]
    #[inline(always)]
    pub const fn set_NOCP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[must_use]
    #[inline(always)]
    pub const fn POTPGT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "PowerOnToPowerGoodTime This byte specifies the duration the HCD has to wait before accessing a powered-on port of the root hub."]
    #[inline(always)]
    pub const fn set_POTPGT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for HCRHDESCRIPTORA {
    #[inline(always)]
    fn default() -> HCRHDESCRIPTORA {
        HCRHDESCRIPTORA(0)
    }
}
impl core::fmt::Debug for HCRHDESCRIPTORA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCRHDESCRIPTORA")
            .field("NDP", &self.NDP())
            .field("PSM", &self.PSM())
            .field("NPS", &self.NPS())
            .field("DT", &self.DT())
            .field("OCPM", &self.OCPM())
            .field("NOCP", &self.NOCP())
            .field("POTPGT", &self.POTPGT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCRHDESCRIPTORA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCRHDESCRIPTORA {{ NDP: {=u8:?}, PSM: {=bool:?}, NPS: {=bool:?}, DT: {=bool:?}, OCPM: {=bool:?}, NOCP: {=bool:?}, POTPGT: {=u8:?} }}",
            self.NDP(),
            self.PSM(),
            self.NPS(),
            self.DT(),
            self.OCPM(),
            self.NOCP(),
            self.POTPGT()
        )
    }
}
#[doc = "Second of the two registers which describes the characteristics of the Root Hub."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCRHDESCRIPTORB(pub u32);
impl HCRHDESCRIPTORB {
    #[doc = "DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[must_use]
    #[inline(always)]
    pub const fn DR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "DeviceRemovable Each bit is dedicated to a port of the Root Hub."]
    #[inline(always)]
    pub const fn set_DR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[must_use]
    #[inline(always)]
    pub const fn PPCM(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "PortPowerControlMask Each bit indicates if a port is affected by a global power control command when PowerSwitchingMode is set."]
    #[inline(always)]
    pub const fn set_PPCM(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for HCRHDESCRIPTORB {
    #[inline(always)]
    fn default() -> HCRHDESCRIPTORB {
        HCRHDESCRIPTORB(0)
    }
}
impl core::fmt::Debug for HCRHDESCRIPTORB {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCRHDESCRIPTORB")
            .field("DR", &self.DR())
            .field("PPCM", &self.PPCM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCRHDESCRIPTORB {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCRHDESCRIPTORB {{ DR: {=u16:?}, PPCM: {=u16:?} }}",
            self.DR(),
            self.PPCM()
        )
    }
}
#[doc = "Controls and reports the port events on a per-port basis."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCRHPORTSTATUS(pub u32);
impl HCRHPORTSTATUS {
    #[doc = "(read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[must_use]
    #[inline(always)]
    pub const fn CCS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "(read) CurrentConnectStatus This bit reflects the current state of the downstream port."]
    #[inline(always)]
    pub const fn set_CCS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "(read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn PES(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortEnableStatus This bit indicates whether the port is enabled or disabled."]
    #[inline(always)]
    pub const fn set_PES(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "(read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn PSS(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortSuspendStatus This bit indicates the port is suspended or in the resume sequence."]
    #[inline(always)]
    pub const fn set_PSS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "(read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[must_use]
    #[inline(always)]
    pub const fn POCI(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortOverCurrentIndicator This bit is only valid when the Root Hub is configured in such a way that overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub const fn set_POCI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "(read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn PRS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortResetStatus When this bit is set by a write to SetPortReset, port reset signaling is asserted."]
    #[inline(always)]
    pub const fn set_PRS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "(read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn PPS(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "(read) PortPowerStatus This bit reflects the porta's power status, regardless of the type of power switching implemented."]
    #[inline(always)]
    pub const fn set_PPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "(read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[must_use]
    #[inline(always)]
    pub const fn LSDA(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LowSpeedDeviceAttached This bit indicates the speed of the device attached to this port."]
    #[inline(always)]
    pub const fn set_LSDA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn CSC(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ConnectStatusChange This bit is set whenever a connect or disconnect event occurs."]
    #[inline(always)]
    pub const fn set_CSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn PESC(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "PortEnableStatusChange This bit is set when hardware events cause the PortEnableStatus bit to be cleared."]
    #[inline(always)]
    pub const fn set_PESC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn PSSC(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "PortSuspendStatusChange This bit is set when the full resume sequence is completed."]
    #[inline(always)]
    pub const fn set_PSSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[must_use]
    #[inline(always)]
    pub const fn OCIC(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "PortOverCurrentIndicatorChange This bit is valid only if overcurrent conditions are reported on a per-port basis."]
    #[inline(always)]
    pub const fn set_OCIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[must_use]
    #[inline(always)]
    pub const fn PRSC(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PortResetStatusChange This bit is set at the end of the 10 ms port reset signal."]
    #[inline(always)]
    pub const fn set_PRSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for HCRHPORTSTATUS {
    #[inline(always)]
    fn default() -> HCRHPORTSTATUS {
        HCRHPORTSTATUS(0)
    }
}
impl core::fmt::Debug for HCRHPORTSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCRHPORTSTATUS")
            .field("CCS", &self.CCS())
            .field("PES", &self.PES())
            .field("PSS", &self.PSS())
            .field("POCI", &self.POCI())
            .field("PRS", &self.PRS())
            .field("PPS", &self.PPS())
            .field("LSDA", &self.LSDA())
            .field("CSC", &self.CSC())
            .field("PESC", &self.PESC())
            .field("PSSC", &self.PSSC())
            .field("OCIC", &self.OCIC())
            .field("PRSC", &self.PRSC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCRHPORTSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCRHPORTSTATUS {{ CCS: {=bool:?}, PES: {=bool:?}, PSS: {=bool:?}, POCI: {=bool:?}, PRS: {=bool:?}, PPS: {=bool:?}, LSDA: {=bool:?}, CSC: {=bool:?}, PESC: {=bool:?}, PSSC: {=bool:?}, OCIC: {=bool:?}, PRSC: {=bool:?} }}",
            self.CCS(),
            self.PES(),
            self.PSS(),
            self.POCI(),
            self.PRS(),
            self.PPS(),
            self.LSDA(),
            self.CSC(),
            self.PESC(),
            self.PSSC(),
            self.OCIC(),
            self.PRSC()
        )
    }
}
#[doc = "This register is divided into two parts."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCRHSTATUS(pub u32);
impl HCRHSTATUS {
    #[doc = "(read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[must_use]
    #[inline(always)]
    pub const fn LPS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LocalPowerStatus The Root Hub does not support the local power status feature; thus, this bit is always read as 0."]
    #[inline(always)]
    pub const fn set_LPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn OCI(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentIndicator This bit reports overcurrent conditions when the global reporting is implemented."]
    #[inline(always)]
    pub const fn set_OCI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "(read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DRWE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "(read) DeviceRemoteWakeupEnable This bit enables a ConnectStatusChange bit as a resume event, causing a USBSUSPEND to USBRESUME state transition and setting the ResumeDetected interrupt."]
    #[inline(always)]
    pub const fn set_DRWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "(read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[must_use]
    #[inline(always)]
    pub const fn LPSC(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "(read) LocalPowerStatusChange The root hub does not support the local power status feature."]
    #[inline(always)]
    pub const fn set_LPSC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[must_use]
    #[inline(always)]
    pub const fn OCIC(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "OverCurrentIndicatorChange This bit is set by hardware when a change has occurred to the OCI field of this register."]
    #[inline(always)]
    pub const fn set_OCIC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "(write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[must_use]
    #[inline(always)]
    pub const fn CRWE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "(write) ClearRemoteWakeupEnable Writing a 1 clears DeviceRemoveWakeupEnable."]
    #[inline(always)]
    pub const fn set_CRWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HCRHSTATUS {
    #[inline(always)]
    fn default() -> HCRHSTATUS {
        HCRHSTATUS(0)
    }
}
impl core::fmt::Debug for HCRHSTATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCRHSTATUS")
            .field("LPS", &self.LPS())
            .field("OCI", &self.OCI())
            .field("DRWE", &self.DRWE())
            .field("LPSC", &self.LPSC())
            .field("OCIC", &self.OCIC())
            .field("CRWE", &self.CRWE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCRHSTATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCRHSTATUS {{ LPS: {=bool:?}, OCI: {=bool:?}, DRWE: {=bool:?}, LPSC: {=bool:?}, OCIC: {=bool:?}, CRWE: {=bool:?} }}",
            self.LPS(),
            self.OCI(),
            self.DRWE(),
            self.LPSC(),
            self.OCIC(),
            self.CRWE()
        )
    }
}
#[doc = "Controls the port if it is attached to the host block or the device block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORTMODE(pub u32);
impl PORTMODE {
    #[doc = "Port ID pin value."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port ID pin value."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port ID pin pull-up enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_EN(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port ID pin pull-up enable."]
    #[inline(always)]
    pub const fn set_ID_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1: device 0: host."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_ENABLE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1: device 0: host."]
    #[inline(always)]
    pub const fn set_DEV_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for PORTMODE {
    #[inline(always)]
    fn default() -> PORTMODE {
        PORTMODE(0)
    }
}
impl core::fmt::Debug for PORTMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTMODE")
            .field("ID", &self.ID())
            .field("ID_EN", &self.ID_EN())
            .field("DEV_ENABLE", &self.DEV_ENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORTMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PORTMODE {{ ID: {=bool:?}, ID_EN: {=bool:?}, DEV_ENABLE: {=bool:?} }}",
            self.ID(),
            self.ID_EN(),
            self.DEV_ENABLE()
        )
    }
}
