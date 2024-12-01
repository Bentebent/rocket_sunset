#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();

        //Passing
        t.pass("tests/build/basic.rs");

        //Failing
        t.compile_fail("tests/build/invalid_ident.rs");
        t.compile_fail("tests/build/invalid_time_format.rs");
        t.compile_fail("tests/build/invalid_sunset_format.rs");
        t.compile_fail("tests/build/sunset_before_deprecation.rs");
        t.compile_fail("tests/build/missing_deprecation.rs");
    }
}
