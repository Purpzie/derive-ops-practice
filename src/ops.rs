macro_rules! ops { ($($trait:ident $method:ident)*) => {$(

    #[proc_macro_derive($trait)]
    pub fn $method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let mut input: DeriveInput = parse_macro_input!(input);

        for param in &mut input.generics.params {
            if let GenericParam::Type(ref mut g) = param {
                let name = &g.ident;
                g.bounds.push(parse_quote! { ::core::ops::$trait<Output = #name> })
            }
        }

        let code = if let Data::Struct(ref struct_data) = input.data {
            match struct_data.fields {
                // struct { .. }
                Fields::Named(ref fields) => {
                    let code = fields.named.iter().map(|field| {
                        let name = &field.ident;
                        let ty = &field.ty;
                        quote_spanned! {field.span()=>
                            #name: <#ty as ::core::ops::$trait>::$method(self.#name, other.#name)
                        }
                    });

                    quote! {
                        Self { #(#code),* }
                    }
                }

                // struct(..);
                Fields::Unnamed(ref fields) => {
                    let code = fields.unnamed.iter().enumerate().map(|(i, field)| {
                        let i = Index::from(i);
                        let ty = &field.ty;
                        quote_spanned! {field.span()=>
                            <#ty as ::core::ops::$trait>::$method(self.#i, other.#i)
                        }
                    });

                    quote! {
                        Self(#(#code),*)
                    }
                }

                // struct;
                Fields::Unit => quote! { Self }
            }
        } else {
            return quote!(compile_error!("This can only be derived on structs");).into();
        };

        let name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        quote!(
            //#[automatically_derived]
            //#[allow(unused_qualifications)]
            impl #impl_generics ::core::ops::$trait for #name #ty_generics #where_clause {
                type Output = Self;
                fn $method(self, other: Self) -> Self {
                    #code
                }
            }
        ).into()
    }
)*}}
