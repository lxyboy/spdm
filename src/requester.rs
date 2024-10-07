use core::borrow::Borrow;

use crate::protocol;
use crate::utils::Bytes;

const MAX_SPDM_MSG_BUFFER_SIZE: usize = 1024;

pub struct RequesterConfig {
    pub SPDMVersion: u8,
}

enum State {
    GET_VERSION,
    GET_VERSION_DONE,
    GET_CAPABILITIES,
    GET_CAPABILITIES_DONE,
    NEGOTIATE_ALGORITHMS,
    NEGOTIATE_ALGORITHMS_DONE,
}
pub struct Requester {
    state: State,
}

impl Requester {
    pub fn new() -> Self {
        Requester {
            state: State::GET_VERSION,
        }
    }

    /// Version Capabilities Algorithms
    /// return (need read bytes, need write bytes)
    pub fn vca(
        &mut self,
        read_buffer: &[u8],
        write_buffer: &mut [u8],
    ) -> Result<(usize, usize), ()> {
        match self.state {
            State::GET_VERSION => {
                let v = protocol::GET_VERSION::new();
                let nwrite = v.to_bytes(write_buffer).map_err(|_| ())?;
                self.state = State::GET_VERSION_DONE;
                Ok((0, nwrite))
            }
            State::GET_VERSION_DONE => todo!(),
            State::GET_CAPABILITIES => todo!(),
            State::GET_CAPABILITIES_DONE => todo!(),
            State::NEGOTIATE_ALGORITHMS => todo!(),
            State::NEGOTIATE_ALGORITHMS_DONE => todo!(),
        }
    }
}

pub fn get_version() {
    use protocol::GET_VERSION;
    use protocol::VERSION;

    let _get_version = GET_VERSION::new();

    let _version = VERSION {
        SPDMVersion: todo!(),
        RequestResponseCode: todo!(),
        Param1: todo!(),
        Param2: todo!(),
        Reserved: todo!(),
        VersionNumberEntryCount: todo!(),
        VersionNumberEntry1_n: todo!(),
    };
    // let buffer = [0, 1, 2, 3];
    // let get_version = GET_VERSION::new_checked(buffer).unwrap();
}
