use nalgebra::DVector;

/// Time and proximity related parameters.
#[derive(Debug, Clone)]
pub struct TimeParams {
    /// Time period over which proximity is integrated.
    pub t: f64,
    /// Initial distance/proximity factor.
    pub x_0: f64,
}

impl Default for TimeParams {
    fn default() -> Self {
        Self {
            t: 365.0,
            x_0: 20.0,
        }
    }
}

/// Parameters related to gifts given.
#[derive(Debug, Clone)]
pub struct GiftParams {
    /// Emotional value of gifts.
    pub g_emotional: f64,
    /// Practical value of gifts.
    pub g_practical: f64,
}

impl Default for GiftParams {
    fn default() -> Self {
        Self {
            g_emotional: 5.0,
            g_practical: 2.0,
        }
    }
}

/// Parameters related to contact frequency and decay.
#[derive(Debug, Clone)]
pub struct ContactParams {
    /// Initial contact frequency factor.
    pub f_initial: f64,
    /// Decay constant for lack of contact.
    pub decay_constant: f64,
    /// Time since the last contact occurred.
    pub time_since_last_contact: f64,
}

impl Default for ContactParams {
    fn default() -> Self {
        Self {
            f_initial: 7.0,
            decay_constant: 0.05,
            time_since_last_contact: 7.0,
        }
    }
}

/// Parameters related to personality and success traits.
#[derive(Debug, Clone)]
pub struct PersonalityParams {
    /// Intelligence score.
    pub intelligence: f64,
    /// Emotional sensitivity score.
    pub emotional_sensitivity: f64,
    /// Wealth score.
    pub wealth: f64,
    /// Talent score.
    pub talent: f64,
    /// Weight for intelligence.
    pub w_i: f64,
    /// Weight for emotional sensitivity.
    pub w_es: f64,
    /// Weight for wealth.
    pub w_w: f64,
    /// Weight for talent.
    pub w_t: f64,
}

impl Default for PersonalityParams {
    fn default() -> Self {
        Self {
            intelligence: 7.0,
            emotional_sensitivity: 6.0,
            wealth: 9.0,
            talent: 8.0,
            w_i: 1.2,
            w_es: 1.5,
            w_w: 1.1,
            w_t: 1.3,
        }
    }
}

/// Parameters related to social behavior and life events.
#[derive(Debug, Clone)]
pub struct SocialParams {
    /// Weight assigned based on birth order (e.g., oldest, youngest).
    pub birth_order_weight: f64,
    /// Score for major life events shared.
    pub major_life_events: f64,
    /// Whether the child helped during a crisis.
    pub helped_during_crisis: bool,
    /// Whether the child is active on social media (visibility).
    pub active_on_social_media: bool,
}

impl Default for SocialParams {
    fn default() -> Self {
        Self {
            birth_order_weight: 1.2,
            major_life_events: 3.0,
            helped_during_crisis: true,
            active_on_social_media: true,
        }
    }
}

/// Parameters related to compliments given.
#[derive(Debug, Clone)]
pub struct ComplimentParams {
    /// Vector of compliment scores.
    pub compliments: DVector<f64>,
    /// Weights corresponding to each compliment.
    pub compliment_weights: DVector<f64>,
}

impl Default for ComplimentParams {
    fn default() -> Self {
        Self {
            compliments: DVector::from_vec(vec![10.0, 5.0, 8.0]),
            compliment_weights: DVector::from_vec(vec![1.0, 0.5, 0.75]),
        }
    }
}

/// Parameters related to family context.
#[derive(Debug, Clone)]
pub struct FamilyParams {
    /// Distances of siblings (used for the denominator).
    pub sibling_distances: Vec<f64>,
}

impl Default for FamilyParams {
    fn default() -> Self {
        Self {
            sibling_distances: vec![100.0, 50.0, 10.0],
        }
    }
}

/// Input parameters for the favoritism calculation.
#[derive(Debug, Clone, Default)]
pub struct FavoritismInputs {
    /// Time and proximity settings.
    pub time: TimeParams,
    /// Gift giving parameters.
    pub gifts: GiftParams,
    /// Contact and decay parameters.
    pub contact: ContactParams,
    /// Personality and success traits.
    pub personality: PersonalityParams,
    /// Social behavior and life events.
    pub social: SocialParams,
    /// Compliments and praise.
    pub compliments: ComplimentParams,
    /// Family context (siblings).
    pub family: FamilyParams,
}
