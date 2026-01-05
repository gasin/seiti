// シンプルなRNGユーティリティ（WASM時代の依存を避けるため自前）

pub(crate) fn xorshift32(mut x: u32) -> u32 {
    // seed=0でも動くように少しだけ混ぜる
    if x == 0 {
        x = 0x6d2b_79f5;
    }
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

pub(crate) fn next_u32(state: &mut u32) -> u32 {
    *state = xorshift32(*state);
    *state
}

pub(crate) fn rand_chance_1_in(state: &mut u32, n: u32) -> bool {
    (next_u32(state) % n) == 0
}
