💡 What: Replaced `Vec` with `std::collections::VecDeque` in `SpikeAnalysisTool` for managing `history` and `isis` rolling windows.

🎯 Why: In the highly frequent per-frame UI rendering loops, maintaining a rolling window using `Vec::remove(0)` causes severe O(N) memory shifting bottlenecks, especially when the history vector grows up to 20,000 points. Replacing it with `VecDeque::pop_front()` achieves this element removal in O(1) time, drastically improving rendering performance during simulation.

📊 Impact: Converts an O(N) array shifting operation that occurs on every simulated spike/step into an O(1) operation. Significantly reduces CPU overhead and stuttering during spike analysis execution.

🔬 Measurement: Run the application, open the "Spike Train Analysis" tool, set "Simulation Speed" to maximum (100), and click "▶ Run". Observe CPU usage profiling and UI responsiveness. The UI frame times should be noticeably more stable compared to using `Vec::remove(0)`.
