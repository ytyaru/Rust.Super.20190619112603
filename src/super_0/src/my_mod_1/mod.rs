pub fn assert() {
    // super::で先祖のprivate要素も参照可
    assert_eq!(super::public(), String::from("rust_use_2::public()"));
    assert_eq!(super::private(), String::from("rust_use_2::private()"));
}
