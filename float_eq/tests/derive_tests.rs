#![cfg(feature = "derive")]

#[test]
fn tests() {
    let t = trybuild::TestCases::new();

    // FloatEqUlpsTol
    t.pass("tests/derive_tests/ulps_tol/ulps_tol_struct.rs");
    t.pass("tests/derive_tests/ulps_tol/ulps_tol_struct_custom_debug.rs");
    t.pass("tests/derive_tests/ulps_tol/ulps_tol_struct_no_fields.rs");
    t.pass("tests/derive_tests/ulps_tol/ulps_tol_tuple_struct.rs");
    t.pass("tests/derive_tests/ulps_tol/ulps_tol_unit.rs");
    t.compile_fail("tests/derive_tests/ulps_tol/ulps_tol_enum.rs");
    t.compile_fail("tests/derive_tests/ulps_tol/ulps_tol_generic.rs");
    t.compile_fail("tests/derive_tests/ulps_tol/ulps_tol_missing_type_name.rs");
    t.compile_fail("tests/derive_tests/ulps_tol/ulps_tol_duplicate_type_name.rs");

    // FloatEqDebugUlpsDiff
    t.pass("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_struct.rs");
    t.pass("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_struct_custom_debug.rs");
    t.pass("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_struct_no_fields.rs");
    t.pass("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_tuple_struct.rs");
    t.pass("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_unit.rs");
    t.compile_fail("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_enum.rs");
    t.compile_fail("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_generic.rs");
    t.compile_fail("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_missing_type_name.rs");
    t.compile_fail("tests/derive_tests/debug_ulps_diff/debug_ulps_diff_duplicate_type_name.rs");

    // FloatEq
    t.pass("tests/derive_tests/float_eq/float_eq_struct.rs");
    t.pass("tests/derive_tests/float_eq/float_eq_struct_no_fields.rs");
    t.pass("tests/derive_tests/float_eq/float_eq_tuple_struct.rs");
    t.pass("tests/derive_tests/float_eq/float_eq_unit.rs");
    t.compile_fail("tests/derive_tests/float_eq/float_eq_enum.rs");
    t.compile_fail("tests/derive_tests/float_eq/float_eq_generic.rs");

    // FloatEqAll
    t.pass("tests/derive_tests/float_eq_all/float_eq_all_struct.rs");
    t.pass("tests/derive_tests/float_eq_all/float_eq_all_struct_no_fields.rs");
    t.pass("tests/derive_tests/float_eq_all/float_eq_all_tuple_struct.rs");
    t.pass("tests/derive_tests/float_eq_all/float_eq_all_unit.rs");
    t.compile_fail("tests/derive_tests/float_eq_all/float_eq_all_enum.rs");
    t.compile_fail("tests/derive_tests/float_eq_all/float_eq_all_generic.rs");
    t.compile_fail("tests/derive_tests/float_eq_all/float_eq_all_missing_tol.rs");
    t.compile_fail("tests/derive_tests/float_eq_all/float_eq_all_duplicate_tol.rs");

    // AssertFloatEq
    t.pass("tests/derive_tests/assert_float_eq/assert_float_eq_struct.rs");
    t.pass("tests/derive_tests/assert_float_eq/assert_float_eq_struct_no_fields.rs");
    t.pass("tests/derive_tests/assert_float_eq/assert_float_eq_tuple_struct.rs");
    t.pass("tests/derive_tests/assert_float_eq/assert_float_eq_unit.rs");
    t.compile_fail("tests/derive_tests/assert_float_eq/assert_float_eq_enum.rs");
    t.compile_fail("tests/derive_tests/assert_float_eq/assert_float_eq_generic.rs");

    // AssertFloatEqAll
    t.pass("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_struct.rs");
    t.pass("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_struct_no_fields.rs");
    t.pass("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_tuple_struct.rs");
    t.pass("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_unit.rs");
    t.compile_fail("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_enum.rs");
    t.compile_fail("tests/derive_tests/assert_float_eq_all/assert_float_eq_all_generic.rs");

    // #[float_eq(...)]
    t.compile_fail("tests/derive_tests/float_eq_attribute/float_eq_no_params_list.rs");
    t.compile_fail("tests/derive_tests/float_eq_attribute/float_eq_malformed_param.rs");
    t.compile_fail("tests/derive_tests/float_eq_attribute/float_eq_malformed_value.rs");
    t.compile_fail("tests/derive_tests/float_eq_attribute/float_eq_unknown_param.rs");

    // #[derive_float_eq(...)]
    t.pass("tests/derive_tests/derive_float_eq/derive_float_eq.rs");
    t.pass("tests/derive_tests/derive_float_eq/derive_float_eq_all.rs");
    t.pass("tests/derive_tests/derive_float_eq/derive_float_eq_all_custom_debug.rs");
    t.compile_fail("tests/derive_tests/derive_float_eq/derive_float_eq_missing_ulps_tol.rs");
    t.compile_fail("tests/derive_tests/derive_float_eq/derive_float_eq_missing_debug_ulps_diff.rs");
}
