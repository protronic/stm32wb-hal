use crate::tl_mbox::{SafeBootInfoTable, RssInfoTable, WirelessFwInfoTable, TL_REF_TABLE, DeviceInfoTable, TL_EVT_HEADER_SIZE};
use crate::tl_mbox::cmd::CmdPacket;
use crate::tl_mbox::evt::{EvtSerial, CcEvt, EvtPacket};
use crate::tl_mbox::consts::TlPacketType;

const TL_BLEEVT_CC_OPCODE: u8 = 0x0e;
const TL_BLEEVT_CS_OPCODE: u8 = 0x0f;

const LHCI_OPCODE_C1_DEVICE_INF: u16 = 0xfd62;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct LhciC1DeviceInformationCcrp {
    pub status: u8,
    pub rev_id: u16,
    pub dev_code_id: u16,
    pub package_type: u8,
    pub device_type_id: u8,
    pub st_company_id: u32,
    pub uid64: u32,

    pub uid96_0: u32,
    pub uid96_1: u32,
    pub uid96_2: u32,

    pub safe_boot_info_table: SafeBootInfoTable,
    pub rss_info_table: RssInfoTable,
    pub wireless_fw_info_table: WirelessFwInfoTable,

    pub app_fw_inf: u32,
}

impl LhciC1DeviceInformationCcrp {
    pub fn new() -> Self {
        let DeviceInfoTable { safe_boot_info_table,
            rss_info_table,
            wireless_fw_info_table
        } = unsafe { &*(&*TL_REF_TABLE.as_ptr()).device_info_table }.clone();

        let dbgmcu = unsafe { stm32wb_pac::Peripherals::steal() }.DBGMCU;
        let rev_id = dbgmcu.idcode.read().rev_id().bits();
        let dev_code_id = dbgmcu.idcode.read().dev_id().bits();

        // TODO: fill the rest of the fields

        LhciC1DeviceInformationCcrp {
            status: 0,
            rev_id,
            dev_code_id,
            package_type: 0,
            device_type_id: 0,
            st_company_id: 0,
            uid64: 0,
            uid96_0: 0,
            uid96_1: 0,
            uid96_2: 0,
            safe_boot_info_table,
            rss_info_table,
            wireless_fw_info_table,
            app_fw_inf: 0
        }
    }

    pub fn write(&self, cmd_packet: &mut CmdPacket) {
        let self_size = core::mem::size_of::<LhciC1DeviceInformationCcrp>();

        unsafe {
            let cmd_packet_ptr: *mut CmdPacket = cmd_packet;
            let evt_packet_ptr: *mut EvtPacket = cmd_packet_ptr.cast();

            let evt_serial: *mut EvtSerial = &mut (*evt_packet_ptr).evt_serial;
            let evt_payload: *mut u8 = (&mut *evt_serial).evt.payload.as_mut_ptr();
            let evt_cc: *mut CcEvt = evt_payload.cast();
            let evt_cc_payload_buf: *mut u8 = (&mut *evt_cc).payload.as_mut_ptr();

            (*evt_serial).kind = TlPacketType::LocRsp as u8;
            (*evt_serial).evt.evt_code = TL_BLEEVT_CC_OPCODE;
            (*evt_serial).evt.payload_len = TL_EVT_HEADER_SIZE as u8 + self_size as u8;

            (*evt_cc).cmd_code = LHCI_OPCODE_C1_DEVICE_INF;
            (*evt_cc).num_cmd = 1;

            let self_ptr: *const LhciC1DeviceInformationCcrp = self;
            let self_buf: *const u8 = self_ptr.cast();

            core::ptr::copy(self_buf, evt_cc_payload_buf, self_size);
        }
    }
}