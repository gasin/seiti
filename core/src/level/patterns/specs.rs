use super::types::{PatternKind, PatternSlot, PatternSpec};

const EMPTY_CELLS: &[(usize, usize)] = &[];
const C34_W4_H3: &[(usize, usize)] = &[(1, 1), (2, 1)];
const C34_W3_H4: &[(usize, usize)] = &[(1, 1), (1, 2)];
const C37_W3_H7: &[(usize, usize)] = &[(1, 3)];
const C37_W7_H3: &[(usize, usize)] = &[(3, 1)];

pub(crate) const PATTERN_SPECS: [PatternSpec; 6] = [
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect2x5,
        w: 5,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect2x5,
        w: 2,
        h: 5,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect3x4,
        w: 4,
        h: 3,
        missing_corner: 0,
        anchor_cells: C34_W4_H3,
    },
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect3x4,
        w: 3,
        h: 4,
        missing_corner: 0,
        anchor_cells: C34_W3_H4,
    },
    // 3x7（中心1マスは「石がない場合にペナルティ」かつ適用時に石を残す/追加する）
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect3x7,
        w: 3,
        h: 7,
        missing_corner: 0,
        anchor_cells: C37_W3_H7,
    },
    PatternSpec {
        slot: PatternSlot::Main,
        kind: PatternKind::Rect3x7,
        w: 7,
        h: 3,
        missing_corner: 0,
        anchor_cells: C37_W7_H3,
    },
];

// 端数(1..=9)用: 1×n（横）と n×1（縦） + 2×(n/2)（奇数は欠け角4パターン）
pub(crate) const REMAINDER_SPECS: [PatternSpec; 38] = [
    // n=1（1x1のみ）
    PatternSpec {
        slot: PatternSlot::Remainder(1),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=2
    PatternSpec {
        slot: PatternSlot::Remainder(2),
        kind: PatternKind::Rect1xN,
        w: 2,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(2),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=3
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect1xN,
        w: 3,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 3,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=4
    PatternSpec {
        slot: PatternSlot::Remainder(4),
        kind: PatternKind::Rect1xN,
        w: 4,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(4),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 4,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=5
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect1xN,
        w: 5,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 5,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=6
    PatternSpec {
        slot: PatternSlot::Remainder(6),
        kind: PatternKind::Rect1xN,
        w: 6,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(6),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 6,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=7
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect1xN,
        w: 7,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 7,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=8
    PatternSpec {
        slot: PatternSlot::Remainder(8),
        kind: PatternKind::Rect1xN,
        w: 8,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(8),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 8,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=9
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect1xN,
        w: 9,
        h: 1,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect1xN,
        w: 1,
        h: 9,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // n=9 のときのみ 3x3 も候補にする
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect3x3,
        w: 3,
        h: 3,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // 2x(N/2)（偶数）
    PatternSpec {
        slot: PatternSlot::Remainder(2),
        kind: PatternKind::Rect2xHalf,
        w: 1,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(4),
        kind: PatternKind::Rect2xHalf,
        w: 2,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(6),
        kind: PatternKind::Rect2xHalf,
        w: 3,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(8),
        kind: PatternKind::Rect2xHalf,
        w: 4,
        h: 2,
        missing_corner: 0,
        anchor_cells: EMPTY_CELLS,
    },
    // 2x(floor(N/2))+1（奇数は欠け角4パターン）
    // n=3: w=2,h=2 - one corner missing
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect2xHalf,
        w: 2,
        h: 2,
        missing_corner: 1,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect2xHalf,
        w: 2,
        h: 2,
        missing_corner: 2,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect2xHalf,
        w: 2,
        h: 2,
        missing_corner: 3,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(3),
        kind: PatternKind::Rect2xHalf,
        w: 2,
        h: 2,
        missing_corner: 4,
        anchor_cells: EMPTY_CELLS,
    },
    // n=5: w=3,h=2
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect2xHalf,
        w: 3,
        h: 2,
        missing_corner: 1,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect2xHalf,
        w: 3,
        h: 2,
        missing_corner: 2,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect2xHalf,
        w: 3,
        h: 2,
        missing_corner: 3,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(5),
        kind: PatternKind::Rect2xHalf,
        w: 3,
        h: 2,
        missing_corner: 4,
        anchor_cells: EMPTY_CELLS,
    },
    // n=7: w=4,h=2
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect2xHalf,
        w: 4,
        h: 2,
        missing_corner: 1,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect2xHalf,
        w: 4,
        h: 2,
        missing_corner: 2,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect2xHalf,
        w: 4,
        h: 2,
        missing_corner: 3,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(7),
        kind: PatternKind::Rect2xHalf,
        w: 4,
        h: 2,
        missing_corner: 4,
        anchor_cells: EMPTY_CELLS,
    },
    // n=9: w=5,h=2
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect2xHalf,
        w: 5,
        h: 2,
        missing_corner: 1,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect2xHalf,
        w: 5,
        h: 2,
        missing_corner: 2,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect2xHalf,
        w: 5,
        h: 2,
        missing_corner: 3,
        anchor_cells: EMPTY_CELLS,
    },
    PatternSpec {
        slot: PatternSlot::Remainder(9),
        kind: PatternKind::Rect2xHalf,
        w: 5,
        h: 2,
        missing_corner: 4,
        anchor_cells: EMPTY_CELLS,
    },
];

