#[doc = "USB0 Full-speed Host controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBFSH {
    ptr: *mut u8,
}
unsafe impl Send for USBFSH {}
unsafe impl Sync for USBFSH {}
impl USBFSH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)."]
    #[inline(always)]
    pub const fn HCREVISION(self) -> crate::common::Reg<regs::HCREVISION, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Defines the operating modes of the HC."]
    #[inline(always)]
    pub const fn HCCONTROL(self) -> crate::common::Reg<regs::HCCONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)."]
    #[inline(always)]
    pub const fn HCCOMMANDSTATUS(
        self,
    ) -> crate::common::Reg<regs::HCCOMMANDSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits."]
    #[inline(always)]
    pub const fn HCINTERRUPTSTATUS(
        self,
    ) -> crate::common::Reg<regs::HCINTERRUPTSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt."]
    #[inline(always)]
    pub const fn HCINTERRUPTENABLE(
        self,
    ) -> crate::common::Reg<regs::HCINTERRUPTENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt."]
    #[inline(always)]
    pub const fn HCINTERRUPTDISABLE(
        self,
    ) -> crate::common::Reg<regs::HCINTERRUPTDISABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Contains the physical address of the host controller communication area."]
    #[inline(always)]
    pub const fn HCHCCA(self) -> crate::common::Reg<regs::HCHCCA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor."]
    #[inline(always)]
    pub const fn HCPERIODCURRENTED(
        self,
    ) -> crate::common::Reg<regs::HCPERIODCURRENTED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Contains the physical address of the first endpoint descriptor of the control list."]
    #[inline(always)]
    pub const fn HCCONTROLHEADED(
        self,
    ) -> crate::common::Reg<regs::HCCONTROLHEADED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Contains the physical address of the current endpoint descriptor of the control list."]
    #[inline(always)]
    pub const fn HCCONTROLCURRENTED(
        self,
    ) -> crate::common::Reg<regs::HCCONTROLCURRENTED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Contains the physical address of the first endpoint descriptor of the bulk list."]
    #[inline(always)]
    pub const fn HCBULKHEADED(self) -> crate::common::Reg<regs::HCBULKHEADED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Contains the physical address of the current endpoint descriptor of the bulk list."]
    #[inline(always)]
    pub const fn HCBULKCURRENTED(
        self,
    ) -> crate::common::Reg<regs::HCBULKCURRENTED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue."]
    #[inline(always)]
    pub const fn HCDONEHEAD(self) -> crate::common::Reg<regs::HCDONEHEAD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun."]
    #[inline(always)]
    pub const fn HCFMINTERVAL(self) -> crate::common::Reg<regs::HCFMINTERVAL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "A 14-bit counter showing the bit time remaining in the current frame."]
    #[inline(always)]
    pub const fn HCFMREMAINING(self) -> crate::common::Reg<regs::HCFMREMAINING, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD."]
    #[inline(always)]
    pub const fn HCFMNUMBER(self) -> crate::common::Reg<regs::HCFMNUMBER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list."]
    #[inline(always)]
    pub const fn HCPERIODICSTART(
        self,
    ) -> crate::common::Reg<regs::HCPERIODICSTART, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF."]
    #[inline(always)]
    pub const fn HCLSTHRESHOLD(self) -> crate::common::Reg<regs::HCLSTHRESHOLD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "First of the two registers which describes the characteristics of the root hub."]
    #[inline(always)]
    pub const fn HCRHDESCRIPTORA(
        self,
    ) -> crate::common::Reg<regs::HCRHDESCRIPTORA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Second of the two registers which describes the characteristics of the Root Hub."]
    #[inline(always)]
    pub const fn HCRHDESCRIPTORB(
        self,
    ) -> crate::common::Reg<regs::HCRHDESCRIPTORB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "This register is divided into two parts."]
    #[inline(always)]
    pub const fn HCRHSTATUS(self) -> crate::common::Reg<regs::HCRHSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Controls and reports the port events on a per-port basis."]
    #[inline(always)]
    pub const fn HCRHPORTSTATUS(
        self,
    ) -> crate::common::Reg<regs::HCRHPORTSTATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Controls the port if it is attached to the host block or the device block."]
    #[inline(always)]
    pub const fn PORTMODE(self) -> crate::common::Reg<regs::PORTMODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
