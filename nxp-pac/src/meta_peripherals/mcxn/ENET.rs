#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "ENET."]
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
    #[doc = "MAC Configuration."]
    #[inline(always)]
    pub const fn mac_configuration(
        self,
    ) -> crate::pac::common::Reg<MacConfiguration, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MAC Extended Configuration Register."]
    #[inline(always)]
    pub const fn mac_ext_configuration(
        self,
    ) -> crate::pac::common::Reg<MacExtConfiguration, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MAC Packet Filter."]
    #[inline(always)]
    pub const fn mac_packet_filter(
        self,
    ) -> crate::pac::common::Reg<MacPacketFilter, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Watchdog Timeout."]
    #[inline(always)]
    pub const fn mac_watchdog_timeout(
        self,
    ) -> crate::pac::common::Reg<MacWatchdogTimeout, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MAC VLAN Tag Control."]
    #[inline(always)]
    pub const fn mac_vlan_tag_ctrl(
        self,
    ) -> crate::pac::common::Reg<MacVlanTagCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "VLAN Tag Inclusion or Replacement."]
    #[inline(always)]
    pub const fn mac_vlan_incl(
        self,
    ) -> crate::pac::common::Reg<MacVlanIncl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "MAC Inner VLAN Tag Inclusion or Replacement."]
    #[inline(always)]
    pub const fn mac_inner_vlan_incl(
        self,
    ) -> crate::pac::common::Reg<MacInnerVlanIncl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MAC Q0 Tx Flow Control."]
    #[inline(always)]
    pub const fn mac_q0_tx_flow_ctrl(
        self,
    ) -> crate::pac::common::Reg<MacQ0TxFlowCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "MAC Rx Flow Control."]
    #[inline(always)]
    pub const fn mac_rx_flow_ctrl(
        self,
    ) -> crate::pac::common::Reg<MacRxFlowCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Receive Queue Control 4."]
    #[inline(always)]
    pub const fn mac_rxq_ctrl4(
        self,
    ) -> crate::pac::common::Reg<MacRxqCtrl4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Receive Queue Control 0."]
    #[inline(always)]
    pub const fn mac_rxq_ctrl0(
        self,
    ) -> crate::pac::common::Reg<MacRxqCtrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Receive Queue Control 1."]
    #[inline(always)]
    pub const fn mac_rxq_ctrl1(
        self,
    ) -> crate::pac::common::Reg<MacRxqCtrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Receive Queue Control 2."]
    #[inline(always)]
    pub const fn mac_rxq_ctrl2(
        self,
    ) -> crate::pac::common::Reg<MacRxqCtrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn mac_interrupt_status(
        self,
    ) -> crate::pac::common::Reg<MacInterruptStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn mac_interrupt_enable(
        self,
    ) -> crate::pac::common::Reg<MacInterruptEnable, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Receive Transmit Status."]
    #[inline(always)]
    pub const fn mac_rx_tx_status(
        self,
    ) -> crate::pac::common::Reg<MacRxTxStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "PMT Control and Status."]
    #[inline(always)]
    pub const fn mac_pmt_control_status(
        self,
    ) -> crate::pac::common::Reg<MacPmtControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Remote Wakeup Filter."]
    #[inline(always)]
    pub const fn mac_rwk_packet_filter(
        self,
    ) -> crate::pac::common::Reg<MacRwkPacketFilter, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "LPI Control and Status."]
    #[inline(always)]
    pub const fn mac_lpi_control_status(
        self,
    ) -> crate::pac::common::Reg<MacLpiControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "LPI Timers Control."]
    #[inline(always)]
    pub const fn mac_lpi_timers_control(
        self,
    ) -> crate::pac::common::Reg<MacLpiTimersControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Tx LPI Entry Timer Control."]
    #[inline(always)]
    pub const fn mac_lpi_entry_timer(
        self,
    ) -> crate::pac::common::Reg<MacLpiEntryTimer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "One-microsecond Reference Timer."]
    #[inline(always)]
    pub const fn mac_oneus_tic_counter(
        self,
    ) -> crate::pac::common::Reg<MacOneusTicCounter, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "MAC Version."]
    #[inline(always)]
    pub const fn mac_version(self) -> crate::pac::common::Reg<MacVersion, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "MAC Debug."]
    #[inline(always)]
    pub const fn mac_debug(self) -> crate::pac::common::Reg<MacDebug, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Hardware Features 0."]
    #[inline(always)]
    pub const fn mac_hw_feature0(
        self,
    ) -> crate::pac::common::Reg<MacHwFeature0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Hardware Features 1."]
    #[inline(always)]
    pub const fn mac_hw_feature1(
        self,
    ) -> crate::pac::common::Reg<MacHwFeature1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Hardware Features 2."]
    #[inline(always)]
    pub const fn mac_hw_feature2(
        self,
    ) -> crate::pac::common::Reg<MacHwFeature2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Hardware Features 3."]
    #[inline(always)]
    pub const fn mac_hw_feature3(
        self,
    ) -> crate::pac::common::Reg<MacHwFeature3, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "MDIO Address."]
    #[inline(always)]
    pub const fn mac_mdio_address(
        self,
    ) -> crate::pac::common::Reg<MacMdioAddress, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MAC MDIO Data."]
    #[inline(always)]
    pub const fn mac_mdio_data(
        self,
    ) -> crate::pac::common::Reg<MacMdioData, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "CSR Software Control."]
    #[inline(always)]
    pub const fn mac_csr_sw_ctrl(
        self,
    ) -> crate::pac::common::Reg<MacCsrSwCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "MAC Address0 High."]
    #[inline(always)]
    pub const fn mac_address0_high(
        self,
    ) -> crate::pac::common::Reg<MacAddress0High, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "MAC Address0 Low."]
    #[inline(always)]
    pub const fn mac_address0_low(
        self,
    ) -> crate::pac::common::Reg<MacAddress0Low, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Indirect Access Control."]
    #[inline(always)]
    pub const fn indir_access_ctrl(
        self,
    ) -> crate::pac::common::Reg<IndirAccessCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a70usize) as _) }
    }
    #[doc = "Indirect Access Data."]
    #[inline(always)]
    pub const fn indir_access_data(
        self,
    ) -> crate::pac::common::Reg<IndirAccessData, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a74usize) as _) }
    }
    #[doc = "Timestamp Control."]
    #[inline(always)]
    pub const fn mac_timestamp_control(
        self,
    ) -> crate::pac::common::Reg<MacTimestampControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Subsecond Increment."]
    #[inline(always)]
    pub const fn mac_sub_second_increment(
        self,
    ) -> crate::pac::common::Reg<MacSubSecondIncrement, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b04usize) as _) }
    }
    #[doc = "System Time Seconds."]
    #[inline(always)]
    pub const fn mac_system_time_seconds(
        self,
    ) -> crate::pac::common::Reg<MacSystemTimeSeconds, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b08usize) as _) }
    }
    #[doc = "System Time Nanoseconds."]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds(
        self,
    ) -> crate::pac::common::Reg<MacSystemTimeNanoseconds, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b0cusize) as _) }
    }
    #[doc = "System Time Seconds Update."]
    #[inline(always)]
    pub const fn mac_system_time_seconds_update(
        self,
    ) -> crate::pac::common::Reg<MacSystemTimeSecondsUpdate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b10usize) as _) }
    }
    #[doc = "System Time Nanoseconds Update."]
    #[inline(always)]
    pub const fn mac_system_time_nanoseconds_update(
        self,
    ) -> crate::pac::common::Reg<MacSystemTimeNanosecondsUpdate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b14usize) as _) }
    }
    #[doc = "Timestamp Addend."]
    #[inline(always)]
    pub const fn mac_timestamp_addend(
        self,
    ) -> crate::pac::common::Reg<MacTimestampAddend, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b18usize) as _) }
    }
    #[doc = "Timestamp Status."]
    #[inline(always)]
    pub const fn mac_timestamp_status(
        self,
    ) -> crate::pac::common::Reg<MacTimestampStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Nanoseconds."]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_nanoseconds(
        self,
    ) -> crate::pac::common::Reg<MacTxTimestampStatusNanoseconds, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Transmit Timestamp Status Seconds."]
    #[inline(always)]
    pub const fn mac_tx_timestamp_status_seconds(
        self,
    ) -> crate::pac::common::Reg<MacTxTimestampStatusSeconds, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "Timestamp Ingress Correction Nanosecond."]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_corr_nanosecond(
        self,
    ) -> crate::pac::common::Reg<MacTimestampIngressCorrNanosecond, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "Timestamp Egress Correction Nanosecond."]
    #[inline(always)]
    pub const fn mac_timestamp_egress_corr_nanosecond(
        self,
    ) -> crate::pac::common::Reg<MacTimestampEgressCorrNanosecond, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "Timestamp Ingress Latency."]
    #[inline(always)]
    pub const fn mac_timestamp_ingress_latency(
        self,
    ) -> crate::pac::common::Reg<MacTimestampIngressLatency, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Timestamp Egress Latency."]
    #[inline(always)]
    pub const fn mac_timestamp_egress_latency(
        self,
    ) -> crate::pac::common::Reg<MacTimestampEgressLatency, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "PPS Control."]
    #[inline(always)]
    pub const fn mac_pps_control(
        self,
    ) -> crate::pac::common::Reg<MacPpsControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b70usize) as _) }
    }
    #[doc = "PPS0 Target Time Seconds."]
    #[inline(always)]
    pub const fn pps0_target_time_seconds(
        self,
    ) -> crate::pac::common::Reg<Pps0TargetTimeSeconds, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[doc = "PPS0 Target Time Nanoseconds."]
    #[inline(always)]
    pub const fn pps0_target_time_nanoseconds(
        self,
    ) -> crate::pac::common::Reg<Pps0TargetTimeNanoseconds, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b84usize) as _) }
    }
    #[doc = "MTL Operation Mode."]
    #[inline(always)]
    pub const fn mtl_operation_mode(
        self,
    ) -> crate::pac::common::Reg<MtlOperationMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "MTL Interrupt Status."]
    #[inline(always)]
    pub const fn mtl_interrupt_status(
        self,
    ) -> crate::pac::common::Reg<MtlInterruptStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c20usize) as _) }
    }
    #[doc = "Receive Queue and DMA Channel Mapping 0."]
    #[inline(always)]
    pub const fn mtl_rxq_dma_map0(
        self,
    ) -> crate::pac::common::Reg<MtlRxqDmaMap0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c30usize) as _) }
    }
    #[doc = "Queue 0 Transmit Operation Mode."]
    #[inline(always)]
    pub const fn mtl_txq0_operation_mode(
        self,
    ) -> crate::pac::common::Reg<MtlTxq0OperationMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d00usize) as _) }
    }
    #[doc = "Queue 0 Underflow Counter."]
    #[inline(always)]
    pub const fn mtl_txq0_underflow(
        self,
    ) -> crate::pac::common::Reg<MtlTxq0Underflow, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d04usize) as _) }
    }
    #[doc = "Queue 0 Transmit Debug."]
    #[inline(always)]
    pub const fn mtl_txq0_debug(
        self,
    ) -> crate::pac::common::Reg<MtlTxq0Debug, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d08usize) as _) }
    }
    #[doc = "Queue 0 ETS Status."]
    #[inline(always)]
    pub const fn mtl_txq0_ets_status(
        self,
    ) -> crate::pac::common::Reg<MtlTxq0EtsStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d14usize) as _) }
    }
    #[doc = "Queue 0 Quantum or Weights."]
    #[inline(always)]
    pub const fn mtl_txq0_quantum_weight(
        self,
    ) -> crate::pac::common::Reg<MtlTxq0QuantumWeight, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d18usize) as _) }
    }
    #[doc = "Queue 0 Interrupt Control Status."]
    #[inline(always)]
    pub const fn mtl_q0_interrupt_control_status(
        self,
    ) -> crate::pac::common::Reg<MtlQ0InterruptControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d2cusize) as _) }
    }
    #[doc = "Queue 0 Receive Operation Mode."]
    #[inline(always)]
    pub const fn mtl_rxq0_operation_mode(
        self,
    ) -> crate::pac::common::Reg<MtlRxq0OperationMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d30usize) as _) }
    }
    #[doc = "Queue 0 Missed Packet and Overflow Counter."]
    #[inline(always)]
    pub const fn mtl_rxq0_missed_packet_overflow_cnt(
        self,
    ) -> crate::pac::common::Reg<MtlRxq0MissedPacketOverflowCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d34usize) as _) }
    }
    #[doc = "Queue 0 Receive Debug."]
    #[inline(always)]
    pub const fn mtl_rxq0_debug(
        self,
    ) -> crate::pac::common::Reg<MtlRxq0Debug, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d38usize) as _) }
    }
    #[doc = "Queue 0 Receive Control."]
    #[inline(always)]
    pub const fn mtl_rxq0_control(
        self,
    ) -> crate::pac::common::Reg<MtlRxq0Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d3cusize) as _) }
    }
    #[doc = "Queue 1 Transmit Operation Mode."]
    #[inline(always)]
    pub const fn mtl_txq1_operation_mode(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1OperationMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d40usize) as _) }
    }
    #[doc = "Queue 1 Underflow Counter."]
    #[inline(always)]
    pub const fn mtl_txq1_underflow(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1Underflow, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d44usize) as _) }
    }
    #[doc = "Queue 1 Transmit Debug."]
    #[inline(always)]
    pub const fn mtl_txq1_debug(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1Debug, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d48usize) as _) }
    }
    #[doc = "Queue 1 ETS Control."]
    #[inline(always)]
    pub const fn mtl_txq1_ets_control(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1EtsControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d50usize) as _) }
    }
    #[doc = "Queue 1 ETS Status."]
    #[inline(always)]
    pub const fn mtl_txq1_ets_status(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1EtsStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d54usize) as _) }
    }
    #[doc = "Queue 1 idleSlopeCredit, Quantum or Weights."]
    #[inline(always)]
    pub const fn mtl_txq1_quantum_weight(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1QuantumWeight, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d58usize) as _) }
    }
    #[doc = "Queue 1 sendSlopeCredit."]
    #[inline(always)]
    pub const fn mtl_txq1_sendslopecredit(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1Sendslopecredit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d5cusize) as _) }
    }
    #[doc = "Queue 1 hiCredit."]
    #[inline(always)]
    pub const fn mtl_txq1_hicredit(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1Hicredit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d60usize) as _) }
    }
    #[doc = "Queue 1 loCredit."]
    #[inline(always)]
    pub const fn mtl_txq1_locredit(
        self,
    ) -> crate::pac::common::Reg<MtlTxq1Locredit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d64usize) as _) }
    }
    #[doc = "Queue 1 Interrupt Control Status."]
    #[inline(always)]
    pub const fn mtl_q1_interrupt_control_status(
        self,
    ) -> crate::pac::common::Reg<MtlQ1InterruptControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d6cusize) as _) }
    }
    #[doc = "Queue 1 Receive Operation Mode."]
    #[inline(always)]
    pub const fn mtl_rxq1_operation_mode(
        self,
    ) -> crate::pac::common::Reg<MtlRxq1OperationMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d70usize) as _) }
    }
    #[doc = "Queue 1 Missed Packet and Overflow Counter."]
    #[inline(always)]
    pub const fn mtl_rxq1_missed_packet_overflow_cnt(
        self,
    ) -> crate::pac::common::Reg<MtlRxq1MissedPacketOverflowCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d74usize) as _) }
    }
    #[doc = "Queue 1 Receive Debug."]
    #[inline(always)]
    pub const fn mtl_rxq1_debug(
        self,
    ) -> crate::pac::common::Reg<MtlRxq1Debug, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d78usize) as _) }
    }
    #[doc = "Queue 1 Receive Control."]
    #[inline(always)]
    pub const fn mtl_rxq1_control(
        self,
    ) -> crate::pac::common::Reg<MtlRxq1Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d7cusize) as _) }
    }
    #[doc = "DMA Bus Mode."]
    #[inline(always)]
    pub const fn dma_mode(self) -> crate::pac::common::Reg<DmaMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "DMA System Bus Mode."]
    #[inline(always)]
    pub const fn dma_sysbus_mode(
        self,
    ) -> crate::pac::common::Reg<DmaSysbusMode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "DMA Interrupt Status."]
    #[inline(always)]
    pub const fn dma_interrupt_status(
        self,
    ) -> crate::pac::common::Reg<DmaInterruptStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "DMA Debug Status 0."]
    #[inline(always)]
    pub const fn dma_debug_status0(
        self,
    ) -> crate::pac::common::Reg<DmaDebugStatus0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "DMA Channel 0 Control."]
    #[inline(always)]
    pub const fn dma_ch0_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh0Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "DMA Channel 0 Transmit Control."]
    #[inline(always)]
    pub const fn dma_ch0_tx_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh0TxControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "DMA Channel 0 Receive Control."]
    #[inline(always)]
    pub const fn dma_ch0_rx_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor List Address register."]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_list_address(
        self,
    ) -> crate::pac::common::Reg<DmaCh0TxdescListAddress, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "Channel 0 Rx Descriptor List Address register."]
    #[inline(always)]
    pub const fn dma_ch0_rxdesc_list_address(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxdescListAddress, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_tail_pointer(
        self,
    ) -> crate::pac::common::Reg<DmaCh0TxdescTailPointer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "Channel 0 Rx Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn dma_ch0_rxdesc_tail_pointer(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxdescTailPointer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "Channel 0 Tx Descriptor Ring Length."]
    #[inline(always)]
    pub const fn dma_ch0_txdesc_ring_length(
        self,
    ) -> crate::pac::common::Reg<DmaCh0TxdescRingLength, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "Channeli Receive Control."]
    #[inline(always)]
    pub const fn dma_ch0_rx_control2(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxControl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "Channeli Interrupt Enable."]
    #[inline(always)]
    pub const fn dma_ch0_interrupt_enable(
        self,
    ) -> crate::pac::common::Reg<DmaCh0InterruptEnable, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer."]
    #[inline(always)]
    pub const fn dma_ch0_rx_interrupt_watchdog_timer(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxInterruptWatchdogTimer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "Channel 0 Slot Function Control and Status."]
    #[inline(always)]
    pub const fn dma_ch0_slot_function_control_status(
        self,
    ) -> crate::pac::common::Reg<DmaCh0SlotFunctionControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Descriptor."]
    #[inline(always)]
    pub const fn dma_ch0_current_app_txdesc(
        self,
    ) -> crate::pac::common::Reg<DmaCh0CurrentAppTxdesc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Descriptor."]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rxdesc(
        self,
    ) -> crate::pac::common::Reg<DmaCh0CurrentAppRxdesc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "Channel 0 Current Application Transmit Buffer Address."]
    #[inline(always)]
    pub const fn dma_ch0_current_app_txbuffer(
        self,
    ) -> crate::pac::common::Reg<DmaCh0CurrentAppTxbuffer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "Channel 0 Current Application Receive Buffer Address."]
    #[inline(always)]
    pub const fn dma_ch0_current_app_rxbuffer(
        self,
    ) -> crate::pac::common::Reg<DmaCh0CurrentAppRxbuffer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "DMA Channel 0 Status."]
    #[inline(always)]
    pub const fn dma_ch0_status(
        self,
    ) -> crate::pac::common::Reg<DmaCh0Status, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "Channel 0 Missed Frame Counter."]
    #[inline(always)]
    pub const fn dma_ch0_miss_frame_cnt(
        self,
    ) -> crate::pac::common::Reg<DmaCh0MissFrameCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "Channel 0 Receive ERI Counter."]
    #[inline(always)]
    pub const fn dma_ch0_rx_eri_cnt(
        self,
    ) -> crate::pac::common::Reg<DmaCh0RxEriCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "DMA Channel 1 Control."]
    #[inline(always)]
    pub const fn dma_ch1_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh1Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "DMA Channel 1 Transmit Control."]
    #[inline(always)]
    pub const fn dma_ch1_tx_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh1TxControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "DMA Channel 1 Receive Control."]
    #[inline(always)]
    pub const fn dma_ch1_rx_control(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxControl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor List Address."]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_list_address(
        self,
    ) -> crate::pac::common::Reg<DmaCh1TxdescListAddress, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "Channel 1 Rx Descriptor List Address."]
    #[inline(always)]
    pub const fn dma_ch1_rxdesc_list_address(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxdescListAddress, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_tail_pointer(
        self,
    ) -> crate::pac::common::Reg<DmaCh1TxdescTailPointer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "Channel 1 Rx Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn dma_ch1_rxdesc_tail_pointer(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxdescTailPointer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "Channel 1 Tx Descriptor Ring Length."]
    #[inline(always)]
    pub const fn dma_ch1_txdesc_ring_length(
        self,
    ) -> crate::pac::common::Reg<DmaCh1TxdescRingLength, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "DMA Channel 1 Receive Control."]
    #[inline(always)]
    pub const fn dma_ch1_rx_control2(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxControl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "Channel 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn dma_ch1_interrupt_enable(
        self,
    ) -> crate::pac::common::Reg<DmaCh1InterruptEnable, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "Channel 1 Receive Interrupt Watchdog Timer."]
    #[inline(always)]
    pub const fn dma_ch1_rx_interrupt_watchdog_timer(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxInterruptWatchdogTimer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "Channel 1 Slot Function Control and Status."]
    #[inline(always)]
    pub const fn dma_ch1_slot_function_control_status(
        self,
    ) -> crate::pac::common::Reg<DmaCh1SlotFunctionControlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "Channel 1 Current Application Transmit Descriptor."]
    #[inline(always)]
    pub const fn dma_ch1_current_app_txdesc(
        self,
    ) -> crate::pac::common::Reg<DmaCh1CurrentAppTxdesc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "Channel 1 Current Application Receive Descriptor."]
    #[inline(always)]
    pub const fn dma_ch1_current_app_rxdesc(
        self,
    ) -> crate::pac::common::Reg<DmaCh1CurrentAppRxdesc, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "Channel 1 Current Application Transmit Buffer Address."]
    #[inline(always)]
    pub const fn dma_ch1_current_app_txbuffer(
        self,
    ) -> crate::pac::common::Reg<DmaCh1CurrentAppTxbuffer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "Channel 1 Current Application Receive Buffer Address."]
    #[inline(always)]
    pub const fn dma_ch1_current_app_rxbuffer(
        self,
    ) -> crate::pac::common::Reg<DmaCh1CurrentAppRxbuffer, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "DMA Channel 1 Status."]
    #[inline(always)]
    pub const fn dma_ch1_status(
        self,
    ) -> crate::pac::common::Reg<DmaCh1Status, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "Channel 1 Missed Frame Counter."]
    #[inline(always)]
    pub const fn dma_ch1_miss_frame_cnt(
        self,
    ) -> crate::pac::common::Reg<DmaCh1MissFrameCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "Channel 1 Receive ERI Counter."]
    #[inline(always)]
    pub const fn dma_ch1_rx_eri_cnt(
        self,
    ) -> crate::pac::common::Reg<DmaCh1RxEriCnt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
}
#[doc = "DMA Channel 0 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0Control(pub u32);
impl DmaCh0Control {
    #[doc = "8xPBL mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pb_lx8(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "8xPBL mode."]
    #[inline(always)]
    pub const fn set_pb_lx8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Descriptor Skip Length."]
    #[must_use]
    #[inline(always)]
    pub const fn dsl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Descriptor Skip Length."]
    #[inline(always)]
    pub const fn set_dsl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for DmaCh0Control {
    #[inline(always)]
    fn default() -> DmaCh0Control {
        DmaCh0Control(0)
    }
}
impl core::fmt::Debug for DmaCh0Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0Control")
            .field("pb_lx8", &self.pb_lx8())
            .field("dsl", &self.dsl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0Control {{ pb_lx8: {=bool:?}, dsl: {=u8:?} }}",
            self.pb_lx8(),
            self.dsl()
        )
    }
}
#[doc = "Channel 0 Current Application Receive Buffer Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppRxbuffer(pub u32);
impl DmaCh0CurrentAppRxbuffer {
    #[doc = "Application Receive Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn currbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_currbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppRxbuffer {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppRxbuffer {
        DmaCh0CurrentAppRxbuffer(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppRxbuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppRxbuffer")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppRxbuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppRxbuffer {{ currbufaptr: {=u32:?} }}",
            self.currbufaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Receive Descriptor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppRxdesc(pub u32);
impl DmaCh0CurrentAppRxdesc {
    #[doc = "Application Receive Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn currdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_currdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppRxdesc {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppRxdesc {
        DmaCh0CurrentAppRxdesc(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppRxdesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppRxdesc")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppRxdesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppRxdesc {{ currdesaptr: {=u32:?} }}",
            self.currdesaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Transmit Buffer Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppTxbuffer(pub u32);
impl DmaCh0CurrentAppTxbuffer {
    #[doc = "Application Transmit Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn curtbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_curtbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppTxbuffer {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppTxbuffer {
        DmaCh0CurrentAppTxbuffer(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppTxbuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppTxbuffer")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppTxbuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppTxbuffer {{ curtbufaptr: {=u32:?} }}",
            self.curtbufaptr()
        )
    }
}
#[doc = "Channel 0 Current Application Transmit Descriptor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0CurrentAppTxdesc(pub u32);
impl DmaCh0CurrentAppTxdesc {
    #[doc = "Application Transmit Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn curtdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_curtdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh0CurrentAppTxdesc {
    #[inline(always)]
    fn default() -> DmaCh0CurrentAppTxdesc {
        DmaCh0CurrentAppTxdesc(0)
    }
}
impl core::fmt::Debug for DmaCh0CurrentAppTxdesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0CurrentAppTxdesc")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0CurrentAppTxdesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0CurrentAppTxdesc {{ curtdesaptr: {=u32:?} }}",
            self.curtdesaptr()
        )
    }
}
#[doc = "Channeli Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0InterruptEnable(pub u32);
impl DmaCh0InterruptEnable {
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Stopped Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Stopped Enable."]
    #[inline(always)]
    pub const fn set_txse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbue(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable Enable."]
    #[inline(always)]
    pub const fn set_tbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rbue(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable Enable."]
    #[inline(always)]
    pub const fn set_rbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Stopped Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Stopped Enable."]
    #[inline(always)]
    pub const fn set_rse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwte(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout Enable."]
    #[inline(always)]
    pub const fn set_rwte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn etie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_etie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fbee(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Enable."]
    #[inline(always)]
    pub const fn set_fbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cdee(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error Enable."]
    #[inline(always)]
    pub const fn set_cdee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_aie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_nie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh0InterruptEnable {
    #[inline(always)]
    fn default() -> DmaCh0InterruptEnable {
        DmaCh0InterruptEnable(0)
    }
}
impl core::fmt::Debug for DmaCh0InterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0InterruptEnable")
            .field("tie", &self.tie())
            .field("txse", &self.txse())
            .field("tbue", &self.tbue())
            .field("rie", &self.rie())
            .field("rbue", &self.rbue())
            .field("rse", &self.rse())
            .field("rwte", &self.rwte())
            .field("etie", &self.etie())
            .field("erie", &self.erie())
            .field("fbee", &self.fbee())
            .field("cdee", &self.cdee())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0InterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0InterruptEnable {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}",
            self.tie(),
            self.txse(),
            self.tbue(),
            self.rie(),
            self.rbue(),
            self.rse(),
            self.rwte(),
            self.etie(),
            self.erie(),
            self.fbee(),
            self.cdee(),
            self.aie(),
            self.nie()
        )
    }
}
#[doc = "Channel 0 Missed Frame Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0MissFrameCnt(pub u32);
impl DmaCh0MissFrameCnt {
    #[doc = "Dropped Packet Counters."]
    #[must_use]
    #[inline(always)]
    pub const fn mfc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Dropped Packet Counters."]
    #[inline(always)]
    pub const fn set_mfc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow status of the MFC Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn mfco(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow status of the MFC Counter."]
    #[inline(always)]
    pub const fn set_mfco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh0MissFrameCnt {
    #[inline(always)]
    fn default() -> DmaCh0MissFrameCnt {
        DmaCh0MissFrameCnt(0)
    }
}
impl core::fmt::Debug for DmaCh0MissFrameCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0MissFrameCnt")
            .field("mfc", &self.mfc())
            .field("mfco", &self.mfco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0MissFrameCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0MissFrameCnt {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
            self.mfc(),
            self.mfco()
        )
    }
}
#[doc = "DMA Channel 0 Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxControl(pub u32);
impl DmaCh0RxControl {
    #[doc = "Start or Stop Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> DmaCh0RxControlSr {
        let val = (self.0 >> 0usize) & 0x01;
        DmaCh0RxControlSr::from_bits(val as u8)
    }
    #[doc = "Start or Stop Receive."]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: DmaCh0RxControlSr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Buffer size Low."]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_x_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Buffer size Low."]
    #[inline(always)]
    pub const fn set_rbsz_x_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Receive Buffer size High."]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_13_y(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x0fff;
        val as u16
    }
    #[doc = "Receive Buffer size High."]
    #[inline(always)]
    pub const fn set_rbsz_13_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 3usize)) | (((val as u32) & 0x0fff) << 3usize);
    }
    #[doc = "Receive Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Receive Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_rx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Receive Interrupt Control."]
    #[must_use]
    #[inline(always)]
    pub const fn eric(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Control."]
    #[inline(always)]
    pub const fn set_eric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Rx Packet Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn rpf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Packet Flush."]
    #[inline(always)]
    pub const fn set_rpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DmaCh0RxControl {
    #[inline(always)]
    fn default() -> DmaCh0RxControl {
        DmaCh0RxControl(0)
    }
}
impl core::fmt::Debug for DmaCh0RxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxControl")
            .field("sr", &self.sr())
            .field("rbsz_x_0", &self.rbsz_x_0())
            .field("rbsz_13_y", &self.rbsz_13_y())
            .field("rx_pbl", &self.rx_pbl())
            .field("eric", &self.eric())
            .field("rpf", &self.rpf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxControl {{ sr: {:?}, rbsz_x_0: {=u8:?}, rbsz_13_y: {=u16:?}, rx_pbl: {=u8:?}, eric: {=bool:?}, rpf: {=bool:?} }}",
            self.sr(),
            self.rbsz_x_0(),
            self.rbsz_13_y(),
            self.rx_pbl(),
            self.eric(),
            self.rpf()
        )
    }
}
#[doc = "Channeli Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxControl2(pub u32);
impl DmaCh0RxControl2 {
    #[doc = "Receive Descriptor Ring Length."]
    #[must_use]
    #[inline(always)]
    pub const fn rdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive Descriptor Ring Length."]
    #[inline(always)]
    pub const fn set_rdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Alternate Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn arbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Alternate Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_arbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for DmaCh0RxControl2 {
    #[inline(always)]
    fn default() -> DmaCh0RxControl2 {
        DmaCh0RxControl2(0)
    }
}
impl core::fmt::Debug for DmaCh0RxControl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxControl2")
            .field("rdrl", &self.rdrl())
            .field("arbs", &self.arbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxControl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxControl2 {{ rdrl: {=u16:?}, arbs: {=u8:?} }}",
            self.rdrl(),
            self.arbs()
        )
    }
}
#[doc = "Channel 0 Receive ERI Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxEriCnt(pub u32);
impl DmaCh0RxEriCnt {
    #[doc = "ERI Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ecnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ERI Counter."]
    #[inline(always)]
    pub const fn set_ecnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for DmaCh0RxEriCnt {
    #[inline(always)]
    fn default() -> DmaCh0RxEriCnt {
        DmaCh0RxEriCnt(0)
    }
}
impl core::fmt::Debug for DmaCh0RxEriCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxEriCnt")
            .field("ecnt", &self.ecnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxEriCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaCh0RxEriCnt {{ ecnt: {=u16:?} }}", self.ecnt())
    }
}
#[doc = "Channel 0 Receive Interrupt Watchdog Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxInterruptWatchdogTimer(pub u32);
impl DmaCh0RxInterruptWatchdogTimer {
    #[doc = "Receive Interrupt Watchdog Timer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count."]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units."]
    #[must_use]
    #[inline(always)]
    pub const fn rwtu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units."]
    #[inline(always)]
    pub const fn set_rwtu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for DmaCh0RxInterruptWatchdogTimer {
    #[inline(always)]
    fn default() -> DmaCh0RxInterruptWatchdogTimer {
        DmaCh0RxInterruptWatchdogTimer(0)
    }
}
impl core::fmt::Debug for DmaCh0RxInterruptWatchdogTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxInterruptWatchdogTimer")
            .field("rwt", &self.rwt())
            .field("rwtu", &self.rwtu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxInterruptWatchdogTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxInterruptWatchdogTimer {{ rwt: {=u8:?}, rwtu: {=u8:?} }}",
            self.rwt(),
            self.rwtu()
        )
    }
}
#[doc = "Channel 0 Rx Descriptor List Address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxdescListAddress(pub u32);
impl DmaCh0RxdescListAddress {
    #[doc = "Start of Receive List."]
    #[must_use]
    #[inline(always)]
    pub const fn rdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Receive List."]
    #[inline(always)]
    pub const fn set_rdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0RxdescListAddress {
    #[inline(always)]
    fn default() -> DmaCh0RxdescListAddress {
        DmaCh0RxdescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh0RxdescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxdescListAddress")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxdescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxdescListAddress {{ rdesla: {=u32:?} }}",
            self.rdesla()
        )
    }
}
#[doc = "Channel 0 Rx Descriptor Tail Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0RxdescTailPointer(pub u32);
impl DmaCh0RxdescTailPointer {
    #[doc = "Receive Descriptor Tail Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Receive Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn set_rdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0RxdescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh0RxdescTailPointer {
        DmaCh0RxdescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh0RxdescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0RxdescTailPointer")
            .field("rdtp", &self.rdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0RxdescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0RxdescTailPointer {{ rdtp: {=u32:?} }}",
            self.rdtp()
        )
    }
}
#[doc = "Channel 0 Slot Function Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0SlotFunctionControlStatus(pub u32);
impl DmaCh0SlotFunctionControlStatus {
    #[doc = "Enable Slot Comparison."]
    #[must_use]
    #[inline(always)]
    pub const fn esc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Slot Comparison."]
    #[inline(always)]
    pub const fn set_esc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Advance Slot Check."]
    #[must_use]
    #[inline(always)]
    pub const fn asc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Advance Slot Check."]
    #[inline(always)]
    pub const fn set_asc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Slot Interval Value."]
    #[must_use]
    #[inline(always)]
    pub const fn siv(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slot Interval Value."]
    #[inline(always)]
    pub const fn set_siv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "Reference Slot Number."]
    #[must_use]
    #[inline(always)]
    pub const fn rsn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Reference Slot Number."]
    #[inline(always)]
    pub const fn set_rsn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for DmaCh0SlotFunctionControlStatus {
    #[inline(always)]
    fn default() -> DmaCh0SlotFunctionControlStatus {
        DmaCh0SlotFunctionControlStatus(0)
    }
}
impl core::fmt::Debug for DmaCh0SlotFunctionControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0SlotFunctionControlStatus")
            .field("esc", &self.esc())
            .field("asc", &self.asc())
            .field("siv", &self.siv())
            .field("rsn", &self.rsn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0SlotFunctionControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0SlotFunctionControlStatus {{ esc: {=bool:?}, asc: {=bool:?}, siv: {=u16:?}, rsn: {=u8:?} }}",
            self.esc(),
            self.asc(),
            self.siv(),
            self.rsn()
        )
    }
}
#[doc = "DMA Channel 0 Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0Status(pub u32);
impl DmaCh0Status {
    #[doc = "Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Process Stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Process Stopped."]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable."]
    #[inline(always)]
    pub const fn set_tbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt."]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable."]
    #[must_use]
    #[inline(always)]
    pub const fn rbu(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable."]
    #[inline(always)]
    pub const fn set_rbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Process Stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn rps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Process Stopped."]
    #[inline(always)]
    pub const fn set_rps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout."]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eti(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_eti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eri(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt."]
    #[inline(always)]
    pub const fn set_eri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error."]
    #[inline(always)]
    pub const fn set_fbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error."]
    #[inline(always)]
    pub const fn set_cde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_ais(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_nis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Tx DMA Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn teb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Tx DMA Error Bits."]
    #[inline(always)]
    pub const fn set_teb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Rx DMA Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn reb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Rx DMA Error Bits."]
    #[inline(always)]
    pub const fn set_reb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
}
impl Default for DmaCh0Status {
    #[inline(always)]
    fn default() -> DmaCh0Status {
        DmaCh0Status(0)
    }
}
impl core::fmt::Debug for DmaCh0Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0Status")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("eri", &self.eri())
            .field("fbe", &self.fbe())
            .field("cde", &self.cde())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("teb", &self.teb())
            .field("reb", &self.reb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0Status {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, eri: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}",
            self.ti(),
            self.tps(),
            self.tbu(),
            self.ri(),
            self.rbu(),
            self.rps(),
            self.rwt(),
            self.eti(),
            self.eri(),
            self.fbe(),
            self.cde(),
            self.ais(),
            self.nis(),
            self.teb(),
            self.reb()
        )
    }
}
#[doc = "DMA Channel 0 Transmit Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxControl(pub u32);
impl DmaCh0TxControl {
    #[doc = "Start or Stop Transmission Command."]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> DmaCh0TxControlSt {
        let val = (self.0 >> 0usize) & 0x01;
        DmaCh0TxControlSt::from_bits(val as u8)
    }
    #[doc = "Start or Stop Transmission Command."]
    #[inline(always)]
    pub const fn set_st(&mut self, val: DmaCh0TxControlSt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Channel Weight."]
    #[must_use]
    #[inline(always)]
    pub const fn tcw(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Channel Weight."]
    #[inline(always)]
    pub const fn set_tcw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Operate on Second Packet."]
    #[must_use]
    #[inline(always)]
    pub const fn osf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Operate on Second Packet."]
    #[inline(always)]
    pub const fn set_osf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_tx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Transmit Interrupt Control."]
    #[must_use]
    #[inline(always)]
    pub const fn etic(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Control."]
    #[inline(always)]
    pub const fn set_etic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for DmaCh0TxControl {
    #[inline(always)]
    fn default() -> DmaCh0TxControl {
        DmaCh0TxControl(0)
    }
}
impl core::fmt::Debug for DmaCh0TxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxControl")
            .field("st", &self.st())
            .field("tcw", &self.tcw())
            .field("osf", &self.osf())
            .field("tx_pbl", &self.tx_pbl())
            .field("etic", &self.etic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxControl {{ st: {:?}, tcw: {=u8:?}, osf: {=bool:?}, tx_pbl: {=u8:?}, etic: {=bool:?} }}",
            self.st(),
            self.tcw(),
            self.osf(),
            self.tx_pbl(),
            self.etic()
        )
    }
}
#[doc = "Channel 0 Tx Descriptor List Address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxdescListAddress(pub u32);
impl DmaCh0TxdescListAddress {
    #[doc = "Start of Transmit List."]
    #[must_use]
    #[inline(always)]
    pub const fn tdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Transmit List."]
    #[inline(always)]
    pub const fn set_tdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0TxdescListAddress {
    #[inline(always)]
    fn default() -> DmaCh0TxdescListAddress {
        DmaCh0TxdescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh0TxdescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxdescListAddress")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxdescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxdescListAddress {{ tdesla: {=u32:?} }}",
            self.tdesla()
        )
    }
}
#[doc = "Channel 0 Tx Descriptor Ring Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxdescRingLength(pub u32);
impl DmaCh0TxdescRingLength {
    #[doc = "Transmit Descriptor Ring Length."]
    #[must_use]
    #[inline(always)]
    pub const fn tdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Transmit Descriptor Ring Length."]
    #[inline(always)]
    pub const fn set_tdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DmaCh0TxdescRingLength {
    #[inline(always)]
    fn default() -> DmaCh0TxdescRingLength {
        DmaCh0TxdescRingLength(0)
    }
}
impl core::fmt::Debug for DmaCh0TxdescRingLength {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxdescRingLength")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxdescRingLength {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxdescRingLength {{ tdrl: {=u16:?} }}",
            self.tdrl()
        )
    }
}
#[doc = "Channel 0 Tx Descriptor Tail Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh0TxdescTailPointer(pub u32);
impl DmaCh0TxdescTailPointer {
    #[doc = "Transmit Descriptor Tail Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn tdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Transmit Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn set_tdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh0TxdescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh0TxdescTailPointer {
        DmaCh0TxdescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh0TxdescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh0TxdescTailPointer")
            .field("tdtp", &self.tdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh0TxdescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh0TxdescTailPointer {{ tdtp: {=u32:?} }}",
            self.tdtp()
        )
    }
}
#[doc = "DMA Channel 1 Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1Control(pub u32);
impl DmaCh1Control {
    #[doc = "8xPBL mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pb_lx8(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "8xPBL mode."]
    #[inline(always)]
    pub const fn set_pb_lx8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Descriptor Skip Length."]
    #[must_use]
    #[inline(always)]
    pub const fn dsl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Descriptor Skip Length."]
    #[inline(always)]
    pub const fn set_dsl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
}
impl Default for DmaCh1Control {
    #[inline(always)]
    fn default() -> DmaCh1Control {
        DmaCh1Control(0)
    }
}
impl core::fmt::Debug for DmaCh1Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1Control")
            .field("pb_lx8", &self.pb_lx8())
            .field("dsl", &self.dsl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1Control {{ pb_lx8: {=bool:?}, dsl: {=u8:?} }}",
            self.pb_lx8(),
            self.dsl()
        )
    }
}
#[doc = "Channel 1 Current Application Receive Buffer Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1CurrentAppRxbuffer(pub u32);
impl DmaCh1CurrentAppRxbuffer {
    #[doc = "Application Receive Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn currbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_currbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh1CurrentAppRxbuffer {
    #[inline(always)]
    fn default() -> DmaCh1CurrentAppRxbuffer {
        DmaCh1CurrentAppRxbuffer(0)
    }
}
impl core::fmt::Debug for DmaCh1CurrentAppRxbuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1CurrentAppRxbuffer")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1CurrentAppRxbuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1CurrentAppRxbuffer {{ currbufaptr: {=u32:?} }}",
            self.currbufaptr()
        )
    }
}
#[doc = "Channel 1 Current Application Receive Descriptor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1CurrentAppRxdesc(pub u32);
impl DmaCh1CurrentAppRxdesc {
    #[doc = "Application Receive Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn currdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Receive Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_currdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh1CurrentAppRxdesc {
    #[inline(always)]
    fn default() -> DmaCh1CurrentAppRxdesc {
        DmaCh1CurrentAppRxdesc(0)
    }
}
impl core::fmt::Debug for DmaCh1CurrentAppRxdesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1CurrentAppRxdesc")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1CurrentAppRxdesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1CurrentAppRxdesc {{ currdesaptr: {=u32:?} }}",
            self.currdesaptr()
        )
    }
}
#[doc = "Channel 1 Current Application Transmit Buffer Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1CurrentAppTxbuffer(pub u32);
impl DmaCh1CurrentAppTxbuffer {
    #[doc = "Application Transmit Buffer Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn curtbufaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Buffer Address Pointer."]
    #[inline(always)]
    pub const fn set_curtbufaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh1CurrentAppTxbuffer {
    #[inline(always)]
    fn default() -> DmaCh1CurrentAppTxbuffer {
        DmaCh1CurrentAppTxbuffer(0)
    }
}
impl core::fmt::Debug for DmaCh1CurrentAppTxbuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1CurrentAppTxbuffer")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1CurrentAppTxbuffer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1CurrentAppTxbuffer {{ curtbufaptr: {=u32:?} }}",
            self.curtbufaptr()
        )
    }
}
#[doc = "Channel 1 Current Application Transmit Descriptor."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1CurrentAppTxdesc(pub u32);
impl DmaCh1CurrentAppTxdesc {
    #[doc = "Application Transmit Descriptor Address Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn curtdesaptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Application Transmit Descriptor Address Pointer."]
    #[inline(always)]
    pub const fn set_curtdesaptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DmaCh1CurrentAppTxdesc {
    #[inline(always)]
    fn default() -> DmaCh1CurrentAppTxdesc {
        DmaCh1CurrentAppTxdesc(0)
    }
}
impl core::fmt::Debug for DmaCh1CurrentAppTxdesc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1CurrentAppTxdesc")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1CurrentAppTxdesc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1CurrentAppTxdesc {{ curtdesaptr: {=u32:?} }}",
            self.curtdesaptr()
        )
    }
}
#[doc = "Channel 1 Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1InterruptEnable(pub u32);
impl DmaCh1InterruptEnable {
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Stopped Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Stopped Enable."]
    #[inline(always)]
    pub const fn set_txse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbue(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable Enable."]
    #[inline(always)]
    pub const fn set_tbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rbue(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable Enable."]
    #[inline(always)]
    pub const fn set_rbue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Stopped Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rse(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Stopped Enable."]
    #[inline(always)]
    pub const fn set_rse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwte(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout Enable."]
    #[inline(always)]
    pub const fn set_rwte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn etie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_etie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Enable."]
    #[inline(always)]
    pub const fn set_erie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fbee(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error Enable."]
    #[inline(always)]
    pub const fn set_fbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cdee(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error Enable."]
    #[inline(always)]
    pub const fn set_cdee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_aie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub const fn set_nie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh1InterruptEnable {
    #[inline(always)]
    fn default() -> DmaCh1InterruptEnable {
        DmaCh1InterruptEnable(0)
    }
}
impl core::fmt::Debug for DmaCh1InterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1InterruptEnable")
            .field("tie", &self.tie())
            .field("txse", &self.txse())
            .field("tbue", &self.tbue())
            .field("rie", &self.rie())
            .field("rbue", &self.rbue())
            .field("rse", &self.rse())
            .field("rwte", &self.rwte())
            .field("etie", &self.etie())
            .field("erie", &self.erie())
            .field("fbee", &self.fbee())
            .field("cdee", &self.cdee())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1InterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1InterruptEnable {{ tie: {=bool:?}, txse: {=bool:?}, tbue: {=bool:?}, rie: {=bool:?}, rbue: {=bool:?}, rse: {=bool:?}, rwte: {=bool:?}, etie: {=bool:?}, erie: {=bool:?}, fbee: {=bool:?}, cdee: {=bool:?}, aie: {=bool:?}, nie: {=bool:?} }}",
            self.tie(),
            self.txse(),
            self.tbue(),
            self.rie(),
            self.rbue(),
            self.rse(),
            self.rwte(),
            self.etie(),
            self.erie(),
            self.fbee(),
            self.cdee(),
            self.aie(),
            self.nie()
        )
    }
}
#[doc = "Channel 1 Missed Frame Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1MissFrameCnt(pub u32);
impl DmaCh1MissFrameCnt {
    #[doc = "Dropped Packet Counters."]
    #[must_use]
    #[inline(always)]
    pub const fn mfc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Dropped Packet Counters."]
    #[inline(always)]
    pub const fn set_mfc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow status of the MFC Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn mfco(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow status of the MFC Counter."]
    #[inline(always)]
    pub const fn set_mfco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaCh1MissFrameCnt {
    #[inline(always)]
    fn default() -> DmaCh1MissFrameCnt {
        DmaCh1MissFrameCnt(0)
    }
}
impl core::fmt::Debug for DmaCh1MissFrameCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1MissFrameCnt")
            .field("mfc", &self.mfc())
            .field("mfco", &self.mfco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1MissFrameCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1MissFrameCnt {{ mfc: {=u16:?}, mfco: {=bool:?} }}",
            self.mfc(),
            self.mfco()
        )
    }
}
#[doc = "DMA Channel 1 Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxControl(pub u32);
impl DmaCh1RxControl {
    #[doc = "Start or Stop Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> DmaCh1RxControlSr {
        let val = (self.0 >> 0usize) & 0x01;
        DmaCh1RxControlSr::from_bits(val as u8)
    }
    #[doc = "Start or Stop Receive."]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: DmaCh1RxControlSr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Buffer size Low."]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_x_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Buffer size Low."]
    #[inline(always)]
    pub const fn set_rbsz_x_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Receive Buffer size High."]
    #[must_use]
    #[inline(always)]
    pub const fn rbsz_13_y(&self) -> u16 {
        let val = (self.0 >> 3usize) & 0x0fff;
        val as u16
    }
    #[doc = "Receive Buffer size High."]
    #[inline(always)]
    pub const fn set_rbsz_13_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 3usize)) | (((val as u32) & 0x0fff) << 3usize);
    }
    #[doc = "Receive Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Receive Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_rx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Receive Interrupt Control."]
    #[must_use]
    #[inline(always)]
    pub const fn eric(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt Control."]
    #[inline(always)]
    pub const fn set_eric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Rx Packet Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn rpf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Packet Flush."]
    #[inline(always)]
    pub const fn set_rpf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DmaCh1RxControl {
    #[inline(always)]
    fn default() -> DmaCh1RxControl {
        DmaCh1RxControl(0)
    }
}
impl core::fmt::Debug for DmaCh1RxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxControl")
            .field("sr", &self.sr())
            .field("rbsz_x_0", &self.rbsz_x_0())
            .field("rbsz_13_y", &self.rbsz_13_y())
            .field("rx_pbl", &self.rx_pbl())
            .field("eric", &self.eric())
            .field("rpf", &self.rpf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1RxControl {{ sr: {:?}, rbsz_x_0: {=u8:?}, rbsz_13_y: {=u16:?}, rx_pbl: {=u8:?}, eric: {=bool:?}, rpf: {=bool:?} }}",
            self.sr(),
            self.rbsz_x_0(),
            self.rbsz_13_y(),
            self.rx_pbl(),
            self.eric(),
            self.rpf()
        )
    }
}
#[doc = "DMA Channel 1 Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxControl2(pub u32);
impl DmaCh1RxControl2 {
    #[doc = "Receive Descriptor Ring Length."]
    #[must_use]
    #[inline(always)]
    pub const fn rdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Receive Descriptor Ring Length."]
    #[inline(always)]
    pub const fn set_rdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Alternate Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn arbs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Alternate Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_arbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for DmaCh1RxControl2 {
    #[inline(always)]
    fn default() -> DmaCh1RxControl2 {
        DmaCh1RxControl2(0)
    }
}
impl core::fmt::Debug for DmaCh1RxControl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxControl2")
            .field("rdrl", &self.rdrl())
            .field("arbs", &self.arbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxControl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1RxControl2 {{ rdrl: {=u16:?}, arbs: {=u8:?} }}",
            self.rdrl(),
            self.arbs()
        )
    }
}
#[doc = "Channel 1 Receive ERI Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxEriCnt(pub u32);
impl DmaCh1RxEriCnt {
    #[doc = "ERI Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ecnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "ERI Counter."]
    #[inline(always)]
    pub const fn set_ecnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for DmaCh1RxEriCnt {
    #[inline(always)]
    fn default() -> DmaCh1RxEriCnt {
        DmaCh1RxEriCnt(0)
    }
}
impl core::fmt::Debug for DmaCh1RxEriCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxEriCnt")
            .field("ecnt", &self.ecnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxEriCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DmaCh1RxEriCnt {{ ecnt: {=u16:?} }}", self.ecnt())
    }
}
#[doc = "Channel 1 Receive Interrupt Watchdog Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxInterruptWatchdogTimer(pub u32);
impl DmaCh1RxInterruptWatchdogTimer {
    #[doc = "Receive Interrupt Watchdog Timer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count."]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units."]
    #[must_use]
    #[inline(always)]
    pub const fn rwtu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Interrupt Watchdog Timer Count Units."]
    #[inline(always)]
    pub const fn set_rwtu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for DmaCh1RxInterruptWatchdogTimer {
    #[inline(always)]
    fn default() -> DmaCh1RxInterruptWatchdogTimer {
        DmaCh1RxInterruptWatchdogTimer(0)
    }
}
impl core::fmt::Debug for DmaCh1RxInterruptWatchdogTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxInterruptWatchdogTimer")
            .field("rwt", &self.rwt())
            .field("rwtu", &self.rwtu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxInterruptWatchdogTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1RxInterruptWatchdogTimer {{ rwt: {=u8:?}, rwtu: {=u8:?} }}",
            self.rwt(),
            self.rwtu()
        )
    }
}
#[doc = "Channel 1 Rx Descriptor List Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxdescListAddress(pub u32);
impl DmaCh1RxdescListAddress {
    #[doc = "Start of Receive List."]
    #[must_use]
    #[inline(always)]
    pub const fn rdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Receive List."]
    #[inline(always)]
    pub const fn set_rdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh1RxdescListAddress {
    #[inline(always)]
    fn default() -> DmaCh1RxdescListAddress {
        DmaCh1RxdescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh1RxdescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxdescListAddress")
            .field("rdesla", &self.rdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxdescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1RxdescListAddress {{ rdesla: {=u32:?} }}",
            self.rdesla()
        )
    }
}
#[doc = "Channel 1 Rx Descriptor Tail Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1RxdescTailPointer(pub u32);
impl DmaCh1RxdescTailPointer {
    #[doc = "Receive Descriptor Tail Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Receive Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn set_rdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh1RxdescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh1RxdescTailPointer {
        DmaCh1RxdescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh1RxdescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1RxdescTailPointer")
            .field("rdtp", &self.rdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1RxdescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1RxdescTailPointer {{ rdtp: {=u32:?} }}",
            self.rdtp()
        )
    }
}
#[doc = "Channel 1 Slot Function Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1SlotFunctionControlStatus(pub u32);
impl DmaCh1SlotFunctionControlStatus {
    #[doc = "Enable Slot Comparison."]
    #[must_use]
    #[inline(always)]
    pub const fn esc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Slot Comparison."]
    #[inline(always)]
    pub const fn set_esc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Advance Slot Check."]
    #[must_use]
    #[inline(always)]
    pub const fn asc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Advance Slot Check."]
    #[inline(always)]
    pub const fn set_asc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Slot Interval Value."]
    #[must_use]
    #[inline(always)]
    pub const fn siv(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Slot Interval Value."]
    #[inline(always)]
    pub const fn set_siv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "Reference Slot Number."]
    #[must_use]
    #[inline(always)]
    pub const fn rsn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Reference Slot Number."]
    #[inline(always)]
    pub const fn set_rsn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for DmaCh1SlotFunctionControlStatus {
    #[inline(always)]
    fn default() -> DmaCh1SlotFunctionControlStatus {
        DmaCh1SlotFunctionControlStatus(0)
    }
}
impl core::fmt::Debug for DmaCh1SlotFunctionControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1SlotFunctionControlStatus")
            .field("esc", &self.esc())
            .field("asc", &self.asc())
            .field("siv", &self.siv())
            .field("rsn", &self.rsn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1SlotFunctionControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1SlotFunctionControlStatus {{ esc: {=bool:?}, asc: {=bool:?}, siv: {=u16:?}, rsn: {=u8:?} }}",
            self.esc(),
            self.asc(),
            self.siv(),
            self.rsn()
        )
    }
}
#[doc = "DMA Channel 1 Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1Status(pub u32);
impl DmaCh1Status {
    #[doc = "Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ti(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_ti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Process Stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Process Stopped."]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Buffer Unavailable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbu(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer Unavailable."]
    #[inline(always)]
    pub const fn set_tbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt."]
    #[inline(always)]
    pub const fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Buffer Unavailable."]
    #[must_use]
    #[inline(always)]
    pub const fn rbu(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Unavailable."]
    #[inline(always)]
    pub const fn set_rbu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive Process Stopped."]
    #[must_use]
    #[inline(always)]
    pub const fn rps(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Process Stopped."]
    #[inline(always)]
    pub const fn set_rps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Watchdog Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout."]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Early Transmit Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eti(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt."]
    #[inline(always)]
    pub const fn set_eti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Early Receive Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eri(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Early Receive Interrupt."]
    #[inline(always)]
    pub const fn set_eri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Fatal Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn fbe(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal Bus Error."]
    #[inline(always)]
    pub const fn set_fbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Context Descriptor Error."]
    #[must_use]
    #[inline(always)]
    pub const fn cde(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Context Descriptor Error."]
    #[inline(always)]
    pub const fn set_cde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn ais(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Abnormal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_ais(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Normal Interrupt Summary."]
    #[must_use]
    #[inline(always)]
    pub const fn nis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Normal Interrupt Summary."]
    #[inline(always)]
    pub const fn set_nis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Tx DMA Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn teb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Tx DMA Error Bits."]
    #[inline(always)]
    pub const fn set_teb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Rx DMA Error Bits."]
    #[must_use]
    #[inline(always)]
    pub const fn reb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Rx DMA Error Bits."]
    #[inline(always)]
    pub const fn set_reb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
}
impl Default for DmaCh1Status {
    #[inline(always)]
    fn default() -> DmaCh1Status {
        DmaCh1Status(0)
    }
}
impl core::fmt::Debug for DmaCh1Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1Status")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("eri", &self.eri())
            .field("fbe", &self.fbe())
            .field("cde", &self.cde())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("teb", &self.teb())
            .field("reb", &self.reb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1Status {{ ti: {=bool:?}, tps: {=bool:?}, tbu: {=bool:?}, ri: {=bool:?}, rbu: {=bool:?}, rps: {=bool:?}, rwt: {=bool:?}, eti: {=bool:?}, eri: {=bool:?}, fbe: {=bool:?}, cde: {=bool:?}, ais: {=bool:?}, nis: {=bool:?}, teb: {=u8:?}, reb: {=u8:?} }}",
            self.ti(),
            self.tps(),
            self.tbu(),
            self.ri(),
            self.rbu(),
            self.rps(),
            self.rwt(),
            self.eti(),
            self.eri(),
            self.fbe(),
            self.cde(),
            self.ais(),
            self.nis(),
            self.teb(),
            self.reb()
        )
    }
}
#[doc = "DMA Channel 1 Transmit Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1TxControl(pub u32);
impl DmaCh1TxControl {
    #[doc = "Start or Stop Transmission Command."]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> DmaCh1TxControlSt {
        let val = (self.0 >> 0usize) & 0x01;
        DmaCh1TxControlSt::from_bits(val as u8)
    }
    #[doc = "Start or Stop Transmission Command."]
    #[inline(always)]
    pub const fn set_st(&mut self, val: DmaCh1TxControlSt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Channel Weight."]
    #[must_use]
    #[inline(always)]
    pub const fn tcw(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Channel Weight."]
    #[inline(always)]
    pub const fn set_tcw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Operate on Second Packet."]
    #[must_use]
    #[inline(always)]
    pub const fn osf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Operate on Second Packet."]
    #[inline(always)]
    pub const fn set_osf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit Programmable Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_pbl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit Programmable Burst Length."]
    #[inline(always)]
    pub const fn set_tx_pbl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Early Transmit Interrupt Control."]
    #[must_use]
    #[inline(always)]
    pub const fn etic(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Early Transmit Interrupt Control."]
    #[inline(always)]
    pub const fn set_etic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for DmaCh1TxControl {
    #[inline(always)]
    fn default() -> DmaCh1TxControl {
        DmaCh1TxControl(0)
    }
}
impl core::fmt::Debug for DmaCh1TxControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1TxControl")
            .field("st", &self.st())
            .field("tcw", &self.tcw())
            .field("osf", &self.osf())
            .field("tx_pbl", &self.tx_pbl())
            .field("etic", &self.etic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1TxControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1TxControl {{ st: {:?}, tcw: {=u8:?}, osf: {=bool:?}, tx_pbl: {=u8:?}, etic: {=bool:?} }}",
            self.st(),
            self.tcw(),
            self.osf(),
            self.tx_pbl(),
            self.etic()
        )
    }
}
#[doc = "Channel 1 Tx Descriptor List Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1TxdescListAddress(pub u32);
impl DmaCh1TxdescListAddress {
    #[doc = "Start of Transmit List."]
    #[must_use]
    #[inline(always)]
    pub const fn tdesla(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Start of Transmit List."]
    #[inline(always)]
    pub const fn set_tdesla(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh1TxdescListAddress {
    #[inline(always)]
    fn default() -> DmaCh1TxdescListAddress {
        DmaCh1TxdescListAddress(0)
    }
}
impl core::fmt::Debug for DmaCh1TxdescListAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1TxdescListAddress")
            .field("tdesla", &self.tdesla())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1TxdescListAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1TxdescListAddress {{ tdesla: {=u32:?} }}",
            self.tdesla()
        )
    }
}
#[doc = "Channel 1 Tx Descriptor Ring Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1TxdescRingLength(pub u32);
impl DmaCh1TxdescRingLength {
    #[doc = "Transmit Descriptor Ring Length."]
    #[must_use]
    #[inline(always)]
    pub const fn tdrl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Transmit Descriptor Ring Length."]
    #[inline(always)]
    pub const fn set_tdrl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DmaCh1TxdescRingLength {
    #[inline(always)]
    fn default() -> DmaCh1TxdescRingLength {
        DmaCh1TxdescRingLength(0)
    }
}
impl core::fmt::Debug for DmaCh1TxdescRingLength {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1TxdescRingLength")
            .field("tdrl", &self.tdrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1TxdescRingLength {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1TxdescRingLength {{ tdrl: {=u16:?} }}",
            self.tdrl()
        )
    }
}
#[doc = "Channel 1 Tx Descriptor Tail Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaCh1TxdescTailPointer(pub u32);
impl DmaCh1TxdescTailPointer {
    #[doc = "Transmit Descriptor Tail Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn tdtp(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Transmit Descriptor Tail Pointer."]
    #[inline(always)]
    pub const fn set_tdtp(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for DmaCh1TxdescTailPointer {
    #[inline(always)]
    fn default() -> DmaCh1TxdescTailPointer {
        DmaCh1TxdescTailPointer(0)
    }
}
impl core::fmt::Debug for DmaCh1TxdescTailPointer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaCh1TxdescTailPointer")
            .field("tdtp", &self.tdtp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaCh1TxdescTailPointer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaCh1TxdescTailPointer {{ tdtp: {=u32:?} }}",
            self.tdtp()
        )
    }
}
#[doc = "DMA Debug Status 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaDebugStatus0(pub u32);
impl DmaDebugStatus0 {
    #[doc = "AHB Master Status."]
    #[must_use]
    #[inline(always)]
    pub const fn axwhsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Master Status."]
    #[inline(always)]
    pub const fn set_axwhsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Channel 0 Receive Process State."]
    #[must_use]
    #[inline(always)]
    pub const fn rps0(&self) -> Rps0 {
        let val = (self.0 >> 8usize) & 0x0f;
        Rps0::from_bits(val as u8)
    }
    #[doc = "DMA Channel 0 Receive Process State."]
    #[inline(always)]
    pub const fn set_rps0(&mut self, val: Rps0) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "DMA Channel 0 Transmit Process State."]
    #[must_use]
    #[inline(always)]
    pub const fn tps0(&self) -> Tps0 {
        let val = (self.0 >> 12usize) & 0x0f;
        Tps0::from_bits(val as u8)
    }
    #[doc = "DMA Channel 0 Transmit Process State."]
    #[inline(always)]
    pub const fn set_tps0(&mut self, val: Tps0) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "DMA Channel 1 Receive Process State."]
    #[must_use]
    #[inline(always)]
    pub const fn rps1(&self) -> Rps1 {
        let val = (self.0 >> 16usize) & 0x0f;
        Rps1::from_bits(val as u8)
    }
    #[doc = "DMA Channel 1 Receive Process State."]
    #[inline(always)]
    pub const fn set_rps1(&mut self, val: Rps1) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "DMA Channel 1 Transmit Process State."]
    #[must_use]
    #[inline(always)]
    pub const fn tps1(&self) -> Tps1 {
        let val = (self.0 >> 20usize) & 0x0f;
        Tps1::from_bits(val as u8)
    }
    #[doc = "DMA Channel 1 Transmit Process State."]
    #[inline(always)]
    pub const fn set_tps1(&mut self, val: Tps1) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for DmaDebugStatus0 {
    #[inline(always)]
    fn default() -> DmaDebugStatus0 {
        DmaDebugStatus0(0)
    }
}
impl core::fmt::Debug for DmaDebugStatus0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaDebugStatus0")
            .field("axwhsts", &self.axwhsts())
            .field("rps0", &self.rps0())
            .field("tps0", &self.tps0())
            .field("rps1", &self.rps1())
            .field("tps1", &self.tps1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaDebugStatus0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaDebugStatus0 {{ axwhsts: {=bool:?}, rps0: {:?}, tps0: {:?}, rps1: {:?}, tps1: {:?} }}",
            self.axwhsts(),
            self.rps0(),
            self.tps0(),
            self.rps1(),
            self.tps1()
        )
    }
}
#[doc = "DMA Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaInterruptStatus(pub u32);
impl DmaInterruptStatus {
    #[doc = "DMA Channel 0 Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dc0is(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel 0 Interrupt Status."]
    #[inline(always)]
    pub const fn set_dc0is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Channel 1 Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dc1is(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel 1 Interrupt Status."]
    #[inline(always)]
    pub const fn set_dc1is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MTL Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn mtlis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Interrupt Status."]
    #[inline(always)]
    pub const fn set_mtlis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn macis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Interrupt Status."]
    #[inline(always)]
    pub const fn set_macis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for DmaInterruptStatus {
    #[inline(always)]
    fn default() -> DmaInterruptStatus {
        DmaInterruptStatus(0)
    }
}
impl core::fmt::Debug for DmaInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaInterruptStatus")
            .field("dc0is", &self.dc0is())
            .field("dc1is", &self.dc1is())
            .field("mtlis", &self.mtlis())
            .field("macis", &self.macis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaInterruptStatus {{ dc0is: {=bool:?}, dc1is: {=bool:?}, mtlis: {=bool:?}, macis: {=bool:?} }}",
            self.dc0is(),
            self.dc1is(),
            self.mtlis(),
            self.macis()
        )
    }
}
#[doc = "DMA Bus Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaMode(pub u32);
impl DmaMode {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Tx or Rx Arbitration Scheme."]
    #[must_use]
    #[inline(always)]
    pub const fn da(&self) -> Da {
        let val = (self.0 >> 1usize) & 0x01;
        Da::from_bits(val as u8)
    }
    #[doc = "DMA Tx or Rx Arbitration Scheme."]
    #[inline(always)]
    pub const fn set_da(&mut self, val: Da) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Arbitration Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn taa(&self) -> Taa {
        let val = (self.0 >> 2usize) & 0x07;
        Taa::from_bits(val as u8)
    }
    #[doc = "Transmit Arbitration Algorithm."]
    #[inline(always)]
    pub const fn set_taa(&mut self, val: Taa) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "Transmit Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn txpr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Priority."]
    #[inline(always)]
    pub const fn set_txpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Priority Ratio."]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> DmaModePr {
        let val = (self.0 >> 12usize) & 0x07;
        DmaModePr::from_bits(val as u8)
    }
    #[doc = "Priority Ratio."]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: DmaModePr) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
}
impl Default for DmaMode {
    #[inline(always)]
    fn default() -> DmaMode {
        DmaMode(0)
    }
}
impl core::fmt::Debug for DmaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaMode")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("taa", &self.taa())
            .field("txpr", &self.txpr())
            .field("pr", &self.pr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaMode {{ swr: {=bool:?}, da: {:?}, taa: {:?}, txpr: {=bool:?}, pr: {:?} }}",
            self.swr(),
            self.da(),
            self.taa(),
            self.txpr(),
            self.pr()
        )
    }
}
#[doc = "DMA System Bus Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaSysbusMode(pub u32);
impl DmaSysbusMode {
    #[doc = "Fixed Burst Length."]
    #[must_use]
    #[inline(always)]
    pub const fn fb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed Burst Length."]
    #[inline(always)]
    pub const fn set_fb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address-Aligned Beats."]
    #[must_use]
    #[inline(always)]
    pub const fn aal(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Address-Aligned Beats."]
    #[inline(always)]
    pub const fn set_aal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Mixed Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn mb(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Mixed Burst."]
    #[inline(always)]
    pub const fn set_mb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Rebuild INCRx Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn rb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Rebuild INCRx Burst."]
    #[inline(always)]
    pub const fn set_rb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for DmaSysbusMode {
    #[inline(always)]
    fn default() -> DmaSysbusMode {
        DmaSysbusMode(0)
    }
}
impl core::fmt::Debug for DmaSysbusMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaSysbusMode")
            .field("fb", &self.fb())
            .field("aal", &self.aal())
            .field("mb", &self.mb())
            .field("rb", &self.rb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaSysbusMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaSysbusMode {{ fb: {=bool:?}, aal: {=bool:?}, mb: {=bool:?}, rb: {=bool:?} }}",
            self.fb(),
            self.aal(),
            self.mb(),
            self.rb()
        )
    }
}
#[doc = "Indirect Access Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IndirAccessCtrl(pub u32);
impl IndirAccessCtrl {
    #[doc = "Operation Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn ob(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Operation Busy."]
    #[inline(always)]
    pub const fn set_ob(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command type."]
    #[must_use]
    #[inline(always)]
    pub const fn com(&self) -> Com {
        let val = (self.0 >> 1usize) & 0x01;
        Com::from_bits(val as u8)
    }
    #[doc = "Command type."]
    #[inline(always)]
    pub const fn set_com(&mut self, val: Com) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto increment."]
    #[must_use]
    #[inline(always)]
    pub const fn auto(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Auto increment."]
    #[inline(always)]
    pub const fn set_auto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Address Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn aoff(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Address Offset."]
    #[inline(always)]
    pub const fn set_aoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Mode Select."]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for IndirAccessCtrl {
    #[inline(always)]
    fn default() -> IndirAccessCtrl {
        IndirAccessCtrl(0)
    }
}
impl core::fmt::Debug for IndirAccessCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IndirAccessCtrl")
            .field("ob", &self.ob())
            .field("com", &self.com())
            .field("auto", &self.auto())
            .field("aoff", &self.aoff())
            .field("msel", &self.msel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IndirAccessCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IndirAccessCtrl {{ ob: {=bool:?}, com: {:?}, auto: {=bool:?}, aoff: {=u8:?}, msel: {=u8:?} }}",
            self.ob(),
            self.com(),
            self.auto(),
            self.aoff(),
            self.msel()
        )
    }
}
#[doc = "Indirect Access Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IndirAccessData(pub u32);
impl IndirAccessData {
    #[doc = "This field contains data to read/write for Indirect address access associated with MAC_Indir_Access_Ctrl."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field contains data to read/write for Indirect address access associated with MAC_Indir_Access_Ctrl."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IndirAccessData {
    #[inline(always)]
    fn default() -> IndirAccessData {
        IndirAccessData(0)
    }
}
impl core::fmt::Debug for IndirAccessData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IndirAccessData")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IndirAccessData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IndirAccessData {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "MAC Address0 High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAddress0High(pub u32);
impl MacAddress0High {
    #[doc = "MAC Address0\\[47:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn addrhi(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC Address0\\[47:32\\]."]
    #[inline(always)]
    pub const fn set_addrhi(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "DMA Channel Select."]
    #[must_use]
    #[inline(always)]
    pub const fn dcs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "DMA Channel Select."]
    #[inline(always)]
    pub const fn set_dcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Address Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ae(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Address Enable."]
    #[inline(always)]
    pub const fn set_ae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacAddress0High {
    #[inline(always)]
    fn default() -> MacAddress0High {
        MacAddress0High(0)
    }
}
impl core::fmt::Debug for MacAddress0High {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAddress0High")
            .field("addrhi", &self.addrhi())
            .field("dcs", &self.dcs())
            .field("ae", &self.ae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAddress0High {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacAddress0High {{ addrhi: {=u16:?}, dcs: {=u8:?}, ae: {=bool:?} }}",
            self.addrhi(),
            self.dcs(),
            self.ae()
        )
    }
}
#[doc = "MAC Address0 Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacAddress0Low(pub u32);
impl MacAddress0Low {
    #[doc = "MAC Address0\\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn addrlo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "MAC Address0\\[31:0\\]."]
    #[inline(always)]
    pub const fn set_addrlo(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacAddress0Low {
    #[inline(always)]
    fn default() -> MacAddress0Low {
        MacAddress0Low(0)
    }
}
impl core::fmt::Debug for MacAddress0Low {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacAddress0Low")
            .field("addrlo", &self.addrlo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacAddress0Low {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacAddress0Low {{ addrlo: {=u32:?} }}", self.addrlo())
    }
}
#[doc = "MAC Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacConfiguration(pub u32);
impl MacConfiguration {
    #[doc = "Receiver Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable."]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Preamble Length for Transmit packets."]
    #[must_use]
    #[inline(always)]
    pub const fn prelen(&self) -> Prelen {
        let val = (self.0 >> 2usize) & 0x03;
        Prelen::from_bits(val as u8)
    }
    #[doc = "Preamble Length for Transmit packets."]
    #[inline(always)]
    pub const fn set_prelen(&mut self, val: Prelen) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Deferral Check."]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Deferral Check."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Back-Off Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> Bl {
        let val = (self.0 >> 5usize) & 0x03;
        Bl::from_bits(val as u8)
    }
    #[doc = "Back-Off Limit."]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: Bl) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Disable Retry."]
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self) -> Dr {
        let val = (self.0 >> 8usize) & 0x01;
        Dr::from_bits(val as u8)
    }
    #[doc = "Disable Retry."]
    #[inline(always)]
    pub const fn set_dr(&mut self, val: Dr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Disable Carrier Sense During Transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn dcrs(&self) -> Dcrs {
        let val = (self.0 >> 9usize) & 0x01;
        Dcrs::from_bits(val as u8)
    }
    #[doc = "Disable Carrier Sense During Transmission."]
    #[inline(always)]
    pub const fn set_dcrs(&mut self, val: Dcrs) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Disable Receive Own."]
    #[must_use]
    #[inline(always)]
    pub const fn do_(&self) -> Do {
        let val = (self.0 >> 10usize) & 0x01;
        Do::from_bits(val as u8)
    }
    #[doc = "Disable Receive Own."]
    #[inline(always)]
    pub const fn set_do_(&mut self, val: Do) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ecrsfd(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode."]
    #[inline(always)]
    pub const fn set_ecrsfd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Loopback Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback Mode."]
    #[inline(always)]
    pub const fn set_lm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Duplex Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dm(&self) -> Dm {
        let val = (self.0 >> 13usize) & 0x01;
        Dm::from_bits(val as u8)
    }
    #[doc = "Duplex Mode."]
    #[inline(always)]
    pub const fn set_dm(&mut self, val: Dm) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn fes(&self) -> Fes {
        let val = (self.0 >> 14usize) & 0x01;
        Fes::from_bits(val as u8)
    }
    #[doc = "Speed."]
    #[inline(always)]
    pub const fn set_fes(&mut self, val: Fes) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Port Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> Ps {
        let val = (self.0 >> 15usize) & 0x01;
        Ps::from_bits(val as u8)
    }
    #[doc = "Port Select."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: Ps) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Jumbo Packet Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn je(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Jumbo Packet Enable."]
    #[inline(always)]
    pub const fn set_je(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Jabber Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn jd(&self) -> Jd {
        let val = (self.0 >> 17usize) & 0x01;
        Jd::from_bits(val as u8)
    }
    #[doc = "Jabber Disable."]
    #[inline(always)]
    pub const fn set_jd(&mut self, val: Jd) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Watchdog Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn wd(&self) -> Wd {
        let val = (self.0 >> 19usize) & 0x01;
        Wd::from_bits(val as u8)
    }
    #[doc = "Watchdog Disable."]
    #[inline(always)]
    pub const fn set_wd(&mut self, val: Wd) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Automatic Pad or CRC Stripping."]
    #[must_use]
    #[inline(always)]
    pub const fn acs(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic Pad or CRC Stripping."]
    #[inline(always)]
    pub const fn set_acs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "CRC stripping for Type packets."]
    #[must_use]
    #[inline(always)]
    pub const fn cst(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "CRC stripping for Type packets."]
    #[inline(always)]
    pub const fn set_cst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "IEEE 802.3as Support for 2K Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn s2kp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 802.3as Support for 2K Packets."]
    #[inline(always)]
    pub const fn set_s2kp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Giant Packet Size Limit Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gpslce(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Giant Packet Size Limit Control Enable."]
    #[inline(always)]
    pub const fn set_gpslce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Inter-Packet Gap."]
    #[must_use]
    #[inline(always)]
    pub const fn ipg(&self) -> Ipg {
        let val = (self.0 >> 24usize) & 0x07;
        Ipg::from_bits(val as u8)
    }
    #[doc = "Inter-Packet Gap."]
    #[inline(always)]
    pub const fn set_ipg(&mut self, val: Ipg) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Checksum Offload."]
    #[must_use]
    #[inline(always)]
    pub const fn ipc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Checksum Offload."]
    #[inline(always)]
    pub const fn set_ipc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Source Address Insertion or Replacement Control."]
    #[must_use]
    #[inline(always)]
    pub const fn sarc(&self) -> Sarc {
        let val = (self.0 >> 28usize) & 0x07;
        Sarc::from_bits(val as u8)
    }
    #[doc = "Source Address Insertion or Replacement Control."]
    #[inline(always)]
    pub const fn set_sarc(&mut self, val: Sarc) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for MacConfiguration {
    #[inline(always)]
    fn default() -> MacConfiguration {
        MacConfiguration(0)
    }
}
impl core::fmt::Debug for MacConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacConfiguration")
            .field("re", &self.re())
            .field("te", &self.te())
            .field("prelen", &self.prelen())
            .field("dc", &self.dc())
            .field("bl", &self.bl())
            .field("dr", &self.dr())
            .field("dcrs", &self.dcrs())
            .field("do_", &self.do_())
            .field("ecrsfd", &self.ecrsfd())
            .field("lm", &self.lm())
            .field("dm", &self.dm())
            .field("fes", &self.fes())
            .field("ps", &self.ps())
            .field("je", &self.je())
            .field("jd", &self.jd())
            .field("wd", &self.wd())
            .field("acs", &self.acs())
            .field("cst", &self.cst())
            .field("s2kp", &self.s2kp())
            .field("gpslce", &self.gpslce())
            .field("ipg", &self.ipg())
            .field("ipc", &self.ipc())
            .field("sarc", &self.sarc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacConfiguration {{ re: {=bool:?}, te: {=bool:?}, prelen: {:?}, dc: {=bool:?}, bl: {:?}, dr: {:?}, dcrs: {:?}, do_: {:?}, ecrsfd: {=bool:?}, lm: {=bool:?}, dm: {:?}, fes: {:?}, ps: {:?}, je: {=bool:?}, jd: {:?}, wd: {:?}, acs: {=bool:?}, cst: {=bool:?}, s2kp: {=bool:?}, gpslce: {=bool:?}, ipg: {:?}, ipc: {=bool:?}, sarc: {:?} }}",
            self.re(),
            self.te(),
            self.prelen(),
            self.dc(),
            self.bl(),
            self.dr(),
            self.dcrs(),
            self.do_(),
            self.ecrsfd(),
            self.lm(),
            self.dm(),
            self.fes(),
            self.ps(),
            self.je(),
            self.jd(),
            self.wd(),
            self.acs(),
            self.cst(),
            self.s2kp(),
            self.gpslce(),
            self.ipg(),
            self.ipc(),
            self.sarc()
        )
    }
}
#[doc = "CSR Software Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacCsrSwCtrl(pub u32);
impl MacCsrSwCtrl {
    #[doc = "Register Clear on Write 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rcwe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Clear on Write 1 Enable."]
    #[inline(always)]
    pub const fn set_rcwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for MacCsrSwCtrl {
    #[inline(always)]
    fn default() -> MacCsrSwCtrl {
        MacCsrSwCtrl(0)
    }
}
impl core::fmt::Debug for MacCsrSwCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacCsrSwCtrl")
            .field("rcwe", &self.rcwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacCsrSwCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacCsrSwCtrl {{ rcwe: {=bool:?} }}", self.rcwe())
    }
}
#[doc = "MAC Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacDebug(pub u32);
impl MacDebug {
    #[doc = "MAC GMII or MII Receive Protocol Engine Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rpests(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MAC GMII or MII Receive Protocol Engine Status."]
    #[inline(always)]
    pub const fn set_rpests(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MAC Receive Packet Controller FIFO Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rfcfcsts(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "MAC Receive Packet Controller FIFO Status."]
    #[inline(always)]
    pub const fn set_rfcfcsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tpests(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status."]
    #[inline(always)]
    pub const fn set_tpests(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Transmit Packet Controller Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tfcsts(&self) -> Tfcsts {
        let val = (self.0 >> 17usize) & 0x03;
        Tfcsts::from_bits(val as u8)
    }
    #[doc = "MAC Transmit Packet Controller Status."]
    #[inline(always)]
    pub const fn set_tfcsts(&mut self, val: Tfcsts) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
}
impl Default for MacDebug {
    #[inline(always)]
    fn default() -> MacDebug {
        MacDebug(0)
    }
}
impl core::fmt::Debug for MacDebug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacDebug")
            .field("rpests", &self.rpests())
            .field("rfcfcsts", &self.rfcfcsts())
            .field("tpests", &self.tpests())
            .field("tfcsts", &self.tfcsts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacDebug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacDebug {{ rpests: {=bool:?}, rfcfcsts: {=u8:?}, tpests: {=bool:?}, tfcsts: {:?} }}",
            self.rpests(),
            self.rfcfcsts(),
            self.tpests(),
            self.tfcsts()
        )
    }
}
#[doc = "MAC Extended Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacExtConfiguration(pub u32);
impl MacExtConfiguration {
    #[doc = "Giant Packet Size Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn gpsl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Giant Packet Size Limit."]
    #[inline(always)]
    pub const fn set_gpsl(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Disable CRC Checking for Received Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn dcrcc(&self) -> Dcrcc {
        let val = (self.0 >> 16usize) & 0x01;
        Dcrcc::from_bits(val as u8)
    }
    #[doc = "Disable CRC Checking for Received Packets."]
    #[inline(always)]
    pub const fn set_dcrcc(&mut self, val: Dcrcc) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Slow Protocol Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slow Protocol Detection Enable."]
    #[inline(always)]
    pub const fn set_spen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Unicast Slow Protocol Packet Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn usp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Slow Protocol Packet Detect."]
    #[inline(always)]
    pub const fn set_usp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Packet Duplication Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pdc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Packet Duplication Control."]
    #[inline(always)]
    pub const fn set_pdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Extended Inter-Packet Gap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eipgen(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Inter-Packet Gap Enable."]
    #[inline(always)]
    pub const fn set_eipgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Extended Inter-Packet Gap."]
    #[must_use]
    #[inline(always)]
    pub const fn eipg(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Inter-Packet Gap."]
    #[inline(always)]
    pub const fn set_eipg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for MacExtConfiguration {
    #[inline(always)]
    fn default() -> MacExtConfiguration {
        MacExtConfiguration(0)
    }
}
impl core::fmt::Debug for MacExtConfiguration {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacExtConfiguration")
            .field("gpsl", &self.gpsl())
            .field("dcrcc", &self.dcrcc())
            .field("spen", &self.spen())
            .field("usp", &self.usp())
            .field("pdc", &self.pdc())
            .field("eipgen", &self.eipgen())
            .field("eipg", &self.eipg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacExtConfiguration {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacExtConfiguration {{ gpsl: {=u16:?}, dcrcc: {:?}, spen: {=bool:?}, usp: {=bool:?}, pdc: {=bool:?}, eipgen: {=bool:?}, eipg: {=u8:?} }}",
            self.gpsl(),
            self.dcrcc(),
            self.spen(),
            self.usp(),
            self.pdc(),
            self.eipgen(),
            self.eipg()
        )
    }
}
#[doc = "Hardware Features 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature0(pub u32);
impl MacHwFeature0 {
    #[doc = "10 or 100 Mbps Support."]
    #[must_use]
    #[inline(always)]
    pub const fn miisel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "10 or 100 Mbps Support."]
    #[inline(always)]
    pub const fn set_miisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1000 Mbps Support."]
    #[must_use]
    #[inline(always)]
    pub const fn gmiisel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1000 Mbps Support."]
    #[inline(always)]
    pub const fn set_gmiisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Half-duplex Support."]
    #[must_use]
    #[inline(always)]
    pub const fn hdsel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Half-duplex Support."]
    #[inline(always)]
    pub const fn set_hdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)."]
    #[must_use]
    #[inline(always)]
    pub const fn pcssel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)."]
    #[inline(always)]
    pub const fn set_pcssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VLAN Hash Filter Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn vlhash(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Hash Filter Selected."]
    #[inline(always)]
    pub const fn set_vlhash(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SMA (MDIO) Interface."]
    #[must_use]
    #[inline(always)]
    pub const fn smasel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SMA (MDIO) Interface."]
    #[inline(always)]
    pub const fn set_smasel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PMT Remote Wake-up Packet Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwksel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Remote Wake-up Packet Enable."]
    #[inline(always)]
    pub const fn set_rwksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PMT Magic Packet Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mgksel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Magic Packet Enable."]
    #[inline(always)]
    pub const fn set_mgksel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RMON Module Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mmcsel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "RMON Module Enable."]
    #[inline(always)]
    pub const fn set_mmcsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ARP Offload Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn arpoffsel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "ARP Offload Enabled."]
    #[inline(always)]
    pub const fn set_arpoffsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn tssel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled."]
    #[inline(always)]
    pub const fn set_tssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Energy Efficient Ethernet Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn eeesel(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Energy Efficient Ethernet Enabled."]
    #[inline(always)]
    pub const fn set_eeesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmit Checksum Offload Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn txcoesel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Checksum Offload Enabled."]
    #[inline(always)]
    pub const fn set_txcoesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Receive Checksum Offload Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcoesel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Checksum Offload Enabled."]
    #[inline(always)]
    pub const fn set_rxcoesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "MAC Addresses 1-31 Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn addmacadrsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "MAC Addresses 1-31 Selected."]
    #[inline(always)]
    pub const fn set_addmacadrsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "MAC Addresses 32-63 Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn macadr32sel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Addresses 32-63 Selected."]
    #[inline(always)]
    pub const fn set_macadr32sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC Addresses 64-127 Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn macadr64sel(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Addresses 64-127 Selected."]
    #[inline(always)]
    pub const fn set_macadr64sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Timestamp System Time Source."]
    #[must_use]
    #[inline(always)]
    pub const fn tsstssel(&self) -> Tsstssel {
        let val = (self.0 >> 25usize) & 0x03;
        Tsstssel::from_bits(val as u8)
    }
    #[doc = "Timestamp System Time Source."]
    #[inline(always)]
    pub const fn set_tsstssel(&mut self, val: Tsstssel) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Source Address or VLAN Insertion Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn savlanins(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address or VLAN Insertion Enable."]
    #[inline(always)]
    pub const fn set_savlanins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Active PHY Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn actphysel(&self) -> Actphysel {
        let val = (self.0 >> 28usize) & 0x07;
        Actphysel::from_bits(val as u8)
    }
    #[doc = "Active PHY Selected."]
    #[inline(always)]
    pub const fn set_actphysel(&mut self, val: Actphysel) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for MacHwFeature0 {
    #[inline(always)]
    fn default() -> MacHwFeature0 {
        MacHwFeature0(0)
    }
}
impl core::fmt::Debug for MacHwFeature0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature0")
            .field("miisel", &self.miisel())
            .field("gmiisel", &self.gmiisel())
            .field("hdsel", &self.hdsel())
            .field("pcssel", &self.pcssel())
            .field("vlhash", &self.vlhash())
            .field("smasel", &self.smasel())
            .field("rwksel", &self.rwksel())
            .field("mgksel", &self.mgksel())
            .field("mmcsel", &self.mmcsel())
            .field("arpoffsel", &self.arpoffsel())
            .field("tssel", &self.tssel())
            .field("eeesel", &self.eeesel())
            .field("txcoesel", &self.txcoesel())
            .field("rxcoesel", &self.rxcoesel())
            .field("addmacadrsel", &self.addmacadrsel())
            .field("macadr32sel", &self.macadr32sel())
            .field("macadr64sel", &self.macadr64sel())
            .field("tsstssel", &self.tsstssel())
            .field("savlanins", &self.savlanins())
            .field("actphysel", &self.actphysel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature0 {{ miisel: {=bool:?}, gmiisel: {=bool:?}, hdsel: {=bool:?}, pcssel: {=bool:?}, vlhash: {=bool:?}, smasel: {=bool:?}, rwksel: {=bool:?}, mgksel: {=bool:?}, mmcsel: {=bool:?}, arpoffsel: {=bool:?}, tssel: {=bool:?}, eeesel: {=bool:?}, txcoesel: {=bool:?}, rxcoesel: {=bool:?}, addmacadrsel: {=u8:?}, macadr32sel: {=bool:?}, macadr64sel: {=bool:?}, tsstssel: {:?}, savlanins: {=bool:?}, actphysel: {:?} }}",
            self.miisel(),
            self.gmiisel(),
            self.hdsel(),
            self.pcssel(),
            self.vlhash(),
            self.smasel(),
            self.rwksel(),
            self.mgksel(),
            self.mmcsel(),
            self.arpoffsel(),
            self.tssel(),
            self.eeesel(),
            self.txcoesel(),
            self.rxcoesel(),
            self.addmacadrsel(),
            self.macadr32sel(),
            self.macadr64sel(),
            self.tsstssel(),
            self.savlanins(),
            self.actphysel()
        )
    }
}
#[doc = "Hardware Features 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature1(pub u32);
impl MacHwFeature1 {
    #[doc = "MTL Receive FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> Rxfifosize {
        let val = (self.0 >> 0usize) & 0x1f;
        Rxfifosize::from_bits(val as u8)
    }
    #[doc = "MTL Receive FIFO Size."]
    #[inline(always)]
    pub const fn set_rxfifosize(&mut self, val: Rxfifosize) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Single Port RAM Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn spram(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Single Port RAM Enable."]
    #[inline(always)]
    pub const fn set_spram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "MTL Transmit FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifosize(&self) -> Txfifosize {
        let val = (self.0 >> 6usize) & 0x1f;
        Txfifosize::from_bits(val as u8)
    }
    #[doc = "MTL Transmit FIFO Size."]
    #[inline(always)]
    pub const fn set_txfifosize(&mut self, val: Txfifosize) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val.to_bits() as u32) & 0x1f) << 6usize);
    }
    #[doc = "One-Step Timestamping Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn osten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "One-Step Timestamping Enable."]
    #[inline(always)]
    pub const fn set_osten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "PTP Offload Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ptoen(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "PTP Offload Enable."]
    #[inline(always)]
    pub const fn set_ptoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IEEE 1588 High Word Register Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn advthword(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE 1588 High Word Register Enable."]
    #[inline(always)]
    pub const fn set_advthword(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Address Width."]
    #[must_use]
    #[inline(always)]
    pub const fn addr64(&self) -> Addr64 {
        let val = (self.0 >> 14usize) & 0x03;
        Addr64::from_bits(val as u8)
    }
    #[doc = "Address Width."]
    #[inline(always)]
    pub const fn set_addr64(&mut self, val: Addr64) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "DCB Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcben(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DCB Feature Enable."]
    #[inline(always)]
    pub const fn set_dcben(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Split Header Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sphen(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Split Header Feature Enable."]
    #[inline(always)]
    pub const fn set_sphen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TCP Segmentation Offload Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsoen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "TCP Segmentation Offload Enable."]
    #[inline(always)]
    pub const fn set_tsoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DMA Debug Registers Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgmema(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Debug Registers Enable."]
    #[inline(always)]
    pub const fn set_dbgmema(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "AV Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn avsel(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "AV Feature Enable."]
    #[inline(always)]
    pub const fn set_avsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Rx Side Only AV Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ravsel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Side Only AV Feature Enable."]
    #[inline(always)]
    pub const fn set_ravsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pouost(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable."]
    #[inline(always)]
    pub const fn set_pouost(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Hash Table Size."]
    #[must_use]
    #[inline(always)]
    pub const fn hashtblsz(&self) -> Hashtblsz {
        let val = (self.0 >> 24usize) & 0x03;
        Hashtblsz::from_bits(val as u8)
    }
    #[doc = "Hash Table Size."]
    #[inline(always)]
    pub const fn set_hashtblsz(&mut self, val: Hashtblsz) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Total number of L3 or L4 Filters."]
    #[must_use]
    #[inline(always)]
    pub const fn l3l4fnum(&self) -> L3l4fnum {
        let val = (self.0 >> 27usize) & 0x0f;
        L3l4fnum::from_bits(val as u8)
    }
    #[doc = "Total number of L3 or L4 Filters."]
    #[inline(always)]
    pub const fn set_l3l4fnum(&mut self, val: L3l4fnum) {
        self.0 = (self.0 & !(0x0f << 27usize)) | (((val.to_bits() as u32) & 0x0f) << 27usize);
    }
}
impl Default for MacHwFeature1 {
    #[inline(always)]
    fn default() -> MacHwFeature1 {
        MacHwFeature1(0)
    }
}
impl core::fmt::Debug for MacHwFeature1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature1")
            .field("rxfifosize", &self.rxfifosize())
            .field("spram", &self.spram())
            .field("txfifosize", &self.txfifosize())
            .field("osten", &self.osten())
            .field("ptoen", &self.ptoen())
            .field("advthword", &self.advthword())
            .field("addr64", &self.addr64())
            .field("dcben", &self.dcben())
            .field("sphen", &self.sphen())
            .field("tsoen", &self.tsoen())
            .field("dbgmema", &self.dbgmema())
            .field("avsel", &self.avsel())
            .field("ravsel", &self.ravsel())
            .field("pouost", &self.pouost())
            .field("hashtblsz", &self.hashtblsz())
            .field("l3l4fnum", &self.l3l4fnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature1 {{ rxfifosize: {:?}, spram: {=bool:?}, txfifosize: {:?}, osten: {=bool:?}, ptoen: {=bool:?}, advthword: {=bool:?}, addr64: {:?}, dcben: {=bool:?}, sphen: {=bool:?}, tsoen: {=bool:?}, dbgmema: {=bool:?}, avsel: {=bool:?}, ravsel: {=bool:?}, pouost: {=bool:?}, hashtblsz: {:?}, l3l4fnum: {:?} }}",
            self.rxfifosize(),
            self.spram(),
            self.txfifosize(),
            self.osten(),
            self.ptoen(),
            self.advthword(),
            self.addr64(),
            self.dcben(),
            self.sphen(),
            self.tsoen(),
            self.dbgmema(),
            self.avsel(),
            self.ravsel(),
            self.pouost(),
            self.hashtblsz(),
            self.l3l4fnum()
        )
    }
}
#[doc = "Hardware Features 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature2(pub u32);
impl MacHwFeature2 {
    #[doc = "Number of MTL Receive Queues."]
    #[must_use]
    #[inline(always)]
    pub const fn rxqcnt(&self) -> Rxqcnt {
        let val = (self.0 >> 0usize) & 0x0f;
        Rxqcnt::from_bits(val as u8)
    }
    #[doc = "Number of MTL Receive Queues."]
    #[inline(always)]
    pub const fn set_rxqcnt(&mut self, val: Rxqcnt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of MTL Transmit Queues."]
    #[must_use]
    #[inline(always)]
    pub const fn txqcnt(&self) -> Txqcnt {
        let val = (self.0 >> 6usize) & 0x0f;
        Txqcnt::from_bits(val as u8)
    }
    #[doc = "Number of MTL Transmit Queues."]
    #[inline(always)]
    pub const fn set_txqcnt(&mut self, val: Txqcnt) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val.to_bits() as u32) & 0x0f) << 6usize);
    }
    #[doc = "Number of DMA Receive Channels."]
    #[must_use]
    #[inline(always)]
    pub const fn rxchcnt(&self) -> Rxchcnt {
        let val = (self.0 >> 12usize) & 0x0f;
        Rxchcnt::from_bits(val as u8)
    }
    #[doc = "Number of DMA Receive Channels."]
    #[inline(always)]
    pub const fn set_rxchcnt(&mut self, val: Rxchcnt) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Rx DMA Descriptor Cache Size in terms of 16 bytes descriptors:."]
    #[must_use]
    #[inline(always)]
    pub const fn rdcsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Rx DMA Descriptor Cache Size in terms of 16 bytes descriptors:."]
    #[inline(always)]
    pub const fn set_rdcsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Number of DMA Transmit Channels."]
    #[must_use]
    #[inline(always)]
    pub const fn txchcnt(&self) -> Txchcnt {
        let val = (self.0 >> 18usize) & 0x0f;
        Txchcnt::from_bits(val as u8)
    }
    #[doc = "Number of DMA Transmit Channels."]
    #[inline(always)]
    pub const fn set_txchcnt(&mut self, val: Txchcnt) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
    }
    #[doc = "Tx DMA Descriptor Cache Size in terms of 16 bytes descriptors:."]
    #[must_use]
    #[inline(always)]
    pub const fn tdcsz(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Tx DMA Descriptor Cache Size in terms of 16 bytes descriptors:."]
    #[inline(always)]
    pub const fn set_tdcsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Number of PPS Outputs."]
    #[must_use]
    #[inline(always)]
    pub const fn ppsoutnum(&self) -> Ppsoutnum {
        let val = (self.0 >> 24usize) & 0x07;
        Ppsoutnum::from_bits(val as u8)
    }
    #[doc = "Number of PPS Outputs."]
    #[inline(always)]
    pub const fn set_ppsoutnum(&mut self, val: Ppsoutnum) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "Number of Auxiliary Snapshot Inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn auxsnapnum(&self) -> Auxsnapnum {
        let val = (self.0 >> 28usize) & 0x07;
        Auxsnapnum::from_bits(val as u8)
    }
    #[doc = "Number of Auxiliary Snapshot Inputs."]
    #[inline(always)]
    pub const fn set_auxsnapnum(&mut self, val: Auxsnapnum) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
}
impl Default for MacHwFeature2 {
    #[inline(always)]
    fn default() -> MacHwFeature2 {
        MacHwFeature2(0)
    }
}
impl core::fmt::Debug for MacHwFeature2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature2")
            .field("rxqcnt", &self.rxqcnt())
            .field("txqcnt", &self.txqcnt())
            .field("rxchcnt", &self.rxchcnt())
            .field("rdcsz", &self.rdcsz())
            .field("txchcnt", &self.txchcnt())
            .field("tdcsz", &self.tdcsz())
            .field("ppsoutnum", &self.ppsoutnum())
            .field("auxsnapnum", &self.auxsnapnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature2 {{ rxqcnt: {:?}, txqcnt: {:?}, rxchcnt: {:?}, rdcsz: {=u8:?}, txchcnt: {:?}, tdcsz: {=u8:?}, ppsoutnum: {:?}, auxsnapnum: {:?} }}",
            self.rxqcnt(),
            self.txqcnt(),
            self.rxchcnt(),
            self.rdcsz(),
            self.txchcnt(),
            self.tdcsz(),
            self.ppsoutnum(),
            self.auxsnapnum()
        )
    }
}
#[doc = "Hardware Features 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacHwFeature3(pub u32);
impl MacHwFeature3 {
    #[doc = "Number of Extended VLAN Tag Filters Enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn nrvf(&self) -> Nrvf {
        let val = (self.0 >> 0usize) & 0x07;
        Nrvf::from_bits(val as u8)
    }
    #[doc = "Number of Extended VLAN Tag Filters Enabled."]
    #[inline(always)]
    pub const fn set_nrvf(&mut self, val: Nrvf) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cbtisel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable."]
    #[inline(always)]
    pub const fn set_cbtisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Double VLAN Tag Processing Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn dvlan(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Double VLAN Tag Processing Selected."]
    #[inline(always)]
    pub const fn set_dvlan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Broadcast/Multicast Packet Duplication."]
    #[must_use]
    #[inline(always)]
    pub const fn pdupsel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Broadcast/Multicast Packet Duplication."]
    #[inline(always)]
    pub const fn set_pdupsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Flexible Receive Parser Selected."]
    #[must_use]
    #[inline(always)]
    pub const fn frpsel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Flexible Receive Parser Selected."]
    #[inline(always)]
    pub const fn set_frpsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Flexible Receive Parser Buffer size."]
    #[must_use]
    #[inline(always)]
    pub const fn frpbs(&self) -> Frpbs {
        let val = (self.0 >> 11usize) & 0x03;
        Frpbs::from_bits(val as u8)
    }
    #[doc = "Flexible Receive Parser Buffer size."]
    #[inline(always)]
    pub const fn set_frpbs(&mut self, val: Frpbs) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "Flexible Receive Parser Table Entries size."]
    #[must_use]
    #[inline(always)]
    pub const fn frpes(&self) -> Frpes {
        let val = (self.0 >> 13usize) & 0x03;
        Frpes::from_bits(val as u8)
    }
    #[doc = "Flexible Receive Parser Table Entries size."]
    #[inline(always)]
    pub const fn set_frpes(&mut self, val: Frpes) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Enhancements to Scheduled Traffic Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn estsel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhancements to Scheduled Traffic Enable."]
    #[inline(always)]
    pub const fn set_estsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Depth of the Gate Control List."]
    #[must_use]
    #[inline(always)]
    pub const fn estdep(&self) -> Estdep {
        let val = (self.0 >> 17usize) & 0x07;
        Estdep::from_bits(val as u8)
    }
    #[doc = "Depth of the Gate Control List."]
    #[inline(always)]
    pub const fn set_estdep(&mut self, val: Estdep) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Width of the Time Interval field in the Gate Control List."]
    #[must_use]
    #[inline(always)]
    pub const fn estwid(&self) -> Estwid {
        let val = (self.0 >> 20usize) & 0x03;
        Estwid::from_bits(val as u8)
    }
    #[doc = "Width of the Time Interval field in the Gate Control List."]
    #[inline(always)]
    pub const fn set_estwid(&mut self, val: Estwid) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Frame Preemption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fpesel(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Preemption Enable."]
    #[inline(always)]
    pub const fn set_fpesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Time Based Scheduling Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbssel(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Time Based Scheduling Enable."]
    #[inline(always)]
    pub const fn set_tbssel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Automotive Safety Package."]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> Asp {
        let val = (self.0 >> 28usize) & 0x03;
        Asp::from_bits(val as u8)
    }
    #[doc = "Automotive Safety Package."]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: Asp) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for MacHwFeature3 {
    #[inline(always)]
    fn default() -> MacHwFeature3 {
        MacHwFeature3(0)
    }
}
impl core::fmt::Debug for MacHwFeature3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacHwFeature3")
            .field("nrvf", &self.nrvf())
            .field("cbtisel", &self.cbtisel())
            .field("dvlan", &self.dvlan())
            .field("pdupsel", &self.pdupsel())
            .field("frpsel", &self.frpsel())
            .field("frpbs", &self.frpbs())
            .field("frpes", &self.frpes())
            .field("estsel", &self.estsel())
            .field("estdep", &self.estdep())
            .field("estwid", &self.estwid())
            .field("fpesel", &self.fpesel())
            .field("tbssel", &self.tbssel())
            .field("asp", &self.asp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacHwFeature3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacHwFeature3 {{ nrvf: {:?}, cbtisel: {=bool:?}, dvlan: {=bool:?}, pdupsel: {=bool:?}, frpsel: {=bool:?}, frpbs: {:?}, frpes: {:?}, estsel: {=bool:?}, estdep: {:?}, estwid: {:?}, fpesel: {=bool:?}, tbssel: {=bool:?}, asp: {:?} }}",
            self.nrvf(),
            self.cbtisel(),
            self.dvlan(),
            self.pdupsel(),
            self.frpsel(),
            self.frpbs(),
            self.frpes(),
            self.estsel(),
            self.estdep(),
            self.estwid(),
            self.fpesel(),
            self.tbssel(),
            self.asp()
        )
    }
}
#[doc = "MAC Inner VLAN Tag Inclusion or Replacement."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInnerVlanIncl(pub u32);
impl MacInnerVlanIncl {
    #[doc = "VLAN Tag for Transmit Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn vlt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag for Transmit Packets."]
    #[inline(always)]
    pub const fn set_vlt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VLAN Tag Control in Transmit Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn vlc(&self) -> MacInnerVlanInclVlc {
        let val = (self.0 >> 16usize) & 0x03;
        MacInnerVlanInclVlc::from_bits(val as u8)
    }
    #[doc = "VLAN Tag Control in Transmit Packets."]
    #[inline(always)]
    pub const fn set_vlc(&mut self, val: MacInnerVlanInclVlc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "VLAN Priority Control."]
    #[must_use]
    #[inline(always)]
    pub const fn vlp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Priority Control."]
    #[inline(always)]
    pub const fn set_vlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "C-VLAN or S-VLAN."]
    #[must_use]
    #[inline(always)]
    pub const fn csvl(&self) -> MacInnerVlanInclCsvl {
        let val = (self.0 >> 19usize) & 0x01;
        MacInnerVlanInclCsvl::from_bits(val as u8)
    }
    #[doc = "C-VLAN or S-VLAN."]
    #[inline(always)]
    pub const fn set_csvl(&mut self, val: MacInnerVlanInclCsvl) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "VLAN Tag Input."]
    #[must_use]
    #[inline(always)]
    pub const fn vlti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Input."]
    #[inline(always)]
    pub const fn set_vlti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for MacInnerVlanIncl {
    #[inline(always)]
    fn default() -> MacInnerVlanIncl {
        MacInnerVlanIncl(0)
    }
}
impl core::fmt::Debug for MacInnerVlanIncl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInnerVlanIncl")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .field("vlti", &self.vlti())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInnerVlanIncl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInnerVlanIncl {{ vlt: {=u16:?}, vlc: {:?}, vlp: {=bool:?}, csvl: {:?}, vlti: {=bool:?} }}",
            self.vlt(),
            self.vlc(),
            self.vlp(),
            self.csvl(),
            self.vlti()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInterruptEnable(pub u32);
