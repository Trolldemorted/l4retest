use crate::L4_IPC_NEVER;
use crate::L4_MSGTAG_SCHEDULE;
use crate::L4_PROTO_LOG;
use crate::L4_VCON_WRITE_OP;
use crate::L4ReEnvPtr;
use crate::UtcbPtr;
use crate::l4_ipc_send;
use crate::l4_msgtag;
use log::Level;

struct L4Logger {}

impl log::Log for L4Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        let foo = record.args();
        todo!()
    }

    fn flush(&self) {}
}

pub fn print(dat: &str) {
    let bytes = dat.as_bytes();
    unsafe {
        let utcb = UtcbPtr::new();
        let env = L4ReEnvPtr::new();
        let mut mr = utcb.get_message_registers();
        mr.write(0, L4_VCON_WRITE_OP);
        mr.write(1, 8);
        mr.memcopy(2, bytes);
        l4_ipc_send(
            env.get_log().0,
            utcb.address,
            l4_msgtag(L4_PROTO_LOG, 2 + 1, 0, L4_MSGTAG_SCHEDULE),
            L4_IPC_NEVER,
        );
    }
}
