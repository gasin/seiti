use super::types::{Cand, PatternKind, PatternSlot, PatternSpec};

pub(crate) fn log_patterns_enabled() -> bool {
    std::env::var_os("SEITI_LOG_PATTERNS").is_some()
}

fn spec_name(spec: &PatternSpec) -> &'static str {
    match (spec.kind, spec.w, spec.h) {
        (PatternKind::Rect2x5, 5, 2) => "2x5(5x2)",
        (PatternKind::Rect2x5, 2, 5) => "2x5(2x5)",
        (PatternKind::Rect3x4, 4, 3) => "3x4(4x3)",
        (PatternKind::Rect3x4, 3, 4) => "3x4(3x4)",
        (PatternKind::Rect3x7, 3, 7) => "3x7(3x7)",
        (PatternKind::Rect3x7, 7, 3) => "3x7(7x3)",
        (PatternKind::Rect1xN, _, _) => "1xN",
        (PatternKind::Rect2xHalf, _, _) => "2x(N/2)",
        (PatternKind::Rect3x3, 3, 3) => "3x3",
        (PatternKind::Rect2x5, _, _) => "2x5",
        (PatternKind::Rect3x4, _, _) => "3x4",
        (PatternKind::Rect3x3, _, _) => "3x3",
        (PatternKind::Rect3x7, _, _) => "3x7",
    }
}

fn slot_name(slot: PatternSlot) -> String {
    match slot {
        PatternSlot::Main => "main".to_string(),
        PatternSlot::Remainder(r) => format!("rem({r})"),
    }
}

// spec_nameとslot_nameはログ出力用に公開（candidate.rsで使用）
pub(crate) fn spec_name_pub(spec: &PatternSpec) -> &'static str {
    spec_name(spec)
}

pub(crate) fn slot_name_pub(slot: PatternSlot) -> String {
    slot_name(slot)
}

pub(crate) fn cell_in_pattern(dx: usize, dy: usize, spec: &PatternSpec) -> bool {
    let missing = match spec.missing_corner {
        0 => false,
        1 => dx == 0 && dy == 0,                   // TL missing
        2 => dx + 1 == spec.w && dy == 0,          // TR missing
        3 => dx == 0 && dy + 1 == spec.h,          // BL missing
        4 => dx + 1 == spec.w && dy + 1 == spec.h, // BR missing
        _ => false,
    };
    !missing
}

pub(crate) fn mask_set(mask: &mut [u64], idx: usize) {
    let w = idx / 64;
    let b = idx % 64;
    mask[w] |= 1u64 << b;
}

pub(crate) fn mask_overlaps(a: &[u64], b: &[u64]) -> bool {
    for i in 0..a.len() {
        if (a[i] & b[i]) != 0 {
            return true;
        }
    }
    false
}

pub(crate) fn long_edge_ok_2x5_only(a: &Cand, b: &Cand) -> bool {
    // 例外（2x5のみ）: 長辺が完全一致し、短辺方向にぴったり2枚並ぶ場合だけ隣接を許可
    // 3x4は「連結禁止」なので例外を一切認めない
    if a.spec.kind != PatternKind::Rect2x5 || b.spec.kind != PatternKind::Rect2x5 {
        return false;
    }
    if a.spec.w != b.spec.w || a.spec.h != b.spec.h {
        return false;
    }
    if a.spec.w >= a.spec.h {
        // 横長: 上下に積む
        a.x == b.x && (a.y + a.spec.h == b.y || b.y + b.spec.h == a.y)
    } else {
        // 縦長: 左右に並べる
        a.y == b.y && (a.x + a.spec.w == b.x || b.x + b.spec.w == a.x)
    }
}