impl MacInterruptEnable {
    #[doc = "PHY Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn phyie(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Interrupt Enable."]
    #[inline(always)]
    pub const fn set_phyie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PMT Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pmtie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PMT Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pmtie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPI Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpiie(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lpiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Timestamp Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txstsie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txstsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxstsie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxstsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MDIO Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mdioie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO Interrupt Enable."]
    #[inline(always)]
    pub const fn set_mdioie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for MacInterruptEnable {
    #[inline(always)]
    fn default() -> MacInterruptEnable {
        MacInterruptEnable(0)
    }
}
impl core::fmt::Debug for MacInterruptEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInterruptEnable")
            .field("phyie", &self.phyie())
            .field("pmtie", &self.pmtie())
            .field("lpiie", &self.lpiie())
            .field("tsie", &self.tsie())
            .field("txstsie", &self.txstsie())
            .field("rxstsie", &self.rxstsie())
            .field("mdioie", &self.mdioie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInterruptEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInterruptEnable {{ phyie: {=bool:?}, pmtie: {=bool:?}, lpiie: {=bool:?}, tsie: {=bool:?}, txstsie: {=bool:?}, rxstsie: {=bool:?}, mdioie: {=bool:?} }}",
            self.phyie(),
            self.pmtie(),
            self.lpiie(),
            self.tsie(),
            self.txstsie(),
            self.rxstsie(),
            self.mdioie()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacInterruptStatus(pub u32);
impl MacInterruptStatus {
    #[doc = "PHY Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn phyis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Interrupt."]
    #[inline(always)]
    pub const fn set_phyis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PMTIS."]
    #[must_use]
    #[inline(always)]
    pub const fn pmtis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PMTIS."]
    #[inline(always)]
    pub const fn set_pmtis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPIIS."]
    #[must_use]
    #[inline(always)]
    pub const fn lpiis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPIIS."]
    #[inline(always)]
    pub const fn set_lpiis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "TSIS."]
    #[must_use]
    #[inline(always)]
    pub const fn tsis(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TSIS."]
    #[inline(always)]
    pub const fn set_tsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Transmit Status Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txstsis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Status Interrupt."]
    #[inline(always)]
    pub const fn set_txstsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Receive Status Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxstsis(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Status Interrupt."]
    #[inline(always)]
    pub const fn set_rxstsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MDIO Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn mdiois(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO Interrupt Status."]
    #[inline(always)]
    pub const fn set_mdiois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for MacInterruptStatus {
    #[inline(always)]
    fn default() -> MacInterruptStatus {
        MacInterruptStatus(0)
    }
}
impl core::fmt::Debug for MacInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacInterruptStatus")
            .field("phyis", &self.phyis())
            .field("pmtis", &self.pmtis())
            .field("lpiis", &self.lpiis())
            .field("tsis", &self.tsis())
            .field("txstsis", &self.txstsis())
            .field("rxstsis", &self.rxstsis())
            .field("mdiois", &self.mdiois())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacInterruptStatus {{ phyis: {=bool:?}, pmtis: {=bool:?}, lpiis: {=bool:?}, tsis: {=bool:?}, txstsis: {=bool:?}, rxstsis: {=bool:?}, mdiois: {=bool:?} }}",
            self.phyis(),
            self.pmtis(),
            self.lpiis(),
            self.tsis(),
            self.txstsis(),
            self.rxstsis(),
            self.mdiois()
        )
    }
}
#[doc = "LPI Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacLpiControlStatus(pub u32);
impl MacLpiControlStatus {
    #[doc = "Transmit LPI Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn tlpien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit LPI Entry."]
    #[inline(always)]
    pub const fn set_tlpien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit LPI Exit."]
    #[must_use]
    #[inline(always)]
    pub const fn tlpiex(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit LPI Exit."]
    #[inline(always)]
    pub const fn set_tlpiex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive LPI Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn rlpien(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive LPI Entry."]
    #[inline(always)]
    pub const fn set_rlpien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive LPI Exit."]
    #[must_use]
    #[inline(always)]
    pub const fn rlpiex(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive LPI Exit."]
    #[inline(always)]
    pub const fn set_rlpiex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit LPI State."]
    #[must_use]
    #[inline(always)]
    pub const fn tlpist(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit LPI State."]
    #[inline(always)]
    pub const fn set_tlpist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive LPI State."]
    #[must_use]
    #[inline(always)]
    pub const fn rlpist(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive LPI State."]
    #[inline(always)]
    pub const fn set_rlpist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPI Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpien(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Enable."]
    #[inline(always)]
    pub const fn set_lpien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "PHY Link Status."]
    #[must_use]
    #[inline(always)]
    pub const fn pls(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "PHY Link Status."]
    #[inline(always)]
    pub const fn set_pls(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI Tx Automate."]
    #[must_use]
    #[inline(always)]
    pub const fn lpitxa(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Tx Automate."]
    #[inline(always)]
    pub const fn set_lpitxa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpiate(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Timer Enable."]
    #[inline(always)]
    pub const fn set_lpiate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI Tx Clock Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lpitcse(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPI Tx Clock Stop Enable."]
    #[inline(always)]
    pub const fn set_lpitcse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for MacLpiControlStatus {
    #[inline(always)]
    fn default() -> MacLpiControlStatus {
        MacLpiControlStatus(0)
    }
}
impl core::fmt::Debug for MacLpiControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacLpiControlStatus")
            .field("tlpien", &self.tlpien())
            .field("tlpiex", &self.tlpiex())
            .field("rlpien", &self.rlpien())
            .field("rlpiex", &self.rlpiex())
            .field("tlpist", &self.tlpist())
            .field("rlpist", &self.rlpist())
            .field("lpien", &self.lpien())
            .field("pls", &self.pls())
            .field("lpitxa", &self.lpitxa())
            .field("lpiate", &self.lpiate())
            .field("lpitcse", &self.lpitcse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacLpiControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacLpiControlStatus {{ tlpien: {=bool:?}, tlpiex: {=bool:?}, rlpien: {=bool:?}, rlpiex: {=bool:?}, tlpist: {=bool:?}, rlpist: {=bool:?}, lpien: {=bool:?}, pls: {=bool:?}, lpitxa: {=bool:?}, lpiate: {=bool:?}, lpitcse: {=bool:?} }}",
            self.tlpien(),
            self.tlpiex(),
            self.rlpien(),
            self.rlpiex(),
            self.tlpist(),
            self.rlpist(),
            self.lpien(),
            self.pls(),
            self.lpitxa(),
            self.lpiate(),
            self.lpitcse()
        )
    }
}
#[doc = "Tx LPI Entry Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacLpiEntryTimer(pub u32);
impl MacLpiEntryTimer {
    #[doc = "LPI Entry Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn lpiet(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "LPI Entry Timer."]
    #[inline(always)]
    pub const fn set_lpiet(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 3usize)) | (((val as u32) & 0x0001_ffff) << 3usize);
    }
}
impl Default for MacLpiEntryTimer {
    #[inline(always)]
    fn default() -> MacLpiEntryTimer {
        MacLpiEntryTimer(0)
    }
}
impl core::fmt::Debug for MacLpiEntryTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacLpiEntryTimer")
            .field("lpiet", &self.lpiet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacLpiEntryTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLpiEntryTimer {{ lpiet: {=u32:?} }}", self.lpiet())
    }
}
#[doc = "LPI Timers Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacLpiTimersControl(pub u32);
impl MacLpiTimersControl {
    #[doc = "LPI TW Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn twt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "LPI TW Timer."]
    #[inline(always)]
    pub const fn set_twt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "LPI LS Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn lst(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "LPI LS Timer."]
    #[inline(always)]
    pub const fn set_lst(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for MacLpiTimersControl {
    #[inline(always)]
    fn default() -> MacLpiTimersControl {
        MacLpiTimersControl(0)
    }
}
impl core::fmt::Debug for MacLpiTimersControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacLpiTimersControl")
            .field("twt", &self.twt())
            .field("lst", &self.lst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacLpiTimersControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacLpiTimersControl {{ twt: {=u16:?}, lst: {=u16:?} }}",
            self.twt(),
            self.lst()
        )
    }
}
#[doc = "MDIO Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacMdioAddress(pub u32);
impl MacMdioAddress {
    #[doc = "GMII Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn gb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Busy."]
    #[inline(always)]
    pub const fn set_gb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clause 45 PHY Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn c45e(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clause 45 PHY Enable."]
    #[inline(always)]
    pub const fn set_c45e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GMII Operation Command 0."]
    #[must_use]
    #[inline(always)]
    pub const fn goc_0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Operation Command 0."]
    #[inline(always)]
    pub const fn set_goc_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GMII Operation Command 1."]
    #[must_use]
    #[inline(always)]
    pub const fn goc_1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "GMII Operation Command 1."]
    #[inline(always)]
    pub const fn set_goc_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Skip Address Packet."]
    #[must_use]
    #[inline(always)]
    pub const fn skap(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Skip Address Packet."]
    #[inline(always)]
    pub const fn set_skap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CR."]
    #[must_use]
    #[inline(always)]
    pub const fn cr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "CR."]
    #[inline(always)]
    pub const fn set_cr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "NTC."]
    #[must_use]
    #[inline(always)]
    pub const fn ntc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "NTC."]
    #[inline(always)]
    pub const fn set_ntc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Register/Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn rda(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Register/Device Address."]
    #[inline(always)]
    pub const fn set_rda(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Physical Layer Address."]
    #[must_use]
    #[inline(always)]
    pub const fn pa(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x1f;
        val as u8
    }
    #[doc = "Physical Layer Address."]
    #[inline(always)]
    pub const fn set_pa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
    }
    #[doc = "Back to Back transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn btb(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Back to Back transactions."]
    #[inline(always)]
    pub const fn set_btb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Preamble Suppression Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pse(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Preamble Suppression Enable."]
    #[inline(always)]
    pub const fn set_pse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MacMdioAddress {
    #[inline(always)]
    fn default() -> MacMdioAddress {
        MacMdioAddress(0)
    }
}
impl core::fmt::Debug for MacMdioAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacMdioAddress")
            .field("gb", &self.gb())
            .field("c45e", &self.c45e())
            .field("goc_0", &self.goc_0())
            .field("goc_1", &self.goc_1())
            .field("skap", &self.skap())
            .field("cr", &self.cr())
            .field("ntc", &self.ntc())
            .field("rda", &self.rda())
            .field("pa", &self.pa())
            .field("btb", &self.btb())
            .field("pse", &self.pse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacMdioAddress {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacMdioAddress {{ gb: {=bool:?}, c45e: {=bool:?}, goc_0: {=bool:?}, goc_1: {=bool:?}, skap: {=bool:?}, cr: {=u8:?}, ntc: {=u8:?}, rda: {=u8:?}, pa: {=u8:?}, btb: {=bool:?}, pse: {=bool:?} }}",
            self.gb(),
            self.c45e(),
            self.goc_0(),
            self.goc_1(),
            self.skap(),
            self.cr(),
            self.ntc(),
            self.rda(),
            self.pa(),
            self.btb(),
            self.pse()
        )
    }
}
#[doc = "MAC MDIO Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacMdioData(pub u32);
impl MacMdioData {
    #[doc = "GMII Data."]
    #[must_use]
    #[inline(always)]
    pub const fn gd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "GMII Data."]
    #[inline(always)]
    pub const fn set_gd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Register Address."]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Register Address."]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacMdioData {
    #[inline(always)]
    fn default() -> MacMdioData {
        MacMdioData(0)
    }
}
impl core::fmt::Debug for MacMdioData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacMdioData")
            .field("gd", &self.gd())
            .field("ra", &self.ra())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacMdioData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacMdioData {{ gd: {=u16:?}, ra: {=u16:?} }}",
            self.gd(),
            self.ra()
        )
    }
}
#[doc = "One-microsecond Reference Timer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacOneusTicCounter(pub u32);
impl MacOneusTicCounter {
    #[doc = "1US TIC Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tic_1us_cntr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "1US TIC Counter."]
    #[inline(always)]
    pub const fn set_tic_1us_cntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for MacOneusTicCounter {
    #[inline(always)]
    fn default() -> MacOneusTicCounter {
        MacOneusTicCounter(0)
    }
}
impl core::fmt::Debug for MacOneusTicCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacOneusTicCounter")
            .field("tic_1us_cntr", &self.tic_1us_cntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacOneusTicCounter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacOneusTicCounter {{ tic_1us_cntr: {=u16:?} }}",
            self.tic_1us_cntr()
        )
    }
}
#[doc = "MAC Packet Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPacketFilter(pub u32);
impl MacPacketFilter {
    #[doc = "Promiscuous Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Promiscuous Mode."]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DA Inverse Filtering."]
    #[must_use]
    #[inline(always)]
    pub const fn daif(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DA Inverse Filtering."]
    #[inline(always)]
    pub const fn set_daif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Pass All Multicast."]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Pass All Multicast."]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Disable Broadcast Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn dbf(&self) -> Dbf {
        let val = (self.0 >> 5usize) & 0x01;
        Dbf::from_bits(val as u8)
    }
    #[doc = "Disable Broadcast Packets."]
    #[inline(always)]
    pub const fn set_dbf(&mut self, val: Dbf) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Pass Control Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn pcf(&self) -> Pcf {
        let val = (self.0 >> 6usize) & 0x03;
        Pcf::from_bits(val as u8)
    }
    #[doc = "Pass Control Packets."]
    #[inline(always)]
    pub const fn set_pcf(&mut self, val: Pcf) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "VLAN Tag Filter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vtfe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Filter Enable."]
    #[inline(always)]
    pub const fn set_vtfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive All."]
    #[must_use]
    #[inline(always)]
    pub const fn ra(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive All."]
    #[inline(always)]
    pub const fn set_ra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacPacketFilter {
    #[inline(always)]
    fn default() -> MacPacketFilter {
        MacPacketFilter(0)
    }
}
impl core::fmt::Debug for MacPacketFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPacketFilter")
            .field("pr", &self.pr())
            .field("daif", &self.daif())
            .field("pm", &self.pm())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("vtfe", &self.vtfe())
            .field("ra", &self.ra())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPacketFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPacketFilter {{ pr: {=bool:?}, daif: {=bool:?}, pm: {=bool:?}, dbf: {:?}, pcf: {:?}, vtfe: {=bool:?}, ra: {=bool:?} }}",
            self.pr(),
            self.daif(),
            self.pm(),
            self.dbf(),
            self.pcf(),
            self.vtfe(),
            self.ra()
        )
    }
}
#[doc = "PMT Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPmtControlStatus(pub u32);
impl MacPmtControlStatus {
    #[doc = "Power Down."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrdwn(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn set_pwrdwn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Magic Packet Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mgkpkten(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet Enable."]
    #[inline(always)]
    pub const fn set_mgkpkten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Remote Wake-Up Packet Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwkpkten(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Wake-Up Packet Enable."]
    #[inline(always)]
    pub const fn set_rwkpkten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Magic Packet Received."]
    #[must_use]
    #[inline(always)]
    pub const fn mgkprcvd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet Received."]
    #[inline(always)]
    pub const fn set_mgkprcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Remote Wake-Up Packet Received."]
    #[must_use]
    #[inline(always)]
    pub const fn rwkprcvd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Wake-Up Packet Received."]
    #[inline(always)]
    pub const fn set_rwkprcvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Unicast."]
    #[must_use]
    #[inline(always)]
    pub const fn glblucast(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Global Unicast."]
    #[inline(always)]
    pub const fn set_glblucast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Remote Wake-up Packet Forwarding Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rwkpfe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Wake-up Packet Forwarding Enable."]
    #[inline(always)]
    pub const fn set_rwkpfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Remote Wake-up FIFO Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn rwkptr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Remote Wake-up FIFO Pointer."]
    #[inline(always)]
    pub const fn set_rwkptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Remote Wake-Up Packet Filter Register Pointer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rwkfiltrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Wake-Up Packet Filter Register Pointer Reset."]
    #[inline(always)]
    pub const fn set_rwkfiltrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacPmtControlStatus {
    #[inline(always)]
    fn default() -> MacPmtControlStatus {
        MacPmtControlStatus(0)
    }
}
impl core::fmt::Debug for MacPmtControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPmtControlStatus")
            .field("pwrdwn", &self.pwrdwn())
            .field("mgkpkten", &self.mgkpkten())
            .field("rwkpkten", &self.rwkpkten())
            .field("mgkprcvd", &self.mgkprcvd())
            .field("rwkprcvd", &self.rwkprcvd())
            .field("glblucast", &self.glblucast())
            .field("rwkpfe", &self.rwkpfe())
            .field("rwkptr", &self.rwkptr())
            .field("rwkfiltrst", &self.rwkfiltrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPmtControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPmtControlStatus {{ pwrdwn: {=bool:?}, mgkpkten: {=bool:?}, rwkpkten: {=bool:?}, mgkprcvd: {=bool:?}, rwkprcvd: {=bool:?}, glblucast: {=bool:?}, rwkpfe: {=bool:?}, rwkptr: {=u8:?}, rwkfiltrst: {=bool:?} }}",
            self.pwrdwn(),
            self.mgkpkten(),
            self.rwkpkten(),
            self.mgkprcvd(),
            self.rwkprcvd(),
            self.glblucast(),
            self.rwkpfe(),
            self.rwkptr(),
            self.rwkfiltrst()
        )
    }
}
#[doc = "PPS Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacPpsControl(pub u32);
impl MacPpsControl {
    #[doc = "PPS Output Frequency Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ppsctrl_ppscmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PPS Output Frequency Control."]
    #[inline(always)]
    pub const fn set_ppsctrl_ppscmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for MacPpsControl {
    #[inline(always)]
    fn default() -> MacPpsControl {
        MacPpsControl(0)
    }
}
impl core::fmt::Debug for MacPpsControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacPpsControl")
            .field("ppsctrl_ppscmd", &self.ppsctrl_ppscmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacPpsControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacPpsControl {{ ppsctrl_ppscmd: {=u8:?} }}",
            self.ppsctrl_ppscmd()
        )
    }
}
#[doc = "MAC Q0 Tx Flow Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacQ0TxFlowCtrl(pub u32);
impl MacQ0TxFlowCtrl {
    #[doc = "Flow Control Busy or Backpressure Activate."]
    #[must_use]
    #[inline(always)]
    pub const fn fcb_bpa(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flow Control Busy or Backpressure Activate."]
    #[inline(always)]
    pub const fn set_fcb_bpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Flow Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Flow Control Enable."]
    #[inline(always)]
    pub const fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pause Low Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn plt(&self) -> Plt {
        let val = (self.0 >> 4usize) & 0x07;
        Plt::from_bits(val as u8)
    }
    #[doc = "Pause Low Threshold."]
    #[inline(always)]
    pub const fn set_plt(&mut self, val: Plt) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Disable Zero-Quanta Pause."]
    #[must_use]
    #[inline(always)]
    pub const fn dzpq(&self) -> Dzpq {
        let val = (self.0 >> 7usize) & 0x01;
        Dzpq::from_bits(val as u8)
    }
    #[doc = "Disable Zero-Quanta Pause."]
    #[inline(always)]
    pub const fn set_dzpq(&mut self, val: Dzpq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Pause Time."]
    #[must_use]
    #[inline(always)]
    pub const fn pt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Pause Time."]
    #[inline(always)]
    pub const fn set_pt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MacQ0TxFlowCtrl {
    #[inline(always)]
    fn default() -> MacQ0TxFlowCtrl {
        MacQ0TxFlowCtrl(0)
    }
}
impl core::fmt::Debug for MacQ0TxFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacQ0TxFlowCtrl")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("tfe", &self.tfe())
            .field("plt", &self.plt())
            .field("dzpq", &self.dzpq())
            .field("pt", &self.pt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacQ0TxFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacQ0TxFlowCtrl {{ fcb_bpa: {=bool:?}, tfe: {=bool:?}, plt: {:?}, dzpq: {:?}, pt: {=u16:?} }}",
            self.fcb_bpa(),
            self.tfe(),
            self.plt(),
            self.dzpq(),
            self.pt()
        )
    }
}
#[doc = "Remote Wakeup Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRwkPacketFilter(pub u32);
impl MacRwkPacketFilter {
    #[doc = "RWK Packet Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn wkupfrmftr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RWK Packet Filter."]
    #[inline(always)]
    pub const fn set_wkupfrmftr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacRwkPacketFilter {
    #[inline(always)]
    fn default() -> MacRwkPacketFilter {
        MacRwkPacketFilter(0)
    }
}
impl core::fmt::Debug for MacRwkPacketFilter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRwkPacketFilter")
            .field("wkupfrmftr", &self.wkupfrmftr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRwkPacketFilter {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRwkPacketFilter {{ wkupfrmftr: {=u32:?} }}",
            self.wkupfrmftr()
        )
    }
}
#[doc = "MAC Rx Flow Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxFlowCtrl(pub u32);
impl MacRxFlowCtrl {
    #[doc = "Receive Flow Control Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Flow Control Enable."]
    #[inline(always)]
    pub const fn set_rfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unicast Pause Packet Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn up(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Pause Packet Detect."]
    #[inline(always)]
    pub const fn set_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MacRxFlowCtrl {
    #[inline(always)]
    fn default() -> MacRxFlowCtrl {
        MacRxFlowCtrl(0)
    }
}
impl core::fmt::Debug for MacRxFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxFlowCtrl")
            .field("rfe", &self.rfe())
            .field("up", &self.up())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxFlowCtrl {{ rfe: {=bool:?}, up: {=bool:?} }}",
            self.rfe(),
            self.up()
        )
    }
}
#[doc = "Receive Transmit Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxTxStatus(pub u32);
impl MacRxTxStatus {
    #[doc = "Transmit Jabber Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn tjt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Jabber Timeout."]
    #[inline(always)]
    pub const fn set_tjt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "No Carrier."]
    #[must_use]
    #[inline(always)]
    pub const fn ncarr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "No Carrier."]
    #[inline(always)]
    pub const fn set_ncarr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Loss of Carrier."]
    #[must_use]
    #[inline(always)]
    pub const fn lcarr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Loss of Carrier."]
    #[inline(always)]
    pub const fn set_lcarr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Excessive Deferral."]
    #[must_use]
    #[inline(always)]
    pub const fn exdef(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Excessive Deferral."]
    #[inline(always)]
    pub const fn set_exdef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Late Collision."]
    #[must_use]
    #[inline(always)]
    pub const fn lcol(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Late Collision."]
    #[inline(always)]
    pub const fn set_lcol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Excessive Collisions."]
    #[must_use]
    #[inline(always)]
    pub const fn excol(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Excessive Collisions."]
    #[inline(always)]
    pub const fn set_excol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive Watchdog Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn rwt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Watchdog Timeout."]
    #[inline(always)]
    pub const fn set_rwt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for MacRxTxStatus {
    #[inline(always)]
    fn default() -> MacRxTxStatus {
        MacRxTxStatus(0)
    }
}
impl core::fmt::Debug for MacRxTxStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxTxStatus")
            .field("tjt", &self.tjt())
            .field("ncarr", &self.ncarr())
            .field("lcarr", &self.lcarr())
            .field("exdef", &self.exdef())
            .field("lcol", &self.lcol())
            .field("excol", &self.excol())
            .field("rwt", &self.rwt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxTxStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxTxStatus {{ tjt: {=bool:?}, ncarr: {=bool:?}, lcarr: {=bool:?}, exdef: {=bool:?}, lcol: {=bool:?}, excol: {=bool:?}, rwt: {=bool:?} }}",
            self.tjt(),
            self.ncarr(),
            self.lcarr(),
            self.exdef(),
            self.lcol(),
            self.excol(),
            self.rwt()
        )
    }
}
#[doc = "Receive Queue Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxqCtrl0(pub u32);
impl MacRxqCtrl0 {
    #[doc = "Receive Queue 0 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq0en(&self) -> Rxq0en {
        let val = (self.0 >> 0usize) & 0x03;
        Rxq0en::from_bits(val as u8)
    }
    #[doc = "Receive Queue 0 Enable."]
    #[inline(always)]
    pub const fn set_rxq0en(&mut self, val: Rxq0en) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive Queue 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq1en(&self) -> Rxq1en {
        let val = (self.0 >> 2usize) & 0x03;
        Rxq1en::from_bits(val as u8)
    }
    #[doc = "Receive Queue 1 Enable."]
    #[inline(always)]
    pub const fn set_rxq1en(&mut self, val: Rxq1en) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for MacRxqCtrl0 {
    #[inline(always)]
    fn default() -> MacRxqCtrl0 {
        MacRxqCtrl0(0)
    }
}
impl core::fmt::Debug for MacRxqCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxqCtrl0")
            .field("rxq0en", &self.rxq0en())
            .field("rxq1en", &self.rxq1en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxqCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxqCtrl0 {{ rxq0en: {:?}, rxq1en: {:?} }}",
            self.rxq0en(),
            self.rxq1en()
        )
    }
}
#[doc = "Receive Queue Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxqCtrl1(pub u32);
impl MacRxqCtrl1 {
    #[doc = "AV Untagged Control Packets Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn avcpq(&self) -> Avcpq {
        let val = (self.0 >> 0usize) & 0x07;
        Avcpq::from_bits(val as u8)
    }
    #[doc = "AV Untagged Control Packets Queue."]
    #[inline(always)]
    pub const fn set_avcpq(&mut self, val: Avcpq) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "PTP Packets Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ptpq(&self) -> Ptpq {
        let val = (self.0 >> 4usize) & 0x07;
        Ptpq::from_bits(val as u8)
    }
    #[doc = "PTP Packets Queue."]
    #[inline(always)]
    pub const fn set_ptpq(&mut self, val: Ptpq) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Untagged Packet Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn upq(&self) -> Upq {
        let val = (self.0 >> 12usize) & 0x07;
        Upq::from_bits(val as u8)
    }
    #[doc = "Untagged Packet Queue."]
    #[inline(always)]
    pub const fn set_upq(&mut self, val: Upq) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Multicast and Broadcast Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn mcbcq(&self) -> Mcbcq {
        let val = (self.0 >> 16usize) & 0x07;
        Mcbcq::from_bits(val as u8)
    }
    #[doc = "Multicast and Broadcast Queue."]
    #[inline(always)]
    pub const fn set_mcbcq(&mut self, val: Mcbcq) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Multicast and Broadcast Queue Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mcbcqen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Multicast and Broadcast Queue Enable."]
    #[inline(always)]
    pub const fn set_mcbcqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Tagged AV Control Packets Queuing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tacpqe(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tagged AV Control Packets Queuing Enable."]
    #[inline(always)]
    pub const fn set_tacpqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tpqc(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
    #[inline(always)]
    pub const fn set_tpqc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "OMCBCQ."]
    #[must_use]
    #[inline(always)]
    pub const fn omcbcq(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "OMCBCQ."]
    #[inline(always)]
    pub const fn set_omcbcq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Type Field Based Rx Queuing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tbrqe(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Type Field Based Rx Queuing Enable."]
    #[inline(always)]
    pub const fn set_tbrqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for MacRxqCtrl1 {
    #[inline(always)]
    fn default() -> MacRxqCtrl1 {
        MacRxqCtrl1(0)
    }
}
impl core::fmt::Debug for MacRxqCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxqCtrl1")
            .field("avcpq", &self.avcpq())
            .field("ptpq", &self.ptpq())
            .field("upq", &self.upq())
            .field("mcbcq", &self.mcbcq())
            .field("mcbcqen", &self.mcbcqen())
            .field("tacpqe", &self.tacpqe())
            .field("tpqc", &self.tpqc())
            .field("omcbcq", &self.omcbcq())
            .field("tbrqe", &self.tbrqe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxqCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxqCtrl1 {{ avcpq: {:?}, ptpq: {:?}, upq: {:?}, mcbcq: {:?}, mcbcqen: {=bool:?}, tacpqe: {=bool:?}, tpqc: {=u8:?}, omcbcq: {=bool:?}, tbrqe: {=bool:?} }}",
            self.avcpq(),
            self.ptpq(),
            self.upq(),
            self.mcbcq(),
            self.mcbcqen(),
            self.tacpqe(),
            self.tpqc(),
            self.omcbcq(),
            self.tbrqe()
        )
    }
}
#[doc = "Receive Queue Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxqCtrl2(pub u32);
impl MacRxqCtrl2 {
    #[doc = "Priorities Selected in the Receive Queue 0."]
    #[must_use]
    #[inline(always)]
    pub const fn psrq0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Priorities Selected in the Receive Queue 0."]
    #[inline(always)]
    pub const fn set_psrq0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Priorities Selected in the Receive Queue 1."]
    #[must_use]
    #[inline(always)]
    pub const fn psrq1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Priorities Selected in the Receive Queue 1."]
    #[inline(always)]
    pub const fn set_psrq1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for MacRxqCtrl2 {
    #[inline(always)]
    fn default() -> MacRxqCtrl2 {
        MacRxqCtrl2(0)
    }
}
impl core::fmt::Debug for MacRxqCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxqCtrl2")
            .field("psrq0", &self.psrq0())
            .field("psrq1", &self.psrq1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxqCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxqCtrl2 {{ psrq0: {=u8:?}, psrq1: {=u8:?} }}",
            self.psrq0(),
            self.psrq1()
        )
    }
}
#[doc = "Receive Queue Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacRxqCtrl4(pub u32);
impl MacRxqCtrl4 {
    #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uffqe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
    #[inline(always)]
    pub const fn set_uffqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unicast Address Filter Fail Packets Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn uffq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unicast Address Filter Fail Packets Queue."]
    #[inline(always)]
    pub const fn set_uffq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mffqe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
    #[inline(always)]
    pub const fn set_mffqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Multicast Address Filter Fail Packets Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn mffq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Multicast Address Filter Fail Packets Queue."]
    #[inline(always)]
    pub const fn set_mffq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "VLAN Tag Filter Fail Packets Queuing Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vffqe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Filter Fail Packets Queuing Enable."]
    #[inline(always)]
    pub const fn set_vffqe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VLAN Tag Filter Fail Packets Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn vffq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Filter Fail Packets Queue."]
    #[inline(always)]
    pub const fn set_vffq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for MacRxqCtrl4 {
    #[inline(always)]
    fn default() -> MacRxqCtrl4 {
        MacRxqCtrl4(0)
    }
}
impl core::fmt::Debug for MacRxqCtrl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacRxqCtrl4")
            .field("uffqe", &self.uffqe())
            .field("uffq", &self.uffq())
            .field("mffqe", &self.mffqe())
            .field("mffq", &self.mffq())
            .field("vffqe", &self.vffqe())
            .field("vffq", &self.vffq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacRxqCtrl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacRxqCtrl4 {{ uffqe: {=bool:?}, uffq: {=bool:?}, mffqe: {=bool:?}, mffq: {=bool:?}, vffqe: {=bool:?}, vffq: {=bool:?} }}",
            self.uffqe(),
            self.uffq(),
            self.mffqe(),
            self.mffq(),
            self.vffqe(),
            self.vffq()
        )
    }
}
#[doc = "Subsecond Increment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSubSecondIncrement(pub u32);
impl MacSubSecondIncrement {
    #[doc = "Sub-nanosecond Increment Value."]
    #[must_use]
    #[inline(always)]
    pub const fn snsinc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Sub-nanosecond Increment Value."]
    #[inline(always)]
    pub const fn set_snsinc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for MacSubSecondIncrement {
    #[inline(always)]
    fn default() -> MacSubSecondIncrement {
        MacSubSecondIncrement(0)
    }
}
impl core::fmt::Debug for MacSubSecondIncrement {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSubSecondIncrement")
            .field("snsinc", &self.snsinc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSubSecondIncrement {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSubSecondIncrement {{ snsinc: {=u8:?} }}",
            self.snsinc()
        )
    }
}
#[doc = "System Time Nanoseconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeNanoseconds(pub u32);
impl MacSystemTimeNanoseconds {
    #[doc = "Timestamp Sub Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn tsss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Timestamp Sub Seconds."]
    #[inline(always)]
    pub const fn set_tsss(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeNanoseconds {
    #[inline(always)]
    fn default() -> MacSystemTimeNanoseconds {
        MacSystemTimeNanoseconds(0)
    }
}
impl core::fmt::Debug for MacSystemTimeNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeNanoseconds")
            .field("tsss", &self.tsss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeNanoseconds {{ tsss: {=u32:?} }}",
            self.tsss()
        )
    }
}
#[doc = "System Time Nanoseconds Update."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeNanosecondsUpdate(pub u32);
impl MacSystemTimeNanosecondsUpdate {
    #[doc = "Timestamp Sub Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn tsss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Timestamp Sub Seconds."]
    #[inline(always)]
    pub const fn set_tsss(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Add or Subtract Time."]
    #[must_use]
    #[inline(always)]
    pub const fn addsub(&self) -> Addsub {
        let val = (self.0 >> 31usize) & 0x01;
        Addsub::from_bits(val as u8)
    }
    #[doc = "Add or Subtract Time."]
    #[inline(always)]
    pub const fn set_addsub(&mut self, val: Addsub) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MacSystemTimeNanosecondsUpdate {
    #[inline(always)]
    fn default() -> MacSystemTimeNanosecondsUpdate {
        MacSystemTimeNanosecondsUpdate(0)
    }
}
impl core::fmt::Debug for MacSystemTimeNanosecondsUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeNanosecondsUpdate")
            .field("tsss", &self.tsss())
            .field("addsub", &self.addsub())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeNanosecondsUpdate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeNanosecondsUpdate {{ tsss: {=u32:?}, addsub: {:?} }}",
            self.tsss(),
            self.addsub()
        )
    }
}
#[doc = "System Time Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeSeconds(pub u32);
impl MacSystemTimeSeconds {
    #[doc = "Timestamp Second."]
    #[must_use]
    #[inline(always)]
    pub const fn tss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Second."]
    #[inline(always)]
    pub const fn set_tss(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeSeconds {
    #[inline(always)]
    fn default() -> MacSystemTimeSeconds {
        MacSystemTimeSeconds(0)
    }
}
impl core::fmt::Debug for MacSystemTimeSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeSeconds")
            .field("tss", &self.tss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacSystemTimeSeconds {{ tss: {=u32:?} }}", self.tss())
    }
}
#[doc = "System Time Seconds Update."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacSystemTimeSecondsUpdate(pub u32);
impl MacSystemTimeSecondsUpdate {
    #[doc = "Timestamp Seconds."]
    #[must_use]
    #[inline(always)]
    pub const fn tss(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Seconds."]
    #[inline(always)]
    pub const fn set_tss(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacSystemTimeSecondsUpdate {
    #[inline(always)]
    fn default() -> MacSystemTimeSecondsUpdate {
        MacSystemTimeSecondsUpdate(0)
    }
}
impl core::fmt::Debug for MacSystemTimeSecondsUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacSystemTimeSecondsUpdate")
            .field("tss", &self.tss())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacSystemTimeSecondsUpdate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacSystemTimeSecondsUpdate {{ tss: {=u32:?} }}",
            self.tss()
        )
    }
}
#[doc = "Timestamp Addend."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampAddend(pub u32);
impl MacTimestampAddend {
    #[doc = "Timestamp Addend Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tsar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Addend Register."]
    #[inline(always)]
    pub const fn set_tsar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampAddend {
    #[inline(always)]
    fn default() -> MacTimestampAddend {
        MacTimestampAddend(0)
    }
}
impl core::fmt::Debug for MacTimestampAddend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampAddend")
            .field("tsar", &self.tsar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampAddend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacTimestampAddend {{ tsar: {=u32:?} }}", self.tsar())
    }
}
#[doc = "Timestamp Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampControl(pub u32);
impl MacTimestampControl {
    #[doc = "Enable Timestamp."]
    #[must_use]
    #[inline(always)]
    pub const fn tsena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp."]
    #[inline(always)]
    pub const fn set_tsena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Fine or Coarse Timestamp Update."]
    #[must_use]
    #[inline(always)]
    pub const fn tscfupdt(&self) -> Tscfupdt {
        let val = (self.0 >> 1usize) & 0x01;
        Tscfupdt::from_bits(val as u8)
    }
    #[doc = "Fine or Coarse Timestamp Update."]
    #[inline(always)]
    pub const fn set_tscfupdt(&mut self, val: Tscfupdt) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Initialize Timestamp."]
    #[must_use]
    #[inline(always)]
    pub const fn tsinit(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Initialize Timestamp."]
    #[inline(always)]
    pub const fn set_tsinit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Update Timestamp."]
    #[must_use]
    #[inline(always)]
    pub const fn tsupdt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Update Timestamp."]
    #[inline(always)]
    pub const fn set_tsupdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Timestamp Interrupt Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn tstrig(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp Interrupt Trigger."]
    #[inline(always)]
    pub const fn set_tstrig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Update Addend Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tsaddreg(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Update Addend Register."]
    #[inline(always)]
    pub const fn set_tsaddreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Timestamp for All Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn tsenall(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp for All Packets."]
    #[inline(always)]
    pub const fn set_tsenall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Timestamp Digital or Binary Rollover Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tsctrlssr(&self) -> Tsctrlssr {
        let val = (self.0 >> 9usize) & 0x01;
        Tsctrlssr::from_bits(val as u8)
    }
    #[doc = "Timestamp Digital or Binary Rollover Control."]
    #[inline(always)]
    pub const fn set_tsctrlssr(&mut self, val: Tsctrlssr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format."]
    #[must_use]
    #[inline(always)]
    pub const fn tsver2ena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format."]
    #[inline(always)]
    pub const fn set_tsver2ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn tsipena(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets."]
    #[inline(always)]
    pub const fn set_tsipena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP."]
    #[must_use]
    #[inline(always)]
    pub const fn tsipv6ena(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP."]
    #[inline(always)]
    pub const fn set_tsipv6ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP."]
    #[must_use]
    #[inline(always)]
    pub const fn tsipv4ena(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP."]
    #[inline(always)]
    pub const fn set_tsipv4ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages."]
    #[must_use]
    #[inline(always)]
    pub const fn tsevntena(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages."]
    #[inline(always)]
    pub const fn set_tsevntena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master."]
    #[must_use]
    #[inline(always)]
    pub const fn tsmstrena(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master."]
    #[inline(always)]
    pub const fn set_tsmstrena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Select PTP packets for Taking Snapshots."]
    #[must_use]
    #[inline(always)]
    pub const fn snaptypsel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Select PTP packets for Taking Snapshots."]
    #[inline(always)]
    pub const fn set_snaptypsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering."]
    #[must_use]
    #[inline(always)]
    pub const fn tsenmacaddr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering."]
    #[inline(always)]
    pub const fn set_tsenmacaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "External System Time Input."]
    #[must_use]
    #[inline(always)]
    pub const fn esti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "External System Time Input."]
    #[inline(always)]
    pub const fn set_esti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Transmit Timestamp Status Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn txtsstsm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Timestamp Status Mode."]
    #[inline(always)]
    pub const fn set_txtsstsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "AV 802."]
    #[must_use]
    #[inline(always)]
    pub const fn av8021asmen(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "AV 802."]
    #[inline(always)]
    pub const fn set_av8021asmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for MacTimestampControl {
    #[inline(always)]
    fn default() -> MacTimestampControl {
        MacTimestampControl(0)
    }
}
impl core::fmt::Debug for MacTimestampControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampControl")
            .field("tsena", &self.tsena())
            .field("tscfupdt", &self.tscfupdt())
            .field("tsinit", &self.tsinit())
            .field("tsupdt", &self.tsupdt())
            .field("tstrig", &self.tstrig())
            .field("tsaddreg", &self.tsaddreg())
            .field("tsenall", &self.tsenall())
            .field("tsctrlssr", &self.tsctrlssr())
            .field("tsver2ena", &self.tsver2ena())
            .field("tsipena", &self.tsipena())
            .field("tsipv6ena", &self.tsipv6ena())
            .field("tsipv4ena", &self.tsipv4ena())
            .field("tsevntena", &self.tsevntena())
            .field("tsmstrena", &self.tsmstrena())
            .field("snaptypsel", &self.snaptypsel())
            .field("tsenmacaddr", &self.tsenmacaddr())
            .field("esti", &self.esti())
            .field("txtsstsm", &self.txtsstsm())
            .field("av8021asmen", &self.av8021asmen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampControl {{ tsena: {=bool:?}, tscfupdt: {:?}, tsinit: {=bool:?}, tsupdt: {=bool:?}, tstrig: {=bool:?}, tsaddreg: {=bool:?}, tsenall: {=bool:?}, tsctrlssr: {:?}, tsver2ena: {=bool:?}, tsipena: {=bool:?}, tsipv6ena: {=bool:?}, tsipv4ena: {=bool:?}, tsevntena: {=bool:?}, tsmstrena: {=bool:?}, snaptypsel: {=u8:?}, tsenmacaddr: {=bool:?}, esti: {=bool:?}, txtsstsm: {=bool:?}, av8021asmen: {=bool:?} }}",
            self.tsena(),
            self.tscfupdt(),
            self.tsinit(),
            self.tsupdt(),
            self.tstrig(),
            self.tsaddreg(),
            self.tsenall(),
            self.tsctrlssr(),
            self.tsver2ena(),
            self.tsipena(),
            self.tsipv6ena(),
            self.tsipv4ena(),
            self.tsevntena(),
            self.tsmstrena(),
            self.snaptypsel(),
            self.tsenmacaddr(),
            self.esti(),
            self.txtsstsm(),
            self.av8021asmen()
        )
    }
}
#[doc = "Timestamp Egress Correction Nanosecond."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampEgressCorrNanosecond(pub u32);
impl MacTimestampEgressCorrNanosecond {
    #[doc = "Timestamp Egress Correction."]
    #[must_use]
    #[inline(always)]
    pub const fn tsec(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Egress Correction."]
    #[inline(always)]
    pub const fn set_tsec(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampEgressCorrNanosecond {
    #[inline(always)]
    fn default() -> MacTimestampEgressCorrNanosecond {
        MacTimestampEgressCorrNanosecond(0)
    }
}
impl core::fmt::Debug for MacTimestampEgressCorrNanosecond {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampEgressCorrNanosecond")
            .field("tsec", &self.tsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampEgressCorrNanosecond {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampEgressCorrNanosecond {{ tsec: {=u32:?} }}",
            self.tsec()
        )
    }
}
#[doc = "Timestamp Egress Latency."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampEgressLatency(pub u32);
impl MacTimestampEgressLatency {
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn etlsns(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds."]
    #[inline(always)]
    pub const fn set_etlsns(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds."]
    #[must_use]
    #[inline(always)]
    pub const fn etlns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds."]
    #[inline(always)]
    pub const fn set_etlns(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for MacTimestampEgressLatency {
    #[inline(always)]
    fn default() -> MacTimestampEgressLatency {
        MacTimestampEgressLatency(0)
    }
}
impl core::fmt::Debug for MacTimestampEgressLatency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampEgressLatency")
            .field("etlsns", &self.etlsns())
            .field("etlns", &self.etlns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampEgressLatency {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampEgressLatency {{ etlsns: {=u8:?}, etlns: {=u16:?} }}",
            self.etlsns(),
            self.etlns()
        )
    }
}
#[doc = "Timestamp Ingress Correction Nanosecond."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampIngressCorrNanosecond(pub u32);
impl MacTimestampIngressCorrNanosecond {
    #[doc = "Timestamp Ingress Correction."]
    #[must_use]
    #[inline(always)]
    pub const fn tsic(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Timestamp Ingress Correction."]
    #[inline(always)]
    pub const fn set_tsic(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTimestampIngressCorrNanosecond {
    #[inline(always)]
    fn default() -> MacTimestampIngressCorrNanosecond {
        MacTimestampIngressCorrNanosecond(0)
    }
}
impl core::fmt::Debug for MacTimestampIngressCorrNanosecond {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampIngressCorrNanosecond")
            .field("tsic", &self.tsic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampIngressCorrNanosecond {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampIngressCorrNanosecond {{ tsic: {=u32:?} }}",
            self.tsic()
        )
    }
}
#[doc = "Timestamp Ingress Latency."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampIngressLatency(pub u32);
impl MacTimestampIngressLatency {
    #[doc = "ITLSNS."]
    #[must_use]
    #[inline(always)]
    pub const fn itlsns(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "ITLSNS."]
    #[inline(always)]
    pub const fn set_itlsns(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ITLNS."]
    #[must_use]
    #[inline(always)]
    pub const fn itlns(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "ITLNS."]
    #[inline(always)]
    pub const fn set_itlns(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for MacTimestampIngressLatency {
    #[inline(always)]
    fn default() -> MacTimestampIngressLatency {
        MacTimestampIngressLatency(0)
    }
}
impl core::fmt::Debug for MacTimestampIngressLatency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampIngressLatency")
            .field("itlsns", &self.itlsns())
            .field("itlns", &self.itlns())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampIngressLatency {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampIngressLatency {{ itlsns: {=u8:?}, itlns: {=u16:?} }}",
            self.itlsns(),
            self.itlns()
        )
    }
}
#[doc = "Timestamp Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTimestampStatus(pub u32);
impl MacTimestampStatus {
    #[doc = "Timestamp Seconds Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn tssovf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Seconds Overflow."]
    #[inline(always)]
    pub const fn set_tssovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timestamp Target Time Reached."]
    #[must_use]
    #[inline(always)]
    pub const fn tstargt0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Target Time Reached."]
    #[inline(always)]
    pub const fn set_tstargt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timestamp Target Time Error."]
    #[must_use]
    #[inline(always)]
    pub const fn tstrgterr0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Target Time Error."]
    #[inline(always)]
    pub const fn set_tstrgterr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Tx Timestamp Status Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txtssis(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Timestamp Status Interrupt Status."]
    #[inline(always)]
    pub const fn set_txtssis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for MacTimestampStatus {
    #[inline(always)]
    fn default() -> MacTimestampStatus {
        MacTimestampStatus(0)
    }
}
impl core::fmt::Debug for MacTimestampStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTimestampStatus")
            .field("tssovf", &self.tssovf())
            .field("tstargt0", &self.tstargt0())
            .field("tstrgterr0", &self.tstrgterr0())
            .field("txtssis", &self.txtssis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTimestampStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTimestampStatus {{ tssovf: {=bool:?}, tstargt0: {=bool:?}, tstrgterr0: {=bool:?}, txtssis: {=bool:?} }}",
            self.tssovf(),
            self.tstargt0(),
            self.tstrgterr0(),
            self.txtssis()
        )
    }
}
#[doc = "Transmit Timestamp Status Nanoseconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTxTimestampStatusNanoseconds(pub u32);
impl MacTxTimestampStatusNanoseconds {
    #[doc = "Transmit Timestamp Status Low."]
    #[must_use]
    #[inline(always)]
    pub const fn txtsslo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Transmit Timestamp Status Low."]
    #[inline(always)]
    pub const fn set_txtsslo(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "TXTSSMIS."]
    #[must_use]
    #[inline(always)]
    pub const fn txtssmis(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TXTSSMIS."]
    #[inline(always)]
    pub const fn set_txtssmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacTxTimestampStatusNanoseconds {
    #[inline(always)]
    fn default() -> MacTxTimestampStatusNanoseconds {
        MacTxTimestampStatusNanoseconds(0)
    }
}
impl core::fmt::Debug for MacTxTimestampStatusNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTxTimestampStatusNanoseconds")
            .field("txtsslo", &self.txtsslo())
            .field("txtssmis", &self.txtssmis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTxTimestampStatusNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTxTimestampStatusNanoseconds {{ txtsslo: {=u32:?}, txtssmis: {=bool:?} }}",
            self.txtsslo(),
            self.txtssmis()
        )
    }
}
#[doc = "Transmit Timestamp Status Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacTxTimestampStatusSeconds(pub u32);
impl MacTxTimestampStatusSeconds {
    #[doc = "Transmit Timestamp Status High."]
    #[must_use]
    #[inline(always)]
    pub const fn txtsshi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Timestamp Status High."]
    #[inline(always)]
    pub const fn set_txtsshi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MacTxTimestampStatusSeconds {
    #[inline(always)]
    fn default() -> MacTxTimestampStatusSeconds {
        MacTxTimestampStatusSeconds(0)
    }
}
impl core::fmt::Debug for MacTxTimestampStatusSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacTxTimestampStatusSeconds")
            .field("txtsshi", &self.txtsshi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacTxTimestampStatusSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacTxTimestampStatusSeconds {{ txtsshi: {=u32:?} }}",
            self.txtsshi()
        )
    }
}
#[doc = "MAC Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVersion(pub u32);
impl MacVersion {
    #[doc = "Synopsys-defined Version."]
    #[must_use]
    #[inline(always)]
    pub const fn snpsver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Synopsys-defined Version."]
    #[inline(always)]
    pub const fn set_snpsver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "User-defined Version."]
    #[must_use]
    #[inline(always)]
    pub const fn userver(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "User-defined Version."]
    #[inline(always)]
    pub const fn set_userver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for MacVersion {
    #[inline(always)]
    fn default() -> MacVersion {
        MacVersion(0)
    }
}
impl core::fmt::Debug for MacVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVersion")
            .field("snpsver", &self.snpsver())
            .field("userver", &self.userver())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVersion {{ snpsver: {=u8:?}, userver: {=u8:?} }}",
            self.snpsver(),
            self.userver()
        )
    }
}
#[doc = "VLAN Tag Inclusion or Replacement."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVlanIncl(pub u32);
impl MacVlanIncl {
    #[doc = "VLAN Tag for Transmit Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn vlt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag for Transmit Packets."]
    #[inline(always)]
    pub const fn set_vlt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "VLAN Tag Control in Transmit Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn vlc(&self) -> MacVlanInclVlc {
        let val = (self.0 >> 16usize) & 0x03;
        MacVlanInclVlc::from_bits(val as u8)
    }
    #[doc = "VLAN Tag Control in Transmit Packets."]
    #[inline(always)]
    pub const fn set_vlc(&mut self, val: MacVlanInclVlc) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "VLAN Priority Control."]
    #[must_use]
    #[inline(always)]
    pub const fn vlp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Priority Control."]
    #[inline(always)]
    pub const fn set_vlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "C-VLAN or S-VLAN."]
    #[must_use]
    #[inline(always)]
    pub const fn csvl(&self) -> MacVlanInclCsvl {
        let val = (self.0 >> 19usize) & 0x01;
        MacVlanInclCsvl::from_bits(val as u8)
    }
    #[doc = "C-VLAN or S-VLAN."]
    #[inline(always)]
    pub const fn set_csvl(&mut self, val: MacVlanInclCsvl) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "VLAN Tag Input."]
    #[must_use]
    #[inline(always)]
    pub const fn vlti(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Input."]
    #[inline(always)]
    pub const fn set_vlti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Channel based tag insertion."]
    #[must_use]
    #[inline(always)]
    pub const fn cbti(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Channel based tag insertion."]
    #[inline(always)]
    pub const fn set_cbti(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Address."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Read write control."]
    #[must_use]
    #[inline(always)]
    pub const fn rdwr(&self) -> Rdwr {
        let val = (self.0 >> 30usize) & 0x01;
        Rdwr::from_bits(val as u8)
    }
    #[doc = "Read write control."]
    #[inline(always)]
    pub const fn set_rdwr(&mut self, val: Rdwr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Busy."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacVlanIncl {
    #[inline(always)]
    fn default() -> MacVlanIncl {
        MacVlanIncl(0)
    }
}
impl core::fmt::Debug for MacVlanIncl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVlanIncl")
            .field("vlt", &self.vlt())
            .field("vlc", &self.vlc())
            .field("vlp", &self.vlp())
            .field("csvl", &self.csvl())
            .field("vlti", &self.vlti())
            .field("cbti", &self.cbti())
            .field("addr", &self.addr())
            .field("rdwr", &self.rdwr())
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVlanIncl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVlanIncl {{ vlt: {=u16:?}, vlc: {:?}, vlp: {=bool:?}, csvl: {:?}, vlti: {=bool:?}, cbti: {=bool:?}, addr: {=bool:?}, rdwr: {:?}, busy: {=bool:?} }}",
            self.vlt(),
            self.vlc(),
            self.vlp(),
            self.csvl(),
            self.vlti(),
            self.cbti(),
            self.addr(),
            self.rdwr(),
            self.busy()
        )
    }
}
#[doc = "MAC VLAN Tag Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacVlanTagCtrl(pub u32);
impl MacVlanTagCtrl {
    #[doc = "VLAN Tag Identifier for Receive Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn vl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "VLAN Tag Identifier for Receive Packets."]
    #[inline(always)]
    pub const fn set_vl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Enable 12-Bit VLAN Tag Comparison."]
    #[must_use]
    #[inline(always)]
    pub const fn etv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable 12-Bit VLAN Tag Comparison."]
    #[inline(always)]
    pub const fn set_etv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VLAN Tag Inverse Match Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vtim(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Inverse Match Enable."]
    #[inline(always)]
    pub const fn set_vtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enable S-VLAN."]
    #[must_use]
    #[inline(always)]
    pub const fn esvl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enable S-VLAN."]
    #[inline(always)]
    pub const fn set_esvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enable Receive S-VLAN Match."]
    #[must_use]
    #[inline(always)]
    pub const fn ersvlm(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Receive S-VLAN Match."]
    #[inline(always)]
    pub const fn set_ersvlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Disable VLAN Type Check."]
    #[must_use]
    #[inline(always)]
    pub const fn dovltc(&self) -> Dovltc {
        let val = (self.0 >> 20usize) & 0x01;
        Dovltc::from_bits(val as u8)
    }
    #[doc = "Disable VLAN Type Check."]
    #[inline(always)]
    pub const fn set_dovltc(&mut self, val: Dovltc) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable VLAN Tag Stripping on Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn evls(&self) -> Evls {
        let val = (self.0 >> 21usize) & 0x03;
        Evls::from_bits(val as u8)
    }
    #[doc = "Enable VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub const fn set_evls(&mut self, val: Evls) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Enable VLAN Tag in Rx status."]
    #[must_use]
    #[inline(always)]
    pub const fn evlrxs(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable VLAN Tag in Rx status."]
    #[inline(always)]
    pub const fn set_evlrxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enable Double VLAN Processing."]
    #[must_use]
    #[inline(always)]
    pub const fn edvlp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Double VLAN Processing."]
    #[inline(always)]
    pub const fn set_edvlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enable Inner VLAN Tag."]
    #[must_use]
    #[inline(always)]
    pub const fn erivlt(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Inner VLAN Tag."]
    #[inline(always)]
    pub const fn set_erivlt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive."]
    #[must_use]
    #[inline(always)]
    pub const fn eivls(&self) -> Eivls {
        let val = (self.0 >> 28usize) & 0x03;
        Eivls::from_bits(val as u8)
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive."]
    #[inline(always)]
    pub const fn set_eivls(&mut self, val: Eivls) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status."]
    #[must_use]
    #[inline(always)]
    pub const fn eivlrxs(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status."]
    #[inline(always)]
    pub const fn set_eivlrxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacVlanTagCtrl {
    #[inline(always)]
    fn default() -> MacVlanTagCtrl {
        MacVlanTagCtrl(0)
    }
}
impl core::fmt::Debug for MacVlanTagCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacVlanTagCtrl")
            .field("vl", &self.vl())
            .field("etv", &self.etv())
            .field("vtim", &self.vtim())
            .field("esvl", &self.esvl())
            .field("ersvlm", &self.ersvlm())
            .field("dovltc", &self.dovltc())
            .field("evls", &self.evls())
            .field("evlrxs", &self.evlrxs())
            .field("edvlp", &self.edvlp())
            .field("erivlt", &self.erivlt())
            .field("eivls", &self.eivls())
            .field("eivlrxs", &self.eivlrxs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacVlanTagCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacVlanTagCtrl {{ vl: {=u16:?}, etv: {=bool:?}, vtim: {=bool:?}, esvl: {=bool:?}, ersvlm: {=bool:?}, dovltc: {:?}, evls: {:?}, evlrxs: {=bool:?}, edvlp: {=bool:?}, erivlt: {=bool:?}, eivls: {:?}, eivlrxs: {=bool:?} }}",
            self.vl(),
            self.etv(),
            self.vtim(),
            self.esvl(),
            self.ersvlm(),
            self.dovltc(),
            self.evls(),
            self.evlrxs(),
            self.edvlp(),
            self.erivlt(),
            self.eivls(),
            self.eivlrxs()
        )
    }
}
#[doc = "Watchdog Timeout."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacWatchdogTimeout(pub u32);
impl MacWatchdogTimeout {
    #[doc = "Watchdog Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn wto(&self) -> Wto {
        let val = (self.0 >> 0usize) & 0x0f;
        Wto::from_bits(val as u8)
    }
    #[doc = "Watchdog Timeout."]
    #[inline(always)]
    pub const fn set_wto(&mut self, val: Wto) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Programmable Watchdog Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pwe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Watchdog Enable."]
    #[inline(always)]
    pub const fn set_pwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for MacWatchdogTimeout {
    #[inline(always)]
    fn default() -> MacWatchdogTimeout {
        MacWatchdogTimeout(0)
    }
}
impl core::fmt::Debug for MacWatchdogTimeout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacWatchdogTimeout")
            .field("wto", &self.wto())
            .field("pwe", &self.pwe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacWatchdogTimeout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MacWatchdogTimeout {{ wto: {:?}, pwe: {=bool:?} }}",
            self.wto(),
            self.pwe()
        )
    }
}
#[doc = "MTL Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlInterruptStatus(pub u32);
impl MtlInterruptStatus {
    #[doc = "Queue 0 Interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn q0is(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 0 Interrupt status."]
    #[inline(always)]
    pub const fn set_q0is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Queue 1 Interrupt status."]
    #[must_use]
    #[inline(always)]
    pub const fn q1is(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 1 Interrupt status."]
    #[inline(always)]
    pub const fn set_q1is(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for MtlInterruptStatus {
    #[inline(always)]
    fn default() -> MtlInterruptStatus {
        MtlInterruptStatus(0)
    }
}
impl core::fmt::Debug for MtlInterruptStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlInterruptStatus")
            .field("q0is", &self.q0is())
            .field("q1is", &self.q1is())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlInterruptStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlInterruptStatus {{ q0is: {=bool:?}, q1is: {=bool:?} }}",
            self.q0is(),
            self.q1is()
        )
    }
}
#[doc = "MTL Operation Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlOperationMode(pub u32);
impl MtlOperationMode {
    #[doc = "Drop Transmit Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dtxsts(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Drop Transmit Status."]
    #[inline(always)]
    pub const fn set_dtxsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive Arbitration Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn raa(&self) -> Raa {
        let val = (self.0 >> 2usize) & 0x01;
        Raa::from_bits(val as u8)
    }
    #[doc = "Receive Arbitration Algorithm."]
    #[inline(always)]
    pub const fn set_raa(&mut self, val: Raa) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Tx Scheduling Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn schalg(&self) -> Schalg {
        let val = (self.0 >> 5usize) & 0x03;
        Schalg::from_bits(val as u8)
    }
    #[doc = "Tx Scheduling Algorithm."]
    #[inline(always)]
    pub const fn set_schalg(&mut self, val: Schalg) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Counters Preset."]
    #[must_use]
    #[inline(always)]
    pub const fn cntprst(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Counters Preset."]
    #[inline(always)]
    pub const fn set_cntprst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Counters Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cntclr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Counters Reset."]
    #[inline(always)]
    pub const fn set_cntclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for MtlOperationMode {
    #[inline(always)]
    fn default() -> MtlOperationMode {
        MtlOperationMode(0)
    }
}
impl core::fmt::Debug for MtlOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlOperationMode")
            .field("dtxsts", &self.dtxsts())
            .field("raa", &self.raa())
            .field("schalg", &self.schalg())
            .field("cntprst", &self.cntprst())
            .field("cntclr", &self.cntclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlOperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlOperationMode {{ dtxsts: {=bool:?}, raa: {:?}, schalg: {:?}, cntprst: {=bool:?}, cntclr: {=bool:?} }}",
            self.dtxsts(),
            self.raa(),
            self.schalg(),
            self.cntprst(),
            self.cntclr()
        )
    }
}
#[doc = "Queue 0 Interrupt Control Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlQ0InterruptControlStatus(pub u32);
impl MtlQ0InterruptControlStatus {
    #[doc = "Transmit Queue Underflow Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txunfis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Status."]
    #[inline(always)]
    pub const fn set_txunfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Average Bits Per Slot Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn abpsis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Average Bits Per Slot Interrupt Status."]
    #[inline(always)]
    pub const fn set_abpsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txuie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txuie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Average Bits Per Slot Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn abpsie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Average Bits Per Slot Interrupt Enable."]
    #[inline(always)]
    pub const fn set_abpsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxovfis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Status."]
    #[inline(always)]
    pub const fn set_rxovfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxoie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for MtlQ0InterruptControlStatus {
    #[inline(always)]
    fn default() -> MtlQ0InterruptControlStatus {
        MtlQ0InterruptControlStatus(0)
    }
}
impl core::fmt::Debug for MtlQ0InterruptControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlQ0InterruptControlStatus")
            .field("txunfis", &self.txunfis())
            .field("abpsis", &self.abpsis())
            .field("txuie", &self.txuie())
            .field("abpsie", &self.abpsie())
            .field("rxovfis", &self.rxovfis())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlQ0InterruptControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlQ0InterruptControlStatus {{ txunfis: {=bool:?}, abpsis: {=bool:?}, txuie: {=bool:?}, abpsie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
            self.txunfis(),
            self.abpsis(),
            self.txuie(),
            self.abpsie(),
            self.rxovfis(),
            self.rxoie()
        )
    }
}
#[doc = "Queue 1 Interrupt Control Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlQ1InterruptControlStatus(pub u32);
impl MtlQ1InterruptControlStatus {
    #[doc = "Transmit Queue Underflow Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txunfis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Status."]
    #[inline(always)]
    pub const fn set_txunfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Average Bits Per Slot Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn abpsis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Average Bits Per Slot Interrupt Status."]
    #[inline(always)]
    pub const fn set_abpsis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txuie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txuie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Average Bits Per Slot Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn abpsie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Average Bits Per Slot Interrupt Enable."]
    #[inline(always)]
    pub const fn set_abpsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxovfis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Status."]
    #[inline(always)]
    pub const fn set_rxovfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Queue Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxoie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxoie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for MtlQ1InterruptControlStatus {
    #[inline(always)]
    fn default() -> MtlQ1InterruptControlStatus {
        MtlQ1InterruptControlStatus(0)
    }
}
impl core::fmt::Debug for MtlQ1InterruptControlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlQ1InterruptControlStatus")
            .field("txunfis", &self.txunfis())
            .field("abpsis", &self.abpsis())
            .field("txuie", &self.txuie())
            .field("abpsie", &self.abpsie())
            .field("rxovfis", &self.rxovfis())
            .field("rxoie", &self.rxoie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlQ1InterruptControlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlQ1InterruptControlStatus {{ txunfis: {=bool:?}, abpsis: {=bool:?}, txuie: {=bool:?}, abpsie: {=bool:?}, rxovfis: {=bool:?}, rxoie: {=bool:?} }}",
            self.txunfis(),
            self.abpsis(),
            self.txuie(),
            self.abpsie(),
            self.rxovfis(),
            self.rxoie()
        )
    }
}
#[doc = "Queue 0 Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq0Control(pub u32);
impl MtlRxq0Control {
    #[doc = "Receive Queue Weight."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq_wegt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Queue Weight."]
    #[inline(always)]
    pub const fn set_rxq_wegt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive Queue Packet Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq_frm_arbit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Packet Arbitration."]
    #[inline(always)]
    pub const fn set_rxq_frm_arbit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MtlRxq0Control {
    #[inline(always)]
    fn default() -> MtlRxq0Control {
        MtlRxq0Control(0)
    }
}
impl core::fmt::Debug for MtlRxq0Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq0Control")
            .field("rxq_wegt", &self.rxq_wegt())
            .field("rxq_frm_arbit", &self.rxq_frm_arbit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq0Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq0Control {{ rxq_wegt: {=u8:?}, rxq_frm_arbit: {=bool:?} }}",
            self.rxq_wegt(),
            self.rxq_frm_arbit()
        )
    }
}
#[doc = "Queue 0 Receive Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq0Debug(pub u32);
impl MtlRxq0Debug {
    #[doc = "MTL Rx Queue Write Controller Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rwcsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Rx Queue Write Controller Active Status."]
    #[inline(always)]
    pub const fn set_rwcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Rx Queue Read Controller State."]
    #[must_use]
    #[inline(always)]
    pub const fn rrcsts(&self) -> MtlRxq0DebugRrcsts {
        let val = (self.0 >> 1usize) & 0x03;
        MtlRxq0DebugRrcsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Read Controller State."]
    #[inline(always)]
    pub const fn set_rrcsts(&mut self, val: MtlRxq0DebugRrcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Rx Queue Fill-Level Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxqsts(&self) -> MtlRxq0DebugRxqsts {
        let val = (self.0 >> 4usize) & 0x03;
        MtlRxq0DebugRxqsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Fill-Level Status."]
    #[inline(always)]
    pub const fn set_rxqsts(&mut self, val: MtlRxq0DebugRxqsts) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of Packets in Receive Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn prxq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Number of Packets in Receive Queue."]
    #[inline(always)]
    pub const fn set_prxq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for MtlRxq0Debug {
    #[inline(always)]
    fn default() -> MtlRxq0Debug {
        MtlRxq0Debug(0)
    }
}
impl core::fmt::Debug for MtlRxq0Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq0Debug")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq0Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq0Debug {{ rwcsts: {=bool:?}, rrcsts: {:?}, rxqsts: {:?}, prxq: {=u16:?} }}",
            self.rwcsts(),
            self.rrcsts(),
            self.rxqsts(),
            self.prxq()
        )
    }
}
#[doc = "Queue 0 Missed Packet and Overflow Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq0MissedPacketOverflowCnt(pub u32);
impl MtlRxq0MissedPacketOverflowCnt {
    #[doc = "Overflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ovfpktcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Overflow Packet Counter."]
    #[inline(always)]
    pub const fn set_ovfpktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Counter Overflow Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ovfcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Counter Overflow Bit."]
    #[inline(always)]
    pub const fn set_ovfcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Missed Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn mispktcnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Missed Packet Counter."]
    #[inline(always)]
    pub const fn set_mispktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Missed Packet Counter Overflow Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn miscntovf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Missed Packet Counter Overflow Bit."]
    #[inline(always)]
    pub const fn set_miscntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MtlRxq0MissedPacketOverflowCnt {
    #[inline(always)]
    fn default() -> MtlRxq0MissedPacketOverflowCnt {
        MtlRxq0MissedPacketOverflowCnt(0)
    }
}
impl core::fmt::Debug for MtlRxq0MissedPacketOverflowCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq0MissedPacketOverflowCnt")
            .field("ovfpktcnt", &self.ovfpktcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .field("mispktcnt", &self.mispktcnt())
            .field("miscntovf", &self.miscntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq0MissedPacketOverflowCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq0MissedPacketOverflowCnt {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}",
            self.ovfpktcnt(),
            self.ovfcntovf(),
            self.mispktcnt(),
            self.miscntovf()
        )
    }
}
#[doc = "Queue 0 Receive Operation Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq0OperationMode(pub u32);
impl MtlRxq0OperationMode {
    #[doc = "Receive Queue Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> MtlRxq0OperationModeRtc {
        let val = (self.0 >> 0usize) & 0x03;
        MtlRxq0OperationModeRtc::from_bits(val as u8)
    }
    #[doc = "Receive Queue Threshold Control."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: MtlRxq0OperationModeRtc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Forward Undersized Good Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn fup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Undersized Good Packets."]
    #[inline(always)]
    pub const fn set_fup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Forward Error Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn fep(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Error Packets."]
    #[inline(always)]
    pub const fn set_fep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Queue Store and Forward."]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Store and Forward."]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_tcp_ef(&self) -> MtlRxq0OperationModeDisTcpEf {
        let val = (self.0 >> 6usize) & 0x01;
        MtlRxq0OperationModeDisTcpEf::from_bits(val as u8)
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
    #[inline(always)]
    pub const fn set_dis_tcp_ef(&mut self, val: MtlRxq0OperationModeDisTcpEf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Queue Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rqs(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Queue Size."]
    #[inline(always)]
    pub const fn set_rqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for MtlRxq0OperationMode {
    #[inline(always)]
    fn default() -> MtlRxq0OperationMode {
        MtlRxq0OperationMode(0)
    }
}
impl core::fmt::Debug for MtlRxq0OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq0OperationMode")
            .field("rtc", &self.rtc())
            .field("fup", &self.fup())
            .field("fep", &self.fep())
            .field("rsf", &self.rsf())
            .field("dis_tcp_ef", &self.dis_tcp_ef())
            .field("rqs", &self.rqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq0OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq0OperationMode {{ rtc: {:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, dis_tcp_ef: {:?}, rqs: {=u8:?} }}",
            self.rtc(),
            self.fup(),
            self.fep(),
            self.rsf(),
            self.dis_tcp_ef(),
            self.rqs()
        )
    }
}
#[doc = "Queue 1 Receive Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq1Control(pub u32);
impl MtlRxq1Control {
    #[doc = "Receive Queue Weight."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq_wegt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Queue Weight."]
    #[inline(always)]
    pub const fn set_rxq_wegt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive Queue Packet Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn rxq_frm_arbit(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Packet Arbitration."]
    #[inline(always)]
    pub const fn set_rxq_frm_arbit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MtlRxq1Control {
    #[inline(always)]
    fn default() -> MtlRxq1Control {
        MtlRxq1Control(0)
    }
}
impl core::fmt::Debug for MtlRxq1Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq1Control")
            .field("rxq_wegt", &self.rxq_wegt())
            .field("rxq_frm_arbit", &self.rxq_frm_arbit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq1Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq1Control {{ rxq_wegt: {=u8:?}, rxq_frm_arbit: {=bool:?} }}",
            self.rxq_wegt(),
            self.rxq_frm_arbit()
        )
    }
}
#[doc = "Queue 1 Receive Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq1Debug(pub u32);
impl MtlRxq1Debug {
    #[doc = "MTL Rx Queue Write Controller Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rwcsts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Rx Queue Write Controller Active Status."]
    #[inline(always)]
    pub const fn set_rwcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Rx Queue Read Controller State."]
    #[must_use]
    #[inline(always)]
    pub const fn rrcsts(&self) -> MtlRxq1DebugRrcsts {
        let val = (self.0 >> 1usize) & 0x03;
        MtlRxq1DebugRrcsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Read Controller State."]
    #[inline(always)]
    pub const fn set_rrcsts(&mut self, val: MtlRxq1DebugRrcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Rx Queue Fill-Level Status."]
    #[must_use]
    #[inline(always)]
    pub const fn rxqsts(&self) -> MtlRxq1DebugRxqsts {
        let val = (self.0 >> 4usize) & 0x03;
        MtlRxq1DebugRxqsts::from_bits(val as u8)
    }
    #[doc = "MTL Rx Queue Fill-Level Status."]
    #[inline(always)]
    pub const fn set_rxqsts(&mut self, val: MtlRxq1DebugRxqsts) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of Packets in Receive Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn prxq(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Number of Packets in Receive Queue."]
    #[inline(always)]
    pub const fn set_prxq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for MtlRxq1Debug {
    #[inline(always)]
    fn default() -> MtlRxq1Debug {
        MtlRxq1Debug(0)
    }
}
impl core::fmt::Debug for MtlRxq1Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq1Debug")
            .field("rwcsts", &self.rwcsts())
            .field("rrcsts", &self.rrcsts())
            .field("rxqsts", &self.rxqsts())
            .field("prxq", &self.prxq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq1Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq1Debug {{ rwcsts: {=bool:?}, rrcsts: {:?}, rxqsts: {:?}, prxq: {=u16:?} }}",
            self.rwcsts(),
            self.rrcsts(),
            self.rxqsts(),
            self.prxq()
        )
    }
}
#[doc = "Queue 1 Missed Packet and Overflow Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq1MissedPacketOverflowCnt(pub u32);
impl MtlRxq1MissedPacketOverflowCnt {
    #[doc = "Overflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ovfpktcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Overflow Packet Counter."]
    #[inline(always)]
    pub const fn set_ovfpktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Counter Overflow Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn ovfcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Counter Overflow Bit."]
    #[inline(always)]
    pub const fn set_ovfcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Missed Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn mispktcnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Missed Packet Counter."]
    #[inline(always)]
    pub const fn set_mispktcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[doc = "Missed Packet Counter Overflow Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn miscntovf(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Missed Packet Counter Overflow Bit."]
    #[inline(always)]
    pub const fn set_miscntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MtlRxq1MissedPacketOverflowCnt {
    #[inline(always)]
    fn default() -> MtlRxq1MissedPacketOverflowCnt {
        MtlRxq1MissedPacketOverflowCnt(0)
    }
}
impl core::fmt::Debug for MtlRxq1MissedPacketOverflowCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq1MissedPacketOverflowCnt")
            .field("ovfpktcnt", &self.ovfpktcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .field("mispktcnt", &self.mispktcnt())
            .field("miscntovf", &self.miscntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq1MissedPacketOverflowCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq1MissedPacketOverflowCnt {{ ovfpktcnt: {=u16:?}, ovfcntovf: {=bool:?}, mispktcnt: {=u16:?}, miscntovf: {=bool:?} }}",
            self.ovfpktcnt(),
            self.ovfcntovf(),
            self.mispktcnt(),
            self.miscntovf()
        )
    }
}
#[doc = "Queue 1 Receive Operation Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxq1OperationMode(pub u32);
impl MtlRxq1OperationMode {
    #[doc = "Receive Queue Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> MtlRxq1OperationModeRtc {
        let val = (self.0 >> 0usize) & 0x03;
        MtlRxq1OperationModeRtc::from_bits(val as u8)
    }
    #[doc = "Receive Queue Threshold Control."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: MtlRxq1OperationModeRtc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Forward Undersized Good Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn fup(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Undersized Good Packets."]
    #[inline(always)]
    pub const fn set_fup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Forward Error Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn fep(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Forward Error Packets."]
    #[inline(always)]
    pub const fn set_fep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive Queue Store and Forward."]
    #[must_use]
    #[inline(always)]
    pub const fn rsf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Queue Store and Forward."]
    #[inline(always)]
    pub const fn set_rsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_tcp_ef(&self) -> MtlRxq1OperationModeDisTcpEf {
        let val = (self.0 >> 6usize) & 0x01;
        MtlRxq1OperationModeDisTcpEf::from_bits(val as u8)
    }
    #[doc = "Disable Dropping of TCP/IP Checksum Error Packets."]
    #[inline(always)]
    pub const fn set_dis_tcp_ef(&mut self, val: MtlRxq1OperationModeDisTcpEf) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive Queue Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rqs(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Queue Size."]
    #[inline(always)]
    pub const fn set_rqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for MtlRxq1OperationMode {
    #[inline(always)]
    fn default() -> MtlRxq1OperationMode {
        MtlRxq1OperationMode(0)
    }
}
impl core::fmt::Debug for MtlRxq1OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxq1OperationMode")
            .field("rtc", &self.rtc())
            .field("fup", &self.fup())
            .field("fep", &self.fep())
            .field("rsf", &self.rsf())
            .field("dis_tcp_ef", &self.dis_tcp_ef())
            .field("rqs", &self.rqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxq1OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxq1OperationMode {{ rtc: {:?}, fup: {=bool:?}, fep: {=bool:?}, rsf: {=bool:?}, dis_tcp_ef: {:?}, rqs: {=u8:?} }}",
            self.rtc(),
            self.fup(),
            self.fep(),
            self.rsf(),
            self.dis_tcp_ef(),
            self.rqs()
        )
    }
}
#[doc = "Receive Queue and DMA Channel Mapping 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlRxqDmaMap0(pub u32);
impl MtlRxqDmaMap0 {
    #[doc = "Queue 0 Mapped to DMA Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn q0mdmach(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 0 Mapped to DMA Channel."]
    #[inline(always)]
    pub const fn set_q0mdmach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn q0ddmach(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection."]
    #[inline(always)]
    pub const fn set_q0ddmach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Queue 1 Mapped to DMA Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn q1mdmach(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 1 Mapped to DMA Channel."]
    #[inline(always)]
    pub const fn set_q1mdmach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn q1ddmach(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection."]
    #[inline(always)]
    pub const fn set_q1ddmach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for MtlRxqDmaMap0 {
    #[inline(always)]
    fn default() -> MtlRxqDmaMap0 {
        MtlRxqDmaMap0(0)
    }
}
impl core::fmt::Debug for MtlRxqDmaMap0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlRxqDmaMap0")
            .field("q0mdmach", &self.q0mdmach())
            .field("q0ddmach", &self.q0ddmach())
            .field("q1mdmach", &self.q1mdmach())
            .field("q1ddmach", &self.q1ddmach())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlRxqDmaMap0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlRxqDmaMap0 {{ q0mdmach: {=bool:?}, q0ddmach: {=bool:?}, q1mdmach: {=bool:?}, q1ddmach: {=bool:?} }}",
            self.q0mdmach(),
            self.q0ddmach(),
            self.q1mdmach(),
            self.q1ddmach()
        )
    }
}
#[doc = "Queue 0 Transmit Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq0Debug(pub u32);
impl MtlTxq0Debug {
    #[doc = "Transmit Queue in Pause."]
    #[must_use]
    #[inline(always)]
    pub const fn txqpaused(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue in Pause."]
    #[inline(always)]
    pub const fn set_txqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Tx Queue Read Controller Status."]
    #[must_use]
    #[inline(always)]
    pub const fn trcsts(&self) -> MtlTxq0DebugTrcsts {
        let val = (self.0 >> 1usize) & 0x03;
        MtlTxq0DebugTrcsts::from_bits(val as u8)
    }
    #[doc = "MTL Tx Queue Read Controller Status."]
    #[inline(always)]
    pub const fn set_trcsts(&mut self, val: MtlTxq0DebugTrcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Tx Queue Write Controller Status."]
    #[must_use]
    #[inline(always)]
    pub const fn twcsts(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Write Controller Status."]
    #[inline(always)]
    pub const fn set_twcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MTL Tx Queue Not Empty Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txqsts(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Not Empty Status."]
    #[inline(always)]
    pub const fn set_txqsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MTL Tx Status FIFO Full Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txstsfsts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Status FIFO Full Status."]
    #[inline(always)]
    pub const fn set_txstsfsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number of Packets in the Transmit Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxq(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Packets in the Transmit Queue."]
    #[inline(always)]
    pub const fn set_ptxq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn stxstsf(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
    #[inline(always)]
    pub const fn set_stxstsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for MtlTxq0Debug {
    #[inline(always)]
    fn default() -> MtlTxq0Debug {
        MtlTxq0Debug(0)
    }
}
impl core::fmt::Debug for MtlTxq0Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq0Debug")
            .field("txqpaused", &self.txqpaused())
            .field("trcsts", &self.trcsts())
            .field("twcsts", &self.twcsts())
            .field("txqsts", &self.txqsts())
            .field("txstsfsts", &self.txstsfsts())
            .field("ptxq", &self.ptxq())
            .field("stxstsf", &self.stxstsf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq0Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq0Debug {{ txqpaused: {=bool:?}, trcsts: {:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}",
            self.txqpaused(),
            self.trcsts(),
            self.twcsts(),
            self.txqsts(),
            self.txstsfsts(),
            self.ptxq(),
            self.stxstsf()
        )
    }
}
#[doc = "Queue 0 ETS Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq0EtsStatus(pub u32);
impl MtlTxq0EtsStatus {
    #[doc = "Average Bits per Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn abs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Average Bits per Slot."]
    #[inline(always)]
    pub const fn set_abs(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for MtlTxq0EtsStatus {
    #[inline(always)]
    fn default() -> MtlTxq0EtsStatus {
        MtlTxq0EtsStatus(0)
    }
}
impl core::fmt::Debug for MtlTxq0EtsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq0EtsStatus")
            .field("abs", &self.abs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq0EtsStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlTxq0EtsStatus {{ abs: {=u32:?} }}", self.abs())
    }
}
#[doc = "Queue 0 Transmit Operation Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq0OperationMode(pub u32);
impl MtlTxq0OperationMode {
    #[doc = "Flush Transmit Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ftq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush Transmit Queue."]
    #[inline(always)]
    pub const fn set_ftq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Store and Forward."]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Store and Forward."]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Queue Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txqen(&self) -> MtlTxq0OperationModeTxqen {
        let val = (self.0 >> 2usize) & 0x03;
        MtlTxq0OperationModeTxqen::from_bits(val as u8)
    }
    #[doc = "Transmit Queue Enable."]
    #[inline(always)]
    pub const fn set_txqen(&mut self, val: MtlTxq0OperationModeTxqen) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Transmit Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ttc(&self) -> MtlTxq0OperationModeTtc {
        let val = (self.0 >> 4usize) & 0x07;
        MtlTxq0OperationModeTtc::from_bits(val as u8)
    }
    #[doc = "Transmit Threshold Control."]
    #[inline(always)]
    pub const fn set_ttc(&mut self, val: MtlTxq0OperationModeTtc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit Queue Size."]
    #[must_use]
    #[inline(always)]
    pub const fn tqs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Queue Size."]
    #[inline(always)]
    pub const fn set_tqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for MtlTxq0OperationMode {
    #[inline(always)]
    fn default() -> MtlTxq0OperationMode {
        MtlTxq0OperationMode(0)
    }
}
impl core::fmt::Debug for MtlTxq0OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq0OperationMode")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("txqen", &self.txqen())
            .field("ttc", &self.ttc())
            .field("tqs", &self.tqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq0OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq0OperationMode {{ ftq: {=bool:?}, tsf: {=bool:?}, txqen: {:?}, ttc: {:?}, tqs: {=u8:?} }}",
            self.ftq(),
            self.tsf(),
            self.txqen(),
            self.ttc(),
            self.tqs()
        )
    }
}
#[doc = "Queue 0 Quantum or Weights."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq0QuantumWeight(pub u32);
impl MtlTxq0QuantumWeight {
    #[doc = "Quantum or Weights."]
    #[must_use]
    #[inline(always)]
    pub const fn iscqw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Quantum or Weights."]
    #[inline(always)]
    pub const fn set_iscqw(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for MtlTxq0QuantumWeight {
    #[inline(always)]
    fn default() -> MtlTxq0QuantumWeight {
        MtlTxq0QuantumWeight(0)
    }
}
impl core::fmt::Debug for MtlTxq0QuantumWeight {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq0QuantumWeight")
            .field("iscqw", &self.iscqw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq0QuantumWeight {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq0QuantumWeight {{ iscqw: {=u32:?} }}",
            self.iscqw()
        )
    }
}
#[doc = "Queue 0 Underflow Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq0Underflow(pub u32);
impl MtlTxq0Underflow {
    #[doc = "Underflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn uffrmcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Underflow Packet Counter."]
    #[inline(always)]
    pub const fn set_uffrmcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Bit for Underflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ufcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Bit for Underflow Packet Counter."]
    #[inline(always)]
    pub const fn set_ufcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for MtlTxq0Underflow {
    #[inline(always)]
    fn default() -> MtlTxq0Underflow {
        MtlTxq0Underflow(0)
    }
}
impl core::fmt::Debug for MtlTxq0Underflow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq0Underflow")
            .field("uffrmcnt", &self.uffrmcnt())
            .field("ufcntovf", &self.ufcntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq0Underflow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq0Underflow {{ uffrmcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
            self.uffrmcnt(),
            self.ufcntovf()
        )
    }
}
#[doc = "Queue 1 Transmit Debug."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1Debug(pub u32);
impl MtlTxq1Debug {
    #[doc = "Transmit Queue in Pause."]
    #[must_use]
    #[inline(always)]
    pub const fn txqpaused(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Queue in Pause."]
    #[inline(always)]
    pub const fn set_txqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MTL Tx Queue Read Controller Status."]
    #[must_use]
    #[inline(always)]
    pub const fn trcsts(&self) -> MtlTxq1DebugTrcsts {
        let val = (self.0 >> 1usize) & 0x03;
        MtlTxq1DebugTrcsts::from_bits(val as u8)
    }
    #[doc = "MTL Tx Queue Read Controller Status."]
    #[inline(always)]
    pub const fn set_trcsts(&mut self, val: MtlTxq1DebugTrcsts) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "MTL Tx Queue Write Controller Status."]
    #[must_use]
    #[inline(always)]
    pub const fn twcsts(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Write Controller Status."]
    #[inline(always)]
    pub const fn set_twcsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MTL Tx Queue Not Empty Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txqsts(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Queue Not Empty Status."]
    #[inline(always)]
    pub const fn set_txqsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MTL Tx Status FIFO Full Status."]
    #[must_use]
    #[inline(always)]
    pub const fn txstsfsts(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MTL Tx Status FIFO Full Status."]
    #[inline(always)]
    pub const fn set_txstsfsts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number of Packets in the Transmit Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ptxq(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Packets in the Transmit Queue."]
    #[inline(always)]
    pub const fn set_ptxq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn stxstsf(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Status Words in Tx Status FIFO of Queue."]
    #[inline(always)]
    pub const fn set_stxstsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for MtlTxq1Debug {
    #[inline(always)]
    fn default() -> MtlTxq1Debug {
        MtlTxq1Debug(0)
    }
}
impl core::fmt::Debug for MtlTxq1Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1Debug")
            .field("txqpaused", &self.txqpaused())
            .field("trcsts", &self.trcsts())
            .field("twcsts", &self.twcsts())
            .field("txqsts", &self.txqsts())
            .field("txstsfsts", &self.txstsfsts())
            .field("ptxq", &self.ptxq())
            .field("stxstsf", &self.stxstsf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq1Debug {{ txqpaused: {=bool:?}, trcsts: {:?}, twcsts: {=bool:?}, txqsts: {=bool:?}, txstsfsts: {=bool:?}, ptxq: {=u8:?}, stxstsf: {=u8:?} }}",
            self.txqpaused(),
            self.trcsts(),
            self.twcsts(),
            self.txqsts(),
            self.txstsfsts(),
            self.ptxq(),
            self.stxstsf()
        )
    }
}
#[doc = "Queue 1 ETS Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1EtsControl(pub u32);
impl MtlTxq1EtsControl {
    #[doc = "AV Algorithm."]
    #[must_use]
    #[inline(always)]
    pub const fn avalg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AV Algorithm."]
    #[inline(always)]
    pub const fn set_avalg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Credit Control."]
    #[must_use]
    #[inline(always)]
    pub const fn cc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Control."]
    #[inline(always)]
    pub const fn set_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Slot Count."]
    #[must_use]
    #[inline(always)]
    pub const fn slc(&self) -> Slc {
        let val = (self.0 >> 4usize) & 0x07;
        Slc::from_bits(val as u8)
    }
    #[doc = "Slot Count."]
    #[inline(always)]
    pub const fn set_slc(&mut self, val: Slc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for MtlTxq1EtsControl {
    #[inline(always)]
    fn default() -> MtlTxq1EtsControl {
        MtlTxq1EtsControl(0)
    }
}
impl core::fmt::Debug for MtlTxq1EtsControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1EtsControl")
            .field("avalg", &self.avalg())
            .field("cc", &self.cc())
            .field("slc", &self.slc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1EtsControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq1EtsControl {{ avalg: {=bool:?}, cc: {=bool:?}, slc: {:?} }}",
            self.avalg(),
            self.cc(),
            self.slc()
        )
    }
}
#[doc = "Queue 1 ETS Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1EtsStatus(pub u32);
impl MtlTxq1EtsStatus {
    #[doc = "Average Bits per Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn abs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Average Bits per Slot."]
    #[inline(always)]
    pub const fn set_abs(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for MtlTxq1EtsStatus {
    #[inline(always)]
    fn default() -> MtlTxq1EtsStatus {
        MtlTxq1EtsStatus(0)
    }
}
impl core::fmt::Debug for MtlTxq1EtsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1EtsStatus")
            .field("abs", &self.abs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1EtsStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlTxq1EtsStatus {{ abs: {=u32:?} }}", self.abs())
    }
}
#[doc = "Queue 1 hiCredit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1Hicredit(pub u32);
impl MtlTxq1Hicredit {
    #[doc = "hiCredit Value."]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "hiCredit Value."]
    #[inline(always)]
    pub const fn set_hc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for MtlTxq1Hicredit {
    #[inline(always)]
    fn default() -> MtlTxq1Hicredit {
        MtlTxq1Hicredit(0)
    }
}
impl core::fmt::Debug for MtlTxq1Hicredit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1Hicredit")
            .field("hc", &self.hc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1Hicredit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlTxq1Hicredit {{ hc: {=u32:?} }}", self.hc())
    }
}
#[doc = "Queue 1 loCredit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1Locredit(pub u32);
impl MtlTxq1Locredit {
    #[doc = "loCredit Value."]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "loCredit Value."]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for MtlTxq1Locredit {
    #[inline(always)]
    fn default() -> MtlTxq1Locredit {
        MtlTxq1Locredit(0)
    }
}
impl core::fmt::Debug for MtlTxq1Locredit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1Locredit")
            .field("lc", &self.lc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1Locredit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlTxq1Locredit {{ lc: {=u32:?} }}", self.lc())
    }
}
#[doc = "Queue 1 Transmit Operation Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1OperationMode(pub u32);
impl MtlTxq1OperationMode {
    #[doc = "Flush Transmit Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn ftq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush Transmit Queue."]
    #[inline(always)]
    pub const fn set_ftq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit Store and Forward."]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Store and Forward."]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Queue Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txqen(&self) -> MtlTxq1OperationModeTxqen {
        let val = (self.0 >> 2usize) & 0x03;
        MtlTxq1OperationModeTxqen::from_bits(val as u8)
    }
    #[doc = "Transmit Queue Enable."]
    #[inline(always)]
    pub const fn set_txqen(&mut self, val: MtlTxq1OperationModeTxqen) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Transmit Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ttc(&self) -> MtlTxq1OperationModeTtc {
        let val = (self.0 >> 4usize) & 0x07;
        MtlTxq1OperationModeTtc::from_bits(val as u8)
    }
    #[doc = "Transmit Threshold Control."]
    #[inline(always)]
    pub const fn set_ttc(&mut self, val: MtlTxq1OperationModeTtc) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit Queue Size."]
    #[must_use]
    #[inline(always)]
    pub const fn tqs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Queue Size."]
    #[inline(always)]
    pub const fn set_tqs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for MtlTxq1OperationMode {
    #[inline(always)]
    fn default() -> MtlTxq1OperationMode {
        MtlTxq1OperationMode(0)
    }
}
impl core::fmt::Debug for MtlTxq1OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1OperationMode")
            .field("ftq", &self.ftq())
            .field("tsf", &self.tsf())
            .field("txqen", &self.txqen())
            .field("ttc", &self.ttc())
            .field("tqs", &self.tqs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1OperationMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq1OperationMode {{ ftq: {=bool:?}, tsf: {=bool:?}, txqen: {:?}, ttc: {:?}, tqs: {=u8:?} }}",
            self.ftq(),
            self.tsf(),
            self.txqen(),
            self.ttc(),
            self.tqs()
        )
    }
}
#[doc = "Queue 1 idleSlopeCredit, Quantum or Weights."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1QuantumWeight(pub u32);
impl MtlTxq1QuantumWeight {
    #[doc = "idleSlopeCredit, Quantum or Weights."]
    #[must_use]
    #[inline(always)]
    pub const fn iscqw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "idleSlopeCredit, Quantum or Weights."]
    #[inline(always)]
    pub const fn set_iscqw(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for MtlTxq1QuantumWeight {
    #[inline(always)]
    fn default() -> MtlTxq1QuantumWeight {
        MtlTxq1QuantumWeight(0)
    }
}
impl core::fmt::Debug for MtlTxq1QuantumWeight {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1QuantumWeight")
            .field("iscqw", &self.iscqw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1QuantumWeight {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq1QuantumWeight {{ iscqw: {=u32:?} }}",
            self.iscqw()
        )
    }
}
#[doc = "Queue 1 sendSlopeCredit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1Sendslopecredit(pub u32);
impl MtlTxq1Sendslopecredit {
    #[doc = "sendSlopeCredit Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ssc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "sendSlopeCredit Value."]
    #[inline(always)]
    pub const fn set_ssc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for MtlTxq1Sendslopecredit {
    #[inline(always)]
    fn default() -> MtlTxq1Sendslopecredit {
        MtlTxq1Sendslopecredit(0)
    }
}
impl core::fmt::Debug for MtlTxq1Sendslopecredit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1Sendslopecredit")
            .field("ssc", &self.ssc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1Sendslopecredit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MtlTxq1Sendslopecredit {{ ssc: {=u16:?} }}", self.ssc())
    }
}
#[doc = "Queue 1 Underflow Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtlTxq1Underflow(pub u32);
impl MtlTxq1Underflow {
    #[doc = "Underflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn uffrmcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Underflow Packet Counter."]
    #[inline(always)]
    pub const fn set_uffrmcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Overflow Bit for Underflow Packet Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ufcntovf(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overflow Bit for Underflow Packet Counter."]
    #[inline(always)]
    pub const fn set_ufcntovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for MtlTxq1Underflow {
    #[inline(always)]
    fn default() -> MtlTxq1Underflow {
        MtlTxq1Underflow(0)
    }
}
impl core::fmt::Debug for MtlTxq1Underflow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MtlTxq1Underflow")
            .field("uffrmcnt", &self.uffrmcnt())
            .field("ufcntovf", &self.ufcntovf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MtlTxq1Underflow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MtlTxq1Underflow {{ uffrmcnt: {=u16:?}, ufcntovf: {=bool:?} }}",
            self.uffrmcnt(),
            self.ufcntovf()
        )
    }
}
#[doc = "PPS0 Target Time Nanoseconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pps0TargetTimeNanoseconds(pub u32);
impl Pps0TargetTimeNanoseconds {
    #[doc = "Target Time Low for PPS Register."]
    #[must_use]
    #[inline(always)]
    pub const fn ttsl0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Target Time Low for PPS Register."]
    #[inline(always)]
    pub const fn set_ttsl0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for Pps0TargetTimeNanoseconds {
    #[inline(always)]
    fn default() -> Pps0TargetTimeNanoseconds {
        Pps0TargetTimeNanoseconds(0)
    }
}
impl core::fmt::Debug for Pps0TargetTimeNanoseconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pps0TargetTimeNanoseconds")
            .field("ttsl0", &self.ttsl0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pps0TargetTimeNanoseconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pps0TargetTimeNanoseconds {{ ttsl0: {=u32:?} }}",
            self.ttsl0()
        )
    }
}
#[doc = "PPS0 Target Time Seconds."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pps0TargetTimeSeconds(pub u32);
impl Pps0TargetTimeSeconds {
    #[doc = "PPS Target Time Seconds Register."]
    #[must_use]
    #[inline(always)]
    pub const fn tstrh0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "PPS Target Time Seconds Register."]
    #[inline(always)]
    pub const fn set_tstrh0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pps0TargetTimeSeconds {
    #[inline(always)]
    fn default() -> Pps0TargetTimeSeconds {
        Pps0TargetTimeSeconds(0)
    }
}
impl core::fmt::Debug for Pps0TargetTimeSeconds {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pps0TargetTimeSeconds")
            .field("tstrh0", &self.tstrh0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pps0TargetTimeSeconds {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pps0TargetTimeSeconds {{ tstrh0: {=u32:?} }}",
            self.tstrh0()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actphysel {
    #[doc = "GMII or MII."]
    GmiiMii = 0x0,
    #[doc = "RGMII."]
    Rgmii = 0x01,
    #[doc = "SGMII."]
    Sgmii = 0x02,
    #[doc = "TBI."]
    Tbi = 0x03,
    #[doc = "RMII."]
    Rmii = 0x04,
    #[doc = "RTBI."]
    Rtbi = 0x05,
    #[doc = "SMII."]
    Smii = 0x06,
    #[doc = "RevMII."]
    Revmiii = 0x07,
}
impl Actphysel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Actphysel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Actphysel {
    #[inline(always)]
    fn from(val: u8) -> Actphysel {
        Actphysel::from_bits(val)
    }
}
impl From<Actphysel> for u8 {
    #[inline(always)]
    fn from(val: Actphysel) -> u8 {
        Actphysel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addr64 {
    #[doc = "32."]
    M32 = 0x0,
    #[doc = "40."]
    M40 = 0x01,
    #[doc = "48."]
    M48 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Addr64 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addr64 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addr64 {
    #[inline(always)]
    fn from(val: u8) -> Addr64 {
        Addr64::from_bits(val)
    }
}
impl From<Addr64> for u8 {
    #[inline(always)]
    fn from(val: Addr64) -> u8 {
        Addr64::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addsub {
    #[doc = "Add time."]
    Add = 0x0,
    #[doc = "Subtract time."]
    Sub = 0x01,
}
impl Addsub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addsub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addsub {
    #[inline(always)]
    fn from(val: u8) -> Addsub {
        Addsub::from_bits(val)
    }
}
impl From<Addsub> for u8 {
    #[inline(always)]
    fn from(val: Addsub) -> u8 {
        Addsub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Asp {
    #[doc = "No Safety features selected."]
    None = 0x0,
    #[doc = "Only \"ECC protection for external memory\" feature is selected."]
    EccOnly = 0x01,
    #[doc = "All the Automotive Safety features are selected without the \"Parity Port Enable for external interface\" feature."]
    AsNppe = 0x02,
    #[doc = "All the Automotive Safety features are selected with the \"Parity Port Enable for external interface\" feature."]
    AsPpe = 0x03,
}
impl Asp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asp {
    #[inline(always)]
    fn from(val: u8) -> Asp {
        Asp::from_bits(val)
    }
}
impl From<Asp> for u8 {
    #[inline(always)]
    fn from(val: Asp) -> u8 {
        Asp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Auxsnapnum {
    #[doc = "No auxiliary input."]
    NoAuxi = 0x0,
    #[doc = "1 auxiliary input."]
    M1Auxi = 0x01,
    #[doc = "2 auxiliary input."]
    M2Auxi = 0x02,
    #[doc = "3 auxiliary input."]
    M3Auxi = 0x03,
    #[doc = "4 auxiliary input."]
    M4Auxi = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Auxsnapnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Auxsnapnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Auxsnapnum {
    #[inline(always)]
    fn from(val: u8) -> Auxsnapnum {
        Auxsnapnum::from_bits(val)
    }
}
impl From<Auxsnapnum> for u8 {
    #[inline(always)]
    fn from(val: Auxsnapnum) -> u8 {
        Auxsnapnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avcpq {
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Avcpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avcpq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avcpq {
    #[inline(always)]
    fn from(val: u8) -> Avcpq {
        Avcpq::from_bits(val)
    }
}
impl From<Avcpq> for u8 {
    #[inline(always)]
    fn from(val: Avcpq) -> u8 {
        Avcpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bl {
    #[doc = "k = min(n,10)."]
    MinN10 = 0x0,
    #[doc = "k = min(n,8)."]
    MinN8 = 0x01,
    #[doc = "k = min(n,4)."]
    MinN4 = 0x02,
    #[doc = "k = min(n,1)."]
    MinN1 = 0x03,
}
impl Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bl {
    #[inline(always)]
    fn from(val: u8) -> Bl {
        Bl::from_bits(val)
    }
}
impl From<Bl> for u8 {
    #[inline(always)]
    fn from(val: Bl) -> u8 {
        Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Com {
    #[doc = "Write operation."]
    Write = 0x0,
    #[doc = "Read operation."]
    Read = 0x01,
}
impl Com {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Com {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Com {
    #[inline(always)]
    fn from(val: u8) -> Com {
        Com::from_bits(val)
    }
}
impl From<Com> for u8 {
    #[inline(always)]
    fn from(val: Com) -> u8 {
        Com::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Da {
    #[doc = "Weighted Round-Robin with Rx:Tx or Tx:Rx."]
    Wrr = 0x0,
    #[doc = "Fixed Priority."]
    Fp = 0x01,
}
impl Da {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Da {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Da {
    #[inline(always)]
    fn from(val: u8) -> Da {
        Da::from_bits(val)
    }
}
impl From<Da> for u8 {
    #[inline(always)]
    fn from(val: Da) -> u8 {
        Da::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbf {
    #[doc = "Enable Broadcast Packets."]
    Enable = 0x0,
    #[doc = "Disable Broadcast Packets."]
    Disable = 0x01,
}
impl Dbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbf {
    #[inline(always)]
    fn from(val: u8) -> Dbf {
        Dbf::from_bits(val)
    }
}
impl From<Dbf> for u8 {
    #[inline(always)]
    fn from(val: Dbf) -> u8 {
        Dbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrcc {
    #[doc = "CRC Checking is enabled."]
    Enable = 0x0,
    #[doc = "CRC Checking is disabled."]
    Disable = 0x01,
}
impl Dcrcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrcc {
    #[inline(always)]
    fn from(val: u8) -> Dcrcc {
        Dcrcc::from_bits(val)
    }
}
impl From<Dcrcc> for u8 {
    #[inline(always)]
    fn from(val: Dcrcc) -> u8 {
        Dcrcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcrs {
    #[doc = "Enable Carrier Sense During Transmission."]
    Enable = 0x0,
    #[doc = "Disable Carrier Sense During Transmission."]
    Disable = 0x01,
}
impl Dcrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcrs {
    #[inline(always)]
    fn from(val: u8) -> Dcrs {
        Dcrs::from_bits(val)
    }
}
impl From<Dcrs> for u8 {
    #[inline(always)]
    fn from(val: Dcrs) -> u8 {
        Dcrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dm {
    #[doc = "Half-duplex mode."]
    Hduplx = 0x0,
    #[doc = "Full-duplex mode."]
    Fduplx = 0x01,
}
impl Dm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dm {
    #[inline(always)]
    fn from(val: u8) -> Dm {
        Dm::from_bits(val)
    }
}
impl From<Dm> for u8 {
    #[inline(always)]
    fn from(val: Dm) -> u8 {
        Dm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0RxControlSr {
    #[doc = "Stop Receive."]
    Stop = 0x0,
    #[doc = "Start Receive."]
    Start = 0x01,
}
impl DmaCh0RxControlSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0RxControlSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0RxControlSr {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0RxControlSr {
        DmaCh0RxControlSr::from_bits(val)
    }
}
impl From<DmaCh0RxControlSr> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0RxControlSr) -> u8 {
        DmaCh0RxControlSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh0TxControlSt {
    #[doc = "Stop Transmission Command."]
    Stop = 0x0,
    #[doc = "Start Transmission Command."]
    Start = 0x01,
}
impl DmaCh0TxControlSt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh0TxControlSt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh0TxControlSt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh0TxControlSt {
        DmaCh0TxControlSt::from_bits(val)
    }
}
impl From<DmaCh0TxControlSt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh0TxControlSt) -> u8 {
        DmaCh0TxControlSt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1RxControlSr {
    #[doc = "Stop Receive."]
    Stop = 0x0,
    #[doc = "Start Receive."]
    Start = 0x01,
}
impl DmaCh1RxControlSr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1RxControlSr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1RxControlSr {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1RxControlSr {
        DmaCh1RxControlSr::from_bits(val)
    }
}
impl From<DmaCh1RxControlSr> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1RxControlSr) -> u8 {
        DmaCh1RxControlSr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaCh1TxControlSt {
    #[doc = "Stop Transmission Command."]
    Stop = 0x0,
    #[doc = "Start Transmission Command."]
    Start = 0x01,
}
impl DmaCh1TxControlSt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaCh1TxControlSt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaCh1TxControlSt {
    #[inline(always)]
    fn from(val: u8) -> DmaCh1TxControlSt {
        DmaCh1TxControlSt::from_bits(val)
    }
}
impl From<DmaCh1TxControlSt> for u8 {
    #[inline(always)]
    fn from(val: DmaCh1TxControlSt) -> u8 {
        DmaCh1TxControlSt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmaModePr {
    #[doc = "The priority ratio is 1:1."]
    R11 = 0x0,
    #[doc = "The priority ratio is 2:1."]
    R21 = 0x01,
    #[doc = "The priority ratio is 3:1."]
    R31 = 0x02,
    #[doc = "The priority ratio is 4:1."]
    R41 = 0x03,
    #[doc = "The priority ratio is 5:1."]
    R51 = 0x04,
    #[doc = "The priority ratio is 6:1."]
    R61 = 0x05,
    #[doc = "The priority ratio is 7:1."]
    R71 = 0x06,
    #[doc = "The priority ratio is 8:1."]
    R81 = 0x07,
}
impl DmaModePr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmaModePr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmaModePr {
    #[inline(always)]
    fn from(val: u8) -> DmaModePr {
        DmaModePr::from_bits(val)
    }
}
impl From<DmaModePr> for u8 {
    #[inline(always)]
    fn from(val: DmaModePr) -> u8 {
        DmaModePr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Do {
    #[doc = "Enable Receive Own."]
    Enable = 0x0,
    #[doc = "Disable Receive Own."]
    Disable = 0x01,
}
impl Do {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Do {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Do {
    #[inline(always)]
    fn from(val: u8) -> Do {
        Do::from_bits(val)
    }
}
impl From<Do> for u8 {
    #[inline(always)]
    fn from(val: Do) -> u8 {
        Do::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dovltc {
    #[doc = "VLAN Type Check is enabled."]
    Enable = 0x0,
    #[doc = "VLAN Type Check is disabled."]
    Disable = 0x01,
}
impl Dovltc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dovltc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dovltc {
    #[inline(always)]
    fn from(val: u8) -> Dovltc {
        Dovltc::from_bits(val)
    }
}
impl From<Dovltc> for u8 {
    #[inline(always)]
    fn from(val: Dovltc) -> u8 {
        Dovltc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dr {
    #[doc = "Enable Retry."]
    Enable = 0x0,
    #[doc = "Disable Retry."]
    Disable = 0x01,
}
impl Dr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dr {
    #[inline(always)]
    fn from(val: u8) -> Dr {
        Dr::from_bits(val)
    }
}
impl From<Dr> for u8 {
    #[inline(always)]
    fn from(val: Dr) -> u8 {
        Dr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dzpq {
    #[doc = "Zero-Quanta Pause packet generation is enabled."]
    Enable = 0x0,
    #[doc = "Zero-Quanta Pause packet generation is disabled."]
    Disable = 0x01,
}
impl Dzpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dzpq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dzpq {
    #[inline(always)]
    fn from(val: u8) -> Dzpq {
        Dzpq::from_bits(val)
    }
}
impl From<Dzpq> for u8 {
    #[inline(always)]
    fn from(val: Dzpq) -> u8 {
        Dzpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eivls {
    #[doc = "Do not strip."]
    Donot = 0x0,
    #[doc = "Strip if VLAN filter passes."]
    Ifpass = 0x01,
    #[doc = "Strip if VLAN filter fails."]
    Iffail = 0x02,
    #[doc = "Always strip."]
    Always = 0x03,
}
impl Eivls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eivls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eivls {
    #[inline(always)]
    fn from(val: u8) -> Eivls {
        Eivls::from_bits(val)
    }
}
impl From<Eivls> for u8 {
    #[inline(always)]
    fn from(val: Eivls) -> u8 {
        Eivls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estdep {
    #[doc = "No Depth configured."]
    Nodepth = 0x0,
    #[doc = "64."]
    Depth64 = 0x01,
    #[doc = "128."]
    Depth128 = 0x02,
    #[doc = "256."]
    Depth256 = 0x03,
    #[doc = "512."]
    Depth512 = 0x04,
    #[doc = "1024."]
    Depth1024 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Estdep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estdep {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estdep {
    #[inline(always)]
    fn from(val: u8) -> Estdep {
        Estdep::from_bits(val)
    }
}
impl From<Estdep> for u8 {
    #[inline(always)]
    fn from(val: Estdep) -> u8 {
        Estdep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Estwid {
    #[doc = "Width not configured."]
    Nowidth = 0x0,
    #[doc = "16."]
    Width16 = 0x01,
    #[doc = "20."]
    Width20 = 0x02,
    #[doc = "24."]
    Width24 = 0x03,
}
impl Estwid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Estwid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Estwid {
    #[inline(always)]
    fn from(val: u8) -> Estwid {
        Estwid::from_bits(val)
    }
}
impl From<Estwid> for u8 {
    #[inline(always)]
    fn from(val: Estwid) -> u8 {
        Estwid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evls {
    #[doc = "Do not strip."]
    Donot = 0x0,
    #[doc = "Strip if VLAN filter passes."]
    Ifpass = 0x01,
    #[doc = "Strip if VLAN filter fails."]
    Iffail = 0x02,
    #[doc = "Always strip."]
    Always = 0x03,
}
impl Evls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evls {
    #[inline(always)]
    fn from(val: u8) -> Evls {
        Evls::from_bits(val)
    }
}
impl From<Evls> for u8 {
    #[inline(always)]
    fn from(val: Evls) -> u8 {
        Evls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fes {
    #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0."]
    M101000m = 0x0,
    #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0."]
    M1002500m = 0x01,
}
impl Fes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fes {
    #[inline(always)]
    fn from(val: u8) -> Fes {
        Fes::from_bits(val)
    }
}
impl From<Fes> for u8 {
    #[inline(always)]
    fn from(val: Fes) -> u8 {
        Fes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpbs {
    #[doc = "64 Bytes."]
    M64bytes = 0x0,
    #[doc = "128 Bytes."]
    M128bytes = 0x01,
    #[doc = "256 Bytes."]
    M256bytes = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpbs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpbs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpbs {
    #[inline(always)]
    fn from(val: u8) -> Frpbs {
        Frpbs::from_bits(val)
    }
}
impl From<Frpbs> for u8 {
    #[inline(always)]
    fn from(val: Frpbs) -> u8 {
        Frpbs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frpes {
    #[doc = "64 Entries."]
    M64entr = 0x0,
    #[doc = "128 Entries."]
    M128entr = 0x01,
    #[doc = "256 Entries."]
    M256entr = 0x02,
    _RESERVED_3 = 0x03,
}
impl Frpes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frpes {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frpes {
    #[inline(always)]
    fn from(val: u8) -> Frpes {
        Frpes::from_bits(val)
    }
}
impl From<Frpes> for u8 {
    #[inline(always)]
    fn from(val: Frpes) -> u8 {
        Frpes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hashtblsz {
    #[doc = "No hash table."]
    NoHt = 0x0,
    #[doc = "64."]
    M64 = 0x01,
    #[doc = "128."]
    M128 = 0x02,
    #[doc = "256."]
    M256 = 0x03,
}
impl Hashtblsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hashtblsz {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hashtblsz {
    #[inline(always)]
    fn from(val: u8) -> Hashtblsz {
        Hashtblsz::from_bits(val)
    }
}
impl From<Hashtblsz> for u8 {
    #[inline(always)]
    fn from(val: Hashtblsz) -> u8 {
        Hashtblsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipg {
    #[doc = "96 bit times IPG."]
    Ipg96 = 0x0,
    #[doc = "88 bit times IPG."]
    Ipg88 = 0x01,
    #[doc = "80 bit times IPG."]
    Ipg80 = 0x02,
    #[doc = "72 bit times IPG."]
    Ipg72 = 0x03,
    #[doc = "64 bit times IPG."]
    Ipg64 = 0x04,
    #[doc = "56 bit times IPG."]
    Ipg56 = 0x05,
    #[doc = "48 bit times IPG."]
    Ipg48 = 0x06,
    #[doc = "40 bit times IPG."]
    Ipg40 = 0x07,
}
impl Ipg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipg {
    #[inline(always)]
    fn from(val: u8) -> Ipg {
        Ipg::from_bits(val)
    }
}
impl From<Ipg> for u8 {
    #[inline(always)]
    fn from(val: Ipg) -> u8 {
        Ipg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Jd {
    #[doc = "Jabber is enabled."]
    Enable = 0x0,
    #[doc = "Jabber is disabled."]
    Disable = 0x01,
}
impl Jd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Jd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Jd {
    #[inline(always)]
    fn from(val: u8) -> Jd {
        Jd::from_bits(val)
    }
}
impl From<Jd> for u8 {
    #[inline(always)]
    fn from(val: Jd) -> u8 {
        Jd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L3l4fnum {
    #[doc = "No L3 or L4 Filter."]
    Nofilt = 0x0,
    #[doc = "1 L3 or L4 Filter."]
    M1filt = 0x01,
    #[doc = "2 L3 or L4 Filters."]
    M2filt = 0x02,
    #[doc = "3 L3 or L4 Filters."]
    M3filt = 0x03,
    #[doc = "4 L3 or L4 Filters."]
    M4filt = 0x04,
    #[doc = "5 L3 or L4 Filters."]
    M5filt = 0x05,
    #[doc = "6 L3 or L4 Filters."]
    M6filt = 0x06,
    #[doc = "7 L3 or L4 Filters."]
    M7filt = 0x07,
    #[doc = "8 L3 or L4 Filters."]
    M8filt = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl L3l4fnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L3l4fnum {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L3l4fnum {
    #[inline(always)]
    fn from(val: u8) -> L3l4fnum {
        L3l4fnum::from_bits(val)
    }
}
impl From<L3l4fnum> for u8 {
    #[inline(always)]
    fn from(val: L3l4fnum) -> u8 {
        L3l4fnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted."]
    Cvlan = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted."]
    Svlan = 0x01,
}
impl MacInnerVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclCsvl {
        MacInnerVlanInclCsvl::from_bits(val)
    }
}
impl From<MacInnerVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclCsvl) -> u8 {
        MacInnerVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacInnerVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement."]
    None = 0x0,
    #[doc = "VLAN tag deletion."]
    Delete = 0x01,
    #[doc = "VLAN tag insertion."]
    Insert = 0x02,
    #[doc = "VLAN tag replacement."]
    Replace = 0x03,
}
impl MacInnerVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacInnerVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacInnerVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacInnerVlanInclVlc {
        MacInnerVlanInclVlc::from_bits(val)
    }
}
impl From<MacInnerVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacInnerVlanInclVlc) -> u8 {
        MacInnerVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclCsvl {
    #[doc = "C-VLAN type (0x8100) is inserted or replaced."]
    CVlan = 0x0,
    #[doc = "S-VLAN type (0x88A8) is inserted or replaced."]
    SVlan = 0x01,
}
impl MacVlanInclCsvl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclCsvl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclCsvl {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclCsvl {
        MacVlanInclCsvl::from_bits(val)
    }
}
impl From<MacVlanInclCsvl> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclCsvl) -> u8 {
        MacVlanInclCsvl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MacVlanInclVlc {
    #[doc = "No VLAN tag deletion, insertion, or replacement."]
    None = 0x0,
    #[doc = "VLAN tag deletion."]
    Delete = 0x01,
    #[doc = "VLAN tag insertion."]
    Insert = 0x02,
    #[doc = "VLAN tag replacement."]
    Replace = 0x03,
}
impl MacVlanInclVlc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MacVlanInclVlc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MacVlanInclVlc {
    #[inline(always)]
    fn from(val: u8) -> MacVlanInclVlc {
        MacVlanInclVlc::from_bits(val)
    }
}
impl From<MacVlanInclVlc> for u8 {
    #[inline(always)]
    fn from(val: MacVlanInclVlc) -> u8 {
        MacVlanInclVlc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mcbcq {
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mcbcq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mcbcq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mcbcq {
    #[inline(always)]
    fn from(val: u8) -> Mcbcq {
        Mcbcq::from_bits(val)
    }
}
impl From<Mcbcq> for u8 {
    #[inline(always)]
    fn from(val: Mcbcq) -> u8 {
        Mcbcq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0DebugRrcsts {
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Reading packet data."]
    ReadData = 0x01,
    #[doc = "Reading packet status (or timestamp)."]
    ReadSts = 0x02,
    #[doc = "Flushing the packet data and status."]
    Flush = 0x03,
}
impl MtlRxq0DebugRrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0DebugRrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0DebugRrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0DebugRrcsts {
        MtlRxq0DebugRrcsts::from_bits(val)
    }
}
impl From<MtlRxq0DebugRrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0DebugRrcsts) -> u8 {
        MtlRxq0DebugRrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0DebugRxqsts {
    #[doc = "Rx Queue empty."]
    Empty = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold."]
    BlwThr = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold."]
    AbvThr = 0x02,
    #[doc = "Rx Queue full."]
    Full = 0x03,
}
impl MtlRxq0DebugRxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0DebugRxqsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0DebugRxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0DebugRxqsts {
        MtlRxq0DebugRxqsts::from_bits(val)
    }
}
impl From<MtlRxq0DebugRxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0DebugRxqsts) -> u8 {
        MtlRxq0DebugRxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeDisTcpEf {
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled."]
    Enable = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled."]
    Disable = 0x01,
}
impl MtlRxq0OperationModeDisTcpEf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeDisTcpEf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeDisTcpEf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeDisTcpEf {
        MtlRxq0OperationModeDisTcpEf::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeDisTcpEf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeDisTcpEf) -> u8 {
        MtlRxq0OperationModeDisTcpEf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq0OperationModeRtc {
    #[doc = "64."]
    M64byte = 0x0,
    #[doc = "32."]
    M32byte = 0x01,
    #[doc = "96."]
    M96byte = 0x02,
    #[doc = "128."]
    M128byte = 0x03,
}
impl MtlRxq0OperationModeRtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq0OperationModeRtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq0OperationModeRtc {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq0OperationModeRtc {
        MtlRxq0OperationModeRtc::from_bits(val)
    }
}
impl From<MtlRxq0OperationModeRtc> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq0OperationModeRtc) -> u8 {
        MtlRxq0OperationModeRtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1DebugRrcsts {
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Reading packet data."]
    ReadData = 0x01,
    #[doc = "Reading packet status (or timestamp)."]
    ReadSts = 0x02,
    #[doc = "Flushing the packet data and status."]
    Flush = 0x03,
}
impl MtlRxq1DebugRrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1DebugRrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1DebugRrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1DebugRrcsts {
        MtlRxq1DebugRrcsts::from_bits(val)
    }
}
impl From<MtlRxq1DebugRrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1DebugRrcsts) -> u8 {
        MtlRxq1DebugRrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1DebugRxqsts {
    #[doc = "Rx Queue empty."]
    Empty = 0x0,
    #[doc = "Rx Queue fill-level below flow-control deactivate threshold."]
    BlwThr = 0x01,
    #[doc = "Rx Queue fill-level above flow-control activate threshold."]
    AbvThr = 0x02,
    #[doc = "Rx Queue full."]
    Full = 0x03,
}
impl MtlRxq1DebugRxqsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1DebugRxqsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1DebugRxqsts {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1DebugRxqsts {
        MtlRxq1DebugRxqsts::from_bits(val)
    }
}
impl From<MtlRxq1DebugRxqsts> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1DebugRxqsts) -> u8 {
        MtlRxq1DebugRxqsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeDisTcpEf {
    #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled."]
    Enable = 0x0,
    #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled."]
    Disable = 0x01,
}
impl MtlRxq1OperationModeDisTcpEf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeDisTcpEf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeDisTcpEf {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeDisTcpEf {
        MtlRxq1OperationModeDisTcpEf::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeDisTcpEf> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeDisTcpEf) -> u8 {
        MtlRxq1OperationModeDisTcpEf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlRxq1OperationModeRtc {
    #[doc = "64."]
    M64byte = 0x0,
    #[doc = "32."]
    M32byte = 0x01,
    #[doc = "96."]
    M96byte = 0x02,
    #[doc = "128."]
    M128byte = 0x03,
}
impl MtlRxq1OperationModeRtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlRxq1OperationModeRtc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlRxq1OperationModeRtc {
    #[inline(always)]
    fn from(val: u8) -> MtlRxq1OperationModeRtc {
        MtlRxq1OperationModeRtc::from_bits(val)
    }
}
impl From<MtlRxq1OperationModeRtc> for u8 {
    #[inline(always)]
    fn from(val: MtlRxq1OperationModeRtc) -> u8 {
        MtlRxq1OperationModeRtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0DebugTrcsts {
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)."]
    Read = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter."]
    Wait = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC."]
    Flush = 0x03,
}
impl MtlTxq0DebugTrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0DebugTrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0DebugTrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0DebugTrcsts {
        MtlTxq0DebugTrcsts::from_bits(val)
    }
}
impl From<MtlTxq0DebugTrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0DebugTrcsts) -> u8 {
        MtlTxq0DebugTrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeTtc {
    #[doc = "32."]
    M32bytes = 0x0,
    #[doc = "64."]
    M64bytes = 0x01,
    #[doc = "96."]
    M96bytes = 0x02,
    #[doc = "128."]
    M128bytes = 0x03,
    #[doc = "192."]
    M192bytes = 0x04,
    #[doc = "256."]
    M256bytes = 0x05,
    #[doc = "384."]
    M384bytes = 0x06,
    #[doc = "512."]
    M512bytes = 0x07,
}
impl MtlTxq0OperationModeTtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeTtc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeTtc {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeTtc {
        MtlTxq0OperationModeTtc::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeTtc> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeTtc) -> u8 {
        MtlTxq0OperationModeTtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq0OperationModeTxqen {
    #[doc = "Not enabled."]
    Disable = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)."]
    EnIfAv = 0x01,
    #[doc = "Enabled."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MtlTxq0OperationModeTxqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq0OperationModeTxqen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq0OperationModeTxqen {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq0OperationModeTxqen {
        MtlTxq0OperationModeTxqen::from_bits(val)
    }
}
impl From<MtlTxq0OperationModeTxqen> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq0OperationModeTxqen) -> u8 {
        MtlTxq0OperationModeTxqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1DebugTrcsts {
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Read state (transferring data to the MAC transmitter)."]
    Read = 0x01,
    #[doc = "Waiting for pending Tx Status from the MAC transmitter."]
    Wait = 0x02,
    #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC."]
    Flush = 0x03,
}
impl MtlTxq1DebugTrcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1DebugTrcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1DebugTrcsts {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1DebugTrcsts {
        MtlTxq1DebugTrcsts::from_bits(val)
    }
}
impl From<MtlTxq1DebugTrcsts> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1DebugTrcsts) -> u8 {
        MtlTxq1DebugTrcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeTtc {
    #[doc = "32."]
    M32bytes = 0x0,
    #[doc = "64."]
    M64bytes = 0x01,
    #[doc = "96."]
    M96bytes = 0x02,
    #[doc = "128."]
    M128bytes = 0x03,
    #[doc = "192."]
    M192bytes = 0x04,
    #[doc = "256."]
    M256bytes = 0x05,
    #[doc = "384."]
    M384bytes = 0x06,
    #[doc = "512."]
    M512bytes = 0x07,
}
impl MtlTxq1OperationModeTtc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeTtc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeTtc {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeTtc {
        MtlTxq1OperationModeTtc::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeTtc> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeTtc) -> u8 {
        MtlTxq1OperationModeTtc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MtlTxq1OperationModeTxqen {
    #[doc = "Not enabled."]
    Disable = 0x0,
    #[doc = "Enable in AV mode (Reserved in non-AV)."]
    EnIfAv = 0x01,
    #[doc = "Enabled."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl MtlTxq1OperationModeTxqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MtlTxq1OperationModeTxqen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MtlTxq1OperationModeTxqen {
    #[inline(always)]
    fn from(val: u8) -> MtlTxq1OperationModeTxqen {
        MtlTxq1OperationModeTxqen::from_bits(val)
    }
}
impl From<MtlTxq1OperationModeTxqen> for u8 {
    #[inline(always)]
    fn from(val: MtlTxq1OperationModeTxqen) -> u8 {
        MtlTxq1OperationModeTxqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nrvf {
    #[doc = "No Extended Rx VLAN Filters."]
    NoErvlan = 0x0,
    #[doc = "4 Extended Rx VLAN Filters."]
    M4Ervlan = 0x01,
    #[doc = "8 Extended Rx VLAN Filters."]
    M8Ervlan = 0x02,
    #[doc = "16 Extended Rx VLAN Filters."]
    M16Ervlan = 0x03,
    #[doc = "24 Extended Rx VLAN Filters."]
    M24Ervlan = 0x04,
    #[doc = "32 Extended Rx VLAN Filters."]
    M32Ervlan = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Nrvf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nrvf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nrvf {
    #[inline(always)]
    fn from(val: u8) -> Nrvf {
        Nrvf::from_bits(val)
    }
}
impl From<Nrvf> for u8 {
    #[inline(always)]
    fn from(val: Nrvf) -> u8 {
        Nrvf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pcf {
    #[doc = "MAC filters all control packets from reaching the application."]
    FltrAll = 0x0,
    #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the address filter."]
    FwXcptPau = 0x01,
    #[doc = "MAC forwards all control packets to the application even if they fail the address filter."]
    FwAll = 0x02,
    #[doc = "MAC forwards the control packets that pass the Address filter."]
    FwPass = 0x03,
}
impl Pcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pcf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pcf {
    #[inline(always)]
    fn from(val: u8) -> Pcf {
        Pcf::from_bits(val)
    }
}
impl From<Pcf> for u8 {
    #[inline(always)]
    fn from(val: Pcf) -> u8 {
        Pcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plt {
    #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)."]
    Pt4 = 0x0,
    #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)."]
    Pt28 = 0x01,
    #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)."]
    Pt36 = 0x02,
    #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)."]
    Pt144 = 0x03,
    #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)."]
    Pt256 = 0x04,
    #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)."]
    Pt512 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Plt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plt {
    #[inline(always)]
    fn from(val: u8) -> Plt {
        Plt::from_bits(val)
    }
}
impl From<Plt> for u8 {
    #[inline(always)]
    fn from(val: Plt) -> u8 {
        Plt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ppsoutnum {
    #[doc = "No PPS output."]
    NoPpso = 0x0,
    #[doc = "1 PPS output."]
    M1Ppso = 0x01,
    #[doc = "2 PPS output."]
    M2Ppso = 0x02,
    #[doc = "3 PPS output."]
    M3Ppso = 0x03,
    #[doc = "4 PPS output."]
    M4Ppso = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ppsoutnum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ppsoutnum {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ppsoutnum {
    #[inline(always)]
    fn from(val: u8) -> Ppsoutnum {
        Ppsoutnum::from_bits(val)
    }
}
impl From<Ppsoutnum> for u8 {
    #[inline(always)]
    fn from(val: Ppsoutnum) -> u8 {
        Ppsoutnum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prelen {
    #[doc = "7 bytes of preamble."]
    M7bytes = 0x0,
    #[doc = "5 bytes of preamble."]
    M5bytes = 0x01,
    #[doc = "3 bytes of preamble."]
    M3bytes = 0x02,
    _RESERVED_3 = 0x03,
}
impl Prelen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prelen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prelen {
    #[inline(always)]
    fn from(val: u8) -> Prelen {
        Prelen::from_bits(val)
    }
}
impl From<Prelen> for u8 {
    #[inline(always)]
    fn from(val: Prelen) -> u8 {
        Prelen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ps {
    #[doc = "For 1000 or 2500 Mbps operations."]
    M10002500m = 0x0,
    #[doc = "For 10 or 100 Mbps operations."]
    M10100m = 0x01,
}
impl Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ps {
    #[inline(always)]
    fn from(val: u8) -> Ps {
        Ps::from_bits(val)
    }
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(val: Ps) -> u8 {
        Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptpq {
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ptpq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptpq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptpq {
    #[inline(always)]
    fn from(val: u8) -> Ptpq {
        Ptpq::from_bits(val)
    }
}
impl From<Ptpq> for u8 {
    #[inline(always)]
    fn from(val: Ptpq) -> u8 {
        Ptpq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raa {
    #[doc = "Strict priority (SP)."]
    Sp = 0x0,
    #[doc = "Weighted Strict Priority (WSP)."]
    Wsp = 0x01,
}
impl Raa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raa {
    #[inline(always)]
    fn from(val: u8) -> Raa {
        Raa::from_bits(val)
    }
}
impl From<Raa> for u8 {
    #[inline(always)]
    fn from(val: Raa) -> u8 {
        Raa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdwr {
    #[doc = "Read operation of indirect access."]
    Read = 0x0,
    #[doc = "Write operation of indirect access."]
    Write = 0x01,
}
impl Rdwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdwr {
    #[inline(always)]
    fn from(val: u8) -> Rdwr {
        Rdwr::from_bits(val)
    }
}
impl From<Rdwr> for u8 {
    #[inline(always)]
    fn from(val: Rdwr) -> u8 {
        Rdwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps0 {
    #[doc = "Stopped (Reset or Stop Receive Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)."]
    RunFrtd = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)."]
    RunWrp = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)."]
    Suspnd = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)."]
    RunCrd = 0x05,
    #[doc = "Timestamp write state."]
    Tstmp = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)."]
    RunTrp = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps0 {
    #[inline(always)]
    fn from(val: u8) -> Rps0 {
        Rps0::from_bits(val)
    }
}
impl From<Rps0> for u8 {
    #[inline(always)]
    fn from(val: Rps0) -> u8 {
        Rps0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rps1 {
    #[doc = "Stopped (Reset or Stop Receive Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Rx Transfer Descriptor)."]
    RunFrtd = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Running (Waiting for Rx packet)."]
    RunWrp = 0x03,
    #[doc = "Suspended (Rx Descriptor Unavailable)."]
    Suspnd = 0x04,
    #[doc = "Running (Closing the Rx Descriptor)."]
    RunCrd = 0x05,
    #[doc = "Timestamp write state."]
    Tstmp = 0x06,
    #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)."]
    RunTrp = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rps1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rps1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rps1 {
    #[inline(always)]
    fn from(val: u8) -> Rps1 {
        Rps1::from_bits(val)
    }
}
impl From<Rps1> for u8 {
    #[inline(always)]
    fn from(val: Rps1) -> u8 {
        Rps1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxchcnt {
    #[doc = "1 MTL Rx Channel."]
    M1rxch = 0x0,
    #[doc = "2 MTL Rx Channels."]
    M2rxch = 0x01,
    #[doc = "3 MTL Rx Channels."]
    M3rxch = 0x02,
    #[doc = "4 MTL Rx Channels."]
    M4rxch = 0x03,
    #[doc = "5 MTL Rx Channels."]
    M5rxch = 0x04,
    #[doc = "6 MTL Rx Channels."]
    M6rxch = 0x05,
    #[doc = "7 MTL Rx Channels."]
    M7rxch = 0x06,
    #[doc = "8 MTL Rx Channels."]
    M8rxch = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxchcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxchcnt {
        Rxchcnt::from_bits(val)
    }
}
impl From<Rxchcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxchcnt) -> u8 {
        Rxchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "128 bytes."]
    M128b = 0x0,
    #[doc = "256 bytes."]
    M256b = 0x01,
    #[doc = "512 bytes."]
    M512b = 0x02,
    #[doc = "1024 bytes."]
    M1024b = 0x03,
    #[doc = "2048 bytes."]
    M2048b = 0x04,
    #[doc = "4096 bytes."]
    M4096b = 0x05,
    #[doc = "8192 bytes."]
    M8192b = 0x06,
    #[doc = "16384 bytes."]
    M16384b = 0x07,
    #[doc = "32 KB."]
    M32kb = 0x08,
    #[doc = "64 KB."]
    M64kb = 0x09,
    #[doc = "128 KB."]
    M128kb = 0x0a,
    #[doc = "256 KB."]
    M256kb = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxq0en {
    #[doc = "Queue not enabled."]
    Disable = 0x0,
    #[doc = "Queue enabled for AV."]
    EnAv = 0x01,
    #[doc = "Queue enabled for DCB/Generic."]
    EnDcbGen = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rxq0en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxq0en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxq0en {
    #[inline(always)]
    fn from(val: u8) -> Rxq0en {
        Rxq0en::from_bits(val)
    }
}
impl From<Rxq0en> for u8 {
    #[inline(always)]
    fn from(val: Rxq0en) -> u8 {
        Rxq0en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxq1en {
    #[doc = "Queue not enabled."]
    Disable = 0x0,
    #[doc = "Queue enabled for AV."]
    EnAv = 0x01,
    #[doc = "Queue enabled for DCB/Generic."]
    EnDcbGen = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rxq1en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxq1en {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxq1en {
    #[inline(always)]
    fn from(val: u8) -> Rxq1en {
        Rxq1en::from_bits(val)
    }
}
impl From<Rxq1en> for u8 {
    #[inline(always)]
    fn from(val: Rxq1en) -> u8 {
        Rxq1en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxqcnt {
    #[doc = "1 MTL Rx Queue."]
    M1rxq = 0x0,
    #[doc = "2 MTL Rx Queues."]
    M2rxq = 0x01,
    #[doc = "3 MTL Rx Queues."]
    M3rxq = 0x02,
    #[doc = "4 MTL Rx Queues."]
    M4rxq = 0x03,
    #[doc = "5 MTL Rx Queues."]
    M5rxq = 0x04,
    #[doc = "6 MTL Rx Queues."]
    M6rxq = 0x05,
    #[doc = "7 MTL Rx Queues."]
    M7rxq = 0x06,
    #[doc = "8 MTL Rx Queues."]
    M8rxq = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rxqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxqcnt {
    #[inline(always)]
    fn from(val: u8) -> Rxqcnt {
        Rxqcnt::from_bits(val)
    }
}
impl From<Rxqcnt> for u8 {
    #[inline(always)]
    fn from(val: Rxqcnt) -> u8 {
        Rxqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sarc {
    #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation."]
    SaCtrlIn = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Contents of MAC Addr-0 inserted in SA field."]
    Mac0InsSa = 0x02,
    #[doc = "Contents of MAC Addr-0 replaces SA field."]
    Mac0RepSa = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Contents of MAC Addr-1 inserted in SA field."]
    Mac1InsSa = 0x06,
    #[doc = "Contents of MAC Addr-1 replaces SA field."]
    Mac1RepSa = 0x07,
}
impl Sarc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sarc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sarc {
    #[inline(always)]
    fn from(val: u8) -> Sarc {
        Sarc::from_bits(val)
    }
}
impl From<Sarc> for u8 {
    #[inline(always)]
    fn from(val: Sarc) -> u8 {
        Sarc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Schalg {
    #[doc = "WRR algorithm."]
    Wrr = 0x0,
    #[doc = "WFQ algorithm when DCB feature is selected.Otherwise, Reserved."]
    Wfq = 0x01,
    #[doc = "DWRR algorithm when DCB feature is selected.Otherwise, Reserved."]
    Dwrr = 0x02,
    #[doc = "Strict priority algorithm."]
    Sp = 0x03,
}
impl Schalg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Schalg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Schalg {
    #[inline(always)]
    fn from(val: u8) -> Schalg {
        Schalg::from_bits(val)
    }
}
impl From<Schalg> for u8 {
    #[inline(always)]
    fn from(val: Schalg) -> u8 {
        Schalg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slc {
    #[doc = "1 slot."]
    M1Slot = 0x0,
    #[doc = "2 slots."]
    M2Slot = 0x01,
    #[doc = "4 slots."]
    M4Slot = 0x02,
    #[doc = "8 slots."]
    M8Slot = 0x03,
    #[doc = "16 slots."]
    M16Slot = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Slc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slc {
    #[inline(always)]
    fn from(val: u8) -> Slc {
        Slc::from_bits(val)
    }
}
impl From<Slc> for u8 {
    #[inline(always)]
    fn from(val: Slc) -> u8 {
        Slc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Taa {
    #[doc = "Fixed priority."]
    Fp = 0x0,
    #[doc = "Weighted Strict Priority (WSP)."]
    Wsp = 0x01,
    #[doc = "Weighted Round-Robin (WRR)."]
    Wrr = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Taa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Taa {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Taa {
    #[inline(always)]
    fn from(val: u8) -> Taa {
        Taa::from_bits(val)
    }
}
impl From<Taa> for u8 {
    #[inline(always)]
    fn from(val: Taa) -> u8 {
        Taa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfcsts {
    #[doc = "Idle state."]
    Idle = 0x0,
    #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over."]
    Waiting = 0x01,
    #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)."]
    GenTxPau = 0x02,
    #[doc = "Transferring input packet for transmission."]
    Trnsfr = 0x03,
}
impl Tfcsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfcsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfcsts {
    #[inline(always)]
    fn from(val: u8) -> Tfcsts {
        Tfcsts::from_bits(val)
    }
}
impl From<Tfcsts> for u8 {
    #[inline(always)]
    fn from(val: Tfcsts) -> u8 {
        Tfcsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps0 {
    #[doc = "Stopped (Reset or Stop Transmit Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)."]
    RunFttd = 0x01,
    #[doc = "Running (Waiting for status)."]
    RunWs = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))."]
    RunRds = 0x03,
    #[doc = "Timestamp write state."]
    TstmpWs = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)."]
    Suspnd = 0x06,
    #[doc = "Running (Closing Tx Descriptor)."]
    RunCtd = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tps0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps0 {
    #[inline(always)]
    fn from(val: u8) -> Tps0 {
        Tps0::from_bits(val)
    }
}
impl From<Tps0> for u8 {
    #[inline(always)]
    fn from(val: Tps0) -> u8 {
        Tps0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tps1 {
    #[doc = "Stopped (Reset or Stop Transmit Command issued)."]
    Stop = 0x0,
    #[doc = "Running (Fetching Tx Transfer Descriptor)."]
    RunFttd = 0x01,
    #[doc = "Running (Waiting for status)."]
    RunWs = 0x02,
    #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))."]
    RunRds = 0x03,
    #[doc = "Timestamp write state."]
    TstmpWs = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)."]
    Suspnd = 0x06,
    #[doc = "Running (Closing Tx Descriptor)."]
    RunCtd = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tps1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tps1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tps1 {
    #[inline(always)]
    fn from(val: u8) -> Tps1 {
        Tps1::from_bits(val)
    }
}
impl From<Tps1> for u8 {
    #[inline(always)]
    fn from(val: Tps1) -> u8 {
        Tps1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tscfupdt {
    #[doc = "Coarse method is used to update system timestamp."]
    Coarse = 0x0,
    #[doc = "Fine method is used to update system timestamp."]
    Fine = 0x01,
}
impl Tscfupdt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscfupdt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscfupdt {
    #[inline(always)]
    fn from(val: u8) -> Tscfupdt {
        Tscfupdt::from_bits(val)
    }
}
impl From<Tscfupdt> for u8 {
    #[inline(always)]
    fn from(val: Tscfupdt) -> u8 {
        Tscfupdt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsctrlssr {
    #[doc = "Timestamp Digital Rollover Control is disabled and Binary Rollover Control is enabled."]
    DigDisable = 0x0,
    #[doc = "Timestamp Digital Rollover Control is enabled and Binary Rollover Control is disabled."]
    DigEnable = 0x01,
}
impl Tsctrlssr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsctrlssr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsctrlssr {
    #[inline(always)]
    fn from(val: u8) -> Tsctrlssr {
        Tsctrlssr::from_bits(val)
    }
}
impl From<Tsctrlssr> for u8 {
    #[inline(always)]
    fn from(val: Tsctrlssr) -> u8 {
        Tsctrlssr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsstssel {
    #[doc = "Internal."]
    Intrnl = 0x0,
    #[doc = "External."]
    Extrnl = 0x01,
    #[doc = "Both."]
    Both = 0x02,
    _RESERVED_3 = 0x03,
}
impl Tsstssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsstssel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsstssel {
    #[inline(always)]
    fn from(val: u8) -> Tsstssel {
        Tsstssel::from_bits(val)
    }
}
impl From<Tsstssel> for u8 {
    #[inline(always)]
    fn from(val: Tsstssel) -> u8 {
        Tsstssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txchcnt {
    #[doc = "1 MTL Tx Channel."]
    M1txch = 0x0,
    #[doc = "2 MTL Tx Channels."]
    M2txch = 0x01,
    #[doc = "3 MTL Tx Channels."]
    M3txch = 0x02,
    #[doc = "4 MTL Tx Channels."]
    M4txch = 0x03,
    #[doc = "5 MTL Tx Channels."]
    M5txch = 0x04,
    #[doc = "6 MTL Tx Channels."]
    M6txch = 0x05,
    #[doc = "7 MTL Tx Channels."]
    M7txch = 0x06,
    #[doc = "8 MTL Tx Channels."]
    M8txch = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txchcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txchcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txchcnt {
    #[inline(always)]
    fn from(val: u8) -> Txchcnt {
        Txchcnt::from_bits(val)
    }
}
impl From<Txchcnt> for u8 {
    #[inline(always)]
    fn from(val: Txchcnt) -> u8 {
        Txchcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "128 bytes."]
    M128b = 0x0,
    #[doc = "256 bytes."]
    M256b = 0x01,
    #[doc = "512 bytes."]
    M512b = 0x02,
    #[doc = "1024 bytes."]
    M1024b = 0x03,
    #[doc = "2048 bytes."]
    M2048b = 0x04,
    #[doc = "4096 bytes."]
    M4096b = 0x05,
    #[doc = "8192 bytes."]
    M8192b = 0x06,
    #[doc = "16384 bytes."]
    M16384b = 0x07,
    #[doc = "32 KB."]
    M32kb = 0x08,
    #[doc = "64 KB."]
    M64kb = 0x09,
    #[doc = "128 KB."]
    M128kb = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txqcnt {
    #[doc = "1 MTL Tx Queue."]
    M1txq = 0x0,
    #[doc = "2 MTL Tx Queues."]
    M2txq = 0x01,
    #[doc = "3 MTL Tx Queues."]
    M3txq = 0x02,
    #[doc = "4 MTL Tx Queues."]
    M4txq = 0x03,
    #[doc = "5 MTL Tx Queues."]
    M5txq = 0x04,
    #[doc = "6 MTL Tx Queues."]
    M6txq = 0x05,
    #[doc = "7 MTL Tx Queues."]
    M7txq = 0x06,
    #[doc = "8 MTL Tx Queues."]
    M8txq = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Txqcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txqcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txqcnt {
    #[inline(always)]
    fn from(val: u8) -> Txqcnt {
        Txqcnt::from_bits(val)
    }
}
impl From<Txqcnt> for u8 {
    #[inline(always)]
    fn from(val: Txqcnt) -> u8 {
        Txqcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Upq {
    #[doc = "Receive Queue 0."]
    Queue0 = 0x0,
    #[doc = "Receive Queue 1."]
    Queue1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Upq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Upq {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Upq {
    #[inline(always)]
    fn from(val: u8) -> Upq {
        Upq::from_bits(val)
    }
}
impl From<Upq> for u8 {
    #[inline(always)]
    fn from(val: Upq) -> u8 {
        Upq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wd {
    #[doc = "Watchdog is enabled."]
    Enable = 0x0,
    #[doc = "Watchdog is disabled."]
    Disable = 0x01,
}
impl Wd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wd {
    #[inline(always)]
    fn from(val: u8) -> Wd {
        Wd::from_bits(val)
    }
}
impl From<Wd> for u8 {
    #[inline(always)]
    fn from(val: Wd) -> u8 {
        Wd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wto {
    #[doc = "2 KB."]
    Bf2kbytes = 0x0,
    #[doc = "3 KB."]
    Bf3kbytes = 0x01,
    #[doc = "4 KB."]
    Bf4kbytes = 0x02,
    #[doc = "5 KB."]
    Bf5kbytes = 0x03,
    #[doc = "6 KB."]
    Bf6kbytes = 0x04,
    #[doc = "7 KB."]
    Bf7kbytes = 0x05,
    #[doc = "8 KB."]
    Bf8kbytes = 0x06,
    #[doc = "9 KB."]
    Bf9kbytes = 0x07,
    #[doc = "10 KB."]
    Bf10kbytes = 0x08,
    #[doc = "11 KB."]
    Bf11kbytes = 0x09,
    #[doc = "12 KB."]
    Bf12kbytes = 0x0a,
    #[doc = "13 KB."]
    Bf13kbytes = 0x0b,
    #[doc = "14 KB."]
    Bf14kbytes = 0x0c,
    #[doc = "15 KB."]
    Bf15kbytes = 0x0d,
    #[doc = "16383 Bytes."]
    Bf16383bytes = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Wto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wto {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wto {
    #[inline(always)]
    fn from(val: u8) -> Wto {
        Wto::from_bits(val)
    }
}
impl From<Wto> for u8 {
    #[inline(always)]
    fn from(val: Wto) -> u8 {
        Wto::to_bits(val)
    }
}
