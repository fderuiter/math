#![allow(warnings)]
//! Comprehensive demonstration of all sports analytics modules.

use math_explorer::pure_math::statistics::{
    copula::{Correlation, CorrelationMatrix, Probability, sgp_joint_probability},
    glicko2::{
        GlickoPlayer, MatchResult, Rating, RatingDeviation, SystemConstant, Volatility,
        update_rating,
    },
    kelly::{EdgeProbability, Odds, kelly_fraction},
    markov::dtmc::{MarkovChain, StateType},
    ou_process::{EulerMaruyama, OuParams, TimeStep},
    zip_regression::{Count, ZipDistribution, ZipParams},
};
use nalgebra::{DMatrix, DVector};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn main() {
    println!("=== Sports Analytics Framework Demo ===\n");

    // 1. ZIP Regression: Model player blocks (discrete counts with excess zeros)
    println!("1. ZIP REGRESSION - Player Block Modeling");
    let zip_params = ZipParams::from_values(0.25, 1.5).unwrap();
    let zip_dist = ZipDistribution::new(zip_params);
    println!("   Zero-inflation: 25%, Poisson rate: 1.5");
    println!("   P(0 blocks) = {:.3}", zip_dist.pmf(Count::new(0)));
    println!(
        "   Mean: {:.2}, Variance: {:.2} (overdispersed!)",
        zip_dist.mean(),
        zip_dist.variance()
    );

    // 2. OU Process: Model shooting percentage momentum
    println!("\n2. OU PROCESS - Shooting Percentage Momentum");
    let ou_params = OuParams::from_values(0.45, 1.0, 0.15).unwrap();
    let dt = TimeStep::new(0.01).unwrap();
    let solver = EulerMaruyama::new(ou_params, dt);
    let mut rng = StdRng::seed_from_u64(42);
    let trajectory = solver.simulate(0.60, 100, &mut rng);
    println!("   True skill: 45%, Currently: 60% (hot)");
    println!(
        "   After 100 possessions: {:.1}%",
        trajectory.last().unwrap() * 100.0
    );

    // 3. Copula: Same Game Parlay pricing
    println!("\n3. GAUSSIAN COPULA - Same Game Parlay");
    let p_star_50 = Probability::new(0.99).unwrap();
    let p_win = Probability::new(0.60).unwrap();
    let rho = Correlation::new(-0.15).unwrap();
    let corr_matrix = CorrelationMatrix::bivariate(rho).unwrap();
    let joint = sgp_joint_probability(&[p_star_50, p_win], &corr_matrix).unwrap();
    println!("   Event A: Star scores 50+ (99th percentile)");
    println!("   Event B: Team wins (60% base)");
    println!("   Naive: {:.4}, Copula: {:.4}", 0.99 * 0.60, joint.value());

    // 4. Glicko-2: Team rating update
    println!("\n4. GLICKO-2 RATING - Team Ranking");
    let mut player = GlickoPlayer::new(
        Rating::new(1500.0).unwrap(),
        RatingDeviation::new(200.0).unwrap(),
        Volatility::new(0.06).unwrap(),
    );
    let opponent_rating = Rating::new(1400.0).unwrap();
    let opponent_rd = RatingDeviation::new(30.0).unwrap();
    // Create an opponent player with the given rating/RD and default volatility
    let opponent = GlickoPlayer::new(opponent_rating, opponent_rd, Volatility::default());
    // Create match result (win = 1.0)
    let results = vec![MatchResult::new(opponent, 1.0).unwrap()];
    // Update rating with system constant tau = 0.5
    let tau = SystemConstant::new(0.5).unwrap();
    player = update_rating(&player, &results, &tau).unwrap();
    println!("   Before: R=1500, RD=200");
    println!(
        "   After win: R={:.0}, RD={:.0}",
        player.rating.value(),
        player.rating_deviation.value()
    );

    // 5. Kelly Criterion: Optimal bet sizing
    println!("\n5. KELLY CRITERION - Bet Sizing");
    let prob = EdgeProbability::new(0.55).unwrap();
    let odds = Odds::new(2.0).unwrap();
    let kelly = kelly_fraction(&prob, &odds).unwrap();
    println!("   Win probability: 55%, Odds: 2.0");
    println!("   Full Kelly: {:.1}%", kelly.value() * 100.0);
    println!(
        "   Quarter Kelly: {:.1}% (recommended)",
        kelly.value() * 25.0
    );

    // 6. Markov Chain: Expected Possession Value
    println!("\n6. MARKOV CHAIN - Expected Possession Value");
    let states = vec![
        StateType::Transient,
        StateType::Transient,
        StateType::Absorbing,
        StateType::Absorbing,
    ];
    let p = DMatrix::from_row_slice(
        4,
        4,
        &[
            0.3, 0.5, 0.1, 0.1, 0.2, 0.4, 0.3, 0.1, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
    // Correct order: Matrix, then State Types
    let chain = MarkovChain::new(p, states).unwrap();
    let rewards = DVector::from_vec(vec![3.0, 0.0]);
    let epv = chain.expected_possession_value(&rewards).unwrap();
    println!("   State 0 EPV: {:.2} points", epv[0]);
    println!("   State 1 EPV: {:.2} points", epv[1]);

    println!("\n✅ All 6 sports analytics modules demonstrated successfully!");
}
