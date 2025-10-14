#[cfg(all(not(feature = "nightly"), not(feature = "wide")))]
use crate::software_simd::*;
#[cfg(feature = "nightly")]
use core::simd::{simd_swizzle, u32x8, u64x4};
#[cfg(all(not(feature = "nightly"), feature = "wide"))]
use {
    crate::wide_support::*,
    wide::{u32x8, u64x4},
};

#[cfg(not(feature = "nightly"))]
macro_rules! simd_swizzle {
    ($simd:expr, $shuffle:expr) => {
        $shuffle.map(|i| $simd.to_array()[i])
    };
}


pub const STATE_LANES: usize = 4;
pub const STATE_SIZE: usize = 4;


const PHI: [u64; 16] = [
    0x9E3779B97F4A7C15,
    0xF39CC0605CEDC834,
    0x1082276BF3A27251,
    0xF86C6A11D0C18E95,
    0x2767F0B153D27B7F,
    0x0347045B5BF1827F,
    0x01886F0928403002,
    0xC1D64BA40F335E36,
    0xF06AD7AE9717877E,
    0x85839D6EFFBD7DC6,
    0x64D325D1C5371682,
    0xCADD0CCCFDFFBBE1,
    0x626E33B8D04B4331,
    0xBBF73C790D94F79D,
    0x471C4AB3ED3D82A5,
    0xFEC507705E4AE6E5,
];

/// The raw ShiShuA implementation. Random values are generated in `[u64; 16]` chunks with [round_unpack](ShiShuAState::round_unpack).
#[derive(Copy, Clone)]
pub struct ShiShuAState {
    state: [u64x4; STATE_SIZE],
    output: [u64x4; STATE_SIZE],
    counter: u64x4,
}

impl ShiShuAState {
    pub fn new(seed: [u64; STATE_LANES]) -> Self {
        const STEPS: usize = 13;
        const ROUNDS: usize = 1;

        let mut buffer = [0u64; STATE_LANES * STATE_SIZE * ROUNDS];

        let mut state = ShiShuAState {
            state: [
                u64x4::from([
                    PHI[3],
                    PHI[2] ^ seed[1],
                    PHI[1],
                    PHI[0] ^ seed[0],
                ]),
                u64x4::from([
                    PHI[7],
                    PHI[6] ^ seed[3],
                    PHI[5],
                    PHI[4] ^ seed[2],
                ]),
                u64x4::from([
                    PHI[11],
                    PHI[10] ^ seed[3],
                    PHI[9],
                    PHI[8] ^ seed[2],
                ]),
                u64x4::from([
                    PHI[15],
                    PHI[14] ^ seed[1],
                    PHI[13],
                    PHI[12] ^ seed[0],
                ]),
            ],
            output: [u64x4::splat(0); 4],
            counter: u64x4::splat(0),
        };

        for _ in 0..STEPS {
            state.generate(&mut buffer);
            state.state[0] = state.output[3];
            state.state[1] = state.output[2];
            state.state[2] = state.output[1];
            state.state[3] = state.output[0];
        }

        state
    }

    fn generate(&mut self, output_slice: &mut [u64]) {
        assert_eq!(output_slice.len() % (STATE_LANES * STATE_SIZE), 0);

        for output_chunk in
            output_slice.chunks_exact_mut(STATE_LANES * STATE_SIZE)
        {
            let output = self.round_unpack();
            output_chunk.copy_from_slice(&output);
        }
    }

    pub fn round_unpack(&mut self) -> [u64; STATE_SIZE * STATE_LANES] {
        let raw = self.round();
        let mut output = [0u64; STATE_SIZE * STATE_LANES];

        for (group, value) in raw.iter().enumerate() {
            let group_slice_index = group * STATE_LANES;
            for i in 0..STATE_LANES {
                output[group_slice_index + i] =
                    value.to_array()[STATE_LANES - 1 - i];
            }
        }

        output
    }

    #[inline(always)]
    fn round(&mut self) -> [u64x4; STATE_SIZE] {
        const fn correct_index(index: usize) -> usize {
            (u32x8::LEN - 1 - index) ^ 1
        }

        // Shuffle values work differently in Rust than in the C source.
        //
        // High and low 32 bits are flipped.
        // Indexing is the other way around
        //
        // I spent quite some time figuring this out.
        const SHUFFLE: [[usize; 8]; 2] = [
            // [4, 3, 2, 1, 0, 7, 6, 5],
            [
                correct_index(3),
                correct_index(4),
                correct_index(1),
                correct_index(2),
                correct_index(7),
                correct_index(0),
                correct_index(5),
                correct_index(6),
            ],
            // [2, 1, 0, 7, 6, 5, 4, 3],
            [
                correct_index(1),
                correct_index(2),
                correct_index(7),
                correct_index(0),
                correct_index(5),
                correct_index(6),
                correct_index(3),
                correct_index(4),
            ],
        ];

        let increment = u64x4::from([1, 3, 5, 7]);

        let ShiShuAState {
            state,
            output,
            counter,
        } = self;

        // Perform the round
        state[1] += *counter;
        state[3] += *counter;
        *counter += increment;

        let u0 = state[0] >> 1;
        let u1 = state[1] >> 3;
        let u2 = state[2] >> 1;
        let u3 = state[3] >> 3;

        macro_rules! shuffle_u64_as_u32 {
            ($data:expr, $shuffle:expr) => {{
                let as_u32: u32x8 = bytemuck::cast($data);
                let shuffled = simd_swizzle!(as_u32, $shuffle);
                bytemuck::cast(shuffled)
            }};
        }


        let t0: u64x4 = shuffle_u64_as_u32!(state[0], SHUFFLE[0]);
        let t1: u64x4 = shuffle_u64_as_u32!(state[1], SHUFFLE[1]);
        let t2: u64x4 = shuffle_u64_as_u32!(state[2], SHUFFLE[0]);
        let t3: u64x4 = shuffle_u64_as_u32!(state[3], SHUFFLE[1]);

        state[0] = t0 + u0;
        state[1] = t1 + u1;
        state[2] = t2 + u2;
        state[3] = t3 + u3;

        let result = *output;

        output[0] = u0 ^ t1;
        output[1] = u2 ^ t3;
        output[2] = state[0] ^ state[3];
        output[3] = state[2] ^ state[1];

        result
    }
}
