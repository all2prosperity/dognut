fn main() {
    println!("hello build ------------------------------------------------!");
    prost_build::compile_protos(&["pb/debugger.proto"], 
                                &["pb/"]).unwrap();
}
