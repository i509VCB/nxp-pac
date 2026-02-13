#[doc = "ENET"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enet0 {
    ptr: *mut u8,
}
unsafe impl Send for Enet0 {}
unsafe impl Sync for Enet0 {}
impl Enet0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MAC Configuration"]
    #[inline(always)]
    pub const fn mac_configuration(
        self,
    ) -> crate::common::Reg<regs::MacConfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC Extended Configuration Register"]
    #[inline(always)]
    pub const fn mac_ext_configuration(
        self,
    ) -> crate::common::Reg<regs::MacExtConfiguration, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MAC Packet Filter"]
    #[inline(always)]
    pub const fn mac_packet_filter(
        self,
    ) -> crate::common::Reg<regs::MacPacketFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog Timeout"]
    #[inline(always)]
    pub const fn mac_watchdog_timeout(
        self,
    ) -> crate::common::Reg<regs::MacWatchdogTimeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MAC VLAN Tag Control"]
    #[inline(always)]
    pub const fn mac_vlan_tag_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacVlanTagCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "VLAN Tag Inclusion or Replacement"]
    #[inline(always)]
    pub const fn mac_vlan_incl(self) -> crate::common::Reg<regs::MacVlanIncl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
    #[inline(always)]
    pub const fn mac_inner_vlan_incl(
        self,
    ) -> crate::common::Reg<regs::MacInnerVlanIncl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MAC Q0 Tx Flow Control"]
    #[inline(always)]
    pub const fn mac_q0_tx_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacQ0TxFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "MAC Rx Flow Control"]
    #[inline(always)]
    pub const fn mac_rx_flow_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacRxFlowCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Receive Queue Control 4"]
    #[inline(always)]
    pub const fn mac_rxq_ctrl4(self) -> crate::common::Reg<regs::MacRxqCtrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Receive Queue Control 0"]
    #[inline(always)]
    pub const fn mac_rxq_ctrl0(self) -> crate::common::Reg<regs::MacRxqCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Receive Queue Control 1"]
    #[inline(always)]
    pub const fn mac_rxq_ctrl1(self) -> crate::common::Reg<regs::MacRxqCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Receive Queue Control 2"]
    #[inline(always)]
    pub const fn mac_rxq_ctrl2(self) -> crate::common::Reg<regs::MacRxqCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn mac_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::MacInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn mac_interrupt_enable(
        self,
    ) -> crate::common::Reg<regs::MacInterruptEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Receive Transmit Status"]
    #[inline(always)]
    pub const fn mac_rx_tx_status(
        self,
    ) -> crate::common::Reg<regs::MacRxTxStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "PMT Control and Status"]
    #[inline(always)]
    pub const fn mac_pmt_control_status(
        self,
    ) -> crate::common::Reg<regs::MacPmtControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Remote Wakeup Filter"]
    #[inline(always)]
    pub const fn mac_rwk_packet_filter(
        self,
    ) -> crate::common::Reg<regs::MacRwkPacketFilter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "LPI Control and Status"]
    #[inline(always)]
    pub const fn mac_lpi_control_status(
        self,
    ) -> crate::common::Reg<regs::MacLpiControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "LPI Timers Control"]
    #[inline(always)]
    pub const fn mac_lpi_timers_control(
        self,
    ) -> crate::common::Reg<regs::MacLpiTimersControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Tx LPI Entry Timer Control"]
    #[inline(always)]
    pub const fn mac_lpi_entry_timer(
        self,
    ) -> crate::common::Reg<regs::MacLpiEntryTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "One-microsecond Reference Timer"]
    #[inline(always)]
    pub const fn mac_oneus_tic_counter(
        self,
    ) -> crate::common::Reg<regs::MacOneusTicCounter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "MAC Version"]
    #[inline(always)]
    pub const fn mac_version(self) -> crate::common::Reg<regs::MacVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "MAC Debug"]
    #[inline(always)]
    pub const fn mac_debug(self) -> crate::common::Reg<regs::MacDebug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Hardware Features 0"]
    #[inline(always)]
    pub const fn mac_hw_feature0(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Hardware Features 1"]
    #[inline(always)]
    pub const fn mac_hw_feature1(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Hardware Features 2"]
    #[inline(always)]
    pub const fn mac_hw_feature2(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Hardware Features 3"]
    #[inline(always)]
    pub const fn mac_hw_feature3(
        self,
    ) -> crate::common::Reg<regs::MacHwFeature3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "MDIO Address"]
    #[inline(always)]
    pub const fn mac_mdio_address(
        self,
    ) -> crate::common::Reg<regs::MacMdioAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MAC MDIO Data"]
    #[inline(always)]
    pub const fn mac_mdio_data(self) -> crate::common::Reg<regs::MacMdioData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "CSR Software Control"]
    #[inline(always)]
    pub const fn mac_csr_sw_ctrl(
        self,
    ) -> crate::common::Reg<regs::MacCsrSwCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "MAC Address0 High"]
    #[inline(always)]
    pub const fn mac_address0_high(
        self,
    ) -> crate::common::Reg<regs::MacAddress0High, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "MAC Address0 Low"]
    #[inline(always)]
    pub const fn mac_address0_low(
        self,
    ) -> crate::common::Reg<regs::MacAddress0Low, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Indirect Access Control"]
    #[inline(always)]
    pub const fn indir_access_ctrl(
        self,
    ) -> crate::common::Reg<regs::IndirAccessCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a70usize) as _) }
    }
    #[doc = "Indirect Access Data"]
    #[inline(always)]
    pub const fn indir_access_data(
        self,
    ) -> crate::common::Reg<regs::IndirAccessData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a74usize) as _) }
    }
    #[doc = "Timestamp Control"]
    #[inline(always)]
    pub const fn mac_timestamp_control(
        self,
    ) -> crate::common::Reg<regs::MacTimestampControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Subsecond Increment"]
    #[inline(always)]
    pub const fn mac_sub_second_increment(
        self,
    ) -> crate::common::Reg<regs::MacSubSecondIncrement, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "System Time Seconds"]
    #[inline(always)]
    pub const fn mac_system_time_seconds(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeSeconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "System Time Nanoseconds"]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeNanoseconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "System Time Seconds Update"]
    #[inline(always)]
    pub const fn mac_system_time_seconds_update(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeSecondsUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "System Time Nanoseconds Update"]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds_update(
        self,
    ) -> crate::common::Reg<regs::MacSystemTimeNanosecondsUpdate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Timestamp Addend"]
    #[inline(always)]
    pub const fn mac_timestamp_addend(
        self,
    ) -> crate::common::Reg<regs::MacTimestampAddend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Timestamp Status"]
    #[inline(always)]
    pub const fn mac_timestamp_status(
        self,
    ) -> crate::common::Reg<regs::MacTimestampStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Nanoseconds"]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::MacTxTimestampStatusNanoseconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Seconds"]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_seconds(
        self,
    ) -> crate::common::Reg<regs::MacTxTimestampStatusSeconds, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "Timestamp Ingress Correction Nanosecond"]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_corr_nanosecond(
        self,
    ) -> crate::common::Reg<regs::MacTimestampIngressCorrNanosecond, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "Timestamp Egress Correction Nanosecond"]
    #[inline(always)]
    pub const fn mac_timestamp_egress_corr_nanosecond(
        self,
    ) -> crate::common::Reg<regs::MacTimestampEgressCorrNanosecond, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "Timestamp Ingress Latency"]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_latency(
        self,
    ) -> crate::common::Reg<regs::MacTimestampIngressLatency, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Timestamp Egress Latency"]
    #[inline(always)]
    pub const fn mac_timestamp_egress_latency(
        self,
    ) -> crate::common::Reg<regs::MacTimestampEgressLatency, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "PPS Control"]
    #[inline(always)]
    pub const fn mac_pps_control(
        self,
    ) -> crate::common::Reg<regs::MacPpsControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[doc = "PPS0 Target Time Seconds"]
    #[inline(always)]
    pub const fn pps0_target_time_seconds(
        self,
    ) -> crate::common::Reg<regs::Pps0TargetTimeSeconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[doc = "PPS0 Target Time Nanoseconds"]
    #[inline(always)]
    pub const fn pps0_target_time_nanoseconds(
        self,
    ) -> crate::common::Reg<regs::Pps0TargetTimeNanoseconds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b84usize) as _) }
    }
    #[doc = "MTL Operation Mode"]
    #[inline(always)]
    pub const fn mtl_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlOperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "MTL Interrupt Status"]
    #[inline(always)]
    pub const fn mtl_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::MtlInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c20usize) as _) }
    }
    #[doc = "Receive Queue and DMA Channel Mapping 0"]
    #[inline(always)]
    pub const fn mtl_rxq_dma_map0(
        self,
    ) -> crate::common::Reg<regs::MtlRxqDmaMap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c30usize) as _) }
    }
    #[doc = "Queue 0 Transmit Operation Mode"]
    #[inline(always)]
    pub const fn mtl_txq0_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlTxq0OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[doc = "Queue 0 Underflow Counter"]
    #[inline(always)]
    pub const fn mtl_txq0_underflow(
        self,
    ) -> crate::common::Reg<regs::MtlTxq0Underflow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[doc = "Queue 0 Transmit Debug"]
    #[inline(always)]
    pub const fn mtl_txq0_debug(self) -> crate::common::Reg<regs::MtlTxq0Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[doc = "Queue 0 ETS Status"]
    #[inline(always)]
    pub const fn mtl_txq0_ets_status(
        self,
    ) -> crate::common::Reg<regs::MtlTxq0EtsStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d14usize) as _) }
    }
    #[doc = "Queue 0 Quantum or Weights"]
    #[inline(always)]
    pub const fn mtl_txq0_quantum_weight(
        self,
    ) -> crate::common::Reg<regs::MtlTxq0QuantumWeight, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d18usize) as _) }
    }
    #[doc = "Queue 0 Interrupt Control Status"]
    #[inline(always)]
    pub const fn mtl_q0_interrupt_control_status(
        self,
    ) -> crate::common::Reg<regs::MtlQ0InterruptControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d2cusize) as _) }
    }
    #[doc = "Queue 0 Receive Operation Mode"]
    #[inline(always)]
    pub const fn mtl_rxq0_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlRxq0OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[doc = "Queue 0 Missed Packet and Overflow Counter"]
    #[inline(always)]
    pub const fn mtl_rxq0_missed_packet_overflow_cnt(
        self,
    ) -> crate::common::Reg<regs::MtlRxq0MissedPacketOverflowCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d34usize) as _) }
    }
    #[doc = "Queue 0 Receive Debug"]
    #[inline(always)]
    pub const fn mtl_rxq0_debug(self) -> crate::common::Reg<regs::MtlRxq0Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d38usize) as _) }
    }
    #[doc = "Queue 0 Receive Control"]
    #[inline(always)]
    pub const fn mtl_rxq0_control(
        self,
    ) -> crate::common::Reg<regs::MtlRxq0Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d3cusize) as _) }
    }
    #[doc = "Queue 1 Transmit Operation Mode"]
    #[inline(always)]
    pub const fn mtl_txq1_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d40usize) as _) }
    }
    #[doc = "Queue 1 Underflow Counter"]
    #[inline(always)]
    pub const fn mtl_txq1_underflow(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1Underflow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d44usize) as _) }
    }
    #[doc = "Queue 1 Transmit Debug"]
    #[inline(always)]
    pub const fn mtl_txq1_debug(self) -> crate::common::Reg<regs::MtlTxq1Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d48usize) as _) }
    }
    #[doc = "Queue 1 ETS Control"]
    #[inline(always)]
    pub const fn mtl_txq1_ets_control(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1EtsControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d50usize) as _) }
    }
    #[doc = "Queue 1 ETS Status"]
    #[inline(always)]
    pub const fn mtl_txq1_ets_status(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1EtsStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d54usize) as _) }
    }
    #[doc = "Queue 1 idleSlopeCredit, Quantum or Weights"]
    #[inline(always)]
    pub const fn mtl_txq1_quantum_weight(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1QuantumWeight, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d58usize) as _) }
    }
    #[doc = "Queue 1 sendSlopeCredit"]
    #[inline(always)]
    pub const fn mtl_txq1_sendslopecredit(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1Sendslopecredit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d5cusize) as _) }
    }
    #[doc = "Queue 1 hiCredit"]
    #[inline(always)]
    pub const fn mtl_txq1_hicredit(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1Hicredit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d60usize) as _) }
    }
    #[doc = "Queue 1 loCredit"]
    #[inline(always)]
    pub const fn mtl_txq1_locredit(
        self,
    ) -> crate::common::Reg<regs::MtlTxq1Locredit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d64usize) as _) }
    }
    #[doc = "Queue 1 Interrupt Control Status"]
    #[inline(always)]
    pub const fn mtl_q1_interrupt_control_status(
        self,
    ) -> crate::common::Reg<regs::MtlQ1InterruptControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d6cusize) as _) }
    }
    #[doc = "Queue 1 Receive Operation Mode"]
    #[inline(always)]
    pub const fn mtl_rxq1_operation_mode(
        self,
    ) -> crate::common::Reg<regs::MtlRxq1OperationMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d70usize) as _) }
    }
    #[doc = "Queue 1 Missed Packet and Overflow Counter"]
    #[inline(always)]
    pub const fn mtl_rxq1_missed_packet_overflow_cnt(
        self,
    ) -> crate::common::Reg<regs::MtlRxq1MissedPacketOverflowCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d74usize) as _) }
    }
    #[doc = "Queue 1 Receive Debug"]
    #[inline(always)]
    pub const fn mtl_rxq1_debug(self) -> crate::common::Reg<regs::MtlRxq1Debug, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d78usize) as _) }
    }
    #[doc = "Queue 1 Receive Control"]
    #[inline(always)]
    pub const fn mtl_rxq1_control(
        self,
    ) -> crate::common::Reg<regs::MtlRxq1Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d7cusize) as _) }
    }
    #[doc = "DMA Bus Mode"]
    #[inline(always)]
    pub const fn dma_mode(self) -> crate::common::Reg<regs::DmaMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "DMA System Bus Mode"]
    #[inline(always)]
    pub const fn dma_sysbus_mode(
        self,
    ) -> crate::common::Reg<regs::DmaSysbusMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "DMA Interrupt Status"]
    #[inline(always)]
    pub const fn dma_interrupt_status(
        self,
    ) -> crate::common::Reg<regs::DmaInterruptStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "DMA Debug Status 0"]
    #[inline(always)]
    pub const fn dma_debug_status0(
        self,
    ) -> crate::common::Reg<regs::DmaDebugStatus0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "DMA Channel 0 Control"]
    #[inline(always)]
    pub const fn dma_ch0_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "DMA Channel 0 Transmit Control"]
    #[inline(always)]
    pub const fn dma_ch0_tx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "DMA Channel 0 Receive Control"]
    #[inline(always)]
    pub const fn dma_ch0_rx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor List Address register"]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxdescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "Channel 0 Rx Descriptor List Address register"]
    #[inline(always)]
    pub const fn dma_ch0_rxdesc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxdescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxdescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "Channel 0 Rx Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch0_rxdesc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxdescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor Ring Length"]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_ring_length(
        self,
    ) -> crate::common::Reg<regs::DmaCh0TxdescRingLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "Channeli Receive Control"]
    #[inline(always)]
    pub const fn dma_ch0_rx_control2(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxControl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Channeli Interrupt Enable"]
    #[inline(always)]
    pub const fn dma_ch0_interrupt_enable(
        self,
    ) -> crate::common::Reg<regs::DmaCh0InterruptEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
    #[inline(always)]
    pub const fn dma_ch0_rx_interrupt_watchdog_timer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxInterruptWatchdogTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "Channel 0 Slot Function Control and Status"]
    #[inline(always)]
    pub const fn dma_ch0_slot_function_control_status(
        self,
    ) -> crate::common::Reg<regs::DmaCh0SlotFunctionControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Descriptor"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_txdesc(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppTxdesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Descriptor"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rxdesc(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppRxdesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_txbuffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppTxbuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rxbuffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh0CurrentAppRxbuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "DMA Channel 0 Status"]
    #[inline(always)]
    pub const fn dma_ch0_status(self) -> crate::common::Reg<regs::DmaCh0Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "Channel 0 Missed Frame Counter"]
    #[inline(always)]
    pub const fn dma_ch0_miss_frame_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh0MissFrameCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "Channel 0 Receive ERI Counter"]
    #[inline(always)]
    pub const fn dma_ch0_rx_eri_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh0RxEriCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "DMA Channel 1 Control"]
    #[inline(always)]
    pub const fn dma_ch1_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh1Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "DMA Channel 1 Transmit Control"]
    #[inline(always)]
    pub const fn dma_ch1_tx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh1TxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "DMA Channel 1 Receive Control"]
    #[inline(always)]
    pub const fn dma_ch1_rx_control(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor List Address"]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh1TxdescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "Channel 1 Rx Descriptor List Address"]
    #[inline(always)]
    pub const fn dma_ch1_rxdesc_list_address(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxdescListAddress, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh1TxdescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "Channel 1 Rx Descriptor Tail Pointer"]
    #[inline(always)]
    pub const fn dma_ch1_rxdesc_tail_pointer(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxdescTailPointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor Ring Length"]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_ring_length(
        self,
    ) -> crate::common::Reg<regs::DmaCh1TxdescRingLength, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "DMA Channel 1 Receive Control"]
    #[inline(always)]
    pub const fn dma_ch1_rx_control2(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxControl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub const fn dma_ch1_interrupt_enable(
        self,
    ) -> crate::common::Reg<regs::DmaCh1InterruptEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "Channel 1 Receive Interrupt Watchdog Timer"]
    #[inline(always)]
    pub const fn dma_ch1_rx_interrupt_watchdog_timer(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxInterruptWatchdogTimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "Channel 1 Slot Function Control and Status"]
    #[inline(always)]
    pub const fn dma_ch1_slot_function_control_status(
        self,
    ) -> crate::common::Reg<regs::DmaCh1SlotFunctionControlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "Channel 1 Current Application Transmit Descriptor"]
    #[inline(always)]
    pub const fn dma_ch1_current_app_txdesc(
        self,
    ) -> crate::common::Reg<regs::DmaCh1CurrentAppTxdesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "Channel 1 Current Application Receive Descriptor"]
    #[inline(always)]
    pub const fn dma_ch1_current_app_rxdesc(
        self,
    ) -> crate::common::Reg<regs::DmaCh1CurrentAppRxdesc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "Channel 1 Current Application Transmit Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch1_current_app_txbuffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh1CurrentAppTxbuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "Channel 1 Current Application Receive Buffer Address"]
    #[inline(always)]
    pub const fn dma_ch1_current_app_rxbuffer(
        self,
    ) -> crate::common::Reg<regs::DmaCh1CurrentAppRxbuffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "DMA Channel 1 Status"]
    #[inline(always)]
    pub const fn dma_ch1_status(self) -> crate::common::Reg<regs::DmaCh1Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "Channel 1 Missed Frame Counter"]
    #[inline(always)]
    pub const fn dma_ch1_miss_frame_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh1MissFrameCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "Channel 1 Receive ERI Counter"]
    #[inline(always)]
    pub const fn dma_ch1_rx_eri_cnt(
        self,
    ) -> crate::common::Reg<regs::DmaCh1RxEriCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
}
pub mod regs;
pub mod vals;
