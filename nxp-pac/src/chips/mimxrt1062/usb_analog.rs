#[doc = "USB Analog"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbAnalog {
    ptr: *mut u8,
}
unsafe impl Send for UsbAnalog {}
unsafe impl Sync for UsbAnalog {}
impl UsbAnalog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "USB VBUS Detect Status Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "USB Charger Detect Status Register"]
    #[inline(always)]
    pub const fn usb1_chrg_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb1ChrgDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback(self) -> crate::common::Reg<regs::Usb1Loopback, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_set(
        self,
    ) -> crate::common::Reg<regs::Usb1LoopbackSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1LoopbackClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb1_loopback_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1LoopbackTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc(self) -> crate::common::Reg<regs::Usb1Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_set(self) -> crate::common::Reg<regs::Usb1MiscSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_clr(self) -> crate::common::Reg<regs::Usb1MiscClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb1_misc_tog(self) -> crate::common::Reg<regs::Usb1MiscTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb2_vbus_detect(
        self,
    ) -> crate::common::Reg<regs::Usb2VbusDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb2_vbus_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb2VbusDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb2_vbus_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb2VbusDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "USB VBUS Detect Register"]
    #[inline(always)]
    pub const fn usb2_vbus_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb2VbusDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb2_chrg_detect(
        self,
    ) -> crate::common::Reg<regs::Usb2ChrgDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb2_chrg_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb2ChrgDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb2_chrg_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb2ChrgDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "USB Charger Detect Register"]
    #[inline(always)]
    pub const fn usb2_chrg_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb2ChrgDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "USB VBUS Detect Status Register"]
    #[inline(always)]
    pub const fn usb2_vbus_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb2VbusDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "USB Charger Detect Status Register"]
    #[inline(always)]
    pub const fn usb2_chrg_detect_stat(
        self,
    ) -> crate::common::Reg<regs::Usb2ChrgDetectStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb2_loopback(self) -> crate::common::Reg<regs::Usb2Loopback, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb2_loopback_set(
        self,
    ) -> crate::common::Reg<regs::Usb2LoopbackSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb2_loopback_clr(
        self,
    ) -> crate::common::Reg<regs::Usb2LoopbackClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "USB Loopback Test Register"]
    #[inline(always)]
    pub const fn usb2_loopback_tog(
        self,
    ) -> crate::common::Reg<regs::Usb2LoopbackTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb2_misc(self) -> crate::common::Reg<regs::Usb2Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb2_misc_set(self) -> crate::common::Reg<regs::Usb2MiscSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb2_misc_clr(self) -> crate::common::Reg<regs::Usb2MiscClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "USB Misc Register"]
    #[inline(always)]
    pub const fn usb2_misc_tog(self) -> crate::common::Reg<regs::Usb2MiscTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Chip Silicon Version"]
    #[inline(always)]
    pub const fn digprog(self) -> crate::common::Reg<regs::Digprog, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
}
pub mod regs;
pub mod vals;
