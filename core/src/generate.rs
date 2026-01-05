use crate::noise::fbm2;
use crate::rng::rand_chance_1_in;
use crate::types::{BOARD_SIZE, BoardState, NEIGH4, idx2};

// 盤面生成パラメータ
const SEED_XOR_MASK: u32 = 0x9e37_79b9;
const FBM_SEED_XOR: u32 = 0x1234_5678;
const FBM_SCALE: f32 = 6.0;
const FBM_OCTAVES: u32 = 4;
const FBM_LACUNARITY: f32 = 2.0;
const FBM_GAIN: f32 = 0.5;
const TERRITORY_CARVE_ITERATIONS: usize = 3;
const TERRITORY_CARVE_CHANCE_DENOM: u32 = 5; // 1/5の確率
const MIN_COMPONENT_SIZE_FOR_SINGLE_TERRITORY: usize = 4;

/// 盤面状態を生成する
///
/// Perlinノイズを用いて自然な盤面を生成します。
///
/// # 引数
/// - `seed`: 乱数シード
///
/// # 戻り値
/// 生成された盤面状態を返します。
fn is_color_stone_or_territory(
    stones: &[u8],
    territory: &[u8],
    x: usize,
    y: usize,
    color: u8,
) -> bool {
    let i = idx2(BOARD_SIZE, x, y);
    stones[i] == color || territory[i] == color
}

fn is_color_stone_or_territory_or_boundary(
    stones: &[u8],
    territory: &[u8],
    x: isize,
    y: isize,
    color: u8,
) -> bool {
    if x < 0 || y < 0 || x >= BOARD_SIZE as isize || y >= BOARD_SIZE as isize {
        // 端の境界は「囲まれている」とみなす
        return true;
    }
    is_color_stone_or_territory(stones, territory, x as usize, y as usize, color)
}

fn carve_territory(
    rng: &mut u32,
    stones: &mut [u8],
    territory: &mut [u8],
    color: u8,
    max_iters: usize,
    chance_denominator: u32,
) {
    for _ in 0..max_iters {
        let mut changed = 0usize;

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let i = idx2(BOARD_SIZE, x, y);
                if stones[i] == 0 {
                    continue; // すでに空
                }
                if territory[i] != 0 {
                    continue; // すでに地（石は残っていても今回は対象外）
                }

                let xi = x as isize;
                let yi = y as isize;
                // 条件: 8方向（周囲8マス）がすべて「同色(石or地)または境界」であること
                let n =
                    is_color_stone_or_territory_or_boundary(stones, territory, xi, yi - 1, color);
                let s =
                    is_color_stone_or_territory_or_boundary(stones, territory, xi, yi + 1, color);
                let w =
                    is_color_stone_or_territory_or_boundary(stones, territory, xi - 1, yi, color);
                let e =
                    is_color_stone_or_territory_or_boundary(stones, territory, xi + 1, yi, color);

                let nw = is_color_stone_or_territory_or_boundary(
                    stones,
                    territory,
                    xi - 1,
                    yi - 1,
                    color,
                );
                let ne = is_color_stone_or_territory_or_boundary(
                    stones,
                    territory,
                    xi + 1,
                    yi - 1,
                    color,
                );
                let sw = is_color_stone_or_territory_or_boundary(
                    stones,
                    territory,
                    xi - 1,
                    yi + 1,
                    color,
                );
                let se = is_color_stone_or_territory_or_boundary(
                    stones,
                    territory,
                    xi + 1,
                    yi + 1,
                    color,
                );

                let surrounded = n && s && w && e && nw && ne && sw && se;

                if surrounded && rand_chance_1_in(rng, chance_denominator) {
                    stones[i] = 0;
                    territory[i] = color;
                    changed += 1;
                }
            }
        }

        if changed == 0 {
            break;
        }
    }
}

