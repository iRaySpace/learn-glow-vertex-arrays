use glow::{Context, HasContext, NativeBuffer, NativeVertexArray};
use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

fn main() {
    unsafe {
        let (event_loop, windowed, gl) = create_windowed_context("Hello, Triangle 3");
        let (vao, vbo) = create_vertex_array(&gl);

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    windowed.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);
                    windowed.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        windowed.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        gl.delete_vertex_array(vao);
                        gl.delete_buffer(vbo);
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

unsafe fn create_windowed_context(
    title: &str,
) -> (
    EventLoop<()>,
    ContextWrapper<PossiblyCurrent, Window>,
    Context,
) {
    let window_builder = WindowBuilder::new().with_title(title);
    let event_loop = EventLoop::new();
    let windowed = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .unwrap()
        .make_current()
        .unwrap();
    let gl = Context::from_loader_function(|s| windowed.get_proc_address(s) as *const _);
    (event_loop, windowed, gl)
}

unsafe fn create_vertex_array(gl: &Context) -> (NativeVertexArray, NativeBuffer) {
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));

    let triangle_vertices = [
        -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32,
    ];
    let triangle_vertices_u8: &[u8] = std::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * 4,
    );

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);

    // TODO: add colors

    (vao, vbo)
}
