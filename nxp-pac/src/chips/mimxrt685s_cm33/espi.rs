#[doc = "a variant of SPI used by Intel to communicate with its processors via the PCH (aka Southbridge)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Espi {
    ptr: *mut u8,
}
unsafe impl Send for Espi {}
unsafe impl Sync for Espi {}
impl Espi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Control for whole peripheral"]
    #[inline(always)]
    pub const fn mctrl(self) -> crate::common::Reg<regs::Mctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Master Status of whole peripheral"]
    #[inline(always)]
    pub const fn mstat(self) -> crate::common::Reg<regs::Mstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Set (enable)"]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Interrupt Clear (disable)"]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Masked interrupt status (causes)"]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Selects DMA for Ports (if used)"]
    #[inline(always)]
    pub const fn dmactrl(self) -> crate::common::Reg<regs::Dmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Address of RAM to use for data. This tells the application where the RAM is located (up to 16K addressable)."]
    #[inline(always)]
    pub const fn rambase(self) -> crate::common::Reg<regs::Rambase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Base0 and Base1 mapped offsets for ports"]
    #[inline(always)]
    pub const fn mapbase(self) -> crate::common::Reg<regs::Mapbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "IRQ to drive into Host (with eSPI)"]
    #[inline(always)]
    pub const fn irqpush(self) -> crate::common::Reg<regs::Irqpush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Wire states for Host to see; if LPC, this is the IRQ states."]
    #[inline(always)]
    pub const fn wirewo(self) -> crate::common::Reg<regs::Wirewo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Wire states from Host"]
    #[inline(always)]
    pub const fn wirero(self) -> crate::common::Reg<regs::Wirero, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Port 80 Status (byte and prev byte)"]
    #[inline(always)]
    pub const fn p80stat(self) -> crate::common::Reg<regs::P80stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Location of Status block in memory space, if enabled."]
    #[inline(always)]
    pub const fn stataddr(self) -> crate::common::Reg<regs::Stataddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "eSPI Capabilities of MCU to send to Host"]
    #[inline(always)]
    pub const fn espicap(self) -> crate::common::Reg<regs::Espicap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "eSPI Configuration settings from eSPI"]
    #[inline(always)]
    pub const fn espicfg(self) -> crate::common::Reg<regs::Espicfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Miscellaneous uses, such as Alert pin as GPIO (when not used for Alert)."]
    #[inline(always)]
    pub const fn espimisc(self) -> crate::common::Reg<regs::Espimisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p0cfg(self) -> crate::common::Reg<regs::P0cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p0stat(self) -> crate::common::Reg<regs::P0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p0irule_stat(self) -> crate::common::Reg<regs::P0iruleStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
    #[inline(always)]
    pub const fn p0addr(self) -> crate::common::Reg<regs::P0addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p0omflen(self) -> crate::common::Reg<regs::P0omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p0data_in(self) -> crate::common::Reg<regs::P0dataIn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p0data_out(self) -> crate::common::Reg<regs::P0dataOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p0ramuse(self) -> crate::common::Reg<regs::P0ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p1cfg(self) -> crate::common::Reg<regs::P1cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p1stat(self) -> crate::common::Reg<regs::P1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p1irule_stat(self) -> crate::common::Reg<regs::P1iruleStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
    #[inline(always)]
    pub const fn p1addr(self) -> crate::common::Reg<regs::P1addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p1omflen(self) -> crate::common::Reg<regs::P1omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p1data_in(self) -> crate::common::Reg<regs::P1dataIn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p1data_out(self) -> crate::common::Reg<regs::P1dataOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p1ramuse(self) -> crate::common::Reg<regs::P1ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p2cfg(self) -> crate::common::Reg<regs::P2cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p2stat(self) -> crate::common::Reg<regs::P2stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p2irule_stat(self) -> crate::common::Reg<regs::P2iruleStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
    #[inline(always)]
    pub const fn p2addr(self) -> crate::common::Reg<regs::P2addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p2omflen(self) -> crate::common::Reg<regs::P2omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p2data_in(self) -> crate::common::Reg<regs::P2dataIn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p2data_out(self) -> crate::common::Reg<regs::P2dataOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p2ramuse(self) -> crate::common::Reg<regs::P2ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p3cfg(self) -> crate::common::Reg<regs::P3cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p3stat(self) -> crate::common::Reg<regs::P3stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p3irule_stat(self) -> crate::common::Reg<regs::P3iruleStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
    #[inline(always)]
    pub const fn p3addr(self) -> crate::common::Reg<regs::P3addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p3omflen(self) -> crate::common::Reg<regs::P3omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p3data_in(self) -> crate::common::Reg<regs::P3dataIn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p3data_out(self) -> crate::common::Reg<regs::P3dataOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p3ramuse(self) -> crate::common::Reg<regs::P3ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p4cfg(self) -> crate::common::Reg<regs::P4cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p4stat(self) -> crate::common::Reg<regs::P4stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "The Port Set Interrupt-Rule and Set User Status register is used to set: The interrupt causes per port. That is, it is used to select what events from the port should cause an interrupt, if any. The user Status bits. The status byte returned to the host will be composed of both these user bits (which the application defines) and automatically generated status. The interrupt masks then are matched by sticky cause bits in PnSTAT (which can be read and then write-1 cleared). The sticky bits are set whether the interrupt is masked or not, but the masks cause an interrupt when the bits are set and the port is int enabled via INTENSET."]
    #[inline(always)]
    pub const fn p4irule_stat(self) -> crate::common::Reg<regs::P4iruleStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "The Port Address register is used to set the mapped address in the LPC/eSPI memory space. For normal Ports, this is a 16-bit location in IO space or a 32-bit offset into PCIe address mapped space. It is not used for OOB, Bus Mastering, or Flash; see the PnOMFLEN register for those. For Endpoints, it is the base of a dword (64 bits). For Index/Data, it is the base of a word (32 bits). For mailbox memory, the address is modulus the length of the mailbox (x2 for both directions). So, if a 16-byte mailbox (single), the offset must have bits 3:0 set to 0 (that is &~0xF). If split directions, then normally bits 4:0 would be set to 0 (that is &~0x1F)."]
    #[inline(always)]
    pub const fn p4addr(self) -> crate::common::Reg<regs::P4addr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "The Port OOB, Mastering, Flash length is used for OOB to Host and Bus Mastering and Flash to or from Host. The PnADDR register is not used for either OOB or Bus Mastering. Instead, this register (in same location) is written with byte length message (including address for Mastering and Flash) along with direction. Writing to the SSTCL field of the PIRuleState will notify the Host to perform the transaction. Clearing it before the Host has started a transaction will cancel it, but only if it has not yet seen the Status."]
    #[inline(always)]
    pub const fn p4omflen(self) -> crate::common::Reg<regs::P4omflen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p4data_in(self) -> crate::common::Reg<regs::P4dataIn, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p4data_out(self) -> crate::common::Reg<regs::P4dataOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn p4ramuse(self) -> crate::common::Reg<regs::P4ramuse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
}
pub mod regs;
pub mod vals;
