use super::bateman::BatemanModel;
use super::parameters::PKParameters;
use super::superposition::SuperpositionModel;
use super::traits::PharmacokineticModel;
use super::two_pulse::TwoPulseModel;

/// A model for a drug composed of two enantiomers (e.g., d- and l-amphetamine).
#[derive(Debug, Clone, Copy)]
pub struct EnantiomerModel {
    /// Parameters for the d-enantiomer. The dose `d` should be the total dose of the mixture.
    pub d_params: PKParameters,
    /// Parameters for the l-enantiomer. The dose `d` should be the total dose of the mixture.
    pub l_params: PKParameters,
    /// Fraction of d-enantiomer in the dose (e.g., 0.75 for Adderall).
    pub f_d: f64,
    /// Fraction of l-enantiomer in the dose (e.g., 0.25 for Adderall).
    pub f_l: f64,
}

impl EnantiomerModel {
    fn get_d_model(&self) -> BatemanModel {
        let dose = self.d_params.d() * self.f_d;
        // We assume f_d is non-negative. If dose becomes negative, with_dose returns Error.
        // Since we can't easily propagate error from here (trait signature), we unwrap.
        // This implies EnantiomerModel should be constructed carefully.
        let params = self
            .d_params
            .with_dose(dose)
            .expect("Invalid dose calculated in EnantiomerModel");
        BatemanModel::new(params)
    }

    fn get_l_model(&self) -> BatemanModel {
        let dose = self.l_params.d() * self.f_l;
        let params = self
            .l_params
            .with_dose(dose)
            .expect("Invalid dose calculated in EnantiomerModel");
        BatemanModel::new(params)
    }
}

impl PharmacokineticModel for EnantiomerModel {
    fn concentration(&self, t: f64) -> f64 {
        self.get_d_model().concentration(t) + self.get_l_model().concentration(t)
    }
}

impl EnantiomerModel {
    /// Calculates the total concentration of both enantiomers at time `t` for a single IR dose.
    pub fn concentration_ir_single_dose(&self, t: f64) -> f64 {
        self.concentration(t)
    }

    /// Calculates the total concentration for multiple IR doses using superposition.
    pub fn concentration_ir_multiple_doses(&self, dose_times: &[f64], t: f64) -> f64 {
        let model = SuperpositionModel::new(*self, dose_times.to_vec());
        model.concentration(t)
    }

    /// Calculates the total concentration for a single XR dose using the two-pulse model.
    pub fn concentration_xr_single_dose(&self, lag_time: f64, f1: f64, f2: f64, t: f64) -> f64 {
        let model = TwoPulseModel::new(*self, lag_time, f1, f2);
        model.concentration(t)
    }

    /// Calculates the total concentration for multiple XR doses using superposition.
    pub fn concentration_xr_multiple_doses(
        &self,
        dose_times: &[f64],
        lag_time: f64,
        f1: f64,
        f2: f64,
        t: f64,
    ) -> f64 {
        let xr_model = TwoPulseModel::new(*self, lag_time, f1, f2);
        let model = SuperpositionModel::new(xr_model, dose_times.to_vec());
        model.concentration(t)
    }
}
