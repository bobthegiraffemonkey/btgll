
use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::DeriveInput;

#[proc_macro_derive(LL)]
pub fn ll_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // panic!("{}", impl_ll_macro(&ast).to_string())
    impl_ll_macro(&ast)
}

fn impl_ll_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let basename = name.to_string();

    // Foo -> FOO_SIZE, FOO_SOLVED, FooAlg
    let size = Ident::new(&format!("{}_SIZE", basename.to_uppercase()), Span::call_site());
    let solved = Ident::new(&format!("{}_SOLVED", basename.to_uppercase()), Span::call_site());
    let alg = Ident::new(&format!("{}Alg", basename), Span::call_site());
    let alg_size = Ident::new(&format!("{}ALG_SIZE", basename.to_uppercase()), Span::call_site());

    let gen = quote! {
        pub struct #alg([usize; #alg_size]);

        const #solved: #name = {
            let mut array = [0; #size];
            let mut index = 0;
            while index < #size {
                array[index] = index;
                index += 1;
            }
            #name(array)
        };

        impl LL for #name {
            type Alg = #alg;

            fn apply(&mut self, algs: &[Self::Alg], index: usize) {
                for e in self {
                    *e = algs[index][*e];
                }
            }

            fn is_solved(&self) -> bool {
                *self == #solved
            }
        }

        impl IntoIterator for &mut #name {
            type Item = &mut usize;

            type IntoIter = std::array::IntoIter<Self::Item, #size>;

            fn into_iter(&mut self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl std::ops::Index<usize> for #alg {
            type Output = usize;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }
    };

    gen.into()
}
