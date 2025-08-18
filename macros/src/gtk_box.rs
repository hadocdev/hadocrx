struct Property {
    name: syn::Ident,
    value: syn::Expr
}

impl syn::parse::Parse for Property {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::token::At>()?;
        let name = input.parse::<syn::Ident>()?;
        let value = input.parse::<syn::Expr>()?;
        Ok(Property { name, value })
    }
}

struct BoxMacro {
    properties: Vec<Property>,
    children: Vec<syn::Expr>
}

impl syn::parse::Parse for BoxMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while input.peek(syn::token::At) {
            let property = input.parse::<Property>()?;
            properties.push(property);   
            if input.peek(syn::token::Comma) {
                _ = input.parse::<syn::token::Comma>()?;
            }
        }
        let children = input.parse_terminated(syn::Expr::parse, syn::token::Comma)?.into_iter().collect();
        Ok(BoxMacro { properties, children })
    }
}

pub fn gtk_box(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let BoxMacro { properties, children } = syn::parse_macro_input!(input as BoxMacro);
    let mut child_var_names = Vec::new();
    let mut child_assignments = Vec::new();

    for (i, child_expr) in children.iter().enumerate() {
        let var_name = quote::format_ident!("child_{}", i);
        child_var_names.push(var_name.clone());
        child_assignments.push(quote::quote! {
            let #var_name = #child_expr;
        });
    }

    let mut root_assignment = quote::quote! {
        let root = gtk::Box::builder()
    };

    for property in properties.iter() {
        let Property { name, value } = &property;
        root_assignment.extend(quote::quote! {
            .#name(#value)
        });
    }
    root_assignment.extend(quote::quote! {
        .build();
    });

    let mut append_statements = Vec::new();
    for child_var_name in &child_var_names {
        append_statements.push(quote::quote! {
            root.append(&#child_var_name);
        });
    }

    let output = quote::quote! {
        {
            #(#child_assignments)*
            #root_assignment
            #(#append_statements)*
            root
        }
    };

    output.into()
}
