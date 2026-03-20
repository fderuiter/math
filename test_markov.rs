use math_explorer::pure_math::statistics::markov::hmm::HiddenMarkovModel;
use nalgebra::{DMatrix, DVector};

fn main() {
    let initial = DVector::from_vec(vec![0.5, 0.5]);
    let transitions = DMatrix::from_row_slice(2, 2, &[
        0.7, 0.3,  // Cold → Cold/Hot
        0.4, 0.6,  // Hot → Cold/Hot
    ]);
    let emissions = DMatrix::from_row_slice(2, 2, &[
        0.7, 0.3,  // Cold: P(Miss)=0.7, P(Make)=0.3
        0.2, 0.8,  // Hot: P(Miss)=0.2, P(Make)=0.8
    ]);
    let hmm = HiddenMarkovModel::new(initial, transitions, emissions).unwrap();
    println!("HMM states: {}", hmm.num_states());
}
