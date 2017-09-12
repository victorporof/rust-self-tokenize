/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use quote;
use syn;

pub fn generate_impl(ast: &syn::DeriveInput, to_tokens_impl: bool, to_custom_tokens_impl: bool, body: quote::Tokens) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let to_tokens_impl_body = if to_tokens_impl {
        quote! {
            impl #impl_generics ::self_tokenize_trait::ToTokens for #name #ty_generics #where_clause {
                fn to_tokens(&self, tokens: &mut ::self_tokenize_trait::Tokens) {
                    self.to_custom_tokens(tokens);
                }
            }
        }
    } else {
        quote!{}
    };

    let to_custom_tokens_impl_body = if to_custom_tokens_impl {
        quote! {
            impl #impl_generics ::self_tokenize_trait::ToCustomTokens for #name #ty_generics #where_clause {
                fn to_custom_tokens(&self, tokens: &mut ::self_tokenize_trait::Tokens) {
                    #body
                }
            }
        }
    } else {
        quote!{}
    };

    quote! {
        #to_tokens_impl_body
        #to_custom_tokens_impl_body
    }
}

pub fn expand_macro(ast: &syn::DeriveInput, to_tokens_impl: bool, to_custom_tokens_impl: bool) -> quote::Tokens {
    match ast.body {
        syn::Body::Struct(ref data) => expand_struct(ast, data, to_tokens_impl, to_custom_tokens_impl),
        syn::Body::Enum(ref data) => expand_enum(ast, data, to_tokens_impl, to_custom_tokens_impl)
    }
}

pub fn expand_struct(ast: &syn::DeriveInput, data: &syn::VariantData, to_tokens_impl: bool, to_custom_tokens_impl: bool) -> quote::Tokens {
    match data {
        &syn::VariantData::Unit => expand_unit_struct(ast, to_tokens_impl, to_custom_tokens_impl),
        &syn::VariantData::Tuple(ref fields) => expand_tuple_struct(ast, fields, to_tokens_impl, to_custom_tokens_impl),
        &syn::VariantData::Struct(ref fields) => expand_struct_struct(ast, fields, to_tokens_impl, to_custom_tokens_impl)
    }
}

pub fn expand_unit_struct(ast: &syn::DeriveInput, to_tokens_impl: bool, to_custom_tokens_impl: bool) -> quote::Tokens {
    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();

    generate_impl(
        ast,
        to_tokens_impl,
        to_custom_tokens_impl,
        quote! {
            tokens.append(#struct_name_str);
        }
    )
}

pub fn expand_tuple_struct(
    ast: &syn::DeriveInput,
    fields: &[syn::Field],
    to_tokens_impl: bool,
    to_custom_tokens_impl: bool
) -> quote::Tokens {
    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();
    let fields_tokenizer = expand_tuple_struct_fields(fields);

    generate_impl(
        ast,
        to_tokens_impl,
        to_custom_tokens_impl,
        quote! {
            tokens.append(#struct_name_str);
            tokens.append("(");
            #fields_tokenizer
            tokens.append(")");
        }
    )
}

pub fn expand_struct_struct(
    ast: &syn::DeriveInput,
    fields: &[syn::Field],
    to_tokens_impl: bool,
    to_custom_tokens_impl: bool
) -> quote::Tokens {
    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();
    let fields_tokenizer = expand_struct_struct_fields(fields);

    generate_impl(
        ast,
        to_tokens_impl,
        to_custom_tokens_impl,
        quote! {
            tokens.append(#struct_name_str);
            tokens.append("{");
            #fields_tokenizer
            tokens.append("}");
        }
    )
}

pub fn expand_enum(ast: &syn::DeriveInput, variants: &[syn::Variant], to_tokens_impl: bool, to_custom_tokens_impl: bool) -> quote::Tokens {
    let variants_iterator = variants.iter().map(|variant| {
        let struct_name = &ast.ident;
        let variant_name = &variant.ident;

        match variant.data {
            syn::VariantData::Unit => expand_match_enum_unit_variant(struct_name, variant_name),
            syn::VariantData::Tuple(ref fields) => expand_match_enum_tuple_variant(struct_name, variant_name, fields),
            syn::VariantData::Struct(ref fields) => expand_match_enum_struct_variant(struct_name, variant_name, fields)
        }
    });

    generate_impl(
        ast,
        to_tokens_impl,
        to_custom_tokens_impl,
        quote! {
            match self {
                #( #variants_iterator )*
            }
        }
    )
}

pub fn expand_tuple_struct_fields(fields: &[syn::Field]) -> quote::Tokens {
    let fields_iterator = fields
        .iter()
        .enumerate()
        .map(|(field_index, _)| expand_tuple_struct_anonymous_field_value(field_index, fields.len()));

    quote! {
        #( #fields_iterator )*
    }
}

pub fn expand_struct_struct_fields(fields: &[syn::Field]) -> quote::Tokens {
    let idents_len = fields
        .iter()
        .filter(|field| field.ident.as_ref().is_some())
        .count();

    let fields_iterator = fields
        .iter()
        .filter(|field| field.ident.as_ref().is_some())
        .enumerate()
        .map(|(field_index, field)| expand_struct_struct_named_field_key_and_value(field_index, idents_len, field));

    quote! {
        #( #fields_iterator )*
    }
}

pub fn expand_match_enum_unit_variant(struct_name: &syn::Ident, variant_name: &syn::Ident) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let ident_str = ident.to_string();

    quote! {
        &#ident => {
            tokens.append(#ident_str);
        },
    }
}

pub fn expand_match_enum_tuple_variant(struct_name: &syn::Ident, variant_name: &syn::Ident, fields: &[syn::Field]) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let ident_str = ident.to_string();

    let fields_ident_iterator = fields
        .iter()
        .enumerate()
        .map(|(field_index, _)| get_ident_for_index(field_index));

    let fields_tokenizer_iterator = fields
        .iter()
        .enumerate()
        .map(|(field_index, _)| expand_enum_anonymous_field_value(field_index, fields.len()));

    quote! {
        &#ident( #(ref #fields_ident_iterator),* ) => {
            tokens.append(#ident_str);
            tokens.append("(");
            #( #fields_tokenizer_iterator )*
            tokens.append(")");
        },
    }
}

pub fn expand_match_enum_struct_variant(struct_name: &syn::Ident, variant_name: &syn::Ident, fields: &[syn::Field]) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let ident_str = ident.to_string();

    let idents_len = fields
        .iter()
        .filter(|field| field.ident.as_ref().is_some())
        .count();

    let fields_ident_iterator = fields
        .iter()
        .map(|field| field.ident.as_ref())
        .filter(|ident| ident.is_some());

    let fields_tokenizer_iterator = fields
        .iter()
        .filter(|field| field.ident.as_ref().is_some())
        .enumerate()
        .map(|(field_index, field)| expand_enum_named_field_value(field_index, idents_len, field));

    quote! {
        &#ident { #(ref #fields_ident_iterator),* } => {
            tokens.append(#ident_str);
            tokens.append("{");
            #( #fields_tokenizer_iterator )*
            tokens.append("}");
        },
    }
}

