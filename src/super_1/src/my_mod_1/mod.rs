pub fn public() -> String { String::from("super_1::my_mod_1::public()") }
fn private() -> String { String::from("super_1::my_mod_1::private()") }

pub fn assert() {
    // super::で先祖のprivate要素も参照可
    assert_eq!(super::public(), String::from("super_1::public()"));
    assert_eq!(super::private(), String::from("super_1::private()"));
}
