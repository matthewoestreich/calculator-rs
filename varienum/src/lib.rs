extern crate proc_macro;
use proc_macro::{Delimiter, TokenStream, TokenTree};

#[proc_macro_attribute]
pub fn variants_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut tokens = item.clone().into_iter();
    let mut enum_name = String::new();
    let mut variants = Vec::new();

    // Find the enum name and the braced body
    while let Some(ref tree) = tokens.next() {
        // Find enum name.
        if let TokenTree::Ident(ident) = tree
            && ident.to_string() == "enum"
            && let Some(TokenTree::Ident(name)) = tokens.next()
        {
            enum_name = name.to_string();
        }

        // Find varaints in enum.
        if let TokenTree::Group(group) = tree
            && group.delimiter() == Delimiter::Brace
        {
            let mut group_tokens = group.stream().into_iter();
            while let Some(g_tree) = group_tokens.next() {
                if let TokenTree::Ident(v_name) = g_tree {
                    variants.push(v_name.to_string());
                    // Skip until the next comma to handle tuple/struct variants or values
                    for t in group_tokens.by_ref() {
                        if let TokenTree::Punct(p) = t
                            && p.as_char() == ','
                        {
                            break;
                        }
                    }
                }
            }
        }
    }

    let variant_names = variants
        .iter()
        .map(|v| format!("\"{}\"", v))
        .collect::<Vec<_>>()
        .join(",");

    let variants_debug = variants
        .iter()
        .map(|v| format!("format!(\"{{:?}}\", {}::{})", enum_name, v))
        .collect::<Vec<_>>()
        .join(",");

    // 3. Generate the implementation as a raw string and parse it
    let mut generated = item.to_string();
    generated.push_str(&format!(
        "impl {} {{ 
            pub fn variants() -> &'static [&'static str] {{ 
                &[{}] 
            }} 
            pub fn variants_debug() -> Vec<String> {{
                vec![{}]
            }}
        }}",
        enum_name, variant_names, variants_debug
    ));

    generated.parse().expect("Failed to parse generated code")
}
