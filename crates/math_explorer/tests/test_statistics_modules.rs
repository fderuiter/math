#![allow(warnings)]
//! Integration tests for new statistics modules

#[cfg(test)]
mod glicko2_tests {
    use math_explorer::pure_math::statistics::glicko2::{
        GlickoPlayer, MatchResult, Rating, RatingDeviation, SystemConstant, Volatility,
        update_rating,
    };

    #[test]
    fn test_glicko2_basic_update() {
        let player = GlickoPlayer::new(
            Rating::new(1500.0).unwrap(),
            RatingDeviation::new(200.0).unwrap(),
            Volatility::new(0.06).unwrap(),
        );

        let opponent = GlickoPlayer::new(
            Rating::new(1400.0).unwrap(),
            RatingDeviation::new(30.0).unwrap(),
            Volatility::new(0.06).unwrap(),
        );

        let results = vec![MatchResult::new(opponent, 1.0).unwrap()];
        let tau = SystemConstant::default();

        let updated = update_rating(&player, &results, &tau).unwrap();

        // Rating should increase after winning
        assert!(updated.rating.value() > player.rating.value());
        println!(
            "Glicko-2: Rating changed from {} to {}",
            player.rating.value(),
            updated.rating.value()
        );
    }
}

#[cfg(test)]
mod kelly_tests {
    use math_explorer::pure_math::statistics::kelly::{
        EdgeProbability, Odds, expected_value, kelly_fraction,
    };

    #[test]
    fn test_kelly_positive_edge() {
        let p = EdgeProbability::new(0.55).unwrap();
        let odds = Odds::new(2.0).unwrap();

        let ev = expected_value(&p, &odds);
        assert!(ev > 0.0);

        let kelly = kelly_fraction(&p, &odds).unwrap();
        assert!(kelly.value() > 0.0);
        assert!(kelly.value() < 1.0);
        println!("Kelly: Optimal fraction = {:.3}", kelly.value());
    }
}

#[cfg(test)]
mod tda_tests {
    use math_explorer::pure_math::statistics::tda::{
        Point2D, PointCloud, betti_numbers, vietoris_rips_complex,
    };

    #[test]
    fn test_tda_basic() {
        // Three points forming a triangle
        let points = vec![
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(0.5, 0.866),
        ];
        let cloud = PointCloud::new(points).unwrap();

        // Build complex at radius where all vertices are connected
        let complex = vietoris_rips_complex(&cloud, 1.0).unwrap();
        let (beta0, beta1) = betti_numbers(&complex).unwrap();

        // Should be one connected component
        assert_eq!(beta0, 1);
        println!("TDA: β₀ = {}, β₁ = {}", beta0, beta1);
    }
}
