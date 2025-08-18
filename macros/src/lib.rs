mod gtk_box;
mod gtk_grid;

#[proc_macro]
pub fn gtk_box(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gtk_box::gtk_box(input)
}

#[proc_macro]
pub fn gtk_grid(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gtk_grid::gtk_grid(input)
}
