use core::simd::{u32x4, u64x2, ToBytes};

pub(crate) fn expand(seed: &[u32; 8], buffer: &mut [u64; 128]) {
    let (chunks, _) = buffer.as_chunks_mut::<32>();
    expand_part(seed, &mut chunks[0], 00);
    expand_part(seed, &mut chunks[1], 04);
    expand_part(seed, &mut chunks[2], 08);
    expand_part(seed, &mut chunks[3], 12);
}

fn expand_part(seed: &[u32; 8], buffer: &mut [u64; 32], counter: u32) {
    let mut state = [
        u32x4::splat(0x61707865),
        u32x4::splat(0x3320646e),
        u32x4::splat(0x79622d32),
        u32x4::splat(0x6b206574),
        u32x4::splat(seed[0]),
        u32x4::splat(seed[1]),
        u32x4::splat(seed[2]),
        u32x4::splat(seed[3]),
        u32x4::splat(seed[4]),
        u32x4::splat(seed[5]),
        u32x4::splat(seed[6]),
        u32x4::splat(seed[7]),
        u32x4::splat(counter) + u32x4::from_array([0, 1, 2, 3]),
        u32x4::splat(0),
        u32x4::splat(0),
        u32x4::splat(0),
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

    state[04] += u32x4::splat(seed[0]);
    state[05] += u32x4::splat(seed[1]);
    state[06] += u32x4::splat(seed[2]);
    state[07] += u32x4::splat(seed[3]);
    state[08] += u32x4::splat(seed[4]);
    state[09] += u32x4::splat(seed[5]);
    state[10] += u32x4::splat(seed[6]);
    state[11] += u32x4::splat(seed[7]);

    buffer[00..02].copy_from_slice(&u64x2::from_le_bytes(state[00].to_le_bytes()).to_array());
    buffer[02..04].copy_from_slice(&u64x2::from_le_bytes(state[01].to_le_bytes()).to_array());
    buffer[04..06].copy_from_slice(&u64x2::from_le_bytes(state[02].to_le_bytes()).to_array());
    buffer[06..08].copy_from_slice(&u64x2::from_le_bytes(state[03].to_le_bytes()).to_array());
    buffer[08..10].copy_from_slice(&u64x2::from_le_bytes(state[04].to_le_bytes()).to_array());
    buffer[10..12].copy_from_slice(&u64x2::from_le_bytes(state[05].to_le_bytes()).to_array());
    buffer[12..14].copy_from_slice(&u64x2::from_le_bytes(state[06].to_le_bytes()).to_array());
    buffer[14..16].copy_from_slice(&u64x2::from_le_bytes(state[07].to_le_bytes()).to_array());
    buffer[16..18].copy_from_slice(&u64x2::from_le_bytes(state[08].to_le_bytes()).to_array());
    buffer[18..20].copy_from_slice(&u64x2::from_le_bytes(state[09].to_le_bytes()).to_array());
    buffer[20..22].copy_from_slice(&u64x2::from_le_bytes(state[10].to_le_bytes()).to_array());
    buffer[22..24].copy_from_slice(&u64x2::from_le_bytes(state[11].to_le_bytes()).to_array());
    buffer[24..26].copy_from_slice(&u64x2::from_le_bytes(state[12].to_le_bytes()).to_array());
    buffer[26..28].copy_from_slice(&u64x2::from_le_bytes(state[13].to_le_bytes()).to_array());
    buffer[28..30].copy_from_slice(&u64x2::from_le_bytes(state[14].to_le_bytes()).to_array());
    buffer[30..32].copy_from_slice(&u64x2::from_le_bytes(state[15].to_le_bytes()).to_array());
}

fn quarter_round<const A: usize, const B: usize, const C: usize, const D: usize>(
    state: &mut [u32x4; 16],
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

fn rotate_left<const OFFSET: u32>(x: u32x4) -> u32x4 {
    x << OFFSET | x >> (u32::BITS - OFFSET)
}
