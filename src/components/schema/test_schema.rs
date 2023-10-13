use super::get_schema_options;
use rand::{random, thread_rng, Rng};
use wasm_bindgen_test::wasm_bindgen_test;
use yew::virtual_dom::VNode;

#[wasm_bindgen_test]
fn test_get_schema_options_with_happy_path() {
    let mut options = Vec::new();
    let mut rand_num_generator = thread_rng();
    let rand_n: u32 = rand_num_generator.gen_range(1..20);
    let n = rand_n as usize;
    let mut counts: Vec<u32> = Vec::new();
    for _ in 0..n {
        options.push(format!("option{}", random::<u32>()));
        counts.push(0);
    }
    let v_node_list = get_schema_options(options.clone());
    for (i, v_node) in v_node_list.iter().enumerate() {
        let tag = match v_node {
            VNode::VTag(x) => x,
            _ => panic!("node is not an html tag node"),
        };
        let value = match tag.children().iter().next().unwrap() {
            VNode::VText(v_text) => String::from(v_text.text.as_str()),
            _ => panic!("node is not an html text node"),
        };
        assert_eq!(options[i], value);
    }
}

#[wasm_bindgen_test]
fn test_get_schema_options_with_empty_list() {
    let options: Vec<String> = Vec::new();
    let v_node_list = get_schema_options(options);
    assert_eq!(v_node_list.iter().count(), 0);
}
