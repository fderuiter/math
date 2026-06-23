use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use std::thread;

pub enum SimCommand {
    Start,
    Pause,
    Reset,
    SetSpeed(usize),
    UpdateParam(String, f64),
    ApplyBrush {
        cx: i32,
        cy: i32,
        r: i32,
        is_obstacle: bool,
    },
    ClearObstacles,
}

pub struct StateSnapshot {
    pub width: usize,
    pub height: usize,
    pub pixels: Arc<Vec<eframe::egui::Color32>>,
    pub custom_data: Vec<f64>,
    pub structured_data: Option<Box<dyn std::any::Any + Send>>,
}

pub enum SimStateUpdate {
    Snapshot(StateSnapshot),
    Status { running: bool },
}

pub trait SimulationRunner: Send + 'static {
    fn process_command(&mut self, cmd: SimCommand);
    fn step(&mut self);
    fn get_snapshot(&self) -> StateSnapshot;
    fn get_steps_per_frame(&self) -> usize;
}

pub struct SimulationController {
    cmd_tx: Sender<SimCommand>,
    state_rx: Receiver<SimStateUpdate>,
    latest_snapshot: Option<StateSnapshot>,
    pub running: bool,
}

impl SimulationController {
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    pub fn new<T: SimulationRunner>(mut runner: T) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<SimCommand>();
        let (state_tx, state_rx) = mpsc::channel::<SimStateUpdate>();

        thread::spawn(move || {
            let mut running = false;
            let mut steps_per_frame = runner.get_steps_per_frame();

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
                            SimCommand::Start => {
                                running = true;
                                let _ = state_tx.send(SimStateUpdate::Status { running: true });
                            }
                            SimCommand::Pause => {
                                running = false;
                                let _ = state_tx.send(SimStateUpdate::Status { running: false });
                            }
                            SimCommand::SetSpeed(speed) => {
                                steps_per_frame = speed;
                                runner.process_command(SimCommand::SetSpeed(speed));
                            }
                            _ => {
                                runner.process_command(cmd);
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
                        runner.step();
                    }
                    let snapshot = runner.get_snapshot();
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
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new<T: SimulationRunner>(mut runner: T) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<SimCommand>();
        let (state_tx, state_rx) = mpsc::channel::<SimStateUpdate>();

        wasm_bindgen_futures::spawn_local(async move {
            let mut running = false;
            let mut steps_per_frame = runner.get_steps_per_frame();

            loop {
                // Yield to event loop
                let promise = js_sys::Promise::resolve(&wasm_bindgen::JsValue::NULL);
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                while let Ok(cmd) = cmd_rx.try_recv() {
                    match cmd {
                        SimCommand::Start => {
                            running = true;
                            let _ = state_tx.send(SimStateUpdate::Status { running: true });
                        }
                        SimCommand::Pause => {
                            running = false;
                            let _ = state_tx.send(SimStateUpdate::Status { running: false });
                        }
                        SimCommand::SetSpeed(speed) => {
                            steps_per_frame = speed;
                            runner.process_command(SimCommand::SetSpeed(speed));
                        }
                        _ => {
                            runner.process_command(cmd);
                        }
                    }
                }

                if running {
                    for _ in 0..steps_per_frame {
                        runner.step();
                    }
                    let snapshot = runner.get_snapshot();
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
        }
    }

    pub fn send_command(&self, cmd: SimCommand) {
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
// theory_verification!
