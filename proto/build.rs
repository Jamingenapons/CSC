









fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prost_build = prost_build::Config::new();
    prost_build.btree_map(&["."]);
    prost_build.out_dir("src/proto");
    prost_build.compile_protos(&["message/hello_world.proto"], &["message"])?;

    Ok(())
}
