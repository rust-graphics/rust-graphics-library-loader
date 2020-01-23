extern crate rust_graphics_library_loader as loader;

fn main() {
    #[cfg(target_os = "windows")]
    let library_name = "opengl32.dll";
    #[cfg(not(target_os = "windows"))]
    let library_name = "libGL.so";

    let linker = loader::Linker::new(library_name).expect("Can not find GL library.");
    let bind_vertex_array: Option<extern "C" fn(u32)> = linker.get_function("glBindVertexArray");
    if bind_vertex_array.is_some() {
        println!("bind_vertex_array found");
    } else {
        println!("bind_vertex_array not found");
    }
}
