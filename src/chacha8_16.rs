use core::simd::{u32x16, u64x8, ToBytes};

pub(crate) fn expand(seed: &[u32; 8], buffer: &mut [u64; 128]) {
    let mut state = [
        u32x16::splat(0x61707865),
        u32x16::splat(0x3320646e),
        u32x16::splat(0x79622d32),
        u32x16::splat(0x6b206574),
        u32x16::splat(seed[0]),
        u32x16::splat(seed[1]),
        u32x16::splat(seed[2]),
        u32x16::splat(seed[3]),
        u32x16::splat(seed[4]),
        u32x16::splat(seed[5]),
        u32x16::splat(seed[6]),
        u32x16::splat(seed[7]),
        u32x16::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        u32x16::splat(0),
        u32x16::splat(0),
        u32x16::splat(0),
    ];

    for _ in 0..4 {
        quarter_round::<00, 04, 08, 12>(&mut state);
        quarter_round::<01, 05, 09, 13>(&mut state);
        quarter_round::<02, 06, 10, 14>(&mut state);
        quarter_round::<03, 07, 11, 15>(&mut state);

        quarter_round::<00, 05, 10, 15>(&mut state);
        quarter_round::<01, 06, 11, 12>(&mut state);
        quarter_round::<02, 07, 08, 13>(&mut state);
        quarter_round::<03, 04, 09, 14>(&mut state);
    }

    state[04] += u32x16::splat(seed[0]);
    state[05] += u32x16::splat(seed[1]);
    state[06] += u32x16::splat(seed[2]);
    state[07] += u32x16::splat(seed[3]);
    state[08] += u32x16::splat(seed[4]);
    state[09] += u32x16::splat(seed[5]);
    state[10] += u32x16::splat(seed[6]);
    state[11] += u32x16::splat(seed[7]);

    buffer[000..008].copy_from_slice(&u64x8::from_le_bytes(state[00].to_le_bytes()).to_array());
    buffer[008..016].copy_from_slice(&u64x8::from_le_bytes(state[01].to_le_bytes()).to_array());
    buffer[016..024].copy_from_slice(&u64x8::from_le_bytes(state[02].to_le_bytes()).to_array());
    buffer[024..032].copy_from_slice(&u64x8::from_le_bytes(state[03].to_le_bytes()).to_array());
    buffer[032..040].copy_from_slice(&u64x8::from_le_bytes(state[04].to_le_bytes()).to_array());
    buffer[040..048].copy_from_slice(&u64x8::from_le_bytes(state[05].to_le_bytes()).to_array());
    buffer[048..056].copy_from_slice(&u64x8::from_le_bytes(state[06].to_le_bytes()).to_array());
    buffer[056..064].copy_from_slice(&u64x8::from_le_bytes(state[07].to_le_bytes()).to_array());
    buffer[064..072].copy_from_slice(&u64x8::from_le_bytes(state[08].to_le_bytes()).to_array());
    buffer[072..080].copy_from_slice(&u64x8::from_le_bytes(state[09].to_le_bytes()).to_array());
    buffer[080..088].copy_from_slice(&u64x8::from_le_bytes(state[10].to_le_bytes()).to_array());
    buffer[088..096].copy_from_slice(&u64x8::from_le_bytes(state[11].to_le_bytes()).to_array());
    buffer[096..104].copy_from_slice(&u64x8::from_le_bytes(state[12].to_le_bytes()).to_array());
    buffer[104..112].copy_from_slice(&u64x8::from_le_bytes(state[13].to_le_bytes()).to_array());
    buffer[112..120].copy_from_slice(&u64x8::from_le_bytes(state[14].to_le_bytes()).to_array());
    buffer[120..128].copy_from_slice(&u64x8::from_le_bytes(state[15].to_le_bytes()).to_array());
}

fn quarter_round<const A: usize, const B: usize, const C: usize, const D: usize>(
    state: &mut [u32x16; 16],
) {
    state[A] += state[B];
    state[D] ^= state[A];
    state[D] = rotate_left::<16>(state[D]);

    state[C] += state[D];
    state[B] ^= state[C];
    state[B] = rotate_left::<12>(state[B]);

    state[A] += state[B];
    state[D] ^= state[A];
    state[D] = rotate_left::<08>(state[D]);

    state[C] += state[D];
    state[B] ^= state[C];
    state[B] = rotate_left::<07>(state[B]);
}

fn rotate_left<const OFFSET: u32>(x: u32x16) -> u32x16 {
    x << OFFSET | x >> (u32::BITS - OFFSET)
}
