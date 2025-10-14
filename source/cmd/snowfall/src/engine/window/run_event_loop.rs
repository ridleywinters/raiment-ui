use super::internal::*;

pub fn run_event_loop(engine: Arc<Engine>) {
    let event_loop = {
        let mut builder = EventLoop::builder();

        if engine.development_mode {
            // This ensures when the new window is created, it will **not** steal focus,
            // which is very useful for auto-recompiling so the focus is not stolen from
            // the code editor.
            #[cfg(target_os = "macos")]
            {
                use winit::platform::macos::EventLoopBuilderExtMacOS;
                builder.with_activate_ignoring_other_apps(false);
            }
        }

        builder.build().unwrap()
    };
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = Application::new(engine.clone());
    event_loop.run_app(&mut app).unwrap();
}
