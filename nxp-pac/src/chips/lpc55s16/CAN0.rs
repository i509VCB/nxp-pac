#[doc = "Controller Area Network Flexible Data (CAN FD)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAN0 {
    ptr: *mut u8,
}
unsafe impl Send for CAN0 {}
unsafe impl Sync for CAN0 {}
impl CAN0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Data Bit Timing Prescaler Register."]
    #[inline(always)]
    pub const fn DBTP(self) -> crate::common::Reg<regs::DBTP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Test Register."]
    #[inline(always)]
    pub const fn TEST(self) -> crate::common::Reg<regs::TEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CC Control Register."]
    #[inline(always)]
    pub const fn CCCR(self) -> crate::common::Reg<regs::CCCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Nominal Bit Timing and Prescaler Register."]
    #[inline(always)]
    pub const fn NBTP(self) -> crate::common::Reg<regs::NBTP, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Timestamp Counter Configuration."]
    #[inline(always)]
    pub const fn TSCC(self) -> crate::common::Reg<regs::TSCC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Timestamp Counter Value."]
    #[inline(always)]
    pub const fn TSCV(self) -> crate::common::Reg<regs::TSCV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timeout Counter Configuration."]
    #[inline(always)]
    pub const fn TOCC(self) -> crate::common::Reg<regs::TOCC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Timeout Counter Value."]
    #[inline(always)]
    pub const fn TOCV(self) -> crate::common::Reg<regs::TOCV, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Error Counter Register."]
    #[inline(always)]
    pub const fn ECR(self) -> crate::common::Reg<regs::ECR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Protocol Status Register."]
    #[inline(always)]
    pub const fn PSR(self) -> crate::common::Reg<regs::PSR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Transmitter Delay Compensator Register."]
    #[inline(always)]
    pub const fn TDCR(self) -> crate::common::Reg<regs::TDCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Register."]
    #[inline(always)]
    pub const fn IR(self) -> crate::common::Reg<regs::IR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn IE(self) -> crate::common::Reg<regs::IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Interrupt Line Select."]
    #[inline(always)]
    pub const fn ILS(self) -> crate::common::Reg<regs::ILS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Interrupt Line Enable."]
    #[inline(always)]
    pub const fn ILE(self) -> crate::common::Reg<regs::ILE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Global Filter Configuration."]
    #[inline(always)]
    pub const fn GFC(self) -> crate::common::Reg<regs::GFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Standard ID Filter Configuration."]
    #[inline(always)]
    pub const fn SIDFC(self) -> crate::common::Reg<regs::SIDFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Extended ID Filter Configuration."]
    #[inline(always)]
    pub const fn XIDFC(self) -> crate::common::Reg<regs::XIDFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Extended ID AND Mask."]
    #[inline(always)]
    pub const fn XIDAM(self) -> crate::common::Reg<regs::XIDAM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "High Priority Message Status."]
    #[inline(always)]
    pub const fn HPMS(self) -> crate::common::Reg<regs::HPMS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "New Data 1."]
    #[inline(always)]
    pub const fn NDAT1(self) -> crate::common::Reg<regs::NDAT1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "New Data 2."]
    #[inline(always)]
    pub const fn NDAT2(self) -> crate::common::Reg<regs::NDAT2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Rx FIFO 0 Configuration."]
    #[inline(always)]
    pub const fn RXF0C(self) -> crate::common::Reg<regs::RXF0C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Rx FIFO 0 Status."]
    #[inline(always)]
    pub const fn RXF0S(self) -> crate::common::Reg<regs::RXF0S, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Rx FIFO 0 Acknowledge."]
    #[inline(always)]
    pub const fn RXF0A(self) -> crate::common::Reg<regs::RXF0A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Rx Buffer Configuration."]
    #[inline(always)]
    pub const fn RXBC(self) -> crate::common::Reg<regs::RXBC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Rx FIFO 1 Configuration."]
    #[inline(always)]
    pub const fn RXF1C(self) -> crate::common::Reg<regs::RXF1C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Rx FIFO 1 Status."]
    #[inline(always)]
    pub const fn RXF1S(self) -> crate::common::Reg<regs::RXF1S, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Rx FIFO 1 Acknowledge."]
    #[inline(always)]
    pub const fn RXF1A(self) -> crate::common::Reg<regs::RXF1A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Rx Buffer and FIFO Element Size Configuration."]
    #[inline(always)]
    pub const fn RXESC(self) -> crate::common::Reg<regs::RXESC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Tx Buffer Configuration."]
    #[inline(always)]
    pub const fn TXBC(self) -> crate::common::Reg<regs::TXBC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Tx FIFO/Queue Status."]
    #[inline(always)]
    pub const fn TXFQS(self) -> crate::common::Reg<regs::TXFQS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Tx Buffer Element Size Configuration."]
    #[inline(always)]
    pub const fn TXESC(self) -> crate::common::Reg<regs::TXESC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Tx Buffer Request Pending."]
    #[inline(always)]
    pub const fn TXBRP(self) -> crate::common::Reg<regs::TXBRP, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Tx Buffer Add Request."]
    #[inline(always)]
    pub const fn TXBAR(self) -> crate::common::Reg<regs::TXBAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Request."]
    #[inline(always)]
    pub const fn TXBCR(self) -> crate::common::Reg<regs::TXBCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Tx Buffer Transmission Occurred."]
    #[inline(always)]
    pub const fn TXBTO(self) -> crate::common::Reg<regs::TXBTO, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished."]
    #[inline(always)]
    pub const fn TXBCF(self) -> crate::common::Reg<regs::TXBCF, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Tx Buffer Transmission Interrupt Enable."]
    #[inline(always)]
    pub const fn TXBTIE(self) -> crate::common::Reg<regs::TXBTIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Tx Buffer Cancellation Finished Interrupt Enable."]
    #[inline(always)]
    pub const fn TXBCIE(self) -> crate::common::Reg<regs::TXBCIE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Tx Event FIFO Configuration."]
    #[inline(always)]
    pub const fn TXEFC(self) -> crate::common::Reg<regs::TXEFC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Tx Event FIFO Status."]
    #[inline(always)]
    pub const fn TXEFS(self) -> crate::common::Reg<regs::TXEFS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Tx Event FIFO Acknowledge."]
    #[inline(always)]
    pub const fn TXEFA(self) -> crate::common::Reg<regs::TXEFA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "CAN Message RAM Base Address."]
    #[inline(always)]
    pub const fn MRBA(self) -> crate::common::Reg<regs::MRBA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "External Timestamp Counter Configuration."]
    #[inline(always)]
    pub const fn ETSCC(self) -> crate::common::Reg<regs::ETSCC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "External Timestamp Counter Value."]
    #[inline(always)]
    pub const fn ETSCV(self) -> crate::common::Reg<regs::ETSCV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
}
pub mod regs;
