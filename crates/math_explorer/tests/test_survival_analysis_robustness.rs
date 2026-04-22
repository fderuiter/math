#![allow(warnings)]
use math_explorer::applied::clinical_trials::survival_analysis::{
    Observation, try_estimate_hazard_ratio,
};
use math_explorer::applied::clinical_trials::types::{ClinicalTrialError, SurvivalTime};

#[test]
fn test_hazard_ratio_valid() {
    let group1 = vec![
        Observation {
            time: SurvivalTime::new(10.0).unwrap(),
            event_occurred: true,
        },
        Observation {
            time: SurvivalTime::new(20.0).unwrap(),
            event_occurred: false,
        },
    ];
    let group2 = vec![
        Observation {
            time: SurvivalTime::new(5.0).unwrap(),
            event_occurred: true,
        },
        Observation {
            time: SurvivalTime::new(5.0).unwrap(),
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
        time: SurvivalTime::new(10.0).unwrap(),
        event_occurred: true,
    }];
    let group_empty = vec![];

    let res = try_estimate_hazard_ratio(&group1, &group_empty);
    assert!(res.is_err());
    match res.unwrap_err() {
        ClinicalTrialError::InvalidData(msg) => {
            assert_eq!(msg, "Group 2 total time is zero or negative");
        }
        e => panic!("Expected InvalidData, got {:?}", e),
    }
}

#[test]
fn test_hazard_ratio_negative_time() {
    // This test now verifies that we cannot construct an invalid SurvivalTime,
    // which prevents invalid Observations from being created in the first place.
    let res = SurvivalTime::new(-10.0);
    assert!(res.is_err());

    match res {
        Err(e) => assert!(e.to_string().contains("Time must be non-negative")),
        _ => panic!("Expected error for negative time"),
    }
}
