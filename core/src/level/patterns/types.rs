/// パターンのスロット（主パターン or 端数パターン）
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum PatternSlot {
    Main,
    /// 端数 r(1..=9) 用
    Remainder(u8),
}

/// パターンの種類
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum PatternKind {
    Rect2x5,
    Rect3x4,
    Rect3x7,
    Rect1xN,
    Rect2xHalf,
    Rect3x3,
}

/// パターン仕様
#[derive(Copy, Clone, Debug)]
pub(crate) struct PatternSpec {
    pub(crate) slot: PatternSlot,
    pub(crate) kind: PatternKind,
    pub(crate) w: usize,
    pub(crate) h: usize,
    /// 0: full, 1: TL, 2: TR, 3: BL, 4: BR（端数の2行パターン用）
    pub(crate) missing_corner: u8,
    /// アンカーセル（ローカル座標）。
    /// - ペナルティ計算: 石があればペナルティ0、なければペナルティ+1（通常セルは逆）
    /// - 適用時: 石を残す（無ければ追加）
    pub(crate) anchor_cells: &'static [(usize, usize)],
}

/// 候補パターン
#[derive(Clone)]
pub(crate) struct Cand {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) spec: PatternSpec,
    pub(crate) cost: u32,
    pub(crate) stones_in_rect: u32,
    pub(crate) penalty_total: u32,
    pub(crate) penalty_perimeter: u32,
    pub(crate) penalty_internal: u32,
    pub(crate) perimeter_opp_cells: u32,
    pub(crate) internal_no_stone_cells: u32,
    pub(crate) mask: Vec<u64>,
    pub(crate) mask_block: Vec<u64>,
}

