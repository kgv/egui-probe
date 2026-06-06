use proc_macro::TokenStream;

mod name_display;
mod probe;

#[proc_macro_derive(EguiProbe, attributes(egui_probe))]
pub fn probe_records(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match probe::derive(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
