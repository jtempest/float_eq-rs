#![cfg(feature = "derive")]

#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    t.pass("tests/derive_tests/derive_all/derive_all.rs");
    t.pass("tests/derive_tests/derive_all/derive_all_custom_ulps.rs");
    t.pass("tests/derive_tests/derive_all/tuple_derive_all.rs");
    t.pass("tests/derive_tests/derive_all/no_fields_derive_all.rs");
    t.pass("tests/derive_tests/derive_all/tuple_no_fields_derive_all.rs");
    t.pass("tests/derive_tests/derive_all/unit_derive_all.rs");

    t.compile_fail("tests/derive_tests/errors/float_eq_no_params_list.rs");
    t.compile_fail("tests/derive_tests/errors/float_eq_no_params.rs");
    t.compile_fail("tests/derive_tests/errors/float_eq_malformed_param.rs");
    t.compile_fail("tests/derive_tests/errors/float_eq_unknown_param.rs");
    t.compile_fail("tests/derive_tests/errors/ulps_missing.rs");
    t.compile_fail("tests/derive_tests/errors/ulps_duplicates.rs");
    t.compile_fail("tests/derive_tests/errors/ulps_duplicates_many_attrs.rs");
    t.compile_fail("tests/derive_tests/errors/float_ulps_enum.rs");
    t.compile_fail("tests/derive_tests/errors/float_eq_all_missing_abs_epsilon.rs");

    t.compile_fail("tests/derive_tests/generic/generic_derive_float_ulps.rs");
    t.compile_fail("tests/derive_tests/generic/generic_derive_float_diff.rs");
    t.compile_fail("tests/derive_tests/generic/generic_derive_float_eq.rs");
    t.compile_fail("tests/derive_tests/generic/generic_derive_float_eq_debug.rs");
    t.compile_fail("tests/derive_tests/generic/tuple_generic_derive_float_ulps.rs");
}
