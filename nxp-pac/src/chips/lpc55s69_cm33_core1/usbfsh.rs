#[doc = "USB0 Full-speed Host controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbfsh {
    ptr: *mut u8,
}
unsafe impl Send for Usbfsh {}
unsafe impl Sync for Usbfsh {}
impl Usbfsh {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
    #[inline(always)]
    pub const fn hcrevision(self) -> crate::common::Reg<regs::Hcrevision, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Defines the operating modes of the HC"]
    #[inline(always)]
    pub const fn hccontrol(self) -> crate::common::Reg<regs::Hccontrol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
    #[inline(always)]
    pub const fn hccommandstatus(
        self,
    ) -> crate::common::Reg<regs::Hccommandstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
    #[inline(always)]
    pub const fn hcinterruptstatus(
        self,
    ) -> crate::common::Reg<regs::Hcinterruptstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
    #[inline(always)]
    pub const fn hcinterruptenable(
        self,
    ) -> crate::common::Reg<regs::Hcinterruptenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
    #[inline(always)]
    pub const fn hcinterruptdisable(
        self,
    ) -> crate::common::Reg<regs::Hcinterruptdisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Contains the physical address of the host controller communication area"]
    #[inline(always)]
    pub const fn hchcca(self) -> crate::common::Reg<regs::Hchcca, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
    #[inline(always)]
    pub const fn hcperiodcurrented(
        self,
    ) -> crate::common::Reg<regs::Hcperiodcurrented, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
    #[inline(always)]
    pub const fn hccontrolheaded(
        self,
    ) -> crate::common::Reg<regs::Hccontrolheaded, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
    #[inline(always)]
    pub const fn hccontrolcurrented(
        self,
    ) -> crate::common::Reg<regs::Hccontrolcurrented, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
    #[inline(always)]
    pub const fn hcbulkheaded(self) -> crate::common::Reg<regs::Hcbulkheaded, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
    #[inline(always)]
    pub const fn hcbulkcurrented(
        self,
    ) -> crate::common::Reg<regs::Hcbulkcurrented, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
    #[inline(always)]
    pub const fn hcdonehead(self) -> crate::common::Reg<regs::Hcdonehead, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
    #[inline(always)]
    pub const fn hcfminterval(self) -> crate::common::Reg<regs::Hcfminterval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
    #[inline(always)]
    pub const fn hcfmremaining(self) -> crate::common::Reg<regs::Hcfmremaining, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
    #[inline(always)]
    pub const fn hcfmnumber(self) -> crate::common::Reg<regs::Hcfmnumber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
    #[inline(always)]
    pub const fn hcperiodicstart(
        self,
    ) -> crate::common::Reg<regs::Hcperiodicstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
    #[inline(always)]
    pub const fn hclsthreshold(self) -> crate::common::Reg<regs::Hclsthreshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "First of the two registers which describes the characteristics of the root hub"]
    #[inline(always)]
    pub const fn hcrhdescriptora(
        self,
    ) -> crate::common::Reg<regs::Hcrhdescriptora, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
    #[inline(always)]
    pub const fn hcrhdescriptorb(
        self,
    ) -> crate::common::Reg<regs::Hcrhdescriptorb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "This register is divided into two parts"]
    #[inline(always)]
    pub const fn hcrhstatus(self) -> crate::common::Reg<regs::Hcrhstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Controls and reports the port events on a per-port basis"]
    #[inline(always)]
    pub const fn hcrhportstatus(
        self,
    ) -> crate::common::Reg<regs::Hcrhportstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Controls the port if it is attached to the host block or the device block"]
    #[inline(always)]
    pub const fn portmode(self) -> crate::common::Reg<regs::Portmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
}
pub mod regs;
