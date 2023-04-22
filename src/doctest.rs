#[allow(unused_doc_comments)]
// https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790
#[cfg(test)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