fn remove_stone_groups_not_touching_two_territories(
    stones: &mut [u8],
    territory: &mut [u8],
) -> bool {
    // 連結（上下左右）する同色の石グループごとに、
    // 隣接（上下左右）する「地(黒地/白地)」の“領域数(連結成分数)”が2未満ならグループを除去し、
    // 除去箇所は相手色の地にする。
    let n = BOARD_SIZE * BOARD_SIZE;
    let mut visited = vec![false; n];
    let mut changed_any = false;

    for sy in 0..BOARD_SIZE {
        for sx in 0..BOARD_SIZE {
            let start = idx2(BOARD_SIZE, sx, sy);
            let color = stones[start];
            if color == 0 || visited[start] {
                continue;
            }

            // BFSで同色グループを集める
            let mut queue = vec![start];
            visited[start] = true;
            let mut group: Vec<usize> = Vec::new();
            group.push(start);

            // 隣接する地セル（重複排除）: 位置の集合として集める
            let mut touched = vec![false; n];
            let mut touched_list: Vec<usize> = Vec::new();

            while let Some(i) = queue.pop() {
                let x = (i % BOARD_SIZE) as isize;
                let y = (i / BOARD_SIZE) as isize;

                for (dx, dy) in NEIGH4 {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 || nx >= BOARD_SIZE as isize || ny >= BOARD_SIZE as isize {
                        continue;
                    }
                    let ni = idx2(BOARD_SIZE, nx as usize, ny as usize);

                    // 地に接しているか
                    if territory[ni] != 0 && !touched[ni] {
                        touched[ni] = true;
                        touched_list.push(ni);
                    }

                    // 同色石の連結
                    if stones[ni] == color && !visited[ni] {
                        visited[ni] = true;
                        queue.push(ni);
                        group.push(ni);
                    }
                }
            }

            // touched_list（地セル集合）の連結成分数（4近傍）と各成分サイズを数える。
            let mut components = 0usize;
            let mut max_component_size = 0usize;
            let mut tvisited = vec![false; n];
            for &ti in &touched_list {
                if tvisited[ti] {
                    continue;
                }
                components += 1;
                let mut tq = vec![ti];
                tvisited[ti] = true;
                let mut comp_size = 0usize;
                while let Some(ci) = tq.pop() {
                    comp_size += 1;
                    let cx = (ci % BOARD_SIZE) as isize;
                    let cy = (ci / BOARD_SIZE) as isize;
                    for (dx, dy) in NEIGH4 {
                        let nx = cx + dx;
                        let ny = cy + dy;
                        if nx < 0
                            || ny < 0
                            || nx >= BOARD_SIZE as isize
                            || ny >= BOARD_SIZE as isize
                        {
                            continue;
                        }
                        let ni = idx2(BOARD_SIZE, nx as usize, ny as usize);
                        if touched[ni] && !tvisited[ni] {
                            tvisited[ni] = true;
                            tq.push(ni);
                        }
                    }
                }
                if comp_size > max_component_size {
                    max_component_size = comp_size;
                }
            }

            let should_remove = !(components >= 2
                || (components == 1
                    && max_component_size >= MIN_COMPONENT_SIZE_FOR_SINGLE_TERRITORY));

            if should_remove {
                let opp = if color == 1 { 2 } else { 1 };
                for &gi in &group {
                    stones[gi] = 0;
                    territory[gi] = match territory[gi] {
                        0 => opp, // 地が無ければ「相手の地」にする
                        1 => 2,   // 黒地→白地
                        2 => 1,   // 白地→黒地
                        v => v,
                    };
                }
                changed_any = true;
            }
        }
    }
    changed_any
}

fn fill_touching_territories_with_stones(stones: &mut [u8], territory: &mut [u8]) -> bool {
    // 黒地(1)と白地(2)が接触（上下左右）している箇所があれば、
    // 接触している黒地は黒石、白地は白石で埋める（地は消す）。
    let n = BOARD_SIZE * BOARD_SIZE;
    let mut to_black = vec![false; n];
    let mut to_white = vec![false; n];
    let mut changed_any = false;

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let i = idx2(BOARD_SIZE, x, y);
            let t = territory[i];
            if t == 0 {
                continue;
            }

            let xi = x as isize;
            let yi = y as isize;
            for (dx, dy) in NEIGH4 {
                let nx = xi + dx;
                let ny = yi + dy;
                if nx < 0 || ny < 0 || nx >= BOARD_SIZE as isize || ny >= BOARD_SIZE as isize {
                    continue;
                }
                let ni = idx2(BOARD_SIZE, nx as usize, ny as usize);
                let nt = territory[ni];
                if (t == 1 && nt == 2) || (t == 2 && nt == 1) {
                    if t == 1 {
                        to_black[i] = true;
                    } else {
                        to_white[i] = true;
                    }
                    break;
                }
            }
        }
    }

    for i in 0..n {
        if to_black[i] {
            stones[i] = 1;
            territory[i] = 0;
            changed_any = true;
        } else if to_white[i] {
            stones[i] = 2;
            territory[i] = 0;
            changed_any = true;
        }
    }
    changed_any
}

/// 盤面状態を生成する
///
/// Perlinノイズを用いて自然な盤面を生成します。
///
/// # 引数
/// - `seed`: 乱数シード
///
/// # 戻り値
/// 生成された盤面状態を返します。
pub fn generate_board_state(seed: u32) -> BoardState {
    let mut rng = seed ^ SEED_XOR_MASK;

    // 1) まず盤面を黒石/白石で埋める（空は作らない）: パーリンノイズで塊を作る
    let mut stones = vec![0u8; BOARD_SIZE * BOARD_SIZE];
    let mut territory = vec![0u8; BOARD_SIZE * BOARD_SIZE];
    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let fx = x as f32 / FBM_SCALE;
            let fy = y as f32 / FBM_SCALE;
            let n = fbm2(
                seed ^ FBM_SEED_XOR,
                fx,
                fy,
                FBM_OCTAVES,
                FBM_LACUNARITY,
                FBM_GAIN,
            );
            stones[idx2(BOARD_SIZE, x, y)] = if n >= 0.0 { 1 } else { 2 };
        }
    }

    // 2) 地化（少しマイルド）
    for color in [1u8, 2u8] {
        carve_territory(
            &mut rng,
            &mut stones,
            &mut territory,
            color,
            TERRITORY_CARVE_ITERATIONS,
            TERRITORY_CARVE_CHANCE_DENOM,
        );
    }

    // 3/4 相互作用する後処理を収束するまで繰り返す（安全のため上限）
    for _ in 0..(BOARD_SIZE * BOARD_SIZE) {
        let changed3 =
            remove_stone_groups_not_touching_two_territories(&mut stones, &mut territory);
        let changed4 = fill_touching_territories_with_stones(&mut stones, &mut territory);
        if !(changed3 || changed4) {
            break;
        }
    }

    BoardState {
        size: BOARD_SIZE as u32,
        seed,
        stones,
        territory,
    }
}
