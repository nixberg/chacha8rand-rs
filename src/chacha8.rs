use core::simd::{u32x4, u64x2, ToBytes};

pub(crate) fn block(buffer: &mut [u64; 32], seed: &[u32; 8], counter: u32) {
    let mut state = State::new(seed, counter);
    state.permute();
    state.fill_buffer(buffer);
}

struct State {
    v00: u32x4,
    v01: u32x4,
    v02: u32x4,
    v03: u32x4,
    v04: u32x4,
    v05: u32x4,
    v06: u32x4,
    v07: u32x4,
    v08: u32x4,
    v09: u32x4,
    v10: u32x4,
    v11: u32x4,
    v12: u32x4,
    v13: u32x4,
    v14: u32x4,
    v15: u32x4,

    s0: u32x4,
    s1: u32x4,
    s2: u32x4,
    s3: u32x4,
    s4: u32x4,
    s5: u32x4,
    s6: u32x4,
    s7: u32x4,
}

impl State {
    #[inline(always)]
    fn new(seed: &[u32; 8], counter: u32) -> Self {
        let s0 = u32x4::splat(seed[0]);
        let s1 = u32x4::splat(seed[1]);
        let s2 = u32x4::splat(seed[2]);
        let s3 = u32x4::splat(seed[3]);
        let s4 = u32x4::splat(seed[4]);
        let s5 = u32x4::splat(seed[5]);
        let s6 = u32x4::splat(seed[6]);
        let s7 = u32x4::splat(seed[7]);

        Self {
            v00: u32x4::splat(0x61707865),
            v01: u32x4::splat(0x3320646e),
            v02: u32x4::splat(0x79622d32),
            v03: u32x4::splat(0x6b206574),
            v04: s0,
            v05: s1,
            v06: s2,
            v07: s3,
            v08: s4,
            v09: s5,
            v10: s6,
            v11: s7,
            v12: u32x4::splat(counter) + u32x4::from_array([0, 1, 2, 3]),
            v13: u32x4::splat(0),
            v14: u32x4::splat(0),
            v15: u32x4::splat(0),

            s0: s0,
            s1: s1,
            s2: s2,
            s3: s3,
            s4: s4,
            s5: s5,
            s6: s6,
            s7: s7,
        }
    }

    #[inline(always)]
    fn permute(&mut self) {
        for _ in 0..4 {
            quarter_round(&mut self.v00, &mut self.v04, &mut self.v08, &mut self.v12);
            quarter_round(&mut self.v01, &mut self.v05, &mut self.v09, &mut self.v13);
            quarter_round(&mut self.v02, &mut self.v06, &mut self.v10, &mut self.v14);
            quarter_round(&mut self.v03, &mut self.v07, &mut self.v11, &mut self.v15);

            quarter_round(&mut self.v00, &mut self.v05, &mut self.v10, &mut self.v15);
            quarter_round(&mut self.v01, &mut self.v06, &mut self.v11, &mut self.v12);
            quarter_round(&mut self.v02, &mut self.v07, &mut self.v08, &mut self.v13);
            quarter_round(&mut self.v03, &mut self.v04, &mut self.v09, &mut self.v14);
        }

        self.v04 += self.s0;
        self.v05 += self.s1;
        self.v06 += self.s2;
        self.v07 += self.s3;
        self.v08 += self.s4;
        self.v09 += self.s5;
        self.v10 += self.s6;
        self.v11 += self.s7;
    }

    fn fill_buffer(self, buffer: &mut [u64; 32]) {
        #[inline(always)]
        fn store(target: &mut [u64], source: u32x4) {
            target.copy_from_slice(&u64x2::from_le_bytes(source.to_le_bytes()).to_array());
        }
        store(&mut buffer[00..02], self.v00);
        store(&mut buffer[02..04], self.v01);
        store(&mut buffer[04..06], self.v02);
        store(&mut buffer[06..08], self.v03);
        store(&mut buffer[08..10], self.v04);
        store(&mut buffer[10..12], self.v05);
        store(&mut buffer[12..14], self.v06);
        store(&mut buffer[14..16], self.v07);
        store(&mut buffer[16..18], self.v08);
        store(&mut buffer[18..20], self.v09);
        store(&mut buffer[20..22], self.v10);
        store(&mut buffer[22..24], self.v11);
        store(&mut buffer[24..26], self.v12);
        store(&mut buffer[26..28], self.v13);
        store(&mut buffer[28..30], self.v14);
        store(&mut buffer[30..32], self.v15);
    }
}

#[inline(always)]
fn quarter_round(a: &mut u32x4, b: &mut u32x4, c: &mut u32x4, d: &mut u32x4) {
    (*a, *b, *c, *d) = functional_quarter_round(*a, *b, *c, *d);
}

#[inline(always)]
fn functional_quarter_round(
    mut a: u32x4,
    mut b: u32x4,
    mut c: u32x4,
    mut d: u32x4,
) -> (u32x4, u32x4, u32x4, u32x4) {
    a += b;
    d ^= a;
    d = rotate_left::<16>(d);

    c += d;
    b ^= c;
    b = rotate_left::<12>(b);

    a += b;
    d ^= a;
    d = rotate_left::<08>(d);

    c += d;
    b ^= c;
    b = rotate_left::<07>(b);

    (a, b, c, d)
}

#[inline(always)]
fn rotate_left<const OFFSET: u32>(x: u32x4) -> u32x4 {
    x << OFFSET | x >> (u32::BITS - OFFSET)
}
