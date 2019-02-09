use crate::mgmt::interface::command::{ManagementCommand, ManagementCommandStatus};

pub mod interface;
pub mod socket;

#[derive(Debug, Clone, Fail)]
pub enum ManagementError {
    #[fail(display = "The socket has not been opened yet.")]
    SocketNotOpen,
    #[fail(display = "Unknown error.")]
    Unknown,
    #[fail(display = "Command {:?} returned {:?}.", status, opcode)]
    CommandError {
        opcode: ManagementCommand,
        status: ManagementCommandStatus,
    },
    #[fail(display = "Unknown opcode: {:x}.", cmd)]
    UnknownCommand { cmd: u16 },
    #[fail(display = "Unknown command status: {:x}.", status)]
    UnknownStatus { status: u8 },
    #[fail(
    display = "The name {} is too long; the maximum length is {} bytes.",
    name, maxlen
    )]
    NameTooLong { name: String, maxlen: u32 },
}
