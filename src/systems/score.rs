// Tower Defense Game - Score & Combo Systems
// Score calculation, combo tracking, star rating evaluation.

use bevy::prelude::*;

use crate::resources::*;

/// Ticks the combo decay timer each frame.
pub fn combo_tick_system(time: Res<Time>, mut combo: ResMut<ComboState>) {
    combo.tick(time.delta_secs());
}

/// Computes the star rating based on level performance.
pub fn compute_star_rating(base_hp: &BaseHealth, score: &ScoreState, economy: &Economy) -> StarRating {
    let mut stars: u8 = 0;

    // Star 1: Survived (base HP > 0)
    if base_hp.current > 0.0 {
        stars += 1;
    }

    // Star 2: Good HP (above 50%)
    if base_hp.fraction() >= 0.5 {
        stars += 1;
    }

    // Star 3: Efficiency – earned more than spent, and good score
    let efficiency = if economy.total_spent > 0 {
        economy.total_earned as f32 / economy.total_spent as f32
    } else {
        1.0
    };
    if efficiency >= 1.2 && score.points >= 200 {
        stars += 1;
    }

    match stars {
        0 => StarRating::Zero,
        1 => StarRating::One,
        2 => StarRating::Two,
        _ => StarRating::Three,
    }
}

/// Final score formula combining kill points, combo, base HP, and time.
pub fn compute_final_score(score: &ScoreState, base_hp: &BaseHealth, stats: &LevelStats) -> u32 {
    let base_score = score.points;
    let hp_bonus = (base_hp.fraction() * 100.0) as u32;
    let time_bonus = if stats.time_elapsed < 120.0 { 50 } else { 0 };
    let combo_bonus = stats.max_combo * 5;

    base_score + hp_bonus + time_bonus + combo_bonus
}

/// Tracks elapsed time for level statistics.
pub fn level_timer_system(time: Res<Time>, mut stats: ResMut<LevelStats>) {
    stats.time_elapsed += time.delta_secs();
}
