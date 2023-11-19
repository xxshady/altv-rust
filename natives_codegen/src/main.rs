use std::{fs, io::Write};

use parser::parse;

mod cpp_generator;
mod helpers;
mod host_call_generator;
mod interface_generator;
mod parser;
mod pub_api_generator;
mod result_struct_generator;

fn main() {
    let natives = parse("natives.json", "https://natives.altv.mp/natives");
    dbg!(natives.len());

    let mut cpp_file = cpp_generator::prepare();
    let mut result_struct_file = result_struct_generator::prepare();
    // let mut pub_api_file = pub_api_generator::prepare(); // TODO: pub api wrappers

    // let natives = &natives[0..600];

    for native in &natives {
        // println!("{idx} {}", native.name);
        // if native.name != "NETWORK_CLAN_GET_EMBLEM_TXD_NAME" {
        //     continue;
        // }

        cpp_file
            .write_all(cpp_generator::gen(&native).as_bytes())
            .unwrap();

        result_struct_file
            .write_all(result_struct_generator::gen(&native).as_bytes())
            .unwrap();

        // pub_api_file
        //     .write_all(pub_api_generator::gen(&native).as_bytes())
        //     .unwrap();
    }

    cpp_file.write_all(b"}").unwrap();

    interface_generator::gen(&natives);
    host_call_generator::gen(&natives);
}
