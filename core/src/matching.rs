use crate::types::{BOARD_SIZE, BoardState, StoneMove, collect_stone_positions};

/// ハンガリアン法で最小コストマッチングを求める
/// コスト行列は正方行列である必要がある
fn hungarian_algorithm(cost_matrix: &[Vec<f64>]) -> Vec<usize> {
    let n = cost_matrix.len();
    if n == 0 {
        return Vec::new();
    }

    // u, v: 双対変数
    // u[i]: 行iの双対変数 (i: 0..n, 実行は 0..n)
    // v[j]: 列jの双対変数 (j: 0..n+1, 実列は 0..n, ダミー列は n)
    let mut u = vec![0.0; n];
    let mut v = vec![0.0; n + 1];
    // p: マッチング（p[j] = i は 列jが行iにマッチ、p[n]はダミー列）
    let mut p = vec![n; n + 1]; // 初期値は全てダミー列nにマッチ
    // way: 増加パスを記録
    let mut way = vec![0; n + 1];

    // 各行i (0..n) について処理
    for i in 0..n {
        p[n] = i; // ダミー列nを行iにマッチ
        let mut j0 = n; // ダミー列から開始
        let mut minv = vec![f64::INFINITY; n + 1];
        let mut used = vec![false; n + 1];

        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = f64::INFINITY;
            let mut j1 = n; // デフォルトはダミー列

            // 実列 j (0..n) を探索
            for j in 0..n {
                if !used[j] {
                    let cur = cost_matrix[i0][j] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }

            // 双対変数を更新
            for j in 0..=n {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if p[j0] == n {
                // ダミー列に到達したら終了
                break;
            }
        }

        // 増加パスを逆にたどってマッチングを更新
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == n {
                break;
            }
        }
    }

    // 結果を構築: result[j] = i (0-indexed, jは実列0..n)
    p[..n].to_vec()
}

/// 盤面生成後と整地後の石の移動対応を計算
///
/// ハンガリアン法を用いて移動距離を最小化する石の対応関係を求めます。
///
/// # 引数
/// - `before`: 整地前の盤面状態
/// - `after`: 整地後の盤面状態
///
/// # 戻り値
/// 石の移動情報のベクタを返します。エラーが発生した場合は`Err`を返します。
pub fn compute_stone_moves(
    before: &BoardState,
    after: &BoardState,
) -> Result<Vec<StoneMove>, String> {
    let size = before.size as usize;
    if size != BOARD_SIZE || after.size as usize != size {
        return Err("only 19x19 supported".to_string());
    }
    if before.stones.len() != size * size || after.stones.len() != size * size {
        return Err("invalid board arrays length".to_string());
    }

    let mut moves = Vec::new();

    // 各色（黒・白）について処理
    for color in [1u8, 2u8] {
        // 移動前の石の位置を収集
        let before_positions = collect_stone_positions(&before.stones, size, color);

        // 移動後の石の位置を収集
        let after_positions = collect_stone_positions(&after.stones, size, color);

        // 石の数が一致していることを確認
        if before_positions.len() != after_positions.len() {
            return Err(format!(
                "stone count mismatch for color {color}: before={}, after={}",
                before_positions.len(),
                after_positions.len()
            ));
        }

        let n = before_positions.len();
        if n == 0 {
            continue;
        }

        // 距離0の対応を先に固定
        let mut before_used = vec![false; n];
        let mut after_used = vec![false; n];

        for (i, &(bx, by)) in before_positions.iter().enumerate() {
            for (j, &(ax, ay)) in after_positions.iter().enumerate() {
                if !before_used[i] && !after_used[j] && bx == ax && by == ay {
                    before_used[i] = true;
                    after_used[j] = true;
                    moves.push(StoneMove {
                        color,
                        from: (bx as u32, by as u32),
                        to: (ax as u32, ay as u32),
                    });
                    break;
                }
            }
        }

        // 固定されていない石の位置
        let before_remaining: Vec<_> = before_positions
            .iter()
            .enumerate()
            .filter(|(i, _)| !before_used[*i])
            .map(|(i, &pos)| (i, pos))
            .collect();
        let after_remaining: Vec<_> = after_positions
            .iter()
            .enumerate()
            .filter(|(j, _)| !after_used[*j])
            .map(|(j, &pos)| (j, pos))
            .collect();

        let m = before_remaining.len();
        if m == 0 {
            continue;
        }

        // コスト行列を作成（ユークリッド距離の2乗）
        let mut cost_matrix = vec![vec![0.0; m]; m];
        for (i, &(_, (bx, by))) in before_remaining.iter().enumerate() {
            for (j, &(_, (ax, ay))) in after_remaining.iter().enumerate() {
                let dx = (bx as f64) - (ax as f64);
                let dy = (by as f64) - (ay as f64);
                cost_matrix[i][j] = dx * dx + dy * dy;
            }
        }

        // ハンガリアン法でマッチング
        let matching = hungarian_algorithm(&cost_matrix);

        // 結果を構築
        for (j, &i) in matching.iter().enumerate() {
            if j < m && i < m {
                let (_, (bx, by)) = before_remaining[i];
                let (_, (ax, ay)) = after_remaining[j];
                moves.push(StoneMove {
                    color,
                    from: (bx as u32, by as u32),
                    to: (ax as u32, ay as u32),
                });
            }
        }
    }

    Ok(moves)
}
