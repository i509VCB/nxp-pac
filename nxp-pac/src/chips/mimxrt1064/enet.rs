#[doc = "ENET"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet {
    ptr: *mut u8,
}
unsafe impl Send for Enet {}
unsafe impl Sync for Enet {}
impl Enet {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Event Register"]
    #[inline(always)]
    pub const fn eir(self) -> crate::common::Reg<regs::Eir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Mask Register"]
    #[inline(always)]
    pub const fn eimr(self) -> crate::common::Reg<regs::Eimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Receive Descriptor Active Register - Ring 0"]
    #[inline(always)]
    pub const fn rdar(self) -> crate::common::Reg<regs::Rdar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Transmit Descriptor Active Register - Ring 0"]
    #[inline(always)]
    pub const fn tdar(self) -> crate::common::Reg<regs::Tdar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Ethernet Control Register"]
    #[inline(always)]
    pub const fn ecr(self) -> crate::common::Reg<regs::Ecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MII Management Frame Register"]
    #[inline(always)]
    pub const fn mmfr(self) -> crate::common::Reg<regs::Mmfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "MII Speed Control Register"]
    #[inline(always)]
    pub const fn mscr(self) -> crate::common::Reg<regs::Mscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "MIB Control Register"]
    #[inline(always)]
    pub const fn mibc(self) -> crate::common::Reg<regs::Mibc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Receive Control Register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Transmit Control Register"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Physical Address Lower Register"]
    #[inline(always)]
    pub const fn palr(self) -> crate::common::Reg<regs::Palr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Physical Address Upper Register"]
    #[inline(always)]
    pub const fn paur(self) -> crate::common::Reg<regs::Paur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Opcode/Pause Duration Register"]
    #[inline(always)]
    pub const fn opd(self) -> crate::common::Reg<regs::Opd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Transmit Interrupt Coalescing Register"]
    #[inline(always)]
    pub const fn txic0(self) -> crate::common::Reg<regs::Txic0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Receive Interrupt Coalescing Register"]
    #[inline(always)]
    pub const fn rxic0(self) -> crate::common::Reg<regs::Rxic0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Descriptor Individual Upper Address Register"]
    #[inline(always)]
    pub const fn iaur(self) -> crate::common::Reg<regs::Iaur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Descriptor Individual Lower Address Register"]
    #[inline(always)]
    pub const fn ialr(self) -> crate::common::Reg<regs::Ialr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Descriptor Group Upper Address Register"]
    #[inline(always)]
    pub const fn gaur(self) -> crate::common::Reg<regs::Gaur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Descriptor Group Lower Address Register"]
    #[inline(always)]
    pub const fn galr(self) -> crate::common::Reg<regs::Galr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Transmit FIFO Watermark Register"]
    #[inline(always)]
    pub const fn tfwr(self) -> crate::common::Reg<regs::Tfwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Receive Descriptor Ring 0 Start Register"]
    #[inline(always)]
    pub const fn rdsr(self) -> crate::common::Reg<regs::Rdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Transmit Buffer Descriptor Ring 0 Start Register"]
    #[inline(always)]
    pub const fn tdsr(self) -> crate::common::Reg<regs::Tdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Maximum Receive Buffer Size Register - Ring 0"]
    #[inline(always)]
    pub const fn mrbr(self) -> crate::common::Reg<regs::Mrbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub const fn rsfl(self) -> crate::common::Reg<regs::Rsfl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Receive FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn rsem(self) -> crate::common::Reg<regs::Rsem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Receive FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn raem(self) -> crate::common::Reg<regs::Raem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn rafl(self) -> crate::common::Reg<regs::Rafl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "Transmit FIFO Section Empty Threshold"]
    #[inline(always)]
    pub const fn tsem(self) -> crate::common::Reg<regs::Tsem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Transmit FIFO Almost Empty Threshold"]
    #[inline(always)]
    pub const fn taem(self) -> crate::common::Reg<regs::Taem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub const fn tafl(self) -> crate::common::Reg<regs::Tafl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub const fn tipg(self) -> crate::common::Reg<regs::Tipg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Frame Truncation Length"]
    #[inline(always)]
    pub const fn ftrl(self) -> crate::common::Reg<regs::Ftrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Transmit Accelerator Function Configuration"]
    #[inline(always)]
    pub const fn tacc(self) -> crate::common::Reg<regs::Tacc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Receive Accelerator Function Configuration"]
    #[inline(always)]
    pub const fn racc(self) -> crate::common::Reg<regs::Racc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Tx Packet Count Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_packets(self) -> crate::common::Reg<regs::RmonTPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Tx Broadcast Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_bc_pkt(self) -> crate::common::Reg<regs::RmonTBcPkt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Tx Multicast Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_mc_pkt(self) -> crate::common::Reg<regs::RmonTMcPkt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Tx Packets with CRC/Align Error Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_crc_align(
        self,
    ) -> crate::common::Reg<regs::RmonTCrcAlign, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Tx Packets Less Than Bytes and Good CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_undersize(
        self,
    ) -> crate::common::Reg<regs::RmonTUndersize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_oversize(
        self,
    ) -> crate::common::Reg<regs::RmonTOversize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Tx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_frag(self) -> crate::common::Reg<regs::RmonTFrag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Tx Packets Greater Than MAX_FL bytes and Bad CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_jab(self) -> crate::common::Reg<regs::RmonTJab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Tx Collision Count Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_col(self) -> crate::common::Reg<regs::RmonTCol, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Tx 64-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p64(self) -> crate::common::Reg<regs::RmonTP64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Tx 65- to 127-byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p65to127(
        self,
    ) -> crate::common::Reg<regs::RmonTP65to127, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Tx 128- to 255-byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p128to255(
        self,
    ) -> crate::common::Reg<regs::RmonTP128to255, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Tx 256- to 511-byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p256to511(
        self,
    ) -> crate::common::Reg<regs::RmonTP256to511, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Tx 512- to 1023-byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p512to1023(
        self,
    ) -> crate::common::Reg<regs::RmonTP512to1023, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Tx 1024- to 2047-byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p1024to2047(
        self,
    ) -> crate::common::Reg<regs::RmonTP1024to2047, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Tx Packets Greater Than 2048 Bytes Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_p_gte2048(
        self,
    ) -> crate::common::Reg<regs::RmonTPGte2048, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "Tx Octets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_t_octets(self) -> crate::common::Reg<regs::RmonTOctets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "Frames Transmitted OK Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_frame_ok(self) -> crate::common::Reg<regs::IeeeTFrameOk, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "Frames Transmitted with Single Collision Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_1col(self) -> crate::common::Reg<regs::IeeeT1col, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Frames Transmitted with Multiple Collisions Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_mcol(self) -> crate::common::Reg<regs::IeeeTMcol, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "Frames Transmitted after Deferral Delay Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_def(self) -> crate::common::Reg<regs::IeeeTDef, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "Frames Transmitted with Late Collision Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_lcol(self) -> crate::common::Reg<regs::IeeeTLcol, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "Frames Transmitted with Excessive Collisions Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_excol(self) -> crate::common::Reg<regs::IeeeTExcol, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "Frames Transmitted with Tx FIFO Underrun Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_macerr(self) -> crate::common::Reg<regs::IeeeTMacerr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Frames Transmitted with Carrier Sense Error Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_cserr(self) -> crate::common::Reg<regs::IeeeTCserr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "Reserved Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_sqe(self) -> crate::common::Reg<regs::IeeeTSqe, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "Flow Control Pause Frames Transmitted Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_fdxfc(self) -> crate::common::Reg<regs::IeeeTFdxfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register"]
    #[inline(always)]
    pub const fn ieee_t_octets_ok(
        self,
    ) -> crate::common::Reg<regs::IeeeTOctetsOk, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "Rx Packet Count Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_packets(self) -> crate::common::Reg<regs::RmonRPackets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Rx Broadcast Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_bc_pkt(self) -> crate::common::Reg<regs::RmonRBcPkt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "Rx Multicast Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_mc_pkt(self) -> crate::common::Reg<regs::RmonRMcPkt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "Rx Packets with CRC/Align Error Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_crc_align(
        self,
    ) -> crate::common::Reg<regs::RmonRCrcAlign, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Rx Packets with Less Than 64 Bytes and Good CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_undersize(
        self,
    ) -> crate::common::Reg<regs::RmonRUndersize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "Rx Packets Greater Than MAX_FL and Good CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_oversize(
        self,
    ) -> crate::common::Reg<regs::RmonROversize, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Rx Packets Less Than 64 Bytes and Bad CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_frag(self) -> crate::common::Reg<regs::RmonRFrag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Rx Packets Greater Than MAX_FL Bytes and Bad CRC Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_jab(self) -> crate::common::Reg<regs::RmonRJab, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Rx 64-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p64(self) -> crate::common::Reg<regs::RmonRP64, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Rx 65- to 127-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p65to127(
        self,
    ) -> crate::common::Reg<regs::RmonRP65to127, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Rx 128- to 255-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p128to255(
        self,
    ) -> crate::common::Reg<regs::RmonRP128to255, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Rx 256- to 511-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p256to511(
        self,
    ) -> crate::common::Reg<regs::RmonRP256to511, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Rx 512- to 1023-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p512to1023(
        self,
    ) -> crate::common::Reg<regs::RmonRP512to1023, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Rx 1024- to 2047-Byte Packets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p1024to2047(
        self,
    ) -> crate::common::Reg<regs::RmonRP1024to2047, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "Rx Packets Greater than 2048 Bytes Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_p_gte2048(
        self,
    ) -> crate::common::Reg<regs::RmonRPGte2048, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Rx Octets Statistic Register"]
    #[inline(always)]
    pub const fn rmon_r_octets(self) -> crate::common::Reg<regs::RmonROctets, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "Frames not Counted Correctly Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_drop(self) -> crate::common::Reg<regs::IeeeRDrop, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "Frames Received OK Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_frame_ok(self) -> crate::common::Reg<regs::IeeeRFrameOk, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "Frames Received with CRC Error Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_crc(self) -> crate::common::Reg<regs::IeeeRCrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "Frames Received with Alignment Error Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_align(self) -> crate::common::Reg<regs::IeeeRAlign, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "Receive FIFO Overflow Count Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_macerr(self) -> crate::common::Reg<regs::IeeeRMacerr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "Flow Control Pause Frames Received Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_fdxfc(self) -> crate::common::Reg<regs::IeeeRFdxfc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "Octet Count for Frames Received without Error Statistic Register"]
    #[inline(always)]
    pub const fn ieee_r_octets_ok(
        self,
    ) -> crate::common::Reg<regs::IeeeROctetsOk, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "Adjustable Timer Control Register"]
    #[inline(always)]
    pub const fn atcr(self) -> crate::common::Reg<regs::Atcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Timer Value Register"]
    #[inline(always)]
    pub const fn atvr(self) -> crate::common::Reg<regs::Atvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "Timer Offset Register"]
    #[inline(always)]
    pub const fn atoff(self) -> crate::common::Reg<regs::Atoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "Timer Period Register"]
    #[inline(always)]
    pub const fn atper(self) -> crate::common::Reg<regs::Atper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "Timer Correction Register"]
    #[inline(always)]
    pub const fn atcor(self) -> crate::common::Reg<regs::Atcor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Time-Stamping Clock Period Register"]
    #[inline(always)]
    pub const fn atinc(self) -> crate::common::Reg<regs::Atinc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Timestamp of Last Transmitted Frame"]
    #[inline(always)]
    pub const fn atstmp(self) -> crate::common::Reg<regs::Atstmp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "Timer Global Status Register"]
    #[inline(always)]
    pub const fn tgsr(self) -> crate::common::Reg<regs::Tgsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Timer Control Status Register"]
    #[inline(always)]
    pub const fn tcsr0(self) -> crate::common::Reg<regs::Tcsr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "Timer Compare Capture Register"]
    #[inline(always)]
    pub const fn tccr0(self) -> crate::common::Reg<regs::Tccr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "Timer Control Status Register"]
    #[inline(always)]
    pub const fn tcsr1(self) -> crate::common::Reg<regs::Tcsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Timer Compare Capture Register"]
    #[inline(always)]
    pub const fn tccr1(self) -> crate::common::Reg<regs::Tccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "Timer Control Status Register"]
    #[inline(always)]
    pub const fn tcsr2(self) -> crate::common::Reg<regs::Tcsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Timer Compare Capture Register"]
    #[inline(always)]
    pub const fn tccr2(self) -> crate::common::Reg<regs::Tccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Timer Control Status Register"]
    #[inline(always)]
    pub const fn tcsr3(self) -> crate::common::Reg<regs::Tcsr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Timer Compare Capture Register"]
    #[inline(always)]
    pub const fn tccr3(self) -> crate::common::Reg<regs::Tccr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
}
pub mod regs;
pub mod vals;
