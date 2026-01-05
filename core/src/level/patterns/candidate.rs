use super::specs::{PATTERN_SPECS, REMAINDER_SPECS};
use super::types::{Cand, PatternKind, PatternSlot, PatternSpec};
use super::utils::{cell_in_pattern, log_patterns_enabled, mask_set, slot_name_pub, spec_name_pub};
use crate::types::{Logger, NEIGH4, idx2};

// 外周判定:
// - 同色（石/地）または境界: OK
// - 相手色（石/地）: OK だが 1セルにつきペナルティ+10
// - それ以外（空など）: NG
fn perimeter_class(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    x: isize,
    y: isize,
    color: u8,
) -> u8 {
    if x < 0 || y < 0 || x >= size as isize || y >= size as isize {
        return 1; // boundary
    }
    let i = idx2(size, x as usize, y as usize);
    if stones[i] == color || territory[i] == color {
        return 1; // same color
    }
    let opp = if color == 1 { 2 } else { 1 };
    if stones[i] == opp || territory[i] == opp {
        return 2; // opponent
    }
    0 // other (e.g. empty)
}

// 外周チェック（4方向）: 上/下/左/右が囲まれているか確認
// 戻り値: (ok, perimeter_opp_cells, penalty_perimeter)
#[allow(clippy::too_many_arguments)]
fn check_perimeter_4_sides(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u8,
) -> (bool, u32, u32) {
    let mut perimeter_opp_cells = 0u32;
    let mut penalty_perimeter = 0u32;

    // 上（幅 w）
    for dx in 0..w {
        match perimeter_class(
            size,
            stones,
            territory,
            (x + dx) as isize,
            (y as isize) - 1,
            color,
        ) {
            1 => {}
            2 => {
                perimeter_opp_cells += 1;
                penalty_perimeter += 10;
            }
            _ => return (false, 0, 0),
        }
    }

    // 下（幅 w）
    for dx in 0..w {
        match perimeter_class(
            size,
            stones,
            territory,
            (x + dx) as isize,
            (y + h) as isize,
            color,
        ) {
            1 => {}
            2 => {
                perimeter_opp_cells += 1;
                penalty_perimeter += 10;
            }
            _ => return (false, 0, 0),
        }
    }

    // 左（高さ h）
    for dy in 0..h {
        match perimeter_class(
            size,
            stones,
            territory,
            (x as isize) - 1,
            (y + dy) as isize,
            color,
        ) {
            1 => {}
            2 => {
                perimeter_opp_cells += 1;
                penalty_perimeter += 10;
            }
            _ => return (false, 0, 0),
        }
    }

    // 右（高さ h）
    for dy in 0..h {
        match perimeter_class(
            size,
            stones,
            territory,
            (x + w) as isize,
            (y + dy) as isize,
            color,
        ) {
            1 => {}
            2 => {
                perimeter_opp_cells += 1;
                penalty_perimeter += 10;
            }
            _ => return (false, 0, 0),
        }
    }

    (true, perimeter_opp_cells, penalty_perimeter)
}

// 内部制約チェック: 内部は「同色の石 or 同色の地」のみで構成されていること
fn check_internal_constraint(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    x: usize,
    y: usize,
    spec: &PatternSpec,
    color: u8,
) -> bool {
    for dy in 0..spec.h {
        for dx in 0..spec.w {
            if !cell_in_pattern(dx, dy, spec) {
                continue;
            }
            let i = idx2(size, x + dx, y + dy);
            if !(stones[i] == color || territory[i] == color) {
                return false;
            }
        }
    }
    true
}

// 内部ペナルティ計算とmask生成
// 戻り値: (stones_in_rect, penalty_internal, internal_no_stone_cells, mask, mask_block)
fn calculate_internal_penalty_and_masks(
    size: usize,
    stones: &[u8],
    x: usize,
    y: usize,
    spec: &PatternSpec,
    words: usize,
) -> (u32, u32, u32, Vec<u64>, Vec<u64>) {
    let mut mask = vec![0u64; words];
    let mut mask_block = vec![0u64; words];
    let mut stones_in_rect = 0u32;
    let mut penalty_internal = 0u32;
    let mut internal_no_stone_cells = 0u32;

    for dy in 0..spec.h {
        for dx in 0..spec.w {
            if !cell_in_pattern(dx, dy, spec) {
                continue;
            }
            let i = idx2(size, x + dx, y + dy);
            mask_set(&mut mask, i);
            mask_set(&mut mask_block, i);
            let is_anchor = spec
                .anchor_cells
                .iter()
                .any(|&(cx, cy)| cx == dx && cy == dy);
            let has_stone = stones[i] != 0;
            if has_stone {
                stones_in_rect += 1;
            }
            if is_anchor {
                // anchorセル: 石がなければペナルティ+1
                if !has_stone {
                    internal_no_stone_cells += 1;
                    penalty_internal = penalty_internal.saturating_add(1);
                }
            } else {
                // 通常セル: 石があればペナルティ+1
                if has_stone {
                    penalty_internal = penalty_internal.saturating_add(1);
                }
            }
        }
    }

    (
        stones_in_rect,
        penalty_internal,
        internal_no_stone_cells,
        mask,
        mask_block,
    )
}

