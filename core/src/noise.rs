use crate::rng::xorshift32;

fn fade(t: f32) -> f32 {
    // 6t^5 - 15t^4 + 10t^3
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn hash3_u32(seed: u32, x: i32, y: i32) -> u32 {
    // 座標とseedから安定した乱数を作る（簡易ハッシュ）
    let mut v = seed ^ 0x9e37_79b9;
    v ^= (x as u32).wrapping_mul(0x85eb_ca6b);
    v = xorshift32(v);
    v ^= (y as u32).wrapping_mul(0xc2b2_ae35);
    xorshift32(v)
}

fn grad_dot(seed: u32, ix: i32, iy: i32, x: f32, y: f32) -> f32 {
    // 8方向の勾配ベクトルから1つ選び、(x-ix, y-iy)との内積を返す
    let h = hash3_u32(seed, ix, iy) & 7;
    let (gx, gy) = match h {
        0 => (1.0, 0.0),
        1 => (-1.0, 0.0),
        2 => (0.0, 1.0),
        3 => (0.0, -1.0),
        4 => (0.707_106_77, 0.707_106_77),
        5 => (-0.707_106_77, 0.707_106_77),
        6 => (0.707_106_77, -0.707_106_77),
        _ => (-0.707_106_77, -0.707_106_77),
    };
    let dx = x - ix as f32;
    let dy = y - iy as f32;
    gx * dx + gy * dy
}

fn perlin2(seed: u32, x: f32, y: f32) -> f32 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let sx = fade(x - x0 as f32);
    let sy = fade(y - y0 as f32);

    let n00 = grad_dot(seed, x0, y0, x, y);
    let n10 = grad_dot(seed, x1, y0, x, y);
    let n01 = grad_dot(seed, x0, y1, x, y);
    let n11 = grad_dot(seed, x1, y1, x, y);

    let ix0 = lerp(n00, n10, sx);
    let ix1 = lerp(n01, n11, sx);
    lerp(ix0, ix1, sy)
}

pub fn fbm2(seed: u32, x: f32, y: f32, octaves: u32, lacunarity: f32, gain: f32) -> f32 {
    let mut amp = 1.0f32;
    let mut freq = 1.0f32;
    let mut sum = 0.0f32;
    let mut norm = 0.0f32;
    for i in 0..octaves {
        // オクターブごとにseedをずらす（同じ格子に落ちても相関しすぎないように）
        let s = seed.wrapping_add(i.wrapping_mul(0x6d2b_79f5));
        sum += amp * perlin2(s, x * freq, y * freq);
        norm += amp;
        amp *= gain;
        freq *= lacunarity;
    }
    if norm == 0.0 { 0.0 } else { sum / norm }
}
