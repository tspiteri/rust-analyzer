//! See test_utils/src/marks.rs

test_utils::marks!(
    inserts_angle_brackets_for_generics
    inserts_parens_for_function_calls
    call_info_bad_offset
    dont_complete_current_use
    test_resolve_parent_module_on_module_decl
    search_filters_by_range
    dont_insert_macro_call_parens_unncessary
    self_fulfilling_completion
    test_struct_field_completion_in_func_call
    test_struct_field_completion_in_record_lit
    test_rename_struct_field_for_shorthand
    test_rename_local_for_field_shorthand
);
