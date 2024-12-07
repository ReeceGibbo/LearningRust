mod basic_func_tests;
mod basic_number_tests;
mod vector_tests;
mod semantics_tests;
mod struct_tests;
mod struct_further_tests;
mod enum_tests;
mod string_tests;
mod module_tests;
mod hashmap_tests;

fn main() {
    println!("Hello, world!");
    basic_func_tests::first_test_function();
    basic_func_tests::variable_test_fix();
    basic_func_tests::add_numbers(27, 42);
}
