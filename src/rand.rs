use crate::{
    core::{STATE_LANES, STATE_SIZE},
    ShiShuAState,
};
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

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        if self.buffer_index >= STATE_WRAPPER_BUFFER_SIZE {
            self.buffer_index = 0;

            let data = self.state.round_unpack();

            let buffer = &mut self.buffer.as_mut();
            for (index, value) in data.iter().enumerate() {
                buffer[(index * size_of::<u64>())
                    ..((index + 1) * size_of::<u64>())]
                    .copy_from_slice(&value.to_le_bytes());
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
        u32::from_le_bytes(buffer)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buffer = [0u8; size_of::<u64>()];
        self.fill_bytes(&mut buffer);
        u64::from_le_bytes(buffer)
    }

    fn fill_bytes(&mut self, mut dest: &mut [u8]) {
        while self.buffer_index < STATE_WRAPPER_BUFFER_SIZE && dest.len() > 0 {
            dest[0] = self.buffer[self.buffer_index];
            self.buffer_index += 1;
            dest = &mut dest[1..];
        }

        while dest.len() >= STATE_WRAPPER_BUFFER_SIZE {
            let data = self.state.round_unpack();

            for (index, value) in data.iter().enumerate() {
                dest[(index * size_of::<u64>())
                    ..((index + 1) * size_of::<u64>())]
                    .copy_from_slice(&value.to_le_bytes());
            }

            dest = &mut dest[STATE_WRAPPER_BUFFER_SIZE..];
        }

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
