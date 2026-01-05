use crate::level::patterns::{
    Cand, PatternSlot, log_patterns_enabled, long_edge_ok_2x5_only, mask_overlaps,
};
use crate::types::Logger;
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, constraint, highs, variable};

fn spec_name(c: &Cand) -> &'static str {
    // patterns.rs と同じ分類（ログ用）
    use crate::level::patterns::PatternKind;
    let k = c.spec.kind;
    if k == PatternKind::Rect2x5 {
        return match (c.spec.w, c.spec.h) {
            (5, 2) => "2x5(5x2)",
            (2, 5) => "2x5(2x5)",
            _ => "2x5",
        };
    }
    if k == PatternKind::Rect3x4 {
        return match (c.spec.w, c.spec.h) {
            (4, 3) => "3x4(4x3)",
            (3, 4) => "3x4(3x4)",
            _ => "3x4",
        };
    }
    if k == PatternKind::Rect3x7 {
        return match (c.spec.w, c.spec.h) {
            (3, 7) => "3x7(3x7)",
            (7, 3) => "3x7(7x3)",
            _ => "3x7",
        };
    }
    if k == PatternKind::Rect1xN {
        return "1xN";
    }
    if k == PatternKind::Rect2xHalf {
        return "2x(N/2)";
    }
    // PatternKind::Rect3x3
    "3x3"
}

pub(crate) fn solve_select(
    cands: &[Cand],
    main_target: usize,
    remainder_required: usize,
    logger: Option<&dyn Logger>,
) -> Result<Vec<usize>, String> {
    if cands.is_empty() || (main_target == 0 && remainder_required == 0) {
        return Ok(Vec::new());
    }

    let m = cands.len();
    let mut pb = ProblemVariables::new();
    let xs = (0..m)
        .map(|_| pb.add(variable().binary()))
        .collect::<Vec<_>>();

    // 目的: コスト最小化（個数は制約で一致させる）
    let mut obj: Expression = 0.0.into();
    for (i, cand) in cands.iter().enumerate() {
        obj += (cand.cost as f64) * xs[i];
    }
    let mut model = pb.minimise(obj).using(highs);

    // 主パターン重み: Σ w_i * x_main_i == main_target
    let mut sum_main: Expression = 0.0.into();
    for (i, cand) in cands.iter().enumerate() {
        if cand.spec.slot == PatternSlot::Main {
            let w = if cand.spec.kind == crate::level::patterns::PatternKind::Rect3x7 {
                2.0
            } else {
                1.0
            };
            sum_main += w * xs[i];
        }
    }
    let sum_main_le = sum_main.clone();
    model = model.with(constraint!(sum_main_le <= main_target as f64));
    model = model.with(constraint!(sum_main >= main_target as f64));

    // 端数パターン数: Σ x_rem == remainder_required（0 or 1）
    let mut sum_rem: Expression = 0.0.into();
    for (i, cand) in cands.iter().enumerate() {
        if matches!(cand.spec.slot, PatternSlot::Remainder(_)) {
            sum_rem += xs[i];
        }
    }
    let sum_rem_le = sum_rem.clone();
    model = model.with(constraint!(sum_rem_le <= remainder_required as f64));
    model = model.with(constraint!(sum_rem >= remainder_required as f64));

    // 衝突ペア制約 x_i + x_j <= 1
    let mut conflicts = 0usize;
    for i in 0..m {
        for j in (i + 1)..m {
            if mask_overlaps(&cands[i].mask, &cands[j].mask) {
                model = model.with(constraint!(xs[i] + xs[j] <= 1));
                conflicts += 1;
                continue;
            }
            let adjacent = mask_overlaps(&cands[i].mask_block, &cands[j].mask)
                || mask_overlaps(&cands[j].mask_block, &cands[i].mask);
            if adjacent && !long_edge_ok_2x5_only(&cands[i], &cands[j]) {
                model = model.with(constraint!(xs[i] + xs[j] <= 1));
                conflicts += 1;
            }
        }
    }

    if let Some(l) = logger {
        l.log(&format!(
            "[highs] vars={m} conflicts={conflicts} main_target={main_target} rem_required={remainder_required}"
        ));
    }

    let solution = model.solve().map_err(|e| e.to_string())?;
    let solve_status = solution.status();

    let mut picked: Vec<usize> = Vec::new();
    for (i, x) in xs.iter().enumerate() {
        let v = solution.value(*x);
        if v.is_finite() && v > 0.5 {
            picked.push(i);
        }
    }

    if let Some(l) = logger {
        let mut cost_sum: u32 = 0;
        let mut penalty_sum: u32 = 0;
        let mut pen_perim_sum: u32 = 0;
        let mut pen_inner_sum: u32 = 0;
        let mut picked_penalty: usize = 0;
        for &i in &picked {
            cost_sum = cost_sum.saturating_add(cands[i].cost);
            penalty_sum = penalty_sum.saturating_add(cands[i].penalty_total);
            pen_perim_sum = pen_perim_sum.saturating_add(cands[i].penalty_perimeter);
            pen_inner_sum = pen_inner_sum.saturating_add(cands[i].penalty_internal);
            if cands[i].penalty_total > 0 {
                picked_penalty += 1;
            }
        }
        l.log(&format!(
            "[highs] status={solve_status:?} picked={} picked_penalty={picked_penalty} cost_sum={cost_sum} penalty_sum={penalty_sum} (perim={pen_perim_sum} inner={pen_inner_sum})",
            picked.len()
        ));

        if log_patterns_enabled() {
            for &i in &picked {
                let c = &cands[i];
                l.log(&format!(
                    "[pick] i={i} spec={} x={} y={} w={} h={} stones={} perimOpp={} innerNoStone={} penPerim={} penInner={} cost={}",
                    spec_name(c),
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

    Ok(picked)
}
