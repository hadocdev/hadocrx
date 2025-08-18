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

struct GridItem {
    properties: Vec<Property>,
    item: syn::Expr
}

impl syn::parse::Parse for GridItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = input.parse::<syn::Expr>()?;
        let mut properties = Vec::new();
        while input.peek(syn::token::At) {
            let property = input.parse::<Property>()?;
            properties.push(property);   
        }
        Ok(GridItem { properties, item })
    }
}

struct Row {
    items: Vec<GridItem>
}

impl syn::parse::Parse for Row {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content; syn::bracketed!(content in input);
        let items = content.parse_terminated(GridItem::parse, syn::token::Comma)?.into_iter().collect();
        Ok(Row { items })
    }
}

struct GridMacro {
    properties: Vec<Property>,
    rows: Vec<Row>
}

impl syn::parse::Parse for GridMacro {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut properties = Vec::new();
        while input.peek(syn::token::At) {
            let property = input.parse::<Property>()?;
            properties.push(property);   
            if input.peek(syn::token::Comma) { _ = input.parse::<syn::token::Comma>()?; }
        }
        let rows = input.parse_terminated(Row::parse, syn::token::Comma)?.into_iter().collect(); 
        Ok(GridMacro { properties, rows })
    }
}

struct GridItemConfig {
    name: syn::Ident,
    col: syn::Expr,
    row: syn::Expr,
    colspan: syn::Expr,
    rowspan: syn::Expr
}

impl Default for GridItemConfig {
    fn default() -> Self {
        Self {
            name: quote::format_ident!("unnamed"),
            col: syn::parse_quote!(0),
            row: syn::parse_quote!(0),
            colspan: syn::parse_quote!(1),
            rowspan: syn::parse_quote!(1),
        }
    }
}

pub fn gtk_grid(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let GridMacro { properties, rows } = syn::parse_macro_input!(input as GridMacro);
    let mut child_var_configs = Vec::new();
    let mut child_assignments = Vec::new();

    for (i, row) in rows.iter().enumerate() {
        for (j, col) in row.items.iter().enumerate() {
            let var_name = quote::format_ident!("child_{}_{}", i, j);
            let mut grid_item_config = GridItemConfig::default();
            grid_item_config.name = var_name.clone();
            grid_item_config.col = syn::ExprLit { attrs: vec![], lit: syn::Lit::Int(syn::LitInt::new(&j.to_string(), proc_macro2::Span::call_site())) }.into();
            grid_item_config.row = syn::ExprLit { attrs: vec![], lit: syn::Lit::Int(syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site())) }.into();
            for property in col.properties.iter() {
                match property.name.to_string().as_str() {
                    "colspan" => { grid_item_config.colspan = property.value.clone() },
                    "rowspan" => { grid_item_config.rowspan = property.value.clone() },
                    _ => {}
                }
            }
            child_var_configs.push(grid_item_config);
            let col_expr = col.item.clone();
            child_assignments.push(quote::quote! {
                let #var_name = #col_expr;
            });
        }
    }

    let mut root_assignment = quote::quote! {
        let root = gtk::Grid::builder()
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
    for config in &child_var_configs {
        let GridItemConfig { name, col, row, colspan, rowspan } = config;
        append_statements.push(quote::quote! {
            root.attach(#name, #col, #row, #colspan, #rowspan);
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