pub fn expand_struct_struct_named_field_key_and_value(field_index: usize, fields_count: usize, field: &syn::Field) -> quote::Tokens {
    let field_key_tokenizer = expand_struct_struct_named_field_key(field);
    let field_value_tokenizer = expand_struct_struct_named_field_value(field_index, fields_count, field);

    quote! {
        #field_key_tokenizer
        tokens.append(":");
        #field_value_tokenizer
    }
}

pub fn expand_struct_struct_named_field_key(field: &syn::Field) -> quote::Tokens {
    let field_name = field
        .ident
        .as_ref()
        .expect(&format!("Expected identifier for field `{:?}`", field));

    let field_name_str = field_name.to_string();

    quote! {
        tokens.append(#field_name_str);
    }
}

pub fn expand_struct_struct_named_field_value(field_index: usize, fields_count: usize, field: &syn::Field) -> quote::Tokens {
    let field_name = field
        .ident
        .as_ref()
        .expect(&format!("Expected identifier for field `{:?}`", field));

    let accessor = quote! { self.#field_name };
    expand_field_value(field_index, fields_count, &accessor)
}

pub fn expand_tuple_struct_anonymous_field_value(field_index: usize, fields_count: usize) -> quote::Tokens {
    let field_name = syn::Ident::from(field_index);
    let accessor = quote! { self.#field_name };
    expand_field_value(field_index, fields_count, &accessor)
}

pub fn expand_enum_named_field_value(field_index: usize, fields_count: usize, field: &syn::Field) -> quote::Tokens {
    let field_name = field
        .ident
        .as_ref()
        .expect(&format!("Expected identifier for field `{:?}`", field));

    let field_name_str = field_name.to_string();
    let accessor = quote! { #field_name };
    let field_tokenizer = expand_field_value(field_index, fields_count, &accessor);

    quote! {
        tokens.append(#field_name_str);
        tokens.append(":");
        #field_tokenizer
    }
}

pub fn expand_enum_anonymous_field_value(field_index: usize, fields_count: usize) -> quote::Tokens {
    let field_name = get_ident_for_index(field_index);
    let accessor = quote! { #field_name };
    expand_field_value(field_index, fields_count, &accessor)
}

pub fn expand_field_value(field_index: usize, fields_count: usize, accessor: &quote::Tokens) -> quote::Tokens {
    let separator = if field_index == fields_count - 1 {
        quote!{}
    } else {
        quote! { tokens.append(","); }
    };

    quote! {
        #accessor.to_custom_tokens(tokens);
        #separator
    }
}

pub fn get_ident_for_index(i: usize) -> syn::Ident {
    syn::Ident::from(format!("_{}", i))
}
