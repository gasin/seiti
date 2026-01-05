use crate::types::idx2;

pub(crate) fn apply_rects_and_fill(
    size: usize,
    stones: &mut [u8],
    territory: &mut [u8],
    color: u8,
    rects: &[(usize, usize, crate::level::patterns::PatternSpec)],
    used: &[bool],
) {
    for &(x, y, spec) in rects {
        // specに従って適用
        for dy in 0..spec.h {
            for dx in 0..spec.w {
                if !crate::level::patterns::cell_in_pattern(dx, dy, &spec) {
                    continue;
                }
                let i = idx2(size, x + dx, y + dy);
                if spec
                    .anchor_cells
                    .iter()
                    .any(|&(cx, cy)| cx == dx && cy == dy)
                {
                    stones[i] = color; // 残す（無ければ追加）
                    territory[i] = 0;
                } else {
                    stones[i] = 0;
                    territory[i] = color;
                }
            }
        }
    }

    // 「選ばれた領域以外」の同色地は石で埋める
    for i in 0..(size * size) {
        if territory[i] == color && !used[i] {
            stones[i] = color;
            territory[i] = 0;
        }
    }
}
