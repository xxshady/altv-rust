wai_bindgen_rust::import!("api.wai");

fn main() {
    println!("hello world!");

    let id = api::create_marker(0, 0., 0., 74.);
    println!("created marker id: {id:?}");
}
