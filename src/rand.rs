use crate::{
    core::{STATE_LANES, STATE_SIZE},
    ShiShuAState,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rand_core::{RngCore, SeedableRng};

const STATE_WRAPPER_BUFFER_SIZE: usize =
    STATE_LANES * STATE_SIZE * size_of::<u64>();

/// A rand compatible wrapper around the raw ShiShuAState.
///
/// An internal buffer is used to split up big chunks of randomness into the requested size.
pub struct ShiShuARng {
    state: ShiShuAState,
    buffer: [u8; STATE_WRAPPER_BUFFER_SIZE],
    buffer_index: usize,
}

impl ShiShuARng {
    pub fn new(seed: [u64; STATE_LANES]) -> Self {
        ShiShuARng {
            state: ShiShuAState::new(seed),
            buffer: [0; STATE_WRAPPER_BUFFER_SIZE],
            buffer_index: STATE_WRAPPER_BUFFER_SIZE,
        }
    }

    pub fn get_byte(&mut self) -> u8 {
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
        let mut buffer = [0u8; size_of::<u32>()];
        self.fill_bytes(&mut buffer);
        buffer.as_slice().read_u32::<LittleEndian>().unwrap()
    }

    fn next_u64(&mut self) -> u64 {
        let mut buffer = [0u8; size_of::<u64>()];
        self.fill_bytes(&mut buffer);
        buffer.as_slice().read_u64::<LittleEndian>().unwrap()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest.iter_mut() {
            *byte = self.get_byte();
        }
    }
}

impl SeedableRng for ShiShuARng {
    type Seed = [u8; STATE_LANES * size_of::<u64>()];

    fn from_seed(seed: Self::Seed) -> Self {
        Self::new(bytemuck::cast(seed))
    }
}
