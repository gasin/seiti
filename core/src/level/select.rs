use crate::level::ip::solve_select;
use crate::level::patterns::{PatternSpec, cell_in_pattern, generate_candidates};
use crate::types::{Logger, idx2};

/// 選択結果（パターンリストと使用済みセルマスク）
type SelectResult = (Vec<(usize, usize, PatternSpec)>, Vec<bool>);

/// 候補からパターンを選択し、使用済みセルをマークする
///
/// # 引数
/// - `size`: 盤面サイズ
/// - `stones`: 石の配列
/// - `territory`: 地の配列
/// - `color`: 対象色
/// - `main_target`: 主パターンの目標数
/// - `remainder`: 端数（1-9）
/// - `logger`: ログ出力用のLogger（オプション）
///
/// # 戻り値
/// 選択されたパターンのリストと使用済みセルのマスクを返します。
pub(crate) fn select_rects_and_used(
    size: usize,
    stones: &[u8],
    territory: &[u8],
    color: u8,
    main_target: usize,
    remainder: u8,
    logger: Option<&dyn Logger>,
) -> Result<SelectResult, String> {
    let n = size * size;
    let used_cells = vec![false; n];
    let selected: Vec<(usize, usize, PatternSpec)> = Vec::new();
    let rem_required = if (1..=9).contains(&remainder) {
        1usize
    } else {
        0usize
    };
    if main_target == 0 && rem_required == 0 {
        return Ok((selected, used_cells));
    }

    let cands = generate_candidates(size, stones, territory, color, remainder, logger);
    if cands.is_empty() {
        return Ok((selected, used_cells));
    }

    // 主パターン数==main_target、端数パターン数==rem_required を満たす
    let picked = solve_select(&cands, main_target, rem_required, logger)?;

    let mut used = vec![false; n];
    let mut rects: Vec<(usize, usize, PatternSpec)> = Vec::new();
    for si in picked {
        let c = &cands[si];
        rects.push((c.x, c.y, c.spec));
        for dy in 0..c.spec.h {
            for dx in 0..c.spec.w {
                if !cell_in_pattern(dx, dy, &c.spec) {
                    continue;
                }
                let i = idx2(size, c.x + dx, c.y + dy);
                used[i] = true;
            }
        }
    }
    Ok((rects, used))
}
