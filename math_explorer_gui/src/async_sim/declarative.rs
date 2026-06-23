pub struct PlotData { pub lines: Vec<PlotLine> }
pub struct PlotLine { pub name: String, pub points: Vec<[f64; 2]>, pub color: [u8; 3] }
use crate::async_sim::{StateSnapshot, SimStateUpdate};
use crate::tabs::ExplorerTab;
use eframe::egui;
use std::sync::mpsc::{self, Receiver, Sender};
use egui_plot::{Legend, Line, Plot, PlotPoints};

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParamValue {
    F64(f64),
    Usize(usize),
    Bool(bool),
}

pub enum ParamType {
    F64 { min: f64, max: f64 },
    Usize { min: usize, max: usize },
    Bool,
}

pub trait ParamField {
    fn to_value(&self) -> ParamValue;
    fn from_value(&mut self, v: ParamValue);
    fn default_type(min: Option<Self>, max: Option<Self>) -> ParamType where Self: Sized;
}

impl ParamField for f64 {
    fn to_value(&self) -> ParamValue { ParamValue::F64(*self) }
    fn from_value(&mut self, v: ParamValue) { if let ParamValue::F64(val) = v { *self = val; } }
    fn default_type(min: Option<f64>, max: Option<f64>) -> ParamType {
        ParamType::F64 { min: min.unwrap_or(0.0), max: max.unwrap_or(1.0) }
    }
}

impl ParamField for usize {
    fn to_value(&self) -> ParamValue { ParamValue::Usize(*self) }
    fn from_value(&mut self, v: ParamValue) { if let ParamValue::Usize(val) = v { *self = val; } }
    fn default_type(min: Option<usize>, max: Option<usize>) -> ParamType {
        ParamType::Usize { min: min.unwrap_or(0), max: max.unwrap_or(100) }
    }
}

impl ParamField for bool {
    fn to_value(&self) -> ParamValue { ParamValue::Bool(*self) }
    fn from_value(&mut self, v: ParamValue) { if let ParamValue::Bool(val) = v { *self = val; } }
    fn default_type(_min: Option<bool>, _max: Option<bool>) -> ParamType {
        ParamType::Bool
    }
}

pub struct ParamDescriptor<P> {
    pub name: String,
    pub description: String,
    pub ptype: ParamType,
    pub update: Box<dyn Fn(&mut P, ParamValue) + Send + Sync>,
    pub read: Box<dyn Fn(&P) -> ParamValue + Send + Sync>,
}

#[macro_export]
macro_rules! declare_params {
    (
        $vis:vis struct $StructName:ident {
            $(
                #[param(name = $name:expr $(, min = $min:expr, max = $max:expr)?)]
                $field_vis:vis $field:ident : $ftype:ty
            ),* $(,)?
        }
    ) => {
        #[derive(Clone, PartialEq, Debug)]
        $vis struct $StructName {
            $(
                $field_vis $field: $ftype,
            )*
        }

        impl $StructName {
            pub fn descriptors() -> Vec<$crate::async_sim::declarative::ParamDescriptor<Self>> {
                vec![
                    $(
                        $crate::async_sim::declarative::ParamDescriptor {
                            name: $name.to_string(),
                            description: "".to_string(),
                            ptype: <$ftype as $crate::async_sim::declarative::ParamField>::default_type(
                                declare_params!(@opt $($min)?),
                                declare_params!(@opt $($max)?)
                            ),
                            update: Box::new(|p: &mut Self, v: $crate::async_sim::declarative::ParamValue| {
                                $crate::async_sim::declarative::ParamField::from_value(&mut p.$field, v);
                            }),
                            read: Box::new(|p: &Self| {
                                $crate::async_sim::declarative::ParamField::to_value(&p.$field)
                            }),
                        }
                    ),*
                ]
            }
        }
    };
    (@opt $val:expr) => { Some($val) };
    (@opt) => { None };
}

pub trait DeclarativeSimulation: Send + 'static {
    type Params: Clone + Send + Sync + 'static;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str { "" }
    
    fn default_params(&self) -> Self::Params;
    fn param_descriptors(&self) -> Vec<ParamDescriptor<Self::Params>>;
    
    fn setup(&mut self, params: &Self::Params);
    fn step(&mut self, params: &Self::Params);
    fn get_snapshot(&self) -> StateSnapshot;
}

pub enum DeclarativeCommand<P> {
    Start,
    Pause,
    Reset,
    SetSpeed(usize),
    UpdateParams(P),
}

