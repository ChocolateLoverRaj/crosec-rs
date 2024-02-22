use crosec_rs::commands::hello::ec_cmd_hello;
use crosec_rs::commands::version::ec_cmd_version;

fn main() {
    println!("Hello command");
    let status = ec_cmd_hello();
    if status == true {
        println!("EC says hello!");
    } else {
        println!("EC did not say hello :(");
    }

    println!("Version command");
    let (ro_ver, rw_ver, firmware_copy, build_info, tool_version) =  ec_cmd_version();
    println!("RO version:    {ro_ver}");
    println!("RW version:    {rw_ver}");
    println!("Firmware copy: {firmware_copy}");
    println!("Build info:    {build_info}");
    println!("Tool version:  {tool_version}")
}
