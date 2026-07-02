import init, { run_simulation_worker } from './math_explorer_gui.js';

self.onmessage = async (e) => {
    if (e.data.type === 'init') {
        await init(e.data.module, e.data.memory);
        run_simulation_worker(e.data.ptr);
    }
};
