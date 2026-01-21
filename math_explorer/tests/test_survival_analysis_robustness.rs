use math_explorer::applied::clinical_trials::survival_analysis::{try_estimate_hazard_ratio, Observation};

#[test]
fn test_hazard_ratio_valid() {
    let group1 = vec![
        Observation { time: 10.0, event_occurred: true },
        Observation { time: 20.0, event_occurred: false },
    ];
    let group2 = vec![
        Observation { time: 5.0, event_occurred: true },
        Observation { time: 5.0, event_occurred: true },
    ];

    // HR = (1 event / 30 time) / (2 events / 10 time) = 0.0333... / 0.2 = 0.1666...
    let hr = try_estimate_hazard_ratio(&group1, &group2).unwrap();
    assert!((hr - 0.166666).abs() < 1e-4);
}

#[test]
fn test_hazard_ratio_error() {
     let group1 = vec![
        Observation { time: 10.0, event_occurred: true },
    ];
    let group_empty = vec![];

    let res = try_estimate_hazard_ratio(&group1, &group_empty);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Group 2 total time is zero or negative");
}

#[test]
fn test_hazard_ratio_negative_time() {
     let group1 = vec![
        Observation { time: -10.0, event_occurred: true },
    ];
    let group2 = vec![
         Observation { time: 10.0, event_occurred: true },
    ];

    let res = try_estimate_hazard_ratio(&group1, &group2);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Negative time values encountered");
}
