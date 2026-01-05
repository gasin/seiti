use serde::{Deserialize, Serialize};

pub const BOARD_SIZE: usize = 19;

/// 4方向の隣接セル（上下左右）
pub const NEIGH4: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

/// 盤面状態:
/// - stones: 0=空, 1=黒石, 2=白石
/// - territory: 0=どちらでもない, 1=黒地, 2=白地
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardState {
    pub size: u32,
    pub seed: u32,
    pub stones: Vec<u8>,
    pub territory: Vec<u8>,
}

pub trait Logger {
    fn log(&self, s: &str);
}

/// 2次元座標を1次元インデックスに変換
pub fn idx2(size: usize, x: usize, y: usize) -> usize {
    y * size + x
}

/// 盤面上の指定色の石の位置を収集する
pub fn collect_stone_positions(stones: &[u8], size: usize, color: u8) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for y in 0..size {
        for x in 0..size {
            let i = idx2(size, x, y);
            if stones[i] == color {
                positions.push((x, y));
            }
        }
    }
    positions
}

/// 指定色の地の数をカウントする
pub fn count_territory(territory: &[u8], color: u8) -> usize {
    territory.iter().filter(|&&t| t == color).count()
}

/// 石の移動情報
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoneMove {
    pub color: u8,        // 1=黒, 2=白
    pub from: (u32, u32), // (x, y)
    pub to: (u32, u32),   // (x, y)
}
