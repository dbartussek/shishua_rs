use crate::{
    core::{GROUP_SIZE, STATE_SIZE},
    ShiShuAState,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rand_core::{Error, RngCore};
use std::io::Cursor;

const STATE_WRAPPER_BUFFER_SIZE: usize =
    GROUP_SIZE * STATE_SIZE * std::mem::size_of::<u64>();

pub struct ShiShuARng {
    state: ShiShuAState,
    buffer: [u8; STATE_WRAPPER_BUFFER_SIZE],
    buffer_index: usize,
}

impl ShiShuARng {
    pub fn new(seed: [u64; GROUP_SIZE]) -> Self {
        ShiShuARng {
            state: ShiShuAState::new(seed),
            buffer: [0; STATE_WRAPPER_BUFFER_SIZE],
            buffer_index: STATE_WRAPPER_BUFFER_SIZE,
        }
    }

    fn get_byte(&mut self) -> u8 {
        if self.buffer_index >= STATE_WRAPPER_BUFFER_SIZE {
            self.buffer_index = 0;

            let data = self.state.round_unpack();

            let buffer = &mut self.buffer.as_mut();
            for v in &data {
                buffer.write_u64::<LittleEndian>(*v).unwrap();
            }
        }

        let index = self.buffer_index;
        self.buffer_index += 1;

        self.buffer[index]
    }
}

impl RngCore for ShiShuARng {
    fn next_u32(&mut self) -> u32 {
        let mut buffer = [0u8; std::mem::size_of::<u32>()];
        self.fill_bytes(&mut buffer);
        Cursor::new(buffer).read_u32::<LittleEndian>().unwrap()
    }

    fn next_u64(&mut self) -> u64 {
        let mut buffer = [0u8; std::mem::size_of::<u64>()];
        self.fill_bytes(&mut buffer);
        Cursor::new(buffer).read_u64::<LittleEndian>().unwrap()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.get_byte();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