// mask_blockに4近傍を追加（隣接禁止用）
fn build_mask_block(size: usize, x: usize, y: usize, spec: &PatternSpec, mask_block: &mut [u64]) {
    for dy in 0..spec.h {
        for dx in 0..spec.w {
            if !cell_in_pattern(dx, dy, spec) {
                continue;
            }
            let cx = (x + dx) as isize;
            let cy = (y + dy) as isize;
            for (nx, ny) in NEIGH4 {
                let px = cx + nx;
                let py = cy + ny;
                if px < 0 || py < 0 || px >= size as isize || py >= size as isize {
                    continue;
                }
                let pi = idx2(size, px as usize, py as usize);
                mask_set(mask_block, pi);
            }
        }
    }
}

// 1つの候補を生成する処理
#[allow(clippy::too_many_arguments)]
fn try_create_candidate(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    x: usize,
    y: usize,
    spec: &PatternSpec,
    color: u8,
    words: usize,
) -> Option<Cand> {
    // 外周チェック
    let (ok, perimeter_opp_cells, penalty_perimeter) =
        check_perimeter_4_sides(size, stones, territory, x, y, spec.w, spec.h, color);
    if !ok {
        return None;
    }

    // 内部制約チェック
    if !check_internal_constraint(size, stones, territory, x, y, spec, color) {
        return None;
    }

    // 内部ペナルティ計算とmask生成
    let (stones_in_rect, penalty_internal, internal_no_stone_cells, mask, mut mask_block) =
        calculate_internal_penalty_and_masks(size, stones, x, y, spec, words);

    // mask_blockに4近傍を追加
    build_mask_block(size, x, y, spec, &mut mask_block);

    let penalty_total = penalty_perimeter.saturating_add(penalty_internal);
    let cost = penalty_total;

    Some(Cand {
        x,
        y,
        spec: *spec,
        cost,
        stones_in_rect,
        penalty_total,
        penalty_perimeter,
        penalty_internal,
        perimeter_opp_cells,
        internal_no_stone_cells,
        mask,
        mask_block,
    })
}

// 1つのspecに対する候補生成
fn generate_candidates_for_spec(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    spec: &PatternSpec,
    color: u8,
    words: usize,
) -> Vec<Cand> {
    let mut cands = Vec::new();
    if size < spec.w || size < spec.h {
        return cands;
    }
    for y in 0..=(size - spec.h) {
        for x in 0..=(size - spec.w) {
            if let Some(cand) =
                try_create_candidate(size, stones, territory, x, y, spec, color, words)
            {
                cands.push(cand);
            }
        }
    }
    cands
}

pub(crate) fn generate_candidates(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    color: u8,
    remainder: u8,
    logger: Option<&dyn Logger>,
) -> Vec<Cand> {
    let n = size * size;
    #[allow(clippy::manual_div_ceil)]
    let words = (n + 63) / 64;

    let mut cands: Vec<Cand> = Vec::new();

    // 主パターン
    for spec in PATTERN_SPECS {
        cands.extend(generate_candidates_for_spec(
            size, stones, territory, &spec, color, words,
        ));
    }

    // 端数パターン（r=1..9 のときのみ、該当rだけ列挙）
    if (1..=9).contains(&remainder) {
        for spec in REMAINDER_SPECS {
            match spec.slot {
                PatternSlot::Remainder(r) if r == remainder => {}
                _ => continue,
            }
            // 仕様: 1xN は N>=6 を禁止（端数用の線形パターンを抑制）
            if spec.kind == PatternKind::Rect1xN && spec.w.max(spec.h) >= 6 {
                continue;
            }
            cands.extend(generate_candidates_for_spec(
                size, stones, territory, &spec, color, words,
            ));
        }
    }

    cands.sort_by_key(|c| c.cost);

    if let Some(l) = logger {
        let penalty_candidates = cands.iter().filter(|c| c.penalty_total > 0).count();
        l.log(&format!(
            "[patterns] color={color} remainder={remainder} candidates={} penalty_candidates={penalty_candidates}",
            cands.len()
        ));
        if log_patterns_enabled() {
            for c in &cands {
                l.log(&format!(
                    "[cand] color={color} slot={} spec={} x={} y={} w={} h={} stones={} perimOpp={} innerNoStone={} penPerim={} penInner={} cost={}",
                    slot_name_pub(c.spec.slot),
                    spec_name_pub(&c.spec),
                    c.x,
                    c.y,
                    c.spec.w,
                    c.spec.h,
                    c.stones_in_rect,
                    c.perimeter_opp_cells,
                    c.internal_no_stone_cells,
                    c.penalty_perimeter,
                    c.penalty_internal,
                    c.cost
                ));
            }
        }
    }

    cands
}
