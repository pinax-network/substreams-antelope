#[cfg(test)]
pub(crate) fn assert_ast_eq(actual: proc_macro2::TokenStream, expected: proc_macro2::TokenStream) {
    let expected_tokens = expected.to_string();
    let actual_tokens = actual.to_string();

    pretty_assertions::assert_eq!(expected_tokens, actual_tokens, "\n\nActual:\n {}", pretty_print_item(actual));
}

#[cfg(test)]
fn pretty_print_item(item: proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&item.to_string()).unwrap();

    prettyplease::unparse(&file)
}
