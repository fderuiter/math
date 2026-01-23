use math_explorer::applied::clinical_trials::survival_analysis::{
    Observation, try_estimate_hazard_ratio,
};
use math_explorer::applied::clinical_trials::SurvivalError;

#[test]
fn test_hazard_ratio_valid() {
    let group1 = vec![
        Observation {
            time: 10.0,
            event_occurred: true,
        },
        Observation {
            time: 20.0,
            event_occurred: false,
        },
    ];
    let group2 = vec![
        Observation {
            time: 5.0,
            event_occurred: true,
        },
        Observation {
            time: 5.0,
            event_occurred: true,
        },
    ];

    // HR = (1 event / 30 time) / (2 events / 10 time) = 0.0333... / 0.2 = 0.1666...
    let hr = try_estimate_hazard_ratio(&group1, &group2).unwrap();
    assert!((hr - 0.166666).abs() < 1e-4);
}

#[test]
fn test_hazard_ratio_error() {
    let group1 = vec![Observation {
        time: 10.0,
        event_occurred: true,
    }];
    let group_empty = vec![];

    let res = try_estimate_hazard_ratio(&group1, &group_empty);
    assert!(res.is_err());
    match res.unwrap_err() {
        SurvivalError::ZeroTotalTime(_) => {} // Expected
        err => panic!("Expected ZeroTotalTime, got {:?}", err),
    }
}

#[test]
fn test_hazard_ratio_negative_time() {
    let group1 = vec![Observation {
        time: -10.0,
        event_occurred: true,
    }];
    let group2 = vec![Observation {
        time: 10.0,
        event_occurred: true,
    }];

    let res = try_estimate_hazard_ratio(&group1, &group2);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), SurvivalError::NegativeTime);
}
