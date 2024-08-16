
use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use syn::DeriveInput;

#[proc_macro_derive(LL)]
pub fn ll_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // println!("{}", impl_ll_macro(&ast).to_string());
    impl_ll_macro(&ast)
}

fn impl_ll_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let basename = name.to_string();

    // Foo -> FOO_SIZE, FOO_SOLVED, FooAlg, FOO_ALG_SIZE, FOO_ALG_IDENTITY
    let size = Ident::new(&format!("{}_SIZE", basename.to_uppercase()), Span::call_site());
    let solved = Ident::new(&format!("{}_SOLVED", basename.to_uppercase()), Span::call_site());
    let alg = Ident::new(&format!("{}Alg", basename), Span::call_site());
    let alg_size = Ident::new(&format!("{}_ALG_SIZE", basename.to_uppercase()), Span::call_site());
    let alg_e = Ident::new(&format!("{}_ALG_IDENTITY", basename.to_uppercase()), Span::call_site());

    let gen = quote! {
        impl std::ops::Deref for #name {
            type Target = [usize; #size];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

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
                for ref mut e in **self {
                    *e = algs[index][*e];
                }
            }

            fn is_solved(&self) -> bool {
                *self == #solved
            }
        }

        impl std::ops::Index<usize> for #alg {
            type Output = usize;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        pub struct #alg(pub [usize; #alg_size]);

        const #alg_e: #alg = {
            let mut array = [0; #alg_size];
            let mut index = 0;
            while index < #alg_size {
                array[index] = index;
                index += 1;
            }
            #alg(array)
        };
    };

    gen.into()
}

#[proc_macro_derive(Puzzle)]
pub fn puzzle_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    // println!("{}", impl_ll_macro(&ast).to_string());
    impl_puzzle_macro(&ast)
}

fn impl_puzzle_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let basename = name.to_string();

    // Foo -> FOO_SIZE, FOO_SOLVED, FooAlg, FOO_ALG_SIZE, FOO_ALG_IDENTITY
    let size = Ident::new(&format!("{}_SIZE", basename.to_uppercase()), Span::call_site());
    let solved = Ident::new(&format!("{}_SOLVED", basename.to_uppercase()), Span::call_site());
    let alg = Ident::new(&format!("{}Alg", basename), Span::call_site());
    let alg_size = Ident::new(&format!("{}_ALG_SIZE", basename.to_uppercase()), Span::call_site());
    let alg_e = Ident::new(&format!("{}_ALG_IDENTITY", basename.to_uppercase()), Span::call_site());

    let gen = quote! {
        impl std::ops::Deref for #name {
            type Target = [usize; #size];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

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
                for ref mut e in **self {
                    *e = algs[index][*e];
                }
            }

            fn is_solved(&self) -> bool {
                *self == #solved
            }
        }

        impl std::ops::Index<usize> for #alg {
            type Output = usize;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        pub struct #alg(pub [usize; #alg_size]);

        const #alg_e: #alg = {
            let mut array = [0; #alg_size];
            let mut index = 0;
            while index < #alg_size {
                array[index] = index;
                index += 1;
            }
            #alg(array)
        };
    };

    gen.into()
}
