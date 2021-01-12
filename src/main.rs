use autocxx::include_cpp;

include_cpp! {
    //#include "PrusaSlicer/src/libslic3r/Model.hpp"
    //#include "PrusaSlicer/src/libslic3r/Print.hpp"
    //#include "PrusaSlicer/src/libslic3r/Point.hpp"
    #include "PrusaSlicer/src/libslic3r/PrintConfig.hpp"

    safety!(unsafe)

    //generate!("Print")
    generate!("Slic3r::DynamicPrintConfig")
    //generate!("Slic3r::Model")
}

fn main() {
    //println!("{:?}", ffi::Model);
    //ffi::Slic3r::PrintConfig();
    //let config = ffi::DynamicPrintConfig();
    //let model = ffi::Model::read_from_file("/tmp/cupsTest/rust.stl");
}