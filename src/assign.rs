macro_rules! assign { ($($trait:ident $method:ident)*) => {$(
    #[proc_macro_derive($trait)]
    pub fn $method(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let mut input: DeriveInput = parse_macro_input!(input);

        for param in &mut input.generics.params {
            if let GenericParam::Type(ref mut g) = param {
                g.bounds.push(parse_quote! { ::core::ops::$trait });
            }
        }

        let code = if let Data::Struct(ref struct_data) = input.data {
            match struct_data.fields {
                // struct { .. }
                Fields::Named(ref fields) => {
                    fields.named.iter().map(|field| {
                        let name = &field.ident;
                        let ty = &field.ty;
                        quote_spanned! {field.span()=>
                            <#ty as ::core::ops::$trait>::$method(&mut self.#name, other.#name);
                        }
                    })
                    .collect::<TokenStream>()
                }

                // struct(..);
                Fields::Unnamed(ref fields) => {
                    fields.unnamed.iter().enumerate().map(|(i, field)| {
                        let i = Index::from(i);
                        let ty = &field.ty;
                        quote_spanned! {field.span()=>
                            <#ty as ::core::ops::$trait>::$method(&mut self.#i, other.#i);
                        }
                    })
                    .collect::<TokenStream>()
                }

                // struct;
                Fields::Unit => TokenStream::new(), // nothing
            }
        } else {
            return quote!(compile_error!("This can only be derived on structs");).into();
        };

        let name = &input.ident;
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

        quote!(
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl #impl_generics ::core::ops::$trait for #name #ty_generics #where_clause {
                fn $method(&mut self, other: Self) {
                    #code
                }
            }
        ).into()
    }

)*}}
