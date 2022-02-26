//use anyhow::Result;
use anyhow::Result;
//use shaderc;
//use walkdir::WalkDir;

fn main() {
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=vulkan");
    }
    //compile_shaders().expect("failed to compile shaders");
}

//fn compile_shaders() -> Result<()> {
//    let mut compiler = shaderc::Compiler::new().unwrap();
//    let options = shaderc::CompileOptions::new().unwrap();
//
//    for entry in WalkDir::new("./assets/shaders")
//        .into_iter()
//        .filter_map(|e| e.ok())
//        .filter(|e| e.file_type().is_file())
//    {
//        let path = entry.path();
//        let shaderkind;
//        let out_extension;
//
//        match path.extension().and_then(|p| p.to_str()) {
//            Some("frag") => {
//                shaderkind = shaderc::ShaderKind::Fragment;
//                out_extension = "frag.spv";
//            }
//            Some("vert") => {
//                shaderkind = shaderc::ShaderKind::Vertex;
//                out_extension = "vert.spv";
//            }
//            _ => continue,
//        }
//
//        let source = std::fs::read_to_string(path).expect(&format!("failed to read {:?}", path));
//
//        let compiled = compiler.compile_into_spirv(
//            &source,
//            shaderkind,
//            &path.file_name().unwrap().to_string_lossy(),
//            "main",
//            Some(&options),
//        );
//
//        match compiled {
//            Ok(spirv) => {
//                let outpath = path.with_extension(out_extension);
//
//                std::fs::write(outpath, spirv.as_binary_u8()).expect("failed to write spirv");
//            }
//            Err(err) => {
//                eprintln!("failed to compile {:?}: {:?}", path, err);
//            }
//        }
//    }
//    Ok(())
//}
//
