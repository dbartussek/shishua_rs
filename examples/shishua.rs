use std::{
    io::{self, Write},
    process,
};

use rand::{RngCore, SeedableRng};
use shishua::ShiShuARng;

fn main() {
    let mut rng = ShiShuARng::from_os_rng();
    let mut buf = vec![0; 1 << 17];
    let mut stdout = io::stdout().lock();
    loop {
        rng.fill_bytes(&mut buf);
        let Ok(_) = stdout.write_all(&buf) else {
            process::exit(1);
        };
    }
}