pub struct DeclarativeController<P> {
    cmd_tx: Sender<DeclarativeCommand<P>>,
    state_rx: Receiver<SimStateUpdate>,
    latest_snapshot: Option<StateSnapshot>,
    pub running: bool,
    pub speed: usize,
    pub params: P,
}

impl<P: Clone + Send + 'static> DeclarativeController<P> {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<T: DeclarativeSimulation<Params = P>>(mut sim: T, initial_speed: usize) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<DeclarativeCommand<P>>();
        let (state_tx, state_rx) = mpsc::channel::<SimStateUpdate>();
        
        let params = sim.default_params();
        let mut bg_params = params.clone();
        
        sim.setup(&bg_params);

        thread::spawn(move || {
            let mut running = false;
            let mut steps_per_frame = initial_speed;

            loop {
                let mut cmd_opt = None;
                if running {
                    if let Ok(c) = cmd_rx.try_recv() {
                        cmd_opt = Some(c);
                    }
                } else {
                    if let Ok(c) = cmd_rx.recv() {
                        cmd_opt = Some(c);
                    } else {
                        break;
                    }
                }

                if let Some(mut cmd) = cmd_opt {
                    loop {
                        match cmd {
                            DeclarativeCommand::Start => {
                                running = true;
                                let _ = state_tx.send(SimStateUpdate::Status { running: true });
                            }
                            DeclarativeCommand::Pause => {
                                running = false;
                                let _ = state_tx.send(SimStateUpdate::Status { running: false });
                            }
                            DeclarativeCommand::SetSpeed(speed) => {
                                steps_per_frame = speed;
                            }
                            DeclarativeCommand::UpdateParams(p) => {
                                bg_params = p;
                            }
                            DeclarativeCommand::Reset => {
                                sim.setup(&bg_params);
                            }
                        }
                        if let Ok(c) = cmd_rx.try_recv() {
                            cmd = c;
                        } else {
                            break;
                        }
                    }
                }

                if running {
                    for _ in 0..steps_per_frame {
                        sim.step(&bg_params);
                    }
                    let snapshot = sim.get_snapshot();
                    if state_tx.send(SimStateUpdate::Snapshot(snapshot)).is_err() {
                        break;
                    }
                }
            }
        });

        Self {
            cmd_tx,
            state_rx,
            latest_snapshot: None,
            running: false,
            speed: initial_speed,
            params,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new<T: DeclarativeSimulation<Params = P>>(mut sim: T, initial_speed: usize) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<DeclarativeCommand<P>>();
        let (state_tx, state_rx) = mpsc::channel::<SimStateUpdate>();
        
        let params = sim.default_params();
        let mut bg_params = params.clone();
        
        sim.setup(&bg_params);

        wasm_bindgen_futures::spawn_local(async move {
            let mut running = false;
            let mut steps_per_frame = initial_speed;

            loop {
                let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::NULL);
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        DeclarativeCommand::Start => {
                            running = true;
                            let _ = state_tx.send(SimStateUpdate::Status { running: true });
                        }
                        DeclarativeCommand::Pause => {
                            running = false;
                            let _ = state_tx.send(SimStateUpdate::Status { running: false });
                        }
                        DeclarativeCommand::SetSpeed(speed) => {
                            steps_per_frame = speed;
                        }
                        DeclarativeCommand::UpdateParams(p) => {
                            bg_params = p;
                        }
                        DeclarativeCommand::Reset => {
                            sim.setup(&bg_params);
                        }
                    }
                }

                if running {
                    for _ in 0..steps_per_frame {
                        sim.step(&bg_params);
                    }
                    let snapshot = sim.get_snapshot();
                    if state_tx.send(SimStateUpdate::Snapshot(snapshot)).is_err() {
                        break;
                    }
                }
            }
        });

        Self {
            cmd_tx,
            state_rx,
            latest_snapshot: None,
            running: false,
            speed: initial_speed,
            params,
        }
    }

    pub fn send_command(&self, cmd: DeclarativeCommand<P>) {
        let _ = self.cmd_tx.send(cmd);
    }

    pub fn update(&mut self) -> Option<&StateSnapshot> {
        while let Ok(update) = self.state_rx.try_recv() {
            match update {
                SimStateUpdate::Snapshot(snapshot) => {
                    self.latest_snapshot = Some(snapshot);
                }
                SimStateUpdate::Status { running } => {
                    self.running = running;
                }
            }
        }
        self.latest_snapshot.as_ref()
    }
}

