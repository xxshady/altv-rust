use std::io::Write;

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
    let mut pub_api_file = pub_api_generator::prepare();

    // let natives = natives
    //     .into_iter()
    //     .filter(|n| n.name == "get_dlc_weapon_data")
    //     .collect::<Vec<_>>();
    // let natives = natives
    //     .into_iter()
    //     .filter(|n| n.name == "datafile_get_file_dict_for_additional_data_file")
    //     .collect::<Vec<_>>();
    // dbg!(natives.len());

    for native in &natives {
        cpp_file
            .write_all(cpp_generator::gen(&native).as_bytes())
            .unwrap();

        result_struct_file
            .write_all(result_struct_generator::gen(&native).as_bytes())
            .unwrap();

        pub_api_file
            .write_all(pub_api_generator::gen(&native).as_bytes())
            .unwrap();
    }

    cpp_file.write_all(b"}").unwrap();

    interface_generator::gen(&natives);
    host_call_generator::gen(&natives);
}
