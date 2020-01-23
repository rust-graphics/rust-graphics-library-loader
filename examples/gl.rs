extern crate rust_graphics_library_loader as loader;

fn main() {
    #[cfg(target_os = "windows")]
    let library_name = "opengl32.dll";
    #[cfg(target_os = "linux")]
    let library_name = "libGL.so";
    #[cfg(target_os = "macos")]
    let library_name = "/System/Library/Frameworks/OpenGL.framework/Libraries/libGL.dylib";

    let linker = loader::Linker::new(library_name).expect(&format!("Can not find {} library.", library_name));
    let bind_vertex_array: Option<extern "C" fn(u32)> = linker.get_function("glBindVertexArray");
    if bind_vertex_array.is_some() {
        println!("bind_vertex_array found");
    } else {
        println!("bind_vertex_array not found");
    }
}
