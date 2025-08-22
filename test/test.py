import numpy as np
import moderngl
import glfw

def magnetic_field_cable_gpu(current, cable_pos, cable_length=2.0, num_points=20, z_layers=3, radius=1.0):
    all_points = []
    all_colors = []
    cable_x, cable_y, cable_z = cable_pos

    z_layer_offsets = np.linspace(-cable_length / 2, cable_length / 2, z_layers)

    for z_offset in z_layer_offsets:
        for angle in np.linspace(0, 2 * np.pi, num_points):
            x = cable_x + radius * np.cos(angle)
            y = cable_y + radius * np.sin(angle)
            z = cable_z + z_offset

            dx = -np.sin(angle)
            dy = np.cos(angle)
            dz = 0

            magnitude = np.sqrt(dx**2 + dy**2 + dz**2)
            if magnitude > 0:
                dx /= magnitude
                dy /= magnitude
                dz /= magnitude

            end_x = x + dx * 0.3
            end_y = y + dy * 0.3
            end_z = z + dz * 0.3

            all_points.extend([x, y, z, end_x, end_y, end_z])
            if current > 0:
                all_colors.extend([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
            else:
                all_colors.extend([0.0, 0.0, 1.0, 0.0, 0.0, 1.0])

    return np.array(all_points, dtype='f4'), np.array(all_colors, dtype='f4')

def render():
    if not glfw.init():
        return

    window = glfw.create_window(800, 600, "Magnetic Field (GPU)", None, None)
    if not window:
        glfw.terminate()
        return

    glfw.make_context_current(window)
    ctx = moderngl.create_context()

    points, colors = magnetic_field_cable_gpu(2.0, (0.0, 0.0, 0.0))

    prog = ctx.program(
        vertex_shader="""
            #version 330 core
            in vec3 in_vert;
            in vec3 in_color;
            out vec3 v_color;
            uniform mat4 mvp;
            void main() {
                gl_Position = mvp * vec4(in_vert, 1.0);
                v_color = in_color;
            }
        """,
        fragment_shader="""
            #version 330 core
            in vec3 v_color;
            out vec4 f_color;
            void main() {
                f_color = vec4(v_color, 1.0);
            }
        """,
    )

    vbo_vert = ctx.buffer(points)
    vbo_color = ctx.buffer(colors)

    vao = ctx.vertex_array(
        prog,
        [(vbo_vert, "3f", "in_vert"), (vbo_color, "3f", "in_color")],
    )

    # Projection and view matrices (modern way)
    perspective = np.array([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, -1.0, -1.0],
        [0.0, 0.0, -1.0, 0.0]
    ], dtype='f4') # simple perspective projection.

    lookat = np.array([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-3.0, -3.0, -3.0, 1.0]
    ], dtype='f4') # simple look at, from 3,3,3

    mvp = perspective @ lookat

    prog["mvp"].write(mvp)

    while not glfw.window_should_close(window):
        ctx.clear(0.0, 0.0, 0.0)
        vao.render(moderngl.LINES)
        glfw.swap_buffers(window)
        glfw.poll_events()

        # Update view matrix (you can add camera controls here)
        lookat = np.array([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-3.0, -3.0, -3.0, 1.0]
        ], dtype='f4')
        mvp = perspective @ lookat
        prog["mvp"].write(mvp)

    glfw.terminate()

if __name__ == "__main__":
    render()