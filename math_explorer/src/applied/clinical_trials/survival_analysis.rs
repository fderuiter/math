use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Observation {
    pub time: f64,
    pub event_occurred: bool, // true if event (e.g., death), false if censored
}

#[derive(Debug, Clone)]
pub struct TimePoint {
    pub time: f64,
    pub survival_probability: f64,
    pub n_at_risk: usize,
    pub n_events: usize,
    pub n_censored: usize,
}

/// Computes the Kaplan-Meier survival curve.
///
/// # Arguments
/// * `observations` - A list of observations (time, event status).
pub fn kaplan_meier(observations: &[Observation]) -> Vec<TimePoint> {
    let mut obs = observations.to_vec();
    // Sort by time. If times are equal, put events before censored (conservative).
    obs.sort_by(|a, b| {
        if (a.time - b.time).abs() < 1e-9 {
            // Equal time: events come first?
            // Standard KM handles ties. Usually event is counted against risk set.
            // If censored at T, they are in risk set at T? Yes.
            // If event at T, they are in risk set at T.
            // Order doesn't matter for risk set calculation if we group by time.
            Ordering::Equal
        } else if a.time < b.time {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut curve = Vec::new();
    let mut current_survival = 1.0;
    let total_subjects = obs.len();

    // Group by unique time points
    let mut i = 0;
    let mut n_at_risk = total_subjects;

    while i < obs.len() {
        let t = obs[i].time;
        let mut n_events = 0;
        let mut n_censored = 0;

        // Process all events at this time t
        while i < obs.len() && (obs[i].time - t).abs() < 1e-9 {
            if obs[i].event_occurred {
                n_events += 1;
            } else {
                n_censored += 1;
            }
            i += 1;
        }

        // KM Formula: S(t) = S(t-1) * (1 - d_i / n_i)
        // Only update if there are events. If only censored, S(t) stays same.
        if n_events > 0 {
            current_survival *= 1.0 - (n_events as f64 / n_at_risk as f64);
        }

        curve.push(TimePoint {
            time: t,
            survival_probability: current_survival,
            n_at_risk,
            n_events,
            n_censored,
        });

        // Update risk set for next time point
        n_at_risk -= n_events + n_censored;
    }

    curve
}

/// Calculates a simple Hazard Ratio estimate based on incidence density.
/// HR = (Events1 / TotalTime1) / (Events2 / TotalTime2)
/// Note: This assumes constant hazard (exponential distribution), which is a simplification.
pub fn estimate_hazard_ratio_simple(group1: &[Observation], group2: &[Observation]) -> f64 {
    let events1 = group1.iter().filter(|o| o.event_occurred).count() as f64;
    let time1: f64 = group1.iter().map(|o| o.time).sum();

    let events2 = group2.iter().filter(|o| o.event_occurred).count() as f64;
    let time2: f64 = group2.iter().map(|o| o.time).sum();

    if time1 == 0.0 || time2 == 0.0 || events2 == 0.0 {
        return f64::NAN; // Should return Result, but keeping signature simple for now
    }

    let rate1 = events1 / time1;
    let rate2 = events2 / time2;

    rate1 / rate2
}
