mod apply;
mod ip;
mod patterns;
mod select;

use crate::types::{BOARD_SIZE, BoardState, Logger, count_territory};

/// 盤面を整地する
///
/// 整数計画法を用いて最適な整地パターンを探索し、盤面を整地します。
///
/// # 引数
/// - `state`: 整地前の盤面状態
/// - `logger`: ログ出力用のLogger（オプション）
///
/// # 戻り値
/// 整地後の盤面状態を返します。エラーが発生した場合は`Err`を返します。
pub fn level_board(
    mut state: BoardState,
    logger: Option<&dyn Logger>,
) -> Result<BoardState, String> {
    let size = state.size as usize;
    if size != BOARD_SIZE {
        return Err("only 19x19 supported".to_string());
    }
    let n = size * size;
    if state.stones.len() != n || state.territory.len() != n {
        return Err("invalid board arrays length".to_string());
    }

    for color in [1u8, 2u8] {
        let tcount = count_territory(&state.territory, color);
        // 主パターン数は floor(tcount/10)、端数は tcount%10
        let main_target = tcount / 10;
        let remainder = (tcount % 10) as u8;
        let (rects, used) = select::select_rects_and_used(
            size,
            &state.stones,
            &state.territory,
            color,
            main_target,
            remainder,
            logger,
        )?;
        apply::apply_rects_and_fill(
            size,
            &mut state.stones,
            &mut state.territory,
            color,
            &rects,
            &used,
        );
    }

    Ok(state)
}
