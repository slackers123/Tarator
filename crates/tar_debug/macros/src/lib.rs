use proc_macro::TokenStream;
use rand::Rng;

enum TraceInput {
    Block(syn::Block),
    ItemFn(syn::ItemFn),
    Stmt(syn::Stmt)
}

impl syn::parse::Parse for TraceInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(item_fn) = syn::ItemFn::parse(input) {
            Ok(Self::ItemFn(item_fn))
        } else if let Ok(block) = syn::Block::parse(input) {
            Ok(Self::Block(block))
        } else if let Ok(stmt) = syn::Stmt::parse(input) {
            Ok(Self::Stmt(stmt))
        } else {
            panic!("Expected function or block")
        }
    }
}


#[proc_macro_attribute]
pub fn trace(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let let_ident = random_ident("trace");

    match syn::parse_macro_input!(stream as TraceInput) {
        TraceInput::ItemFn(syn::ItemFn { attrs, vis, sig, block }) => {
            let name = format!("\"{}\"", sig.ident);
            let len = block.stmts.len();

            assert!(len > 0, "Empty Function");

            let stmts = &block.stmts[..len-1];
            let last = &block.stmts[len-1];

            if should_be_last_stmt(last) {
                quote::quote! {
                    #(#attrs)* #vis #sig {
                        #[cfg(debug_assertions)]
                        let #let_ident = Trace::new(#name, TraceType::Function);
                        #(#stmts)*
                        #[cfg(debug_assertions)]
                        #let_ident.end();
                        #last
                    }
                }
            } else {
                quote::quote! {
                    #(#attrs)* #vis #sig {
                        #[cfg(debug_assertions)]
                        let #let_ident = Trace::new(#name, TraceType::Function);
                        #(#stmts)*
                        #last
                        #[cfg(debug_assertions)]
                        #let_ident.end();
                    }
                }
            }
        }
        TraceInput::Block(syn::Block { stmts, .. }) => {
            let name = syn::parse_macro_input!(attr as syn::LitStr);
            let len = stmts.len();

            assert!(len > 0, "Empty Block");
            
            let stmts = &stmts[..len-1];
            let last = &stmts[len-1];

            if should_be_last_stmt(last) {
                quote::quote! {
                    {
                        #[cfg(debug_assertions)]
                        let #let_ident = Trace::new(#name, TraceType::Block);
                        #(#stmts)*
                        #[cfg(debug_assertions)]
                        #let_ident.end();
                        #last
                    }
                }
            } else {
                 quote::quote! {
                    {
                        #[cfg(debug_assertions)]
                        let #let_ident = Trace::new(#name, TraceType::Block);
                        #(#stmts)*
                        #last
                        #[cfg(debug_assertions)]
                        #let_ident.end();
                    }
                }               
            }
        }
        TraceInput::Stmt(stmt) => {
            let name = syn::parse_macro_input!(attr as syn::LitStr);
            
            quote::quote! {
                #[cfg(debug_assertions)]
                let #let_ident = Trace::new(#name, TraceType::Stmt);
                #stmt
                #[cfg(debug_assertions)]
                #let_ident.end();
            }
        }
    }.into()
}


#[proc_macro_attribute]
pub fn session(attr: TokenStream, stream: TokenStream) -> TokenStream {
    let file_name = syn::parse_macro_input!(attr as syn::LitStr);
    let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(stream as syn::ItemFn);

    let len = block.stmts.len();

    assert!(len > 0, "Empty Function");

    let stmts = &block.stmts[..len-1];
    let last = &block.stmts[len-1];
    let let_ident = random_ident("session");

    if should_be_last_stmt(last) {
        quote::quote! {
            #(#attrs)* #vis #sig {
                #[cfg(debug_assertions)]
                let #let_ident = Session::new(#file_name);
                #(#stmts)*
                #[cfg(debug_assertions)]
                #let_ident.end();
                #last
            }
        }
    } else {
        quote::quote! {
            #(#attrs)* #vis #sig {
                #[cfg(debug_assertions)]
                let #let_ident = Session::new(#file_name);
                #(#stmts)*
                #last
                #[cfg(debug_assertions)]
                #let_ident.end();
            }
        }
    }.into()
}


#[inline]
fn should_be_last_stmt(last: &syn::Stmt) -> bool {
    match last {
        syn::Stmt::Local(_) => true,
        syn::Stmt::Expr(_) => true,
        syn::Stmt::Semi(expr, _) => match expr {
            syn::Expr::Return(_) => true,
            _ => false
        }
        _ => false
    }
}

#[inline]
fn random_ident(name: &'static str) -> syn::Ident {
    let id: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(9)
        .map(char::from)
        .collect();

    syn::Ident::new(format!("__{name}_{id}__").as_str(), proc_macro2::Span::call_site())
}