pub struct DeclarativeTab<T: DeclarativeSimulation> {
    name: &'static str,
    description: &'static str,
    controller: DeclarativeController<T::Params>,
    descriptors: Vec<ParamDescriptor<T::Params>>,
    texture: Option<egui::TextureHandle>,
}

impl<T: DeclarativeSimulation> DeclarativeTab<T> {
    pub fn new(sim: T, initial_speed: usize) -> Self {
        let name = sim.name();
        let description = sim.description();
        let descriptors = sim.param_descriptors();
        let controller = DeclarativeController::new(sim, initial_speed);
        
        Self {
            name,
            description,
            controller,
            descriptors,
            texture: None,
        }
    }

    pub fn show_ctx(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left(format!("{}_controls", self.name)).show(ctx, |ui| {
            ui.heading(self.name);
            
            ui.separator();

            ui.collapsing("Parameters", |ui| {
                let mut changed = false;
                for desc in &self.descriptors {
                    match desc.ptype {
                        ParamType::F64 { min, max } => {
                            if let ParamValue::F64(mut val) = (desc.read)(&self.controller.params) {
                                if ui.add(egui::Slider::new(&mut val, min..=max).text(&desc.name)).changed() {
                                    (desc.update)(&mut self.controller.params, ParamValue::F64(val));
                                    changed = true;
                                }
                            }
                        }
                        ParamType::Usize { min, max } => {
                            if let ParamValue::Usize(mut val) = (desc.read)(&self.controller.params) {
                                if ui.add(egui::Slider::new(&mut val, min..=max).text(&desc.name)).changed() {
                                    (desc.update)(&mut self.controller.params, ParamValue::Usize(val));
                                    changed = true;
                                }
                            }
                        }
                        ParamType::Bool => {
                            if let ParamValue::Bool(mut val) = (desc.read)(&self.controller.params) {
                                if ui.checkbox(&mut val, &desc.name).changed() {
                                    (desc.update)(&mut self.controller.params, ParamValue::Bool(val));
                                    changed = true;
                                }
                            }
                        }
                    }
                }
                
                if changed {
                    self.controller.send_command(DeclarativeCommand::UpdateParams(self.controller.params.clone()));
                }
            });

            ui.separator();
            if ui.add(egui::Slider::new(&mut self.controller.speed, 1..=50).text("Speed (steps/frame)")).changed() {
                self.controller.send_command(DeclarativeCommand::SetSpeed(self.controller.speed));
            }

            let pause_btn = ui.button(if !self.controller.running { "▶ Resume" } else { "⏸ Pause" });
            if pause_btn.clicked() {
                if self.controller.running {
                    self.controller.send_command(DeclarativeCommand::Pause);
                } else {
                    self.controller.send_command(DeclarativeCommand::Start);
                }
            }

            if ui.button("↻ Reset / Randomize").clicked() {
                 self.controller.send_command(DeclarativeCommand::Reset);
            }

            if !self.description.is_empty() {
                ui.separator();
                ui.label("Description:");
                ui.label(self.description);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.controller.running {
                ui.ctx().request_repaint();
            }

            if let Some(snapshot) = self.controller.update() {
                if let Some(any_data) = &snapshot.structured_data {
                    if let Some(plot_data) = any_data.downcast_ref::<PlotData>() {
                        Plot::new(format!("{}_egui_plot", self.name))
                            .legend(Legend::default())
                            .view_aspect(2.0)
                            .show(ui, |plot_ui| {
                                for line_data in &plot_data.lines {
                                    plot_ui.line(
                                        Line::new(&line_data.name, PlotPoints::new(line_data.points.clone()))
                                            .color(egui::Color32::from_rgb(line_data.color[0], line_data.color[1], line_data.color[2]))
                                            .width(2.0_f32),
                                    );
                                }
                            });
                        return;
                    }
                }
                
                if snapshot.width > 0 && snapshot.height > 0 {
                    let mut image = egui::ColorImage::default();
                    image.size = [snapshot.width, snapshot.height];
                    image.pixels = snapshot.pixels.as_ref().clone();
                    let texture = self.texture.get_or_insert_with(|| {
                        ui.ctx().load_texture(format!("{}_plot", self.name), image.clone(), Default::default())
                    });
                    texture.set(image, Default::default());
                }
            }

            if let Some(texture) = &self.texture {
                ui.image((texture.id(), texture.size_vec2()));
            }
        });
    }
}

impl<T: DeclarativeSimulation> ExplorerTab for DeclarativeTab<T> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_ctx(ctx);
    }
}
