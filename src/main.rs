extern crate glfw;

use glfw::{Action, Context, Key};
use std::sync::mpsc::{channel, Receiver};
use std::thread::Builder;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    let render_context = window.render_context();
        let (send, recv) = channel();

        let render_task = Builder::new().name("render task".to_string());
        let render_task_done = render_task.spawn(move || {
            render(render_context, recv);
    });

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    // Tell the render task to exit.
    send.send(()).ok().expect("Failed signal to render thread.");
    // Wait for acknowledgement that the rendering was completed.
    let _ = render_task_done;
}

fn render(mut context: glfw::RenderContext, finish: Receiver<()>) {
    context.make_current();
    loop {
        // Check if the rendering should stop.
        if finish.try_recv() == Ok(()) { break };

        // Perform rendering calls

        context.swap_buffers();
    }

    // required on some platforms
    glfw::make_context_current(None);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
