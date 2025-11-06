use tao::event_loop::EventLoop;

pub fn get_primary_monitor_info() -> (f64, u32, u32) {
    // Создаём временный event loop для доступа к мониторам
    let event_loop = EventLoop::new();

    if let Some(monitor) = event_loop.primary_monitor() {
        let scale_factor = monitor.scale_factor();
        let size = monitor.size();

        (scale_factor, size.width, size.height)
    } else {
        // Fallback если монитор не найден
        (1.0, 1920, 1080)
    }
}
