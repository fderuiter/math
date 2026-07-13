import init, { run_simulation_worker } from './math_explorer_gui.js';

self.onmessage = async (e) => {
    if (e.data.type === 'init') {
        await init(e.data.module, e.data.memory);
        await run_simulation_worker(e.data.ptr);
    } else if (e.data.type === 'wake') {
        if (typeof self.__sim_worker_wake === 'function') {
            self.__sim_worker_wake();
        }
    }
};
