use rand_core::{RngCore, SeedableRng};
use shishua::ShiShuARng;
use std::process::Command;

fn generate_c(seed: u64, length: usize) -> Vec<u8> {
    Command::new("./shishua")
        .arg("--seed")
        .arg(format!("{:x}", seed))
        .arg("--bytes")
        .arg(format!("{}", length))
        .output()
        .unwrap()
        .stdout
}

fn generate_rust(seed: u64, length: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; length];

    ShiShuARng::seed_from_u64(seed).fill_bytes(&mut buffer);

    buffer
}

fn compare(seed: u64, length: usize) {
    dbg!(length);

    let c_value = generate_c(seed, length);
    let rust_value = generate_rust(seed, length);

    dbg!(c_value.len());
    dbg!(rust_value.len());

    assert!(c_value == rust_value, "Seed: {:#X}", seed);
}

#[test]
#[cfg_attr(miri, ignore)]
fn native_works() {
    generate_c(0x123, 8);
}

#[test]
#[cfg_attr(miri, ignore)]
fn native_compare_zero() {
    compare(0, 4 * 4 * 8);
}

#[test]
#[cfg_attr(miri, ignore)]
fn native_compare_1234() {
    compare(0x1234_5678_9ABC_DEF0, 4 * 4 * 8);
}

#[test]
#[cfg_attr(miri, ignore)]
fn native_compare_long() {
    compare(0x1234_5678_9ABC_DEF0, 4 * 4 * 8 * 100);
}

const COMPARE_ZERO: [u8; 128] = [
    149, 93, 150, 249, 15, 180, 170, 83, 9, 45, 130, 230, 58, 124, 9, 226, 44,
    165, 164, 165, 167, 90, 90, 57, 220, 104, 180, 18, 93, 231, 206, 43, 107,
    110, 254, 245, 139, 217, 204, 66, 18, 221, 116, 78, 129, 253, 24, 185, 88,
    240, 98, 93, 56, 239, 204, 27, 111, 219, 13, 163, 54, 247, 229, 238, 107,
    219, 232, 234, 92, 218, 64, 199, 83, 68, 208, 213, 191, 193, 213, 7, 224,
    44, 245, 18, 8, 113, 27, 234, 136, 130, 207, 214, 204, 247, 29, 6, 98, 199,
    94, 241, 152, 93, 242, 198, 213, 109, 61, 46, 53, 218, 214, 133, 58, 193,
    118, 183, 77, 183, 224, 38, 81, 45, 206, 52, 139, 166, 3, 241,
];

#[test]
fn hard_coded_zero() {
    assert_eq!(
        &COMPARE_ZERO as &[u8],
        generate_rust(0, 4 * 4 * 8).as_slice()
    );
}

#[test]
#[cfg_attr(miri, ignore)]
fn hard_coded_zero_c() {
    assert_eq!(&COMPARE_ZERO as &[u8], generate_c(0, 4 * 4 * 8).as_slice());
}
