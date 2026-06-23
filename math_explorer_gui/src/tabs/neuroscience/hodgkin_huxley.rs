use super::NeuroscienceTool;
use crate::async_sim::declarative::{DeclarativeSimulation, DeclarativeTab, PlotData, PlotLine};
use crate::declare_params;
use crate::async_sim::StateSnapshot;
use crate::tabs::ExplorerTab;
use eframe::egui;
use math_explorer::biology::neuroscience::{HodgkinHuxleyNeuron, HodgkinHuxleyParameters};
use std::collections::VecDeque;
use std::sync::Arc;

declare_params! {
    pub struct HHParams {
        #[param(name = "g_Na (Sodium)", min = 0.0, max = 200.0)]
        pub g_na: f64,
        #[param(name = "g_K (Potassium)", min = 0.0, max = 100.0)]
        pub g_k: f64,
        #[param(name = "g_L (Leak)", min = 0.0, max = 1.0)]
        pub g_l: f64,
        #[param(name = "I_ext (Injected Current)", min = -10.0, max = 50.0)]
        pub i_ext: f64,
    }
}

pub struct HodgkinHuxleyRunner {
    neuron: HodgkinHuxleyNeuron,
    history: VecDeque<[f64; 2]>,
    time: f64,
    dt: f64,
    last_params: Option<HHParams>,
}

impl Default for HodgkinHuxleyRunner {
    fn default() -> Self {
        Self {
            neuron: HodgkinHuxleyNeuron::new(-65.0),
            history: VecDeque::new(),
            time: 0.0,
            dt: 0.01,
            last_params: None,
        }
    }
}

impl DeclarativeSimulation for HodgkinHuxleyRunner {
    type Params = HHParams;

    fn name(&self) -> &'static str {
        "Hodgkin-Huxley Model"
    }

    fn default_params(&self) -> Self::Params {
        HHParams {
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            i_ext: 10.0,
        }
    }

    fn param_descriptors(&self) -> Vec<crate::async_sim::declarative::ParamDescriptor<Self::Params>> {
        HHParams::descriptors()
    }

    fn setup(&mut self, _params: &Self::Params) {
        self.neuron = HodgkinHuxleyNeuron::new(-65.0);
        self.time = 0.0;
        self.history.clear();
        self.last_params = None;
    }

    fn step(&mut self, params: &Self::Params) {
        if self.last_params.as_ref() != Some(params) {
            let mut hh_params = HodgkinHuxleyParameters::default();
            hh_params.g_na = params.g_na;
            hh_params.g_k = params.g_k;
            hh_params.g_l = params.g_l;
            
            let current_v = self.neuron.v();
            let mut new_neuron = HodgkinHuxleyNeuron::try_new_with_params(current_v, hh_params).unwrap();
            let _ = new_neuron.set_n(self.neuron.n());
            let _ = new_neuron.set_m(self.neuron.m());
            let _ = new_neuron.set_h(self.neuron.h());
            self.neuron = new_neuron;
            self.last_params = Some(params.clone());
        }

        self.neuron.update(self.dt, params.i_ext);
        self.time += self.dt;

        self.history.push_back([self.time, self.neuron.v()]);
        if self.history.len() > 5000 {
            self.history.pop_front();
        }
    }

    fn get_snapshot(&self) -> StateSnapshot {
        let points: Vec<[f64; 2]> = self.history.iter().copied().collect();
        
        let plot_data = PlotData {
            lines: vec![
                PlotLine {
                    name: "Membrane Potential (mV)".to_string(),
                    points,
                    color: [255, 0, 0],
                }
            ],
        };

        StateSnapshot {
            width: 0,
            height: 0,
            pixels: Arc::new(Vec::new()),
            custom_data: Vec::new(),
            structured_data: Some(Box::new(plot_data) as Box<dyn std::any::Any + Send>),
        }
    }
}

pub struct HodgkinHuxleyTool {
    inner: DeclarativeTab<HodgkinHuxleyRunner>,
}

impl Default for HodgkinHuxleyTool {
    fn default() -> Self {
        Self {
            inner: DeclarativeTab::new(HodgkinHuxleyRunner::default(), 200),
        }
    }
}

impl NeuroscienceTool for HodgkinHuxleyTool {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn show(&mut self, ctx: &egui::Context) {
        self.inner.show_ctx(ctx);
    }
}
